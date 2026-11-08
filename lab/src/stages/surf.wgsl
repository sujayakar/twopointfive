// FIRST HIT. What each pixel shows, and the clear layer before it, if any. (Pieces are traced, not rasterized: exact for curves, and the same shapes light meets.)
fn moved(k: u32, x: vec3f) -> vec3f {                              // where this point of a moving piece was last frame
  if (k < frame.piece0.z) { return x; } let l = toLocal(k, x); let o = (k - frame.piece0.z) * 4u; return past[o].xyz + past[o + 1u].xyz * l.x + past[o + 2u].xyz * l.y + past[o + 3u].xyz * l.z;
}
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = vec2u(frame.size.xy); if (id.x >= dim.x || id.y >= dim.y) { return; }
  let rd = pxRay(id.xy); let ro = frame.camPos.xyz; let px = length(frame.camUp.xyz) * 2.0 / frame.size.w;
  var tmin = 1e-3; var thru = vec3f(1.0); var front = vec4u(0u); var wetLen = 0.0; var wetN = vec3f(0.0); var wetT = 0.0; var h: Hit;
  for (var k = 0u; k < 2u; k++) {
    h = nearest(ro, rd, tmin, 2000.0, 0u, frame.misc.y); thru *= h.thru;
    if (front.x == 0u && h.gpiece != NOTHING && h.gcode != CODE_CUT) { let lp = toLocal(h.gpiece, ro + rd * h.gt); var n = normalize(toWorld(h.gpiece, localNormal(h.gpiece, h.gcode, lp))); front = vec4u(bitcast<u32>(h.gt), octPack(n), packRGB(unpackTint(pieces[h.gpiece].tint)) | (1u << 24u), 0u); }
    if (h.piece != NOTHING && (h.piece & LEAF_HIT) == 0u && (pieces[h.piece].flags & WATER) != 0u && h.code == 3u) {
      let x = ro + rd * h.t; wetN = waveNormal(x.xz, frame.misc.x); wetT = h.t; if (front.x == 0u) { front = vec4u(bitcast<u32>(h.t), octPack(wetN), 2u << 24u, 0u); } tmin = h.t + 0.03; continue;
    }
    break;
  }
  if (h.piece == NOTHING) {
    let far = ro + rd * 1900.0; let q = projectPrev(far); let now = project(far);
    var s: Surf; s.n = vec3f(0.0, 1.0, 0.0); s.ng = s.n; textureStore(outSurf, id.xy, packPx(1900.0, s)); textureStore(outAux, id.xy, vec4f(q.xy - now.xy, 0.0, 0.0)); textureStore(outEmit, id.xy, vec4f(sky(rd) * thru, 0.0)); textureStore(outFront, id.xy, front); return;
  }
  var r0 = ro; if (wetT > 0.0) { wetLen = h.t - wetT; let b = refract(rd, wetN, 0.75); r0 = ro + (b - rd) * wetLen * 0.55; }     // seen through water: what is drawn there shifts; where it is does not
  var s = surfaceAt(r0, rd, h, px * h.t); s.x = ro + rd * h.t; if (dot(s.n, -rd) < 0.03) { s.n = normalize(s.n - rd * (0.05 - dot(s.n, -rd))); }
  let was = select(s.x, moved(h.piece, s.x), (h.piece & LEAF_HIT) == 0u); let q = projectPrev(was); let now = project(s.x);
  if (wetLen > 0.0) { front.w = bitcast<u32>(wetLen); }
  textureStore(outSurf, id.xy, packPx(h.t, s)); textureStore(outAux, id.xy, vec4f(q.xy - now.xy, wetLen, select(0.0, 1.0, (s.flags & MOVES) != 0u)));
  textureStore(outEmit, id.xy, vec4f(s.glow, 0.0)); textureStore(outFront, id.xy, front);
}
