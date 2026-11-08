// HISTORY, for any signal: last frame's answer for this surface, checked, held to what this frame says nearby, and mixed by how many stand behind it.
// const DIV = the signal's grid over the render grid; CAP = frames; SPREAD = how far (sigmas) history may stand off; NARROW = 1 for a mirror's image
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = textureDimensions(outH); if (id.x >= dim.x || id.y >= dim.y) { return; }
  let rdim = vec2f(frame.size.xy); let full = min(id.xy * u32(DIV), vec2u(rdim) - 1u); let rd = pxRay(full); let p = unpackPx(textureLoad(surf, full, 0), rd); let c = textureLoad(cur, id.xy, 0);
  if (p.sky && NARROW == 0) { textureStore(outH, id.xy, c); textureStore(outM, id.xy, vec4f(0.0)); return; }
  if (NARROW == 1 && c.a < 0.0) { textureStore(outH, id.xy, c); textureStore(outM, id.xy, vec4f(0.0)); return; }
  var m1 = vec4f(0.0); var m2 = vec4f(0.0); var wn = 0.0; let R = 2;
  for (var y = -R; y <= R; y++) { for (var x = -R; x <= R; x++) { let q = clamp(vec2i(id.xy) + vec2i(x, y), vec2i(0), vec2i(dim) - 1); let v = textureLoad(cur, q, 0); if (NARROW == 1 && v.a < 0.0) { continue; } m1 += v; m2 += v * v; wn += 1.0; } }
  m1 /= wn; let sd = sqrt(max(m2 / wn - m1 * m1, vec4f(0.0)));
  let aux = textureLoad(auxT, full, 0); var uv = (vec2f(full) + 0.5) / rdim + aux.xy; var x0 = p.x; let fr = textureLoad(frontT, full, 0); var checkT = p.t; var front = false;
  if (NARROW == 1) {                                               // a mirror's image moves as if it stood behind the surface
    var t = p.t; var rough = p.rough; if (fr.x != 0u) { t = bitcast<f32>(fr.x); rough = 0.02; front = true; } let img = frame.camPos.xyz + rd * (t + min(c.a, 60.0) * (1.0 - smoothstep(0.15, 0.45, rough))); let q = projectPrev(img); let now = project(img); uv = (vec2f(full) + 0.5) / rdim + (q.xy - now.xy);
  }
  let hp = uv * rdim / f32(DIV) - 0.5 + select(0.0, 0.25, DIV == 2); let b = floor(hp); let f = hp - b; var h = vec4f(0.0); var mo = vec4f(0.0); var ws = 0.0;
  for (var k = 0u; k < 4u; k++) {
    let q = vec2i(b) + vec2i(i32(k & 1u), i32(k >> 1u)); if (any(q < vec2i(0)) || any(q >= vec2i(dim))) { continue; }
    var w = select(1.0 - f.x, f.x, (k & 1u) == 1u) * select(1.0 - f.y, f.y, k > 1u); if (w < 1e-3) { continue; }
    let fq = min(q * DIV, vec2i(rdim) - 1); let o = unpackPx(textureLoad(surfPrev, fq, 0), vec3f(0.0)); let was = frame.prevPos.xyz + pxRayPrev((vec2f(fq) + 0.5 + frame.jitter.zw) / rdim) * o.t;
    var ok = abs(dot(was - p.x, p.ng)) < 0.006 * p.t + 0.02 && dot(o.ng, p.ng) > 0.7; if (aux.w > 0.5) { ok = abs(o.t - p.t) < 0.25 && dot(o.ng, p.ng) > 0.4; } if (p.leaf) { ok = o.leaf && abs(o.t - p.t) < 0.5; } if (NARROW == 1 && front) { ok = true; }
    if (!ok) { continue; } let v = textureLoad(hist, q, 0); let pm = textureLoad(momPrev, q, 0); if (bad4(v) || bad4(pm)) { continue; } if (NARROW == 1 && v.a < 0.0) { continue; } h += v * w; mo += pm * w; ws += w;
  }
  var n = 0.0; var o = c; var mom = vec4f(lum(c.rgb), lum(c.rgb) * lum(c.rgb), 0.0, 0.0); if (NARROW == 0) { mom.w = c.a * c.a; }
  if (ws > 0.02) {
    h /= ws; mo /= ws; let st = steady(p.x); let cap = mix(3.0, f32(CAP), st) * select(1.0, 0.5, aux.w > 0.5 || p.leaf); n = min(mo.z, cap - 1.0);
    var se = sd; if (KIND == 1) { let q = (clamp(m1, vec4f(0.0), vec4f(1.0)) * wn + 1.0) / (wn + 2.0); let doubt = sqrt(q * (1.0 - q)); se = vec4f(max(sd.xy, doubt.xy), sd.z, sd.w + 0.04); }   // (the sun's share is a fraction already: what moves in it should be let go) let e = se * SPREAD + abs(m1) * 0.04 + 1e-4; let hc = clamp(h, m1 - e, m1 + e); let moved = select(length((hc - h).rgb) / (length(e.rgb) + 1e-4), length((hc - h).xyw) / (length(e.xyw) + 1e-4), KIND == 1); n *= 1.0 / (1.0 + moved * 0.35);
    var k = 1.0 / (n + 1.0); o = mix(hc, c, k); mom = vec4f(mix(mo.x, mom.x, max(k, 0.06)), mix(mo.y, mom.y, max(k, 0.06)), 0.0, mix(mo.w, mom.w, max(k, 0.06)));
    if (NARROW == 1) { o.a = mix(h.a, c.a, max(k, 0.2)); }
  }
  mom.z = n + 1.0;
  textureStore(outH, id.xy, o); textureStore(outM, id.xy, mom);
}
