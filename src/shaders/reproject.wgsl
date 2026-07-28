// ===========================================================================
// Temporal accumulation.
//
// Reprojects last frame's filtered illumination through the previous camera,
// validates it against depth+normal, then blends. A moving flashlight makes
// naive accumulation ghost badly, so history is additionally clamped to the
// current frame's local colour neighbourhood (the standard TAA trick).
// ===========================================================================

@group(1) @binding(0) var illumRaw : texture_2d<f32>;
@group(1) @binding(1) var gPos : texture_2d<f32>;
@group(1) @binding(2) var gNormalDepth : texture_2d<f32>;
@group(1) @binding(3) var prevIllum : texture_2d<f32>;
@group(1) @binding(4) var prevMoments : texture_2d<f32>;
@group(1) @binding(5) var prevNormalDepth : texture_2d<f32>;
@group(1) @binding(6) var illumOut : texture_storage_2d<rgba16float, write>;
@group(1) @binding(7) var momentsOut : texture_storage_2d<rgba16float, write>;

/**
 * Per-signal filter strength.
 *
 * The neighbourhood statistics below are a decent estimator of the local mean
 * only when the signal is dense. Direct light is; a 1-spp indirect estimate is
 * not — it is mostly zeros with occasional bright hits, so a 3x3 window reads
 * mean ~ 0, sigma ~ 0 and both the firefly ceiling and the history clamp
 * collapse onto zero, deleting the entire signal. The indirect pass therefore
 * runs with a far more permissive band.
 */
struct ReprojectParams {
  /** Sigma multiplier for the current-sample firefly ceiling. */
  outlierK     : f32,
  /** Absolute allowance added to that ceiling, in demodulated radiance. */
  outlierFloor : f32,
  /** Sigma multiplier for the history clamp band. */
  clampK       : f32,
  /** Absolute widening of that band. */
  clampFloor   : f32,
  /**
   * 1 when alpha carries a "this pixel traced indirect this frame" flag rather
   * than volumetric density. Pixels sitting out a checkerboard frame must pass
   * their history through untouched, not accumulate black into it.
   */
  validityInAlpha : f32,
  /**
   * History length cap. Interactive rendering wants this low so the image stays
   * responsive to a moving light; the reference accumulator wants it unbounded
   * so the running average actually converges.
   */
  maxHistory : f32,
  /** Floor on the blend factor. 0 gives a true 1/n running average. */
  alphaFloor : f32,
  _pad1 : f32,
}

@group(1) @binding(8) var<uniform> P : ReprojectParams;


fn validTap(coord: vec2i, dims: vec2i, curNormal: vec3f, curDepth: f32) -> bool {
  if (coord.x < 0 || coord.y < 0 || coord.x >= dims.x || coord.y >= dims.y) { return false; }
  let pnd = textureLoad(prevNormalDepth, coord, 0);
  // Relative depth test scales with distance, so far geometry is not rejected
  // purely for being far away.
  if (abs(pnd.w - curDepth) > 0.12 * max(curDepth, 1.0)) { return false; }
  if (dot(pnd.xyz, curNormal) < 0.90) { return false; }
  return true;
}

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) gid: vec3u) {
  let dims = vec2i(textureDimensions(illumRaw));
  let pixel = vec2i(gid.xy);
  if (pixel.x >= dims.x || pixel.y >= dims.y) { return; }

  let raw = textureLoad(illumRaw, pixel, 0);
  let nd = textureLoad(gNormalDepth, pixel, 0);
  let posSample = textureLoad(gPos, pixel, 0);

  // ---- neighbourhood statistics -----------------------------------------
  // Computed once and used twice: to reject fireflies in the current sample,
  // and to clamp stale history further down. The centre pixel is excluded so a
  // spike cannot mask itself.
  let dims2 = dims;
  var m1 = vec3f(0.0);
  var m2 = vec3f(0.0);
  var nTaps = 0.0;
  for (var dy = -1; dy <= 1; dy = dy + 1) {
    for (var dx = -1; dx <= 1; dx = dx + 1) {
      if (dx == 0 && dy == 0) { continue; }
      let c = clamp(pixel + vec2i(dx, dy), vec2i(0), dims2 - 1);
      let sTap = textureLoad(illumRaw, c, 0).rgb;
      m1 = m1 + sTap;
      m2 = m2 + sTap * sTap;
      nTaps = nTaps + 1.0;
    }
  }
  let mean = m1 / nTaps;
  let sigma = sqrt(max(m2 / nTaps - mean * mean, vec3f(0.0)));

  // Anti-firefly: an isolated pixel far above its neighbours is a sampling
  // accident, not detail. This has to happen before accumulation — the a-trous
  // luminance weight actively protects lone bright pixels, so anything that
  // reaches the history stays for dozens of frames.
  let outlierCeiling = mean + sigma * P.outlierK + vec3f(P.outlierFloor);
  let cur = vec4f(min(raw.rgb, outlierCeiling), raw.a);
  // Alpha doubles as the validity flag on the indirect pass.
  let sittingOut = P.validityInAlpha > 0.5 && raw.a < 0.5;

  let curLuma = luminance(cur.rgb);
  // A pixel with no history has a meaningless variance estimate, so seed it
  // high: the a-trous luminance weight treats low variance as "converged, keep
  // sharp", and writing 0 here left disocclusions completely unfiltered — a
  // frame of raw path-tracer noise trailing anything that moves.
  let seedVar = max(curLuma * curLuma, 1.0) * 4.0;
  let curMoments = vec4f(curLuma, curLuma * curLuma, 1.0, seedVar);

  // Sky pixels have no stable world position to reproject; just pass through.
  if (posSample.w < 0.5) {
    textureStore(illumOut, pixel, cur);
    textureStore(momentsOut, pixel, curMoments);
    return;
  }

  // ---- reproject ---------------------------------------------------------
  let clip = U.prevViewProj * vec4f(posSample.xyz, 1.0);
  var histIllum = vec4f(0.0);
  var histMoments = vec4f(0.0);
  var histLen = 0.0;
  var found = false;

  if (clip.w > 0.0) {
    let ndc = clip.xy / clip.w;
    let uv = vec2f(ndc.x * 0.5 + 0.5, 0.5 - ndc.y * 0.5);
    if (uv.x >= 0.0 && uv.x <= 1.0 && uv.y >= 0.0 && uv.y <= 1.0) {
      let fpos = uv * vec2f(dims) - 0.5;
      let base = vec2i(floor(fpos));
      let frac = fpos - floor(fpos);
      let bw = array<f32, 4>(
        (1.0 - frac.x) * (1.0 - frac.y),
        frac.x * (1.0 - frac.y),
        (1.0 - frac.x) * frac.y,
        frac.x * frac.y,
      );
      let offs = array<vec2i, 4>(vec2i(0, 0), vec2i(1, 0), vec2i(0, 1), vec2i(1, 1));

      var wsum = 0.0;
      for (var i = 0; i < 4; i = i + 1) {
        let c = base + offs[i];
        if (bw[i] > 0.0 && validTap(c, dims, nd.xyz, nd.w)) {
          histIllum = histIllum + textureLoad(prevIllum, c, 0) * bw[i];
          histMoments = histMoments + textureLoad(prevMoments, c, 0) * bw[i];
          wsum = wsum + bw[i];
        }
      }
      if (wsum > 0.01) {
        histIllum = histIllum / wsum;
        histMoments = histMoments / wsum;
        histLen = histMoments.z;
        found = true;
      }
    }
  }

  if (!found) {
    textureStore(illumOut, pixel, cur);
    textureStore(momentsOut, pixel, curMoments);
    return;
  }

  if (sittingOut) {
    textureStore(illumOut, pixel, vec4f(histIllum.rgb, 0.0));
    textureStore(momentsOut, pixel, histMoments);
    return;
  }

  // ---- neighbourhood clamp ----------------------------------------------
  // Without this the beam drags a comet tail behind it as the player turns.
  // A wide box (3 sigma) keeps genuine detail while still cutting long tails.
  let band = sigma * P.clampK + vec3f(P.clampFloor);
  let lo = mean - band;
  let hi = mean + band;
  let clampedHist = clamp(histIllum.rgb, lo, hi);
  // How much we had to clamp tells us the history was stale — shorten it so
  // the pixel re-converges quickly instead of fighting the clamp every frame.
  let rejection = length(clampedHist - histIllum.rgb) / (length(sigma) + 1e-3);
  histLen = max(1.0, histLen * exp(-rejection * 0.5));

  histLen = min(histLen + 1.0, P.maxHistory);
  let alpha = max(1.0 / histLen, P.alphaFloor);
  let alphaMoments = max(1.0 / histLen, max(P.alphaFloor, 0.06));

  let outIllum = mix(clampedHist, cur.rgb, alpha);
  // Volumetrics are smooth and benefit from a longer, unclamped history.
  let outVol = mix(histIllum.a, cur.a, max(1.0 / histLen, min(P.alphaFloor, 0.05)));

  let outM1 = mix(histMoments.x, curMoments.x, alphaMoments);
  let outM2 = mix(histMoments.y, curMoments.y, alphaMoments);
  var variance = max(0.0, outM2 - outM1 * outM1);
  // Short histories have a meaningless variance estimate; inflate it so the
  // a-trous pass filters those pixels hard for the first few frames.
  if (histLen < 4.0) { variance = max(variance, 1.0) * (4.0 / histLen); }

  textureStore(illumOut, pixel, vec4f(outIllum, outVol));
  textureStore(momentsOut, pixel, vec4f(outM1, outM2, histLen, variance));
}
