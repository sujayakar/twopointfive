// FIRST HIT. What each pixel shows, and the clear layer before it, if any. The raster found which piece; the piece itself says where, so curves are exact.
fn moved(k: u32, x: vec3f) -> vec3f {                              // where this point of a moving piece was last frame
  if (k < frame.piece0.z) { return x; } let l = toLocal(k, x); let o = (k - frame.piece0.z) * 4u; return past[o].xyz + past[o + 1u].xyz * l.x + past[o + 2u].xyz * l.y + past[o + 3u].xyz * l.z;
}
fn movedLeaf(i: u32, x: vec3f) -> vec3f {
  let I = &insts[i]; let l = vec3f(dot((*I).r0.xyz, x) + (*I).r0.w, dot((*I).r1.xyz, x) + (*I).r1.w, dot((*I).r2.xyz, x) + (*I).r2.w); let a = pastLeaves[i * 3u]; let b = pastLeaves[i * 3u + 1u]; let c = pastLeaves[i * 3u + 2u];
  let d = l - vec3f(a.w, b.w, c.w); return (a.xyz * d.x + b.xyz * d.y + c.xyz * d.z) / max(dot(a.xyz, a.xyz), 1e-12);
}
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = vec2u(frame.size.xy); if (id.x >= dim.x || id.y >= dim.y) { return; }
  let rd = pxRay(id.xy); let ro = frame.camPos.xyz; let px = length(frame.camUp.xyz) * 2.0 / frame.size.w;
  var thru = vec3f(1.0); var front = vec4u(0u); var wetLen = 0.0; var wetN = vec3f(0.0); var wetT = 0.0; var h: Hit; h.piece = NOTHING; h.t = 1900.0; h.thru = vec3f(1.0);
  let a = textureLoad(idT, id.xy, 0).x; let b = textureLoad(idC, id.xy, 0).x;
  // the raster says which; where exactly is asked of the piece itself
  if ((a & LEAF_HIT) != 0u) {
    let inst = (a >> 13u) & 0x3ffffu; let I = &insts[inst]; let leaf = (*I).leaf + (a & 8191u); let o = vec3f(dot((*I).r0.xyz, ro) + (*I).r0.w, dot((*I).r1.xyz, ro) + (*I).r1.w, dot((*I).r2.xyz, ro) + (*I).r2.w); let d = vec3f(dot((*I).r0.xyz, rd), dot((*I).r1.xyz, rd), dot((*I).r2.xyz, rd));
    let n = octUnpack(extra[leaf * 2u + 1u].x); let den = dot(n, d); h.piece = LEAF_HIT | inst; h.code = leaf; h.t = dot(exF(leaf * 2u).xyz - o, n) / select(den, 1e-4, abs(den) < 1e-4); let zd = textureLoad(depthT, id.xy, 0); let tz = 0.25 / (zd * dot(rd, frame.camFwd.xyz)); if (abs(h.t - tz) > 0.2) { h.t = tz; }
  } else if (a != 0u) {
    h.piece = a - 1u; if (meet(h.piece, ro, rd, 1e-3, 4000.0, frame.misc.y)) { h.t = hT; h.code = hCode; } else { let zd = textureLoad(depthT, id.xy, 0); h.t = 0.25 / (zd * dot(rd, frame.camFwd.xyz)); let lp = toLocal(h.piece, ro + rd * h.t) / pieces[h.piece].half; let m = abs(lp); h.code = select(select(select(4u, 5u, lp.z > 0.0), select(2u, 3u, lp.y > 0.0), m.y > m.z), select(0u, 1u, lp.x > 0.0), m.x > m.y && m.x > m.z); if ((pieces[h.piece].flags & QUAD) != 0u) { h.code = CODE_QUAD; } }
  }
  if (b != 0u && (b & LEAF_HIT) == 0u) {
    let k = b - 1u; if (meet(k, ro, rd, 1e-3, h.t, frame.misc.y)) { let t = hT; let x = ro + rd * t;
      if ((pieces[k].flags & WATER) != 0u) { if (hCode == 3u) { wetN = poolNormal(x.xz, frame.misc.x); wetT = t; front = vec4u(bitcast<u32>(t), octPack(wetN), 2u << 24u, 0u); } }
      else if (hCode != CODE_CUT) { let n = normalize(toWorld(k, localNormal(k, hCode, toLocal(k, x)))); front = vec4u(bitcast<u32>(t), octPack(n), packRGB(unpackTint(pieces[k].tint)) | (1u << 24u), 0u); } } }
  if (h.piece == NOTHING) {
    let far = ro + rd * 1900.0; let q = projectPrev(far); let now = project(far);
    var s: Surf; s.n = vec3f(0.0, 1.0, 0.0); s.ng = s.n; textureStore(outSurf, id.xy, packPx(1900.0, s)); textureStore(outAux, id.xy, vec4f(q.xy - now.xy, 0.0, 0.0)); textureStore(outEmit, id.xy, vec4f(sky(rd) * thru, 0.0)); textureStore(outFront, id.xy, front); return;
  }
  if (wetT > 0.0) { wetLen = h.t - wetT; }
  var s = surfaceAt(ro, rd, h, px * h.t); s = wetFilm(s, px * h.t); s.x = ro + rd * h.t; if (dot(s.n, -rd) < 0.03) { s.n = normalize(s.n - rd * (0.05 - dot(s.n, -rd))); }
  var was = s.x; if ((h.piece & LEAF_HIT) == 0u) { was = moved(h.piece, s.x); } else { was = movedLeaf(h.piece & 0x7fffffffu, s.x); } let q = projectPrev(was); let now = project(s.x);
  if (wetLen > 0.0) { front.w = bitcast<u32>(wetLen); }
  textureStore(outSurf, id.xy, packPx(h.t, s)); textureStore(outAux, id.xy, vec4f(q.xy - now.xy, wetLen, select(0.0, 1.0, (s.flags & MOVES) != 0u)));
  textureStore(outEmit, id.xy, vec4f(s.glow, 0.0)); textureStore(outFront, id.xy, front);
}
