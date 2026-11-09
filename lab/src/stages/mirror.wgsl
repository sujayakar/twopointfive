// SIGNAL 3, NARROW INCOMING. One ray along the lobe of what shines, asked once a layer: the surface's own coat (LAYER 0), and the pane or the water over it (1). The hit is lit by the
// light on surfaces, with the sun asked again so that its shadows stay sharp in the mirror.
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = textureDimensions(outM); if (id.x >= dim.x || id.y >= dim.y) { return; }
  let full = min(id.xy * u32(DIV), vec2u(frame.size.xy) - 1u); let rd = pxRay(full); let p = unpackPx(textureLoad(surf, full, 0), rd); let f = textureLoad(frontT, full, 0); var s = pxSurf(p); var on = !p.sky && p.kind != 63u && (p.rough < asksMirror() || p.metal > 0.5) && !p.leaf && (f.z >> 24u) != 2u; var off = 0.004 * (1.0 + p.t * 0.02);
  if (LAYER == 1) { on = false; }
  if (LAYER == 1 && f.x != 0u) { let t = bitcast<f32>(f.x); s.x = frame.camPos.xyz + rd * t; s.n = octUnpack(f.y); s.ng = select(s.n, vec3f(0.0, 1.0, 0.0), (f.z >> 24u) == 2u); if (dot(s.n, rd) > 0.0) { s.n = -s.n; s.ng = -s.ng; } s.metal = 0.0; s.f0 = select(0.04, 0.02, (f.z >> 24u) == 2u); s.rough = select(0.015, 0.06, (f.z >> 24u) == 2u); on = true; off = 0.01; }
  if (!on) { textureStore(outM, id.xy, vec4f(0.0, 0.0, 0.0, -1.0)); return; }
  rngInit(id.xy, 29u + u32(LAYER) * 13u); let V = -rd; let l = coatDir(s, V, blue2(id.xy, 5u)); leafMode = 2u;
  var d = l.dir; if (dot(d, s.ng) <= 0.005) { d = reflect(d, s.ng); }
  let o = s.x + s.ng * off; let h = nearest(o, d, 2e-3, 1500.0, 0u, 1e9); var L = vec3f(0.0);
  if (h.piece == NOTHING) { L = skyLod(d, s.rough * 5.0); }
  else {
    let q = wetFilm(surfaceAt(o, d, h, 0.004 + h.t * (0.002 + s.rough * s.rough)), 0.2); L = q.glow; if ((h.piece & LEAF_HIT) == 0u && (pieces[h.piece].lamp & 255u) != 255u) { L *= 1.0 - byFormula(s.rough); } let cs = dot(q.n, frame.sunDir.xyz); var V1 = vec3f(0.0); let leaf = (h.piece & LEAF_HIT) != 0u; let c = cacheSmooth(q.x, select(q.ng, vec3f(0.0), leaf));
    if (max3(frame.sunE.xyz) > 0.0 && (cs > 0.0 || any(q.through > vec3f(0.0)))) { V1 = visible(q.x + q.ng * 0.004 * sign(dot(q.ng, frame.sunDir.xyz)), coneDir(frame.sunDir.xyz, frame.sunDir.w, rnd2()), 2e-3, 400.0); }
    let m = returns(q); L += select(m, q.albedo + q.through, leaf) / PI * c.E * select(0.0, 1.0, c.n > 0.0) + select(m / PI * max(cs, 0.0), leafPhase(q, d), leaf) * frame.sunE.xyz * V1;
    if ((h.piece & LEAF_HIT) == 0u && (pieces[h.piece].flags & WATER) != 0u) { L = skyLod(reflect(d, vec3f(0.0, 1.0, 0.0)), 2.0) * 0.12 + vec3f(0.02, 0.09, 0.1) * lum(frame.sunE.xyz) * 0.2; }
  }
  let sm = smokeSeen(o, d, min(h.t, 80.0)); L = L * sm.a + sm.rgb;
  L *= h.thru; { let fo = folWalk(o, d, min(h.t, 150.0), true); L = L * fo.T + fo.L; } let m0 = lum(L); L *= min(1.0, 40.0 / max(m0, 1e-4));
  textureStore(outM, id.xy, vec4f(L * l.w, select(h.t, 60.0, h.piece == NOTHING)));
}
