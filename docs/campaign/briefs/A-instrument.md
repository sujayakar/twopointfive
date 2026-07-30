# Track A — instrument (work counters + reservoir-buffer merge)

Owner branch: `claude/instrument`, worktree `~/src/twopointfive-wt/instrument`.
Base: `claude/campaign` @ `82baea3`.

Read first: `README.md` (whole thing), `docs/campaign/STATUS.md`,
`tools/headless/README.md`, then `src/engine/renderer.ts` (pass
orchestration, `ptLayout`, `Targets`, `buildBindGroups`, the profiler
hookup), `src/shaders/common.wgsl` (`trace`, `occluded`, `occludedSkipping`,
`hitBox`, RIS `proposeCandidate`/`generateReservoir`, `sampleIndirectRIS`),
`src/shaders/pathtrace.wgsl` (the `main` bounce loop, `volumetricBeams`,
`restirDirect`, `restirGI`), `src/engine/profiler.ts` (the readback pattern
you will mirror), `src/main.ts` (`bench`, `benchInner`, `stats`,
`compareToReference`, the `__…` window hooks near the bottom).

## Why this track exists

The campaign has no GPU. Every performance decision downstream needs a
machine-independent measure: how many rays, shadow rays, BVH node visits,
box tests and RIS candidates a frame costs, per pixel. Those numbers are
identical on SwiftShader and on an M1 Max, so they let us evaluate
algorithmic changes here and only ask Sujay's Mac for milliseconds. You are
building that measuring stick. Everything after you leans on it — accuracy
of the counts matters more than anything else you do.

## Deliverables, in order

### 1. Merge the reservoir ping-pong pairs (frees two storage-buffer slots)

The trace pass is at the 10-storage-buffers-per-stage ceiling (scene group
6 + `reservoirPrev/Cur` + `giPrev/Cur`; see the comment in `gpu.ts`). A
counters buffer would be an eleventh. Merge each ping-pong pair into ONE
buffer of twice the size, addressed by a parity offset:

- `reservoirs: array<Reservoir>` with `let base = select(0u, N, parity)`
  style prev/cur addressing (parity from the existing frame parity the
  renderer already tracks — thread it as a uniform field, not a rebound
  buffer). Same for `GIReservoir`. Both accesses become `read_write` on one
  binding; keep the prev half strictly read-only *by convention* and say so
  in a comment.
- Update `ptLayout`, the WGSL decls (`@binding(5..9)` region), the
  `Targets` fields and their creation/destroy sites, and every read/write
  site. The bind group loses two entries. Do not renumber unrelated
  bindings.
- Drop the requirement in `gpu.ts` from 10 to what is now true (count it —
  don't guess), and fix the comments that claim we are at the ceiling
  (`gpu.ts` requiredLimits comment, `renderer.ts` radiosity layout comment).

Commit this alone once green. The estimator must be bit-for-bit the same
computation — this is pure re-addressing.

### 2. Work counters

- A `counters` storage buffer (atomic u32s) bound in the trace pass. A
  small named enum of slots — at minimum: primary rays, bounce rays by
  depth (0..N), shadow rays split by purpose (direct-light NEE, RIS winner,
  transient, flashmap already has its own pass — count its rays too if it is
  cheap), BVH node visits, box (OBB) tests, RIS candidates evaluated,
  volumetric march steps, radiosity gather reads. Justify additions or
  omissions in the track report.
- Gate every increment on a uniform (`countersOn`); off by default. When
  off the cost must be a not-taken branch on a uniform — nothing else.
  Do not workgroup-aggregate for now; note in a comment that atomic
  contention makes counters-ON timings meaningless, which is fine because
  counters mode is for counting, never for timing.
- Readback mirrors `GpuProfiler`: clear the buffer at frame start, copy to
  a mappable buffer after the trace pass, async mapAsync, land the totals
  in `stats` (see `main.ts`). Expose both raw per-frame totals and a
  per-pixel normalization (divide by the internal pixel count). Field:
  `__stats.counters`, updated a frame or two late like the profiler.
- Surface: `settings.counters` (bool, default false, persisted like other
  settings? — no: treat it like the volatile debug modes in
  `settings-store.ts` and do NOT persist it), a debug-panel toggle in the
  same group as the other debug knobs, and inclusion in the `__bench`
  return JSON when on. `__bench` output when counters are off must be
  unchanged.

Correctness bar: counts must reconcile with reasoning. E.g. primary rays =
pixels; bounce-1 rays ≈ pixels × (fraction of pixels whose primary hit is
valid) × spp; shadow rays per pixel should match the README's "3 shadow
rays" claim at bounce 0 within reason. If a count contradicts the code's
own comments, chase it until one of them is corrected — write down which.

### 3. Bench-resolution hook for headless work

`BENCH_WIDTH/HEIGHT` in `main.ts` are constants (1152×720), which makes
`__bench` and `__compareToReference` unusable on SwiftShader (≈14 s/frame).
Add `__benchResolution(w, h)` that overrides them for the session (default
unchanged), with a one-line note in the returned string when non-default
so a result never silently means a different pixel count. Then extend
`tools/headless/run.py` with `--bench-res W H` that calls it before
`--bench`.

## Verification you must do (evidence goes in the track report)

Build/type: `npm run typecheck`, `npm run build` after every step — both
clean.

Headless (from the worktree root; Chromium only launches with the Bash
sandbox disabled — `dangerouslyDisableSandbox: true`, reason "chromium
needs its own namespaces"):

```bash
python3 tools/headless/run.py --scenario /tmp/counters-check.js --shot /tmp/a-instr.png --json /tmp/a-instr.json
```

where your scenario resizes to ~384×240 (`window.__renderer.resize`),
turns counters on via `__settings`, renders ≥5 stills with
`__renderStill`, and returns `window.__stats`. Assert in your report:
counters nonzero, primary rays = pixels, per-pixel ratios stated. Then run
the SAME scenario with counters off and confirm the frame is visually the
same — Read both PNGs and say what you see (not "looks fine": name the
scene features). Also confirm `__bench(2, true)` at a small
`__benchResolution` returns and its JSON gains a `counters` block only when
counters are on.

Regression: `__compareToReference` at a small bench resolution across two
or three of the standard configs must produce RMSE within noise of the same
run on the base commit `82baea3` (run it there first, from a scratch build,
and quote both numbers). The reservoir merge must not move image content;
if RMSE shifts beyond the expected 1-spp jitter, you have changed the
estimator — stop and find it.

## Rules

- Work only inside `~/src/twopointfive-wt/instrument`. Never touch
  `~/src/twopointfive` (the trunk clone) or any other worktree.
- Commit to `claude/instrument` at every green step and at least every
  hour of work (heartbeat). Never `git stash`. Never rewrite pushed history.
- Follow the codebase's comment style: short, *why* not *what*, no edit
  narration. Match the existing WGSL/TS idiom exactly.
- If you find a real bug unrelated to your task, note it in the report's
  "Findings" section — do not fix it here.

## Report

Write `docs/campaign/tracks/A-instrument.md` (committed on your branch):
what changed and why in ≤15 lines, then verification evidence with actual
numbers (counter table at the pinned pose, RMSE before/after, screenshot
descriptions), then Findings, then anything the coordinator must watch
when merging. Your final message returns the same content plus the branch
HEAD sha.
