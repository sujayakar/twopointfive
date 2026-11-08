// half as big, softly: for the glow and for the eye's local sense of brightness
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let d = textureDimensions(outG); if (id.x >= d.x || id.y >= d.y) { return; } let ts = 1.0 / vec2f(textureDimensions(src)); let uv = (vec2f(id.xy) + 0.5) / vec2f(d); var a = vec4f(0.0); var ws = 0.0;
  for (var y = -1; y <= 1; y++) { for (var x = -1; x <= 1; x++) { let c = textureSampleLevel(src, linSamp, uv + vec2f(f32(x), f32(y)) * ts * 1.5, 0.0); let w = f32((2 - abs(x)) * (2 - abs(y))) / (1.0 + select(0.0, lum(c.rgb) * 0.15, FIRST == 1)); a += c * w; ws += w; } }
  textureStore(outG, id.xy, a / ws);
}
