// THE AIR, as one medium: the city's haze, the dust of the rooms, the steam off the vents, the rain. Each adds to one density; what stands at a point is lit one
// way: the sun if it is seen from there, the lamps near it, the sky. Steps go where something is: along the ray, inside each plume, and near each bright lamp.
/** (a floor the section hides has no air to see) */
fn roomAir(x: vec3f) -> f32 { return select(0.0, 1.0, x.y < frame.misc.y && abs(x.x) < 12.0 && abs(x.z) < 8.0 && x.y < 6.7 && (x.y < 3.3 || x.x < 4.0)); }
fn vn3(p: vec3f) -> f32 { let i = floor(p); let f = p - i; let w = f * f * (3.0 - 2.0 * f); let z = f32(i.z) * 37.0; return mix(vnoise(p.xy + z), vnoise(p.xy + z + 37.0), w.z); }
/** 1 where the section leaves the air to be seen, 0 over the rooms it hides, softly */
fn cutShows(x: vec3f) -> f32 { if (frame.misc.y > 1e8) { return 1.0; } let over = abs(x.x) < 12.3 && abs(x.z) < 8.3 && !(x.x > 4.0 && frame.misc.y > OPEN_FLOOR); return select(1.0, smoothstep(0.0, 0.35, frame.misc.y - x.y), over); }
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
  for (var i = 0u; i < n; i++) {
    let x = st[i].xyz; var V = vec3f(0.0); let room = roomAir(x);
    if (max3(frame.sunE.xyz) > 0.0) { V = frame.sunE.xyz * lum(visible(x, coneDir(frame.sunDir.xyz, frame.sunDir.w, vec2f(j.y, 0.5)), 1e-3, 120.0)) * hg(mu, 0.5); }
    let c = cacheRead(vec3f(x.x, floor(x.y / 3.4) * 3.4 + select(0.02, 0.0, room < 0.5), x.z), vec3f(0.0, 1.0, 0.0), vec2f(0.5)); let near = select(amb * 0.25, c.E * (1.0 / (4.0 * PI)) * 0.5, room > 0.5 && c.n > 0.0);
    let e = exp(-st[i].w); let part = exp(-tau) * (1.0 - e); seen += vec2f(dot(x - ro, rd), 1.0) * part; L += part * (V + near); tau += st[i].w;
  }
  // the smoke: small steps where there is any, over empty bricks in one. What the section hides takes its smoke with it
  if (frame.misc2.x > 0.5) { let s = smokeSpan(ro, rd, t1); var t = s.x + 0.1 * j.x; var T = 1.0; var G = vec3f(0.0);
    for (var i = 0u; i < 220u; i++) { if (t >= s.y || T < 0.012) { break; } let x = ro + rd * t; let shown = cutShows(x); let p = smokeSlot(x);
      if (p == 0u || shown <= 0.0) { let c = floor((x - SM_AT) / SM_BRICK); let e = (SM_AT + (c + select(vec3f(0.0), vec3f(1.0), rd > vec3f(0.0))) * SM_BRICK - ro) * safeInv(rd); t = max(min3(e), t) + 0.004; continue; }
      let dt = min(0.1, s.y - t); let v = smokeIn(p, x); var dn = v.w * shown; if (max3(v.rgb) > 24.0) { let q = x * vec3f(9.0, 5.0, 9.0) + vec3f(0.0, -16.0 * frame.misc.x, 0.0); let nz = 0.6 * vn3(q) + 0.4 * vn3(q * 2.3 + 9.0); dn *= 2.2 * smoothstep(0.40, 0.72, nz); } else if (dn > 0.004 && dn < 2.0) { let q = x * 5.2 + vec3f(0.13, -0.5, 0.09) * frame.misc.x; let nz = 0.62 * vn3(q) + 0.38 * vn3(q * 2.7 + 5.0); dn = dn * mix(smoothstep(0.28, 0.62, nz) * 1.8, 1.0, smoothstep(0.2, 2.0, dn)); } if (dn > 0.003) { let e = exp(-dn * dt); let part = T * (1.0 - e) * exp(-tau); seen += vec2f(t, 1.0) * part; G += v.rgb * part; T *= e; } t += dt; }
    L += G; tau -= log(max(T, 1e-5)); }
  // beams: one lamp a pixel, drawn by how near the ray passes it, and one point drawn by its nearness along the ray
  { var W = 0.0; var pick = -1; var w0 = 0.0; var u = j2.y; for (var i = 0u; i < frame.counts.x; i++) { if (lamps[i].kind == 0u || lamps[i].on <= 0.0 || lamps[i].at.y > frame.misc.y) { continue; } let d = lamps[i].at - ro; let b = clamp(dot(d, rd), 0.0, t1); let D = max(length(d - rd * b), 0.15); if (D > lamps[i].reach * 0.7) { continue; } var w = lum(lamps[i].radiance) * lamps[i].area * lamps[i].area / D; if (lamps[i].kind == 2u) { let c = dot(normalize(ro + rd * b - lamps[i].at), lamps[i].n); w *= 0.05 + smoothstep(0.2, 0.95, c); } W += w; let q = w / W; if (u < q) { pick = i32(i); w0 = w; u = u / q; } else { u = (u - q) / (1.0 - q); } }
    if (pick >= 0) { let l = &lamps[u32(pick)]; let d = (*l).at - ro; let b = dot(d, rd); let D = max(length(d - rd * b), 0.12); let ta = max(0.0, b - 9.0); let tb = min(t1, b + 9.0);
      if (tb > ta) { let a0 = atan((ta - b) / D); let a1 = atan((tb - b) / D); for (var m = 0u; m < 3u; m++) { let t = b + D * tan(mix(a0, a1, (f32(m) + fract(j2.x + 0.37 + f32(m) * 0.618)) / 3.0)); let pdf = D / ((a1 - a0) * (D * D + (t - b) * (t - b))); let x = ro + rd * t; let s = select(0.0, airAt(x) + @tune.beams, x.y < frame.misc.y); let a = aimLamp(u32(pick), x, vec2f(0.5));
        if (max3(a.E) > 0.0) { let V = lum(visible(x, a.dir, 1e-3, a.dist - 0.1)); L += min(a.E * V * (s * hg(dot(rd, a.dir), 0.35) / pdf * W / w0), vec3f(6.0)) * (1.0 / 3.0); } } } } }
  textureStore(outD, id.xy, vec4f(seen.x / seen.y, seen.y, 0.0, 0.0));       // (how far off what is seen of the air is: its history moves as that does, not as what stands behind it)
  textureStore(outA, id.xy, vec4f(L, exp(-tau)));
}
