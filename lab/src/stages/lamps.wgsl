// SIGNAL 1, LAMP VISIBILITY. The sun: the share of its disc seen. The rest: one lamp a pixel, drawn by what it would send, and what last frame's draw found
// (a reservoir), so that a few rays go to the lamps that matter. Out: the lamps' light (per unit of matte) and the sun's share.
struct Res { lamp: u32, uv: vec2f, W: f32, M: f32 };
fn unpackRes(v: vec2u) -> Res { var r: Res; r.lamp = v.x & 255u; r.M = f32((v.x >> 8u) & 63u); r.uv = vec2f(f32((v.x >> 14u) & 511u), f32((v.x >> 23u) & 511u)) / 511.0; r.W = bitcast<f32>(v.y); return r; }
fn packRes(r: Res) -> vec2u { let a = u32(sat(r.uv.x) * 511.0 + 0.5); let b = u32(sat(r.uv.y) * 511.0 + 0.5); return vec2u((r.lamp & 255u) | (u32(clamp(r.M, 0.0, 63.0)) << 8u) | (a << 14u) | (b << 23u), bitcast<u32>(r.W)); }
fn target0(l: u32, uv: vec2f, x: vec3f, n: vec3f, leaf: bool) -> f32 { if (l >= frame.counts.x || lamps[l].on <= 0.0) { return 0.0; } let a = aimLamp(l, x, uv); let c = dot(n, a.dir); return lum(a.E) * select(max(c, 0.0), abs(c), leaf); }
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) {
  let dim = vec2u(frame.size.xy); if (id.x >= dim.x || id.y >= dim.y) { return; }
  let rd = pxRay(id.xy); let p = unpackPx(textureLoad(surf, id.xy, 0), rd);
  if (p.sky || p.kind == 99u) { textureStore(outVis, id.xy, vec4f(0.0)); textureStore(outRes, id.xy, vec4u(0u)); return; }
  rngInit(id.xy, 3u); let bn = blue2(id.xy, 0u);
  // the lamps: draw one, and weigh it against last frame's
  var best: Res; best.lamp = 255u; best.W = 0.0; best.M = 0.0; var wsum = 0.0; var ph = 0.0;
  { let k = drawLamp(p.x, select(p.n, vec3f(0.0), p.leaf)); best.M = 1.0; if (k.x >= 0.0) { best.lamp = u32(k.x); best.uv = rnd2(); ph = target0(best.lamp, best.uv, p.x, p.n, p.leaf); wsum = ph * k.y; } }
  let aux = textureLoad(auxT, id.xy, 0); let uv = (vec2f(id.xy) + 0.5) / vec2f(dim) + aux.xy; let q = vec2i(floor(uv * vec2f(dim) + (rnd2() - 0.5) * 0.9));
  if (all(q >= vec2i(0)) && all(q < vec2i(dim))) {
    let o = unpackPx(textureLoad(surfPrev, q, 0), vec3f(0.0)); let was = frame.prevPos.xyz + pxRayPrev((vec2f(q) + 0.5 + frame.jitter.zw) / vec2f(dim)) * o.t;
    if (abs(dot(was - p.x, p.ng)) < 0.004 * p.t + 0.02 && dot(o.n, p.n) > 0.8) {
      var r = unpackRes(textureLoad(resPrev, q, 0).xy); let st = steady(p.x); r.M = min(r.M, mix(1.0, 18.0, st));
      if (r.lamp < 255u && r.M > 0.0) { if (lamps[r.lamp & 63u].kind == 2u) { r.M = min(r.M, 2.0); } let w0 = target0(r.lamp, r.uv, p.x, p.n, p.leaf); let w = w0 * r.W * r.M; let M0 = best.M; wsum += w; if (w > 0.0 && rnd() * wsum < w) { best.lamp = r.lamp; best.uv = r.uv; ph = w0; } best.M = M0 + r.M; } else { best.M += r.M; }
    }
  }
  best.W = select(0.0, wsum / max(ph * best.M, 1e-9), ph > 0.0);
  // two rays, one question
  var E = vec3f(0.0); var sun = 0.0;
  for (var k = 0u; k < 2u; k++) {
    var d: vec3f; var far = 400.0; var a: Aim; fineLeaves = k == 0u;
    if (k == 0u) { d = coneDir(frame.sunDir.xyz, frame.sunDir.w, bn); if (max3(frame.sunE.xyz) <= 0.0 || (dot(p.ng, d) <= 0.0 && !p.leaf)) { continue; } }
    else { if (best.W <= 0.0) { continue; } a = aimLamp(best.lamp, p.x, best.uv); d = a.dir; far = a.dist - 0.02; if (dot(p.ng, d) <= 0.0 && !p.leaf) { best.W = 0.0; continue; } }
    let T = visible(p.x + p.ng * select(-0.003, 0.003, dot(p.ng, d) > 0.0) * (1.0 + p.t * 0.02), d, 1e-3, far);
    if (k == 0u) { sun = lum(T); } else {
      let c = dot(p.n, d); let f = a.E * best.W * T; if (c > 0.0) { E = f * c; } else if (p.leaf) { E = f * -c * vec3f(0.95, 1.15, 0.35) * 0.8; } if (max3(T) <= 0.0) { best.W = 0.0; }
    }
  }
  let m = max(lum(E), 1e-5); E *= min(1.0, 60.0 / m);
  textureStore(outVis, id.xy, vec4f(E, sun)); textureStore(outRes, id.xy, vec4u(packRes(best), 0u, 0u));
}
