# Track B1 — radiosity → sampling hybrid

Branch `claude/radiosity`, base `be298ab` (campaign trunk after Track A).
Commits, in order: `4027994` headless diagnostics (readback hooks,
`__freezeClock`, crouch-matrix scenario), `85f9362` the crouch fix,
`e0771c9` the `indirectMode` split, `2b89f4c` gather + patchRIS, then the
report/verification commits on top.

## What changed and why

The radiosity solve itself is untouched: baked form factors, per-frame
inject (baked light tables + torch depth maps, zero rays), two warm
Jacobi steps. What changed is who trusts it, and how much.

1. **Crouch bug (Deliverable 0).** The flashlight reaches the solve ONLY
   through torch depth-map layer 0. Crouched, the aim layer is off and the
   crouch idle pose parks the pistol slide 2-3 cm in front of the lens, so
   the pinhole map traced from the lens centre read 87-93% of its texels as
   blocked at centimetre depth (measured below) → zero injected flashlight
   energy → the room's flashlight bounce vanished, and the depth-map beam
   march lost the shaft. Fix in the codebase's own idiom: the flashmap
   trace skips its owner's dynamic group (`traceSkipping`, same shape as
   the probe's `occludedSkipping`); owner groups are a per-frame table
   (`Renderer.setTorchGroups`, fed by `Guards.torchGroups`). A table
   rather than a derivation from the light index because a dead guard
   keeps its packed body (a corpse you drag) but leaves the light list, so
   the k-th live light and the k-th body group diverge as soon as anyone is
   down, and only the game knows the pairing. `torchVisPoint` (radiosity
   inject) gets the trace pass's 4-tap PCF (`torchMapSample`'s taps) so
   patch shadows ramp over a texel instead of stepping, plus — post-review
   (R3-1) — a slope-scaled compare slack the biased inject needs and the
   image path's unbiased target does not: the fixed slack was failing
   grazing far receivers on their own slanted surface. The game-side
   crouch pose is untouched, as instructed.
2. **`settings.radiosity` → `settings.indirectMode`** (Deliverable 1):
   `"traced" | "radiosityRead" | "gather" | "patchRIS"`, a panel select in
   the lighting/restir group, persisted — store revision 4 migrates a
   stored `radiosity=false` to `"traced"` (a `true` is the default anyway),
   the stale-keys mechanism was re-keyed by revision so this bump does not
   re-drop revision-3's keys from fresh blobs, and enum values are
   validated on load. Threaded into `Uniforms` at **byte 864**:
   `indirectMode` u32 (864), `radPatchCount` u32 (868), two pads to 880;
   `UNIFORM_SIZE = 880` (bytes 880-943 are the volumetrics track's,
   untouched — see merge notes). The solve runs for every mode but
   `traced`; reference mode and a patchless scene both force `traced` on
   the CPU (`Renderer.effectiveIndirectMode`), so `__compareToReference`
   never validates the approximation against itself. The old `radiosityOn`
   f32 stays (never move fields) as a legacy mirror meaning "the solve is
   live this frame"; the shader branches on `indirectMode`.
3. **Mode `gather`** (Deliverable 2): every primary hit — static or a
   character — traces bounce 1 for real and, at that x1, adds the solve's
   incident irradiance re-radiated by x1's albedo (`radiosityIndirect(x1)
   · albedo(x1)`) instead of tracing on — the classic final gather. The
   character standing in the beam becomes a real occluder of the floor's
   bounce, and a character standing in the room is lit by the room's
   bounce through the same x1 read; the solve is only ever consumed one
   bounce from the eye, where its baked-static error is diffused instead
   of photographed at the primary hit. Fallbacks are per-vertex and are the
   paths that already exist: a dynamic x1 (the bounce ray landed on a
   character — no patch) or a patchless face (`-1` sentinel) falls through
   to the traced continuation, which at the default one bounce is x1's NEE
   and nothing more. x1's direct NEE (`sampleIndirectRIS`) is kept: the
   solve's G row is one-bounce-removed radiosity, so NEE + sky row + G is
   the complete, non-overlapping set at x1 (deleting the NEE is proposed
   under Findings with what it would take to justify it, not done).
   ReSTIR GI composes structurally — the gathered x1 is an ordinary x2
   sample, its cached radiance simply including the gathered term — but the
   review found the reservoir merge inflating sparse-bright reuse 4×, so
   "composes" is only true after the merge fix in the Review resolution
   (R1-1). Checkerboard stance: gather traces exactly one bounce per
   pixel, i.e. the traced-1-bounce cost regime, so the CPU's existing
   `bounces <= 1 → indirectRate inert` rule is left keyed to the bounces
   setting unchanged; where the rate does apply, gather pixels sit tiles
   out exactly like traced ones and their validity alpha follows
   `traceIndirect` (only the two patch-fed modes claim always-valid, and
   a radiosityRead pixel only when its face actually has patch data — a
   face below the patch builder's area floor traces instead).
4. **Mode `patchRIS`** (Deliverable 3): at the primary vertex (b == 0,
   after `restirDirect`) the solved patches ARE the light list. Per pixel:
   M = 8 patches drawn from a per-frame inclusive CDF over
   `luminance(B_j)·A_j` (binary search, ~12 taps each; the survivor's
   source probability is read back exactly from row 2's spare `.w` lane,
   never re-derived as a CDF difference — see R1-4), one point jittered on
   each patch cell (`pos + tu·hu·ξ + tv·hv·ξ`), candidate radiance
   `B_j/π` folded through the bare point form `cos_j·A_j/d²` (unbiased at
   every distance BECAUSE the point is jittered over the cell — the
   original `+A_j/4` guard was pure energy loss, see R1-2), a weighted
   reservoir keeps one by `risTarget`, ONE `occluded()` shadow ray to the
   survivor. It runs for every valid primary hit, so it lights a
   **dynamic** hit (a character) with the room's bounce at the primary
   vertex, where gather does it one bounce out. Static hits with patch
   data add their own sky irradiance from the sky row (patches only carry
   surface-to-surface transfer); dynamic and unpatched hits go without a
   sky term (see Findings). Data plumbing exactly as briefed:
   `radStatic` bound `read` at `@group(1) @binding(16)` — the trace
   pass's **10th and last** storage buffer (`gpu.ts` requirement raised
   9 → 10, both the request clamp and the init gate, comments updated);
   the emitter radiosity B (with the sky term folded in, since as an
   emitter the patch shines with everything it reflects) rides a **third
   row** of the existing `radGSky` texture, and the CDF a **fourth**
   (`buildPatchCdf`: one 256-thread workgroup, chunk-scan of ≤ 16
   elements + Hillis-Steele over the 256 chunk sums, dispatched only in
   this mode after `radSolveB`); each patch's RIS weight parks in the
   spare `.w` lane of its B ping-pong slot, so the scan needs no new
   buffer and `radDyn`'s layout is unchanged. `radDyn` was never bound
   as an eleventh buffer. **Not composed with the GI reservoir** — see
   Findings for the honest reason; patchRIS pixels store an empty GI
   reservoir like radiosityRead pixels do.

Two estimator facts the numbers taught, now in the code with the numbers
in the comments: the light reservoirs' `W` cap (24-32) is sized for a
`~1/lightCount` source pdf, but a patch is one of ~3400 with source pdf
`~1e-3..1e-4`, so `W` in the hundreds is patchRIS's normal operating point —
with the cap the mode read relBias **−0.347**, without it **−0.194**
(alongside every other mode); and `SHADOW_CULL` cannot apply to a number
that is the pixel's WHOLE indirect estimate rather than one light among
many, so it is not applied to the patch survivor. The single-candidate
control (M = 1) read even lower before the fixes (indirect mean 0.00035
vs radiosityRead's 0.0080 in the same frame), which is what pointed at
the weight, not the sampling.

## Verification

`npm run typecheck` and `npm run build` clean at every commit. Everything
below ran headless on SwiftShader (Bash sandbox off for Chromium, reason
"chromium needs its own namespaces outside the command sandbox"), small
resolutions throughout, console clean (`errors: []`) in every run quoted.
Screenshots were Read; the descriptions are of what is actually in them.

### Deliverable 0 — the crouch bug, before / after

New tooling (commit `4027994`): `__readFlashmap(layer)` reads a torch
depth-map layer back; `__readRadiosity()` reads the solve's injected
energy E and both B halves; `__freezeClock(on)` feeds every subsequent
frame one timestamp (dt = 0), so two captures of an animated character
are the SAME scene — the `__compareToReference` freeze, generalised, and
the thing that made every delta below exact instead of pose-noise.
`tools/headless/scenarios/crouch-matrix.js` pins the player at the
corridor spawn, freezes the guards with their torches out (the flashlight
is the only moving light) and reports flashlight-on minus flashlight-off
deltas; it works on both sides of the split (drives `indirectMode` when
present, else `radiosity`).

Premise check first — the campaign's diagnosis, verified rather than
assumed: crouched layer-0 coverage varies with the aim yaw and with the
crouch idle clip's arm sway. An aim sweep read 33-86% blocked; one aim
sampled 6× over the settled clip read 88.6-92.9% (the brief's "78-92%" is
this animation range, and a sample taken mid-transition read only 12.8%).
So the crouched coverage is sampled over the clip, then every light delta
is taken with the clock frozen at one settled pose.

| quantity | before (`be298ab`) | after (`85f9362`) |
|---|---|---|
| layer-0 texels < 10 cm, standing | 0.0% | 0.0% |
| layer-0 texels < 10 cm, crouched (min / median / max, 6 clip samples) | 88.6 / 91.1 / 92.9% | 0.0 / 0.0 / 0.0% |
| Σ luminance(injected E) over patches, flash on − off, standing | 58.28 | 58.18 |
| Σ luminance(injected E) over patches, flash on − off, crouched | **0.000** | **33.18** |
| shaft near the player (window mean, volumetric 1.0 − volumetric 0), depth-map march | 0.00002 | 0.01313 |
| shaft near the player, real-ray march (control) | 0.00521 | 0.01069 |

Reconciliation: standing injection is unchanged by the fix at the
**total** level (58.28 → 58.18 is inside the ~3% band two page loads of
the same build straddle — 60.5 vs 62.5 measured by the review). At the
per-patch level the owner-skip does show, and by design: the review's
per-patch diff found 74-86 patches going from E ≤ 1e-4 to E > 1e-3
(Σ ≈ 2.4, ~4% of injected energy) — the standing player's own map
self-shadow (legs/off-arm inside the cone at ~0.5 m) that the self-skip
removes, offset in the total by the far-receiver taps the PCF was losing
(see R3-1); the documented "owner never occludes its own torch" trade,
not a leak into unrelated patches. Crouched injection goes from exactly
zero to 33.2 — 57% of standing rather than 100% because the crouched
beam starts at thigh height and pools nearer the character, injecting
genuinely less; the fix is not inflating anything. The last two rows are
the shaft's own before/after and the honest caveat with it: before, the
depth-map shaft was dead (0.00002) while the real-ray shaft lived
(0.00521); after, the map path lives (0.0131 in that run). Its ratio to
the real-ray shaft is NOT resolved by this measurement — the real-ray
control alone moved 2× between page loads (0.0052 → 0.0107 → 0.0085 →
0.0110 across four runs of two builds; clip-phase/pose noise), so the
same-run map/rays ratio ranged from +58% down to −6%. Direction (map ≥
rays once the owner is ignored, since rays still hit the thigh in front
of the lens) is the physical expectation and holds in most runs; the
magnitude is inside the noise — see Findings. The exact direct light
(RIS survivor + real shadow ray over the 5 cm lens sphere) is untouched
by all of this.

Screens (`docs/campaign/tracks/img/crouch-before.png` / `crouch-after.png`,
default view, 320×164 internal, crops around the player): the crouched
player at the corridor spawn with a bright pool at his feet in both; after
the fix the pool carries a soft haze halo and the crates and legs around it
pick up a warm secondary lift that is absent before — the window around the
player measures 28% brighter (8-bit luma 22.0 → 28.1) with nothing else in
the frame different. The volumetric shaft coming back is the third axis the
brief asked for, and it is the same map read as the inject: the two are
one fix.

### Deliverables 2-3 — per-mode correctness (`__compareToReference`)

Protocol: `__benchResolution(320, 200)` (harness `--bench-res 320 200`,
deadline raised), pinned bench pose, `__compareToReference(configs,
refFrames, testFrames, { bounces: 4 })`. The reference is brute-force
traced at **4 bounces**: reference mode zeroes every reuse/estimator
trick and forces `indirectMode` to `traced` by construction
(`effectiveIndirectMode` returns `traced` under `reference`, and the
compare's own `Object.assign(settings, refOverrides, { reference: true })`
runs before every reference frame) — verified rather than assumed, since
a reference reading the patch solve would validate the approximation
against itself. Each mode is tested at its default operating point
(1 bounce); `traced4` is the truncation control.

Two independent runs (fresh page loads, so different frozen guard-phase
modes — the bimodality Track A characterised; run 1 sits on the ~−0.19
floor, run 2 on ~−0.23):

| config | run 1 (60 ref / 25 test) relRmse / relBias | run 2 (100 / 40) relRmse / relBias |
|---|---|---|
| traced, 1 bounce | 1.305 / −0.185 | 1.365 / −0.227 |
| traced, 4 bounces (control) | 1.301 / −0.162 | 1.351 / −0.205 |
| radiosityRead | 1.300 / −0.185 | 1.355 / −0.230 |
| gather | 1.306 / −0.176 | 1.358 / −0.221 |
| patchRIS | 1.304 / −0.194 | 1.358 / −0.241 |

(Run 1: traced1/traced4/radiosityRead/gather are from the first 5-config
run; that run predates the patchRIS estimator fixes, so the patchRIS
run-1 figure is from a same-protocol 3-config rerun after them. gather's
code did not change between the two.)

**Post-review rerun** (same protocol, 60 ref / 25 test, after every
Review-resolution fix — corrected reservoir merge included):

| config | relRmse / relBias |
|---|---|
| traced, 1 bounce | 1.203 / −0.260 |
| traced, 4 bounces (control) | 1.191 / −0.239 |
| radiosityRead | 1.170 / −0.159 |
| gather | 1.174 / −0.154 |
| patchRIS | 1.175 / −0.155 |
| traced, 1 bounce, GI off (attribution rerun) | 1.197 / −0.255 |

The patch trio now sits together at −0.155..−0.159 and the traced pair at
−0.24..−0.26. The gap is **not** the GI merge fix: with GI off, traced-1
reads −0.255 too. It is the traced indirect's own energy loss (firefly
clamp / RIS caps on its high-variance bounce estimate) which the trunk's
reservoir merge had been over-brightening by exactly the sparse-reuse
inflation the review found (R1-1) — two biases cancelling at this pose,
one of them a 4× error in the flashlight-only room. The traced modes are
the truncation controls here, not candidates for the default.

Reading. The relRmse column is flat (all within 0.06, under the ±6%
same-build floor): at 320×200 / ≤ 40 test frames this protocol measures
its own noise more than any mode. relBias is the signal: **every mode
sits within 0.02 of traced-1 in the same run**, and traced-4 sits
0.02-0.023 above traced-1 in both — the deeper-bounce energy the
1-bounce configs lack. gather does not visibly recover that gap here
(0.006 above traced-1 in run 2, inside the noise) even though it reads
the infinite-bounce solve at x1; the compare cannot resolve 0.01, and the
flashlight-only shot below shows the multi-bounce recovery the numbers
can't. patchRIS reads 0.009-0.014 below traced-1 in both runs — the one
consistent (if sub-floor) hint of residual energy loss, plausibly its
higher-variance bright tail meeting the luminance-3 firefly clamp and the
history clamps (its 26% empty-proposal pixels are variance, not bias);
worth a longer run on the Mac's converge before believing.

Per-mode stills at this same pinned bench pose (0.5 scale, clock frozen,
30 frames, one PNG per mode, all Read): the north cubicle farm under the
window moon-pools with a lit floor pool lower-left and a guard mid-frame —
and the four are visually indistinguishable at that scale. That is a
finding about the bench pose, not the modes: it is dominated by direct
moonlight and lamps, so it is the wrong place to see indirect differ. The
flashlight-only conference-room shots below are where the modes come
apart, which is why they carry the headline.

What this table is FOR, stated plainly: it validates that no mode is
broken. It caught patchRIS at **−0.347** (twice) before the `W`-cap fix
and −0.16 of that was one line; that is the class of error it resolves.
It cannot rank correct modes 0.01 apart, and this report does not
pretend it can. The −0.19..−0.23 floor itself is systemic (traced-4 sits
on it too) — see Findings.

### Per-mode work counters (the cost story)

Pinned bench pose, `__benchResolution(384, 240)`, `__bench(3, true)` with
counters on, defaults (1 bounce, 1 spp, 8 candidates, 6 spatial taps).
Per pixel; block frame 22 in every run. Same protocol as Track A's
baseline table, so the radiosityRead column reproduces it (31.0 BVH
visits, 3.37 gathers, 0.855 direct shadow rays).

| slot | traced | radiosityRead | gather | patchRIS |
|---|---|---|---|---|
| raysDepth1 (bounce rays) | 0.833 | 0.002 | 0.833 | 0 |
| shadowIndirect (x1 NEE) | 0.632 | 0.002 | 0.628 | 0 |
| shadowGI (reuse revisibility) | 0.724 | 0.001 | 0.825 | 0 |
| shadowPatch (patch survivor) | 0 | 0 | 0 | **0.643** |
| risCandidatesIndirect | 6.62 | 0.019 | 6.61 | 0 |
| risCandidatesPatch | 0 | 0 | 0 | **6.93** |
| patchCdfTaps (texture reads) | 0 | 0 | 0 | **96.6** |
| radiosityGathers (texture/patch reads) | 0 | 3.37 | 3.25 | 11.4 |
| bvhNodeVisits | 68.5 | **31.0** | 69.8 | **41.8** |
| slabTests | 142.2 | 61.6 | 145.1 | 83.1 |
| obbTests | 7.70 | 4.23 | 7.39 | 4.67 |
| shadowDirect / risCandidatesDirect | 0.855 / 6.93 | 0.855 / 6.93 | 0.855 / 6.93 | 0.855 / 6.93 |

Post-review rerun of the same protocol, deltas only (everything not
listed is identical to the third digit): patchRIS `patchCdfTaps` 96.6 →
**82.7** (the per-candidate CDF-difference read is gone) and
`radiosityGathers` 11.4 → **17.2** (row-2 read now counted per candidate
alongside the patch-geometry read — the same number of texture fetches,
booked into the honest column); radiosityRead's near-zero trace counters
rise to raysDepth1 0.022 / shadowIndirect 0.016 / risCandidatesIndirect
0.173 because its ~2% of unpatched static faces now trace their bounce
instead of rendering black.

Reconciliations, counter against counter:
- gather costs exactly a traced-1-bounce frame: same bounce rays (0.833),
  same x1 NEE (0.63), same RIS candidates (6.6), +1.4 BVH visits; its only
  extras are 3.25 patch-texture reads/px and slightly more GI revisibility
  rays (0.825 vs 0.724 — the gathered x1 makes more reused samples valid).
  "Radiosity quality of indirect at traced-1-bounce cost" is the counter
  statement of the trade.
- patchRIS spends **zero bounce rays** and **zero x1 NEE**: its ray budget
  over radiosityRead is one shadow ray at 0.643/px (10.8 more BVH visits,
  21 more slab tests). risCandidatesPatch / 8 = 0.866 = primaryHits: every
  hit proposes 8 patches; shadowPatch (0.643) < primaryHits because 26% of
  hits reject all 8 candidates (behind or coplanar with the receiver —
  Findings). Its non-ray cost is real and named: ~83 CDF taps + ~17
  patch/emitter reads per pixel (≈ 100 dependent texture reads, post-fix
  bookkeeping). Which of "37 BVH visits + 80 slab tests + a bounce ray's
  incoherence" (gather) versus "~100 dependent texture reads, zero
  incoherent rays" (patchRIS) is cheaper is precisely the question
  SwiftShader cannot answer — see the Mac list.
- The direct-light column is identical in all four modes to the ray, as
  it must be: nothing here touches `restirDirect`.

### The headline: the character shadows the bounce light

`tools/headless/scenarios/corner-shot.js`: conference room, every
practical and the moon switched off (`setStaticLightIntensity(i, 0)` for
all, sky/ambient 0), so the flashlight is the ONLY light and every
photon on the walls is bounce. Player 1.2 m south of the north wall,
facing south, beam pooling on the red carpet ~4 m ahead; camera distance
9; the guards frozen with torches out; clock frozen after the pose
settles; 25 frames accumulated; exposure 3.0 (8.5× default, stated
because it is a debug view). Full-composite view, not "indirect only":
with the flashlight the sole light, everything but the pool IS indirect,
and this view is what a player sees. Grid at
`docs/campaign/tracks/img/corner-modes-grid.png` (radiosityRead, gather /
patchRIS, traced-1), difference heat at
`img/corner-gather-minus-radiosityRead.png` (grey = zero, ×3 gain).

What the images show, honestly: this is a soft occlusion, not a hard
silhouette — the source is a broad, low pool, so the character's shadow
in bounce light is a diffuse darkening. In radiosityRead the wall behind
the character and the floor around his feet are lit exactly as if he
were not there (the bake never saw him). In gather the difference image
puts a coherent darker region on the wall and floor immediately around
the character — an 8×5-block mean-difference grid reads −4 to −14 (8-bit
luma, on a ~45 mean) over the region up-right of him where his body sits
between the pool and the wall, and the character's own silhouette shows
as speckle (his surface shades differently, dynamic hit). patchRIS shows
the same signed structure through its shadow ray (−6 to −13 in the wall
column behind him). traced-1 does too, but its far wall is near-black
because one bounce is not enough light for a room lit by a pool. Two
side observations from the same grid: gather's dark far wall carries
sparse fireflies (the x1 NEE catching the beam edge; the luminance-3
firefly clamp is the existing control), and patchRIS renders the
pool-facing walls brighter than radiosityRead — with no reference at
this pose that is left as an observation, and the pinned-pose RMSE table
below is the arbiter of who is right in general.

## Recommendation for the DEFAULT

**`gather`**, with the trade stated: it costs exactly a traced-1-bounce
frame (counters above) — real rays where radiosityRead spends none — and
in return the estimator sees the character (body shadows in bounce
light, the demo's motivating flaw fixed) and lights the character with
the room's bounce; the solve's baked-static error moves one bounce away
from the primary hit where it is diffused; and it inherits infinite-
bounce completeness that traced-1 lacks (traced-1's far wall is black in
the flashlight-only room). It is also the only mode that survives the
energy audit in that room: against a converged reference render, gather
reads within a few percent of the truth while both direct solve-readers
run hot (radiosityRead +17%, patchRIS +47% — Review resolution R1-3),
because gather traces the light's first arrival for real and reads the
solve only from x1 onward. It is a strict superset of what the campaign
asked this track for. `radiosityRead` stays the cheapest fallback (zero
bounce rays, 31 BVH visits/px) for a low-end preset. `patchRIS` is the
promising one and deliberately not the default: it spends no bounce
rays and lights dynamic hits directly at the primary vertex, but it is
the most faithful — hence hottest — reader of a solve whose injected
energy is not yet calibrated in the concentrated-pool scene, its ~100
dependent texture reads/px need the Mac to price, its proposals waste
26% of pixels' candidate sets (Findings), and it is not composed with GI
reuse. Fix the solve's injection calibration and give it a
receiver-aware proposal, and it is the mode to revisit for the default.

## What SwiftShader cannot say, and the Mac bench script

Everything above is rays and counters. Milliseconds — the incoherent
bounce ray in gather versus patchRIS's dependent texture reads versus
radiosityRead's nothing — only mean something on the M1 Max. In Chrome
on the Mac, at the standard bench resolution (do NOT call
`__benchResolution`; the standard 1152×720 is what compares to the
README's table), console:

```js
// once per mode, note fps / wallMsPerFrame / gpu.pathtrace
__settings.counters = false; __settings.bounces = 1;
__settings.indirectMode = "traced";        JSON.parse(await __bench(60))
__settings.indirectMode = "radiosityRead"; JSON.parse(await __bench(60))
__settings.indirectMode = "gather";        JSON.parse(await __bench(60))
__settings.indirectMode = "patchRIS";      JSON.parse(await __bench(60))
// the checkerboard question for gather at 2 bounces (rate becomes live):
__settings.bounces = 2; __settings.indirectRate = 0.5;
__settings.indirectMode = "gather";        JSON.parse(await __bench(60))
__settings.indirectMode = "traced";        JSON.parse(await __bench(60))
// counters ON is contention-inflated garbage for timing — leave it off.
```

Report per mode: `fps`, `wallMsPerFrame`, `gpu.pathtrace`, and for
patchRIS also `gpu.radCdf` (the CDF pass, one workgroup — should be
noise) and radiosityRead/gather/patchRIS `gpu.radInject / radSolveA /
radSolveB` (unchanged from trunk, ~fixed cost). The decision this buys:
whether patchRIS's ~108 texture reads/px cost less than gather's bounce
ray + NEE on real silicon — the one number counters cannot supply.

## Findings (not fixed)

- **Self-skip means the owner never occludes its own torch in the map's
  consumers.** The pre-fix pinhole map under-counted (0-10% open where
  the jittered 5 cm lens sphere is ~60% open); the post-fix map
  over-counts (100% open through the owner's own thigh/arm). Measured:
  crouched depth-map shaft 0.0131 vs real-ray shaft 0.0107 (~23% bright).
  The consumers are the radiosity inject (approximation by construction),
  the beam march's map path (declared approximation, off in reference)
  and the RIS target (unbiased whatever the map says). The exact direct
  light keeps the character's real self-occlusion via its shadow rays.
  This is the trade the brief chose (same as the gameplay probe's), stated
  rather than hidden.
- **patchRIS proposal waste.** 26% of primary hits get no survivor at
  M = 8 (shadowPatch 0.643 vs primaryHits 0.866): candidates behind the
  receiver's plane or coplanar with it contribute correct zeros, so this
  is variance, not bias — but it is a lot of variance. A receiver-aware
  proposal (three or four CDFs bucketed by patch normal orientation,
  chosen against the receiver's normal) would cut the waste without
  touching the estimator's shape; not done here.
- **patchRIS × GI reuse not composed.** The survivor's radiance `B_j/π`
  is a legitimate view-independent x2 sample, but the GI reservoir expects
  a solid-angle-sampled x2 with the reconnection-shift Jacobian, and
  folding a CDF-area-sampled point sample's `W` into the merge weights
  honestly is exactly the "if the merge weights are not clean, land
  without and say so" case — landed without. patchRIS pixels write an
  empty GI reservoir.
- **patchRIS omits sky irradiance on dynamic hits** (no patch data on a
  character to interpolate the sky row from). Static hits get it. Sky is
  0.04 and mostly zero indoors; the miss is real but small.
- **Deleting gather's x1 NEE — proposed, not done.** The solve's injected
  E row is x1's direct light with baked visibility; adding
  `albedo·(E + G + sky)/π` at x1 instead of NEE + `albedo·(G + sky)/π`
  would save the x1 NEE (0.63 shadow rays/px, 6.6 RIS candidates/px, and
  most of gather's extra traversal) at the cost of second-order character
  shadows (a character between a lamp and the wall would still darken
  the wall's direct light, but not the bounce off the floor it stands
  on). It needs the E row exposed like the B row now is (one more radGSky
  row) and a `__compareToReference` config pair to price the bias; the
  counters above already price the saving.
- **The relBias floor is systemic, and it is two floors, not one.** After
  the merge fix, the patch-fed modes sit at −0.155..−0.165 at the moonlit
  pose and the traced modes at −0.24..−0.26 (GI on or off — measured
  both): the accumulator/clamp floor plus the traced indirect's own
  clamp losses. Before the fix the two coincided at −0.19 because the
  reservoir merge's sparse-bright inflation was refilling exactly what
  the clamps take. Track B4 (temporal) territory, now with the traced
  path's loss standing exposed rather than accidentally patched over.
- **The solve's injected energy runs hot in the concentrated-pool scene.**
  Flashlight-only conference room, indirect-only view, against a
  reference-mode render (no reuse, no accumulator heuristics, 4 bounces,
  120 frames): gather −4%, radiosityRead **+17%**, patchRIS **+47%** (see
  R1-3 for the table and why the readers order that way). gather traces
  the pool's first bounce for real and only reads the solve from x1 on;
  the two hot modes read the injected E directly (patch-averaged, then
  bilinear — or exactly, per pixel). The excess is upstream of every
  reader: the depth-map/centre-sampled `inject` of a ~2 m pool onto
  0.3-0.5 m patch cells is the only stage they share and gather does not.
  Untouched by this track (the solve is not this track's), named here
  because both direct readers inherit it.
- **Temporal DI reuse still inflates a lone bright light** (+17..34% in
  the flashlight-only room with `restirTemporal` on) — the "temporal W
  inflation" the trunk already measured (+0.23 relBias) and defaults OFF
  for. The support-aware merge (R1-1) fixes the *spatial* path the
  defaults use, not this. Not this track's; recorded so nobody reads the
  merge fix as blanket permission to re-enable temporal DI.
- **`finalizeGIReservoir`'s W cap (32) costs energy at deeper bounces
  with reuse effectively off** (`restirGI` on, 0 spatial taps, temporal
  off): 4-bounce indirect reads 24% below the same estimator with the
  reservoir disabled, in the flashlight room — the fresh sample alone can
  legitimately exceed W = 32 at 4 bounces. Nobody runs that config; noted
  for whoever revisits the caps.
- **Trace pass at 10 storage buffers = the coordination ceiling.** The
  Track A watch item (9 vs the default 8) gets one step worse: devices
  reporting 8 or 9 now fail the `gpu.ts` gate (they failed at 9 before
  Track A too). The counters-optional-layout idea in Track A's report
  would win back a slot; a texture-based patch-geometry path (two more
  radGSky-style rows) would win back binding 16's slot if a future track
  needs it.
- **`__compareToReference` guard-phase bimodality** (Track A) applies to
  every table here; the reruns quoted show the spread.
- **`corner-shot.js` is parameterised by editing its top-of-file defaults**
  (mode/view/exposure) — a JS scenario is one expression and cannot take
  arguments; the README says so.

## Merge notes

- **Uniform tail:** this track wrote bytes 864-879 as four u32
  (`indirectMode`, `radPatchCount`, `_padIM0`, `_padIM1`) and set
  `UNIFORM_SIZE = 880`. The volumetrics track's block is 880-943 by
  contract; on merge the WGSL struct order must be `..._padRad2`, this
  track's four u32, then their block, and `UNIFORM_SIZE = max(880, 944)
  = 944`. If their branch put 16 bytes of padding at 864-879 to reach 880,
  those pads are what this track's fields replace. `writeUniforms` here
  writes `u[216], u[217]` (bytes 864, 868) — index-stable under that
  merge.
- **Trace pass `@group(1)`:** this track added binding **16** only
  (`radStatic`, `read-only-storage`); 17 is unused; 14/15 are theirs.
  `ptLayout` entries and the bind-group entries merge by union.
- **`counters.ts`:** three slots appended at the END
  (`risCandidatesPatch`, `shadowPatch`, `patchCdfTaps`). Any other
  track appending slots is a textual union; the WGSL constants are
  generated, so order only has to be consistent, not preserved.
- **`pathtrace.wgsl` `main`:** the indirect gates now key on
  `skipTracedIndirect` (`radioStatic || patchRISMode`) rather than
  `radioStatic`; `gatherMode` adds one block after the NEE block. A track
  touching the bounce loop (temporal?) will conflict textually here;
  the semantic rule for resolving is "steady traced indirect is gated by
  `skipTracedIndirect`, transient/direct are not".
- **`settings-store.ts`:** REVISION → 4 and `STALE_KEYS` became
  revision-keyed. If another track bumped REVISION, take the max and keep
  both revisions' entries; the radiosity→indirectMode migration is
  guarded by `storedRev < 4`.
- **`renderer.ts`:** `radGSky` texture height 2 → 4; new
  `radCdfPipeline`; `radStaticBuffer` retained for the trace pass;
  `flashmapLayout` gained binding 2 (owner-group uniform); the storage
  buffer request/gate in `gpu.ts` is 10.
- **`character.ts` / player**: untouched (crouch pose left as is, per the
  brief). `guards.ts` gained only `torchGroups()`.
- **Shared reservoir code (post-review, coordinator conversation).** The
  review-resolution merge fix (R1-1) lives OUTSIDE this track's lanes and
  overlaps Track B4 (temporal) territory by nature — it is the reuse
  denominator: `pathtrace.wgsl` `restirDirect` and `restirGI` gain a
  support-aware Z re-weight after selection (new helpers `pixelWorldPos`,
  `diSupports`, `giSupports`), while `mergeReservoir` (common.wgsl) and
  `giMergePrev` keep their trunk bodies (comment-only edits). The merge
  invariant for any track touching reuse: **a merged stream contributes
  its M to the survivor's denominator iff its own shading point could
  have proposed the surviving sample with a nonzero target, and never if
  its own shadow ray killed it.** If B4 restructures the reservoirs, that
  is the property to preserve; the numbers that justify it are in R1-1.
- **Other shared-file touches (post-review):** `radiosity.wgsl`
  `torchVisPoint` gained a receiver-normal parameter and a slope-scaled
  compare slack (R3-1); the `solve` writes each patch's RIS weight into
  row 2's spare `.w` lane (in addition to the bOut ping-pong lane the CDF
  scan reads) — a lane nothing else uses; `settings-store.ts` keys the
  `radiosity` → `indirectMode` migration on the field's absence instead
  of `storedRev < 4` (R2-1), so it no longer matters whose REVISION bump
  wins on merge.

## Review resolution

Three adversarial reviews ran on `f71d8b9` (estimator correctness;
plumbing and limits; Deliverable-0 verification). Every finding below was
re-derived from the code and re-measured before anything was changed;
"real" rows name the fix, and no reviewer's root-cause line was trusted
until my own measurement confirmed it. Headless SwiftShader throughout,
small resolutions, console `errors: []` in every run quoted.

| id | finding | verdict | resolution |
|---|---|---|---|
| R1-1 | **HIGH** — ReSTIR spatial reuse inflates sparse-bright indirect ~4× (traced-1 + GI in the flashlight room); gather's "composes with zero changes" never validated. Root cause proposed: dead streams excluded from the merge denominator. | **real** — reproduced exactly (traced-1 indirect 0.00904 with 6 taps vs 0.00217 with 0 taps; gather +10%); the reviewer's mechanism is right and **its DI twin was inflating the shipping default's flashlight pool 2.3× in the same room** — found while triaging, fixed together. | Support-aware merge denominator (Z-MIS) in `restirDirect` and `restirGI`; validated against reference-mode renders, not against the config it replaced (below). |
| R1-2 | MED — patchRIS near-field kernel dims; "same guard bakeFF uses" is false (π× stronger). | **real**, and the reviewer's deeper claim (the jittered point already integrates the patch, so ANY guard is loss) checks out numerically: against exact finite-square form factors the jittered bare kernel matches to 5 cm, an A/4 guard reads −38% at 5 cm, a full-disk A guard −60%. | Guard removed (`d² = d²raw`); comment states why. patchRIS's reading rose (+33% → +47% over the reference) — the dimming had been masking part of the excess R1-3 exposes, not fixing anything. |
| R1-3 | MED — the beam-pool energy sanity check was not run; three readers of one solve spread 16-40%. | **real** — run against reference-mode arbiters (below): gather ≈ truth, radiosityRead **+17%**, patchRIS **+47%**; and radiosityRead's black-but-"valid" faces below the patch area floor were a defect. | Unpatched static faces are no longer radiosityRead pixels (they trace, validity honest — counters show them). The reader spread is disclosed with its arbiter and traced to the solve's injected E, upstream of every reader (Findings); gather is the only reader that survives it, which sharpens the default recommendation rather than changing it. |
| R1-4 | LOW — survivor probability from a CDF difference floored at 1e-9: chunk seams can round negative → an unbounded 1/p firefly. | **real** (plausible, mechanism sound; not reproduced) | `p_j` read exactly from row 2's spare `.w` lane (solve writes `luminance(B)·A` there too); a zero-weight pick — reachable only via rounding — is skipped. One fewer texture read per candidate. |
| R1-5 | LOW — gather needlessly excludes dynamic primaries; only x1 must be static. | **real** (design gap) | `gatherMode` keys on `primary.valid`; characters take the room's bounce at x1. The report's "patchRIS uniquely lights dynamic hits" is corrected. |
| R1-6 | LOW — sky bounce order differs across the three readers. | **real, negligible** | Documented, no code: the emitter row folds x1's sky (patchRIS gets one extra sky bounce), the transport E/G stay sky-free by design, sky = 0.04 and window-only. Folding sky into the transport is a solve change, out of scope. |
| R2-1 | LOW — migration keyed on `storedRev < 4`, order-dependent against another track's REVISION bump. | **real** (plausible) | Keyed on the field's absence (`!("indirectMode" in stored) && radiosity === false`). |
| R2 (rest) | Uniform bytes 864-879, size 880; 10 storage buffers = SwiftShader's exact ceiling; row/CDF timing; flag split; migration cases; init clean. | **confirmed correct**, kept as the plumbing baseline. | — |
| R3-1 | MED — the 4-tap PCF changes far-receiver falloff in the inject; the report claimed "same taps/slack" and never checked. | **real** — my emulation shows the fixed slack (`1.02·d + 0.10`) failing at grazing far receivers for BOTH nearest and bilinear taps (0.80/0.87 mean visibility at 15 m level aim; 0.38/0.63 pitched), so it predates the PCF, but the report's specific "same falloff" claim was unverified either way. | `torchVisPoint` gained a slope-scaled slack (receiver-normal + texel-footprint depth extent, capped 1 m); emulation reads 1.000 through 15 m; standing injection +5.8 (≈ +9%, concentrated in previously fractional-visibility patches — the recovered taps — plus 23 newly lit low-energy patches). The image path's `torchMapSample` (unbiased target only) is untouched and the doc comment now says so. The slack is the map's own resolution limit made explicit: at 10 m grazing a texel footprint IS ~0.5 m of depth. |
| R3-2 | LOW — "the group skip leaks no energy into the standing case" is false per patch (74-86 self-shadowed patches go dark→lit, Σ≈2.4). | **real** (report defect, not code) | Reconciliation paragraph rewritten: the total-level statement stands (inside the ~3% cross-load band), the per-patch one is now the documented owner-self-occlusion trade with the reviewer's numbers. |
| R3-3 | LOW — "map ~23% brighter than real rays" is inside the shaft measurement's own 2× pose noise. | **real** (report defect) | Rewritten as a range with the four control readings; direction argued physically, magnitude declared unresolved. |
| R3-4 | LOW — a dragged corpse is not the owner's group, could re-box the flashlight while carrying. | **false positive as stated** — the beam IS emitted while dragging (the emissive lens box is what's stowed), but the dragged body trails ~0.85 m along the floor in front of the player (`main.ts` drag rule), never over the lens. A body at that range is a real occluder the exact shadow rays also see; the self-skip exists only for geometry ON the lens (the crouch case). No change. | — |
| R3-5 | info — cross-load repeatability of the standing injection sum is ~3% (60.5 vs 62.5), unstated. | **accepted** | Stated next to the 58.28 → 58.18 row and used as the band in R3-2. |

### R1-1 — the reservoir merge fix, in detail

**Mechanism.** Both `mergeReservoir` (DI) and `giMergePrev` (GI) returned
early for a dead stream (`W <= 0`), so its M never reached the merge
denominator. In the sparse-bright regime — one flashlight pool in a
black room, where most streams find nothing — that lets each rare
bright find be re-adopted by every dark neighbour at near-full weight.
The naive alternative (count every dead stream's M) is not right
either: it dilutes the beam-edge pixels by their out-of-cone neighbours,
whose domains could never have proposed the sample — measured −0.38
relBias at the flashlight-only bench pose during triage. The invariant
that satisfies all three regimes is the standard support test: **a
merged stream votes M_i in the denominator iff its own shading point
could have produced the surviving sample with a nonzero target**
(cosine hemisphere; plus the spot cone for a spot/flashlight sample),
**and never if its own shadow ray killed it** (that verdict is about
ITS visibility, and dropping such streams is the trunk's tuned
shadow-edge behaviour, kept). Implemented as a post-selection Z
re-weight in `restirDirect`/`restirGI` (neighbour contexts recorded
during the taps, judged once the survivor is known); the merge helpers
themselves are back to their trunk bodies.

**Validation — against reference-mode renders, not against the code it
replaced.** Flashlight-only conference room (every practical, sky and
ambient at 0), corner-shot pose, clock frozen, 384×197, 25 accumulated
frames per config after a history flush; the arbiters are the same pose
in reference mode (no reuse, no accumulator heuristics; 80-120 frames):

| quantity (mean HDR luminance) | trunk `f71d8b9` | after | reference truth |
|---|---|---|---|
| indirect-only, traced-1, GI on (6 taps) | 0.00904 | **0.00306** | 0.00307 (1-bounce ref) |
| indirect-only, traced-1, GI on (0 taps) / GI off | 0.00217 / 0.00221 | 0.00217 / 0.00221 | (accumulator eats −28% of its variance) |
| indirect-only, traced-4, GI 6 taps / GI off | 0.01155 / 0.00382 | **0.00383 / 0.00383** | 0.00606 (4-bounce ref) |
| indirect-only, gather, GI 6 taps / off | 0.00639 / 0.00578 | 0.00632 / 0.00584 | 0.00606 |
| direct-only, default DI (spatial 6, temporal off) | **0.03745** | **0.01427** | 0.01608 |
| direct-only, DI temporal on (both / temporal-only) | 0.02773 / 0.02006 | 0.02153 / 0.01883 | 0.01608 |
| direct-only, DI reuse off | 0.00538 | 0.00538 | 0.01608 (its variance is eaten by the accumulator: expected) |

GI: 4.1× → 1.00× of the reference at 1 bounce; at 4 bounces GI-on and
GI-off are now the same number to five decimals. DI (the shipping
default config): +133% → −11%, i.e. onto the normal-mode accumulator
floor. Temporal DI still inflates (+17-34%) — that is the pre-existing
temporal-W behaviour the trunk defaults off (Findings), untouched here.

At the compare protocol (same reference for every config, bench pose):
moonlit, traced-1 GI-on −0.260 vs GI-off −0.255 — **GI is neutral**;
flashlight-only, traced-1 GI-on −0.371 = GI-off −0.371 (trunk at the
same protocol: +0.459 vs +0.204, a +0.255 GI inflation). The −0.10 gap
between the traced pair and the patch trio at moonlit is the traced
indirect's own loss, previously refilled by exactly this inflation —
see the note under the post-review compare table.

### R1-3 — the energy audit that had not been run

Same room, same arbiters (`ref4` = reference mode, 4 bounces, 120 frames
= 0.006057; the room's infinite-bounce truth sits a few percent above
it): gather 0.005843 (**−4%**), radiosityRead 0.007115 (**+17%**),
patchRIS 0.008878 with the unguarded kernel (**+47%**; +33% with the old
A/4 guard, whose dimming was cancelling part of it). Normal-mode
readings can only sit BELOW their expectation (the accumulator's clamps
subtract), so the two positive numbers are lower bounds on those
readers' bias here. gather traces the pool's first arrival with real
rays and reads the solve only from x1 onward; the other two read the
injected E at the primary (patch-averaged/bilinear, or exactly per
pixel) — the one stage they share and gather does not is the depth-map,
centre-sampled `inject` of a ~2 m pool onto 0.3-0.5 m cells. The
readers order by how faithfully they transport the solve's own energy,
patchRIS most faithful (exact per-pixel geometry, live shadow ray)
hence hottest. Untouched (the solve is not this track's); named as the
next calibration to make, and it does not overturn the default —
gather is the reader that survives it.

### Fresh verification (branch tip)

- `npm run typecheck` and `npm run build` clean.
- **Deliverable 0, rerun** (`crouch-matrix.js`, 320×164): standing/crouched
  layer-0 coverage < 10 cm 0/0 (fix intact); crouched flash injection
  40.43 (the slope slack recovers far-floor taps; the trunk read 32-33
  across the review's and this track's runs), standing 61.04 (trunk
  60.5-62.5 across loads; the +5.8 slack recovery sits on top of the ~3%
  band); crouched shaft 0.01031 (map) vs 0.01100 (rays) in this run —
  map 6% BELOW the rays, further evidence that ratio is not resolved
  (R3-3). Screenshot Read (`crouch-after` scratch shot): the crouched
  player at the corridor spawn, bright warm pool at his feet with the
  soft haze halo over it, warm secondary lift on the crates and legs.
- **`__compareToReference`**, moonlit (report protocol, 320×200,
  60/25) and flashlight-only bench (240×150, 80/40, GI on/off pairs) —
  tables under Deliverables 2-3 and R1-1. Every mode within its
  cluster; no mode broken; GI on/off equal to ±0.005 in both scenes.
- **Counters** (384×240, `__bench(3, true)`): deltas listed under the
  counter table; traced/gather/direct columns unchanged to the third
  digit.
- **Headline shot, re-shot** (`img/post-review-corner-{gather,radiosityRead}.png`,
  composite view, exposure 3.0, both Read): the north wall of the
  conference room with the player standing right of the pillar, beam
  angled down-left into a bright pool on the beige floor tile, red
  carpet dark; the walls near the pool glow soft grey in both. In
  radiosityRead the pillar's top face and the wall column directly
  behind the player are lit exactly like their neighbours — the bake
  never saw him. In gather that wall column reads darker where his body
  sits between the pool and the wall, and — new since the review — his
  own body is lit from the front by the pool's bounce (visibly brighter
  torso/leg than in radiosityRead), the pillar's top face reads grey via
  the ceiling read instead of black, and the unpatched black faces are
  gone from radiosityRead's static geometry (they trace). The
  patchRIS panel (scratch) renders the pool-facing walls hotter than
  either — the +47% of R1-3, on the screen.

### Below 100%, with the reason

- **The compare tables cannot rank the correct modes.** relRmse is flat
  to ±0.03 at these frame counts; only relBias reads, and its floor is
  the accumulator, not the modes — same ceiling as the original report,
  now with two floors instead of one (patch modes −0.155, traced modes
  −0.25). Ranking gather vs radiosityRead vs patchRIS needs the
  reference-mode room audit (done here for one pose) or the Mac's
  converge, not this table.
- **The solve's injection calibration** (radiosityRead +17%, patchRIS
  +47% in the pool room) is diagnosed to the stage but not fixed — it is
  the solve, which this track does not own. It is the reason patchRIS
  cannot yet be a default candidate on correctness grounds.
- **The map-vs-rays shaft ratio** is unresolved (its own control moves
  2× between page loads); direction only.
- **Temporal DI reuse still inflates a lone light** — pre-existing,
  defaults off, deliberately not folded into the merge fix.
- **The Z re-weight's neighbour position is rebuilt from last frame's
  depth against this frame's camera** — exact when the camera is still
  (all measurements here), a sign-test tolerance in motion. If a moving
  camera ever shows edge crawl in reused direct light, this is the term
  to give the previous-frame camera.

Scratch (scenarios + JSON + shots, not committed):
`<scratch>/rr/`
(`energy*.js/json` room energies + `energyRef/refD` reference arbiters,
`di*.js/json` DI axis, `cmpA*/cmpB*/cmpA-ext` compares, `counters-after`,
`corner-*.png`, `perpatch*` per-patch injection vectors, `pcf_emul.py`
and `ff_check.py` the two numeric emulations); pre-fix baseline build at
`.../scratchpad/before-f71` (git archive of `f71d8b9`).
