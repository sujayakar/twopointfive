// THE RADIANCE CACHE: its buffers, its own rays, and the knobs to slow its construction down and poke at it.

import { makeBuffer, type Gpu } from '../gpu/device';
import { f32, struct, u32, vec3f } from '../gpu/layout';
import type { Graph, Grid } from '../gpu/graph';
import type { Views } from '../page/views';
import { choice, onKnob, slider, toggle, valueOf } from '../page/knobs';
import { HitY, LightAtY } from './bounce';
import cacheCode from '../wgsl/cache.wgsl';
import found from './cache_found.wgsl';
import rays from './cache_rays.wgsl';
import lamps from './cache_lamps.wgsl';
import gather from './cache_gather.wgsl';

export const CACHE_SLOTS = 32768;
export const CachePoint = struct('CachePoint', {
  position: vec3f('', { unit: 'm' }),
  born: f32('the round it was founded in'),
  normal: vec3f('', { look: { kind: 'direction' } }),
  frame: u32('the frame it was founded in'),
});
export const Entry = struct('Entry', {
  direct: vec3f('what lands from lamps'),
  samples: f32('rays behind it', { look: { kind: 'heat', from: 0, to: 64 } }),
  bounced: vec3f('what lands from everything else, being gathered'),
  direct_samples: f32(''),
  known: vec3f('what readers are told of it'),
  ready: f32('may it be read at all?', { look: { kind: 'share' } }),
});

const restarts = new Set(['cache_cell', 'cache_push', 'break_no_facing', 'cache_wall_test', 'cache_mode', 'break_reads_itself']);

slider('cache_cell', { label: 'Cell size', min: 0.1, max: 1.6, step: 0.01, value: 0.4, unit: 'm', about: 'How large a patch one entry speaks for. Small: sharp, and slow to fill. Large: quick, and light leaks round corners.' });
slider('cache_push', { label: 'Pushed off the surface', min: 0, max: 0.2, step: 0.01, value: 0.05, unit: 'm', about: 'Before the cell is found. At 0, a floor lying on a cell border is tossed between two cells by rounding.' });
toggle('cache_wall_test', { label: 'Wall test', value: true, about: 'An entry speaks for a place only if a short ray gets from one to the other. A cell with a wall through it then holds two entries. Without it, the dark room glows along its walls.' });
slider('cache_memory', { label: 'Memory of an entry', min: 4, max: 512, step: 1, value: 96, ratio: true, unit: 'samples', about: 'A long memory is calm and slow to notice that a lamp went out.' });
slider('cache_every', { label: 'An entry’s turn comes every', min: 1, max: 16, step: 1, value: 1, unit: 'rounds', about: 'Rays are spent in turns. The cost is by entries, not by pixels.' });
toggle('cache_late', { label: 'Later samples count for more', value: true, about: 'The first samples read a table that was still dark. Weighing later ones more lets that be forgotten.' });
toggle('cache_nudge', { label: 'Read with a nudge', value: true, about: 'A reader looks up a little to one side, by dice: cells blend as noise, which the denoiser removes, and not as blocks.' });
choice('cache_mode', {
  label: 'How it is filled', value: 0, options: [['running', 'Running mean'], ['bounces', 'A bounce at a time']],
  about: 'Running: every sample may read what is newest. A bounce at a time: all entries gather from a table that stands still, and only then is the new one shown. Watch light arrive one bounce after another.',
});
slider('cache_bounce_rounds', { label: 'Rounds a bounce', min: 4, max: 128, step: 1, value: 32, about: 'How many rays each entry gets before a whole bounce counts as known.' });
toggle('cache_paused', { label: 'Pause the filling', about: 'No entry gets a ray, none is founded. Use “One round” to let a single one through.' });
toggle('cache_entries', { label: 'Show the entries in space', about: 'Each entry as a dot with its normal, colored by the light it has. Best with the ghost.' });
toggle('break_no_facing', { label: 'Forget which way it faces', about: 'One entry for the floor and the wall beside it: light meant for one is told to the other.' });
toggle('break_reads_itself', { label: 'An entry may read itself', about: 'A ray that lands in its own cell reads what the entry itself has gathered, and corners glow more and more.' });

export class Cache {
  /** start over: forget every entry */
  zero = true;
  /** let this many rounds through although the filling is paused */
  rounds = 0;
  round = 0;
  private sinceCommit = 0;
  private keys: GPUBuffer;

  constructor(gpu: Gpu, private graph: Graph) {
    this.keys = makeBuffer(gpu, 'cache keys', CACHE_SLOTS * 4, GPUBufferUsage.STORAGE);
    graph.addShared('cache_keys', { type: 'array<atomic<u32>>', structs: [], access: 'read_write', buffer: () => this.keys });
    onKnob((k) => {
      if (restarts.has(k.key)) this.zero = true;
    });
  }

  /** what the frame may know of the cache; called once a frame that is made anew */
  step(encoder: GPUCommandEncoder): { cache_go: number; cache_commit: number; cache_round: number } {
    if (this.zero) {
      encoder.clearBuffer(this.keys);
      for (const name of ['cache', 'cache_points', 'cache_hits', 'cache_direct']) this.graph.clear(encoder, name);
      this.zero = false;
      this.round = 0;
      this.sinceCommit = 0;
    }
    const go = !valueOf('cache_paused') || this.rounds > 0;
    if (this.rounds > 0) this.rounds--;
    let commit = 0;
    if (go) {
      this.round++;
      if (++this.sinceCommit >= valueOf('cache_bounce_rounds') * valueOf('cache_every')) {
        this.sinceCommit = 0;
        commit = 1;
      }
    }
    return { cache_go: go ? 1 : 0, cache_commit: commit, cache_round: this.round };
  }
}

export function addCache(graph: Graph, views: Views): void {
  graph.addModule('cache', { needs: ['ray', 'dice'], shared: ['cache_keys'], code: cacheCode });
  graph.addModule('turn', {
    needs: [], shared: ['cache_keys'],
    code: `
/// Does this entry get a ray this round? Young ones always do.
fn its_turn(slot: u32, p: CachePoint, e: Entry) -> bool {
  if (frame.cache_go == 0u || dot(p.normal, p.normal) == 0.0) { return false; }
  return e.samples < 8.0 || slot % knobs.cache_every == frame.cache_round % knobs.cache_every;
}
/// The entry that speaks for the pinned point, as far as its key tells.
fn pinned_slot() -> u32 {
  if (frame.pinned_on == 0u) { return NOTHING; }
  let hit = hit_x_at(vec2i(frame.pinned));
  return cache_find(cache_key(hit.position, hit.normal, 0u));
}`,
  });
  const entries = { count: CACHE_SLOTS };
  graph.addBuffer({ name: 'cache_points', struct: CachePoint, size: entries, about: 'where each entry is' });
  graph.addBuffer({ name: 'cache', struct: Entry, size: entries, history: true, about: 'what each entry has gathered' });
  graph.addBuffer({ name: 'cache_hits', struct: HitY, size: entries, about: 'where each entry’s ray landed' });
  graph.addBuffer({ name: 'cache_direct', struct: LightAtY, size: entries, about: 'what the lamp each entry asked sends it' });
  const mine = { hit_x: 'hit_x', cache_points: 'cache_points', cache_before: 'cache@before' };
  // rays first, from the entries as they stand
  graph.addStage({
    name: 'cache rays', question: 'What does each entry see along one ray?',
    reads: mine, writes: ['cache_hits'], uses: ['y', 'dice', 'cache', 'turn', 'draw', 'openings'], over: entries, code: rays,
  });
  graph.addStage({
    name: 'cache lamps', question: 'What does one lamp send each entry?',
    reads: mine, writes: ['cache_direct'], uses: ['ray', 'lamps', 'dice', 'cache', 'turn', 'draw'], over: entries, code: lamps,
  });
  graph.addStage({
    name: 'cache gather', question: 'What has each entry learnt?',
    reads: { cache_points: 'cache_points', cache_before: 'cache@before', cache_hits: 'cache_hits', cache_direct: 'cache_direct' },
    writes: ['cache'], uses: ['no ray', 'ray', 'sky', 'dice', 'cache'], over: entries, code: gather,
  });
  // then founding, for the rays to come
  for (const [name, source, over] of [['wide', 'hit_y', 'wide'], ['narrow', 'hit_r', 'narrow'], ['own', 'cache_hits', entries]] as const) {
    graph.addStage({
      name: `cache found ${name}`, question: 'Where did a ray land that no entry speaks for?',
      reads: { source }, writes: ['cache_points'], uses: ['ray', 'dice', 'cache'],
      over: over as Grid | { count: number },
      code: typeof over === 'string'
        ? `${found.replace('fn run(index: u32)', 'fn found(index: u32)')}\nfn run(pixel: vec2u) { found(pixel.y * frame.${over}_size.x + pixel.x); }`
        : found,
    });
  }

  const look = (id: string, name: string, about: string, body: string) =>
    views.add({
      id, group: 'The cache, where you look', name, about,
      reads: ['hit_x', 'cache', 'cache_points'], uses: ['ray', 'dice', 'cache'],
      code: `
  let hit = hit_x_at(at_render(p));
  if (hit.distance < 0.0 || hit.lamp != NOTHING) { return vec3f(0.02); }
  let spot = cache_spot(hit.position, hit.normal);
  if (spot.slot == NOTHING) { return vec3f(0.16, 0.0, 0.13); }
  let e = cache[spot.slot];
  let at = cache_points[spot.slot];
  ${body}`,
    });
  look('cache cells', 'Cells', 'Each entry in a color of its own. Magenta: none speaks for this place. Bright: founded a moment ago.',
    'let young = exp(-f32(frame.number - at.frame) / 20.0); return mix(show_id(spot.slot), vec3f(1.0), young);');
  look('cache all', 'Light kept', 'What lands here by the cache, as a white wall sends it back: lamps and all else.', 'return show_light((e.direct + e.known) * 0.8);');
  look('cache direct', 'Straight from lamps', 'The part that lamps send.', 'return show_light(e.direct * 0.8);');
  look('cache bounced', 'All else', 'The part that is not straight from lamps: what readers at y are told.', 'return show_light(e.known * 0.8);');
  look('cache samples', 'Samples behind it', 'Dark few, yellow 96.', 'return show_heat(e.samples / 96.0);');
  look('cache side', 'Which side', 'Blue: the first entry of its cell. Orange: the second, for the other side of a wall.', 'return select(vec3f(0.05, 0.15, 0.5), vec3f(0.8, 0.35, 0.05), spot.side == 1u);');
  look('cache point', 'How far its point is', 'From the place to the point that speaks for it. Yellow: a cell away.', 'return show_heat(distance(hit.position, at.position) / (knobs.cache_cell * 1.2));');
  views.add({
    id: 'cache off', group: 'The cache, where you look', name: 'All else, off the truth',
    about: 'What is kept beside what the path tracer finds at the same place: red too bright, blue too dark, full color at one stop.',
    reads: ['hit_x', 'cache', 'cache_points', 'truth'], uses: ['ray', 'dice', 'cache'],
    code: `
  let hit = hit_x_at(at_render(p));
  if (hit.distance < 0.0 || hit.lamp != NOTHING) { return vec3f(0.02); }
  let spot = cache_spot(hit.position, hit.normal);
  if (spot.slot == NOTHING) { return vec3f(0.16, 0.0, 0.13); }
  let t = truth_at(at_render(p));
  return show_difference(brightness(cache[spot.slot].known), brightness(t.sky + t.one + t.two + t.more));`,
  });
}
