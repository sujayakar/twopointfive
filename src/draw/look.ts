// WHAT THE PAGE SAYS OF ITS LOOK to whoever packs the rows (rust/game_draw boxes::Look reads the same numbers in the same order).
import { lookConfig } from '../render/frame';
import { lookStuffOf } from '../render/lookTex';
import { SOLIDGLASS, coatRough, type BoxStuff } from '../scene/stuff';
import type { BoxWorld } from '../scene/boxes';
import type { RenderSettings } from '../settings';
import { placeholderTints } from '../render/charMaterials';
import { SIDES } from './men';
import { STUFFS } from './sections';

const N = STUFFS.length, AT = 11, out = new Float64Array(AT + 2 * N + SIDES.length);
/** (one array, written again by every ask) */
export function lookNumbers(S: RenderSettings, W: BoxWorld, cast: boolean): Float64Array {
  out[0] = 1; out[1] = S.markings ? 1 : 0; out[2] = W.cutOn ? 1 : 0; out[3] = W.roundOn ? 1 : 0; out[4] = S.motionIds !== false ? 1 : 0; out[5] = S.glass ? 1 : 0;
  out[6] = S.beamReflect && S.rtReflect ? 1 : 0; out[7] = S.beamAir && S.debugView === 0 ? 1 : 0; out[8] = SOLIDGLASS.on ? SOLIDGLASS.lum : -1; out[9] = SOLIDGLASS.rough; out[10] = Math.max(0.005, Math.min(0.2, S.markingTexel));
  for (let i = 0; i < N; i++) { out[AT + i] = lookStuffOf(STUFFS[i]); out[AT + N + i] = coatRough(STUFFS[i] as BoxStuff) ?? -1; }
  for (let i = 0; i < SIDES.length; i++) out[AT + 2 * N + i] = cast ? placeholderTints(SIDES[i]) : 0;
  return out;
}
