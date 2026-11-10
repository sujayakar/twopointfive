// Each entry whose turn it is sends one ray over the half-sphere above it. What it meets is a place of its own.

fn run(slot: u32) {
  var out: HitY;
  out.distance = NO_RAY;
  let p = cache_points[slot];
  if (!its_turn(slot, p, cache_before[slot])) { cache_hits[slot] = out; return; }
  let at = vec2u(slot, 7u);
  let way = wide_ray(p.position, p.normal,
    dice_of(DICE_KIND_FILLING, at, frame.cache_round, 700u), dice_of(DICE_KIND_FILLING, at, frame.cache_round, 701u));
  if (way.weight > 0.0) {
    let start = leave_surface(p.position, p.normal, way.toward);
    let ray = Ray(start, way.toward, 1e-4, 1e4);
    let met = nearest(ray);
    out = y_of(ray, met, way.weight);
    if (slot == pinned_slot()) { draw_ray(frame.pinned, start, start + way.toward * min(met.distance, 6.0), RAY_CACHE); }
  }
  cache_hits[slot] = out;
}
