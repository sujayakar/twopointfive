// Then: what pixels round here keep. Another pixel found its y under its own angle and at its own distance, so the
// chance has to be turned from its half-sphere into this one's.

fn run(pixel: vec2u) {
  let p = vec2i(pixel);
  let hit = hit_x_at(render_pixel(p, frame.wide_step));
  var best = time_at(p);
  if (hit.distance < 0.0 || hit.lamp != NOTHING) { put_gi(pixel, best); return; }
  var total = best.score * best.weight * best.memory;
  var memory = best.memory;
  for (var k = 0u; k < knobs.gi_neighbors; k++) {
    let d = dice_of(DICE_KIND_WHITE, pixel, frame.number, 410u + 5u * k);
    let q = vec2i(vec2f(pixel) + 0.5 + sqrt(d.x) * f32(knobs.gi_radius) * vec2f(cos(6.2832 * d.y), sin(6.2832 * d.y)));
    if (any(q < vec2i(0)) || any(q >= vec2i(frame.wide_size))) { continue; }
    let there = hit_x_at(render_pixel(q, frame.wide_step));
    let off = abs(dot(there.position - hit.position, hit.normal));
    if (there.distance < 0.0 || there.lamp != NOTHING || dot(there.normal, hit.normal) < 0.9 || off > 0.02 * hit.distance) { continue; }
    let other = time_at(q);
    if (other.memory <= 0.0) { continue; }
    let to = other.position - hit.position;
    let far = length(to);
    let cosine = max(dot(to / far, hit.normal), 0.0);
    // the patch of the half-sphere that a bit of y's surface fills, here and there
    let from_there = there.position - other.position;
    let far_there = length(from_there);
    let slant_here = abs(dot(other.normal, to / far));
    let slant_there = abs(dot(other.normal, from_there / far_there));
    var turn = (slant_here / max(slant_there, 1e-3)) * (far_there * far_there) / max(far * far, 1e-4);
    if (turn > 8.0 || turn < 0.125) { continue; } // too unlike to be of use
    if (knobs.break_no_turn != 0u) { turn = 1.0; }
    let s = brightness(other.light) * cosine / PI;
    let weight = s * other.weight * other.memory * turn;
    memory += other.memory;
    total += weight;
    if (weight > 0.0 && dice_of(DICE_KIND_WHITE, pixel, frame.number, 412u + 5u * k).x * total < weight) {
      best = other;
      best.score = s;
    }
    draw_ray_of(pixel, frame.wide_step, hit.position, there.position, RAY_REUSED);
  }
  best.memory = memory;
  best.weight = select(0.0, total / (best.score * memory), best.score > 0.0);
  put_gi(pixel, best);
}
