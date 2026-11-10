// THE MEAN OVER NEIGHBORS, one round (à trous: with holes). Neighbors on the same surface count, as far as
// they are alike next to the doubt. STRIDE: how far apart.

fn hit_pixel(p: vec2i) -> vec2i { return select(p, p * 2, IS_HALF == 1u); }

fn run(pixel: vec2u) {
  let size = vec2i(select(frame.render_size, frame.half_size, IS_HALF == 1u));
  let p = vec2i(pixel);
  let here = smooth_in_at(p);
  let hit = hit_x_at(hit_pixel(p));
  if (hit.distance < 0.0) { put_smooth_out(pixel, here); return; }

  // the doubt is itself uncertain: a little blurred first
  var doubt = 0.0;
  var doubt_weight = 0.0;
  for (var y = -1; y <= 1; y++) {
    for (var x = -1; x <= 1; x++) {
      let w = f32((2 - abs(x)) * (2 - abs(y)));
      doubt += smooth_in_at(p + vec2i(x, y)).doubt * w;
      doubt_weight += w;
    }
  }
  let sigma = sqrt(max(doubt / doubt_weight, 0.0)) * knobs.blur_soft + 1e-5;
  let b = brightness(here.value);

  var sum = here.value * 4.0;
  var sum_doubt = here.doubt * 16.0;
  var weights = 4.0;
  for (var y = -2; y <= 2; y++) {
    for (var x = -2; x <= 2; x++) {
      if (x == 0 && y == 0) { continue; }
      let q = p + vec2i(x, y) * i32(STRIDE);
      if (any(q < vec2i(0)) || any(q >= size)) { continue; }
      let h = hit_x_at(hit_pixel(q));
      if (h.distance < 0.0) { continue; }
      let there = smooth_in_at(q);
      let near = f32((3 - abs(x)) * (3 - abs(y))) * 4.0 / 9.0;
      let facing = pow(max(dot(h.normal, hit.normal), 0.0), f32(knobs.blur_normal));
      let off = abs(dot(h.position - hit.position, hit.normal)) / (0.004 * hit.distance);
      let unlike = abs(brightness(there.value) - b) / sigma;
      let w = near * facing * exp(-off - unlike);
      sum += there.value * w;
      sum_doubt += there.doubt * w * w;
      weights += w;
    }
  }
  put_smooth_out(pixel, Smooth(sum / weights, sum_doubt / (weights * weights)));
}
