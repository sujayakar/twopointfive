// SURFELS: discs on surfaces, laid where rays land and nothing covers yet. Each holds the light at its middle; a place is lit by the discs over it, by how near their middles are. A hash of buckets finds them.
//   buckets: for each its key (0 = empty), who asks, how many, HOLD discs; then how many discs in all · surfels[3s] point, radius · [3s+1] E.rgb, samples · [3s+2] sun seen, normal (bits)
const REACH = 24u; const WIDTH = HOLD + 3u; const TOTAL = BUCKETS * WIDTH; const FAR_R = 4.0;
fn cellSize(near: bool) -> f32 { return select(FAR_R * 2.0, RADIUS * 2.0, near); }
fn bucketKey(q: vec3i, near: bool) -> u32 { return max(pcg(u32(q.x) + pcg(u32(q.y) + pcg(u32(q.z) + pcg(select(256u, 0u, near) + 77u)))), 1u); }
fn bucketFind(key: u32) -> u32 { var s = key % BUCKETS; for (var i = 0u; i < REACH; i++) { let k = atomicLoad(&buckets[s * WIDTH]); if (k == key) { return s; } if (k == 0u) { return NOTHING; } s = (s + 1u) % BUCKETS; } return NOTHING; }
fn bucketMake(key: u32) -> u32 {
  var s = key % BUCKETS; var i = 0u;
  for (var tries = 0u; tries < 2u * REACH && i < REACH; tries++) { let k = atomicLoad(&buckets[s * WIDTH]); if (k == key) { return s; } if (k == 0u) { let r = atomicCompareExchangeWeak(&buckets[s * WIDTH], 0u, key); if (r.exchanged || r.old_value == key) { return s; } if (r.old_value == 0u) { continue; } } s = (s + 1u) % BUCKETS; i++; } return NOTHING;
}
/** how much of disc s lies over x: 0 none */
fn over(s: u32, x: vec3f, n: vec3f) -> f32 { let a = surfels[3u * s]; let d = x - a.xyz; let q = dot(d, d) / (a.w * a.w); if (q >= 1.0) { return 0.0; } let m = unOct(bitcast<u32>(surfels[3u * s + 2u].y)); let c = dot(n, m); if (c < 0.4) { return 0.0; } let h = dot(d, m) / a.w; let w = (1.0 - q) * (1.0 - q) * c * c * max(1.0 - 9.0 * h * h, 0.0); if (w <= 0.0) { return 0.0; } if (JOINS && !joined(x + n * 0.08, a.xyz + m * 0.08)) { return 0.0; } return w; }
fn held(x: vec3f, n: vec3f, j: vec2f) -> Cached {
  var c: Cached; if (all(n == vec3f(0.0))) { return c; } let near = cacheNear(x); let b = bucketFind(bucketKey(vec3i(floor(x / cellSize(near))), near)); if (b == NOTHING) { return c; } let m = min(atomicLoad(&buckets[b * WIDTH + 2u]), HOLD); var W = 0.0;
  for (var i = 0u; i < m; i++) { let s = atomicLoad(&buckets[b * WIDTH + 3u + i]); if (s == 0u) { continue; } let w = over(s - 1u, x, n); if (w <= 0.0) { continue; } let e = surfels[3u * s - 2u]; if (e.w < 1.0 || bad4(e)) { continue; } let k = w * min(e.w, 16.0); c.E += e.xyz * k; c.sun += surfels[3u * s - 1u].x * k; W += k; c.n = max(c.n, e.w); }
  if (W < 1e-5) {                                                     // nothing lies over it: whatever is near
    c = Cached(); for (var i = 0u; i < m; i++) { let s = atomicLoad(&buckets[b * WIDTH + 3u + i]); if (s == 0u) { continue; } let a = surfels[3u * s - 3u]; let e = surfels[3u * s - 2u]; if (e.w < 1.0 || bad4(e)) { continue; } let mm = unOct(bitcast<u32>(surfels[3u * s - 1u].y)); let d = x - a.xyz; let k = max(dot(n, mm), 0.05) / (0.01 * a.w * a.w + dot(d, d)); c.E += e.xyz * k; c.sun += surfels[3u * s - 1u].x * k; W += k; c.n = max(c.n, e.w); }
  }
  if (W < 1e-9) { c.n = 0.0; return c; } c.E /= W; c.sun /= W; return c;
}
fn heldSmooth(x: vec3f, n: vec3f) -> Cached { return held(x, n, vec2f(0.5)); }
fn covered(x: vec3f, n: vec3f) -> f32 { let near = cacheNear(x); let b = bucketFind(bucketKey(vec3i(floor(x / cellSize(near))), near)); if (b == NOTHING) { return 0.0; } let m = min(atomicLoad(&buckets[b * WIDTH + 2u]), HOLD); if (m >= HOLD) { return 1.0; } var W = 0.0; for (var i = 0u; i < m; i++) { let s = atomicLoad(&buckets[b * WIDTH + 3u + i]); if (s == 0u) { continue; } W += over(s - 1u, x, n); } return W; }
