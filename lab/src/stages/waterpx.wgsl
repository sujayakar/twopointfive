// THROUGH THE WATER. Where the clear layer is the pool's surface, the first hit is found again along the bent ray. Only those pixels are written.
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = vec2u(frame.size.xy); if (id.x >= dim.x || id.y >= dim.y) { return; }
  let b = textureLoad(idC, id.xy, 0).x; if (b == 0u || (b & LEAF_HIT) != 0u || (pieces[b - 1u].flags & WATER) == 0u) { return; }
  let rd = pxRay(id.xy); let ro = frame.camPos.xyz; let zd = textureLoad(depthT, id.xy, 0); let t0 = select(4000.0, 0.25 / (zd * dot(rd, frame.camFwd.xyz)), zd > 0.0);
  if (!meet(b - 1u, ro, rd, 1e-3, t0, frame.misc.y) || hCode != 3u) { return; }
  let tw = hT; let x0 = ro + rd * tw; let n = poolNormal(x0.xz, frame.misc.x); let d = refract(rd, n, 0.75); fineLeaves = false; noLeaves = true;
  let h = nearest(x0 - vec3f(0.0, 0.004, 0.0), d, 1e-3, 60.0, WATER, frame.misc.y); if (h.piece == NOTHING || (h.piece & LEAF_HIT) != 0u) { return; }
  let px = length(frame.camUp.xyz) * 2.0 / frame.size.w; var s = surfaceAt(x0, d, h, px * (tw + h.t)); if (dot(s.n, -d) < 0.03) { s.n = normalize(s.n - d * (0.05 - dot(s.n, -d))); }
  let seen = ro + rd * (tw + h.t * 0.75); let q = projectPrev(seen); let now = project(seen);       // (it moves on the screen as what stands at its apparent depth does)
  textureStore(outSurf, id.xy, packPx(tw + h.t * 0.75, s)); textureStore(outAux, id.xy, vec4f(q.xy - now.xy, h.t, 0.0)); textureStore(outEmit, id.xy, vec4f(s.glow, 0.0)); textureStore(outFront, id.xy, vec4u(bitcast<u32>(tw), octPack(n), 2u << 24u, bitcast<u32>(h.t)));
}
