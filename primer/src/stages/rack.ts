// THE RACK. Nine denoisers, all fed the same noise. Each rung adds one part to the one before, so that what
// a part buys and what it costs can be seen.

import type { Graph } from '../gpu/graph';
import type { Views } from '../page/views';
import { choice, chosen } from '../page/knobs';
import { addDenoiser, type DenoiserParts } from './denoise';
import { SIGNALS } from './signals';

export interface Rung {
  number: number;
  name: string;
  about: string;
  parts: DenoiserParts;
}
export const RUNGS: Rung[] = [
  { number: 1, name: 'Nothing', about: 'The noise itself.', parts: { noTime: true } },
  { number: 2, name: 'Mean, same pixel', about: 'Perfect while nothing moves; smears when something does.', parts: {} },
  { number: 3, name: 'Mean, follows motion', about: 'Survives a moving camera; ghosts where something was uncovered.', parts: { motion: true } },
  { number: 4, name: '+ tests', about: 'The old pixel must have looked at the same surface. Ghosts go; noise returns in a rim.', parts: { motion: true, tests: true } },
  { number: 5, name: '+ leash', about: 'History is held near what neighbors say now. Lag goes; rare bright light is lost.', parts: { motion: true, tests: true, leash: true } },
  { number: 6, name: 'Neighbors only', about: 'No time at all: no lag, and blotches that crawl.', parts: { noTime: true, blur: true } },
  { number: 7, name: 'Both', about: 'History, leash, then a blur as wide as the doubt that is left.', parts: { motion: true, tests: true, leash: true, blur: true } },
  { number: 8, name: 'Blur fed back', about: 'Last frame’s blurred answer is the history: very smooth, very sticky.', parts: { motion: true, tests: true, leash: true, blur: true, feedback: true } },
];

export function addRack(graph: Graph, views: Views): void {
  choice('rack_signal', {
    label: 'The rack is fed', value: 0, options: SIGNALS.filter((s) => s.size === 'render').map((s) => [s.id, s.name]),
    about: 'Which signal’s noise all the denoisers get.',
  });
  graph.addAlias('rack_noisy', () => `${chosen('rack_signal')}_raw`);
  graph.addAlias('rack_true', () => `${chosen('rack_signal')}_true`);
  for (const rung of RUNGS) {
    const id = `rack${rung.number}`;
    addDenoiser(graph, id, 'rack_noisy', rung.parts);
    views.add({
      id, group: 'The denoiser rack', name: `${rung.number}. ${rung.name}`, about: rung.about,
      reads: [`${id}_out`], code: `return rack_shown(${id}_out_at(at_render(p)).value);`, uses: ['rack'],
      judged: { answer: `${id}_out`, truth: 'rack_true' },
    });
    views.add({
      id: `${id} off`, group: 'The denoiser rack, off the truth', name: `${rung.number}. ${rung.name}`,
      about: 'Red too bright, blue too dark; full color at one stop. A share is compared as it is.',
      reads: [`${id}_out`, 'rack_true'],
      code: `return show_difference(brightness(${id}_out_at(at_render(p)).value), brightness(rack_true_at(at_render(p)).value));`,
    });
  }
  views.add({
    id: 'rack true', group: 'The denoiser rack', name: 'The truth', about: 'What all of them try to say.',
    reads: ['rack_true'], code: 'return rack_shown(rack_true_at(at_render(p)).value);', uses: ['rack'],
  });
  views.add({
    id: 'rack why', group: 'The denoiser rack', name: 'Why history was dropped (7)', about: 'Dark: kept. Green: off the screen. Orange: another surface. Blue: pulled back by the leash.',
    reads: ['rack7_kept'],
    code: `let w = rack7_kept_at(at_render(p)).why;
  if (w == 1.0) { return vec3f(0.1, 0.5, 0.1); }
  if (w == 2.0) { return vec3f(0.8, 0.3, 0.02); }
  if (w == 3.0) { return vec3f(0.05, 0.15, 0.6); }
  return vec3f(0.02);`,
  });
  views.add({
    id: 'rack count', group: 'The denoiser rack', name: 'Frames behind it (7)', about: 'Dark: few. Yellow: 64 and more.',
    reads: ['rack7_kept'], code: 'return show_heat(rack7_kept_at(at_render(p)).count / 64.0);',
  });
  // a share is drawn as it is, light through the exposure
  graph.addModule('rack', {
    needs: ['show'], shared: [],
    code: `fn rack_shown(v: vec3f) -> vec3f {
  if (knobs.denoise_what == DENOISE_WHAT_SHARE && knobs.rack_signal < 2u) { return v; }
  return show_light(v * 0.8);
}`,
  });
}
