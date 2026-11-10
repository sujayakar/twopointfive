// THE RADIANCE CACHE, to its readers: which entry speaks for a place.

struct CacheSpot {
  slot: u32,  // NOTHING: no entry speaks for this place
  side: u32,  // which of the two is free to be founded; 2: neither
}

/// The entry that speaks for a place: of the two that its cell may hold, the one that can be seen from it.
/// `nudge` (0 to 1 each way) looks up a little to one side first, so that cells blend into each other as noise
/// and not as blocks; if no entry speaks for that, the place itself is asked.
fn cache_spot_near(position: vec3f, normal: vec3f, nudge: vec2f) -> CacheSpot {
  let axes = frame_around(normal);
  let d = (nudge - 0.5) * knobs.cache_cell;
  let is_nudged = knobs.cache_nudge != 0u && any(nudge != vec2f(0.5));
  var last = CacheSpot(NOTHING, 2u);
  for (var k = select(2u, 0u, is_nudged); k < 4u; k++) { // nudged: its two sides; then itself: its two sides
    let place = position + (axes[0] * d.x + axes[1] * d.y) * select(0.0, 1.0, k < 2u);
    let side = k & 1u;
    let slot = cache_find(cache_key(place, normal, side));
    if (slot == NOTHING) {
      last = CacheSpot(NOTHING, side);
      if (k < 2u) { k = 1u; continue; } // on to the place itself
      return last;
    }
    let there = cache_points[slot];
    if (knobs.cache_wall_test == 0u || dot(there.normal, there.normal) == 0.0) { return CacheSpot(slot, side); }
    let start = position + normal * 0.02;
    let to = there.position + there.normal * 0.02 - start;
    let far = length(to);
    if (far < 0.02 || nearest(fine_ray(start, to / far, 0.0, far)).piece == NOTHING) { return CacheSpot(slot, side); }
  }
  return CacheSpot(NOTHING, 2u);
}
fn cache_spot(position: vec3f, normal: vec3f) -> CacheSpot {
  return cache_spot_near(position, normal, vec2f(0.5));
}
