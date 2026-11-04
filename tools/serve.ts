// Minimal dev server: bundles index.html and the tool pages (the replay viewer, sounds, music; TS, WGSL as text) via Bun's HTML imports, serves ./public statically,
// and answers the game's fetches of level files under /levels/ (tools/levels-api.ts: index and read, loopback only).
// Listens on LOOPBACK only (HOST=… overrides that on purpose).
import index from "../index.html";
import replay from "../replay.html";   // the REPLAY VIEWER (src/replay/viewer.ts): a replay saved with F7 as a map with a scrubber and notes
import sounds from "../sounds.html";   // the LISTENING PAGE (src/sounds/main.ts): every sound the game makes, one row each, through the game's own mixer graph
import music from "../music.html";   // the SCORE'S LISTENING PAGE (src/music/main.ts): every piece of the score on one row, played by the game's own sequencer and voices
import { handleLevels } from "./levels-api.ts";
import { localCloud } from "./cloud/local.ts";
import { devRunner, type DevRunner } from "./mp/runner.ts";

const COMMIT = (() => { try { const r = Bun.spawnSync(["git", "rev-parse", "--short=12", "HEAD"]); return r.success ? new TextDecoder().decode(r.stdout).trim() : ""; } catch { return ""; } })();   // (read once at start: a dev server outlives commits, but the journal only wants "about which build")
let CLOUD: ReturnType<typeof localCloud> = null;   // the local build's backend (tools/cloud/local.ts), started below once the port is ours: null on a plain start — `bun run dev` asks for one (TPS_LOCAL_CLOUD=1), TPS_DEV_CLOUD names one that is already up
let MP: DevRunner | null = null;   // co-op's game servers on this machine (tools/mp/runner.ts): the local backend asks this server to start and stop them. null where no backend was asked for
const port = Number(process.env.PORT ?? 5173);
const hostname = process.env.HOST ?? "127.0.0.1";
const server = Bun.serve({
  port, hostname,
  development: { hmr: false, console: true },
  routes: {
    "/": index, "/replay": replay, "/sounds": sounds, "/music": music,
    "/sneaklab": req => { const q = new URL(req.url).search.slice(1); return Response.redirect("/?levelFile=sneak-lab.json&solo&sneaklab" + (q ? "&" + q : ""), 302); },   // the sneak lab: the close-in on a man, its numbers live (src/host/sneaklab.ts)
    "/firelab": req => { const q = new URL(req.url).search.slice(1); return Response.redirect("/?levelFile=pass.json&roomclear&squad4&nofow&firelab" + (q ? "&" + q : ""), 302); },   // the fire lab: beside the pass's brazier, the flame's numbers live (src/host/firelab.ts)
  },
  async fetch(req) {
    const url = new URL(req.url);
    let path: string; try { path = decodeURIComponent(url.pathname); } catch { return new Response("bad path encoding", { status: 400 }); }
    if (path.includes("..")) return new Response("bad path", { status: 400 });
    if (path === "/index.html") return Response.redirect("/" + url.search + url.hash, 302);   // the game is '/', and a built page's neighbour link ('index.html?…') must open here too
    if (path === "/replay.html" || path === "/replay/") return Response.redirect("/replay" + url.search, 302);
    if (path === "/sounds.html" || path === "/sounds/") return Response.redirect("/sounds" + url.search, 302);
    if (path === "/music.html" || path === "/music/") return Response.redirect("/music" + url.search, 302);
    { const mp = await MP?.handle(req); if (mp) return mp; }   // /mp/runner/…: the backend's calls, by the runner's key
    const lv = await handleLevels(req); if (lv) return lv;
    if (path === "/qa/commit") return new Response(COMMIT, { headers: { "Cache-Control": "no-store", "Content-Type": "text/plain" } });   // the commit this server was started on (the session journal files it with a recording: src/ui/sessionrec.ts)
    if (path === "/qa/cloud") return new Response(CLOUD?.url() ?? "", { headers: { "Cache-Control": "no-store", "Content-Type": "text/plain" } });   // the backend this server runs for the page, or nothing (src/cloud/cloud.ts cloudFind asks once a page load)
    if (/^\/qa\/[\w.-]+\.js$/.test(path)) { const qa = Bun.file("./tools" + path); if (await qa.exists()) return new Response(qa, { headers: { "Cache-Control": "no-store" } }); }   // the in-page measurement harnesses (dev only, never bundled): `await import('/qa/perf-harness.js')` → window.__perf, '/qa/perf-frames.js' → window.__pf
    { const m = path.match(/^\/qa\/shader\/([a-z_0-9]+)\.wgsl$/); if (m) { const sf = Bun.file(`./src/render/shaders/${m[1]}.wgsl`); if (await sf.exists()) return new Response(sf, { headers: { "Cache-Control": "no-store", "Content-Type": "text/plain" } }); } }   // a pass shader's current source on disk, for the page's hot-swap hook (main.ts __hotVol): edit, fetch, recompile, A/B — without rebooting a staged scene
    const f = Bun.file("./public" + path);
    if (await f.exists()) return new Response(f);
    return new Response("not found: " + path, { status: 404 });
  },
});
// (after the port is ours: a server that could not start must not leave a backend running behind it)
CLOUD = localCloud();
if (CLOUD) MP = devRunner({ cloud: () => CLOUD?.url() ?? "", origin: `http://${hostname === "0.0.0.0" ? "127.0.0.1" : hostname}:${server.port}`, build: COMMIT });
if (CLOUD) for (const sig of ["SIGINT", "SIGTERM", "SIGHUP"] as const) process.on(sig, () => { MP?.stop(); CLOUD?.stop(); process.exit(0); });
console.log(`dev server: http://${hostname === "127.0.0.1" ? "localhost" : hostname}:${server.port}${hostname === "127.0.0.1" || hostname === "localhost" ? "" : "  (HOST override: NOT loopback-only)"}  ·  level files: /levels/`);
