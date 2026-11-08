// SIGNAL 2, WIDE INCOMING. On a leaf it is asked of every direction, whichever way the leaf faces: its mean (outE), and which way most of it comes from (outD),
// so that what is smooth over a crown can be shared by all its leaves and each still be lit by its own facing.
//   Elsewhere: rays by the cosine and through openings; what they hit is read from the light on surfaces. Founds the cells it lands on.
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = textureDimensions(outE); if (id.x >= dim.x || id.y >= dim.y) { return; }
  let full = min(id.xy * 2u, vec2u(frame.size.xy) - 1u); let rd = pxRay(full); let p = underWater(unpackPx(textureLoad(surf, full, 0), rd), textureLoad(frontT, full, 0), rd); let who = id.y * dim.x + id.x;
  if (p.sky || p.kind == 63u) { textureStore(outE, id.xy, vec4f(0.0)); textureStore(outD, id.xy, vec4f(0.0)); return; }
  rngInit(id.xy, 17u); fineLeaves = false; let x = p.x + p.ng * 0.004 * (1.0 + p.t * 0.02); var E = vec3f(0.0); var D = vec3f(0.0); var dist = 0.0; let N = u32(@tune.rays);
  for (var k = 0u; k < N; k++) {
    let u = blue2(id.xy, 1u + k); var w = vec4f(0.0); var at0 = x;
    if (p.leaf) { let z = 1.0 - 2.0 * u.x; let r = sqrt(max(1.0 - z * z, 0.0)); w = vec4f(r * cos(6.2832 * u.y), z, r * sin(6.2832 * u.y), 1.0); at0 = p.x + p.ng * (0.02 * sign(dot(w.xyz, p.ng))); at0 += w.xyz * 0.03; }
    else { w = wideDir(x, p.n, u, rnd()); if (w.w <= 0.0 || dot(w.xyz, p.ng) <= 0.0) { continue; } }
    let h = nearest(at0, w.xyz, 2e-3, 600.0, 0u, 1e9); var L = vec3f(0.0);
    if (h.piece == NOTHING) { L = skyLod(w.xyz, 3.0); dist += 12.0; }
    else {
      let q = surfaceMean(x, w.xyz, h); let leaf = (h.piece & LEAF_HIT) != 0u; let own = !leaf && (pieces[h.piece].lamp & 255u) != 255u; L = select(q.glow, vec3f(0.0), own) + sentBack(q, leaf, rnd2(), w.xyz); dist += min(h.t, 12.0);
      let w0 = who * 2u + k; if (k < 2u && w0 < HITS) { let wn = select(q.ng, vec3f(0.0), leaf); let wants = cacheWants(q.x, wn, !leaf && (q.flags & MOVES) != 0u); if (wants != 0u) { hits[w0] = vec4u(bitcast<vec3u>(q.x), select(octPack(q.ng), EVERY_WAY, leaf)); for (var l = 0u; l < 2u; l++) { if ((wants & (1u << l)) != 0u) { cacheFound(cacheKey(q.x, wn, l), w0); } } } }
    }
    let sm = smokeSeen(at0, w.xyz, min(h.t, 80.0)); L = L * sm.a + sm.rgb;
    let got = min(L * h.thru * w.w, vec3f(50.0)); E += got; if (p.leaf) { D += w.xyz * lum(got); }
  }
  textureStore(outE, id.xy, vec4f(E / f32(N), dist / f32(N)));
  textureStore(outD, id.xy, vec4f(D / f32(N), select(0.0, 1.0, p.leaf)));
}
