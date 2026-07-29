// ===========================================================================
// Shared scene data, intersection, sampling and shading.
// Prepended to every compute pass by src/engine/shaders.ts.
// ===========================================================================

const PI: f32 = 3.14159265359;
const INV_PI: f32 = 0.31830988618;
const EPS: f32 = 1e-4;
const RAY_MAX: f32 = 1e30;
/** Below this luminance a light sample is not worth a shadow ray. */
const SHADOW_CULL: f32 = 0.0015;

const LIGHT_SPHERE: u32 = 0u;
const LIGHT_SPOT: u32 = 1u;

/** Boxes per coarse group. One group is exactly one character. */
const DYN_GROUP_SIZE: u32 = 26u;
const DYN_GROUPS: u32 = 8u;

struct Uniforms {
  invViewProj   : mat4x4f,
  prevViewProj  : mat4x4f,
  camPos        : vec3f,
  frame         : u32,
  resolution    : vec2f,
  invResolution : vec2f,
  flashPos      : vec3f,
  flashRadius   : f32,
  flashDir      : vec3f,
  flashIntensity: f32,
  flashColor    : vec3f,
  flashCosOuter : f32,
  flashCosInner : f32,
  lightCount    : u32,
  bounces       : u32,
  spp           : u32,
  skyIntensity  : f32,
  dynCount      : u32,
  ambient       : f32,
  /** Ray-march steps for the volumetric beam. */
  volSteps      : f32,
  exposure      : f32,
  time          : f32,
  debugView     : u32,
  volumetric    : f32,
  // Retained but unused for culling: gating the dynamic-box loop behind a slab
  // test against these bounds measured 2% SLOWER (14.85 vs 14.51 ms). Secondary
  // rays are incoherent, so a wavefront almost always contains both rays that
  // reach the character and rays that do not — the branch cannot be taken
  // uniformly, and the slab test is pure added cost. Do not re-add without
  // re-measuring.
  /**
   * 1 while the lighting is mid-change (a muzzle flash, a detonation), decaying
   * to 0 shortly after.
   *
   * Amplifies the denoiser's *per-pixel* history rejection rather than capping
   * history globally, so only the pixels whose lighting actually changed lose
   * their history. A muzzle flash lights the room for ~6 frames; with a
   * 48-frame history the decay afterwards would otherwise last nearly a second.
   *
   * Occupies the first slot of the old whole-list dynamic AABB, which the
   * per-group bounds replaced. The two after it are still dead, kept so every
   * later field keeps its offset.
   */
  /**
   * Kept only so later fields keep their byte offsets. The transient-signal
   * split replaced the history brake this used to drive.
   */
  _deadPulse    : f32,
  /**
   * First transient light index; everything from here to lightCount is a muzzle
   * flash or detonation. They are sampled by plain NEE into their own signal
   * and excluded from every resampling path — a reservoir that outlives its
   * light is a whole class of bug that simply cannot arise this way.
   */
  transientStart : u32,
  /**
   * Shadow rays per transient light on the primary hit.
   *
   * This signal has no temporal accumulation to average over frames, so its
   * only variance reduction is sample count and two a-trous passes. Rays are
   * the honest lever: more filtering just blurs the sharp shadows a flash
   * throws, which are most of what makes it read as real.
   *
   * Only paid while a flash is live. See RenderSettings for the cost table.
   */
  transientSamples : f32,
  /** Fraction of pixels that trace indirect bounces this frame. */
  indirectRate  : f32,
  /**
   * Distance from a flash at which its lighting is filtered as hard as the
   * chosen filter goes. Occupies the first of the three dead dynMax slots, so
   * every later offset is unchanged.
   */
  transientBlurDist : f32,
  /** How much bounced flash light counts toward that hint. */
  transientBounceWeight : f32,
  /**
   * First dynamic light index; everything from here to transientStart is a
   * guard's torch. Occupies the last of the three dead dynMax slots.
   */
  dynLightStart : u32,
  /** Initial ReSTIR candidates per pixel before reuse. */
  restirCandidates : f32,
  /** 0 disables temporal reservoir reuse. */
  restirTemporal   : f32,
  /** Cap on reused M, so the estimate stays responsive to a moving light. */
  restirMCap       : f32,
  restirGI         : f32,
  dynGroupCount    : u32,
  /**
   * Coarse bounds over fixed-size chunks of the dynamic box list.
   *
   * Dynamic geometry has no BVH — it is tested linearly, which was fine at one
   * character (24 boxes) and became the single largest cost in the frame at
   * five (120). Chunking is deliberately independent of character boundaries:
   * buildBoxes writes each character contiguously, so a fixed stride lands on
   * them naturally, and nothing in the game code has to describe its layout.
   */
  dynGroupMin      : array<vec4f, DYN_GROUPS>,
  dynGroupMax      : array<vec4f, DYN_GROUPS>,
}

const FLAG_EMISSIVE: u32 = 1u;
/**
 * Present in the world for light transport, absent for camera rays.
 *
 * This is what makes the cutaway view work. Ceilings have to exist or the moon
 * and sky pour straight into every room and there is no darkness to hide in —
 * but if the camera saw them there would be nothing to look at. Bounce and
 * shadow rays hit them, so rooms get real interior GI and the moon only enters
 * through windows.
 */
const FLAG_NO_CAMERA: u32 = 2u;
/** Identity rotation: the slab test can work directly in world space. */
const FLAG_AXIS_ALIGNED: u32 = 4u;

struct Box {
  center : vec3f,
  mat    : u32,
  half   : vec3f,
  // Bit 0: the material emits. Cached here so the shadow-ray inner loop can
  // skip light housings without a dependent load into materials[].
  flags  : u32,
  rot0   : vec3f,
  _pad1  : f32,
  rot1   : vec3f,
  _pad2  : f32,
  rot2   : vec3f,
  _pad3  : f32,
}

struct Material {
  albedo    : vec3f,
  roughness : f32,
  emissive  : vec3f,
  metallic  : f32,
}

struct Light {
  pos       : vec3f,
  kind      : u32,
  dir       : vec3f,
  radius    : f32,
  color     : vec3f,
  intensity : f32,
  cosInner  : f32,
  cosOuter  : f32,
  _pad      : vec2f,
  /** World half-extents of a slab emitter; zero means "sample the sphere". */
  halfExtents : vec3f,
  _pad2     : f32,
}

struct BVHNode {
  bmin      : vec3f,
  leftFirst : u32,
  bmax      : vec3f,
  count     : u32,
}

@group(0) @binding(0) var<uniform> U : Uniforms;
@group(0) @binding(1) var<storage, read> boxes : array<Box>;
@group(0) @binding(2) var<storage, read> materials : array<Material>;
@group(0) @binding(3) var<storage, read> lights : array<Light>;
@group(0) @binding(4) var<storage, read> bvh : array<BVHNode>;
// Moving geometry (the player, doors, anything animated). Kept out of the
// static BVH and tested linearly — there are only a handful of boxes, and this
// avoids rebuilding an acceleration structure every frame.
@group(0) @binding(5) var<storage, read> dynBoxes : array<Box>;
/**
 * The same dynamic boxes as of last frame.
 *
 * Reprojection maps a world position through the previous view-projection,
 * which is only correct for geometry that did not move. Keeping the previous
 * transforms lets a hit on animated geometry be carried back to where that
 * surface point actually was, so the character keeps its temporal history
 * instead of falling back to a raw single sample every frame.
 */
@group(0) @binding(6) var<storage, read> prevDynBoxes : array<Box>;

// ---------------------------------------------------------------------------
// RNG — PCG hash. Decorrelated per pixel, frame and sample.
// ---------------------------------------------------------------------------

var<private> rngState : u32;

fn pcgHash(v: u32) -> u32 {
  let state = v * 747796405u + 2891336453u;
  let word = ((state >> ((state >> 28u) + 4u)) ^ state) * 277803737u;
  return (word >> 22u) ^ word;
}

fn seedRng(pixel: vec2u, frame: u32) {
  rngState = pcgHash(pixel.x + pcgHash(pixel.y + pcgHash(frame * 9781u + 1u)));
}

fn rand() -> f32 {
  rngState = pcgHash(rngState);
  return f32(rngState) * 2.3283064365386963e-10;
}

fn rand2() -> vec2f { return vec2f(rand(), rand()); }

// ---------------------------------------------------------------------------
// Sampling helpers
// ---------------------------------------------------------------------------

/** Builds an orthonormal basis around n (Duff et al., branchless). */
fn onb(n: vec3f) -> mat3x3f {
  let s = select(-1.0, 1.0, n.z >= 0.0);
  let a = -1.0 / (s + n.z);
  let b = n.x * n.y * a;
  let t = vec3f(1.0 + s * n.x * n.x * a, s * b, -s * n.x);
  let bt = vec3f(b, s + n.y * n.y * a, -n.y);
  return mat3x3f(t, bt, n);
}

fn cosineHemisphere(n: vec3f, u: vec2f) -> vec3f {
  let r = sqrt(u.x);
  let phi = 2.0 * PI * u.y;
  let local = vec3f(r * cos(phi), r * sin(phi), sqrt(max(0.0, 1.0 - u.x)));
  return normalize(onb(n) * local);
}

fn uniformSphere(u: vec2f) -> vec3f {
  let z = 1.0 - 2.0 * u.x;
  let r = sqrt(max(0.0, 1.0 - z * z));
  let phi = 2.0 * PI * u.y;
  return vec3f(r * cos(phi), r * sin(phi), z);
}

/** GGX visible-normal sampling (Heitz 2018) in the local frame of n. */
fn sampleGGX(n: vec3f, v: vec3f, rough: f32, u: vec2f) -> vec3f {
  let basis = onb(n);
  let vl = normalize(vec3f(dot(v, basis[0]), dot(v, basis[1]), dot(v, basis[2])));
  let a = max(rough * rough, 1e-3);

  let vh = normalize(vec3f(a * vl.x, a * vl.y, vl.z));
  let lensq = vh.x * vh.x + vh.y * vh.y;
  var t1 = vec3f(1.0, 0.0, 0.0);
  if (lensq > 0.0) { t1 = vec3f(-vh.y, vh.x, 0.0) * inverseSqrt(lensq); }
  let t2 = cross(vh, t1);

  let r = sqrt(u.x);
  let phi = 2.0 * PI * u.y;
  let px = r * cos(phi);
  var py = r * sin(phi);
  let s = 0.5 * (1.0 + vh.z);
  py = (1.0 - s) * sqrt(max(0.0, 1.0 - px * px)) + s * py;

  let nh = px * t1 + py * t2 + sqrt(max(0.0, 1.0 - px * px - py * py)) * vh;
  let hl = normalize(vec3f(a * nh.x, a * nh.y, max(nh.z, 1e-6)));
  return normalize(basis * hl);
}

fn luminance(c: vec3f) -> f32 { return dot(c, vec3f(0.2126, 0.7152, 0.0722)); }

// ---------------------------------------------------------------------------
// Intersection
// ---------------------------------------------------------------------------

struct Hit {
  t      : f32,
  p      : vec3f,
  n      : vec3f,
  mat    : u32,
  valid  : bool,
  /** Index into dynBoxes, or DYN_NONE when the hit was static geometry. */
  dynIdx : u32,
}

const DYN_NONE: u32 = 0xffffffffu;

/** Ray vs. world-space AABB. Returns entry distance, or -1 on miss. */
fn slabAABB(ro: vec3f, invD: vec3f, bmin: vec3f, bmax: vec3f, tmax: f32) -> f32 {
  let t1 = (bmin - ro) * invD;
  let t2 = (bmax - ro) * invD;
  let tn = min(t1, t2);
  let tf = max(t1, t2);
  let tNear = max(max(tn.x, tn.y), tn.z);
  let tFar = min(min(tf.x, tf.y), tf.z);
  if (tFar < max(tNear, 0.0) || tNear > tmax) { return -1.0; }
  return max(tNear, 0.0);
}

struct BoxHit {
  t   : f32,
  n   : vec3f,
  hit : bool,
}

/**
 * Ray vs. oriented box. The rotation rows are the box's local axes in world
 * space, so world->local is just three dot products (the matrix is orthonormal,
 * no inverse needed).
 */
fn hitBox(ro: vec3f, rd: vec3f, b: Box, tmin: f32, tmax: f32) -> BoxHit {
  var res: BoxHit;
  res.hit = false;
  res.t = tmax;
  res.n = vec3f(0.0, 1.0, 0.0);

  // Most of the level (walls, floor, ceiling, carpets) is axis aligned, where
  // the change of basis is the identity and the six dot products are wasted.
  // This has to be a real branch, not select(): select evaluates both operands,
  // so it would compute the dot products anyway and save nothing.
  let d = ro - b.center;
  let aligned = (b.flags & FLAG_AXIS_ALIGNED) != 0u;
  var lo: vec3f;
  var ld: vec3f;
  if (aligned) {
    lo = d;
    ld = rd;
  } else {
    lo = vec3f(dot(d, b.rot0), dot(d, b.rot1), dot(d, b.rot2));
    ld = vec3f(dot(rd, b.rot0), dot(rd, b.rot1), dot(rd, b.rot2));
  }
  let invD = 1.0 / ld;

  let t1 = (-b.half - lo) * invD;
  let t2 = (b.half - lo) * invD;
  let tn = min(t1, t2);
  let tf = max(t1, t2);
  let tNear = max(max(tn.x, tn.y), tn.z);
  let tFar = min(min(tf.x, tf.y), tf.z);

  if (tFar < tNear || tFar < tmin) { return res; }

  // Take the entry point, or the exit point when the origin is inside.
  let inside = tNear < tmin;
  let t = select(tNear, tFar, inside);
  if (t < tmin || t > tmax) { return res; }

  // Pick the slab that produced the extremum. The step() pair selects the axis
  // whose t is the max (entry) or min (exit) without float equality tests.
  var nl: vec3f;
  if (inside) {
    nl = -sign(ld) * step(tf.xyz, tf.yzx) * step(tf.xyz, tf.zxy);
  } else {
    nl = -sign(ld) * step(tn.yzx, tn.xyz) * step(tn.zxy, tn.xyz);
  }
  var nw: vec3f;
  if (aligned) {
    nw = nl;
  } else {
    nw = nl.x * b.rot0 + nl.y * b.rot1 + nl.z * b.rot2;
  }

  res.t = t;
  res.n = normalize(select(nw, vec3f(0.0, 1.0, 0.0), dot(nw, nw) < 1e-8));
  res.hit = true;
  return res;
}

/**
 * Closest-hit BVH traversal, near child first.
 * `cameraRay` makes FLAG_NO_CAMERA geometry transparent to this trace.
 */
fn trace(ro: vec3f, rd: vec3f, tmax: f32, cameraRay: bool) -> Hit {
  var h: Hit;
  h.valid = false;
  h.t = tmax;
  h.mat = 0u;
  h.n = vec3f(0.0, 1.0, 0.0);
  h.p = ro;
  h.dynIdx = DYN_NONE;

  let invD = 1.0 / rd;
  var stack: array<u32, 32>;
  var sp = 0u;
  var cur = 0u;

  loop {
    let nd = bvh[cur];
    if (nd.count > 0u) {
      for (var i = 0u; i < nd.count; i = i + 1u) {
        let b = boxes[nd.leftFirst + i];
        if (cameraRay && (b.flags & FLAG_NO_CAMERA) != 0u) { continue; }
        let bh = hitBox(ro, rd, b, EPS, h.t);
        if (bh.hit) {
          h.t = bh.t;
          h.n = bh.n;
          h.mat = b.mat;
          h.valid = true;
        }
      }
      if (sp == 0u) { break; }
      sp = sp - 1u;
      cur = stack[sp];
    } else {
      let li = nd.leftFirst;
      let ri = li + 1u;
      let nl = bvh[li];
      let nr = bvh[ri];
      let dl = slabAABB(ro, invD, nl.bmin, nl.bmax, h.t);
      let dr = slabAABB(ro, invD, nr.bmin, nr.bmax, h.t);

      if (dl < 0.0 && dr < 0.0) {
        if (sp == 0u) { break; }
        sp = sp - 1u;
        cur = stack[sp];
      } else if (dl < 0.0) {
        cur = ri;
      } else if (dr < 0.0) {
        cur = li;
      } else if (dl < dr) {
        if (sp < 31u) { stack[sp] = ri; sp = sp + 1u; }
        cur = li;
      } else {
        if (sp < 31u) { stack[sp] = li; sp = sp + 1u; }
        cur = ri;
      }
    }
  }

  // Dynamic geometry: no BVH, but one slab test per character rejects all 24 of
  // its boxes at once. An earlier attempt at a *single* AABB over the whole
  // dynamic list measured 2% slower — with one character it rejected nothing
  // and only added divergence. Per-group is a different trade at five.
  //
  // Measured at 1152x720 / 2 bounces, player + 4 guards, median of 3 runs:
  //   linear over all 120 boxes   38.8 ms   (guards alone cost 19.5 ms)
  //   per-group slab test         17.8 ms   (guards alone cost  1.6 ms)
  // Caveat: guards keep patrolling during a run while the bench pins only the
  // player, so how many are on screen varies. One outlier run measured 11.2 ms.
  for (var g = 0u; g < U.dynGroupCount; g = g + 1u) {
    if (slabAABB(ro, invD, U.dynGroupMin[g].xyz, U.dynGroupMax[g].xyz, h.t) < 0.0) {
      continue;
    }
    let lo = g * DYN_GROUP_SIZE;
    let hi = min(lo + DYN_GROUP_SIZE, U.dynCount);
    for (var i = lo; i < hi; i = i + 1u) {
      let b = dynBoxes[i];
      if (cameraRay && (b.flags & FLAG_NO_CAMERA) != 0u) { continue; }
      let bh = hitBox(ro, rd, b, EPS, h.t);
      if (bh.hit) {
        h.t = bh.t;
        h.n = bh.n;
        h.mat = b.mat;
        h.valid = true;
        h.dynIdx = i;
      }
    }
  }

  if (h.valid) {
    h.p = ro + rd * h.t;
    // Face the normal toward the incoming ray so shading is two-sided.
    if (dot(h.n, rd) > 0.0) { h.n = -h.n; }
  }
  return h;
}

/** Any-hit traversal for shadow rays — bails on the first intersection. */
/** Sentinel meaning "do not skip any dynamic group". */
const DYN_GROUP_NONE: u32 = 0xffffffffu;

/**
 * Shadow test that can ignore one character.
 *
 * A gameplay probe placed at a character's chest is *inside* that character's
 * own torso box, so every ray it casts hits the character immediately and every
 * light reads as blocked. Skipping the group the probe belongs to is the fix;
 * the character still occludes everyone else.
 */
fn occludedSkipping(ro: vec3f, rd: vec3f, tmax: f32, skipGroup: u32) -> bool {
  let invD = 1.0 / rd;
  var stack: array<u32, 32>;
  var sp = 0u;
  var cur = 0u;

  loop {
    let nd = bvh[cur];
    if (nd.count > 0u) {
      for (var i = 0u; i < nd.count; i = i + 1u) {
        let b = boxes[nd.leftFirst + i];
        // Emissive surfaces do not cast shadows: otherwise every light is
        // occluded by its own housing geometry.
        if ((b.flags & FLAG_EMISSIVE) != 0u) { continue; }
        if (hitBox(ro, rd, b, EPS, tmax).hit) { return true; }
      }
      if (sp == 0u) { break; }
      sp = sp - 1u;
      cur = stack[sp];
    } else {
      let li = nd.leftFirst;
      let ri = li + 1u;
      let dl = slabAABB(ro, invD, bvh[li].bmin, bvh[li].bmax, tmax);
      let dr = slabAABB(ro, invD, bvh[ri].bmin, bvh[ri].bmax, tmax);
      if (dl < 0.0 && dr < 0.0) {
        if (sp == 0u) { break; }
        sp = sp - 1u;
        cur = stack[sp];
      } else if (dl < 0.0) {
        cur = ri;
      } else if (dr < 0.0) {
        cur = li;
      } else {
        if (sp < 31u) { stack[sp] = ri; sp = sp + 1u; }
        cur = li;
      }
    }
  }

  for (var g = 0u; g < U.dynGroupCount; g = g + 1u) {
    if (g == skipGroup) { continue; }
    if (slabAABB(ro, invD, U.dynGroupMin[g].xyz, U.dynGroupMax[g].xyz, tmax) < 0.0) {
      continue;
    }
    let lo = g * DYN_GROUP_SIZE;
    let hi = min(lo + DYN_GROUP_SIZE, U.dynCount);
    for (var i = lo; i < hi; i = i + 1u) {
      // Same rule as the static loop above: an emissive box is a light housing,
      // not an occluder. Without this the torch lens sits on top of flashPos and
      // shadows the beam it emits, costing roughly half the beam's brightness.
      if ((dynBoxes[i].flags & FLAG_EMISSIVE) != 0u) { continue; }
      if (hitBox(ro, rd, dynBoxes[i], EPS, tmax).hit) { return true; }
    }
  }
  return false;
}

fn occluded(ro: vec3f, rd: vec3f, tmax: f32) -> bool {
  return occludedSkipping(ro, rd, tmax, DYN_GROUP_NONE);
}

// ---------------------------------------------------------------------------
// Shading
// ---------------------------------------------------------------------------

fn skyRadiance(d: vec3f) -> vec3f {
  // Dim moonlit sky. Deliberately weak — the flashlight has to stay dominant
  // or the whole stealth read falls apart.
  let up = clamp(d.y * 0.5 + 0.5, 0.0, 1.0);
  let horizon = vec3f(0.030, 0.042, 0.075);
  let zenith = vec3f(0.045, 0.070, 0.135);
  return mix(horizon, zenith, up) * U.skyIntensity;
}

fn fresnelSchlick(f0: vec3f, u: f32) -> vec3f {
  let f = pow(clamp(1.0 - u, 0.0, 1.0), 5.0);
  return f0 + (vec3f(1.0) - f0) * f;
}

fn distributionGGX(nDotH: f32, rough: f32) -> f32 {
  let a = max(rough * rough, 1e-3);
  let a2 = a * a;
  let d = nDotH * nDotH * (a2 - 1.0) + 1.0;
  return a2 / max(PI * d * d, 1e-9);
}

fn visibilitySmith(nDotV: f32, nDotL: f32, rough: f32) -> f32 {
  // Height-correlated Smith visibility (already includes the 1/(4 NoL NoV)).
  let a = max(rough * rough, 1e-3);
  let a2 = a * a;
  let gv = nDotL * sqrt(nDotV * nDotV * (1.0 - a2) + a2);
  let gl = nDotV * sqrt(nDotL * nDotL * (1.0 - a2) + a2);
  return 0.5 / max(gv + gl, 1e-6);
}

/** Full BSDF evaluation for a given light direction. */
fn evalBSDF(m: Material, n: vec3f, v: vec3f, l: vec3f) -> vec3f {
  let nDotL = dot(n, l);
  let nDotV = dot(n, v);
  if (nDotL <= 0.0 || nDotV <= 0.0) { return vec3f(0.0); }
  let h = normalize(v + l);
  let nDotH = clamp(dot(n, h), 0.0, 1.0);
  let vDotH = clamp(dot(v, h), 0.0, 1.0);

  let f0 = mix(vec3f(0.04), m.albedo, m.metallic);
  let f = fresnelSchlick(f0, vDotH);
  let d = distributionGGX(nDotH, m.roughness);
  let vis = visibilitySmith(nDotV, nDotL, m.roughness);
  let spec = f * (d * vis);
  let kd = (vec3f(1.0) - f) * (1.0 - m.metallic);
  let diff = kd * m.albedo * INV_PI;
  return (diff + spec) * nDotL;
}

struct LightSample {
  dir       : vec3f,
  dist      : f32,
  radiance  : vec3f,
}

/**
 * Softened inverse-square falloff.
 *
 * True 1/d^2 goes to infinity at the source, so anything the player stands next
 * to blows out to pure white however good the tonemapper is. The added constant
 * caps the near field while leaving the falloff essentially exact past ~1m,
 * which is where all the shot composition actually happens.
 */
fn falloff(d2: f32) -> f32 {
  return 1.0 / (d2 + 0.45);
}

/** Samples a point on a spherical emitter to get soft shadows. */
fn sampleSphereLight(l: Light, p: vec3f) -> LightSample {
  var s: LightSample;
  // Slab emitters sample their own box. A sphere sized from the two largest
  // half-extents pokes through the fixture along the thin axis, and every
  // sample that lands in the housing is a shadow ray spent to return black.
  // Sampling the box keeps the long-axis penumbra and guarantees the sample is
  // somewhere that actually emits.
  var jitter = uniformSphere(rand2()) * l.radius;
  if (dot(l.halfExtents, l.halfExtents) > 0.0) {
    let a = rand2();
    let b = rand2();
    jitter = (vec3f(a.x, a.y, b.x) * 2.0 - 1.0) * l.halfExtents;
  }
  let samplePt = l.pos + jitter;
  let delta = samplePt - p;
  let d2 = max(dot(delta, delta), 1e-4);
  s.dist = sqrt(d2);
  s.dir = delta / s.dist;
  s.radiance = l.color * (l.intensity * falloff(d2));
  return s;
}

/** Spot cone attenuation with a smooth penumbra between the two cone angles. */
fn spotAttenuation(lightDir: vec3f, toSurface: vec3f, cosInner: f32, cosOuter: f32) -> f32 {
  let cd = dot(lightDir, toSurface);
  return smoothstep(cosOuter, cosInner, cd);
}

/** Next-event estimation against the player's flashlight. */
fn sampleFlashlight(p: vec3f, n: vec3f, v: vec3f, m: Material) -> vec3f {
  if (U.flashIntensity <= 0.0) { return vec3f(0.0); }
  // Jitter over the lens disc: penumbra grows with distance, which is the
  // single most important cue that the shadows are real.
  let jitter = uniformSphere(rand2()) * U.flashRadius;
  let samplePt = U.flashPos + jitter;
  let delta = samplePt - p;
  let d2 = max(dot(delta, delta), 1e-4);
  let dist = sqrt(d2);
  let dir = delta / dist;

  let cone = spotAttenuation(U.flashDir, -dir, U.flashCosInner, U.flashCosOuter);
  if (cone <= 0.0) { return vec3f(0.0); }

  let f = evalBSDF(m, n, v, dir);
  if (dot(f, f) <= 0.0) { return vec3f(0.0); }

  // Cull before spending a shadow ray. Tracing rays whose contribution would
  // round away is the single biggest source of wasted work in NEE.
  let contrib = f * U.flashColor * (U.flashIntensity * cone * falloff(d2));
  if (luminance(contrib) < SHADOW_CULL) { return vec3f(0.0); }

  if (occluded(p + n * EPS * 4.0, dir, dist - EPS * 8.0)) { return vec3f(0.0); }
  return contrib;
}

/**
 * Cheap RIS ranking function.
 *
 * RIS only needs the target to *order* candidates; the surviving sample is
 * still shaded with the exact BSDF, so the estimator stays unbiased and only
 * the variance changes. Evaluating full GGX (Fresnel, distribution, Smith
 * visibility) eight times per light sample just to pick one is a lot of ALU
 * for a decision a diffuse proxy makes almost identically.
 *
 * Measured at a pinned 1152x720 / 1spp / 2 bounces: 17.29ms with full-BSDF
 * ranking, 16.61ms with this. 3.9%, and free.
 */
fn risTarget(m: Material, n: vec3f, l: vec3f, radiance: vec3f) -> f32 {
  let nDotL = dot(n, l);
  if (nDotL <= 0.0) { return 0.0; }
  let refl = max(mix(m.albedo, vec3f(1.0), m.metallic), vec3f(0.02));
  return nDotL * luminance(radiance * refl);
}

/** Unshadowed contribution of one light sample — used for the final estimate. */
fn lightContribution(l: Light, s: LightSample, p: vec3f, n: vec3f, v: vec3f, m: Material) -> vec3f {
  var atten = 1.0;
  if (l.kind == LIGHT_SPOT) {
    atten = spotAttenuation(l.dir, -s.dir, l.cosInner, l.cosOuter);
  }
  if (atten <= 0.0) { return vec3f(0.0); }
  return evalBSDF(m, n, v, s.dir) * s.radiance * atten;
}

/**
 * NEE against the scene light list using resampled importance sampling.
 *
 * Picking a light uniformly is what produces the sparkle: with lights scattered
 * across a whole floor plate, the 1-in-N pick is almost always a dim distant
 * one, and the rare near hit arrives multiplied by N. RIS instead draws M cheap
 * shadow-ray-free candidates, keeps one in proportion to its actual unshadowed
 * contribution, and spends the single shadow ray on that. Same cost in rays,
 * dramatically lower variance.
 */
/**
 * The key light — lights[0], by convention the moon.
 *
 * It is thousands of times brighter than any practical in the building, so if
 * it competes in the RIS reservoir it wins essentially every draw and no local
 * light is ever sampled. Worse, indoors it is usually occluded, so those wins
 * come back black and the room goes to noise. Sampling it on its own channel
 * fixes both.
 */
fn sampleKeyLight(p: vec3f, n: vec3f, v: vec3f, m: Material) -> vec3f {
  if (U.lightCount == 0u) { return vec3f(0.0); }
  let l = lights[0];
  let s = sampleSphereLight(l, p);
  let contrib = lightContribution(l, s, p, n, v, m);
  if (luminance(contrib) < SHADOW_CULL) { return vec3f(0.0); }
  if (occluded(p + n * EPS * 4.0, s.dir, s.dist - EPS * 8.0)) { return vec3f(0.0); }
  return contrib;
}

/** RIS over the local lights, i.e. everything except the key light. */
fn sampleSceneLight(p: vec3f, n: vec3f, v: vec3f, m: Material) -> vec3f {
  if (U.lightCount <= 1u) { return vec3f(0.0); }
  let localCount = U.lightCount - 1u;

  let M = min(localCount, 8u);
  var chosen: Light;
  var chosenSample: LightSample;
  var chosenTarget = 0.0;
  var wsum = 0.0;

  for (var i = 0u; i < M; i = i + 1u) {
    let idx = 1u + min(u32(rand() * f32(localCount)), localCount - 1u);
    let l = lights[idx];
    let s = sampleSphereLight(l, p);
    var atten = 1.0;
    if (l.kind == LIGHT_SPOT) {
      atten = spotAttenuation(l.dir, -s.dir, l.cosInner, l.cosOuter);
    }
    let targetPdf = risTarget(m, n, s.dir, s.radiance * atten);
    wsum = wsum + targetPdf;
    // Reservoir update: keep this candidate with probability target/wsum.
    if (targetPdf > 0.0 && rand() * wsum < targetPdf) {
      chosen = l;
      chosenSample = s;
      chosenTarget = targetPdf;
    }
  }

  if (wsum <= 0.0 || chosenTarget <= 0.0) { return vec3f(0.0); }

  // RIS estimator weight: mean candidate weight over the chosen target pdf.
  // The uniform source pdf cancels, so no lightCount factor appears here.
  // Capped because an unlikely low-target candidate that does get selected
  // returns a huge weight, and SVGF's luminance edge-stopping preserves single
  // bright pixels rather than filtering them.
  let W = min((wsum / f32(M)) / chosenTarget, 24.0);
  let contrib = lightContribution(chosen, chosenSample, p, n, v, m) * W;
  if (luminance(contrib) < SHADOW_CULL) { return vec3f(0.0); }

  if (occluded(p + n * EPS * 4.0, chosenSample.dir, chosenSample.dist - EPS * 8.0)) {
    return vec3f(0.0);
  }
  return contrib;
}

/** One light proposal: index 0 is the flashlight, 1.. index into lights[]. */
struct Candidate {
  dir      : vec3f,
  dist     : f32,
  radiance : vec3f,
  valid    : bool,
  /** World point on the emitter. Stored so a reservoir can be re-evaluated at
   *  a different shading point next frame without re-sampling the light. */
  samplePos: vec3f,
}

fn proposeCandidate(idx: u32, p: vec3f) -> Candidate {
  var c: Candidate;
  c.valid = false;
  c.radiance = vec3f(0.0);
  c.dir = vec3f(0.0, 1.0, 0.0);
  c.dist = 0.0;

  if (idx == 0u) {
    if (U.flashIntensity <= 0.0) { return c; }
    let jitter = uniformSphere(rand2()) * U.flashRadius;
    let delta = U.flashPos + jitter - p;
    let d2 = max(dot(delta, delta), 1e-4);
    c.dist = sqrt(d2);
    c.dir = delta / c.dist;
    let cone = spotAttenuation(U.flashDir, -c.dir, U.flashCosInner, U.flashCosOuter);
    if (cone <= 0.0) { return c; }
    c.radiance = U.flashColor * (U.flashIntensity * cone * falloff(d2));
    c.samplePos = U.flashPos + jitter;
    c.valid = true;
    return c;
  }

  let l = lights[idx - 1u];
  let s = sampleSphereLight(l, p);
  var atten = 1.0;
  if (l.kind == LIGHT_SPOT) {
    atten = spotAttenuation(l.dir, -s.dir, l.cosInner, l.cosOuter);
  }
  if (atten <= 0.0) { return c; }
  c.dir = s.dir;
  c.dist = s.dist;
  c.radiance = s.radiance * atten;
  c.samplePos = p + s.dir * s.dist;
  c.valid = true;
  return c;
}

// ---------------------------------------------------------------------------
// ReSTIR DI
//
// The reservoir the RIS loop already builds is thrown away every frame. Keeping
// it and merging last frame's into this one raises the effective candidate
// count from a handful to hundreds, which is what makes a single unified
// reservoir over every light viable. Sampling all lights in one reservoir was
// tried without this and produced a green wash across the floor: with only 8
// candidates the estimator weight is total/M ~ 3.75 on whatever gets picked, so
// a bright exit sign occasionally outranked the flashlight. More effective
// samples is precisely the cure.
// ---------------------------------------------------------------------------

struct Reservoir {
  samplePos : vec3f,
  lightIdx  : u32,
  wSum      : f32,
  /** Candidates seen. Capped on reuse so the estimate stays responsive. */
  M         : f32,
  /** Final unbiased estimator weight. */
  W         : f32,
  targetPdf : f32,
}

fn emptyReservoir() -> Reservoir {
  var r: Reservoir;
  r.samplePos = vec3f(0.0);
  r.lightIdx = 0u;
  r.wSum = 0.0;
  r.M = 0.0;
  r.W = 0.0;
  r.targetPdf = 0.0;
  return r;
}

/** Streaming reservoir update: keep the candidate with probability w/wSum. */
fn reservoirUpdate(
  r: ptr<function, Reservoir>, pos: vec3f, idx: u32, targetPdf: f32, w: f32,
) {
  (*r).wSum = (*r).wSum + w;
  (*r).M = (*r).M + 1.0;
  if (w > 0.0 && rand() * (*r).wSum < w) {
    (*r).samplePos = pos;
    (*r).lightIdx = idx;
    (*r).targetPdf = targetPdf;
  }
}

/** Emitted radiance arriving at `p` from the stored sample of light `idx`. */
fn radianceFromLight(idx: u32, p: vec3f, samplePos: vec3f) -> vec3f {
  let delta = samplePos - p;
  let d2 = max(dot(delta, delta), 1e-4);
  let dir = delta * inverseSqrt(d2);

  if (idx == 0u) {
    if (U.flashIntensity <= 0.0) { return vec3f(0.0); }
    let cone = spotAttenuation(U.flashDir, -dir, U.flashCosInner, U.flashCosOuter);
    if (cone <= 0.0) { return vec3f(0.0); }
    return U.flashColor * (U.flashIntensity * cone * falloff(d2));
  }

  let l = lights[idx - 1u];
  var atten = 1.0;
  if (l.kind == LIGHT_SPOT) {
    atten = spotAttenuation(l.dir, -dir, l.cosInner, l.cosOuter);
  }
  if (atten <= 0.0) { return vec3f(0.0); }
  return l.color * (l.intensity * falloff(d2)) * atten;
}

/** Builds a fresh reservoir from M uniform light proposals. */
fn generateReservoir(p: vec3f, n: vec3f, v: vec3f, m: Material, count: u32) -> Reservoir {
  var r = emptyReservoir();
  // Steady lights only, plus the flashlight. See Uniforms.transientStart.
  let total = U.transientStart + 1u;
  let M = max(1u, min(count, total));
  for (var i = 0u; i < M; i = i + 1u) {
    let idx = min(u32(rand() * f32(total)), total - 1u);
    let c = proposeCandidate(idx, p);
    if (!c.valid) {
      r.M = r.M + 1.0;
      continue;
    }
    let tp = risTarget(m, n, c.dir, c.radiance);
    // Uniform source pdf of 1/total, so each weight carries a factor of total.
    reservoirUpdate(&r, c.samplePos, idx, tp, tp * f32(total));
  }
  return r;
}

/** Merges `other` into `r`, re-weighting it by this pixel's target function. */
fn mergeReservoir(
  r: ptr<function, Reservoir>, other: Reservoir,
  p: vec3f, n: vec3f, v: vec3f, m: Material,
) {
  if (other.M <= 0.0 || other.W <= 0.0) { return; }
  let delta = other.samplePos - p;
  let d2 = max(dot(delta, delta), 1e-4);
  let dir = delta * inverseSqrt(d2);
  let tp = risTarget(m, n, dir, radianceFromLight(other.lightIdx, p, other.samplePos));
  if (tp <= 0.0) {
    (*r).M = (*r).M + other.M;
    return;
  }
  let w = tp * other.W * other.M;
  (*r).wSum = (*r).wSum + w;
  (*r).M = (*r).M + other.M;
  if (rand() * (*r).wSum < w) {
    (*r).samplePos = other.samplePos;
    (*r).lightIdx = other.lightIdx;
    (*r).targetPdf = tp;
  }
}

/** Finalises the estimator weight from the accumulated stream. */
fn finalizeReservoir(r: ptr<function, Reservoir>) {
  let denom = (*r).M * (*r).targetPdf;
  if (denom > 1e-9) {
    (*r).W = (*r).wSum / denom;
  } else {
    (*r).W = 0.0;
  }
}

/**
 * RIS over every light in the scene, flashlight included.
 *
 * Used for indirect bounces. The previous scheme picked one of three light
 * channels uniformly and scaled by 3, which is a large variance amplifier
 * exactly where it hurts: a bounce point sitting inside the flashlight beam
 * alternates between a bright sample (flashlight drawn) and zero (moon drawn,
 * occluded indoors), so a wall lit only by bounce light off that floor fills
 * with speckle. Resampling by unshadowed contribution instead picks whichever
 * light actually matters at that point, and the estimator weight stays near 1.
 */
fn sampleIndirectRIS(p: vec3f, n: vec3f, v: vec3f, m: Material) -> vec3f {
  // Steady lights only. Transients are added separately, un-resampled.
  let total = U.transientStart + 1u;
  let M = min(total, 8u);

  var chosen: Candidate;
  var chosenTarget = 0.0;
  var wsum = 0.0;

  for (var i = 0u; i < M; i = i + 1u) {
    let idx = min(u32(rand() * f32(total)), total - 1u);
    let c = proposeCandidate(idx, p);
    if (!c.valid) { continue; }
    let targetPdf = risTarget(m, n, c.dir, c.radiance);
    wsum = wsum + targetPdf;
    if (targetPdf > 0.0 && rand() * wsum < targetPdf) {
      chosen = c;
      chosenTarget = targetPdf;
    }
  }

  if (wsum <= 0.0 || chosenTarget <= 0.0) { return vec3f(0.0); }

  // Uniform proposal over `total` lights, so each candidate weight carries a
  // factor of `total`.
  let W = min((wsum / f32(M)) * f32(total) / chosenTarget, 32.0);
  let contrib = evalBSDF(m, n, v, chosen.dir) * chosen.radiance * W;
  if (luminance(contrib) < SHADOW_CULL) { return vec3f(0.0); }
  if (occluded(p + n * EPS * 4.0, chosen.dir, chosen.dist - EPS * 8.0)) {
    return vec3f(0.0);
  }
  return contrib;
}

struct BsdfSample {
  dir      : vec3f,
  weight   : vec3f,
  specular : bool,
}

/** Importance-samples the BSDF, returning throughput weight (brdf*cos/pdf). */
fn sampleBSDF(m: Material, n: vec3f, v: vec3f) -> BsdfSample {
  var s: BsdfSample;
  s.specular = false;
  s.dir = n;
  s.weight = vec3f(0.0);

  // Probabilistically choose the specular or diffuse lobe. The selection
  // probability tracks the view-dependent Fresnel rather than just F0: at
  // grazing angles F approaches 1, and picking specular only 4% of the time
  // there would divide throughput by 0.04 and manufacture fireflies.
  let f0 = mix(vec3f(0.04), m.albedo, m.metallic);
  let nDotVsel = clamp(dot(n, v), 0.0, 1.0);
  let fEstimate = fresnelSchlick(f0, nDotVsel);
  let specWeight = clamp(luminance(fEstimate) + m.metallic * 0.4, 0.12, 0.9);
  let pickSpec = rand() < specWeight;

  if (pickSpec) {
    let h = sampleGGX(n, v, m.roughness, rand2());
    let l = reflect(-v, h);
    if (dot(l, n) <= 0.0) { return s; }
    let nDotL = dot(n, l);
    let nDotV = max(dot(n, v), 1e-4);
    let vDotH = clamp(dot(v, h), 0.0, 1.0);
    let f = fresnelSchlick(f0, vDotH);

    // With VNDF sampling, D and the view-side masking cancel against the pdf;
    // what survives is F * G2/G1(V).
    let a = max(m.roughness * m.roughness, 1e-3);
    let a2 = a * a;
    let gv = nDotL * sqrt(nDotV * nDotV * (1.0 - a2) + a2);
    let gl = nDotV * sqrt(nDotL * nDotL * (1.0 - a2) + a2);
    let vis = 0.5 / max(gv + gl, 1e-6);              // G2 / (4 NoL NoV)
    let g2 = vis * 4.0 * nDotL * nDotV;
    let lambdaV = sqrt(a2 + (1.0 - a2) * nDotV * nDotV);
    let g1 = 2.0 * nDotV / max(nDotV + lambdaV, 1e-6);
    let g2OverG1 = g2 / max(g1, 1e-6);

    s.dir = l;
    s.weight = f * g2OverG1 / specWeight;
    s.specular = m.roughness < 0.15;
  } else {
    let l = cosineHemisphere(n, rand2());
    if (dot(l, n) <= 0.0) { return s; }
    let h = normalize(v + l);
    let vDotH = clamp(dot(v, h), 0.0, 1.0);
    let f = fresnelSchlick(f0, vDotH);
    let kd = (vec3f(1.0) - f) * (1.0 - m.metallic);
    // Cosine-weighted pdf cancels the cosine and the 1/PI.
    s.dir = l;
    s.weight = kd * m.albedo / (1.0 - specWeight);
  }
  return s;
}

// ---------------------------------------------------------------------------
// Camera
// ---------------------------------------------------------------------------

fn cameraRay(uv: vec2f) -> vec3f {
  // uv in [0,1], y down. Convert to NDC and unproject through the far plane.
  let ndc = vec4f(uv.x * 2.0 - 1.0, 1.0 - uv.y * 2.0, 1.0, 1.0);
  let world = U.invViewProj * ndc;
  return normalize(world.xyz / world.w - U.camPos);
}

// ---------------------------------------------------------------------------
// ReSTIR GI.
//
// Where DI reservoirs hold a light, these hold a *secondary surface point* plus
// the outgoing radiance cached there. Reuse is a reconnection shift: keep x2
// fixed and rebuild the connection from whichever visible point is reusing it.
//
// That cached radiance is treated as view-independent, which is only exactly
// true for a Lambertian x2 — the one structural bias in ReSTIR GI, and one this
// project does not pay, since its materials are diffuse by design.
// ---------------------------------------------------------------------------

/** 64 bytes: four vec3f/f32 pairs, so std430 packs it with no padding. */
struct GIReservoir {
  /** x2, the secondary hit. Stored in world space, fixed under the shift. */
  samplePos  : vec3f,
  M          : f32,
  /** Normal at x2. Needed for the reconnection Jacobian's cosine. */
  sampleNrm  : vec3f,
  wSum       : f32,
  /** Outgoing radiance at x2 toward the visible point that generated it. */
  radiance   : vec3f,
  W          : f32,
  /** x1 of the pixel holding this sample — the other end of the connection. */
  visiblePos : vec3f,
  targetPdf  : f32,
}

fn emptyGIReservoir() -> GIReservoir {
  var r: GIReservoir;
  r.samplePos = vec3f(0.0);
  r.sampleNrm = vec3f(0.0, 1.0, 0.0);
  r.radiance = vec3f(0.0);
  r.visiblePos = vec3f(0.0);
  r.M = 0.0;
  r.wSum = 0.0;
  r.W = 0.0;
  r.targetPdf = 0.0;
  return r;
}

/**
 * Target function: the luminance of what this sample actually contributes.
 *
 * evalBSDF already folds in the cosine, so this is |f_s * cos * L_o| — the
 * integrand the estimator is trying to be proportional to.
 */
fn giTarget(m: Material, n: vec3f, v: vec3f, dir: vec3f, rad: vec3f) -> f32 {
  return luminance(evalBSDF(m, n, v, dir) * rad);
}

/**
 * Reconnection Jacobian for moving a sample from one visible point to another.
 *
 * The sample is fixed in area measure at x2, but the reservoir is carried in
 * solid angle at the visible point, and dw = cos(theta_x2) dA / d^2. So the
 * density changes by the ratio of those two factors between the new and old
 * connections. Omitting this is the classic silent ReSTIR bias: the image still
 * converges, just to the wrong answer, most visibly where geometry is at a
 * grazing angle to the reused sample.
 */
fn giJacobian(r: GIReservoir, newVisible: vec3f) -> f32 {
  let toOld = r.visiblePos - r.samplePos;
  let toNew = newVisible - r.samplePos;
  let dOld2 = max(dot(toOld, toOld), 1e-6);
  let dNew2 = max(dot(toNew, toNew), 1e-6);
  let cosOld = abs(dot(r.sampleNrm, toOld * inverseSqrt(dOld2)));
  let cosNew = abs(dot(r.sampleNrm, toNew * inverseSqrt(dNew2)));
  if (cosOld < 1e-6) { return 0.0; }
  let j = (cosNew / dNew2) * (dOld2 / cosOld);
  // A shift that lands at a near-grazing angle produces an enormous Jacobian
  // and a single blinding pixel. Clamping trades a little bias for stability;
  // ReSTIR implementations universally do this.
  return clamp(j, 0.0, 16.0);
}

/** Weighted-reservoir-sampling update with an already-computed weight. */
fn giUpdate(
  r: ptr<function, GIReservoir>,
  samplePos: vec3f, sampleNrm: vec3f, radiance: vec3f, visiblePos: vec3f,
  weight: f32, targetPdf: f32, count: f32,
) {
  (*r).wSum = (*r).wSum + weight;
  (*r).M = (*r).M + count;
  if (weight > 0.0 && rand() * (*r).wSum < weight) {
    (*r).samplePos = samplePos;
    (*r).sampleNrm = sampleNrm;
    (*r).radiance = radiance;
    (*r).visiblePos = visiblePos;
    (*r).targetPdf = targetPdf;
  }
}

fn finalizeGIReservoir(r: ptr<function, GIReservoir>) {
  let denom = (*r).M * (*r).targetPdf;
  if (denom <= 0.0) {
    (*r).W = 0.0;
  } else {
    // Same cap as the other RIS paths in this file, and for the same reason:
    // an unlucky near-zero target pdf would otherwise produce one huge pixel.
    (*r).W = min((*r).wSum / denom, 32.0);
  }
}


/**
 * Direct lighting from transient lights only — muzzle flashes, detonations.
 *
 * Deliberately plain: no resampling, no reservoir, one shadow ray per light.
 * There are only ever a handful of these, they are extremely bright, and they
 * exist for a few frames. Resampling them would buy nothing and would mean
 * carrying reservoirs that outlive their own light.
 *
 * The result goes to its own signal, which is never temporally accumulated, so
 * a flash appears and disappears exactly when the light does.
 */
/**
 * Distance from `p` to the nearest live transient light.
 *
 * Drives how hard the transient signal is filtered. Returns a large value when
 * no flash is live, which reads as "blur freely" — correct, since there is no
 * transient energy to protect.
 */
fn nearestTransientDist(p: vec3f) -> f32 {
  var best = 1e6;
  for (var i = U.transientStart; i < U.lightCount; i = i + 1u) {
    if (lights[i].intensity <= 0.0) { continue; }
    best = min(best, distance(p, lights[i].pos));
  }
  return best;
}

fn sampleTransientLights(
  p: vec3f, n: vec3f, v: vec3f, m: Material, samples: u32,
) -> vec3f {
  let ns = max(1u, samples);
  var sum = vec3f(0.0);
  for (var i = U.transientStart; i < U.lightCount; i = i + 1u) {
    let l = lights[i];
    if (l.intensity <= 0.0) { continue; }
    var acc = vec3f(0.0);
    for (var k = 0u; k < ns; k = k + 1u) {
      let s = sampleSphereLight(l, p);
      var atten = 1.0;
      if (l.kind == LIGHT_SPOT) {
        atten = spotAttenuation(l.dir, -s.dir, l.cosInner, l.cosOuter);
        if (atten <= 0.001) { continue; }
      }
      let contrib = evalBSDF(m, n, v, s.dir) * s.radiance * atten;
      if (luminance(contrib) < SHADOW_CULL) { continue; }
      if (occluded(p + n * EPS * 4.0, s.dir, s.dist - EPS * 8.0)) { continue; }
      acc = acc + contrib;
    }
    sum = sum + acc / f32(ns);
  }
  return sum;
}
