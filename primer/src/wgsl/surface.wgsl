// WHAT A RAY FOUND, as a surface: its material, with the pattern at that place.

struct Found {
  surface: Surface,
  glow: vec3f, // what it sends by itself
  lamp: u32,   // which lamp it is, if it glows
}

fn pattern_at(m: Material, p: vec3f, normal: vec3f) -> f32 {
  if (m.pattern == PATTERN_PLAIN) { return 0.0; }
  // along the two axes the surface lies in
  let n = abs(normal);
  var uv = p.xz;
  if (n.x > n.y && n.x > n.z) { uv = p.zy; }
  else if (n.z > n.y) { uv = p.xy; }
  let cell = floor(uv / m.pattern_size + 1e-4);
  if (m.pattern == PATTERN_CHECKER) { return abs(cell.x + cell.y) % 2.0; }
  return clamp((cell.x + 4.0) / 7.0, 0.0, 1.0); // strips: eight steps across
}

fn found_at(ray: Ray, met: Meeting) -> Found {
  let piece = pieces[met.piece];
  let m = materials[piece.material];
  var f: Found;
  f.lamp = NOTHING;
  f.surface.position = ray.origin + ray.toward * met.distance;
  f.surface.eye = -ray.toward;
  f.surface.normal = met.normal;
  let object = objects[piece.object];
  let own = turn_back(object.turn, f.surface.position - object.position);
  let own_normal = turn_back(object.turn, met.normal);
  let mixing = pattern_at(m, own, own_normal);
  f.surface.matte = mix(m.matte, m.matte_other, mixing);
  f.surface.roughness = mix(m.roughness, m.roughness_other, mixing);
  f.surface.through = m.through;
  f.surface.metal = m.metal;
  if (piece.lamp != NOTHING) {
    let lamp = lamps[piece.lamp];
    if (lamp.kind == LAMP_BALL || dot(met.normal, lamp.normal) > 0.9) {
      f.lamp = piece.lamp;
      f.glow = lamp.radiance;
      f.surface.matte = vec3f(0.0);
      f.surface.roughness = 1.0;
      f.surface.metal = 1.0; // black: it only shines
    }
  }
  return f;
}
