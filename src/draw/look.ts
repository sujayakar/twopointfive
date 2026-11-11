// WHAT THE PAGE SAYS OF ITS LOOK to whoever packs the rows (rust/game_draw boxes::Look).
import { lookStuffOf } from '../render/lookTex';
import { SOLIDGLASS, coatRough, type BoxStuff } from '../scene/stuff';
import type { BoxWorld } from '../scene/boxes';
import type { RenderSettings } from '../settings';
import { placeholderTints } from '../render/charMaterials';
import { PageLook, pageLook, type Doubles } from './input';
import { SIDES } from './men';
import { STUFFS } from './sections';

const out = pageLook(), yes = (v: unknown): number => v ? 1 : 0;
/** (one row, written again by every ask) */
export function lookNumbers(S: RenderSettings, W: BoxWorld, cast: boolean): Doubles<'PageLook'> {
  out[PageLook.on] = 1; out[PageLook.marks] = yes(S.markings); out[PageLook.cut] = out[PageLook.round] = yes(W.cutOn); out[PageLook.motion] = yes(S.motionIds !== false); out[PageLook.panes] = yes(S.glass);
  out[PageLook.mirrors] = yes(S.beamReflect && S.rtReflect); out[PageLook.air] = yes(S.beamAir && S.debugView === 0);
  out[PageLook.glassLum] = SOLIDGLASS.on ? SOLIDGLASS.lum : -1; out[PageLook.glassRough] = SOLIDGLASS.rough; out[PageLook.texel] = Math.max(0.005, Math.min(0.2, S.markingTexel));
  for (let i = 0; i < STUFFS.length; i++) { out[PageLook.stuff + i] = lookStuffOf(STUFFS[i]); out[PageLook.coat + i] = coatRough(STUFFS[i] as BoxStuff) ?? -1; }
  for (let i = 0; i < SIDES.length; i++) out[PageLook.swaps + i] = cast ? placeholderTints(SIDES[i]) : 0;
  out[PageLook.wet] = yes(S.wetChars);
  return out;
}
