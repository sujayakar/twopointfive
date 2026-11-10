// THE MEAN OVER TIME. MOTION, TESTS, LEASH, NO_TIME, IMAGE: the parts this denoiser has.

/// the render pixel that stands for a pixel of this signal
fn hit_pixel(p: vec2i) -> vec2i { return render_pixel(p, frame.GRID_step); }

fn run(pixel: vec2u) {
  let size = vec2i(frame.GRID_size);
  let p = vec2i(pixel);
  let now = noisy_at(p);
  let hit = hit_x_at(hit_pixel(p));
  var out: Kept;
  out.value = now.value;
  out.count = 1.0;
  out.distance = now.distance;
  let b = brightness(now.value);
  out.square = b * b;
  if (hit.distance < 0.0) { put_kept(pixel, out); return; }

  // what this frame says round here: mean and spread
  var sum = vec3f(0.0);
  var squares = vec3f(0.0);
  var n = 0.0;
  for (var y = -2; y <= 2; y++) {
    for (var x = -2; x <= 2; x++) {
      let q = p + vec2i(x, y);
      if (any(q < vec2i(0)) || any(q >= size)) { continue; }
      let h = hit_x_at(hit_pixel(q));
      if (h.distance < 0.0 || dot(h.normal, hit.normal) < 0.8) { continue; }
      let v = noisy_at(q).value;
      sum += v;
      squares += v * v;
      n += 1.0;
    }
  }
  let mean = sum / max(n, 1.0);
  let spread = sqrt(max(squares / max(n, 1.0) - mean * mean, vec3f(0.0)));

  var count = 0.0;
  var old = vec3f(0.0);
  var old_square = 0.0;
  var why = 0.0;
  if (NO_TIME == 0u) {
    // where was this spot last frame?
    var at = vec2f(p);
    if (MOTION == 1u) {
      var spot = hit.position_before;
      if (IMAGE == 1u) {
        // a mirror's image stands behind the surface, as far as what it shows is before it. The rougher,
        // the more it is simply the surface's
        let sharp = 1.0 - smoothstep(0.1, 0.5, hit.roughness);
        let through = normalize(hit.position - frame.eye);
        spot += through * (now.distance * sharp);
      }
      let seen = on_screen_before(spot);
      let step = f32(frame.GRID_step);
      at = (seen.uv * vec2f(frame.render_size) - 0.5 - frame.jitter_before - floor((step - 1.0) * 0.5)) / step;
      if (IMAGE == 0u && MOTION == 1u && all(frame.eye == frame.eye_before) && all(hit.position == hit.position_before) && all(frame.forward == frame.forward_before)) {
        at = vec2f(p); // nothing moved: exactly itself, or rounding would blur it a little every frame
      }
    }
    let base = vec2i(floor(at + 1e-3));
    let f = clamp(at - vec2f(base), vec2f(0.0), vec2f(1.0));
    var weight = 0.0;
    var tried = 0.0;
    for (var k = 0; k < 4; k++) {
      let corner = vec2i(k & 1, k >> 1);
      let q = base + corner;
      let side = select(1.0 - f, f, corner == vec2i(1));
      let w = side.x * side.y;
      if (w < 1e-3 || any(q < vec2i(0)) || any(q >= size)) { continue; }
      tried += w;
      if (TESTS == 1u) {
        let h = hit_x_before_at(hit_pixel(q));
        let off = abs(dot(h.position - hit.position_before, hit.normal));
        let same = h.distance >= 0.0 && h.object == hit.object && dot(h.normal, hit.normal) > 0.85 && off < 0.01 * hit.distance + 0.005;
        if (!same) { continue; }
      }
      let kept = kept_before_at(q);
      if (kept.count <= 0.0) { continue; }
      old += fed(q) * w;
      count += kept.count * w;
      old_square += kept.square * w;
      weight += w;
    }
    if (weight > 0.01) {
      old /= weight;
      count /= weight;
      old_square /= weight;
    } else {
      count = 0.0;
      why = select(2.0, 1.0, tried < 0.01);
    }
  }

  // where something changed, the past counts for less
  count = min(count, mix(2.0, f32(knobs.history_frames) - 1.0, hit.trust));
  count = min(count, f32(knobs.history_frames) - 1.0);
  if (LEASH == 1u && count > 0.0) {
    let reach = spread * knobs.history_leash + abs(mean) * 0.02 + 1e-4;
    let held = clamp(old, mean - reach, mean + reach);
    let moved = length(held - old) / (length(reach) + 1e-6);
    if (moved > 0.02) { why = 3.0; }
    // what had to be pulled back is worth less
    count /= 1.0 + moved;
    old = held;
  }
  let k = 1.0 / (count + 1.0);
  out.value = stored(mix(old, now.value, k), pixel);
  out.square = mix(old_square, b * b, k);
  out.count = count + 1.0;
  out.why = why;

  // the doubt: the spread of single frames, over how many stand behind the mean
  let m = brightness(out.value);
  let over_time = max(out.square - m * m, 0.0);
  let s = brightness(spread);
  let single = mix(s * s, over_time, clamp((out.count - 2.0) / 6.0, 0.0, 1.0));
  out.doubt = single / out.count;
  put_kept(pixel, out);
}

/// As the history's number format keeps it. In 16 bits a mean over many frames stops learning: the step it
/// should take is less than half of the last place.
fn stored(v: vec3f, pixel: vec2u) -> vec3f {
  if (knobs.history_format == HISTORY_FORMAT_F32) { return v; }
  var x = v;
  if (knobs.history_format == HISTORY_FORMAT_DITHERED) {
    let place = exp2(floor(log2(max(abs(v), vec3f(1e-5)))) - 10.0);
    let d = dice_of(DICE_KIND_WHITE, pixel, frame.number, 800u);
    x += place * (vec3f(d.x, d.y, fract(d.x + d.y)) - 0.5);
  }
  return vec3f(unpack2x16float(pack2x16float(x.xy)), unpack2x16float(pack2x16float(x.zz)).x);
}
