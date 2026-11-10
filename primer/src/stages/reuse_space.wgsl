// What pixels round here have found, merged with what this one has.

const MOST_OTHERS = 8u;

fn run(pixel: vec2u) {
  let hit = hit_x_at(vec2i(pixel));
  let own = time_at(vec2i(pixel));
  if (hit.distance < 0.0 || hit.lamp != NOTHING) {
    put_merged(pixel, own);
    return;
  }
  var g = empty_gathering();
  let own_memory = max(own.memory, 1.0);
  offer(&g, own.lamp, own.dice, own.score, own.score * own.weight * own_memory, own_memory, 0.0);
  g.age = own.age;

  var others: array<vec2i, 8>;  // where each reservoir came from
  var behind: array<f32, 8>;    // how many samples stand behind it
  var count = 0u;
  for (var k = 0u; k < min(knobs.reuse_neighbors, MOST_OTHERS); k++) {
    let d = dice_of(DICE_KIND_WHITE, pixel, frame.number, 310u + 10u * k + 200u * CLASS);
    let q = vec2i(vec2f(pixel) + 0.5 + sqrt(d.x) * f32(knobs.reuse_radius) * vec2f(cos(6.2832 * d.y), sin(6.2832 * d.y)));
    if (any(q < vec2i(0)) || any(q >= vec2i(frame.render_size))) { continue; }
    let there = hit_x_at(q);
    if (!same_surface(hit, hit.position, there)) { continue; }
    let other = time_at(q);
    if (other.memory <= 0.0) { continue; }
    var s = 0.0;
    if (other.lamp < world.lamp_count) { s = brightness(landing(other.lamp, other.dice, hit)); }
    let die = dice_of(DICE_KIND_WHITE, pixel, frame.number, 315u + 10u * k + 200u * CLASS).x;
    if (offer(&g, other.lamp, other.dice, s, s * other.weight * other.memory, other.memory, die)) { g.age = other.age; }
    others[count] = q;
    behind[count] = other.memory;
    count++;
    draw_ray(pixel, hit.position, there.position, RAY_REUSED);
  }

  // those who could never have found the sample that was kept must not count in the mean
  var samples = g.memory;
  if (knobs.reuse_correct != 0u && g.lamp != NOTHING) {
    samples = own_memory;
    for (var k = 0u; k < count; k++) {
      let o = hit_x_at(others[k]);
      if (brightness(landing(g.lamp, g.dice, o)) > 0.0) { samples += behind[k]; }
    }
  }
  put_merged(pixel, weigh(g, samples));
}
