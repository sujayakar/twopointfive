// Post: HDR scene colour + bloom → exposure → AgX tonemap (+ a mild 'punchy' look) → night vision / hit flash /
// dither → LDR. Debug views that are not scene-referred (albedo, normals, depth, hints) pass straight through.
@group(1) @binding(0) var hdrTex: texture_2d<f32>;
@group(1) @binding(1) var bloomTex: texture_2d<f32>;     // half-res, top of the upsampled chain
struct PostParams { bloom: f32, saturation: f32, _unused: f32, nvPhosphor: f32 };
@group(1) @binding(2) var<uniform> post: PostParams;

struct VOut { @builtin(position) pos: vec4f, @location(0) uv: vec2f };
@vertex fn vs(@builtin(vertex_index) i: u32) -> VOut {
  var o: VOut;
  let p = vec2f(f32((i << 1u) & 2u), f32(i & 2u));
  o.pos = vec4f(p * 2.0 - 1.0, 0.0, 1.0);
  o.uv = vec2f(p.x, 1.0 - p.y);
  return o;
}

fn linearToSrgb(c: vec3f) -> vec3f {
  return select(1.055 * pow(c, vec3f(1.0 / 2.4)) - 0.055, c * 12.92, c <= vec3f(0.0031308));
}

// AgX (Sobotka) via the widely used polynomial fit of its base contrast curve; input linear Rec.709, output display sRGB-ish
fn agxContrast(x: vec3f) -> vec3f {
  let x2 = x * x; let x4 = x2 * x2;
  return 15.5 * x4 * x2 - 40.14 * x4 * x + 31.96 * x4 - 6.868 * x2 * x + 0.4298 * x2 + 0.1191 * x - 0.00232;
}
fn agx(cin: vec3f) -> vec3f {
  let inset = mat3x3f(vec3f(0.842479062253094, 0.0423282422610123, 0.0423756549057051),
                      vec3f(0.0784335999999992, 0.878468636469772, 0.0784336),
                      vec3f(0.0792237451477643, 0.0791661274605434, 0.879142973793104));
  let minEv = -12.47393; let maxEv = 4.026069;
  var v = inset * max(cin, vec3f(1e-10));
  v = clamp(log2(v), vec3f(minEv), vec3f(maxEv));
  v = (v - minEv) / (maxEv - minEv);
  v = agxContrast(v);
  // look: a touch of punch so mid-tones do not go flat/grey (slope 1, power 1.15, sat from params)
  let lum = dot(v, vec3f(0.2126, 0.7152, 0.0722));
  v = pow(v, vec3f(1.15));
  v = lum + post.saturation * (v - lum);
  let outset = mat3x3f(vec3f(1.19687900512017, -0.0528968517574562, -0.0529716355144438),
                       vec3f(-0.0980208811401368, 1.15190312990417, -0.0980434501171241),
                       vec3f(-0.0990297440797205, -0.0989611768448433, 1.15107367264116));
  v = outset * v;
  return clamp(v, vec3f(0.0), vec3f(1.0));          // already display-encoded (~2.2)
}

@fragment fn fs(v: VOut) -> @location(0) vec4f {
  let pixf = floor(v.pos.xy) + 0.5;
  let pix = vec2i(pixf);
  let hdr = textureLoad(hdrTex, pix, 0);
  let dbg = frame.debugView;
  if (dbg == 1u || dbg == 2u || dbg == 6u || dbg == 9u) { return vec4f(linearToSrgb(clamp(hdr.rgb, vec3f(0.0), vec3f(1.0))), 1.0); }
  var color = hdr.rgb;
  if (post.bloom > 0.0 && dbg == 0u) { color += textureSampleLevel(bloomTex, linSamp, v.uv, 0.0).rgb * post.bloom; }
  var c: vec3f;
  if ((frame.flags & FLAG_NIGHTVISION) != 0u && frame.post.x > 0.5) {
    // image-intensifier tube (after twopointfive): a single-channel photon counter with huge log gain — near-black detail
    // lifts into view, anything bright slams into the ceiling; scintillation noise loudest where the signal is weakest;
    // faint row banding from the scan; white phosphor (P45) with a touch of green; hard eyepiece edge. Uses the bloomed
    // HDR colour so light sources still flare.
    let lum = max(dot(color * frame.exposure, vec3f(0.2126, 0.7152, 0.0722)), 0.0);
    let g = max(frame.post.y, 1.0);
    var sig = log(1.0 + lum * g) / log(1.0 + g);
    let n = hash13(vec3f(pixf, floor(frame.time * 24.0))) - 0.5;
    sig = sig + n * 0.16 * (1.0 - sig * 0.85);
    sig = sig * (1.0 - 0.05 * fract(pixf.y * 0.5));
    sig = clamp(sig, 0.0, 1.0);
    let phos = clamp(post.nvPhosphor, 0.0, 1.0);
    let phosphor = mix(vec3f(0.80, 0.87, 1.0), vec3f(0.13, 1.0, 0.32), phos);
    let hot = mix(vec3f(1.0), vec3f(0.55, 1.0, 0.65), phos);
    var col = phosphor * sig + hot * smoothstep(0.75, 1.0, sig) * 0.55;
    let radial = distance(pixf * frame.invScreen, vec2f(0.5)) * 2.0;
    col = col * (1.0 - smoothstep(0.62, 0.95, radial));
    c = linearToSrgb(clamp(col, vec3f(0.0), vec3f(1.0)));
  } else {
    c = agx(color * frame.exposure);   // (film grain is added after FXAA, in the final blit, so the AA does not smooth it away)
  }
  if (frame.post.z > 0.0) { c = mix(c, vec3f(0.6, 0.02, 0.02), frame.post.z * 0.35 * smoothstep(0.2, 0.9, distance(pixf * frame.invScreen, vec2f(0.5)) * 1.4)); }
  if ((frame.flags & FLAG_DITHER) != 0u) {
    let n = ign(pixf) + ign(pixf + vec2f(17.0, 59.0)) - 1.0; // triangular
    c += vec3f(n / 255.0);
  }
  return vec4f(clamp(c, vec3f(0.0), vec3f(1.0)), 1.0);
}
