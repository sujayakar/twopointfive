/** smoke by the same rule: in steps half as long as the ray is wide there, from the cells of 8 cm where it is narrow, from a whole brick's mean where it is wider than one. Light, and how much gets through */
fn smokeWalk(ro: vec3f, rd: vec3f, t1: f32, lit: bool) -> vec4f {
  var o = vec4f(0.0, 0.0, 0.0, 1.0); if (frame.misc2.x < 0.5) { return o; } let s = smokeSpan(ro, rd, t1); if (s.y <= s.x) { return o; } let inv = safeInv(rd); let ahead = select(vec3f(0.0), vec3f(1.0), rd > vec3f(0.0)); var t = s.x + 1e-3; var tau = 0.0;
  for (var i = 0u; i < 96u; i++) {
    if (t >= s.y || tau > 5.0) { break; } let x = ro + rd * t; let f = footAt(t);
    if (smokeSlot(x) == 0u) { let e = (SM_AT + (floor((x - SM_AT) / SM_BRICK) + ahead) * SM_BRICK - ro) * inv; t = max(max(min3(e), t) + 2e-3, t + 0.25 * f); continue; }
    let dt = min(max(0.12, 0.5 * f), s.y - t); let y = x + rd * (0.5 * dt); let p = smokeSlot(y);
    if (p != 0u) { var v: vec4f; if (f > SM_BRICK) { v = extra[BLK_SMOKE + p - 1u]; } else { v = smokeIn(p, y); if (max3(v.rgb) > 24.0) { v = vec4f(0.0, 0.0, 0.0, v.w); } } let d = v.w * dt; if (lit) { o = vec4f(o.rgb + v.rgb * (exp(-tau) * (1.0 - exp(-d))), 1.0); } tau += d; }      /* (a flame lights by its lamp: rays that are blurred see only its smoke) */
    t += dt;
  }
  o.w = exp(-tau); return o;
}
fn smokeSeen(ro: vec3f, rd: vec3f, t1: f32) -> vec4f { return smokeWalk(ro, rd, t1, true); }
fn smokeThrough(ro: vec3f, rd: vec3f, t1: f32) -> f32 { return smokeWalk(ro, rd, t1, false).w; }
