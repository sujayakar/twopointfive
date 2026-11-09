// PROBES ONLY NEAR SURFACES. A hash of cells (no facing): an entry is a small probe a hand off the surface that first asked for it, and holds the mean light and its first moment. Two may stand for a cell, when
// something stands between. The lamps' and the sun's light is asked at each reading.
//   cells[4s] mean light, looks · cells[4s+1..3] its first moment, x y z (w: its point)
const REACH = 32u; const FAR_CELL = 6.0; const OFF = 0.10;
fn cacheKeyOf(x: vec3f, again: u32) -> u32 { let near = cacheNear(x); let c = select(FAR_CELL, CELL, near); let q = vec3i(floor(x / c)); return max(pcg(u32(q.x) + pcg(u32(q.y) + pcg(u32(q.z) + pcg(select(256u, 0u, near) + 77u + 1024u * again)))), 1u); }
fn cacheFind(key: u32) -> u32 { var s = key % SLOTS; for (var i = 0u; i < REACH; i++) { let k = atomicLoad(&keys[s]); if (k == key) { return s; } if (k == 0u) { return NOTHING; } s = (s + 1u) % SLOTS; } return NOTHING; }
fn shuffled(who: u32, key: u32) -> u32 { return who ^ (pcg(key) & 0xfffffu); }
fn cacheFound(key: u32, who0: u32) {
  let who = shuffled(who0, key); var s = key % SLOTS; var i = 0u;
  for (var tries = 0u; tries < 2u * REACH && i < REACH; tries++) {
    let k = atomicLoad(&keys[s]); if (k == key) { atomicMin(&cand[s], who); return; }
    if (k == 0u) { let r = atomicCompareExchangeWeak(&keys[s], 0u, key); if (r.exchanged || r.old_value == key) { atomicMin(&cand[s], who); return; } if (r.old_value == 0u) { continue; } }
    s = (s + 1u) % SLOTS; i++;
  }
}
fn pointOf(s: u32) -> vec3f { return vec3f(cells[4u * s + 1u].w, cells[4u * s + 2u].w, cells[4u * s + 3u].w); }
fn cacheEntry(y: vec3f) -> vec2u { for (var v = 0u; v < 2u; v++) { let s = cacheFind(cacheKeyOf(y, v)); if (s == NOTHING) { return vec2u(NOTHING, v); } let p = pointOf(s); if (all(p == vec3f(0.0)) || !cacheNear(y) || joined(y, p)) { return vec2u(s, v); } } return vec2u(NOTHING, 2u); }
fn lightOfEntry(s: u32, n: vec3f, least: f32) -> vec4f { let a = cells[4u * s]; if (bad4(a) || a.w < least) { return vec4f(0.0); } let L = vec3f(0.0); return vec4f(max(a.xyz * PI + (cells[4u * s + 1u].xyz * n.x + cells[4u * s + 2u].xyz * n.y + cells[4u * s + 3u].xyz * n.z) * (2.0 * PI / 3.0), a.xyz * 0.15) + L, a.w); }
fn held(x: vec3f, n: vec3f, j: vec2f) -> Cached {
  let every = all(n == vec3f(0.0)); let b = basis(select(n, vec3f(0.0, 1.0, 0.0), every)); let y = x + n * OFF; let o = y + (b[0] * (j.x - 0.5) + b[1] * (j.y - 0.5)) * CELL + b[2] * ((fract(j.x * 7.31 + j.y * 3.17) - 0.5) * CELL * 0.5); var c: Cached;
  for (var go = 0u; go < 2u; go++) { if (go == 0u && !joined(y, o)) { continue; } let s = cacheEntry(select(y, o, go == 0u)).x; if (s == NOTHING) { continue; } let e = lightOfEntry(s, n, select(1.0, 3.0, go == 0u)); if (e.w <= 0.0) { continue; } c.E = e.xyz; c.n = e.w; return c; }
  return c;
}
fn heldSmooth(x: vec3f, n: vec3f) -> Cached {
  let y = x + n * OFF; let g = y / CELL - 0.5; let w = fract(g); var c: Cached; var W = 0.0;
  for (var i = 0u; i < 8u; i++) { let o = vec3f(f32(i & 1u), f32((i >> 1u) & 1u), f32(i >> 2u)); let q = (floor(g) + o + 0.5) * CELL; if (!joined(y, q)) { continue; } let s = cacheEntry(q).x; if (s == NOTHING) { continue; } let e = lightOfEntry(s, n, 3.0); if (e.w <= 0.0) { continue; } let t = mix(1.0 - w, w, o); let k = t.x * t.y * t.z + 1e-4; c.E += e.xyz * k; c.n = max(c.n, e.w); W += k; }
  if (W < 0.02) { return held(x, n, vec2f(0.5)); } c.E /= W; return c;
}
