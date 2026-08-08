// Composite: albedo * (direct + upsampled indirect) / pi + emissive, then volumetrics → HDR scene colour (rgba16f).
// Exposure / tonemap / bloom / night vision happen in post.wgsl.
@group(1) @binding(0) var gAlbedo: texture_2d<f32>;
@group(1) @binding(1) var gNormal: texture_2d<f32>;
@group(1) @binding(2) var gDepth: texture_depth_2d;
@group(1) @binding(3) var gId: texture_2d<u32>;
@group(1) @binding(4) var directTex: texture_2d<f32>;
@group(1) @binding(5) var indirectTex: texture_2d<f32>;   // 1/frame.lossyCfg.x res (half; a third with gatherThird)
@group(1) @binding(6) var volTex: texture_2d<f32>;        // 1/frame.lossyCfg.y res (half; a quarter with volQuarter)
@group(1) @binding(7) var diceTex: texture_3d<f32>;
@group(1) @binding(8) var softTex: texture_2d<f32>;

const PI: f32 = 3.14159265;

struct VOut { @builtin(position) pos: vec4f, @location(0) uv: vec2f };
@vertex fn vs(@builtin(vertex_index) i: u32) -> VOut {
  var o: VOut;
  let p = vec2f(f32((i << 1u) & 2u), f32(i & 2u));
  o.pos = vec4f(p * 2.0 - 1.0, 0.0, 1.0);
  o.uv = vec2f(p.x, 1.0 - p.y);
  return o;
}

fn linearDepth(d: f32, pix: vec2f) -> f32 { return distance(worldFromDepth(pix, d), frame.camPos); }

fn upsampleIndirect(pix: vec2i, z: f32, N: vec3f) -> vec3f {
  let hdims = vec2i(textureDimensions(indirectTex));
  let gdiv = i32(frame.lossyCfg.x); let goff = (gdiv - 1) / 2;   // gather divisor (2, or 3 with the lossy option) and the full-res offset each texel was computed at (fgather)
  let base = pix / gdiv;                                          // the texel whose sample pixel (gdiv·h + goff) is nearest: floor(p / gdiv) for both divisors
  var acc = vec3f(0.0); var wsum = 0.0;
  for (var dy = -1; dy <= 1; dy++) {
    for (var dx = -1; dx <= 1; dx++) {
      let h = clamp(base + vec2i(dx, dy), vec2i(0), hdims - 1);
      let src = min(h * gdiv + goff, vec2i(frame.screen) - 1);     // guides from exactly the pixel the texel was traced at
      let d = textureLoad(gDepth, src, 0);
      if (d >= 1.0) { continue; }
      let zs = linearDepth(d, vec2f(src) + 0.5);
      let ns = textureLoad(gNormal, src, 0).xyz * 2.0 - 1.0;
      let sp = vec2f(src - pix);
      var w = exp(-dot(sp, sp) * (0.18 * (4.0 / f32(gdiv * gdiv))));   // spatial falloff in units of the texel spacing (0.18 /px² at gdiv 2)
      w *= exp(-abs(zs - z) / max(z * 0.02, 0.02) );
      w *= pow(max(dot(ns, N), 0.0), 8.0) + 0.001;
      acc += textureLoad(indirectTex, h, 0).xyz * w; wsum += w;
    }
  }
  if (wsum < 1e-5) { return textureLoad(indirectTex, clamp(base, vec2i(0), hdims - 1), 0).xyz; }
  return acc / wsum;
}

fn upsampleVol(pix: vec2i, z: f32) -> vec4f {
  let hdims = vec2i(textureDimensions(volTex));
  let vdiv = i32(frame.lossyCfg.y);                             // volumetric divisor: 2, or 4 with the lossy option
  let base = pix / vdiv;
  var acc = vec4f(0.0); var wsum = 0.0;
  for (var dy = -1; dy <= 1; dy++) {
    for (var dx = -1; dx <= 1; dx++) {
      let h = clamp(base + vec2i(dx, dy), vec2i(0), hdims - 1);
      let src = min(h * vdiv + vec2i(h.y & 1, h.x & 1) * (vdiv / 2), vec2i(frame.screen) - 1);   // the pixel that texel marched through (volumetrics.wgsl)
      let d = textureLoad(gDepth, src, 0);
      let zs = select(linearDepth(d, vec2f(src) + 0.5), 1e4, d >= 1.0);
      let sp = vec2f(src - pix);
      var w = exp(-dot(sp, sp) * (0.15 * (4.0 / f32(vdiv * vdiv))));   // spatial falloff in units of the texel spacing (0.15 /px² at vdiv 2)
      w *= exp(-abs(zs - z) / max(z * 0.05, 0.05));
      acc += textureLoad(volTex, h, 0) * w; wsum += w;
    }
  }
  if (wsum < 1e-5) { return textureLoad(volTex, clamp(base, vec2i(0), hdims - 1), 0); }
  return acc / wsum;
}

fn acesFilm(x: vec3f) -> vec3f {
  let a = 2.51; let b = 0.03; let c = 2.43; let d = 0.59; let e = 0.14;
  return clamp((x * (a * x + b)) / (x * (c * x + d) + e), vec3f(0.0), vec3f(1.0));
}
fn linearToSrgb(c: vec3f) -> vec3f {
  return select(1.055 * pow(c, vec3f(1.0 / 2.4)) - 0.055, c * 12.92, c <= vec3f(0.0031308));
}

@fragment fn fs(v: VOut) -> @location(0) vec4f {
  let pixf = v.pos.xy;
  let pix = vec2i(pixf);
  let depth = textureLoad(gDepth, pix, 0);
  var color: vec3f;
  let dbg = frame.debugView;
  if (depth >= 1.0) {
    // background: sky through the view ray
    let Pfar = worldFromDepth(pixf, 1.0);
    color = skyRadiance(normalize(Pfar - frame.camPos));
    if (dbg == 0u || dbg == 5u) {
      let vol = upsampleVol(pix, 1e4);
      color = color * vol.a + vol.rgb;
    }
  } else {
    let albedo = textureLoad(gAlbedo, pix, 0).rgb;
    let N = normalize(textureLoad(gNormal, pix, 0).xyz * 2.0 - 1.0);
    let id = textureLoad(gId, pix, 0).x;
    let boxIdx = id & 0xFFFFFu;
    let z = linearDepth(depth, pixf);
    let Ed = textureLoad(directTex, pix, 0).rgb + textureLoad(softTex, pix, 0).rgb;
    let Ei = upsampleIndirect(pix, z, N);
    var emissive = vec3f(0.0);
    if (((id >> 28u) & 1u) == 0u && boxIdx != 0xFFFFFu) {
      let b = boxGeo[boxIdx];
      if ((b.flags & BOX_EMISSIVE) != 0u) { emissive = boxEmissive(boxIdx); }
    }
    color = albedo * (Ed + Ei) * (1.0 / PI) + emissive;
    if (((id >> 29u) & 1u) == 1u) { color = mix(frame.capColor.rgb, albedo * frame.capColor.rgb * 4.0, frame.capColor.a); }
    if (dbg == 1u) { color = albedo; }
    else if (dbg == 2u) { color = N * 0.5 + 0.5; }
    else if (dbg == 3u) { color = Ed * (1.0 / PI); }
    else if (dbg == 4u) { color = Ei * (1.0 / PI); }
    else if (dbg == 6u) { color = vec3f(1.0 - exp(-z * 0.06)); }   // view distance, monotonic (white = far)
    else if (dbg == 7u) { let P = worldFromDepth(pixf, depth); color = diceIrradiance(P + N * 0.27, N) * (1.0 / PI); }
    else if (dbg == 8u) { color = (Ed + Ei) * (1.0 / PI); }
    else if (dbg == 9u) { color = vec3f(abs(textureLoad(directTex, pix, 0).a), abs(textureLoad(softTex, pix, 0).a), 0.0); }   // denoiser blur hints (r sharp, g broad)
    if (dbg == 0u || dbg == 5u) {
      let vol = upsampleVol(pix, z);
      color = select(color * vol.a + vol.rgb, vol.rgb, dbg == 5u);
    }
  }
  return vec4f(max(color, vec3f(0.0)), 1.0);   // scene-referred; post.wgsl takes it from here (debug views 1/2/6/9 are passed through untouched there)
}
