// THE SUN'S OWN FIRST HITS. For each texel of a square that looks along the sunlight, how far the light gets. It is a
// kept answer to "what is nearest, from the sun?"; a texel's size is its width.

fn run(index: u32) {
  let at = vec2u(index % SUN_MAP, index / SUN_MAP);
  let sun = sun_lamp();
  if (sun == NOTHING) { sun_map[index] = SunDepth(1e9); return; }
  let toward = -lamps[sun].normal;
  let axes = frame_around(lamps[sun].normal);
  let uv = ((vec2f(at) + 0.5) / f32(SUN_MAP) * 2.0 - 1.0) * SUN_REACH;
  let start = world.sun_center + axes[0] * uv.x + axes[1] * uv.y + lamps[sun].normal * 20.0;
  var ray = fine_ray(start, toward, 0.0, 60.0);
  ray.width = 2.0 * SUN_REACH / f32(SUN_MAP) * knobs.sun_map_width;
  if (knobs.sun_by == SUN_BY_LEAVES) { ray.leaves = ONLY_LEAVES; }
  sun_map[index] = SunDepth(nearest(ray).distance);
}
