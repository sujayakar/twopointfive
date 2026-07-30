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
  camera march clips to exactly this box. CPU-side constants:
  `MEDIUM_ORIGIN`, `MEDIUM_SIZE`, `SMOKE_CELL`, `SMOKE_DIMS` exported from
  `src/engine/renderer.ts`; the uniform block carries origin + cell (dims
  come from the texture, so a resize is a texture allocation, not a
  uniform change).
- **Density model:** `mediumDensity(p) = densityStatic(p) + smokeDensity(p)`
  (dimensionless); `sigma_t(p) = U.volumetric * mediumDensity(p)` per
  metre, albedo 1. `densityStatic` = fog (mean `U.fogAmount`, noise
  texture) + the puff uniforms — the default source, unchanged, and yours
  to retire once the simulation carries the puffs.
- **The texture:** `renderer.smokeVolume`, `texture_3d`, dimension "3d",
  format `rgba16float`, usage STORAGE_BINDING | TEXTURE_BINDING | COPY_DST
  | COPY_SRC. **R = density**; G/B/A are yours (temperature, velocity
  divergence, whatever the sim wants to co-locate). Trace pass reads it at
  `@group(1) @binding(14)` as `texture_3d<f32>`, trilinear through the
  shared linear-clamp sampler at `@binding(15)`; sampling convention
  `uvw = (p - U.smokeOrigin) / (dims * U.smokeCell)`, voxel centres at half
  cells, zero outside the box. WebGPU textures start zeroed, so today it
  reads as no smoke everywhere. Write it however you like each frame (a
  storage-3d write in your last kernel, or copyTextureToTexture from your
  ping-pong); nothing else in this track needs to change.
- **The debug filler:** `window.__smokeTest(x, z, radius, density)` writes a
  single soft spherical blob (density x f^2, f = 1 - d^2/r^2, centred at
  y = clamp(radius, 0.3, 1.6)) into the CPU-side copy and uploads the whole
  volume; density 0 clears. Your sim replaces it; keep the hook as a way to
  poke the channel without the solver, or delete it — nothing depends on
  it.
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
  is gone with the fog.
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
- **Dark air with a lit pool behind it** (`WHICH = "corner"`, blob r 1.0,
  density 30 at (-1.1, 1.0, -14.5), same framing): the blob is 3 m south of
  the shaft in unlit air, and the camera ray through it lands on the
  moonlit pool: the pool's near portion is visibly darkened compared with
  the moon shot — an occluding puff, not a glowing one. Numbers: T floor
  0.439, in-scatter peak 0.0305 (barely above the 0.0232 baseline: it is
  not lit). Honesty: at 448x280 from the top-down camera the darkening is
  a soft bite out of the pool rather than a crisp black cloud; the
  transmittance number is the unambiguous evidence, and denser blobs (the
  hook takes any density) go blacker.

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
emitter with real shadow rays, isotropic phase; torches by real rays
(`flashVisVolumetric` was already forced off in reference); the volume
channel accumulates through the reference reproject slot (unbounded 1/n
average, no clamps) and the reference composite reads its accumulator. So
the RMSE below measures what the shipped estimator approximates: the light
volume's 0.5 m voxels + 6-ray visibility, the depth-map beam visibility,
and the volume denoiser's short clamped history.

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

**Volume-channel-only compare** (`refOverrides = { debugView: 8 }` and the
test configs also at debug view 8, so both sides output only the in-scattered
radiance: surface noise drops out and the error is the in-scatter
estimator's own — bake + flashmap + 12-frame clamped history vs Monte Carlo
statics + real torch rays, 1/n-accumulated), same protocol, refMean
0.00406:

| config | relRmse | relBias | maxRelErr |
|---|---|---|---|
| inscatter (defaults) | 0.4723 | **+0.0222** | 0.94 |
| inscatter, extinction off vs the with-extinction reference | 0.4526 | +0.1105 | 0.96 |

The shipped in-scatter estimator lands **2.2% high** on mean energy against
the transport it approximates — the bake's 0.5 m voxels + 6-ray visibility,
the depth-map beam and the volume denoiser together, well inside anything
visible. relRmse 0.47 is 40 filtered test frames against a 140-frame
reference of a jittered march (the volume signal is small and smooth, so
relative RMSE reads large where absolute error is 0.0019); the second row is
not a config but a scale bar: dropping the camera-ray attenuation of the
in-scatter alone moves the mean by +8.8 points, i.e. the +2.2% is a quarter
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
  world position, so their volume channel is unaccumulated raw march noise
  — rare from a top-down camera.
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
  moonlight or fluorescent scatter around it. The MC reference has the
  same property by construction, so the compare does not measure it.
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
