// The kept place, seen from here: one ray to make sure that it is.

fn run(pixel: vec2u) {
  let hit = hit_x_at(render_pixel(vec2i(pixel), frame.wide_step));
  let r = picked_at(vec2i(pixel));
  var out: Signal;
  if (hit.distance < 0.0 || hit.lamp != NOTHING || r.memory <= 0.0 || r.weight <= 0.0) { put_raw(pixel, out); return; }
  let to = r.position - hit.position;
  let far = length(to);
  let toward = to / far;
  let cosine = max(dot(toward, hit.normal), 0.0);
  let start = leave_surface(hit.position, hit.normal, toward);
  var seen = 1.0;
  if (knobs.gi_check != 0u) {
    let met = nearest(fine_ray(start, toward, 1e-4, far - 0.02));
    seen = select(0.0, 1.0, met.piece == NOTHING);
  }
  draw_ray_of(pixel, frame.wide_step, start, start + toward * min(far, 6.0), select(RAY_BLOCKED, RAY_WIDE, seen > 0.0));
  out.value = r.light * (cosine / PI * r.weight * seen);
  out.distance = min(far, 60.0);
  put_raw(pixel, out);
}
