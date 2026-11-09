// PATCHES IN A HASH, with everything about them a constant: SLOTS, CELL, COARSE (0: none), LEVELS (> 1: the level is picked by how wide the asking ray is, WIDEN of it), FLAT (a cell is thin along its
// normal, however long it is), JOINS (asked whether two places are joined), TRIES (how many entries may stand for a cell), SOFT (where none of them is joined to it, the first answers after all), ROAM (an entry's point is wherever rays last landed in it, so that it is the mean of
// its patch), SPREAD (the entries' own rays found entries too), DIRECT (the lamps' and the sun's light is kept; else it is asked at each reading).
//   keys[slot] the key, 0 = empty · cand[slot] who founds it or moves it · cells[2s] E.rgb, samples · cells[2s+1] its point, sun seen · norms[slot] · spot[slot] where it looks from, its normal
const REACH = 32u; const FAR_CELL = 6.0;
fn sizeOf(level: u32) -> f32 { if (LEVELS > 1u) { return CELL * exp2(f32(level)); } return select(CELL, COARSE, level == 1u); }
fn offOf(level: u32) -> f32 { return select(0.06, 0.2, sizeOf(level) > 1.0); }
fn cacheKeyOf(x: vec3f, n: vec3f, level: u32, again: u32) -> u32 {
  let near = cacheNear(x); let c = select(FAR_CELL, sizeOf(level), near); var q = vec3i(floor((x + n * select(offOf(level), 0.06, FLAT)) / c)); let f = facing(n);
  if (FLAT && near && f < 6u) { let k = f >> 1u; q[k] = i32(floor((x[k] + n[k] * 0.06) / min(c, 0.2))); }
  let h = pcg(u32(q.x) + pcg(u32(q.y) + pcg(u32(q.z) + pcg(f + 8u * select(0u, level, near) + select(256u, 0u, near) + 77u + 1024u * again)))); return max(h, 1u);
}
fn cacheFind(key: u32) -> u32 { var s = key % SLOTS; for (var i = 0u; i < REACH; i++) { let k = atomicLoad(&keys[s]); if (k == key) { return s; } if (k == 0u) { return NOTHING; } s = (s + 1u) % SLOTS; } return NOTHING; }
/** who wins a cell: the smallest of its callers' numbers, shuffled by the cell, so that a run repeats and no corner of the screen is favored */
fn shuffled(who: u32, key: u32) -> u32 { return who ^ (pcg(key) & 0xfffffu); }
fn cacheFound(key: u32, who0: u32) {
  let who = shuffled(who0, key);
  var s = key % SLOTS; var i = 0u;
  for (var tries = 0u; tries < 2u * REACH && i < REACH; tries++) {
    let k = atomicLoad(&keys[s]); if (k == key) { atomicMin(&cand[s], who); return; }
    if (k == 0u) { let r = atomicCompareExchangeWeak(&keys[s], 0u, key); if (r.exchanged || r.old_value == key) { atomicMin(&cand[s], who); return; } if (r.old_value == 0u) { continue; } }
    s = (s + 1u) % SLOTS; i++;
  }
}
fn sees(a: vec3f, p: vec3f, n: vec3f) -> bool { return !JOINS || joined(a + n * 0.08, p + n * 0.08); }
/** the entry for a place: of those that may stand for its cell, the one whose own point is joined to it (slot; or none, and which is free to found: TRIES = none) */
fn cacheEntry(x: vec3f, n: vec3f, level: u32) -> vec2u {
  for (var v = 0u; v < TRIES; v++) { let s = cacheFind(cacheKeyOf(x, n, level, v)); if (s == NOTHING) { return vec2u(NOTHING, v); } let p = cells[2u * s + 1u].xyz; if (all(p == vec3f(0.0))) { return vec2u(s, v); } if (!cacheNear(x) || sees(x, p, n)) { return vec2u(s, v); } } return vec2u(NOTHING, 2u);
}
/** the level a reader wants: as coarse as its ray is wide, by dice between two */
var<private> askLevel: u32 = 0u;
fn levelNow(j: f32) -> u32 { if (LEVELS <= 1u) { return 0u; } return u32(clamp(floor(log2(max(askWide * WIDEN / CELL, 1.0)) + j), 0.0, f32(LEVELS - 1u))); }
fn levelMost() -> u32 { if (LEVELS > 1u) { return LEVELS; } return select(1u, 2u, COARSE > 0.0); }
/** one entry's light, if it knows enough */
fn entryLight(s: u32, least: f32) -> Cached { var c: Cached; let a = cells[2u * s]; if (bad4(a) || a.w < least) { return c; } c.E = a.xyz; c.n = a.w; c.sun = finite(cells[2u * s + 1u].w, 1.0); return c; }
fn held(x: vec3f, n: vec3f, j: vec2f) -> Cached {
  let every = all(n == vec3f(0.0)); let b = basis(select(n, vec3f(0.0, 1.0, 0.0), every)); let l0 = levelNow(fract(j.x * 13.7 + j.y * 5.3)); askLevel = l0; let o = x + (b[0] * (j.x - 0.5) + b[1] * (j.y - 0.5) + b[2] * select(0.0, fract(j.x * 7.31 + j.y * 3.17) - 0.5, every)) * sizeOf(l0); var c: Cached;
  for (var go = 0u; go < select(levelMost() + 1u, 5u, LEVELS > 1u); go++) {      // beside it; where it is; then coarser (or, if none is, finer)
    var level = l0; if (LEVELS > 1u) { if (go == 2u) { level = l0 + 1u; } else if (go == 3u) { level = l0 + 2u; } else if (go == 4u) { if (l0 == 0u) { break; } level = l0 - 1u; } if (level >= LEVELS) { continue; } } else if (go == 2u) { level = 1u; }
    if (go == 0u && !sees(x, o, n)) { continue; } let s = cacheEntry(select(x, o, go == 0u), n, level).x; if (s == NOTHING) { continue; }
    c = entryLight(s, select(1.0, 3.0, go < 2u)); if (c.n > 0.0) { return c; }
  }
  if (SOFT) { for (var v = 0u; v < TRIES; v++) { let s = cacheFind(cacheKeyOf(x, n, l0, v)); if (s == NOTHING) { break; } c = entryLight(s, 1.0); if (c.n > 0.0) { return c; } } }
  return c;
}
/** the same with no dice: the four fine cells round x in its plane, by nearness. For a mirror's image, where blocks or their grain would be seen */
fn heldSmooth(x: vec3f, n: vec3f) -> Cached {
  let f = facing(n); if (f > 5u || !cacheNear(x)) { return held(x, n, vec2f(0.5)); } let k = f >> 1u; let u = (k + 1u) % 3u; let v = (k + 2u) % 3u; var c: Cached;
  for (var level = 0u; level < select(1u, LEVELS, LEVELS > 1u); level++) {
    let cs = sizeOf(level); let g = x / cs - 0.5; let w = fract(g); var ws = 0.0;
    for (var i = 0u; i < 4u; i++) {
      var q = x; q[u] = (floor(g[u]) + select(0.5, 1.5, (i & 1u) == 1u)) * cs; q[v] = (floor(g[v]) + select(0.5, 1.5, i > 1u)) * cs; if (!sees(x, q, n)) { continue; } let s = cacheEntry(q, n, level).x; if (s == NOTHING) { continue; }
      let e = entryLight(s, 3.0); if (e.n <= 0.0) { continue; } let wt = select(1.0 - w[u], w[u], (i & 1u) == 1u) * select(1.0 - w[v], w[v], i > 1u) + 1e-4; c.E += e.E * wt; c.sun += e.sun * wt; c.n = max(c.n, e.n); ws += wt;
    }
    if (ws >= 0.02) { c.E /= ws; c.sun /= ws; return c; }
  }
  return held(x, n, vec2f(0.5));
}
