// The one ray. CLASS: 0 the lamps that stand, 1 the lamps that change

fn run(pixel: vec2u) {
  let hit = hit_x_at(vec2i(pixel));
  var r = picked_at(vec2i(pixel));
  var out: Signal;
  if (r.lamp == NOTHING || r.weight <= 0.0) {
    r.seen = 0.0;
    put_reservoir(pixel, r);
    put_raw(pixel, out);
    return;
  }
  let aim = aim_at(lamps[r.lamp], hit.position, r.dice);
  let start = leave_surface(hit.position, hit.normal, aim.toward);
  var ray = shadow_cone(start, aim.toward, aim.distance - 2e-3, lamp_size(lamps[r.lamp]));
  let is_sun = lamps[r.lamp].kind == LAMP_SUN && knobs.sun_by != SUN_BY_RAYS;
  if (is_sun && knobs.sun_by == SUN_BY_LEAVES) { ray.leaves = NO_LEAVES; }
  if (is_sun && knobs.sun_by == SUN_BY_MAP) { ray.most = 0.0; } // its map is asked instead
  let met = nearest(ray);
  let seen = select(0.0, 1.0, met.piece == NOTHING);
  draw_ray(pixel, start, start + aim.toward * min(met.distance, 20.0), select(RAY_BLOCKED, RAY_SEEN, seen > 0.0));
  var passes = vec3f(1.0);
  if (seen > 0.0 && knobs.break_air_skips == 0u && knobs.air_shadow_steps > 0u) {
    passes = smoke_between(start, aim.toward, aim.distance, 0.0, 0.02, knobs.air_shadow_steps, dice1(pixel, DICE_SHADOW_STEP), false).passes;
  }
  r.seen = seen;
  put_reservoir(pixel, r);

  out.distance = min(met.distance, aim.distance);
  if (knobs.denoise_what == DENOISE_WHAT_SHARE) {
    out.value = vec3f(seen) * passes;
  } else {
    out.value = landing(r.lamp, r.dice, hit) * (seen * r.weight) * passes;
  }
  put_raw(pixel, out);
}
