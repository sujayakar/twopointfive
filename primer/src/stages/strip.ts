// THE STRIP: one object, asked by rays of every width. A level is right if what it blocks and what it sends back
// match the finest one's mean, so both lines should be flat.

import { f32, struct, vec3f } from '../gpu/layout';
import type { Graph } from '../gpu/graph';

export const STRIP_WIDTHS = 48;
export const STRIP_MOST = 1.2;
const PER_FRAME = 64;
export const Strip = struct('Strip', {
  sends: vec3f('the mean share of light that comes back from it'),
  blocks: f32('the share of rays that it stops', { look: { kind: 'share' } }),
  steps: f32('tree nodes a ray looked at'),
  count: f32('rays so far'),
});

export function addStrip(graph: Graph): void {
  graph.addBuffer({ name: 'strip', struct: Strip, size: { count: STRIP_WIDTHS * PER_FRAME }, about: 'one object, by ray width' });
  graph.addStage({
    name: 'strip',
    question: 'What does one object block and send back, to a ray of each width?',
    reads: [],
    writes: ['strip'],
    uses: ['ray', 'surface', 'dice'],
    over: { count: STRIP_WIDTHS * PER_FRAME },
    code: `
fn run(index: u32) {
  if (world.subject_radius <= 0.0) { return; }
  var s = strip[index];
  // a ray through a ball round the object, from any side, as wide as this bin's
  let bin = index / ${PER_FRAME}u;
  let width = ${STRIP_MOST} * pow(0.004, 1.0 - (f32(bin) + 0.5) / ${STRIP_WIDTHS}.0);
  let a = dice_of(DICE_KIND_WHITE, vec2u(index, 1u), frame.number, 960u);
  let b = dice_of(DICE_KIND_WHITE, vec2u(index, 2u), frame.number, 961u);
  let z = 1.0 - a.x * 1.2; // not from far below: the floor is in the way
  let side = sqrt(max(0.0, 1.0 - z * z));
  let toward = -vec3f(side * cos(6.2832 * a.y), z, side * sin(6.2832 * a.y));
  let axes = frame_around(toward);
  let r = sqrt(b.x) * world.subject_radius;
  let spot = world.subject_center + (axes[0] * cos(6.2832 * b.y) + axes[1] * sin(6.2832 * b.y)) * r;
  var ray = fine_ray(spot - toward * world.subject_radius * 1.5, toward, 0.0, world.subject_radius * 3.0);
  ray.width = width * octave(toward);
  let met = nearest(ray);
  var blocks = 0.0;
  var sends = vec3f(0.0);
  if (met.piece != NOTHING) {
    let found = found_at(ray, met);
    // only what is inside the ball is the object; the floor is not
    if (distance(found.surface.position, world.subject_center) < world.subject_radius && met.normal.y * met.normal.y + found.surface.position.y > 0.02) {
      blocks = 1.0;
      sends = matte_share(found.surface) + coat_share(found.surface);
    }
  }
  s.count += 1.0;
  let k = 1.0 / s.count;
  s.blocks = mix(s.blocks, blocks, k);
  s.sends = mix(s.sends, sends, k);
  s.steps = mix(s.steps, f32(met.steps), k);
  strip[index] = s;
}`,
  });
}
