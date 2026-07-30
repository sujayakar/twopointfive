// ===========================================================================
// Radiosity: bake and per-frame solve.
//
// Diffuse-only materials over static boxes make classical radiosity exact,
// so steady-state indirect light is solved on a few thousand face patches
// instead of estimated per pixel. See src/scene/radiosity.ts for how the
// patches are diced.
//
// Bake (once, at load): dense patch-to-patch form factors with traced
// visibility, per-(patch, static light) soft visibility, per-patch sky-dome
// irradiance. Per frame: inject direct light into the patches (baked tables
// + torch depth maps, zero rays), then two warm-started Jacobi steps whose
// gather lands in a texture the trace pass reads per pixel. Bounces deepen
// across frames — infinite-bounce with a frame of lag per bounce.
//
// Storage-buffer budget forces the layout: Apple GPUs cap a stage at 10
// storage buffers and the scene group already uses 6, so everything static
// lives in ONE packed f32 buffer and everything per-frame in another, with
// offsets derived from the counts. The per-pixel consumers (trace pass) read
// textures instead, which cost nothing against that limit:
//
//   radStatic (f32): [patches 16N][skyE 4N][vis N*S][ff N*N]
//   radDyn    (f32): [injectE 4N][b0 4N][b1 4N]
//
// The B ping-pong is two tiny uniform blocks naming bIn/bOut offsets — the
// two bind groups differ only in which block they carry. Each B slot's 4th
// lane is spare padding; the solve parks the patch's RIS weight there for the
// CDF pass. The trace pass reads patch geometry from radStatic directly
// (read-only) once patches become sample-able emitters — see patchRIS.
//
// Bake kernels run at init, BEFORE the first frame writes the shared
// uniforms; every U.* field is zero then. Bake inputs must come through
// RadParams, and occluded()'s dynamic-group count being zero is exactly
// right — characters must not be baked into permanent shadows.
// ===========================================================================

struct RadParams {
  count  : u32,
  /** Static light count — U.dynLightStart is not yet written at bake time. */
  lights : u32,
  /** f32 offsets of this pass's B-in / B-out in radDyn. */
  bIn    : u32,
  bOut   : u32,
}

@group(1) @binding(0) var<uniform> P : RadParams;
@group(1) @binding(1) var<storage, read_write> radStatic : array<f32>;
@group(1) @binding(2) var<storage, read_write> radDyn : array<f32>;
@group(1) @binding(3) var torchDepth : texture_2d_array<f32>;
/**
 * Row 0: gathered indirect irradiance G. Row 1: sky irradiance (unit sky).
 * Row 2: outgoing radiosity B (sky term folded in) — what a patch emits as a
 * virtual light. Row 3: inclusive CDF over luminance(B)*area, for sampling
 * patches in proportion to how much they shine (buildPatchCdf).
 */
@group(1) @binding(4) var radGSkyOut : texture_storage_2d<rgba32float, write>;

fn skyOff() -> u32 { return 16u * P.count; }
fn visOff() -> u32 { return 20u * P.count; }
fn ffOff() -> u32 { return 20u * P.count + P.count * P.lights; }

struct Patch {
  pos  : vec3f,
  area : f32,
  n    : vec3f,
  mat  : u32,
  tu   : vec3f,
  hu   : f32,
  tv   : vec3f,
  hv   : f32,
}

fn patchAt(i: u32) -> Patch {
  let b = i * 16u;
  var p: Patch;
  p.pos = vec3f(radStatic[b], radStatic[b + 1u], radStatic[b + 2u]);
  p.area = radStatic[b + 3u];
  p.n = vec3f(radStatic[b + 4u], radStatic[b + 5u], radStatic[b + 6u]);
  p.mat = u32(radStatic[b + 7u]);
  p.tu = vec3f(radStatic[b + 8u], radStatic[b + 9u], radStatic[b + 10u]);
  p.hu = radStatic[b + 11u];
  p.tv = vec3f(radStatic[b + 12u], radStatic[b + 13u], radStatic[b + 14u]);
  p.hv = radStatic[b + 15u];
  return p;
}

fn dynLoad3(base: u32, i: u32) -> vec3f {
  let o = base + i * 4u;
  return vec3f(radDyn[o], radDyn[o + 1u], radDyn[o + 2u]);
}

fn dynStore3(base: u32, i: u32, v: vec3f) {
  let o = base + i * 4u;
  radDyn[o] = v.x;
  radDyn[o + 1u] = v.y;
  radDyn[o + 2u] = v.z;
}

/**
 * 4-tap PCF depth compare against a torch layer — the same taps and slack as
 * the trace pass's torchMapSample, so a patch shadow edge is a texel-wide
 * ramp rather than a single-texel step across a metre of floor.
 */
fn torchVisPoint(layer: u32, lpos: vec3f, axis: vec3f, cosOuter: f32, p: vec3f) -> f32 {
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
    let d = textureLoad(torchDepth, c, i32(layer), 0).r;
    // Relative + absolute slack, matching the image path: a receiver that IS
    // the stored surface must compare visible against its own depth sample.
    vis = vis + w[i] * select(0.0, 1.0, r <= d * 1.02 + 0.10);
  }
  return vis;
}

// ---------------------------------------------------------------------------
// Bake
// ---------------------------------------------------------------------------

@compute @workgroup_size(8, 8, 1)
fn bakeFF(@builtin(global_invocation_id) gid: vec3u) {
  let i = gid.y;
  let j = gid.x;
  if (i >= P.count || j >= P.count) { return; }
  let o = ffOff() + i * P.count + j;
  if (i == j) { radStatic[o] = 0.0; return; }

  seedRng(gid.xy, 7u);
  let pi = patchAt(i);
  let pj = patchAt(j);

  // Four samples with BOTH endpoints jittered over their patches. A fixed
  // receiver point misses most of the transfer between adjacent
  // floor-and-wall patches — exactly the pairs that dominate indoor bounce —
  // and measured as a systematic energy loss (relBias -0.25 vs the tracer's
  // -0.15 against a 4-bounce reference). The point-to-disk kernel (area in
  // the denominator) keeps close parallel patches finite where the raw
  // point-to-point form factor diverges.
  var sum = 0.0;
  for (var s = 0u; s < 4u; s = s + 1u) {
    let src = pi.pos
      + pi.tu * ((rand() * 2.0 - 1.0) * pi.hu)
      + pi.tv * ((rand() * 2.0 - 1.0) * pi.hv);
    let dst = pj.pos
      + pj.tu * ((rand() * 2.0 - 1.0) * pj.hu)
      + pj.tv * ((rand() * 2.0 - 1.0) * pj.hv);
    let delta = dst - src;
    let d2 = dot(delta, delta);
    if (d2 < 1e-4) { continue; }
    let dist = sqrt(d2);
    let dir = delta / dist;
    let ci = dot(pi.n, dir);
    let cj = dot(pj.n, -dir);
    if (ci <= 0.0 || cj <= 0.0) { continue; }
    if (occluded(src + pi.n * EPS * 8.0, dir, dist - EPS * 16.0)) { continue; }
    sum = sum + ci * cj * pj.area / (PI * d2 + pj.area * 0.25);
  }
  radStatic[o] = sum * 0.25;
}

/**
 * Clamp each row's gather to <= 0.98. A row that gathers more than unity
 * (point-sample bias on close parallel faces) would make Jacobi amplify
 * instead of converge.
 */
@compute @workgroup_size(64, 1, 1)
fn bakeNorm(@builtin(global_invocation_id) gid: vec3u) {
  let i = gid.x;
  if (i >= P.count) { return; }
  let base = ffOff() + i * P.count;
  var sum = 0.0;
  for (var j = 0u; j < P.count; j = j + 1u) { sum = sum + radStatic[base + j]; }
  if (sum > 0.98) {
    let k = 0.98 / sum;
    for (var j = 0u; j < P.count; j = j + 1u) {
      radStatic[base + j] = radStatic[base + j] * k;
    }
  }
}

/**
 * Static lights never move, so their patch shadowing is a table forever
 * after. Intensity stays live at inject — a shot-out light goes dark in the
 * same frame everywhere.
 */
@compute @workgroup_size(64, 1, 1)
fn bakeVis(@builtin(global_invocation_id) gid: vec3u) {
  let i = gid.x;
  let li = gid.y;
  if (i >= P.count || li >= P.lights) { return; }
  seedRng(gid.xy, 11u);
  let p = patchAt(i);
  let ro = p.pos + p.n * EPS * 8.0;
  var vis = 0.0;
  for (var s = 0u; s < 4u; s = s + 1u) {
    let smp = sampleSphereLight(lights[li], ro);
    if (dot(p.n, smp.dir) <= 0.0) { continue; }
    if (!occluded(ro, smp.dir, smp.dist - EPS * 8.0)) { vis = vis + 0.25; }
  }
  radStatic[visOff() + i * P.lights + li] = vis;
}

@compute @workgroup_size(64, 1, 1)
fn bakeSky(@builtin(global_invocation_id) gid: vec3u) {
  let i = gid.x;
  if (i >= P.count) { return; }
  seedRng(vec2u(i, 3u), 13u);
  let p = patchAt(i);
  let ro = p.pos + p.n * EPS * 8.0;
  var acc = vec3f(0.0);
  for (var s = 0u; s < 16u; s = s + 1u) {
    let dir = cosineHemisphere(p.n, rand2());
    if (occluded(ro, dir, RAY_MAX)) { continue; }
    // skyRadiance() minus its U.skyIntensity factor, which is zero at bake
    // time — the shape is baked, the intensity is applied at read.
    let up = clamp(dir.y * 0.5 + 0.5, 0.0, 1.0);
    acc = acc + mix(vec3f(0.030, 0.042, 0.075), vec3f(0.045, 0.070, 0.135), up);
  }
  // Cosine-weighted, so irradiance = pi * mean. Written twice: the buffer
  // copy feeds inject, the texture row feeds the trace pass.
  let e = acc * (PI / 16.0);
  let b = skyOff() + i * 4u;
  radStatic[b] = e.x;
  radStatic[b + 1u] = e.y;
  radStatic[b + 2u] = e.z;
  textureStore(radGSkyOut, vec2i(i32(i), 1), vec4f(e, 0.0));
}

// ---------------------------------------------------------------------------
// Per frame
// ---------------------------------------------------------------------------

@compute @workgroup_size(64, 1, 1)
fn inject(@builtin(global_invocation_id) gid: vec3u) {
  let i = gid.x;
  if (i >= P.count) { return; }
  let p = patchAt(i);
  var e = vec3f(0.0);

  // Static lights: baked visibility, live intensity.
  for (var li = 0u; li < P.lights; li = li + 1u) {
    let l = lights[li];
    if (l.intensity <= 0.0) { continue; }
    let v = radStatic[visOff() + i * P.lights + li];
    if (v <= 0.0) { continue; }
    let delta = l.pos - p.pos;
    let d2 = max(dot(delta, delta), 1e-4);
    let dir = delta * inverseSqrt(d2);
    let c = dot(p.n, dir);
    if (c <= 0.0) { continue; }
    var atten = 1.0;
    if (l.kind == LIGHT_SPOT) {
      atten = spotAttenuation(l.dir, -dir, l.cosInner, l.cosOuter);
      if (atten <= 0.0) { continue; }
    }
    e = e + l.color * (l.intensity * falloff(d2) * c * atten * v);
  }

  // The player's torch, from its depth-map layer.
  if (U.flashIntensity > 0.0) {
    let delta = U.flashPos - p.pos;
    let d2 = max(dot(delta, delta), 1e-4);
    let dir = delta * inverseSqrt(d2);
    let c = dot(p.n, dir);
    let cone = spotAttenuation(U.flashDir, -dir, U.flashCosInner, U.flashCosOuter);
    if (c > 0.0 && cone > 0.0) {
      let v = torchVisPoint(0u, U.flashPos, U.flashDir, U.flashCosOuter, p.pos);
      e = e + U.flashColor * (U.flashIntensity * falloff(d2) * c * cone * v);
    }
  }

  // Guard torches, from theirs.
  for (var li = U.dynLightStart; li < U.transientStart; li = li + 1u) {
    let l = lights[li];
    if (l.intensity <= 0.0 || l.kind != LIGHT_SPOT) { continue; }
    let layer = li - U.dynLightStart + 1u;
    if (layer >= TORCH_LAYERS) { continue; }
    let delta = l.pos - p.pos;
    let d2 = max(dot(delta, delta), 1e-4);
    let dir = delta * inverseSqrt(d2);
    let c = dot(p.n, dir);
    if (c <= 0.0) { continue; }
    let cone = spotAttenuation(l.dir, -dir, l.cosInner, l.cosOuter);
    if (cone <= 0.0) { continue; }
    let v = torchVisPoint(layer, l.pos, l.dir, l.cosOuter, p.pos);
    e = e + l.color * (l.intensity * falloff(d2) * c * cone * v);
  }

  dynStore3(0u, i, e);
}

@compute @workgroup_size(64, 1, 1)
fn solve(@builtin(global_invocation_id) gid: vec3u) {
  let i = gid.x;
  if (i >= P.count) { return; }
  let base = ffOff() + i * P.count;
  var g = vec3f(0.0);
  for (var j = 0u; j < P.count; j = j + 1u) {
    g = g + radStatic[base + j] * dynLoad3(P.bIn, j);
  }
  textureStore(radGSkyOut, vec2i(i32(i), 0), vec4f(g, 0.0));
  let p = patchAt(i);
  let alb = materials[p.mat].albedo;
  dynStore3(P.bOut, i, alb * (dynLoad3(0u, i) + g));

  // Row 2: the same B plus the sky-lit term the transport solve leaves to
  // read time — as an emitter the patch shines with everything it reflects.
  // The transport state above stays sky-free, unchanged. Its RIS weight
  // (luminance x area) rides the free w lane of the bOut slot for the CDF.
  let so = skyOff() + i * 4u;
  let sky = vec3f(radStatic[so], radStatic[so + 1u], radStatic[so + 2u]) * U.skyIntensity;
  let bOut = alb * (dynLoad3(0u, i) + g + sky);
  textureStore(radGSkyOut, vec2i(i32(i), 2), vec4f(bOut, 0.0));
  radDyn[P.bOut + i * 4u + 3u] = luminance(bOut) * p.area;
}

/**
 * Inclusive CDF over the per-patch RIS weights, one workgroup: 256 threads
 * each scanning a contiguous chunk (<= 16 at the 4096-patch cap), then a
 * Hillis-Steele scan over the 256 chunk sums. The trace pass binary-searches
 * row 3; cdf[count-1] is the total.
 */
var<workgroup> cdfPartial : array<f32, 256>;

@compute @workgroup_size(256, 1, 1)
fn buildPatchCdf(@builtin(local_invocation_id) lid: vec3u) {
  let t = lid.x;
  let chunk = (P.count + 255u) / 256u;
  let start = t * chunk;
  var local : array<f32, 16>;
  var acc = 0.0;
  for (var k = 0u; k < chunk; k = k + 1u) {
    let i = start + k;
    var w = 0.0;
    if (i < P.count) { w = max(radDyn[P.bOut + i * 4u + 3u], 0.0); }
    acc = acc + w;
    local[k] = acc;
  }
  cdfPartial[t] = acc;
  workgroupBarrier();
  var off = 1u;
  for (var s = 0u; s < 8u; s = s + 1u) {
    var v = 0.0;
    if (t >= off) { v = cdfPartial[t - off]; }
    workgroupBarrier();
    cdfPartial[t] = cdfPartial[t] + v;
    workgroupBarrier();
    off = off * 2u;
  }
  let base = cdfPartial[t] - acc;
  for (var k = 0u; k < chunk; k = k + 1u) {
    let i = start + k;
    if (i < P.count) {
      textureStore(radGSkyOut, vec2i(i32(i), 3), vec4f(base + local[k], 0.0, 0.0, 0.0));
    }
  }
}
