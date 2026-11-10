// What every stage shares: the frame, the scene's buffers, a random stream, and how a ray meets one piece or one leaf. (The two ray queries: rays.wgsl.)
struct Frame {
  camPos: vec4f, camRight: vec4f, camUp: vec4f, camFwd: vec4f,     // right and up are scaled by tan(fov / 2); right by the aspect too
  prevPos: vec4f, prevRight: vec4f, prevUp: vec4f, prevFwd: vec4f,
  sunDir: vec4f,       // w = the disc's angular radius
  sunE: vec4f,         // irradiance on a surface facing it
  size: vec4f,         // render size, output size
  jitter: vec4f,       // this frame's, last frame's, in render pixels
  misc: vec4f,         // time (s), cut height, exposure, haze
  knobs: vec4f,        // x y free for a stage under study; z where the grid of what is joined starts in extra; w how lit the city's windows are
  counts: vec4u,       // lamps, openings, frame, seed
  per: vec4u,          // nodes per octant of the four trees: standing pieces the section cuts, standing pieces it does not, moving pieces, leaf clusters
  node0: vec4u,        // where each starts in nodes
  step: vec4u,         // from one octant's order to the next (0: one order serves all)
  piece0: vec4u,       // where each one's pieces start (the fourth's are instances)
  pool: vec4f,         // the water: x0 z0 x1 z1
  pool2: vec4f,        // its level, its depth
  weather: vec4f,      // how wet the open ground is, how hard it rains, is there power, the wind's way (rad)
  misc2: vec4f,        // is there smoke, where the blue noise starts in extra, 0, how hard the wind blows (1)
  tune: array<vec4f, 8>, more: vec4f,       // how much snow lies, how hard it snows, time since the last frame (s), 0;  // what can be turned while looking (tune.ts)
};
struct Piece { r0: vec4f, r1: vec4f, r2: vec4f, half: vec3f, flags: u32, quad: vec4f, mat: u32, tint: u32, planes: u32, lamp: u32 };
struct Node { mn: vec3f, skip: u32, mx: vec3f, leaf: u32 };
struct Inst { r0: vec4f, r1: vec4f, r2: vec4f, node: u32, per: u32, leaf: u32, tint: u32 };
struct LeafP { at: vec3f, size: f32, n: u32, t: u32, tone: f32, kind: u32 };
struct Lamp { at: vec3f, kind: u32, u: vec3f, cone: f32, v: vec3f, area: f32, n: vec3f, on: f32, radiance: vec3f, reach: f32 };
struct Opening { at: vec3f, area: f32, u: vec3f, a: f32, v: vec3f, b: f32, n: vec3f, c: f32 };
struct Mat { base: vec3f, kind: u32, rough: f32, metal: f32, f0: f32, glow: f32, b2: vec3f, scale: f32 };
struct Mark { at: vec2f, size: vec2f, uv: vec4f, color: vec3f, face: u32 };

@group(0) @binding(0) var<uniform> frame: Frame;
@group(0) @binding(1) var<storage, read> nodes: array<Node>;
@group(0) @binding(2) var<storage, read> pieces: array<Piece>;
@group(0) @binding(3) var<storage, read> extra: array<vec4f>;        // planes, then marks (4 rows each), then leaves (2 rows), by the offsets below
@group(0) @binding(4) var<storage, read> insts: array<Inst>;
@group(0) @binding(5) var<uniform> lamps: array<Lamp, 64>;
@group(0) @binding(6) var<uniform> openings: array<Opening, 32>;
@group(0) @binding(7) var<uniform> mats: array<Mat, 64>;
@group(0) @binding(8) var linSamp: sampler;
@group(0) @binding(9) var skyTex: texture_2d<f32>;
@group(0) @binding(10) var artTex: texture_2d<f32>;
@group(0) @binding(12) var causticTex: texture_2d<f32>;
@group(0) @binding(13) var repSamp: sampler;
@group(0) @binding(14) var lampTex: texture_3d<f32>;
@group(0) @binding(15) var smokeTex: texture_3d<f32>;       // the pool's bricks, each with a cell of its neighbours round it: light scattered per unit, and how much smoke per metre
@group(0) @binding(16) var smokePage: texture_3d<u32>;      // which of them stands where (+ 1)

const GLASS = 256u; const WATER = 512u; const CUT = 1024u; const GLOW = 2048u; const NOSHADOW = 4096u; const QUAD = 8192u; const MOVES = 16384u; const FAR = 32768u; const OPEN = 0x10000000u; const BEND = 0x20000000u;
const NOTHING = 0xffffffffu; const LEAF_HIT = 0x80000000u;

// ---- random numbers: a stream per pixel and frame, the same every run
var<private> rngState: u32;
fn rngInit(p: vec2u, salt: u32) { rngState = pcg(p.x + pcg(p.y + pcg(frame.counts.z + pcg(salt + frame.counts.w)))); }
fn rngKey(k: u32, salt: u32) { rngState = pcg(k + pcg(frame.counts.z + pcg(salt + frame.counts.w))); }
var<private> qIndex: u32; var<private> qSeed: u32; var<private> qDim: u32;
fn owen(x0: u32, seed: u32) -> u32 { var x = reverseBits(x0); x ^= x * 0x3d20adeau; x += seed; x *= (seed >> 16u) | 1u; x ^= x * 0x05526c56u; x ^= x * 0x53a22864u; return reverseBits(x); }
fn sobol2(i0: u32) -> u32 { var i = i0; var r = 0u; var v = 0x80000000u; for (var k = 0u; k < 20u; k++) { if (i == 0u) { break; } if ((i & 1u) == 1u) { r ^= v; } i >>= 1u; v ^= v >> 1u; } return r; }
fn quasi() -> vec2f { qDim += 1u; let s = pcg(qSeed + qDim * 0x9e3779b9u); let i = owen(qIndex, s) ; return vec2f(f32(owen(reverseBits(i), pcg(s + 1u)) >> 8u), f32(owen(sobol2(i), pcg(s + 2u)) >> 8u)) * (1.0 / 16777216.0); }
fn rnd() -> f32 { if (QUASI) { return quasi().x; } rngState = rngState * 747796405u + 2891336453u; let w = ((rngState >> ((rngState >> 28u) + 4u)) ^ rngState) * 277803737u; return f32(((w >> 22u) ^ w) >> 8u) * (1.0 / 16777216.0); }
fn rnd2() -> vec2f { if (QUASI) { return quasi(); } return vec2f(rnd(), rnd()); }
/** blue noise over the screen, stepped through the frames by the golden ratio: neighbours differ, and so do frames */
fn blue2(p: vec2u, k: u32) -> vec2f {
  let o = vec2u((k * 23u) & 63u, (k * 41u) & 63u);
  let c = (p + o) & vec2u(63u); let i = c.y * 64u + c.x; let r = extra[u32(frame.misc2.y) + (i >> 1u)]; let n = select(r.xy, r.zw, (i & 1u) == 1u);
  return fract(n + f32(frame.counts.z & 1023u) * vec2f(0.61803399, 0.75487767));
}
// ---- the sky: one function, one image (its mips are the blur)
fn skyLod(d: vec3f, lod: f32) -> vec3f { return textureSampleLevel(skyTex, linSamp, octEnc(d), lod).xyz; }
fn sky(d: vec3f) -> vec3f { return skyLod(d, 0.0); }

// ================================================================ rays
struct Hit { t: f32, piece: u32, code: u32, thru: vec3f, gt: f32, gpiece: u32, gcode: u32 };   // g = the nearest pane before it
var<private> hT: f32; var<private> hCode: u32;
const CODE_QUAD = 6u; const CODE_CUT = 15u;

// A BLOCK OF CELLS, in a piece's own frame: one row (cells along x y z, their size), then a word a place (a cell's number + 1), then three rows a cell: depth along x y z and the share of leaves · the tight box's
// corners · six colors · six normals and how alike they are. A ray goes through it as through fog, and by a die stops where a cell's tight box begins.
const VOX = 128u; const VOX_CODE = 4096u; var<private> voxDie = 1u;
fn voxWord(at: u32, i: u32) -> u32 { return bitcast<vec4u>(extra[at + (i >> 2u)])[i & 3u]; }
struct Vox { t: f32, cell: u32, axis: u32, tau: f32 };
fn voxWalk(k: u32, o: vec3f, d: vec3f, t0: f32, t1: f32, want: f32) -> Vox {
  var v: Vox; v.cell = 0u; v.tau = 0.0; let at = pieces[k].planes; let head = extra[at]; let n = vec3i(head.xyz); let h = head.w; let half = pieces[k].half; let words = at + 1u; let cells = words + (u32(n.x * n.y * n.z) + 3u) / 4u;
  let flat = abs(d) < vec3f(1e-9); let inv = 1.0 / select(d, vec3f(1e-9), flat); let ahead = select(vec3f(0.0), vec3f(1.0), d > vec3f(0.0)); var t = t0; var c = clamp(vec3i(floor((o + d * (t + 1e-5) + half) / h)), vec3i(0), n - 1);
  for (var i = 0u; i < 220u; i++) {
    if (t >= t1) { break; } let e = select(((vec3f(c) + ahead) * h - half - o) * inv, vec3f(1e9), flat); let out = min(min3(e), t1); let w = voxWord(words, u32(c.x + n.x * (c.y + n.y * c.z)));
    if (w != 0u) {
      let r = bitcast<vec4u>(extra[cells + (w - 1u) * 3u]); let a8 = vec3f(vec3u(r.y, r.y >> 8u, r.y >> 16u) & vec3u(255u)); let z8 = vec3f((vec3u(r.z, r.z >> 8u, r.z >> 16u) & vec3u(255u)) + vec3u(1u)); let lo = (vec3f(c) + a8 / 256.0) * h - half; let hi = (vec3f(c) + z8 / 256.0) * h - half;
      let u = (lo - o) * inv; let z = (hi - o) * inv; let mn = select(min(u, z), select(vec3f(1e9), vec3f(-1e9), (o > lo) & (o < hi)), flat); let mx = select(max(u, z), vec3f(1e9), flat); let ta = max(t, max3(mn)); let tb = min(out, min3(mx));
      /* (the cell a ray begins at does not shade itself) */ if (tb > ta && !(all(o > lo - 0.004) && all(o < hi + 0.004))) { let sg = length(vec3f(vec3u(r.x, r.x >> 8u, r.x >> 16u) & vec3u(255u)) * (6.4 / 255.0) / (hi - lo) * d); let dep = sg * (tb - ta); if (v.tau + dep >= want) { v.t = ta; v.cell = w; v.axis = select(select(2u, 1u, mn.y >= mn.z), 0u, mn.x >= mn.y && mn.x >= mn.z); return v; } v.tau += dep; }
    }
    t = out; if (e.x <= e.y && e.x <= e.z) { c.x += select(-1, 1, d.x > 0.0); } else if (e.y <= e.z) { c.y += select(-1, 1, d.y > 0.0); } else { c.z += select(-1, 1, d.z > 0.0); } if (any(c < vec3i(0)) || any(c >= n)) { break; }
  }
  return v;
}
/** one piece: the overlap of the stays its box, its quadric and its planes allow. Sets hT and hCode */
fn meet(k: u32, ro: vec3f, rd: vec3f, tmin: f32, tmax: f32, cut: f32) -> bool {
  let p = &pieces[k]; let flags = (*p).flags; if ((flags & OPEN) != 0u && cut < OPEN_FLOOR) { return false; }
  let o = vec3f(dot((*p).r0.xyz, ro) + (*p).r0.w, dot((*p).r1.xyz, ro) + (*p).r1.w, dot((*p).r2.xyz, ro) + (*p).r2.w);
  let d = vec3f(dot((*p).r0.xyz, rd), dot((*p).r1.xyz, rd), dot((*p).r2.xyz, rd));
  let inv = 1.0 / select(d, vec3f(1e-12), abs(d) < vec3f(1e-12));
  let h = (*p).half; let a = (-h - o) * inv; let b = (h - o) * inv; let mn = min(a, b); let mx = max(a, b);
  var tn = max3(mn); var tf = min3(mx);
  if (tn >= tf || tf < tmin || tn >= tmax) { return false; }
  var code = select(select(select(5u, 4u, d.z > 0.0), select(3u, 2u, d.y > 0.0), tn == mn.y), select(1u, 0u, d.x > 0.0), tn == mn.x);   // 0 -x 1 +x 2 -y 3 +y 4 -z 5 +z
  if ((flags & VOX) != 0u) { let a = max(tn, tmin); let b = min(tf, tmax); if (a >= b) { return false; } let v = voxWalk(k, o, d, a, b, -log(1.0 - f32(pcg(voxDie ^ (k * 747796405u)) >> 8u) / 16777217.0)); if (v.cell == 0u) { return false; } hT = v.t; hCode = VOX_CODE + v.cell * 4u + v.axis; return true; }
  // from where the ray enters its box: from far off, a thin round thing's roots are lost in the rounding
  let sh = tn; let e0 = o + d * sh;
  if ((flags & QUAD) != 0u) {
    let q = (*p).quad; let qa = q.xyz * d;
    let A = dot(qa, d); let B = dot(qa, e0) + 0.5 * q.w * d.y; let C = dot(q.xyz * e0, e0) + q.w * e0.y - 1.0;
    let disc = B * B - A * C;
    if (A > 1e-9) {
      if (disc <= 0.0) { return false; }
      let s = sqrt(disc); let t0 = (-B - s) / A + sh; let t1 = (-B + s) / A + sh;
      if (t0 > tn) { tn = t0; code = CODE_QUAD; } tf = min(tf, t1);
    } else if (A < -1e-9) {                                          // along a taper: inside is outside the two roots, on the nappe that is really there
      if (disc > 0.0) {
        let s = sqrt(disc); let t0 = (-B + s) / A + sh; let t1 = (-B - s) / A + sh;    // (A < 0: t0 < t1)
        if (1.0 - 0.5 * q.w * (o.y + d.y * t0) > 0.0) { tf = min(tf, t0); } else { if (t1 > tn) { tn = t1; code = CODE_QUAD; } }
      }
    } else if (abs(B) > 1e-9) { let t0 = sh - 0.5 * C / B; if (B > 0.0) { tf = min(tf, t0); } else if (t0 > tn) { tn = t0; code = CODE_QUAD; } } else if (C > 0.0) { return false; }
  }
  let np = flags & 15u;
  for (var i = 0u; i < np; i++) {
    let pl = extra[(*p).planes + i]; let den = dot(pl.xyz, d); let num = pl.w - dot(pl.xyz, e0);
    if (abs(den) < 1e-12) { if (num < 0.0) { return false; } continue; }
    let t = num / den + sh; if (den < 0.0) { if (t > tn) { tn = t; code = 7u + i; } } else { tf = min(tf, t); }
  }
  let nc = (flags >> 4u) & 3u; for (var i = 0u; i < nc; i++) {        // the curves: each one stay of its own
    let q = extra[(*p).planes + np + 2u * i].xyz; let e = e0 - extra[(*p).planes + np + 2u * i + 1u].xyz; let A = dot(q * d, d); let B = dot(q * d, e); let C = dot(q * e, e) - 1.0;
    if (A < 1e-9) { if (C > 0.0) { return false; } continue; } let disc = B * B - A * C; if (disc <= 0.0) { return false; } let s = sqrt(disc); let t0 = (-B - s) / A + sh; if (t0 > tn) { tn = t0; code = 16u + i; } tf = min(tf, (-B + s) / A + sh);
  }
  if ((flags & WATER) != 0u) {                                       // water: somewhere in its box, where the ray meets the waves
    let a = max(tn, tmin); if (a >= tf) { return false; } let n = 5; let st = (tf - a) / f32(n); var t0 = a; var f0 = o.y + d.y * a - waveHeight(ro.xz + rd.xz * a, frame.misc.x); let above = f0 > 0.0; var found = false;
    for (var i = 1; i <= n; i++) { let t1 = a + st * f32(i); let f1 = o.y + d.y * t1 - waveHeight(ro.xz + rd.xz * t1, frame.misc.x); if ((f1 > 0.0) != above) { var l = t0; var fl = f0; var r = t1; var fr = f1; for (var j = 0; j < 3; j++) { let m = l + (r - l) * clamp(fl / (fl - fr), 0.02, 0.98); let fm = o.y + d.y * m - waveHeight(ro.xz + rd.xz * m, frame.misc.x); if ((fm > 0.0) == above) { l = m; fl = fm; } else { r = m; fr = fm; } } tn = l + (r - l) * clamp(fl / (fl - fr), 0.0, 1.0); found = true; break; } t0 = t1; f0 = f1; }
    if (!found) { return false; } code = select(2u, 3u, above);
  }
  if (cut < 1e8 && (flags & CUT) != 0u) {                            // the section: whatever stands over it is not there to this ray
    if (abs(rd.y) > 1e-9) { let t = (cut - ro.y) / rd.y; if (rd.y < 0.0) { if (t > tn) { tn = t; code = CODE_CUT; } } else { tf = min(tf, t); } } else if (ro.y > cut) { return false; }
  }
  if (tn >= tf || tn < tmin || tn >= tmax) { return false; }
  hT = tn; hCode = code; return true;
}
fn boxHit(n: ptr<storage, Node, read>, ro: vec3f, inv: vec3f, tmin: f32, tmax: f32) -> bool {
  let a = ((*n).mn - ro) * inv; let b = ((*n).mx - ro) * inv; let mn = min(a, b); let mx = max(a, b);
  return max(max3(mn), tmin) <= min(min3(mx), tmax);
}
fn octant(d: vec3f) -> u32 { return select(0u, 1u, d.x < 0.0) | select(0u, 2u, d.y < 0.0) | select(0u, 4u, d.z < 0.0); }
fn safeInv(d: vec3f) -> vec3f { return 1.0 / select(d, vec3f(1e-12), abs(d) < vec3f(1e-12)); }
/** how wide to how long: a leaf, a blade of grass, a twig */
fn leafWide(kind: u32) -> f32 { return select(select(0.46, 0.06, kind == 1u), 0.022, kind == 2u); }
/** a leaf: a pointed oval, seen from both sides */
fn meetLeaf(k: u32, ro: vec3f, rd: vec3f, tmin: f32, tmax: f32) -> f32 {
  let a = extra[k * 2u]; let b = bitcast<vec4u>(extra[k * 2u + 1u]);
  let n = octUnpack(b.x); let den = dot(n, rd); if (abs(den) < 1e-6) { return -1.0; }
  let t = dot(a.xyz - ro, n) / den; if (t <= tmin || t >= tmax) { return -1.0; }
  let x = ro + rd * t - a.xyz; let tl = octUnpack(b.y); let u = dot(x, tl) / a.w; let v = dot(x, cross(n, tl)) / a.w;
  let w = leafWide(b.w & 255u) * (1.0 - u * u) * (1.0 + 0.35 * u);
  return select(-1.0, t, abs(u) < 1.0 && abs(v) < w);
}

/** where a point is in the sun's map of the leaves: 0‥1, 0‥1, and how far along (1 = toward the sun) */
fn sunMap(x: vec3f) -> vec3f { let b = basis(frame.sunDir.xyz); let d = x - SUN_AT; return vec3f(dot(d, b[0]) / SUN_WIDE + 0.5, dot(d, b[1]) / SUN_WIDE + 0.5, dot(d, frame.sunDir.xyz) / 80.0 + 0.5); }
// ================================================================ smoke
const SM_AT = vec3f(-24.0, -0.16, -19.2); const SM_SIZE = vec3f(48.0, 15.36, 38.4); const SM_BRICK = 0.64;
fn smokeSlot(x: vec3f) -> u32 { let c = vec3i(floor((x - SM_AT) / SM_BRICK)); if (any(c < vec3i(0)) || any(c >= vec3i(75, 24, 60))) { return 0u; } return textureLoad(smokePage, c, 0).x; }
fn smokeBlock(x: vec3f) -> bool { return smokeSlot(x) != 0u; }
fn smokeIn(p: u32, x: vec3f) -> vec4f { let s = p - 1u; let l = fract((x - SM_AT) / SM_BRICK) * 8.0; return textureSampleLevel(smokeTex, linSamp, (vec3f(f32(s & 15u), f32((s >> 4u) & 15u), f32(s >> 8u)) * 10.0 + 1.0 + l) / vec3f(160.0, 160.0, 120.0), 0.0); }
fn smokeAt(x: vec3f) -> vec4f { let p = smokeSlot(x); if (p == 0u) { return vec4f(0.0); } return smokeIn(p, x); }
/** where a ray is in the grid */
fn smokeSpan(ro: vec3f, rd: vec3f, t1: f32) -> vec2f { let inv = safeInv(rd); let a = (SM_AT - ro) * inv; let b = (SM_AT + SM_SIZE - ro) * inv; return vec2f(max(max3(min(a, b)), 0.0), min(min3(max(a, b)), t1)); }
/** smoke by the same rule: in steps half as long as the ray is wide there, from the cells of 8 cm where it is narrow, from a whole brick's mean where it is wider than one. Light, and how much gets through */
fn smokeWalk(ro: vec3f, rd: vec3f, t1: f32, lit: bool) -> vec4f {
  var o = vec4f(0.0, 0.0, 0.0, 1.0); if (frame.misc2.x < 0.5) { return o; } let s = smokeSpan(ro, rd, t1); if (s.y <= s.x) { return o; } let inv = safeInv(rd); let ahead = select(vec3f(0.0), vec3f(1.0), rd > vec3f(0.0)); var t = s.x + 1e-3; var tau = 0.0;
  for (var i = 0u; i < 192u; i++) {
    if (t >= s.y || tau > 5.0) { break; } let x = ro + rd * t; let f = footAt(t);
    if (smokeSlot(x) == 0u) { let e = (SM_AT + (floor((x - SM_AT) / SM_BRICK) + ahead) * SM_BRICK - ro) * inv; t = max(max(min3(e), t) + 2e-3, t + 0.25 * f); continue; }
    let dt = min(max(max(0.12, 0.5 * f), (s.y - s.x) / 96.0), s.y - t); let y = x + rd * (0.5 * dt); let p = smokeSlot(y);
    if (p != 0u) { var v: vec4f; if (f > SM_BRICK) { v = extra[BLK_SMOKE + p - 1u]; } else { v = smokeIn(p, y); if (max3(v.rgb) > 24.0) { v = vec4f(0.0, 0.0, 0.0, v.w); } } let d = v.w * dt; if (lit) { o = vec4f(o.rgb + v.rgb * (exp(-tau) * (1.0 - exp(-d))), 1.0); } tau += d; }      /* (a flame lights by its lamp: rays that are blurred see only its smoke) */
    t += dt;
  }
  o.w = exp(-tau); return o;
}
fn smokeSeen(ro: vec3f, rd: vec3f, t1: f32) -> vec4f { return smokeWalk(ro, rd, t1, true); }
fn smokeThrough(ro: vec3f, rd: vec3f, t1: f32) -> f32 { return smokeWalk(ro, rd, t1, false).w; }
/** the camera's ray through a point of the picture (0‥1, y down) */
fn camRay(uv: vec2f) -> vec3f { let p = uv * 2.0 - 1.0; return normalize(frame.camFwd.xyz + frame.camRight.xyz * p.x - frame.camUp.xyz * p.y); }
/** where a world point is in the picture, now or last frame (z = its distance along the view; <= 0 behind) */
fn project(x: vec3f) -> vec3f { let v = x - frame.camPos.xyz; let z = dot(v, frame.camFwd.xyz); let r = frame.camRight.xyz; let u = frame.camUp.xyz; return vec3f(dot(v, r) / dot(r, r) / z * 0.5 + 0.5, 0.5 - dot(v, u) / dot(u, u) / z * 0.5, z); }
fn projectPrev(x: vec3f) -> vec3f { let v = x - frame.prevPos.xyz; let z = dot(v, frame.prevFwd.xyz); let r = frame.prevRight.xyz; let u = frame.prevUp.xyz; return vec3f(dot(v, r) / dot(r, r) / z * 0.5 + 0.5, 0.5 - dot(v, u) / dot(u, u) / z * 0.5, z); }
