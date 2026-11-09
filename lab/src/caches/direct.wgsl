/** DIRECT LIGHT, ASKED: the sun and one lamp, a ray each. xyz what the lamp lands, w how much of the sun is seen. LAMPS: what the lamp would land with nothing in the way is its formula's (exact for a rectangle), and the
 *  ray only says whether it is seen: near a lamp a point of it drawn evenly says next to nothing most times and a hundred times too much now and then */
fn askDirect(x: vec3f, n: vec3f, sunToo: bool) -> vec4f {
  let k0 = vec4f(footW, footS, leafTo, smallTo); let br = byReach; byReach = true; footSolid(0.32, 1.0); var r = vec4f(0.0); let every = all(n == vec3f(0.0)); let y = x + n * 0.012; let cs = select(dot(n, frame.sunDir.xyz), 1.0, every);
  for (var k = select(1u, 0u, sunToo); k < 2u; k++) {
    var d: vec3f; var far = 400.0; var e = vec3f(0.0);
    if (k == 0u) { if (cs <= 0.0 || max3(frame.sunE.xyz) <= 0.0) { continue; } d = coneDir(frame.sunDir.xyz, frame.sunDir.w, rnd2()); }
    else { let l = drawLamp(y, n); if (l.x < 0.0) { continue; } if (LAMPS) { let i = u32(l.x); for (var a = 0u; a < 3u; a++) { let v = aimSeen(i, y, rnd2()); d = v.xyz; far = v.w - 0.02; if (every || dot(n, d) > 0.0) { break; } } e = lampE(i, y, n) * (l.y * select(1.0, 0.5, every)); if (!every && dot(n, d) <= 0.0) { continue; } }
      else { let q = aimLamp(u32(l.x), y, rnd2()); d = q.dir; far = q.dist - 0.02; e = q.E * l.y * select(max(dot(n, d), 0.0), 0.25, every); } if (max3(e) <= 0.0) { continue; } }
    let T = visible(y, d, 2e-3, far); if (k == 0u) { r.w = lum(T); } else { r += vec4f(min(e * T, vec3f(select(300.0, 1e4, LAMPS))), 0.0); }
  }
  footW = k0.x; footS = k0.y; leafTo = k0.z; smallTo = k0.w; byReach = br; return r;
}
/** the same for what has no facing: one lamp (which way, how much), and the sun */
fn lampsFrom(x: vec3f) -> mat2x3f { let l = drawLamp(x, vec3f(0.0)); if (l.x < 0.0) { return mat2x3f(vec3f(0.0, 1.0, 0.0), vec3f(0.0)); } let q = aimLamp(u32(l.x), x, rnd2()); let e = min(q.E * l.y, vec3f(300.0)); if (max3(e) <= 0.0) { return mat2x3f(vec3f(0.0, 1.0, 0.0), vec3f(0.0)); } return mat2x3f(q.dir, e * visible(x, q.dir, 2e-3, q.dist - 0.02)); }
fn sunFrom(x: vec3f) -> f32 { if (max3(frame.sunE.xyz) <= 0.0) { return 0.0; } return lum(visible(x, coneDir(frame.sunDir.xyz, frame.sunDir.w, rnd2()), 2e-3, 400.0)); }
