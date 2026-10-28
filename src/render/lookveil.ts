// THE LOOK PROTOTYPE's BLOWING SNOW, the frame's side (the look panel's "Blowing snow" row, render/frame.ts lookConfig.veil / .wind; drawn by
// shaders/composite.wgsl lookVeil on open snow pixels). The maintainer, 10-28: "prototype getting the wisps of snow from something like the fluid sim … those
// should all be reactive to the wind". Picture only: nothing in the game reads it. The check: tools/qa/lookveil-check.ts.
import { smoothstep } from '../math/vec';

/** THE LOOK PROTOTYPE, blowing snow (frame.veilA / veilB; composite.wgsl lookVeil). speedK: × the wind for how fast the veil runs (the snow's weather blows
 *  0.85 m/s, where powder that reads as blowing runs three or four: the game's own wisps take × 5 for the same reason). from ‥ full: the wind (m/s, as the game
 *  has it this frame, gust and all) the veil comes in over, so it thins in a lull and thickens in a gust, and still air has none. */
export const LOOK_VEIL = { speedK: 4, from: 0.45, full: 0.95 };
/** this frame's two fields. `wind` = the storm's air at the ground (m/s, x z; null = none); `drift` = how far the patches have run, kept by the caller and moved
 *  on here by the wind × dt (summed, so a gust speeds the veil up and never jumps it); the sun (or the moon) = the first directional light that is on. */
export function lookVeilFrame(wind: [number, number] | null, amount: number, dt: number, drift: [number, number], lights: readonly { kind: number; enabled: boolean; transient?: boolean; intensity: number; dir: ArrayLike<number> }[]): [[number, number, number, number], [number, number, number, number]] | null {
  if (!wind) return null;
  const K = LOOK_VEIL, wx = wind[0] * K.speedK, wz = wind[1] * K.speedK, step = Math.max(0, Math.min(dt, 0.1));
  drift[0] += wx * step; drift[1] += wz * step;
  const k = Math.min(1, amount) * smoothstep(K.from, K.full, Math.hypot(wind[0], wind[1])); if (!(k > 0)) return null;
  const sun = lights.find(l => l.kind === 2 && l.enabled && !l.transient && l.intensity > 0);
  const h = sun && sun.dir[1] < -0.05 ? Math.max(-sun.dir[1], 0.25) : 0;
  return [[wx, wz, k, 0], [drift[0], drift[1], h ? -sun!.dir[0] / h : 0, h ? -sun!.dir[2] / h : 0]];
}
