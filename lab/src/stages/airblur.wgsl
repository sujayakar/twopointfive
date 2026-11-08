// The air, smoothed over its neighbours at the same depth: it has no edges of its own finer than this, and while the camera moves its history is short.
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = vec2i(textureDimensions(outB)); let p0 = vec2i(id.xy); if (any(p0 >= dim)) { return; } let rdim = vec2i(frame.size.xy); let t0 = bitcast<f32>(textureLoad(surf, min(p0 * 2, rdim - 1), 0).x); let c = textureLoad(src, p0, 0); var a = c; var ws = 1.0;
  for (var y = -2; y <= 2; y++) { for (var x = -2; x <= 2; x++) { if (x == 0 && y == 0) { continue; } let q = p0 + vec2i(x, y) * STRIDE; if (any(q < vec2i(0)) || any(q >= dim)) { continue; } let t = bitcast<f32>(textureLoad(surf, min(q * 2, rdim - 1), 0).x); let v = textureLoad(src, q, 0); let w = exp(-f32(x * x + y * y) * 0.28) * exp(-abs(t - t0) / (0.05 * min(t0, t) + 0.15)) * exp(-abs(v.a - c.a) * 14.0); a += v * w; ws += w; } }
  textureStore(outB, id.xy, a / ws);
}
