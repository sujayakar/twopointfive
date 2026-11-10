// THE TRUTH. A plain path tracer whose answer is kept in the same parts as the real-time frame, so that
// every buffer has a right answer beside it. It settles while nothing moves.

import { f32, struct, vec3f } from '../gpu/layout';
import type { Graph } from '../gpu/graph';
import { slider, toggle } from '../page/knobs';
import code from './truth.wgsl';

export const Truth = struct('Truth', {
  still: vec3f('lands on a matte surface from lamps that stand'),
  count: f32('paths so far', { look: { kind: 'heat', from: 0, to: 1024 } }),
  changing: vec3f('…from lamps that change'),
  shine: vec3f('sent to the eye by the coat, of light straight from lamps'),
  sky: vec3f('lands from the sky'),
  one: vec3f('lands from surfaces lit by a lamp or the sky: one bounce'),
  two: vec3f('two bounces'),
  more: vec3f('three bounces and more'),
  narrow: vec3f('sent to the eye by the coat, of all other light'),
  spare: f32('', { look: { kind: 'none' } }),
  air: vec3f('what the air between the eye and x adds'),
});

slider('truth_bounces', {
  label: 'Longest path', min: 0, max: 16, step: 1, value: 8, unit: 'bounces', resetsTruth: true,
  about: 'How many surfaces a path may visit after x. Turn it up from 0 and watch light arrive a bounce at a time.',
});
toggle('break_no_cosine', {
  label: 'Lose the cosine', resetsTruth: true,
  about: 'Light from a slant lands spread over more surface. Leave that out and every bounce is too bright at grazing angles.',
});
toggle('break_no_sharing', {
  label: 'Coat and matte both get all', resetsTruth: true,
  about: 'The matte lobe should get only what the coat did not turn away. Without that, surfaces send back more than they get: the grey ball glows.',
});
slider('start_margin', {
  label: 'Start of a ray off its surface', min: 0, max: 0.02, step: 0.0005, value: 0.001, unit: 'm', resetsTruth: true,
  about: 'A ray starts a hair off the surface it leaves, or rounding makes it hit that same surface. Too much, and shadows come loose from what casts them.',
});
toggle('break_no_margin', {
  label: 'Start rays on the surface', resetsTruth: true,
  about: 'No hair of distance: half the rays hit the surface they left. The speckle is called acne.',
});

export function addTruth(graph: Graph): void {
  graph.addBuffer({ name: 'truth', struct: Truth, size: 'render', about: 'the right answer, in parts' });
  graph.addStage({
    name: 'truth',
    question: 'What would infinitely many paths find?',
    reads: ['hit_x'],
    writes: ['truth'],
    uses: ['ray', 'surface', 'sky', 'lamps', 'material', 'smoke', 'dice', 'draw'],
    over: 'render',
    code,
  });
}
