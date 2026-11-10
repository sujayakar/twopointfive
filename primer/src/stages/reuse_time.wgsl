// What this spot kept last frame, merged with what this frame found. Two reservoirs merge as one candidate
// joins: what the other kept is offered, with the weight of all that stands behind it.

fn run(pixel: vec2u) {
  let hit = hit_x_at(vec2i(pixel));
  let fresh = fresh_at(vec2i(pixel));
  if (hit.distance < 0.0 || hit.lamp != NOTHING || knobs.reuse_time == 0u) {
    put_merged(pixel, fresh);
    return;
  }
  var g = empty_gathering();
  let fresh_memory = max(fresh.memory, 1.0);
  offer(&g, fresh.lamp, fresh.dice, fresh.score, fresh.score * fresh.weight * fresh_memory, fresh_memory, 0.0);
  var samples = fresh_memory;

  // the pixel that looked at this spot last frame
  let at = vec2i(floor(on_screen_before(hit.position_before).uv * vec2f(frame.render_size) - frame.jitter_before));
  if (all(at >= vec2i(0)) && all(at < vec2i(frame.render_size))) {
    let old_hit = hit_x_before_at(at);
    let old = old_at(at);
    let memory = min(old.memory, f32(knobs.reuse_memory) * fresh_memory);
    if (memory > 0.0 && same_surface(hit, hit.position_before, old_hit)) {
      var s = 0.0;
      if (old.lamp < world.lamp_count) { s = brightness(landing(old.lamp, old.dice, hit.position, hit.normal)); }
      // a lamp that was hidden from here is known to send nothing
      let known = select(1.0, old.seen, knobs.reuse_seen != 0u);
      let die = dice_of(DICE_KIND_WHITE, pixel, frame.number, 300u + 200u * CLASS).x;
      if (offer(&g, old.lamp, old.dice, s, s * old.weight * memory * known, memory, die)) { g.age = old.age + 1.0; }
      samples += memory;
    }
  }
  put_merged(pixel, weigh(g, samples));
}
