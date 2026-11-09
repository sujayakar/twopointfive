// HAZE AND DUST, read along each ray from the light kept in the air. Far off it is the city's haze, lit alike; near, small steps through the grid.
/** (a floor the section hides has no air to see) */
fn roomAir(x: vec3f) -> f32 { return select(1.0, 0.0, openAir(x) || x.y > frame.misc.y); }
fn hg(mu: f32, g: f32) -> f32 { return (1.0 - g * g) / (4.0 * PI * pow(1.0 + g * g - 2.0 * g * mu, 1.5)); }
fn rainAir(x: vec3f) -> f32 { return frame.weather.y * 0.0011 * (0.4 + 1.2 * smoothstep(0.35, 0.65, fbm((x.xz - vec2f(cos(frame.weather.w), sin(frame.weather.w)) * (frame.pool2.z * 1.4)) * 0.11 + 3.0))); }
fn farAir(y: f32) -> f32 { return frame.misc.w * 0.0015 * @tune.haze * exp(-max(y + 12.0, 0.0) * 0.045); }
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = textureDimensions(outA); if (id.x >= dim.x || id.y >= dim.y) { return; }
  let full = min(id.xy * 2u, vec2u(frame.size.xy) - 1u); let rd = pxRay(full); let t1 = min(bitcast<f32>(textureLoad(surf, full, 0).x), 260.0); let ro = frame.camPos.xyz; let j = blue2(id.xy, 9u).x; let mu = dot(rd, frame.sunDir.xyz); let amb = skyLod(vec3f(0.0, 1.0, 0.0), 6.0); let sunFar = frame.sunE.xyz * hg(mu, 0.55) + amb * 0.25; let sunNear = frame.sunE.xyz * hg(mu, 0.5);
  let inv = safeInv(rd); let a = (AL_LO - ro) * inv; let b = (AL_LO + vec3f(AL_N) * AL_H - ro) * inv; let t0 = clamp(max3(min(a, b)), 0.0, t1); let te = clamp(min3(max(a, b)), t0, t1); var T = 1.0; var L = vec3f(0.0);
  { let m = ro + rd * (t0 * 0.6); let e = exp(-(farAir(m.y) + rainAir(m)) * t0); L += (1.0 - e) * sunFar; T *= e; }
  if (te > t0 + 1e-3) { let n = clamp(ceil((te - t0) / 0.45), 4.0, 28.0); let dt = (te - t0) / n; let size = vec3f(AL_N) * AL_H;
    for (var i = 0.0; i < n; i += 1.0) { let x = ro + rd * (t0 + dt * (i + j)); let room = roomAir(x); let q = textureSampleLevel(light, linSamp, (x - AL_LO) / size, 0.0); let shown = select(1.0, 0.0, x.y > frame.misc.y && overCut(x, frame.misc.y)); let s = (select(farAir(x.y) + rainAir(x), farAir(x.y) + frame.misc.w * @tune.dust, room > 0.5)) * shown; let beam = select(@tune.beams, 0.0, room > 0.5) * frame.misc.w * shown; let e = exp(-s * dt); let out = select(1.0, 0.0, room > 0.5); L += T * ((1.0 - e) * (q.rgb + sunNear * q.a + amb * 0.25 * out) + beam * dt * q.rgb); T *= e; }
  }
  { let m = ro + rd * (te + (t1 - te) * 0.5); let e = exp(-(farAir(m.y) + rainAir(m)) * (t1 - te)); L += T * (1.0 - e) * sunFar; T *= e; }
  textureStore(outA, id.xy, vec4f(L, T));
}
