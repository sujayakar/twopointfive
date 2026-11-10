// CLASS: 0 the lamps that stand, 1 the lamps that change

fn run(pixel: vec2u) {
  let hit = hit_x_at(vec2i(pixel));
  var g = empty_gathering();
  if (hit.distance < 0.0 || hit.lamp != NOTHING) {
    put_fresh(pixel, weigh(g, 1.0));
    return;
  }
  var of_class = 0u;
  for (var i = 0u; i < world.lamp_count; i++) { of_class += u32(lamps[i].changing == CLASS); }

  let place = dice2(pixel, DICE_LAMP_POINT + 40u * CLASS);
  let die = dice2(pixel, DICE_LAMP_PICK + 40u * CLASS);
  var samples = 1.0;
  if (knobs.lamp_pick == LAMP_PICK_ALL) {
    // every lamp is offered once; what it sends is its weight. One die serves all the way: what is left
    // of it after a choice is as even as it was
    var u = die.x;
    for (var i = 0u; i < world.lamp_count; i++) {
      if (lamps[i].changing != CLASS) { continue; }
      let s = brightness(landing(i, place, hit));
      if (s <= 0.0) { continue; }
      let take = s / (g.total + s);
      if (offer(&g, i, place, s, s, 0.0, u)) { u = u / take; } else { u = (u - take) / (1.0 - take); }
    }
    g.memory = 1.0;
  } else if (of_class > 0u) {
    // some lamps, each drawn with the same chance
    var tries = 1u;
    if (knobs.lamp_pick == LAMP_PICK_FEW) { tries = knobs.lamp_candidates; }
    for (var k = 0u; k < tries; k++) {
      let first = k == 0u;
      let h = hash2(hash2(pixel.x, pixel.y), hash2(frame.number, 77u + k + 100u * CLASS));
      let u = select(unit(h), die.x, first);
      let at = select(vec2f(unit(hash(h)), unit(hash(hash(h)))), place, first);
      var nth = min(u32(u * f32(of_class)), of_class - 1u);
      var lamp = 0u;
      for (var i = 0u; i < world.lamp_count; i++) {
        if (lamps[i].changing != CLASS) { continue; }
        if (nth == 0u) { lamp = i; break; }
        nth--;
      }
      let s = brightness(landing(lamp, at, hit));
      let keep = offer(&g, lamp, at, s, s * f32(of_class), 1.0, unit(hash2(h, 5u)));
      if (keep || k < 24u) { draw_ray(pixel, hit.position, lamps[lamp].center, RAY_CANDIDATE); }
    }
    samples = f32(tries);
  }
  put_fresh(pixel, weigh(g, samples));
}
