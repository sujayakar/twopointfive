// ===========================================================================
// Bloom: COD-style 13-tap downsample chain, tent-filter upsample chain.
//
// In a scene this dark the bloom is doing real work — it is what makes the
// flashlight lens and the exit signs read as genuinely bright rather than just
// light grey.
// ===========================================================================

struct BloomParams {
  srcTexel  : vec2f,
  threshold : f32,
  intensity : f32,
  prefilter : u32,
  _pad0     : u32,
  _pad1     : u32,
  _pad2     : u32,
}

@group(0) @binding(0) var samp : sampler;
@group(0) @binding(1) var src : texture_2d<f32>;
@group(0) @binding(2) var srcLower : texture_2d<f32>;
@group(0) @binding(3) var dst : texture_storage_2d<rgba16float, write>;
@group(0) @binding(4) var<uniform> B : BloomParams;

fn karisAverage(c: vec3f) -> f32 {
  // Weight by inverse luma so a single firefly cannot dominate the average.
  let l = dot(c, vec3f(0.2126, 0.7152, 0.0722)) * 0.25;
  return 1.0 / (1.0 + l);
}

@compute @workgroup_size(8, 8, 1)
fn downsample(@builtin(global_invocation_id) gid: vec3u) {
  let dims = vec2u(textureDimensions(dst));
  if (gid.x >= dims.x || gid.y >= dims.y) { return; }
  let uv = (vec2f(gid.xy) + 0.5) / vec2f(dims);
  let t = B.srcTexel;

  // 13 taps in a ring-plus-quad pattern.
  let a = textureSampleLevel(src, samp, uv + vec2f(-2.0 * t.x, 2.0 * t.y), 0.0).rgb;
  let b = textureSampleLevel(src, samp, uv + vec2f(0.0, 2.0 * t.y), 0.0).rgb;
  let c = textureSampleLevel(src, samp, uv + vec2f(2.0 * t.x, 2.0 * t.y), 0.0).rgb;
  let d = textureSampleLevel(src, samp, uv + vec2f(-2.0 * t.x, 0.0), 0.0).rgb;
  let e = textureSampleLevel(src, samp, uv, 0.0).rgb;
  let f = textureSampleLevel(src, samp, uv + vec2f(2.0 * t.x, 0.0), 0.0).rgb;
  let g = textureSampleLevel(src, samp, uv + vec2f(-2.0 * t.x, -2.0 * t.y), 0.0).rgb;
  let h = textureSampleLevel(src, samp, uv + vec2f(0.0, -2.0 * t.y), 0.0).rgb;
  let i = textureSampleLevel(src, samp, uv + vec2f(2.0 * t.x, -2.0 * t.y), 0.0).rgb;
  let j = textureSampleLevel(src, samp, uv + vec2f(-t.x, t.y), 0.0).rgb;
  let k = textureSampleLevel(src, samp, uv + vec2f(t.x, t.y), 0.0).rgb;
  let l = textureSampleLevel(src, samp, uv + vec2f(-t.x, -t.y), 0.0).rgb;
  let m = textureSampleLevel(src, samp, uv + vec2f(t.x, -t.y), 0.0).rgb;

  var result: vec3f;
  if (B.prefilter == 1u) {
    // Karis-weighted groups on the first downsample only; deeper mips are
    // already smooth and the weighting would just darken them.
    let g0 = (a + b + d + e) * 0.25;
    let g1 = (b + c + e + f) * 0.25;
    let g2 = (d + e + g + h) * 0.25;
    let g3 = (e + f + h + i) * 0.25;
    let g4 = (j + k + l + m) * 0.25;
    let w0 = karisAverage(g0) * 0.125;
    let w1 = karisAverage(g1) * 0.125;
    let w2 = karisAverage(g2) * 0.125;
    let w3 = karisAverage(g3) * 0.125;
    let w4 = karisAverage(g4) * 0.5;
    let wsum = w0 + w1 + w2 + w3 + w4;
    result = (g0 * w0 + g1 * w1 + g2 * w2 + g3 * w3 + g4 * w4) / max(wsum, 1e-5);
    // Soft-knee threshold so the falloff into bloom is gradual.
    let lum = dot(result, vec3f(0.2126, 0.7152, 0.0722));
    let knee = max(1e-4, B.threshold * 0.6);
    let soft = clamp((lum - B.threshold + knee) / (2.0 * knee), 0.0, 1.0);
    let contrib = max(lum - B.threshold, lum * soft * 0.5) / max(lum, 1e-5);
    result = result * clamp(contrib, 0.0, 1.0);
  } else {
    result = e * 0.125;
    result = result + (a + c + g + i) * 0.03125;
    result = result + (b + d + f + h) * 0.0625;
    result = result + (j + k + l + m) * 0.125;
  }

  textureStore(dst, vec2i(gid.xy), vec4f(result, 1.0));
}

@compute @workgroup_size(8, 8, 1)
fn upsample(@builtin(global_invocation_id) gid: vec3u) {
  let dims = vec2u(textureDimensions(dst));
  if (gid.x >= dims.x || gid.y >= dims.y) { return; }
  let uv = (vec2f(gid.xy) + 0.5) / vec2f(dims);
  let t = B.srcTexel;

  // 3x3 tent filter over the smaller mip.
  var s = textureSampleLevel(srcLower, samp, uv + vec2f(-t.x, t.y), 0.0).rgb * 1.0;
  s = s + textureSampleLevel(srcLower, samp, uv + vec2f(0.0, t.y), 0.0).rgb * 2.0;
  s = s + textureSampleLevel(srcLower, samp, uv + vec2f(t.x, t.y), 0.0).rgb * 1.0;
  s = s + textureSampleLevel(srcLower, samp, uv + vec2f(-t.x, 0.0), 0.0).rgb * 2.0;
  s = s + textureSampleLevel(srcLower, samp, uv, 0.0).rgb * 4.0;
  s = s + textureSampleLevel(srcLower, samp, uv + vec2f(t.x, 0.0), 0.0).rgb * 2.0;
  s = s + textureSampleLevel(srcLower, samp, uv + vec2f(-t.x, -t.y), 0.0).rgb * 1.0;
  s = s + textureSampleLevel(srcLower, samp, uv + vec2f(0.0, -t.y), 0.0).rgb * 2.0;
  s = s + textureSampleLevel(srcLower, samp, uv + vec2f(t.x, -t.y), 0.0).rgb * 1.0;
  s = s / 16.0;

  let base = textureSampleLevel(src, samp, uv, 0.0).rgb;
  textureStore(dst, vec2i(gid.xy), vec4f(base + s * B.intensity, 1.0));
}
