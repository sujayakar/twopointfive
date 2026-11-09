// NOTHING IS KEPT: whoever asks what lands somewhere follows a path on from there, BOUNCES surfaces long.
const CELL = 0.32;
fn cacheRead(x0: vec3f, n0: vec3f, j: vec2f) -> Cached { return follow(x0, n0); }
fn cacheSmooth(x0: vec3f, n0: vec3f) -> Cached { return follow(x0, n0); }
fn follow(x0: vec3f, n0: vec3f) -> Cached {
  let k0 = vec4f(footW, footS, leafTo, smallTo); let br = byReach; byReach = !EXACT; footSolid(0.32, 1.0); var c: Cached; c.n = 100.0; var x = x0 + n0 * 0.012; var n = n0; var thr = vec3f(1.0); let most = select(300.0, 1e4, EXACT);
  for (var v = 0u; v < BOUNCES; v++) {
    let every = all(n == vec3f(0.0)); let cs = select(dot(n, frame.sunDir.xyz), 1.0, every);
    for (var k = 0u; k < 2u; k++) {
      var d: vec3f; var far = 400.0; var e = vec3f(0.0);
      if (k == 0u) { if (cs <= 0.0 || max3(frame.sunE.xyz) <= 0.0) { continue; } d = coneDir(frame.sunDir.xyz, frame.sunDir.w, rnd2()); e = frame.sunE.xyz * cs; }
      else { let l = drawLamp(x, n); if (l.x < 0.0) { continue; } let q = aimLamp(u32(l.x), x, rnd2()); d = q.dir; far = q.dist - 0.02; e = q.E * l.y * select(max(dot(n, d), 0.0), 0.25, every); if (max3(e) <= 0.0) { continue; } }
      let T = visible(x, d, 2e-3, far); if (k == 0u && v == 0u) { c.sun = lum(T); } else { c.E += thr * min(e * T, vec3f(most)); }
    }
    if (v + 1u >= BOUNCES) { break; }
    let u = rnd2(); let pk = rnd(); var w: vec4f; if (every) { let z = u.x * 2.0 - 1.0; let r = sqrt(max(1.0 - z * z, 0.0)); w = vec4f(r * cos(6.2832 * u.y), z, r * sin(6.2832 * u.y), PI); } else { w = wideDir(x, n, u, pk); } if (w.w <= 0.0) { break; }
    let h = nearest(x, w.xyz, 2e-3, 600.0, 0u, 1e9); let sm = smokeSeen(x, w.xyz, min(h.t, 80.0)); let fo = folWalk(x, w.xyz, min(h.t, 120.0), true); c.E += min(thr * (sm.rgb * fo.T + fo.L) * w.w, vec3f(most)); thr *= h.thru * fo.T * (w.w * sm.a);
    if (h.piece == NOTHING) { c.E += min(thr * skyLod(w.xyz, 1.0), vec3f(most)); break; }
    if ((h.piece & LEAF_HIT) != 0u) { c.E += min(thr * folBack(x + w.xyz * h.t, w.xyz), vec3f(most)); break; }
    let q = surfaceMean(x, w.xyz, h); let own = (pieces[h.piece].lamp & 255u) != 255u; if (!own) { c.E += min(thr * q.glow, vec3f(most)); }
    thr *= returns(q) / PI; n = q.ng; x = q.x + n * 0.012;
    if (v >= 3u) { let p = clamp(max3(thr) * 1.5, 0.05, 0.95); if (rnd() > p) { break; } thr /= p; }
  }
  footW = k0.x; footS = k0.y; leafTo = k0.z; smallTo = k0.w; byReach = br; return c;
}
