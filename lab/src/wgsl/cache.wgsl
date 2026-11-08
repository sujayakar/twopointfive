// LIGHT ON SURFACES. A table keyed by where a surface is and which way it faces; an entry holds the light landing there (lamps and every bounce; the sun
// apart, as a share of it seen). Nothing outside this file knows how it is kept: readers call cacheRead.
//   keys[slot]   the key's fingerprint, 0 = empty      cand[slot]   who founded it, until claimed (the smallest wins, so a run repeats)
//   (a round reads cells and writes fresh, which is then copied over: what an entry sees of another never depends on which ran first)
//   cells[2s]    E.rgb, samples                        cells[2s+1]  its point, sun seen          norms[slot]  its normal, packed
const SLOTS = 262144u; const REACH = 32u; const CELL = 0.32; const COARSE = 1.35; const FAR_CELL = 6.0;
const EVERY_WAY = 0x80010003u;                                      // a normal that is none: foliage, which faces every way
fn facing(n: vec3f) -> u32 { if (all(n == vec3f(0.0))) { return 6u; } let a = abs(n); if (a.x >= a.y && a.x >= a.z) { return select(1u, 0u, n.x > 0.0); } if (a.y >= a.z) { return select(3u, 2u, n.y > 0.0); } return select(5u, 4u, n.z > 0.0); }
/** the building and its roof are kept finely; the city only coarsely, or it would fill the table by itself */
fn cacheNear(x: vec3f) -> bool { return abs(x.x) < 20.0 && abs(x.z) < 15.0 && x.y > -0.6 && x.y < 12.0; }
/** `again`: the second entry for a place whose first belongs to the other side of something */
fn cacheKeyOf(x: vec3f, n: vec3f, level: u32, again: u32) -> u32 {
  let near = cacheNear(x); let c = select(CELL, select(FAR_CELL, COARSE, near), level == 1u); let q = vec3i(floor((x + n * select(0.06, 0.2, level == 1u)) / c)); let h = pcg(u32(q.x) + pcg(u32(q.y) + pcg(u32(q.z) + pcg(facing(n) + 8u * level + select(16u, 0u, near) + 77u + 64u * again))));
  return max(h, 1u);
}
fn cacheKey(x: vec3f, n: vec3f, level: u32) -> u32 { return cacheKeyOf(x, n, level, 0u); }
fn cacheFind(key: u32) -> u32 { var s = key % SLOTS; for (var i = 0u; i < REACH; i++) { let k = atomicLoad(&keys[s]); if (k == key) { return s; } if (k == 0u) { return NOTHING; } s = (s + 1u) % SLOTS; } return NOTHING; }
fn cacheFound(key: u32, who: u32) {
  var s = key % SLOTS; var i = 0u;
  for (var tries = 0u; tries < 2u * REACH && i < REACH; tries++) {
    let k = atomicLoad(&keys[s]); if (k == key) { atomicMin(&cand[s], who); return; }
    if (k == 0u) { let r = atomicCompareExchangeWeak(&keys[s], 0u, key); if (r.exchanged || r.old_value == key) { atomicMin(&cand[s], who); return; } if (r.old_value == 0u) { continue; } }   // (a weak exchange may fail for nothing: the same slot again)
    s = (s + 1u) % SLOTS; i++;
  }
}
/** the entry for a place: of the two that may stand for its cell, the one whose own point is joined to it (slot; or none, and which is free to found: 0, 1, 2 = neither) */
fn cacheEntry(x: vec3f, n: vec3f, level: u32) -> vec2u {
  for (var v = 0u; v < 2u; v++) { let s = cacheFind(cacheKeyOf(x, n, level, v)); if (s == NOTHING) { return vec2u(NOTHING, v); } let p = cells[2u * s + 1u].xyz; if (all(p == vec3f(0.0))) { return vec2u(s, v); } if (level > 1u || !cacheNear(x) || joined(x + n * 0.08, p + n * 0.08)) { return vec2u(s, v); } } return vec2u(NOTHING, 2u);
}
/** which levels this surface still wants an entry at (bit 0 fine, bit 1 coarse; bits 2 and 3 the same, as the second entry). Asked of cells, which no one writes while rays are out, so that who founds it
 *  never depends on who came first. What moves founds only coarse entries: fine ones would be left behind wherever it has been */
fn cacheWants(x: vec3f, n: vec3f, moves: bool) -> u32 {
  var w = 0u; for (var l = select(0u, 1u, moves || !cacheNear(x)); l < 2u; l++) { let e = cacheEntry(x, n, l); if (e.x == NOTHING) { if (e.y < 2u) { w |= select(1u, 4u, e.y == 1u) << l; } continue; } if (all(cells[2u * e.x + 1u].xyz == vec3f(0.0))) { w |= select(1u, 4u, e.y == 1u) << l; } } return w;
}
/** the same with no dice: the four fine cells round x in its plane, by nearness. For a mirror's image, where blocks or their grain would be seen */
fn cacheSmooth(x: vec3f, n: vec3f) -> Cached {
  let f = facing(n); if (f > 5u || !cacheNear(x)) { return cacheRead(x, n, vec2f(0.5)); } let k = f >> 1u; let u = (k + 1u) % 3u; let v = (k + 2u) % 3u; let g = x / CELL - 0.5; let w = fract(g); var c: Cached; var ws = 0.0;
  for (var i = 0u; i < 4u; i++) {
    var q = x; q[u] = (floor(g[u]) + select(0.5, 1.5, (i & 1u) == 1u)) * CELL; q[v] = (floor(g[v]) + select(0.5, 1.5, i > 1u)) * CELL; if (!joined(x + n * 0.08, q + n * 0.08)) { continue; } let s = cacheEntry(q, n, 0u).x; if (s == NOTHING) { continue; }
    let a = cells[2u * s]; if (a.w < 3.0 || bad4(a)) { continue; } let wt = select(1.0 - w[u], w[u], (i & 1u) == 1u) * select(1.0 - w[v], w[v], i > 1u) + 1e-4; c.E += a.xyz * wt; c.sun += finite(cells[2u * s + 1u].w, 1.0) * wt; c.n = max(c.n, a.w); ws += wt;
  }
  if (ws < 0.02) { return cacheRead(x, n, vec2f(0.5)); } c.E /= ws; c.sun /= ws; return c;
}
struct Cached { E: vec3f, sun: f32, n: f32 };
/** what lands at x, facing n. `j` spreads the lookups over neighbouring cells, so that blocks blend; far or young falls back on the coarse cells */
fn cacheRead(x: vec3f, n: vec3f, j: vec2f) -> Cached {
  let every = all(n == vec3f(0.0)); let b = basis(select(n, vec3f(0.0, 1.0, 0.0), every)); let o = x + (b[0] * (j.x - 0.5) + b[1] * (j.y - 0.5) + b[2] * select(0.0, fract(j.x * 7.31 + j.y * 3.17) - 0.5, every)) * CELL; var c: Cached;
  for (var level = 0u; level < 2u; level++) {
    var s = NOTHING; if (level == 0u && joined(x + n * 0.08, o + n * 0.08)) { s = cacheEntry(o, n, 0u).x; } else { s = cacheEntry(x, n, level).x; } if (s == NOTHING) { continue; }
    let a = cells[2u * s]; if (bad4(a)) { continue; } if (a.w < select(1.0, 3.0, level == 0u)) { if (level == 0u) { continue; } }
    c.E = a.xyz; c.n = a.w; c.sun = finite(cells[2u * s + 1u].w, 1.0); if (a.w > 0.0) { return c; }
  }
  c.n = 0.0; return c;
}
