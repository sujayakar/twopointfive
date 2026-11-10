// One ray along the coat's lobe. The same question as a wide ray's, asked of a narrow cone.

fn run(pixel: vec2u) {
  let hit = hit_x_at(render_pixel(vec2i(pixel), frame.narrow_step));
  var out: HitY;
  out.distance = NO_RAY;
  if (hit.distance < 0.0 || hit.lamp != NOTHING) { put_hit_r(pixel, out); return; }
  let s = surface_x(hit);
  let toward = coat_draw(s, dice2(pixel, DICE_NARROW_RAY));
  // what the coat sends of light that is 1 everywhere is known; the ray only brings what the light is
  let weight = brightness(coat_weight(s, toward)) / max(brightness(coat_share(s)), 1e-6);
  if (dot(hit.normal, toward) <= 0.0 || weight <= 0.0) { put_hit_r(pixel, out); return; }
  let start = leave_surface(hit.position, hit.normal, toward);
  let ray = Ray(start, toward, 1e-4, 1e4);
  let met = nearest(ray);
  out = y_of(ray, met, weight);
  draw_ray_of(pixel, frame.narrow_step, start, start + toward * min(met.distance, 6.0), RAY_NARROW);
  put_hit_r(pixel, out);
}
