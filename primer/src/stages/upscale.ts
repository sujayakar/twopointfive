// THE LAST STAGE BEFORE THE SCREEN: from render size to output size.

import { f32, struct, vec3f } from '../gpu/layout';
import type { Graph } from '../gpu/graph';
import type { Views } from '../page/views';
import { choice, slider, toggle } from '../page/knobs';
import code from './upscale.wgsl';

const Upscaled = struct('Upscaled', {
  color: vec3f('the picture at output size'),
  weight: f32('samples behind it', { look: { kind: 'heat', from: 0, to: 32 } }),
});

choice('upscale_by', {
  label: 'Upscaling', value: 2, options: [['nearest', 'Nearest'], ['linear', 'Linear'], ['time', 'Over time']],
  about: 'Nearest shows the render pixels. Linear blurs them. Over time keeps, for each output pixel, a mean of the jittered samples that fell near it: more detail than any one frame has.',
});
slider('upscale_frames', { label: 'Samples kept', min: 1, max: 64, step: 1, value: 24, about: 'More is sharper and calmer, and slower to let go.' });
toggle('upscale_leash', { label: 'Hold to this frame’s neighbors', value: true, about: 'What is kept must lie between the colors round it now. Ghosts go; so does detail finer than a render pixel, which then flickers.' });

export function addUpscale(graph: Graph, views: Views): void {
  graph.addBuffer({ name: 'upscaled', struct: Upscaled, size: 'output', history: true, about: 'the picture at output size' });
  graph.addStage({
    name: 'upscale', question: 'What is the picture between the render pixels?',
    reads: { picture: 'picture', hit_x: 'hit_x', upscaled_before: 'upscaled@before' },
    writes: ['upscaled'], uses: [], over: 'output', code,
  });
  views.add({
    id: 'final', group: 'Pictures', name: 'The frame', about: 'Real time, at output size: what would go to the screen.',
    reads: ['upscaled'], code: 'return show_light(upscaled_at(vec2i(p)).color);',
  });
  views.add({
    id: 'sizes', group: 'Pictures', name: 'Buffers at their sizes', about: 'The frame, the sum at render size, wide light and mirrors as they are found: each drawn at its own number of pixels.',
    reads: ['upscaled', 'picture', 'wide_raw', 'narrow_raw'],
    code: `
  let full = vec2f(frame.output_size);
  let uv = (vec2f(p) + 0.5) / full;
  let spot = vec2f(p) * 2.0; // in the pixels of whatever is drawn here
  if (uv.x < 0.5 && uv.y < 0.5) { return show_light(upscaled_at(vec2i(spot)).color); }
  let a = spot - vec2f(full.x, 0.0);
  if (uv.y < 0.5) { if (all(a < vec2f(frame.render_size))) { return show_light(picture_at(vec2i(a)).light); } return vec3f(0.02); }
  let b = spot - vec2f(0.0, full.y);
  if (uv.x < 0.5) { if (all(b < vec2f(frame.wide_size))) { return show_light(wide_raw_at(vec2i(b)).value * 0.8); } return vec3f(0.02); }
  let c = spot - full;
  if (all(c < vec2f(frame.narrow_size))) { return show_light(narrow_raw_at(vec2i(c)).value * 0.8); }
  return vec3f(0.02);`,
  });
}
