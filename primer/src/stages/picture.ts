// THE SUM. Light adds, so each part has a gain of its own: the mixer. Two pictures: one of the truth's
// parts, one of the signals as they are switched.

import { struct, vec3f } from '../gpu/layout';
import type { Graph } from '../gpu/graph';
import type { Views } from '../page/views';
import { slider } from '../page/knobs';

export const Light = struct('Light', { light: vec3f('what goes to the eye') });

const gain = (key: string, label: string, about: string) =>
  slider(key, { label, about, min: 0, max: 2, step: 0.01, value: 1 });
export const MIXER = [
  gain('mix_glow', 'Glow', 'Lamps and sky, looked at directly.'),
  gain('mix_still', 'Lamps that stand', 'Straight from lamps that stand, sent back by the matte lobe.'),
  gain('mix_changing', 'Lamps that change', 'The same, of lamps that are switched, swung or carried.'),
  gain('mix_shine', 'Highlights', 'Straight from any lamp, sent back by the coat.'),
  gain('mix_sky', 'Sky', 'Straight from the sky, sent back by the matte lobe. (The truth only: in real time it is part of wide light.)'),
  gain('mix_one', 'One bounce', 'Light that has met one other surface. (The truth only.)'),
  gain('mix_two', 'Two bounces', 'Light that has met two. (The truth only.)'),
  gain('mix_more', 'More bounces', 'All the rest. (The truth only.)'),
  gain('mix_narrow', 'Mirror', 'All light but the lamps’ own, sent back by the coat.'),
];

export function addTruePicture(graph: Graph, views: Views): void {
  graph.addBuffer({ name: 'true_picture', struct: Light, size: 'render', about: 'the sum of the truth’s parts' });
  graph.addStage({
    name: 'true picture',
    question: 'What does the eye see, if every part is true?',
    reads: ['hit_x', 'truth'],
    writes: ['true_picture'],
    uses: ['x'],
    over: 'render',
    code: `
fn run(pixel: vec2u) {
  let hit = hit_x_at(vec2i(pixel));
  var sum = hit.glow * knobs.mix_glow;
  if (hit.distance >= 0.0 && hit.lamp == NOTHING) {
    let t = truth_at(vec2i(pixel));
    let s = surface_x(hit);
    let lands = t.still * knobs.mix_still + t.changing * knobs.mix_changing + t.sky * knobs.mix_sky
      + t.one * knobs.mix_one + t.two * knobs.mix_two + t.more * knobs.mix_more;
    sum += matte_share(s) * lands + t.shine * knobs.mix_shine + t.narrow * knobs.mix_narrow;
  }
  put_true_picture(pixel, Light(sum));
}`,
  });
  views.add({
    id: 'true picture', group: 'Pictures', name: 'The truth', about: 'The sum of the truth’s parts, by the mixer. It settles as you wait.',
    reads: ['true_picture'], code: 'return show_light(true_picture_at(at_render(p)).light);',
  });
}

export function addPicture(graph: Graph, views: Views): void {
  graph.addBuffer({ name: 'picture', struct: Light, size: 'render', about: 'the sum of the signals' });
  graph.addStage({
    name: 'sum',
    question: 'What does the eye see, from the signals as they are switched?',
    reads: ['hit_x', 'unblocked', 'still_used', 'changing_used', 'shine_used'],
    writes: ['picture'],
    uses: ['x'],
    over: 'render',
    code: `
fn run(pixel: vec2u) {
  let p = vec2i(pixel);
  let hit = hit_x_at(p);
  var sum = hit.glow * knobs.mix_glow;
  if (hit.distance >= 0.0 && hit.lamp == NOTHING) {
    let s = surface_x(hit);
    var still = still_used_at(p).value;
    var changing = changing_used_at(p).value;
    if (knobs.denoise_what == DENOISE_WHAT_SHARE) {
      let u = unblocked_at(p);
      still *= u.still;
      changing *= u.changing;
    }
    sum += matte_share(s) * (still * knobs.mix_still + changing * knobs.mix_changing);
    sum += shine_used_at(p).value * knobs.mix_shine;
  }
  put_picture(pixel, Light(sum));
}`,
  });
  views.add({
    id: 'picture', group: 'Pictures', name: 'Real time', about: 'The sum of the signals, each as its switch says: off, raw, settled or true.',
    reads: ['picture'], code: 'return show_light(picture_at(at_render(p)).light);',
  });
  views.add({
    id: 'picture off', group: 'Pictures', name: 'Real time, off the truth',
    about: 'Red too bright, blue too dark; full color at one stop. Only the parts built so far are compared, if the mixer has the others off.',
    reads: ['picture', 'true_picture'],
    code: 'return show_difference(brightness(picture_at(at_render(p)).light), brightness(true_picture_at(at_render(p)).light));',
  });
}
