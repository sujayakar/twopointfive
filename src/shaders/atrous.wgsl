// ===========================================================================
// Edge-aware a-trous wavelet filter (SVGF).
//
// Run several times with a doubling stride. Each pass is a 5x5 cross-bilateral
// blur whose weights are driven by normal, depth and — crucially — the
// temporally estimated luminance variance, so converged pixels stay sharp
// while noisy ones get smoothed hard.
// ===========================================================================

struct AtrousParams {
  stepSize : i32,
  /**
   * Scales the luminance edge-stopping term. 1 preserves detail; 0 disables it
   * entirely, which is what the indirect chain wants — that term is what keeps
   * bounce-light noise alive, because it treats a noisy pixel as an edge worth
   * protecting.
   */
  lumaWeight : f32,
  /** Multiplies the normal/depth strictness; indirect can afford to be looser. */
  edgeRelax  : f32,
  _pad2      : f32,
}

@group(1) @binding(0) var illumIn : texture_2d<f32>;
@group(1) @binding(1) var momentsIn : texture_2d<f32>;
@group(1) @binding(2) var gNormalDepth : texture_2d<f32>;
@group(1) @binding(3) var illumOut : texture_storage_2d<rgba16float, write>;
@group(1) @binding(4) var momentsOut : texture_storage_2d<rgba16float, write>;
@group(1) @binding(5) var<uniform> P : AtrousParams;

const SIGMA_Z: f32 = 2.0;
const SIGMA_N: f32 = 128.0;
const SIGMA_L: f32 = 4.0;

/** 3x3 gaussian prefilter of the variance channel — stabilises the luma weight. */
fn filteredVariance(pixel: vec2i, dims: vec2i) -> f32 {
  let k = array<f32, 3>(0.25, 0.5, 0.25);
  var sum = 0.0;
  var wsum = 0.0;
  for (var dy = -1; dy <= 1; dy = dy + 1) {
    for (var dx = -1; dx <= 1; dx = dx + 1) {
      let c = clamp(pixel + vec2i(dx, dy), vec2i(0), dims - 1);
      let w = k[dx + 1] * k[dy + 1];
      sum = sum + textureLoad(momentsIn, c, 0).w * w;
      wsum = wsum + w;
    }
  }
  return max(sum / wsum, 0.0);
}

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) gid: vec3u) {
  let dims = vec2i(textureDimensions(illumIn));
  let pixel = vec2i(gid.xy);
  if (pixel.x >= dims.x || pixel.y >= dims.y) { return; }

  let centerIllum = textureLoad(illumIn, pixel, 0);
  let centerMoments = textureLoad(momentsIn, pixel, 0);
  let centerND = textureLoad(gNormalDepth, pixel, 0);

  // Sky: nothing to filter against, and its "normal" is meaningless.
  if (centerND.w > 9e3) {
    textureStore(illumOut, pixel, centerIllum);
    textureStore(momentsOut, pixel, centerMoments);
    return;
  }

  // Uniform branch: the indirect chain runs with lumaWeight 0, and mix() would
  // still evaluate the exp() -- the same trap as select(), which this codebase
  // has been bitten by before. Skipping it drops 9 texture loads and 24 exp()
  // per pixel per dispatch.
  let useLuma = P.lumaWeight > 0.0;
  var lumaScale = 1.0;
  if (useLuma) { lumaScale = SIGMA_L * sqrt(filteredVariance(pixel, dims)) + 1e-3; }
  let lumaCenter = luminance(centerIllum.rgb);
  let relax = max(P.edgeRelax, 1e-3);

  let kernel = array<f32, 3>(3.0 / 8.0, 1.0 / 4.0, 1.0 / 16.0);

  // The centre tap is weighted 1.0, not the B3-spline kernel's own (3/8)^2.
  // That makes the centre ~54% of the output and the filter roughly 4x weaker
  // than textbook SVGF at every stride.
  //
  // Left alone deliberately. Switching to 9/64 was measured against the
  // converged reference and the runs would not settle (relRMSE 2.59 / 1.25 /
  // 0.90 for identical configs, against a ~1.2% floor elsewhere), so there is
  // no honest evidence either way — and unlike the variance normalisation below
  // this is a sharpness preference, not a correctness bug. Needs a human A/B.
  var sum = centerIllum;
  var wsum = 1.0;
  var varSum = centerMoments.w;

  for (var dy = -2; dy <= 2; dy = dy + 1) {
    for (var dx = -2; dx <= 2; dx = dx + 1) {
      if (dx == 0 && dy == 0) { continue; }
      let c = pixel + vec2i(dx, dy) * P.stepSize;
      if (c.x < 0 || c.y < 0 || c.x >= dims.x || c.y >= dims.y) { continue; }

      let nd = textureLoad(gNormalDepth, c, 0);
      if (nd.w > 9e3) { continue; }

      let s = textureLoad(illumIn, c, 0);

      let wn = pow(max(0.0, dot(centerND.xyz, nd.xyz)), SIGMA_N / relax);
      let wz = exp(-abs(centerND.w - nd.w) / (SIGMA_Z * relax * 0.05 * max(centerND.w, 1.0)));
      var wl = 1.0;
      if (useLuma) {
        wl = mix(1.0, exp(-abs(lumaCenter - luminance(s.rgb)) / lumaScale), P.lumaWeight);
      }

      let h = kernel[abs(dx)] * kernel[abs(dy)];
      let w = h * wn * wz * wl;
      if (w <= 0.0) { continue; }

      sum = sum + s * w;
      wsum = wsum + w;
      // Variance is a second moment: it filters with the squared weight.
      varSum = varSum + textureLoad(momentsIn, c, 0).w * w * w;
    }
  }

  let outIllum = sum / wsum;
  // Var of a weighted mean is sum(w^2 * var) / (sum w)^2 -- the SAME wsum that
  // normalises the colour, not sum(w^2). Dividing by sum(w^2) computes a
  // weighted *average* of the input variances instead, so the estimate never
  // falls as the filter widens: it over-reports by ~3.3x per pass, compounding
  // to ~120x by iteration 3, which inflates lumaScale ~11x and switches the
  // luminance edge-stop off almost entirely on the wider passes.
  let outVar = varSum / max(wsum * wsum, 1e-6);

  textureStore(illumOut, pixel, outIllum);
  textureStore(momentsOut, pixel, vec4f(centerMoments.xyz, outVar));
}
