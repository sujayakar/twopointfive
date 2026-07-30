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
3. A `counters` atomic-u32 buffer (22 slots: rays by bounce depth, primary
   hits, shadow rays by purpose incl. probe, BVH node visits, OBB and slab
   tests, RIS candidates split direct/indirect, volume steps, radiosity
   gathers, flashmap rays) gated on a `countersOn` uniform. Shared
   intersection code tallies into per-invocation private slots; the trace pass
   flushes one atomic per slot, and the flashmap and probe passes each
   atomicAdd only their own ray count (their traversal is not folded into the
   per-image-pixel slots). Slot names live once (`counters.ts`); the WGSL
   constants are generated from them. Readback mirrors `GpuProfiler` →
   `__stats.counters`; every block carries the renderer frame index it belongs
   to; volatile panel toggle; `__bench` gains a `counters` block only while
   on. Trace pass now needs 9 storage buffers (gpu.ts); probe pass 8.

## Verification

`npm run typecheck` and `npm run build` clean at every commit.

**Reservoir merge is estimator-neutral.** The load-bearing evidence is the
addressing trace: all seven reservoir access sites keep the identical
`y*dims.x + x` expression differing only by `+resBase(dims, parity)`; frame
N writes half p and frame N+1 reads half p as prev — exactly what buffer index
p was pre-merge; reads and writes never share a location within a dispatch. A
review reproduced this trace independently and compared stills PNG-to-PNG:
identical.

`__compareToReference` at 320×200, 140 reference / 40 test frames, bounces 1,
same protocol on a scratch build of `82baea3` (base plus only the bench hook)
vs this branch:

| config | base relRmse | head relRmse | base relBias | head relBias |
|---|---|---|---|---|
| default (initial) | 1.2722 | 1.2458 | -0.1881 | -0.1878 |
| restirGI off (initial) | 1.2721 | 1.2457 | -0.1877 | -0.1874 |
| spatial taps off (initial) | 1.9872 | 1.9208 | -0.4586 | -0.4537 |
| default (post-review, run 1) | 1.2705 | 1.2716 | -0.1889 | -0.1879 |
| spatial taps off (post-review, run 1) | 1.9811 | 2.0380 | -0.4586 | -0.4608 |
| default (post-review, run 2) | — | 1.1911 | — | -0.1565 |
| spatial taps off (post-review, run 2) | — | 1.7490 | — | -0.4232 |

The measured same-build noise floor (a reviewer's 4× reruns per build, this
protocol) is ±6 % relRmse and 0.05 relBias, and it is bimodal: the guard
patrol/dynamic-geometry phase advances on wall-clock before `pinBenchPose`,
so each page load freezes a slightly different scene (refMean 0.0286–0.0290;
head runs 1 and 2 above are the two modes). Both builds land in both modes;
every base/head delta is inside that band, so the table is consistent with —
but cannot by itself prove — the invariance. The absolute relRmse is high
only because this is a 320×200 / 40-frame test, not the standard 1152×720 /
120.

**Counters at the pinned bench pose** (`__bench(3, true)` at
`__benchResolution(384, 240)`, defaults: 1 bounce, 1 spp, radiosity on,
8 candidates, 6 spatial taps, 12 volume steps; 92,160 px; block frame 22 =
the run's last measured frame):

| slot | frame total | per pixel |
|---|---|---|
| raysDepth0 (primary) | 92,160 | 1.0000 |
| raysDepth1 | 207 | 0.0022 |
| primaryHits | 79,853 | 0.8665 |
| shadowDirect | 78,761 | 0.8546 |
| shadowIndirect (RIS winner, bounce ≥1) | 171 | 0.0019 |
| shadowGI (reused-sample revisibility) | 85 | 0.0009 |
| shadowProbe | 31 | 0.0003 |
| shadowTransient / shadowVolumetric | 0 / 0 | 0 |
| bvhNodeVisits | 2,855,706 | 30.99 |
| obbTests (leaf ray-vs-OBB) | 391,979 | 4.25 |
| slabTests (BVH child + group cull) | 5,673,637 | 61.56 |
| risCandidatesDirect | 638,824 | 6.93 |
| risCandidatesIndirect | 1,648 | 0.018 |
| volumeSteps | 1,105,920 | 12.000 |
| radiosityGathers | 310,748 | 3.37 |
| flashmapRays | 81,920 | 0.889 |

Reconciliation, counter against counter, all exact or within one ray:
- raysDepth0 = pixels (92,160): every invocation traces one primary.
- risCandidatesDirect / 8 = 79,853 = primaryHits exactly: `restirDirect` runs
  once per valid primary hit with 8 candidates (~13 % of this wide view is
  sky through windows).
- shadowDirect (78,761) ≤ primaryHits (79,853): at most one shadow ray per
  hit, spent only when the survivor has W > 0 (98.6 % of hits here).
- risCandidatesIndirect / 8 = 206 vs raysDepth1 = 207: `sampleIndirectRIS`
  runs once per valid bounce-1 hit; one bounce ray escaped to sky.
- bounce-1 rays are only 0.002/px because with radiosity on, static hits take
  indirect from the patch solve and never trace the bounce; only character
  (dynamic) hits do. Confirmed by construction and by ablation at the spawn
  view: radiosity off → 0.969 bounce-1 rays/px, 0.71 indirect + 0.82 GI shadow
  rays/px, 15.7 RIS candidates/px (8 primary + 8 per bounce); bounces=2 adds
  0.169 depth-2 rays/px after Russian roulette.
- volumeSteps = exactly 12/px; flashmapRays = 5 live torch layers × 128²;
  shadowProbe = 31 (probes × lights that survive the spot cone); transient /
  volumetric shadow rays 0 (no flash live; the depth-map path replaces the
  march's rays, as designed).
- One count contradicted the docs: the README claimed 3 shadow rays at
  bounce 0. The code spends at most 1 (a single unified ReSTIR survivor — the
  moon and flashlight are inside the reservoir; `sampleKeyLight` /
  `sampleFlashlight` are dead). The README's Lighting paragraph and ablation
  table were the ones wrong; corrected, and the README now states the
  invariant rather than a pose-specific ratio.

**Counters on vs off, same view** (384×240, 6 stills at the spawn camera; both
PNGs read again at this commit): same scene, same look — the red-carpeted
meeting room with beige table, blue chairs and green plant, grey partitions,
the blue cubicle block with desks right, warm ceiling-light pools over
cardboard boxes, the player and a guard in the corridor, same HUD (weapons
1 hands / 2 five-seven / 3 OCP, ammo 11, light meter HIDDEN, brightness
0.350). The two page loads differ only in the frozen character/guard pose
(wall-clock phase before the still), not in brightness or noise. Console
clean in both runs; the counters run's `__stats.counters` populated, the off
run's read `null`, and after toggling off mid-session it read `null` and
stayed `null`.

**`__bench` counters block, both modes** at `__benchResolution(192, 120)`:
serial `__bench(2, true)` returns `frames: 2, truncated: false`, its
`counters.frame` is 21 (the last of measured frames 20–21) and
`raysDepth0 = pixels = 23,040`; pipelined `__bench(4, false)` returns
`frames: 4` with `counters.frame: 24` — an end-of-run frame rather than an
early one (measured frames were 20–23). The JSON gains a `counters` block only
with `settings.counters` on (keys diffed both ways); counters off leaves the
blob byte-for-byte the pre-track shape plus `nonStandardRes`.

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
  the merge freed two slots and the counters buffer took one back (probe pass:
  8, radiosity: 8). Devices that report exactly 8 (some mobile) now fail init
  at the `gpu.ts` gate; they failed at 10 before, so this is a reach
  improvement, not the full one. A counters-optional pipeline layout would
  reach 8 and would also make counters-off truly free of the always-bound
  buffer — deliberately not done here.
- Merged reservoir buffers double the per-binding size (GI: 128 B/px). Under
  the 128 MB default `maxStorageBufferBindingSize` the GI buffer now stops
  fitting at ~1.0 Mpx internal (was ~2.0 Mpx); we request 512 MB where the
  adapter has it, so only default-limit devices are affected. `resize()` has
  no clamp against `device.limits`, so the failure surfaces as a bind-group
  validation error on resize, not at init — a pre-existing shape this track
  did not add and did not fix.
- The "Two harness runs cannot be bit-identical" finding is the RMSE table's
  noise floor: same-build reruns of `__compareToReference` spread ±6 %
  relRmse. Any future A/B on this rig needs ≥3 runs per side or a
  deterministic freeze (pin the guards) before it can call a small delta.
- Counters OFF must cost only not-taken branches, but every scene shader now
  also declares a 22-slot private tally array; a compiler that fails to drop
  the never-written array would spend registers. Confirm on the M1 Max with
  `__bench(60)` counters-off vs `82baea3` — that number cannot come from here.
- Slot list is authoritative in `src/engine/counters.ts`; adding a slot needs
  no WGSL edit for the constant, only the increment site.
- `WorkCounters` deliberately mirrors `GpuProfiler`'s readback ring; a shared
  mapped-read ring is the obvious cleanup once a third consumer appears.

## Review resolution

Three adversarial reviewers examined the branch; every finding was re-verified
against the code before deciding. Fixes are in the commit that adds this section.

| # | finding (reviewer) | verdict | resolution |
|---|---|---|---|
| 1 | `risCandidates/8 = valid-hit pixels` reconciliation is false — the slot mixes primary and indirect RIS, and there is no independent hit count (R1.1) | **real** | Slot split into `risCandidatesDirect` / `risCandidatesIndirect`, counted at the two call sites instead of inside `proposeCandidate`; new `primaryHits` slot counted once per valid primary hit. The reconciliations below are now counter-vs-counter cross-checks and hold to the unit. |
| 2 | Pipelined `__bench(n>3, false)` counters block is an early frame, not the last; blob carries no frame index (R1.2, R2.2) | **real** | `CounterFrame` now carries `frame` (renderer frame index). Pipelined bench takes one event-loop turn after the timed loop and renders one untimed frame before flushing, so the block is end-of-run (verified: `frames: 4` measured over indices 20–23, `counters.frame: 24`; serial `__bench(2, true)` reports frame 21 = its last). Comments in `benchInner` and `resolve()` corrected. |
| 3 | README "0.99 direct shadow rays per pixel" is a pose-specific number, not the invariant (R1.3) | **real** | README now states the invariant — at most one direct shadow ray per pixel with a valid primary hit — with the observed 0.85–0.99 range and why it falls short of 1. |
| 4 | Probe-pass shadow rays uncounted and undocumented, falsifying "totals are complete here" (R1.4, ray half of R2.1) | **real** | New `shadowProbe` slot; the probe layout gains a counters binding (probe stage = 8 SSBOs, under the trace pass's 9) and flushes only its ray count, like the flashmap. `resolve()` moved after the probe pass; the completeness comment now names each ray-tracing pass. |
| 5 | Per-frame radiosity passes trace through `common.wgsl` and drop their counts; radiosity looks free (R2.1, traversal half) | **false positive** | `radiosity.wgsl`'s three `occluded()` sites are in the bake kernels (`bakeFF`/`bakeVis`/`bakeSky`), which run once at init. Per-frame `inject` reads baked tables + torch depth maps and `solve` is `patchCount²` MADs — neither traces a ray. Stated in `counters.ts` and the renderer comment; the solve's cost is a scene constant, so it gets no counter (see #7). |
| 6 | `boxTests` counts only leaf OBB tests; BVH-child and dynamic-group slab tests are counted nowhere (R1.5) | **real** | Renamed to `obbTests`; new `slabTests` counted inside `slabAABB` (every BVH-child and group-cull test). A slab-vs-leaf traversal trade now shows on both sides. |
| 7 | `radiosityGathers` names the trace pass's patch-texture reads, not the solve's gather; slot name invites misreading (R1.7) | **real (doc gap)** | Docstring in `counters.ts` now says exactly that, and that the per-frame solve is `patchCount²` MADs with no rays and no counter. No rename — the brief's own name for the slot. |
| 8 | `bounces ≥ 7` (settable via `__settings`, only the slider clamps at 6) folds silently into `raysDepth6` (R1.6) | **real** | Slot renamed `raysDepth6plus`; docstring says depth ≥ 6 lands there. The `min(b, 6u)` fold is now what the name promises. |
| 9 | An in-flight readback re-sets `latest` after `begin()` nulled it, so `__stats.counters` can read non-null for a frame after counters go off (R1.8) | **real** | `WorkCounters` tracks `on`; the mapAsync callback assigns `latest` only while on. |
| 10 | Counters-off does not "cost nothing else": the buffer stays bound (9 SSBOs even when off) and every shader declares the private tally array (R2.3) | **real (overstated comment)** | Comment in `common.wgsl` now says off = untaken branches plus a never-written private array the compiler is expected to drop, and that the buffer stays bound. The 9-slot init gate and the register question are unchanged and stay in Watch — the register cost is only settled by a real-GPU counters-off A/B, and a counters-optional layout is deliberately not done here. |
| 11 | Merged GI buffer halves the resolution ceiling under the 128 MiB default `maxStorageBufferBindingSize`, and `resize()` has no clamp (R2.4) | **real, not fixed here** | Pre-existing shape (the unclamped limit existed at 2.1 Mpx before the merge; the merge moves it to 1.05 Mpx). Already the second Watch item; an init-time clamp against `device.limits` is renderer work outside this track. |
| 12 | RMSE table is one run per build; the "≈2% relRmse noise" claim is unmeasured and ~5× low (R3.1, ran 4×/build) | **real (evidence quality)** | Verification section now quotes the reviewer's measured same-build spread (±6 % relRmse, 0.05 relBias, bimodal on the frozen dynamic-geometry phase) as the noise floor, adds this session's HEAD/base reruns, and says outright that the load-bearing evidence for estimator invariance is the 1:1 addressing trace, not the table. |
| 13 | Temporal-reuse `pp = vec2i(uv * dims)` is not bounds-checked; RNE can round the product up to `dims`, and post-merge that indexes the half being written (R3.2) | **real (measure zero)** | `pp` clamped to `dims - 1` in both `restirDirect` and `restirGI`, same guard as the spatial-tap `qp`. Unreachable at any resolution actually run; closes the read-of-live-half race outright. |

Verified again after the fixes (this commit): `npm run typecheck` and
`npm run build` clean; headless counters scenario at 384×240 clean
(`ok: true`, no console errors), counters-off returns `null`, `__bench` JSON
gains a `counters` block only while counters are on; both PNGs read and
described above; RMSE reruns in the Verification table. Numbers in the
counter table and reconciliation above were regenerated at this commit.
