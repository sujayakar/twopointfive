# Track B2b — GPU smoke solver: the mass leak, found, fixed, and bounded

Branch `claude/fluid`, base `9940888` (the solver, sources, canister and
occupancy as received for review).

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

## What changed and why

1. **One owner for solver tuning.** `fluidJacobi` lived in `RenderSettings` too
   and was synced into `tune` every frame, so a sweep measured one count N times.
   `tune` owns it now; `lastJacobi` records what the dispatch loop ran.
2. **Divergence, gradient and Laplacian are one consistent triple.** Velocity is
   differenced as a staggered field, composing to the Laplacian `jacobi` solves.
   Before, the pressure it converged to could not null the divergence measured.
3. **The wall condition moved inside the solve** — zero flux written wherever
   velocity is written, not clamped after projection, which had left the lowest
   fluid row of the lattice unable to receive any mass at all.
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
   measured noise floor), `fluid-pressure`; `vol-shot` now weighs its own frame.

## How to see it

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
```

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

**Build provenance.** Bundles 1–3, 5, 7–10 and 12–14 were measured on
`917c37f`; bundles 4, 6, 11 and the determinism re-confirmation on the final
commit. `git diff 917c37f..HEAD -- src/` is one readback method
(`pressureStats`), one texture usage flag (`COPY_SRC` on the pressure pair),
the instant-source envelope fix, and comment text — no kernel, no dispatch, no
tuning value. The envelope fix is inert for everything measured: `puff()` is the
only instant source in the game and it passes no temperature, so the packed
uniform is byte-identical either way. §10 shows the determinism checksum
unchanged across the two builds, which is the evidence rather than the claim.

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
41, 81 and 121 (`fluid-stats.js`) — the control that fp16 storage and solid-cell
zeroing are lossless, so every loss below it is transport. († the pre-fix
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
5.01 over 8 s while cells above the 0.05 visibility threshold go 176 → 641 — the
same mass spread over 3.6× the volume). Renormalisation multiplies an
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
- **5.2 s to 1/e of what was injected** — the number the game actually shows,
  because the front-loaded transient offsets the whole curve: the actual/modeled
  ratio is 0.765 once the transient ends at 3 s and stays in the 0.70–0.76 band
  out to 20 s, so mass tracks roughly `0.76 × exp(−0.13 t)` rather than
  `exp(−0.13 t)`.

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
fixture, `MODE = "warm"`: a sustained warm source (smoulder's shape and heat at
ten times its density) at (−20, 0.5, 1), left running for 4 s. The field at
capture is the opposite of the blob's in every respect that matters:

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

The two zero rows are correct behaviour, not a gap: a canister only starts
smoking once its rigid body **sleeps** (`canister.ts`, `emitAge < 0` until
`body.sleeping`), so at 0.5 s and 1.0 s it is still bouncing. `emitting` brackets
the onset between 1.0 s and 2.0 s.

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

On the field the renderer marches: column integral 67.83, **τ = 3.392**,
transmittance out **0.0337**. Half the beam is gone by 3.8 m and 90% of it by
4.2 m — the beam is not merely dimmed, it is extinguished inside a 0.4 m span of
the cloud, and **3.4% of it survives** to light anything beyond. The gameplay
readback on the same ray gives τ = 1.434 (T = 0.238) with the peak smoothed
5.3× — see Findings.

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
| 1 | `917c37f` | 11.319137 | 9.320313 | 838 | (5.0034, 0.9755, 0.0034) | **`d9b49a13`** |
| 2 | `917c37f` | 11.319137 | 9.320313 | 838 | (5.0034, 0.9755, 0.0034) | **`d9b49a13`** |
| 3 | final | 11.319137 | 9.320313 | 838 | (5.0034, 0.9755, 0.0034) | **`d9b49a13`** |
| 4 | final | 11.319137 | 9.320313 | 838 | (5.0034, 0.9755, 0.0034) | **`d9b49a13`** |
| 5 | final | 11.319137 | 9.320313 | 838 | (5.0034, 0.9755, 0.0034) | **`d9b49a13`** |

Five separate harness invocations across the two builds — four in parallel with
nine other Chromium instances, one alone in the foreground — byte-identical field.
That is simultaneously the determinism result and the evidence that the build
difference described above touches nothing the solver computes. The scenario also asserts the
protocol rather than only printing the hash — 40 steps taken, zero emitters still
packing, mass > 0, field finite — because a run that silently emitted nothing
would produce a beautifully stable checksum that means nothing.

A second, independent datum falls out of the suite for free: `fluid-canister.js`
at T = 2.0 s and `fluid-sequence.js` at its t = 2.0 s checkpoint are different
scenario files driving the same protocol, and both report mass **15.8350** and
checksum **`5f380f8a`** — bit-identical across separate invocations and separate
scripts.

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

### 11. Pressure warm start

PENDING-PRESSURE

### 12. `physics` selfTest

`window.__physicsSelfTest()` via `fluid-stats.js`: `pass: true`, **25 PASS,
0 FAIL, 5 INFO** over 30 lines. Representative:

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
`texture_storage_3d`**. The last is the one that matters: SwiftShader reports
`maxStorageTexturesPerShaderStage = 4` and the solver uses 3, so init succeeds
with one slot spare. The trace pass's own budget is untouched — the solver has
its own bind group and shares nothing but the density texture.

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

### 14. Scenario suite — every verdict

Every scenario in `tools/headless/scenarios/` that touches the fluid or the
volumetric channel, with the `ok` it returned. `run.py` fails the run on
`ok: false`, so these are asserts, not prose.

| scenario | verdict | what it asserts |
|---|---|---|
| `fluid-mass.js` (6 phases, 4 lanes) | **ok: true** | every phase's tuning reached the dispatch loop, across a 25× iteration spread |
| `fluid-jacobi.js` | **ok: true** | each requested Jacobi count equals `lastJacobi` |
| `fluid-dt.js` | PENDING-DT | per-step defect first order in dt, total loss zeroth order |
| `fluid-leak.js` | **ok: true** | per-step advection defect stays under 2% |
| `fluid-lifetime.js` × 2 | **ok: true** | mass finite and non-negative; no runaway with dissipation off |
| `fluid-stats.js` | **ok: true** | still air retains exactly 1; `physics` selfTest passes |
| `fluid-sequence.js` | **ok: true** | cloud wider than tall, thickest at the floor, field finite |
| `fluid-beam.js` | **ok: true** | solver transmittance reaches 0.5 inside the cloud |
| `fluid-column.js` | **ok: true** | column cell solid, zero density in it, smoke both sides |
| `fluid-canister.js` | **ok: true** | smoke present at T, exactly one canister live |
| `fluid-determinism.js` × 4 (2 per build) | **ok: true (4/4)** | 40 steps, no emitters left packing, field finite |
| `fluid-still.js` × 2 modes | **ok: true (2/2)** | pinned size held, cloud clear of the noise floor, both signs present, underside shadowed, rise/fall matches the mode |
| `fluid-pressure.js` | PENDING-PRESSOK | pressure finite; residual does not degrade over 50 s of forcing |
| `vol-shot.js` × 7 modes | **ok: true (7/7)** | frame finite, not black, pinned size held |
| `vol-counters.js` | **ok: true** (run-level; returns a JSON string) | B2a's work counters still resolve |
| `vol-compare.js` | **ok: true** (run-level; returns a JSON string) | reference comparison completes untruncated |
| `crouch-matrix.js` | **ok: true** (run-level; returns no verdict) | A-track's; drives `settings.volumetric` on and off |
| `smoke.js` (cold start) | **ok: true** | probe sees 33 lights, nav raster non-degenerate, 4 guards |

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
6. **No timings.** SwiftShader ms are not measurements. The three fluid passes
   are present and named in the profiler (3 of the frame's 26), which is all this
   box can say. The Mac bench list is below.
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
10. **The suite's numbers come from two builds**, differing by a readback method,
   a usage flag, the instant-source envelope fix and comment text. §10's
   unchanged checksum across both is the evidence that the difference is inert;
   a single-build sweep would be cleaner and costs another 40 minutes of matrix.
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
   is iterative and never exact") and it destroyed the solve's work: it made the
   bottom fluid row of the lattice unable to receive mass at all.
   `rowMass[0] ≡ 0.0000` had been sitting in the readback all along.
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
   readback τ = 1.434 (T = 0.238), peak smoothed 5.3× (88.8 → 16.7) and the cloud
   smeared over ±1.5 m along the beam. A guard LOS threshold tuned against the
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
   field bit-identical to the cold one — the checksums matched, which is what
   gave it away.
12. **`parked/README.md`'s "26/26 assertions" is one out**: the suite emits 25
   PASS and 5 INFO lines, all passing.
13. **`run.py` leaks its Chromium profile directory, and `/tmp` here is RAM.**
   Each run creates `/tmp/twopointfive-headless-<pid>` and never removes it. This
   track's matrices left **1099 of them holding 12 GB** — on a tmpfs, so 12 GB of
   resident memory doing nothing. A one-line `shutil.rmtree(udd, ignore_errors=True)`
   in `run.py`'s `finally` would fix it; anyone running matrices before then should
   `rm -rf /tmp/twopointfive-headless-*` afterwards. Nothing failed because of it
   — 14 GB was still free — but a longer campaign on a smaller box would.
14. **The two longest runs were OOM-killed, and the evidence was in the cgroup,
   not the logs.** They died part-way with no output and no exit line — which
   looks exactly like a run still in progress — while shorter runs in the same
   batch finished. `/sys/fs/cgroup/memory.events` settled it: `oom_kill 1` with
   `memory.peak` equal to the 220 GiB limit, on a box whose cgroup is shared. The
   leaked tmpfs profile directories above were 12 GB of that. Two lessons for
   whoever runs the next matrix: keep the longest run in a lane of its own and
   sweep `/tmp` between waves, and make the driver assert that every lane produced
   its result file rather than trusting that no FAIL line means success. Timing:
   this cost about 40 minutes of re-runs at the end of the track.

## Merge notes

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
  are unaffected — the gate's behaviour with no pin is what it was.
- **The density-interface contract is untouched.** `writeVolume` still writes
  R = density with G/B/A zero over B2a's grid, dims, origin and cell; no
  B2a-owned file changed.
- **Anyone porting the solver to a coarser lattice**: `wallFaces` must be applied
  wherever velocity is written, or the divergence pass is handed a right-hand
  side the Neumann pressure cannot satisfy and the projection stops being a
  projection. That invariant is the whole fix.
- **Anyone touching `advectScl`, the projection operators, or `dt`**: the numbers
  in §3–§5 are load-bearing for the `jacobi` and `dissipation` comments, and
  `fluid-dt.js` asserts the *order* of the mass error. Making advection
  conservative will trip that assert — correctly. Update the bundle, don't
  loosen it.

## Mac bench script (Sujay, M1 Max, Chrome)

SwiftShader cannot answer the ms question. One-liners, in the console:

`__bench(n)` returns a JSON string; the fields that matter here are
`wallMsPerFrame` and the per-pass `gpu` map, which carries `fluidAdvect`,
`fluidPressure` and `fluidScalars` by name.

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
__fluid.setScale(1); console.table([await ms()]);
__fluid.setScale(2); console.table([await ms()]);
__fluid.setScale(1);

// 4. With a real cloud in the room rather than empty air — the pressure solve
//    does the same work either way, but the volumetric march does not.
__throwCanister(-8, -6); await __renderStill(60, 50); console.table([await ms()]);
```

What to look for. `fluidPressure` should dominate the solver's share, and in run
2 it should scale close to linearly in the iteration count: 20 dispatches of a
7-point stencil over 389k cells is a bandwidth-bound job on paper. If it does
*not* scale linearly, the dispatches are launch-bound at this grid size, and
since the residual keeps falling as ~1/N (§2) the default could go back up to 40
for nearly free. If it scales worse than linearly, drop toward 10 — the mass
table says 8 iterations retain within 0.24% of 200, so a higher count buys only a
smaller residual, and §2 quantifies exactly how much. Run 3 is the one that
decides whether the halved lattice is worth keeping as a quality preset: it
quarters the solver's cell count and the interface texture the tracer samples is
unchanged, so any visual cost is resampling blur, not resolution loss.
