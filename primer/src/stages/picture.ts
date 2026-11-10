// THE SUM, of the truth's parts. Light adds, so each part has a gain of its own: the mixer.

import { struct, vec3f } from '../gpu/layout';
import type { Graph } from '../gpu/graph';
import { slider } from '../page/knobs';

export const Light = struct('Light', { light: vec3f('what goes to the eye') });

const gain = (key: string, label: string, about: string) =>
  slider(key, { label, about, min: 0, max: 2, step: 0.01, value: 1 });
export const MIXER = [
  gain('mix_glow', 'Glow', 'Lamps and sky, looked at directly.'),
  gain('mix_still', 'Lamps that stand', 'Straight from lamps that stand, sent back by the matte lobe.'),
  gain('mix_changing', 'Lamps that change', 'The same, of lamps that are switched, swung or carried.'),
  gain('mix_shine', 'Highlights', 'Straight from any lamp, sent back by the coat.'),
  gain('mix_sky', 'Sky', 'Straight from the sky, sent back by the matte lobe.'),
  gain('mix_one', 'One bounce', 'Light that has met one other surface.'),
  gain('mix_two', 'Two bounces', 'Light that has met two.'),
  gain('mix_more', 'More bounces', 'All the rest.'),
  gain('mix_narrow', 'Mirror', 'All light but the lamps’ own, sent back by the coat.'),
];

export const SURFACE_OF_HIT = `
/// x, as a surface
fn surface_x(hit: HitX, pixel: vec2u) -> Surface {
  return Surface(hit.position, hit.normal, normalize(frame.eye - hit.position), hit.matte, hit.through, hit.roughness, hit.metal);
}`;

export function addPicture(graph: Graph): void {
  graph.addBuffer({ name: 'true_picture', struct: Light, size: 'render', about: 'the sum of the truth’s parts' });
  graph.addStage({
    name: 'true picture',
    question: 'What does the eye see, if every part is true?',
    reads: ['hit_x', 'truth'],
    writes: ['true_picture'],
    uses: ['material'],
    over: 'render',
    code: `${SURFACE_OF_HIT}
fn run(pixel: vec2u) {
  let hit = hit_x_at(vec2i(pixel));
  var sum = hit.glow * knobs.mix_glow;
  if (hit.distance >= 0.0 && hit.lamp == NOTHING) {
    let t = truth_at(vec2i(pixel));
    let s = surface_x(hit, pixel);
    let lands = t.still * knobs.mix_still + t.changing * knobs.mix_changing + t.sky * knobs.mix_sky
      + t.one * knobs.mix_one + t.two * knobs.mix_two + t.more * knobs.mix_more;
    sum += matte_share(s) * lands + t.shine * knobs.mix_shine + t.narrow * knobs.mix_narrow;
  }
  put_true_picture(pixel, Light(sum));
}`,
  });
}
