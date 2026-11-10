// REUSE AT y. The same trick as picking lamps, with the places that wide rays found in the lamps' place (ReSTIR GI).

import type { Graph } from '../gpu/graph';
import { f32, struct, vec3f } from '../gpu/layout';
import type { Views } from '../page/views';
import { slider, toggle, valueOf } from '../page/knobs';
import { Signal } from './signals';
import time from './gi_time.wgsl';
import space from './gi_space.wgsl';
import answer from './gi_answer.wgsl';

const GiReservoir = struct('GiReservoir', {
  position: vec3f('the place it kept', { unit: 'm', look: { kind: 'place' } }),
  weight: f32('W', { look: { kind: 'heat', from: 0, to: 12 } }),
  normal: vec3f('which way that faces', { look: { kind: 'direction' } }),
  score: f32('what it would send here', { look: { kind: 'heat', from: 0, to: 0.3 } }),
  light: vec3f('what it sends back'),
  memory: f32('M: how many rays stand behind it', { look: { kind: 'heat', from: 0, to: 40 } }),
  age: f32('frames since it was found', { look: { kind: 'heat', from: 0, to: 30 } }),
});

export function addGi(graph: Graph, views: Views): void {
  toggle('gi_time', { label: 'Remember the bright y', about: 'What this pixel’s wide rays found last frame is offered again.' });
  toggle('gi_space', { label: 'Share it with neighbors', about: 'What pixels round here keep is offered too, its chance turned from their half-sphere into this one’s.' });
  slider('gi_memory', { label: 'Memory of y', min: 1, max: 40, step: 1, value: 10, unit: 'frames', about: 'Long: a find serves many frames, and lasts after the door is shut.' });
  slider('gi_neighbors', { label: 'Neighbors asked', min: 1, max: 8, step: 1, value: 4, about: 'More spread a find faster, and make blotches.' });
  slider('gi_radius', { label: 'How far off', min: 1, max: 40, step: 1, value: 12, unit: 'wide px', about: 'Near ones found much the same; far ones know more that is new, and fit worse.' });
  toggle('gi_check', { label: 'Check that y is seen', value: true, about: 'One ray from x to the place that was kept. Without it, light comes through walls from where a neighbor could see.' });
  toggle('break_no_turn', { label: 'Leave out the change of angle', about: 'A neighbor found y under another angle and distance. Take its weight as it is, and the light drifts: in the Door room some points darker, more in corners.' });

  graph.addBuffer({ name: 'gi_kept', struct: GiReservoir, size: 'wide', history: true, about: 'the place each wide pixel keeps' });
  graph.addBuffer({ name: 'gi_shared', struct: GiReservoir, size: 'wide', about: 'the same, after the neighbors’' });
  graph.addBuffer({ name: 'wide_raw_gi', struct: Signal, size: 'wide', about: 'wide light, from the places that are kept' });
  graph.addStage({
    name: 'y reuse time', question: 'Which place did this pixel keep, and is it worth more than what it found now?',
    reads: { hit_x: 'hit_x', hit_x_before: 'hit_x@before', hit_y: 'hit_y', found_at_y: 'wide_raw_y', gi_before: 'gi_kept@before' },
    writes: { gi: 'gi_kept' }, uses: ['no ray', 'dice'], over: 'wide', code: time,
  });
  graph.addStage({
    name: 'y reuse space', question: 'What places do the pixels round here keep?',
    reads: { hit_x: 'hit_x', time: 'gi_kept' }, writes: { gi: 'gi_shared' }, uses: ['dice', 'draw'], over: 'wide', code: space,
  });
  graph.addAlias('gi_picked', () => (valueOf('gi_space') ? 'gi_shared' : 'gi_kept'));
  graph.addStage({
    name: 'y reuse answer', question: 'Is the place that was kept seen from here?',
    reads: { hit_x: 'hit_x', picked: 'gi_picked' }, writes: { raw: 'wide_raw_gi' }, uses: ['ray', 'draw'], over: 'wide', code: answer,
  });
  views.add({
    id: 'gi light', group: 'Bounces', name: 'reuse: the light that is kept', about: 'What the kept place sends back.',
    reads: ['gi_picked'], code: 'return show_light(gi_picked_at(at_wide(p)).light);',
  });
}
