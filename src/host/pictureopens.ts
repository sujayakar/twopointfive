// HOW THE PICTURE OPENS, on whichever page draws: what the renderer is set to before a level or a preset says anything of its own. Every module here opens with
// its row as a tool finds it; a page that skips a line draws another picture, and nothing else says so.
import type { Engine } from '../engine';
import { bounceWallsConfig, glintPickConfig, highlightConfig, lookConfig, rainScaleConfig, rainShadowConfig, rainWallsConfig, reflWallsConfig, sharpCapConfig, sunGlintConfig, wallSidesConfig, weatherEdgeConfig, weatherVolConfig } from '../render/frame';
import { lookAmounts } from '../render/gbuffer';
import { LOOK_WORKERS } from '../render/lookWorkers';
import { MESH_MAX_CHARS } from '../render/meshbvh';
import { putSmokeLook, smokeLook } from '../render/smokeLook';
import { BoxWorld } from '../scene/boxes';
import { lookPageRows } from '../scene/look';
import { pageIndirect } from '../smoke/dispatch';
import { smokeGridConfig } from '../smoke/smokeGrid';
import type { SmokeSystem } from '../smoke/smoke';
import type { Quality } from '../ui/quality';
import { EVENPACE, FLIGHT_PAGE } from './pace';
import { PLAN_PACE } from '../render/planPace';

/** before the engine is made: what its shaders and its buffers are made by, by the address's words (each of them the picture as it was before the rule beside it) */
export function pictureWords(q: URLSearchParams): void {
  LOOK_WORKERS.on = q.get('lookworkers') !== '0';   // =0: every stuff's map on the page's own thread
  smokeLook.on = q.get('smokelook') !== '0';        // =0: smoke as the engine and the presets have it
  // ?look=<row>:<amount>,…: the look's amounts, as its panel's sliders would put them
  for (const kv of lookPageRows(q).join(',').split(',')) { const [k, v] = kv.split(':'); if (Object.hasOwn(lookAmounts, k) && v !== undefined && v !== '' && isFinite(+v)) (lookAmounts as Record<string, number>)[k] = Math.min(4, Math.max(0, +v)); }
  wallSidesConfig.on = lookAmounts.walls > 0;
  if (q.get('longfoot') === '0') lookConfig.longFoot = 0;   // the wall's band of wear however long its face
  if (q.get('walked') === '0') lookConfig.walked = 0;       // the traffic of a level's routes and thresholds alone
  if (q.get('firesnow') === '0') lookConfig.fireSnow = 0;
  if (q.has('nowallfog')) weatherEdgeConfig.on = false;     // a weather's fog classed by the surface point alone
  if (q.has('flatfog')) weatherVolConfig.on = false;        // a weather's dust and fog as a flat veil
  if (q.has('norainwalls')) rainWallsConfig.on = false;     // the rain hidden from every pixel under a roof's rectangle
  if (q.has('norainscale')) rainScaleConfig.on = false;     // a splash ring as bright however wide its texel
  if (q.has('norainshadow')) rainShadowConfig.on = false;   // the rain lit by the reach masks alone
  if (q.has('nobouncewalls')) bounceWallsConfig.on = false; // the bounce read through a wall
  if (q.has('noreflwalls')) reflWallsConfig.on = false;     // …and by a metal's rays
  if (q.get('sharpcap') === '0') sharpCapConfig.on = false;
  if (q.get('glintpick') === '1') glintPickConfig.on = true;
  if (q.get('hireach') === '0') highlightConfig.reach = 0;
  if (q.has('nosunglint')) sunGlintConfig.on = false;
  if (q.get('allproxies') === '0') BoxWorld.castCap = MESH_MAX_CHARS;   // shadows and reflections of the first eight men drawn alone
  { const n = parseInt(q.get('smokepool') || '', 10); if (n >= 256) smokeGridConfig.cap = Math.min(8192, Math.ceil(n / 256) * 256); }   // the smoke solver's pool, in bricks
}
/** how the page keeps its pace (host/pace.ts): ?pace=0 = every display frame worked; ?giveway=0 = a window nobody is at draws as another does; ?flight=0 = nothing waits for the GPU, =1 one frame with it at a
 *  time; ?planpace=old = the picture's planners made whole on the frame they are owed */
export function pictureKeepsPace(q: URLSearchParams): void {
  EVENPACE.on = q.get('pace') !== '0'; EVENPACE.giveway = q.get('giveway') !== '0';
  EVENPACE.flight = q.get('flight') === '0' ? 0 : q.get('flight') === '1' ? 1 : FLIGHT_PAGE; PLAN_PACE.on = q.get('planpace') !== 'old';
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
/** how the smoke solver goes about a step, by the adapter that runs it (smoke/dispatch.ts): ?smokeindirect=1 = by the GPU's own count on any card, =0 = over the whole pool on any adapter */
export function pictureSolves(q: URLSearchParams, engine: Engine, smoke: SmokeSystem): void { smoke.sparse.indirect = pageIndirect(q.get('smokeindirect'), engine.gpu.adapterFallback === true); }
