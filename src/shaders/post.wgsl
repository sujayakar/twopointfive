// ===========================================================================
// Final composite: bloom, AgX tonemap, grade, vignette, grain, crosshair.
// ===========================================================================

struct PostParams {
  resolution     : vec2f,
  mouse          : vec2f,
  exposure       : f32,
  bloomIntensity : f32,
  vignette       : f32,
  grain          : f32,
  time           : f32,
  debugView      : u32,
  chromatic      : f32,
  saturation     : f32,
  nightVision    : f32,
  nvGain         : f32,
  /** 0 = white phosphor (P45), 1 = classic green (P43). */
  nvPhosphor     : f32,
  _pad0          : f32,
}

@group(0) @binding(0) var samp : sampler;
@group(0) @binding(1) var hdrTex : texture_2d<f32>;
@group(0) @binding(2) var bloomTex : texture_2d<f32>;
@group(0) @binding(3) var<uniform> P : PostParams;

struct VSOut {
  @builtin(position) pos : vec4f,
  @location(0) uv : vec2f,
}

@vertex
fn vs(@builtin(vertex_index) vi: u32) -> VSOut {
  // Oversized triangle covering the viewport — no vertex buffer needed.
  var p = array<vec2f, 3>(vec2f(-1.0, -1.0), vec2f(3.0, -1.0), vec2f(-1.0, 3.0));
  var o: VSOut;
  o.pos = vec4f(p[vi], 0.0, 1.0);
  o.uv = vec2f(p[vi].x * 0.5 + 0.5, 0.5 - p[vi].y * 0.5);
  return o;
}

// --- AgX (Troy Sobotka), the 6th-order contrast approximation ---------------

const AGX_IN = mat3x3f(
  vec3f(0.842479062253094, 0.0423282422610123, 0.0423756549057051),
  vec3f(0.0784335999999992, 0.878468636469772, 0.0784336000000000),
  vec3f(0.0792237451477643, 0.0791661274605434, 0.879142973793104),
);

const AGX_OUT = mat3x3f(
  vec3f(1.19687900512017, -0.0528968517574562, -0.0529716355144438),
  vec3f(-0.0980208811401368, 1.15190312990417, -0.0980434501171241),
  vec3f(-0.0990297440797205, -0.0989611768448433, 1.15107367264116),
);

fn agxContrast(x: vec3f) -> vec3f {
  let x2 = x * x;
  let x4 = x2 * x2;
  return 15.5 * x4 * x2 - 40.14 * x4 * x + 31.96 * x4 - 6.868 * x2 * x
       + 0.4298 * x2 + 0.1191 * x - 0.00232;
}

fn agx(colIn: vec3f) -> vec3f {
  let minEv = -12.47393;
  let maxEv = 4.026069;
  var col = AGX_IN * max(colIn, vec3f(0.0));
  col = clamp(log2(max(col, vec3f(1e-10))), vec3f(minEv), vec3f(maxEv));
  col = (col - minEv) / (maxEv - minEv);
  col = agxContrast(col);
  col = AGX_OUT * col;
  return max(col, vec3f(0.0));
}

fn hash13(p: vec3f) -> f32 {
  var q = fract(p * 0.1031);
  q = q + dot(q, q.zyx + 31.32);
  return fract((q.x + q.y) * q.z);
}

/**
 * Image-intensifier response.
 *
 * A night vision tube is not a green filter over the normal image. It is a
 * single-channel photon counter with enormous gain and a logarithmic response,
 * so it lifts near-black detail into view while everything already bright slams
 * into the same ceiling. The characteristic noise is scintillation — individual
 * photon events — which is loudest where there is least signal, so the grain
 * has to scale with darkness rather than being applied uniformly.
 */
fn nightVisionTube(hdr: vec3f, px: vec2f, radial: f32, t: f32) -> vec3f {
  let lum = max(dot(hdr, vec3f(0.2126, 0.7152, 0.0722)), 0.0);

  // Log gain curve, normalised so full scale stays at 1.
  let g = max(P.nvGain, 1.0);
  var signal = log(1.0 + lum * g) / log(1.0 + g);

  // Scintillation, weighted toward the low end of the signal.
  let n = hash13(vec3f(px, floor(t * 24.0))) - 0.5;
  signal = signal + n * 0.16 * (1.0 - signal * 0.85);

  // Coarse horizontal banding from the intensifier's scan.
  signal = signal * (1.0 - 0.05 * fract(px.y * 0.5));

  signal = clamp(signal, 0.0, 1.0);

  // Phosphor. White (P45) is the modern tube and reads as a cool neutral grey;
  // green (P43) is the classic. Highlights push toward white on both as the
  // tube blooms out.
  let white = vec3f(0.80, 0.87, 1.0);
  let green = vec3f(0.13, 1.0, 0.32);
  let phosphor = mix(white, green, clamp(P.nvPhosphor, 0.0, 1.0));
  let hot = mix(vec3f(1.0, 1.0, 1.0), vec3f(0.55, 1.0, 0.65), clamp(P.nvPhosphor, 0.0, 1.0));

  var col = phosphor * signal;
  col = col + hot * smoothstep(0.75, 1.0, signal) * 0.55;

  // Hard tube edge — this is the eyepiece, not a cinematic vignette.
  col = col * (1.0 - smoothstep(0.62, 0.95, radial));
  return col;
}

@fragment
fn fs(in: VSOut) -> @location(0) vec4f {
  let uv = in.uv;

  // Slight lateral chromatic aberration toward the frame edges — cheap, and it
  // does a lot to stop the image reading as clinically synthetic.
  let toCenter = uv - vec2f(0.5);
  let ca = P.chromatic * dot(toCenter, toCenter);
  var hdr: vec3f;
  if (ca > 1e-6) {
    hdr = vec3f(
      textureSampleLevel(hdrTex, samp, uv - toCenter * ca, 0.0).r,
      textureSampleLevel(hdrTex, samp, uv, 0.0).g,
      textureSampleLevel(hdrTex, samp, uv + toCenter * ca, 0.0).b,
    );
  } else {
    hdr = textureSampleLevel(hdrTex, samp, uv, 0.0).rgb;
  }

  let bloom = textureSampleLevel(bloomTex, samp, uv, 0.0).rgb;
  var color = hdr + bloom * P.bloomIntensity;
  let d = length(toCenter * vec2f(P.resolution.x / P.resolution.y, 1.0));

  if (P.debugView != 0u) {
    // Debug views bypass tonemapping so values are read literally.
    color = clamp(color, vec3f(0.0), vec3f(1.0));
  } else if (P.nightVision > 0.5) {
    // The tube supplies its own response curve, noise and edge falloff, so it
    // replaces AgX and the film grain rather than stacking on top of them.
    color = nightVisionTube(color * P.exposure, in.pos.xy, d, P.time);
  } else {
    color = color * P.exposure;
    color = agx(color);
    // Grade: gentle saturation lift keeps the blues and the warm beam distinct
    // after AgX's desaturating shoulder.
    let luma = dot(color, vec3f(0.2126, 0.7152, 0.0722));
    color = mix(vec3f(luma), color, P.saturation);

    color = color * (1.0 - P.vignette * smoothstep(0.35, 1.05, d));

    // Film grain, scaled down in highlights where it would look like noise.
    let g = hash13(vec3f(in.pos.xy, P.time * 60.0)) - 0.5;
    let lum = dot(color, vec3f(0.2126, 0.7152, 0.0722));
    color = color + g * P.grain * (1.0 - smoothstep(0.0, 0.8, lum));
  }

  // Crosshair at the aim point.
  let px = uv * P.resolution;
  let toMouse = abs(px - P.mouse);
  let onCross = (toMouse.x < 1.0 && toMouse.y < 9.0 && toMouse.y > 3.0)
             || (toMouse.y < 1.0 && toMouse.x < 9.0 && toMouse.x > 3.0);
  if (onCross) { color = color + vec3f(0.30); }

  return vec4f(color, 1.0);
}
