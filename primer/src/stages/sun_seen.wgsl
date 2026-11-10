// Where a ray was not sent to the sun, its map is asked: is x as far from the sun as the first thing the light met
// there, or behind it? One texel, drawn by a die from as wide a patch as the sun's disc would blur.

fn run(pixel: vec2u) {
  let hit = hit_x_at(vec2i(pixel));
  var out = ray_raw_at(vec2i(pixel));
  let r = reservoir_at(vec2i(pixel));
  let sun = sun_lamp();
  if (hit.distance < 0.0 || hit.lamp != NOTHING || sun == NOTHING || r.lamp != sun) { put_raw(pixel, out); return; }
  let n = lamps[sun].normal;
  let axes = frame_around(n);
  let place = hit.position + hit.normal * (select(-1.0, 1.0, dot(hit.normal, n) > 0.0) * knobs.sun_map_margin);
  let from_center = place - world.sun_center;
  let depth = 20.0 - dot(from_center, n);
  let spot = vec2f(dot(from_center, axes[0]), dot(from_center, axes[1]));
  // how far up the blocker is says how soft its edge is: the sun is a disc
  let blocker = sun_depth(spot);
  let up = max(depth - blocker, 0.0);
  let d = dice2(pixel, DICE_SUN_MAP);
  let texel = 2.0 * SUN_REACH / f32(SUN_MAP);
  let blur = max(up * lamps[sun].radius, texel * 0.5);
  let seen = select(0.0, 1.0, sun_depth(spot + sqrt(d.x) * blur * vec2f(cos(6.2832 * d.y), sin(6.2832 * d.y))) > depth - 0.002);
  out.value *= seen;
  put_raw(pixel, out);
}

fn sun_depth(spot: vec2f) -> f32 {
  let uv = (spot / SUN_REACH * 0.5 + 0.5) * f32(SUN_MAP);
  let at = vec2i(floor(uv));
  if (any(at < vec2i(0)) || any(at >= vec2i(i32(SUN_MAP)))) { return 1e9; }
  return sun_map[u32(at.y) * SUN_MAP + u32(at.x)].depth;
}
