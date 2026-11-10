// THE FRAME: where the eye is now and was last frame, and how pixels become rays.

fn eye_toward(uv: vec2f) -> vec3f {
  // uv from 0 to 1, y down
  let p = uv * 2.0 - 1.0;
  return normalize(frame.forward + frame.right * p.x - frame.up * p.y);
}

/// The direction of a render pixel's ray, this frame's jitter included.
fn pixel_toward(pixel: vec2u) -> vec3f {
  return eye_toward((vec2f(pixel) + 0.5 + frame.jitter) / vec2f(frame.render_size));
}

struct Seen { uv: vec2f, depth: f32 }

fn seen_from(eye: vec3f, forward: vec3f, right: vec3f, up: vec3f, position: vec3f) -> Seen {
  let v = position - eye;
  let z = dot(v, forward);
  let x = dot(v, right) / (dot(right, right) * z);
  let y = dot(v, up) / (dot(up, up) * z);
  return Seen(vec2f(x, -y) * 0.5 + 0.5, z);
}
/// Where a point is on the screen, 0 to 1.
fn on_screen(position: vec3f) -> Seen {
  return seen_from(frame.eye, frame.forward, frame.right, frame.up, position);
}
/// Where a point was on last frame's screen.
fn on_screen_before(position: vec3f) -> Seen {
  return seen_from(frame.eye_before, frame.forward_before, frame.right_before, frame.up_before, position);
}
