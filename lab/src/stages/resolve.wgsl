// TOGETHER: the exact parts times the three signals. Nothing here looks for light.
var<private> upWay = vec3f(0.0);                                     // which way the wide light comes from, as a share of it, along the surface (0 = alike from all sides)
fn up(id: vec2u, p: Px) -> vec4f {
  var wv = vec3f(0.0);
  let hd = vec2i(textureDimensions(bounce)); let b = vec2f(id) / 2.0 - 0.25; let q0 = vec2i(floor(b)); let f = b - floor(b); var a = vec4f(0.0); var ws = 0.0;
  for (var k = 0; k < 4; k++) { let q = clamp(q0 + vec2i(k & 1, k >> 1u), vec2i(0), hd - 1); let fq = min(q * 2, vec2i(frame.size.xy) - 1); let s = unpackPx(textureLoad(surf, fq, 0), pxRay(vec2u(fq))); if (s.sky || s.leaf != p.leaf) { continue; }
    var w = (select(1.0 - f.x, f.x, (k & 1) == 1) * select(1.0 - f.y, f.y, k > 1) + 0.01); w *= pow(sat(dot(s.ng, p.ng)), 8.0) * exp(-abs(dot(s.x - p.x, p.ng)) / (0.003 * p.t + 0.01)) + 1e-4; a += textureLoad(bounce, q, 0) * w; wv += textureLoad(bounceV, q, 0).xyz * w; ws += w; }
  upWay = wv / max(lum(a.rgb), 1e-5); upWay -= p.ng * dot(upWay, p.ng); let l = length(upWay); upWay *= min(1.0, 0.45 / max(l, 1e-4)); return a / max(ws, 1e-5);
}
/** the wide light on a bump that leans, and what of it comes from one way, against what is alike from all sides: the first two terms of its spread */
fn onBump(p: Px) -> f32 { return clamp(1.0 + 2.667 * dot(upWay, p.n), 0.2, 2.6); }
fn fromWay(d: vec3f, sharp: f32) -> f32 { return clamp(1.0 + 4.0 * sharp * dot(upWay, d), 0.12, 3.0); }
/** a mirror at this pixel: the texels that show the same kind of thing. front: the clear layer's */
fn upMirror(mirror: texture_2d<f32>, id: vec2u, p: Px, f: vec4u, front: bool) -> vec4f {
  let hd = vec2i(textureDimensions(mirror)); let b = vec2f(id) / 2.0; let q0 = vec2i(floor(b)); let fr = b - floor(b); var a = vec4f(0.0); var ws = 0.0;
  for (var k = 0; k < 4; k++) { let q = clamp(q0 + vec2i(k & 1, k >> 1u), vec2i(0), hd - 1); let fq = min(q * 2, vec2i(frame.size.xy) - 1); let m = textureLoad(mirror, q, 0); if (m.a < 0.0) { continue; }
    var w = select(1.0 - fr.x, fr.x, (k & 1) == 1) * select(1.0 - fr.y, fr.y, k > 1) + 0.02; if (front) { let g = textureLoad(frontT, fq, 0); w *= pow(sat(dot(octUnpack(g.y), octUnpack(f.y))), 8.0) + 1e-3; } else { let s = unpackPx(textureLoad(surf, fq, 0), pxRay(vec2u(fq))); w *= pow(sat(dot(s.n, p.n)), 16.0) * exp(-abs(dot(s.x - p.x, p.ng)) / (0.003 * p.t + 0.01)) * exp(-abs(s.rough - p.rough) * 12.0) + 1e-4; }
    a += m * w; ws += w; }
  if (ws <= 0.0) { return vec4f(0.0, 0.0, 0.0, -1.0); } return a / ws;
}
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = vec2u(frame.size.xy); if (id.x >= dim.x || id.y >= dim.y) { return; }
  let rd = pxRay(id.xy); let V = -rd; let f = textureLoad(frontT, id.xy, 0); let p = underWater(unpackPx(textureLoad(surf, id.xy, 0), rd), f, rd); let em = textureLoad(emit, id.xy, 0); var c = em.rgb * part(0u); var mir = vec4f(0.0, 0.0, 0.0, -1.0); if (!p.leaf && (p.rough < asksMirror() || p.metal > 0.5)) { mir = upMirror(mirror, id.xy, p, f, false); }
  if (p.kind == 63u) { c = up(id.xy, p).rgb * p.albedo * (@tune.bounce * part(4u) / PI); }
  else if (!p.sky) {
    let s = pxSurf(p); let lt = textureLoad(lampT, id.xy, 0); let vq = lightFirst(lt); let vz = vec4f(0.0, vq.x, vq.y, vq.z); var b = vec4f(0.0); var bb = vec3f(0.0); var ao = vec2f(1.0); var kept = false; if (p.leaf) { let e = folLight(p.x, p.n, p.ao.x); if (e.w > 0.0) { kept = true; b = vec4f(e.rgb, 0.0); bb = folLight(p.x, -p.n, p.ao.y).rgb * @tune.bounce; } } var flat = vec3f(0.0); if (!kept) { b = up(id.xy, p); flat = b.rgb * @tune.bounce; bb = flat; if (!p.leaf) { b *= onBump(p); } } b *= @tune.bounce; let rv = reflect(rd, p.n); let nv = max(dot(p.n, V), 0.02); let F = fresnel(coatF0(s), nv); let m = matteOf(s) * (1.0 - lum(F)) / PI; var ma = m * matteSeen(s.kind, s.albedo, nv); let car = isPaint(p.kind); let tv = coatT(nv); if (car) { ma = paintMatte(s.albedo) * (tv * 0.91 / PI); } let L = frame.sunDir.xyz; let cs = dot(p.n, L);
    var sun = frame.sunE.xyz * vz.a * poolLight(p.x) * part(1u);
    if (cs > 0.0) { if (car) { c += sun * (paintBase(s, V, L) + coatOf(s, V, L, 0.006)); } else { c += sun * (m * cs * roughMatte(s.kind, s.albedo, s.n, V, L) + coatOf(s, V, L, 0.006)); } } else if (p.leaf) { c += sun * s.through * (-cs / PI); }
    let u = lightAll(lt); let El = u.rgb + vec3f(1.06, 0.99, 0.87) * u.a * vz.y; var Cl = vec3f(0.0);
    if (byFormula(p.rough) > 0.0 || car) { let hc = textureLoad(lampC, id.xy, 0); let f0 = select(coatF0(s), flakeF0(s.albedo) * FLAKES, car); Cl = hc.rgb * vz.z + vec3f(1.06, 0.99, 0.87) * f0 / max(lum(f0), 1e-4) * hc.a * vz.y; }
    c += part(2u) * El * select(ma, (m + s.through / PI) * 0.5, p.leaf) + part(3u) * Cl * select(byFormula(p.rough), tv * 0.91, car); if (car) { c += part(4u) * flat / PI * fromWay(rv, 0.85) * flakeF0(s.albedo) * (FLAKES * tv * 0.86); } if (p.leaf) { c += part(4u) * (b.rgb * m * ao.x + bb * s.through / PI * ao.y); } else { c += part(4u) * b.rgb * ma; }
    if (mir.a >= 0.0) { let k = 1.0 - smoothstep(asksMirror() - 0.07, asksMirror(), p.rough); let amb = b.rgb / PI * (coatF0(s) + (max(vec3f(1.0 - p.rough), coatF0(s)) - coatF0(s)) * pow(1.0 - nv, 5.0)) * 0.9; c += mix(amb * part(4u), mir.rgb * (part(5u) * @tune.mirror), select(k, 1.0, p.metal > 0.5)); }
    else if (f.x == 0u && kept) { let g = coatF0(s); let r = folLight(p.x, reflect(rd, p.n), p.ao.x); c += part(4u) * @tune.bounce * r.rgb / PI * (g + (max(vec3f(1.0 - p.rough), g) - g) * pow(1.0 - nv, 5.0)); }     // a waxy leaf's broad shine: what comes to it from that way
    else if (f.x == 0u && p.leaf) { let R = reflect(rd, p.n); let open = sat(lum(b.rgb) / max(lum(skyLod(p.n, 7.0)) * PI, 1e-5)); c += part(4u) * skyLod(R, 4.0) * (open * open * ao.x) * F; }     // a waxy leaf shows the sky, as much of it as reaches here
    else if (f.x == 0u) { c += part(4u) * b.rgb / PI * (coatF0(s) + (max(vec3f(1.0 - p.rough), coatF0(s)) - coatF0(s)) * pow(1.0 - nv, 5.0)) * 0.8; }
    else { c += part(4u) * b.rgb / PI * coatF0(s) * 0.6; }
  }
  if (f.x != 0u && part(6u) > 0.0) {                                 // the clear layer: what it lets through, and what it shows
    let k = f.z >> 24u; let n = octUnpack(f.y); let cv = abs(dot(n, V)); let f0 = select(0.04, 0.02, k == 2u); let F = f0 + (1.0 - f0) * pow(1.0 - cv, 5.0); let x = frame.camPos.xyz + rd * bitcast<f32>(f.x);
    var T = unpackRGB(f.z); if (k == 2u) { let w = bitcast<f32>(f.w); T = exp(-WATER_SIGMA * w * 1.15); c = c * T + vec3f(0.010, 0.050, 0.055) * (1.0 - exp(-w * 0.8)) * (lum(frame.sunE.xyz) * max(frame.sunDir.y, 0.0) * 0.25 + lum(skyLod(vec3f(0.0, 1.0, 0.0), 6.0)) * 0.3); T = vec3f(1.0); let L = frame.sunDir.xyz; var nn = n; let hv = normalize(L + V); c += frame.sunE.xyz * ggxD(0.016, sat(dot(nn, hv))) * 0.02 * max(dot(nn, L), 0.0) * 0.25 * lightFirst(textureLoad(lampT, id.xy, 0)).z; }
    c = c * T * (1.0 - F) + max(upMirror(shine, id.xy, p, f, true).rgb, vec3f(0.0));
  }
  let ai = vec2i(id.xy) / 2; let ad = vec2i(textureDimensions(air)); var a = vec4f(0.0); var ws = 0.0; for (var k = 0; k < 4; k++) { let q = clamp(ai + vec2i(k & 1, k >> 1u), vec2i(0), ad - 1); let t = bitcast<f32>(textureLoad(surf, min(q * 2, vec2i(dim) - 1), 0).x); let w = exp(-abs(t - p.t) / (0.04 * p.t + 0.1)) + 1e-3; a += textureLoad(air, q, 0) * w; ws += w; } a /= ws;
  if (part(7u) > 0.0) { c = c * a.a + a.rgb; }
  if (sightK[0].w > 0.5) { var q = 0.0; if (!p.sky && p.kind != 63u) { q = seenBy(p.x, p.ng); } c = remembered(c, q); }
  textureStore(outC, id.xy, vec4f(min(c, vec3f(500.0)), p.t));
}
