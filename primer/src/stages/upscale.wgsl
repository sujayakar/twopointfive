// The picture at output size. Over time: each frame's rays go through other spots of their pixels, and a mean
// is kept, for each output pixel, of the samples that fell near its middle.

/// bright samples count for a little less, or one of them flickers
fn calm(c: vec3f) -> f32 { return 1.0 / (1.0 + brightness(c) * exp2(knobs.exposure)); }

fn run(pixel: vec2u) {
  let uv = (vec2f(pixel) + 0.5) / vec2f(frame.output_size);
  let at = uv * vec2f(frame.render_size) - 0.5 - frame.jitter; // in render pixels, where their samples are
  let base = vec2i(floor(at + 0.5));
  var out: Upscaled;
  if (knobs.upscale_by == UPSCALE_BY_NEAREST) {
    out.color = picture_at(base).light;
    out.weight = 1.0;
    put_upscaled(pixel, out);
    return;
  }
  if (knobs.upscale_by == UPSCALE_BY_LINEAR) {
    let low = vec2i(floor(at));
    let f = at - vec2f(low);
    let top = mix(picture_at(low).light, picture_at(low + vec2i(1, 0)).light, f.x);
    let bottom = mix(picture_at(low + vec2i(0, 1)).light, picture_at(low + vec2i(1, 1)).light, f.x);
    out.color = mix(top, bottom, f.y);
    out.weight = 1.0;
    put_upscaled(pixel, out);
    return;
  }

  // this frame's samples near the middle of the output pixel
  let scale = vec2f(frame.output_size) / vec2f(frame.render_size);
  var sum = vec3f(0.0);
  var weights = 0.0;
  var least = vec3f(1e30);
  var most = vec3f(-1e30);
  var nearest_pixel = base;
  var nearest_distance = 1e30;
  for (var y = -1; y <= 1; y++) {
    for (var x = -1; x <= 1; x++) {
      let q = base + vec2i(x, y);
      let c = picture_at(q).light;
      let off = (vec2f(q) - at) * scale; // in output pixels
      let w = exp(-dot(off, off) * 2.0) * calm(c);
      sum += c * w;
      weights += w;
      least = min(least, c);
      most = max(most, c);
      let h = hit_x_at(q);
      if (h.distance >= 0.0 && h.distance < nearest_distance) { nearest_distance = h.distance; nearest_pixel = q; }
    }
  }
  let now = sum / max(weights, 1e-8);
  // how much this frame knows: 1 when a sample fell in the pixel's middle, little when none did
  let knows = clamp(exp(-dot((vec2f(base) - at) * scale, (vec2f(base) - at) * scale) * 2.0), 0.02, 1.0);

  // where was it? By the motion of the nearest thing round here, so that edges move with what is in front
  var motion = vec2f(0.0);
  let h = hit_x_at(nearest_pixel);
  if (h.distance >= 0.0) { motion = on_screen(h.position).uv - on_screen_before(h.position_before).uv; }
  let was = (uv - motion) * vec2f(frame.output_size) - 0.5;
  var old = vec3f(0.0);
  var old_weight = 0.0;
  if (all(was > vec2f(-0.5)) && all(was < vec2f(frame.output_size) - 0.5)) {
    let low = vec2i(floor(was));
    let f = was - vec2f(low);
    var total = 0.0;
    for (var k = 0; k < 4; k++) {
      let corner = vec2i(k & 1, k >> 1);
      let side = select(1.0 - f, f, corner == vec2i(1));
      let w = side.x * side.y;
      let o = upscaled_before_at(low + corner);
      old += o.color * w;
      old_weight += o.weight * w;
      total += w;
    }
    old /= total;
    old_weight /= total;
  }
  if (knobs.upscale_leash != 0u && old_weight > 0.0) {
    let slack = (most - least) * 0.15;
    let held = clamp(old, least - slack, most + slack);
    old_weight *= 1.0 - 0.7 * clamp(length(held - old) / (length(old) + 1e-4), 0.0, 1.0);
    old = held;
  }
  old_weight = min(old_weight, f32(knobs.upscale_frames));
  out.weight = old_weight + knows;
  out.color = mix(old, now, knows / out.weight);
  if (is_bad(out.color.x + out.color.y + out.color.z)) { out.color = now; out.weight = knows; }
  put_upscaled(pixel, out);
}
