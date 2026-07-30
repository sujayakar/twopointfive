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
 * Radiance output: one array texture, three layers, one storage-texture slot
 * (which keeps this pass inside WebGPU's default budget of 4 per stage).
 *
 * The three signals are separate layers, never summed, because they want
 * incompatible denoising. Direct carries hard shadow edges that must survive
 * filtering; indirect is low frequency and wants blurring hard — one filter
 * cannot serve both. Transient (muzzle flash) light wants no temporal history
 * at all: a long history leaves a glow hanging after the flash is gone, and a
 * short one throws away the whole screen's convergence, so that layer is
 * simply not accumulated and appears/vanishes with the light by construction.
 */
@group(1) @binding(3) var illumOut : texture_storage_2d_array<rgba16float, write>;
const ILLUM_DIRECT : u32 = 0u;
const ILLUM_INDIRECT : u32 = 1u;
const ILLUM_TRANSIENT : u32 = 2u;
@group(1) @binding(4) var prevNormalDepth : texture_2d<f32>;
@group(1) @binding(5) var<storage, read> reservoirPrev : array<Reservoir>;
@group(1) @binding(6) var<storage, read_write> reservoirCur : array<Reservoir>;
@group(1) @binding(8) var<storage, read> giPrev : array<GIReservoir>;
@group(1) @binding(9) var<storage, read_write> giCur : array<GIReservoir>;
/** Torch depth maps, traced by flashmap.wgsl earlier in the frame. */
@group(1) @binding(11) var flashDepth : texture_2d_array<f32>;
/**
 * Radiosity per-patch data as a texture (the storage-buffer budget is full):
 * texel (i, 0) = gathered indirect irradiance G, texel (i, 1) = sky-dome
 * irradiance per unit sky intensity.
 */
@group(1) @binding(12) var radGSky : texture_2d<f32>;
/** Radiosity face table: texel (boxIdx*6+face, 0) = {patchBase, gridW, gridH}. */
@group(1) @binding(13) var radFaces : texture_2d<u32>;

/** G + sky, combined, for one patch index. */
fn radPatchIrradiance(i: u32) -> vec3f {
  let c = vec2i(i32(i), 0);
  return textureLoad(radGSky, c, 0).xyz
    + textureLoad(radGSky, c + vec2i(0, 1), 0).xyz * U.skyIntensity;
}

/**
 * Indirect illumination at a static hit, from the radiosity patches.
 *
 * Returns DEMODULATED radiance (incident indirect irradiance / pi — the same
 * quantity the traced indirect stores after albedo division), or x = -1 for
 * faces below the patch builder's area floor, which the caller treats as
 * "no data". Bilinear over the face's patch grid; the face and cell
 * conventions must match src/scene/radiosity.ts exactly.
 */
fn radiosityIndirect(h: Hit) -> vec3f {
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
  if (ft.x == BOX_NONE) { return vec3f(-1.0, 0.0, 0.0); }
  let gw = f32(ft.y);
  let gh = f32(ft.z);
  let fu = clamp((u01 * 0.5 + 0.5) * gw - 0.5, 0.0, gw - 1.0);
  let fv = clamp((v01 * 0.5 + 0.5) * gh - 0.5, 0.0, gh - 1.0);
  let u0 = u32(floor(fu));
  let v0 = u32(floor(fv));
  let u1 = min(u0 + 1u, ft.y - 1u);
  let v1 = min(v0 + 1u, ft.z - 1u);
  let du = fract(fu);
  let dv = fract(fv);
  let g = mix(
    mix(radPatchIrradiance(ft.x + v0 * ft.y + u0), radPatchIrradiance(ft.x + v0 * ft.y + u1), du),
    mix(radPatchIrradiance(ft.x + v1 * ft.y + u0), radPatchIrradiance(ft.x + v1 * ft.y + u1), du),
    dv,
  );
  return g * INV_PI;
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
// Participating medium: animated fog + smoke puffs.
// ---------------------------------------------------------------------------

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

/**
 * Local density of the medium: drifting fog noise around mean 1, plus any
 * smoke puffs covering the point. Multiplies the in-scatter of every light at
 * this march step. In-scatter only — no transmittance, no self-shadowing: at
 * puff scale the eye reads drift and fade, not transport, and the march is
 * far too sparse to integrate optical depth without banding.
 */
fn mediumDensity(p: vec3f) -> f32 {
  var d = 1.0;
  if (U.fogAmount > 0.0) {
    let w = U.time * U.fogSpeed;
    // Two octaves; the second drifts against the first so the fog churns
    // rather than sliding as one sheet.
    let n = valueNoise(p * 0.85 + vec3f(w, w * 0.31, -w * 0.62)) * 0.7
          + valueNoise(p * 2.1 + vec3f(-w * 0.8, w * 0.47, w * 1.13)) * 0.3;
    // Mean of valueNoise is 0.5; this remaps to mean ~1 so fogAmount changes
    // texture, not exposure.
    d = mix(1.0, clamp(2.0 * n, 0.0, 2.0), U.fogAmount);
  }
  for (var i = 0u; i < MAX_PUFFS; i = i + 1u) {
    let pr = U.puffPosR[i];
    if (pr.w <= 0.0) { continue; }
    let q = (p - pr.xyz) / pr.w;
    let r2 = dot(q, q);
    if (r2 >= 1.0) { continue; }
    let prm = U.puffParams[i];
    // Quadratic falloff to the shell, with the puff's own churn on top.
    let fall = (1.0 - r2) * (1.0 - r2);
    let churn = valueNoise(p * 2.4 + vec3f(prm.z, U.time * 0.45 + prm.z, prm.z * 1.7));
    d = d + prm.x * fall * (0.5 + 1.0 * churn);
  }
  return d;
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
 * In-scattering from the flashlight along the primary ray. The result is a
 * single scalar because all of it comes from one light with one colour, so the
 * tint is applied once at composite time.
 */
/**
 * In-scattering along the camera ray from every torch in the level.
 *
 * One scalar for all beams, and therefore one tint for all of them, even
 * though the player's torch is warm and a guard's is cool. The alternative
 * needs a second filtered channel and there isn't one free: this rides the
 * direct signal's alpha precisely so it gets reprojected and a-trous'd, and
 * the march is jittered hard enough that an unfiltered copy would be nothing
 * but noise. The surfaces still carry each light's true colour, which is where
 * telling whose beam it is actually happens.
 */
const VOL_TORCH_RANGE2: f32 = 14.0 * 14.0;

struct VolumetricResult {
  /** In-scatter from the steady beams; rides the direct signal's alpha. */
  steady : f32,
  /**
   * Coloured in-scatter from transient lights (muzzle flashes, detonations).
   *
   * Routed through the TRANSIENT signal, not the steady scalar, for the same
   * reason surface transients are: the steady alpha is temporally accumulated,
   * and a 3-frame flash pushed through a ~20-frame history would hang in the
   * air as a half-second glow. The transient chain is never accumulated, so
   * the glow lives and dies with the light. Carried as colour because flashes
   * have their own tints and the steady scalar's single shared tint cannot.
   */
  flash  : vec3f,
}

fn volumetricBeams(ro: vec3f, rd: vec3f, tmax: f32) -> VolumetricResult {
  var out: VolumetricResult;
  out.steady = 0.0;
  out.flash = vec3f(0.0);
  if (U.volumetric <= 0.0) { return out; }

  // Step count is the dominant cost of the whole trace, because every step
  // inside the beam fires a shadow ray that is unoccluded by definition and so
  // walks the entire BVH without ever early-outing. The march is jittered and
  // the result goes through temporal accumulation, so far fewer steps than you
  // would need for a single clean frame still resolve.
  let steps = max(2u, u32(U.volSteps));
  let maxDist = min(tmax, 26.0);
  let dt = maxDist / f32(steps);
  let jitter = rand();
  var acc = 0.0;

  for (var i = 0u; i < steps; i = i + 1u) {
    let t = (f32(i) + jitter) * dt;
    if (t >= maxDist) { break; }
    let p = ro + rd * t;

    // The medium is shared by every light at this step: churn in the fog and
    // smoke from a fresh shot modulate all the beams alike.
    let dens = mediumDensity(p);

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
        } else if (!occluded(p, dir, dist - EPS * 8.0)) {
          vis = 1.0;
        }
        if (vis > 0.0) {
          // dir points from the march point toward the lamp, so light
          // propagates along -dir and the scattered light reaching the camera
          // propagates along -rd. cos(theta) = dot(-dir, -rd) = dot(dir, rd).
          let phase = phaseHG(dot(rd, dir), 0.55);
          out.steady = out.steady + vis * cone * phase / d2 * dt * U.flashIntensity * dens;
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
      } else if (occluded(p, dir, dist - EPS * 8.0)) { continue; }
      let phase = phaseHG(dot(rd, dir), 0.55);
      out.steady = out.steady + vis * cone * phase / d2 * dt * l.intensity * dens;
    }

    // ---- transient lights ---------------------------------------------------
    // A muzzle flash lives ~3 frames, but while it does the smoke it just made
    // should glow from the inside — that pairing is most of why the puffs
    // exist. Isotropic phase: a flash has no beam axis. The loop costs nothing
    // while no flash is live (intensity check, no rays), and a live flash is
    // close and brief.
    for (var li = U.transientStart; li < U.lightCount; li = li + 1u) {
      let l = lights[li];
      if (l.intensity <= 0.0) { continue; }
      let delta = l.pos - p;
      let d2 = max(dot(delta, delta), 0.25);
      // Same range cut as the torches: inverse-square makes the far field
      // cheaper to skip than to march.
      if (d2 > VOL_TORCH_RANGE2) { continue; }
      let dist = sqrt(d2);
      let dir = delta / dist;
      if (occluded(p, dir, dist - EPS * 8.0)) { continue; }
      out.flash = out.flash + l.color * ((1.0 / (4.0 * PI)) / d2 * dt * l.intensity * dens);
    }
  }
  out.steady = out.steady * U.volumetric;
  out.flash = out.flash * U.volumetric;
  return out;
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
  let spatialTaps = u32(U.restirSpatialTaps);
  if (U.restirTemporal > 0.5 || spatialTaps > 0u) {
    let clip = U.prevViewProj * vec4f(prevWorld, 1.0);
    if (clip.w > 0.0) {
      let ndc = clip.xy / clip.w;
      let uv = vec2f(ndc.x * 0.5 + 0.5, 0.5 - ndc.y * 0.5);
      if (uv.x >= 0.0 && uv.x < 1.0 && uv.y >= 0.0 && uv.y < 1.0) {
        let pp = vec2i(uv * vec2f(dims));
        if (U.restirTemporal > 0.5) {
          let pnd = textureLoad(prevNormalDepth, pp, 0);
          // Same surface test as the denoiser: reuse across a depth or normal
          // discontinuity would drag a neighbouring surface's lighting in.
          let okDepth = abs(pnd.w - h.t) < 0.12 * max(h.t, 1.0);
          let okNormal = dot(pnd.xyz, h.n) > 0.9;
          if (okDepth && okNormal) {
            var prev = reservoirPrev[u32(pp.y) * dims.x + u32(pp.x)];
            // No index guard needed: reservoirs only ever hold steady lights
            // now, and the steady range never shrinks mid-session. Transients
            // used to be reusable, which meant a reservoir could outlive its
            // own light.
            prev.M = min(prev.M, U.restirMCap);
            mergeReservoir(&res, prev, h.p, h.n, v, m, fVis);
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
          var prev = reservoirPrev[u32(qp.y) * dims.x + u32(qp.x)];
          prev.M = min(prev.M, U.restirMCap * 0.5);
          mergeReservoir(&res, prev, h.p, h.n, v, m, fVis);
        }
      }
    }
  }

  finalizeReservoir(&res);
  finalizeReservoir(&carry);

  // ---- visibility ------------------------------------------------------
  var contrib = vec3f(0.0);
  if (res.W > 0.0) {
    let delta = res.samplePos - h.p;
    let d2 = max(dot(delta, delta), 1e-4);
    let dist = sqrt(d2);
    let dir = delta / dist;
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

  reservoirCur[pixel.y * dims.x + pixel.x] = carry;
  return contrib;
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

  let spatialTaps = u32(U.restirSpatialTaps);
  // Same store/shade split as restirDirect: the spatially-merged reservoir
  // shades this frame, the temporal-only stream is what next frame reuses.
  var carry = res;
  if (U.restirTemporal > 0.5 || spatialTaps > 0u) {
    let clip = U.prevViewProj * vec4f(prevWorld, 1.0);
    if (clip.w > 0.0) {
      let ndc = clip.xy / clip.w;
      let uv = vec2f(ndc.x * 0.5 + 0.5, 0.5 - ndc.y * 0.5);
      if (uv.x >= 0.0 && uv.x < 1.0 && uv.y >= 0.0 && uv.y < 1.0) {
        let pp = vec2i(uv * vec2f(dims));
        if (U.restirTemporal > 0.5) {
          let pnd = textureLoad(prevNormalDepth, pp, 0);
          let okDepth = abs(pnd.w - h.t) < 0.12 * max(h.t, 1.0);
          let okNormal = dot(pnd.xyz, h.n) > 0.9;
          if (okDepth && okNormal) {
            giMergePrev(&res, giPrev[u32(pp.y) * dims.x + u32(pp.x)], U.restirMCap, h, v, m);
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
          giMergePrev(&res, giPrev[u32(qp.y) * dims.x + u32(qp.x)], U.restirMCap * 0.5, h, v, m);
        }
      }
    }
  }

  finalizeGIReservoir(&res);
  finalizeGIReservoir(&carry);

  var contrib = vec3f(0.0);
  if (res.W > 0.0) {
    let delta = res.samplePos - h.p;
    let d2 = max(dot(delta, delta), 1e-6);
    let dist = sqrt(d2);
    let dir = delta / dist;
    // The fresh candidate is visible by construction, but a reused one was
    // traced from a different point and may now be behind something.
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

  giCur[pixel.y * dims.x + pixel.x] = carry;
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

  let primary = trace(ro, rd, RAY_MAX, true, false);

  // ---- G-buffer ----------------------------------------------------------
  var demod = vec3f(1.0);
  var worldPos = ro + rd * 1e4;
  var normal = -rd;
  var depth = 1e4;

  if (primary.valid) {
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
    // would explode.
    if (dot(m.emissive, vec3f(1.0)) > 0.0) { demod = vec3f(1.0); }
  }

  textureStore(gAlbedo, pixel, vec4f(demod, 1.0));
  textureStore(gNormalDepth, pixel, vec4f(normal, depth));
  // Note this holds the PREVIOUS frame's world position of the visible point,
  // which is exactly what reprojection needs.
  textureStore(gPos, pixel, vec4f(worldPos, select(0.0, 1.0, primary.valid)));

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

  // Static hits with radiosity read their steady indirect from the patch
  // solve; every steady-bounce accumulation below is gated off for them, and
  // bounce rays only fire at all while a transient light needs its bounce
  // traced. Dynamic geometry has no patches and keeps the traced path.
  let radioStatic = U.radiosityOn > 0.5 && primary.valid
    && primary.dynIdx == DYN_NONE && primary.boxIdx != BOX_NONE;
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
        h = trace(rayO, rayD, RAY_MAX, false, false);
      }
      if (!h.valid) {
        let sky = throughput * skyRadiance(rayD);
        if (b == 0u) {
          radiance = radiance + sky;
        } else if (!radioStatic) {
          // A ray that escapes has no surface to reconnect to, so it cannot
          // become a GI reservoir sample. Bank it directly instead.
          indirect = indirect + sky;
          giRad = giRad + giThroughput * skyRadiance(rayD);
        }
        break;
      }
      if (b == 1u && !radioStatic) {
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
        } else if (!radioStatic) {
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
      } else if (!radioStatic) {
        // Deeper bounces resample across every light at once rather than
        // picking a channel and scaling up — see sampleIndirectRIS.
        let li = sampleIndirectRIS(h.p, h.n, v, m, flashTargetVis(h.p));
        indirect = indirect + throughput * li;
        giRad = giRad + giThroughput * li;
      }

      if (b == U.bounces) { break; }
      // Pixels sitting out this frame stop after direct lighting; so do
      // radiosity pixels unless a live flash still needs its bounce traced.
      if (b == 0u && !traceIndirect) { break; }
      if (b == 0u && radioStatic && !transientsLive) { break; }

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
    giCur[pixel.y * dims.x + pixel.x] = emptyGIReservoir();
  }

  // Clamp indirect fireflies before averaging. A single unlucky path would
  // otherwise dominate the temporal history for dozens of frames, and the
  // a-trous luminance weight actively protects lone bright pixels from being
  // filtered away rather than removing them.
  let il = luminance(indirect);
  if (il > 3.0) { indirect = indirect * (3.0 / il); }

  radiance = radiance / f32(spp);
  indirect = indirect * indirectWeight / f32(spp);
  transient = transient / f32(spp);

  let vol = volumetricBeams(ro, rd, depth);
  // Flash in-scatter joins the transient signal BEFORE demodulation: the
  // composite re-multiplies by albedo, so the round trip cancels and the glow
  // arrives at the screen unscaled by whatever surface happens to be behind it
  // (up to the filter mixing neighbours, which is invisible at flash length).
  transient = transient + vol.flash;
  var illum = radiance / demod;
  var illumIndirect = indirect / demod;
  let illumTransient = transient / demod;

  // Radiosity pixels replace whatever the (skipped) bounce loop produced.
  // The lookup already returns demodulated radiance, so it lands after the
  // albedo division; the firefly clamp and checkerboard weighting above are
  // Monte-Carlo medicine a deterministic solve does not need. Faces below the
  // patch builder's area floor return -1 and keep the traced value (black,
  // since tracing was skipped — small trim only).
  if (radioStatic) {
    let rg = radiosityIndirect(primary);
    if (rg.x >= 0.0) { illumIndirect = rg; }
  }

  // A small ambient floor. Physically the sealed ceiling means an unlit corner
  // really is black, but a stealth game still has to be playable: this lifts
  // shadowed geometry just enough to read silhouettes without touching the
  // contrast where the beam actually falls. Demodulated, so it tints by albedo.
  if (primary.valid) { illum = illum + vec3f(U.ambient); }

  // The raw luminance moments are derivable from these, so reproject computes
  // them itself rather than us burning extra render targets on them.
  textureStore(illumOut, pixel, ILLUM_DIRECT, vec4f(illum, vol.steady));
  // Alpha is the validity flag, so a checkerboard pixel that sat this frame out
  // is distinguishable from one that genuinely received no bounce light.
  // Radiosity pixels are always valid — the solve ran whether or not this
  // pixel's tile was tracing bounces this frame.
  textureStore(illumOut, pixel, ILLUM_INDIRECT,
    vec4f(illumIndirect, select(0.0, 1.0, traceIndirect || radioStatic)));
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
}
