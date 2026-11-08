// TOGETHER: the exact parts times the three signals. Nothing here looks for light.
fn up(id: vec2u, p: Px) -> vec4f {
  let hd = vec2i(textureDimensions(bounce)); let b = vec2f(id) / 2.0 - 0.25; let q0 = vec2i(floor(b)); let f = b - floor(b); var a = vec4f(0.0); var ws = 0.0;
  for (var k = 0; k < 4; k++) { let q = clamp(q0 + vec2i(k & 1, k >> 1u), vec2i(0), hd - 1); let fq = min(q * 2, vec2i(frame.size.xy) - 1); let s = unpackPx(textureLoad(surf, fq, 0), pxRay(vec2u(fq))); if (s.sky) { continue; }
    var w = (select(1.0 - f.x, f.x, (k & 1) == 1) * select(1.0 - f.y, f.y, k > 1) + 0.01); w *= pow(sat(dot(s.n, p.n)), 8.0) * exp(-abs(dot(s.x - p.x, p.ng)) / (0.003 * p.t + 0.01)) + 1e-4; a += textureLoad(bounce, q, 0) * w; ws += w; }
  return a / max(ws, 1e-5);
}
/** the mirror at this pixel: the texels that show the same kind of thing */
fn upMirror(id: vec2u, p: Px, f: vec4u) -> vec4f {
  let hd = vec2i(textureDimensions(mirror)); let b = vec2f(id) / 2.0; let q0 = vec2i(floor(b)); let fr = b - floor(b); var a = vec4f(0.0); var ws = 0.0; let front = f.x != 0u;
  for (var k = 0; k < 4; k++) { let q = clamp(q0 + vec2i(k & 1, k >> 1u), vec2i(0), hd - 1); let fq = min(q * 2, vec2i(frame.size.xy) - 1); let m = textureLoad(mirror, q, 0); if (m.a < 0.0) { continue; } let g = textureLoad(frontT, fq, 0); if ((g.x != 0u) != front) { continue; }
    var w = select(1.0 - fr.x, fr.x, (k & 1) == 1) * select(1.0 - fr.y, fr.y, k > 1) + 0.02; if (front) { w *= pow(sat(dot(octUnpack(g.y), octUnpack(f.y))), 8.0) + 1e-3; } else { let s = unpackPx(textureLoad(surf, fq, 0), pxRay(vec2u(fq))); w *= pow(sat(dot(s.n, p.n)), 16.0) * exp(-abs(dot(s.x - p.x, p.ng)) / (0.003 * p.t + 0.01)) * exp(-abs(s.rough - p.rough) * 12.0) + 1e-4; }
    a += m * w; ws += w; }
  if (ws <= 0.0) { return vec4f(0.0, 0.0, 0.0, -1.0); } return a / ws;
}
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = vec2u(frame.size.xy); if (id.x >= dim.x || id.y >= dim.y) { return; }
  let rd = pxRay(id.xy); let V = -rd; let f = textureLoad(frontT, id.xy, 0); let p = underWater(unpackPx(textureLoad(surf, id.xy, 0), rd), f, rd); let em = textureLoad(emit, id.xy, 0); var c = em.rgb; let mir = upMirror(id.xy, p, f);
  if (p.kind == 63u) { c = vec3f(0.0); }
  else if (!p.sky) {
    let s = pxSurf(p); let vz = textureLoad(vis, id.xy, 0); let b = up(id.xy, p); let nv = max(dot(p.n, V), 0.02); let F = fresnel(coatF0(s), nv); let m = matteOf(s) * (1.0 - lum(F)) / PI; let L = frame.sunDir.xyz; let cs = dot(p.n, L);
    var sun = frame.sunE.xyz * vz.a * poolLight(p.x);
    if (cs > 0.0) { c += sun * (m * cs + coatOf(s, V, L, 0.006)); } else if (p.leaf) { c += sun * s.through * (-cs / PI); }
    let u = textureLoad(lampU, id.xy, 0); let El = u.rgb * vz.x + vec3f(1.06, 0.99, 0.87) * u.a * vz.y; var Cl = vec3f(0.0);
    if (p.rough >= 0.3 && !p.leaf && (u.r + u.g + u.b + u.a) > 1e-3) { for (var i = 0u; i < frame.counts.x; i++) { let k = select(vz.x, vz.y, lamps[i].kind == 2u); if (k > 0.0) { Cl += lampCoat(i, s, V) * k; } } }
    c += El * select(m, (m + s.through / PI) * 0.5, p.leaf) + Cl * smoothstep(0.3, 0.42, p.rough); c += b.rgb * select(m, m + s.through / PI, p.leaf);
    if (f.x == 0u && mir.a >= 0.0) { let k = 1.0 - smoothstep(0.36, 0.43, p.rough); let amb = b.rgb / PI * (coatF0(s) + (max(vec3f(1.0 - p.rough), coatF0(s)) - coatF0(s)) * pow(1.0 - nv, 5.0)) * 0.9; c += mix(amb, mir.rgb, select(k, 1.0, p.metal > 0.5)); }
    else if (f.x == 0u) { c += select(1.0, 0.25, p.leaf) * b.rgb / PI * (coatF0(s) + (max(vec3f(1.0 - p.rough), coatF0(s)) - coatF0(s)) * pow(1.0 - nv, 5.0)) * 0.8; }
    else { c += b.rgb / PI * coatF0(s) * 0.6; }
  }
  if (f.x != 0u) {                                                   // the clear layer: what it lets through, and what it shows
    let k = f.z >> 24u; let n = octUnpack(f.y); let cv = abs(dot(n, V)); let f0 = select(0.04, 0.02, k == 2u); let F = f0 + (1.0 - f0) * pow(1.0 - cv, 5.0); let x = frame.camPos.xyz + rd * bitcast<f32>(f.x);
    var T = unpackRGB(f.z); if (k == 2u) { let w = bitcast<f32>(f.w); T = exp(-WATER_SIGMA * w * 1.15); c = c * T + vec3f(0.010, 0.050, 0.055) * (1.0 - exp(-w * 0.8)) * (lum(frame.sunE.xyz) * max(frame.sunDir.y, 0.0) * 0.25 + 0.15); T = vec3f(1.0); let L = frame.sunDir.xyz; var nn = n; let hv = normalize(L + V); c += frame.sunE.xyz * ggxD(0.016, sat(dot(nn, hv))) * 0.02 * max(dot(nn, L), 0.0) * 0.25 * textureLoad(vis, id.xy, 0).a; }
    c = c * T * (1.0 - F) + max(mir.rgb, vec3f(0.0)) * select(0.0, 1.0, mir.a >= 0.0);
  }
  let ai = vec2i(id.xy) / 2; let ad = vec2i(textureDimensions(air)); var a = vec4f(0.0); var ws = 0.0; for (var k = 0; k < 4; k++) { let q = clamp(ai + vec2i(k & 1, k >> 1u), vec2i(0), ad - 1); let t = bitcast<f32>(textureLoad(surf, min(q * 2, vec2i(dim) - 1), 0).x); let w = exp(-abs(t - p.t) / (0.04 * p.t + 0.1)) + 1e-3; a += textureLoad(air, q, 0) * w; ws += w; } a /= ws;
  c = c * a.a + a.rgb;
  if (frame.weather.y > 0.0) {                                       // rain: falling streaks in three sheets before the surface, where the ray is in the open
    for (var k = 0u; k < 3u; k++) {
      let y = array<f32, 3>(8.6, 5.2, 2.4)[k] ; let t = (y - frame.camPos.y) / min(rd.y, -0.05); if (t < 2.0 || t > p.t) { continue; } let x = frame.camPos.xyz + rd * t; if (!openAir(x)) { continue; }
      let r = normalize(frame.camRight.xyz); let q = vec2f(dot(x, r) * 7.0 + f32(k) * 31.0, dot(x, normalize(frame.camUp.xyz)) * 0.75 + frame.misc.x * 8.5 + f32(k) * 3.1); let cell = floor(q.x); let h = hash33(vec3f(cell, f32(k), 1.0)); let v = fract(q.y * 0.8 + h.x * 9.0); let u = abs(q.x - cell - 0.5 - (h.y - 0.5) * 0.5);
      let px = t * 2.0 * length(frame.camUp.xyz) / frame.size.y * 7.0; let line = (1.0 - smoothstep(0.02, 0.02 + px, u)) * min(0.05 / (0.04 + px), 1.0); let drop = smoothstep(0.0, 0.03, v) * (1.0 - smoothstep(0.10, 0.22, v)) * step(0.45, h.z);
      c += (a.rgb * 1.5 + skyLod(vec3f(0.0, 1.0, 0.0), 6.0) * 0.18) * (line * drop * frame.weather.y * 0.55); } }
  textureStore(outC, id.xy, vec4f(min(c, vec3f(500.0)), p.t));
}
