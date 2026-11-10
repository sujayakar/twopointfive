// Which clusters are drawn? Each is asked the same question, by itself: are you fine enough for where the eye is,
// and is what would take your place not? So the clusters that say yes cover each mesh once, without a crack.

fn limit_at(center: vec3f, radius: f32) -> f32 {
  // the error that may be seen, in meters, at the nearest point of the ball
  let d = max(distance(frame.cut_eye, center) - radius, 0.01);
  return max(d * width_of_pixel() * knobs.mesh_pixels, knobs.forced_width * 0.5);
}

fn run(index: u32) {
  if (index >= world.mesh_jobs) { return; }
  let job = mesh_jobs[index];
  let c = mesh_clusters[job.cluster];
  let m = mesh_instances[job.instance];
  let center = m.position + turn_by(m.turn, c.center * m.scale);
  let parent = m.position + turn_by(m.turn, c.parent_center * m.scale);
  if (c.error * m.scale > limit_at(center, c.radius * m.scale)) { return; }
  if (c.parent_error * m.scale <= limit_at(parent, c.parent_radius * m.scale)) { return; }
  if (knobs.mesh_cull != 0u) {
    // out of the picture: behind the eye, or past one of its four sides
    let v = center - frame.cut_eye;
    let z = dot(v, frame.forward);
    let r = c.radius * m.scale;
    let side = length(frame.right);
    let high = length(frame.up);
    let x = abs(dot(v, frame.right)) / side;
    let y = abs(dot(v, frame.up)) / high;
    if (z < -r || x > z * side + r * sqrt(1.0 + side * side) || y > z * high + r * sqrt(1.0 + high * high)) { return; }
  }
  let i = atomicAdd(&mesh_args[1], 1u);
  mesh_picked[i] = job;
}
