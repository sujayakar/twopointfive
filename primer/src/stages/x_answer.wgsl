// No ray at all: the cache, read where the eye looks. To see what it holds.

fn run(pixel: vec2u) {
  let hit = hit_x_at(render_pixel(vec2i(pixel), frame.wide_step));
  var out: Signal;
  if (hit.distance >= 0.0 && hit.lamp == NOTHING) {
    let spot = cache_spot_near(hit.position, hit.normal, dice2(pixel, DICE_NUDGE));
    if (spot.slot != NOTHING) { out.value = cache[spot.slot].known * cache[spot.slot].ready; }
  }
  put_raw(pixel, out);
}
