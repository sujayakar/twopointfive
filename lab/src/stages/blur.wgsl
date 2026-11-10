// BLUR, one round (à trous): neighbours on the same surface, alike next to the noise. const STRIDE, DIV, SHARE (sigmas), KIND: 0 light, 1 fractions (their noise is known from their mean), NARROW
fn varOf(q: vec2i) -> vec2f { let m = textureLoad(mom, q, 0); var v = max(m.y - m.x * m.x, 0.0); if (m.z < 4.0) { v = max(v, m.x * m.x * 0.6 / max(m.z, 1.0)); } return vec2f(v, m.z); }
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = vec2i(textureDimensions(outB)); let p0 = vec2i(id.xy); if (any(p0 >= dim)) { return; }
  let rdim = vec2i(frame.size.xy); let full = min(p0 * DIV, rdim - 1); let p = unpackPx(textureLoad(surf, full, 0), pxRay(vec2u(full))); let c = textureLoad(src, p0, 0);
  //2 var acc2 = textureLoad(src2, p0, 0); textureStore(outB2, id.xy, acc2);
  if (p.sky && NARROW == 0) { textureStore(outB, id.xy, c); return; } if (NARROW == 1 && c.a < 0.0) { textureStore(outB, id.xy, c); return; }
  var vm = vec2f(0.0); var wv = 0.0; for (var y = -1; y <= 1; y++) { for (var x = -1; x <= 1; x++) { let q = clamp(p0 + vec2i(x, y), vec2i(0), dim - 1); let w = f32((2 - abs(x)) * (2 - abs(y))); vm += varOf(q) * w; wv += w; } } vm /= wv;
  let m0 = textureLoad(mom, p0, 0); let n = m0.z; let sg = sqrt(vm.x) * (SHARE * @tune.soft) / sqrt(max(n * 0.5, 1.0)) + 1e-3 + 0.012 * lum(c.rgb);
  let young = 1.0 - smoothstep(1.0, 10.0, n); if (KIND == 0 && NARROW == 0 && n > 2.0 * f32(64 / STRIDE)) { textureStore(outB, id.xy, c); return; }      // (what many frames stand behind needs no wide blur)
  var acc = c; var first = 0.0; var ws = 1.0; var wf = vec4f(1.0); let nv = vec4f(n, m0.w, n, n); let yv = vec4f(1.0) - smoothstep(vec4f(1.0), vec4f(10.0), nv); var vc = max(c * (1.0 - c), vec4f(0.0));
  if (KIND == 1) { if (all(((nv > vec4f(13.0)) & (vc < vec4f(0.004))) | (c < vec4f(0.0)))) { textureStore(outB, id.xy, c); return; } wf = lightOf(p0); acc = c * wf; first = firstNow;
    /* (one look says yes or no, never how sure it is: while few frames stand behind it, its neighbours say whether this is an edge of light or the middle of a soft one) */
    if (any((yv > vec4f(0.0)) & (c >= vec4f(0.0)))) { var q = vec4f(0.0); var qn = vec4f(0.0); for (var y = -1; y <= 1; y++) { for (var x = -1; x <= 1; x++) { let v = textureLoad(src, clamp(p0 + vec2i(x, y), vec2i(0), dim - 1), 0); let k = select(vec4f(0.0), vec4f(1.0), v >= vec4f(0.0)); q += v * k; qn += k; } } q /= max(qn, vec4f(1.0)); vc = max(vc, q * (1.0 - q) * yv); } }
  let sf = sqrt(vc) * 0.6 / sqrt(max(nv, vec4f(1.0))) + vec4f(0.012) + 0.35 * yv; let l0 = lum(c.rgb); let fr = LAYER == 1;
  var reach = 1.0; if (NARROW == 1) { var rough = p.rough; if (fr) { rough = 0.02; } let lobe = rough * rough * 1.6 * c.a / max(p.t + c.a, 0.1) * frame.size.y / (2.0 * length(frame.camUp.xyz) * f32(DIV)); reach = lobe * (0.6 + young) + 1.5 * young; if (reach < 0.6 * f32(STRIDE)) { textureStore(outB, id.xy, c); return; } }
  for (var y = -1; y <= 1; y++) { for (var x = -1; x <= 1; x++) {
    if (x == 0 && y == 0) { continue; } let q = p0 + vec2i(x, y) * STRIDE; if (any(q < vec2i(0)) || any(q >= dim)) { continue; }
    let fq = min(q * DIV, rdim - 1); let s = unpackPx(textureLoad(surf, fq, 0), pxRay(vec2u(fq))); if (s.sky && NARROW == 0) { continue; } let v = textureLoad(src, q, 0); if (NARROW == 1) { if (v.a < 0.0) { continue; } }
    var w = f32((2 - abs(x)) * (2 - abs(y))) * 0.25;
    if (!(NARROW == 1 && fr)) { if (p.leaf || s.leaf) { w *= select(0.0, 1.0, p.leaf == s.leaf) * exp(-abs(s.t - p.t) * 4.0) * 0.8; } else { w *= pow(sat(select(dot(s.ng, p.ng), dot(s.n, p.n), NARROW == 1 || KIND == 1)), select(24.0, 64.0, NARROW == 1)) * exp(-abs(dot(s.x - p.x, p.ng)) / (0.0012 * p.t * f32(DIV) + 0.01)); } }
    if (NARROW == 1) { w *= exp(-abs(s.rough - p.rough) * 16.0); }
    if (KIND == 1) { let lq = lightOf(q); let wq = select(vec4f(0.0), w * exp(-abs(v - c) / (sf * @tune.soft)) * lq * vec4f(1.0, 1.0, 1.0, select(0.0, 1.0, firstNow == first)), v >= vec4f(0.0)); acc += v * wq; wf += wq; continue; }     // (by how much light each one's answer is about)
    let wl = w * exp(-abs(lum(v.rgb) - l0) / sg); acc += vec4f(v.rgb * wl, 0.0); ws += wl;
    //2 acc2 += textureLoad(src2, q, 0) * wl;
  } }
  //2 textureStore(outB2, id.xy, acc2 / ws);
  var o = vec4f(acc.rgb / ws, c.a); if (KIND == 1) { o = select(acc / max(wf, vec4f(1e-9)), c, c < vec4f(0.0)); if (p.leaf && first < 0.5) { o.a = c.a; } }     // (the sun between leaves is fine, and nearly free of noise: it is left sharp)
  textureStore(outB, id.xy, o);
}
