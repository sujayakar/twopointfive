// PROBES IN A GRID: a fine grid over the level, a coarse one over the city. Each holds what arrives at a point, by direction: OCT = irradiance over an octahedron (ON × ON), else the mean light and its first moment
// (L1). What stands between a place and a probe: VIS 2 = by each probe's mean distances (an octahedron, DN × DN), 1 = by what is joined to what, 0 = not asked. The lamps' and the sun's light is asked at each reading.
//   at[p] where it is, alive · irr[] irradiance, halves · far[] mean distance and its square, halves · looks[p] how many times it has looked
const CELL = 0.32;
struct Grid { lo: vec3f, s: f32, n: vec3u, first: u32 };
fn gridFor(x: vec3f) -> Grid { var g: Grid; g.lo = LO; g.s = STEP; g.n = NUM; g.first = 0u; if (any(x < LO) || any(x > LO + vec3f(NUM - 1u) * STEP)) { g.lo = LO2; g.s = STEP2; g.n = NUM2; g.first = NUM.x * NUM.y * NUM.z; } return g; }
fn irrAt(p: u32, n: vec3f) -> vec3f {
  if (!OCT) { let o = p * WORDS; let a = unpack2x16float(irr[o]); let b = unpack2x16float(irr[o + 1u]); let c = unpack2x16float(irr[o + 2u]); let d = unpack2x16float(irr[o + 3u]); let e = unpack2x16float(irr[o + 4u]); let f = unpack2x16float(irr[o + 5u]);
    return max(vec3f(a, b.x) * PI + (vec3f(b.y, c) * n.x + vec3f(d, e.x) * n.y + vec3f(e.y, f) * n.z) * (2.0 * PI / 3.0), vec3f(0.0)); }
  let g = octEnc(n) * f32(ON) - 0.5; let i = vec2i(floor(g)); let f = g - floor(g); var r = vec3f(0.0);
  for (var k = 0u; k < 4u; k++) { let o = vec2i(i32(k & 1u), i32(k >> 1u)); let q = octWrap(i + o, ON); let w = mix(1.0 - f.x, f.x, f32(o.x)) * mix(1.0 - f.y, f.y, f32(o.y)); let at = p * WORDS + (u32(q.y) * ON + u32(q.x)) * 2u; r += w * vec3f(unpack2x16float(irr[at]), unpack2x16float(irr[at + 1u]).x); } return r;
}
/** a texel past an octahedron's edge is the one over that edge */
fn octWrap(p: vec2i, n: u32) -> vec2i { var q = p; let m = i32(n); if (q.x < 0 || q.x >= m) { q = vec2i(clamp(q.x, 0, m - 1), m - 1 - clamp(q.y, 0, m - 1)); } if (q.y < 0 || q.y >= m) { q = vec2i(m - 1 - q.x, clamp(q.y, 0, m - 1)); } return q; }
fn farAt(p: u32, d: vec3f) -> vec2f { let g = octEnc(d) * f32(DN) - 0.5; let i = vec2i(floor(g)); let f = g - floor(g); var r = vec2f(0.0); for (var k = 0u; k < 4u; k++) { let o = vec2i(i32(k & 1u), i32(k >> 1u)); let q = octWrap(i + o, DN); r += mix(1.0 - f.x, f.x, f32(o.x)) * mix(1.0 - f.y, f.y, f32(o.y)) * unpack2x16float(dist[p * DN * DN + u32(q.y) * DN + u32(q.x)]); } return r; }
fn joinedFar(a: vec3f, b: vec3f) -> bool { let m = ceil(distance(a, b) / 1.3); for (var k = 0.0; k < m; k += 1.0) { if (!joined(mix(a, b, k / m), mix(a, b, (k + 1.0) / m))) { return false; } } return true; }
fn keptAt(x: vec3f, n: vec3f) -> vec4f {
  let g = gridFor(x); let y = x + n * min(0.25 * g.s, 0.2); let q = (y - g.lo) / g.s; let b = clamp(vec3i(floor(q)), vec3i(0), vec3i(g.n) - 2); let f = clamp(q - vec3f(b), vec3f(0.0), vec3f(1.0)); var E = vec3f(0.0); var W = 0.0;
  for (var k = 0u; k < 8u; k++) {
    let o = vec3u(k & 1u, (k >> 1u) & 1u, k >> 2u); let c = vec3u(b) + o; let p = g.first + (c.z * g.n.y + c.y) * g.n.x + c.x; let a = at[p]; if (a.w < 0.5 || irr[p * WORDS + WORDS - 1u] == 0u) { continue; }
    let t = mix(1.0 - f, f, vec3f(o)); var w = max(t.x * t.y * t.z, 1e-4); let v = a.xyz - x; let l = length(v); let cs = dot(v, n) / max(l, 1e-3) * 0.5 + 0.5; w *= cs * cs + 0.05;
    if (g.first == 0u && VIS == 1u && !joinedFar(x + n * 0.08, a.xyz)) { w *= 0.004; }
    if (g.first == 0u && VIS == 2u) { let u = y - a.xyz; let r = length(u); let m = farAt(p, u / max(r, 1e-3)); if (r > m.x) { let s2 = max(m.y - m.x * m.x, 0.0); let e = r - m.x; let z = s2 / (s2 + e * e + 1e-6); w *= max(z * z * z, 0.0); } }
    if (VIS != 1u && w < 0.02) { w *= w * w * 2500.0; } E += irrAt(p, n) * w; W += w;
  }
  return vec4f(E, W);
}
fn held(x: vec3f, n0: vec3f, j: vec2f) -> Cached { var c: Cached; let every = all(n0 == vec3f(0.0)); var r = keptAt(x, select(n0, vec3f(0.0, 1.0, 0.0), every)); if (every) { r += keptAt(x, vec3f(0.0, -1.0, 0.0)); } if (r.w < 1e-7) { return c; } c.E = r.xyz / r.w; c.n = 10.0; return c; }
fn heldSmooth(x: vec3f, n: vec3f) -> Cached { return held(x, n, vec2f(0.5)); }
