// ===========================================================================
// Radiance cascades.
//
// A hierarchy of world-space directional probe volumes. Cascade 0 has dense
// probes, few directions and short rays; each further cascade halves the probe
// density in x/z, quadruples the directions, and doubles the ray interval. The
// two changes cancel, so every cascade costs about the same and holds about
// the same number of texels — which is the whole reason this scales where the
// patch solve does not (that stores a dense N x N form-factor matrix, so
// halving patch size is 16x the memory AND 16x the solve; measured 44.4 MB at
// 0.4 m and 711 MB at 0.2 m). Nothing here is quadratic in anything.
//
// Halving in x/z only, not y: the playable volume is a 3.25 m slab, so a
// cascade that also coarsened vertically would be down to two layers by
// cascade 2 and have nothing left to interpolate. That makes this 2.5D
// cascades, which is the same assumption the rest of the game makes.
//
//   cascade  spacing  probes      dirs  interval
//   0        0.5 m    104x7x72      16  [0,    0.5)
//   1        1.0 m     52x7x36      64  [0.5,  1.5)
//   2        2.0 m     26x7x18     256  [1.5,  3.5)
//   3        4.0 m     13x7x9     1024  [3.5,  7.5)
//   4        8.0 m      7x7x5     4096  [7.5, 15.5)
//   5       16.0 m      4x7x3    16384  [15.5, 31.5)
//
// Cascade 0 starts at a 4x4 direction grid, not 2x2: at res 2 the four cell
// centres land on the octahedron's equator and every direction comes out
// coplanar, leaving no vertical coverage at all. See CASCADE0_DIR_RES.
//
// Two passes per cascade. `traceCascade` shoots each probe's rays over that
// cascade's interval and shades the hits with the scene's own NEE. `merge`
// then folds the coarser cascade in behind whatever the near ray did not
// block, from the top down, so cascade 0 ends up carrying the whole room. The
// trace output and the merged result are separate textures because a pass
// cannot read the storage texture it writes.
//
// Storage per texel: rgb = radiance arriving along that direction, a = 1 when
// the ray hit inside its own interval. `a` is the occlusion term the merge
// needs — a direction that already hit something must not also receive what
// the longer cascade found behind it.
// ===========================================================================

struct CascadeParams {
  /** World position of the lattice's minimum corner; shared by all cascades. */
  origin     : vec3f,
  /** Octahedral direction grid side for THIS cascade. */
  dirRes     : u32,
  /** Metres between probes of this cascade, per axis. */
  spacing    : vec3f,
  /** Ray interval start, metres from the probe. */
  tStart     : f32,
  /** Probe counts of this cascade. */
  probes     : vec3u,
  tEnd       : f32,
  /**
   * Weight of this frame's trace against history, 1 = replace. Probes are
   * fixed in world space, so this needs no reprojection and no validation the
   * way a screen-space history does — a probe is the same probe every frame.
   * That is what makes one NEE shadow ray per probe ray affordable.
   */
  alpha      : f32,
  frame      : u32,
  /** 0 for the top cascade, which has nothing coarser to merge. */
  hasCoarser : u32,
  /** RIS candidates per probe hit — see probeDirect. */
  candidates : u32,
}

@group(1) @binding(0) var<uniform> C : CascadeParams;
@group(1) @binding(1) var cascadeOut : texture_storage_3d<rgba16float, write>;
/** Trace only: this cascade's previous trace, for the temporal average. */
@group(1) @binding(2) var cascadeHist : texture_3d<f32>;
/** Merge only: this cascade's raw trace. */
@group(1) @binding(3) var cascadeRaw : texture_3d<f32>;
/** Merge only: the next cascade out, already merged. Also the irradiance
 *  pass's input, where it is bound to merged cascade 0. */
@group(1) @binding(4) var cascadeCoarse : texture_3d<f32>;
/**
 * Trace only: the second-bounce irradiance volume, four texels per cascade-0
 * probe — see `irradiance`. Built from LAST frame's merged cascade 0, which is
 * what makes the bounce count recursive: frame n's probes light frame n+1's
 * probe hits, so the series converges to infinite bounces the same way the
 * radiosity solve's Jacobi iteration does, one frame per bounce.
 */
@group(1) @binding(5) var shVolume : texture_3d<f32>;

/**
 * Second-bounce irradiance at a surface, from the SH volume.
 *
 * Nearest probe, not trilinear. This is a SECONDARY bounce: it is low
 * frequency, small in magnitude, and its result is deposited into a probe that
 * is itself trilinearly interpolated at shading time, so the lattice step is
 * smoothed downstream. Trilinear here would be 32 texture loads on every one
 * of ~4M probe rays, which costs more than the whole merge.
 *
 * The grid is cascade 0's, derived the same way the shading lookup derives it
 * (U.smokeOrigin/U.smokeCell plus textureDimensions), because C holds the
 * CURRENT cascade's spacing and this volume is always cascade 0's.
 */
fn bounceIrradiance(p: vec3f, n: vec3f) -> vec3f {
  let dims = vec3i(textureDimensions(shVolume));
  let probes = vec3i(dims.x / 4, dims.y, dims.z);
  let sp = U.smokeCell * CASCADE_SPACING_CELLS;
  // Normal bias before snapping. A hit sits ON a surface, so the nearest probe
  // is as likely to be the one BEHIND it — inside the wall, or in the dead air
  // on the far side — as the one in front. Those probes trace nothing and read
  // black, so an unbiased lookup loses roughly half the bounce on every wall.
  // Half a spacing along the normal moves the query into the air the surface
  // actually faces.
  let f = (p + n * sp * 0.5 - U.smokeOrigin) / sp - 0.5;
  let c = clamp(vec3i(round(f)), vec3i(0), probes - vec3i(1));
  let b = c.x * 4;
  let A  = textureLoad(shVolume, vec3i(b,     c.y, c.z), 0).rgb;
  let Bx = textureLoad(shVolume, vec3i(b + 1, c.y, c.z), 0).rgb;
  let By = textureLoad(shVolume, vec3i(b + 2, c.y, c.z), 0).rgb;
  let Bz = textureLoad(shVolume, vec3i(b + 3, c.y, c.z), 0).rgb;
  // E(n) = pi*Y00^2*A + (2pi/3)*Y1^2*dot(B,n), and both constants collapse:
  // pi*(1/4pi) = 1/4 and (2pi/3)*(3/4pi) = 1/2. Sanity: uniform radiance L
  // gives A = 4*pi*L, B = 0, E = pi*L, which is the hemisphere integral.
  return max(0.25 * A + 0.5 * (Bx * n.x + By * n.y + Bz * n.z), vec3f(0.0));
}

/** Splits a fused texel coordinate back into probe and direction cell. */
struct ProbeDir {
  probe : vec3u,
  dir   : vec2u,
}

fn splitTexel(gid: vec3u, res: u32) -> ProbeDir {
  var pd : ProbeDir;
  pd.probe = vec3u(gid.x / res, gid.y / res, gid.z);
  pd.dir = vec2u(gid.x % res, gid.y % res);
  return pd;
}

/**
 * Direct light at a probe-ray hit: ONE resampled candidate, ONE shadow ray.
 *
 * The pathtracer splits this in two — sampleKeyLight always spends a ray on
 * lights[0], and sampleSceneLight resamples the rest and spends another —
 * because the key light has its own channel in the image. A probe has no such
 * channel, so pooling every steady light into one reservoir halves the shadow
 * rays per hit, and shadow rays are what this pass is made of.
 *
 * Two deliberate differences from sampleSceneLight:
 *
 * Only STEADY lights, [0, transientStart). Muzzle flashes are a three-frame
 * event and the probe volume is a temporal average, so a flash sampled here
 * would smear into the field and linger long after the shot — the same reason
 * the radiosity solve stops at transientStart.
 *
 * The candidate count is a tunable rather than a fixed 8. A probe's estimate is
 * averaged over ~8 frames by the EMA and then trilinearly smeared across its
 * neighbours before anything sees it, so it can absorb variance the per-pixel
 * path cannot.
 *
 * The estimator weight does NOT follow sampleSceneLight, whose comment claims
 * the uniform source pdf cancels. It does not — see the note on W below. That
 * function has no callers anywhere in the shaders (the shipping direct path is
 * ReSTIR DI), so its convention was never exercised and the error sat there
 * unnoticed; this was the first code to copy it.
 */
fn probeDirect(p: vec3f, n: vec3f, v: vec3f, m: Material) -> vec3f {
  let count = U.transientStart;
  if (count == 0u) { return vec3f(0.0); }
  let take = max(1u, min(count, C.candidates));
  var chosen : Light;
  var chosenSample : LightSample;
  var chosenTarget = 0.0;
  var wsum = 0.0;
  for (var i = 0u; i < take; i = i + 1u) {
    let idx = min(u32(rand() * f32(count)), count - 1u);
    let l = lights[idx];
    if (l.intensity <= 0.0) { continue; }
    let s = sampleSphereLight(l, p);
    var atten = 1.0;
    if (l.kind == LIGHT_SPOT) {
      atten = spotAttenuation(l.dir, -s.dir, l.cosInner, l.cosOuter);
    }
    let t = risTarget(m, n, s.dir, s.radiance * atten);
    wsum = wsum + t;
    if (t > 0.0 && rand() * wsum < t) {
      chosen = l;
      chosenSample = s;
      chosenTarget = t;
    }
  }
  if (wsum <= 0.0 || chosenTarget <= 0.0) { return vec3f(0.0); }
  // The `count` factor is the source pdf, and it does NOT cancel.
  //
  // Candidates are drawn uniformly, p(x) = 1/count, so each RIS weight is
  // w_i = target_i / p(x_i) = count * target_i, and the estimator is
  // f(y)/target(y) * (1/M) * sum(w_i) = f(y)/target(y) * (count/M) * sum(target).
  // Dropping `count` makes the result exactly `count` times too dark, which is
  // invisible in a one-light room and catastrophic in a level with 33 lights.
  //
  // Measured in the manyStrips synthetic scene (16 identical emitters, one
  // shell): 0.040 of a converged reference without the factor, 0.60 with it.
  // It also explains why energy was FLAT across candidate counts — M was never
  // the missing term, `count` was.
  let W = min(f32(count) * (wsum / f32(take)) / chosenTarget, 24.0);
  let contrib = lightContribution(chosen, chosenSample, p, n, v, m) * W;
  // NOT culled at SHADOW_CULL.
  //
  // That threshold is calibrated for a PIXEL, where a contribution below it
  // rounds away in the final image. A probe is not a pixel: it integrates the
  // whole sphere and its result is then spread over every surface that
  // interpolates it, so the office's ~29 dim practicals each land under the
  // pixel threshold individually while summing to most of the room's bounce.
  // Culling them measured as cascades delivering 11% of radiosity's indirect
  // with the torch off, against 35% with it on — the torch is the one light
  // bright enough to survive.
  if (luminance(contrib) <= 0.0) { return vec3f(0.0); }
  if (occluded(p + n * EPS * 4.0, chosenSample.dir, chosenSample.dist - EPS * 8.0)) {
    return vec3f(0.0);
  }
  return contrib;
}

@compute @workgroup_size(4, 4, 4)
fn traceCascade(@builtin(global_invocation_id) gid: vec3u) {
  let res = C.dirRes;
  let pd = splitTexel(gid, res);
  if (any(pd.probe >= C.probes)) { return; }

  seedRng(vec2u(gid.x, gid.z), gid.y * 9781u + C.frame * 6151u);

  let probe = C.origin + (vec3f(pd.probe) + 0.5) * C.spacing;
  // Jittered inside the direction cell, not its centre.
  //
  // One texel is a whole cone — 4*pi/16 sr at cascade 0, about 29 degrees
  // across — and tracing its exact centre every frame makes occlusion binary
  // for that entire cone: a single desk leg blocks all of it from receiving
  // anything the coarser cascades found behind. In a room of 532 boxes that
  // over-occludes badly, which is invisible in a demo room of eight.
  //
  // Jittering costs nothing (same ray count) and the temporal average turns
  // `closed` into fractional cone coverage instead of a coin flip, so it also
  // buys angular resolution over time that the direction count does not have.
  let d = cascadeDirJitter(pd.dir, res, rand2());

  // The ray covers this cascade's interval only: it starts where the finer
  // cascade stopped, so no length of the world is integrated twice.
  let ro = probe + d * C.tStart;
  let h = trace(ro, d, C.tEnd - C.tStart, false, false);

  var rad = vec3f(0.0);
  var closed = 0.0;
  if (h.valid) {
    closed = 1.0;
    let m = materials[h.mat];
    // Radiance leaving the hit back down the ray. `v` is what the BSDF inside
    // lightContribution expects.
    let v = -d;
    // NOT m.emissive.
    //
    // The probe field feeds illumIndirect, and the renderer computes direct
    // light separately per pixel. Light travelling straight from an emitter to
    // a shaded surface is DIRECT — counting the emitter's own glow here adds it
    // a second time. Measured in the onePanel synthetic scene, where one large
    // ceiling emitter made cascades 7.6x brighter than a converged reference;
    // the office hid it because its emitters are small and mostly recessed in
    // housings. What belongs here is only radiance that has already bounced,
    // which is exactly what probeDirect plus the multi-bounce term give.
    //
    // Emitters still occlude: the ray stops on them, it just collects nothing
    // (their albedo is zero, so probeDirect returns zero there too).
    rad = probeDirect(h.p, h.n, v, m)
        // The torch stays separate: it cone-culls before spending a ray, so it
        // is nearly free, and it is the one light whose bounce the player is
        // actively steering.
        + sampleFlashlight(h.p, h.n, v, m)
        // Everything above is direct light at the hit. This is what the hit
        // received from the rest of the room last frame, which is the whole of
        // the multi-bounce term: albedo/pi * E is a diffuse surface's outgoing
        // radiance for irradiance E.
        + m.albedo * INV_PI * bounceIrradiance(h.p, h.n);
  }

  var out = vec4f(rad, closed);
  if (C.alpha < 1.0) {
    out = mix(textureLoad(cascadeHist, vec3i(gid), 0), out, C.alpha);
  }
  textureStore(cascadeOut, gid, out);
}

// ---------------------------------------------------------------------------

/**
 * The coarser cascade's contribution behind this direction: its four child
 * directions, trilinearly interpolated across its probes at `p`.
 *
 * The four children share the same eight spatial corners and the same eight
 * weights, so the corner loop is the outer one and the children are summed
 * inside it. Doing it the other way round — a full trilinear per child —
 * recomputes the weights and the bounds tests four times for the same 32
 * texture loads, and the loads themselves are then scattered instead of
 * landing on an adjacent 2x2 block.
 *
 * Interpolating spatially at all is what keeps this continuous: the coarse
 * lattice is up to 8 m apart, and reading the nearest probe would put a
 * visible cell edge everywhere its choice flips. Weights are renormalised
 * over the probes that exist so the lattice edge leans inward rather than
 * fading to black.
 */
fn coarseBehind(
  p: vec3f, dir: vec2u, res: u32, probes: vec3i, spacing: vec3f,
) -> vec4f {
  let f = (p - C.origin) / spacing - 0.5;
  let base = vec3i(floor(f));
  let fr = f - floor(f);
  let child = vec2i(dir * 2u);
  var acc = vec4f(0.0);
  var wsum = 0.0;
  for (var k = 0; k < 2; k = k + 1) {
    for (var j = 0; j < 2; j = j + 1) {
      for (var i = 0; i < 2; i = i + 1) {
        let cc = base + vec3i(i, j, k);
        if (any(cc < vec3i(0)) || any(cc >= probes)) { continue; }
        let w = select(1.0 - fr.x, fr.x, i == 1)
              * select(1.0 - fr.y, fr.y, j == 1)
              * select(1.0 - fr.z, fr.z, k == 1);
        if (w <= 0.0) { continue; }
        let ox = cc.x * i32(res) + child.x;
        let oy = cc.y * i32(res) + child.y;
        // The four children are the adjacent 2x2 block at (2dx, 2dy).
        var sum = textureLoad(cascadeCoarse, vec3i(ox, oy, cc.z), 0)
                + textureLoad(cascadeCoarse, vec3i(ox + 1, oy, cc.z), 0)
                + textureLoad(cascadeCoarse, vec3i(ox, oy + 1, cc.z), 0)
                + textureLoad(cascadeCoarse, vec3i(ox + 1, oy + 1, cc.z), 0);
        acc = acc + w * sum * 0.25;
        wsum = wsum + w;
      }
    }
  }
  if (wsum < 1e-5) { return vec4f(0.0); }
  return acc / wsum;
}

/**
 * Projects merged cascade 0 onto an order-1 spherical-harmonic irradiance
 * volume, one probe per thread, four texels out per probe: A, then B.x/B.y/B.z.
 *
 * SH rather than storing the directional field again because the consumer is a
 * secondary bounce that only ever needs irradiance for one normal. Four loads
 * and a dot product replace a 16-direction cosine sum, and unlike a single
 * ambient scalar it still knows which way the light came from — so a floor and
 * the ceiling above it do not get the same bounce.
 */
@compute @workgroup_size(4, 4, 4)
fn irradiance(@builtin(global_invocation_id) gid: vec3u) {
  if (any(gid >= C.probes)) { return; }
  let res = C.dirRes;
  let dw = 4.0 * PI / f32(res * res);
  var A = vec3f(0.0);
  var Bx = vec3f(0.0);
  var By = vec3f(0.0);
  var Bz = vec3f(0.0);
  for (var dy = 0u; dy < res; dy = dy + 1u) {
    for (var dx = 0u; dx < res; dx = dx + 1u) {
      let L = textureLoad(cascadeCoarse, vec3i(
        i32(gid.x * res + dx), i32(gid.y * res + dy), i32(gid.z)), 0).rgb;
      let d = cascadeDirOct(vec2u(dx, dy), res);
      let Ld = L * dw;
      A = A + Ld;
      Bx = Bx + Ld * d.x;
      By = By + Ld * d.y;
      Bz = Bz + Ld * d.z;
    }
  }
  let b = gid.x * 4u;
  textureStore(cascadeOut, vec3u(b, gid.y, gid.z), vec4f(A, 0.0));
  textureStore(cascadeOut, vec3u(b + 1u, gid.y, gid.z), vec4f(Bx, 0.0));
  textureStore(cascadeOut, vec3u(b + 2u, gid.y, gid.z), vec4f(By, 0.0));
  textureStore(cascadeOut, vec3u(b + 3u, gid.y, gid.z), vec4f(Bz, 0.0));
}

@compute @workgroup_size(4, 4, 4)
fn merge(@builtin(global_invocation_id) gid: vec3u) {
  let res = C.dirRes;
  let pd = splitTexel(gid, res);
  if (any(pd.probe >= C.probes)) { return; }

  let own = textureLoad(cascadeRaw, vec3i(gid), 0);
  if (C.hasCoarser == 0u) {
    // Top cascade: nothing behind it. Copying rather than skipping the pass
    // keeps one code path for the lookup, which always reads a merged texture.
    textureStore(cascadeOut, gid, own);
    return;
  }

  // Whatever this cascade's own ray did not block, the next one out may fill.
  let transmit = 1.0 - own.a;
  var far = vec4f(0.0);
  if (transmit > 0.0) {
    let probe = C.origin + (vec3f(pd.probe) + 0.5) * C.spacing;
    // The coarser cascade halves probe density in x/z and doubles the
    // direction grid.
    let cspacing = vec3f(C.spacing.x * 2.0, C.spacing.y, C.spacing.z * 2.0);
    let cprobes = vec3i(
      i32((C.probes.x + 1u) / 2u), i32(C.probes.y), i32((C.probes.z + 1u) / 2u));
    far = coarseBehind(probe, pd.dir, res * 2u, cprobes, cspacing);
  }

  textureStore(
    cascadeOut, gid,
    vec4f(own.rgb + transmit * far.rgb, max(own.a, transmit * far.a)));
}
