# Track B2a — volumetric renderer: every light, in the room, in colour

Branch `claude/volumetrics`, base `be298ab` (campaign trunk with Track A
merged). Commits: `d06a355` slab-clipped march, `d629ce9` coloured
in-scatter channel + denoise chain, `f103fc9` baked static light volume,
`586b464` extinction, `d63a7b8` density interface (smokeVolume contract),
`410ef34` CPU smoke readback, `981ebd4` fog-as-density + deterministic light
taps + tuning, `9e6ffb2` comment wording, plus the report/scenarios commit.

## What changed and why

1. **The march covers the slab, not the sky.** The camera march is clipped
   to the medium's AABB (the room's air: x ±26, z ±18, y 0-3.25) instead of
   the first 26 m of the ray. The camera sits ~20 m up, so the same 12 steps
   now sample the room ~7x denser (dt 0.31 m at the pinned pose vs 2.2 m
   before), and pixels whose ray never enters the room pay no steps (10.4
   steps/px vs 12.0 at the pinned pose).
2. **A real volumetric radiance channel.** `RAW_ILLUM_LAYERS` 3 → 4: layer 3
   carries RGB in-scattered radiance with the camera-ray transmittance in
   alpha, written by the march. It has its own reproject slot ([4], with a
   reference twin [5]: 12-frame history, neighbourhood clamp ON, alpha
   floor 0.08) and its own 4-iteration a-trous chain (strides 2/4/8/16, no
   luminance edge stop, relaxed geometry terms). Composite is now
   `color = surface * T + inscatter`. Removed: the alpha-of-direct scalar,
   its unclamped 48-frame history in reproject, `volStrength`, and the
   shared-tint reconstruction — a warm torch and a cool one keep their own
   colours in the air, and transient flashes scatter in their own colour
   through the same channel (its short clamped history takes the glow up and
   down with the light). `settings.volumetric` is now the medium's
   extinction coefficient per metre at unit density (default **0.05**; with
   the default fog density 0.55 the mean sigma is ~0.028/m — tuned by eye
   against the pinned pose: 0.05 mean sigma veils the whole floor, 0.02
   leaves the moon pools with no halo; the chosen value puts a soft skirt on
   the pools and a glowing shaft in the torch beam while taking ~10% off a
   3.7 m through-slab view path). `fogAmount` became the ambient haze's mean
   *density* (0 = no medium, the control the brief's shafts-vanish shot
   needs) rather than the noise's texture strength; the noise still supplies
   the texture. Settings revision bumped so stale stored values reset.
3. **Every light scatters.** Static lights (moon + all 28 practicals) via a
   baked static light volume: a 104x8x72 `texture_3d` (rgba16float,
   cell 0.5 x 0.406 x 0.5 m over the medium's box) holding, per voxel, the
   isotropic in-scattered radiance per unit sigma — sum over static lights
   of colour x intensity x falloff x spot x traced visibility, 6 jittered
   emitter samples per light, folded through 1/(4 pi). Isotropic phase for
   this ambient sum is deliberate (there is no per-light direction left to
   feed an anisotropic phase). Baked once at init (5.7 s on SwiftShader after
   the 10 s radiosity bake) and re-baked whole on the next frame after any
   static intensity changes — `rebakeLightVolume()` is called from
   `setStaticLightIntensity`, so the OCP darkening a lamp, a shot-out
   fixture and a restore all coalesce into one dispatch per frame. The bake
   traces the static scene only (`occludedStatic`, a new DYN_GROUP_ALL
   sentinel on `occludedSkipping`), so a character standing in a beam at
   rebake time is not frozen into permanent shadow. Sampled trilinearly per
   march step at binding 10 (0.5 m voxels: the pool edges the moon throws
   through the windows are soft by that much). Dynamic lights stay in-march
   exactly as before — player flashlight and guard torches through their
   depth maps with the HG phase, transients through real shadow rays — all
   now writing RGB into the new channel.
4. **Extinction.** The camera march accumulates transmittance in closed
   form per step (albedo 1, so sigma_s is also sigma_t; energy is
   consistent at any step size), the composite dims the surface behind by
   it, and the march stops once T < 0.005. Torch beams (flashlight and
   guard torches) are dimmed by a 4-tap deterministic density integral
   toward the lamp (`towardLightTransmittance` over a light-integral
   density: fog by its mean, puffs without churn, plus the smoke volume) so
   dense smoke between torch and beam visibly darkens the beam. NOT
   attenuated, by design: the baked static volume (it ignores dynamic
   density — dense smoke does not dim the moon's or a practical's
   scatter), and transient flashes (over before the difference reads).
   `settings.volExtinction` toggles the whole thing for A/B.
5. **The density interface.** See the contract below. The medium's box IS
   the smoke grid, driven by uniforms; `mediumDensity` is the single entry
   point every scatterer and absorber goes through.
6. **Guard-vision hook.** A compute pass box-averages the smoke volume 4x
   in x/z (1 m x 0.25 m x 1 m coarse cells) into a buffer read back
   asynchronously every 4th frame (the light probe's map-after-submit,
   drop-if-busy pattern); `sampleSmokeDensityCPU(x, y, z)` trilinears the
   CPU copy, 0 outside the room, exposed as `__sampleSmokeDensity`. Smoke
   only — the ambient fog is texture, not concealment. Verified: a test
   blob of peak density 6 reads 5.06 at its centre through the 1 m coarse
   cells, 1.49 at its edge, 0 outside and above the ceiling.

## The density-interface contract (B2b codes to this)

Also stated at the top of the volumetric section of
`src/shaders/pathtrace.wgsl`, which is the copy of record.

- **The medium's box** is the smoke grid's box: world minimum corner
  `U.smokeOrigin` = (-26, 0, -18), cubic cells of `U.smokeCell` = 0.25 m,
  dims = `textureDimensions(smokeVolume)` = **208 x 13 x 144** (x, y, z) —
  the room's air, x in [-26, 26], y in [0, 3.25], z in [-18, 18]. The
  camera march clips to exactly this box. CPU-side: `MEDIUM_ORIGIN`,
  `SMOKE_CELL`, `SMOKE_DIMS` in `src/engine/renderer.ts` are the single
  source of truth — `MEDIUM_SIZE`, the light-volume cell and the coarse
  readback grid all derive from them, and the uniform block carries origin
  + cell (dims come from the texture). A resize is therefore a
  coordinated compile-time edit of those three constants plus the texture
  reallocation, not a runtime knob; cells stay cubic and the box stays
  dims x cell exactly. **The interface grid is fixed at 208 x 13 x 144:**
  halving it is not available (3.25 / 0.5 is not an integer and the cell
  is one scalar) — a solver that wants a coarser lattice for iteration
  speed simulates on its own grid and resamples into this texture. **The
  box top overshoots the ceiling underside (y 3.2) by 5 cm** because 0.25 m
  cells cannot tile 3.2: grid row 12 (y 3.0-3.25) straddles the ceiling,
  the solver treats y >= 3.2 as its solid top wall, and the camera march
  pays those 5 cm inside the invisible slab (~1.5% of a floor-to-ceiling
  column's optical path — below anything measurable).
- **Density model:** `mediumDensity(p) = densityStatic(p) + smokeDensity(p)`
  (dimensionless); `sigma_t(p) = U.volumetric * mediumDensity(p)` per
  metre, albedo 1. `densityStatic` = fog (mean `U.fogAmount`, noise
  texture) + the puff uniforms — the default source, unchanged, and yours
  to retire once the simulation carries the puffs.
- **The texture:** `renderer.smokeVolume`, `texture_3d`, dimension "3d",
  format `rgba16float`, usage STORAGE_BINDING | TEXTURE_BINDING | COPY_DST
  | COPY_SRC. **R = density; write G/B/A as 0.** (WebGPU storage textures
  in `rgba16float` are write-only from a kernel — no read-modify-write — so
  this is the sim's OUTPUT surface, not its state store; keep your fields
  in your own ping-pong textures and write density here.) Trace pass reads
  it at `@group(1) @binding(14)` as `texture_3d<f32>`, trilinear through the
  shared linear-clamp sampler at `@binding(15)`; sampling convention
  `uvw = (p - U.smokeOrigin) / (dims * U.smokeCell)`, voxel centres at half
  cells, zero outside the box. WebGPU textures start zeroed, so today it
  reads as no smoke everywhere. Write it however you like each frame (a
  storage-3d write in your last kernel, or copyTextureToTexture from your
  ping-pong); nothing else in this track needs to change.
- **The debug filler:** `window.__smokeTest(x, z, radius, density)`
  **overwrites the whole volume** with a single soft spherical blob (R only,
  G/B/A zeroed; density x f^2, f = 1 - d^2/r^2, centred at
  y = clamp(radius, 0.3, 1.6)) built CPU-side and uploaded in full; density
  0 clears; a radius under one cell (0.25 m) writes almost nothing. It is
  this track's standalone test harness and it destroys any simulated field
  — once the solver owns the texture, delete the hook or leave it and never
  call it; nothing else depends on it.
- **The gameplay readback** (`sampleSmokeDensityCPU` / `__sampleSmokeDensity`)
  reads the same texture through the coarse downsample, so it starts
  returning real smoke the moment you write the volume. Track B3
  integrates it along guard line of sight.

## Coordination facts (locks obeyed)

- Uniforms: my block is bytes **880-943 exactly** (smokeOrigin vec3 +
  smokeCell f32 @880, lightVolOrigin vec3 + volRefMode f32 @896,
  lightVolCell vec3 + volExtinction f32 @912, one pad vec4 @928);
  `UNIFORM_SIZE` = 944. Bytes 864-879 are declared as a `_radiosityTrack :
  vec4u` placeholder and never written from renderer.ts — the radiosity
  track's `indirectMode` block lands there; the coordinator swaps the
  placeholder for their field on merge (the byte layout already agrees).
- Trace-pass `@group(1)` bindings: **14** smoke volume + **15** shared
  linear-clamp sampler (my lane), and additionally **10** for the baked
  light volume (a free binding — not in either track's locked pair; noted
  here so nobody else takes it). 16/17 untouched (radiosity's).
- No `gpu.ts` requirement change: the trace pass still binds 4 storage
  textures (the 4th radiance layer rides the existing array texture) and 9
  storage buffers; sampled textures per stage go 4 → 6 (default limit 16),
  samplers 0 → 1 (default 16). New passes: `lightvolume` bake (1 uniform, 1
  storage-3d write, counters buffer + scene group = 7 storage buffers),
  `smokeprobe` downsample (1 storage buffer + scene = 7). SwiftShader init
  verified at every commit.
- New counter slot `lightVolBakeRays` (counters.ts, the authoritative
  list): the whole-volume rebake's shadow rays, zero except on a rebake
  frame. Existing `volumeSteps` / `shadowVolumetric` semantics unchanged.
- Memory: the volume chain adds 9 rgba16f per-pixel targets (accum,
  history x2, moments x2, scratch x2, moment scratch x2) = 72 B/px, ~60 MB
  at 1152x720. The smoke volume is 3.1 MB, the light volume 0.5 MB, the
  coarse readback 97 KB x2.

## Verification

`npm run typecheck` and `npm run build` clean at every commit. Headless
SwiftShader (Chromium, sandbox off for the command sandbox as instructed):
init succeeds at every stage (radiosity bake 10.3 s + light volume bake
5.7 s), no WGSL/validation errors, `ok: true` on every run. All screenshots
below were Read; the shots are reproducible from
`tools/headless/scenarios/vol-shot.js` (edit `WHICH`), the numbers from
`vol-counters.js` and `vol-compare.js`.

### 1. Moon god-rays at the pinned pose (fog on / off)

Pinned bench pose (player at (-2, -11) in the north cubicle farm, camera
settled the way `pinBenchPose` settles it, 448x280 internal), defaults
(volumetric 0.05, fogAmount 0.55, 12 steps).

- **Fog on** (`WHICH = "fogon"`): the north cubicle farm from above; four
  blue moonlight pools on the blue carpet, each with a soft blue halo
  around and above it — the shaft seen almost end-on from the elevated
  camera, so it reads as a glowing skirt spreading from the pool toward the
  window band rather than a slanted beam; the player's flashlight is a warm
  wedge of glow in the air to the right of the character, filled smooth
  rather than showing the carpet through it; the corridor bottom-left
  carries a warm haze from the failing fluorescent; the far cubicles read
  through a faint grey veil. Present, tasteful, not fogged.
- **Fog off** (`fogAmount = 0`, `WHICH = "fogoff"`): same framing; the
  pools are crisp with no halos, the flashlight lights the floor but not
  the air, the corridor haze is gone and the frame is a touch darker and
  sharper everywhere. Pools stay, shafts vanish — the control the brief
  asked for, and the reason `fogAmount` had to become a density.
- Numbers at that pose (mean HDR luminance over the frame, 448x280): beauty
  0.0301 fog on vs 0.0242 fog off; the in-scatter channel alone (debug
  view "volume in-scatter") 0.00835 fog on vs 0.00028 fog off — the
  ambient medium is ~28% of the frame's mean luminance at defaults, and it
  is gone with the fog. Attribution: that in-scatter mean is the whole
  medium — torch beam glow, the failing fluorescent's corridor haze and
  every practical's ambient scatter as well as the moon shafts. The moon's
  share is local: the isolated moon-shaft in-scatter peaks at 0.0232 over
  its pool but is invisible in a whole-frame mean (moon-only vs
  all-statics-off frame means sit within run-to-run noise), so the halos
  are a screenshot claim, not this number's.
- Honesty on "shafts in the air": from the game's 59-degree top-down camera
  a window shaft projects almost onto its own pool, so it presents as the
  pool's halo, not a Hollywood god-ray. A low camera looking north at the
  facade (`vol-shot.js` has the pose in its history; not the game's view)
  shows the pools with soft glow but no dramatic slanted beams at this
  density — that is the medium being tasteful, not broken. `medium
  extinction` on the panel scales it if the demo wants heavier air.

### 2. `__smokeTest` blob: lit under a fluorescent, lit in a moon shaft, dark in a corner

Three shots at 448x280, defaults, one blob each (three separate runs;
the numeric run measures each blob's transmittance floor and in-scatter
peak against the same pose with the volume cleared).

- **Under the warm conference-room fluorescent** (`WHICH = "fluoro"`, blob
  r 1.2 m, peak density 25 at (-21.5, 1.2, 3.2), player at (-19, 4.5)): a
  soft grey-warm cloud sits over the red carpet by the wall left of the
  character, brighter and warmer on top where the tube lights it, greying
  the carpet and softening the chair legs behind it. Numbers: minimum
  camera-ray transmittance in the frame 0.196 (0.852 with no blob), peak
  in-scatter 0.192 (0.112 no blob) — the puff both occludes and glows warm.
- **In a moonlight shaft** (`WHICH = "moon"`, blob r 1.0 m, density 15 at
  (-1.2, 1.0, -16.5), player at (1.5, -13.5)): the pool for the window at
  x = -0.8 sits up-left of the character; the blob rides low in the
  descending shaft (the shaft at that z is only ~0.8 m off the floor), so it
  reads as the pool swelling into a soft blue-white glow rising out of the
  floor toward the window band — lit smoke, coloured by the moon. Numbers:
  T floor 0.588 (0.851 no blob), in-scatter peak 0.0868 (0.0232 no blob) —
  3.7x the shaft's own peak.
- **Dark air with a lit pool behind it** (`WHICH = "corner"`, blob r 1.0
  at (-1.1, 1.0, -14.5), same framing). The blob sits just south of the
  shaft in unlit air (the shaft is below y 0.33 at that z, so the blob's
  bottom third is inside it) and the top-down camera's rays through it
  land on the moonlit pool. At the original density 30 the review measured
  the darkening at the perceptual floor (37 px above 10/255 against a
  same-pose control, ~run-to-run noise), so the scenario now ships at
  **density 60**: against its no-blob control the pool carries a soft dark
  grey bite diagonally through its middle where the control pool is a
  clean blue ellipse — 2,823 px darkened by more than 20/765 (summed RGB),
  peak −46, mean −18 over the bite, versus 47 brightened px (the blob's
  in-shaft third glowing) — an occluding puff, not a glowing one. Numbers
  at density 30 for reference: T floor 0.439, in-scatter peak 0.0305
  against the 0.0232 no-blob baseline. Honesty: from the 59-degree camera
  the effect is a soft bite, not a crisp black cloud, because the pool is
  ~2 m across and the blob's rays land on a 1 m band of it; a low camera
  looking north through the blob at the pool is where a silhouette would
  show, and that is not the game's view.

### 3. Flashlight through the test blob — extinction working

Pinned pose, blob (r 1.3, density 30) placed 4 m down the actual beam
(read from `flashlightOrigin()/flashlightDir()` after the aim rig settles;
in headless the aim points west-southwest), 448x280.

- **Extinction on** (`WHICH = "exton"`): the beam wedge is bright between
  the character and the puff, the puff shows a lit rim toward the torch and
  a darker body, and the wedge past the puff toward the far cubicles is
  visibly dimmer than the near half.
- **Extinction off** (`WHICH = "extoff"`): the puff is a uniform bright
  wash (no self-shadowing, no camera-ray attenuation) and the beam past it
  is as bright as before it — it sails through.
- Numbers (debug view "volume in-scatter", 5x5-pixel patch of the beam glow
  projected 2.0 m before and 6.5 m past the blob along the beam, 384x240):
  no blob 0.0278 / 0.0141; blob with extinction 0.0392 / **0.0112**; blob
  without extinction 0.0563 / **0.0156**. Past the blob the beam glow drops
  28% with extinction and rises 10% without it (the blob's own scatter);
  before the blob it rises in both (the puff's glow bleeds into the
  patch), less with extinction because the puff also shades itself. The
  pixel-level 28% understates the beam's own dimming (~8x from the optical
  depth): each pixel integrates a 12-step column through the whole slab, so
  ambient fog + baked static scatter along the ray dilute the beam term.
  First cut used 3 jittered taps toward the light and measured only 24%
  because a noisy optical depth inside exp() biases the accumulated
  transmittance upward (Jensen); the shipped version uses 4 deterministic
  midpoint taps over a cheaper light-integral density (fog mean, puffs
  without churn, smoke volume).

### 4. Work counters vs the base commit, and the reference comparison

**Counters** (`vol-counters.js`, `__bench(3, true)` at
`__benchResolution(320, 200)`, block frame = the run's last measured
frame; base = trunk `be298ab` at the same pose/res).

| config | volumeSteps/px | shadowVolumetric/px | bvhNodeVisits/px | slabTests/px | obbTests/px |
|---|---|---|---|---|---|
| base (trunk) | 12.00 | 0 | 31.02 | 61.63 | 4.20 |
| defaults (bake + flashmap + extinction) | 10.40 | 0 | 31.03 | 61.68 | 4.34 |
| flashVisVolumetric off | 10.40 | 6.32 | 114.79 | 240.38 | 88.43 |
| extinction off | 10.40 | 0 | 31.03 | 61.62 | 4.16 |
| reference mode (MC static + real torch rays) | 10.42 | 47.93 | 917.05 | 1816.41 | 119.14 |

Reconciliation: 10.40 steps/px = 12 x 0.867, and 0.866 is the primary-hit
fraction — pixels whose ray never enters the room's box (outside the
facade at the top of this wide view) pay no steps; every ray that enters
pays all 12, but inside the slab instead of 80% above the ceiling. Zero
shadow rays in the default march is unchanged from base (the depth maps
carry the torch visibility); the static lights cost texture samples, not
rays — traversal counters are flat against base to the second decimal (the
+0.14 obbTests/px is the frozen dynamic-geometry phase differing between
page loads, present in the base's own reruns). Reference mode's 47.93 =
4 static-light shadow rays x 12 steps, plus the trace's own, which is why
a reference frame costs ~2.4x a normal one on SwiftShader. **The rebake
spike:** one frame after `setStaticLightIntensity` (an OCP darkening)
carried `lightVolBakeRays` = 10,063,872 (59,904 voxels x ~28 lit lights x
6 rays) and took 18.5 s wall on SwiftShader vs ~1 s for a normal 160x100
frame — a whole-volume dispatch at OCP event rate, invisible in the
counters otherwise. Its cost on silicon is a Mac question below.

**`__compareToReference`** (`vol-compare.js`, `__benchResolution(320, 200)`,
140 reference / 40 test frames, bounces 1, the freeze protocol Track A
documented). Reference mode was made honest for the new channel rather
than gated out: it renders the same volumetric *model* by Monte Carlo — per
march step, lights[0] (the moon, source of the pools) always plus 3
uniformly subsampled practicals re-weighted (S-1)/3, jittered over each
emitter with real static-scene shadow rays (`occludedStatic`, the bake's
visibility — dynamic geometry shadows neither estimator's static
in-scatter), isotropic phase; torches by real rays (`flashVisVolumetric`
was already forced off in reference); transients are excluded from the
volume channel in reference exactly as they are from its surface signal.
The volume channel accumulates through the reference reproject slot
(unbounded 1/n average, no clamps) and the reference composite reads its
accumulator. So the RMSE below measures what the shipped estimator
approximates: the light volume's 0.5 m voxels + 6-ray visibility, the
depth-map beam visibility, and the volume denoiser's short clamped
history.

Whole-image compare (140 ref / 40 test, not truncated, refMean 0.0302):

| config | relRmse | relBias | maxRelErr |
|---|---|---|---|
| defaults | 1.1285 | -0.1603 | 106.8 |
| extinctionOff | 1.0682 | -0.0825 | 109.6 |
| noVolume (control) | 1.1146 | -0.2337 | 109.4 |

Reading it: Track A measured trunk's own baseline at this protocol at
relRmse 1.19-1.27 / relBias -0.157 to -0.189 (a 1-spp 320x200/40-frame
test is dominated by surface transport noise, and the pipeline carries a
standing negative bias). `defaults` at 1.13 / -0.160 sits inside that band
— the new channel adds no bias measurable above the pipeline's own. The
volumetric signal is in the deltas against the shared reference: removing
the medium (`noVolume`) drops the mean by 7.4 points of relBias (the medium
is ~7% of this frame's energy and the shipped estimator recovers it to
within the baseline), and removing extinction lifts it by 7.8 points (the
absorption's magnitude, correctly directional). Noise floor caveat from
Track A's protocol: same-build reruns spread ±6% relRmse / ±0.05 relBias
(bimodal on the frozen guard phase), so relRmse orderings here are not
significant; the relBias deltas (0.074, 0.078) are above the floor.

**Volume-channel-only compare** (`vol-compare.js` with `MODE = "inscatter"`:
`refOverrides = { debugView: 8 }` and the test configs also at debug view 8,
so both sides output only the in-scattered radiance: surface noise drops out
and the error is the in-scatter estimator's own — bake + flashmap + 12-frame
clamped history vs Monte Carlo statics + real torch rays, 1/n-accumulated),
same protocol, refMean 0.0041, at the post-review HEAD (reference statics
shadowed by the static scene only, as the bake is):

| config | relRmse | relBias | maxRelErr |
|---|---|---|---|
| inscatter (defaults) | 0.4782 | **+0.0206** | 0.94 |
| inscatter, extinction off vs the with-extinction reference | 0.4571 | +0.1094 | 0.95 |

(The pre-review run, with dynamic geometry also shadowing the reference's
statics, read +0.0222 / +0.1105 — the character's shadow in the medium was
inside the noise, as expected.)

The shipped in-scatter estimator lands **~2% high** on mean energy against
the transport it approximates — the bake's 0.5 m voxels + 6-ray visibility,
the depth-map beam and the volume denoiser together, well inside anything
visible. relRmse 0.48 is 40 filtered test frames against a 140-frame
reference of a jittered march (the volume signal is small and smooth, so
relative RMSE reads large where absolute error is 0.0020); the second row is
not a config but a scale bar: dropping the camera-ray attenuation of the
in-scatter alone moves the mean by +8.9 points, i.e. the +2.1% is a quarter
of the extinction effect it correctly models.

## Not verified / below 100%, honestly

- **Milliseconds**: nothing here is a performance claim; SwiftShader
  timings mean nothing. Cost is expressed in counters (flat traversal, no
  march shadow rays at defaults, 0.87x steps) and the Mac list below owns
  the ms — including the rebake hitch, whose 18.5 s SwiftShader wall is a
  smoke signal only.
- **The compare configs ran once each.** Track A measured a ±6% relRmse /
  ±0.05 relBias same-build spread at this protocol; I did not spend the
  ~25 minutes per repeat. The claims above lean on relBias deltas (0.074,
  0.078, 0.088) that clear that floor and on the isolated +0.022, which is
  under it — read that one as "no bias detectable at this protocol", not
  as a measured 2.2%.
- **A live muzzle flash through the volume channel was not fired
  headlessly** (see Findings). The channel's short clamped history is the
  design answer to the hang the old comment warned about; unverified in
  motion.
- **God-ray drama**: the shafts read as soft skirts on the pools from the
  game camera, not as slanted beams — a property of the top-down view and
  the tasteful default density, verified by screenshot, judged by me. The
  knob (medium extinction) is on the panel.
- **B2b/B3 integration** is by contract, not by test: no simulation writes
  the smoke volume yet and nothing consumes `sampleSmokeDensityCPU`. The
  channel was exercised end to end with the CPU test blob (density,
  extinction, readback numbers above).
- **Real-GPU limits**: verified against SwiftShader's default-limit device
  (the strict one for storage textures/buffers), which is the campaign's
  init gate; not run on the M1 Max.

## Mac bench script (Sujay, M1 Max, Chrome)

Milliseconds are the unknown; SwiftShader cannot say. All at the standard
1152x720 via plain `__bench(60)` (do NOT set `__benchResolution` — a
non-standard result blob is marked and not comparable). Run each from a
fresh reload so history and reservoirs start equal.

```js
// 1. Cost of the whole change at defaults (compare wallMsPerFrame / gpu.pathtrace against trunk claude/campaign at the same commit protocol)
await __bench(60)
// 2. The medium off entirely — the ceiling of what the volumetric term costs
__settings.volumetric = 0; await __bench(60)
// 3. Extinction only (camera-ray T + 4-tap light integrals) — its marginal cost
__settings.volumetric = 0.05; __settings.volExtinction = false; await __bench(60)
__settings.volExtinction = true
// 4. Fog off (density evaluation drops to puffs + one texture sample per step)
__settings.fogAmount = 0; await __bench(60); __settings.fogAmount = 0.55
// 5. Step count sensitivity (the march is the term that scales)
__settings.volumetricSteps = 6;  await __bench(60)
__settings.volumetricSteps = 24; await __bench(60)
__settings.volumetricSteps = 12
// 6. The rebake hitch: darken a practical the way the OCP does, then bench a
//    handful of frames immediately (the first frame carries the whole-volume
//    dispatch). p99/max frame time in that window is the OCP's hitch.
__renderer.setStaticLightIntensity(6, 0); await __bench(10)
__renderer.setStaticLightIntensity(6, 4.95)
// 7. Counters-off vs pre-A build (Track A's open watch item) is unaffected by
//    this track but still owed.
```

Also worth a look on the Mac: the smoke-density readback pass runs every
4th frame (a 6.5k-thread box average + 97 KB copy) — it should be lost in
the noise; `gpu.smokeprobe` is not in the profiler (it has no timestamp
pass, deliberately: it is below the profiler's resolution), so judge it by
`wallMsPerFrame` with `SMOKE_READ_EVERY` at 4 vs a build with it at 10^6.

## Findings

- **Surface direct lighting does not see the medium.** Shadow rays from
  surfaces to lights never sample density, so the floor spot beyond a smoke
  cloud is as bright as without the cloud; only the beam's *air glow* and
  what the camera sees *through* the cloud are attenuated. Physically the
  floor should dim too. Doing it right means density taps on every direct
  shadow ray (the frame's dominant ray class) — a real cost, out of this
  track's lane, and the estimator-shape decision belongs with the direct
  path. Noted, not fixed; from the top-down camera it reads as "the smoke
  glows and hides the floor under it", which is most of the effect anyway.
- **Volume in-scatter is reprojected as if painted on the surface behind
  it** (the pre-existing model, kept). With a 12-frame clamped history the
  parallax error is invisible from the game camera and a swept beam
  re-converges in a handful of frames; a fast lateral camera move over a
  bright shaft would smear it briefly. Sky pixels (through windows) have no
  world position, so their volume channel is a raw single sample —
  neither reprojected nor a-trous-filtered (both key on world position);
  rare from the top-down camera, but a low camera looking out a window
  would flicker there.
- **The static light volume bakes with 6 visibility rays per light per
  voxel and 0.5 m cells:** pool edges the moon throws are stepped by
  ~0.25 m under trilinear filtering (soft enough at the game camera; a low
  camera close to a pool would show the voxel grid). Voxels inside walls
  bake dark and trilinear bleeds that darkness ~0.25 m into adjacent air —
  a faint dark seam along walls in dense fog, not visible at the default
  density. Both are the resolution/ray-count trade; the bake is a one-shot
  10M-ray dispatch, so both knobs are cheap to raise.
- **Static-volume scatter ignores dynamic density** (deliverable 4's stated
  gap): dense smoke dims a torch beam passing through it but not the
  moonlight or fluorescent scatter around it. Magnitude, measured in
  review: a dense blob (r 1.5, density 200) in the moon shaft peaks at
  in-scatter 0.139 against the moonlit-carpet pool floor's 0.028 — the
  saturating albedo-1 scatter of an unshadowed E/(4 pi), so an opaque puff
  in a shaft glows through its whole surface as if unshadowed. The MC
  reference has the same property by construction, so the compare does
  not measure it; the fix is a density integral toward each static light
  (or a shadowed second volume), not free.
- **Transient (muzzle-flash) in-scatter now rides the accumulated volume
  channel** instead of the never-accumulated transient signal — the brief's
  routing. The channel's neighbourhood-clamped 12-frame history is what
  makes that safe (a flash's glow appears and disappears with its
  neighbourhood, not through a 48-frame fade), but it was not exercised
  under a live flash headlessly: no verification bundle fires the pistol.
  If a flash's air-glow visibly hangs, the fix is one line (route
  `l.color * ...` for `li >= transientStart` back into the transient
  signal), not a redesign.
- **`__smokeTest` uploads the full 3.1 MB volume per call** — a debug hook,
  fine at debug rates, not something to call per frame; B2b's sim writes
  the texture on the GPU and never touches this path.
- **The atrous/reproject alpha semantics changed:** alpha is now a fourth
  signal channel unless `validityInAlpha` is set (indirect chain only). The
  direct chain's alpha is dead (written 0); nothing else read it. The
  indirect chain's validity path is untouched.

## Watch when merging

- **Uniform block:** `_radiosityTrack : vec4u` at bytes 864-879 in
  `common.wgsl` is a placeholder for the radiosity track's field — replace
  it with theirs on merge (16 bytes, same offset); `UNIFORM_SIZE` here is
  944, take the max of the two. `writeUniforms` never touches f32 indices
  216-219 (their bytes).
- **Bindings:** the trace pass gains 10 (light volume), 14 (smoke volume),
  15 (sampler). If the radiosity track also adds a sampler, share this one
  (15) rather than adding a second — the layout entry is a plain filtering
  sampler.
- **REPROJECT_SLOTS 4 → 6 and the atrous slot count:** the reproject and
  atrous parameter buffers grew; a merge that reintroduces the old sizes
  fails validation at the first `setBindGroup` with the volume offsets.
- **`RAW_ILLUM_LAYERS` 3 → 4:** anything else that creates the illum array
  or its views must agree.
- **Composite params shrank** (flashColor/volStrength gone, struct is
  {debugView, transientOn, pad, pad}); a merge that keeps the old scratch
  writes at f32 index 6 would silently zero `transientOn`.
- **DEBUG_VIEWS gained two entries** (8 volume in-scatter, 9 volume
  transmittance); `G` cycles through them.
- **B2b's contract deltas go in ONE place** (their brief says so): the header
  comment in pathtrace.wgsl and this section move together.

## Review resolution

Three adversarial reviews (volumetric-physics, plumbing/denoise, and the
B2b-contract lens) examined HEAD `b99fbbd`. Every finding was re-verified
by reading the code (and, where it was cheap, by measurement) before acting
— trusting neither the reviewer nor my own report. Ordered by reviewer.

| # | Finding | Verdict | Action |
|---|---|---|---|
| 1.1 / 3.3 | Medium box top y 3.25 vs ceiling underside 3.2: top voxel row straddles the ceiling; camera rays march ~5 cm of medium inside the invisible slab; B2b would have to guess whether row 12 is fluid or solid. | **Real** (contract gap; effect at defaults ~1.5% of a column's optical path). 0.25 m cells cannot tile 3.2, so the overshoot is by design and stays. | Stated in the contract of record (pathtrace.wgsl header), the report's contract section and the `renderer.ts` constants comment: the box top overshoots by 5 cm, row 12 straddles the ceiling, the solver treats y >= 3.2 as its solid top wall, the march pays those 5 cm. No behaviour change. |
| 1.2 | Static-light in-scatter has no self-shadowing by dynamic density; a dense blob in the moon shaft peaks at ~5x the pool floor (0.139 vs 0.028), i.e. an opaque puff glows through its whole surface as if unshadowed. | **Real, already documented** (deliverable 4's stated gap and a Findings bullet); the reviewer's measurement quantifies it and confirms it is the model, not a units bug. Not fixable inside this track's cost lane. | Reviewer's numbers folded into the Findings bullet with the honest cost of the fix (a density integral toward each static light). No code change. |
| 1.3 | `sampleSmokeDensityCPU` returns 0 in the outer half coarse cell (0.5 m band) at each room edge. | **False positive.** The bounds test (`gx > nx - 0.5`, x/z divided by the *coarse* 1 m cell) rejects exactly at the box wall (x = ±26), and the interpolation edge-clamps inside it. Verified by measurement: blob banked against the east wall reads 8.23 at x = 25.9 **and** at 25.99 (clamped edge cell), 7.55 at its centre 25.4, 0 only at 26.1 (outside the box); same on z. | None to the code; recorded here so B3 does not inherit the doubt. |
| 1.4 | Sky/window pixels: no world position, so the volume channel is a raw unaccumulated, unfiltered single sample there. | **Real, already documented** (Findings); rare from the game camera. | Findings bullet sharpened (neither reprojected nor a-trous-filtered; a low camera looking out a window would flicker there). No code change: giving sky pixels a synthetic position on the far march boundary is a reproject/atrous geometry change out of this track's lane. |
| 2.1 | Coarse smoke readback: a rejected `mapAsync` never clears `smokeCoarseBusy`, freezing `sampleSmokeDensityCPU` at its last snapshot forever with no signal. | **Real.** (Nuance on the claimed sibling pattern: `counters.ts` clears its flag on rejection; the probe readback deliberately does not, to stop retrying a dead device. For gameplay data a stale frozen field is the worse failure — B3 would see phantom smoke — and the retry is one map per SMOKE_READ_EVERY frames, so a dead device costs nothing.) | Rejection handler now releases the slot (`renderer.ts`, `smokeCoarseStaging.mapAsync` rejection) with the why-comment. |
| 2.2 | Reference `staticScatterSample` shadows the medium with `occluded()` (dynamic geometry, incl. the character) while the bake uses `occludedStatic()`; the compare measures a by-construction mismatch, not estimator error. | **Real** (small: the character-in-medium slice was inside the noise). | Reference statics now use `occludedStatic` — the same integrand the bake discretises (`pathtrace.wgsl`, `staticScatterSample` + its doc comment). Compare re-run at HEAD: in-scatter relBias +0.0206 (was +0.0222), whole-image table below; reference-mode `obbTests`/px dropped 119.1 → 116.7 (the dynamic-OBB shadow tests no longer traced), confirming the change bites. |
| 2.3 | Reference-mode volume accumulator ingests transients (they march unconditionally; only the surface transient signal is zeroed in reference), so a flash fired during accumulation would average in permanently. | **Real, theoretical** (no compare scenario fires the pistol). | The transient loop in `volumetricBeams` is skipped when `U.volRefMode` is set — reference now excludes transients from the volume channel exactly as its composite excludes them from the surface (`pathtrace.wgsl`). |
| 2.4 | The volume-channel-only compare (`refOverrides {debugView: 8}`) had no committed scenario. | **Real** (reproducibility). | `vol-compare.js` gained a `MODE` switch (`"whole"` / `"inscatter"`) carrying both configs; the report's in-scatter table now points at it and carries the numbers re-run through it. |
| 2.5 | The moon "god-ray" claim is weaker than the screenshot prose: moon-only vs all-statics-off in-scatter frame means are within run-to-run noise; the fog on/off in-scatter delta is beams + practical haze, not moon shafts. | **Real as a report-attribution wart, false as read against the prose.** The screenshot prose ("four pools each with a soft blue halo") is what the fog-on shot shows and reviewer 1 read the same thing; the report never attributed the 0.00835 in-scatter mean to the moon, but it did not say what it *was* attributable to, which invites the misread. The halo is local (isolated shaft peak 0.0232), invisible in a whole-frame mean. | The numbers bullet now states the attribution explicitly (whole medium: beams + fluorescent haze + all practicals + shafts; the moon's share is local and a screenshot claim, not the mean's). No code change. |
| 3.1 (MED) | "A resize is a texture allocation, not a uniform change" is false: CPU constants (`SMOKE_DIMS`, `SMOKE_CELL`, coarse dims, the smokeprobe params, `MEDIUM_SIZE`) are hardcoded and must satisfy dims x cell = box exactly — an unstated constraint; and B2b's brief-mandated half-res debug grid is unsatisfiable (3.25 / 0.5 non-integer, cell is one scalar). | **Real.** The claim overstated it; the coupling was implicit. | (a) Single source of truth: `MEDIUM_SIZE` is now *derived* from `SMOKE_DIMS x SMOKE_CELL` in `renderer.ts` (light-volume cell, coarse dims and probe params already derived), so a resize is one constant edit + reallocation and cannot desync the box. (b) Contract (WGSL + report) corrected: resize = coordinated compile-time constant edit, cells cubic, box = dims x cell exactly, the **interface** grid fixed at 208x13x144; a coarser sim runs on its own lattice and resamples into the interface texture — the sanctioned answer to B2b's half-res knob. Halving the interface itself needs a coordinator ruling (it changes B2a constants and the box height); flagged for B2b, not silently promised. |
| 3.2 | `__smokeTest` overwrites the WHOLE volume (single blob, R only, G/B/A zeroed) via a 3.1 MB CPU upload, so any debug call destroys a simulated field the contract invites B2b to keep in the same texture; the contract called it harmless without stating the clobber; a sub-cell radius writes ~nothing. | **Real** (contract omission; behaviour is fine for a debug hook). | Contract (WGSL header + report) now states the whole-volume overwrite, the R-only/GBA-zero semantics, the sub-cell-radius caveat, and tells B2b to delete or never call it once the solver owns the texture. No behaviour change (a debug filler that composites multiple blobs is not a deliverable). |
| 3.3 | Same as 1.1 (box top vs ceiling), from the B2b side. | **Real** — resolved with 1.1. | See 1.1. |
| 3.4 | "G/B/A are yours (temperature, velocity divergence…)" advertises unusable capacity: `rgba16float` storage textures are write-only from a kernel (no read-modify-write), so co-located channels cannot be sim state; and `__smokeTest` zeroes them. | **Real** (decorative offer, not a break). | Contract corrected: R = density, write G/B/A = 0; the texture is the sim's *output* surface, its state lives in its own ping-pong textures. |
| 3.5 | The "dark occluding puff" corner shot is at the perceptual floor at density 30 (37 px above 10/255 vs a same-pose control, ≈ run-to-run noise; best blurred bite −5.1/255); "reads as a dark occluding puff" unproven by image. | **Real** (verification claim, not contract). Re-measured myself at density 30 and confirmed the reviewer's numbers. | The corner scenario now ships at **density 60** (`vol-shot.js`); against a fresh same-pose no-blob control the pool shows a soft dark grey bite through its middle (2,823 px darkened > 20/765 summed RGB, peak −46, versus 47 brightened px where the blob's bottom third sits in the shaft) — visible, and described honestly as a bite rather than a black cloud, with the reason (59-degree camera, blob rays land on a ~1 m band of a 2 m pool). Shots Read: `rev_corner60.png` vs `rev_cornerctl.png`. |

Also considered and left as-is: the volume reproject slot's `alphaFloor` 0.08 < 1/12 never binds against a 12-frame history (cosmetic; the max-history term is the operative clamp).

**Fresh verification at the post-review HEAD** (all `ok: true`, SwiftShader,
init through both bakes every run):

- `npm run typecheck`, `npm run build` clean.
- Counters (`vol-counters.js`, `__bench(3, true)` at 320x200), per pixel:
  defaults volumeSteps 10.41 / shadowVolumetric 0 / bvhNodeVisits 31.05 /
  slabTests 61.68 / obbTests 4.21; flashmap off 10.40 / 6.31 / 114.75 /
  240.23 / 88.13; extinction off 10.40 / 0 / 31.02 / 61.61 / 4.14;
  reference mode 10.42 / 47.94 / 917.02 / 1756.14 / 116.69; rebake frame
  `lightVolBakeRays` 10,063,872 (7.9 s wall at 160x100 on SwiftShader). The
  table above is unchanged to the second decimal except reference-mode
  `obbTests` (119.14 → 116.69) and `slabTests` (1816 → 1756): the reference's
  static-scatter shadow rays no longer walk dynamic geometry (finding 2.2).
- `__compareToReference`, whole image at HEAD, same protocol (320x200, 140
  ref / 40 test, one run each, refMean 0.0301): defaults relRmse 1.1307 /
  relBias −0.1580 / maxRelErr 107.5; extinctionOff 1.0709 / −0.0801 /
  110.3; noVolume 1.1154 / −0.2308 / 110.1. Against the pre-review run the
  numbers moved by less than the ±0.05 same-build noise (defaults were
  1.1285 / −0.1603): the deltas that carry the argument are intact —
  removing extinction lifts relBias +7.8 points, removing the medium drops
  it −7.3 — so the conclusion stands: no bias detectable above the
  pipeline's own at this protocol. In-scatter-only table above: +0.0206 /
  +0.1094.
- Screenshots Read at HEAD (448x280, `vol-shot.js`): `fogon` — the north
  cubicle farm from above, blue moon pools on the carpet each with a soft
  blue halo, the flashlight a warm-white wedge in the air spreading from
  the character, warm haze from the failing fluorescent bottom-left, a
  faint grey veil over the far cubicles; `fogoff` — same framing, pools
  crisp with no halos, the flashlight lights the floor but not the air,
  corridor haze gone, frame a touch darker; `fluoro` — a grey-warm cloud
  over the red carpet left of the character in the conference room,
  brighter and warmer on top where the tube lights it; `moon` — the pool
  up-left of the character swells into a soft blue-white glow on its
  window side (the blob lit inside the shaft); `corner` at density 60 — a
  soft dark grey bite diagonally through the same pool, absent in the
  same-pose control shot (both Read); `exton` — the beam wedge carries a
  bright puff with a lit near rim and darker body, and the wedge past it
  toward the far cubicles is dimmer than the near half; `extoff` — the
  puff is a uniform brighter wash and the wedge past it is as bright as
  before it. Guard-vision readback re-checked headlessly: a centre blob
  (r 1, peak 6) reads 1.495 through the 1 m coarse cells within two render
  calls of the write, an east-wall blob reads to the wall (8.23 at
  x 25.99) and 0 past it — the false-positive check in the table.
