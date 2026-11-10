// THE TWO RAY QUERIES, by one rule. A ray is f(t) = w + s t wide: so wide an area its answer will be averaged over. Solids it meets as they are, however wide it is; leaves as the world's block
// (blocks.ts): cells of 0.32 m, and where it is wider of 0.64.
const CELL_CODE = 0xfffffffeu; const FINEST = 0.32; const BRICK = 2.56;
/** does a ray stop at one cell, drawn (the path tracer that obeys the rule), or is what cells send it summed? */
const BLK_DICE = false;
/** leaves: 0 one by one (as far as leafTo), 2 not but by the rule. noLeaves: somebody else asks for them (the sun's and the torches' pictures) */
var<private> leafMode: u32 = 0u; var<private> noLeaves: bool = false;
var<private> footW = 0.0; var<private> footS = 0.0; var<private> leafTo = 1e9;
var<private> blk = vec2f(1e9);               // where levels 0 and 1 of the block begin
var<private> blkOn: bool = true; var<private> blkPast = 0.0;      /* (and how far off the block is not met at all: for leaves' own cells, which know what is near) */
 var<private> fromCell: u32 = 0xffffffffu;
/** what the block sent back along the last ray that `nearest` was asked for, and how much came through it (where it is summed, not drawn) */
var<private> blkL = vec3f(0.0); var<private> blkT = 1.0;
fn foot(w: f32, s: f32) {
  footW = w; footS = s; if (s <= 0.0) { blk = vec2f(1e9); leafTo = 1e9; return; }
  let g = FINEST * 0.70710678 * exp2(rnd() - 0.5); blk = max((vec2f(1.0, 2.0) * g - w) / s, vec2f(0.0)); leafTo = select(0.0, blk.x, BLK_DICE);     /* (one by one near by cost three to eight milliseconds and gained nothing) */
}
/** for a ray whose edges are looked at (a lamp's own shadow): leaves by the cells from its start */
fn footSharp(w: f32, s: f32) { foot(w, s); leafTo = 0.0; }
/** for a ray to a light so big (its width, metres; the sun's: over its distance) and so far: it begins at a point, for whoever asks throws the dice over what it averages, and is as wide as the light
 *  looks. (Not as wide as its asker: it draws together toward the light, and what stands close to a torch must be there.) */
fn footLight(size: f32, far: f32) { foot(0.02, size / max(far, 0.1)); }
fn footAt(t: f32) -> f32 { return footW + footS * t; }
/** boxes looked at by the last rays (nothing reads it but the panel) */
var<private> walked: u32 = 0u;

/** NEAREST: what is out there. `cut` < 1e8 is the camera's section; `skip` = flags a ray does not meet. Glass is passed, and what it lets through is in thru */
fn nearest(ro: vec3f, rd: vec3f, tmin: f32, tmaxIn: f32, skip: u32, cut: f32) -> Hit {
  var best = tmaxIn; var id = NOTHING; var code = 0u; let inv = safeInv(rd); let oc = octant(rd);
  var g0 = vec2f(1e30, 0.0); var g1 = vec2f(1e30, 0.0); var gp = NOTHING; var gc = 0u;
  let t0 = select(tmin, max(tmin, (cut - ro.y) / rd.y), cut < 1e8 && ro.y > cut && rd.y < -1e-5);
  for (var s = 0u; s < 3u; s++) {
    let per = frame.per[s]; let b0 = frame.node0[s] + oc * frame.step[s]; let pb = frame.piece0[s]; let tm = select(t0, tmin, s == 1u);
    var i = 0u;
    while (i < per) {
      walked += 1u; let n = &nodes[b0 + i]; let lf = (*n).leaf; let cnt = lf & 15u;
      if (!boxHit(n, ro, inv, tm, best)) { i = (*n).skip; continue; }
      let first = pb + (lf >> 4u);
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
  if (leafMode == 0u && leafTo > tmin) { let per = frame.per.w; let b0 = frame.node0.w + oc * frame.step.w; var i = 0u; let solid = best; best = min(best, leafTo); var found = false;
    while (i < per) {
      walked += 1u; let n = &nodes[b0 + i];
      if (!boxHit(n, ro, inv, tmin, best)) { i = (*n).skip; continue; }
      let lf = (*n).leaf;
      if ((lf & 15u) != 0u) {
        let k = lf >> 4u; let I = &insts[k]; let lc = select(cut, 1e9, ((*I).tint & 0x1000000u) != 0u && cut > OPEN_FLOOR);
        let o = vec3f(dot((*I).r0.xyz, ro) + (*I).r0.w, dot((*I).r1.xyz, ro) + (*I).r1.w, dot((*I).r2.xyz, ro) + (*I).r2.w);
        let d = vec3f(dot((*I).r0.xyz, rd), dot((*I).r1.xyz, rd), dot((*I).r2.xyz, rd));
        let inv2 = safeInv(d); let q0 = (*I).node + octant(d) * (*I).per; var q = 0u; let qn = (*I).per;
        while (q < qn) {
          walked += 1u; let m = &nodes[q0 + q];
          if (!boxHit(m, o, inv2, tmin, best)) { q = (*m).skip; continue; }
          let l2 = (*m).leaf; let c2 = l2 & 15u;
          for (var j = 0u; j < c2; j++) { let w = (*I).leaf + (l2 >> 4u) + j; let t = meetLeaf(w, o, d, tmin, best); if (t > 0.0 && ro.y + rd.y * t < lc) { best = t; id = LEAF_HIT | k; code = w; found = true; } }
          q++;
        }
      }
      i++;
    } if (!found) { best = solid; } }
  blkL = vec3f(0.0); blkT = 1.0;
  if (blkOn && footS > 0.0 && min(blk.x, leafTo) < best) { let b = blockWalk(ro, rd, max(max(tmin, blkPast), min(blk.x, leafTo)), best, select(1e9, -log(1.0 - rnd()), BLK_DICE), !BLK_DICE); if (b.cell != NOTHING) { best = b.t; id = LEAF_HIT | b.cell; code = CELL_CODE; } else if (!BLK_DICE) { blkL = b.L; blkT = exp(-b.tau); } }
  var h: Hit; h.t = best; h.piece = id; h.code = code; h.thru = vec3f(1.0); h.gt = g0.x; h.gpiece = gp; h.gcode = gc; if (g0.x >= best) { h.gpiece = NOTHING; }
  if (g0.x < best) { h.thru *= unpackTint(bitcast<u32>(g0.y)); } if (g1.x < best) { h.thru *= unpackTint(bitcast<u32>(g1.y)); }
  return h;
}
/** VISIBLE: how much gets through from ro along rd to tmax. Glass tints it, a leaf stops it */
fn visible(ro: vec3f, rd: vec3f, tmin: f32, tmax: f32) -> vec3f {
  var T = vec3f(1.0); let inv = safeInv(rd); let oc = octant(rd);
  for (var s = 0u; s < 3u; s++) {
    let per = frame.per[s]; let b0 = frame.node0[s] + oc * frame.step[s]; let pb = frame.piece0[s];
    var i = 0u;
    while (i < per) {
      let n = &nodes[b0 + i]; let lf = (*n).leaf; let cnt = lf & 15u;
      if (!boxHit(n, ro, inv, tmin, tmax)) { i = (*n).skip; continue; }
      let first = pb + (lf >> 4u);
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
  if (!noLeaves && leafMode == 0u && leafTo > tmin) { let per = frame.per.w; let b0 = frame.node0.w + oc * frame.step.w; var i = 0u; let lmax = min(tmax, leafTo);
    while (i < per) {
      let n = &nodes[b0 + i];
      if (!boxHit(n, ro, inv, tmin, lmax)) { i = (*n).skip; continue; }
      let lf = (*n).leaf;
      if ((lf & 15u) != 0u) {
        let k = lf >> 4u; let I = &insts[k];
        let o = vec3f(dot((*I).r0.xyz, ro) + (*I).r0.w, dot((*I).r1.xyz, ro) + (*I).r1.w, dot((*I).r2.xyz, ro) + (*I).r2.w);
        let d = vec3f(dot((*I).r0.xyz, rd), dot((*I).r1.xyz, rd), dot((*I).r2.xyz, rd));
        let inv2 = safeInv(d); let q0 = (*I).node + octant(d) * (*I).per; var q = 0u; let qn = (*I).per;
        while (q < qn) {
          let m = &nodes[q0 + q];
          if (!boxHit(m, o, inv2, tmin, lmax)) { q = (*m).skip; continue; }
          let l2 = (*m).leaf; let c2 = l2 & 15u;
          for (var j = 0u; j < c2; j++) { if (meetLeaf((*I).leaf + (l2 >> 4u) + j, o, d, tmin, lmax) > 0.0) { return vec3f(0.0); } }
          q++;
        }
      }
      i++;
    } }
  if (blkOn && footS > 0.0 && !noLeaves) { let b0 = min(blk.x, leafTo); if (b0 < tmax) { T *= exp(-blockWalk(ro, rd, max(max(tmin, blkPast), b0), tmax, 1e9, false).tau); } }
  if (frame.misc2.x > 0.5) { T *= smokeThrough(ro + rd * tmin, rd, tmax); }
  return T;
}

// ================================================================ the block
/** a cell's first row: thickness · tight box, is it grass · the box's far corner · two of its colors (and between them how far each two ways go together) */
struct Blk { t: f32, cell: u32, tau: f32, L: vec3f };
fn blkWord(at: u32, i: u32) -> u32 { return extra[at + (i >> 2u)][i & 3u]; }
/** how thick a cell's contents stand, a metre, along x y z (within its tight box), and that box */
struct CellBox { sigma: vec3f, lo: vec3f, hi: vec3f };
fn cellBox(r: vec4u, lo: vec3f, h: f32) -> CellBox {
  var b: CellBox; let e = h / 256.0; let a = vec3f(vec3u(r.y, r.y >> 8u, r.y >> 16u) & vec3u(255u)); let z = vec3f((vec3u(r.z, r.z >> 8u, r.z >> 16u) & vec3u(255u)) + vec3u(1u));
  b.lo = lo + a * e; b.hi = lo + z * e; b.sigma = vec3f(vec3u(r.x, r.x >> 8u, r.x >> 16u) & vec3u(255u)) * (6.4 / 255.0) / ((z - a) * e); return b;
}
/** how thick a cell's contents stand to a ray along them, a metre, q = sigma * rd: leaves in sheets askew of the axes say how far each two of those go together */
fn cellThick(r: vec4u, q: vec3f) -> f32 { let k = (vec3f(vec3u(r.y >> 25u, r.z >> 24u, (r.y >> 30u) | ((r.z >> 29u) << 2u)) & vec3u(31u)) - 15.0) / 15.0; let s = dot(q, q); return sqrt(max(s + 2.0 * dot(k, q.xxy * q.yzz), 0.05 * s)); }
/** along a ray from t0 to t1 until it has met `want` of thickness; `lit`: what it is sent is summed. */
fn blockWalk(ro: vec3f, rd: vec3f, t0: f32, t1: f32, want: f32, lit: bool) -> Blk {
  var o: Blk; o.cell = NOTHING; o.t = t1; o.tau = 0.0; if (BLK_MANY == 0u) { return o; }
  let inv = safeInv(rd); let top = BLK_LO + vec3f(BLK_N) * BRICK; let a = (BLK_LO - ro) * inv; let b = (top - ro) * inv; var t = max(max3(min(a, b)), t0) + 1e-4; let te = min(min3(max(a, b)), min(t1, t0 + 150.0)) - 1e-4; let ahead = select(vec3f(0.0), vec3f(1.0), rd > vec3f(0.0));
  for (var i = 0u; i < 192u; i++) {
    if (t >= te || o.tau > 4.6) { break; }
    let level = select(0u, 1u, t >= blk.y); let g = (ro + rd * t - BLK_LO) / BRICK; let c = clamp(vec3i(floor(g)), vec3i(0), BLK_N - 1);
    let p = blkWord(BLK_PAGE, u32(c.x + BLK_N.x * (c.y + BLK_N.y * c.z)));
    if (p == 0u) { let e = (BLK_LO + (vec3f(c) + ahead) * BRICK - ro) * inv; t = max(min3(e), t) + 1e-3; continue; }
    let per = 8u >> level; let h = FINEST * f32(1u << level); let q = clamp(vec3i(floor((g - vec3f(c)) * f32(per))), vec3i(0), vec3i(i32(per) - 1)); let lo = BLK_LO + vec3f(c) * BRICK + vec3f(q) * h;
    let e = (lo + ahead * h - ro) * inv; var out = max(min3(e), t); if (level == 0u) { out = min(out, max(blk.y, t)); }
    let cell = blkWord(BLK_BRICKS, (p - 1u) * 588u + level * 512u + u32(q.x) + per * (u32(q.y) + per * u32(q.z)));
    if (cell >= 0x80000000u) { let d = i32(cell & 255u); let e2 = (BLK_LO + vec3f(c) * BRICK + select(vec3f(max(q - d + 1, vec3i(0))), vec3f(min(q + d, vec3i(i32(per)))), rd > vec3f(0.0)) * h - ro) * inv; t = max(t, min(min3(e2), select(1e9, max(blk.y, t), level == 0u))) + 1e-3; continue; }      /* (nothing so many cells round) */
    if (cell != fromCell) {
      let r = extra[BLK_CELLS + (cell - 1u) * 2u]; let B = cellBox(r, lo, h); let u = (B.lo - ro) * inv; let v = (B.hi - ro) * inv; let ta = max(t - 1e-3, max3(min(u, v))); let tb = min(min(out, te), min3(max(u, v)));
      if (tb > ta && (r.x & 0xffffffu) != 0u) { let qs = B.sigma * rd; let sigma = cellThick(r, qs); let d = sigma * (tb - ta); if (lit && d > 0.0) { o.L += exp(-o.tau) * (1.0 - exp(-d)) * cellSent(cell, r, rd); } if (o.tau + d >= want) { o.t = ta + (want - o.tau) / sigma; o.cell = cell; o.tau = want; return o; } o.tau += d; }
    }
    t = out + 1e-3;
  }
  return o;
}
