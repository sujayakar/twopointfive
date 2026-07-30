# Track A — instrument (work counters + reservoir merge)

Branch `claude/instrument`, base `82baea3`. Three commits:
`1ab7489` bench-resolution/deadline hook, `ee577da` reservoir merge,
`5503118` work counters.

## What changed and why

1. `__benchResolution(w, h, capSeconds?)` overrides `__bench` /
   `__compareToReference`'s pinned 1152×720 and their 8 s / 120 s wall-clock
   guards for the session; result blobs carry `nonStandardRes` so a small-frame
   number never reads as a standard one. `run.py --bench-res W H` calls it
   before any scenario. Without it neither hook could run on SwiftShader — a
   frame is ~1 s regardless of resolution (per-pass dispatch overhead), so the
   guards fired before anything was measured.
2. Reservoir ping-pong pairs (DI, GI) merged into one buffer each holding both
   parity halves, addressed by a new `U.parity` uniform; the prev half is
   read-only by convention. Trace pass 10 → 8 storage buffers (the WebGPU
   default); `gpu.ts` requirement recounted, ceiling comments corrected. Pure
   re-addressing — RMSE below.
3. A `counters` atomic-u32 buffer (18 slots: rays by bounce depth, shadow rays
   by purpose, BVH node visits, box tests, RIS candidates, volume steps,
   radiosity gathers, flashmap rays) gated on a `countersOn` uniform. Shared
   intersection code tallies into per-invocation private slots; the pass flushes
   one atomic per slot, so no other pass has to bind the buffer. Slot names live
   once (`counters.ts`); the WGSL constants are generated from them. Readback
   mirrors `GpuProfiler` → `__stats.counters`; volatile panel toggle; `__bench`
   gains a `counters` block only while on. Trace pass now needs 9 storage
   buffers (gpu.ts).

## Verification

`npm run typecheck` and `npm run build` clean at every commit.

**Reservoir merge is estimator-neutral.** `__compareToReference` at 320×200,
140 reference / 40 test frames, bounces 1, same protocol on a scratch build of
`82baea3` (base plus only the bench hook) vs the merge commit:

| config | base relRmse | merged relRmse | base relBias | merged relBias |
|---|---|---|---|---|
| default | 1.2722 | 1.2458 | -0.1881 | -0.1878 |
| restirGI off | 1.2721 | 1.2457 | -0.1877 | -0.1874 |
| spatial taps off | 1.9872 | 1.9208 | -0.4586 | -0.4537 |

Deltas are inside the run-to-run noise of a 40-frame 1-spp test (relRmse
≈2%, relBias ≤0.005); a parity error would surface as feedback or dilution well
outside that. The absolute relRmse is high only because this is a 320×200 /
40-frame test, not the standard 1152×720 / 120.

**Counters at the pinned bench pose** (`__bench(3, true)` at
`__benchResolution(384, 240)`, defaults: 1 bounce, 1 spp, radiosity on,
8 candidates, 6 spatial taps, 12 volume steps; 92,160 px):

| slot | frame total | per pixel |
|---|---|---|
| raysDepth0 (primary) | 92,160 | 1.0000 |
| raysDepth1 | 213 | 0.0023 |
| shadowDirect | 78,777 | 0.855 |
| shadowIndirect (RIS winner, bounce ≥1) | 183 | 0.002 |
| shadowGI (reused-sample revisibility) | 89 | 0.001 |
| shadowTransient / shadowVolumetric | 0 / 0 | 0 |
| bvhNodeVisits | 2,857,514 | 31.0 |
| boxTests | 391,855 | 4.25 |
| risCandidates | 640,576 | 6.95 |
| volumeSteps | 1,105,920 | 12.000 |
| radiosityGathers | 310,804 | 3.37 |
| flashmapRays | 81,920 | 0.889 |

Reconciliation, all against the code and each within reason:
- primary rays = pixels exactly (92,160); every invocation traces one.
- risCandidates/8 = 80,072 = pixels with a valid primary hit (~13% of this
  wide view is sky through windows); radiosityGathers/4 = 77,701 static
  radiosity-hit pixels; shadowDirect (78,777) is the subset whose reservoir
  survivor has W>0 — one shadow ray each, never more.
- bounce-1 rays are only 0.002/px because with radiosity on, static hits take
  indirect from the patch solve and never trace the bounce; only character
  (dynamic) hits do. Confirmed by construction and by ablation at the spawn
  view: radiosity off → 0.969 bounce-1 rays/px, 0.71 indirect + 0.82 GI shadow
  rays/px, 15.7 RIS candidates/px (8 primary + 8 per bounce); bounces=2 adds
  0.169 depth-2 rays/px after Russian roulette.
- volumeSteps = exactly 12/px; flashmapRays = 5 live torch layers × 128²;
  transient/volumetric shadow rays 0 (no flash live; the depth-map path
  replaces the march's rays, as designed).
- One count contradicted the docs: the README claimed 3 shadow rays at
  bounce 0. The code spends 1 (a single unified ReSTIR survivor — the moon
  and flashlight are inside the reservoir; `sampleKeyLight`/`sampleFlashlight`
  are dead). The README's Lighting paragraph and ablation table were the ones
  wrong; corrected in `5503118`.

**Counters on vs off, same view** (384×240, 6 stills at the spawn camera; both
PNGs read): identical images — the red-carpeted meeting room with beige table,
blue chairs and green plant upper-left, grey partitions, the blue cubicle
block with desks/monitors right, two warm ceiling-light pools over cardboard
boxes lower-right, the player and one dark figure mid-corridor, same HUD
(ammo 11, light meter HIDDEN, brightness 0.350). No brightness or noise delta.

**`__bench(2, true)`** at `__benchResolution(192, 120, 900)`: returns
(`frames: 2, truncated: false`); JSON gains a `counters` block only with
`settings.counters` on (keys diffed both ways); `__stats.counters` reads
`null` again once counters go off.

## Findings (not fixed)

- README staleness beyond the shadow-ray claim: the "Next steps" list still
  proposes ReSTIR DI and per-object motion vectors, both already shipped.
  Phase D's README pass owns it.
- `sampleFlashlight`, `sampleKeyLight`, `sampleSceneLight` in common.wgsl are
  dead since the ReSTIR unification; they carry uncounted shadow rays if ever
  revived.
- A truncated `__compareToReference` (deadline hit during the reference) runs
  every test config for zero frames and reports the black-image error
  (relBias −1) with only `truncated: true` to say so — easy to misread.
- Two harness runs cannot be bit-identical: guard patrol and rig phase advance
  on wall-clock before any pin/freeze, so "same run twice" always differs
  in the dynamic geometry it froze. Fine for RMSE-within-noise; useless for
  bit-exact A/B.

## Watch when merging

- Storage-buffer count in the trace pass is 9, one over WebGPU's default 8:
  the merge freed two slots and the counters buffer took one back. Devices
  that report exactly 8 (some mobile) now fail init at the `gpu.ts` gate; they
  failed at 10 before, so this is a reach improvement, not the full one. A
  counters-optional pipeline layout would reach 8 — deliberately not done here.
- Merged reservoir buffers double the per-binding size (GI: 128 B/px). Under
  the 128 MB default `maxStorageBufferBindingSize` the GI buffer now stops
  fitting at ~1.0 Mpx internal (was ~2.0 Mpx); we request 512 MB where the
  adapter has it, so only default-limit devices are affected.
- Counters OFF must cost only not-taken branches, but every scene shader now
  also declares an 18-slot private tally array; a compiler that fails to drop
  the never-written array would spend registers. Confirm on the M1 Max with
  `__bench(60)` counters-off vs `82baea3` — that number cannot come from here.
- Slot list is authoritative in `src/engine/counters.ts`; adding a slot needs
  no WGSL edit for the constant, only the increment site.
- `WorkCounters` deliberately mirrors `GpuProfiler`'s readback ring; a shared
  mapped-read ring is the obvious cleanup once a third consumer appears.
