// SIGNAL 2, WIDE INCOMING. Rays by the cosine and through openings; what they hit is read from the light on surfaces. Founds the cells it lands on.
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = textureDimensions(outE); if (id.x >= dim.x || id.y >= dim.y) { return; }
  let full = min(id.xy * 2u, vec2u(frame.size.xy) - 1u); let rd = pxRay(full); let p = unpackPx(textureLoad(surf, full, 0), rd); let who = id.y * dim.x + id.x;
  if (p.sky || p.kind == 99u) { textureStore(outE, id.xy, vec4f(0.0)); return; }
  rngInit(id.xy, 17u); fineLeaves = false; let x = p.x + p.ng * 0.004 * (1.0 + p.t * 0.02); var E = vec3f(0.0); var dist = 0.0; let N = 2u;
  for (var k = 0u; k < N; k++) {
    let side = p.leaf && rnd() < 0.45; let n = select(p.n, -p.n, side); let u = blue2(id.xy, 1u + k); let w = wideDir(select(x, p.x - p.ng * 0.02, side), n, u, rnd()); if (w.w <= 0.0) { continue; }
    if (dot(w.xyz, p.ng) * select(1.0, -1.0, side) <= 0.0) { continue; }
    let h = nearest(select(x, p.x - p.ng * 0.02, side) + w.xyz * select(0.0, 0.03, p.leaf), w.xyz, 2e-3, 600.0, 0u, 1e9); var L = vec3f(0.0);
    if (h.piece == NOTHING) { L = skyLod(w.xyz, 3.0); dist += 12.0; }
    else {
      let q = surfaceMean(x, w.xyz, h); let c = cacheRead(q.x, q.ng, rnd2()); let own = (h.piece & LEAF_HIT) == 0u && (pieces[h.piece].lamp & 255u) != 255u; L = select(q.glow, vec3f(0.0), own); dist += min(h.t, 12.0);
      if (c.n > 0.0) { L += matteOf(q) / PI * (c.E + frame.sunE.xyz * max(dot(q.ng, frame.sunDir.xyz), 0.0) * c.sun); }
      if ((h.piece & LEAF_HIT) == 0u) { let w0 = who * 2u + k; if (w0 < HITS) { let wants = cacheWants(q.x, q.ng, (q.flags & MOVES) != 0u); if (wants != 0u) { hits[w0] = vec4f(q.x, bitcast<f32>(octPack(q.ng))); for (var l = 0u; l < 2u; l++) { if ((wants & (1u << l)) != 0u) { cacheFound(cacheKey(q.x, q.ng, l), w0); } } } } } else if (c.n <= 0.0) { L += (q.albedo + q.through) / PI * (frame.sunE.xyz * 0.22 + skyLod(vec3f(0.0, 1.0, 0.0), 6.0) * 1.2); }
    }
    E += min(L * h.thru * w.w, vec3f(50.0)) * select(1.0, 0.8, p.leaf);
  }
  textureStore(outE, id.xy, vec4f(E / f32(N), dist / f32(N)));
}
