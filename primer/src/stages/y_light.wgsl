// y → light. One lamp by its score, one ray. It has no history: y is somewhere else next frame.

fn run(pixel: vec2u) {
  let y = hit_at(vec2i(pixel));
  var out: LightAtY;
  if (y.distance < 0.0 || y.lamp != NOTHING || !wanted()) { put_light_y(pixel, out); return; }
  let pick = pick_by_score(y.position, y.normal, dice1(pixel, DICE_Y_LAMP + DICE_OFFSET));
  if (pick.lamp == NOTHING) { put_light_y(pixel, out); return; }
  out.lamp = pick.lamp;
  let aim = aim_at(lamps[pick.lamp], y.position, dice2(pixel, DICE_Y_POINT + DICE_OFFSET));
  let cosine = dot(y.normal, aim.toward);
  if (cosine <= 0.0 || aim.chance <= 0.0) { put_light_y(pixel, out); return; }
  let start = leave_surface(y.position, y.normal, aim.toward);
  let met = nearest(shadow_cone(start, aim.toward, aim.distance - 2e-3, lamp_size(lamps[pick.lamp])));
  out.seen = select(0.0, 1.0, met.piece == NOTHING);
  let passes = smoke_between(start, aim.toward, aim.distance, 0.0, 0.1, knobs.air_shadow_steps, dice1(pixel, DICE_SHADOW_STEP + DICE_OFFSET), false).passes;
  out.lands = passes * aim.light * (out.seen * cosine / (PI * pick.chance * aim.chance));
  draw_ray_of(pixel, frame.GRID_step, start, start + aim.toward * min(met.distance, 20.0), select(RAY_BLOCKED, RAY_SEEN, out.seen > 0.0));
  put_light_y(pixel, out);
}

/// Is a fresh ray to a lamp wanted, or does the cache answer for lamps too?
fn wanted() -> bool { return knobs.stop_at == STOP_AT_Z || knobs.stop_at == STOP_AT_NOWHERE; }
