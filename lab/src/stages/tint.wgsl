// The lamps' light as the hue of what comes through: how much there is stays exact, which of it is seen was asked lamp by lamp.
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = vec2u(frame.size.xy); if (id.x >= dim.x || id.y >= dim.y) { return; } let u = textureLoad(lampU, id.xy, 0); let l0 = lum(u.rgb); if (l0 <= 1e-5) { textureStore(outT, id.xy, u); return; }
  let p = unpackPx(textureLoad(surf, id.xy, 0), pxRay(id.xy)); let hd = vec2i(textureDimensions(tone)); let b = vec2f(id.xy) / 2.0 - 0.25; let q0 = vec2i(floor(b)); let f = b - floor(b); var a = vec3f(0.0); var ws = 0.0;
  for (var k = 0; k < 4; k++) { let q = clamp(q0 + vec2i(k & 1, k >> 1u), vec2i(0), hd - 1); let fq = min(q * 2, vec2i(frame.size.xy) - 1); let s = unpackPx(textureLoad(surf, fq, 0), pxRay(vec2u(fq))); if (s.sky) { continue; } let w = (select(1.0 - f.x, f.x, (k & 1) == 1) * select(1.0 - f.y, f.y, k > 1) + 0.01) * (pow(sat(dot(s.n, p.n)), 8.0) * exp(-abs(dot(s.x - p.x, p.ng)) / (0.003 * p.t + 0.01)) + 1e-4); a += textureLoad(tone, q, 0).rgb * w; ws += w; }
  a /= max(ws, 1e-6); let l1 = lum(a * u.rgb); var o = u; if (l1 > 1e-4 * l0) { o = vec4f(u.rgb * clamp(a * (l0 / l1), vec3f(0.0), vec3f(6.0)), u.a); } textureStore(outT, id.xy, o);
}
