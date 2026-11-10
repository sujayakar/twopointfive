// WHAT A TORCH LIGHTS IS A LIGHT OF ITS OWN. A ray thrown for bounced light finds a small bright spot by luck, and what is kept of it trails behind; so the spot is measured instead, anew each frame: 256 rays over the
// cone, gathered by which of six ways the surface they met faces. Each of the six is a rectangle that sends as much as they do together, from where they are, as wide as they are spread: one lamp more, for everything that
// asks lamps (lamps, the light kept on surfaces, in cells and in the air). A way that catches more as the torch turns grows brighter from nothing, so nothing pops.
var<workgroup> at: array<vec4f, 256>; var<workgroup> to: array<vec4f, 256>; var<workgroup> sends: array<vec4f, 256>;
@compute @workgroup_size(64) fn main(@builtin(workgroup_id) wg: vec3u, @builtin(local_invocation_index) li: u32) {
  let k = wg.x; let live = k < torchK[0].x; let l = torchK[1u + (k >> 2u)][k & 3u]; let lamp = &lamps[select(0u, l, live)]; let n0 = (*lamp).n; let bs = basis(n0); let wide = torchWide(select(0u, l, live)); let cell = 2.0 * wide / 16.0; leafMode = 2u;
  for (var i = li; i < 256u; i += 64u) {
    at[i] = vec4f(0.0); to[i] = vec4f(0.0); sends[i] = vec4f(0.0); if (!live) { continue; }
    let p = ((vec2f(f32(i & 15u), f32(i >> 4u)) + 0.5) / 16.0 * 2.0 - 1.0) * wide; let d = normalize(n0 + bs[0] * p.x + bs[1] * p.y); let c = dot(d, n0); let part = smoothstep((*lamp).cone, mix((*lamp).cone, 1.0, 0.55), c) * (0.35 + 0.65 * smoothstep(0.9, 1.0, c)); if (part <= 0.0) { continue; }
    foot(0.0, cell); let h = nearest((*lamp).at, d, 0.08, 24.0, 0u, 1e9); if (h.piece == NOTHING) { continue; }
    let q = surfaceMean((*lamp).at, d, h); let sent = (*lamp).radiance * ((*lamp).on * PI * (*lamp).area * (*lamp).area * part * cell * cell * c * c * c) * returns(q) * h.thru * blkT; let m = lum(sent); let ng = select(q.ng, -q.ng, dot(q.ng, d) > 0.0);
    at[i] = vec4f(q.x, f32(facing6(ng))); to[i] = vec4f(ng * m, 0.0); sends[i] = vec4f(sent, m);
  }
  workgroupBarrier(); if (li >= 6u) { return; }
  var W = 0.0; var flux = vec3f(0.0); var mid = vec3f(0.0); var nn = vec3f(0.0); for (var i = 0u; i < 256u; i++) { let s = sends[i]; if (s.w > 0.0 && u32(at[i].w) == li) { W += s.w; flux += s.rgb; mid += at[i].xyz * s.w; nn += to[i].xyz; } }
  let o = (k * 6u + li) * 5u; if (W <= 1e-7 || dot(nn, nn) <= 0.0) { for (var j = 0u; j < 5u; j++) { cards[o + j] = vec4f(0.0); } return; }
  mid /= W; let n = normalize(nn); let b = basis(n); var c = vec3f(0.0); for (var i = 0u; i < 256u; i++) { let s = sends[i]; if (s.w > 0.0 && u32(at[i].w) == li) { let v = at[i].xyz - mid; let p = vec2f(dot(v, b[0]), dot(v, b[1])); c += vec3f(p.x * p.x, p.y * p.y, p.x * p.y) * s.w; } } c /= W;
  // as wide and as long as they are spread (an even square is the root of three times that, half of it)
  let a = 0.5 * atan2(2.0 * c.z, c.x - c.y); let r = sqrt(max(0.25 * (c.x - c.y) * (c.x - c.y) + c.z * c.z, 0.0)); let m = 0.5 * (c.x + c.y); let hu = max(sqrt(3.0 * max(m + r, 0.0)), 0.06); let hv = max(sqrt(3.0 * max(m - r, 0.0)), 0.06); let u = b[0] * cos(a) + b[1] * sin(a); let v = cross(n, u); let area = 4.0 * hu * hv; let sent = flux / (PI * area);
  cards[o] = vec4f(mid + n * 0.015, 0.0); cards[o + 1u] = vec4f(u * hu, -1.0); cards[o + 2u] = vec4f(v * hv, area); cards[o + 3u] = vec4f(n, 1.0); cards[o + 4u] = vec4f(sent, min(40.0, sqrt(max3(sent) * area / 0.004)));
}
fn facing6(n: vec3f) -> u32 { let a = abs(n); if (a.x >= a.y && a.x >= a.z) { return select(1u, 0u, n.x > 0.0); } if (a.y >= a.z) { return select(3u, 2u, n.y > 0.0); } return select(5u, 4u, n.z > 0.0); }
