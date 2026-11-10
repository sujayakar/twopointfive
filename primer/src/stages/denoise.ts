// DENOISERS. Two stages, each with parts that can be left out: history (the mean over frames, following
// what moves) and blur (the mean over neighbors, as far as the doubt that is left). A rung of the rack is a
// choice of parts.

import { f32, struct, vec3f } from '../gpu/layout';
import type { Graph, Grid } from '../gpu/graph';
import { choice, slider, valueOf } from '../page/knobs';
import { Signal } from './signals';
import history from './history.wgsl';
import blur from './blur.wgsl';
import upscale from './upscale_signal.wgsl';

export const Kept = struct('Kept', {
  value: vec3f('the mean over time'),
  count: f32('frames behind it', { look: { kind: 'heat', from: 0, to: 64 } }),
  square: f32('mean of the squared brightness', { look: { kind: 'none' } }),
  doubt: f32('how far the mean may be off: its variance', { look: { kind: 'heat', from: 0, to: 0.05 } }),
  distance: f32('how far off it came from', { unit: 'm', look: { kind: 'heat', from: 0, to: 6 } }),
  why: f32('why history was dropped: 0 kept, 1 off screen, 2 another surface, 3 held back by the leash', { look: { kind: 'heat', from: 0, to: 3 } }),
});
export const Smooth = struct('Smooth', {
  value: vec3f('after this round of blur'),
  doubt: f32('what doubt is left', { look: { kind: 'heat', from: 0, to: 0.05 } }),
});

slider('history_frames', { label: 'Frames remembered', min: 1, max: 256, step: 1, value: 32, ratio: true, unit: 'frames', about: 'A mean of so many frames has 1/√n of the noise, and lags as long behind any change.' });
slider('history_leash', { label: 'Leash', min: 0.2, max: 8, step: 0.1, value: 1.5, unit: 'σ', about: 'How far, in this frame’s own spread, the past may stand from what neighbors say now. Short: quick, and rare bright light is lost. Long: calm, and ghosts.' });
slider('blur_rounds', { label: 'Rounds of blur', min: 0, max: 5, step: 1, value: 4, about: 'Each round reaches twice as far as the one before: 1, 2, 4, 8, 16 pixels apart.' });
slider('blur_soft', { label: 'How unlike neighbors may be', min: 0.5, max: 12, step: 0.1, value: 2, unit: 'σ', about: 'In units of the doubt. Small keeps edges and noise; large loses both.' });
slider('blur_normal', { label: 'How alike they must face', min: 1, max: 128, step: 1, value: 32, ratio: true, about: 'High: a blur stops at the slightest bend.' });

choice('history_format', {
  label: 'History is kept in', value: 0, options: [['f32', '32 bits'], ['f16', '16 bits'], ['dithered', '16, rounded by dice']],
  about: 'In 16 bits, x += (now − x) / n with n = 64 rounds to no change at all when the difference is under about 3 %. History looks settled and is wrong. Rounding up or down by dice is right in the mean.',
});
choice('signal_up', {
  label: 'Small grids come up by', value: 2, options: [['nearest', 'Nearest'], ['linear', 'Linear'], ['guided', 'Guided by x']],
  about: 'How a pixel reads a signal that was found at half size. Nearest shows the blocks. Linear bleeds across edges. Guided takes only neighbors that look at the same surface.',
});

export interface DenoiserParts {
  /** follow what moved */
  motion?: boolean;
  /** the old pixel must have looked at the same surface */
  tests?: boolean;
  /** hold history near what neighbors say now */
  leash?: boolean;
  /** no mean over time at all */
  noTime?: boolean;
  /** rounds of blur after it */
  blur?: boolean;
  /** last frame's blurred answer is the history */
  feedback?: boolean;
  /** history follows the image behind the surface, as a mirror's does */
  image?: boolean;
}

const ROUNDS = 5;

/** Adds a denoiser for a noisy signal. Its answer is `<id>_out`, a Smooth. */
export function addDenoiser(graph: Graph, id: string, noisy: string, parts: DenoiserParts, size: Grid = 'render'): void {
  graph.addBuffer({ name: `${id}_kept`, struct: Kept, size, history: true, about: 'the mean over time' });
  const flag = (on?: boolean) => (on ? 1 : 0);
  const onGrid = (code: string) => code.replaceAll('GRID', size);
  graph.addStage({
    name: `${id} history`,
    question: 'What did this spot say before, and may it still count?',
    reads: {
      noisy, hit_x: 'hit_x', hit_x_before: 'hit_x@before', kept_before: `${id}_kept@before`,
      ...(parts.feedback ? { fed_before: `${id}_blur1@before` } : {}),
    },
    writes: { kept: `${id}_kept` },
    uses: ['dice'],
    over: size,
    constants: {
      MOTION: flag(parts.motion), TESTS: flag(parts.tests), LEASH: flag(parts.leash),
      NO_TIME: flag(parts.noTime), IMAGE: flag(parts.image),
    },
    code: onGrid(parts.feedback ? `${history}\nfn fed(q: vec2i) -> vec3f { return fed_before_at(q).value; }` : `${history}\nfn fed(q: vec2i) -> vec3f { return kept_before_at(q).value; }`),
  });
  graph.addBuffer({ name: `${id}_blur0`, struct: Smooth, size, about: 'the mean over time, about to be blurred' });
  graph.addStage({
    name: `${id} blur 0`, question: 'What goes into the blur?', reads: { kept: `${id}_kept` }, writes: { smooth_out: `${id}_blur0` },
    uses: [], over: size,
    code: 'fn run(pixel: vec2u) { let k = kept_at(vec2i(pixel)); put_smooth_out(pixel, Smooth(k.value, k.doubt)); }',
  });
  for (let i = 1; parts.blur && i <= ROUNDS; i++) {
    graph.addBuffer({ name: `${id}_blur${i}`, struct: Smooth, size, history: i === 1 && parts.feedback, about: `after ${i} rounds of blur` });
    graph.addStage({
      name: `${id} blur ${i}`,
      question: `What do neighbors ${1 << (i - 1)} pixels off say?`,
      reads: { smooth_in: `${id}_blur${i - 1}`, hit_x: 'hit_x' },
      writes: { smooth_out: `${id}_blur${i}` },
      uses: [], over: size, constants: { STRIDE: 1 << (i - 1) }, code: onGrid(blur),
    });
  }
  graph.addAlias(`${id}_out`, () => `${id}_blur${parts.blur ? Math.max(parts.feedback ? 1 : 0, valueOf('blur_rounds')) : 0}`);
}

/** the denoiser the picture uses for a signal: its answer as a signal again */
export function addSettled(graph: Graph, id: string, size: Grid): void {
  const image = id === 'narrow' || id === 'shine';
  addDenoiser(graph, `${id}_mean`, `${id}_raw`, { motion: true, tests: true, leash: true, blur: true, image }, size);
  graph.addBuffer({ name: `${id}_settled`, struct: Signal, size: 'render', about: 'after its denoiser' });
  if (size === 'render') {
    graph.addStage({
      name: `${id} settled`, question: 'What does the denoiser answer?',
      reads: { out: `${id}_mean_out` }, writes: { settled: `${id}_settled` }, uses: [], over: 'render',
      code: 'fn run(pixel: vec2u) { put_settled(pixel, Signal(out_at(vec2i(pixel)).value, 0.0)); }',
    });
    return;
  }
  // found on a smaller grid: up to render size, once as it was found and once denoised
  graph.addBuffer({ name: `${id}_raw_up`, struct: Signal, size: 'render', about: 'as it was found, brought up to render size' });
  for (const [name, from, to] of [['settled', `${id}_mean_out`, `${id}_settled`], ['raw up', `${id}_raw`, `${id}_raw_up`]] as const) {
    graph.addStage({
      name: `${id} ${name}`, question: 'What does a small grid say about this pixel?',
      reads: { small: from, hit_x: 'hit_x' }, writes: { large: to }, uses: [], over: 'render',
      code: upscale.replaceAll('GRID', size),
    });
  }
}
