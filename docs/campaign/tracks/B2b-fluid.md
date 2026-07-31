# Track B2b — GPU smoke solver: the mass leak, found and fixed

Branch `claude/fluid`, base `9940888` (the solver, sources, canister and
occupancy as received for review). Commits: `a00e648` one owner for solver
tuning, `708eda4` the consistent operator triple, `00ce581` measured Jacobi
default + instruments + scenarios, `0b0c3c2` measured dissipation claim +
solver-field probes + per-scenario verdicts, plus this report's commit.

This track was handed a solver that ran inside its limits, never produced a
NaN, and was deterministic — and that lost 57% of a sinking cloud's mass in
8 seconds. The leading hypothesis on arrival was residual divergence from an
under-converged pressure solve, untested because a scenario's Jacobi override
never reached the dispatch loop. The override is fixed, the hypothesis is
**refuted by measurement**, and the actual cause turned out to be two
mismatched discretisations sitting on either side of the pressure solve.

## What changed and why

1. **One owner for solver tuning.** The Jacobi count lived in both
   `RenderSettings.fluidJacobi` and `FluidSim.tune.jacobi`, and the renderer
   copied settings → tune every frame, so a scenario writing `tune.jacobi` was
   overwritten and a sweep measured one count N times. `FluidSim.tune` is now
   sole owner, and `lastJacobi` records what the dispatch loop really ran.
2. **Divergence, gradient and Laplacian are now one consistent triple.**
   `divergence`/`project` differenced centrally (2h) while `jacobi` solved the
   compact h Laplacian, so the solve converged to a pressure that could not
   null the divergence it measured: reduction saturated at 1.5×, residual flat
   from 4 to 80 iterations. Velocity is now differenced as a staggered field —
   forward-difference divergence, backward-difference gradient across the same
   faces — composing to the stencil `jacobi` already solved.
3. **The wall condition moved inside the solve.** Clamping wall-normal velocity
   *after* projection left the lowest fluid row at v·n = 0 with the row above
   flowing into it; a gather cannot cross that, and row 0 could never receive
   any mass at all. Zero flux is now written wherever velocity is written.
4. **Honest instruments**: an active-cell residual filter returning `null`
   (not `0.000000`) on an empty set, plus `advectionBalance`, `columnDensity`,
   `densitySamples`, `solidAtWorld`, `solidRow`, `resources`, cloud bbox.
5. **Defaults from curves.** Jacobi 40 → **20** from the measured sweep; the
   `dissipation` comment now states the lifetime the field actually has.

## Verification

Every run below is Chromium/SwiftShader via `tools/headless/run.py`, sim grid
at full production resolution (208 × 13 × 144, 0.25 m cells, `scale = 1`) —
no run here used the halved debug lattice. Render resolution is noted per
bundle and never affects the solver — the lattice is fixed by `SMOKE_DIMS`, so
no number in this report depends on how many pixels were traced. Where a
bundle reads back its own render size it is quoted as measured, which in four
cases is not the size the scenario asked for (see Findings 9). Every figure
below is reproducible from the named scenario via
`python3 tools/headless/run.py --scenario … --json …`
(the harness needs `dangerouslyDisableSandbox` on this box — Chromium wants
its own namespaces); the JSON blobs themselves live in gitignored scratch.

### 1. The plumbing fix, proved rather than asserted

`fluid-mass.js` runs two phases that differ only in `jacobi`, and fails the
run unless each phase's requested count equals `lastJacobi`:

| phase | requested | dispatched |
|---|---|---|
| `noDissJ8` | 8 | 8 |
| `noDissJ200` | 200 | 200 |

Before the fix both dispatched 40 — the value `settings.fluidJacobi` pushed
in every frame — which is exactly why the two phases had come back
identical.

### 2. Iteration sweep: hypothesis refuted, cause found

`fluid-jacobi.js` (requests 320×200). A sustained 4 m/s horizontal jet from a
zeroed field, read at step 3 (transient) and step 24 (quasi-steady).
`postMean` is the whole-room mean |∇·v| after projection (1/s); `relResid` is the mean over
cells moving faster than 0.05 m/s, relative to `activeVelRms / cell`.

Quasi-steady (step 24), before vs after the operator fix:

| Jacobi | meanRed before | meanRed after | relResid before | relResid after |
|---|---|---|---|---|
| 4  | 1.27 | 2.13  | 2.86e-2 | 1.07e-2 |
| 10 | 1.42 | 3.62  | 2.54e-2 | 3.92e-3 |
| 20 | 1.49 | 5.70  | 2.54e-2 | 1.64e-3 |
| 40 | 1.52 | 9.46  | 2.55e-2 | 7.97e-4 |
| 80 | 1.53 | 17.67 | 2.57e-2 | 3.87e-4 |

**The refutation.** With the plumbing fixed but the operators unchanged, mass
retention over 8 s was 0.4301 at 8 iterations and 0.4287 at 200 — a 0.3%
difference, in the wrong direction. An under-converged pressure solve was not
the mass sink. The `relResid` column says why: before the fix the residual
was *flat* — 20× the work bought 10% — because the iteration was converging
to the wrong pressure, not converging slowly. The projection reduced the
divergence it measured by only 1.5×.

**The cause.** The per-step advection mass defect (`fluid-leak.js`,
`advectionBalance`) was 1.4% per step while the central-difference instrument
read a residual of 0.019/s — which over a 50 ms step can account for about
0.1%. Fourteen times too small to explain the loss. That gap is the
signature of an operator blind to what advection transports: central
differences on a collocated grid are a 2h stencil and cannot see h-scale
divergence at all, while the Jacobi kernel was solving the h-spacing
Laplacian. After the fix the residual scales as ~1/N with no plateau, which
is what an iteration-limited solve looks like.

### 3. The mass table

`fluid-mass.js` (requests 320×200). One instant blob — peak density 25,
radius 0.9 m, at (5, 1.4, 0) in open corridor air — then **every** emitter
silenced (permanent wisps dropped, canisters cleared, guards frozen). The room
is closed, so with `dissipation = 0` any change in the density integral is
the solver's own numerical error.

| phase | tuning | retained @ 8 s (before) | retained @ 8 s (after) |
|---|---|---|---|
| `stillAir` | no forces at all | 1.00000 † | **1.00000** |
| `sinkOnly` | weight only, no confinement | 0.2685 | **0.93278** |
| `noDiss` | weight + confinement, 20 iterations | — ‡ | **0.71758** |
| `noDissJ8` | as above, 8 iterations | 0.4301 | **0.71873** |
| `noDissJ200` | as above, 200 iterations | 0.4287 | **0.71702** |
| `defaults` | shipped tuning (dissipation 0.13) | — | 0.24727 |

`stillAir` retaining exactly 1.000000 is the control that fp16 storage and
solid-cell zeroing are lossless; every loss below it is transport. († the
before-figure for `stillAir` is from the pre-fix `fluid-stats.js` run, which
measures the same phase over 6 s rather than 8 — the pre-fix `fluid-mass.js`
phase list did not include it. `defaults` was not measured pre-fix at all.
‡ pre-fix there was no 20-iteration measurement to have: every phase
dispatched 40 whatever it asked for, which is the bug in item 1.)

The three `noDiss` variants — 8, 20 and 200 iterations — now land within
**0.25% of each other**, which is the positive form of the refutation: with a
projection that actually converges, iteration count above 8 buys no mass.

**Where it went, and the proof.** `fluid-leak.js` weighs the field
immediately before and after one advection step, per y row. Before the fix,
row 0 held exactly 0.0000 mass at every checkpoint of every phase while row 1
held up to 4.63 — the lowest fluid layer of the lattice was *unreachable*.
Its wall-normal velocity was clamped to zero, so its backtrace never left the
cell, and the only cells that could hand mass down to it were above it. After
the fix row 0 fills normally (0.058 → 1.92 → 5.96 → 9.41 over t = 1.5…5 s)
and the per-step defect drops from −1.1% to +0.11%.

Per-step defect and residual at Jacobi 20 (after):

| t (s) | mass before | mass after | rel. defect | active cells | relResid |
|---|---|---|---|---|---|
| 0.5 | 17.349 | 17.324 | −0.144% | 168 | 7.8e-4 |
| 1.5 | 16.606 | 16.558 | −0.292% | 682 | 2.7e-4 |
| 2.5 | 15.517 | 15.473 | −0.279% | 875 | 2.3e-4 |
| 3.5 | 15.256 | 15.268 | **+0.077%** | 902 | 1.8e-4 |
| 5.0 | 15.802 | 15.819 | **+0.108%** | 934 | 1.9e-4 |

The sign flips: the scheme is no longer a monotone sink but a bounded
two-sided error. It loses while the cloud is descending fast and gains
slightly once it is pooled (the solid-aware gather renormalises its weights
against the floor, which over-counts a little).

### 4. Lifetime out to 20 s

`fluid-lifetime.js`, same blob, 400 steps per phase.

`dissipation = 0` — the numerical error alone:

| t (s) | 0 | 1 | 2 | 3 | 5 | 8 | 12 | 16 | 20 |
|---|---|---|---|---|---|---|---|---|---|
| mass | 17.454 | 16.608 | 14.468 | 13.345 | 12.841 | 12.525 | 12.402 | 12.595 | 13.330 |
| peak | 23.08 | 18.67 | 11.64 | 8.38 | 6.90 | 5.01 | 3.86 | 3.00 | 2.46 |

Retained to 3 s: **0.7646**. Retained to 20 s: **0.7637**. The rate over
8-20 s is −0.0052/s — a slight *gain*. So this is not an e-folding leak at
all: it is a **one-off deficit of ~24% taken while the cloud is billowing in
the first ~3 s, and a bounded ±0.008/s drift thereafter that changes sign.**
Quoting a single e-folding number across both regimes would be a fiction, so
the report and the code comment quote both.

`dissipation = 0.13` (shipped) against its own model `exp(−0.13 t)`:

| t (s) | 0 | 3 | 8 | 12 | 16 | 20 |
|---|---|---|---|---|---|---|
| mass | 17.344 | 9.042 | 4.289 | 2.594 | 1.583 | 0.984 |
| actual / modeled | 0.994 | 0.765 | 0.695 | 0.707 | 0.726 | 0.759 |

The measured rate over 8-20 s is **0.1227/s against the modeled 0.130** — the
settled decay *is* the modeled one, within 6%. The whole discrepancy is the
front-loaded transient, so mass follows `0.76 × exp(−0.13 t)`.

**`dissipation` is deliberately left at 0.13.** The deficit is front-loaded;
lowering the rate to absorb it would leave the late haze hanging around too
long. What was wrong was the claim, not the number, and the claim is now the
measured one. At t = 20 s the field still carries peak density 0.25 over 5763
cells (σ ≈ 0.013/m) — "still a haze at 20 s" survives contact with the
instrument.

### 5. Canister sequence — density-slice stats at t = 0.5 / 1 / 2 / 5 / 10 s

`fluid-sequence.js`, measured render size 640×328, one scripted throw from a
fixed release point (no animation state in the protocol). "Grows, curls,
spreads along the floor" is a measured box and a row histogram, not a
camera's opinion.

| t (s) | emitting | mass | peak | cells ≥0.05 | bbox size (x,y,z) m | centroid y | relResid |
|---|---|---|---|---|---|---|---|
| 0.5  | no  | 0.000   | 0.00   | 0    | —                  | —     | null |
| 1.0  | no  | 0.000   | 0.00   | 0    | —                  | —     | null |
| 2.0  | yes | 15.835  | 25.03  | 237  | 3.00 × 1.25 × 2.50 | 0.501 | null |
| 5.0  | yes | 78.045  | 138.63 | 567  | 5.00 × 1.25 × 4.50 | 0.293 | 2.4e-4 |
| 10.0 | no  | 133.028 | 79.94  | 1008 | 5.25 × 1.50 × 6.50 | 0.211 | 4.3e-4 |

Read from the numbers: the cloud grows monotonically; it spreads **3.5-4.3×
further horizontally than vertically** (a 5.25 × 6.50 m footprint against
1.50 m of height) and its mass centroid *falls* from 0.50 m to 0.21 m while
it grows, which is pooling on the floor rather than rising. The occupied-cell
histogram by row at t = 10 s is 293/280/238/163/31/3 from the floor up — a
wedge thickest at the floor. Peak density falls from 138 to 80 between 5 s
and 10 s as emission ends and the cloud spreads.

The two zero rows are correct behaviour, not a gap: a canister only starts
smoking once its rigid body **sleeps** (`canister.ts`, `emitAge < 0` until
`body.sleeping`), so at 0.5 s and 1.0 s it is still bouncing. `emitting`
brackets the onset between 1.0 s and 2.0 s.

`relResid` is `null` at 2.0 s because *nothing in the room is moving faster
than 0.05 m/s* — the pooled cloud's spread there is numerical diffusion, not
flow. That is a finding, not a defect in the run (see Findings).

### 6. Beam through the cloud

`fluid-beam.js`, measured render size 640×328, torch on, a canister-strength
emitter pinned on the beam axis 4 m out (Findings 7 says why it is pinned).
σ_t = 0.05 per unit density (`settings.volumetric`), `volExtinction` on,
marched at 10 cm.

| s (m) | 3.0 | 3.5 | 4.0 | 4.5 | 5.0 | 5.5 |
|---|---|---|---|---|---|---|
| solver ρ | 0.00 | 23.81 | 88.75 | 27.31 | 0.06 | 0.00 |
| solver T | 1.000 | 0.830 | 0.161 | 0.037 | 0.034 | 0.034 |
| gameplay ρ | 4.72 | 10.14 | 13.25 | 16.33 | 9.86 | 2.48 |
| gameplay T | 0.918 | 0.751 | 0.557 | 0.381 | 0.271 | 0.238 |

On the field the renderer marches: column integral 67.81, **τ = 3.39**,
transmittance out **0.034**. Half the beam is gone by 3.8 m and 90% of it by
4.2 m: the beam is not merely dimmed, it is extinguished inside a 0.4 m span
of the cloud, and **3.4% of it survives** to light anything beyond.

### 7. Column obstacle

`fluid-column.js`, measured render size 640×328. A held-on 3.5 m/s jet aimed
down +x at the corridor support column at (0, ·, −3.9) — 0.28 m half-extent,
full height — from 3 m upwind, probed after 4 s on the solver's own lattice.

| probe | at (x, y, z) | occupancy says | solver ρ | coarse readback ρ |
|---|---|---|---|---|
| upwind | (−1.0, 1.5, −3.9) | fluid | **4.4145** | 1.7456 |
| inside the column | (0, 1.5, −3.9) | **solid** | **0.0000** | **1.0075** |
| beside it, +z | (0, 1.5, −3.4) | fluid | **1.2606** | 0.9935 |
| beside it, −z | (0, 1.5, −4.4) | fluid | **1.9327** | 0.9222 |
| lee, 0.6 m | (0.6, 1.5, −3.9) | fluid | 0.0941 | 0.4863 |
| lee, 1.5 m | (1.5, 1.5, −3.9) | fluid | 0.0000 | 0.0022 |

Three numbers make the claim: the column's cell is the one the occupancy bake
calls solid, density inside it is **exactly zero**, and density 0.5 m to
either side is not — 1.26 and 1.93 against 4.41 arriving upwind. The plume
splits. Behind the column the wake is still open at 4 s: 0.094 at 0.6 m
(2% of the upwind value) and 0.000 at 1.5 m, so the two halves have not
closed up yet. Cloud at 4 s: mass 13.74, 1230 cells ≥ 0.05, box
(−4, 0, −5.75) → (1.25, 3.25, −2).

The coarse column is here because it is alarming: the gameplay readback
reports **density 1.0075 inside solid geometry**, since its 1 m × 0.25 m × 1 m
box average straddles a 0.56 m column. It also flattens the whole profile to
1.75 / 1.01 / 0.99 / 0.92 — the split is gone. See Findings 6.

### 8. Determinism

`fluid-determinism.js` pins everything before the first sim step: paused rAF
loop, guards frozen, solver **and** every emitter reset (`SM.reset(true)`
drops the permanent wisps too), canisters cleared, one instant blob, fixed
50 ms dt, then 40 steps and an FNV-1a hash of the raw fp16 density field.

| run | steps | dt | emitters live | mass | peak | cells | centroid | checksum |
|---|---|---|---|---|---|---|---|---|
| 1 | 40 | 50 ms | 0 | 11.319137 | 9.320313 | 838 | (5.0034, 0.9755, 0.0034) | **`d9b49a13`** |
| 2 | 40 | 50 ms | 0 | 11.319137 | 9.320313 | 838 | (5.0034, 0.9755, 0.0034) | **`d9b49a13`** |

Two separate harness invocations, byte-identical field. The scenario also now
asserts the protocol rather than only printing the hash — 40 steps taken, zero
emitters still packing, mass > 0, field finite — because a run that silently
emitted nothing would produce a beautifully stable checksum that means
nothing.

A second, independent determinism datum falls out of the suite for free:
`fluid-canister.js` at T = 2.0 s and `fluid-sequence.js` at its t = 2.0 s
checkpoint are different scenario files driving the same protocol, and both
report mass **15.8350** and checksum **`5f380f8a`** — bit-identical across
separate harness invocations and separate scripts.

### 9. `physics` selfTest

`window.__physicsSelfTest()` via `fluid-stats.js`: `pass: true`, **25 PASS,
0 FAIL, 5 INFO**. Representative lines:

```
PASS  ray/rotated-obb  t=1.885786438 (expect 1.885786438) n=(0.707107,0.000000,0.707107)
PASS  drop/no-jitter  peak-to-peak=9.71e-17 m, residual |v|=0.00e+0 m/s
PASS  fixed-step/frame-rate-independent  30fps->y=0.060000000000  60fps->y=0.060000000000  144fps->y=0.060000000000  240fps->y=0.060000000000
PASS  energy/e=1.05 does diverge  unclamped e=1.05 gains 217.5% in 10 s — the clamp is not decorative
PASS  tunnelling/thin-wall  0.06 m body vs a 0.04 m wall: 12 m/s -> x=-1.100, 20 m/s -> x=-3.500, 34 m/s -> x=-7.696
INFO  cost  32 awake bodies, 530 static boxes: 0.0613 ms per step(1/60) = 0.0307 ms per 120 Hz tick
```

`parked/README.md` claims 26/26 assertions; the suite as it stands emits 25
PASS lines plus 5 INFO lines. Nothing fails — the README's count is one out
against the current suite, which is worth a one-line fix by whoever owns it.

### 10. Resource counts (counted, not estimated)

`FluidSim.resources()`, read back in `fluid-sequence.js`.

Bind-group layout, one group, compute-only visibility:

| kind | count |
|---|---|
| uniform buffer | 1 |
| sampler (filtering) | 1 |
| sampled `texture_3d<f32>` | 4 |
| read-only storage buffer | 1 |
| **write-only `texture_storage_3d`** | **3** |

The storage-texture count is the one that matters: SwiftShader reports
`maxStorageTexturesPerShaderStage = 4` and the solver uses 3, so init
succeeds with one slot spare. The trace pass's own budget is untouched — the
solver has its own bind group and shares nothing but the density texture.

Solver-owned memory, 208 × 13 × 144 = 389,376 cells:

| resource | format | MB |
|---|---|---|
| `fluid-vel-0/1` | rgba16float | 2.971 each |
| `fluid-scl-0/1` | rgba16float | 2.971 each |
| `fluid-curl` | rgba16float | 2.971 |
| `fluid-prs-0/1` | r32float | 1.485 each |
| `fluid-div` | r32float | 1.485 |
| `fluid-occupancy` | packed bytes | 0.371 |
| `fluid-params` | uniform | 0.0016 (1648 B) |
| **total** | | **19.682** |

B2a's `smoke-volume` interface texture (2.971 MB) is excluded — it is the
volumetric channel's, not the solver's. Occupancy bake: **19,420 solid cells**
(5.0% of the lattice); per-row solid counts
`[2085, 2085, 2070, 1877, 1599, 1242, 1242, 1242, 1106, 1218, 1218, 1218, 1218]`
from floor to ceiling.

### 11. Scenario suite

Every fluid scenario now returns `ok` with real assertions rather than numbers
a reader has to judge; `run.py` fails the run on `ok: false`. All runs below
are against the single final build (`0b0c3c2`), and the numbers quoted
throughout this report are that build's.

| scenario | verdict | what it asserts |
|---|---|---|
| `fluid-jacobi.js` | ok: true | each requested Jacobi count equals `lastJacobi` |
| `fluid-mass.js` (defaults/noDiss/stillAir) | ok: true | every phase's tuning reached the dispatch loop |
| `fluid-mass.js` (sinkOnly/J8/J200) | ok: true | as above, across a 25× iteration-count spread |
| `fluid-lifetime.js` × 2 | ok: true | mass finite and non-negative; no runaway with dissipation off |
| `fluid-leak.js` | ok: true | per-step advection defect stays under 2% |
| `fluid-stats.js` | ok: true | still air retains exactly 1; `physics` selfTest passes |
| `fluid-sequence.js` | ok: true | cloud wider than tall, thickest at the floor, field finite |
| `fluid-beam.js` | ok: true | solver transmittance reaches 0.5 inside the cloud |
| `fluid-column.js` | ok: true | column cell solid, zero density in it, smoke both sides |
| `fluid-canister.js` | ok: true | smoke present at T, exactly one canister live |
| `fluid-determinism.js` × 2 | ok: true | 40 steps, no emitters left packing, field finite |
| `vol-shot.js` × 7 modes | ok: true (7/7) | B2a's shots still render: no WGSL, validation or device errors |

`vol-shot.js` needed no porting, and this is worth saying plainly because the
brief expected it to: all four of its blob call sites already drive
`__smokePuff`, having been ported in one of the branch's earlier salvage
commits (`git diff 9940888..HEAD -- tools/headless/scenarios/vol-shot.js` is
empty). The only surviving mention of `__smokeTest` is the comment explaining
the replacement. All seven modes (`fogon`, `fogoff`, `fluoro`, `moon`,
`corner`, `exton`, `extoff`) come back clean with a screenshot. They were run
from generated per-mode copies in gitignored scratch rather than by `sed`-ing
the checked-in file, so seven modes run in parallel without fighting over one
`WHICH` constant.

`fluid-mass.js` with the `noDissJ200` phase is the most expensive run in the
suite (200 Jacobi dispatches × 160 steps at full lattice) and wants its own
lane: it hit the driver's 2400 s cap while sharing the box with five other
Chromium instances, and finished normally re-run alone (1384 s in a
four-instance matrix). All three of its phases then reproduced the
first matrix's figures exactly — 0.93278 / 0.71873 / 0.71702.

## Below 100%, with reasons

1. **The advection scheme is still not conservative, by construction.** A
   semi-Lagrangian *gather* conserves mass only when the velocity field is
   uniform; the residual is a one-off ~24% deficit during a fast transient
   plus a bounded drift under 0.01/s that changes sign (−0.0052/s measured
   over 8-20 s, i.e. a slight gain). Fixing it properly means either a scatter
   pass with fixed-point atomics or a per-source-cell gather-weight
   normalisation (a 27-neighbour search per cell). Neither was attempted:
   the first would need integer fixed-point to stay deterministic, and
   determinism is a hard deliverable here. **No global mass renormalisation
   was added** — the brief allowed one as a last resort and it was not
   needed once the operator mismatch was fixed.
2. **Vorticity confinement costs 21.5 points of retention** — 0.93278 without
   it against 0.71758 with it, over 8 s. Cell-scale velocity structure is exactly
   what the confinement exists to create and exactly what makes a gather
   non-conservative. It is kept: it is what makes the medium read as smoke.
   Anyone who wants the mass back knows the knob and the price.
3. **Velocity advection carries a half-cell offset.** With velocity read as a
   face field, `advectVel` still traces from the cell centre and samples the
   face field through the linear sampler, so the velocity it transports is
   interpreted half a cell off. It is an O(h) error in a scheme that is O(h)
   anyway, it does not accumulate, and it does not touch conservation — the
   mass-critical pass (`advectScl`) is fully consistent, because scalars
   genuinely live at cell centres and `velCentreB` reconstructs the centred
   velocity from the faces. Fixing it means three separate traces in
   `advectVel`, i.e. 3× that pass.
4. **`curl` and the confinement force keep the same half-cell offset**, for
   the same reason and with the same justification: the vorticity magnitude
   is an aesthetic driver, and doing it properly costs 4× the texture loads
   on a hot path.
5. **Buoyancy is applied to a face component from cell-centred scalars.**
   A strict MAC scheme would average the two cells sharing the face. Sub-cell
   offset in where lift acts; magnitudes unchanged.
6. **No timings.** SwiftShader ms are not measurements. The three fluid
   passes are present and named in the GPU profiler (`fluidAdvect`,
   `fluidPressure`, `fluidScalars` — 3 of the frame's 26 passes), which is all
   this box can say. The Mac bench list is below.
7. **The gameplay readback under-reports optical depth by 2.3×** — see
   Findings. Not fixed here because it is B3's threshold to set and the
   smoothing is inherent to a 1 m × 0.25 m × 1 m coarse cell; it is
   *measured* so B3 can set the threshold knowingly.
8. **`fluid-beam.js` reproduces to 4 significant figures, not bit-exactly.**
   Its release point is derived from `player.flashlightOrigin()`, so it
   inherits the aim rig's state from the harness's pre-scenario settle frames
   — the exact thing `__throwCanister`'s `from` override exists to avoid. Two
   runs gave τ = 3.3904 and 3.3907. The determinism, canister and sequence
   bundles all use fixed world points and are bit-identical; the beam bundle
   is a measurement of optical depth, not of determinism, so it was left
   reading the real weapon pose.
9. **The screenshots were verified numerically, not by eye.** All eleven
   (`beam`, `column`, `sequence`, `canister`, seven `vol-shot` modes) are
   1280×657, none black — sampled luma spans 0-217 with 40-64% of sampled
   pixels above 8/255, which for a night-time interior is a rendered frame and
   not a cleared buffer. But the `Read` tool's hook was unresponsive on this
   box for the whole session, so I could not open them. **Every descriptive
   claim in this report is therefore sourced from a readback, not from looking
   at an image** — the cloud shape claims are the bounding box and the row
   histogram, the beam claim is the optical-depth march, the obstacle claim is
   the six probes. Someone with a working image viewer should still look at
   `beam.png` and `sequence-t10.png` before this ships; the numbers say what
   the field is doing, not that it is pretty.
10. **`activeSpeed` is one threshold, not a curve.** The residual instrument
   filters at 0.05 m/s. It is a parameter (`divergenceStats(activeSpeed)`),
   but no sensitivity sweep over it was run; the canister scene has no cells
   above it at all at t = 2 s, which the instrument now says out loud instead
   of reporting a zero.

## Findings

1. **A duplicated tuning field with a one-way per-frame sync is a
   measurement-destroying bug, not a style problem.** It cost this track its
   starting hypothesis: the two-iteration-count comparison that "proved"
   projection quality was irrelevant had never dispatched two different
   counts. The structural fix is one owner; the guard that makes it stay
   fixed is `lastJacobi` plus a scenario assert on it.
2. **A residual number is only as good as the operator that measures it.**
   The code claimed a ~2e-3 relative residual. The whole-room mean was
   4.6e-4 — but over 389,376 cells of which 2,645 were moving, so it was a
   measurement of still air. The active-cell figure was 2.5e-2, and the
   divergence the *advection* felt was larger still and invisible to a 2h
   central-difference stencil. Three different numbers, one of them quoted in
   a comment as the solver's quality.
3. **Enforcing a boundary condition after a solve can be worse than not
   enforcing it.** The post-projection wall clamp was defensible-looking
   ("the solve is iterative and never exact") and it destroyed the solve's
   work: it made the bottom fluid row of the lattice unable to receive mass
   at all. `rowMass[0] ≡ 0.0000` had been sitting in the readback all along.
4. **A mean over an empty set is not zero.** `activeRelResidual` read
   `0.000000` whenever no cell was moving, which is indistinguishable from a
   perfect projection. It now returns `null`.
5. **The canister cloud's spread is mostly numerical diffusion, not flow.**
   At t = 2 s of the sequence, no cell in the room exceeds 0.05 m/s, yet the
   cloud's footprint grows from 3.0 m to 5.25 m over the next 8 s. With
   `dissipation = 0` the peak still falls 23 → 2.46 over 20 s purely by
   spreading. Visually this is fine — thinning smoke is what smoke does — but
   nobody should reason about the cloud's motion from its growth rate.
6. **The gameplay readback is not the renderer's field, and the gap is
   2.3× in optical depth.** On the same beam: solver τ = 3.39 (T = 0.034),
   coarse readback τ = 1.45 (T = 0.236), with peak density smoothed 5.3×
   (88.75 → 16.81) and the cloud smeared over ±1.5 m along the beam. A guard
   LOS threshold tuned against the coarse field will let guards see through
   smoke the player cannot see through. Worse, the column probe shows the
   coarse field reporting **density 1.0075 inside a solid column** where the
   solver holds exactly 0 — a 1 m × 1 m box average straddles a 0.56 m
   column, so smoke leaks across geometry in the gameplay view only. Both
   numbers are for B3 to set thresholds against; neither is a solver defect,
   and neither is fixable without a finer coarse grid or a solid-aware
   downsample (the probe kernel currently box-averages without consulting the
   occupancy field — that is the cheap fix if B3 wants it).
7. **A thrown canister is not a placeable emitter.** The first version of the
   beam scenario threw a canister at a point on the beam axis; it bounced and
   rolled 2.8 m away, and the beam clipped the cloud's upper edge for τ =
   0.047. The measurement was reported as a failure by its own assert rather
   than as a beam that "didn't stop", which is the reason to write asserts
   into scenarios at all.
8. **The first version of the column probe read exactly zero at all six
   points** while the field carried a 13-unit plume, because it probed the
   quarter-resolution gameplay readback. Probing the solver's own lattice is
   both authoritative and cheaper to reason about; the coarse value is still
   reported alongside for the B3 comparison.
9. **A scenario's `__renderer.resize()` does not always survive the run, and
   I could not isolate when.** `fluid-sequence`, `fluid-beam`, `fluid-column`
   and `fluid-canister` ask for 384×240 / 448×280 and all four read back
   640×328 (= canvas 1280×657 × `resolutionScale` 0.5) at the end. A minimal
   probe — pause, resize, step 200 frames — held 384×240 at every 20-frame
   checkpoint, so it is not simply "long runs revert". The mechanism is in
   reach: `frameBody` carries a deferred-resize gate suppressed only for
   `benchGuard`, and `__renderStill` drives `frameBody` directly, so any
   resize event landing after a scenario's own resize re-derives the size from
   the canvas. It costs render time, not correctness — no number in this
   report depends on pixel count — so it is flagged for whoever owns the
   harness rather than fixed here, and every resolution quoted above is the
   measured one.
10. **`parked/README.md`'s "26/26 assertions" is one out**: the suite emits 25
   PASS and 5 INFO lines, all passing.

## Merge notes

- **`RenderSettings` loses `fluidJacobi`.** No settings-revision bump is
  needed: `loadInto` iterates `DEFAULT_SETTINGS` and skips any stored key it
  does not own (`hasOwnProperty` on defaults), so a stored `fluidJacobi` is
  dropped silently. Anyone rebasing a branch that reads
  `settings.fluidJacobi` should point it at `renderer.fluid.tune.jacobi`.
- **Uniform bytes unchanged: 1648** (`(28 + 32 × 12) × 4`) — the operator fix
  added no uniform fields, so nothing in the append-order rules is disturbed.
- **Bindings unchanged**: 1 uniform + 1 sampler + 4 sampled `texture_3d` + 1
  read-only storage buffer + 3 storage `texture_3d`, one bind group, compute
  only. Storage textures per stage stay at 3 of SwiftShader's 4.
- **Profiler pass count unchanged: 3** (`fluidAdvect`, `fluidPressure`,
  `fluidScalars`). The Jacobi default halving 40 → 20 removes 20 dispatches
  per step from inside `fluidPressure` without changing the pass count.
- **`fluid-scl-1` gained `COPY_SRC`** so `advectionBalance` can weigh the
  pre-advection field. Costs nothing but a usage flag.
- **The density-interface contract is untouched.** `writeVolume` still writes
  R = density with G/B/A zero over B2a's grid, dims, origin and cell; no
  B2a-owned file changed.
- **Anyone porting the solver to a coarser lattice**: `wallFaces` must be
  applied wherever velocity is written, or the divergence pass is handed a
  right-hand side the Neumann pressure cannot satisfy and the projection
  stops being a projection. That invariant is the whole fix.

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

What to look for. `fluidPressure` should dominate the solver's share, and in
run 2 it should scale close to linearly in the iteration count: 20 dispatches
of a 7-point stencil over 389k cells is a bandwidth-bound job on paper. If it
does *not* scale linearly, the dispatches are launch-bound at this grid size,
and since the residual keeps falling as ~1/N (§2) the default could go back up
to 40 for nearly free. If it scales worse than linearly, drop toward 10 — the
mass table says 8 iterations retain within 0.25% of 200, so the only thing a
higher count buys is a smaller residual, and §2 quantifies exactly how much.
