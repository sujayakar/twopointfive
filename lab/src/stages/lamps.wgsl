// SIGNAL 1, LAMP VISIBILITY: three fractions, 0‥1. x = of the standing lamps' light, y = of the changing ones' (torches), w = of the sun's disc. A lamp is
// drawn by what it would send, so the mean of "was it seen" is the seen share of their light; how much that light is, is exact, and never filtered (lampU: rgb the standing ones', a the torches').
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = vec2u(frame.size.xy); if (id.x >= dim.x || id.y >= dim.y) { return; }
  let rd = pxRay(id.xy); let p = unpackPx(textureLoad(surf, id.xy, 0), rd);
  if (p.sky || p.kind == 99u) { textureStore(outVis, id.xy, vec4f(0.0)); textureStore(outU, id.xy, vec4f(0.0)); return; }
  rngInit(id.xy, 3u); let bn = blue2(id.xy, 0u); let b2 = blue2(id.xy, 7u); let b3 = blue2(id.xy, 11u); let n = select(p.n, vec3f(0.0), p.leaf);
  var W = vec2f(0.0); var U = vec3f(0.0); var pick = vec2i(-1); var u = vec2f(b2.x, b3.x);
  for (var i = 0u; i < frame.counts.x; i++) {
    let e = lampE(i, p.x, n); let w = lum(e); if (w <= 1e-5) { continue; } let c = select(0u, 1u, lamps[i].kind == 2u); if (c == 0u) { U += e; } W[c] += w; let q = w / W[c]; if (u[c] < q) { pick[c] = i32(i); u[c] = u[c] / q; } else { u[c] = (u[c] - q) / (1.0 - q); }
  }
  var v = vec4f(1.0, 1.0, 0.0, 0.0); let o0 = 0.003 * (1.0 + p.t * 0.02);
  for (var k = 0u; k < 3u; k++) {
    var d: vec3f; var far = 400.0; fineLeaves = k == 0u;
    if (k == 0u) { d = coneDir(frame.sunDir.xyz, frame.sunDir.w, bn); if (max3(frame.sunE.xyz) <= 0.0 || (dot(p.ng, d) <= 0.0 && !p.leaf)) { continue; } }
    else { let l = pick[k - 1u]; if (l < 0) { continue; } let a = aimLamp(u32(l), p.x, select(vec2f(b2.y, b3.y), vec2f(b3.y, b2.y), k == 2u)); d = a.dir; far = a.dist - 0.02; if (max3(a.E) <= 0.0 || (dot(p.ng, d) <= 0.0 && !p.leaf)) { v[k - 1u] = 0.0; continue; } }
    let T = lum(visible(p.x + p.ng * select(-o0, o0, dot(p.ng, d) > 0.0), d, 1e-3, far)); if (k == 0u) { v.w = T; } else { v[k - 1u] = T; }
  }
  textureStore(outVis, id.xy, v); textureStore(outU, id.xy, vec4f(U, W.y));
}
