/** ONE LOOK: what comes back along a ray from x, with what it meets lit by what is kept. lookT = how far it went */
var<private> lookT: f32; var<private> lookX: vec3f; var<private> lookN: vec3f; var<private> lookMoves: bool; var<private> lookPiece: u32;
fn look(x: vec3f, d: vec3f, skyBlur: f32) -> vec3f {
  let h = nearest(x, d, 2e-3, 600.0, 0u, 1e9); lookT = h.t; lookPiece = h.piece; let sm = smokeSeen(x, d, min(h.t, 80.0)); let fo = folWalk(x, d, min(h.t, 120.0), true); var L = sm.rgb * fo.T + fo.L;
  if (h.piece == NOTHING) { return L + skyLod(d, skyBlur) * h.thru * fo.T * sm.a; }
  askWide = footAt(h.t); let q = surfaceMean(x, d, h); let leaf = (h.piece & LEAF_HIT) != 0u; let own = !leaf && (pieces[h.piece].lamp & 255u) != 255u; lookX = q.x; lookN = q.ng; lookMoves = (q.flags & MOVES) != 0u;
  return L + (select(q.glow, vec3f(0.0), own) + sentBack(q, leaf, rnd2(), d)) * h.thru * fo.T * sm.a;
}
