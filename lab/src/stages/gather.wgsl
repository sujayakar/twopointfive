// SIGNAL 2, WIDE INCOMING. Rays by the cosine and through openings; what they hit is read from the light on surfaces. Founds the cells it lands on. outE: what lands on the flat of the surface; outV:
// which way it comes from (its luminance times its way, summed), so that bumps and rough shine can be lit from the right side, at every pixel.
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = textureDimensions(outE); if (id.x >= dim.x || id.y >= dim.y) { return; }
  let full = min(id.xy * 2u, vec2u(frame.size.xy) - 1u); let rd = pxRay(full); let p = underWater(unpackPx(textureLoad(surf, full, 0), rd), textureLoad(frontT, full, 0), rd); let who = id.y * dim.x + id.x;
  if (p.sky) { textureStore(outE, id.xy, vec4f(0.0)); textureStore(outV, id.xy, vec4f(0.0)); return; }
  if (p.kind == 63u) {                                             // a cut wall: what falls on one side of it or the other
    rngInit(id.xy, 17u); let n = p.n * select(-1.0, 1.0, rnd() < 0.5); let x = p.x + n * (p.rough + 0.003); let c = cacheRead(x, n, rnd2()); let w0 = who * 2u; if (w0 < HITS) { let wants = cacheWants(x, n, false); if (wants != 0u) { hits[w0] = vec4u(bitcast<vec3u>(x), octPack(n)); for (var l = 0u; l < 4u; l++) { if ((wants & (1u << l)) != 0u) { cacheFound(cacheKeyOf(x, n, l & 1u, l >> 1u), w0); } } } }
    textureStore(outE, id.xy, vec4f(select(vec3f(0.0), min(c.E + frame.sunE.xyz * max(dot(n, frame.sunDir.xyz), 0.0) * c.sun, vec3f(50.0)), c.n > 0.0), 4.0)); textureStore(outV, id.xy, vec4f(0.0)); return;
  }
  if (p.leaf && folHas(p.x)) { textureStore(outE, id.xy, vec4f(0.0)); textureStore(outV, id.xy, vec4f(0.0)); return; }          // (a leaf reads its light from the foliage, at every pixel: see resolve)
  rngInit(id.xy, 17u); leafMode = 2u; let x = p.x + p.ng * 0.004 * (1.0 + p.t * 0.02); var E = vec3f(0.0); var way = vec3f(0.0); var dist = 0.0; let N = u32(@tune.rays); let px = 2.0 * length(frame.camUp.xyz) / frame.size.y * 2.0 * p.t;
  for (var k = 0u; k < N; k++) {
    let side = p.leaf && rnd() < 0.45; let n = select(select(p.ng, p.n, p.leaf), -p.n, side); let u = blue2(id.xy, 1u + k); let w = wideDir(select(x, p.x - p.ng * 0.02, side), n, u, rnd()); if (w.w <= 0.0) { continue; }
    if (dot(w.xyz, p.ng) * select(1.0, -1.0, side) <= 0.0) { continue; }
    // the path goes on till it is as wide as what is kept: then that answers
    var d = w.xyz; var o = select(x, p.x - p.ng * 0.02, side) + d * select(0.0, 0.03, p.leaf); var wide = px; var thr = vec3f(1.0); var L = vec3f(0.0);
    for (var v = 0u; v < 4u; v++) {
      foot(wide, @tune.wide); leafTo = 0.0; /* (leaf by leaf near by cost five milliseconds in the garden and gained nothing to speak of) */ let h = nearest(o, d, 2e-3, 600.0, 0u, 1e9); let miss = h.piece == NOTHING; let leaf = !miss && (h.piece & LEAF_HIT) != 0u; let lt = min(leafTo, h.t); if (v == 0u) { dist += min(h.t, 12.0); }
      let sm = smokeSeen(o, d, min(h.t, 80.0)); let fo = folWalk(o + d * lt, d, min(h.t, 120.0) - lt, true); L += thr * (sm.rgb * fo.T * h.thru + fo.L); thr *= h.thru * fo.T * sm.a;
      if (miss) { L += thr * skyLod(d, 1.0); break; }
      if (leaf) { L += thr * folBack(o + d * h.t, d); break; }
      let q = surfaceMean(o, d, h); let own = (pieces[h.piece].lamp & 255u) != 255u; if (!own) { L += thr * q.glow; } wide = footAt(h.t);
      if (wide >= @tune.fine * CELL || v == 3u) {
        L += thr * sentBack(q, false, rnd2(), d);
        let w0 = who * 2u + k; if (k < 2u && w0 < HITS) { let wants = cacheWants(q.x, q.ng, (q.flags & MOVES) != 0u); if (wants != 0u) { hits[w0] = vec4u(bitcast<vec3u>(q.x), octPack(q.ng)); for (var l = 0u; l < 4u; l++) { if ((wants & (1u << l)) != 0u) { cacheFound(cacheKeyOf(q.x, q.ng, l & 1u, l >> 1u), w0); } } } }
        break;
      }
      let a = returns(q); let y = q.x + q.ng * 0.004;
      for (var m = 0u; m < 2u; m++) {
        var e = vec3f(0.0); var dl = vec3f(0.0); var far = 400.0; if (m == 0u) { let c = dot(q.ng, frame.sunDir.xyz); if (c <= 0.0 || max3(frame.sunE.xyz) <= 0.0) { continue; } dl = coneDir(frame.sunDir.xyz, frame.sunDir.w, rnd2()); e = frame.sunE.xyz * c; }
        else { let l = drawLamp(y, q.ng); if (l.x < 0.0) { continue; } let g = aimLamp(u32(l.x), y, rnd2()); dl = g.dir; far = g.dist - 0.02; e = g.E * l.y * max(dot(q.ng, dl), 0.0); if (max3(e) <= 0.0) { continue; } }
        L += thr * a / PI * min(e, vec3f(300.0)) * visible(y, dl, 1e-3, far); }
      thr *= a; o = y; d = cosineDir(q.ng, rnd2());
    }
    let Ew = min(L * w.w, vec3f(50.0)); E += Ew; way += w.xyz * lum(Ew);
  }
  textureStore(outE, id.xy, vec4f(E / f32(N), dist / f32(N))); textureStore(outV, id.xy, vec4f(way / f32(N), 0.0));
}
