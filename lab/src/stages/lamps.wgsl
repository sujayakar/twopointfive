// SIGNAL 1, LAMP VISIBILITY: four fractions, 0‥1. w = of the first light here, the strongest by what it would send: the sun's disc or one lamp (lampU's second half: what it sends, and which + 1; 0 = the sun). x = of
// the other standing lamps' light (and the sun's where a lamp outshines it), y = of the changing ones' (torches), z = of the standing lamps' highlight. The first is asked at every pixel, so its shadows are
// its own; of the others one is drawn by what it would send, so the mean of "was it seen" is the seen share of their light. How much light that is, is exact, and never filtered; where no such lamp
// reaches the fraction is −1, and has no say (lampU's first half: rgb the others', a the torches').
/** the sun through the leaves, from the sun's own picture of them */
fn leafSun(x: vec3f, n: vec3f, r: vec2f) -> f32 {
  /* (the surface's own plane is known: what lies in it is not over it, at any tilt, and what is a finger's breadth over it is) */
  let S = frame.sunDir.xyz; let q = sunMap(x); if (any(q.xy < vec2f(0.002)) || any(q.xy > vec2f(0.998))) { return 1.0; } let d = vec2f(textureDimensions(sunLeaves)); let bs = basis(S); let ns = dot(n, S); let tilt = select(-1.0, 1.0, ns >= 0.0) / max(abs(ns), 0.12); var s = 0.0; let a = r.x * 6.2832; let m = mat2x2f(cos(a), sin(a), -sin(a), cos(a));
  // how far up the first leaves are says how soft their edges are: the sun is a disc
  var up = 0.0; var un = 0.0; let px = SUN_WIDE / d.x; for (var k = 0u; k < 4u; k++) { let off = m * (array<vec2f, 4>(vec2f(2.1, 0.9), vec2f(-1.2, 2.4), vec2f(-2.5, -1.0), vec2f(1.3, -2.6))[k]); let p = vec2i(vec2f(q.x, 1.0 - q.y) * d + off); let over = (textureLoad(sunLeaves, p, 0) - q.z) * 80.0; if (over > 0.3) { up += over; un += 1.0; } }
  let wide = max(0.8, select(0.0, up / max(un, 1.0) * frame.sunDir.w / px, un > 0.0));
  for (var k = 0u; k < 4u; k++) { let off = m * (array<vec2f, 4>(vec2f(0.375, 0.125), vec2f(-0.25, 0.69), vec2f(-0.94, -0.375), vec2f(0.56, -1.0))[k] * 0.9 * wide); let p = vec2i(vec2f(q.x, 1.0 - q.y) * d + off); let e = ((vec2f(p) + 0.5) / d - vec2f(q.x, 1.0 - q.y)) * SUN_WIDE; let h = -dot(bs[0] * e.x - bs[1] * e.y, n) * tilt; let over = (textureLoad(sunLeaves, p, 0) - q.z) * 80.0 - h; s += select(0.0, 1.0, over < 0.014); }
  return s * 0.25;
}
const SUN_LAMP = 255u;
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = vec2u(frame.size.xy); if (id.x >= dim.x || id.y >= dim.y) { return; }
  let rd = pxRay(id.xy); let p = underWater(unpackPx(textureLoad(surf, id.xy, 0), rd), textureLoad(frontT, id.xy, 0), rd);
  if (p.sky || p.kind == 63u) { textureStore(outVis, id.xy, vec4f(0.0)); textureStore(outU, id.xy, vec4u(0u)); textureStore(outC, id.xy, vec4f(0.0)); if (((id.x | id.y) & 1u) == 0u) { textureStore(outTone, id.xy / 2u, vec4f(0.0)); } return; }
  rngInit(id.xy, 3u); let bn = blue2(id.xy, 0u); let b2 = blue2(id.xy, 7u); let b3 = blue2(id.xy, 11u); let n = select(p.n, vec3f(0.0), p.leaf);
  /* (torches are few and each is asked: where two reach and one is behind a wall, a throw of the dice between them would be seen) */ var tw = vec3f(0.0); var ti = vec3i(-1); var tv = vec2f(0.0); var W = vec2f(0.0); var U = vec3f(0.0); var pe = vec3f(0.0); var pw = 1.0; var pick = vec3i(-1); var u = vec2f(b2.x, b3.x); var s = pxSurf(p); let shines = byFormula(p.rough) > 0.0 || isPaint(p.kind); if (isPaint(p.kind)) { s = paintFlakes(s); } var C = vec4f(0.0); var WC = 0.0; var uc = rnd();
  // the sun is one of them
  let cs = dot(p.n, frame.sunDir.xyz); let sunE = frame.sunE.xyz * select(max(cs, 0.0), 0.5, p.leaf); let sunW = lum(sunE); var fi = SUN_LAMP; var fw = sunW; var fe = sunE; var W0 = 0.0; let sees = p.leaf || dot(p.ng, frame.sunDir.xyz) > 0.0; if (!sees) { fw = 0.0; fe = vec3f(0.0); }
  for (var g = 0u; g * 4u < frame.counts.x; g++) { let reach = lampReach(p.x, g); if (max4(reach) <= 0.0) { continue; }
   for (var j = 0u; j < 4u; j++) { let i = g * 4u + j; if (reach[j] <= 0.0 || i >= frame.counts.x) { continue; }
    let e = lampE(i, p.x, n) * reach[j]; let w = lum(e); if (w <= 1e-5) { continue; } let torch = lamps[i].kind == 2u;
    if (torch) { W.y += w; if (w > tw.x) { tw = vec3f(w, tw.xy); ti = vec3i(i32(i), ti.xy); } else if (w > tw.y) { tw = vec3f(tw.x, w, tw.y); ti = vec3i(ti.x, i32(i), ti.y); } else if (w > tw.z) { tw.z = w; ti.z = i32(i); } }
    else { var qe = e; var qw = w; var qi = i; if (w > fw) { qe = fe; qw = fw; qi = fi; fe = e; fw = w; fi = i; } if (qw > 0.0) { U += qe; W0 += qw; let q = qw / W0; if (u.x < q) { pick.x = i32(qi); u.x = u.x / q; pe = qe; pw = qw; } else { u.x = (u.x - q) / (1.0 - q); } } }
    if (shines) { let h = lampCoat(i, s, -rd) * reach[j]; let m = lum(h); if (torch) { C.a += m; } else if (m > 0.0) { C = vec4f(C.rgb + h, C.a); WC += m; let g = m / WC; if (uc < g) { pick.z = i32(i); uc = uc / g; } else { uc = (uc - g) / (1.0 - g); } } }
  } }
  W.x = W0; let lampsW = W0 + select(fw, 0.0, fi == SUN_LAMP); var v = vec4f(-1.0, -1.0, -1.0, 0.0); let o0 = 0.003 * (1.0 + p.t * 0.02);
  for (var k = 0u; k < 6u; k++) { let torch = k == 2u || k > 3u; let t = select(0u, k - 3u, k > 3u);
    var l = SUN_LAMP; if (k == 0u) { l = fi; if (fw <= 0.0 || fw < 0.02 * (W0 + W.y)) { continue; } }
    else if (k == 1u) { if (pick.x < 0) { continue; } l = u32(pick.x); if (W0 < 0.02 * fw * v.w || W0 < 0.02 * W.y) { continue; } }     /* (what is lost beside the first where that is seen, or a torch) */
    else if (torch) { if (W.y < 0.01 * lampsW || ti[t] < 0 || tw[t] < 0.03 * tw.x) { continue; } l = u32(ti[t]); }
    else { if (pick.z < 0 || WC < 0.01 * (lampsW * lum(p.albedo) * 0.32 + 2e-4)) { continue; } l = u32(pick.z); }
    var d: vec3f; var far = 400.0; let sun = l == SUN_LAMP; let fine = ((u32(FINE) >> select(select(k, 2u, torch), 0u, sun)) & 1u) != 0u; var slot = 99u; if (torch) { slot = (torchK[3u + (l >> 4u)][(l >> 2u) & 3u] >> (8u * (l & 3u))) & 255u; } let exact = fine && slot > 98u; leafMode = select(2u, 0u, exact); noLeaves = (sun && !fine) || slot < 99u;     /* (a torch has a picture of the leaves, as the sun has) */
    let pw = 2.0 * length(frame.camUp.xyz) / frame.size.y * p.t; if (sun) { d = coneDir(frame.sunDir.xyz, frame.sunDir.w, bn); footSharp(pw, 2.0 * frame.sunDir.w); }
    else { var a: vec4f; if (k == 3u) { a = aimCoat(l, s, -rd, vec2f(b3.y, b2.y)); } else { a = aimSeen(l, p.x, select(vec2f(b2.y, b3.y), vec2f(b3.y, b2.y), torch)); } d = a.xyz; far = a.w - 0.02; footSharp(pw, lampSize(l) / max(a.w, 0.1)); } leafTo = select(0.0, 1e9, exact); blkOn = !exact;
    var T = 0.0; if (dot(p.ng, d) > 0.0 || p.leaf) { let side = select(-1.0, 1.0, dot(p.ng, d) > 0.0); T = lum(visible(p.x + p.ng * (o0 * side), d, 1e-3, far)); if (T > 0.0 && sun && noLeaves) { T *= leafSun(p.x, p.ng * side, b3); } else if (T > 0.0 && slot < 99u) { T *= leafTorch(slot, l, p.x, p.ng * side, b3); } }
    if (k == 0u) { v.w = T; } else if (torch) { tv += vec2f(T, 1.0) * tw[t]; } else if (k == 1u) { v.x = T; } else { v.z = T; }
  }
  if (tv.y > 0.0) { v.y = tv.x / tv.y; }
  if (((id.x | id.y) & 1u) == 0u) { let seen = max(v.x, 0.0); textureStore(outTone, id.xy / 2u, vec4f(clamp(pe / pw * W.x / max(U, vec3f(1e-6)), vec3f(0.0), vec3f(8.0)) * seen, seen)); }     /* (lamps of other colors may be hidden here: which color comes through is kept beside how much) */
  textureStore(outVis, id.xy, v); textureStore(outU, id.xy, packLight(vec4f(U, W.y), select(vec4f(fe, f32(fi) + 1.0), vec4f(0.0), fi == SUN_LAMP))); textureStore(outC, id.xy, C);
}
