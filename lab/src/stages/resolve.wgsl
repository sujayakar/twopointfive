// TOGETHER: the exact parts times the three signals. Nothing here looks for light.
fn up(id: vec2u, p: Px) -> vec4f {
  let hd = vec2i(textureDimensions(bounce)); let b = vec2f(id) / 2.0 - 0.25; let q0 = vec2i(floor(b)); let f = b - floor(b); var a = vec4f(0.0); var ws = 0.0;
  for (var k = 0; k < 4; k++) { let q = clamp(q0 + vec2i(k & 1, k >> 1u), vec2i(0), hd - 1); let fq = min(q * 2, vec2i(frame.size.xy) - 1); let s = unpackPx(textureLoad(surf, fq, 0), pxRay(vec2u(fq))); if (s.sky) { continue; }
    var w = (select(1.0 - f.x, f.x, (k & 1) == 1) * select(1.0 - f.y, f.y, k > 1) + 0.01); w *= pow(sat(dot(s.n, p.n)), 8.0) * exp(-abs(dot(s.x - p.x, p.ng)) / (0.003 * p.t + 0.01)) + 1e-4; a += textureLoad(bounce, q, 0) * w; ws += w; }
  return a / max(ws, 1e-5);
}
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = vec2u(frame.size.xy); if (id.x >= dim.x || id.y >= dim.y) { return; }
  let rd = pxRay(id.xy); let V = -rd; let p = unpackPx(textureLoad(surf, id.xy, 0), rd); let em = textureLoad(emit, id.xy, 0); let f = textureLoad(frontT, id.xy, 0); let mir = textureLoad(mirror, id.xy, 0); var c = em.rgb;
  if (p.kind == 99u) { c = p.albedo * 0.05; }
  else if (!p.sky) {
    let s = pxSurf(p); let vz = textureLoad(vis, id.xy, 0); let b = up(id.xy, p); let nv = max(dot(p.n, V), 0.02); let F = fresnel(coatF0(s), nv); let m = matteOf(s) * (1.0 - lum(F)) / PI; let L = frame.sunDir.xyz; let cs = dot(p.n, L);
    var sun = frame.sunE.xyz * vz.a * poolLight(p.x);
    if (cs > 0.0) { c += sun * (m * cs + coatOf(s, V, L, 0.006)); } else if (p.leaf) { c += sun * s.through * (-cs / PI); }
    var El = vec3f(0.0); var Cl = vec3f(0.0); let ln = select(p.n, vec3f(0.0), p.leaf); let wantCoat = p.rough >= 0.3 && !p.leaf;
    for (var i = 0u; i < frame.counts.x; i++) { let e = lampE(i, p.x, ln); if (max3(e) <= 1e-5) { continue; } let k = select(vz.x, vz.y, lamps[i].kind == 2u); El += e * k; if (wantCoat && k > 0.0) { Cl += lampCoat(i, s, V) * k; } }
    c += El * select(m, (m + s.through / PI) * 0.8, p.leaf) + Cl * smoothstep(0.3, 0.42, p.rough); c += b.rgb * select(m, (m + s.through / PI * 0.8), p.leaf);
    if (f.x == 0u && mir.a >= 0.0) { let k = 1.0 - smoothstep(0.36, 0.43, p.rough); let amb = b.rgb / PI * (coatF0(s) + (max(vec3f(1.0 - p.rough), coatF0(s)) - coatF0(s)) * pow(1.0 - nv, 5.0)) * 0.9; c += mix(amb, mir.rgb, select(k, 1.0, p.metal > 0.5)); }
    else if (f.x == 0u) { c += b.rgb / PI * (coatF0(s) + (max(vec3f(1.0 - p.rough), coatF0(s)) - coatF0(s)) * pow(1.0 - nv, 5.0)) * 0.8; }
    else { c += b.rgb / PI * coatF0(s) * 0.6; }
  }
  if (f.x != 0u) {                                                   // the clear layer: what it lets through, and what it shows
    let k = f.z >> 24u; let n = octUnpack(f.y); let cv = abs(dot(n, V)); let f0 = select(0.04, 0.02, k == 2u); let F = f0 + (1.0 - f0) * pow(1.0 - cv, 5.0); let x = frame.camPos.xyz + rd * bitcast<f32>(f.x);
    var T = unpackRGB(f.z); if (k == 2u) { let w = bitcast<f32>(f.w); T = exp(-WATER_SIGMA * w * 1.15); c = c * T + vec3f(0.010, 0.050, 0.055) * (1.0 - exp(-w * 0.8)) * (lum(frame.sunE.xyz) * max(frame.sunDir.y, 0.0) * 0.25 + 0.15); T = vec3f(1.0); let L = frame.sunDir.xyz; var nn = n; let hv = normalize(L + V); c += frame.sunE.xyz * ggxD(0.016, sat(dot(nn, hv))) * 0.02 * max(dot(nn, L), 0.0) * 0.25 * textureLoad(vis, id.xy, 0).a; }
    c = c * T * (1.0 - F) + max(mir.rgb, vec3f(0.0)) * select(0.0, 1.0, mir.a >= 0.0);
  }
  let ai = vec2i(id.xy) / 2; let ad = vec2i(textureDimensions(air)); var a = vec4f(0.0); var ws = 0.0; for (var k = 0; k < 4; k++) { let q = clamp(ai + vec2i(k & 1, k >> 1u), vec2i(0), ad - 1); let t = bitcast<f32>(textureLoad(surf, min(q * 2, vec2i(dim) - 1), 0).x); let w = exp(-abs(t - p.t) / (0.04 * p.t + 0.1)) + 1e-3; a += textureLoad(air, q, 0) * w; ws += w; } a /= ws;
  c = c * a.a + a.rgb; if (any(c != c)) { c = vec3f(0.0); }
  textureStore(outC, id.xy, vec4f(min(c, vec3f(500.0)), p.t));
}
