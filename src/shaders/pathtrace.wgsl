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
@group(1) @binding(3) var illumOut : texture_storage_2d<rgba16float, write>;
@group(1) @binding(4) var prevNormalDepth : texture_2d<f32>;
@group(1) @binding(5) var<storage, read> reservoirPrev : array<Reservoir>;
@group(1) @binding(6) var<storage, read_write> reservoirCur : array<Reservoir>;
/**
 * Indirect radiance goes to its own target.
 *
 * Summing it into the direct signal before denoising forces one filter to
 * serve both, and they want opposite things: direct light carries hard shadow
 * edges that must survive, while bounce light is low frequency and wants
 * blurring hard. Separated, each can be filtered on its own terms.
 */
@group(1) @binding(7) var illumIndirectOut : texture_storage_2d<rgba16float, write>;
@group(1) @binding(8) var<storage, read> giPrev : array<GIReservoir>;
@group(1) @binding(9) var<storage, read_write> giCur : array<GIReservoir>;
/**
 * Transient lighting, kept apart from both steady signals.
 *
 * Steady light wants a 48-frame history; a muzzle flash wants none at all.
 * One set of temporal parameters cannot serve both — a long history leaves a
 * glow hanging after the light is gone, and a short one throws away the whole
 * screen's convergence. Separated, this signal simply is not accumulated, so it
 * appears and vanishes with the light by construction rather than by tuning.
 */
@group(1) @binding(10) var illumTransientOut : texture_storage_2d<rgba16float, write>;

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
fn volumetricFlashlight(ro: vec3f, rd: vec3f, tmax: f32) -> f32 {
  if (U.volumetric <= 0.0 || U.flashIntensity <= 0.0) { return 0.0; }

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
    let delta = U.flashPos - p;
    let d2 = max(dot(delta, delta), 0.05);
    let dist = sqrt(d2);
    let dir = delta / dist;

    // Cheap cone rejection first — most of the screen is outside the beam and
    // pays nothing beyond this test.
    let cone = spotAttenuation(U.flashDir, -dir, U.flashCosInner, U.flashCosOuter);
    if (cone <= 0.001) { continue; }

    if (occluded(p, dir, dist - EPS * 8.0)) { continue; }

    // dir points from the march point toward the lamp, so light propagates
    // along -dir and the scattered light reaching the camera propagates along
    // -rd. cos(theta) = dot(-dir, -rd) = dot(dir, rd).
    let phase = phaseHG(dot(rd, dir), 0.55);
    acc = acc + cone * phase / d2 * dt;
  }
  return acc * U.flashIntensity * U.volumetric;
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
  var res = generateReservoir(h.p, h.n, v, m, u32(U.restirCandidates));

  // ---- temporal reuse --------------------------------------------------
  if (U.restirTemporal > 0.5) {
    let clip = U.prevViewProj * vec4f(prevWorld, 1.0);
    if (clip.w > 0.0) {
      let ndc = clip.xy / clip.w;
      let uv = vec2f(ndc.x * 0.5 + 0.5, 0.5 - ndc.y * 0.5);
      if (uv.x >= 0.0 && uv.x < 1.0 && uv.y >= 0.0 && uv.y < 1.0) {
        let pp = vec2i(uv * vec2f(dims));
        let pnd = textureLoad(prevNormalDepth, pp, 0);
        // Same surface test as the denoiser: reuse across a depth or normal
        // discontinuity would drag a neighbouring surface's lighting in.
        let okDepth = abs(pnd.w - h.t) < 0.12 * max(h.t, 1.0);
        let okNormal = dot(pnd.xyz, h.n) > 0.9;
        if (okDepth && okNormal) {
          var prev = reservoirPrev[u32(pp.y) * dims.x + u32(pp.x)];
          // No index guard needed: reservoirs only ever hold steady lights now,
          // and the steady range never shrinks mid-session. Transients used to
          // be reusable, which meant a reservoir could outlive its own light.
          prev.M = min(prev.M, U.restirMCap);
          mergeReservoir(&res, prev, h.p, h.n, v, m);
        }
      }
    }
  }

  finalizeReservoir(&res);

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
    } else {
      let rad = radianceFromLight(res.lightIdx, h.p, res.samplePos);
      contrib = evalBSDF(m, h.n, v, dir) * rad * res.W;
    }
  }

  reservoirCur[pixel.y * dims.x + pixel.x] = res;
  return contrib;
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

  if (U.restirTemporal > 0.5) {
    let clip = U.prevViewProj * vec4f(prevWorld, 1.0);
    if (clip.w > 0.0) {
      let ndc = clip.xy / clip.w;
      let uv = vec2f(ndc.x * 0.5 + 0.5, 0.5 - ndc.y * 0.5);
      if (uv.x >= 0.0 && uv.x < 1.0 && uv.y >= 0.0 && uv.y < 1.0) {
        let pp = vec2i(uv * vec2f(dims));
        let pnd = textureLoad(prevNormalDepth, pp, 0);
        let okDepth = abs(pnd.w - h.t) < 0.12 * max(h.t, 1.0);
        let okNormal = dot(pnd.xyz, h.n) > 0.9;
        if (okDepth && okNormal) {
          var prev = giPrev[u32(pp.y) * dims.x + u32(pp.x)];
          if (prev.M > 0.0 && prev.W > 0.0) {
            prev.M = min(prev.M, U.restirMCap);
            // Shift: keep x2, rebuild the connection from our visible point.
            let delta = prev.samplePos - h.p;
            let d2 = dot(delta, delta);
            if (d2 > 1e-6) {
              let dir = delta * inverseSqrt(d2);
              let j = giJacobian(prev, h.p);
              let tp = giTarget(m, h.n, v, dir, prev.radiance);
              giUpdate(
                &res, prev.samplePos, prev.sampleNrm, prev.radiance, h.p,
                tp * prev.W * prev.M * j, tp, prev.M,
              );
            }
          }
        }
      }
    }
  }

  finalizeGIReservoir(&res);

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
    } else {
      contrib = evalBSDF(m, h.n, v, dir) * res.radiance * res.W;
    }
  }

  giCur[pixel.y * dims.x + pixel.x] = res;
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

  let primary = trace(ro, rd, RAY_MAX, true);

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
        h = trace(rayO, rayD, RAY_MAX, false);
      }
      if (!h.valid) {
        let sky = throughput * skyRadiance(rayD);
        if (b == 0u) {
          radiance = radiance + sky;
        } else {
          // A ray that escapes has no surface to reconnect to, so it cannot
          // become a GI reservoir sample. Bank it directly instead.
          indirect = indirect + sky;
          giRad = giRad + giThroughput * skyRadiance(rayD);
        }
        break;
      }
      if (b == 1u) {
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
        } else {
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
      } else {
        // Deeper bounces resample across every light at once rather than
        // picking a channel and scaling up — see sampleIndirectRIS.
        let li = sampleIndirectRIS(h.p, h.n, v, m);
        indirect = indirect + throughput * li;
        giRad = giRad + giThroughput * li;
      }

      if (b == U.bounces) { break; }
      // Pixels sitting out this frame stop after direct lighting.
      if (b == 0u && !traceIndirect) { break; }

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

  let vol = volumetricFlashlight(ro, rd, depth);
  var illum = radiance / demod;
  var illumIndirect = indirect / demod;
  let illumTransient = transient / demod;

  // A small ambient floor. Physically the sealed ceiling means an unlit corner
  // really is black, but a stealth game still has to be playable: this lifts
  // shadowed geometry just enough to read silhouettes without touching the
  // contrast where the beam actually falls. Demodulated, so it tints by albedo.
  if (primary.valid) { illum = illum + vec3f(U.ambient); }

  // The raw luminance moments are derivable from these, so reproject computes
  // them itself rather than us burning extra render targets on them.
  textureStore(illumOut, pixel, vec4f(illum, vol));
  // Alpha is the validity flag, so a checkerboard pixel that sat this frame out
  // is distinguishable from one that genuinely received no bounce light.
  textureStore(illumIndirectOut, pixel, vec4f(illumIndirect, select(0.0, 1.0, traceIndirect)));
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
  textureStore(illumTransientOut, pixel, vec4f(illumTransient, transBlur));
}
