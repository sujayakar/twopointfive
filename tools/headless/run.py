#!/usr/bin/env python3
"""
Headless render harness for twopointfive.

Runs the built demo in Chromium against SwiftShader (software WebGPU), then
drives the debug hooks main.ts hangs off window (__bench, __renderStill,
__compareToReference, __stats, ...). This exists so a machine with no GPU can
still catch WGSL compile errors, validation errors, black frames, and can read
the tracer's machine-independent work counters. Timings on SwiftShader mean
nothing for a real GPU — the cost structure is different — so bench numbers
from here are a smoke signal only; measure ms on real hardware.

    npm run build
    python3 tools/headless/run.py --bench 3 --shot out.png --json out.json

Exits non-zero on: page/console error, GPU init failure, WGSL compile error,
device loss, or a thrown scenario. Requires Playwright's Python package and a
Chromium build (CHROME_BIN, else the newest ~/.cache/ms-playwright one).
"""
import argparse
import asyncio
import glob
import http.server
import json
import os
import socket
import socketserver
import subprocess
import sys
import threading
import time
import urllib.request

from playwright.async_api import async_playwright

REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
DIST = os.path.join(REPO, "dist")

# Console lines matching these are diagnostics the app emits on purpose, not
# failures. Everything else at error level fails the run.
BENIGN_ERROR_SUBSTRINGS = (
    "favicon.ico",
    "Failed to load resource",
)


def find_chrome() -> str:
    env = os.environ.get("CHROME_BIN")
    if env:
        return env
    pats = [
        os.path.expanduser("~/.cache/ms-playwright/chromium-*/chrome-linux*/chrome"),
        os.path.expanduser("~/.cache/ms-playwright/chromium-*/chrome-mac/Chromium.app/Contents/MacOS/Chromium"),
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
    ]
    for pat in pats:
        hits = sorted(glob.glob(pat))
        if hits:
            return hits[-1]
    sys.exit("no Chromium found; set CHROME_BIN")


def free_port() -> int:
    with socket.socket() as s:
        s.bind(("127.0.0.1", 0))
        return s.getsockname()[1]


def serve_dist(port: int) -> socketserver.TCPServer:
    class Handler(http.server.SimpleHTTPRequestHandler):
        def __init__(self, *a, **kw):
            super().__init__(*a, directory=DIST, **kw)

        def log_message(self, format, *args):  # noqa: A002 — keep the harness output readable
            pass

    httpd = socketserver.TCPServer(("127.0.0.1", port), Handler)
    threading.Thread(target=httpd.serve_forever, daemon=True).start()
    return httpd


# Chromium refuses hardware WebGPU headless on Linux, so this is SwiftShader
# throughout. The GL side must be ANGLE-on-SwiftShader too: without it the
# compositor cannot allocate the canvas swapchain images, the crash tears the
# WebGPU instance down, and the page runs on with a dead device that no-ops
# every call (frames "run" at rAF rate doing nothing). The watchdog is off
# because software path tracing legitimately takes seconds per submission.
CHROME_FLAGS = [
    "--headless=new",
    "--no-sandbox",
    "--disable-dev-shm-usage",
    "--no-first-run",
    "--enable-unsafe-webgpu",
    "--ignore-gpu-blocklist",
    "--enable-features=Vulkan",
    "--use-vulkan=swiftshader",
    "--use-gl=angle",
    "--use-angle=swiftshader",
    "--enable-unsafe-swiftshader",
    "--disable-gpu-watchdog",
    "--disable-gpu-process-crash-limit",
    "--enable-logging=stderr",
    "--v=0",
    "--window-size=1280,800",
]
CHROME_FLAGS += os.environ.get("TPF_CHROME_FLAGS_EXTRA", "").split()

SEED_SETTINGS = json.dumps({"version": 1, "revision": 3, "calibrated": True, "settings": {}})


async def run(args: argparse.Namespace) -> int:
    if not os.path.isfile(os.path.join(DIST, "index.html")):
        sys.exit("dist/ missing — run `npm run build` first")
    chrome = find_chrome()
    http_port = free_port()
    httpd = serve_dist(http_port)
    cdp_port = free_port()
    udd = f"/tmp/twopointfive-headless-{os.getpid()}"
    proc = subprocess.Popen(
        [chrome, *CHROME_FLAGS, f"--remote-debugging-port={cdp_port}", f"--user-data-dir={udd}"],
        stdout=subprocess.DEVNULL, stderr=open(f"{udd}.log", "w"),
    )
    result: dict = {"ok": False, "console": [], "errors": [], "chrome_log": f"{udd}.log"}
    try:
        for _ in range(120):
            try:
                urllib.request.urlopen(f"http://127.0.0.1:{cdp_port}/json/version", timeout=1)
                break
            except Exception:
                time.sleep(0.25)
        else:
            result["errors"].append("chromium never opened its DevTools port")
            return finish(args, result)

        async with async_playwright() as pw:
            browser = await pw.chromium.connect_over_cdp(f"http://127.0.0.1:{cdp_port}")
            page = await browser.contexts[0].new_page()

            def on_console(m):
                line = f"{m.type}: {m.text}"
                result["console"].append(line)
                if m.type == "error" and not any(b in m.text for b in BENIGN_ERROR_SUBSTRINGS):
                    result["errors"].append(line)
                if args.verbose:
                    print(line, flush=True)
            page.on("console", on_console)
            page.on("pageerror", lambda e: result["errors"].append(f"pageerror: {e}"))

            await page.add_init_script(
                f"localStorage.setItem('twopointfive.settings', JSON.stringify({SEED_SETTINGS}));")
            await page.goto(f"http://127.0.0.1:{http_port}/", wait_until="load")

            ready = False
            for _ in range(args.startup_timeout):
                ready = await page.evaluate("() => !!(window.__renderer && window.__bench)")
                if ready or result["errors"]:
                    break
                await asyncio.sleep(1)
            result["ready"] = ready
            if not ready:
                result["errors"].append("renderer never came up (window.__renderer unset)")
            else:
                # Let it draw a few real frames before poking it.
                await asyncio.sleep(args.settle)
                # First, so a scenario that calls __compareToReference / __bench
                # inherits the small resolution rather than the standard one.
                if args.bench_res is not None:
                    w, h = args.bench_res
                    result["bench_res"] = await page.evaluate(
                        f"() => window.__benchResolution({w}, {h}, {args.bench_cap_s})")
                if args.scenario:
                    with open(args.scenario) as f:
                        src = f.read()
                    result["scenario"] = await page.evaluate(src)
                    # A scenario that reports its own verdict fails the run when
                    # it says no — numbers alone are not an assert.
                    sc = result["scenario"]
                    if isinstance(sc, dict) and sc.get("ok") is False:
                        result["errors"].append(
                            f"scenario failed: {json.dumps(sc.get('failures'))}")
                if args.bench is not None:
                    result["bench"] = await page.evaluate(
                        f"async () => await window.__bench({args.bench}, true)")
                result["stats"] = await page.evaluate(
                    "() => JSON.parse(JSON.stringify(window.__stats ?? null))")
                if args.shot:
                    await page.screenshot(path=args.shot)
                    result["shot"] = args.shot
            result["ok"] = ready and not result["errors"]
            await browser.close()
    except Exception as e:  # noqa: BLE001 — surface it as a harness failure
        result["errors"].append(f"harness: {e}")
    finally:
        proc.terminate()
        try:
            proc.wait(timeout=10)
        except subprocess.TimeoutExpired:
            proc.kill()
        httpd.shutdown()
    return finish(args, result)


def finish(args: argparse.Namespace, result: dict) -> int:
    if args.json:
        with open(args.json, "w") as f:
            json.dump(result, f, indent=2)
    print(json.dumps({k: v for k, v in result.items() if k != "console"}, indent=2))
    if result["errors"]:
        print(f"\nFAILED with {len(result['errors'])} error(s):")
        for e in result["errors"]:
            print("  " + e)
        return 1
    return 0 if result["ok"] else 1


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--bench", type=int, default=None, help="frames for __bench(n, serial)")
    ap.add_argument("--bench-res", type=int, nargs=2, metavar=("W", "H"), default=None,
                    help="__benchResolution(W, H) before --bench; keep it small under SwiftShader")
    ap.add_argument("--bench-cap-s", type=int, default=1500,
                    help="wall-clock deadline (s) passed with --bench-res; software frames are ~1s each")
    ap.add_argument("--scenario", help="JS file whose expression is evaluated in the page")
    ap.add_argument("--shot", help="write a screenshot here")
    ap.add_argument("--json", help="write the result blob here")
    ap.add_argument("--settle", type=float, default=3.0, help="seconds of free-running frames first")
    ap.add_argument("--startup-timeout", type=int, default=240, help="seconds to wait for init")
    ap.add_argument("--verbose", action="store_true", help="stream console lines")
    args = ap.parse_args()
    sys.exit(asyncio.run(run(args)))


if __name__ == "__main__":
    main()
