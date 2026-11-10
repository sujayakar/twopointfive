// THE SIGNALS. Everything noisy ends in one of these at x. Each exists raw (this frame's), settled (after a
// denoiser) and true; which of them the picture uses is a switch.

import { f32, struct, vec3f } from '../gpu/layout';
import type { Graph } from '../gpu/graph';
import { choice, chosen } from '../page/knobs';

export const Signal = struct('Signal', {
  value: vec3f('what the dice found this frame'),
  distance: f32('how far off it came from', { unit: 'm', look: { kind: 'heat', from: 0, to: 6 } }),
});

export interface SignalSpec {
  id: string;
  name: string;
  about: string;
  /** what the true value is, of the truth's parts, as an expression over `t` (Truth) and `s` (Surface) */
  truth: string;
  size: 'render' | 'half';
}

export const SIGNALS: SignalSpec[] = [];

/** Declares a signal: its raw and true buffers, its switch, and `<id>_used`. */
export function addSignal(graph: Graph, spec: SignalSpec, off: string, offReads: string[]): void {
  SIGNALS.push(spec);
  const { id } = spec;
  const key = `source_${id}`;
  choice(key, {
    label: spec.name, value: 2,
    options: [['off', 'Off'], ['raw', 'Raw'], ['settled', 'Settled'], ['truth', 'Truth']],
    about: `${spec.about} Off: ${off}. Raw: this frame’s rays. Settled: denoised. Truth: from the path tracer. Replace one signal by its truth to see whether an error is its.`,
  });
  graph.addBuffer({ name: `${id}_raw`, struct: Signal, size: spec.size, about: `${spec.name}: what this frame’s rays found` });
  graph.addBuffer({ name: `${id}_true`, struct: Signal, size: 'render', about: `${spec.name}, by the path tracer` });
  graph.addBuffer({ name: `${id}_off`, struct: Signal, size: 'render', about: `${spec.name}, switched off` });
  graph.addStage({
    name: `${id} true`,
    question: `What is ${spec.name.toLowerCase()} really?`,
    reads: ['hit_x', 'truth', 'unblocked'],
    writes: { out: `${id}_true` },
    uses: ['x'],
    over: 'render',
    code: `
fn run(pixel: vec2u) {
  let t = truth_at(vec2i(pixel));
  let u = unblocked_at(vec2i(pixel));
  let hit = hit_x_at(vec2i(pixel));
  let s = surface_x(hit);
  put_out(pixel, Signal(${spec.truth}, 0.0));
}`,
  });
  graph.addStage({
    name: `${id} off`,
    question: `What if ${spec.name.toLowerCase()} is switched off?`,
    reads: offReads,
    writes: { out: `${id}_off` },
    uses: [],
    over: 'render',
    code: `fn run(pixel: vec2u) { ${off.startsWith('nothing is in the way') ? `
  let u = unblocked_at(vec2i(pixel));
  put_out(pixel, Signal(select(u.${id}, vec3f(1.0), knobs.denoise_what == DENOISE_WHAT_SHARE), 0.0));` : 'put_out(pixel, Signal(vec3f(0.0), 0.0));'} }`,
  });
  graph.addAlias(`${id}_used`, () => `${id}_${{ off: 'off', raw: 'raw', settled: 'settled', truth: 'true' }[chosen(key)]!}`);
}
