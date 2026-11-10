// x → LIGHT. What the lamps would send (exact), which of them one ray should ask, and whether it is seen.

import { f32, struct, u32, vec2f, vec3f } from '../gpu/layout';
import type { Graph } from '../gpu/graph';
import type { Views } from '../page/views';
import { choice, slider, toggle, valueOf } from '../page/knobs';
import { addSignal } from './signals';
import unblocked from './unblocked.wgsl';
import freshPick from './fresh_pick.wgsl';
import reuseTime from './reuse_time.wgsl';
import reuseSpace from './reuse_space.wgsl';
import shadow from './shadow.wgsl';
import shine from './shine.wgsl';

const Unblocked = struct('Unblocked', {
  still: vec3f('what lamps that stand would land here with nothing in the way, exact'),
  lamps: f32('how many lamps can reach this', { look: { kind: 'heat', from: 0, to: 60 } }),
  changing: vec3f('the same, of lamps that change'),
});
export const Reservoir = struct('Reservoir', {
  lamp: u32('the lamp it kept'),
  weight: f32('W: what to multiply that lamp’s light by', { look: { kind: 'heat', from: 0, to: 20 } }),
  dice: vec2f('where on the lamp'),
  score: f32('what the kept sample would send, unblocked', { look: { kind: 'heat', from: 0, to: 0.5 } }),
  memory: f32('M: how many samples stand behind it', { look: { kind: 'heat', from: 0, to: 64 } }),
  seen: f32('was it seen, when a ray asked', { look: { kind: 'share' } }),
  age: f32('frames since it was found', { look: { kind: 'heat', from: 0, to: 60 } }),
});
const ShineWays = struct('ShineWays', {
  aimed: f32('what aiming found, weighed', { look: { kind: 'heat', from: 0, to: 1 } }),
  followed: f32('what following the lobe found, weighed', { look: { kind: 'heat', from: 0, to: 1 } }),
});

choice('lamp_pick', {
  label: 'Which lamp does a ray ask?', value: 2,
  options: [['any', 'Any'], ['few', 'Best of a few'], ['all', 'Walk all']],
  about: 'Any: each lamp with the same chance. Best of a few: some are drawn and one is kept by what it would send. Walk all: every lamp is scored, so the only noise left is whether it is seen.',
});
slider('lamp_candidates', {
  label: 'Candidates', min: 1, max: 64, step: 1, value: 8,
  about: 'How many lamps “best of a few” looks at. Looking is cheap (no ray); it is how the one ray gets spent well.',
});
toggle('reuse_time', { label: 'Reuse: last frame', about: 'Offer what this spot kept last frame. Candidates add up over frames, at the price of one.' });
toggle('reuse_space', { label: 'Reuse: neighbors', about: 'Offer what pixels round here kept last frame. They look at much the same lamps.' });
slider('reuse_neighbors', { label: 'Neighbors asked', min: 1, max: 8, step: 1, value: 4, about: 'More of them spread a good find faster, and make neighbors’ noise more alike: blotches.' });
slider('reuse_radius', { label: 'How far off', min: 1, max: 64, step: 1, value: 20, unit: 'px', about: 'Near ones see the same lamps; far ones know more that is new.' });
slider('reuse_memory', { label: 'Memory', min: 1, max: 64, step: 1, value: 20, unit: 'frames', about: 'How much the past may count against what is found now. Large: calm, and slow to notice change.' });
toggle('reuse_seen', { label: 'Forget what was hidden', value: true, about: 'A kept lamp whose ray was blocked last frame is not offered again, so rays go to lamps that are seen. By itself it is right. With neighbors on, the picture turns a few percent dark: their hidden finds still count in the mean, and checking that would cost a ray each.' });
toggle('reuse_correct', { label: 'Count only who could have found it', value: true, about: 'A neighbor that cannot see a lamp’s face could never have offered it; counting it in the mean makes the picture too dark. Checking costs one evaluation a neighbor.' });
toggle('break_no_tests', { label: 'Reuse across corners', about: 'Takes samples from any surface near on the screen. Each was weighed for another place: edges go wrong. (With “count only who could have found it” on, the mean stays right and only the noise grows; switch that off too.)' });
toggle('break_no_mean', { label: 'Forget 1/M', about: 'The kept sample’s weight must be a mean over all that were offered. Leave that out and every candidate makes the picture brighter.' });
choice('denoise_what', {
  label: 'What is kept as signal?', value: 0, options: [['share', 'Share seen'], ['light', 'Light']],
  about: 'Share seen: only 0 to 1, whether the lamp was seen; the light itself is exact and multiplied afterwards, so no filter can blur it. It needs “walk all”. Light: what landed, dice and all.',
});
choice('shine_by', {
  label: 'Highlights are found by', value: 2, options: [['aim', 'Aiming'], ['lobe', 'The lobe'], ['both', 'Both, weighed']],
  about: 'Aiming finds small lamps and fails on mirrors; following the lobe finds large ones and fails on rough coats with small lamps. Weighed, each is used where it is good.',
});
toggle('break_count_twice', { label: 'Count both in full', about: 'Both ways find the same light. Adding them without weights makes highlights twice as bright.' });

export function addLamps(graph: Graph, views: Views): void {
  graph.addBuffer({ name: 'unblocked', struct: Unblocked, size: 'render', about: 'what the lamps would land with nothing in the way' });
  graph.addStage({
    name: 'unblocked', question: 'What would the lamps send, with nothing in the way?',
    reads: ['hit_x'], writes: ['unblocked'], uses: ['lamps'], over: 'render', code: unblocked,
  });
  for (const [cls, id] of [[0, 'still'], [1, 'changing']] as const) {
    addSignal(graph, {
      id, size: 'render',
      name: cls === 0 ? 'Lamps that stand' : 'Lamps that change',
      about: cls === 0 ? 'Light straight from lamps that stand.' : 'Light straight from lamps that are switched, swung or carried.',
      truth: `select(t.${id}, t.${id} / max(u.${id}, vec3f(1e-9)), knobs.denoise_what == DENOISE_WHAT_SHARE)`,
    }, 'nothing is in the way', ['unblocked']);
    const about = cls === 0 ? 'of lamps that stand' : 'of lamps that change';
    graph.addBuffer({ name: `fresh_${id}`, struct: Reservoir, size: 'render', about: `this frame’s own pick ${about}` });
    graph.addBuffer({ name: `time_${id}`, struct: Reservoir, size: 'render', about: `the pick ${about}, after reuse in time` });
    graph.addBuffer({ name: `space_${id}`, struct: Reservoir, size: 'render', about: `the pick ${about}, after reuse from neighbors` });
    graph.addBuffer({ name: `reservoir_${id}`, struct: Reservoir, size: 'render', about: `the lamp that was asked ${about}, and whether it was seen` });
    graph.addBuffer({ name: `memory_${id}`, struct: Reservoir, size: 'render', history: true, about: `the pick ${about} that is kept for next frame` });
    graph.addStage({
      name: `pick ${id}`, question: 'Which lamp should one ray ask?',
      reads: ['hit_x'], writes: { fresh: `fresh_${id}` },
      uses: ['lamps', 'dice', 'reservoir', 'draw'], over: 'render', constants: { CLASS: cls }, code: freshPick,
    });
    graph.addStage({
      name: `reuse ${id} time`, question: 'What did this spot keep last frame?',
      reads: { hit_x: 'hit_x', hit_x_before: 'hit_x@before', fresh: `fresh_${id}`, old: `memory_${id}@before` },
      writes: { merged: `time_${id}` },
      uses: ['lamps', 'dice', 'reservoir'], over: 'render', constants: { CLASS: cls }, code: reuseTime,
    });
    graph.addStage({
      name: `reuse ${id} space`, question: 'What have the pixels round here found?',
      reads: { hit_x: 'hit_x', time: `time_${id}` },
      writes: { merged: `space_${id}` },
      uses: ['lamps', 'dice', 'reservoir', 'draw'], over: 'render', constants: { CLASS: cls }, code: reuseSpace,
    });
    graph.addAlias(`picked_${id}`, () =>
      valueOf('reuse_space') ? `space_${id}` : valueOf('reuse_time') ? `time_${id}` : `fresh_${id}`);
    graph.addStage({
      name: `shadow ${id}`, question: 'Is the lamp that was picked seen?',
      reads: { hit_x: 'hit_x', picked: `picked_${id}` },
      writes: { reservoir: `reservoir_${id}`, raw: `${id}_raw` },
      uses: ['ray', 'lamps', 'reservoir', 'draw'], over: 'render', constants: { CLASS: cls }, code: shadow,
    });
    graph.addStage({
      name: `remember ${id}`, question: 'What is kept for next frame?',
      reads: { time: `time_${id}`, asked: `reservoir_${id}` },
      writes: { memory: `memory_${id}` },
      uses: [], over: 'render',
      code: `
// What this spot found by itself and kept over time; not what its neighbors gave it, or the same few samples
// would go round and round. If it was the one a ray asked about, whether it was seen is known.
fn run(pixel: vec2u) {
  var r = time_at(vec2i(pixel));
  let a = asked_at(vec2i(pixel));
  r.seen = 1.0;
  if (a.lamp == r.lamp && all(a.dice == r.dice)) { r.seen = a.seen; }
  put_memory(pixel, r);
}`,
    });
  }
  addSignal(graph, {
    id: 'shine', size: 'render', name: 'Highlights',
    about: 'Light straight from lamps, sent to the eye by the coat.',
    truth: 't.shine',
  }, 'none', []);
  graph.addBuffer({ name: 'shine_ways', struct: ShineWays, size: 'render', about: 'which way found the highlight' });
  graph.addStage({
    name: 'shine', question: 'What does the coat show of the lamps?',
    reads: ['hit_x'], writes: ['shine_raw', 'shine_ways'],
    uses: ['x', 'ray', 'lamps', 'dice', 'draw'], over: 'render', code: shine,
  });

  views.add({
    id: 'shine ways', group: 'Lamps', name: 'Which way found it', about: 'Red: aiming at lamps. Green: following the lobe. Each already weighed, so the two add up to the highlight.',
    reads: ['shine_ways'], code: 'let w = shine_ways_at(at_render(p)); return show_light(vec3f(w.aimed, w.followed, 0.0));',
  });
  for (const id of ['still', 'changing']) {
    views.add({
      id: `${id} unblocked`, group: 'Lamps', name: `${id}: unblocked`, about: 'What they would land with nothing in the way, as a white wall sends it back. Exact: no dice.',
      reads: ['unblocked'], code: `return show_light(unblocked_at(at_render(p)).${id} * 0.8);`,
    });
    views.add({
      id: `${id} lamp`, group: 'Lamps', name: `${id}: lamp picked`, about: 'Each lamp in a color of its own.',
      reads: [`reservoir_${id}`], code: `return show_id(reservoir_${id}_at(at_render(p)).lamp);`,
    });
  }
}
