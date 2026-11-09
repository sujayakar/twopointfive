// UPSCALER: this frame's jittered samples into the standing picture. Counts samples a pixel, so it is quick at first and calm later.
fn tm(c: vec3f) -> vec3f { return c / (1.0 + lum(c) * taauK.x); }
fn un(c: vec3f) -> vec3f { return c / max(1.0 - lum(c) * taauK.x, 1e-3); }
fn ycc(c: vec3f) -> vec3f { return vec3f(0.25 * c.r + 0.5 * c.g + 0.25 * c.b, 0.5 * (c.r - c.b), -0.25 * c.r + 0.5 * c.g - 0.25 * c.b); }
fn rgb(y: vec3f) -> vec3f { return vec3f(y.x + y.y - y.z, y.x + y.z, y.x - y.y - y.z); }
fn cat(uv: vec2f, dim: vec2f) -> vec4f { let p = uv * dim; let c = floor(p - 0.5) + 0.5; let f = p - c; let w0 = f * (-0.5 + f * (1.0 - 0.5 * f)); let w1 = 1.0 + f * f * (-2.5 + 1.5 * f); let w2 = f * (0.5 + f * (2.0 - 1.5 * f)); let w3 = f * f * (-0.5 + 0.5 * f); let w12 = w1 + w2; let t = (c + w2 / w12) / dim; let a = (c - 1.0) / dim; let b = (c + 2.0) / dim;
  var s = textureSampleLevel(hist, linSamp, vec2f(t.x, a.y), 0.0) * (w12.x * w0.y) + textureSampleLevel(hist, linSamp, vec2f(a.x, t.y), 0.0) * (w0.x * w12.y) + textureSampleLevel(hist, linSamp, t, 0.0) * (w12.x * w12.y) + textureSampleLevel(hist, linSamp, vec2f(b.x, t.y), 0.0) * (w3.x * w12.y) + textureSampleLevel(hist, linSamp, vec2f(t.x, b.y), 0.0) * (w12.x * w3.y); s /= w12.x * w0.y + w0.x * w12.y + w12.x * w12.y + w3.x * w12.y + w12.x * w3.y; return max(s, vec4f(0.0)); }
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let od = vec2f(frame.size.zw); if (f32(id.x) >= od.x || f32(id.y) >= od.y) { return; }
  let rd = frame.size.xy; let r = rd / od; let so = vec2f(id.xy) + 0.5; let s = so * r; let q = vec2i(floor(s - frame.jitter.xy)); let hi = vec2i(rd) - 1;
  let go = sat(length(textureLoad(auxT, clamp(q, vec2i(0), vec2i(rd) - 1), 0).xy * od)); var acc = vec3f(0.0); var wa = 0.0; var m1 = vec3f(0.0); var m2 = vec3f(0.0); var near = 1e9; var nq = clamp(q, vec2i(0), hi); var prox = 0.0;
  for (var y = -1; y <= 1; y++) { for (var x = -1; x <= 1; x++) {
    let t = clamp(q + vec2i(x, y), vec2i(0), hi); let v = textureLoad(color, t, 0); let c = ycc(tm(v.rgb)); m1 += c; m2 += c * c; let d = (vec2f(t) + 0.5 + frame.jitter.xy - s) / r; let w = exp(-dot(d, d) * mix(4.0, 1.2, go)); acc += c * w; wa += w; let a = abs(d); prox = max(prox, sat(1.35 - a.x) * sat(1.35 - a.y)); if (v.a < near) { near = v.a; nq = t; }
  } }
  let mean = m1 / 9.0; let sd = sqrt(max(m2 / 9.0 - mean * mean, vec3f(0.0))); let cur = select(mean, acc / max(wa, 1e-6), wa > 1e-4);
  var aux = textureLoad(auxT, nq, 0);
  // where what is seen is smoke, it moves as the smoke does and not as what stands behind it; and it is never the same for long
  let av = textureSampleLevel(airD, linSamp, so / od, 0.0).xy; let thick = smoothstep(0.15, 0.55, av.y); if (thick > 0.5) { let x = frame.camPos.xyz + camRay(so / od) * av.x; let was = projectPrev(x - windAt(x) * frame.more.z).xy - project(x).xy; aux.x = was.x; aux.y = was.y; }
  let uv = so / od + aux.xy; var o = cur; var n = 0.0;
  if (all(uv > vec2f(0.0)) && all(uv < vec2f(1.0)) && taauK.y < 0.5) {
    let h0 = cat(uv, od); let lost = bad4(h0); let h = select(h0, vec4f(0.0), lost); let hy = ycc(tm(h.rgb)); let mv = length(aux.xy * od); n = min(h.a, mix(@tune.taau, min(9.0, @tune.taau), max(sat(mv * 0.5), thick)) * taauK.z); let ext = (sd * mix(2.6, 1.0, sat(mv)) + vec3f(0.02, 0.01, 0.01) * mean.x) * 1.1; let dv = hy - mean; let rr = abs(dv) / max(ext, vec3f(1e-5)); let k = max(rr.x, max(rr.y, rr.z)); var hc = hy; if (k > 1.0) { hc = mean + dv / k; n = min(n, @tune.taau / (0.6 + 0.4 * k)); }
    let a = sat(prox * prox + 0.02) / (n * mix(1.0, 0.55, go) + 1.0); o = mix(hc, cur, select(max(a, 0.02), 1.0, lost)); n = min(n + prox, max(40.0, @tune.taau + 4.0));
  }
  textureStore(outT, id.xy, vec4f(max(un(rgb(o)), vec3f(0.0)), max(n, 0.5)));
}
