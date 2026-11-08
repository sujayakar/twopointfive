// What every stage shares: the frame, the scene's buffers, a random stream, and the two ray queries.
struct Frame {
  camPos: vec4f, camRight: vec4f, camUp: vec4f, camFwd: vec4f,     // right and up are scaled by tan(fov / 2); right by the aspect too
  prevPos: vec4f, prevRight: vec4f, prevUp: vec4f, prevFwd: vec4f,
  sunDir: vec4f,       // w = the disc's angular radius
  sunE: vec4f,         // irradiance on a surface facing it
  size: vec4f,         // render size, output size
  jitter: vec4f,       // this frame's, last frame's, in render pixels
  misc: vec4f,         // time (s), cut height, exposure, haze
  knobs: vec4f,        // free for a stage under study
  counts: vec4u,       // lamps, openings, frame, seed
  per: vec4u,          // nodes per octant of the four trees: standing pieces the section cuts, standing pieces it does not, moving pieces, leaf clusters
  node0: vec4u,        // where each starts in nodes
  piece0: vec4u,       // where each one's pieces start (the fourth's are instances)
  pool: vec4f,         // the water: x0 z0 x1 z1
  pool2: vec4f,        // its level, its depth
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
@group(0) @binding(11) var noiseTex: texture_2d<f32>;
@group(0) @binding(12) var causticTex: texture_2d<f32>;
@group(0) @binding(13) var repSamp: sampler;

const GLASS = 256u; const WATER = 512u; const CUT = 1024u; const GLOW = 2048u; const NOSHADOW = 4096u; const QUAD = 8192u; const MOVES = 16384u; const FAR = 32768u;
const NOTHING = 0xffffffffu; const LEAF_HIT = 0x80000000u;

// ---- random numbers: a stream per pixel and frame, the same every run
var<private> rngState: u32;
fn rngInit(p: vec2u, salt: u32) { rngState = pcg(p.x + pcg(p.y + pcg(frame.counts.z + pcg(salt + frame.counts.w)))); }
fn rngKey(k: u32, salt: u32) { rngState = pcg(k + pcg(frame.counts.z + pcg(salt + frame.counts.w))); }
fn rnd() -> f32 { rngState = rngState * 747796405u + 2891336453u; let w = ((rngState >> ((rngState >> 28u) + 4u)) ^ rngState) * 277803737u; return f32(((w >> 22u) ^ w) >> 8u) * (1.0 / 16777216.0); }
fn rnd2() -> vec2f { return vec2f(rnd(), rnd()); }
/** blue noise over the screen, stepped through the frames by the golden ratio: neighbours differ, and so do frames */
fn blue2(p: vec2u, k: u32) -> vec2f {
  let o = vec2u((k * 23u) & 63u, (k * 41u) & 63u);
  let n = textureLoad(noiseTex, (p + o) & vec2u(63u), 0).xy;
  return fract(n + f32(frame.counts.z & 1023u) * vec2f(0.61803399, 0.75487767));
}
// ---- the sky: one function, one image (its mips are the blur)
fn skyLod(d: vec3f, lod: f32) -> vec3f { return textureSampleLevel(skyTex, linSamp, octEnc(d), lod).xyz; }
fn sky(d: vec3f) -> vec3f { return skyLod(d, 0.0); }

// ================================================================ rays
struct Hit { t: f32, piece: u32, code: u32, thru: vec3f, gt: f32, gpiece: u32, gcode: u32 };   // g = the nearest pane before it
var<private> hT: f32; var<private> hCode: u32;
const CODE_QUAD = 6u; const CODE_CUT = 15u;

/** one piece: the overlap of the stays its box, its quadric and its planes allow. Sets hT and hCode */
fn meet(k: u32, ro: vec3f, rd: vec3f, tmin: f32, tmax: f32, cut: f32) -> bool {
  let p = &pieces[k]; let flags = (*p).flags;
  let o = vec3f(dot((*p).r0.xyz, ro) + (*p).r0.w, dot((*p).r1.xyz, ro) + (*p).r1.w, dot((*p).r2.xyz, ro) + (*p).r2.w);
  let d = vec3f(dot((*p).r0.xyz, rd), dot((*p).r1.xyz, rd), dot((*p).r2.xyz, rd));
  let inv = 1.0 / select(d, vec3f(1e-12), abs(d) < vec3f(1e-12));
  let h = (*p).half; let a = (-h - o) * inv; let b = (h - o) * inv; let mn = min(a, b); let mx = max(a, b);
  var tn = max3(mn); var tf = min3(mx);
  if (tn >= tf || tf < tmin || tn >= tmax) { return false; }
  var code = select(select(select(5u, 4u, d.z > 0.0), select(3u, 2u, d.y > 0.0), tn == mn.y), select(1u, 0u, d.x > 0.0), tn == mn.x);   // 0 -x 1 +x 2 -y 3 +y 4 -z 5 +z
  if ((flags & QUAD) != 0u) {
    let q = (*p).quad; let qa = q.xyz * d;
    let A = dot(qa, d); let B = dot(qa, o) + 0.5 * q.w * d.y; let C = dot(q.xyz * o, o) + q.w * o.y - 1.0;
    let disc = B * B - A * C;
    if (A > 1e-9) {
      if (disc <= 0.0) { return false; }
      let s = sqrt(disc); let t0 = (-B - s) / A; let t1 = (-B + s) / A;
      if (t0 > tn) { tn = t0; code = CODE_QUAD; } tf = min(tf, t1);
    } else if (A < -1e-9) {                                          // along a taper: inside is outside the two roots, on the nappe that is really there
      if (disc > 0.0) {
        let s = sqrt(disc); let t0 = (-B + s) / A; let t1 = (-B - s) / A;    // (A < 0: t0 < t1)
        if (1.0 - 0.5 * q.w * (o.y + d.y * t0) > 0.0) { tf = min(tf, t0); } else { if (t1 > tn) { tn = t1; code = CODE_QUAD; } }
      }
    } else if (abs(B) > 1e-9) { let t0 = -0.5 * C / B; if (B > 0.0) { tf = min(tf, t0); } else if (t0 > tn) { tn = t0; code = CODE_QUAD; } } else if (C > 0.0) { return false; }
  }
  let np = flags & 15u;
  for (var i = 0u; i < np; i++) {
    let pl = extra[(*p).planes + i]; let den = dot(pl.xyz, d); let num = pl.w - dot(pl.xyz, o);
    if (abs(den) < 1e-12) { if (num < 0.0) { return false; } continue; }
    let t = num / den; if (den < 0.0) { if (t > tn) { tn = t; code = 7u + i; } } else { tf = min(tf, t); }
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
/** a leaf: a pointed oval, seen from both sides */
fn meetLeaf(k: u32, ro: vec3f, rd: vec3f, tmin: f32, tmax: f32) -> f32 {
  let a = extra[k * 2u]; let b = bitcast<vec4u>(extra[k * 2u + 1u]);
  let n = octUnpack(b.x); let den = dot(n, rd); if (abs(den) < 1e-6) { return -1.0; }
  let t = dot(a.xyz - ro, n) / den; if (t <= tmin || t >= tmax) { return -1.0; }
  let x = ro + rd * t - a.xyz; let tl = octUnpack(b.y); let u = dot(x, tl) / a.w; let v = dot(x, cross(n, tl)) / a.w;
  let w = select(0.46, 0.06, b.w == 1u) * (1.0 - u * u) * (1.0 + 0.35 * u);
  return select(-1.0, t, abs(u) < 1.0 && abs(v) < w);
}

/** leaves one by one, or as a ball of them: a medium. Rays from what the camera sees take them one by one; the rest do not need to */
var<private> fineLeaves: bool = true;
const BALL_SIGMA = 0.80;                                            // per unit of the ball's radius, with the clumping a ball of leaves has (three quarters of its even density)
fn ballStay(o: vec3f, d: vec3f, tmin: f32, tmax: f32) -> vec2f { let a = dot(d, d); let b = dot(o, d); let c = dot(o, o) - 1.0; let q = b * b - a * c; if (q <= 0.0) { return vec2f(1.0, 0.0); } let s = sqrt(q); return vec2f(max((-b - s) / a, tmin), min((-b + s) / a, tmax)); }
/** NEAREST: what is out there. `cut` < 1e8 is the camera's section; `skip` = flags a ray does not meet. Glass is passed, and what it lets through is in thru */
fn nearest(ro: vec3f, rd: vec3f, tmin: f32, tmaxIn: f32, skip: u32, cut: f32) -> Hit {
  var best = tmaxIn; var id = NOTHING; var code = 0u; let inv = safeInv(rd); let oc = octant(rd);
  var g0 = vec2f(1e30, 0.0); var g1 = vec2f(1e30, 0.0); var gp = NOTHING; var gc = 0u;   // the two nearest panes: (t, tint)
  let t0 = select(tmin, max(tmin, (cut - ro.y) / rd.y), cut < 1e8 && ro.y > cut && rd.y < -1e-5);   // above the section nothing it cuts is there
  for (var s = 0u; s < 3u; s++) {
    let per = frame.per[s]; let b0 = frame.node0[s] + oc * per; let pb = frame.piece0[s]; let tm = select(t0, tmin, s == 1u);
    var i = 0u;
    while (i < per) {
      let n = &nodes[b0 + i];
      if (!boxHit(n, ro, inv, tm, best)) { i = (*n).skip; continue; }
      let lf = (*n).leaf; let cnt = lf & 15u; let first = pb + (lf >> 4u);
      for (var j = 0u; j < cnt; j++) {
        let k = first + j; let fl = pieces[k].flags;
        if ((fl & skip) != 0u) { continue; }
        if (meet(k, ro, rd, tmin, best, cut)) {
          if ((fl & GLASS) != 0u) { let g = vec2f(hT, bitcast<f32>(pieces[k].tint | 0x3f000000u)); if (hT < g0.x) { g1 = g0; g0 = g; gp = k; gc = hCode; } else if (hT < g1.x) { g1 = g; } continue; }
          best = hT; id = k; code = hCode;
        }
      }
      i++;
    }
  }
  { let per = frame.per.w; let b0 = frame.node0.w + oc * per; var i = 0u;
    while (i < per) {
      let n = &nodes[b0 + i];
      if (!boxHit(n, ro, inv, tmin, best)) { i = (*n).skip; continue; }
      let lf = (*n).leaf;
      if ((lf & 15u) != 0u) {
        let k = lf >> 4u; let I = &insts[k];
        let o = vec3f(dot((*I).r0.xyz, ro) + (*I).r0.w, dot((*I).r1.xyz, ro) + (*I).r1.w, dot((*I).r2.xyz, ro) + (*I).r2.w);
        let d = vec3f(dot((*I).r0.xyz, rd), dot((*I).r1.xyz, rd), dot((*I).r2.xyz, rd));      // (scaled: t is the world's)
        if (!fineLeaves) { if ((*I).per < 60u) { i++; continue; } let st = ballStay(o, d, tmin, best); let t = st.x - log(1.0 - rnd()) / (BALL_SIGMA * length(d)); if (st.y > st.x && t < st.y && ro.y + rd.y * t < cut) { best = t; id = LEAF_HIT | k; code = NOTHING; } i++; continue; }
        let inv2 = safeInv(d); let q0 = (*I).node + octant(d) * (*I).per; var q = 0u; let qn = (*I).per;
        while (q < qn) {
          let m = &nodes[q0 + q];
          if (!boxHit(m, o, inv2, tmin, best)) { q = (*m).skip; continue; }
          let l2 = (*m).leaf; let c2 = l2 & 15u;
          for (var j = 0u; j < c2; j++) { let w = (*I).leaf + (l2 >> 4u) + j; let t = meetLeaf(w, o, d, tmin, best); if (t > 0.0 && ro.y + rd.y * t < cut) { best = t; id = LEAF_HIT | k; code = w; } }
          q++;
        }
      }
      i++;
    } }
  var h: Hit; h.t = best; h.piece = id; h.code = code; h.thru = vec3f(1.0); h.gt = g0.x; h.gpiece = gp; h.gcode = gc; if (g0.x >= best) { h.gpiece = NOTHING; }
  if (g0.x < best) { h.thru *= unpackTint(bitcast<u32>(g0.y)); } if (g1.x < best) { h.thru *= unpackTint(bitcast<u32>(g1.y)); }
  return h;
}
/** VISIBLE: how much gets through from ro along rd to tmax. Glass tints it, leaves let a little by */
fn visible(ro: vec3f, rd: vec3f, tmin: f32, tmax: f32) -> vec3f {
  var T = vec3f(1.0); let inv = safeInv(rd); let oc = octant(rd);
  for (var s = 0u; s < 3u; s++) {
    let per = frame.per[s]; let b0 = frame.node0[s] + oc * per; let pb = frame.piece0[s];
    var i = 0u;
    while (i < per) {
      let n = &nodes[b0 + i];
      if (!boxHit(n, ro, inv, tmin, tmax)) { i = (*n).skip; continue; }
      let lf = (*n).leaf; let cnt = lf & 15u; let first = pb + (lf >> 4u);
      for (var j = 0u; j < cnt; j++) {
        let k = first + j; let fl = pieces[k].flags;
        if ((fl & NOSHADOW) != 0u) { continue; }
        if (meet(k, ro, rd, tmin, tmax, 1e9)) {
          if ((fl & (GLASS | WATER)) != 0u) { T *= select(vec3f(0.62, 0.86, 0.9), unpackTint(pieces[k].tint), (fl & GLASS) != 0u); continue; }
          return vec3f(0.0);
        }
      }
      i++;
    }
  }
  { let per = frame.per.w; let b0 = frame.node0.w + oc * per; var i = 0u;
    while (i < per) {
      let n = &nodes[b0 + i];
      if (!boxHit(n, ro, inv, tmin, tmax)) { i = (*n).skip; continue; }
      let lf = (*n).leaf;
      if ((lf & 15u) != 0u) {
        let k = lf >> 4u; let I = &insts[k];
        let o = vec3f(dot((*I).r0.xyz, ro) + (*I).r0.w, dot((*I).r1.xyz, ro) + (*I).r1.w, dot((*I).r2.xyz, ro) + (*I).r2.w);
        let d = vec3f(dot((*I).r0.xyz, rd), dot((*I).r1.xyz, rd), dot((*I).r2.xyz, rd));
        if (!fineLeaves) { if ((*I).per < 60u) { i++; continue; } let st = ballStay(o, d, tmin, tmax); if (st.y > st.x) { T *= exp(-vec3f(1.0, 0.86, 1.1) * BALL_SIGMA * (st.y - st.x) * length(d)); if (T.y < 0.02) { return vec3f(0.0); } } i++; continue; }
        let inv2 = safeInv(d); let q0 = (*I).node + octant(d) * (*I).per; var q = 0u; let qn = (*I).per;
        while (q < qn) {
          let m = &nodes[q0 + q];
          if (!boxHit(m, o, inv2, tmin, tmax)) { q = (*m).skip; continue; }
          let l2 = (*m).leaf; let c2 = l2 & 15u;
          for (var j = 0u; j < c2; j++) { if (meetLeaf((*I).leaf + (l2 >> 4u) + j, o, d, tmin, tmax) > 0.0) { T *= vec3f(0.1, 0.16, 0.03); if (T.y < 0.02) { return vec3f(0.0); } } }
          q++;
        }
      }
      i++;
    } }
  return T;
}

/** the camera's ray through a point of the picture (0‥1, y down) */
fn camRay(uv: vec2f) -> vec3f { let p = uv * 2.0 - 1.0; return normalize(frame.camFwd.xyz + frame.camRight.xyz * p.x - frame.camUp.xyz * p.y); }
/** where a world point is in the picture, now or last frame (z = its distance along the view; <= 0 behind) */
fn project(x: vec3f) -> vec3f { let v = x - frame.camPos.xyz; let z = dot(v, frame.camFwd.xyz); let r = frame.camRight.xyz; let u = frame.camUp.xyz; return vec3f(dot(v, r) / dot(r, r) / z * 0.5 + 0.5, 0.5 - dot(v, u) / dot(u, u) / z * 0.5, z); }
fn projectPrev(x: vec3f) -> vec3f { let v = x - frame.prevPos.xyz; let z = dot(v, frame.prevFwd.xyz); let r = frame.prevRight.xyz; let u = frame.prevUp.xyz; return vec3f(dot(v, r) / dot(r, r) / z * 0.5 + 0.5, 0.5 - dot(v, u) / dot(u, u) / z * 0.5, z); }
