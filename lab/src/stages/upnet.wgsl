// What the network is told of a patch of four by four output pixels: how bright, how varied, how far history stands off, how it moves. Four of the sixteen are looked at.
fn tm(c: vec3f) -> vec3f { return c / (1.0 + lum(c) * taauK.x); }
fn ycc(c: vec3f) -> vec3f { return vec3f(0.25 * c.r + 0.5 * c.g + 0.25 * c.b, 0.5 * (c.r - c.b), -0.25 * c.r + 0.5 * c.g - 0.25 * c.b); }
fn e(x: f32) -> f32 { return log(1.0 + max(x, 0.0) * 8.0); }
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = textureDimensions(f0); if (id.x >= dim.x || id.y >= dim.y) { return; } let od = frame.size.zw; let rd = frame.size.xy; let r = rd / od; let hi = vec2i(rd) - 1; var a = vec4f(0.0); var b = vec4f(0.0); var c = vec4f(0.0);
  for (var k = 0u; k < 4u; k++) { let so = vec2f(id.xy * 4u + vec2u(1u + 2u * (k & 1u), 1u + 2u * (k >> 1u))) + 0.5; let s = so * r; let q = vec2i(floor(s - frame.jitter.xy)); var m1 = vec3f(0.0); var m2 = vec3f(0.0); var near = 1e9; var nq = clamp(q, vec2i(0), hi);
    for (var y = -1; y <= 1; y++) { for (var x = -1; x <= 1; x++) { let t = clamp(q + vec2i(x, y), vec2i(0), hi); let v = textureLoad(color, t, 0); let w = ycc(tm(v.rgb)); m1 += w; m2 += w * w; if (v.a < near) { near = v.a; nq = t; } } }
    let m = m1 / 9.0; let sd = sqrt(max(m2 / 9.0 - m * m, vec3f(1e-9))); let mv = textureLoad(auxT, nq, 0).xy; let uv = so / od + mv; let h = textureSampleLevel(hist, linSamp, uv, 0.0); let hy = ycc(tm(max(h.rgb, vec3f(0.0)))); let inside = select(0.0, 1.0, all(uv > vec2f(0.0)) && all(uv < vec2f(1.0)));
    a += vec4f(e(m.x), m.yz, e(sd.x)); b += vec4f(e(abs(hy.x - m.x)), clamp((hy.x - m.x) / (sd.x + 0.02 * abs(m.x) + 1e-3) * 0.2, -3.0, 3.0), hy.yz - m.yz); c += vec4f(e(hy.x), min(length(mv * od) * 0.25, 3.0), max(h.a, 0.0) / 28.0, inside); }
  textureStore(f0, id.xy, a * 0.25); textureStore(f1, id.xy, b * 0.25); textureStore(f2, id.xy, c * 0.25);
}
