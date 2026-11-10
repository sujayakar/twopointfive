// Founding. Where a ray landed and no entry speaks for the place, one is made. Its point is the place where the
// first ray landed.

fn run(index: u32) {
  if (frame.cache_go == 0u) { return; }
  let y = source[index];
  if (y.distance < 0.0 || y.lamp != NOTHING) { return; }
  let spot = cache_spot(y.position, y.normal);
  if (spot.slot != NOTHING || spot.side > 1u) { return; }
  let founding = cache_take(cache_key(y.position, y.normal, spot.side));
  if (!founding.is_new) { return; }
  cache_points[founding.slot] = CachePoint(y.position, f32(frame.cache_round), y.normal, frame.number);
}
