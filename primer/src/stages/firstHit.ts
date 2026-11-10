// P → x. What does each pixel look at?

import { f32, struct, u32, vec3f } from '../gpu/layout';
import type { Graph } from '../gpu/graph';
import code from './first_hit.wgsl';

export const HitX = struct('HitX', {
  position: vec3f('where the ray met the surface', { unit: 'm', look: { kind: 'place' } }),
  distance: f32('along the ray; negative where there is only sky', { unit: 'm', look: { kind: 'heat', from: 0, to: 12 } }),
  normal: vec3f('the way the surface faces', { look: { kind: 'direction' } }),
  object: u32('what it moves with; 0 is the room'),
  matte: vec3f('paint: the share sent back evenly, no light in it', { look: { kind: 'color' } }),
  roughness: f32('0 a mirror, 1 dull', { look: { kind: 'share' } }),
  through: vec3f('the share that comes out on the other side', { look: { kind: 'color' } }),
  metal: f32('1: only a coat, colored', { look: { kind: 'share' } }),
  glow: vec3f('what it sends by itself: lamps, and the sky', { look: { kind: 'light' } }),
  piece: u32('which piece'),
  position_before: vec3f('where this spot was last frame', { unit: 'm', look: { kind: 'place' } }),
  lamp: u32('which lamp it is, if it shines'),
  trust: f32('how far the past can be trusted here', { look: { kind: 'share' } }),
  steps: f32('tree nodes the ray looked at', { look: { kind: 'heat', from: 0, to: 30 } }),
});

export function addFirstHit(graph: Graph): void {
  graph.addBuffer({ name: 'hit_x', struct: HitX, size: 'render', history: true, about: 'what each pixel looks at' });
  graph.addStage({
    name: 'first hit',
    question: 'What does each pixel look at?',
    reads: { mesh_hit: 'mesh_hit_used' },
    writes: ['hit_x'],
    uses: ['ray', 'surface', 'sky', 'change'],
    over: 'render',
    code,
  });
}
