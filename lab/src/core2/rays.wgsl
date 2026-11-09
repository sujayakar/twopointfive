// THE SECOND CORE'S RAYS: one rule. A ray is f(t) = w + s t wide; what is smaller than that is met as part of the world's block, the rest as it is. Things leave in three steps (classes), and
// the block's level follows, so each stretch of a ray has one level: exact · cells of 0.32 m, with class 0 and the leaves gone · 0.64, class 1 gone · 1.28 · 2.56.
const CELL_CODE = 0xfffffffeu; const FINEST = 0.32; const BRICK = 2.56;
var<private> footW = 0.0; var<private> footS = 0.0; var<private> leafTo = 1e9; var<private> smallTo = 1e9;       /* (the last two: for stages written for the first core) */
var<private> lim = vec4f(1e9);               // how far classes 0, 1, 2, 3 are met as they are
var<private> blk = vec4f(1e9);               // where levels 0, 1, 2, 3 of the block begin
var<private> leafOnly: bool = false; var<private> blkOn: bool = true; var<private> fromCell: u32 = 0xffffffffu;
/** what the block sent back along the last ray that `nearest` was asked for, and how much came through it (where it is summed, not drawn) */
var<private> blkL = vec3f(0.0); var<private> blkT = 1.0;
fn foot(w: f32, s: f32) {
  footW = w; footS = s; leafOnly = false; if (s <= 0.0) { lim = vec4f(1e9); blk = vec4f(1e9); leafTo = 1e9; return; }
  let g = FINEST * 0.70710678 * exp2(rnd() - 0.5); let t = max((vec4f(1.0, 2.0, 4.0, 8.0) * g - w) / s, vec4f(0.0)); blk = t; lim = vec4f(t.xy, 1e9, 1e9); leafTo = t.x * LEAF_NEAR; smallTo = 1e9;
}
fn footSolid(w: f32, s: f32) { foot(w, s); }
/** for a ray whose edges are looked at (a lamp's own shadow): solids as they are all the way, and only leaves by the cells */
fn footSharp(w: f32, s: f32) { foot(w, s); lim = vec4f(1e9); leafTo = 0.0; leafOnly = true; }
fn footAt(t: f32) -> f32 { return footW + footS * t; }
var<private> walked: u32 = 0u;
fn classOf(fl: u32) -> u32 { return fl >> 30u; }

fn nearest(ro: vec3f, rd: vec3f, tmin: f32, tmaxIn: f32, skip: u32, cut: f32) -> Hit {
  var best = tmaxIn; var id = NOTHING; var code = 0u; let inv = safeInv(rd); let oc = octant(rd);
  var g0 = vec2f(1e30, 0.0); var g1 = vec2f(1e30, 0.0); var gp = NOTHING; var gc = 0u;
  let t0 = select(tmin, max(tmin, (cut - ro.y) / rd.y), cut < 1e8 && ro.y > cut && rd.y < -1e-5);
  for (var s = 0u; s < 4u; s++) {
    let sm = s == 3u; let per = select(frame.per[s], frame.fine.x, sm); let b0 = select(frame.node0[s] + oc * frame.step[s], frame.fine.y + oc * frame.fine.z, sm); let pb = select(frame.piece0[s], frame.fine.w, sm); let tm = select(t0, tmin, s == 1u || sm);
    var i = 0u;
    while (i < per) {
      walked += 1u; let n = &nodes[b0 + i]; let lf = (*n).leaf; let cnt = lf & 15u;
      let far = select(best, min(best, lim[(lf >> 4u) & 3u]), cnt == 0u);      /* (a node knows the biggest class under it) */
      if (!boxHit(n, ro, inv, tm, far)) { i = (*n).skip; continue; }
      let first = pb + (lf >> 4u);
      for (var j = 0u; j < cnt; j++) {
        let k = first + j; let fl = pieces[k].flags; let to = min(best, lim[classOf(fl)]);
        if ((fl & skip) != 0u || to <= tmin) { continue; }
        if (meet(k, ro, rd, tmin, to, cut)) {
          if ((fl & GLASS) != 0u) { let g = vec2f(hT, bitcast<f32>(pieces[k].tint | 0x3f000000u)); if (hT < g0.x) { g1 = g0; g0 = g; gp = k; gc = hCode; } else if (hT < g1.x) { g1 = g; } continue; }
          best = hT; id = k; code = hCode;
        }
      }
      i++;
    }
  }
  if ((leafMode != 2u || footS > 0.0) && leafTo > tmin) { let per = frame.per.w; let b0 = frame.node0.w + oc * frame.step.w; var i = 0u; let solid = best; best = min(best, leafTo); var found = false;
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
  lastLimb = 0u; if (blkOn && footS > 0.0) { let c = carryHit(ro, rd, tmin, best); if (c.y > 0.0) { best = c.x; lastLimb = u32(c.y); id = LEAF_HIT | (lastLimb - 1u); code = CARRY_CODE; } }
  blkL = vec3f(0.0); blkT = 1.0;
  if (blkOn && footS > 0.0 && min(blk.x, leafTo) < best) { let b = blockWalk(ro, rd, max(tmin, min(blk.x, leafTo)), best, select(1e9, -log(1.0 - rnd()), BLK_DICE), !BLK_DICE); if (b.cell != NOTHING) { lastLimb = 0u; best = b.t; id = LEAF_HIT | b.cell; code = CELL_CODE; } else if (!BLK_DICE) { blkL = b.L; blkT = exp(-b.tau); } }
  var h: Hit; h.t = best; h.piece = id; h.code = code; h.thru = vec3f(1.0); h.gt = g0.x; h.gpiece = gp; h.gcode = gc; if (g0.x >= best) { h.gpiece = NOTHING; }
  if (g0.x < best) { h.thru *= unpackTint(bitcast<u32>(g0.y)); } if (g1.x < best) { h.thru *= unpackTint(bitcast<u32>(g1.y)); }
  return h;
}
fn visible(ro: vec3f, rd: vec3f, tmin: f32, tmax: f32) -> vec3f {
  var T = vec3f(1.0); let inv = safeInv(rd); let oc = octant(rd);
  for (var s = 0u; s < 4u; s++) {
    let sm = s == 3u; let per = select(frame.per[s], frame.fine.x, sm); let b0 = select(frame.node0[s] + oc * frame.step[s], frame.fine.y + oc * frame.fine.z, sm); let pb = select(frame.piece0[s], frame.fine.w, sm);
    var i = 0u;
    while (i < per) {
      let n = &nodes[b0 + i]; let lf = (*n).leaf; let cnt = lf & 15u; let far = select(tmax, min(tmax, lim[(lf >> 4u) & 3u]), cnt == 0u);
      if (!boxHit(n, ro, inv, tmin, far)) { i = (*n).skip; continue; }
      let first = pb + (lf >> 4u);
      for (var j = 0u; j < cnt; j++) {
        let k = first + j; let fl = pieces[k].flags; let to = min(tmax, lim[classOf(fl)]);
        if ((fl & NOSHADOW) != 0u || to <= tmin) { continue; }
        if (meet(k, ro, rd, tmin, to, 1e9)) {
          if ((fl & (GLASS | WATER)) != 0u) { T *= select(vec3f(0.62, 0.86, 0.9), unpackTint(pieces[k].tint), (fl & GLASS) != 0u); continue; }
          return vec3f(0.0);
        }
      }
      i++;
    }
  }
  if (!noLeaves && (leafMode != 2u || footS > 0.0) && leafTo > tmin) { let per = frame.per.w; let b0 = frame.node0.w + oc * frame.step.w; var i = 0u; let lmax = min(tmax, leafTo);
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
  if (blkOn && footS > 0.0 && !leafOnly) { T *= exp(-carryTau(ro, rd, tmin, tmax)); }
  if (blkOn && footS > 0.0) { let b0 = select(min(blk.x, leafTo), blk.x, noLeaves); if (b0 < tmax && !(leafOnly && noLeaves)) { T *= exp(-blockWalk(ro, rd, max(tmin, b0), tmax, 1e9, false).tau); } }
  if (frame.misc2.x > 0.5) { T *= smokeThrough(ro + rd * tmin, rd, tmax); }
  return T;
}

// ================================================================ the block
struct Blk { t: f32, cell: u32, tau: f32, L: vec3f };
fn blkWord(at: u32, i: u32) -> u32 { return bitcast<vec4u>(extra[at + (i >> 2u)])[i & 3u]; }
/** how thick a cell's contents stand, a metre, along x y z (within its tight box), and that box */
struct CellBox { sigma: vec3f, lo: vec3f, hi: vec3f };
fn cellBox(r: vec4u, lo: vec3f, h: f32) -> CellBox {
  var b: CellBox; let e = h / 16.0; let a = vec3f(vec3u(r.y, r.y >> 4u, r.y >> 8u) & vec3u(15u)); let z = vec3f((vec3u(r.y >> 12u, r.y >> 16u, r.y >> 20u) & vec3u(15u)) + vec3u(1u));
  b.lo = lo + a * e; b.hi = lo + z * e; b.sigma = vec3f(vec3u(r.x, r.x >> 8u, r.x >> 16u) & vec3u(255u)) * (6.4 / 255.0) / ((z - a) * e); return b;
}
/** along a ray from t0 to t1 until it has met `want` of thickness; `lit`: what it is sent is summed. (Before blk.x only leaves are met: the rest is there as it is) */
fn blockWalk(ro: vec3f, rd: vec3f, t0: f32, t1: f32, want: f32, lit: bool) -> Blk {
  var o: Blk; o.cell = NOTHING; o.t = t1; o.tau = 0.0; if (BLK_MANY == 0u) { return o; }
  let inv = safeInv(rd); let top = BLK_LO + vec3f(BLK_N) * BRICK; let a = (BLK_LO - ro) * inv; let b = (top - ro) * inv; var t = max(max3(min(a, b)), t0) + 1e-4; let te = min(min3(max(a, b)), t1) - 1e-4; let ahead = select(vec3f(0.0), vec3f(1.0), rd > vec3f(0.0));
  for (var i = 0u; i < 192u; i++) {
    if (t >= te || o.tau > 4.6) { break; }
    let level = min(TOP, select(select(select(0u, 1u, t >= blk.y), 2u, t >= blk.z), 3u, t >= blk.w)); let g = (ro + rd * t - BLK_LO) / BRICK; let c = clamp(vec3i(floor(g)), vec3i(0), BLK_N - 1);
    let w = blkWord(BLK_PAGE, u32(c.x + BLK_N.x * (c.y + BLK_N.y * c.z))); let p = w & 0x7fffffffu;       /* (the top bit: there are leaves in it) */
    if (p == 0u || ((leafOnly || t < blk.x) && (w >> 31u) == 0u)) { let e = (BLK_LO + (vec3f(c) + ahead) * BRICK - ro) * inv; t = max(min3(e), t) + 1e-3; continue; }
    let per = 8u >> level; let h = FINEST * f32(1u << level); let q = clamp(vec3i(floor((g - vec3f(c)) * f32(per))), vec3i(0), vec3i(i32(per) - 1)); let lo = BLK_LO + vec3f(c) * BRICK + vec3f(q) * h;
    let e = (lo + ahead * h - ro) * inv; var out = max(min3(e), t); if (level < 3u && blk[level + 1u] > t) { out = min(out, blk[level + 1u]); } if (blk.x > t) { out = min(out, blk.x); }
    if (level < 2u) { let u = vec3u(clamp(vec3i(floor((g - vec3f(c)) * 2.0)), vec3i(0), vec3i(1))); if (blkWord(BLK_BRICKS, (p - 1u) * 588u + 576u + u.x + 2u * (u.y + 2u * u.z)) == 0u) { let e2 = (BLK_LO + (vec3f(c) + 0.5 * (vec3f(u) + ahead)) * BRICK - ro) * inv; t = max(min3(e2), t) + 1e-3; continue; } }      /* (nothing in the 1.28 m round here) */
    let cell = blkWord(BLK_BRICKS, (p - 1u) * 588u + array<u32, 4>(0u, 512u, 576u, 584u)[level] + u32(q.x) + per * (u32(q.y) + per * u32(q.z)));
    if (cell != 0u && cell != fromCell) {
      let r = bitcast<vec4u>(extra[BLK_CELLS + (cell - 1u) * 2u]); let B = cellBox(r, lo, h); let u = (B.lo - ro) * inv; let v = (B.hi - ro) * inv; let ta = max(t - 1e-3, max3(min(u, v))); let tb = min(min(out, te), min3(max(u, v)));
      if (tb > ta && (r.x & 0xffffffu) != 0u) { let part = f32(r.x >> 24u) / 255.0; let sg = length(B.sigma * rd); let sigma = sg * select(select(1.0, 1.0 - part, noLeaves), part, t < blk.x || leafOnly); let d = sigma * (tb - ta); if (lit && d > 0.0) { o.L += exp(-o.tau) * (1.0 - exp(-d)) * cellSent(cell, r, B.sigma * rd / sg, rd); } if (o.tau + d >= want) { o.t = ta + (want - o.tau) / sigma; o.cell = cell; o.tau = want; return o; } o.tau += d; }
    }
    t = out + 1e-3;
  }
  return o;
}
