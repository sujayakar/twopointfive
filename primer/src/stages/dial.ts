// THE DIAL: what one object costs a ray, in each form. A great many rays through a ball round one stone, and the
// time of each form's own pass. For a feel of where the forms cross over; not a benchmark.

import type { Graph } from '../gpu/graph';
import { f32, struct } from '../gpu/layout';
import { slider, toggle } from '../page/knobs';

export const DIAL_RAYS = 1 << 17;
const Asked = struct('Asked', { distance: f32(''), steps: f32('') });

export interface DialForm {
  id: string;
  name: string;
  about: string;
}
export const DIAL_FORMS: DialForm[] = [
  { id: 'fine', name: 'Fine triangles', about: 'A tree over 5,120 triangles.' },
  { id: 'coarse', name: 'Coarse triangles', about: 'A tree over the 80 of its coarsest cut.' },
  { id: 'piece', name: 'One ball', about: 'A quadric cut by a box: one test.' },
  { id: 'weight', name: 'One ball, triangles in the shader', about: 'The same test; the triangle code is there and never runs.' },
  { id: 'cells', name: 'Cells', about: 'A grid of 12 × 12 × 12 cells, each stopping a share of rays: one step a cell.' },
];

const BODY: Record<string, string> = {
  fine: 'let t = nearest_triangle(world.dial_fine, o, d, 0.0, 1e4); out = Asked(t.distance, f32(t.steps));',
  coarse: 'let t = nearest_triangle(world.dial_coarse, o, d, 0.0, 1e4); out = Asked(t.distance, f32(t.steps));',
  piece: 'let s = one_ball(o, d); if (s.enter <= s.leave) { out = Asked(s.enter, 1.0); }',
  weight: `let s = one_ball(o, d); if (s.enter <= s.leave) { out = Asked(s.enter, 1.0); }
  if (frame.number == 0xfffffffeu) { let t = nearest_triangle(world.dial_fine, o, d, 0.0, 1e4); out = Asked(t.distance, f32(t.steps)); }`,
  cells: `
  // from cell to cell, along whichever axis comes first
  let half = world.dial_half * 1.05;
  let stay = stay_in_box(o, d, half);
  if (stay.enter <= stay.leave) {
    let size = 2.0 * half / 12.0;
    var t = max(stay.enter, 0.0) + 1e-5;
    let safe = select(d, vec3f(1e-9), abs(d) < vec3f(1e-9));
    for (var k = 0u; k < 40u; k++) {
      let p = o + d * t;
      let cell = vec3i(floor((p + half) / size));
      if (any(cell < vec3i(0)) || any(cell >= vec3i(12)) || t > stay.leave) { break; }
      let middle = (vec3f(cell) + 0.5) * size - half;
      // how full: by how deep the cell's middle lies in the ball that stands for the stone
      let depth = 1.0 - length(middle / world.dial_half);
      let cover = clamp(depth * 4.0 + 0.5, 0.0, 1.0);
      out.steps += 1.0;
      if (unit(hash2(index, hash2(k, frame.number))) < cover) { out.distance = t; break; }
      let edge = (vec3f(cell) + select(vec3f(0.0), vec3f(1.0), safe > vec3f(0.0))) * size - half;
      let next = (edge - o) / safe;
      t = min(next.x, min(next.y, next.z)) + 1e-5;
    }
  }`,
};

export function addDial(graph: Graph): void {
  toggle('dial_coherent', { label: 'Rays side by side', about: 'Neighbors in the list go almost the same way, as a camera’s do. Off: every ray its own way, as a bounce’s do.' });
  slider('dial_repeat', { label: 'Each ray asked again', min: 1, max: 8, step: 1, value: 1, unit: 'times', about: 'To make small times large enough to measure.' });
  for (const { id, name } of DIAL_FORMS) {
    graph.addBuffer({ name: `dial ${id}`, struct: Asked, size: { count: DIAL_RAYS }, about: `what rays found of ${name.toLowerCase()}` });
    graph.addStage({
      name: `dial ${id}`,
      question: `What does a ray cost, when the stone is ${name.toLowerCase()}?`,
      reads: [],
      writes: { asked: `dial ${id}` },
      uses: [id === 'piece' || id === 'cells' ? 'ray of pieces' : 'ray of triangles'],
      over: { count: DIAL_RAYS },
      code: `
fn one_ball(o: vec3f, d: vec3f) -> Stay {
  return overlap(stay_in_box(o, d, world.dial_half), stay_in_round(SHAPE_BALL, o, d, world.dial_half));
}
fn run(index: u32) {
  var total = Asked(0.0, 0.0);
  for (var again = 0u; again < knobs.dial_repeat; again++) {
    // where from and which way
    let group = select(index, index / 4096u, knobs.dial_coherent != 0u);
    let h = hash2(group, hash2(frame.number, again));
    let z = 1.0 - 2.0 * unit(h);
    let phi = 6.2832 * unit(hash(h));
    let side = sqrt(max(0.0, 1.0 - z * z));
    let way = vec3f(side * cos(phi), z, side * sin(phi));
    let axes = frame_around(way);
    let g = select(hash(h + 7u), reverseBits(index & 4095u) ^ (index << 12u), knobs.dial_coherent != 0u);
    let spot = vec2f(unit(g) * 2.0 - 1.0, f32(index & 63u) / 31.5 - 1.0) * 0.62;
    let o = axes * vec3f(spot, 0.0) - way * 2.0;
    let d = way;
    var out = Asked(1e4, 0.0);
    ${BODY[id]}
    total.distance += out.distance;
    total.steps += out.steps;
  }
  asked[index] = total;
}`,
    });
  }
}
