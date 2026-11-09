// LIGHT IN THE FOLIAGE: one more look an entry. Six rays, one about each way a leaf may face; what they meet is read from the light on surfaces and from the foliage itself, so that light goes
// on through it, leaf to leaf. The sun by what gets through, one lamp. Words: J, sun · six colors · how many looks
fn six(i: u32) -> vec3f { var d = vec3f(0.0); d[i >> 1u] = select(1.0, -1.0, (i & 1u) == 1u); return d; }
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3u) {
  let s = id.x; if (s >= FOL_MANY) { return; } let base = s * 12u; var q: array<u32, 12>; for (var k = 0u; k < 12u; k++) { q[k] = field[base + k]; } var n = f32(q[11]); let x0 = folAt[s].xyz; let st = steady(x0); if (changes.all.y > 0.5) { n = 0.0; }
  if (n > 16.0 && st > 0.9 && (s & 7u) != (frame.counts.z & 7u)) { return; }
  rngKey(s, 203u); leafMode = 2u; byReach = true; let x = x0 + (vec3f(rnd(), rnd(), rnd()) - 0.5) * 0.16; let c0 = clamp(vec3i(floor((x0 - FOL_LO) / FOL_C)), vec3i(0), FOL_N - 1); let col = folColors(folCell(c0));
  var sun = 0.0; if (max3(frame.sunE.xyz) > 0.0) { sun = lum(visible(x, coneDir(frame.sunDir.xyz, frame.sunDir.w, rnd2()), 2e-3, 400.0)); }
  var lampE = vec3f(0.0); { let l = drawLamp(x, vec3f(0.0)); if (l.x >= 0.0) { let a = aimLamp(u32(l.x), x, rnd2()); let e = a.E * l.y * 0.25; if (max3(e) > 0.0) { lampE = min(e * visible(x, a.dir, 2e-3, a.dist - 0.02), vec3f(300.0)); } } }
  var E: array<vec3f, 6>; var mean = vec3f(0.0); let k1 = 1.0 / (n + 1.0);
  for (var i = 0u; i < 6u; i++) {
    let u = rnd2(); let r = sqrt(u.x); let bs = basis(six(i)); let d = normalize(bs[0] * (r * cos(6.2832 * u.y)) + bs[1] * (r * sin(6.2832 * u.y)) + bs[2] * sqrt(max(1.0 - u.x, 0.0)));
    let h = nearest(x, d, 2e-3, 600.0, 0u, 1e9); var L = vec3f(0.0); if (h.piece == NOTHING) { L = skyLod(d, 2.0); } else { let p = surfaceMean(x, d, h); let own = (pieces[h.piece].lamp & 255u) != 255u; L = select(p.glow, vec3f(0.0), own) + sentBack(p, false, rnd2(), d); }
    let sm = smokeSeen(x, d, min(h.t, 80.0)); L = (L * sm.a + sm.rgb) * h.thru; let f = folWalk(x, d, min(h.t, 120.0), true); E[i] = min(L * f.T + f.L, vec3f(50.0)) * PI;
  }
  // (kept as halves in a row: three a color)
  var old: array<f32, 18>; for (var j = 0u; j < 9u; j++) { let v = unpack2x16float(q[2u + j]); old[2u * j] = v.x; old[2u * j + 1u] = v.y; }
  for (var i = 0u; i < 6u; i++) { for (var j = 0u; j < 3u; j++) { let o = i * 3u + j; let was = select(0.0, old[o], n > 0.0); old[o] = clamp(finite(mix(was, E[i][j], k1), 0.0), 0.0, 6e4); mean[j] += old[o] / 6.0; } }
  let jw = unpack2x16float(q[0]); let jz = unpack2x16float(q[1]); let a = (col[0] + col[1]) / PI; let keep = vec4f(select(vec3f(0.0), vec3f(jw, jz.x), n > 0.0), select(0.0, jz.y, n > 0.0)); let J = mix(keep.xyz, a * (mean + lampE), k1); let sv = mix(keep.w, sun, k1);
  field[base] = pack2x16float(clamp(J.xy, vec2f(0.0), vec2f(6e4))); field[base + 1u] = pack2x16float(clamp(vec2f(J.z, sv), vec2f(0.0), vec2f(6e4))); for (var j = 0u; j < 9u; j++) { field[base + 2u + j] = pack2x16float(vec2f(old[2u * j], old[2u * j + 1u])); } field[base + 11u] = u32(min(n + 1.0, mix(6.0, 64.0, st)));
}
