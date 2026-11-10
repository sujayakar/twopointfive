// OPENINGS: doors and windows a room names, so that some wide rays can be aimed through them. The chance of
// a direction is then a mixture, and dividing by the mixture keeps the mean right.

fn opening_chance(origin: vec3f, toward: vec3f) -> f32 {
  var chance = 0.0;
  for (var i = 0u; i < world.opening_count; i++) {
    let o = openings[i];
    let n = cross(o.half_u, o.half_v);
    let area = 4.0 * length(n);
    let facing = dot(toward, n) * 4.0 / area;
    if (abs(facing) < 1e-4) { continue; }
    let t = dot(o.center - origin, n) / dot(toward, n);
    if (t <= 0.0) { continue; }
    let spot = origin + toward * t - o.center;
    if (abs(dot(spot, o.half_u)) > dot(o.half_u, o.half_u) || abs(dot(spot, o.half_v)) > dot(o.half_v, o.half_v)) { continue; }
    chance += t * t / (abs(facing) * area * f32(world.opening_count));
  }
  return chance;
}

struct WideRay { toward: vec3f, weight: f32 }

/// A wide ray: by the cosine, or (a share of them) through an opening. `weight` is cosine / π over its chance.
fn wide_ray(origin: vec3f, normal: vec3f, dice: vec2f, way: vec2f) -> WideRay {
  var toward = wide_draw(normal, dice);
  let share = select(0.0, knobs.wide_aim, world.opening_count > 0u);
  if (way.x < share) {
    let o = openings[min(u32(way.y * f32(world.opening_count)), world.opening_count - 1u)];
    toward = normalize(o.center + o.half_u * (2.0 * dice.x - 1.0) + o.half_v * (2.0 * dice.y - 1.0) - origin);
  }
  let by_cosine = wide_chance(normal, toward);
  if (by_cosine <= 0.0) { return WideRay(toward, 0.0); }
  if (share == 0.0) { return WideRay(toward, 1.0); }
  return WideRay(toward, by_cosine / mix(by_cosine, opening_chance(origin, toward), share));
}
