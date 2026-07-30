# twopointfive

A 2.5D top-down stealth tech demo with a real-time path tracer, written in raw
WebGPU + WGSL. Every light in the scene is traced: shadows, penumbrae, bounce
light and the volumetric beam all fall out of the same path tracer rather than
being faked.

```
npm install
npm run dev      # http://127.0.0.1:5173
```

Requires Chrome/Edge 113+ (recommended), Firefox 145+ on Apple Silicon, or
Safari 26+ on macOS Tahoe. See "Browser notes" below — Chrome is materially
better for this workload.

## Controls

| | |
|---|---|
| `WASD` | move (camera-relative) |
| `Shift` / `Alt` | sprint / walk |
| `C` | crouch |
| mouse | aim — the character turns to face the cursor, beam follows |
| `F` | toggle flashlight |
| `N` | night vision (white phosphor) |
| `` ` `` | tweak panel — 26 live sliders |
| `G` | cycle debug views: albedo, normal, variance/history, raw 1-spp |
| `1`–`4` | quality presets |

## What it actually does

**No rasterization anywhere.** Primary visibility, direct lighting, indirect
bounces and volumetrics are all traced in one compute pass.

**Geometry is oriented boxes, not triangles.** The reference art (cubicle
partitions, desks, crates, carpet slabs) is genuinely box-shaped, and a ray-OBB
slab test is several times cheaper than a triangle intersection while being
exact — no tessellation, no normal interpolation, no cracks. A binned-SAH BVH is
built on the CPU and flattened for stackless GPU traversal. Animated geometry
(the player) lives in a small separate list tested linearly, so nothing has to
be rebuilt per frame.

**Lighting.** Next-event estimation through one **ReSTIR DI** reservoir over
every steady light at once — the flashlight (a spot with a finite lens radius,
so the penumbra widens with distance — this is most of what sells the shadows as
real), the moon, and ~30 local practicals (fluorescents, exit signs, monitor
glow, emergency units): 8 shadow-ray-free RIS candidates per pixel, merged with
last frame's reservoirs at 6 spatial taps, and **one** shadow ray spent on the
survivor (counter-verified: 0.99 direct shadow rays per pixel at bounce 0).
Transient lights (muzzle flashes) are sampled by plain NEE on their own
un-accumulated signal.

**Denoising.** SVGF: temporal reprojection with depth/normal validation and
neighbourhood colour clamping, then four à-trous wavelet iterations with
edge-stopping on normal, depth and temporally-estimated luminance variance.
Radiance is albedo-demodulated before filtering so the blur cannot smear surface
colour across material boundaries.

**Post.** Progressive bloom (Karis-weighted 13-tap downsample, tent upsample),
AgX tonemapping, vignette, film grain, subtle chromatic aberration.

## The character

Driven by the **Quaternius Universal Animation Library** (CC0): a 65-joint
UE-style rig with 86 clips. `tools/extract-rig.mjs` reduces the two 15 MB GLBs to
a **171 KB** runtime asset by dropping the 40 finger joints, keeping 19
stealth-relevant clips, resampling onto fixed 30 fps frames (so runtime sampling
is an array index, not a per-channel keyframe search) and quantising rotations to
int16. It also *verifies* that only the root joint translates rather than
assuming it, which is what makes folding the other 22 into the rest pose safe.

Limbs are boxes spanning **joint pairs**, so each one takes its length and
orientation straight from the animation. The world is boxes and looks it;
dropping a smooth 13.7k-triangle mannequin into it would read as foreign and
would mean adding a triangle path to a tracer that is fast precisely because it
only does oriented boxes. The motion is the authored motion, the shading proxy
is blocky.

Locomotion picks between idle / walk / jog / sprint / crouch with cross-fades,
and the playback rate scales with actual ground speed so the feet do not slide.
An upper-body bone mask layers a weapon-ready pose over the legs while moving,
so the arms stay forward instead of swinging away from where you are aiming. The
torch takes its *position* from the animated hand but its *direction* from the
aim — a beam that bobbed with the walk cycle would be unusable.

**Directional movement without directional clips.** The library has no strafe,
backward or turn animations, so those are synthesised:

- The feet aim along the direction of travel, *clamped to within the twist limit
  of where you are looking*. Without that clamp a 90-degree strafe demands a
  twist the spine cannot deliver and the torso visibly detaches from the cursor.
- Past ~100 degrees between aim and travel the character **backpedals** instead
  of turning its back on the cursor: the feet hold facing the aim and the walk
  cycle runs in reverse. Negative playback works because clip time wraps, and it
  retraces the forward cycle exactly.
- Standing still, the feet stay planted while the torso winds up, then step round
  once the twist gets uncomfortable — the turn-in-place. Crouched uses a tighter
  limit and a faster foot turn, since you cannot wind up as far in a crouch.

All of the thresholds are live on the tweak panel under *movement / turning*;
`movementTuning` in `game/player.ts` holds the defaults.

## Measured performance

Apple M1 Max (32-core GPU), Chrome, fixed benchmark pose, 1920×1200 output.
`npm run dev`, then `__bench(60)` in the console.

| Preset | Internal res | Bounces | FPS | Frame | Trace |
|---|---|---|---|---|---|
| 1 performance | 960×600 | 1 | **140** | 7.2 ms | 5.2 ms |
| 2 balanced | 1152×720 | 2 | **81** | 12.3 ms | 9.7 ms |
| 3 high | 1632×1020 | 3 | **38** | 26.5 ms | 21.5 ms |
| 4 ultra | 1920×1200 | 4 | **18** | 55.2 ms | 48.2 ms |

Presets are chosen for **vblank headroom, not for the highest resolution that
"hits 60"** — see frame pacing below.

`spp` stays at 1 everywhere deliberately. With temporal accumulation running, a
second sample per frame costs a full extra trace for far less benefit than
spending the same time on resolution or another bounce.

## Frame pacing

Average FPS is close to useless for judging smoothness. A build alternating
16 ms / 25 ms and one holding a flat 20 ms both report "about 50 fps", and only
the first feels broken. On a fixed-refresh display what matters is how many
vblank intervals each frame occupies and whether that count is *stable*.

The original "balanced" preset ran ~16.6 ms on a 120 Hz panel — sitting exactly
on the two-vblank boundary, so normal variance flipped 44% of frames to three
intervals and the result juddered between 60 and 40 fps while the HUD serenely
reported 60. Modelling that:

| GPU work | Result at 120 Hz |
|---|---|
| 7 ms | p95 8.3 ms, judder 0% |
| 16.6 ms ±2 | p95 25 ms, **judder 44%** |
| 13 ms ±2 | p95 16.7 ms, judder 0% |

So the HUD now reports p50/p95/p99, measured refresh, headroom against the
budget, judder %, and a live sparkline — and **adaptive resolution** regulates
p95 against a chosen vblank target instead of hoping a fixed preset fits.

### Where the time actually goes

Decomposed by ablation at 1152×720 / 2 bounces, because guessing here is how you
optimise the wrong thing:

| Component | Cost | Share |
|---|---|---|
| Primary + bounce-0 NEE (3 shadow rays) | 3.56 ms | 24% |
| **Bounce 1** | **6.64 ms** | **44%** |
| Bounce 2 | 4.22 ms | 28% |
| Volumetric | 0.65 ms | 4% |

Incoherent secondary rays are **72%** of the trace. That is what demoted the
hybrid raster G-buffer from "do this first" to "worth ~8%": it can only touch
part of the 24% primary slice.

This ablation predates the unified ReSTIR reservoir: bounce 0 now spends one
shadow ray, not three (see the work counters, `__stats.counters`, for the
current per-pixel ray and node-visit counts — machine-independent, so they can
be read off any device).

### Optimisations that mattered

Measured, not assumed, each against the same pinned pose:

- **Unified RIS for indirect bounces.** Deeper bounces used to pick one of three
  light channels uniformly and scale by 3. That is a large variance amplifier
  exactly where it shows: a bounce point inside the flashlight beam alternates
  between a bright sample and zero (moon drawn, occluded indoors), so any wall
  lit only by bounce off that floor fills with speckle. Resampling across every
  light including the flashlight picks whichever one actually matters and keeps
  the estimator weight near 1. It also made `spp=2` unnecessary — a **2x saving**,
  since the second sample had only ever been there to fight this noise.
- **BVH leaf size 4 → 1** — 15.07 / 14.00 / 13.12 ms at leaf ≤4 / ≤2 / ≤1.
  Ray-OBB is cheap enough that maximising cull quality beats minimising tree
  depth. **13% for a one-line change.**
- **Shadow-ray contribution culling** — skip the ray when the unshadowed
  estimate is below a threshold. Combined with caching the emissive flag in the
  box struct (instead of a dependent load into `materials[]` inside the shadow
  loop) and halving shadow rays on secondary bounces: **11.1 ms → 6.7 ms**.
- **RIS for light selection** — cost ~1.8 ms, removed nearly all the sparkle,
  and made adding 15 more practicals essentially free.
- **Firefly control.** SVGF's luminance edge-stopping actively *protects* lone
  bright pixels from being filtered, so fireflies have to be killed at the
  source. Three fixes: correcting the VNDF `G2/G1` weight, making the
  specular-lobe selection probability track view-dependent Fresnel (at grazing
  angles `F→1`, and picking specular only 4% of the time there divided
  throughput by 0.04 and manufactured fireflies), and a 3σ neighbourhood
  outlier clamp before temporal accumulation.

### Indirect checkerboarding: off by default

Tracing indirect bounces for half the pixels per frame (alternating whole
workgroup tiles, so entire wavefronts skip — a per-pixel checkerboard saves
almost nothing, because adjacent pixels share a wavefront and one active lane
keeps all of them walking the indirect path) is available on the `indirect rate`
slider, but it is **off by default** and forced inert below two bounces.

The saving scales with how much work sits behind the first bounce, and at low
bounce counts there is almost none:

| Bounces | rate 1.0 | rate 0.5 | Saved |
|---|---|---|---|
| 1 | 11.30 ms | 11.08 ms | 0.21 ms (2%) |
| 2 | 14.68 ms | 11.89 ms | 2.79 ms (19%) |

The noise cost, meanwhile, is the same either way — and it only appears *in
motion*. A converged A/B (pin the camera, render 120 frames, screenshot) cannot
find it, because the whole premise of checkerboarding is that temporal
accumulation reconstructs the missing pixels, which is exactly what a static
camera lets it do. Judge this one while moving, or not at all.

### Optimisations that did not work

Both looked obviously correct and both measured slower. Each is recorded in a
comment at the site so nobody re-tries it.

- **Tiling the ceiling** into an 8×6 grid so the BVH could cull it: **12%
  regression** (63.8 → 56.3 fps). One huge box costs a single cheap OBB test
  that resolves immediately; 48 tiles add BVH levels every upward ray descends.
- **Bounding the dynamic boxes** and gating the character's 20 limbs behind one
  slab test: **2% regression** (14.51 → 14.85 ms, consistent over 4 samples).
  Secondary rays are incoherent, so a wavefront nearly always holds both rays
  that reach the character and rays that do not — the branch cannot be taken
  uniformly and the slab test is pure added cost.
- **Workgroup size** was already optimal: 8×4 (32 threads) 16.30 ms, 8×8 (64)
  13.12 ms, 16×8 (128) 13.73 ms.

### A note on the profiler

`post` reports ~10–20 ms in the HUD. That is not real work — it is swapchain
acquire wait being caught inside the pass timestamps, and it inflates whenever
the tab is not being actively composited. Wall-clock frame time matches the sum
of *all other* passes almost exactly, which is how the artefact was identified.
Trust `wallMsPerFrame`.

## Two things worth knowing about the lighting setup

**Ceilings exist but the camera cannot see them** (`FLAG_NO_CAMERA`). This is
what makes the cutaway view work. Without a ceiling the moon and sky pour into
every room and there is nowhere dark to hide; with a visible one there is
nothing to look at. Shadow and bounce rays hit it, so rooms get real interior
bounce light and the moon only enters through windows.

**The windows are real holes.** They started as opaque "glass" boxes sunk into a
solid wall, which blocks every shadow ray, so no exterior light could ever reach
the floor. The facade is now built as piers, sills and headers with genuine
openings, which is what produces the long slanted moonlight pools.

## Browser notes

Chrome/Dawn is the development target. It has `subgroups` (134),
`subgroup_uniformity` (145), immediates/push constants (149–150) and raised
storage-buffer limits (146). Safari 26.6 has none of those — WebGPU there is
gated on macOS Tahoe 26+ regardless of Safari version, and the one rigorous
published cross-browser measurement on Apple Silicon has Chrome ahead of Safari
by 11–21% on a compute workload.

The demo requests `maxStorageTexturesPerShaderStage: 8` (the default of 4 is not
enough for the G-buffer pass); Apple reports 8.

## Layout

```
src/
  core/math.ts          vec3/mat4; rotation convention documented here
  scene/
    scene.ts            Box/Material/Light, GPU packing, flags
    bvh.ts              binned-SAH BVH, flattened for the GPU
    level.ts            the procedural office
  engine/
    gpu.ts              device init, feature/limit negotiation
    shaders.ts          WGSL assembly + compilation diagnostics
    renderer.ts         pass orchestration, ping-pong resources
    profiler.ts         per-pass timestamp queries
  anim/rig.ts           skeleton sampling, blending, bone masks
  game/                 input, camera, player, character box rig
tools/extract-rig.mjs   offline GLB -> packed rig (15 MB -> 171 KB)
public/rig.{json,bin}   generated; re-run the tool after changing the clip list
  shaders/
    common.wgsl         scene bindings, intersection, BSDF, light sampling
    pathtrace.wgsl      primary rays, path integration, volumetrics
    reproject.wgsl      temporal accumulation, anti-firefly
    atrous.wgsl         edge-aware wavelet filter
    composite.wgsl      re-modulate, debug views
    bloom.wgsl / post.wgsl
```

## Next steps

Roughly in order of value:

1. **ReSTIR DI** — temporal and spatial reservoir reuse. The RIS reservoir is
   already there; reusing it across frames and neighbours is the single biggest
   remaining quality win, and would let the local-light shadow ray count drop.
2. **Per-object motion vectors.** Reprojection is correct for static geometry
   only. This mattered little when the player was seven boxes; now that it is a
   fully animated character it is the main source of lost accumulation.
3. **`shader-f16`** for BVH bounds and box rotations. Note that f16 *positions*
   are not viable — at 30 m the ulp is ~3 cm, which would visibly distort
   geometry. BVH bounds can use it safely with outward rounding, since a
   conservatively larger AABB is still correct.
4. **A hybrid raster G-buffer** — worth ~8% given the measured breakdown, so it
   is well down the list despite being the obvious-sounding move.
5. Guards, vision cones, and light-based detection — the tracer already computes
   how lit the player is, so detection can be the lighting you are looking at
   rather than a separate system that disagrees with it.
