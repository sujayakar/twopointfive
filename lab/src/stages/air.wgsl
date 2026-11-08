// THE AIR, one walk from the eye: a thin haze and the dust of the rooms, lit by the sun where it gets in, and by the lamps' own light on surfaces nearby.
fn dustAt(x: vec3f) -> f32 { let inside = select(0.0, 1.0, x.x > -12.0 && x.x < 12.0 && x.z > -8.0 && x.z < 8.0 && x.y < 6.7 && (x.y < 3.3 || x.x < 4.0)); return frame.misc.w * (0.0016 * exp(-max(x.y + 12.0, 0.0) * 0.045) + 0.013 * inside); }
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = textureDimensions(outA); if (id.x >= dim.x || id.y >= dim.y) { return; }
  let full = min(id.xy * 2u, vec2u(frame.size.xy) - 1u); let rd = pxRay(full); let t1 = min(bitcast<f32>(textureLoad(surf, full, 0).x), 260.0); let ro = frame.camPos.xyz; fineLeaves = false;
  // most of the steps go where the building is
  let mid = max(0.0, (9.5 - ro.y) / min(rd.y, -1e-3)); let t0 = clamp(mid, 0.0, t1 * 0.999); let n = 5; let j = blue2(id.xy, 9u); var L = vec3f(0.0); var T = 1.0; let mu = dot(rd, frame.sunDir.xyz); let g = 0.55; let ph = (1.0 - g * g) / (4.0 * PI * pow(1.0 + g * g - 2.0 * g * mu, 1.5)); let amb = skyLod(vec3f(0.0, 1.0, 0.0), 6.0);
  { let m = dustAt(ro + rd * t0 * 0.6) * t0; let e = exp(-m); L += T * (1.0 - e) * (frame.sunE.xyz * ph + amb * 0.25); T *= e; }
  for (var i = 0; i < n; i++) {
    let dt = (t1 - t0) / f32(n); let t = t0 + dt * (f32(i) + fract(j.x + f32(i) * 0.618)); let x = ro + rd * t; let s = dustAt(x); if (s <= 1e-5) { continue; }
    var V = 0.0; if (max3(frame.sunE.xyz) > 0.0 && x.y < frame.misc.y + 30.0) { V = lum(visible(x, coneDir(frame.sunDir.xyz, frame.sunDir.w, vec2f(j.y, 0.5)), 1e-3, 120.0)); }
    let e = exp(-s * dt); L += T * (1.0 - e) * (frame.sunE.xyz * (V * ph) + amb * select(0.25, 0.02, s > 0.004)); T *= e;
  }
  textureStore(outA, id.xy, vec4f(L, T));
}
