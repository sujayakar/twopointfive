// THE SEAM between what is drawn and what rays meet. Every pixel's first hit is on the finest mesh; from there a ray to the sun and a wide one are asked of coarser levels, in several ways at
// once (configs), with the same dice for all. The first config meets the finest, so the rest can be set against it.
struct SeamK { n: u32, frame: u32, px: f32, pad: u32, c: array<vec4f, 48> };        // config: levels coarser than a pixel's, how it starts (1 off along the normal, 2 not meeting backs, 4 asks the depth first, 8 by its footprint), how far off (errors), salt
/** the coarser levels' error here: what the pixel's own level of this mesh is off, and the one so many coarser */
fn errorAt(h: Hit, more: u32, px: f32) -> f32 { let p = &pieces[h.piece]; let head = mrowU((*p).planes); let c = head.z + ((h.code & 0x7fffffffu) >> head.y) * 4u; let c3 = mrowU(c + 3u); let n = bitcast<vec4u>(mrow(c3.x)).w; let k = (*p).quad.x * bitcast<f32>(c3.z); meshMore = more; let l = meshPick(c3.x, n, px / k); meshMore = 0u; return mrow(c3.x + 4u + l).x * k; }
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = vec2u(frame.size.zw); if (id.x >= dim.x || id.y >= dim.y) { return; } let at = id.y * dim.x + id.x; let rd = camRay((vec2f(id.xy) + 0.5) / vec2f(dim)); let ro = frame.camPos.xyz; leafMode = 1u;
  if (seamK.frame == 0u) {
    let h = nearest(ro, rd, 1e-3, 2000.0, 0u, 1e9); var s: Surf; if (h.piece != NOTHING) { s = surfaceAt(ro, rd, h, 2.0 * length(frame.camUp.xyz) / frame.size.w * h.t); }
    first[at * 3u] = vec4u(bitcast<vec3u>(s.x), select(0u, h.piece + 1u, h.piece != NOTHING && (pieces[h.piece].flags & MESH) != 0u)); first[at * 3u + 1u] = vec4u(octPack(s.n), octPack(s.ng), h.code, bitcast<u32>(select(h.t, 1e6, h.piece == NOTHING))); first[at * 3u + 2u] = bitcast<vec4u>(vec4f(s.albedo, 0.0)); return;
  }
  let a = first[at * 3u]; if (a.w == 0u) { return; } let b = first[at * 3u + 1u]; let x = bitcast<vec3f>(a.xyz); let n = octUnpack(b.x); let ng = octUnpack(b.y); var h: Hit; h.piece = a.w - 1u; h.code = b.z; let px = seamK.px * bitcast<f32>(b.w);
  meshOwn = false;
  for (var c = 0u; c < seamK.n; c++) {
    let k = seamK.c[c]; let more = u32(k.x); let how = u32(k.y); let finest = k.x < 0.0; rngInit(id.xy, 4001u + u32(k.w)); let e = select(errorAt(h, more, px), 0.0, finest); let off = select(0.0, e * k.z, (how & 1u) != 0u) + 0.0002 + px * 0.05; let back = select(0.0, 3.0 * e, (how & 2u) != 0u);
    let u = rnd2(); let u2 = rnd2(); let u3 = rnd2(); var sun = 0.0; var E = 0.0; let wide = cosineDir(n, u2); var hitAt = vec3f(0.0); var hitN = vec3f(0.0); var go = false; var alb = vec3f(0.0);
    meshMore = select(more, 0u, finest); meshError = select(px, 0.0, finest); if ((how & 8u) != 0u) { meshMore = 0u; meshFoot = 1.0; }
    // a wide ray, and the sun where it lands; the sun here
    if (dot(wide, ng) > 0.0) {
      meshBack = back; foot(px, 0.5); let o = x + ng * off; let near = (how & 4u) != 0u && closeBy(x + ng * (0.0002 + px * 0.05), wide, off + e, px); let q = nearest(o, wide, 1e-4, 600.0, 0u, 1e9); meshBack = 0.0;
      if (q.piece == NOTHING && !near) { E = lum(skyLod(wide, 1.0)); } else { let s = surfaceMean(o, wide, q); if (near || q.piece == NOTHING) { hitAt = x; hitN = -wide; alb = bitcast<vec4f>(first[at * 3u + 2u]).rgb; } else { hitAt = s.x; hitN = s.ng; alb = returns(s); } go = true; E = lum(alb) * 0.25 * lum(skyLod(hitN, 5.0)); } }
    for (var j = 0u; j < 2u; j++) {
      if (j == 1u && !go) { continue; } let g = select(ng, hitN, j == 1u); let d = coneDir(frame.sunDir.xyz, frame.sunDir.w, select(u, u3, j == 1u)); let cs = dot(g, d); if (cs <= 0.0) { continue; }
      var y = x + ng * off; if (j == 1u) { y = hitAt + hitN * (0.002 + e); } meshBack = back; foot(px, 2.0 * frame.sunDir.w); var T = 1.0; if (j == 0u && (how & 4u) != 0u && closeBy(x + ng * (0.0002 + px * 0.05), d, off + e, px)) { T = 0.0; } T *= lum(visible(y, d, 1e-4, 400.0)); meshBack = 0.0;
      if (j == 0u) { sun = T; } else { E += lum(alb) / PI * lum(frame.sunE.xyz) * cs * T; }
    }
    meshFoot = 0.0; meshError = 0.0; meshMore = 0u; let o = (c * dim.x * dim.y + at); sums[o] = select(sums[o], vec4f(0.0), seamK.frame == 1u) + vec4f(sun, E, select(0.0, 1.0, dot(ng, frame.sunDir.xyz) > 0.02), e);
  }
}
/** is something of the picture in the way, within `far` of x? A few looks at the depth there */
fn closeBy(x: vec3f, d: vec3f, far: f32, px: f32) -> bool {
  let steps = 8u; let dim = vec2f(frame.size.zw); for (var i = 1u; i <= steps; i++) { let y = x + d * (far * f32(i) / f32(steps)); let p = project(y); if (p.z <= 0.0 || any(p.xy <= vec2f(0.0)) || any(p.xy >= vec2f(1.0))) { return false; } let q = vec2u(p.xy * dim); let t = bitcast<f32>(first[(q.y * u32(dim.x) + q.x) * 3u + 1u].w); let mine = distance(y, frame.camPos.xyz); if (t < mine - px * 0.75 && t > mine - 2.0 * far - px * 2.0) { return true; } }
  return false;
}
