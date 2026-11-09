// SIGNAL 2, WIDE INCOMING. Rays by the cosine and through openings; what they hit is read from the light on surfaces. Founds the cells it lands on.
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = textureDimensions(outE); if (id.x >= dim.x || id.y >= dim.y) { return; }
  let full = min(id.xy * 2u, vec2u(frame.size.xy) - 1u); let rd = pxRay(full); let p = underWater(unpackPx(textureLoad(surf, full, 0), rd), textureLoad(frontT, full, 0), rd); let who = id.y * dim.x + id.x;
  if (p.sky || p.kind == 63u) { textureStore(outE, id.xy, vec4f(0.0)); return; }
  if (p.leaf && folHas(p.x)) { textureStore(outE, id.xy, vec4f(0.0)); return; }          // (a leaf reads its light from the foliage, at every pixel: see resolve)
  rngInit(id.xy, 17u); leafMode = 2u; let x = p.x + p.ng * 0.004 * (1.0 + p.t * 0.02); var E = vec3f(0.0); var dist = 0.0; let N = u32(@tune.rays);
  for (var k = 0u; k < N; k++) {
    let side = p.leaf && rnd() < 0.45; let n = select(p.n, -p.n, side); let u = blue2(id.xy, 1u + k); let w = wideDir(select(x, p.x - p.ng * 0.02, side), n, u, rnd()); if (w.w <= 0.0) { continue; }
    if (dot(w.xyz, p.ng) * select(1.0, -1.0, side) <= 0.0) { continue; }
    let h = nearest(select(x, p.x - p.ng * 0.02, side) + w.xyz * select(0.0, 0.03, p.leaf), w.xyz, 2e-3, 600.0, 0u, 1e9); var L = vec3f(0.0);
    if (h.piece == NOTHING) { L = skyLod(w.xyz, 1.0); dist += 12.0; }
    else {
      let q = surfaceMean(x, w.xyz, h); let leaf = (h.piece & LEAF_HIT) != 0u; let own = !leaf && (pieces[h.piece].lamp & 255u) != 255u; L = select(q.glow, vec3f(0.0), own) + sentBack(q, leaf, rnd2(), w.xyz); dist += min(h.t, 12.0);
      let w0 = who * 2u + k; if (k < 2u && w0 < HITS) { let wn = select(q.ng, vec3f(0.0), leaf); let wants = cacheWants(q.x, wn, !leaf && (q.flags & MOVES) != 0u); if (wants != 0u) { hits[w0] = vec4u(bitcast<vec3u>(q.x), select(octPack(q.ng), EVERY_WAY, leaf)); for (var l = 0u; l < 4u; l++) { if ((wants & (1u << l)) != 0u) { cacheFound(cacheKeyOf(q.x, wn, l & 1u, l >> 1u), w0); } } } }
    }
    let so = select(x, p.x - p.ng * 0.02, side); let sm = smokeSeen(so, w.xyz, min(h.t, 80.0)); L = (L * sm.a + sm.rgb) * h.thru; let fo = folWalk(so, w.xyz, min(h.t, 120.0), true); L = L * fo.T + fo.L;
    E += min(L * w.w, vec3f(50.0));
  }
  textureStore(outE, id.xy, vec4f(E / f32(N), dist / f32(N)));
}
