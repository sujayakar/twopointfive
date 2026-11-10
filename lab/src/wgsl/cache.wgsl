// LIGHT ON SURFACES: patches in a table, keyed by where a surface is and which way it faces. A patch holds the mean light landing on it (lamps and every bounce; the sun apart, as a share of it seen), because
// the point it looks from roams: to wherever a ray last landed in it. It is CELL long and wide but THIN along its normal, and where walls part it up to TRIES entries stand for one cell, each for the places
// joined to where it began. Nothing outside this file and the cache stages knows how it is kept: readers call cacheRead, rays that land call cacheTell (cachetell.wgsl). What was measured: notes/kept.md.
//   keys[slot]   the key (31 bits), 0 = empty          cand[slot]   who asks to found it or move it, until claimed
//   cells[2s]    E.rgb, samples                        cells[2s+1]  where it began, sun seen         spot[slot]  where it looks from now; its normal and when it was last called for (0: founded by nobody yet)
//   (a round reads cells and writes fresh, which is then copied over: what an entry sees of another never depends on which ran first. No rule here goes by slot, which is where a race put it: so a run repeats)
const SLOTS = 262144u; const REACH = 128u; const CELL = 0.64; const THIN = 0.2; const FAR_CELL = 6.0; const TRIES = 4u; const LEAST = 3.0;
// how wide its own ray begins · how much more a later sample weighs (2: as its number; more: the first ones, lit by a table that was still dark, are forgotten sooner) · from how many samples on it looks every fourth
// and every eighth frame · how many it keeps when the light changes near it · after how long it sleeps when no ray has landed in it (in eights of a frame; it then looks every 64th)
const START = 0.32; const RISE = 2.0; const FOURTH = 24.0; const EIGHTH = 60.0; const FORGET = 7.0; const SLEEP = 75u;
fn facing(n: vec3f) -> u32 { let a = abs(n); if (a.x >= a.y && a.x >= a.z) { return select(1u, 0u, n.x > 0.0); } if (a.y >= a.z) { return select(3u, 2u, n.y > 0.0); } return select(5u, 4u, n.z > 0.0); }
/** the level and its roofs are kept finely; the city only coarsely, or it would fill the table by itself */
fn cacheNear(x: vec3f) -> bool { return abs(x.x) < 20.0 && abs(x.z) < 15.0 && x.y > -0.6 && x.y < 12.0; }
/** `again`: the entries after the first that may stand for a place whose earlier ones belong to the other side of something */
fn cacheKey(x: vec3f, n: vec3f, again: u32) -> u32 {
  let near = cacheNear(x); let y = x + n * 0.06; var q = vec3i(floor(y / select(FAR_CELL, CELL, near))); let f = facing(n); if (near) { q[f >> 1u] = i32(floor(y[f >> 1u] / THIN)); }
  return max(pcg(u32(q.x) + pcg(u32(q.y) + pcg(u32(q.z) + pcg(f + select(256u, 0u, near) + 1024u * again + 77u)))) & 0x7fffffffu, 1u);
}
fn cacheFind(key: u32) -> u32 { var s = key % SLOTS; for (var i = 0u; i < REACH; i++) { let k = atomicLoad(&keys[s]); if (k == key) { return s; } if (k == 0u) { return NOTHING; } s = (s + 1u) % SLOTS; } return NOTHING; }
/** does the entry that began at p speak for x? (Near it, for two cells may share a key; and joined to it) */
fn speaksFor(p: vec3f, x: vec3f, n: vec3f) -> bool { if (!cacheNear(x)) { return true; } return distance(p, x) < 2.0 * CELL && joined(x + n * 0.08, p + n * 0.08); }
/** the entry for a place: of those that may stand for its cell, the first that speaks for it (slot; or none, and which is free to found: TRIES = none) */
fn cacheEntry(x: vec3f, n: vec3f) -> vec2u {
  for (var v = 0u; v < TRIES; v++) { let s = cacheFind(cacheKey(x, n, v)); if (s == NOTHING) { return vec2u(NOTHING, v); } let p = cells[2u * s + 1u].xyz; if (all(p == vec3f(0.0)) || speaksFor(p, x, n)) { return vec2u(s, v); } } return vec2u(NOTHING, TRIES);
}
fn spotPack(n: vec3f) -> u32 { let q = vec2u(octEnc(n) * 1022.0 + 0.5); return 1u | (q.x << 1u) | (q.y << 11u) | (((frame.counts.z >> 3u) & 2047u) << 21u); }
fn spotNormal(w: u32) -> vec3f { return octDec(vec2f(f32((w >> 1u) & 1023u), f32((w >> 11u) & 1023u)) / 1022.0); }
fn asleep(w: u32) -> bool { return (((frame.counts.z >> 3u) - (w >> 21u)) & 2047u) > SLEEP; }
struct Cached { E: vec3f, sun: f32, n: f32 };
fn entryLight(s: u32, least: f32) -> Cached { var c: Cached; let a = cells[2u * s]; if (bad4(a) || a.w < least) { return c; } c.E = a.xyz; c.n = a.w; c.sun = finite(cells[2u * s + 1u].w, 1.0); return c; }
/** what lands at x, facing n. `j` spreads the lookups over neighbouring patches, so that their edges blend */
fn cacheRead(x: vec3f, n: vec3f, j: vec2f) -> Cached {
  let b = basis(n); let o = x + (b[0] * (j.x - 0.5) + b[1] * (j.y - 0.5)) * CELL; var c: Cached;
  for (var go = 0u; go < 2u; go++) { if (go == 0u && !joined(x + n * 0.08, o + n * 0.08)) { continue; } let s = cacheEntry(select(x, o, go == 0u), n).x; if (s == NOTHING) { continue; } c = entryLight(s, select(1.0, LEAST, go == 0u)); if (c.n > 0.0) { return c; } }     // beside it, then where it is
  for (var v = 0u; v < TRIES; v++) { let s = cacheFind(cacheKey(x, n, v)); if (s == NOTHING) { break; } if (distance(cells[2u * s + 1u].xyz, x) > 2.0 * CELL && cacheNear(x)) { continue; } c = entryLight(s, 1.0); if (c.n > 0.0) { return c; } }     // (where none speaks for it, the first that is near: wrong by some, and not black)
  return c;
}
/** the same with no dice: the four patches round x in its plane, by nearness. For a mirror's image, where their edges or their grain would be seen */
fn cacheSmooth(x: vec3f, n: vec3f) -> Cached {
  if (!cacheNear(x)) { return cacheRead(x, n, vec2f(0.5)); } let k = facing(n) >> 1u; let u = (k + 1u) % 3u; let v = (k + 2u) % 3u; let g = x / CELL - 0.5; let w = fract(g); var c: Cached; var ws = 0.0;
  for (var i = 0u; i < 4u; i++) {
    var q = x; q[u] = (floor(g[u]) + select(0.5, 1.5, (i & 1u) == 1u)) * CELL; q[v] = (floor(g[v]) + select(0.5, 1.5, i > 1u)) * CELL; if (!joined(x + n * 0.08, q + n * 0.08)) { continue; } let s = cacheEntry(q, n).x; if (s == NOTHING) { continue; }
    let e = entryLight(s, LEAST); if (e.n <= 0.0) { continue; } let wt = select(1.0 - w[u], w[u], (i & 1u) == 1u) * select(1.0 - w[v], w[v], i > 1u) + 1e-4; c.E += e.E * wt; c.sun += e.sun * wt; c.n = max(c.n, e.n); ws += wt;
  }
  if (ws < 0.02) { return cacheRead(x, n, vec2f(0.5)); } c.E /= ws; c.sun /= ws; return c;
}
