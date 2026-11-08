// SIGNAL 3, NARROW INCOMING. One ray along the lobe of what shines: the clear layer where there is one, else the surface's own coat. The hit is lit by the
// light on surfaces, with the sun asked again so that its shadows stay sharp in the mirror.
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = textureDimensions(outM); if (id.x >= dim.x || id.y >= dim.y) { return; }
  let full = min(id.xy * u32(DIV), vec2u(frame.size.xy) - 1u); let rd = pxRay(full); let p = unpackPx(textureLoad(surf, full, 0), rd); let f = textureLoad(frontT, full, 0); var s = pxSurf(p); var on = !p.sky && p.kind != 99u && (p.rough < 0.43 || p.metal > 0.5) && !p.leaf; var off = 0.004 * (1.0 + p.t * 0.02);
  if (f.x != 0u) { let t = bitcast<f32>(f.x); s.x = frame.camPos.xyz + rd * t; s.n = octUnpack(f.y); s.ng = select(s.n, vec3f(0.0, 1.0, 0.0), (f.z >> 24u) == 2u); if (dot(s.n, rd) > 0.0) { s.n = -s.n; s.ng = -s.ng; } s.metal = 0.0; s.f0 = select(0.04, 0.02, (f.z >> 24u) == 2u); s.rough = select(0.015, 0.06, (f.z >> 24u) == 2u); on = true; off = 0.01; }
  if (!on) { textureStore(outM, id.xy, vec4f(0.0, 0.0, 0.0, -1.0)); return; }
  rngInit(id.xy, 29u); let V = -rd; let l = coatDir(s, V, blue2(id.xy, 5u)); fineLeaves = false;
  var d = l.dir; if (dot(d, s.ng) <= 0.005) { d = reflect(d, s.ng); }
  let o = s.x + s.ng * off; let h = nearest(o, d, 2e-3, 1500.0, 0u, 1e9); var L = vec3f(0.0);
  if (h.piece == NOTHING) { L = skyLod(d, s.rough * 5.0); }
  else {
    let q = surfaceAt(o, d, h, 0.004 + h.t * (0.002 + s.rough * s.rough)); L = q.glow; let cs = dot(q.n, frame.sunDir.xyz); var V1 = vec3f(0.0); let c = cacheRead(q.x, q.ng, rnd2());
    if (max3(frame.sunE.xyz) > 0.0 && (cs > 0.0 || any(q.through > vec3f(0.0)))) { V1 = visible(q.x + q.ng * 0.004 * sign(dot(q.ng, frame.sunDir.xyz)), coneDir(frame.sunDir.xyz, frame.sunDir.w, rnd2()), 2e-3, 400.0); }
    let m = matteOf(q) + select(vec3f(0.0), coatF0(q), q.metal > 0.5); L += m / PI * (c.E * select(0.0, 1.0, c.n > 0.0) + frame.sunE.xyz * max(cs, 0.0) * V1) + q.through / PI * frame.sunE.xyz * max(-cs, 0.0) * V1;
    if ((h.piece & LEAF_HIT) == 0u && (pieces[h.piece].flags & WATER) != 0u) { L = skyLod(reflect(d, vec3f(0.0, 1.0, 0.0)), 2.0) * 0.12 + vec3f(0.02, 0.09, 0.1) * lum(frame.sunE.xyz) * 0.2; }
  }
  L *= h.thru; let m0 = lum(L); L *= min(1.0, 40.0 / max(m0, 1e-4));
  textureStore(outM, id.xy, vec4f(L * l.w, select(h.t, 60.0, h.piece == NOTHING)));
}
