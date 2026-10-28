// THE LOOK PROTOTYPE — THE EXPOSURE FOLLOWS SAM, the frame's side (the look panel's "Eyes follow Sam" row, render/frame.ts lookConfig.eyes / .eye; the shader's
// side is composite.wgsl lookShadeGain, which says what the three numbers do). The maintainer, 10-28: "i wonder if we need to adjust our tone mapping based on
// where the characters eyes are though. like in this setting on overcast the room should probably be a bit brighter (and then it's okay for outside to be a bit
// overexposed?)". By day only, as the shade row it rides on is. PICTURE ONLY: the guards and the light meter read the light as it is.
// The check: tools/qa/lookeye-check.ts.
import { smoothstep } from '../math/vec';

export const LOOK_EYE = {
  open: 0.9,     // s for the eye to open (Sam walks into the dark) …
  close: 0.35,   // … and to close (he steps out): faster, as an eye is
  more: 0.4,     // with Sam in the dark his place is brought up this much further toward the open air's own brightness than the shade row alone does ("a bit brighter")
  drop: 0.5,     // the share of a still darker place's own extra that is dropped (so a room keeps a dark end), once Sam's place is three times darker than the open
  out: 0.3,      // the outside is overexposed by his place's gain to this power ("a bit")
  cap: 0.5,      // with Sam in the dark a place may be brought up this much further than the shade row's own most (× 1.5)
};
/** the eye's state the caller keeps: ln(how many times darker than open ground the light on Sam is), eased; `snap` = take the next reading outright (the first
 *  frame, the frame after any with no reading, a restart: game/airSim.ts tells the engine; a still sets it by hand) */
export interface LookEye { log: number; snap: boolean }
/** this frame's eyeCfg (x = Sam's place, y = the share dropped, z = × the outside, w = the most a place is brought up), or null (the row at 0, a night, no reading: the frame's field stays zeros = off), and the eye moved on by `dt`. `shade` = this frame's
 *  shadeCfg (render lookShade: x = the shade row's exponent by day, z = the most a place is brought up, w = what open ground gets from the sky alone);
 *  `E` = the light on Sam as his meter has it, in the same units; `amount` = the row. */
export function lookEyeFrame(eye: LookEye, shade: readonly [number, number, number, number] | undefined, E: number | null | undefined, amount: number, dt: number): [number, number, number, number] | null {
  const K = LOOK_EYE, k = Math.min(1, amount), x = shade ? shade[0] : 0;
  if (!(k > 0) || !shade || !(x > 0) || !(shade[3] > 0) || E == null || !Number.isFinite(E) || !(E > 0)) { eye.log = 0; eye.snap = true; return null; }   // (0 is the meter before its first answer, not a place with no light at all by day: taken for a reading it threw the eye wide open on the page's first frames)
  const xs = x + K.more * (1 - x);   // (the exponent his own place is brought up by)
  const want = Math.min(Math.log(shade[2] * (1 + K.cap)) / xs, Math.log(Math.max(1, shade[3] / Math.max(E, 1e-6))));   // (darker than the cap can show changes nothing: the eye does not wait there on the way back out)
  if (eye.snap) { eye.log = want; eye.snap = false; }
  else eye.log += (want - eye.log) * (1 - Math.exp(-Math.max(0, Math.min(dt, 0.25)) / (want > eye.log ? K.open : K.close)));
  const deep = k * smoothstep(0, Math.log(3), eye.log), cap = shade[2] * (1 + K.cap * deep);
  const sam = Math.exp(eye.log * k * xs / x), gain = Math.min(cap, Math.pow(sam, x));
  return [sam, K.drop * deep, Math.pow(gain, K.out), cap];
}
