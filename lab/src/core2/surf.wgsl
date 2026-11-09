/** what a ray that stopped in a cell finds there: one facet of its contents, drawn as they face the ray (an ellipsoid's facets), with their color from that side */
/** a limb a ray stopped in: its color, facing the ray. (What it is lit by is told by folBack, sentBack, folJ: there its surface adds nothing) */
fn limbSurf(h: Hit, x: vec3f, rd: vec3f) -> Surf {
  let k = h.piece & 0x7fffffffu; let r = limbRow(k); var s: Surf; let u = rnd2(); s.n = -cosineDir(rd, u); s.ng = s.n; s.x = x; s.albedo = select(vec3f(0.0), extra[r + 4u].xyz, BLK_DICE); s.through = vec3f(0.0); s.rough = 1.0; s.f0 = 0.0; s.metal = 0.0; s.glow = vec3f(0.0); s.kind = 0u; s.piece = h.piece; s.flags = MOVES; s.ao = vec2f(1.0); fromLimb = k + 1u; return s;
}
fn cellSurf(h: Hit, x: vec3f, rd: vec3f) -> Surf {
  let cell = h.piece & 0x7fffffffu; let r = bitcast<vec4u>(extra[BLK_CELLS + (cell - 1u) * 2u]); let r2 = bitcast<vec4u>(extra[BLK_CELLS + (cell - 1u) * 2u + 1u]); let B = cellBox(r, vec3f(0.0), 1.0); let sg = max(B.sigma, vec3f(1e-3) * max3(B.sigma)); var s: Surf;
  let q = normalize(sg * rd); let u = rnd2(); let k = sqrt(u.x); let m = basis(-q) * vec3f(k * cos(6.2831853 * u.y), k * sin(6.2831853 * u.y), sqrt(max(1.0 - u.x, 0.0))); s.n = normalize(sg * m); s.ng = s.n; s.x = x;
  let n2 = s.n * s.n; var c = vec3f(0.0); let words = array<u32, 3>(r.w, r2.x, r2.y);
  for (var i = 0u; i < 3u; i++) { let w = (words[i] >> select(0u, 16u, s.n[i] > 0.0)) & 0xffffu; let v = vec3f(f32(w & 31u) / 31.0, f32((w >> 5u) & 63u) / 63.0, f32(w >> 11u) / 31.0); c += v * v * n2[i]; }
  s.albedo = c; s.through = vec3f(0.0); s.rough = 1.0; s.f0 = 0.0; s.metal = 0.0; s.glow = vec3f(0.0); s.kind = 0u; s.piece = h.piece; s.flags = 0u; s.ao = vec2f(1.0); fromCell = cell;
  if (rnd() * 255.0 < f32(r.x >> 24u)) { s.through = leafThrough(c, ((r.y >> 24u) & 1u) != 0u); let keep = leafKeeps(c + s.through); s.albedo *= keep; s.through *= keep; s.kind = 14u; fromCell = NOTHING; }
  return s;
}
