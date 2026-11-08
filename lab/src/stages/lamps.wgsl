// SIGNAL 1, LAMP VISIBILITY: three fractions, 0‥1. x = of the standing lamps' light, y = of the changing ones' (torches), w = of the sun's disc. A lamp is
// drawn by what it would send, so the mean of "was it seen" is the seen share of their light; how much that light is, is exact, and never filtered (lampU: rgb the standing ones', a the torches').
//   Rays are asked in two steps. Each pixel lists those it wants; then the whole group works through the list, so that none waits for a neighbour with more. A
// pixel whose answer has stood at all or nothing for a while asks one frame in four, and else repeats it: what changes is found by the ones that do ask, and
// the history lets go of the rest.
/** the sun through the leaves, from the sun's own picture of them */
fn leafSun(x: vec3f, n: vec3f, r: vec2f) -> f32 {
  let o = x + n * 0.015 + frame.sunDir.xyz * 0.05; let q = sunMap(o); if (any(q.xy < vec2f(0.002)) || any(q.xy > vec2f(0.998))) { return 1.0; } let d = vec2f(textureDimensions(sunLeaves)); var s = 0.0; let a = r.x * 6.2832; let m = mat2x2f(cos(a), sin(a), -sin(a), cos(a));
  for (var k = 0u; k < 4u; k++) { let off = m * (array<vec2f, 4>(vec2f(0.3, 0.1), vec2f(-0.2, 0.55), vec2f(-0.75, -0.3), vec2f(0.45, -0.8))[k]); let p = vec2i(vec2f(q.x, 1.0 - q.y) * d + off * 1.3); let over = (textureLoad(sunLeaves, p, 0) - q.z) * 80.0; s += select(mix(0.12, 0.025, smoothstep(0.1, 0.8, over)), 1.0, over < 0.032); } /* the first leaf over it, metres up: the farther, the more of them */ return s * 0.25;
}
var<workgroup> rays: array<vec4f, 192>; var<workgroup> whose: array<u32, 192>; var<workgroup> at: array<vec3f, 64>; var<workgroup> out: array<vec3f, 64>; var<workgroup> got: array<f32, 192>; var<workgroup> count: atomic<u32>;
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u, @builtin(local_invocation_index) li: u32) {
  let dim = vec2u(frame.size.xy); let inside = id.x < dim.x && id.y < dim.y; let px = min(id.xy, dim - 1u); if (li == 0u) { atomicStore(&count, 0u); } for (var k = 0u; k < 3u; k++) { got[li * 3u + k] = -1.0; }
  let rd = pxRay(px); let p = underWater(unpackPx(textureLoad(surf, px, 0), rd), textureLoad(frontT, px, 0), rd); let lit = inside && !p.sky && p.kind != 63u;
  rngInit(px, 3u); let bn = blue2(px, 0u); let b2 = blue2(px, 7u); let b3 = blue2(px, 11u); let n = select(p.n, vec3f(0.0), p.leaf);
  var W = vec2f(0.0); var U = vec3f(0.0); var pick = vec2i(-1); var u = vec2f(b2.x, b3.x); var v = vec4f(1.0, 1.0, 0.0, 0.0); let o0 = 0.003 * (1.0 + p.t * 0.02); at[li] = p.x; out[li] = p.ng * o0;
  workgroupBarrier();
  if (lit) {
    for (var i = 0u; i < frame.counts.x; i++) {
      let e = lampE(i, p.x, n); let w = lum(e); if (w <= 1e-5) { continue; } let c = select(0u, 1u, lamps[i].kind == 2u); if (c == 0u) { U += e; } W[c] += w; let q = w / W[c]; if (u[c] < q) { pick[c] = i32(i); u[c] = u[c] / q; } else { u[c] = (u[c] - q) / (1.0 - q); }
    }
    // what stood here last frame, if it is sure of itself
    var was = vec4f(0.5); if (((px.x + 2u * px.y + frame.counts.z) & 3u) != 0u) { let q = vec2i(floor(((vec2f(px) + 0.5) / vec2f(dim) + textureLoad(auxT, px, 0).xy) * vec2f(dim))); if (all(q >= vec2i(0)) && all(q < vec2i(dim))) { let o = unpackPx(textureLoad(surfPrev, q, 0), vec3f(0.0)); if (textureLoad(momPrev, q, 0).z >= 10.0 && abs(o.t - p.t) < 0.02 * p.t && dot(o.ng, p.ng) > 0.95 && !p.leaf && steady(p.x) > 0.95) { was = textureLoad(hist, q, 0); } } }
    for (var k = 0u; k < 3u; k++) {
      var d: vec3f; var far = 400.0; let ch = select(k - 1u, 3u, k == 0u);
      if (k == 0u) { d = coneDir(frame.sunDir.xyz, frame.sunDir.w, bn); if (max3(frame.sunE.xyz) <= 0.0 || (dot(p.ng, d) <= 0.0 && !p.leaf)) { continue; } }
      else { let l = pick[k - 1u]; if (l < 0) { continue; } let a = aimLamp(u32(l), p.x, select(vec2f(b2.y, b3.y), vec2f(b3.y, b2.y), k == 2u)); d = a.dir; far = a.dist - 0.02; if (max3(a.E) <= 0.0 || (dot(p.ng, d) <= 0.0 && !p.leaf)) { v[k - 1u] = 0.0; continue; } }
      if (k < 2u && (was[ch] < 0.004 || was[ch] > 0.996)) { v[ch] = was[ch]; continue; }
      let i = atomicAdd(&count, 1u); rays[i] = vec4f(d, far); whose[i] = (li * 3u + k) | select(0u, 0x80000000u, dot(p.ng, d) > 0.0);
    }
  }
  workgroupBarrier(); let n0 = atomicLoad(&count);
  for (var i = li; i < n0; i += 64u) { let r = rays[i]; let w = whose[i] & 0x7fffffffu; fineLeaves = false; noLeaves = w % 3u == 0u; got[w] = lum(visible(at[w / 3u] + out[w / 3u] * select(-1.0, 1.0, whose[i] > 0x7fffffffu), r.xyz, 1e-3, r.w)); }
  workgroupBarrier(); if (!inside) { return; }
  if (!lit) { textureStore(outVis, id.xy, vec4f(0.0)); textureStore(outU, id.xy, vec4f(0.0)); return; }
  let s = got[li * 3u]; if (s >= 0.0) { v.w = s; if (s > 0.0) { v.w = s * leafSun(p.x, p.ng * select(-1.0, 1.0, dot(p.ng, frame.sunDir.xyz) > 0.0), b3); } }
  if (got[li * 3u + 1u] >= 0.0) { v.x = got[li * 3u + 1u]; } if (got[li * 3u + 2u] >= 0.0) { v.y = got[li * 3u + 2u]; }
  textureStore(outVis, id.xy, v); textureStore(outU, id.xy, vec4f(U, W.y));
}
