// THE DISC. The half-sphere above the pinned point, seen from below and drawn as a disc, so that a bit of
// the disc counts as much as any other of the same size: the plain mean of the disc is the light that lands.

import { f32, struct, vec3f } from '../gpu/layout';
import type { Graph } from '../gpu/graph';
import type { Views } from '../page/views';

export const DISC = 192;
const Disc = struct('Disc', { light: vec3f('what comes from that way'), count: f32('rays so far') });

export function addDisc(graph: Graph, views: Views): void {
  graph.addBuffer({ name: 'disc', struct: Disc, size: { count: DISC * DISC }, about: 'the half-sphere above the pinned point' });
  graph.addStage({
    name: 'disc',
    question: 'What does the pinned point see?',
    reads: ['hit_x'],
    writes: ['disc'],
    uses: ['x', 'ray', 'surface', 'sky', 'lamps', 'dice'],
    over: { count: DISC * DISC },
    code: `
fn run(index: u32) {
  let at = vec2u(index % ${DISC}u, index / ${DISC}u);
  var sum = disc[index];
  let hit = hit_x_at(vec2i(frame.pinned));
  let d = dice_of(DICE_KIND_WHITE, at, frame.number, 900u);
  let spot = (vec2f(at) + d) / ${(DISC / 2).toFixed(1)} - 1.0;
  let r2 = dot(spot, spot);
  if (frame.pinned_on == 0u || hit.distance < 0.0 || r2 >= 1.0) { disc[index] = Disc(vec3f(0.0), 0.0); return; }
  let axes = frame_around(hit.normal);
  var place = hit.position;
  var normal = hit.normal;
  var toward = axes * vec3f(spot.x, spot.y, sqrt(1.0 - r2));
  var reach = 1e4;
  var light = vec3f(0.0);
  var waiting = vec3f(0.0);
  // a ray to what is there; then, from a surface, one to a lamp
  for (var k = 0u; k < 2u; k++) {
    var ray = wide_cone(leave_surface(place, normal, toward), toward);
    ray.most = reach;
    let met = nearest(ray);
    if (k == 1u) {
      if (met.piece == NOTHING) { light += waiting; }
      break;
    }
    if (met.piece == NOTHING) {
      let sun = sun_met(toward);
      light = sky(toward);
      if (sun != NOTHING) { light += lamps[sun].radiance; }
      break;
    }
    let found = found_at(ray, met);
    light = found.glow;
    if (found.lamp != NOTHING) { break; }
    let s = found.surface;
    let pick = pick_by_score(s.position, s.normal, dice_of(DICE_KIND_WHITE, at, frame.number, 901u).x);
    if (pick.lamp == NOTHING) { break; }
    let aim = aim_at(lamps[pick.lamp], s.position, dice_of(DICE_KIND_WHITE, at, frame.number, 902u));
    let cosine = dot(s.normal, aim.toward);
    if (cosine <= 0.0) { break; }
    waiting = matte_share(s) * aim.light * (cosine / (PI * pick.chance * aim.chance));
    place = s.position;
    normal = s.normal;
    toward = aim.toward;
    reach = aim.distance - 2e-3;
  }
  let count = sum.count + 1.0;
  disc[index] = Disc(mix(sum.light, light, 1.0 / count), count);
}`,
  });
  views.add({
    id: 'disc', group: 'The pinned point', name: 'What it sees, as a disc',
    about: 'The half-sphere above the pinned point. Every bit of the disc counts alike: its plain mean is the light that lands. Dots are where this frame’s rays went.',
    reads: ['disc'],
    code: `
  let size = vec2f(frame.output_size);
  let spot = (vec2f(p) + 0.5 - size * 0.5) / (0.48 * min(size.x, size.y));
  if (dot(spot, spot) >= 1.0) { return vec3f(0.02); }
  let at = vec2u((vec2f(spot.x, -spot.y) * 0.5 + 0.5) * ${DISC}.0);
  return show_light(disc[min(at.y, ${DISC - 1}u) * ${DISC}u + min(at.x, ${DISC - 1}u)].light);`,
  });
}
