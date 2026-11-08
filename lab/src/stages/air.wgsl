// THE AIR, as one medium: the city's haze, the dust of the rooms, the steam off the vents, the rain. Each adds to one density; what stands at a point is lit one
// way: the sun if it is seen from there, the lamps near it, the sky. Steps go where something is: along the ray, inside each plume, and near each bright lamp.
/** (a floor the section hides has no air to see) */
fn roomAir(x: vec3f) -> f32 { return select(0.0, 1.0, x.y < frame.misc.y && abs(x.x) < 12.0 && abs(x.z) < 8.0 && x.y < 6.7 && (x.y < 3.3 || x.x < 4.0)); }
fn vn3(p: vec3f) -> f32 { let i = floor(p); let f = p - i; let w = f * f * (3.0 - 2.0 * f); let z = f32(i.z) * 37.0; return mix(vnoise(p.xy + z), vnoise(p.xy + z + 37.0), w.z); }
fn steamAt(x: vec3f) -> f32 {
  var s = 0.0; if (frame.weather.z <= 0.0) { return 0.0; } let wind = vec2f(cos(frame.weather.w), sin(frame.weather.w));
  for (var i = 0u; i < u32(plumes[8].x); i++) { let p = plumes[i]; if (p.y > frame.misc.y) { continue; } let q = plumes[9u + i]; let y = (x.y - p.y) / q.x; if (y < 0.0 || y > 1.0) { continue; }
    let c = p.xz + wind * (y * y * q.x * 0.55) + vec2f(sin(y * 7.0 - frame.misc.x * 1.3 + f32(i)), cos(y * 5.0 - frame.misc.x * 1.1 + f32(i) * 2.0)) * (0.10 * y * q.x * 0.3); let r = p.w * (0.75 + 2.6 * y); let d = length(x.xz - c) / r; if (d > 1.0) { continue; }
    let n = vn3(vec3f(x.x * 2.2, (x.y - frame.misc.x * 1.25) * 1.6, x.z * 2.2)) * 0.65 + vn3(vec3f(x.x * 5.1, (x.y - frame.misc.x * 1.6) * 4.0, x.z * 5.1)) * 0.35;
    s += q.y * smoothstep(0.30, 0.72, n + 0.22 * (1.0 - d)) * (1.0 - d * d) * smoothstep(0.0, 0.07, y) * (1.0 - y) * 2.6; }
  return s * frame.weather.z;
}
/** scattering per metre of everything but steam */
fn airAt(x: vec3f) -> f32 { return frame.misc.w * (0.0015 * @tune.haze * exp(-max(x.y + 12.0, 0.0) * 0.045) + @tune.dust * roomAir(x)) + frame.weather.y * 0.0011 * (0.4 + 1.2 * smoothstep(0.35, 0.65, fbm((x.xz - vec2f(cos(frame.weather.w), sin(frame.weather.w)) * frame.misc.x * 4.0) * 0.11 + 3.0))) * (1.0 - roomAir(x)); }
fn hg(mu: f32, g: f32) -> f32 { return (1.0 - g * g) / (4.0 * PI * pow(1.0 + g * g - 2.0 * g * mu, 1.5)); }
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = textureDimensions(outA); if (id.x >= dim.x || id.y >= dim.y) { return; }
  let full = min(id.xy * 2u, vec2u(frame.size.xy) - 1u); let rd = pxRay(full); let t1 = min(bitcast<f32>(textureLoad(surf, full, 0).x), 260.0); let ro = frame.camPos.xyz; fineLeaves = false; rngInit(id.xy, 41u);
  let j = blue2(id.xy, 9u); let j2 = blue2(id.xy, 13u); let mu = dot(rd, frame.sunDir.xyz); let amb = skyLod(vec3f(0.0, 1.0, 0.0), 6.0); let tc = clamp(max(0.0, (10.5 - ro.y) / min(rd.y, -1e-3)), 0.0, t1 * 0.999);
  var seen = vec2f(1e-6 * t1, 1e-6); var L = vec3f(0.0); var tau = airAt(ro + rd * tc * 0.6) * tc; { let e = exp(-tau); L += (1.0 - e) * (frame.sunE.xyz * hg(mu, 0.55) + amb * 0.25); }
  // what the steps are: (place, scattering x length)
  var st: array<vec4f, 12>; var n = 0u;
  for (var i = 0u; i < 3u; i++) { let dt = (t1 - tc) / 3.0; let t = tc + dt * (f32(i) + fract(j.x + f32(i) * 0.618)); let s = airAt(ro + rd * t) * dt; if (s > 1e-6) { st[n] = vec4f(ro + rd * t, s); n++; } }
  for (var i = 0u; i < u32(plumes[8].x); i++) {                                // a plume: three steps of its own where the ray is near it
    let p = plumes[i]; let q = plumes[9u + i]; if (frame.weather.z <= 0.0 || n > 8u) { break; } if (p.y > frame.misc.y) { continue; } let c = p.xyz + vec3f(0.0, q.x * 0.5, 0.0); let R = q.x * 0.62 + 0.5; let b = dot(c - ro, rd); let d2 = dot(c - ro, c - ro) - b * b; if (d2 > R * R) { continue; } let h = sqrt(R * R - d2); let a = max(b - h, 0.0); let e = min(b + h, t1); if (e <= a) { continue; }
    for (var k = 0u; k < 3u; k++) { let dt = (e - a) / 3.0; let x = ro + rd * (a + dt * (f32(k) + fract(j2.x + f32(k) * 0.41))); let s = steamAt(x) * dt; if (s > 1e-4 && n < 12u) { st[n] = vec4f(x, s); n++; } }
  }
  for (var i = 0u; i < n; i++) {
    let x = st[i].xyz; var V = vec3f(0.0); let room = roomAir(x);
    if (max3(frame.sunE.xyz) > 0.0) { V = frame.sunE.xyz * lum(visible(x, coneDir(frame.sunDir.xyz, frame.sunDir.w, vec2f(j.y, 0.5)), 1e-3, 120.0)) * hg(mu, 0.5); }
    let c = cacheRead(vec3f(x.x, floor(x.y / 3.4) * 3.4 + select(0.02, 0.0, room < 0.5), x.z), vec3f(0.0, 1.0, 0.0), vec2f(0.5)); let near = select(amb * 0.25, c.E * (1.0 / (4.0 * PI)) * 0.5, room > 0.5 && c.n > 0.0);
    let e = exp(-st[i].w); let part = exp(-tau) * (1.0 - e); seen += vec2f(dot(x - ro, rd), 1.0) * part; L += part * (V + near); tau += st[i].w;
  }
  // beams: one lamp a pixel, drawn by how near the ray passes it, and one point drawn by its nearness along the ray
  { var W = 0.0; var pick = -1; var w0 = 0.0; var u = j2.y; for (var i = 0u; i < frame.counts.x; i++) { if (lamps[i].kind == 0u || lamps[i].on <= 0.0 || lamps[i].at.y > frame.misc.y) { continue; } let d = lamps[i].at - ro; let b = clamp(dot(d, rd), 0.0, t1); let D = max(length(d - rd * b), 0.15); if (D > lamps[i].reach * 0.7) { continue; } var w = lum(lamps[i].radiance) * lamps[i].area * lamps[i].area / D; if (lamps[i].kind == 2u) { let c = dot(normalize(ro + rd * b - lamps[i].at), lamps[i].n); w *= 0.05 + smoothstep(0.2, 0.95, c); } W += w; let q = w / W; if (u < q) { pick = i32(i); w0 = w; u = u / q; } else { u = (u - q) / (1.0 - q); } }
    if (pick >= 0) { let l = &lamps[u32(pick)]; let d = (*l).at - ro; let b = dot(d, rd); let D = max(length(d - rd * b), 0.12); let ta = max(0.0, b - 9.0); let tb = min(t1, b + 9.0);
      if (tb > ta) { let a0 = atan((ta - b) / D); let a1 = atan((tb - b) / D); let t = b + D * tan(mix(a0, a1, fract(j2.x + 0.37))); let pdf = D / ((a1 - a0) * (D * D + (t - b) * (t - b))); let x = ro + rd * t; let s = select(0.0, airAt(x) + @tune.beams + steamAt(x), x.y < frame.misc.y); let a = aimLamp(u32(pick), x, vec2f(0.5));
        if (max3(a.E) > 0.0) { let V = lum(visible(x, a.dir, 1e-3, a.dist - 0.1)); L += min(a.E * V * (s * hg(dot(rd, a.dir), 0.35) / pdf * W / w0), vec3f(6.0)); } } } }
  textureStore(outD, id.xy, vec4f(seen.x / seen.y, seen.y, 0.0, 0.0));       // (how far off what is seen of the air is: its history moves as that does, not as what stands behind it)
  textureStore(outA, id.xy, vec4f(L, exp(-tau)));
}
