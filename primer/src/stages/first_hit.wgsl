fn run(pixel: vec2u) {
  let toward = pixel_toward(pixel);
  let ray = eye_ray(frame.eye, toward);
  var met = nearest(ray);
  // a mesh that was drawn may be nearer
  let drawn = mesh_hit_at(vec2i(pixel));
  let is_mesh = drawn.distance > 0.0 && drawn.distance < met.distance && knobs.mesh_eye == MESH_EYE_RASTER;

  var hit: HitX;
  hit.steps = f32(met.steps);
  hit.object = NOTHING;
  hit.piece = NOTHING;
  hit.lamp = NOTHING;
  hit.distance = -1.0;
  if (is_mesh) {
    let m = materials[drawn.material];
    hit.position = frame.eye + toward * drawn.distance;
    hit.distance = drawn.distance;
    hit.normal = select(drawn.normal, normalize(drawn.normal - toward * (dot(drawn.normal, toward) + 0.02)), dot(drawn.normal, toward) > -0.02);
    hit.matte = m.matte;
    hit.through = m.through;
    hit.roughness = m.roughness;
    hit.metal = m.metal;
    hit.object = 0u;
    hit.position_before = hit.position;
    hit.trust = trust_at(hit.position);
    put_hit_x(pixel, hit);
    return;
  }
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
  if (knobs.break_no_before != 0u) { hit.position_before = hit.position; }
  hit.trust = trust_at(hit.position);
  put_hit_x(pixel, hit);
}
