// ===========================================================================
// Primary visibility + path traced radiance.
//
// Writes an albedo-demodulated radiance buffer plus the G-buffer the denoiser
// needs. Demodulating by albedo before filtering is what lets the a-trous pass
// blur aggressively without smearing texture/colour detail across surfaces.
// ===========================================================================

@group(1) @binding(0) var gAlbedo : texture_storage_2d<rgba8unorm, write>;
@group(1) @binding(1) var gNormalDepth : texture_storage_2d<rgba16float, write>;
@group(1) @binding(2) var gPos : texture_storage_2d<rgba32float, write>;
/**
 * Radiance output: one array texture, four layers, one storage-texture slot
 * (which keeps this pass inside WebGPU's default budget of 4 per stage).
 *
 * The signals are separate layers, never summed, because they want
 * incompatible denoising. Direct carries hard shadow edges that must survive
 * filtering; indirect is low frequency and wants blurring hard — one filter
 * cannot serve both. Transient (muzzle flash) light wants no temporal history
 * at all: a long history leaves a glow hanging after the flash is gone, and a
 * short one throws away the whole screen's convergence, so that layer is
 * simply not accumulated and appears/vanishes with the light by construction.
 * The volume layer is in-scattered radiance (rgb) with the ray transmittance
 * in alpha: smooth, so it takes a short clamped history and a wide a-trous.
 */
@group(1) @binding(3) var illumOut : texture_storage_2d_array<rgba16float, write>;
const ILLUM_DIRECT : u32 = 0u;
const ILLUM_INDIRECT : u32 = 1u;
const ILLUM_TRANSIENT : u32 = 2u;
const ILLUM_VOLUME : u32 = 3u;
@group(1) @binding(4) var prevNormalDepth : texture_2d<f32>;
/**
 * Reservoirs for both frames of the ping-pong in one buffer of 2*W*H entries:
 * half U.parity is this frame's (written), the other half is last frame's and
 * is read-only by convention — see resBase(). GI likewise. One read_write
 * binding per pair, so the two pairs cost two storage-buffer slots, not four.
 */
@group(1) @binding(5) var<storage, read_write> reservoirs : array<Reservoir>;
/** Work counters — see common.wgsl. Flushed once per invocation at the end of main. */
@group(1) @binding(6) var<storage, read_write> counters : array<atomic<u32>>;
@group(1) @binding(8) var<storage, read_write> giReservoirs : array<GIReservoir>;
/**
 * Baked static light volume — see lightvolume.wgsl. rgb = in-scattered
 * radiance per unit scattering coefficient from every static light,
 * trilinearly sampled at each march step. Its sampler (linear, clamp) is
 * shared with the smoke density volume.
 */
@group(1) @binding(10) var lightVol : texture_3d<f32>;
@group(1) @binding(15) var volSampler : sampler;

/** Baked static-light in-scatter at p (per unit scattering coefficient). */
fn lightVolumeSample(p: vec3f) -> vec3f {
  let ext = vec3f(textureDimensions(lightVol)) * U.lightVolCell;
  let uvw = (p - U.lightVolOrigin) / ext;
  return textureSampleLevel(lightVol, volSampler, uvw, 0.0).rgb;
}

/**
 * Reference-mode ground truth for the light volume: the same integrand it
 * discretises — every static light, jittered emitter, static-scene
 * visibility (occludedStatic, as the bake: dynamic geometry never shadows
 * the static in-scatter in either estimator), isotropic phase — estimated
 * by Monte Carlo per march step. lights[0] (the moon, source of the
 * god-ray pools) is sampled every step; the practicals are subsampled
 * uniformly and re-weighted, and the accumulator averages the rest away.
 */
fn staticScatterSample(li: u32, p: vec3f, weight: f32) -> vec3f {
  let l = lights[li];
  if (l.intensity <= 0.0) { return vec3f(0.0); }
  let smp = sampleSphereLight(l, p);
  var atten = 1.0;
  if (l.kind == LIGHT_SPOT) {
    atten = spotAttenuation(l.dir, -smp.dir, l.cosInner, l.cosOuter);
    if (atten <= 0.0) { return vec3f(0.0); }
  }
  countWork(CT_shadowVolumetric);
  if (occludedStatic(p, smp.dir, smp.dist - EPS * 8.0)) { return vec3f(0.0); }
  return smp.radiance * (atten * weight * (1.0 / (4.0 * PI)));
}

fn staticScatterMC(p: vec3f) -> vec3f {
  let S = U.dynLightStart;
  if (S == 0u) { return vec3f(0.0); }
  var e = staticScatterSample(0u, p, 1.0);
  if (S > 1u) {
    const K = 3u;
    for (var k = 0u; k < K; k = k + 1u) {
      let idx = 1u + min(u32(rand() * f32(S - 1u)), S - 2u);
      e = e + staticScatterSample(idx, p, f32(S - 1u) / f32(K));
    }
  }
  return e;
}

/** Base index of a parity half; the two halves swap roles every frame. */
fn resBase(dims: vec2u, half: u32) -> u32 {
  return select(0u, dims.x * dims.y, half == 1u);
}
/** Torch depth maps, traced by flashmap.wgsl earlier in the frame. */
@group(1) @binding(11) var flashDepth : texture_2d_array<f32>;
/**
 * Radiosity per-patch data as a texture (the storage-buffer budget is full):
 * texel (i, 0) = gathered indirect irradiance G, texel (i, 1) = sky-dome
 * irradiance per unit sky intensity, texel (i, 2) = outgoing radiosity B as
 * an emitter, texel (i, 3) = inclusive CDF over luminance(B)*area.
 */
@group(1) @binding(12) var radGSky : texture_2d<f32>;
/** Radiosity face table: texel (boxIdx*6+face, 0) = {patchBase, gridW, gridH}. */
@group(1) @binding(13) var radFaces : texture_2d<u32>;
// 14/15: the volumetrics track's smoke volume and sampler. Do not use here.
/**
 * Patch geometry, the same packed layout radiosity.wgsl writes (16 f32 per
 * patch: pos/area, n/mat, tu/hu, tv/hv). Read-only, and this is the trace
 * pass's tenth and final storage buffer — see gpu.ts.
 */
@group(1) @binding(16) var<storage, read> radStatic : array<f32>;

struct RadPatch {
  pos  : vec3f,
  area : f32,
  n    : vec3f,
  tu   : vec3f,
  hu   : f32,
  tv   : vec3f,
  hv   : f32,
}

fn radPatchAt(i: u32) -> RadPatch {
  countWork(CT_radiosityGathers);
  let b = i * 16u;
  var p: RadPatch;
  p.pos = vec3f(radStatic[b], radStatic[b + 1u], radStatic[b + 2u]);
  p.area = radStatic[b + 3u];
  p.n = vec3f(radStatic[b + 4u], radStatic[b + 5u], radStatic[b + 6u]);
  p.tu = vec3f(radStatic[b + 8u], radStatic[b + 9u], radStatic[b + 10u]);
  p.hu = radStatic[b + 11u];
  p.tv = vec3f(radStatic[b + 12u], radStatic[b + 13u], radStatic[b + 14u]);
  p.hv = radStatic[b + 15u];
  return p;
}

/** G + sky, combined, for one patch index. */
fn radPatchIrradiance(i: u32) -> vec3f {
  countWork(CT_radiosityGathers);
  let c = vec2i(i32(i), 0);
  return textureLoad(radGSky, c, 0).xyz
    + textureLoad(radGSky, c + vec2i(0, 1), 0).xyz * U.skyIntensity;
}

/** Sky row only, for one patch index. */
fn radPatchSky(i: u32) -> vec3f {
  countWork(CT_radiosityGathers);
  return textureLoad(radGSky, vec2i(i32(i), 1), 0).xyz * U.skyIntensity;
}

/**
 * The patch grid of the face `h` lies on, plus the bilinear cell address.
 * base == BOX_NONE marks a face below the patch builder's area floor. The
 * face and cell conventions must match src/scene/radiosity.ts exactly.
 */
struct RadGrid {
  base : u32,
  gw   : u32,
  gh   : u32,
  fu   : f32,
  fv   : f32,
}

fn radFaceGrid(h: Hit) -> RadGrid {
  var out: RadGrid;
  out.base = BOX_NONE;
  let b = boxes[h.boxIdx];
  let d = h.p - b.center;
  var lp: vec3f;
  var ln: vec3f;
  if ((b.flags & FLAG_AXIS_ALIGNED) != 0u) {
    lp = d;
    ln = h.n;
  } else {
    lp = vec3f(dot(d, b.rot0), dot(d, b.rot1), dot(d, b.rot2));
    ln = vec3f(dot(h.n, b.rot0), dot(h.n, b.rot1), dot(h.n, b.rot2));
  }

  var face: u32;
  var u01: f32;
  var v01: f32;
  let a = abs(ln);
  if (a.x >= a.y && a.x >= a.z) {
    face = select(1u, 0u, ln.x > 0.0);
    u01 = lp.z / max(b.half.z, 1e-4);
    v01 = lp.y / max(b.half.y, 1e-4);
  } else if (a.y >= a.z) {
    face = select(3u, 2u, ln.y > 0.0);
    u01 = lp.x / max(b.half.x, 1e-4);
    v01 = lp.z / max(b.half.z, 1e-4);
  } else {
    face = select(5u, 4u, ln.z > 0.0);
    u01 = lp.x / max(b.half.x, 1e-4);
    v01 = lp.y / max(b.half.y, 1e-4);
  }

  let ft = textureLoad(radFaces, vec2i(i32(h.boxIdx * 6u + face), 0), 0);
  if (ft.x == BOX_NONE) { return out; }
  let gw = f32(ft.y);
  let gh = f32(ft.z);
  out.base = ft.x;
  out.gw = ft.y;
  out.gh = ft.z;
  out.fu = clamp((u01 * 0.5 + 0.5) * gw - 0.5, 0.0, gw - 1.0);
  out.fv = clamp((v01 * 0.5 + 0.5) * gh - 0.5, 0.0, gh - 1.0);
  return out;
}

/** Bilinear over the face grid of one radGSky quantity: row 0 = G+sky, 1 = sky. */
fn radBilinear(g: RadGrid, skyOnly: bool) -> vec3f {
  let u0 = u32(floor(g.fu));
  let v0 = u32(floor(g.fv));
  let u1 = min(u0 + 1u, g.gw - 1u);
  let v1 = min(v0 + 1u, g.gh - 1u);
  let du = fract(g.fu);
  let dv = fract(g.fv);
  let i00 = g.base + v0 * g.gw + u0;
  let i10 = g.base + v0 * g.gw + u1;
  let i01 = g.base + v1 * g.gw + u0;
  let i11 = g.base + v1 * g.gw + u1;
  if (skyOnly) {
    return mix(mix(radPatchSky(i00), radPatchSky(i10), du),
               mix(radPatchSky(i01), radPatchSky(i11), du), dv);
  }
  return mix(mix(radPatchIrradiance(i00), radPatchIrradiance(i10), du),
             mix(radPatchIrradiance(i01), radPatchIrradiance(i11), du), dv);
}

/**
 * Indirect illumination at a static hit, from the radiosity patches.
 *
 * Returns DEMODULATED radiance (incident indirect irradiance / pi — the same
 * quantity the traced indirect stores after albedo division), or x = -1 for
 * faces below the patch builder's area floor, which the caller treats as
 * "no data".
 */
fn radiosityIndirect(h: Hit) -> vec3f {
  let grid = radFaceGrid(h);
  if (grid.base == BOX_NONE) { return vec3f(-1.0, 0.0, 0.0); }
  return radBilinear(grid, false) * INV_PI;
}

/**
 * Direct sky irradiance / pi at a static hit, from the sky row alone. Zero
 * where there is no patch data. patchRIS rebuilds surface-to-surface indirect
 * from the patches themselves, so the sky's own part has to come from here.
 */
fn radiositySky(h: Hit) -> vec3f {
  let grid = radFaceGrid(h);
  if (grid.base == BOX_NONE) { return vec3f(0.0); }
  return radBilinear(grid, true) * INV_PI;
}

/**
 * PCF visibility of a spot light from `p`, read from its depth-map layer.
 *
 * Returns 1.0 outside the cone or behind the lens: the cone attenuation
 * already owns those zeros, and double-counting them into a target function
 * would just re-zero something that is zero.
 */
fn torchMapSample(layer: u32, lpos: vec3f, axis: vec3f, cosOuter: f32, p: vec3f) -> f32 {
  let basis = onb(axis);
  let delta = p - lpos;
  let local = vec3f(dot(delta, basis[0]), dot(delta, basis[1]), dot(delta, basis[2]));
  if (local.z <= 1e-3) { return 1.0; }
  let uv = local.xy / (local.z * tanFromCos(cosOuter));
  if (max(abs(uv.x), abs(uv.y)) >= 1.0) { return 1.0; }

  let r = length(delta);
  let f = (uv * 0.5 + 0.5) * f32(FLASHMAP_RES) - 0.5;
  let base = vec2i(floor(f));
  let fr = f - floor(f);
  let w = array<f32, 4>(
    (1.0 - fr.x) * (1.0 - fr.y),
    fr.x * (1.0 - fr.y),
    (1.0 - fr.x) * fr.y,
    fr.x * fr.y,
  );
  let offs = array<vec2i, 4>(vec2i(0, 0), vec2i(1, 0), vec2i(0, 1), vec2i(1, 1));

  var vis = 0.0;
  for (var i = 0; i < 4; i = i + 1) {
    let c = clamp(base + offs[i], vec2i(0), vec2i(FLASHMAP_RES - 1));
    let d = textureLoad(flashDepth, c, i32(layer), 0).r;
    // Relative + absolute slack: a receiver that IS the stored surface has to
    // compare visible against its own depth sample.
    vis = vis + w[i] * select(0.0, 1.0, r <= d * 1.02 + 0.10);
  }
  return vis;
}

/** Layer 0: the player's flashlight. */
fn flashmapSample(p: vec3f) -> f32 {
  return torchMapSample(0u, U.flashPos, U.flashDir, U.flashCosOuter, p);
}

/**
 * The depth-map visibility as an RIS target factor.
 *
 * Floored, never zero: an RIS target must stay positive wherever the
 * integrand can be nonzero, or map error turns into energy loss instead of a
 * little extra variance. A truly-lit point the map calls blocked still gets
 * proposed occasionally, survives its real shadow ray, and is re-weighted by
 * 1/p-hat exactly as RIS prescribes.
 */
fn flashTargetVis(p: vec3f) -> f32 {
  if (U.flashVisP <= 0.5 || U.flashIntensity <= 0.0) { return 1.0; }
  return max(flashmapSample(p), 0.08);
}

// ---------------------------------------------------------------------------
// PARTICIPATING MEDIUM — the density interface (contract with Track B2b).
//
// The medium occupies the smokeVolume grid's box: origin U.smokeOrigin (its
// world minimum corner), cubic cells of U.smokeCell metres, dims from
// textureDimensions(smokeVolume) — 208 x 13 x 144 (x, y, z) at 0.25 m: the
// room's air, x in [-26, 26], y in [0, 3.25], z in [-18, 18]. The camera
// march clips to exactly this box. The grid is a compile-time constant
// (SMOKE_DIMS / SMOKE_CELL / MEDIUM_ORIGIN in renderer.ts, everything else
// derived from them): a resize is a coordinated constant edit + texture
// reallocation, never a runtime uniform change; cells stay cubic and the box
// stays dims x cell exactly. The top row (y 3.0-3.25) straddles the ceiling
// underside at y = 3.2 — 0.25 m cells cannot tile 3.2, so the box overshoots
// the room by 5 cm; treat y >= 3.2 as your solid top wall.
//
//   mediumDensity(p) = densityStatic(p) + smokeDensity(p)   (dimensionless)
//   sigma_t(p)       = U.volumetric * mediumDensity(p)      (1/m, albedo 1)
//
// densityStatic: the drifting fog (mean density U.fogAmount, noise-textured).
//   The old puff uniforms are retired: every smoke source (muzzle bursts,
//   impacts, smoulder, wisps, canisters) now enters the fluid simulation
//   (src/engine/fluid.ts, fluid.wgsl), which owns the texture below.
// smokeVolume: texture_3d<f32>, storage format rgba16float, R = density
//   (G/B/A zero; rgba16float storage is write-only from a kernel, so the
//   sim keeps its own state textures and writes density here as an OUTPUT
//   in its last pass each frame), @group(1) @binding(14), read trilinearly
//   through the shared linear-clamp sampler at @binding(15); zero outside
//   its box. The simulation runs on its own lattice (the interface grid at
//   the default debug scale) and resamples into this fixed 208 x 13 x 144
//   interface; the retired __smokeTest debug filler is gone.
// Sampling convention: uvw = (p - U.smokeOrigin) / (dims * U.smokeCell),
//   voxel centres at half cells. Anything that scatters or absorbs enters
//   through mediumDensity() and nowhere else.
// ---------------------------------------------------------------------------

/** The simulation's smoke density; see the contract above. */
@group(1) @binding(14) var smokeVolume : texture_3d<f32>;

fn smokeDensity(p: vec3f) -> f32 {
  let ext = vec3f(textureDimensions(smokeVolume)) * U.smokeCell;
  let uvw = (p - U.smokeOrigin) / ext;
  if (any(uvw < vec3f(0.0)) || any(uvw > vec3f(1.0))) { return 0.0; }
  return textureSampleLevel(smokeVolume, volSampler, uvw, 0.0).r;
}

/** Integer-lattice hash reusing the RNG's PCG core — no trig, no precision cliffs. */
fn hashLattice(p: vec3i) -> f32 {
  let n = pcgHash((u32(p.x) * 1597334673u) ^ (u32(p.y) * 3812015801u) ^ (u32(p.z) * 2798796415u));
  return f32(n) * 2.3283064365386963e-10;
}

fn valueNoise(p: vec3f) -> f32 {
  let i = vec3i(floor(p));
  let f = p - floor(p);
  let u = f * f * (3.0 - 2.0 * f);
  let c000 = hashLattice(i);
  let c100 = hashLattice(i + vec3i(1, 0, 0));
  let c010 = hashLattice(i + vec3i(0, 1, 0));
  let c110 = hashLattice(i + vec3i(1, 1, 0));
  let c001 = hashLattice(i + vec3i(0, 0, 1));
  let c101 = hashLattice(i + vec3i(1, 0, 1));
  let c011 = hashLattice(i + vec3i(0, 1, 1));
  let c111 = hashLattice(i + vec3i(1, 1, 1));
  let x00 = mix(c000, c100, u.x);
  let x10 = mix(c010, c110, u.x);
  let x01 = mix(c001, c101, u.x);
  let x11 = mix(c011, c111, u.x);
  return mix(mix(x00, x10, u.y), mix(x01, x11, u.y), u.z);
}

/** Drifting fog: mean density U.fogAmount, the value noise its texture. */
fn fogDensity(p: vec3f) -> f32 {
  if (U.fogAmount <= 0.0) { return 0.0; }
  let w = U.time * U.fogSpeed;
  // Two octaves; the second drifts against the first so the fog churns
  // rather than sliding as one sheet.
  let n = valueNoise(p * 0.85 + vec3f(w, w * 0.31, -w * 0.62)) * 0.7
        + valueNoise(p * 2.1 + vec3f(-w * 0.8, w * 0.47, w * 1.13)) * 0.3;
  // valueNoise has mean 0.5, so the remap has mean ~1: fogAmount sets the
  // density and the noise only its texture.
  return U.fogAmount * clamp(2.0 * n, 0.0, 2.0);
}

/**
 * The static half of the density — the ambient fog. Everything that used to
 * live here as smoke puffs is simulated now and arrives via smokeDensity().
 */
fn densityStatic(p: vec3f) -> f32 {
  return fogDensity(p);
}

/** Local density of the medium: the static field plus the smoke simulation. */
fn mediumDensity(p: vec3f) -> f32 {
  return densityStatic(p) + smokeDensity(p);
}

/**
 * Density seen by a light integral: the fog by its mean (a beam's dimming
 * does not need the fog's texture, which is most of the density's cost) plus
 * the simulated smoke.
 */
fn densityForLight(p: vec3f) -> f32 {
  return U.fogAmount + smokeDensity(p);
}

/**
 * Henyey-Greenstein phase function.
 *
 * This form peaks at cosTheta = +1, so its argument must be the cosine between
 * the *propagation* directions in and out — not the outward-facing vectors the
 * PBRT form uses. Passing the wrong one negates the effective g, turning
 * forward scattering into back scattering, which is what this call used to do.
 */
fn phaseHG(cosTheta: f32, g: f32) -> f32 {
  let g2 = g * g;
  let d = 1.0 + g2 - 2.0 * g * cosTheta;
  return (1.0 - g2) / (4.0 * PI * max(d * sqrt(max(d, 1e-6)), 1e-6));
}

/**
 * Transmittance from p toward a lamp `dist` away along `dir`, from four
 * midpoint taps of the light-integral density in between — enough that a
 * beam passing through dense smoke arrives visibly dimmed. Deterministic
 * taps, not jittered ones: a noisy optical depth inside exp() biases the
 * temporally averaged transmittance upward (Jensen), which hid most of the
 * dimming. Torch beams only: the baked static volume ignores dynamic
 * density, and a transient flash is over before the difference would read.
 */
fn towardLightTransmittance(p: vec3f, dir: vec3f, dist: f32) -> f32 {
  if (U.volExtinction <= 0.5) { return 1.0; }
  let seg = dist * 0.25;
  var od = 0.0;
  for (var k = 0; k < 4; k = k + 1) {
    let q = p + dir * ((f32(k) + 0.5) * seg);
    od = od + densityForLight(q);
  }
  return exp(-U.volumetric * od * seg);
}

/**
 * In-scattering along the camera ray, in colour, from every light in the
 * level: the moon and the practicals via the baked light volume, the player's
 * and guards' torches via their depth maps, live transients via real shadow
 * rays. Written to its own radiance layer (ILLUM_VOLUME) with the ray's
 * transmittance in alpha, and denoised by its own reproject/a-trous chain
 * tuned for a volume — so a warm torch and a cool one keep their own tints
 * in the air, not just on the surfaces.
 */
const VOL_TORCH_RANGE2: f32 = 14.0 * 14.0;

/**
 * Ray parameter range inside the medium (the smokeVolume box — the room's
 * air), clipped to [0, tmax]. Returns y <= x when the ray never enters it.
 * The camera sits ~20 m above the slab, so an unclipped march would spend
 * most of its steps in air that cannot scatter; clipping makes the same step
 * count sample the room several times denser.
 */
fn mediumRange(ro: vec3f, rd: vec3f, tmax: f32) -> vec2f {
  let bmin = U.smokeOrigin;
  let bmax = bmin + vec3f(textureDimensions(smokeVolume)) * U.smokeCell;
  let invD = 1.0 / rd;
  let t1 = (bmin - ro) * invD;
  let t2 = (bmax - ro) * invD;
  let tn = min(t1, t2);
  let tf = max(t1, t2);
  let tNear = max(max(max(tn.x, tn.y), tn.z), 0.0);
  let tFar = min(min(min(tf.x, tf.y), tf.z), tmax);
  return vec2f(tNear, tFar);
}

struct VolumetricResult {
  /** In-scattered radiance reaching the camera along the ray, in colour. */
  inscatter : vec3f,
  /** Transmittance of the medium between the camera and the surface behind. */
  transmittance : f32,
}

fn volumetricBeams(ro: vec3f, rd: vec3f, tmax: f32) -> VolumetricResult {
  var out: VolumetricResult;
  out.inscatter = vec3f(0.0);
  out.transmittance = 1.0;
  if (U.volumetric <= 0.0) { return out; }

  // The march is jittered and the result goes through temporal accumulation,
  // so far fewer steps than a single clean frame would need still resolve.
  let range = mediumRange(ro, rd, tmax);
  if (range.y <= range.x) { return out; }
  let steps = max(2u, u32(U.volSteps));
  let dt = (range.y - range.x) / f32(steps);
  let jitter = rand();
  let absorb = U.volExtinction > 0.5;
  // Camera-ray transmittance so far; the surface behind is dimmed by the
  // final value and each step's scatter is what still reaches the camera.
  var T = 1.0;

  for (var i = 0u; i < steps; i = i + 1u) {
    let t = range.x + (f32(i) + jitter) * dt;
    if (t >= range.y) { break; }
    countWork(CT_volumeSteps);
    let p = ro + rd * t;

    // The medium is shared by every light at this step: churn in the fog and
    // smoke from a fresh shot modulate all the beams alike. U.volumetric is
    // the extinction coefficient at unit density (1/m); scattering albedo is
    // 1, so the same figure scatters.
    let sigmaS = U.volumetric * mediumDensity(p);
    // Radiance the medium at p scatters toward the camera, per unit sigmaS.
    var stepIn = vec3f(0.0);

    // ---- static lights: the moon and every practical ---------------------
    // Read from the baked light volume — the reason haze and god-rays are
    // free of per-step shadow rays. Reference mode traces the real thing.
    if (U.volRefMode > 0.5) {
      stepIn = stepIn + staticScatterMC(p);
    } else {
      stepIn = stepIn + lightVolumeSample(p);
    }

    // ---- the player's torch ----------------------------------------------
    if (U.flashIntensity > 0.0) {
      let delta = U.flashPos - p;
      let d2 = max(dot(delta, delta), 0.05);
      let dist = sqrt(d2);
      let dir = delta / dist;

      // Cheap cone rejection first — most of the screen is outside the beam
      // and pays nothing beyond this test. This is what keeps the extra beams
      // below from costing what a second full march would.
      let cone = spotAttenuation(U.flashDir, -dir, U.flashCosInner, U.flashCosOuter);
      if (cone > 0.001) {
        // The depth map replaces the per-step shadow ray — the march's
        // dominant cost, since rays inside the beam are unoccluded by
        // definition and walk the whole BVH. The PCF fraction is used as soft
        // attenuation rather than a binary test; the jittered march resolves
        // it into smooth beam edges for free. Guards' torches below still
        // trace: only the player's light has a map.
        var vis = 0.0;
        if (U.flashVisVol > 0.5) {
          vis = flashmapSample(p);
        } else {
          countWork(CT_shadowVolumetric);
          if (!occluded(p, dir, dist - EPS * 8.0)) { vis = 1.0; }
        }
        if (vis > 0.0) {
          // dir points from the march point toward the lamp, so light
          // propagates along -dir and the scattered light reaching the camera
          // propagates along -rd. cos(theta) = dot(-dir, -rd) = dot(dir, rd).
          let phase = phaseHG(dot(rd, dir), 0.55);
          let tl = towardLightTransmittance(p, dir, dist);
          stepIn = stepIn
            + U.flashColor * (vis * cone * phase * falloff(d2) * U.flashIntensity * tl);
        }
      }
    }

    // ---- guards' torches --------------------------------------------------
    // A guard's beam is the one thing in the level that sweeps, and a visible
    // shaft is what makes it read as a thing to stay out of rather than a
    // bright patch on the floor. Transients are excluded: a muzzle flash is
    // omnidirectional and lasts a few frames, so a shaft would be wrong twice.
    for (var li = U.dynLightStart; li < U.transientStart; li = li + 1u) {
      let l = lights[li];
      if (l.intensity <= 0.0 || l.kind != LIGHT_SPOT) { continue; }
      let delta = l.pos - p;
      let d2 = max(dot(delta, delta), 0.05);
      // Distance first, before the cone test and long before a shadow ray. A
      // torch is inverse-square at intensity 170, so past this radius it
      // contributes less than the dither and is not worth a BVH walk.
      if (d2 > VOL_TORCH_RANGE2) { continue; }
      let dist = sqrt(d2);
      let dir = delta / dist;
      let cone = spotAttenuation(l.dir, -dir, l.cosInner, l.cosOuter);
      if (cone <= 0.001) { continue; }
      // Each guard torch has its own depth-map layer, so the march pays a
      // texture load per step instead of a BVH walk — the same trade as the
      // player's beam, and the reason five beams no longer cost 16% of the
      // frame. Torches past the layer budget fall back to the real ray.
      let layer = li - U.dynLightStart + 1u;
      var vis = 1.0;
      if (U.flashVisVol > 0.5 && layer < TORCH_LAYERS) {
        vis = torchMapSample(layer, l.pos, l.dir, l.cosOuter, p);
        if (vis <= 0.0) { continue; }
      } else {
        countWork(CT_shadowVolumetric);
        if (occluded(p, dir, dist - EPS * 8.0)) { continue; }
      }
      let phase = phaseHG(dot(rd, dir), 0.55);
      let tl = towardLightTransmittance(p, dir, dist);
      stepIn = stepIn + l.color * (vis * cone * phase * falloff(d2) * l.intensity * tl);
    }

    // ---- transient lights ---------------------------------------------------
    // A muzzle flash lives ~3 frames, but while it does the smoke it just made
    // should glow from the inside — that pairing is most of why the puffs
    // exist. Isotropic phase: a flash has no beam axis. The loop costs nothing
    // while no flash is live (intensity check, no rays), and a live flash is
    // close and brief. The volume chain's clamped short history takes the
    // glow up and down with the light instead of hanging it in the air.
    // Reference mode excludes them, as its composite does the surface signal:
    // a flash is a lighting event, not part of the steady scene the 1/n
    // accumulator converges to.
    for (var li = select(U.transientStart, U.lightCount, U.volRefMode > 0.5);
         li < U.lightCount; li = li + 1u) {
      let l = lights[li];
      if (l.intensity <= 0.0) { continue; }
      let delta = l.pos - p;
      let d2 = max(dot(delta, delta), 0.25);
      // Same range cut as the torches: inverse-square makes the far field
      // cheaper to skip than to march.
      if (d2 > VOL_TORCH_RANGE2) { continue; }
      let dist = sqrt(d2);
      let dir = delta / dist;
      countWork(CT_shadowVolumetric);
      if (occluded(p, dir, dist - EPS * 8.0)) { continue; }
      stepIn = stepIn + l.color * ((1.0 / (4.0 * PI)) * falloff(d2) * l.intensity);
    }

    if (absorb) {
      // Closed-form segment: constant density over the step, albedo 1, so
      // sigmaS is also the extinction and (1 - stepT) of the arriving light
      // is what scatters — energy stays consistent at any step size.
      let stepT = exp(-sigmaS * dt);
      out.inscatter = out.inscatter + stepIn * (T * (1.0 - stepT));
      T = T * stepT;
      // Opaque smoke: nothing behind it reaches the camera.
      if (T < 0.005) { T = 0.0; break; }
    } else {
      out.inscatter = out.inscatter + stepIn * (sigmaS * dt);
    }
  }
  out.transmittance = T;
  return out;
}

/**
 * World position of another pixel's stored visible point, rebuilt from its
 * ray depth. Uses this frame's camera against last frame's depth — the same
 * near-static assumption the reprojection test already leans on — which is
 * ample for the sign test it feeds (diSupports below), never for geometry.
 */
fn pixelWorldPos(px: vec2i, t: f32) -> vec3f {
  let uv = (vec2f(px) + 0.5) * U.invResolution;
  return U.camPos + cameraRay(uv) * t;
}

/**
 * MIS support for the merge denominator: could the domain shaded at (x, n)
 * have proposed light sample `r` with a nonzero target? Mirrors
 * proposeCandidate's own rejections — behind the surface, or outside a spot's
 * cone at that point — so the answer is what the neighbour's RIS loop saw.
 */
fn diSupports(x: vec3f, n: vec3f, r: Reservoir) -> bool {
  let delta = r.samplePos - x;
  let d2 = dot(delta, delta);
  if (d2 <= 1e-8) { return true; }
  let dir = delta * inverseSqrt(d2);
  if (dot(n, dir) <= 0.0) { return false; }
  if (r.lightIdx == 0u) {
    return spotAttenuation(U.flashDir, -dir, U.flashCosInner, U.flashCosOuter) > 0.0;
  }
  let l = lights[r.lightIdx - 1u];
  if (l.kind == LIGHT_SPOT) {
    return spotAttenuation(l.dir, -dir, l.cosInner, l.cosOuter) > 0.0;
  }
  return true;
}

/**
 * ReSTIR direct illumination.
 *
 * Generates a few fresh candidates, merges last frame's reservoir for this
 * surface point, then spends exactly one shadow ray on the survivor. An
 * occluded survivor is zeroed rather than stored, so a sample known to be in
 * shadow is not handed to the next frame as if it were lit — that visibility
 * carry-over is where most of the saving comes from, because the old estimator
 * kept re-picking bright lights that turned out to be blocked.
 */
fn restirDirect(
  h: Hit, v: vec3f, m: Material,
  pixel: vec2u, prevWorld: vec3f, dims: vec2u,
) -> vec3f {
  // One depth-map lookup serves every candidate and merge at this shading
  // point: the map is a function of the receiver, not of the lens jitter.
  let fVis = flashTargetVis(h.p);
  var res = generateReservoir(h.p, h.n, v, m, u32(U.restirCandidates), fVis);

  /**
   * The merge denominator is Z = the M of every stream whose domain could
   * have produced the surviving sample, not the naive sum. Our own candidates
   * always qualify; each merged stream is judged against the FINAL sample
   * once it is known (support test at its own shading point), with one biased
   * carve-out kept from the tuned trunk: a stream killed by ITS OWN shadow
   * ray (W = 0, targetPdf > 0) is about its visibility, not our brightness,
   * and never votes. What this stops, measured: dead-but-supporting streams
   * left out let one lucky find be re-adopted at full weight by every dark
   * neighbour (5x direct inflation on a lone flashlight pool with temporal
   * reuse); non-supporting streams counted anyway diluted every beam-edge
   * pixel by its out-of-cone neighbours (-0.38 relBias flashlight-only).
   */
  let mOwn = res.M;
  /** M of same-point (temporal) streams that vote. */
  var zSame = 0.0;
  /** Spatial-tap shading contexts, judged after the sample is chosen. */
  var nCtx = 0u;
  var ctxPos : array<vec3f, 8>;
  var ctxNrm : array<vec3f, 8>;
  var ctxM   : array<f32, 8>;

  /**
   * What next frame reuses: the fresh + temporal stream, WITHOUT the spatial
   * taps. Storing the spatially-merged reservoir instead creates a feedback
   * loop — neighbours re-merge weight that already flowed through them, and
   * because the merge re-weights by the ratio of two pixels' target functions
   * (a ratio of random variables, expectation > 1), the drift compounds every
   * frame. Measured with the merged result stored: relBias 0.22 -> 1.28 at
   * 3 taps, 1.93 at 6. Kept separate, reuse draws from streams that are only
   * temporally fed and the loop cannot close.
   */
  var carry = res;

  // ---- temporal + spatial reuse ------------------------------------------
  let prevBase = resBase(dims, 1u - U.parity);
  let spatialTaps = u32(U.restirSpatialTaps);
  if (U.restirTemporal > 0.5 || spatialTaps > 0u) {
    let clip = U.prevViewProj * vec4f(prevWorld, 1.0);
    if (clip.w > 0.0) {
      let ndc = clip.xy / clip.w;
      let uv = vec2f(ndc.x * 0.5 + 0.5, 0.5 - ndc.y * 0.5);
      if (uv.x >= 0.0 && uv.x < 1.0 && uv.y >= 0.0 && uv.y < 1.0) {
        // uv < 1 does not bound the product: uv*dims can round up to exactly
        // dims, which with the merged buffer indexes the half being written.
        let pp = min(vec2i(uv * vec2f(dims)), vec2i(dims) - 1);
        if (U.restirTemporal > 0.5) {
          let pnd = textureLoad(prevNormalDepth, pp, 0);
          // Same surface test as the denoiser: reuse across a depth or normal
          // discontinuity would drag a neighbouring surface's lighting in.
          let okDepth = abs(pnd.w - h.t) < 0.12 * max(h.t, 1.0);
          let okNormal = dot(pnd.xyz, h.n) > 0.9;
          if (okDepth && okNormal) {
            var prev = reservoirs[prevBase + u32(pp.y) * dims.x + u32(pp.x)];
            // No index guard needed: reservoirs only ever hold steady lights
            // now, and the steady range never shrinks mid-session. Transients
            // used to be reusable, which meant a reservoir could outlive its
            // own light.
            prev.M = min(prev.M, U.restirMCap);
            mergeReservoir(&res, prev, h.p, h.n, v, m, fVis);
            // Same shading point (reprojected): its domain is ours, so it
            // votes — unless its own shadow ray killed it.
            if (!(prev.W <= 0.0 && prev.targetPdf > 0.0)) { zSame = zSame + prev.M; }
          }
        }
        // Spatial taps: the same merge against the previous frame's reservoirs
        // at jittered offsets. This is where a pixel that just lost its history
        // (disocclusion, light sweep) re-converges from, and it is what lets a
        // good sample found by one pixel spread sideways instead of only
        // forwards in time. Neighbour M is capped at half the temporal cap:
        // neighbours are evidence about a *different* shading point, and
        // letting them outvote the pixel's own stream both slows response and
        // deepens the energy-dilution bias this merge scheme carries.
        //
        // The merged result is used for SHADING ONLY — see `carry` below for
        // what gets stored, and why storing this instead blows up.
        carry = res;
        for (var t = 0u; t < spatialTaps; t = t + 1u) {
          let off = vec2i((rand2() * 2.0 - 1.0) * U.restirSpatialRadius);
          let qp = pp + off;
          if (qp.x < 0 || qp.y < 0 || qp.x >= i32(dims.x) || qp.y >= i32(dims.y)) {
            continue;
          }
          let qnd = textureLoad(prevNormalDepth, qp, 0);
          if (abs(qnd.w - h.t) > 0.12 * max(h.t, 1.0)) { continue; }
          if (dot(qnd.xyz, h.n) < 0.9) { continue; }
          var prev = reservoirs[prevBase + u32(qp.y) * dims.x + u32(qp.x)];
          prev.M = min(prev.M, U.restirMCap * 0.5);
          mergeReservoir(&res, prev, h.p, h.n, v, m, fVis);
          // Its vote in the denominator waits for the final sample.
          if (nCtx < 8u && prev.M > 0.0 && !(prev.W <= 0.0 && prev.targetPdf > 0.0)) {
            ctxPos[nCtx] = pixelWorldPos(qp, qnd.w);
            ctxNrm[nCtx] = qnd.xyz;
            ctxM[nCtx] = prev.M;
            nCtx = nCtx + 1u;
          }
        }
      }
    }
  }

  finalizeReservoir(&res);
  finalizeReservoir(&carry);
  // Re-weight the SHADING stream by the support-aware denominator Z. carry
  // keeps the plain finalize: it is temporal-only and the stored M is a
  // stream's history, not a shading weight.
  if (res.targetPdf > 0.0) {
    var z = mOwn + zSame;
    for (var i = 0u; i < nCtx; i = i + 1u) {
      if (diSupports(ctxPos[i], ctxNrm[i], res)) { z = z + ctxM[i]; }
    }
    if (z > 0.0) { res.W = res.wSum / (z * res.targetPdf); }
  }

  // ---- visibility ------------------------------------------------------
  var contrib = vec3f(0.0);
  if (res.W > 0.0) {
    let delta = res.samplePos - h.p;
    let d2 = max(dot(delta, delta), 1e-4);
    let dist = sqrt(d2);
    let dir = delta / dist;
    countWork(CT_shadowDirect);
    if (occluded(h.p + h.n * EPS * 4.0, dir, dist - EPS * 8.0)) {
      res.W = 0.0;
      res.wSum = 0.0;
      // The visibility verdict transfers to the stored stream only when it is
      // about the same sample. When a neighbour's sample won the shading draw
      // the carry survivor went untested this frame — leave it; it was tested
      // the last frame it won.
      if (carry.lightIdx == res.lightIdx
          && all(carry.samplePos == res.samplePos)) {
        carry.W = 0.0;
        carry.wSum = 0.0;
      }
    } else {
      let rad = radianceFromLight(res.lightIdx, h.p, res.samplePos);
      contrib = evalBSDF(m, h.n, v, dir) * rad * res.W;
    }
  }

  reservoirs[resBase(dims, U.parity) + pixel.y * dims.x + pixel.x] = carry;
  return contrib;
}

/**
 * GI support test for the merge denominator: could the domain shaded at
 * (x, n) have produced sample point `r.samplePos`? The cosine lobe is the
 * only proposal (materials are diffuse), so it is a hemisphere test.
 */
fn giSupports(x: vec3f, n: vec3f, r: GIReservoir) -> bool {
  let delta = r.samplePos - x;
  let d2 = dot(delta, delta);
  if (d2 <= 1e-8) { return true; }
  return dot(n, delta * inverseSqrt(d2)) > 0.0;
}

/**
 * Shift-and-merge one previous-frame GI reservoir into `res`.
 *
 * The shift keeps x2 fixed and rebuilds the connection from our visible point;
 * the Jacobian accounts for the change of measure. Shared by the temporal
 * merge and the spatial taps, which differ only in which pixel the reservoir
 * came from and how much M it is allowed to carry.
 */
fn giMergePrev(
  res: ptr<function, GIReservoir>, prevIn: GIReservoir, mCap: f32,
  h: Hit, v: vec3f, m: Material,
) {
  var prev = prevIn;
  // A dead stream carries no weight; whether its M still votes in the
  // denominator is a domain-support question restirGI answers against the
  // FINAL sample (its Z re-weight) — this only accumulates weight.
  if (prev.M <= 0.0 || prev.W <= 0.0) { return; }
  prev.M = min(prev.M, mCap);
  let delta = prev.samplePos - h.p;
  let d2 = dot(delta, delta);
  if (d2 <= 1e-6) { return; }
  let dir = delta * inverseSqrt(d2);
  let j = giJacobian(prev, h.p);
  let tp = giTarget(m, h.n, v, dir, prev.radiance);
  giUpdate(
    res, prev.samplePos, prev.sampleNrm, prev.radiance, h.p,
    tp * prev.W * prev.M * j, tp, prev.M,
  );
}

/**
 * ReSTIR GI: resample the indirect bounce against last frame's reservoir.
 *
 * `bounceWeight` is f_s*cos/pdf for the freshly traced x1->x2 bounce, so
 * luminance(bounceWeight * L_o) is exactly the RIS weight target/sourcePdf
 * without ever needing the pdf itself.
 *
 * Returns the indirect contribution at x1. With reuse disabled this reduces
 * algebraically to bounceWeight * L_o, i.e. exactly the plain path-traced
 * estimate — which is the invariant worth checking first if it ever looks off.
 */
fn restirGI(
  h: Hit, v: vec3f, m: Material,
  samplePos: vec3f, sampleNrm: vec3f, rad: vec3f,
  bounceDir: vec3f, bounceWeight: vec3f,
  pixel: vec2u, prevWorld: vec3f, dims: vec2u,
) -> vec3f {
  var res = emptyGIReservoir();

  let freshTarget = giTarget(m, h.n, v, bounceDir, rad);
  giUpdate(
    &res, samplePos, sampleNrm, rad, h.p,
    luminance(bounceWeight * rad), freshTarget, 1.0,
  );

  let prevBase = resBase(dims, 1u - U.parity);
  let spatialTaps = u32(U.restirSpatialTaps);
  // Support-aware denominator, exactly as restirDirect: our fresh sample is
  // one candidate; each merged stream votes iff its own shading point could
  // have generated the surviving x2, except a stream killed by its own
  // shadow ray never votes.
  let mOwn = res.M;
  var zSame = 0.0;
  var nCtx = 0u;
  var ctxPos : array<vec3f, 8>;
  var ctxNrm : array<vec3f, 8>;
  var ctxM   : array<f32, 8>;
  // Same store/shade split as restirDirect: the spatially-merged reservoir
  // shades this frame, the temporal-only stream is what next frame reuses.
  var carry = res;
  if (U.restirTemporal > 0.5 || spatialTaps > 0u) {
    let clip = U.prevViewProj * vec4f(prevWorld, 1.0);
    if (clip.w > 0.0) {
      let ndc = clip.xy / clip.w;
      let uv = vec2f(ndc.x * 0.5 + 0.5, 0.5 - ndc.y * 0.5);
      if (uv.x >= 0.0 && uv.x < 1.0 && uv.y >= 0.0 && uv.y < 1.0) {
        // See restirDirect: uv*dims can round up to dims.
        let pp = min(vec2i(uv * vec2f(dims)), vec2i(dims) - 1);
        if (U.restirTemporal > 0.5) {
          let pnd = textureLoad(prevNormalDepth, pp, 0);
          let okDepth = abs(pnd.w - h.t) < 0.12 * max(h.t, 1.0);
          let okNormal = dot(pnd.xyz, h.n) > 0.9;
          if (okDepth && okNormal) {
            let prev = giReservoirs[prevBase + u32(pp.y) * dims.x + u32(pp.x)];
            giMergePrev(&res, prev, U.restirMCap, h, v, m);
            if (!(prev.W <= 0.0 && prev.targetPdf > 0.0)) {
              zSame = zSame + min(prev.M, U.restirMCap);
            }
          }
        }
        carry = res;
        // Spatial taps, same rationale as restirDirect. The reconnection shift
        // was built for exactly this: a neighbour's x2 is as reusable as our
        // own past x2, and the Jacobian already prices the move.
        for (var t = 0u; t < spatialTaps; t = t + 1u) {
          let off = vec2i((rand2() * 2.0 - 1.0) * U.restirSpatialRadius);
          let qp = pp + off;
          if (qp.x < 0 || qp.y < 0 || qp.x >= i32(dims.x) || qp.y >= i32(dims.y)) {
            continue;
          }
          let qnd = textureLoad(prevNormalDepth, qp, 0);
          if (abs(qnd.w - h.t) > 0.12 * max(h.t, 1.0)) { continue; }
          if (dot(qnd.xyz, h.n) < 0.9) { continue; }
          let prev = giReservoirs[prevBase + u32(qp.y) * dims.x + u32(qp.x)];
          giMergePrev(&res, prev, U.restirMCap * 0.5, h, v, m);
          if (nCtx < 8u && prev.M > 0.0 && !(prev.W <= 0.0 && prev.targetPdf > 0.0)) {
            ctxPos[nCtx] = pixelWorldPos(qp, qnd.w);
            ctxNrm[nCtx] = qnd.xyz;
            ctxM[nCtx] = min(prev.M, U.restirMCap * 0.5);
            nCtx = nCtx + 1u;
          }
        }
      }
    }
  }

  finalizeGIReservoir(&res);
  finalizeGIReservoir(&carry);
  // Support-aware denominator for the SHADING stream (see restirDirect).
  if (res.targetPdf > 0.0) {
    var z = mOwn + zSame;
    for (var i = 0u; i < nCtx; i = i + 1u) {
      if (giSupports(ctxPos[i], ctxNrm[i], res)) { z = z + ctxM[i]; }
    }
    if (z > 0.0) { res.W = min(res.wSum / (z * res.targetPdf), 32.0); }
  }

  var contrib = vec3f(0.0);
  if (res.W > 0.0) {
    let delta = res.samplePos - h.p;
    let d2 = max(dot(delta, delta), 1e-6);
    let dist = sqrt(d2);
    let dir = delta / dist;
    // The fresh candidate is visible by construction, but a reused one was
    // traced from a different point and may now be behind something.
    countWork(CT_shadowGI);
    if (occluded(h.p + h.n * EPS * 4.0, dir, dist - EPS * 8.0)) {
      res.W = 0.0;
      res.wSum = 0.0;
      // Same transfer rule as restirDirect: the verdict is only about this
      // sample, so it only reaches the stored stream if that holds it too.
      if (all(carry.samplePos == res.samplePos)) {
        carry.W = 0.0;
        carry.wSum = 0.0;
      }
    } else {
      contrib = evalBSDF(m, h.n, v, dir) * res.radiance * res.W;
    }
  }

  giReservoirs[resBase(dims, U.parity) + pixel.y * dims.x + pixel.x] = carry;
  return contrib;
}

// ---------------------------------------------------------------------------
// patchRIS: the solved patches as an emitter cloud.
//
// Every patch is a Lambertian light of radiance B/pi. RIS proposes M patches
// in proportion to luminance(B)*area (row-3 CDF), jitters a point on each,
// keeps one by its unshadowed contribution here, and spends ONE shadow ray
// on the survivor — the same shape as the direct-light reservoir, aimed at
// bounce light. It answers indirect at any primary hit, dynamic included: a
// character standing in a lit room is finally lit by the room.
// ---------------------------------------------------------------------------

/** Smallest index whose inclusive CDF entry reaches `u`. */
fn patchCdfIndex(u: f32, count: u32) -> u32 {
  var lo = 0u;
  var hi = count - 1u;
  while (lo < hi) {
    countWork(CT_patchCdfTaps);
    let mid = (lo + hi) / 2u;
    let c = textureLoad(radGSky, vec2i(i32(mid), 3), 0).x;
    if (u <= c) { hi = mid; } else { lo = mid + 1u; }
  }
  return lo;
}

fn samplePatchRIS(h: Hit, v: vec3f, m: Material) -> vec3f {
  let count = U.radPatchCount;
  if (count == 0u) { return vec3f(0.0); }
  countWork(CT_patchCdfTaps);
  let total = textureLoad(radGSky, vec2i(i32(count) - 1, 3), 0).x;
  if (total <= 0.0) { return vec3f(0.0); }

  const M: u32 = 8u;
  var chosenDir = vec3f(0.0, 1.0, 0.0);
  var chosenDist = 0.0;
  var chosenRad = vec3f(0.0);
  var chosenTarget = 0.0;
  var wsum = 0.0;

  for (var i = 0u; i < M; i = i + 1u) {
    countWork(CT_risCandidatesPatch);
    let u = rand() * total;
    let j = patchCdfIndex(u, count);
    // Selection probability of patch j from its own stored weight (row 2's
    // .w lane), not a CDF difference: the scan's chunk seams are not exactly
    // monotone in f32, and a rounding-negative difference floored to ~0 would
    // hand this candidate an unbounded 1/p — a firefly, not a probability.
    // A zero-weight patch is reachable only through such rounding; skip it.
    countWork(CT_radiosityGathers);
    let bw = textureLoad(radGSky, vec2i(i32(j), 2), 0);
    if (bw.w <= 0.0) { continue; }
    let pj = bw.w / total;

    let pt = radPatchAt(j);
    let r2 = rand2();
    let y = pt.pos
      + pt.tu * ((r2.x * 2.0 - 1.0) * pt.hu)
      + pt.tv * ((r2.y * 2.0 - 1.0) * pt.hv);
    let delta = y - h.p;
    // A receiver in the patch's own plane exchanges nothing along it (this
    // also drops the receiver's own patch, which sits at distance ~0).
    if (abs(dot(delta, pt.n)) < 1e-3) { continue; }
    let d2raw = max(dot(delta, delta), 1e-4);
    let dist = sqrt(d2raw);
    let dir = delta / dist;
    let cosR = dot(h.n, dir);
    let cosJ = dot(pt.n, -dir);
    if (cosR <= 0.0 || cosJ <= 0.0) { continue; }
    // Point-equivalent VPL: the patch's outgoing radiosity B leaves as
    // radiance B/pi; folded through cos_j*A_j/(pi*d^2) it is a point emitter.
    // No near-field softening: y is jittered over the whole cell, so this
    // point form is an unbiased estimate of the exact point-to-patch transfer
    // at ANY distance (checked numerically: the jittered bare kernel matches
    // the finite-square form factor down to 5 cm, where an A/4 guard reads
    // 38% low and a full-disk A guard 60% low). The 1e-4 d^2 floor above only
    // caps a coincident sample; the luminance firefly clamp bounds the rest.
    let d2 = d2raw;
    let Bj = bw.xyz;
    let rad = Bj * (INV_PI * cosJ * pt.area / d2);
    let tgt = risTarget(m, h.n, dir, rad);
    let w = tgt / pj;
    wsum = wsum + w;
    if (w > 0.0 && rand() * wsum < w) {
      chosenDir = dir;
      chosenDist = dist;
      chosenRad = rad;
      chosenTarget = tgt;
    }
  }

  if (wsum <= 0.0 || chosenTarget <= 0.0) { return vec3f(0.0); }
  // RIS estimator weight. Deliberately NOT capped like the light
  // reservoirs: their source pdf is ~1/lightCount so W near 30 is the
  // ceiling of normal, but a patch is one of thousands (p ~ 1e-3..1e-4) and W
  // in the hundreds is this estimator's ordinary operating point — the same
  // cap here measured a -0.16 relBias (energy chopped from nearly every
  // survivor). The luminance firefly clamp on `indirect` still bounds the
  // stored value; that clamp trades a little bias for stability everywhere.
  let W = (wsum / f32(M)) / chosenTarget;
  let contrib = evalBSDF(m, h.n, v, chosenDir) * chosenRad * W;
  // No SHADOW_CULL here: this IS the pixel's whole indirect estimate, not
  // one light among many, and bounce light in this scene routinely sits
  // near the cull line — culling it truncates the dim end into a bias
  // (measured −0.16 relBias before this was removed).
  countWork(CT_shadowPatch);
  if (occluded(h.p + h.n * EPS * 4.0, chosenDir, chosenDist - EPS * 16.0)) {
    return vec3f(0.0);
  }
  return contrib;
}

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) gid: vec3u) {
  let dims = vec2u(U.resolution);
  if (gid.x >= dims.x || gid.y >= dims.y) { return; }
  let pixel = gid.xy;
  seedRng(pixel, U.frame);

  // Sub-pixel jitter: over many accumulated frames this is what gives us
  // antialiasing for free.
  let jitter = rand2() - 0.5;
  let uv = (vec2f(pixel) + 0.5 + jitter) * U.invResolution;
  let rd = cameraRay(uv);
  let ro = U.camPos;

  countWork(CT_raysDepth0);
  let primary = trace(ro, rd, RAY_MAX, true, false);

  // ---- G-buffer ----------------------------------------------------------
  var demod = vec3f(1.0);
  var worldPos = ro + rd * 1e4;
  var normal = -rd;
  var depth = 1e4;

  if (primary.valid) {
    countWork(CT_primaryHits);
    let m = materials[primary.mat];
    worldPos = primary.p;

    // Carry hits on animated geometry back to where that surface point was last
    // frame, so reprojection finds it. The box rotation is orthonormal, so
    // world->local is three dot products.
    if (primary.dynIdx != DYN_NONE) {
      let cur = dynBoxes[primary.dynIdx];
      let prv = prevDynBoxes[primary.dynIdx];
      let d = primary.p - cur.center;
      let local = vec3f(dot(d, cur.rot0), dot(d, cur.rot1), dot(d, cur.rot2));
      worldPos = prv.center
        + local.x * prv.rot0 + local.y * prv.rot1 + local.z * prv.rot2;
    }
    normal = primary.n;
    depth = primary.t;
    demod = max(mix(m.albedo, vec3f(1.0), m.metallic), vec3f(0.02));
    // Emitters carry their radiance directly; dividing by a near-black albedo
    // would blow the demodulated signal up.
    if (dot(m.emissive, vec3f(1.0)) > 0.0) { demod = vec3f(1.0); }
  }

  textureStore(gAlbedo, pixel, vec4f(demod, 1.0));
  textureStore(gNormalDepth, pixel, vec4f(normal, depth));
  // Note this holds the PREVIOUS frame's world position of the visible point,
  // which is exactly what reprojection needs. w tags the surface class:
  // 0 = miss, 1 = static, 2 = dynamic. Reprojection validates the two
  // differently: a dynamic hit already came through its box's exact rigid
  // transform, so its previous ray depth is known and identifies it, while
  // the character's own overlapping, rotating limb boxes would fail a normal
  // test against themselves.
  let posTag = select(0.0, select(1.0, 2.0, primary.dynIdx != DYN_NONE), primary.valid);
  textureStore(gPos, pixel, vec4f(worldPos, posTag));

  // ---- radiance ----------------------------------------------------------
  var radiance = vec3f(0.0);
  let spp = max(U.spp, 1u);

  // Checkerboard the indirect bounces.
  //
  // Measured breakdown at 1152x720/2 bounces: primary + direct is 3.6ms while
  // the indirect bounces are 10.9ms, i.e. 72% of the trace. Indirect light is
  // low-frequency and already goes through temporal accumulation plus four
  // a-trous iterations, so tracing it for half the pixels per frame and
  // alternating the parity costs far less image quality than it saves time.
  // The surviving samples are weighted by 1/rate to stay unbiased.
  // The alternation is per WORKGROUP TILE, not per pixel. A per-pixel
  // checkerboard measured only 13% faster and got no faster at all when
  // dropped to a quarter of pixels, because adjacent pixels land in the same
  // SIMD wavefront: with any lane active the whole wavefront still walks the
  // indirect path and the idle lanes save nothing. Alternating whole 8x8 tiles
  // lets entire wavefronts skip the work.
  let rate = clamp(U.indirectRate, 0.05, 1.0);
  let tile = (pixel.x >> 3u) + (pixel.y >> 3u);
  let period = max(1u, u32(round(1.0 / rate)));
  let traceIndirect = rate >= 0.999 || ((tile + U.frame) % period) == 0u;
  // f32(period), not 1/rate: period is what the tile test actually uses, and
  // rounding makes them disagree (rate 0.75 -> period 1, i.e. 33% too bright).
  let indirectWeight = select(0.0, f32(period), traceIndirect);

  // Static hits in radiosityRead mode take their steady indirect from the
  // patch solve; every steady-bounce accumulation below is gated off for
  // them, and bounce rays only fire at all while a transient light needs its
  // bounce traced. Dynamic geometry has no patches and keeps the traced path.
  let staticPrimary = primary.valid && primary.dynIdx == DYN_NONE && primary.boxIdx != BOX_NONE;
  // A static face below the patch builder's area floor has no grid data, so
  // it is not a radiosityRead pixel at all: it traces like the traced mode
  // instead of skipping its bounce and then reporting black as "valid".
  let patchMode = U.indirectMode == IMODE_RADIOSITY_READ || U.indirectMode == IMODE_PATCH_RIS;
  var primaryGrid : RadGrid;
  primaryGrid.base = BOX_NONE;
  if (staticPrimary && patchMode) { primaryGrid = radFaceGrid(primary); }
  let primaryPatched = primaryGrid.base != BOX_NONE;
  let radioStatic = U.indirectMode == IMODE_RADIOSITY_READ && primaryPatched;
  // Gather: trace bounce 1 for real, then read the solve at x1 instead of
  // tracing on — the character in the beam now shadows the floor's bounce,
  // and the solve is only ever consumed one bounce away from the eye. Only x1
  // has to be static and patched; the primary hit can be a character, which
  // is how a body in the room picks up the room's bounce.
  let gatherMode = U.indirectMode == IMODE_GATHER && primary.valid;
  // patchRIS: indirect at the primary vertex from the patches as emitters,
  // for every valid primary hit — dynamic geometry included.
  let patchRISMode = U.indirectMode == IMODE_PATCH_RIS && primary.valid && U.radPatchCount > 0u;
  // Both patch-based modes replace the traced steady bounce at the primary
  // vertex; the loop below traces on only while a transient needs it.
  let skipTracedIndirect = radioStatic || patchRISMode;
  let transientsLive = U.lightCount > U.transientStart;

  // Direct and indirect are kept apart so the firefly clamp can be applied only
  // to the indirect term. Direct lighting is shadow-tested and importance
  // sampled, so it is well behaved and genuinely reaches high values inside the
  // beam; clamping it would visibly crush the flashlight.
  var indirect = vec3f(0.0);
  var transient = vec3f(0.0);
  /**
   * The part of `transient` that arrived via a bounce.
   *
   * Tracked only to decide how hard to filter. Bounce light from a flash gets
   * one shadow ray per bounce, so its visibility term is a hard 0/1 sampled
   * once — that, not any throughput spike, is where the far-field sparkle
   * comes from (these materials are diffuse, so throughput cannot exceed 1).
   * It is also the smoothest part of the answer, so it is what can be blurred
   * hardest without losing anything real.
   */
  var transientBounce = vec3f(0.0);

  // ---- ReSTIR GI candidate ------------------------------------------------
  // The first bounce hit is the sample point. Its outgoing radiance is
  // accumulated on a throughput that resets to 1 there, so what lands in the
  // reservoir is radiance *at x2*, independent of the BSDF at x1 — which is
  // exactly what makes it reusable by a different pixel.
  var giValid = false;
  var giPos = vec3f(0.0);
  var giNrm = vec3f(0.0, 1.0, 0.0);
  var giRad = vec3f(0.0);
  var giDir = vec3f(0.0, 1.0, 0.0);
  /** f_s*cos/pdf of the x1->x2 bounce; luminance(this * L_o) is the RIS weight. */
  var giBounceWeight = vec3f(0.0);

  for (var s = 0u; s < spp; s = s + 1u) {
    var throughput = vec3f(1.0);
    var giThroughput = vec3f(1.0);
    var rayO = ro;
    var rayD = rd;
    var h = primary;
    var specular = true;

    for (var b = 0u; b <= U.bounces; b = b + 1u) {
      if (b > 0u) {
        countWork(CT_raysDepth0 + min(b, 6u));
        h = trace(rayO, rayD, RAY_MAX, false, false);
      }
      if (!h.valid) {
        let sky = throughput * skyRadiance(rayD);
        if (b == 0u) {
          radiance = radiance + sky;
        } else if (!skipTracedIndirect) {
          // A ray that escapes has no surface to reconnect to, so it cannot
          // become a GI reservoir sample. Bank it directly instead.
          indirect = indirect + sky;
          giRad = giRad + giThroughput * skyRadiance(rayD);
        }
        break;
      }
      if (b == 1u && !skipTracedIndirect) {
        giValid = true;
        giPos = h.p;
        giNrm = h.n;
      }

      let m = materials[h.mat];
      let v = -rayD;

      // Emission is added on camera rays and after specular bounces only;
      // everything else arrives via NEE, so adding it here would double count.
      if (b == 0u || specular) {
        let e = throughput * m.emissive;
        if (b == 0u) {
          radiance = radiance + e;
        } else if (!skipTracedIndirect) {
          indirect = indirect + e;
          giRad = giRad + giThroughput * m.emissive;
        }
      }

      // Direct lighting goes through ReSTIR: one unified reservoir over every
      // light, reused across frames, costing a single shadow ray.
      // Transient lights at every bounce, so a flash still throws light onto
      // the ceiling and back off the far wall — the bounce is most of why this
      // is worth tracing rather than compositing a sprite.
      if (U.lightCount > U.transientStart) {
        // Full sample count on the primary hit only. Bounce light from a flash
        // is low frequency and gets filtered hard anyway, so extra rays there
        // buy nothing you can see and multiply the cost by the bounce count.
        let ts = select(1u, u32(U.transientSamples), b == 0u);
        let tc = throughput * sampleTransientLights(h.p, h.n, v, m, ts);
        transient = transient + tc;
        if (b > 0u) { transientBounce = transientBounce + tc; }
      }

      if (b == 0u) {
        radiance = radiance + restirDirect(h, v, m, pixel, worldPos, dims);
        // patchRIS: the whole steady indirect estimate at x0 — patches as
        // emitters plus x0's own sky irradiance (which the patches cannot
        // supply: they cover surface-to-surface transfer only). Dynamic hits
        // have no sky row and go without it.
        if (patchRISMode) {
          indirect = indirect + samplePatchRIS(h, v, m);
          if (primaryPatched) { indirect = indirect + m.albedo * radiositySky(h); }
        }
      } else if (!skipTracedIndirect) {
        // Deeper bounces resample across every light at once rather than
        // picking a channel and scaling up — see sampleIndirectRIS.
        let li = sampleIndirectRIS(h.p, h.n, v, m, flashTargetVis(h.p));
        indirect = indirect + throughput * li;
        giRad = giRad + giThroughput * li;
      }

      // Gather at x1: the solve's incident irradiance re-radiated by x1's
      // albedo stands in for everything a longer path would have fetched
      // (the traced x1 above still owes it this closure at any bounce cap).
      // Dynamic x1 or a face below the patch floor keeps the traced path.
      if (gatherMode && b == 1u && h.dynIdx == DYN_NONE && h.boxIdx != BOX_NONE) {
        let rg = radiosityIndirect(h);
        if (rg.x >= 0.0) {
          let reradiated = m.albedo * rg;
          indirect = indirect + throughput * reradiated;
          giRad = giRad + giThroughput * reradiated;
          break;
        }
      }

      if (b == U.bounces) { break; }
      // Pixels sitting out this frame stop after direct lighting; so do
      // patch-fed pixels unless a live flash still needs its bounce traced.
      if (b == 0u && !traceIndirect) { break; }
      if (b == 0u && skipTracedIndirect && !transientsLive) { break; }

      let bs = sampleBSDF(m, h.n, v);
      if (dot(bs.weight, bs.weight) <= 1e-8) { break; }
      if (b == 0u) {
        giBounceWeight = bs.weight;
        giDir = bs.dir;
      } else {
        giThroughput = giThroughput * bs.weight;
      }
      throughput = throughput * bs.weight;
      specular = bs.specular;
      rayO = h.p + h.n * EPS * 4.0;
      rayD = bs.dir;

      // Russian roulette once the path has had a chance to pick up energy.
      if (b >= 1u) {
        let q = clamp(max(throughput.r, max(throughput.g, throughput.b)), 0.05, 1.0);
        if (rand() > q) { break; }
        throughput = throughput / q;
      }
    }
  }

  // ReSTIR GI replaces the whole indirect term when there is a sample point to
  // reconnect to. Rays that escaped to the sky have no x2, so those keep the
  // plain accumulated estimate.
  if (U.restirGI > 0.5 && giValid && dot(giBounceWeight, giBounceWeight) > 0.0) {
    indirect = restirGI(
      primary, -rd, materials[primary.mat],
      giPos, giNrm, giRad, giDir, giBounceWeight,
      pixel, worldPos, dims,
    );
  } else {
    giReservoirs[resBase(dims, U.parity) + pixel.y * dims.x + pixel.x] = emptyGIReservoir();
  }

  // Clamp indirect fireflies before averaging. A single unlucky path would
  // otherwise dominate the temporal history for dozens of frames, and the
  // a-trous luminance weight actively protects lone bright pixels from being
  // filtered away rather than removing them.
  let il = luminance(indirect);
  if (il > 3.0) { indirect = indirect * (3.0 / il); }

  radiance = radiance / f32(spp);
  // The checkerboard weight compensates the pixels that skipped their
  // traced bounce; patch-fed indirect is computed every frame regardless of
  // the tile schedule, so it takes no weight.
  indirect = indirect * select(indirectWeight, 1.0, skipTracedIndirect) / f32(spp);
  transient = transient / f32(spp);

  let vol = volumetricBeams(ro, rd, depth);
  var illum = radiance / demod;
  var illumIndirect = indirect / demod;
  let illumTransient = transient / demod;

  // Radiosity pixels replace whatever the (skipped) bounce loop produced.
  // The lookup already returns demodulated radiance, so it lands after the
  // albedo division; the firefly clamp and checkerboard weighting above are
  // Monte-Carlo medicine a deterministic solve does not need. radioStatic
  // already guarantees the face has patch data (the -1 sentinel cannot fire).
  if (radioStatic) { illumIndirect = radBilinear(primaryGrid, false) * INV_PI; }

  // A small ambient floor. Physically the sealed ceiling means an unlit corner
  // really is black, but a stealth game still has to be playable: this lifts
  // shadowed geometry just enough to read silhouettes without touching the
  // contrast where the beam actually falls. Demodulated, so it tints by albedo.
  if (primary.valid) { illum = illum + vec3f(U.ambient); }

  // The raw luminance moments are derivable from these, so reproject computes
  // them itself rather than us burning extra render targets on them.
  textureStore(illumOut, pixel, ILLUM_DIRECT, vec4f(illum, 0.0));
  // In-scattered radiance is not demodulated: it never touched a surface.
  textureStore(illumOut, pixel, ILLUM_VOLUME, vec4f(vol.inscatter, vol.transmittance));
  // Alpha is the validity flag, so a checkerboard pixel that sat this frame out
  // is distinguishable from one that genuinely received no bounce light.
  // Patch-fed pixels (radiosityRead, patchRIS) are always valid — the solve
  // ran and was read whether or not this tile was tracing bounces. Gather
  // pixels are exactly as valid as their traced bounce: they follow
  // traceIndirect like the traced mode does.
  textureStore(illumOut, pixel, ILLUM_INDIRECT,
    vec4f(illumIndirect, select(0.0, 1.0, traceIndirect || skipTracedIndirect)));
  // Alpha carries how hard this pixel wants to be filtered, 0..1. What reads
  // it depends on U.transientFilter; see the transient chain in renderer.ts.
  //
  // Two terms, whichever is larger. Distance, because the flash falls off as
  // the inverse square while its sampling noise does not: across a frame the
  // median transient luminance drops ~675x from the muzzle to the far corner
  // while the 99.9th percentile drops only ~144x, so the far field is dark
  // ground with isolated bright pixels on it. And bounce fraction, because
  // once-bounced flash light is both the noisiest part of the estimate and the
  // lowest frequency part of the answer.
  let tLum = luminance(transient);
  let bounceFrac = select(0.0, luminance(transientBounce) / tLum, tLum > 1e-6);
  let distBlur = clamp(nearestTransientDist(worldPos) / max(U.transientBlurDist, 0.5), 0.0, 1.0);
  let transBlur = clamp(max(distBlur, bounceFrac * U.transientBounceWeight), 0.0, 1.0);
  textureStore(illumOut, pixel, ILLUM_TRANSIENT, vec4f(illumTransient, transBlur));

  // Work counters: one atomic per non-zero slot per invocation. The
  // contention this creates is why counters-ON timings mean nothing.
  if (U.countersOn > 0.5) {
    for (var i = 0u; i < CT_COUNT; i = i + 1u) {
      if (counterTally[i] > 0u) { atomicAdd(&counters[i], counterTally[i]); }
    }
  }
}
