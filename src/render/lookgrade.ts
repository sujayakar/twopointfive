// THE LOOK PROTOTYPE — THE GRADING ROWS, the frame's side (the look panel's Contrast row; render/frame.ts lookConfig.gradeC; the tone map's side is post.wgsl
// agx, which turns the picture's stops about the stop this file names). The maintainer, 10-29, of midday on a GPU: "image looks a little flat in midday … maybe
// needs some tuning for exposure/contrast/etc.? that's what i'd reach for in lightroom". PICTURE ONLY. The check: tools/qa/lookgrade-check.ts.
import { smoothstep } from '../math/vec';

export const LOOK_GRADE = {
  albedo: 0.85,     // the ground the contrast turns about sends this much of its light back (lying snow)
  minEv: -12.47393, stops: 16.5,   // the tone map's range (post.wgsl agx): a radiance of 2^minEv is its 0, `stops` over that its 1
  grey: 0.606,      // middle grey (0.18) on that scale: the turn is never under it …
  top: 0.9,         // … and never over this
  night: [0.08, 0.35] as const,   // the row fades out as the sky's own mean radiance falls through this span (the shade row's own: engine.ts lookShade)
};
/** [the contrast the tone map takes, the stop it turns about (0‥1 of the tone map's range)]. BY DAY ONLY, as the shade row is: a night's picture is the game's.
 *  The stop is the one SUNLIT GROUND sits on at this hour: what level ground of `albedo` sends back under this sky (`skyE` = the irradiance it gets from the sky
 *  alone) and this sun (the first directional light that is on and is no flash), at this `exposure`. So the snow stays where the hour's exposure put it and
 *  whatever is darker goes down. `skyUp` = the sky's own mean radiance (how much of a day it is); `amount` = the row (1 = the tone map as it is). */
export function lookGrade(skyUp: number, skyE: number, lights: readonly { kind: number; enabled: boolean; transient?: boolean; intensity: number; dir: ArrayLike<number>; color: ArrayLike<number> }[], exposure: number, amount: number): [number, number] {
  const K = LOOK_GRADE, day = smoothstep(K.night[0], K.night[1], skyUp), sun = lights.find(l => l.kind === 2 && l.enabled && !l.transient && l.intensity > 0);
  const E = Math.max(0, skyE) + (sun ? sun.intensity * Math.max(0, -sun.dir[1]) * (0.2126 * sun.color[0] + 0.7152 * sun.color[1] + 0.0722 * sun.color[2]) : 0);
  const stop = (Math.log2(Math.max(K.albedo * E / Math.PI * exposure, 1e-9)) - K.minEv) / K.stops;
  return [1 + (amount - 1) * day, Math.min(K.top, Math.max(K.grey, stop))];
}
/** THE DAY'S CURVE (the look panel's Day curve row; post.wgsl dayCurve; docs/agents/render.md has the why). The tone map's own curve spreads sixteen and a
 *  half stops over the screen, which a night needs, and its top is flat: by day everything pale and sunlit (ground of 0.25 to 0.85) lands in two fifths of a
 *  stop of screen, and the Contrast row's turn, made to give a day its blacks, sets whatever is under sunlit snow a third further down, so a shadow under a
 *  roof is fifteen times darker than the stone beside it and three times bluer than the sky that fills it. A camera's curve is nearly straight under its
 *  white: this is that. live: __look.day */
export const LOOK_DAY = {
  slope: 0.75,     // stops of screen a stop of the scene's BRIGHTNESS is given under the white (1 = as the light is; a print's 0.75 opens the shade)
  chroma: 1,       // … and a stop BETWEEN a colour's own channels (1 = a paint keeps its colour whatever the slope; = slope: each channel by itself, which greys a paint)
  knee: 0.7,       // of the screen's linear range: over it the curve rolls off toward white (a glint, a lamp's glass, the sky in a pane)
  head: 0.17,      // stops over sunlit ground of LOOK_GRADE.albedo at which the screen is white: lying snow in full sun is shown at nine tenths
  from: 0.74, full: 0.83,   // the curve comes in as that sunlit ground climbs through this span of the night curve's range: none of it at dusk (0.71 there) or dawn, all of it at a clear midday (0.85), part on a grey day
};
/** [how far the day's curve stands in for the night's (0‥1), the stop of its white (0‥1 of the tone map's range)] for the stop sunlit ground sits on this
 *  frame (lookGrade's second number) and the row. HOW FAR is asked of the light and not of the clock: the night's curve is right while sunlit ground sits in
 *  its middle and wrong once it sits in its flat top. */
export function lookDay(stop: number, amount: number): [number, number] {
  const K = LOOK_DAY; return [Math.max(0, Math.min(1, amount)) * smoothstep(K.from, K.full, stop), stop + K.head / LOOK_GRADE.stops];
}
/** the frame's gradeCfg for a lookGrade answer, or null (no turn: the frame's field stays zeros): x = 1 ∕ the contrast, y = the RADIANCE the turn is about,
 *  before the exposure (the composite's own units). THE DAY'S CONTRAST IS THE OPEN AIR'S: the shade row and Sam's eye bring a room up to a level the picture
 *  can show, and a turn about sunlit ground would push it back down by a third of its distance under the snow. So the one exposure model tones a place
 *  as the open air is and only then shows it as much darker as it is to be (place_common.wgsl lookExpoFor's line on gradeCfg.x), and a room is shown as
 *  it was with the row at 1. */
export function lookGradeFrame(g: readonly [number, number], exposure: number): [number, number, number, number] | null {
  return g[0] > 1 && exposure > 0 ? [1 / g[0], Math.pow(2, g[1] * LOOK_GRADE.stops + LOOK_GRADE.minEv) / exposure, 0, 0] : null;
}
