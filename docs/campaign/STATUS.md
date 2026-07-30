# Campaign status — "make it an amazing tech demo"

Coordinator: Claude (session started 2026-07-30). Sujay reviews at milestones
and runs real-GPU benches on his M1 Max. This file is the top of the
context stack: read it first, then the brief for whatever track you own.

## Ground truth on tooling

- **This box has no GPU.** `tools/headless/run.py` runs the demo on
  SwiftShader inside headless Chromium: full functional loop (WGSL compile,
  validation, real frames, screenshots, per-pass profiler), zero timing
  meaning. Read `tools/headless/README.md` before touching it.
- **Machine-independent measurement = work counters** (rays, shadow rays,
  BVH node visits, RIS candidates per pixel). Landing in Phase A. Every perf
  claim in this campaign is expressed in counters first, ms second.
- **Milliseconds come from Sujay's Mac** via `__bench(60)` at the pinned
  pose. Batch questions for him; do not block on them.
- **Correctness gate for tracer changes:** `__compareToReference` (renders a
  ground-truth reference and diffs configs against it), plus screenshot
  self-check. A change to the estimator ships with its RMSE numbers.

## Branch discipline

- Trunk of this campaign: `claude/campaign` (base `87f25df` + the storage
  texture fix + harness). Sujay's `main` is untouched until he decides.
- Each track: worktree at `~/src/twopointfive-wt/<track>` on branch
  `claude/<track>`, forked from the campaign trunk at a pinned SHA.
  `node_modules` is a symlink into the main clone — do not `npm install`
  inside a worktree.
- Tracks commit small and often to their own branch. The coordinator merges
  into `claude/campaign` serially and owns every conflict resolution.
- No force-push, no rebase of a branch someone else builds on, never
  `git stash` (worktrees share the stash).

## Phases

| Phase | What | State |
|---|---|---|
| 0 | Foundation: storage-texture fit + headless harness | done, `82baea3` |
| A | Work counters + reservoir merge + bench-res hook | done, merged (`c0a72a9`) |
| B | detection ✔ radiosity ✔ volumetrics ✔ merged; temporal pending audit; fluid launching | in flight |
| C | Perf: half-res indirect, ray compaction, ReSTIR GI spatial | queued |
| D | Convergence: merged trunk, review pass, README + bench recipe for the Mac | queued |

## Open questions for Sujay

(none yet — items land here rather than in chat unless blocking)

## Log

- 07-30 Track B1 (radiosity hybrid) merged: crouch fix (flashmap self-group skip), indirectMode {traced, radiosityRead, gather, patchRIS}, and a shared-code estimator fix (Z-MIS merge denominators — spatial reuse was inflating the default flashlight pool 2.3× and GI sparse-bright ~4×).
- 07-30 Track B2a (volumetrics) merged: slab-clipped march, 4th illum layer = RGB volumetric channel, baked static light volume (moon god-rays), extinction, density contract (@binding 14/15, uniforms 880-943), coarse density CPU readback.

- 07-30 Track B3 (detection AI) merged after 17-item review round; central fix was gating suspicion on LOS × light, not LOS alone. 10 asserting headless scenarios under tools/headless/scenarios/.
- 07-30 Track B4 (temporal) done pending final audit numbers: character was shedding history every frame even at idle (0.90 normal test vs animated micro-motion); fixed via depth-identity validation for dynamic taps.

- 07-30 clone; lockfile regenerated from the upstream registry (rollup 4.62.2, one
  patch behind the original 4.62.3 lock).
- 07-30 SwiftShader caps storage textures at 4 → merged the trace pass's three
  radiance targets into one 2d-array binding. Demo now inits at WebGPU's
  default limits — a real reach improvement, not just a harness enabler.
- 07-30 Headless loop verified: scene + BVH + 3413-patch radiosity bake +
  rig, full frame including volumetrics, HUD screenshot. pathtrace ≈1.9 s /
  1152×720 frame under SwiftShader; profiler timestamps live.
- 07-30 Firefox+lavapipe explored and abandoned (release channel force-
  blocks WebGPU on Linux); Chrome+lavapipe abandoned (headless pins the
  bundled SwiftShader). Recorded so nobody re-treads it.

## Coordination locks (parallel tracks)

- Uniform tail: bytes 864–879 = radiosity track (`indirectMode`), 880–943
  = volumetrics track (grid transform). UNIFORM_SIZE = max of the two on
  merge; the coordinator reconciles.
- Trace-pass @group(1) bindings: 14/15 = volumetrics (smoke volume +
  sampler), 16/17 = radiosity (patch data). Storage buffers per stage after
  radiosity binds radStatic = 10 = ceiling; nothing else adds a trace-pass
  storage buffer without a coordinator conversation.
- Baseline counters (trunk, pinned pose, 320×200, defaults): 1.00 primary,
  0.87 primary hits, 0.86 direct shadow rays, 6.94 direct RIS candidates,
  31.1 BVH node visits, 61.8 slab tests, 4.49 OBB tests, 12.0 volume steps,
  3.37 radiosity gathers, 1.28 flashmap rays — per pixel.

## Watch (from Track A)

- Counters-OFF register cost unverified on silicon (private tally array in
  every scene shader) — Mac `__bench(60)` counters-off vs pre-A build.
- Trace pass at 9 storage buffers (default is 8): counters-optional
  pipeline variant would restore default-8 reach.
- GI reservoir binding doubles per-px size: ~1 Mpx internal ceiling on
  128 MB-binding devices; no resize clamp yet.
- `__compareToReference` truncation reports black-image error with only
  `truncated: true` as the tell.

## Recovery card (read this first in a fresh session)

Repo `~/src/twopointfive`, trunk branch `claude/campaign`; worktrees under
`~/src/twopointfive-wt/{name}` on branches `claude/{name}`. All state is in
git — nothing important lives only in a conversation.

- Merged to trunk: foundation harness (00e5b3f), Track A instrument
  (c0a72a9), B3 detection (be07685), B4 temporal (b717285), B1 radiosity
  impl (f71d8b9), B2a volumetrics impl (b99fbbd); STATUS 2f6326b.
- Also merged: `claude/radiosity` review-fix 371c41f (ReSTIR spatial-
  reuse support-aware merge denominator — the default flashlight pool was
  ~2.3x hot before — plus patch-mode energy verification) via e644cc1, and
  `claude/volumetrics` review-fix 75f9856 (contract wording, reference-mode
  integrand parity, readback recovery) via bea6a41. Trunk builds clean.
- Track B2b smoke fluid: `claude/fluid` @ 7e0d45e. Solver, sources and the
  throwable smoke canister are committed and build clean. UNFINISHED: no
  track report; determinism scenario not deterministic (must pause+reset,
  fixed injected source); divergence-residual claim needs an active-cell
  metric (room-mean is dominated by still air; Jacobi 20 vs 120 iterations
  barely differs — a boundary/collocated-stencil floor); mass leaks ~2x the
  modeled dissipation with sources off (25.0 -> 9.9 over 3.0 s); vol-shot
  scenario modes call a removed injection hook; readback mapAsync lacks a
  rejection path. Sub-agents on this track have died repeatedly on the
  usage-policy classifier — brief any further help on this track around
  pure fluid dynamics and volume rendering, and hand it only fluid.ts /
  fluid.wgsl / the renderer's fluid passes / the harness.
- Mac to-do for the human (Apple M1 Max, real GPU): `__bench(60)` at
  defaults with counters off vs a pre-Track-A build (register cost of the
  counters tally array); `__bench(60)` per indirect mode (gather /
  patchRIS / traced) and with smoke fog on; `__compareToReference` at the
  standard 1152x720 protocol. Push branches to origin (agent push has no
  auth here).
- Phase C candidates after fluid: half-res indirect + upsample; ray-order
  compaction between bounces; ReSTIR GI temporal on; counters-optional
  pipeline (reach: default 8 storage buffers); solve inject +17-47% hot
  (patchRIS off default until fixed); GI buffer resize clamp. Phase D:
  full-scene review, README rewrite, layered HTML report.
