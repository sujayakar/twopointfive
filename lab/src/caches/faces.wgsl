// TEXELS ON FACES. The level is known pieces, so each piece has a grid of texels on the six faces of its box, in its own frame: no hash, nothing to found, and what moves takes its light along.
//   faces[slot] first texel, counts (10 bits an axis) · faces[PIECES + id].x the slot a piece lies in now · texels[2i] E.rgb, samples · texels[2i+1] sun seen, whose (id), where in it (12 + 12 bits) and face, alive
fn counts(p: vec4u) -> vec3u { return vec3u(p.y & 1023u, (p.y >> 10u) & 1023u, p.y >> 20u); }
fn faceStart(c: vec3u, f: u32) -> u32 { let a = f >> 1u; var o = 0u; if (a > 0u) { o += 2u * c.y * c.z; } if (a > 1u) { o += 2u * c.z * c.x; } return o + (f & 1u) * c[(a + 1u) % 3u] * c[(a + 2u) % 3u]; }
fn texelPoint(slot: u32, face: u32, uv: vec2f) -> vec3f { let p = &pieces[slot]; let a = face >> 1u; var l: vec3f; l[a] = (*p).half[a] * select(-1.0, 1.0, (face & 1u) == 1u); l[(a + 1u) % 3u] = (uv.x * 2.0 - 1.0) * (*p).half[(a + 1u) % 3u]; l[(a + 2u) % 3u] = (uv.y * 2.0 - 1.0) * (*p).half[(a + 2u) % 3u]; return (*p).r0.xyz * (l.x - (*p).r0.w) + (*p).r1.xyz * (l.y - (*p).r1.w) + (*p).r2.xyz * (l.z - (*p).r2.w); }
fn texelNormal(slot: u32, face: u32) -> vec3f { let p = &pieces[slot]; let a = face >> 1u; var r = (*p).r0.xyz; if (a == 1u) { r = (*p).r1.xyz; } else if (a == 2u) { r = (*p).r2.xyz; } return r * select(-1.0, 1.0, (face & 1u) == 1u); }
/** where texel i looks from: x, and its slot */
fn texelUv(i: u32) -> vec2f { let w = bitcast<u32>(texels[2u * i + 1u].z); return vec2f(f32(w & 4095u), f32((w >> 12u) & 4095u)) / 4095.0; }
struct Grid { first: u32, face: u32, n: vec2u, g: vec2f };
fn gridOf(slot: u32, x: vec3f, n: vec3f) -> Grid {
  let p = &pieces[slot]; let l = vec3f(dot((*p).r0.xyz, x) + (*p).r0.w, dot((*p).r1.xyz, x) + (*p).r1.w, dot((*p).r2.xyz, x) + (*p).r2.w); let m = vec3f(dot((*p).r0.xyz, n), dot((*p).r1.xyz, n), dot((*p).r2.xyz, n)); let k = abs(m); var a = 2u; if (k.x >= k.y && k.x >= k.z) { a = 0u; } else if (k.y >= k.z) { a = 1u; }
  let c = counts(faces[slot]); var r: Grid; r.face = 2u * a + select(0u, 1u, m[a] > 0.0); r.first = faces[slot].x + faceStart(c, r.face); let u = (a + 1u) % 3u; let v = (a + 2u) % 3u; r.n = vec2u(c[u], c[v]); let h = max((*p).half, vec3f(1e-4)); r.g = clamp(vec2f(l[u] / h[u], l[v] / h[v]) * 0.5 + 0.5, vec2f(0.0), vec2f(0.9999)) * vec2f(r.n); return r;
}
fn texelLight(i: u32, least: f32) -> Cached { var c: Cached; let a = texels[2u * i]; if (bad4(a) || a.w < least) { return c; } c.E = a.xyz; c.n = a.w; c.sun = finite(texels[2u * i + 1u].x, 1.0); return c; }
fn whose(x: vec3f, n: vec3f) -> u32 { return askPiece; }
fn held(x: vec3f, n: vec3f, j: vec2f) -> Cached {
  var c: Cached; if (all(n == vec3f(0.0))) { return c; } let slot = whose(x, n); if (slot == NOTHING || faces[slot].y == 0u) { return c; } let r = gridOf(slot, x, n);
  for (var go = 0u; go < 6u; go++) {                                  // beside it by dice, where it is, then whichever neighbor knows
    var t = r.g; if (go == 0u) { t += j - 0.5; } else if (go > 1u) { t += select(vec2f(-1.0, 0.0), vec2f(1.0, 0.0), go == 2u); if (go > 3u) { t = r.g + select(vec2f(0.0, -1.0), vec2f(0.0, 1.0), go == 4u); } } let q = vec2u(clamp(t, vec2f(0.0), vec2f(r.n) - 0.5)); let i = r.first + q.y * r.n.x + q.x;
    if (go == 0u && JOINS && !joined(x + n * 0.08, texelPoint(slot, r.face, texelUv(i)) + n * 0.08)) { continue; } c = texelLight(i, 1.0); if (c.n > 0.0) { return c; }
  }
  return c;
}
fn heldSmooth(x: vec3f, n: vec3f) -> Cached {
  var c: Cached; if (all(n == vec3f(0.0))) { return c; } let slot = whose(x, n); if (slot == NOTHING || faces[slot].y == 0u) { return c; } let r = gridOf(slot, x, n); let g = r.g - 0.5; let f = fract(g); var ws = 0.0;
  for (var k = 0u; k < 4u; k++) {
    let o = vec2f(f32(k & 1u), f32(k >> 1u)); let q = vec2u(clamp(floor(g) + o, vec2f(0.0), vec2f(r.n) - 1.0)); let i = r.first + q.y * r.n.x + q.x; let e = texelLight(i, 1.0); if (e.n <= 0.0 || (JOINS && !joined(x + n * 0.08, texelPoint(slot, r.face, texelUv(i)) + n * 0.08))) { continue; }
    let w = mix(1.0 - f.x, f.x, o.x) * mix(1.0 - f.y, f.y, o.y) + 1e-4; c.E += e.E * w; c.sun += e.sun * w; c.n = max(c.n, e.n); ws += w;
  }
  if (ws < 0.02) { return held(x, n, vec2f(0.5)); } c.E /= ws; c.sun /= ws; return c;
}
