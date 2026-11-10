// WHAT A LEVEL LOST OF ITS SHAPE, AS LIGHT. The same looks at the finest mesh, lit and hidden as it is, and at what is drawn in its place for pixels so wide, lit by rays as wide as the picture's: fifteen suns
// (five heights, three ways round) and one ray to all round. Sums go out; what one is of the other is the table (mesh/lost.ts).
// Twice: the second time what a wide ray lands on sends what the first found, as it will.
struct LostK { tau: f32, rays: f32, kinds: u32, round: u32, pad: vec4u, at: array<vec4f, 16> };
const NEAR = 0.7;                                                   // farther than this things are somebody else's shade
@compute @workgroup_size(64) fn main(@builtin(workgroup_id) g: vec3u, @builtin(local_invocation_index) li: u32) {
  let c = g.x; let up = c % LOST_UP; let pi = (c / LOST_UP) % LOST_PX; let kind = c / (LOST_UP * LOST_PX); let px = 0.00025 * exp2(f32(pi)); let lo = lostK.at[kind * 2u]; let hi = lostK.at[kind * 2u + 1u]; let level = bitcast<u32>(lo.w); let cls = bitcast<u32>(hi.w);
  meshLostOn = lostK.pad.x > 0u; leafMode = 1u; var F: array<f32, 36>; for (var i = 0u; i < 36u; i++) { F[i] = 0.0; }
  for (var s = 0u; s < 4u; s++) {
    rngState = pcg(li + 64u * (s + 4u * (lostK.round + 64u * c))); let r3 = vec3f(rnd(), rnd(), rnd()); let turn = rnd() * 6.2832; let sy = array<f32, 3>(0.342, 0.707, 0.966)[up]; let V = vec3f(cos(turn) * sqrt(1.0 - sy * sy), sy, sin(turn) * sqrt(1.0 - sy * sy)); let ro = mix(lo.xyz, hi.xyz, r3) + V * 4.0; let seed = rngState;
    for (var side = 0u; side < 2u; side++) {
      let fine = side == 0u; rngState = seed; foot(0.0, 0.0); meshFoot = 0.0; meshError = select(lostK.tau * px, 0.0, fine); meshObject = select(level, 0u, fine); meshDrawn = !fine; let h = nearest(ro, -V, 1e-3, 30.0, 0u, 1e9); meshObject = 0u; meshDrawn = false; meshError = 0.0; if (h.piece == NOTHING) { continue; } let mine = (pieces[h.piece].flags & MESH) != 0u && mrowU(pieces[h.piece].planes + 2u).y == cls; if (!mine && cls != 0u) { continue; }      /* (the ground that shows between stones is part of their floor) */
      F[32u + side] += 1.0;
      var q = surfaceAt(ro, -V, h, px); if (dot(q.n, V) < 0.03) { q.n = normalize(q.n + V * (0.05 - dot(q.n, V))); } let x = select(ro - V * h.t, q.x, fine); let m = lum(q.albedo) / PI; meshFoot = select(lostK.rays, 0.0, fine);
      for (var k = 0u; k < 15u; k++) {
        let el = array<f32, 5>(0.07, 0.208, 0.423, 0.707, 0.966)[k / 3u]; let az = turn + f32(k % 3u) * 1.5707963 * select(-1.0, 1.0, (s & 1u) == 0u); let L = coneDir(vec3f(cos(az) * sqrt(1.0 - el * el), el, sin(az) * sqrt(1.0 - el * el)), frame.sunDir.w, rnd2()); let cs = dot(q.n, L); if (cs <= 0.0) { continue; }
        foot(0.0, 0.0); if (!fine) { footSharp(px, 2.0 * frame.sunDir.w); } F[side * 16u + k] += (m * (1.0 - lum(fresnel(coatF0(q), max(dot(q.n, V), 0.02)))) * cs * lum(roughMatte(q.kind, q.albedo, q.n, V, L)) + lum(coatOf(q, V, L, 0.005))) * lum(visible(x + q.n * select(0.003 + 0.166 * px, 0.002, fine), L, 1e-3, NEAR));
      }
      // from all round: the sky, all alike, and what stands near, which sees less of it itself. (What a wide ray is told of a surface is the mean of a patch of it)
      let d = cosineDir(q.n, rnd2()); foot(0.0, 0.0); if (!fine) { foot(2.0 * px, 0.5); } let o = x + q.n * select(0.004 + 0.22 * px, 0.002, fine); let w = nearest(o, d, 1e-3, NEAR, 0u, 1e9); var e = select(0.0, 1.0, d.y > 0.15);      /* (low down something else stands: walls, a hill) */
      if (w.piece != NOTHING) { let b = surfaceMean(o, d, w); e = lum(b.albedo) * 0.85; if (fine) { foot(0.0, 0.0); e = lum(b.albedo) * mix(0.3, 1.0, lum(visible(b.x + b.ng * 0.002, cosineDir(b.ng, rnd2()), 1e-3, NEAR))); } }
      F[side * 16u + 15u] += m * e * lum(matteSeen(q.kind, q.albedo, max(dot(q.n, V), 0.02))); meshFoot = 0.0;
    }
  }
  for (var i = 0u; i < 9u; i++) { sums[(c * 64u + li) * 9u + i] = vec4f(F[i * 4u], F[i * 4u + 1u], F[i * 4u + 2u], F[i * 4u + 3u]); }
}
