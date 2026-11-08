// LIGHT ON SURFACES. A table keyed by where a surface is and which way it faces; an entry holds the light landing there (lamps and every bounce; the sun
// apart, as a share of it seen). Nothing outside this file knows how it is kept: readers call cacheRead.
//   keys[slot]   the key's fingerprint, 0 = empty      cand[slot]   who founded it, until claimed (the smallest wins, so a run repeats)
//   cells[2s]    E.rgb, samples                        cells[2s+1]  its point, sun seen          norms[slot]  its normal, packed
const SLOTS = 262144u; const CELL = 0.32; const COARSE = 1.35;
fn facing(n: vec3f) -> u32 { let a = abs(n); if (a.x >= a.y && a.x >= a.z) { return select(1u, 0u, n.x > 0.0); } if (a.y >= a.z) { return select(3u, 2u, n.y > 0.0); } return select(5u, 4u, n.z > 0.0); }
fn cacheKey(x: vec3f, n: vec3f, level: u32) -> u32 {
  let c = select(CELL, COARSE, level == 1u); let q = vec3i(floor((x + n * select(0.06, 0.2, level == 1u)) / c)); let h = pcg(u32(q.x) + pcg(u32(q.y) + pcg(u32(q.z) + pcg(facing(n) + 8u * level + 77u))));
  return max(h, 1u);
}
fn cacheFind(key: u32) -> u32 { var s = key % SLOTS; for (var i = 0u; i < 8u; i++) { let k = atomicLoad(&keys[s]); if (k == key) { return s; } if (k == 0u) { return NOTHING; } s = (s + 1u) % SLOTS; } return NOTHING; }
fn cacheFound(key: u32, who: u32) {
  var s = key % SLOTS;
  for (var i = 0u; i < 8u; i++) { let k = atomicLoad(&keys[s]); if (k == key) { atomicMin(&cand[s], who); return; } if (k == 0u) { let r = atomicCompareExchangeWeak(&keys[s], 0u, key); if (r.exchanged || r.old_value == key) { atomicMin(&cand[s], who); return; } } s = (s + 1u) % SLOTS; }
}
struct Cached { E: vec3f, sun: f32, n: f32 };
/** what lands at x, facing n. `j` spreads the lookups over neighbouring cells, so that blocks blend; far or young falls back on the coarse cells */
fn cacheRead(x: vec3f, n: vec3f, j: vec2f) -> Cached {
  let b = basis(n); let o = x + (b[0] * (j.x - 0.5) + b[1] * (j.y - 0.5)) * CELL; var c: Cached;
  for (var level = 0u; level < 2u; level++) {
    let s = cacheFind(cacheKey(select(x, o, level == 0u), n, level)); if (s == NOTHING) { continue; }
    let a = cells[2u * s]; if (a.w < select(1.0, 3.0, level == 0u)) { if (level == 0u) { continue; } }
    c.E = a.xyz; c.n = a.w; c.sun = cells[2u * s + 1u].w; if (a.w > 0.0) { return c; }
  }
  c.n = 0.0; return c;
}
