// x → y, y → LIGHT, y → z. Rays from x, what lamps send where they land, and the cache for the rest.

import { f32, struct, u32, vec3f } from '../gpu/layout';
import type { Graph } from '../gpu/graph';
import type { Views } from '../page/views';
import { choice, chosen, slider, toggle, valueOf } from '../page/knobs';
import { addSignal, Signal } from './signals';
import wideRays from './wide_rays.wgsl';
import narrowRays from './narrow_rays.wgsl';
import yLight from './y_light.wgsl';
import xAnswer from './x_answer.wgsl';
import yAnswer from './y_answer.wgsl';

export const HitY = struct('HitY', {
  position: vec3f('where the ray landed', { unit: 'm', look: { kind: 'place' } }),
  distance: f32('how far off; −1 sky, −2 no ray', { unit: 'm', look: { kind: 'heat', from: 0, to: 6 } }),
  normal: vec3f('which way it faces there', { look: { kind: 'direction' } }),
  lamp: u32('which lamp it is, if it shines'),
  sends: vec3f('the share of landing light it sends back', { look: { kind: 'color' } }),
  weight: f32('cosine / π over the ray’s chance', { look: { kind: 'heat', from: 0, to: 4 } }),
  toward: vec3f('the ray’s way', { look: { kind: 'direction' } }),
  object: u32('what it moves with'),
  glow: vec3f('what it sends by itself'),
  steps: f32('tree nodes the ray looked at', { look: { kind: 'heat', from: 0, to: 30 } }),
  passes: vec3f('what of its light gets through the air on the way', { look: { kind: 'color' } }),
  spare0: f32('', { look: { kind: 'none' } }),
  added: vec3f('what the air on the way sends by itself'),
  spare1: f32('', { look: { kind: 'none' } }),
});
export const LightAtY = struct('LightAtY', {
  lands: vec3f('what lands there straight from lamps'),
  seen: f32('was the lamp that was asked seen', { look: { kind: 'share' } }),
  lamp: u32('which lamp was asked'),
});

slider('wide_step', { label: 'Wide light: one ray in', min: 1, max: 4, step: 1, value: 2, unit: '² pixels', about: '1: a ray a pixel. 2: half size, a quarter of the rays. The denoiser and the guided upscale make up for it.' });
slider('narrow_step', { label: 'Mirrors: one ray in', min: 1, max: 4, step: 1, value: 1, unit: '² pixels', about: 'Sharp mirrors show the blocks of a small grid.' });
choice('stop_at', {
  label: 'Where do paths stop?', value: 2,
  options: [['x', 'x'], ['y', 'y'], ['z', 'z'], ['nowhere', 'No cache']],
  about: 'x: no wide rays, the cache is read where you look (to see it). y: the cache answers for lamps and all else where a wide ray lands. z: lamps are asked freshly at y, the cache answers for the rest. No cache: lamps at y, and black beyond.',
});
slider('wide_aim', { label: 'Aimed through openings', min: 0, max: 0.9, step: 0.05, value: 0, about: 'The share of wide rays that go through a door or window the room has named. The rest go by the cosine; dividing by the mixture keeps the mean right.' });
toggle('coat_as_matte', { label: 'Coats at y count as matte', value: true, about: 'Past x, what a coat sends back is spread evenly. Without it metals are black in bounces. It is not true, and small.' });
toggle('break_lamps_twice', { label: 'Wide rays count lamps too', about: 'x → light has counted them. The picture gets brighter where wide rays run into lamps, and very noisy.' });

const Y_OF = `
const NO_RAY = -2.0;

/// What a ray met, as y.
fn y_of(ray: Ray, met: Meeting, weight: f32) -> HitY {
  var y: HitY;
  y.lamp = NOTHING;
  y.toward = ray.toward;
  y.weight = weight;
  y.distance = -1.0;
  y.steps = f32(met.steps);
  y.object = NOTHING;
  let air = smoke_between(ray.origin, ray.toward, min(met.distance, 60.0), ray.width, ray.spread, max(knobs.air_shadow_steps, 1u), 0.5, true);
  y.passes = air.passes;
  y.added = air.added;
  if (met.piece == NOTHING) { return y; }
  let found = found_at(ray, met);
  let s = found.surface;
  y.position = s.position;
  y.distance = met.distance;
  y.normal = s.normal;
  // something thin sends back light from both its sides: one is asked, as often as it counts
  let front = brightness(front_of(s));
  let die = unit(hash2(bitcast<u32>(ray.toward.x), hash2(bitcast<u32>(ray.toward.z), frame.number + 3u)));
  if (front < 1.0 && die >= front) { y.normal = -s.normal; }
  y.lamp = found.lamp;
  y.glow = found.glow;
  y.object = pieces[met.piece].object;
  y.sends = matte_share(s);
  if (knobs.coat_as_matte != 0u) { y.sends += coat_share(s); }
  return y;
}`;

/** rays, light at their hits, and the answer, for one kind of ray */
export function addBounce(graph: Graph, views: Views): void {
  graph.addModule('y', { needs: ['ray', 'surface', 'material', 'smoke'], shared: [], code: Y_OF });
  graph.addModule('no ray', { needs: [], shared: [], code: 'const NO_RAY = -2.0;' });
  addSignal(graph, {
    id: 'wide', size: 'wide', name: 'Wide light',
    about: 'Light from every other surface and from the sky.',
    truth: 't.sky + t.one + t.two + t.more', rawIsAlias: true,
  }, 'black', []);
  addSignal(graph, {
    id: 'narrow', size: 'narrow', name: 'Mirror',
    about: 'The same, sent to the eye by the coat.',
    truth: 't.narrow / max(coat_share(s), vec3f(1e-4))', rawIsAlias: true,
  }, 'black', []);
  graph.addBuffer({ name: 'hit_y', struct: HitY, size: 'wide', about: 'where the wide rays landed' });
  graph.addBuffer({ name: 'hit_r', struct: HitY, size: 'narrow', about: 'where the narrow rays landed' });
  graph.addStage({
    name: 'wide rays', question: 'What is there, over the half-sphere above x?',
    reads: ['hit_x'], writes: ['hit_y'], uses: ['y', 'front', 'dice', 'draw', 'openings'], over: 'wide', code: wideRays,
  });
  graph.addStage({
    name: 'narrow rays', question: 'What does the coat mirror?',
    reads: ['hit_x'], writes: ['hit_r'], uses: ['x', 'y', 'dice', 'draw'], over: 'narrow', code: narrowRays,
  });
  for (const [grid, hit, name] of [['wide', 'hit_y', 'wide rays'], ['narrow', 'hit_r', 'narrow rays']] as const) {
    views.add({
      id: `${grid}_raw.value`, group: 'Bounces', name: `${name}: what came back`, about: 'This frame’s answer, at the size it was found. As a white wall would send it back.',
      reads: [`${grid}_raw`], code: `return show_light(${grid}_raw_at(at_${grid}(p)).value * 0.8);`,
    });
    views.add({
      id: `${hit} sends`, group: 'Bounces', name: `${name}: what y is made of`, about: 'The share of light each ray’s hit sends back.',
      reads: [hit], code: `return ${hit}_at(at_${grid}(p)).sends;`,
    });
  }
}

/** after the cache's own stages: light at the hits, and what comes back */
export function addAnswers(graph: Graph): void {
  for (const [id, grid, hit, offset] of [['wide', 'wide', 'hit_y', 0], ['narrow', 'narrow', 'hit_r', 20], ['air', 'wide', 'hit_a', 40]] as const) {
    graph.addBuffer({ name: `light_${hit}`, struct: LightAtY, size: grid, about: 'what lamps land where the rays landed' });
    graph.addStage({
      name: `${id} y light`, question: 'What do lamps send to where the ray landed?',
      reads: { hit }, writes: { light_y: `light_${hit}` },
      uses: ['ray', 'lamps', 'smoke', 'dice', 'draw'], over: grid, constants: { DICE_OFFSET: offset }, code: yLight.replaceAll('GRID', grid),
    });
    graph.addBuffer({ name: `${id}_raw_y`, struct: Signal, size: grid, about: 'what came back along each ray' });
    graph.addStage({
      name: `${id === 'air' ? 'air rays' : id} answer`, question: 'What comes back along the ray?',
      reads: { hit, light_y: `light_${hit}`, cache: 'cache', cache_points: 'cache_points' },
      writes: { raw: `${id}_raw_y` },
      uses: ['no ray', 'sky', 'dice', 'cache'], over: grid, constants: { DICE_OFFSET: offset }, code: yAnswer,
    });
  }
  graph.addBuffer({ name: 'wide_raw_x', struct: Signal, size: 'wide', about: 'the cache, read at x' });
  graph.addStage({
    name: 'cache at x', question: 'What does the cache say where the eye looks?',
    reads: ['hit_x', 'cache', 'cache_points'], writes: { raw: 'wide_raw_x' },
    uses: ['dice', 'cache'], over: 'wide', code: xAnswer,
  });
  graph.addAlias('wide_raw', () =>
    chosen('stop_at') === 'x' ? 'wide_raw_x' : valueOf('gi_time') || valueOf('gi_space') ? 'wide_raw_gi' : 'wide_raw_y');
  graph.addAlias('narrow_raw', () => 'narrow_raw_y');
}
