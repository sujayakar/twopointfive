// Highlights: light straight from lamps, sent to the eye by the coat. Two ways to find it, and how to use
// both without counting twice.

fn run(pixel: vec2u) {
  let hit = hit_x_at(vec2i(pixel));
  var out: Signal;
  var weights: ShineWays;
  if (hit.distance < 0.0 || hit.lamp != NOTHING) {
    put_shine_raw(pixel, out);
    put_shine_ways(pixel, weights);
    return;
  }
  let s = surface_x(hit);
  let pick = pick_by_score(s.position, s.normal, dice1(pixel, DICE_SHINE_PICK));
  for (var way = 0u; way < 2u; way++) {
    if (way == 0u && (knobs.shine_by == SHINE_BY_LOBE || pick.lamp == NOTHING)) { continue; }
    if (way == 1u && knobs.shine_by == SHINE_BY_AIM) { continue; }
    var toward = coat_draw(s, dice2(pixel, DICE_NARROW));
    var reach = 1e4;
    var light = vec3f(0.0);
    var by_aim = 0.0;
    if (way == 0u) {
      let aim = aim_at(lamps[pick.lamp], s.position, dice2(pixel, DICE_SHINE_POINT));
      toward = aim.toward;
      reach = aim.distance - 2e-3;
      light = aim.light;
      by_aim = pick.chance * aim.chance;
    }
    let by_lobe = coat_chance(s, toward);
    if (dot(s.normal, toward) <= 0.0) { continue; }
    let start = leave_surface(s.position, s.normal, toward);
    let met = nearest(Ray(start, toward, 1e-4, reach));
    if (way == 0u) {
      draw_ray(pixel, start, start + toward * min(met.distance, 20.0), select(RAY_BLOCKED, RAY_SEEN, met.piece == NOTHING));
      if (met.piece != NOTHING) { continue; }
      let w = select(balance(by_aim, by_lobe), 1.0, knobs.shine_by == SHINE_BY_AIM || knobs.break_count_twice != 0u);
      let found = light * coat_response(s, toward) * (w / max(by_aim, 1e-12));
      out.value += found;
      weights.aimed = brightness(found);
      continue;
    }
    // along the lobe: only lamps count here; what else it meets is the mirror's business
    draw_ray(pixel, start, start + toward * min(met.distance, 6.0), RAY_NARROW);
    var lamp = NOTHING;
    if (met.piece == NOTHING) { lamp = sun_met(toward); }
    else {
      let piece = pieces[met.piece];
      if (piece.lamp != NOTHING && (lamps[piece.lamp].kind == LAMP_BALL || dot(met.normal, lamps[piece.lamp].normal) > 0.9)) { lamp = piece.lamp; }
    }
    if (lamp == NOTHING) { continue; }
    let at = lamps[lamp];
    by_aim = 0.0;
    if (pick.total > 0.0) { by_aim = score(at, s.position, s.normal) / pick.total * aim_chance(at, s.position, toward, met.distance); }
    let w = select(balance(by_lobe, by_aim), 1.0, knobs.shine_by == SHINE_BY_LOBE || knobs.break_count_twice != 0u);
    let found = at.radiance * coat_weight(s, toward) * w;
    out.value += found;
    weights.followed = brightness(found);
  }
  put_shine_raw(pixel, out);
  put_shine_ways(pixel, weights);
}
