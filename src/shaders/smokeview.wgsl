// ===========================================================================
// Direct volume raymarch of the fluid density field.
//
// A deliberate shortcut, and the point of it is iteration speed. The path
// tracer renders smoke properly — as a participating medium inside a full GI
// solve — but that costs ~14 compute passes a frame, which measured at 2.9 s
// per frame under software rasterisation even with bounces at zero. Tuning
// smoke DYNAMICS needs dozens of looks at the same second of simulation, and a
// loop that slow is not a loop.
//
// It is also not only about speed. The tracer's output goes through temporal
// accumulation and an a-trous filter whose entire job is to remove
// high-frequency spatial detail — which is exactly the wispy structure that
// tells you whether the solver is producing smoke or porridge. Judging
// dynamics through a denoiser means tuning detail in and watching it get
// filtered back out.
//
// So: one fragment shader, one ray per pixel, Beer-Lambert along it, and a
// short secondary march toward the light for self-shadowing. Nothing here is
// physically defensible next to the tracer and nothing here is meant to ship.
// It shows the density field honestly and it runs in milliseconds.
// ===========================================================================

struct View {
  invViewProj : mat4x4f,
  camPos      : vec3f,
  /** Marching steps through the box. */
  steps       : f32,
  boxMin      : vec3f,
  /** Steps toward the light per sample; 0 disables self-shadowing. */
  shadowSteps : f32,
  boxMax      : vec3f,
  /** Extinction per unit density per metre. */
  absorption  : f32,
  lightDir    : vec3f,
  /** How much a lit sample scatters back toward the eye. */
  scatter     : f32,
  lightColor  : vec3f,
  ambient     : f32,
  /** Background gradient, top and bottom. */
  skyTop      : vec3f,
  exposure    : f32,
  skyBottom   : vec3f,
  /** Multiplies the sampled density; the fast way to make it read thicker. */
  densityScale: f32,
  /** The flashbang, as a point light inside the medium. */
  flashPos    : vec3f,
  /** Peak radiance times the decay envelope; 0 when no flash is live. */
  flashPower  : f32,
  flashColor  : vec3f,
  /** Steps from a sample toward the flash; 0 gives an unshadowed glow. */
  flashShadow : f32,
}

@group(0) @binding(0) var<uniform> V : View;
@group(0) @binding(1) var densityTex : texture_3d<f32>;
@group(0) @binding(2) var samp : sampler;

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

/** Slab test. Returns (tEnter, tExit); tExit < tEnter means a miss. */
fn boxRange(ro: vec3f, rd: vec3f) -> vec2f {
  let inv = 1.0 / rd;
  let a = (V.boxMin - ro) * inv;
  let b = (V.boxMax - ro) * inv;
  let lo = min(a, b);
  let hi = max(a, b);
  let tn = max(max(lo.x, lo.y), lo.z);
  let tf = min(min(hi.x, hi.y), hi.z);
  return vec2f(max(tn, 0.0), tf);
}

fn densityAt(p: vec3f) -> f32 {
  let uvw = (p - V.boxMin) / (V.boxMax - V.boxMin);
  // Trilinear. The grid is the thing being judged, so nothing is added to it
  // here — no procedural detail, no noise. What you see is what the solver did.
  return max(textureSampleLevel(densityTex, samp, uvw, 0.0).r, 0.0) * V.densityScale;
}

/**
 * Transmittance from `p` toward the light.
 *
 * Few steps on purpose: this is the difference between a flat grey lump and
 * something with a lit rim and a shadowed core, and beyond about eight taps
 * the extra ones stop changing the read.
 */
fn shadow(p: vec3f) -> f32 {
  let n = i32(V.shadowSteps);
  if (n <= 0) { return 1.0; }
  let r = boxRange(p, V.lightDir);
  if (r.y <= r.x) { return 1.0; }
  let dt = (r.y - r.x) / f32(n);
  var tau = 0.0;
  for (var i = 0; i < n; i = i + 1) {
    let q = p + V.lightDir * (r.x + (f32(i) + 0.5) * dt);
    tau = tau + densityAt(q) * dt;
  }
  return exp(-tau * V.absorption);
}

/**
 * Radiance reaching `p` from the flash.
 *
 * A point light *inside* the medium rather than the key light's directional
 * one, and that difference is the whole effect: the falloff is inverse-square
 * from a point a metre away, so the near face of the cloud is blown out while
 * the far side is barely touched, and the smoke between them casts its own
 * shadow across itself. A directional light cannot do any of that, which is
 * why a flashbang lit like the sun reads as a lamp switching on.
 */
fn flashAt(p: vec3f) -> vec3f {
  if (V.flashPower <= 0.0) { return vec3f(0.0); }
  let toL = V.flashPos - p;
  let d2 = max(dot(toL, toL), 0.02);
  let dist = sqrt(d2);
  let dir = toL / dist;

  var tau = 0.0;
  let n = i32(V.flashShadow);
  if (n > 0) {
    // Marched to the light, not to the box wall: the source is inside the
    // volume, so the occluding span is exactly the distance between them.
    let dt = dist / f32(n);
    for (var i = 0; i < n; i = i + 1) {
      tau = tau + densityAt(p + dir * ((f32(i) + 0.5) * dt)) * dt;
    }
  }
  return V.flashColor * (V.flashPower / d2) * exp(-tau * V.absorption);
}

/** Cheap per-pixel dither, so the march's step boundaries do not band. */
fn hash12(p: vec2f) -> f32 {
  var q = fract(vec3f(p.xyx) * 0.1031);
  q = q + dot(q, q.yzx + 33.33);
  return fract((q.x + q.y) * q.z);
}

@fragment
fn fs(in: VSOut) -> @location(0) vec4f {
  let ndc = vec4f(in.uv.x * 2.0 - 1.0, 1.0 - in.uv.y * 2.0, 1.0, 1.0);
  let world = V.invViewProj * ndc;
  // `target` is a WGSL reserved keyword — naming it that compiled to an error
  // whose only visible symptom was the fluid doing nothing, because an invalid
  // pipeline invalidates the whole command buffer and the solver shared it.
  let farPt = world.xyz / world.w;
  let rd = normalize(farPt - V.camPos);
  let ro = V.camPos;

  // Background: a vertical gradient, so the cloud has something to sit against
  // and its silhouette is readable at the top and the bottom of frame.
  let sky = mix(V.skyBottom, V.skyTop, clamp(rd.y * 0.5 + 0.5, 0.0, 1.0));

  let r = boxRange(ro, rd);
  if (r.y <= r.x) { return vec4f(sky, 1.0); }

  let n = i32(V.steps);
  let dt = (r.y - r.x) / f32(n);
  // Jitter the entry point by one step, or the first sample plane is a visible
  // shell wherever the box face crosses the cloud.
  let jitter = hash12(in.pos.xy);

  var transmit = 1.0;
  var lit = vec3f(0.0);
  for (var i = 0; i < n; i = i + 1) {
    let t = r.x + (f32(i) + jitter) * dt;
    if (t > r.y) { break; }
    let p = ro + rd * t;
    let dens = densityAt(p);
    if (dens <= 0.001) { continue; }

    let sigma = dens * V.absorption;
    // Emission-absorption with a single scattering approximation: what leaves
    // this sample toward the eye is the light reaching it, times how much of
    // the ray's remaining transmittance it gets to use.
    let sh = shadow(p);
    let energy = V.lightColor * (sh * V.scatter) + flashAt(p) + vec3f(V.ambient);
    let a = 1.0 - exp(-sigma * dt);
    lit = lit + transmit * a * energy;
    transmit = transmit * (1.0 - a);
    if (transmit < 0.003) { break; }
  }

  let col = sky * transmit + lit;
  // Reinhard rather than AgX: this view is not trying to match the game's
  // grade, it is trying to show the field. A curve that rolls highlights off
  // hard would hide exactly the density differences being judged.
  let mapped = vec3f(1.0) - exp(-col * V.exposure);
  return vec4f(pow(mapped, vec3f(1.0 / 2.2)), 1.0);
}
