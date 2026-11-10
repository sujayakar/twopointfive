// Every stage, in the order they run.

import type { Graph } from '../gpu/graph';
import { choice, slider, toggle } from '../page/knobs';
import type { Views } from '../page/views';
import { addFirstHit } from './firstHit';
import { addBars } from './bars';
import { addDisc } from './disc';
import { addSettled } from './denoise';
import { addLamps } from './lamps';
import { addPicture, addTruePicture } from './picture';
import { addRack } from './rack';
import { SIGNALS } from './signals';
import { addTruth } from './truth';

choice('dice_kind', {
  label: 'Dice', value: 1, options: [['white', 'White'], ['filling', 'Filling'], ['blue', 'Blue']],
  about: 'White: every throw by itself. Filling: a pixel’s throws cover the square evenly over time. Blue: neighbors also throw differently, so the noise is fine-grained and a blur removes it.',
});
toggle('dice_frozen', { label: 'Freeze the dice', about: 'The same throws every frame: noise stands still so that you can look at it.' });
slider('render_scale', {
  label: 'Render size', min: 0.25, max: 1, step: 0.05, value: 1, resetsTruth: true,
  about: 'Render pixels for each output pixel, along one side. The rest is upscaling’s work.',
});
toggle('jitter', { label: 'Jitter the camera', about: 'Each frame’s rays go through another spot of their pixels. Upscaling needs it; without upscaling it only shakes the picture.' });
slider('exposure', { label: 'Exposure', min: -8, max: 8, step: 0.05, value: 0, unit: 'stops', about: 'Locked: it never adapts by itself, so two pictures can be compared.' });
choice('tone', {
  label: 'Shown through', value: 0, options: [['curve', 'Curve'], ['clip', 'Clip'], ['stops', 'Stops']],
  about: 'A curve that rolls bright light off; a straight line that clips; or false color, a band a stop.',
});

const SURFACE_X = `
/// x, as a surface
fn surface_x(hit: HitX) -> Surface {
  return Surface(hit.position, hit.normal, normalize(frame.eye - hit.position), hit.matte, hit.through, hit.roughness, hit.metal);
}`;

export function addStages(graph: Graph, views: Views): void {
  graph.addModule('x', { needs: ['material'], shared: [], code: SURFACE_X });
  addFirstHit(graph);
  addTruth(graph);
  addTruePicture(graph, views);
  addLamps(graph, views);
  addRack(graph, views);
  for (const { id, size } of SIGNALS) addSettled(graph, id, size);
  addPicture(graph, views);
  addDisc(graph, views);
  addBars(graph);

  graph.addAlias('main_picture', () => (graph.buffers.has('view picture') ? 'picture' : 'true_picture'));
  views.add({
    id: 'ghost', group: 'Pictures', name: 'From the ghost', about: 'The frozen frame, seen by another camera. What the frame does not see is grey.',
    reads: ['hit_x', 'main_picture'], uses: ['ray', 'surface'],
    code: `
  let uv = (vec2f(p) + 0.5) / vec2f(frame.output_size) * 2.0 - 1.0;
  let toward = normalize(frame.ghost_forward + frame.ghost_right * uv.x - frame.ghost_up * uv.y);
  let ray = Ray(frame.ghost_eye, toward, 0.0, 1e4);
  let met = nearest(ray);
  if (met.piece == NOTHING) { return vec3f(0.02); }
  let found = found_at(ray, met);
  let seen = on_screen(found.surface.position);
  let grey = vec3f(0.05 + 0.04 * dot(met.normal, vec3f(0.3, 0.9, 0.2)));
  if (seen.depth <= 0.0 || any(seen.uv < vec2f(0.0)) || any(seen.uv >= vec2f(1.0))) { return grey; }
  let at = vec2i(seen.uv * vec2f(frame.render_size));
  let hit = hit_x_at(at);
  if (distance(hit.position, found.surface.position) > 0.02 * hit.distance) { return grey; }
  return show_light(main_picture_at(at).light);`,
  });
  const landing = (id: string, name: string, about: string, field: string) =>
    views.add({
      id, group: 'Truth, part by part', name, about,
      reads: ['truth'], code: `return show_light(truth_at(at_render(p)).${field} * 0.8);`,
    });
  landing('truth lamps', 'Straight from lamps', 'What lands, as a white wall would send it back.', 'still');
  landing('truth sky', 'Straight from the sky', 'What lands, as a white wall would send it back.', 'sky');
  landing('truth one', 'One bounce', 'What lands, as a white wall would send it back.', 'one');
  landing('truth two', 'Two bounces', 'What lands, as a white wall would send it back.', 'two');
  landing('truth more', 'More bounces', 'What lands, as a white wall would send it back.', 'more');
}
