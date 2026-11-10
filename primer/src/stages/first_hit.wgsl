fn run(pixel: vec2u) {
  let toward = pixel_toward(pixel);
  let ray = Ray(frame.eye, toward, 0.0, 1e4);
  let met = nearest(ray);

  var hit: HitX;
  hit.steps = f32(met.steps);
  hit.object = NOTHING;
  hit.piece = NOTHING;
  hit.lamp = NOTHING;
  hit.distance = -1.0;
  if (met.piece == NOTHING) {
    hit.position = frame.eye + toward * 1e4;
    hit.position_before = hit.position;
    hit.glow = sky(toward);
    put_hit_x(pixel, hit);
    return;
  }
  let found = found_at(ray, met);
  let s = found.surface;
  hit.position = s.position;
  hit.distance = met.distance;
  hit.normal = s.normal;
  hit.matte = s.matte;
  hit.through = s.through;
  hit.roughness = s.roughness;
  hit.metal = s.metal;
  hit.glow = found.glow;
  hit.lamp = found.lamp;
  hit.piece = met.piece;
  hit.object = pieces[met.piece].object;

  // the same spot of the same object, where it was last frame
  let o = objects[hit.object];
  let own = turn_back(o.turn, s.position - o.position);
  hit.position_before = o.position_before + turn_by(o.turn_before, own);
  put_hit_x(pixel, hit);
}
