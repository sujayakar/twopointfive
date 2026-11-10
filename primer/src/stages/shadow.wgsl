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
  let ray = Ray(start, aim.toward, 1e-4, aim.distance - 2e-3);
  let met = nearest(ray);
  let seen = select(0.0, 1.0, met.piece == NOTHING);
  draw_ray(pixel, start, start + aim.toward * min(met.distance, 20.0), select(RAY_BLOCKED, RAY_SEEN, seen > 0.0));
  r.seen = seen;
  put_reservoir(pixel, r);

  out.distance = min(met.distance, aim.distance);
  if (knobs.denoise_what == DENOISE_WHAT_SHARE) {
    out.value = vec3f(seen);
  } else {
    out.value = landing(r.lamp, r.dice, hit.position, hit.normal) * (seen * r.weight);
  }
  put_raw(pixel, out);
}
