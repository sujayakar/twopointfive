// ---- shared declarations: frame uniforms, scene buffers, ray queries against the box grid, light evaluation ----
// (WORLD_* / GRID_* / RC_* consts are prepended by the host.)

struct Frame {
  viewProj: mat4x4f,
  invViewProj: mat4x4f,
  prevViewProj: mat4x4f,
  camPos: vec3f, time: f32,
  camDir: vec3f, dt: f32,
  screen: vec2f, invScreen: vec2f,          // full-res target size
  frameIdx: u32, numLights: u32, flags: u32, exposure: f32,
  skyZenith: vec3f, hazeDensity: f32,
  skyHorizon: vec3f, rcInterval0: f32,      // base interval length (m)
  rcC0Dims: vec3u, rcNumCascades: u32,      // c0 probe grid dims (x,y,z)
  rcD0: u32, rcFrameParity: u32, numSmoke: u32, debugView: u32,
  indirectScale: f32, emissiveScale: f32, volSteps: u32, secMinE: f32,   // secMinE: at secondary vertices (cascade interval hits) a light dimmer than this (unshadowed W/m²) is not worth a shadow ray and is dropped
  capColor: vec4f,                          // rgb = section-cap tint for wall tops under the hidden ceiling, a = use albedo fraction
  post: vec4f,                              // x = night-vision amount, y = nv gain, z = hit flash, w = tan(fov/2) (metres → pixels for blur sizing)
  rcJitter: vec4f,                          // xy = this frame's sub-texel offset for the cascades' octahedral direction set (0.5,0.5 = texel centres); z = direct-light adaptive sampling: stop after this many agreeing samples; w = number of transient lights alive this frame
  directCfg: vec4f,                         // x = emitter samples for the strongest light per class, y = samples for the other leaders, z = penumbra filter cap (px), w = direct-light history weight (0 = none)
  lossyCfg: vec4f,                          // the lossy options' numbers (defaults = the untouched path): x = final-gather resolution divisor (2; 3 with gatherThird), y = volumetrics divisor (2; 4 with volQuarter), z = dimRays threshold on a pixel's total unshadowed luminance below which leaders get one shadow sample (0 = off), w spare
};

const FLAG_DIRECT: u32 = 1u;
const FLAG_INDIRECT: u32 = 2u;
const FLAG_VOLUMETRICS: u32 = 4u;
const FLAG_SMOKE: u32 = 8u;
const FLAG_RC_VISWEIGHT: u32 = 16u;
const FLAG_SMOKE_SHADOWS: u32 = 64u;
const FLAG_BOUNCE: u32 = 128u;
const FLAG_DITHER: u32 = 256u;
const FLAG_SOFTSHADOW: u32 = 512u;
const FLAG_NIGHTVISION: u32 = 1024u;
const FLAG_TEMPORAL: u32 = 2048u;
const FLAG_TILECULL: u32 = 4096u;    // direct pass: per-8x8-tile light list (lights provably zero across the tile's bounds are not scored per pixel) — lossless; off = walk every light, for A/B
const FLAG_SOFT_TILESKIP: u32 = 8192u; // penumbra filter: whole 8x8 tiles provably out of reach of every blur hint skip the pass (softtile.wgsl builds the mask) — lossless; off = every pixel decides for itself, for A/B
const FLAG_TAIL_CHROMA: u32 = 16384u; // experiment: per-channel-unbiased tail (E_pick·V/p) instead of the bounded luminance-unbiased form
const FLAG_SMOKE_RSKIP: u32 = 32768u; // smoke samplers: skip the atlas fetch wherever the solver's per-brick render occupancy proves the trilinear footprint all zero — lossless; off = always fetch, for A/B
const FLAG_CHECKER_DIRECT: u32 = 65536u; // lossy option: shadow rays on a checkerboard half of the pixels per frame, the other half borrows its neighbours' visibility (direct.wgsl)
const FLAG_GRID_SKIP: u32 = 131072u;  // trace grid: run past provably empty cells without touching memory (per-cell Chebyshev distance to the nearest occupied cell) — lossless; off = a load pair per cell as before, for A/B
const FLAG_GRID_YCULL: u32 = 262144u; // trace grid: skip cells / boxes (and globals) whose height band the rest of the ray cannot reach (per-cell 24-slice occupancy, per-item quantised y extent, per-global extent) — lossless; off = every registered box is slab-tested, for A/B
const FLAG_AXIS_BOX: u32 = 524288u;   // slab test: unrotated boxes (rot packed as exactly cos 1, sin 0) skip the rotation and reuse the ray's inverse direction — lossless (see isectBox); off = always rotate + divide, for A/B
const FLAG_GRID_SLABS: u32 = 1048576u; // trace grid: a global is skipped unless the ray SEGMENT's bounds touch its own 5 mm-proud bounds on all three axes (not just a 4 cm height band), and a cell's height span is judged scene.grid.y tighter at both ends — lossless (see traceClosest); off = the height cull exactly as before, for A/B

struct BoxGeo { c: vec3f, rot: u32, h: vec3f, flags: u32 };
const BOX_NOPRIMARY: u32 = 1u;
const BOX_NOSHADOW: u32 = 2u;
const BOX_CUTAWAY: u32 = 4u;
const BOX_EMISSIVE: u32 = 16u;
const BOX_AREALIT: u32 = 32u;     // emission carried by an analytic area light instead (see Light kind 3)
const BOX_ROT: u32 = 64u;         // fully rotated box: orientation is the quaternion boxRot(i), not the packed yaw (hand props, sparks, tumbling canisters)

struct SceneInfo { numBoxes: u32, numGlobals: u32, gridW: u32, gridH: u32, globals: array<vec4u, 4>, numCapsules: u32, numChars: u32, capsPerChar: u32, itemsB: u32, charBounds: array<vec4f, 8>, globalsY: array<vec4f, 8>, grid: vec4f, globalsB: array<vec4f, 32> };   // itemsB: offset of the packed item copies in gridItems (= item count); globalsY: padded (ymin, ymax) of global 2i in .xy, of global 2i+1 in .zw; grid.x: registration slack in metres (how far a traced box can reach outside the cells it is registered in — boxes.ts buildGrid), grid.y: FLAG_GRID_SLABS' height-span shrink per end (m); globalsB[2g] / [2g+1]: min / max corner of global g's world bounds, 5 mm + slack proud of the traced box (FLAG_GRID_SLABS)
// Character shadow proxies: analytic capsules (segment a→b, radius r). `misc` packs owner (low 8 bits) and rgb565 albedo (bits 8..23).
struct Capsule { a: vec3f, r: f32, b: vec3f, misc: u32 };

struct Light {
  pos: vec3f, kind: u32,          // 0 point, 1 spot, 2 directional, 3 area (one-sided Lambertian disk: normal = dir, radius = disk radius, color = radiance × area)
  dir: vec3f, range: f32,
  color: vec3f, cosOuter: f32,
  cosInner: f32, volumetric: f32, owner: u32, radius: f32,
};

struct SmokeDomain {
  origin: vec3f, voxel: f32,       // world min corner, voxel size (m)
  atlasOffset: vec3u, live: u32,   // texel offset of this domain inside the atlas
  dims: vec3u, pad: u32,
  densityScale: f32, pad1: f32, pad2: f32, pad3: f32,
};

@group(0) @binding(0) var<uniform> frame: Frame;
@group(0) @binding(1) var<storage, read> boxGeo: array<BoxGeo>;
@group(0) @binding(2) var<storage, read> boxMat: array<vec4f>;
@group(0) @binding(3) var<storage, read> gridCells: array<u32>;   // [0, W*H]: per-cell start offsets into gridItems (cell ci owns [start(ci), start(ci+1))); from GRID_CB on: 2 words per cell for the fast traversal (see gridCell below)
@group(0) @binding(4) var<storage, read> gridItems: array<u32>;   // [0, itemsB): box indices per cell in box order; [itemsB, 2·itemsB): the same items packed index16 | qmin8<<16 | qmax8<<24 (quantised padded height extent)
@group(0) @binding(5) var<uniform> scene: SceneInfo;
@group(0) @binding(6) var<storage, read> lights: array<Light>;   // (tried a uniform array: slower for the cascades' hit shading on Apple GPUs)
@group(0) @binding(7) var<uniform> smoke: array<SmokeDomain, 8>;
@group(0) @binding(8) var smokeAtlas: texture_3d<f32>;   // rgba16f: density, temperature, -, -
@group(0) @binding(9) var linSamp: sampler;
@group(0) @binding(10) var<storage, read> capsules: array<Capsule>;
@group(0) @binding(11) var<uniform> smokeOcc: array<vec4u, 32>;   // smoke render occupancy, written by the solver at the end of every domain step (renderOccPack in smoke.wgsl):
                                                                    // domain i owns u32 words 16i..16i+15 (flat word w lives at [w >> 2][w & 3]); bit b set = 8³-cell brick b, or the one-cell shell round it, holds density

fn boxAlbedo(i: u32) -> vec3f { return boxMat[i * 3u].xyz; }
fn boxCutHeight(i: u32) -> f32 { return boxMat[i * 3u].w; }
fn boxEmissive(i: u32) -> vec3f { return boxMat[i * 3u + 1u].xyz * frame.emissiveScale; }
/** raster orientation of box i as a unit quaternion (equals its yaw for ordinary boxes; raster-only props may tilt) */
fn boxRot(i: u32) -> vec4f { return boxMat[i * 3u + 2u]; }
fn quatRotate(q: vec4f, v: vec3f) -> vec3f { let t = 2.0 * cross(q.xyz, v); return v + q.w * t + cross(q.xyz, t); }
fn boxOwner(b: BoxGeo) -> u32 { return (b.flags >> 8u) & 0xffu; }

fn skyRadiance(rd: vec3f) -> vec3f {
  let t = clamp(rd.y * 2.5, 0.0, 1.0);
  let below = clamp(-rd.y * 8.0, 0.0, 1.0);
  return mix(mix(frame.skyHorizon, frame.skyZenith, t), frame.skyHorizon * 0.15, below);
}

// Ray vs axis-aligned bounds → (tEnter, tExit)
fn isectBounds(ro: vec3f, invRd: vec3f, bmin: vec3f, bmax: vec3f) -> vec2f {
  let t0 = (bmin - ro) * invRd; let t1 = (bmax - ro) * invRd;
  let tmn = min(t0, t1); let tmx = max(t0, t1);
  return vec2f(max(max(tmn.x, tmn.y), tmn.z), min(min(tmx.x, tmx.y), tmx.z));
}

fn safeInv(v: vec3f) -> vec3f {
  return vec3f(select(1.0 / v.x, 1e20, abs(v.x) < 1e-20), select(1.0 / v.y, 1e20, abs(v.y) < 1e-20), select(1.0 / v.z, 1e20, abs(v.z) < 1e-20));
}

// Ray vs yawed (or, with BOX_ROT, fully rotated) box `bi`. Returns vec2(t, code). t < 0 → miss. code: axis*2 + (1 if local normal negative), 7 = origin inside.
// invRd = safeInv(rd) from the caller; axisFast (FLAG_AXIS_BOX): a box with yaw 0 packs rot as exactly (1.0, 0.0) = 0x3C00, and for those the
// local frame IS the world frame — 1·x − 0·z evaluates to x exactly whatever the contraction, so lo = p, ld = rd and inv = invRd bit for bit —
// which skips two half unpacks, both rotations and the three divisions for two thirds of all items and every global. Lossless; off = always rotate.
const ROT_IDENTITY: u32 = 0x3C00u;
fn isectBox(ro: vec3f, rd: vec3f, b: BoxGeo, tmax: f32, bi: u32, invRd: vec3f, axisFast: bool) -> vec2f {
  let p = ro - b.c;
  var lo: vec3f; var ld: vec3f; var inv: vec3f;
  if ((b.flags & BOX_ROT) != 0u) { let qi = vec4f(-boxRot(bi).xyz, boxRot(bi).w); lo = quatRotate(qi, p); ld = quatRotate(qi, rd); inv = safeInv(ld); }   // rare: one extra fetch + two quaternion rotations
  else if (axisFast && b.rot == ROT_IDENTITY) { lo = p; ld = rd; inv = invRd; }
  else { let cs = unpack2x16float(b.rot); lo = vec3f(cs.x * p.x - cs.y * p.z, p.y, cs.y * p.x + cs.x * p.z); ld = vec3f(cs.x * rd.x - cs.y * rd.z, rd.y, cs.y * rd.x + cs.x * rd.z); inv = safeInv(ld); }
  let tA = (-b.h - lo) * inv; let tB = (b.h - lo) * inv;
  let tmn = min(tA, tB); let tmx = max(tA, tB);
  let tN = max(max(tmn.x, tmn.y), tmn.z);
  let tF = min(min(tmx.x, tmx.y), tmx.z);
  if (tN > tF || tF < 0.0 || tN > tmax) { return vec2f(-1.0, 0.0); }
  if (tN < 0.0) { return vec2f(0.0, 7.0); }
  var code = 4.0 + select(0.0, 1.0, ld.z > 0.0);
  if (tN == tmn.x) { code = select(0.0, 1.0, ld.x > 0.0); }
  else if (tN == tmn.y) { code = 2.0 + select(0.0, 1.0, ld.y > 0.0); }
  return vec2f(tN, code);
}

fn boxNormalFromCode(b: BoxGeo, code: f32, bi: u32) -> vec3f {
  let c = u32(code);
  let sgn = select(1.0, -1.0, (c & 1u) == 1u);
  let axis = c >> 1u;
  if ((b.flags & BOX_ROT) != 0u) { var ln = vec3f(0.0); ln[axis] = sgn; return quatRotate(boxRot(bi), ln); }
  let cs = unpack2x16float(b.rot);
  if (axis == 0u) { return vec3f(cs.x, 0.0, -cs.y) * sgn; }   // local +x → world (cos,0,-sin)
  if (axis == 1u) { return vec3f(0.0, sgn, 0.0); }
  if (axis == 2u) { return vec3f(cs.y, 0.0, cs.x) * sgn; }    // local +z → world (sin,0,cos)
  return vec3f(0.0, 1.0, 0.0);
}

struct Hit { t: f32, idx: i32, n: vec3f, inside: bool, capAlbedo: vec3f };
const CAPSULE_HIT: i32 = 0x7ffffff0;

fn capsuleOwner(c: Capsule) -> u32 { return c.misc & 0xffu; }
/** `skip` packs up to two character ids (bits 0-7: usually the receiving pixel's owner; bits 8-15: the light's owner) whose boxes / capsules a ray ignores. */
fn skipsOwner(owner: u32, skip: u32) -> bool { return owner != 0u && (owner == (skip & 0xffu) || owner == ((skip >> 8u) & 0xffu)); }
fn capsuleAlbedo(c: Capsule) -> vec3f { let p = c.misc >> 8u; return vec3f(f32((p >> 11u) & 31u) / 31.0, f32((p >> 5u) & 63u) / 63.0, f32(p & 31u) / 31.0); }

// Ray vs capsule (iq). Returns t of the nearest intersection in front of the origin, or -1.
fn isectCapsule(ro: vec3f, rd: vec3f, pa: vec3f, pb: vec3f, ra: f32) -> f32 {
  let ba = pb - pa; let oa = ro - pa;
  let baba = dot(ba, ba); let bard = dot(ba, rd); let baoa = dot(ba, oa); let rdoa = dot(rd, oa); let oaoa = dot(oa, oa);
  // origin inside the capsule → occluded from t = 0 (matches the old box-proxy 'inside' behaviour: contact shadows under prone bodies)
  let hq = clamp(baoa / baba, 0.0, 1.0); let dq = oa - ba * hq;
  if (dot(dq, dq) < ra * ra) { return 0.0; }
  var a = baba - bard * bard; var b = baba * rdoa - baoa * bard; var c = baba * oaoa - baoa * baoa - ra * ra * baba;
  var h = b * b - a * c;
  if (a > 1e-7 && h >= 0.0) {
    let t = (-b - sqrt(h)) / a;
    let y = baoa + t * bard;
    if (y > 0.0 && y < baba) { return t; }             // body
    let oc = select(ro - pb, oa, y <= 0.0);              // nearer cap
    b = dot(rd, oc); c = dot(oc, oc) - ra * ra; h = b * b - c;
    if (h > 0.0) { return -b - sqrt(h); }
    return -1.0;
  }
  // ray (anti)parallel to the axis (or grazing): test both cap spheres
  var best = -1.0;
  for (var k = 0; k < 2; k++) {
    let oc = select(oa, ro - pb, k == 1);
    let bb = dot(rd, oc); let cc = dot(oc, oc) - ra * ra; let hh = bb * bb - cc;
    if (hh > 0.0) { let t = -bb - sqrt(hh); if (t >= 0.0 && (best < 0.0 || t < best)) { best = t; } }
  }
  return best;
}
fn capsuleNormal(p: vec3f, pa: vec3f, pb: vec3f) -> vec3f {
  let ba = pb - pa; let h = clamp(dot(p - pa, ba) / dot(ba, ba), 0.0, 1.0);
  return normalize(p - (pa + h * ba));
}
fn raySphere(ro: vec3f, rd: vec3f, ce: vec3f, ra: f32, tmax: f32) -> bool {
  let oc = ro - ce; let b = dot(oc, rd); let c = dot(oc, oc) - ra * ra;
  if (c <= 0.0) { return true; }
  let h = b * b - c; if (h < 0.0 || b > 0.0) { return false; }
  return (-b - sqrt(h)) <= tmax;
}
// Closest capsule hit within tmax (skipping `skipOwner`). Returns (t, index) with index < 0 on miss.
fn traceCapsules(ro: vec3f, rd: vec3f, tmax: f32, skipOwner: u32) -> vec2f {
  var best = tmax; var bi = -1.0;
  for (var ch = 0u; ch < scene.numChars; ch++) {
    let bs = scene.charBounds[ch];
    if (!raySphere(ro, rd, bs.xyz, bs.w, best)) { continue; }
    let first = ch * scene.capsPerChar; let last = min(first + scene.capsPerChar, scene.numCapsules);
    for (var i = first; i < last; i++) {
      let cp = capsules[i];
      if (skipsOwner(capsuleOwner(cp), skipOwner)) { break; }   // whole character shares the owner
      let t = isectCapsule(ro, rd, cp.a, cp.b, cp.r);
      if (t >= 0.0 && t < best) { best = t; bi = f32(i); }
    }
  }
  return vec2f(best, bi);
}
fn occludedCapsules(ro: vec3f, rd: vec3f, tmax: f32, skipOwner: u32) -> bool {
  for (var ch = 0u; ch < scene.numChars; ch++) {
    let bs = scene.charBounds[ch];
    if (!raySphere(ro, rd, bs.xyz, bs.w, tmax)) { continue; }
    let first = ch * scene.capsPerChar; let last = min(first + scene.capsPerChar, scene.numCapsules);
    for (var i = first; i < last; i++) {
      let cp = capsules[i];
      if (skipsOwner(capsuleOwner(cp), skipOwner)) { break; }
      let t = isectCapsule(ro, rd, cp.a, cp.b, cp.r);
      if (t >= 0.0 && t < tmax) { return true; }
    }
  }
  return false;
}

// ---- trace-grid fast path (FLAG_GRID_SKIP / FLAG_GRID_YCULL; data written by boxes.ts buildGrid every upload, next to the plain arrays) ----
// gridCells[GRID_CB + 2ci]     = heightMask (bit s ⇔ some box registered in ci covers quantised heights [10s, 10s+9]) | cheby << 24
// gridCells[GRID_CB + 2ci + 1] = start (24 bits) | min(count, 255) << 24          cheby = Chebyshev distance (cells) to the nearest occupied cell, 0 = occupied
// gridItems[itemsB + k]        = box index (16 bits) | qmin << 16 | qmax << 24     q = gridYq of the box's y extent widened by 4 cm each way
// scene.globalsY               = the same padded extent per global, in metres (globalMissesY)
//
// Why both are exact (same hit.t / idx / code, same occlusion answers, flag on or off):
//  * Empty runs. An empty cell has no items, so not loading its (equal) start/end pair changes nothing; a cell at Chebyshev distance d from
//    every occupied cell is followed, whatever the ray does, by d−1 more cells within d−1 king moves of it — all empty too. The DDA still takes
//    exactly one loop iteration per cell with the very same float stepping and the same break tests, so `iter` counts what it always counted
//    (and GRID_W + GRID_H ≤ 96 is asserted host-side: a ray leaves the grid after at most W + H steps, the cap never binds either way).
//  * Height cull. Every test the cull drops is one that could not have changed anything. The DDA visits the cells along the ray in t order and
//    a box is registered in every cell its footprint overlaps, so by the time the walk enters cell k at tCell, any box that the ray meets at a
//    point BEHIND tCell was already met in the (earlier) cell holding that point: traceClosest already has hit.t ≤ that t (the re-test is a
//    no-op under the strict <), occluded / occludedT already returned. What is left are hits ahead, at t in [tCell, tEnd] (tEnd = tmax, or the
//    current hit.t), i.e. at heights between y(tCell) and y(tEnd) — inside the box, hence inside its stored [ymin, ymax], hence inside the
//    cell's mask. A cell whose mask misses the quantised span, or an item whose extent misses it, therefore only ever produced misses / no-ops,
//    and dropping those leaves the sequence of state-changing tests, their order and the hit state they see untouched (ties included). The
//    4 cm pad on the stored extents dwarfs everything float in y: f32 vs f64 quantiser boundaries (~1e-7 m), y(t) rounding (~1e-5 m), the
//    drift of the accumulated tNext against the true cell lines (< 1e-4 m of height over a whole ray), corner tie attribution (~1e-6 m).
//    "Registered in every cell it can be hit in" is true of the f64 footprint, but the box the GPU traces is made of f32 centres / half sizes /
//    origin offsets and, when yawed, f16 cos/sin: its faces can sit up to scene.grid.x metres (measured per build; ~1e-5 axis-aligned, mm for a
//    yawed car) outside the registered cells. A ray grazing such a sliver meets the box just BEFORE the line into its first registered cell,
//    i.e. up to slack/|rd ⟂ line| behind tCell — so the span a cell is judged by starts that far back (tBack: slack × the tDelta of the axis
//    just stepped, which is exactly 1/|rd ⟂|), clamped to the DDA start. For all but grazing rays that is a few micrometres of extra height.
//    (Nothing can lie behind the DDA start itself: every caller's origin is inside WORLD_MIN/MAX, so t0 = 0 — an origin-inside answer comes
//    from the start cell, whose span holds ro.y — and an origin outside the world has no box before its entry point.)
//  * Tighter spans (FLAG_GRID_SLABS). Tallying those terms: the y-cull needs the stored extents to out-reach the tracer by the accumulated tNext
//    drift (≤ 68 steps × ½ ulp(64 m) ≈ 2.6e-4 m of height), y(t) and cell-entry rounding (~2e-5), the f32-vs-f64 quantiser inputs (~2e-6) and
//    the ~2e-5 by which a reported hit can sit outside its box (below) — under 3.5e-4 m in all — and they carry 4 cm. So the span a cell is judged
//    by may be pulled in by scene.grid.y = 4 cm − 5 mm at BOTH ends without a hittable box failing either per-end test (each end was argued on
//    its own above), still with a 5 mm margin, 14× the tally. When the pull-in crosses the ends over (a span under 7 cm: level rays), qlo > qhi
//    only makes both per-item tests stricter, and an item passing both has qmin ≤ qhi < qlo ≤ qmax — its slices cover the whole hull [qhi, qlo] —
//    so the cell is walked only if its mask holds EVERY hull slice (an ordered span: any). That is what stops a ray lifted 1–2 cm off a desk,
//    floor finish or wall top by its pass from slab-testing the surface it just left (its padded band reached up past the origin; nothing
//    else about the walk changes).
//  * Globals by segment bounds (FLAG_GRID_SLABS). If isectBox reports t ≥ 0 for box b there is a t* ∈ [0, tmax] (the reported t, or 0 from
//    inside) lying in every axis' slab interval, and the point ro + rd·t* taken exactly sits within |lo| · 3·2⁻²⁴ + ½ ulp ≈ 2e-5 m of the box
//    on each axis (lo = ro − c and the two slab distances are each one rounding off; yaw 0 makes the local frame the world frame bit for bit, a
//    yawed global's f16 outline is inside footprint + slack by construction of the slack). Every coordinate of that point also lies between ro
//    and pEnd = ro + rd·tmax as computed (affine in t; pEnd is off by ≤ two roundings below 128 m ≈ 1.5e-5). So a global whose bounds — footprint and height
//    extent + slack + 5 mm (boxes.ts GLOBAL_PAD) — clear [min(ro, pEnd), max(ro, pEnd)] on ANY axis cannot report a hit: 5 mm against < 4e-5.
//    A skipped global answered "miss" before, and a miss moves nothing in any of the three queries (closest: no state; any-hit: no early out),
//    so the survivors run in the same order against the same state. The old test was this on y alone with 4 cm: every ray lifted 1–2 cm off a
//    floor still slab-tested all four floor-level globals (ground, carpet base, two finishes) — four of its ~ten box tests — for nothing.
struct CellWalk { start: u32, end: u32, qlo: u32, qhi: u32 };   // items [start, end) to test in this cell; quantised height span for the per-item test
fn gridYq(y: f32) -> u32 { return u32(clamp((y - GRID_YQ0) * GRID_YQS, 0.0, GRID_YQMAX)); }
fn gridSliceMask(qlo: u32, qhi: u32) -> u32 { let slo = qlo / 10u; let shi = qhi / 10u; return (0xffffffu >> (23u - (shi - slo))) << slo; }
// What to walk in cell ci for a ray whose remaining stretch spans heights ya..yb (either order), judged `shrink` tighter at each end (0, or
// scene.grid.y under FLAG_GRID_SLABS). `run` counts down the cells an earlier empty cell's Chebyshev distance has already vouched for: those
// cost no memory traffic at all.
fn gridCell(ci: u32, ya: f32, yb: f32, run: ptr<function, u32>, gskip: bool, ycull: bool, shrink: f32) -> CellWalk {
  var w: CellWalk; w.start = 0u; w.end = 0u; w.qlo = 0u; w.qhi = 255u;
  if (*run > 0u) { *run = *run - 1u; return w; }
  let w0 = gridCells[GRID_CB + 2u * ci];
  let dist = select(0u, w0 >> 24u, gskip);
  if (dist > 0u) { *run = dist - 1u; return w; }
  var m = 0xffffffu;
  var need = 1u;                       // how much of m the cell's mask must hold: any slice of an ordered span; ALL of a crossed one (see above)
  if (ycull) { w.qlo = gridYq(min(ya, yb) + shrink); w.qhi = gridYq(max(ya, yb) - shrink); m = gridSliceMask(min(w.qlo, w.qhi), max(w.qlo, w.qhi)); need = select(1u, m, w.qlo > w.qhi); }   // (shrink 0: + 0.0 and the min/max of an ordered pair are identities, need stays 1 — the flag-off walk is value for value the old one)
  if ((w0 & m) < need) { return w; }   // ordered: (w0 & m) == 0; crossed: (w0 & m) != m, i.e. some hull slice has no box in this cell
  let w1 = gridCells[GRID_CB + 2u * ci + 1u];
  w.start = w1 & 0xffffffu;
  let n = w1 >> 24u;
  if (n == 255u) { w.end = gridCells[ci + 1u]; } else { w.end = w.start + n; }   // 255 = long cell: take the plain end
  return w;
}
// Box index of item k, or −1 when the height cull proves the box outside the ray's remaining span (cull on: reads the packed copy instead).
fn gridItem(k: u32, w: CellWalk, ycull: bool) -> i32 {
  if (!ycull) { return i32(gridItems[k]); }
  let it = gridItems[scene.itemsB + k];
  if ((it >> 24u) < w.qlo || ((it >> 16u) & 255u) > w.qhi) { return -1; }
  return i32(it & 0xffffu);
}
// The globals (huge boxes tested once per ray, all thin horizontal slabs in practice) get the same treatment in float: global g whose padded
// [ymin, ymax] misses the heights the ray covers over [0, tmax] can only answer "miss" — a hit or an origin-inside answer places a ray point of
// that t range inside the box — and a miss changes nothing in any of the three queries, so not asking is exact.
fn globalMissesY(g: u32, ySpan: vec2f) -> bool {
  let gy = scene.globalsY[g >> 1u];
  let e = select(gy.xy, gy.zw, (g & 1u) == 1u);
  return e.y < ySpan.x || e.x > ySpan.y;
}
// FLAG_GRID_SLABS: the same on all three axes against the segment's bounds [sLo, sHi] = [min(ro, pEnd), max(ro, pEnd)] and the global's own
// 5 mm-proud bounds (argument above): outside on any axis ⇒ it can only answer "miss".
fn globalOutside(g: u32, sLo: vec3f, sHi: vec3f) -> bool {
  return any(scene.globalsB[2u * g + 1u].xyz < sLo) || any(scene.globalsB[2u * g].xyz > sHi);
}
// Which of the two global rejects applies to global g for this ray (slabs: the three-axis one; else the height band when the y-cull is on).
fn globalSkipped(g: u32, sLo: vec3f, sHi: vec3f, ycull: bool, slabs: bool) -> bool {
  if (slabs) { return globalOutside(g, sLo, sHi); }
  return ycull && globalMissesY(g, vec2f(sLo.y, sHi.y));
}

// Closest hit through globals + XZ uniform grid. skipOwner: boxes owned by this character id are ignored (0 = none).
fn traceClosest(ro: vec3f, rd: vec3f, tmaxIn: f32, skipOwner: u32) -> Hit {
  var hit: Hit; hit.t = tmaxIn; hit.idx = -1; hit.inside = false; hit.n = vec3f(0.0, 1.0, 0.0);
  let invRd = safeInv(rd);
  let tb = isectBounds(ro, invRd, WORLD_MIN, WORLD_MAX);
  if (tb.x > tb.y || tb.y < 0.0) { return hit; }
  var tmax = min(tmaxIn, tb.y);
  hit.t = tmax;
  var code = 0.0;
  var found = false;
  let gskip = (frame.flags & FLAG_GRID_SKIP) != 0u; let ycull = (frame.flags & FLAG_GRID_YCULL) != 0u; let fast = gskip || ycull; let axisFast = (frame.flags & FLAG_AXIS_BOX) != 0u;
  let slabs = (frame.flags & FLAG_GRID_SLABS) != 0u; let shrink = select(0.0, scene.grid.y, slabs);
  let yFar = ro.y + rd.y * tmax;
  let pEnd = vec3f(ro.x + rd.x * tmax, yFar, ro.z + rd.z * tmax);
  let sLo = min(ro, pEnd); let sHi = max(ro, pEnd);   // the segment's bounds over [0, tmax]: a global clear of them (its padded height band without FLAG_GRID_SLABS, its padded box with it) can only answer "miss", so it is not asked
  for (var g = 0u; g < scene.numGlobals; g++) {
    if (globalSkipped(g, sLo, sHi, ycull, slabs)) { continue; }
    let bi = scene.globals[g >> 2u][g & 3u];
    let b = boxGeo[bi];
    let r = isectBox(ro, rd, b, hit.t, bi, invRd, axisFast);
    if (r.x >= 0.0 && r.x < hit.t) { hit.t = r.x; hit.idx = i32(bi); code = r.y; found = true; }
  }
  var run = 0u;
  // 2D DDA
  let t0 = max(tb.x, 0.0);
  let p = ro + rd * t0;
  var cell = vec2i(clamp(vec2i(floor(p.xz / GRID_CELL)), vec2i(0), vec2i(GRID_W - 1, GRID_H - 1)));
  let stp = vec2i(select(-1, 1, rd.x >= 0.0), select(-1, 1, rd.z >= 0.0));
  let tDelta = vec2f(abs(GRID_CELL * invRd.x), abs(GRID_CELL * invRd.z));
  let nextBoundary = (vec2f(cell) + vec2f(select(0.0, 1.0, rd.x >= 0.0), select(0.0, 1.0, rd.z >= 0.0))) * GRID_CELL;
  var tNext = vec2f(t0) + (nextBoundary - p.xz) * vec2f(invRd.x, invRd.z);
  tNext = select(tNext, vec2f(1e30), vec2<bool>(abs(rd.x) < 1e-20, abs(rd.z) < 1e-20));
  var tCell = t0;
  var tBack = 0.0;                     // slack × tDelta of the axis last stepped (= slack / |rd ⟂ that line|): how far behind tCell a registered box may already have been met (see above)
  let slack = scene.grid.x / GRID_CELL;
  for (var iter = 0; iter < 96; iter++) {
    if (tCell > hit.t) { break; }
    let ci = u32(cell.y * GRID_W + cell.x);
    var cw: CellWalk;
    if (fast) { cw = gridCell(ci, ro.y + rd.y * max(tCell - tBack, t0), ro.y + rd.y * hit.t, &run, gskip, ycull, shrink); }
    else { cw.start = gridCells[ci]; cw.end = gridCells[ci + 1u]; cw.qlo = 0u; cw.qhi = 255u; }
    for (var k = cw.start; k < cw.end; k++) {
      let gi = gridItem(k, cw, ycull); if (gi < 0) { continue; }
      let bi = u32(gi);
      let b = boxGeo[bi];
      if (skipsOwner(boxOwner(b), skipOwner)) { continue; }
      let r = isectBox(ro, rd, b, hit.t, bi, invRd, axisFast);
      if (r.x >= 0.0 && r.x < hit.t) { hit.t = r.x; hit.idx = i32(bi); code = r.y; found = true; }
    }
    if (tNext.x < tNext.y) { tCell = tNext.x; tNext.x += tDelta.x; cell.x += stp.x; tBack = tDelta.x * slack; }
    else { tCell = tNext.y; tNext.y += tDelta.y; cell.y += stp.y; tBack = tDelta.y * slack; }
    if (tCell > tmax || cell.x < 0 || cell.y < 0 || cell.x >= GRID_W || cell.y >= GRID_H) { break; }
  }
  if (found) {
    if (code > 6.5) { hit.inside = true; hit.n = -rd; }
    else { hit.n = boxNormalFromCode(boxGeo[u32(hit.idx)], code, u32(hit.idx)); }
  } else { hit.idx = -1; hit.t = tmaxIn; }
  // character capsules (few, sphere-culled per character)
  if (scene.numCapsules > 0u) {
    let ch = traceCapsules(ro, rd, hit.t, skipOwner);
    if (ch.y >= 0.0) {
      let cp = capsules[u32(ch.y)];
      hit.t = ch.x; hit.idx = CAPSULE_HIT; hit.inside = false; hit.capAlbedo = capsuleAlbedo(cp);
      hit.n = capsuleNormal(ro + rd * ch.x, cp.a, cp.b);
    }
  }
  return hit;
}

// Any-hit visibility query. Returns true if something blocks [0, tmax].
fn occluded(ro: vec3f, rd: vec3f, tmaxIn: f32, skipOwner: u32) -> bool {
  let invRd = safeInv(rd);
  let tb = isectBounds(ro, invRd, WORLD_MIN, WORLD_MAX);
  if (tb.x > tb.y || tb.y < 0.0) { return false; }
  let tmax = min(tmaxIn, tb.y);
  let gskip = (frame.flags & FLAG_GRID_SKIP) != 0u; let ycull = (frame.flags & FLAG_GRID_YCULL) != 0u; let fast = gskip || ycull; let axisFast = (frame.flags & FLAG_AXIS_BOX) != 0u;
  let slabs = (frame.flags & FLAG_GRID_SLABS) != 0u; let shrink = select(0.0, scene.grid.y, slabs);
  let yEnd = ro.y + rd.y * tmax;
  let pEnd = vec3f(ro.x + rd.x * tmax, yEnd, ro.z + rd.z * tmax);
  let sLo = min(ro, pEnd); let sHi = max(ro, pEnd);
  for (var g = 0u; g < scene.numGlobals; g++) {
    if (globalSkipped(g, sLo, sHi, ycull, slabs)) { continue; }
    let bi = scene.globals[g >> 2u][g & 3u];
    let r = isectBox(ro, rd, boxGeo[bi], tmax, bi, invRd, axisFast);
    if (r.x >= 0.0) { return true; }
  }
  var run = 0u;
  let t0 = max(tb.x, 0.0);
  let p = ro + rd * t0;
  var cell = vec2i(clamp(vec2i(floor(p.xz / GRID_CELL)), vec2i(0), vec2i(GRID_W - 1, GRID_H - 1)));
  let stp = vec2i(select(-1, 1, rd.x >= 0.0), select(-1, 1, rd.z >= 0.0));
  let tDelta = vec2f(abs(GRID_CELL * invRd.x), abs(GRID_CELL * invRd.z));
  let nextBoundary = (vec2f(cell) + vec2f(select(0.0, 1.0, rd.x >= 0.0), select(0.0, 1.0, rd.z >= 0.0))) * GRID_CELL;
  var tNext = vec2f(t0) + (nextBoundary - p.xz) * vec2f(invRd.x, invRd.z);
  tNext = select(tNext, vec2f(1e30), vec2<bool>(abs(rd.x) < 1e-20, abs(rd.z) < 1e-20));
  var tCell = t0;
  var tBack = 0.0;
  let slack = scene.grid.x / GRID_CELL;
  for (var iter = 0; iter < 96; iter++) {
    let ci = u32(cell.y * GRID_W + cell.x);
    var cw: CellWalk;
    if (fast) { cw = gridCell(ci, ro.y + rd.y * max(tCell - tBack, t0), yEnd, &run, gskip, ycull, shrink); }
    else { cw.start = gridCells[ci]; cw.end = gridCells[ci + 1u]; cw.qlo = 0u; cw.qhi = 255u; }
    for (var k = cw.start; k < cw.end; k++) {
      let gi = gridItem(k, cw, ycull); if (gi < 0) { continue; }
      let bi = u32(gi);
      let b = boxGeo[bi];
      if (skipsOwner(boxOwner(b), skipOwner)) { continue; }
      let r = isectBox(ro, rd, b, tmax, bi, invRd, axisFast);
      if (r.x >= 0.0) { return true; }
    }
    if (tNext.x < tNext.y) { tCell = tNext.x; tNext.x += tDelta.x; cell.x += stp.x; tBack = tDelta.x * slack; }
    else { tCell = tNext.y; tNext.y += tDelta.y; cell.y += stp.y; tBack = tDelta.y * slack; }
    if (tCell > tmax || cell.x < 0 || cell.y < 0 || cell.x >= GRID_W || cell.y >= GRID_H) { break; }
  }
  if (scene.numCapsules > 0u) { return occludedCapsules(ro, rd, tmax, skipOwner); }
  return false;
}

// Shadow ray that also reports roughly WHERE it was blocked: distance to the first blocker found (capsules exact-nearest,
// boxes in DDA order ≈ near-first), or -1 when the segment is clear. Feeds the PCSS-style penumbra-width hint.
fn occludedT(ro: vec3f, rd: vec3f, tmaxIn: f32, skipOwner: u32) -> f32 {
  var tCap = -1.0;
  if (scene.numCapsules > 0u) { let c = traceCapsules(ro, rd, tmaxIn, skipOwner); if (c.y >= 0.0) { tCap = c.x; } }
  let invRd = safeInv(rd);
  let tb = isectBounds(ro, invRd, WORLD_MIN, WORLD_MAX);
  if (tb.x > tb.y || tb.y < 0.0) { return tCap; }
  let tmax = select(min(tmaxIn, tb.y), tCap, tCap >= 0.0);      // a nearer box than the capsule still wins below
  let gskip = (frame.flags & FLAG_GRID_SKIP) != 0u; let ycull = (frame.flags & FLAG_GRID_YCULL) != 0u; let fast = gskip || ycull; let axisFast = (frame.flags & FLAG_AXIS_BOX) != 0u;
  let slabs = (frame.flags & FLAG_GRID_SLABS) != 0u; let shrink = select(0.0, scene.grid.y, slabs);
  let yEnd = ro.y + rd.y * tmax;
  let pEnd = vec3f(ro.x + rd.x * tmax, yEnd, ro.z + rd.z * tmax);
  let sLo = min(ro, pEnd); let sHi = max(ro, pEnd);
  for (var g = 0u; g < scene.numGlobals; g++) {
    if (globalSkipped(g, sLo, sHi, ycull, slabs)) { continue; }
    let bi = scene.globals[g >> 2u][g & 3u];
    let r = isectBox(ro, rd, boxGeo[bi], tmax, bi, invRd, axisFast);
    if (r.x >= 0.0) { return r.x; }
  }
  var run = 0u;
  let t0 = max(tb.x, 0.0);
  let p = ro + rd * t0;
  var cell = vec2i(clamp(vec2i(floor(p.xz / GRID_CELL)), vec2i(0), vec2i(GRID_W - 1, GRID_H - 1)));
  let stp = vec2i(select(-1, 1, rd.x >= 0.0), select(-1, 1, rd.z >= 0.0));
  let tDelta = vec2f(abs(GRID_CELL * invRd.x), abs(GRID_CELL * invRd.z));
  let nextBoundary = (vec2f(cell) + vec2f(select(0.0, 1.0, rd.x >= 0.0), select(0.0, 1.0, rd.z >= 0.0))) * GRID_CELL;
  var tNext = vec2f(t0) + (nextBoundary - p.xz) * vec2f(invRd.x, invRd.z);
  tNext = select(tNext, vec2f(1e30), vec2<bool>(abs(rd.x) < 1e-20, abs(rd.z) < 1e-20));
  var tCell = t0;
  var tBack = 0.0;
  let slack = scene.grid.x / GRID_CELL;
  for (var iter = 0; iter < 96; iter++) {
    let ci = u32(cell.y * GRID_W + cell.x);
    var cw: CellWalk;
    if (fast) { cw = gridCell(ci, ro.y + rd.y * max(tCell - tBack, t0), yEnd, &run, gskip, ycull, shrink); }
    else { cw.start = gridCells[ci]; cw.end = gridCells[ci + 1u]; cw.qlo = 0u; cw.qhi = 255u; }
    var bestInCell = -1.0;
    for (var k = cw.start; k < cw.end; k++) {
      let gi = gridItem(k, cw, ycull); if (gi < 0) { continue; }
      let bi = u32(gi);
      let b = boxGeo[bi];
      if (skipsOwner(boxOwner(b), skipOwner)) { continue; }
      let r = isectBox(ro, rd, b, tmax, bi, invRd, axisFast);
      if (r.x >= 0.0 && (bestInCell < 0.0 || r.x < bestInCell)) { bestInCell = r.x; }
    }
    if (bestInCell >= 0.0) { return bestInCell; }
    if (tNext.x < tNext.y) { tCell = tNext.x; tNext.x += tDelta.x; cell.x += stp.x; tBack = tDelta.x * slack; }
    else { tCell = tNext.y; tNext.y += tDelta.y; cell.y += stp.y; tBack = tDelta.y * slack; }
    if (tCell > tmax || cell.x < 0 || cell.y < 0 || cell.x >= GRID_W || cell.y >= GRID_H) { break; }
  }
  return tCap;
}

// ---- smoke density at a point: the one sampler behind every smoke read (volumetric march + self-shadow, shadow-ray / beam / probe transmittance) ----
const SMOKE_BRICK: u32 = 8u;   // cells per occupancy-brick edge = smoke.ts BRICK: a 64³ domain is 8×8×8 bricks = 512 bits = the 16 words per domain in smokeOcc (smoke.ts asserts both)
// May a trilinear fetch at texel-space coordinate `tex` of domain slot `dom` (cell dims `dims`) return anything but exactly 0? The footprint of a
// coordinate in cell c = floor(tex) stays within cells c−1..c+1 per axis, and bit(brick(c)) covers brick(c) plus exactly that one-cell shell —
// the full argument (incl. why the words always describe the atlas as it is this frame) sits above renderOccRegions in smoke.wgsl.
fn smokeMayHold(dom: u32, tex: vec3f, dims: vec3u) -> bool {
  let n = dims / SMOKE_BRICK;                                   // bricks per axis
  let b = min(vec3u(tex), dims - vec3u(1u)) / SMOKE_BRICK;      // brick of the cell holding tex (tex ≥ 0.5, so the truncation is a floor)
  let bi = (b.z * n.y + b.y) * n.x + b.x;                       // = smoke.wgsl brickIndex within the domain
  let w = dom * 16u + (bi >> 5u);
  return ((smokeOcc[w >> 2u][w & 3u] >> (bi & 31u)) & 1u) != 0u;
}
fn smokeDensityAt(p: vec3f) -> f32 {
  var d = 0.0;
  let skipEmpty = (frame.flags & FLAG_SMOKE_RSKIP) != 0u;
  for (var i = 0u; i < frame.numSmoke; i++) {
    let sd = smoke[i];
    if (sd.live == 0u) { continue; }
    let l = (p - sd.origin) / (sd.voxel * vec3f(sd.dims));
    if (any(l <= vec3f(0.0)) || any(l >= vec3f(1.0))) { continue; }
    let tex = clamp(l * vec3f(sd.dims), vec3f(0.5), vec3f(sd.dims) - 0.5);
    if (skipEmpty && !smokeMayHold(i, tex, sd.dims)) { continue; }   // every texel the fetch could weight is zero: it would add exactly +0 (lossless skip, flag 32768)
    let atlasSize = vec3f(textureDimensions(smokeAtlas));
    let uvw = (vec3f(sd.atlasOffset) + tex) / atlasSize;
    d += textureSampleLevel(smokeAtlas, linSamp, uvw, 0.0).x * sd.densityScale;
  }
  return d;
}
fn smokeTransmittance(ro: vec3f, rd: vec3f, tmax: f32) -> f32 {
  if (frame.numSmoke == 0u || (frame.flags & FLAG_SMOKE_SHADOWS) == 0u) { return 1.0; }
  // find overlap with any active domain first (cheap reject)
  let invRd = safeInv(rd);
  var tEnter = 1e30; var tExit = -1e30;
  for (var i = 0u; i < frame.numSmoke; i++) {
    let sd = smoke[i];
    if (sd.live == 0u) { continue; }
    let tb = isectBounds(ro, invRd, sd.origin, sd.origin + sd.voxel * vec3f(sd.dims));
    if (tb.x <= tb.y && tb.y > 0.0 && tb.x < tmax) { tEnter = min(tEnter, max(tb.x, 0.0)); tExit = max(tExit, min(tb.y, tmax)); }
  }
  if (tExit <= tEnter) { return 1.0; }
  let steps = 8.0;
  let dt = (tExit - tEnter) / steps;
  var od = 0.0;
  for (var s = 0.5; s < steps; s += 1.0) { od += smokeDensityAt(ro + rd * (tEnter + s * dt)); }
  return exp(-od * dt * 6.0);
}

// ---- analytic lights ----
// Light.kind packs flags above the low byte: bit 8 = transient (muzzle flash etc.: kept out of every accumulated signal), bit 9 = broad emitter class
const LIGHT_TRANSIENT: u32 = 256u;
const LIGHT_BROAD: u32 = 512u;
const MAX_BLUR_PX: f32 = 32.0;   // encoding scale of the direct-light blur hint (|S.a| = px / MAX_BLUR_PX, clamped to 1); the live cap is frame.directCfg.z
fn lightKind(li: Light) -> u32 { return li.kind & 255u; }
fn lightTransient(li: Light) -> bool { return (li.kind & LIGHT_TRANSIENT) != 0u; }
fn lightBroad(li: Light) -> bool { return (li.kind & LIGHT_BROAD) != 0u; }

struct LightEval { L: vec3f, dist: vec3f, E: vec3f, ok: bool }; // dist.x = distance to light (for shadow ray)

fn lightWindow(d: f32, range: f32) -> f32 { let x = d / range; let w = clamp(1.0 - x * x * x * x, 0.0, 1.0); return w * w; }

// Irradiance at P with normal N from light li (before visibility). For volumetrics pass N = vec3f(0) to skip the cosine.
fn evalLight(li: Light, P: vec3f, N: vec3f) -> LightEval {
  var r: LightEval; r.ok = false; r.E = vec3f(0.0);
  let kind = lightKind(li);
  if (kind == 2u) {
    r.L = -li.dir; r.dist = vec3f(1e6, 0.0, 0.0);
    let c = select(max(dot(N, r.L), 0.0), 1.0, all(N == vec3f(0.0)));
    if (c <= 0.0) { return r; }
    r.E = li.color * c; r.ok = true; return r;
  }
  let d = li.pos - P; let dist2 = dot(d, d); let dist = sqrt(dist2);
  if (dist > li.range) { return r; }
  r.L = d / dist; r.dist = vec3f(dist, 0.0, 0.0);
  var att = lightWindow(dist, li.range) / max(dist2, li.radius * li.radius);
  if (kind == 1u) {
    let cd = dot(-r.L, li.dir);
    if (cd < li.cosOuter) { return r; }
    att *= smoothstep(li.cosOuter, li.cosInner, cd);
  } else if (kind == 3u) {
    let ce = dot(-r.L, li.dir);            // emitter-side cosine: Lambertian panel, dark from behind
    if (ce <= 0.0) { return r; }
    att *= ce;
  }
  let c = select(max(dot(N, r.L), 0.0), 1.0, all(N == vec3f(0.0)));
  if (c <= 0.0 || att <= 0.0) { return r; }
  r.E = li.color * (att * c); r.ok = true;
  return r;
}

// Sum of shadowed direct irradiance from all analytic lights at P,N. maxLum: skip shadow rays for negligible contributions.
// includeTransient: false for anything that feeds an accumulated signal (cascades, dice, gather history) so a one-frame flash cannot linger there
fn directIrradiance(P: vec3f, N: vec3f, skipOwner: u32, minE: f32, includeTransient: bool) -> vec3f {
  var E = vec3f(0.0);
  let Po = P + N * 0.01;
  for (var i = 0u; i < frame.numLights; i++) {
    let li = lights[i];
    if (!includeTransient && lightTransient(li)) { continue; }
    let ev = evalLight(li, P, N);
    if (!ev.ok) { continue; }
    if (max(ev.E.x, max(ev.E.y, ev.E.z)) < minE) { continue; }   // too dim to be worth a ray at a secondary vertex: dropped (half-crediting it unshadowed leaked far panels through walls once their range grew)
    if (occluded(Po, ev.L, ev.dist.x - 0.02, skipOwner | ((li.owner & 0xffu) << 8u))) { continue; }
    E += ev.E;
  }
  return E;
}

// ---- misc ----
fn luminance(c: vec3f) -> f32 { return dot(c, vec3f(0.2126, 0.7152, 0.0722)); }

fn worldFromDepth(pix: vec2f, depth: f32) -> vec3f {
  let ndc = vec4f(pix * frame.invScreen * vec2f(2.0, -2.0) + vec2f(-1.0, 1.0), depth, 1.0);
  let w = frame.invViewProj * ndc;
  return w.xyz / w.w;
}

// Octahedral mapping: unit dir <-> [0,1]^2
fn octEncode(n: vec3f) -> vec2f {
  let p = n.xz / (abs(n.x) + abs(n.y) + abs(n.z));
  let q = select(p, (1.0 - abs(p.yx)) * select(vec2f(-1.0), vec2f(1.0), p >= vec2f(0.0)), n.y < 0.0);
  return q * 0.5 + 0.5;
}
fn octDecode(uv: vec2f) -> vec3f {
  let f = uv * 2.0 - 1.0;
  var n = vec3f(f.x, 1.0 - abs(f.x) - abs(f.y), f.y);
  let t = clamp(-n.y, 0.0, 1.0);
  n.x += select(t, -t, n.x >= 0.0);
  n.z += select(t, -t, n.z >= 0.0);
  return normalize(n);
}

fn hash13(p: vec3f) -> f32 { var q = fract(p * 0.1031); q += dot(q, q.zyx + 31.32); return fract((q.x + q.y) * q.z); }
fn ign(pix: vec2f) -> f32 { return fract(52.9829189 * fract(0.06711056 * pix.x + 0.00583715 * pix.y)); }
