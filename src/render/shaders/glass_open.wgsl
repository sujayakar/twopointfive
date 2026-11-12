// ---- GLASS BREAKING (rust/game_rules/src/breakage/glassbreak.rs + rust/sim/src/glass.rs; the scene's shaders call it inside `if (GLASS_BREAK)`, the prelude const; the smoke's solver (smoke/smoke.wgsl solidAt) where it bakes a pane): per
// a listed pane's row (boxmat.wgsl pane…: rust/game_draw boxes/glass.rs is its one writer). The functions below are the rules' own (rust/sim/src/glass.rs, pattern.rs) statement for statement — a blow's SITE cuts the glass round it into pieces (sector s between radial cracks s and
// s+1; band 0 the crater, 1‥2 the ring pieces between the chords, 3‥6 the daggers: near / far of the sector's left and right half), territories are the power diagram of the
// sites — and a piece whose bit is set has fallen out: every ray then passes that part of the pane as if no glass were there (view, mirror, light), which is the hole; the CPU's
// raycast / segment tests ask rust/sim/src/glass.rs open, the same rule. A DICING tempered pane (flag 4) opens by clumps instead: the cells of a jittered lattice (gbClumpKey).
const GP_ANG_JIT: f32 = 0.6;
const GP_RHO_LO: f32 = 0.78;
const GP_RHO_SPAN: f32 = 0.44;
const GP_SPLIT_LO: f32 = 0.35;
const GP_SPLIT_SPAN: f32 = 0.3;
const GP_CUT_NONE: f32 = 0.45;
const GP_CUT_LO: f32 = 0.9;
const GP_CUT_SPAN: f32 = 1.0;
const GP_CUT_TILT: f32 = 0.9;
const GP_C2_DRAWN: f32 = 0.45;
const GP_WEB_RING: f32 = 0.45;
const GP_DICE_JIT: f32 = 0.6;
const GP_DICE_DELAY: f32 = 1.4;
const GP_TAU: f32 = 6.283185307179586;
struct GbSite { c: vec2f, rot: f32, rz: f32, n: u32, seed: u32, e: f32, merge: f32, rho: vec4f, m: vec4f };
fn gbHeader(k: u32) -> vec4f { return vec4f(paneSites(k), paneSays(k), paneAxis(k), paneDamage(k)); }
fn gbSiteLoad(k: u32, i: u32) -> GbSite {
  let o = i * 4u; let a = paneSite(k, o); let b = paneSite(k, o + 1u);
  var S: GbSite; S.c = a.xy; S.rot = a.z; S.rz = a.w; S.n = clamp(u32(b.x + 0.5), 3u, 12u); S.seed = u32(b.y); S.e = b.z; S.merge = b.w; S.rho = paneSite(k, o + 2u); S.m = paneSite(k, o + 3u);
  return S;
}
/** rust/sim/src/glass.rs hash: (seed, a, b) → 24 bits */
fn gpHash(seed: u32, a: u32, b: u32) -> u32 {
  var h = seed ^ ((a + 1u) * 0x9E3779B1u) ^ ((b + 1u) * 0x85EBCA77u);
  h = (h ^ (h >> 15u)) * 0x2C1B3C6Du;
  h = (h ^ (h >> 12u)) * 0x297A2D39u;
  return (h ^ (h >> 15u)) >> 8u;
}
fn gpJit(seed: u32, a: u32, b: u32) -> f32 { return f32(gpHash(seed, a, b)) / 16777216.0; }
fn gpWrap(k: i32, n: u32) -> u32 { return u32(k + i32(n)) % n; }   // (k in −1 ‥ n)
/** where radial k sits, in sectors */
fn gpBound(S: GbSite, k: i32) -> f32 { return f32(k) + GP_ANG_JIT * (gpJit(S.seed, gpWrap(k, S.n), 7u) - 0.5); }
fn gpAngle(S: GbSite, k: i32) -> f32 { return S.rot + GP_TAU * gpBound(S, k) / f32(S.n); }
fn gpRho(S: GbSite, k: i32, b: u32) -> f32 { return S.rho[b] * (GP_RHO_LO + GP_RHO_SPAN * gpJit(S.seed, gpWrap(k, S.n), b)); }
fn gpChordPt(S: GbSite, k: i32, b: u32) -> vec2f { let a = gpAngle(S, k); return S.c + gpRho(S, k, b) * vec2f(cos(a), sin(a)); }
/** past chord b of sector s (the side away from the site)? */
fn gpBeyond(S: GbSite, s: i32, b: u32, uv: vec2f) -> bool { let A = gpChordPt(S, s, b); let B = gpChordPt(S, s + 1, b); return (B.x - A.x) * (uv.y - A.y) - (B.y - A.y) * (uv.x - A.x) < 0.0; }
fn gpMerged(S: GbSite, s: u32) -> bool { return gpJit(S.seed, s, 21u) < S.merge; }
/** where sector s's branch crack sits, in sectors */
fn gpSplit(S: GbSite, s: i32) -> f32 { let b0 = gpBound(S, s); let b1 = gpBound(S, s + 1); return b0 + (b1 - b0) * (GP_SPLIT_LO + GP_SPLIT_SPAN * gpJit(S.seed, u32(s), 23u)); }
/** the snap across half h of sector s: (point, unit normal away from the site); .z of the second = 0 when this dagger runs unbroken */
struct GpCut { p: vec2f, n: vec2f, on: bool };
fn gpCut(S: GbSite, s: i32, h: u32) -> GpCut {
  var c: GpCut; c.on = !(gpJit(S.seed, u32(s), 27u + h) < GP_CUT_NONE); c.p = S.c; c.n = vec2f(1.0, 0.0);
  if (!c.on) { return c; }
  let sp = gpSplit(S, s); let mid = select((sp + gpBound(S, s + 1)) * 0.5, (gpBound(S, s) + sp) * 0.5, h == 0u);
  let am = S.rot + GP_TAU * mid / f32(S.n); let rc = S.rho.w * (GP_CUT_LO + GP_CUT_SPAN * gpJit(S.seed, u32(s), 25u + h));
  let an = am + (gpJit(S.seed, u32(s), 29u + h) - 0.5) * GP_CUT_TILT;
  c.p = S.c + rc * vec2f(cos(am), sin(am)); c.n = vec2f(cos(an), sin(an));
  return c;
}
/** the sector (.x) of pane point uv round site S and its position across the fan in sectors (.y) */
fn gpSector(S: GbSite, uv: vec2f) -> vec2f {
  let d = uv - S.c; let n = i32(S.n);
  if (dot(d, d) < 1e-12) { return vec2f(0.0, max(0.0, gpBound(S, 0))); }
  var t = (atan2(d.y, d.x) - S.rot) / GP_TAU; t = (t - floor(t)) * f32(n);
  let s0 = min(n - 1, i32(floor(t)));
  if (t < gpBound(S, s0)) { if (s0 == 0) { return vec2f(f32(n - 1), t + f32(n)); } return vec2f(f32(s0 - 1), t); }
  if (t >= gpBound(S, s0 + 1)) { if (s0 == n - 1) { return vec2f(0.0, t - f32(n)); } return vec2f(f32(s0 + 1), t); }
  return vec2f(f32(s0), t);
}
/** the sector (.x) and band (.y, 0‥6) of pane point uv round site S */
fn gpSectorBand(S: GbSite, uv: vec2f) -> vec2u {
  let st = gpSector(S, uv); let s = i32(st.x + 0.5); let su = u32(s);
  if (!gpBeyond(S, s, 0u, uv)) { return vec2u(su, 0u); }
  let merged = gpMerged(S, su);
  if (!merged && !gpBeyond(S, s, 1u, uv)) { return vec2u(su, 1u); }
  if (!gpBeyond(S, s, 2u, uv)) { return vec2u(su, select(2u, 1u, merged)); }
  let h = select(0u, 1u, st.y >= gpSplit(S, s)); let c = gpCut(S, s, h);
  return vec2u(su, select(3u, 5u, c.on && dot(c.n, uv - c.p) > 0.0) + h);
}
/** whose territory uv is in: the least power distance |p − c|² − rz² (n ≥ 1) */
fn gbNearest(k: u32, n: u32, uv: vec2f) -> u32 {
  var bi = 0u; var bd = 1e30;
  for (var i = 0u; i < n; i++) { let a = paneSite(k, i * 4u); let q = uv - a.xy; let d = dot(q, q) - a.w * a.w; if (d < bd) { bd = d; bi = i; } }
  return bi;
}
/** is piece (sector s, band b) of site S out? */
fn gbPieceOpen(S: GbSite, s: u32, b: u32) -> bool {
  let w = u32(S.m[b >> 1u]);   // (packed integers are exact in f32; + 0.5 would round a 24-bit odd word up — markings.wgsl's rule)
  return ((w >> (s + 12u * (b & 1u))) & 1u) == 1u;
}
/** pane-local point (glassTrace's frame: Rᵀ·(X − c)) → the record's (u, v) */
fn gbUV(k: u32, lp: vec3f) -> vec2f { return select(vec2f(lp.x, lp.y), vec2f(lp.z, lp.y), gbHeader(k).z > 0.5); }
/** the centre of dice clump (c, r) of pane k (rust/sim/src/pattern.rs Dice::centre): its lattice cell's centre, jittered */
fn gbClumpCentre(hh: vec2f, g: vec2f, seed: u32, c: u32, r: u32) -> vec2f {
  return -hh + (vec2f(f32(c), f32(r)) + 0.5 + GP_DICE_JIT * (vec2f(gpJit(seed, c, r), gpJit(seed ^ 0x5bd1e9u, c, r)) - 0.5)) * 2.0 * hh / g;
}
/** the order key of the dice clump holding uv (gpDiceClump + gpDiceKey): the nearest jittered centre among the 3 × 3 cells round uv's own, then metres from the blow to it + its hashed delay */
fn gbClumpKey(k: u32, uv: vec2f) -> f32 {
  let hh = paneHalf(k); let g = max(vec2f(paneCols(k), paneRows(k)), vec2f(1.0)); let seed = u32(paneSeed(k)); let blow = paneBlow(k);
  let c0 = vec2i(clamp(floor((uv + hh) / (2.0 * hh) * g), vec2f(0.0), g - 1.0)); let gi = vec2i(g + 0.5);
  var best = vec2u(c0); var bp = vec2f(0.0); var bd = 1e30;
  for (var r = max(0, c0.y - 1); r <= min(gi.y - 1, c0.y + 1); r++) { for (var c = max(0, c0.x - 1); c <= min(gi.x - 1, c0.x + 1); c++) {
    let p = gbClumpCentre(hh, g, seed, u32(c), u32(r)); let q = p - uv; let d = dot(q, q);
    if (d < bd) { bd = d; best = vec2u(u32(c), u32(r)); bp = p; }
  } }
  let cell = min(2.0 * hh.x / g.x, 2.0 * hh.y / g.y);
  return distance(bp, blow) + GP_DICE_DELAY * cell * gpJit(seed ^ 0x3c6ef3u, best.x, best.y);
}
/** has the glass of pane k at pane point uv fallen out? (flags = the header's) */
fn gbOpenUV(k: u32, fl: u32, n: u32, uv: vec2f) -> bool {
  if ((fl & 1u) == 0u) { return false; }
  if ((fl & 8u) != 0u) { return true; }
  if ((fl & 4u) != 0u) { return gbClumpKey(k, uv) < paneOpenAt(k); }   // a tempered pane letting go: the clumps the wave has reached
  if (n == 0u) { return false; }
  let S = gbSiteLoad(k, gbNearest(k, n, uv)); let sb = gpSectorBand(S, uv);
  if (sb.y >= 3u && (fl & 2u) == 0u) { return false; }   // the uncut sheet beyond the rings of a pane that has not run
  return gbPieceOpen(S, sb.x, sb.y);
}
/** has the part of pane k at local point lp fallen out? (a whole pane — nothing open — answers in one fetch) */
fn glassCellOpen(k: u32, lp: vec3f) -> bool {
  let h = gbHeader(k); let fl = u32(h.y + 0.5);
  if ((fl & 1u) == 0u) { return false; }
  return gbOpenUV(k, fl, min(u32(h.x + 0.5), 8u), gbUV(k, lp));
}
/** world point → pane-local point (Rᵀ·(X − c)), of the pane's first two rows in SceneInfo.glass */
fn paneLocal(X: vec3f, p0: vec4f, p1: vec4f) -> vec3f { let p = X - p0.xyz; return vec3f(p0.w * p.x - p1.w * p.z, p.y, p1.w * p.x + p0.w * p.z); }
