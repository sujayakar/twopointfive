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
| B | detection + temporal (in flight); radiosity-hybrid + volumetrics (launching); fluid after volumetrics | in flight |
| C | Perf: half-res indirect, ray compaction, ReSTIR GI spatial | queued |
| D | Convergence: merged trunk, review pass, README + bench recipe for the Mac | queued |

## Open questions for Sujay

(none yet — items land here rather than in chat unless blocking)

## Log

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
