// What comes back along the ray: what y sends, from the lamp that was asked there and from the cache.

fn run(pixel: vec2u) {
  let y = hit_at(vec2i(pixel));
  var out: Signal;
  out.distance = max(y.distance, 0.0);
  var known = vec3f(0.0);
  var direct = vec3f(0.0);
  if (y.distance >= 0.0 && y.lamp == NOTHING && knobs.stop_at != STOP_AT_NOWHERE) {
    let spot = cache_spot_near(y.position, y.normal, dice2(pixel, DICE_NUDGE + DICE_OFFSET));
    if (spot.slot != NOTHING) {
      let e = cache[spot.slot];
      known = e.known * e.ready;
      direct = e.direct;
    }
  }
  if (y.distance == NO_RAY) { out.value = vec3f(0.0); }
  else if (y.distance < 0.0) { out.value = sky(y.toward) * y.weight; }
  else if (y.lamp != NOTHING) {
    // a lamp: x → light has counted it already
    out.value = select(vec3f(0.0), y.glow * y.weight, knobs.break_lamps_twice != 0u);
  }
  else if (knobs.stop_at == STOP_AT_Y) { out.value = y.sends * (direct + known) * y.weight; }
  else { out.value = y.sends * (light_y_at(vec2i(pixel)).lands + known) * y.weight; }
  if (is_bad(out.value.x + out.value.y + out.value.z)) { out.value = vec3f(0.0); }
  put_raw(pixel, out);
}
