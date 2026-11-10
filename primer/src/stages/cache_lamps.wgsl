// z → light. Each entry whose turn it is asks one lamp, from its own point: the same question as y → light,
// but from a place that will be there next frame too.

fn run(slot: u32) {
  var out: LightAtY;
  out.seen = -1.0; // not asked
  let p = cache_points[slot];
  if (!its_turn(slot, p, cache_before[slot])) { cache_direct[slot] = out; return; }
  let at = vec2u(slot, 9u);
  out.seen = 0.0;
  let pick = pick_by_score(p.position, p.normal, dice_of(DICE_KIND_FILLING, at, frame.cache_round, 702u).x);
  if (pick.lamp != NOTHING) {
    out.lamp = pick.lamp;
    let aim = aim_at(lamps[pick.lamp], p.position, dice_of(DICE_KIND_FILLING, at, frame.cache_round, 703u));
    let cosine = dot(p.normal, aim.toward);
    if (cosine > 0.0 && aim.chance > 0.0) {
      let start = leave_surface(p.position, p.normal, aim.toward);
      let met = nearest(Ray(start, aim.toward, 1e-4, aim.distance - 2e-3));
      out.seen = select(0.0, 1.0, met.piece == NOTHING);
      out.lands = aim.light * (out.seen * cosine / (PI * pick.chance * aim.chance));
      if (slot == pinned_slot()) { draw_ray(frame.pinned, start, start + aim.toward * min(met.distance, 20.0), select(RAY_BLOCKED, RAY_SEEN, out.seen > 0.0)); }
    }
  }
  cache_direct[slot] = out;
}
