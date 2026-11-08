// HOW THE PICTURE OPENS, on whichever page draws: what the renderer is set to before a level or a preset says anything of its own. Every module here opens with
// its row as a tool finds it; a page that skips a line draws another picture, and nothing else says so (tools/draw/pair-page.cjs sets the two pages' settings side by side).
import type { Engine } from '../engine';
import { LOOK_WORKERS } from '../render/lookWorkers';
import { putSmokeLook, smokeLook } from '../render/smokeLook';
import type { Quality } from '../ui/quality';

/** before the engine is made: ?lookworkers=0 = every stuff's map on the page's own thread; ?smokelook=0 = smoke as the engine and the presets have it */
export function pictureWords(q: URLSearchParams): void {
  LOOK_WORKERS.on = q.get('lookworkers') !== '0';
  smokeLook.on = q.get('smokelook') !== '0';
}
/** the programs made before the first frame, beside the ones every picture needs: a preset puts these on at any tier, and one not asked for here is made inside the first frame that wants it */
export function pictureMakes(q: URLSearchParams): { sharp: boolean; gbaa: boolean } {
  const sharp = !(q.has('nosharp') || q.has('noiq') || q.has('notaau'));
  return { sharp, gbaa: sharp || q.has('gbaa') };
}
/** after the first preset is on: the rows that ride over it. ?nomotioncalm = every row as the preset has it */
export function pictureOpens(q: URLSearchParams, engine: Engine, quality: Quality): void {
  quality.applyMotionCalm(!q.has('nomotioncalm'));
  if (smokeLook.on) putSmokeLook(engine);
}
