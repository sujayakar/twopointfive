// A small grid's answer for a render pixel: from the four small pixels round it, those count that stand for
// the same surface.

fn run(pixel: vec2u) {
  let step = frame.GRID_step;
  let hit = hit_x_at(vec2i(pixel));
  let at = (vec2f(pixel) - floor((f32(step) - 1.0) * 0.5)) / f32(step);
  let base = vec2i(floor(at));
  let f = at - vec2f(base);
  if (knobs.signal_up == SIGNAL_UP_NEAREST || hit.distance < 0.0) {
    put_large(pixel, Signal(small_at(vec2i(round(at))).value, 0.0));
    return;
  }
  var sum = vec3f(0.0);
  var weights = 0.0;
  for (var k = 0; k < 4; k++) {
    let corner = vec2i(k & 1, k >> 1);
    let q = base + corner;
    let side = select(1.0 - f, f, corner == vec2i(1));
    var w = side.x * side.y;
    if (knobs.signal_up == SIGNAL_UP_GUIDED) {
      let h = hit_x_at(render_pixel(q, step));
      let off = abs(dot(h.position - hit.position, hit.normal)) / (0.004 * hit.distance);
      let alike = pow(max(dot(h.normal, hit.normal), 0.0), 16.0) * exp(-off);
      w = (w + 0.02) * select(0.0, alike, h.distance >= 0.0) + 1e-6;
    }
    sum += small_at(q).value * w;
    weights += w;
  }
  put_large(pixel, Signal(sum / weights, 0.0));
}
