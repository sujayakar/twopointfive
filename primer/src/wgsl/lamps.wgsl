// LAMPS. Everything that shines is a surface with a size: a ball, a panel, the sun's disc. Each can be
// asked three things: where to aim, what the chance of that aim was, and what would land here from all
// of it with nothing in the way (exact, no dice).

struct Aim {
  toward: vec3f,
  distance: f32,
  light: vec3f, // radiance that way; zero when the lamp does not shine this way
  chance: f32,  // per unit of solid angle
}

fn around(axis: vec3f, one_minus_cos: f32, dice: vec2f) -> vec3f {
  let c = 1.0 - dice.x * one_minus_cos;
  let s = sqrt(max(0.0, 1.0 - c * c));
  let phi = 2.0 * PI * dice.y;
  return frame_around(axis) * vec3f(s * cos(phi), s * sin(phi), c);
}

/// 1 − cos of a ball's half angle, seen from `distance`
fn cap_of(radius: f32, distance: f32) -> f32 {
  let s2 = min(1.0, (radius * radius) / (distance * distance));
  return s2 / (1.0 + sqrt(1.0 - s2));
}

fn aim_at(lamp: Lamp, origin: vec3f, dice: vec2f) -> Aim {
  if (lamp.kind == LAMP_SUN) {
    let cap = lamp.radius * lamp.radius * 0.5;
    return Aim(around(lamp.normal, cap, dice), 1e4, lamp.radiance, 1.0 / (2.0 * PI * cap));
  }
  let to_center = lamp.center - origin;
  let d = length(to_center);
  if (lamp.kind == LAMP_BALL) {
    let cap = cap_of(lamp.radius, d);
    let toward = around(to_center / d, cap, dice);
    let along = dot(toward, to_center);
    let reach = along - sqrt(max(0.0, lamp.radius * lamp.radius - (d * d - along * along)));
    return Aim(toward, reach, lamp.radiance, 1.0 / (2.0 * PI * max(cap, 1e-9)));
  }
  let at = to_center + lamp.half_u * (2.0 * dice.x - 1.0) + lamp.half_v * (2.0 * dice.y - 1.0);
  let distance = length(at);
  let toward = at / distance;
  let facing = -dot(toward, lamp.normal);
  if (facing <= 1e-5) { return Aim(toward, distance, vec3f(0.0), 1.0); }
  return Aim(toward, distance, lamp.radiance, distance * distance / (facing * lamp.area));
}

/// The chance that `aim_at` would have picked this direction, known to hit the lamp at `distance`.
fn aim_chance(lamp: Lamp, origin: vec3f, toward: vec3f, distance: f32) -> f32 {
  if (lamp.kind == LAMP_SUN) { return 1.0 / (PI * lamp.radius * lamp.radius); }
  if (lamp.kind == LAMP_BALL) {
    return 1.0 / (2.0 * PI * max(cap_of(lamp.radius, length(lamp.center - origin)), 1e-9));
  }
  let facing = max(-dot(toward, lamp.normal), 1e-5);
  return distance * distance / (facing * lamp.area);
}

/// Share of the half-sphere above a surface that a ball fills, weighed by the cosine; the part of it
/// under the horizon left out (Snyder's formula).
fn ball_share(cosine: f32, s2: f32) -> f32 {
  if (cosine * cosine > s2) { return s2 * max(cosine, 0.0); }
  let sine = sqrt(max(1e-6, 1.0 - cosine * cosine));
  let x = sqrt(max(0.0, 1.0 / s2 - 1.0));
  let y = clamp(-x * cosine / sine, -1.0, 1.0);
  let root = sine * sqrt(max(0.0, 1.0 - y * y));
  return max(0.0, ((cosine * acos(y) - x * root) * s2 + atan2(root, x)) / PI);
}

fn edge_of(a: vec3f, b: vec3f) -> vec3f {
  let c = cross(a, b);
  let l = length(c);
  if (l < 1e-9) { return vec3f(0.0); }
  return c * (acos(clamp(dot(a, b), -1.0, 1.0)) / l);
}

/// The same for a panel: Lambert's formula, the part under the horizon cut off first.
fn panel_share(lamp: Lamp, origin: vec3f, normal: vec3f) -> f32 {
  if (dot(lamp.normal, origin - lamp.center) <= 0.0) { return 0.0; }
  var corners = array<vec3f, 4>(
    lamp.center - lamp.half_u - lamp.half_v - origin,
    lamp.center + lamp.half_u - lamp.half_v - origin,
    lamp.center + lamp.half_u + lamp.half_v - origin,
    lamp.center - lamp.half_u + lamp.half_v - origin,
  );
  var kept: array<vec3f, 8>;
  var count = 0u;
  for (var i = 0u; i < 4u; i++) {
    let a = corners[i];
    let b = corners[(i + 1u) & 3u];
    let ha = dot(a, normal);
    let hb = dot(b, normal);
    if (ha >= 0.0) { kept[count] = a; count++; }
    if ((ha >= 0.0) != (hb >= 0.0)) { kept[count] = a + (b - a) * (ha / (ha - hb)); count++; }
  }
  if (count < 3u) { return 0.0; }
  var sum = vec3f(0.0);
  for (var i = 0u; i < count; i++) {
    let next = select(i + 1u, 0u, i + 1u == count);
    sum += edge_of(normalize(kept[i]), normalize(kept[next]));
  }
  return abs(dot(sum, normal)) / (2.0 * PI);
}

/// What lands on a matte surface from a whole lamp with nothing in the way, over π. Exact.
fn light_unblocked(lamp: Lamp, origin: vec3f, normal: vec3f) -> vec3f {
  if (lamp.kind == LAMP_SUN) {
    return lamp.radiance * (lamp.radius * lamp.radius * max(dot(normal, lamp.normal), 0.0));
  }
  if (lamp.kind == LAMP_BALL) {
    let to = lamp.center - origin;
    let d2 = dot(to, to);
    let s2 = min(0.9999, lamp.radius * lamp.radius / d2);
    return lamp.radiance * ball_share(dot(normal, to) / sqrt(d2), s2);
  }
  return lamp.radiance * panel_share(lamp, origin, normal);
}

struct Pick {
  lamp: u32,   // NOTHING: no lamp can reach this
  chance: f32,
  total: f32,  // the sum of all scores
}

/// How much a lamp matters here: about what it would send unblocked. It need not be exact, only never 0
/// where the lamp can be seen; what is wrong in it costs noise, not truth.
fn score(lamp: Lamp, origin: vec3f, normal: vec3f) -> f32 {
  let bright = brightness(lamp.radiance);
  if (lamp.kind == LAMP_SUN) { return bright * lamp.radius * lamp.radius * max(dot(normal, lamp.normal), 0.0); }
  if (lamp.kind == LAMP_BALL) {
    let to = lamp.center - origin;
    let d2 = dot(to, to);
    // the ball's top may stand over the horizon when its middle does not
    let cosine = dot(normal, to) / sqrt(d2) + lamp.radius / sqrt(d2);
    return bright * min(1.0, lamp.radius * lamp.radius / d2) * max(cosine, 0.0);
  }
  var sum = 0.0;
  for (var i = 0u; i < 4u; i++) { // the middles of its four quarters
    let corner = vec2f(f32(i & 1u), f32(i >> 1u)) - 0.5;
    let to = lamp.center + lamp.half_u * corner.x + lamp.half_v * corner.y - origin;
    let d2 = dot(to, to);
    sum += max(dot(normal, to), 0.0) * max(-dot(lamp.normal, to), 0.0) / (d2 * d2 + 1e-4);
  }
  return bright * sum * lamp.area / (4.0 * PI);
}

/// One lamp, with a chance in proportion to its score. Walks them all once.
fn pick_by_score(origin: vec3f, normal: vec3f, die: f32) -> Pick {
  var total = 0.0;
  var kept = NOTHING;
  var kept_score = 0.0;
  var u = die;
  for (var i = 0u; i < world.lamp_count; i++) {
    let s = score(lamps[i], origin, normal);
    if (s <= 0.0) { continue; }
    total += s;
    let take = s / total;
    // the same die serves again: what is left of it is as even as it was
    if (u < take) { kept = i; kept_score = s; u = u / take; }
    else { u = (u - take) / (1.0 - take); }
  }
  if (kept == NOTHING) { return Pick(NOTHING, 0.0, 0.0); }
  return Pick(kept, kept_score / total, total);
}

/// Is the sun's disc that way?
fn sun_met(toward: vec3f) -> u32 {
  for (var i = 0u; i < world.lamp_count; i++) {
    let l = lamps[i];
    if (l.kind == LAMP_SUN && dot(toward, l.normal) > 1.0 - 0.5 * l.radius * l.radius) { return i; }
  }
  return NOTHING;
}
