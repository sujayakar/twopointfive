// A wide ray that found something bright is worth remembering. What is kept is the place y it found, the light y
// sends back, and a weight. First: what this pixel kept last frame, beside what it found now.

fn run(pixel: vec2u) {
  let p = vec2i(pixel);
  let hit = hit_x_at(render_pixel(p, frame.wide_step));
  var out: GiReservoir;
  if (hit.distance < 0.0 || hit.lamp != NOTHING) { put_gi(pixel, out); return; }
  let y = hit_y_at(p);
  if (y.distance != NO_RAY && y.weight > 0.0) {
    // found by the cosine: its chance was cosine / π, so its weight in the mean is 1 over that
    out.light = found_at_y_at(p).value / y.weight;
    out.position = y.position;
    out.normal = y.normal;
    if (y.distance < 0.0) { // the sky: a place very far off that faces us
      out.position = hit.position + y.toward * 1000.0;
      out.normal = -y.toward;
    }
    let chance = max(abs(dot(hit.normal, y.toward)), 1e-4) / PI / y.weight;
    out.score = brightness(out.light) * chance * y.weight;
    out.weight = 1.0 / chance;
    out.memory = 1.0;
  }
  if (knobs.gi_time == 0u) { put_gi(pixel, out); return; }

  // where this spot was looked at last frame
  let step = f32(frame.wide_step);
  let at = (on_screen_before(hit.position_before).uv * vec2f(frame.render_size) - 0.5 - frame.jitter_before - floor((step - 1.0) * 0.5)) / step;
  let q = vec2i(floor(at + 0.5));
  if (any(q < vec2i(0)) || any(q >= vec2i(frame.wide_size))) { put_gi(pixel, out); return; }
  let old_hit = hit_x_before_at(render_pixel(q, frame.wide_step));
  let off = abs(dot(old_hit.position - hit.position_before, hit.normal));
  if (old_hit.distance < 0.0 || dot(old_hit.normal, hit.normal) < 0.9 || off > 0.02 * hit.distance) { put_gi(pixel, out); return; }
  let old = gi_before_at(q);
  let memory = min(old.memory, f32(knobs.gi_memory) * max(hit.trust, 0.1));
  if (memory <= 0.0) { put_gi(pixel, out); return; }
  let to = normalize(old.position - hit.position);
  let cosine = max(dot(to, hit.normal), 0.0);
  let s = brightness(old.light) * cosine / PI;
  let fresh = out.score * out.weight * out.memory;
  let kept = s * old.weight * memory;
  let total = fresh + kept;
  let die = dice_of(DICE_KIND_WHITE, pixel, frame.number, 400u).x;
  var merged = out;
  if (die * total < kept) {
    merged = old;
    merged.score = s;
    merged.age = old.age + 1.0;
  }
  merged.memory = out.memory + memory;
  merged.weight = select(0.0, total / (merged.score * merged.memory), merged.score > 0.0);
  put_gi(pixel, merged);
}
