fn run(pixel: vec2u) {
  let hit = hit_x_at(vec2i(pixel));
  var out: Unblocked;
  if (hit.distance >= 0.0 && hit.lamp == NOTHING) {
    for (var i = 0u; i < world.lamp_count; i++) {
      let light = light_unblocked(lamps[i], hit.position, hit.normal);
      if (brightness(light) <= 0.0) { continue; }
      if (lamps[i].changing != 0u) { out.changing += light; } else { out.still += light; }
      out.lamps += 1.0;
    }
  }
  put_unblocked(pixel, out);
}
