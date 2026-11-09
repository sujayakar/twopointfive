// SIGNAL 1, LAMP VISIBILITY: three fractions, 0‥1. x = of the standing lamps' light, y = of the changing ones' (torches), z = of the standing lamps' highlight, w = of the sun's disc. A lamp is
// drawn by what it would send, so the mean of "was it seen" is the seen share of their light; how much that light is, is exact, and never filtered; where no such lamp reaches the fraction is −1, and has no say (lampU: rgb the standing ones', a the torches').
/** the sun through the leaves, from the sun's own picture of them */
fn leafSun(x: vec3f, n: vec3f, r: vec2f) -> f32 {
  /* (the surface's own plane is known: what lies in it is not over it, at any tilt, and what is a finger's breadth over it is) */
  let S = frame.sunDir.xyz; let q = sunMap(x); if (any(q.xy < vec2f(0.002)) || any(q.xy > vec2f(0.998))) { return 1.0; } let d = vec2f(textureDimensions(sunLeaves)); let bs = basis(S); let ns = dot(n, S); let tilt = select(-1.0, 1.0, ns >= 0.0) / max(abs(ns), 0.12); var s = 0.0; let a = r.x * 6.2832; let m = mat2x2f(cos(a), sin(a), -sin(a), cos(a));
  for (var k = 0u; k < 4u; k++) { let off = m * (array<vec2f, 4>(vec2f(0.3, 0.1), vec2f(-0.2, 0.55), vec2f(-0.75, -0.3), vec2f(0.45, -0.8))[k]); let p = vec2i(vec2f(q.x, 1.0 - q.y) * d + off); let e = ((vec2f(p) + 0.5) / d - vec2f(q.x, 1.0 - q.y)) * SUN_WIDE; let h = -dot(bs[0] * e.x - bs[1] * e.y, n) * tilt; let over = (textureLoad(sunLeaves, p, 0) - q.z) * 80.0 - h; s += select(0.07 * exp(-7.0 * over) + 0.005, 1.0, over < 0.014); } /* the first leaf over it, metres up: the farther, the more of them between */ return s * 0.25;
}
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = vec2u(frame.size.xy); if (id.x >= dim.x || id.y >= dim.y) { return; }
  let rd = pxRay(id.xy); let p = underWater(unpackPx(textureLoad(surf, id.xy, 0), rd), textureLoad(frontT, id.xy, 0), rd);
  if (p.sky || p.kind == 63u) { textureStore(outVis, id.xy, vec4f(0.0)); textureStore(outU, id.xy, vec4f(0.0)); textureStore(outC, id.xy, vec4f(0.0)); if (((id.x | id.y) & 1u) == 0u) { textureStore(outTone, id.xy / 2u, vec4f(0.0)); } return; }
  rngInit(id.xy, 3u); let bn = blue2(id.xy, 0u); let b2 = blue2(id.xy, 7u); let b3 = blue2(id.xy, 11u); let n = select(p.n, vec3f(0.0), p.leaf);
  /* (torches are few and each is asked: where two reach and one is behind a wall, a throw of the dice between them would be seen) */ var tw = vec3f(0.0); var ti = vec3i(-1); var tv = vec2f(0.0); var W = vec2f(0.0); var U = vec3f(0.0); var pe = vec3f(0.0); var pw = 1.0; var pick = vec3i(-1); var u = vec2f(b2.x, b3.x); var s = pxSurf(p); let shines = byFormula(p.rough) > 0.0 || isPaint(p.kind); if (isPaint(p.kind)) { s = paintFlakes(s); } var C = vec4f(0.0); var WC = 0.0; var uc = rnd();
  for (var g = 0u; g * 4u < frame.counts.x; g++) { let reach = lampReach(p.x, g); if (max4(reach) <= 0.0) { continue; }
   for (var j = 0u; j < 4u; j++) { let i = g * 4u + j; if (reach[j] <= 0.0 || i >= frame.counts.x) { continue; }
    let e = lampE(i, p.x, n) * reach[j]; let w = lum(e); if (w <= 1e-5) { continue; } let c = select(0u, 1u, lamps[i].kind == 2u); if (c == 0u) { U += e; } W[c] += w; if (c == 1u) { if (w > tw.x) { tw = vec3f(w, tw.xy); ti = vec3i(i32(i), ti.xy); } else if (w > tw.y) { tw = vec3f(tw.x, w, tw.y); ti = vec3i(ti.x, i32(i), ti.y); } else if (w > tw.z) { tw.z = w; ti.z = i32(i); } } let q = w / W[c]; if (u[c] < q) { pick[c] = i32(i); u[c] = u[c] / q; if (c == 0u) { pe = e; pw = w; } } else { u[c] = (u[c] - q) / (1.0 - q); }
    if (shines) { let h = lampCoat(i, s, -rd) * reach[j]; let m = lum(h); if (c == 1u) { C.a += m; } else if (m > 0.0) { C = vec4f(C.rgb + h, C.a); WC += m; let g = m / WC; if (uc < g) { pick.z = i32(i); uc = uc / g; } else { uc = (uc - g) / (1.0 - g); } } }
  } }
  var v = vec4f(-1.0, -1.0, -1.0, 0.0); let o0 = 0.003 * (1.0 + p.t * 0.02);
  for (var k = 0u; k < 6u; k++) { let torch = k == 2u || k > 3u; let t = select(0u, k - 3u, k > 3u);
    var d: vec3f; var far = 400.0; let fine = ((u32(FINE) >> select(k, 2u, torch)) & 1u) != 0u; leafMode = select(2u, 0u, fine); noLeaves = k == 0u && !fine; if (k == 3u && WC < 0.01 * (lum(U) * lum(p.albedo) * 0.32 + 2e-4)) { continue; }
    if (k == 0u && lum(frame.sunE.xyz) * select(max(dot(p.n, frame.sunDir.xyz), 0.0), 0.5, p.leaf) < 0.02 * (lum(U) + W.y)) { continue; }     /* (a moon lost in lamplight; and below, a torch) */
    if (torch && (W.y < 0.01 * lum(U) || ti[t] < 0 || tw[t] < 0.03 * tw.x)) { continue; }
    if (k == 0u) { d = coneDir(frame.sunDir.xyz, frame.sunDir.w, bn); if (max3(frame.sunE.xyz) <= 0.0 || (dot(p.ng, d) <= 0.0 && !p.leaf)) { continue; } }
    else { var l = ti[t]; if (!torch) { l = pick[k - 1u]; } if (l < 0) { continue; } var a: vec4f; if (k == 3u) { a = aimCoat(u32(l), s, -rd, vec2f(b3.y, b2.y)); } else { a = aimSeen(u32(l), p.x, select(vec2f(b2.y, b3.y), vec2f(b3.y, b2.y), k == 2u)); } d = a.xyz; far = a.w - 0.02; if (dot(p.ng, d) <= 0.0 && !p.leaf) { if (torch) { tv.y += tw[t]; } else { v[k - 1u] = 0.0; } continue; } }
    let T = lum(visible(p.x + p.ng * select(-o0, o0, dot(p.ng, d) > 0.0), d, 1e-3, far)); if (k == 0u) { v.w = T; if (T > 0.0 && noLeaves) { v.w = T * leafSun(p.x, p.ng * select(-1.0, 1.0, dot(p.ng, d) > 0.0), b3); } } else if (torch) { tv += vec2f(T, 1.0) * tw[t]; } else { v[k - 1u] = T; }
  }
  if (tv.y > 0.0) { v.y = tv.x / tv.y; }
  if (((id.x | id.y) & 1u) == 0u) { let seen = max(v.x, 0.0); textureStore(outTone, id.xy / 2u, vec4f(clamp(pe / pw * W.x / max(U, vec3f(1e-6)), vec3f(0.0), vec3f(8.0)) * seen, seen)); }     /* (lamps of other colors may be hidden here: which color comes through is kept beside how much) */
  textureStore(outVis, id.xy, v); textureStore(outU, id.xy, vec4f(U, W.y)); textureStore(outC, id.xy, C);
}
