// THE LAMPS, from the pinned point: what each would send, and how much of it is seen. For the bar chart.

import { f32, struct, vec3f } from '../gpu/layout';
import type { Graph } from '../gpu/graph';

export const MOST_BARS = 80;
export const Bar = struct('Bar', {
  unblocked: vec3f('what it would land here, exact'),
  seen: f32('the share of it that is seen', { look: { kind: 'share' } }),
  color: vec3f('its own color'),
  count: f32('rays so far'),
});

export function addBars(graph: Graph): void {
  graph.addBuffer({ name: 'bars', struct: Bar, size: { count: MOST_BARS }, about: 'the lamps, from the pinned point' });
  graph.addStage({
    name: 'bars',
    question: 'What would each lamp send to the pinned point, and how much of it is seen?',
    reads: ['hit_x'],
    writes: ['bars'],
    uses: ['ray', 'lamps', 'dice'],
    over: { count: MOST_BARS },
    code: `
fn run(index: u32) {
  let hit = hit_x_at(vec2i(frame.pinned));
  var bar = bars[index];
  if (index >= world.lamp_count || frame.pinned_on == 0u || hit.distance < 0.0) { bars[index] = Bar(); return; }
  let lamp = lamps[index];
  bar.unblocked = light_unblocked(lamp, hit.position, hit.normal);
  bar.color = lamp.radiance;
  var seen = 0.0;
  for (var k = 0u; k < 8u; k++) {
    let aim = aim_at(lamp, hit.position, dice_of(DICE_KIND_WHITE, vec2u(index, k), frame.number, 950u));
    if (dot(aim.toward, hit.normal) <= 0.0 || brightness(aim.light) <= 0.0) { continue; }
    let start = leave_surface(hit.position, hit.normal, aim.toward);
    if (nearest(fine_ray(start, aim.toward, 1e-4, aim.distance - 2e-3)).piece == NOTHING) { seen += 0.125; }
  }
  bar.count += 1.0;
  bar.seen = mix(bar.seen, seen, 1.0 / bar.count);
  bars[index] = bar;
}`,
  });
}
