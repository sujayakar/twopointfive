// RAYS YOU CAN SEE. A stage may draw the rays of the pinned pixel; the page reads them back and draws them
// over every panel, and in the ghost's view they hang in space.

fn draw_ray(pixel: vec2u, start: vec3f, end: vec3f, kind: u32) {
  if (frame.pinned_on == 0u || any(pixel != frame.pinned)) { return; }
  let i = atomicAdd(&drawn.count, 1u);
  if (i >= MOST_LINES) { return; }
  drawn.lines[i] = Line(start, kind, end, 0u);
}
/// the same for stages on a smaller grid, whose pixel covers several
fn draw_ray_of(pixel: vec2u, step: u32, start: vec3f, end: vec3f, kind: u32) {
  if (any(pixel != frame.pinned / step)) { return; }
  draw_ray(frame.pinned, start, end, kind);
}
