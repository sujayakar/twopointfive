/** DIRECT LIGHT, ASKED: the sun and one lamp, a ray each. xyz what the lamp lands, w how much of the sun is seen */
fn askDirect(x: vec3f, n: vec3f, sunToo: bool) -> vec4f {
  let k0 = vec4f(footW, footS, leafTo, smallTo); let br = byReach; byReach = true; footSolid(0.32, 1.0); var r = vec4f(0.0); let every = all(n == vec3f(0.0)); let y = x + n * 0.012; let cs = select(dot(n, frame.sunDir.xyz), 1.0, every);
  for (var k = select(1u, 0u, sunToo); k < 2u; k++) {
    var d: vec3f; var far = 400.0; var e = vec3f(0.0);
    if (k == 0u) { if (cs <= 0.0 || max3(frame.sunE.xyz) <= 0.0) { continue; } d = coneDir(frame.sunDir.xyz, frame.sunDir.w, rnd2()); }
    else { let l = drawLamp(y, n); if (l.x < 0.0) { continue; } let q = aimLamp(u32(l.x), y, rnd2()); d = q.dir; far = q.dist - 0.02; e = q.E * l.y * select(max(dot(n, d), 0.0), 0.25, every); if (max3(e) <= 0.0) { continue; } }
    let T = visible(y, d, 2e-3, far); if (k == 0u) { r.w = lum(T); } else { r += vec4f(min(e * T, vec3f(300.0)), 0.0); }
  }
  footW = k0.x; footS = k0.y; leafTo = k0.z; smallTo = k0.w; byReach = br; return r;
}
