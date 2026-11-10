// NOTHING IS KEPT: whoever asks what lands somewhere follows a path on from there, BOUNCES surfaces long. For judging what is kept (kept.ts).
var<private> askMoves: bool = false;
struct Cached { E: vec3f, sun: f32, n: f32 };
fn cacheRead(x0: vec3f, n0: vec3f, j: vec2f) -> Cached { return follow(x0, n0); }
fn cacheSmooth(x0: vec3f, n0: vec3f) -> Cached { return follow(x0, n0); }
fn follow(x0: vec3f, n0: vec3f) -> Cached {
  let was = vec4f(footW, footS, leafTo, 0.0); let was1 = lim; let was2 = blk; let was3 = leafOnly; let br = byReach; byReach = false; var c: Cached; c.n = 100.0; var x = x0 + n0 * 0.012; var n = n0; var thr = vec3f(1.0); var thrSun = vec3f(0.0); let most = 1e4;
  for (var v = 0u; v < BOUNCES; v++) {
    let cs = dot(n, frame.sunDir.xyz);
    for (var k = 0u; k < 2u; k++) {
      var d: vec3f; var far = 400.0; var e = vec3f(0.0);
      if (k == 0u) { if (cs <= 0.0 || max3(frame.sunE.xyz) <= 0.0) { continue; } d = coneDir(frame.sunDir.xyz, frame.sunDir.w, rnd2()); footLight(2.0 * frame.sunDir.w, 1.0); }
      else { let l = drawLamp(x, n); if (l.x < 0.0) { continue; } let q = aimLamp(u32(l.x), x, rnd2()); d = q.dir; far = q.dist - 0.02; footLight(lampSize(u32(l.x)), far); e = q.E * l.y * max(dot(n, d), 0.0); if (max3(e) <= 0.0) { continue; } }
      let T = visible(x, d, 2e-3, far); if (k == 1u) { c.E += thr * min(e * T, vec3f(most)); } else if (v == 0u) { c.sun = lum(T); } else { c.E += thrSun * T; }
    }
    if (v + 1u >= BOUNCES) { break; }
    let w = wideDir(x, n, rnd2(), rnd()); if (w.w <= 0.0) { break; }
    foot(0.02, @tune.wide); let h = nearest(x, w.xyz, 2e-3, 600.0, 0u, 1e9); let sm = smokeSeen(x, w.xyz, min(h.t, 80.0)); let fo = blkSeen(); c.E += min(thr * (sm.rgb * fo.T + fo.L) * w.w, vec3f(most)); thr *= h.thru * fo.T * (w.w * sm.a);
    if (h.piece == NOTHING) { c.E += min(thr * skyLod(w.xyz, 1.0), vec3f(most)); break; }
    let q = surfaceMean(x, w.xyz, h); let own = (pieces[h.piece].lamp & 255u) != 255u; if (!own) { c.E += min(thr * q.glow, vec3f(most)); }
    let p = sentParts(q, w.xyz); thrSun = thr * p[1]; thr *= p[0]; n = q.ng; x = q.x + n * 0.012;
    if (v >= 3u) { let k = clamp(max3(thr) * 1.5, 0.05, 0.95); if (rnd() > k) { break; } thr /= k; thrSun /= k; }
  }
  footW = was.x; footS = was.y; leafTo = was.z; lim = was1; blk = was2; leafOnly = was3; byReach = br; return c;
}
