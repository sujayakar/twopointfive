// One ray a pixel of this grid, over the half-sphere above x. What it meets is y.

fn run(pixel: vec2u) {
  let hit = hit_x_at(render_pixel(vec2i(pixel), frame.wide_step));
  var out: HitY;
  out.distance = NO_RAY;
  if (hit.distance < 0.0 || hit.lamp != NOTHING) { put_hit_y(pixel, out); return; }
  // something thin is lit from both sides: the ray goes out on one, as often as that side counts
  let front = brightness(front_share(hit));
  let side = select(-hit.normal, hit.normal, front >= 1.0 || dice1(pixel, DICE_SIDE) < front);
  let way = wide_ray(hit.position, side, dice2(pixel, DICE_WIDE), dice2(pixel, DICE_WIDE_WAY));
  if (way.weight <= 0.0) { put_hit_y(pixel, out); return; }
  let start = leave_surface(hit.position, hit.normal, way.toward);
  let ray = wide_cone(start, way.toward);
  let met = nearest(ray);
  out = y_of(ray, met, way.weight);
  draw_ray_of(pixel, frame.wide_step, start, start + way.toward * min(met.distance, 6.0), RAY_WIDE);
  put_hit_y(pixel, out);
}
