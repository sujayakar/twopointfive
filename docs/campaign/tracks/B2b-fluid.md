# Track B2b — GPU smoke solver: the mass leak, found, fixed, and bounded

Branch `claude/fluid`. The solver, sources, canister and occupancy were received
for review at **`c78e794`**. The "before" columns in §2/§3 were measured on
**`9940888`**, which is not that commit: it is a branch WIP that already carried
solid-aware advection (`sampleScalarsFluid`), the `grenade → canister` rename and
five scenarios — `git diff --stat c78e794 9940888 -- src/ tools/` is 484
insertions over 20 files. An earlier revision of this report called `9940888`
"as received for review", which was wrong; the pre-fix baseline is a solver that
already had the solid-aware gather in it, and the mass figures attributed to
"before" are that baseline's.

This track was handed a solver that ran inside its limits, never produced a NaN,
and was deterministic — and that lost 57% of a sinking cloud's mass in 8 seconds.
The hypothesis on arrival was residual divergence from an under-converged
pressure solve, untested because a scenario's Jacobi override never reached the
dispatch loop. The override is fixed, the hypothesis is **refuted by
measurement**, and the cause was two mismatched discretisations either side of
the pressure solve. Fixing them takes retention over 8 s from 0.43 to 0.72, and
from 0.27 to 0.93 without vorticity confinement.

What remains is not a bug and is now characterised rather than apologised for:
the per-step mass defect of a semi-Lagrangian **gather** is first order in dt
(measured 1.02 and 1.04 over 50 → 25 → 12.5 ms), so the total over a fixed span
of game time is zeroth order — 0.7176, 0.7082, 0.7044 retained at those three
steps. Refining the step buys nothing, and neither does iterating the projection
harder: in one run at one instant the defect implies an effective divergence of
0.058/s while that run's post-projection active-cell mean |∇·v| is 0.00021/s —
280× smaller. Only flux-form advection changes that number. A global
mass renormalisation was evaluated against this data and rejected, with its
arithmetic in §4.

**Status.** Branch HEAD builds clean (`npm run typecheck && npm run build`) and
carries the resolutions to a read-only review of the branch — 17 findings, all
real, 16 resolved and 1 knowingly left in place (see Review resolutions, and
Below-100% item 20 for the one). The closing pass before that review re-checked
three things on the shipping build under a strict compute budget — cold start, the
still-air plumbing check, and the determinism checksum — and all three reproduced
their recorded values exactly (§0); every other figure in this report is a
prior-session measurement, labelled as such.
**Nothing here measures performance or how the smoke looks in motion**: this box
has no GPU, so both are deferred to Validate on the Mac, which lists the five
things to look at and the four `__bench` comparisons that produce the perf number.
The honest list of what is still below 100% closes the report.

## Review resolutions

A read-only review of this branch at `36a63bb` raised 17 numbered findings plus a
provenance group. Every one was read against the code. **None was a false
positive** — but they split three ways, and the split is the useful part: 8 were
defects in the *code*, 8 were claims in this report that outran their evidence,
and 1 is a real numerical deviation deliberately left in place because fixing it
would invalidate measurements this pass has no compute budget to re-derive.

Fixed code changes are arithmetically inert on every recorded run, which is what
lets the tables below stand unchanged. Where that inertness needed proof it was
measured, not asserted, twice over: statically, by enumerating all 532 static boxes
of the level and comparing the old and new occupancy skip predicates on each (0
kept-set disagreements, CPU only); and on the shipping build, by
`fluid-invariants.js` reproducing §13's solid-cell total, its full per-row table
and its resource counts exactly (§0, "Build D re-check"). That bundle also asserts
the two instrument fixes directly, so findings 6, 9 and 11 are verified rather than
argued.

One of those runs cost more than it should have: the first attempt failed on an
assert of *mine* that was wrong about `__freezeClock`, which turned into
Findings 15. Two headless runs were spent in total.

Two defects in the resolutions themselves were caught before commit and are
recorded here rather than quietly fixed: the `__freezeClock` assert above, and an
incomplete gate in the finding-6 fix — it covered dt = 0 and `silenced` but not
`settings.fluidSim`, which makes the renderer skip the step just as surely
(row 6). Neither is verified by a run in its final form beyond typecheck, build
and the argument that both are inert at the shipped defaults.

| # | finding | verdict | resolution |
|---|---|---|---|
| 1 | Branch declines the density contract's `y >= 3.2` top-wall rule; `renderer.ts` said it complied | **real (code + claim)** | The behaviour is kept and the claim is gone. Row 12 is 80% real air; solidifying it would delete the layer ceiling-hugging smoke lives in (its top sub-lattice y-plane at 3.208 alone gives 9 hits against `SOLID_MIN_HITS = 6`, so the whole row would go solid). `renderer.ts`, `fluid.wgsl` and `solidRow`'s docstring now name it as a **deviation**; new Below-100% item 17. Root-cause fix alongside: `bakeOccupancy` skips the slab because it *caps the lattice* (bounds confined to the top row and continuing above it), not because it carries `FLAG_NO_CAMERA` — that flag means "invisible to camera rays", so an invisible-but-solid obstacle inside the room was being dropped from the fluid. Proven inert on the level's 532 boxes. |
| 2 | Merges with `claude/campaign` with a conflict in `renderer.ts` | **real** | Measured and documented in Merge notes with the resolution (take this branch's text) and the two facts the review checked: `UNIFORM_SIZE = 944` and the trace layout's bindings 14/15 are intact. The 256-byte dead-puff hole is no longer held by prose alone — the comment names `_deadPuffPosR`/`_deadPuffParams` so a grep finds both sides. `profiler.ts`'s `MAX_PASSES` 32 → 48 added to the notes. |
| 3 | `setScale(2)` runs 2× the confinement strength; Validate comparison 3 benches an empty room | **real (code + doc)** | `fluid.wgsl` now scales confinement by `min(cell)` rather than `cell.x`. `cell.y` never changes and x/z only grow, so this is bit-identical at `scale = 1` and scale-invariant at 2. The bench script is fixed to re-inject after each `setScale` (which reallocates every field zeroed and resets `steps`) and the "resampling blur, not resolution loss" line now holds. |
| 4 | Determinism checksum covers one lane; §10 called it a "byte-identical field" | **real (code + claim)** | `densityStats` gains `fieldChecksum` over all four scalar lanes, so **temperature** is covered — the lane whose absence let the first "warm" still come back identical to the cold one. `checksum` keeps its density-lane definition so §10's six recorded invocations stay comparable. Both docstrings and §10/§0 now say what each hash does and does not see (neither sees velocity or pressure). New Below-100% item 18. |
| 5 | `stillAir` cannot fail, so it is not a conservation control | **real (claim)** | Correct: with every force coefficient zero and a blob carrying no temperature, velocity is identically zero, the gather reduces to weight 1 on the cell itself, and `dens * exp(0)` is exact. §0/§3/§14 and `fluid-mass.js`'s own header now call it an arithmetic identity — worth running as a plumbing check, not evidence that fp16 storage or solid-cell zeroing is lossless (the blob is deliberately in open air, so no wall is touched). New Below-100% item 19 names what a real storage control would need. |
| 6 | Instant sources are consumed without injecting when silenced or at dt = 0 | **real (code)** | Fixed in `smoke.ts`: an undelivered instant source waits instead of ageing, and is only spent on a frame that will actually inject it. **All three** gates are checked, not the two the review named — `settings.fluidSim` off makes the renderer skip the step just as surely as dt = 0 does, and the first fix missed it, so `Smoke.update` now takes `simulating` from `main.ts` (default `true`, so no other caller changes and every recorded run, which runs `fluidSim: true`, is unaffected). Verified on build D in both testable arms — held 0, then 17.34404917061329 delivered on release (§0). One correction to the review's mechanism: under `__freezeClock(true)` the *first* frame still takes a full 50 ms step, so the dt = 0 hazard starts at the second frozen frame, not the first (Findings 15). |
| 7 | §11's "telescopes by construction" is false as stated | **real (claim)** | The telescoping is exact for the idealised differences, but each cell's RHS is stored as a rounded r32float sum, so the shared face value does not cancel bit-for-bit and compatibility holds only to rounding. §11 now says that, and says plainly that the 1200-step measurement is the *only* evidence — so a coarser pressure format, a finer grid or a longer run is a reason to re-run the bundle, not just inflow/outflow. |
| 8 | The row-0 fix is credited to the wrong change | **real (claim)** | Correct: row 0's own minus-y flux is *still* zeroed by `wallFaces`; what unblocks it is the MAC face→centre reconstruction in `velCentre`, which averages in the face between rows 0 and 1. Moving a clamp earlier while keeping collocated velocities would leave `rowMass[0] ≡ 0`. Fixed in What-changed item 3, Findings 3 and the Merge-notes invariant. |
| 9 | `resources()` returns hand-written constants while calling them counted | **real (code)** | The bind-group layout's entry descriptors are kept and tallied by kind; `storageTexturesPerStage` is derived from the same entries. Adding a fourth storage texture now moves the number. Same values today, so §13 is unchanged. §13 also now says the SwiftShader limit of 4 is sourced from `tools/headless/README.md`, not from a run — nothing in the tree reads `device.limits`. |
| 10 | The warm still's source is not smoulder's, so the Mac guidance from it is unsupported | **real (claim)** | It is 10× the density, 2.9× the emitting volume and 1.5× the temperature of `Smoke.smolder` — a different buoyancy regime, which is the one thing the shot exists to show. §6, Validate item 1 and Below-100% item 12 now state all three ratios, and note that the shot's `attack: 1.0` is dead (the envelope only applies to a finite `life`), so the plume starts at full strength. |
| 11 | `divergenceStats()` corrupts its own `pre` figures if called twice | **real (code)** | The recomputed post-projection field goes to the spare pressure texture (`prs[1]`, free between steps — the next step's first Jacobi iteration overwrites it), so `div` keeps the solve's RHS and a second call at one checkpoint reports the same `pre`. |
| 12 | Unguarded divide by the source radius | **real (code)** | `max(s.radius, 1e-4)` in `forces`. `__smokePuff` is exposed, so a zero radius was reachable from the console; the outcome was between silently zeroing a cell and a persistent NaN. Bit-identical for every radius the game and the suite use. |
| 13 | `lastJacobi` is the rounded count; `fluid-jacobi.js` asserted the raw request | **real (code)** | The scenario now asserts against the rounded count, matching `fluid-mass.js`. The `jacobi` docstring's "rounded up to even" was also false (4.4 → 4, 5 → 4) and now says nearest-even with a floor of 2. |
| 14 | The τ figures are smoke-only, not "the field the renderer marches" | **real (claim)** | `mediumDensity = densityStatic + smokeDensity`, and `fogAmount` defaults to 0.55 — a mean density, not a rounding term. §6/§8, Findings 7 and Validate item 4 now carry the rendered figure beside the solver figure (≈0.03/m of extra σ_t; 3.4% of the beam surviving becomes ≈2.9% on screen), and `columnDensity`'s docstring says what it excludes. Ratios are unaffected; absolute transmittances quoted for the screen were. |
| 15 | "5.2 s to 1/e" follows from neither source it cites, and ships in a code comment | **real (claim)** | Recomputed: the `0.76 × exp(−0.13 t)` model gives **5.6 s**, and log-linear interpolation of §5's own table (9.042 at 3 s, 4.289 at 8 s, target 6.380) gives **5.3 s**. §5 and `fluid.ts`'s `dissipation` comment now carry 5.3 s as the measured number with 5.6 s as the model's. |
| 16 | Two figures the report's own tables cannot reproduce | **real (presentation)** | (a) The 88.8 → 16.7 peak pair comes from `fluid-beam.js`'s 10 cm march; the printed table is decimated to 0.5 m and tops out at 16.3. §8 and Findings 7 now say so. (b) "the same mass spread over 3.6× the volume" is replaced by what the table holds (72% of the mass over 3.6× the cells). §2's legend now says its post-mean column is `activePostMean`, not a whole-room mean. |
| 17 | `curl` runs inside solids and `curlMagAt` clamps instead of masking | **real (code) — deliberately NOT fixed** | Confirmed: every other kernel guards solids; this one does not, so a cell beside an obstacle gets a confinement gradient with one endpoint inside it. The fix is one `solidAt` early-out plus a solid test in `curlMagAt` — and it changes the field wherever flow touches geometry, which invalidates §3–§5, §7, §9 and both determinism checksums. This pass's compute budget is one 60 s headless run, so re-deriving them is not available. Named in the kernel's own comment and as Below-100% item 20, with the fix written out. **This is the one finding where the shipped code is knowingly wrong.** |
| 18a | `pathtrace.wgsl`'s "would burst" is rename collateral | **real** | Restored to a sentence that means something. |
| 18b | `9940888` is not the as-received commit | **real** | Corrected at the top of this report. |
| 18c | The emission delay is attributed to the sleep gate alone | **real (claim)** | `canister.ts` fires on `body.sleeping || fuse >= FUSE_MAX` with `FUSE_MAX = 1.4`, so an onset bracketed between 1.0 s and 2.0 s cannot distinguish the two, and "emission starting on impact" is unfalsifiable. §7 and Validate item 3 now name both gates and what would actually falsify them. |
| 18d | The occupancy bake is synchronous on the main thread | **real (not a defect)** | 389,376 cells with a BVH query each, re-run by `setScale` and `reset`. A startup hitch and a mid-bench hitch, not a correctness issue; noted in Below-100% item 21 and in the bench script, whose comparison 3 calls `setScale` twice. |

## What changed and why

1. **One owner for solver tuning.** `fluidJacobi` lived in `RenderSettings` too
   and was synced into `tune` every frame, so a sweep measured one count N times.
   `tune` owns it now; `lastJacobi` records what the dispatch loop ran.
2. **Divergence, gradient and Laplacian are one consistent triple.** Velocity is
   differenced as a staggered field, composing to the Laplacian `jacobi` solves.
   Before, the pressure it converged to could not null the divergence measured.
3. **The wall condition moved inside the solve** — zero flux written wherever
   velocity is written, not clamped after projection, so the divergence pass is
   handed a field that already satisfies the boundary condition. Note what
   unblocked the lattice's lowest fluid row, because it is *not* this: row 0's
   own minus-y flux is still zeroed by `wallFaces`. What lets it receive mass is
   the MAC face→centre reconstruction in `velCentre`, which averages in the face
   between rows 0 and 1 — a face that is not a wall. Moving the clamp earlier
   while keeping collocated cell-centre velocities would have left
   `rowMass[0] ≡ 0` exactly as it was.
4. **Instant sources deliver temperature.** The attack envelope is 0 on a
   source's first frame and only density was written past it, so every
   `__smokePuff` blob came out cold whatever it asked for.
5. **`__pinResolution`**, because a scenario's `renderer.resize` did not survive
   the deferred resize the canvas ResizeObserver arms at startup: four bundles
   had been tracing 640×328 while asking for 384×240.
6. **Honest instruments**: an active-cell residual returning `null` rather than
   `0.000000` on an empty set, plus `advectionBalance`, `columnDensity`,
   `densitySamples`, `solidAtWorld`, `solidRow`, `pressureStats`, `resources`.
7. **Defaults and comments from curves.** Jacobi 40 → **20**; the `jacobi` and
   `dissipation` comments now quote what was measured, in the units it was
   measured in.
8. **New bundles**: `fluid-dt` (order of the residual error, plus the
   renormalisation evaluation), `fluid-still` (rendered evidence against a
   measured noise floor), `fluid-pressure`, and `fluid-invariants` (the cheap
   machine-independent re-check: occupancy totals, the resource tally, and both
   readback instruments asserted rather than printed); `vol-shot` now weighs its
   own frame.

## How to see it

The controls, so the thing can be driven. **Validate on the Mac** is the checklist
of what to actually judge, with the figure each item should agree with and what
wrong looks like; this section is just the buttons.

In the browser, on hardware (`npm run dev`):

- **Press 4** (`SMOKE GN`, two charges), aim with the mouse and **left-click**.
  The canister flies on the aim arc, bounces, and starts emitting once its body
  sleeps — about a second — then pours out smoke for 8 s. Watch it billow, curl,
  and settle into a wedge on the floor rather than a ball in the air.
- **Press F** for the torch and walk into that cloud. The beam ends inside it:
  §8 measures τ = 3.39 through a canister-strength cloud, so 3.4% of the beam
  reaches anything beyond.
- **Shoot a ceiling fluorescent** with the pistol (slot 2). The fixture goes
  dark and smoulders for ~20 s — a thin warm plume that climbs and spreads
  under the ceiling it hangs from.
- **Backtick** opens the debug panel: `smoke fluid` carries `simulate`,
  `jacobi iterations` (4–200), `vorticity`, `buoyancy`, `weight` and
  `dissipation`. Turning `vorticity` to 0 makes the medium read as fog rather
  than smoke, and buys back 21.5 points of 8 s mass retention with dissipation
  off (§3) — that trade is the one real tuning decision in this track.
- Console: `__fluid.tune.jacobi = 4` then `await __fluid.divergenceStats()` to
  watch the residual move; `__fluid.densityStats()` to weigh the room;
  `__fluid.setScale(2)` to halve the lattice in x/z.

Headless on a box with no GPU (each needs `dangerouslyDisableSandbox` with the
reason "chromium needs its own namespaces" — Chromium wants its own namespaces):

```bash
npm run build
H=tools/headless
python3 $H/run.py --scenario $H/scenarios/fluid-still.js --shot still.png --json still.json
python3 $H/run.py --scenario $H/scenarios/fluid-mass.js --json mass.json
python3 $H/run.py --scenario $H/scenarios/fluid-dt.js --json dt.json
python3 $H/run.py --scenario $H/scenarios/fluid-jacobi.js --json jacobi.json
python3 $H/run.py --scenario $H/scenarios/fluid-determinism.js --json det.json   # twice
python3 $H/run.py --scenario $H/scenarios/fluid-pressure.js --json pressure.json
python3 $H/run.py --scenario $H/scenarios/fluid-invariants.js --json inv.json
```

`fluid-invariants.js` is the cheap one and the one to reach for first: ~15 frames
at 320×200, every assert machine-independent, and it pins the occupancy bake, the
resource tally and both readback instruments. Run it after any change believed
inert; it is what caught that a "believed inert" change was, and what would catch
one that was not.

`fluid-still.js` and `vol-shot.js` pick their shot with a `MODE`/`WHICH`
constant at the top (a JS scenario cannot take arguments); copy the file, sed
the constant, run the copies in parallel. `fluid-mass.js` and
`fluid-lifetime.js` split by phase the same way — `noDissJ200` is the most
expensive run in the suite and wants its own lane.

## Verification

Every run is Chromium/SwiftShader via `tools/headless/run.py`, sim grid at full
production resolution (208 × 13 × 144, 0.25 m cells, `scale = 1`) — no number
here used the halved debug lattice, and no number depends on how many pixels
were traced. Render size is quoted per bundle and is now the size the scenario
asked for.

**Build provenance.** Four builds, named here as A (`917c37f`), B (`895a8fe`),
C (`36a63bb`) and D (branch HEAD, what ships — the review resolutions). Bundles
1–3, 5, 7–9, 12–14 and determinism runs 1–2 were measured on A; bundles 4, 6, 11,
`crouch-matrix` and determinism runs 3–5 on B; the closing re-checks below on C.

**On D, and why the tables above it still stand.** D changes solver and source
code, so its inertness is a claim that needs support rather than assertion. Every
arithmetic change in it is inert on the recorded runs *by construction*, and here
is each one: confinement scales by `min(cell)` instead of `cell.x`, which are the
same 0.25 at `scale = 1` and every recorded run is `scale = 1`; the source radius
divide gains `max(r, 1e-4)`, and every radius the game and the suite use is
≥ 0.16; the instant-source fix only changes frames that inject nothing, and every
recorded run injects before silencing and at dt = 50 ms; `divergenceStats` writes
its recomputed field to a different texture and computes the same values from the
same inputs; `resources()` tallies the same layout it always did (same five
numbers); `densityStats` adds a second hash without touching the first; and the
occupancy rule change was verified inert by enumerating all 532 static boxes —
old and new predicates select exactly the one ceiling slab, so the bake is
bit-identical. The two changes that are *not* inert were not made: see
Below-100% item 20. Only the cold start was re-run on D (§0); nothing else was,
because the budget was one run.

Stripped of comments, `git diff A..B -- src/` is exactly three things: `COPY_SRC`
added to the pressure texture pair, the new `pressureStats` readback, and `env = 1`
inside the instant-source branch of the source packer. No kernel, no dispatch, no
tuning value, and the third is inert for everything measured — `puff()` is the
only instant source in the game and it passes no temperature, so the packed
uniform is byte-identical either way.

`git diff B..C -- src/` is one hunk in one file: the `__pinResolution` debug hook
in `main.ts` now throws on a half-specified pin instead of silently releasing it
(see Merge notes). No solver, shader, renderer or tuning code, and every existing
caller passes both arguments, so no bundle's behaviour changes. §10 shows the
determinism checksum unchanged across all three builds, and §4's numbers
reproduced across A and B to every digit quoted; that is the evidence, rather
than the claim.

### 0. What this closing pass re-checked, and what it did not

The compute budget for the closing pass was three short headless runs, so most of
the table below is **prior-session measurement** carried forward from the runs
this report already recorded. Numbers are prior-session unless they appear in this
list. Nothing measured earlier was deleted to make the accounting tidy; where a
figure could not be re-run it is cited as it was measured.

Re-checked now, on build C, in three foreground runs — one at a time, nothing else
on the box, 38 s / 42 s / 62 s of wall clock:

| check | run | result | compare against |
|---|---|---|---|
| cold start | `smoke.js`, render 384×240, 6 stills | `ok: true`, `errors: []` — no page exception, no uncaptured device error, no WGSL compile or pipeline/bind-group validation error; probe sees 33 lights, nav 1995/7488 blocked, 4 guards; screenshot read at full size is the lit conference room, not black. One console `error` line, accounted for below | §14's `smoke.js` row |
| still-air plumbing check | `stillAir` tuning, 1 injection step + 12 steps, render 320×200 | mass **17.454481288790703** at step 1 and the **identical** value 12 steps later — retained exactly 1, density-lane checksum `773e40f5` both ends; peak 23.078125 | §3's `stillAir` row, and §5's injection-step peak 23.08 (same blob, so the same peak before any force has acted) — both to every digit. Read what this does and does not prove in §3: with every force coefficient zero it is an arithmetic identity, not a measurement |
| determinism | `fluid-determinism.js` unmodified, 40 steps at 50 ms, render 320×200 | density-lane checksum **`d9b49a13`**, mass 11.319137, peak 9.320313, 838 cells, centroid (5.0034, 0.9755, 0.0034) | §10's five prior runs — identical in every figure, now across three builds and six invocations |
| `physics` selfTest | free with the mass run (no frames) | `pass: true`, **25 PASS / 0 FAIL / 5 INFO** | §12, reproduced exactly |

**Build D re-check (the review resolutions), one run.** `fluid-invariants.js`, a new
bundle written for exactly this situation: every assert in it is
machine-independent and costs a handful of frames, so it is what to run when a
change is believed inert and there is no budget to re-derive transport. 15 frames
at 320×200, `ok: true`, `errors: []`.

| what it pins | result | compare against |
|---|---|---|
| cold start | probe sees 33 lights, nav 1995/7488 blocked, 4 guards; no page exception, no uncaptured device error, no WGSL compile or pipeline/bind-group validation error | §14's `smoke.js` row and the C cold start above — identical |
| occupancy bake | **19,420** solid cells and per-row `[2085, 2085, 2070, 1877, 1599, 1242, 1242, 1242, 1106, 1218, 1218, 1218, 1218]` | §13 — identical, so the geometric ceiling-skip rule bakes the same field the flag-based one did, measured rather than argued |
| `resources()` | 1 uniform / 1 sampler / 4 sampled / 1 read-only storage / 3 storage textures, `storageTexturesPerStage` 3, **19.682 MB** | §13 — identical, now tallied from the layout's entries |
| `divergenceStats()` twice at one checkpoint | `preMeanAbsDiv` 2.2199059996379198e-05 and `preMaxAbsDiv` 0.08289146423339844 on **both** calls | the defect this fixes: the second call used to report post-projection values as pre |
| instant source on a frame that cannot deliver | frozen clock: held **0**, then 17.34404917061329 once released. Silenced: held **0**, then the same 17.34404917061329 | the puff used to be consumed for nothing; the delivered mass also matches §5's `dissipation = 0.13` t = 0 row (17.344) across sessions and builds |
| the three fluid passes | `fluidAdvect`, `fluidPressure`, `fluidScalars` present by name | §13 |

**Two runs were spent here, not one.** The first found a defect in the bundle's own
frozen-clock assert rather than in the branch: `dt` is `now − prev` clamped to
50 ms, and the *first* frame after `__freezeClock(true)` is fed `frozenClock`
against a `prev` left by a synthetic frame that lags real time by ~1 s per traced
frame — so it takes a full 50 ms step, and dt is exactly 0 only from the second
frozen frame. The assert was corrected to render a settling frame first and the
second run is the one tabulated above. Recorded because it changes what a scenario
author can rely on (Findings 15) and because a review of this report should know
the budget went 2 runs, not 1.

**The one console error line, since "no console errors" is the claim being made.**
The cold-start run's console carries exactly five lines: four `log` lines (scene,
radiosity, lightvolume, rig) and one `error` — `Failed to load resource: the
server responded with a status of 404`. It is not the app's: `dist/index.html`
references one asset (the JS bundle) and the bundle fetches two (`rig.bin`,
`rig.json`); all three exist in `dist/`, and the rig line in the same console
proves the fetches succeeded. `dist/` contains no `favicon.ico` and `index.html`
declares no icon, so the 404 is Chromium's implicit favicon request against
`run.py`'s static server. It predates this track and is not fluid-related. I did
not spend a run confirming the URL — the harness records the message text, not the
request — so this is inference from what `dist/` contains, and it is the only
request that can 404.

Deliberately **not** re-run, and cited as prior-session throughout: the mass table
(§3), the dt-order and renormalisation arithmetic (§4), the lifetime curves (§5),
every rendered still and its image reading (§6), the canister sequence (§7), the
beam march (§8), the column probes (§9), the 1200-step pressure warm-start (§11),
and the resource counts (§13). Each of those is minutes to hours of SwiftShader
per bundle — `noDissJ200` alone took 1142 s and `fluid-pressure` 2197 s — which
is why the three cheapest high-signal checks were the ones chosen: one that proves
the renderer still comes up clean, one plumbing check on a field with no motion in
it, and one determinism checksum.

Be precise about the reach of that checksum, since it carries most of the weight
here. It hashes the **density lane of `scl[0]` only**. It would move on any drift
in the solver's arithmetic or dispatch order that has reached density within
40 steps — which is the great majority of them, because density is downstream of
every pass — but a drift confined to velocity, pressure or temperature and not yet
propagated into density would pass it. Branch HEAD adds `fieldChecksum` over all
four scalar lanes (so temperature is covered too); it has no cross-run history
yet, and the density-lane figure is what §10's six invocations compare.

### 1. The plumbing fix, proved rather than asserted

`fluid-mass.js` fails the run unless each phase's requested Jacobi count equals
`lastJacobi`. Across a 25× spread:

| phase | requested | dispatched |
|---|---|---|
| `noDissJ8` | 8 | 8 |
| `noDissJ200` | 200 | 200 |

Before the fix both dispatched 40 — the value `settings.fluidJacobi` pushed in
every frame — which is exactly why the two phases had come back identical.

### 2. Iteration sweep: hypothesis refuted, cause found

`fluid-jacobi.js`, render 320×200. A sustained 4 m/s horizontal jet from a
zeroed field, read at step 3 (transient) and step 24 (quasi-steady). `relResid`
is the mean |∇·v| over cells moving faster than 0.05 m/s, relative to
`activeVelRms / cell`; `meanRed` is pre/post reduction of whole-room mean |∇·v|.
The last column is `activePostMean` — the post-projection mean over *moving* cells
only, not the whole-room mean, which is 4.6e-4 for the reason Findings 2 gives.

Quasi-steady (step 24), before vs after the operator fix:

| Jacobi | meanRed before | meanRed after | relResid before | relResid after | postMean after (1/s) |
|---|---|---|---|---|---|
| 4  | 1.27 | 2.13  | 2.86e-2 | 1.07e-2 | 0.01972 |
| 10 | 1.42 | 3.62  | 2.54e-2 | 3.92e-3 | 0.00758 |
| 20 | 1.49 | 5.70  | 2.54e-2 | 1.64e-3 | 0.00317 |
| 40 | 1.52 | 9.46  | 2.55e-2 | 7.97e-4 | 0.00155 |
| 80 | 1.53 | 17.67 | 2.57e-2 | 3.87e-4 | 0.00076 |

At step 3 the after-fix residual is 5.29e-2 / 1.42e-2 / 4.60e-3 / 2.71e-3 /
1.26e-3 for the same counts. (The "before" columns are the pre-fix run recorded
earlier on this branch; every "after" figure was re-measured in the closing
pass and reproduced to the digits shown.)

**The refutation.** With the plumbing fixed but the operators unchanged, mass
retention over 8 s was 0.4301 at 8 iterations and 0.4287 at 200 — a 0.3%
difference, in the wrong direction. The `relResid` column says why: before the
fix the residual was *flat*, because the iteration was converging to the wrong
pressure rather than converging slowly, and the projection reduced the
divergence it measured by only 1.5×.

**The cause.** Central differences on a collocated grid are a 2h stencil; the
Jacobi kernel solved the h-spacing Laplacian. The pressure the solve converged
to was therefore not the pressure that nulls the measured divergence, and the
h-scale divergence advection actually transports was invisible to the operator.
After the fix the residual falls as ~1/N with no plateau, which is what an
iteration-limited solve looks like.

**Why the default is 20.** In mass terms, leftover divergence costs the density
field dt × the *absolute* active-cell mean |∇·v| per step: 0.05 s × 0.0032/s =
1.6e-4 at 20 iterations, against the advection scheme's own 1e-3…3e-3. So 20
buys an order of margin over the dominant error; everything above it is real in
the residual and unmeasurable in mass (§3: 8 and 200 iterations retain within
0.24% of each other).

### 3. The mass table

`fluid-mass.js`, render 320×200. One instant blob — peak density 25, radius
0.9 m, at (5, 1.4, 0) in open corridor air — then **every** emitter silenced
(permanent wisps dropped, canisters cleared, guards frozen). The room is closed,
so with `dissipation = 0` any change in the density integral is the solver's own
numerical error.

| phase | tuning | retained @ 8 s (before) | retained @ 8 s (after) |
|---|---|---|---|
| `stillAir` | no forces at all | 1.00000 † | **1.00000** |
| `sinkOnly` | weight only, no confinement | 0.2685 | **0.93278** |
| `noDiss` | weight + confinement, 20 iterations | — ‡ | **0.71758** |
| `noDissJ8` | as above, 8 iterations | 0.4301 | **0.71873** |
| `noDissJ200` | as above, 200 iterations | 0.4287 | **0.71702** |
| `defaults` | shipped tuning (dissipation 0.13) | — | 0.24727 |

`stillAir` retains 17.454481288790703 at step 1 and the identical value at steps
41, 81 and 121 (`fluid-stats.js`). **This row is the one mass figure re-checked in
the closing pass** (§0): on build C the same blob under the same tuning gives the
same 17.454481288790703 at step 1 and identically at step 13, density-lane
checksum `773e40f5` at both ends.

**What that row is and is not.** An earlier revision called it "the control that
fp16 storage and solid-cell zeroing are lossless, so every loss below it is
transport". It is not that. With `dissipation`, `buoyancy`, `weight` and
`vorticity` all zero and a blob that carries no temperature, `forces` leaves
velocity untouched (the confinement gate `el > 1e-5` never fires on a zero field),
so velocity is **identically zero forever**; `velCentreB` is exactly 0, the
backtrace lands on the cell centre with `fr = 0` exactly — 0.25 m cells are a
power of two, so the `(p − origin)/cell` round-trip is exact — and the gather
reduces to weight 1 on the cell itself. `dens * exp(−0 · dt)` is exact. Retaining
exactly 1 is therefore an **arithmetic identity, not a measurement**. Solid-cell
zeroing is not exercised at all: the blob sits in open corridor air on purpose,
and fp16 is only asked to re-store a value it already holds. It still earns its
place as a plumbing check — anything other than 1.000 would mean the pipeline
creates or destroys density with nothing moving — but it is not the floor under
the transport figures, and nothing here establishes such a floor. Every other
number in this table is a prior-session measurement. († the pre-fix
`stillAir` figure is from `fluid-stats.js` over 6 s, since the pre-fix
`fluid-mass.js` phase list did not include it; `defaults` was not measured
pre-fix at all. ‡ pre-fix there was no 20-iteration measurement to have: every
phase dispatched 40 whatever it asked for.)

The three `noDiss` variants — 8, 20 and 200 iterations — land within **0.24% of
each other**, which is the positive form of the refutation.

**Where it went, and the proof.** `fluid-leak.js` weighs the field immediately
before and after one advection step, per y row. Row 0 is the lowest *fluid*
layer, the 25 cm of air above the floor plate (2085 of its 29,952 cells are
solid where geometry stands). Before the fix, row 0 held exactly 0.0000 at every
checkpoint of every phase while row 1 held up to 4.63 — that layer was
*unreachable*, because its wall-normal velocity was clamped to zero after
projection so its backtrace never left the cell. After the fix it fills. This
bundle runs the `sinkOnly` tuning (dissipation 0, **confinement 0**), so its
defects are the floor of what the scheme costs, not the shipped figure — with
confinement on the same blob at t = 1.5 s loses 0.729% per step rather than
0.292% (§4):

| t (s) | mass before | mass after | rel. defect | row 0 mass | active cells | postMean (1/s) | relResid |
|---|---|---|---|---|---|---|---|
| 0.5 | 17.349 | 17.324 | −0.144% | 0.0000 | 168 | 0.00041 | 7.8e-4 |
| 1.5 | 16.606 | 16.558 | −0.292% | 0.0578 | 682 | 0.00021 | 2.7e-4 |
| 2.5 | 15.517 | 15.473 | −0.279% | 1.9253 | 875 | 0.00021 | 2.3e-4 |
| 3.5 | 15.256 | 15.269 | **+0.077%** | 5.9657 | 902 | 0.00016 | 1.8e-4 |
| 5.0 | 15.802 | 15.819 | **+0.108%** | 9.4146 | 934 | 0.00014 | 1.9e-4 |

The sign flips: the scheme is no longer a monotone sink but a bounded two-sided
error. It loses while the cloud is descending fast and gains slightly once it is
pooled (the solid-aware gather renormalises its weights against the floor, which
over-counts a little). Per-row: at t = 1.5 s rows 0–3 gain (+0.018, +0.114,
+0.147, +0.020) while rows 4–8 lose (−0.096, −0.121, −0.089, −0.037, −0.004) —
the defect is a transfer down the column with a net loss, not a uniform drain.

### 4. What order of error is left, and why renormalisation was rejected

`fluid-dt.js`, render 320×200. The same blob and the leaky configuration
(`dissipation` 0, weight and confinement on) run to 8 s of game time at three
step sizes. `__renderStill(n, dtMs)` advances exactly `dtMs` of game time per
frame, so this needs no code change — only more frames.

| dt (ms) | steps to 8 s | per-step defect @ 1.5 s | retained @ 8 s | peak retained |
|---|---|---|---|---|
| 50   | 161 | −7.293e-3 | 0.71758 | 0.21699 |
| 25   | 321 | −3.601e-3 | 0.70820 | 0.21192 |
| 12.5 | 641 | −1.756e-3 | 0.70445 | 0.19804 |

Fitted orders in dt: **per-step defect 1.018 and 1.036** (first order),
**total loss −0.047 and −0.018** (zeroth order). The per-step error halves when
the step halves, and there are twice as many steps, so the loss over a fixed
span of game time does not move. Refining the step buys nothing; the tiny
*decrease* in retention is the next-order term.

What the orders establish, without needing a mechanism: the defect is a property
of the advection *discretisation* — gather plus trilinear interpolation — and not
of the pressure solve or the step size. The gather hands each source cell out to
its neighbours with interpolation weights, and mass is conserved exactly only if
the weights every source cell hands out sum to 1; they do not, and the shortfall
is first order in the displacement, which is first order in dt. (A cleaner
derivation than that would need care: the ρ interpolant is only C⁰, so the usual
Jacobian argument — which would predict O(dt²) for a divergence-free flow — does
not apply at the nodes, and I did not verify a sharper statement. The measured
order is the claim; the sentence above is the standard reading of it.)

The size follows, and the cleanest form of it comes
from `fluid-leak.js`, which reads both quantities out of the *same* run at the
*same* instant (confinement off, dissipation off): at t = 1.5 s the advection
step loses 0.292% of the field, i.e. an effective divergence of **0.0585/s**,
while that run's post-projection active-cell mean |∇·v| is **0.00021/s**. **280×
apart.** With confinement on — the shipped tuning, `fluid-dt`'s configuration —
the same blob at the same instant loses 0.729% per step, so the gap is wider
still. No iteration count and no step size can close it; only flux-form
(conservative) advection can.

**Global mass renormalisation: evaluated, rejected.** Scale the whole field each
step so its integral matches a target. What that would restore, computed from the
dt = 50 ms run at t = 8 s:

| quantity | before renorm | after renorm |
|---|---|---|
| mass retained | 0.71758 | 1.00000 (by construction) |
| factor it must apply | — | 1.3936 |
| peak density retained | 0.21699 | 0.30240 |
| peak deficit | 78.3% | 69.8% |

It closes the mass gap exactly and closes 8.5 of the 78.3 points of peak
deficit. The quantity that fails visually is not the integral — it is the peak,
and the peak falls because linear interpolation diffuses the field (peak 23.08 →
5.01 over 8 s while cells above the 0.05 visibility threshold go 176 → 641 — 72%
of the mass, per §5's table, spread over 3.6× the cells). Renormalisation multiplies an
already-diffused field: the smoke gets denser, not less blurry, and it gets
denser *where density already is*, i.e. mostly in the diffuse halo that carries
most of the mass by then, not in the descending core where the mass was lost.

Three further reasons it is not worth it. (a) The defect changes sign — −0.29%
per step while descending, +0.11% once pooled, and a free-air coasting blob
*gains* — so this is a two-sided controller, not a one-way top-up. (b) With live
sources the target integral must include the injected mass, which is the discrete
sum of a soft-sphere splat over the lattice, obtainable only by weighing the
field either side of the injection: two more full-field readbacks per frame, and
this solver's readbacks stall the queue by design (they map a staging buffer and
are documented as diagnostics). A deterministic GPU tree reduction avoids the
stall at two more passes plus a frame of lag. (c) That lag makes the correction
a global, spatially uniform density modulation driven by local events — a muzzle
burst across the room would nudge a pooled canister cloud's density for a frame,
coupling every emitter to every cloud at the source's own flicker rate.

So: **no renormalisation was added.** The fix that would actually work is in the
advection, not after it — either flux-form (conservative) transport, or a gather
that accounts for the weights it hands out, which needs a scatter pass to
accumulate them. Both want integer fixed-point atomics rather than float ones so
determinism survives, which is a hard deliverable here. Note the solid-aware
gather already does exactly this weight-renormalisation *locally* against solid
cells (`sampleScalarsFluid`); doing it globally is the same idea and a whole
pass. Neither was attempted: both are more than a tuning change and neither is
needed for the medium to read correctly.

### 5. Lifetime and the honest e-folding numbers

`fluid-lifetime.js`, render 320×200, same blob, 400 steps per phase.

`dissipation = 0` — the numerical error alone:

| t (s) | 0 | 1 | 2 | 3 | 5 | 8 | 12 | 16 | 20 |
|---|---|---|---|---|---|---|---|---|---|
| mass | 17.454 | 16.608 | 14.468 | 13.345 | 12.841 | 12.525 | 12.402 | 12.595 | 13.330 |
| peak | 23.08 | 18.67 | 11.64 | 8.38 | 6.90 | 5.01 | 3.86 | 3.00 | 2.46 |

Retained to 3 s **0.7646**, to 20 s **0.7637**, and the rate over 8–20 s is
−0.0052/s — a slight *gain*. So there is no e-folding leak: there is a **one-off
deficit of ~24% taken while the cloud is billowing in the first ~3 s, and a
bounded drift under 0.01/s thereafter that changes sign.**

`dissipation = 0.13` (shipped) against its own model `exp(−0.13 t)`:

| t (s) | 0 | 3 | 8 | 12 | 16 | 20 |
|---|---|---|---|---|---|---|
| mass | 17.344 | 9.042 | 4.289 | 2.594 | 1.583 | 0.984 |
| actual / modeled | 0.994 | 0.765 | 0.695 | 0.707 | 0.726 | 0.759 |

Three e-folding numbers, all measured, none of them interchangeable:

- **modeled 7.69 s** — what `dissipation = 0.13` means on its own;
- **settled 8.15 s** — the rate the field actually follows over 8–20 s is
  0.1227/s, within 6% of the model;
- **5.3 s to 1/e of what was injected** — the number the game actually shows,
  because the front-loaded transient offsets the whole curve: the actual/modeled
  ratio is 0.765 once the transient ends at 3 s and stays in the 0.70–0.76 band
  out to 20 s, so mass tracks roughly `0.76 × exp(−0.13 t)` rather than
  `exp(−0.13 t)`.

  Both routes to that third number, since an earlier revision quoted 5.2 s and
  neither route gives it. From the table directly: the target is 17.344/e = 6.380,
  and log-linear interpolation between t = 3 (9.042) and t = 8 (4.289) puts the
  crossing at **5.34 s**. From the offset model: `0.76 × exp(−0.13 t) = 1/e`
  solves to `(1 + ln 0.76)/0.13` = **5.58 s** (5.63 s at the quoted 0.765). The
  measured interpolation is the one to quote — 5.3 s — and the 4% gap between it
  and the model is the model's, not the measurement's. `fluid.ts`'s `dissipation`
  comment carries both.

**`dissipation` stays at 0.13.** The deficit is front-loaded; lowering the rate
to absorb it would leave the late haze hanging around too long. What was wrong
was the claim, and the claim is now the measured one. At t = 20 s the field still
carries peak density 0.25 over 5763 cells (σ_t ≈ 0.013/m) — "still a haze at
20 s" survives contact with the instrument.

### 6. Rendered evidence: a cloud under a ceiling light

`fluid-still.js`, render **384×240** (pinned — see item 5 of What changed; this
is the first bundle in the campaign whose quoted render size is the one it asked
for). The fixture is the conference room's warm fluorescent at (−20, 3.06, 1),
power 3.4, the brightest practical in the level. One `__smokePuff` blob (radius
1.0 m, peak 220) is injected 1.6 m below it and carried by the solver for 1.5 s,
so the camera sees an advected field rather than the analytic sphere it started
as. Three captures from an identical frozen pose: two with an empty field to
measure the sampler's own noise, then the cloud.

One correction to that "1.5 s", measured on build D and applying to every still in
this section. `__freezeClock(true)` does not make the *first* subsequent frame a
dt = 0 frame — dt is `now − prev` clamped to 50 ms, and the first frozen frame is
fed `frozenClock` against a `prev` that lags real time, so it takes a full 50 ms
step (Findings 15). Each capture therefore advances the sim one extra step and
damps the camera once toward the player: the field is 1.55 s old, not 1.50 s, and
the aim is one damping step off the cloud. All three captures run the identical
protocol and `densityStats` is read after the frames, so the noise floor, the
difference measurements and the field figures below are self-consistent; only the
stated age and the exactness of the aim were wrong.

Field at capture: mass 148.63, peak density 61.31, 787 cells ≥ 0.05, box
4.00 × 2.50 × 4.00 m, mass centroid at y = **0.43 m** having been injected at
1.45 m, cells per row from the floor up 188/149/112/87/75/52/52/44/24/4. The blob
has already become a low bank, thickest at the floor.

The cloud's own contribution to the image — cloud frame minus control frame, in
HDR luma, with everything expressed against the measured noise floor because two
ten-frame accumulations of the same scene are not identical:

| quantity | value |
|---|---|
| sampler noise floor (RMS of two identical captures) | 0.00300 |
| peak change | 0.2014 = **67× the noise floor** |
| footprint (pixels changed by more than 4× noise) | 8048 = 8.7% of the frame |
| of those, below a quarter of the peak change | 80% |
| in-scattering pixels (brighter) | 4889, mean +0.0364 |
| occluding pixels (darker) | 3159, mean −0.0321 |
| net change over the footprint | +76.3 |

Both signs are present in quantity, which is the substantive check: the medium
in-scatters the fixture's light where the background was dark **and** extinguishes
the lit desk and carpet behind it. A medium doing only one of those is not in the
light transport.

Self-shadowing is measured in the volume rather than on screen, because under a
50° three-quarter camera "the top of the footprint" is mostly world depth, not
world height. Marching the solver's own lattice straight down from the lamp at
the renderer's σ_t = 0.05 per unit density, transmittance arriving at the cloud's
top (y = 2.50) is **0.9989**, at its mass centroid (y = 0.43) **0.2292**, and at
its underside (y = 0.13) **0.1309**. The underside receives **13% of the light
the top receives**; lamp-to-floor optical depth through the cloud is τ = 2.25.

Those are figures for the **smoke field alone**, which is all `columnDensity`
reads. The renderer's medium is `mediumDensity = densityStatic + smokeDensity`,
and `densityStatic` is the drifting fog at mean `settings.fogAmount` = **0.55** —
a mean density, not a rounding term, worth ≈0.03 of τ per metre. Over the lamp's
own 3.06 m that is +0.08, so on screen the same march reads T = 0.984 at the
cloud's top, 0.213 at the centroid, 0.121 at the underside, lamp-to-floor
τ = 2.33, and an underside/top ratio of **12.3%** rather than 13%. The
self-shadowing *conclusion* is unaffected — the ratio barely moves, because the
fog is nearly common to both paths — but a Mac reviewer comparing absolute
transmittances should compare against the rendered column.

**What the image shows.** I read `still2-puff.png` at full size. The conference
room from the house three-quarter angle: red carpet, the tan desk running
diagonally, blue chairs, the exit sign's green spill at bottom-left, the
fluorescent's warm pool on the wall at top-right. The smoke reads as a **soft,
low-lying bank filling the middle of the room** — pale grey, no boundary
anywhere, fading continuously into clear air rather than ending. It is brightest
just above the desk surface where the fixture's light enters it from directly
overhead, and it *darkens downward*: the carpet under it is duller and flatter
than the same carpet in the control frame, and the desk's grey side panel is
visibly dimmed rather than lit. The chair nearest the camera is half swallowed;
the desk's far end is veiled but still legible through it.

Against the brief's three words: **soft — yes**, and not marginally; there is no
edge in the image at any exposure, which matches the 80% of the footprint sitting
below a quarter of peak. **Self-shadowed — yes**, top bright and underside dark,
and the volume march puts a number on it (13%). **Buoyant — no, and correctly
not**: `__smokePuff` injects density with no temperature, buoyancy in this solver
is driven by temperature while density carries weight, so a puff is cold smoke
and cold smoke sinks. Its centroid fell 1.45 → 0.43 m in 1.5 s. That is the
canister's regime and it is what the game wants there; it is not a picture of
buoyancy, so the warm case below is a second shot rather than a sentence.

Two things the shot does *not* show, worth saying so nobody quotes it further
than it goes. It does not read as an object with a silhouette — a 4 m cloud at
12 m under a top-down camera is a bank, not a puff, and anyone wanting a hero
shot should frame it from lower down. And at roughly `vol-shot.js`'s blob strength
there is almost nothing to see: the same pose with peak 40 injected (12 after two
seconds) measures lamp-to-floor τ = 0.82 instead of 2.25, an underside still
receiving 46% of the top's light instead of 13%, and a footprint of 1.9% of the
frame — an image that is a faint veil over a desk. That is a physically correct
picture of thin smoke, and it is why this bundle uses the density the canister
actually reaches in play (§7: peak 138).

**The warm case, so buoyancy is shown rather than asserted.** Same pose, same
fixture, `MODE = "warm"`: a sustained warm source at (−20, 0.5, 1), left running
for 4 s. Say exactly what that source is, because it is **not** `Smoke.smolder`
and buoyancy is the one thing this shot exists to show. Against smoulder's
`radius 0.35, temp 4, density 4.5, attack 1.5, life 20`, the shot spawns
`radius 0.5, temp 6, density 45, attack 1.0, life Infinity`: **10× the density,
2.9× the emitting volume** (0.5³/0.35³ — so ≈29× the injected mass rate) **and
1.5× the temperature**, which is a different buoyancy regime, not a brighter
version of the same one. Its `attack: 1.0` is inert as written — the envelope only
applies when `life` is finite (`smoke.ts`) — so the plume starts at full strength
instead of ramping. The field at capture is the opposite of the blob's in every
respect that matters:

| | cold puff | warm plume |
|---|---|---|
| injected at y | 1.45 m | 0.50 m |
| mass centroid at capture | **0.43 m** (fell 1.02 m) | **1.17 m** (rose 0.67 m) |
| cells per row, floor upward | 188/149/112/87/75/52/52/44/24/4 | 14/24/28/38/40/50/46/34/19/6 |
| in-scattering : occluding pixels | 4889 : 3159 (1.5 : 1) | 2834 : 375 (**7.6 : 1**) |
| lamp-to-floor τ | 2.25 | 1.67 |
| underside / top transmittance | 0.131 | 0.194 |

Reading `still2-warm.png`: a soft pale column rises from low beside the desk and
**mushrooms outward as it reaches the fixture**, its brightest part high — up in
the lamp's light — and thinning downward toward its root. The carpet and chairs
below are barely touched, which is the 7.6 : 1 in-scatter ratio in visual form: a
plume high in bright air scatters light toward the camera, where the cold bank
low over lit surfaces mostly takes light away from them. The row histogram says
the same thing without the camera: mass peaks at rows 5–6 (y 1.25–1.75 m) for the
plume and at row 0 for the bank.

So of the brief's three words: "soft" and "self-shadowed" hold for both clouds,
and **"buoyant" is a property of the source, not of the medium** — cold smoke
sinks and pools, warm smoke climbs and mushrooms, and the solver produces each
from the same three lines of force (buoyancy on temperature, weight on density,
confinement on curl). Asking the medium to be buoyant is asking for temperature.

### 7. Canister sequence — density-slice stats at t = 0.5 / 1 / 2 / 5 / 10 s

`fluid-sequence.js`, render **384×240** (the size asked for; see item 5 of What
changed). One scripted throw from a fixed release point, no animation state in
the protocol.

| t (s) | emitting | mass | peak | cells ≥0.05 | bbox size (x,y,z) m | centroid y | relResid |
|---|---|---|---|---|---|---|---|
| 0.5  | no  | 0.000   | 0.00   | 0    | —                  | —     | null |
| 1.0  | no  | 0.000   | 0.00   | 0    | —                  | —     | null |
| 2.0  | yes | 15.835  | 25.03  | 237  | 3.00 × 1.25 × 2.50 | 0.501 | null |
| 5.0  | yes | 78.045  | 138.62 | 567  | 5.00 × 1.25 × 4.50 | 0.293 | 2.4e-4 |
| 10.0 | no  | 133.028 | 79.94  | 1008 | 5.25 × 1.50 × 6.50 | 0.211 | 4.3e-4 |

The cloud grows monotonically; it spreads **3.5–4.3× further horizontally than
vertically** (a 5.25 × 6.50 m footprint against 1.50 m of height) and its mass
centroid *falls* from 0.50 m to 0.21 m while it grows, which is pooling rather
than rising. Occupied cells by row at t = 10 s are 293/280/238/163/31/3 from the
floor up — a wedge thickest at the floor. Peak falls 138 → 80 between 5 s and
10 s as emission ends and the cloud spreads.

The two zero rows are correct behaviour, not a gap: at 0.5 s and 1.0 s the
canister is still bouncing and has not begun to emit. `emitting` brackets the
onset between 1.0 s and 2.0 s.

That bracket does *not* identify which gate fired, and an earlier revision said it
did. `canister.ts` starts emitting on `body.sleeping || fuse >= FUSE_MAX` with
**`FUSE_MAX = 1.4`**, so emission is guaranteed by 1.4 s whatever the body is
doing, and a 1.0–2.0 s bracket is consistent with either gate. Distinguishing them
needs a checkpoint inside the bracket plus `body.sleeping` read at the same
instant; neither was measured. What the run does establish is the behaviour that
matters in play — a throw, then a delay, then emission — not its mechanism.

`relResid` is `null` at 2.0 s because *nothing in the room is moving faster than
0.05 m/s* — the pooled cloud's spread there is numerical diffusion, not flow.
That is a finding, not a defect in the run.

### 8. Beam through the cloud

`fluid-beam.js`, render 448×280, torch on, a canister-strength emitter pinned on
the beam axis 4 m out. σ_t = 0.05 per unit density (`settings.volumetric`),
`volExtinction` on, marched at 10 cm.

| s (m) | 3.0 | 3.5 | 4.0 | 4.5 | 5.0 | 5.5 |
|---|---|---|---|---|---|---|
| solver ρ | 0.00 | 23.8 | 88.8 | 27.3 | 0.06 | 0.00 |
| solver T | 1.000 | 0.830 | 0.161 | 0.037 | 0.034 | 0.034 |
| gameplay ρ | 4.7 | 10.1 | 13.3 | 16.3 | 9.9 | 2.5 |
| gameplay T | 0.918 | 0.751 | 0.557 | 0.381 | 0.271 | 0.238 |

On the solver's smoke field: column integral 67.83, **τ = 3.392**, transmittance
out **0.0337**. Half the beam is gone by 3.8 m and 90% of it by 4.2 m — the beam
is not merely dimmed, it is extinguished inside a 0.4 m span of the cloud, and
**3.4% of it survives** to light anything beyond.

On the *rendered* medium, add the fog: `mediumDensity = densityStatic +
smokeDensity` with `fogAmount` = 0.55 mean, so over this ~6 m of beam the fog is
worth another ≈0.17 of τ. τ ≈ 3.56, transmittance out ≈ **2.9%** rather than 3.4%.
Same verdict, and the survivor fraction a Mac reviewer should compare against is
the 2.9%.

The gameplay readback on the same ray gives τ = 1.434 (T = 0.238) with the peak
smoothed 5.3× — see Findings. That 5.3× is `88.8 → 16.7` from the scenario's own
10 cm march; the table above is decimated to 0.5 m and so tops out at 16.3, which
is why dividing the table's own numbers gives 5.45 instead.

I read `beam.png` too, and this is the shot that sells it: the beam leaves the
player and *ends* in a bright soft ball of lit smoke a few metres out, which
throws light sideways onto the cubicle panel and desk beside it while the
corridor beyond stays as black as it was. A moonlit pool further up the corridor
is untouched by the torch. The frame agrees with the march: the beam is consumed,
not attenuated.

### 9. Column obstacle

`fluid-column.js`, render 384×240. A held-on 3.5 m/s jet aimed down +x at the
corridor support column at (0, ·, −3.9) — 0.28 m half-extent, full height — from
3 m upwind, probed after 4 s on the solver's own lattice.

| probe | at (x, y, z) | occupancy says | solver ρ | coarse readback ρ |
|---|---|---|---|---|
| upwind | (−1.0, 1.5, −3.9) | fluid | **4.4145** | 1.7338 |
| inside the column | (0, 1.5, −3.9) | **solid** | **0.0000** | **0.9914** |
| beside it, +z | (0, 1.5, −3.4) | fluid | **1.2606** | 0.9855 |
| beside it, −z | (0, 1.5, −4.4) | fluid | **1.9327** | 0.8974 |
| lee, 0.6 m | (0.6, 1.5, −3.9) | fluid | 0.0941 | 0.4728 |
| lee, 1.5 m | (1.5, 1.5, −3.9) | fluid | 0.0000 | 0.0016 |

Three numbers make the claim: the column's cell is the one the occupancy bake
calls solid, density inside it is **exactly zero**, and density 0.5 m to either
side is not — 1.26 and 1.93 against 4.41 arriving upwind. The plume splits.
Behind the column the wake is still open at 4 s: 0.094 at 0.6 m (2% of the
upwind value) and 0.000 at 1.5 m. Cloud at 4 s: mass 13.74, 1230 cells ≥ 0.05,
box (−4, 0, −5.75) → (1.25, 3.25, −2).

`column.png` is a weak image and should not be quoted as evidence: the plume is
in unlit corridor air, so it reads as a faint grey smudge near the column and
nothing more. Smoke is only visible where a light is on it — the same reason
`sequence-t10.png` shows a 133-unit cloud as barely anything, and the reason §6
puts its cloud under a fixture. The obstacle claim rests on the six probes.

The coarse column is here because it is alarming: the gameplay readback reports
**density 0.99 inside solid geometry**, since its 1 m × 0.25 m × 1 m box average
straddles a 0.56 m column. It also flattens the profile to 1.73 / 0.99 / 0.99 /
0.90 — the split is gone.

### 10. Determinism

`fluid-determinism.js` pins everything before the first sim step: paused rAF
loop, guards frozen, solver **and** every emitter reset (`SM.reset(true)` drops
the permanent wisps too), canisters cleared, one instant blob, fixed 50 ms dt,
then 40 steps and an FNV-1a hash of the raw fp16 density field.

| run | build | mass | peak | cells | centroid | checksum |
|---|---|---|---|---|---|---|
| 1 | A (`917c37f`) | 11.319137 | 9.320313 | 838 | (5.0034, 0.9755, 0.0034) | **`d9b49a13`** |
| 2 | A (`917c37f`) | 11.319137 | 9.320313 | 838 | (5.0034, 0.9755, 0.0034) | **`d9b49a13`** |
| 3 | B (`895a8fe`) | 11.319137 | 9.320313 | 838 | (5.0034, 0.9755, 0.0034) | **`d9b49a13`** |
| 4 | B (`895a8fe`) | 11.319137 | 9.320313 | 838 | (5.0034, 0.9755, 0.0034) | **`d9b49a13`** |
| 5 | B (`895a8fe`) | 11.319137 | 9.320313 | 838 | (5.0034, 0.9755, 0.0034) | **`d9b49a13`** |
| 6 | C (HEAD) — **re-checked in the closing pass** | 11.319137 | 9.320313 | 838 | (5.0034, 0.9755, 0.0034) | **`d9b49a13`** |

Six separate harness invocations across the three builds — four in parallel with
nine other Chromium instances, two alone in the foreground — identical in every
figure. Say what the hash covers, though: it is FNV-1a over the **density lane of
`scl[0]`**, not the whole solver state. Velocity, pressure, divergence, curl and
the **temperature** lane are not in it, so a nondeterminism confined to those and
not yet reaching density within 40 steps would pass all six invocations. Findings
11 below is the concrete case: a "warm" still came back with the same checksum as
the cold one, and what that proved was that density was unchanged — a weaker
statement than the one the hash was being read as making, and the reason branch
HEAD reports `fieldChecksum` (all four scalar lanes, temperature included)
alongside. `fieldChecksum` has no cross-run history yet; the table's figure is the
density lane, comparable across all six runs.

The six-run agreement is simultaneously the determinism result and the evidence
that the build differences described above touch nothing the solver computes for
density. The scenario also asserts the
protocol rather than only printing the hash — 40 steps taken, zero emitters still
packing, mass > 0, field finite — because a run that silently emitted nothing
would produce a beautifully stable checksum that means nothing.

A second, independent datum falls out of the suite for free: `fluid-canister.js`
at T = 2.0 s and `fluid-sequence.js` at its t = 2.0 s checkpoint are different
scenario files driving the same protocol, and both report mass **15.8350** and
density-lane checksum **`5f380f8a`** — identical across separate invocations and
separate scripts.

**What is *not* reproducible, and why it is out of scope.** A throw released from
the posed weapon hand is not. `canisters.throw(from, target)` is deterministic in
its arguments — the tumble is derived from the throw direction, there is no RNG —
but `player.muzzle().pos` and `player.flashlightOrigin()` are latched from the
character rig during `update()`, so they depend on the animation phase, which
depends on how many free-running settle frames the harness happened to render
before the scenario paused the loop. That is wall-clock, so it varies run to run.
Every determinism claim above therefore releases from a fixed world point
(`__throwCanister`'s optional `from`). `fluid-beam.js` deliberately does not — it
reads the real weapon pose, because it measures optical depth rather than
determinism — and it reproduces to 4 significant figures (τ = 3.3904, 3.3907,
3.3917 across three runs), not bit-exactly. Making the animated case
reproducible would mean pinning the character clock, which is the harness's
`__freezeClock` plus a rig-phase seed; it is out of scope here and the fixed-blob
checksum is the determinism test.

### 11. Pressure warm start: a hazard looked for and not found

`fluid-pressure.js`, render 320×200. Every wall is Neumann and the room is
closed, so the pressure Poisson system is singular — its solution is defined only
up to an additive constant — and `step` warm-starts each frame from the previous
frame's pressure. That pairing is a standing hazard: if the discrete right-hand
side were even slightly incompatible, Jacobi would feed the mismatch into the
null space, where it is invisible in the gradient right up to the point where
`p[c] − p[c−e]` loses its significant digits to a large common offset. Nothing in
the solver bounds it, and nothing was measuring it.

So: a sustained 4 m/s jet held for 60 s of game time — 1200 consecutive
warm-started solves, stirring 357,447 of the lattice's 369,956 fluid cells — with
the pressure field and the residual read out every 10 s.

| t (s) | steps | pressure mean | pressure spread | \|mean\|/spread | active cells | postMean (1/s) | relResid | meanRed |
|---|---|---|---|---|---|---|---|---|
| 10 | 200  | −0.000000 | 0.1398 | 0.000000 | 96,407  | 7.02e-4 | 8.23e-4 | 32.9 |
| 20 | 400  | −0.000000 | 0.1749 | 0.000000 | 349,782 | 5.88e-4 | 6.70e-4 | 98.1 |
| 30 | 600  | −0.000000 | 0.1781 | 0.000000 | 356,382 | 6.26e-4 | 6.14e-4 | 108.1 |
| 40 | 800  | −0.000000 | 0.1409 | 0.000000 | 356,856 | 6.27e-4 | 6.10e-4 | 110.8 |
| 50 | 1000 | −0.000000 | 0.1657 | 0.000000 | 357,384 | 6.51e-4 | 6.22e-4 | 107.7 |
| 60 | 1200 | −0.000000 | 0.1419 | 0.000000 | 357,447 | 6.30e-4 | 5.95e-4 | 111.5 |

**No drift, and the projection gets better rather than worse**: the mean stays at
zero to six decimals over 1200 steps, the spread stays bounded at 0.14–0.18, and
the residual *falls* from 8.2e-4 to 6.0e-4 while the reduction factor climbs from
33 to 112. The warm start is paying for itself.

There is a mechanism that explains it, but it is weaker than an earlier revision
claimed and it is **not** what retires the hazard — the 1200 steps above are.
`divergence` sums (vel[c+e]·e − vel[c]·e)/cell over fluid cells, so every interior
face enters twice with opposite signs and wall faces are zero by `wallFaces`. Those
*idealised* differences telescope to exactly zero (checked against all four cases:
both neighbours fluid, either one solid, both solid). What the solve is actually
handed does not: each cell's right-hand side is stored as
`fl((a−b)·inv + (c−d)·inv + (e−f)·inv)` in r32float, so the face value shared by two
cells does not cancel bit-for-bit between them. Compatibility holds **to rounding**
— order 1e-7 relative per cell — and Jacobi on a singular system shifts the mean by
−mean(div)/(2Σw) per iteration, 24,000 iterations over this run. The measurement
says that shift stays below the sixth decimal; the algebra does not say it must.

So the honest scope of the retirement: re-run this bundle if you add an inflow, an
outflow or a non-zero wall flux (which breaks the telescoping outright), and also
if you tighten the grid, lengthen the run well past 60 s, or store pressure or
divergence in anything coarser than r32float — those change the rounding this
result rests on.

### 12. `physics` selfTest

`window.__physicsSelfTest()` via `fluid-stats.js`: `pass: true`, **25 PASS,
0 FAIL, 5 INFO** over 30 lines — **re-checked on build C in the closing pass**
(§0), same counts, and it costs no frames because the self-test does not render.
Representative:

```
PASS  ray/rotated-obb  t=1.885786438 (expect 1.885786438) n=(0.707107,0.000000,0.707107)
PASS  drop/no-jitter  peak-to-peak=9.71e-17 m, residual |v|=0.00e+0 m/s
PASS  fixed-step/frame-rate-independent  30fps->y=0.060000000000  60fps->y=0.060000000000  144fps->y=0.060000000000  240fps->y=0.060000000000
PASS  energy/e=1.05 does diverge  unclamped e=1.05 gains 217.5% in 10 s — the clamp is not decorative
PASS  tunnelling/thin-wall  0.06 m body vs a 0.04 m wall: 12 m/s -> x=-1.100, 20 m/s -> x=-3.500, 34 m/s -> x=-7.696
INFO  sleep-cost  2 m drop, time to sleep — e=0.35:1.62s  e=0.60:2.70s  e=0.90:9.78s  e=0.95:17.78s  e=0.99:>30s
```

`parked/README.md` claims 26/26 assertions; the suite emits 25 PASS plus 5 INFO.
Nothing fails — the README's count is one out, worth a one-line fix by whoever
owns it.

### 13. Resource counts (counted, not estimated)

`FluidSim.resources()`, read back in `fluid-sequence.js`. Bind-group layout, one
group, compute-only visibility: 1 uniform buffer, 1 filtering sampler, 4 sampled
`texture_3d<f32>`, 1 read-only storage buffer, **3 write-only
`texture_storage_3d`**. These are now tallied from the layout's own entry
descriptors — an earlier revision returned the same five numbers as hand-written
constants while the docstring called them counted, so a fourth storage texture
would have kept reporting 3. The last count is the one that matters: the solver
uses 3 against a documented SwiftShader limit of 4, so init succeeds with one slot
spare. **That limit is sourced from `tools/headless/README.md`, not from a run** —
nothing in the tree reads `device.limits.maxStorageTexturesPerShaderStage`;
`gpu.ts` requests up to 8 and asserts ≥ 4, and the assert passing is the only
runtime evidence. The trace pass's own budget is untouched — the solver has its
own bind group and shares nothing but the density texture.

Solver-owned memory, 208 × 13 × 144 = 389,376 cells: `fluid-vel-0/1`,
`fluid-scl-0/1` and `fluid-curl` at 3,115,008 B each (rgba16float);
`fluid-prs-0/1` and `fluid-div` at 1,557,504 B each (r32float);
`fluid-occupancy` 389,376 B; `fluid-params` 1648 B. **Total 19.682 MB.** B2a's
`smoke-volume` interface texture (2.971 MB) is excluded — it is the volumetric
channel's, not the solver's. Occupancy bake: **19,420 solid cells** (5.0% of the
lattice); per-row solid counts
`[2085, 2085, 2070, 1877, 1599, 1242, 1242, 1242, 1106, 1218, 1218, 1218, 1218]`
from floor to ceiling. Profiler passes: `fluidAdvect`, `fluidPressure`,
`fluidScalars` — 3 of the frame's 26, all present by name.

Read row 12's `1218` as the deviation it is, not as a resolution artefact. The
bake **skips the ceiling slab**, so the top row carries only the same
floor-to-ceiling geometry the rows below it do. Had the slab been baked, row 12
would be *entirely* solid — 29,952 cells — because its top sub-lattice y-plane
sits at 3.208, inside the slab, and contributes 9 of the 27 hits against
`SOLID_MIN_HITS = 6`. That is the trade: honour the density contract's
`y >= 3.2` top wall and lose 25 cm of real air where ceiling-hugging smoke lives,
or keep the row fluid and let up to 5 cm of smoke sit inside an invisible slab.
This branch keeps the row fluid; see Below-100% item 17. The skip is now
geometric (a box confined to the top row and continuing above the lattice) rather
than keyed on `FLAG_NO_CAMERA`, and the change was verified inert by enumerating
all **532** static boxes of the level: both predicates select exactly the one
ceiling slab at y ∈ [3.2, 3.5], so the counts above are unchanged.

### 14. Scenario suite — every verdict

Every scenario in `tools/headless/scenarios/` that touches the fluid or the
volumetric channel, with the `ok` it returned. `run.py` fails the run on
`ok: false`, so these are asserts, not prose.

| scenario | verdict | what it asserts |
|---|---|---|
| `fluid-mass.js` (6 phases, 4 lanes) | **ok: true** | every phase's tuning reached the dispatch loop, across a 25× iteration spread |
| `fluid-jacobi.js` | **ok: true** | each requested Jacobi count, rounded as the solver rounds it, equals `lastJacobi` |
| `fluid-dt.js` | **ok: true** | per-step defect first order in dt, total loss zeroth order |
| `fluid-leak.js` | **ok: true** | per-step advection defect stays under 2% |
| `fluid-lifetime.js` × 2 | **ok: true** | mass finite and non-negative; no runaway with dissipation off |
| `fluid-stats.js` | **ok: true** | still air retains exactly 1 (an identity — §3); `physics` selfTest passes |
| `fluid-sequence.js` | **ok: true** | cloud wider than tall, thickest at the floor, field finite |
| `fluid-beam.js` | **ok: true** | solver transmittance reaches 0.5 inside the cloud |
| `fluid-column.js` | **ok: true** | column cell solid, zero density in it, smoke both sides |
| `fluid-canister.js` | **ok: true** | smoke present at T, exactly one canister live |
| `fluid-determinism.js` × 6 (2 A, 3 B, 1 C) ‡ | **ok: true (6/6)** | 40 steps, no emitters left packing, field finite |
| `fluid-still.js` × 2 modes | **ok: true (2/2)** | pinned size held, cloud clear of the noise floor, both signs present, underside shadowed, rise/fall matches the mode |
| `fluid-pressure.js` | **ok: true** | pressure finite; residual does not degrade over 50 s of sustained forcing |
| `vol-shot.js` × 7 modes | **ok: true (7/7)** | frame finite, not black, pinned size held |
| `vol-counters.js` | **ok: true** (run-level; returns a JSON string) | B2a's work counters still resolve |
| `vol-compare.js` | **ok: true** (run-level; returns a JSON string) | reference comparison completes untruncated |
| `crouch-matrix.js` | **ok: true** (run-level; returns no verdict) | A-track's; drives `settings.volumetric` on and off |
| `smoke.js` (cold start) | **ok: true** — **re-checked on C** | probe sees 33 lights, nav raster non-degenerate, 4 guards |
| `fluid-invariants.js` (new) | **ok: true** — **measured on D** | cold start, occupancy totals and per-row counts, the counted bind-group tally, `divergenceStats` idempotent on its `pre` figures, and an instant source surviving a frame that cannot deliver it (frozen clock and silenced) |

‡ This row previously read “× 4 (2 per build)”. §10 tabulates the prior
invocations individually — two on A, three on B — so the summary count was one
out; it is corrected here and carries the closing pass's sixth run on C. Only the
`smoke.js` and `fluid-determinism.js` rows were re-run in the closing pass (§0);
every other verdict in this table is prior-session.

`fluid-still.js`'s 2/2 is at the parameters §6 quotes. An earlier pair of runs at
a tenth of the plume density returned `ok: false` on the underside-shadowed
assert — transmittance 0.9998 at the top against 0.9546 at the underside, i.e. a
plume too thin to shadow itself. That is the assert doing its job rather than a
regression, and it is why §6 states the density it used: a bundle whose verdict
depends on the source strength has to say the strength.

`vol-shot.js` needed no porting off `__smokeTest`, and this is worth saying
plainly because the brief expected it to: all four of its blob call sites already
drove `__smokePuff`, ported in the branch's `e036953` salvage commit —
`git show c78e794:…/vol-shot.js | grep -c __smokeTest` is 4 and at `9940888` it
is 1, the surviving mention being the comment that explains the replacement.
What it *did* need was a verdict: it returned its own name, so the run passed
whatever it drew. It now weighs the delivered frame.

`vol-compare.js` is B2a's estimator-vs-reference check and it still holds with a
simulated field driving the density: relBias `defaults` −0.0245, `extinctionOff`
+0.0741, `noVolume` −0.2033 against a reference mean of 0.02352, 140 reference
frames and 40 per config, untruncated at 320×200. The shipped estimator with the
medium is the closest of the three to the Monte Carlo reference, which is what
B2a claimed.

`fluid-mass.js` with `noDissJ200` is the most expensive run in the suite (200
Jacobi dispatches × 160 steps at full lattice) and wants its own lane: it took
1142 s sharing the box with nine other Chromium instances. One run in the closing
matrix (`vol-compare`) failed a 30 s page-load timeout at load average 100 and
passed on its own; that is contention, not code.

## Findings

1. **A duplicated tuning field with a one-way per-frame sync is a
   measurement-destroying bug, not a style problem.** It cost this track its
   starting hypothesis: the two-iteration-count comparison that "proved"
   projection quality irrelevant had never dispatched two different counts. The
   structural fix is one owner; the guard that keeps it fixed is `lastJacobi`
   plus a scenario assert on it.
2. **A residual number is only as good as the operator that measures it.** The
   code claimed ~2e-3 relative residual. The whole-room mean was 4.6e-4 — but
   over 389,376 cells of which 2,645 were moving, so it was a measurement of
   still air. The active-cell figure was 2.5e-2, and the divergence the
   *advection* feels is 0.058/s, invisible to a 2h central-difference stencil.
   Four different numbers, one of them quoted in a comment as the solver's
   quality. A residual claim needs its operator, its filter and its units
   attached or it is decoration.
3. **Enforcing a boundary condition after a solve can be worse than not
   enforcing it.** The post-projection wall clamp looked defensible ("the solve
   is iterative and never exact") and it destroyed the solve's work.
   `rowMass[0] ≡ 0.0000` had been sitting in the readback all along. But be
   careful attributing the *unblocking*: row 0's own minus-y flux is still
   zeroed by `wallFaces`, and what lets it receive mass is the MAC face→centre
   reconstruction — `velCentre` averages in the face between rows 0 and 1,
   which is not a wall. Moving the clamp inside the solve while keeping
   collocated cell-centre velocities would have left row 0 exactly as
   unreachable. The lesson survives; the mechanism is the reconstruction.
4. **A mean over an empty set is not zero.** `activeRelResidual` read `0.000000`
   whenever no cell was moving, which is indistinguishable from a perfect
   projection. It now returns `null`.
5. **Order-of-error is the question that ends an optimisation argument.** "The
   gather is not conservative" was true and led nowhere for a session. Measuring
   the order (§4) settled in one run that neither smaller steps nor more
   iterations can help, and turned a vague worry into a bounded, quoted property
   with a named fix.
6. **The canister cloud's spread is mostly numerical diffusion, not flow.** At
   t = 2 s of the sequence no cell in the room exceeds 0.05 m/s, yet the cloud's
   footprint grows from 3.0 m to 5.25 m over the next 8 s. With `dissipation = 0`
   the peak still falls 23 → 2.46 over 20 s purely by spreading. Visually this is
   fine — thinning smoke is what smoke does — but nobody should reason about the
   cloud's motion from its growth rate.
7. **The gameplay readback is not the renderer's field, and the gap is 2.4× in
   optical depth.** On the same beam: solver τ = 3.392 (T = 0.034), coarse
   readback τ = 1.434 (T = 0.238), peak smoothed 5.3× (88.8 → 16.7, both from the
   scenario's 10 cm march — §8's printed table is decimated to 0.5 m and tops out
   at 16.3) and the cloud smeared over ±1.5 m along the beam. Neither figure is
   the rendered medium either: both are smoke-only, and the fog adds ≈0.17 of τ
   over this beam (§8). A guard LOS threshold tuned against the
   coarse field will let guards see through smoke the player cannot. Worse, the
   column probe shows the coarse field reporting density 0.99 **inside a solid
   column** where the solver holds exactly 0 — a 1 m × 1 m box average straddles
   a 0.56 m column, so smoke leaks across geometry in the gameplay view only.
   Both are for B3 to set thresholds against; neither is a solver defect, and
   neither is fixable without a finer coarse grid or a solid-aware downsample
   (the probe kernel box-averages without consulting the occupancy field — that
   is the cheap fix if B3 wants it).
8. **A thrown canister is not a placeable emitter.** The first version of the
   beam scenario threw a canister at a point on the beam axis; it bounced and
   rolled 2.8 m away, and the beam clipped the cloud's upper edge for τ = 0.047.
   The measurement was reported as a failure by its own assert rather than as a
   beam that "didn't stop", which is the reason to write asserts into scenarios.
9. **The first version of the column probe read exactly zero at all six points**
   while the field carried a 13-unit plume, because it probed the
   quarter-resolution gameplay readback. Probing the solver's own lattice is both
   authoritative and cheaper to reason about; the coarse value is reported
   alongside for the B3 comparison.
10. **A scenario's `renderer.resize()` was silently reverted, and the mechanism
   was a race, not a duration.** `frameBody` carries a deferred resize gated on a
   pending ResizeObserver notification; `observe()` fires once at startup, so
   whether a scenario's own resize survived depended on whether that pending
   notification had already been consumed before the scenario ran. A minimal
   probe (pause, resize, step 200 frames) held its size and therefore
   "disproved" the effect. `__pinResolution` re-asserts the pin inside the same
   gate; the four bundles that quoted 640×328 now quote the 384×240 or 448×280
   they asked for.
11. **An attack envelope silently ate a one-frame source's temperature.** An
   instant source is delivered whole on its only frame, where the attack
   envelope is exactly 0; density escaped because the instant branch overwrote it
   past the envelope, and temperature did not. Every `__smokePuff` blob was
   therefore cold, which is why the first attempt at a "warm" still produced a
   field whose *density* was identical to the cold one — the checksums matched,
   which is what gave it away. Note the limit of that evidence: the checksum
   hashes the density lane, so it could not have seen the dropped temperature
   directly. It saw density unchanged, which is a weaker statement that happened
   to point at the right bug. `fieldChecksum` (all four scalar lanes) exists now
   so the next one of these is caught by the temperature it drops, not by
   inference.
   **And a second instance of the same bug class was still in the file**: the
   instant branch set `age = life` *before* the pack guard, so a source silenced
   or delivered on a dt = 0 frame was consumed without ever injecting — and
   `__renderStill` under `__freezeClock(true)` is exactly a dt = 0 frame, which
   the same scenarios use to hold a camera aim. Fixed on branch HEAD: an
   undelivered instant source waits instead of ageing.
12. **`parked/README.md`'s "26/26 assertions" is one out**: the suite emits 25
   PASS and 5 INFO lines, all passing.
13. **`run.py` leaks its Chromium profile directory, and `/tmp` here is RAM.**
   Each run creates `/tmp/twopointfive-headless-<pid>` and never removes it. This
   track's matrices left **1099 of them holding 12 GB** — on a tmpfs, so 12 GB of
   resident memory doing nothing. A one-line `shutil.rmtree(udd, ignore_errors=True)`
   in `run.py`'s `finally` would fix it; anyone running matrices before then should
   `rm -rf /tmp/twopointfive-headless-*` afterwards. Nothing failed because of it
   — 14 GB was still free — but a longer campaign on a smaller box would.
14. **`run.py` is silent until it finishes, and I read that as death.** The two
   longest runs (`fluid-dt` 1982 s, `fluid-pressure` 2197 s) write nothing to
   their log but a Playwright deprecation warning until the final result blob, so
   for half an hour each looked exactly like a killed process: an empty log, no
   JSON, and — because Chromium's children outlive a `terminate` that never came —
   a `ps` count that flickers. I concluded they were dead, relaunched them three
   times, and then killed some of my own relaunches; both original runs completed
   normally and their numbers reproduced the earlier matrix exactly. Cost: about
   40 minutes and a spell of two writers racing for one output path. The corrected
   habits are cheap: **give every relaunch a distinct output path** so a race
   cannot silently mix results, and **judge liveness from the process, not the
   log** (`ps -ef | grep '[h]eadless/run.py'`, or have the driver stream a
   heartbeat). `/sys/fs/cgroup/memory.events` did record `oom_kill 1` with
   `memory.peak` at the 220 GiB limit during the ten-lane matrix, so at least one
   process on this shared box was killed for memory that day — but I cannot
   attribute any specific run to it, and I am not going to claim otherwise.
15. **`__freezeClock(true)` does not freeze the frame it is called before.** dt is
   `now - prev` clamped to 50 ms, and the first frozen frame is fed `frozenClock`
   against the `prev` a synthetic `__renderStill` frame left behind. Synthetic time
   advances 50 ms per frame while real time advances ~1 s per traced frame, so that
   difference is large and positive: the first frozen frame takes a **full 50 ms
   step** — one sim step and one camera damp — and dt is exactly 0 only from the
   second. Every scenario that freezes to hold a pose pays one step for it (§6
   quantifies it for the stills: a field 0.05 s older than stated and an aim one
   damping step off the subject). The habit that fixes it is one settling frame
   between `__freezeClock(true)` and whatever the freeze is meant to protect, which
   is what `fluid-invariants.js` does and what its header explains. Found because
   an assert of mine was wrong about the mechanism and the run disagreed with me —
   which is the whole argument for asserts over prose.

## Merge notes

- **This branch conflicts with `claude/campaign`, in one file, measured.**
  `git merge-tree --write-tree claude/campaign claude/fluid` reports
  `CONFLICT (content): Merge conflict in src/engine/renderer.ts` and nothing else.
  Trunk's only source change since the merge base (`2f6326b`) is `e475af4`, which
  deletes `const MAX_PUFFS = 8;` and leaves the comment around it; this branch
  deletes the same line *and* rewrites that comment block. **Resolution: take this
  branch's text.** Trunk's other change is `docs/campaign/STATUS.md`, untouched
  here. The two things a reviewer would check are intact: `UNIFORM_SIZE` is still
  944, and the trace layout still declares `binding: 14` texture / `binding: 15`
  filtering sampler and binds `smokeVolumeView` + sampler to them.
- **The 256-byte dead-puff hole (bytes 592–847) now has no TS-side constant.**
  Both sides of the merge delete `MAX_PUFFS` from `renderer.ts`; the WGSL
  `MAX_PUFFS: u32 = 8u` and the `_deadPuffPosR`/`_deadPuffParams` arrays are what
  actually hold the offsets. `renderer.ts`'s comment names those fields so a grep
  finds both sides — do not "tidy" the WGSL arrays without moving every later
  field.
- **`profiler.ts`'s `MAX_PASSES` goes 32 → 48.** Shared with every other track:
  it is a headroom bump for the worst-case frame (~29 passes with the volume
  chain, radiosity solve, light-volume rebake and the three fluid groups all in
  one frame), costing 16 bytes of readback per slot. Nothing depends on the old
  value, but it is a shared file and belongs in a merge note.
- **`RenderSettings` loses `fluidJacobi`.** No settings-revision bump is needed:
  `loadInto` skips any stored key `DEFAULT_SETTINGS` does not own
  (`hasOwnProperty` on defaults), so a stored `fluidJacobi` is dropped silently.
  Anyone rebasing a branch that reads `settings.fluidJacobi` should point it at
  `renderer.fluid.tune.jacobi`.
- **Uniform bytes unchanged: 1648** (`(28 + 32 × 12) × 4`) — the operator fix
  added no uniform fields, so nothing in the append-order rules is disturbed.
- **Bindings unchanged**: 1 uniform + 1 sampler + 4 sampled `texture_3d` + 1
  read-only storage buffer + 3 storage `texture_3d`, one bind group, compute
  only. Storage textures per stage stay at 3 of SwiftShader's 4.
- **Profiler pass count unchanged: 3**. The Jacobi default halving 40 → 20
  removes 20 dispatches per step from inside `fluidPressure` without changing the
  pass count.
- **`fluid-scl-1` gained `COPY_SRC`** so `advectionBalance` can weigh the
  pre-advection field, and **`fluid-prs-0/1` gained it** for `pressureStats`.
  Usage flags only.
- **`main.ts` gains one debug hook and one branch in `frameBody`**
  (`__pinResolution`). It is the harness's surface rather than this track's, and
  it is flagged here because it changes what every *other* track's scenarios can
  rely on: a scenario that pins its resolution now keeps it. Existing scenarios
  are unaffected — the gate's behaviour with no pin is what it was. The hook
  **throws on a half-specified pin** (`__pinResolution(384)`) rather than falling
  back to the canvas size, because a silent fallback would reintroduce exactly the
  bug the hook exists to prevent (Findings 10) behind an easier trigger; `null`
  remains the way to release. This is the whole of `git diff B..C -- src/`.
- **The density-interface contract's *mechanics* are untouched, its top-wall rule
  is not.** `writeVolume` still writes R = density with G/B/A zero over B2a's
  grid, dims, origin and cell, and no B2a-owned file changed. But the contract
  also says "treat y >= 3.2 as your solid top wall", and B2b does not: the
  lattice's top face at 3.25 is its wall and row 12 stays fluid. An earlier
  revision of this report claimed the contract was untouched full stop, and
  `renderer.ts` claimed compliance. Both now name the deviation; Below-100%
  item 17 has the reasoning.
- **Anyone porting the solver to a coarser lattice**: two invariants, and they are
  not the same one. `wallFaces` must be applied wherever velocity is written, or
  the divergence pass is handed a right-hand side the Neumann pressure cannot
  satisfy and the projection stops being a projection. Separately, the velocity
  store must be *read* as a face field and reconstructed to cell centres for the
  traces (`velCentre`/`velCentreB`) — applying `wallFaces` everywhere while
  keeping collocated cell-centre velocities reproduces `rowMass[0] ≡ 0`, because
  row 0's own minus-y face is a wall and the centred average is what reaches past
  it. Both are load-bearing.
- **Confinement scales by `min(cell)`, not `cell.x`.** `setScale` changes x and z
  and keeps y, so scaling by `cell.x` made the halved lattice a different
  simulation (2× the confinement strength) rather than a coarser one. Bit-identical
  at `scale = 1`. Anyone adding an anisotropic lattice should re-read that line.
- **Anyone touching `advectScl`, the projection operators, or `dt`**: the numbers
  in §3–§5 are load-bearing for the `jacobi` and `dissipation` comments, and
  `fluid-dt.js` asserts the *order* of the mass error. Making advection
  conservative will trip that assert — correctly. Update the bundle, don't
  loosen it.

## Validate on the Mac

This box has no GPU. Everything above is either a machine-independent measurement
(masses, checksums, optical depths, counters) or a reading of a SwiftShader-rendered
still; none of it is a judgement of how the smoke *looks* in motion or what it
costs in ms. Both of those are yours. Two halves: five things to look at, and the
`__bench` comparisons that produce the perf number.

### Five things to look at (`npm run dev`, Chrome)

Each one names what should be true, the measured figure it should agree with, and
what "wrong" looks like — so a disagreement is diagnostic rather than a vague
disappointment.

1. **Buoyant plume vs sinking bank — the contrast, not either one alone.** Shoot a
   ceiling fluorescent (pistol, slot 2): a thin warm plume should climb from the
   fixture, spread under the ceiling, and be brightest *high*, up in its own light.
   Then press **4** and throw a canister: cold smoke should pour *down* and settle
   into a wedge on the floor. Same solver, opposite vertical motion, because
   buoyancy rides temperature and weight rides density (§6) — that contrast is the
   check. Prior-session: the cold puff's centroid falls 1.45 → 0.43 m in 1.5 s, the
   warm plume's rises 0.50 → 1.17 m in 4 s. **Do not expect the fixture's own plume
   to match those numbers, and do not read a slower climb as a defect.** The shot's
   source is 10× smoulder's density, 2.9× its emitting volume and 1.5× its
   temperature (§6) — ≈29× the injected mass rate into a hotter, larger sphere, so
   it is a different buoyancy regime, not a brighter one. What transfers is the
   *sign and shape*: climbs, spreads under the ceiling, brightest high. The
   quantities do not transfer, and this report has no measurement of a real
   smoulder plume's rise.
   **Wrong looks like:** both rise, both sink, or the plume climbing and then
   stopping at a flat horizontal line below the ceiling.
2. **Self-shadowing.** Get a cloud under the brightest practical you can find (the
   conference room's warm fluorescent) and compare its top to its underside. Top
   lit, underside dark, gradient continuous with no edge anywhere. §6 measures the
   underside receiving **13% of the light the top receives**, lamp-to-floor
   τ = 2.25 in the smoke alone (12.3% and τ = 2.33 once the fog is in it — §6), at
   canister strength. **Wrong looks like:** a uniformly bright blob —
   which means the march is not accumulating extinction toward the light — or any
   visible boundary to the medium.
3. **Canister burst.** Press **4**, aim, left-click. Arc, bounce, then roughly a
   second of nothing, then 8 s of emission — the pause is a gate, not a hitch.
   There are two gates and this report cannot tell you which one fires:
   `canister.ts` starts on `body.sleeping || fuse >= 1.4 s`, so emission is
   guaranteed by 1.4 s regardless, and §7's 1.0–2.0 s bracket is consistent with
   either. That also means "emission starting on impact" is *not* a falsifiable
   wrong — the fuse alone rules it out. The falsifiable part is the shape: the
   cloud should end up clearly wider than tall and thickest at the floor (§7
   measures 5.25 × 1.50 × 6.50 m at t = 10 s, 3.5–4.3× wider than tall, centroid
   falling 0.50 → 0.21 m). **Wrong looks like:** a ball hanging in the air, a
   delay long enough to read as a bug (much past 1.4 s), or smoke inside the
   support column or through a wall.
4. **Beam through smoke.** **F** for the torch, walk into the cloud. The beam
   should *end* — a bright soft ball of lit smoke a few metres out that throws
   light sideways onto the nearest panel or desk, with the corridor beyond staying
   as black as it was. §8 measures τ = 3.39 through canister-strength smoke, and
   the fog you are also looking through takes it to τ ≈ 3.56: **≈2.9% of the beam
   survives** on screen (3.4% is the smoke alone), half of it gone by 3.8 m.
   **Wrong looks like:** a beam
   that is merely dimmer but still reaches the far wall (extinction off, or σ_t too
   low), or a cloud that brightens uniformly no matter where the beam points.
5. **Guard sight blocked by a cloud — read the caveat before you judge it.** Throw
   a canister between yourself and a patrolling guard and watch the DETECTION
   meter: suspicion should stop climbing while the cloud is between you. This is
   the item the headless suite cannot settle and the one most likely to
   disappoint, because guards sample the **coarse gameplay readback**, not the
   solver lattice the renderer's medium is built from. Findings 7 measures that readback under-reporting
   optical depth by **2.4×** (τ 1.434 against 3.392) and smoothing the peak 5.3×,
   so the honest prediction is that a guard sees *further* through smoke than your
   eye says it should. If that is what you see, the thing to change is B3's
   threshold, not the solver. While you are there: stand a cloud against the
   corridor support column and check a guard does not lose you *through* the
   column — the coarse field reports density 0.99 inside solid geometry where the
   solver holds exactly 0.

Two things deliberately not on the list. A hero shot: a 4 m cloud seen from 12 m
under the house three-quarter camera is a bank, not a silhouetted puff, so frame
it from lower down if that is what you want to see (§6). And smoke in unlit air:
`column.png` is a grey smudge because nothing is lighting it, which is correct —
judge the medium only where a light is on it.

### The perf number: which `__bench` comparisons give it

SwiftShader cannot answer the ms question at all. `__bench(n)` returns a JSON
string; the fields that matter are `wallMsPerFrame` and the per-pass `gpu` map,
which carries `fluidAdvect`, `fluidPressure` and `fluidScalars` by name. Four
comparisons, in the order that answers the questions:

| # | comparison | what it decides |
|---|---|---|
| 1 | `fluidSim` **on vs off** | **the headline number** — what the whole solver costs as a share of frame time. Quote this one. |
| 2 | jacobi **4 / 10 / 20 / 40 / 80** | the tuning decision. Linear in the count → `fluidPressure` is bandwidth-bound and the default could go back to 40 nearly free (the residual keeps falling as ~1/N, §2). Sublinear → the dispatches are launch-bound at this grid size, so drop toward 10; §3 says 8 iterations retain within 0.24% of 200, so a higher count buys only a smaller residual. |
| 3 | `setScale(1)` **vs** `setScale(2)` | whether the halved lattice is worth keeping as a quality preset. It quarters the solver's cell count while the interface texture the tracer samples is unchanged. **Read the two notes under the script before running this one**: `setScale` reallocates every field zeroed, and the confinement-strength bug that made scale 2 a different simulation was only fixed on branch HEAD. |
| 4 | empty room **vs** a settled thrown canister | the volumetric march's cost with a real medium in front of the camera — the part the pressure solve does not change, and the part that scales with what is on screen. |

The script below runs all four in order.

```js
const ms = async (n = 60) => {
  const b = JSON.parse(await __bench(n));
  return { wall: b.wallMsPerFrame, fps: b.fps, gpuTotal: b.gpuTotalMs,
           advect: b.gpu.fluidAdvect, pressure: b.gpu.fluidPressure,
           scalars: b.gpu.fluidScalars, frames: b.frames, truncated: b.truncated };
};

// 1. What the solver costs at all, at the pinned bench pose.
__settings.fluidSim = true;  console.table([await ms()]);
__settings.fluidSim = false; console.table([await ms()]);
__settings.fluidSim = true;

// 2. Jacobi cost curve. 20 is the measured default; 4 and 80 bracket it.
const curve = [];
for (const j of [4, 10, 20, 40, 80]) {
  __fluid.tune.jacobi = j;
  curve.push({ jacobi: j, ...(await ms()) });
}
__fluid.tune.jacobi = 20; console.table(curve);

// 3. Full lattice vs halved in x/z (the interface grid never changes).
//    setScale reallocates every field ZEROED and resets `steps`, so each arm
//    has to grow its own cloud or both arms bench an empty room — and the
//    second setScale would destroy the field comparison 4 needs.
const atScale = async (s) => {
  __fluid.setScale(s);
  __smoke.reset(true); __canisters.reset();
  __throwCanister(-8, -6);
  await __renderStill(60, 50);            // let it land and pour
  return { scale: s, ...(await ms()) };
};
console.table([await atScale(1), await atScale(2)]);
__fluid.setScale(1);

// 4. With a real cloud in the room rather than empty air — the pressure solve
//    does the same work either way, but the volumetric march does not. Run this
//    AFTER 3, never before: 3's setScale calls would zero the field.
__smoke.reset(true); __canisters.reset();
__throwCanister(-8, -6); await __renderStill(60, 50); console.table([await ms()]);
```

Two things about that script worth knowing before you read its numbers.
**`setScale` re-bakes occupancy synchronously on the main thread** — 389,376 cells
with a BVH query each — so each call is a one-off hitch inside the bench, not a
frame cost; `__bench`'s own frame window should start after it, which the awaited
`__renderStill` above ensures. And **the "any visual cost is resampling blur"
claim only holds on branch HEAD**: before it, `setScale(2)` doubled `cell.x` while
the confinement force scaled by `cell.x`, so the halved lattice ran at 2× the
vorticity-confinement strength — a different simulation. It now scales by
`min(cell)`, which is unchanged at either scale.

One prior expectation worth writing down before you run it, so the result can
contradict something: `fluidPressure` should dominate the solver's share, because
20 dispatches of a 7-point stencil over 389k cells is a bandwidth-bound job on
paper, and the other two passes are one dispatch each. If it does not dominate, or
if run 2's curve is flat, the solver is launch-bound rather than
bandwidth-bound at this grid size and the reasoning behind the jacobi default
(§2) needs redoing on that basis.

## Below 100%, with reasons

1. **The advection scheme is not conservative, and this branch does not make it
   so.** §4 measures what that costs and shows the two cheap levers cannot pay
   it: the per-step defect is first order in dt, so refining the step is
   zeroth-order useless, and the projection's residual is 280× too small to be
   the cause. A conservative scheme (fixed-point-atomic scatter, or a
   flux-limited MacCormack/BFECC pass) is the real fix and was not attempted.
   **No global renormalisation was added** — evaluated in §4 with its arithmetic
   and rejected: it restores the integral, which is not what fails.
2. **Vorticity confinement costs 21.5 points of retention** — 0.93278 without it
   against 0.71758 with it, over 8 s, and it roughly 2.5× the per-step defect
   (0.729% against 0.292% at t = 1.5 s). Cell-scale velocity structure is exactly
   what the confinement exists to create and exactly what the gather handles
   worst. It is kept: it is what makes the medium read as smoke. Anyone who wants
   the mass back knows the knob and the price.
3. **Velocity advection carries a half-cell offset.** With velocity read as a
   face field, `advectVel` still traces from the cell centre and samples the face
   field through the linear sampler, so the velocity it transports is interpreted
   half a cell off. O(h) in a scheme that is O(h) anyway, and it is not the
   mass-critical pass: `advectScl` traces scalars, which genuinely live at cell
   centres, along `velCentreB`'s centred reconstruction of the projected face
   field. The residual non-conservation §4 measures is the gather's own, not this
   offset's. Fixing it means three separate traces in `advectVel`, i.e. 3× that
   pass.
4. **`curl` and the confinement force keep the same half-cell offset**, for the
   same reason: the vorticity magnitude is an aesthetic driver, and doing it
   properly costs 4× the texture loads on a hot path.
5. **Buoyancy is applied to a face component from cell-centred scalars.** A
   strict MAC scheme would average the two cells sharing the face. Sub-cell
   offset in where lift acts; magnitudes unchanged.
6. **No timings, and therefore no performance verdict at all.** SwiftShader ms
   are not measurements. The three fluid passes are present and named in the
   profiler (3 of the frame's 26), which is all this box can say. Whether the
   solver is affordable is entirely open until comparison 1 in Validate on the
   Mac is run; nothing in this report should be read as evidence that it is.
7. **The gameplay readback under-reports optical depth by 2.4×** and reads
   nonzero density inside solid geometry — see Findings. Not fixed here because
   it is B3's threshold to set and the smoothing is inherent to a
   1 m × 0.25 m × 1 m coarse cell; it is *measured* so B3 can set thresholds
   knowingly. Its figures also vary at the 1% level between runs (1.7338 here
   against 1.7456 earlier for the same probe) because the readback lands a
   variable number of frames behind.
8. **`fluid-beam.js` reproduces to 4 significant figures, not bit-exactly**, and
   deliberately so: it releases from the real weapon pose because it measures
   optical depth. τ = 3.3904 / 3.3907 / 3.3917 across three runs. §10 says why
   the animated-source case cannot be bit-exact and why that is out of scope.
9. **`activeSpeed` is one threshold, not a curve.** The residual instrument
   filters at 0.05 m/s. It is a parameter (`divergenceStats(activeSpeed)`), but no
   sensitivity sweep over it was run; the canister scene has no cells above it at
   all at t = 2 s, which the instrument now says out loud instead of reporting a
   zero.
10. **The suite's numbers come from three builds**, differing by a readback
   method, a usage flag, the instant-source envelope fix, the `__pinResolution`
   argument check and comment text (the exact diffs are in Verification's
   provenance note). §10's unchanged checksum across all three, and §4 reproducing
   to every digit across A and B, are the evidence that the differences are inert;
   a single-build sweep would be cleaner and costs another 40 minutes of matrix,
   which the closing pass's compute budget did not have.
11. **Three scenarios pass at run level only.** `vol-compare.js`,
   `vol-counters.js` and `crouch-matrix.js` return JSON with no `ok` field, so
   what passes is the harness's own gate: no WGSL, validation, device or page
   error, and the renderer came up. They are A- and B2a-owned; `vol-shot.js` was
   the one in reach and it now weighs the frame it delivers. Giving the other
   three real verdicts means deciding what their numbers *should* be, which is
   their owners' call.
12. **The still shots use a canister-strength blob, not `vol-shot`'s.** Measured
   at roughly `vol-shot.js`'s strength (injected 40, peak 12 after two seconds),
   the lamp-to-floor optical depth is 0.82 instead of 2.25 and the image is a
   faint veil over a desk — a physically correct picture of almost nothing. §6's
   blob is 220 and the warm plume is ten times smoulder's density, both inside the
   range the canister reaches in play (peak 138 in §7) but above what a
   smouldering fixture makes. The thin case is measured too and quoted in §6 for
   contrast, so nobody has to take the strong shot as typical.
13. **Most of the suite was not re-run at the close.** The closing pass had a
   budget of three short foreground runs, and §0 says exactly which. They are
   bit-exact and they are the right three — a clean cold start, a bit-stable
   conservation control, and a determinism checksum that would have moved if any
   solver arithmetic or dispatch order had drifted — but they exercise storage,
   boundary zeroing and reproducibility, **not** transport. The transport numbers
   (§3–§5), the rendered evidence (§6), the canister sequence (§7), the beam
   (§8), the column probes (§9), the warm-start sweep (§11) and the resource
   counts (§13) are taken on the prior session's word. Anyone who wants them
   re-derived should budget the hours: `noDissJ200` is 1142 s on its own and
   `fluid-pressure` 2197 s.
14. **Every visual verdict in this report is a reading of a SwiftShader still, by
   me.** §6's "soft, self-shadowed, low-lying bank" and §8's "the beam ends in a
   ball of lit smoke" are honest readings backed by numbers measured off the same
   frames, but a still is not motion and software rasterisation is not the target
   pipeline. Nothing here has been seen moving on a GPU. That is what Validate on
   the Mac is for, and its five items are ordered so the two most likely to
   disappoint — buoyancy and guard sight — come first and last.
15. **`__pinResolution`'s new failure path is unexercised.** Branch HEAD makes a
   half-specified pin throw instead of silently releasing to the canvas size
   (build C, Merge notes). Typecheck passes and all 17 call sites in
   `tools/headless/scenarios/` pass both arguments with a non-zero height
   (`grep -o '__pinResolution([^)]*)'`), so nothing regresses, but no scenario
   deliberately calls
   `__pinResolution(384)` to confirm the throw fires — it is a one-line guard
   verified by inspection, not by a run.
16. **`run.py` still leaks its Chromium profile directory** (Findings 13). Not
   fixed here: it is the harness's file, shared with every other track, and this
   pass's budget went to verification rather than to touching shared tooling. The
   closing pass's own three profile directories were removed by hand; anyone
   running a matrix before the one-line fix lands should
   `rm -rf /tmp/twopointfive-headless-*` afterwards, and remember `/tmp` here is
   RAM.
17. **The solver declines the density contract's `y >= 3.2` top-wall rule.** The
   contract (top of pathtrace.wgsl's volumetric section) tells consumers to treat
   y >= 3.2 as a solid top wall. B2b's wall is its lattice's top *face* at 3.25,
   and row 12 (y 3.00–3.25) is fluid. The reason is resolution: 0.25 m cells cannot
   tile 3.2, the ceiling slab overlaps only the row's top 5 cm, and baking it would
   make the whole row solid (§13) — deleting the 25 cm layer where ceiling-hugging
   smoke lives, which is one of the two behaviours this track exists to produce.
   The cost is real and bounded: up to 5 cm of smoke can occupy space inside the
   invisible ceiling slab, `densityStats` counts that mass, `solidRow[12]` calls
   those cells fluid, and camera rays — which pass through the slab by design —
   march it (~1.5% of a floor-to-ceiling column). So a ceiling-hugging plume's
   "retained" figure includes up to one cell-layer of mass sitting nominally inside
   geometry. Sub-cell in magnitude, one flag in decision, and now flagged in three
   places (`renderer.ts`, `fluid.wgsl`, `solidRow`) instead of none.
18. **The determinism checksum covers the density lane, not the field.** §10's
   `d9b49a13` hashes R of `scl[0]`. Velocity, pressure, divergence, curl and
   temperature are outside it, so a drift confined to those and not yet reaching
   density within 40 steps would pass. Branch HEAD adds `fieldChecksum` over all
   four scalar lanes (temperature included) and `fluid-determinism.js` reports it,
   but it has **no cross-run history** — this pass's one permitted headless run
   went to the cold start, so the wider hash is an instrument shipped unexercised
   across invocations. Velocity determinism is still unmeasured by any hash.
19. **There is no measured floor under the transport figures.** `stillAir` was
   being read as that floor and it is an arithmetic identity (§3): zero forces on a
   cold blob means velocity is identically zero, so nothing exercises fp16 round
   trips under motion or solid-cell zeroing at all. A real storage control would
   need a blob given a uniform velocity with the projection and forces off, so the
   gather runs at non-integer offsets against a wall, and it needs a run this pass
   did not have. Everything §3–§5 attributes to "transport" is transport plus
   whatever storage costs, and that split is unmeasured.
20. **`curl` does not guard solid cells, and the confinement force reads across
   them.** Every other kernel early-outs on `solidAt(c)`; `curl` does not, so a
   solid cell is assigned a vorticity differenced from its fluid neighbours'
   velocities, and `curlMagAt` clamps out-of-range reads instead of masking them —
   so the confinement gradient at a cell beside an obstacle has one endpoint
   *inside* the obstacle. The plume gets a spurious tangential kick exactly at the
   geometry §9's six probes characterise. **This is the one place the shipped code
   is knowingly wrong.** The fix is one `solidAt(c)` early-out in `curl` plus a
   solid test in `curlMagAt`, and it is not applied here because it changes the
   field wherever flow touches geometry: §3–§5's mass tables, §7's sequence, §9's
   probes and both determinism checksums would all have to be re-derived, and this
   pass's compute budget is a single 60 s headless run. Anyone with GPU time should
   apply it and re-run the suite; the numbers here describe the code as it ships,
   which is the reason it was left alone rather than half-changed.
21. **The occupancy bake is synchronous on the main thread.** 389,376 cells, one
   BVH query each, and `setScale`/`reset` re-run it. Not a correctness issue: a
   startup hitch, and a mid-bench hitch in Validate's comparison 3, which the
   bench script now accounts for. Moving it off-thread or caching per lattice was
   out of scope.
