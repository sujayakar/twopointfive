// SIGNAL 1, LAMP VISIBILITY: three fractions, 0‥1. x = of the standing lamps' light, y = of the changing ones' (torches), z = of the standing lamps' highlight, w = of the sun's disc. A lamp is
// drawn by what it would send, so the mean of "was it seen" is the seen share of their light; how much that light is, is exact, and never filtered (lampU: rgb the standing ones', a the torches').
/** the sun through the leaves, from the sun's own picture of them */
fn leafSun(x: vec3f, n: vec3f, r: vec2f) -> f32 {
  let o = x + n * 0.015 + frame.sunDir.xyz * 0.05; let q = sunMap(o); if (any(q.xy < vec2f(0.002)) || any(q.xy > vec2f(0.998))) { return 1.0; } let d = vec2f(textureDimensions(sunLeaves)); var s = 0.0; let a = r.x * 6.2832; let m = mat2x2f(cos(a), sin(a), -sin(a), cos(a));
  for (var k = 0u; k < 4u; k++) { let off = m * (array<vec2f, 4>(vec2f(0.3, 0.1), vec2f(-0.2, 0.55), vec2f(-0.75, -0.3), vec2f(0.45, -0.8))[k]); let p = vec2i(vec2f(q.x, 1.0 - q.y) * d + off * 1.3); let over = (textureLoad(sunLeaves, p, 0) - q.z) * 80.0; s += select(mix(0.12, 0.025, smoothstep(0.1, 0.8, over)), 1.0, over < 0.032); } /* the first leaf over it, metres up: the farther, the more of them */ return s * 0.25;
}
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = vec2u(frame.size.xy); if (id.x >= dim.x || id.y >= dim.y) { return; }
  let rd = pxRay(id.xy); let p = underWater(unpackPx(textureLoad(surf, id.xy, 0), rd), textureLoad(frontT, id.xy, 0), rd);
  if (p.sky || p.kind == 63u) { textureStore(outVis, id.xy, vec4f(0.0)); textureStore(outU, id.xy, vec4f(0.0)); textureStore(outC, id.xy, vec4f(0.0)); return; }
  rngInit(id.xy, 3u); let bn = blue2(id.xy, 0u); let b2 = blue2(id.xy, 7u); let b3 = blue2(id.xy, 11u); let n = select(p.n, vec3f(0.0), p.leaf);
  var W = vec2f(0.0); var U = vec3f(0.0); var pick = vec3i(-1); var u = vec2f(b2.x, b3.x); let s = pxSurf(p); let shines = byFormula(p.rough) > 0.0 && !p.leaf; var C = vec4f(0.0); var WC = 0.0; var uc = rnd();
  for (var g = 0u; g * 4u < frame.counts.x; g++) { let reach = lampReach(p.x, g); if (max4(reach) <= 0.0) { continue; }
   for (var j = 0u; j < 4u; j++) { let i = g * 4u + j; if (reach[j] <= 0.0 || i >= frame.counts.x) { continue; }
    let e = lampE(i, p.x, n) * reach[j]; let w = lum(e); if (w <= 1e-5) { continue; } let c = select(0u, 1u, lamps[i].kind == 2u); if (c == 0u) { U += e; } W[c] += w; let q = w / W[c]; if (u[c] < q) { pick[c] = i32(i); u[c] = u[c] / q; } else { u[c] = (u[c] - q) / (1.0 - q); }
    if (shines) { let h = lampCoat(i, s, -rd) * reach[j]; let m = lum(h); if (c == 1u) { C.a += m; } else if (m > 0.0) { C = vec4f(C.rgb + h, C.a); WC += m; let g = m / WC; if (uc < g) { pick.z = i32(i); uc = uc / g; } else { uc = (uc - g) / (1.0 - g); } } }
  } }
  var v = vec4f(1.0, 1.0, 1.0, 0.0); let o0 = 0.003 * (1.0 + p.t * 0.02);
  for (var k = 0u; k < 4u; k++) {
    var d: vec3f; var far = 400.0; fineLeaves = ((u32(FINE) >> k) & 1u) != 0u; noLeaves = k == 0u && !fineLeaves; if (k == 3u && WC < 0.02 * lum(U) + 1e-4) { continue; }
    if (k == 0u) { d = coneDir(frame.sunDir.xyz, frame.sunDir.w, bn); if (max3(frame.sunE.xyz) <= 0.0 || (dot(p.ng, d) <= 0.0 && !p.leaf)) { continue; } }
    else { let l = pick[k - 1u]; if (l < 0) { continue; } let a = aimLamp(u32(l), p.x, select(vec2f(b2.y, b3.y), vec2f(b3.y, b2.y), k == 2u)); d = a.dir; far = a.dist - 0.02; if (max3(a.E) <= 0.0 || (dot(p.ng, d) <= 0.0 && !p.leaf)) { v[k - 1u] = 0.0; continue; } }
    let T = lum(visible(p.x + p.ng * select(-o0, o0, dot(p.ng, d) > 0.0), d, 1e-3, far)); if (k == 0u) { v.w = T; if (T > 0.0 && noLeaves) { v.w = T * leafSun(p.x, p.ng * select(-1.0, 1.0, dot(p.ng, d) > 0.0), b3); } } else { v[k - 1u] = T; }
  }
  textureStore(outVis, id.xy, v); textureStore(outU, id.xy, vec4f(U, W.y)); textureStore(outC, id.xy, C);
}
