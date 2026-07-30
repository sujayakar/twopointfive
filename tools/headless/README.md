# Headless harness

Runs the built demo in Chromium with **SwiftShader** (software WebGPU) and
drives the debug hooks on `window` (`__bench`, `__renderStill`,
`__renderMotion`, `__compareToReference`, `__stats`, `__settings`, …). It
exists so a box with no GPU can still verify the whole renderer end to end:

- WGSL compile errors, pipeline/bind-group validation errors, uncaptured
  device errors and page exceptions all fail the run (`ok: false`);
- a screenshot proves the frame is not black;
- the tracer's work counters (rays, shadow rays, BVH visits) are
  machine-independent, so they *are* a valid measurement here.

**Timings from SwiftShader are not measurements.** The cost structure of a
CPU-side JIT bears no relation to a GPU wavefront — an optimization that
wins here can lose on silicon and vice versa. `pathtrace` alone is ~1.9 s per
1152×720 frame. Bench milliseconds only mean something on real hardware
(`__bench(60)` in Chrome on the target machine).

## Use

```bash
npm run build
python3 tools/headless/run.py --bench 3 --shot /tmp/frame.png --json /tmp/run.json
python3 tools/headless/run.py --scenario my-check.js   # any JS expression, awaited
```

Needs Playwright's Python package and a Chromium build (`$CHROME_BIN`, or
the newest under `~/.cache/ms-playwright/`). Everything is one process
tree: the script serves `dist/` itself, launches Chromium, connects over
CDP, and tears both down.

Software tracing is slow: keep smoke checks at low internal resolution.
`__bench` pins 1152×720 and its own warm-up (~5 min a run here); for
iteration prefer a `--scenario` that calls `__renderer.resize(384, 240)`,
renders a handful of frames via `__renderStill`, and reads `__stats`.

## Gameplay scenarios (`scenarios/`)

Gameplay asserts need game time, not wall time. A scenario opens with
`window.__pause(true)` (the rAF loop stands down) and steps the world with
`__renderStill(n, dtMs)`, which advances exactly `dtMs` per frame however
long SwiftShader spends tracing it — 50 ms is the game's own dt cap, so
`__renderStill(20, 50)` is precisely one second. `__guards.frozen = true`
additionally pins every guard's feet when a scenario wants to measure one
variable against fixed geometry.

A scenario returns a plain object. If it carries `ok: false` the run fails
and its `failures` list is printed; everything else in the object is the
measurement. Numbers, then a verdict — never a verdict alone.

## Why the flags look like that

Headless Chromium on Linux will not do hardware WebGPU, and its GPU process
pins SwiftShader regardless of what Vulkan ICDs the machine has (Mesa's
lavapipe included) — so software it is. Two non-obvious flags carry the
setup:

- `--use-gl=angle --use-angle=swiftshader`: the *compositor's* GL side must
  also be SwiftShader. Without it, canvas presentation cannot allocate the
  swapchain shared images, the failure tears down the WebGPU instance, and
  the page carries on with a dead device whose calls are all no-ops — frames
  appear to run at rAF rate while nothing renders. `device.lost` resolves
  with reason `destroyed`, which the app deliberately ignores.
- `--disable-gpu-watchdog`: one traced frame is a multi-second submission,
  well past the hang threshold.

SwiftShader reports `maxStorageTexturesPerShaderStage = 4` (WebGPU's own
default). The trace pass fits because its three radiance signals share one
`texture_storage_2d_array` binding; keep it that way.
