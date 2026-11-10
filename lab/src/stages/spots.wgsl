// WHAT A TORCH LIGHTS IS LIGHT THAT GOES ON. A ray thrown for bounced light finds a small bright spot by luck, and what is kept of it trails behind. So the spot is listed instead, anew each frame: 256 rays over the cone of
// each of the five nearest torches; where each lands, which way that faces, and how much it sends on. Whoever wants to know what a torch's first bounce sends a place (spotLight) draws a few from the list, keeps
// one by what it would send, and asks whether it is seen. The list lies in `extra`, so that every ray's hit can ask.
@compute @workgroup_size(64) fn main(@builtin(workgroup_id) wg: vec3u, @builtin(local_invocation_index) li: u32) {
  let k = wg.x >> 2u; let i = (wg.x & 3u) * 64u + li; let o = (k * 256u + i) * 2u; spots[o] = vec4u(0u); spots[o + 1u] = vec4u(0u); if (k >= torchK[0].x) { return; }
  let l = torchK[1u + (k >> 2u)][k & 3u]; let lamp = &lamps[l]; let n0 = (*lamp).n; let bs = basis(n0); let wide = torchWide(l); let cell = 2.0 * wide / 16.0; leafMode = 2u; if (i % 17u == 0u) { rngState = pcg(o); } else { rngKey(o, 41u); }       /* (one in sixteen is the same way every frame: by those it is told what has changed, spotChange) */
  let p = ((vec2f(f32(i & 15u), f32(i >> 4u)) + rnd2()) / 16.0 * 2.0 - 1.0) * wide; let d = normalize(n0 + bs[0] * p.x + bs[1] * p.y); let c = dot(d, n0); let part = smoothstep((*lamp).cone, mix((*lamp).cone, 1.0, 0.55), c) * (0.35 + 0.65 * smoothstep(0.9, 1.0, c)); if (part <= 0.0) { return; }
  foot(0.0, cell); let h = nearest((*lamp).at, d, 0.08, 24.0, 0u, 1e9); if (h.piece == NOTHING) { return; }
  let q = surfaceMean((*lamp).at, d, h); let dw = cell * cell * c * c * c; let sent = (*lamp).radiance * ((*lamp).on * PI * (*lamp).area * (*lamp).area * part * dw) * returns(q) * h.thru * blkT; let face = -dot(q.ng, d); if (face <= 0.02 || max3(sent) <= 0.0) { return; }
  spots[o] = vec4u(bitcast<vec3u>(q.x + q.ng * 0.005), octPack(q.ng)); spots[o + 1u] = vec4u(pack2x16float(sent.rg), pack2x16float(vec2f(sent.b, max(h.t * h.t * dw / (face * PI), 0.0025))), 0u, 0u);
}
