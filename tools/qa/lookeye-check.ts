// The look prototype's "Eyes follow Sam" row, the frame's side (src/render/lookeye.ts lookEyeFrame; the shader's side is composite.wgsl lookShadeGain, mirrored
// below): the picture's exposure follows the light where Sam stands, a room keeps its fall-off from the window, and the outside is a little overexposed.
// No GPU: the picture itself is never SEEN here.
//   bun run tools/qa/lookeye-check.ts   (exit 0 = all pass; LOOKEYE_BREAK=1 hands the eye open ground's light wherever Sam stands and must fail)
import { LOOK_EYE, lookEyeFrame, type LookEye } from '../../src/render/lookeye';
let red = 0; const check = (ok: boolean, w: string) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${w}`); if (!ok) red++; };
const BREAK = process.env.LOOKEYE_BREAK === '1';
const shade: [number, number, number, number] = [0.7, 0.1, 6, 1];   // (the shade row at 0.7 by day; open air's mean radiance 0.1; open ground gets 1)
const eyeOf = (E: number | null, amount = 1, eye: LookEye = { log: 0, snap: true }, dt = 1 / 60) => lookEyeFrame(eye, shade, E !== null && BREAK ? shade[3] : E, amount, dt);
/** composite.wgsl lookShadeGain, on the CPU: `here` = the place's mean radiance, Epix = the pixel's own light */
const gain = (here: number, Epix: number, eye: readonly number[] | null) => {
  const e = eye ?? [0, 0, 0, 0], out = Math.max(e[2], 1); if (Epix >= shade[3]) return out;
  const own = Math.max(shade[1] / Math.max(here, 1e-4), 1), sam = Math.max(e[0], 1), dark = own > sam && e[1] > 0 ? own * Math.pow(sam / own, e[1]) : Math.max(own, sam);
  const place = Math.min(Math.max(shade[2], e[3]), Math.max(1, Math.pow(dark, shade[0])));
  return place / (1 + (place / out - 1) * Epix / shade[3]);
};
check(eyeOf(0.05, 0) === null && eyeOf(null) === null && lookEyeFrame({ log: 0, snap: true }, undefined, 0.05, 1, 1 / 60) === null && lookEyeFrame({ log: 0, snap: true }, [0, 0.1, 6, 1], 0.05, 1, 1 / 60) === null && eyeOf(NaN) === null && eyeOf(0) === null, 'the row at 0, no reading, no shade row or a night: no field (zeros = every place by its own darkness, as before)');
{ const out = eyeOf(1.4)!; check(!!out && out[0] === 1 && out[1] === 0 && out[2] === 1 && [0.1, 0.02, 0.005].every(h => Math.abs(gain(h, 0.01, out) - gain(h, 0.01, null)) < 1e-9), 'with Sam out in the open the picture is exactly the shade row\'s: every room is brought up by its own darkness'); }
{ // Sam in a room an eighth as bright as the open: his room's bright end (by the window, a third) and its middle take ONE gain; the shade row alone gives the window less
  const room = eyeOf(0.125)!, by = gain(0.033, 0.01, room), mid = gain(0.0125, 0.01, room), byWas = gain(0.033, 0.01, null), midWas = gain(0.0125, 0.01, null);
  check(!!room && Math.abs(by - mid) < 1e-9 && midWas / byWas > 1.5 && mid < shade[2], `in Sam's room the window's end and the middle are brought up alike (× ${by.toFixed(2)}), where the shade row alone gave the middle ${(midWas / byWas).toFixed(2)} times the window's: the fall-off it flattened is kept`);
  check(mid > midWas * 1.1 && mid < midWas * 1.6, `and his room is a bit brighter than the shade row had it: × ${mid.toFixed(2)} against × ${midWas.toFixed(2)}`);
  const deep = gain(0.001, 0.005, room), deepWas = gain(0.001, 0.005, null);
  check(deep >= mid && deep <= shade[2] * (1 + LOOK_EYE.cap) && deepWas > 0.9 * shade[2] && room[1] > 0 && room[1] <= LOOK_EYE.drop && room[3] > shade[2], 'a place darker still than his is brought up no less than his, and by no more than the cap');
  check(room[2] > 1.2 && room[2] < 2 && gain(0.1, 1.2, room) === room[2] && gain(0.1, 1.2, null) === 1, `the yard through the door is overexposed a little: × ${room[2].toFixed(2)}`); }
{ // the eye eases: slower into the dark than back out
  const eye: LookEye = { log: 0, snap: true }; eyeOf(1.4, 1, eye); const t = (E: number, till: (x: number) => boolean) => { let n = 0; while (n < 600 && !till(eyeOf(E, 1, eye)![0])) n++; return n / 60; };
  const lg = Math.log, first = eyeOf(0.125, 1, eye)![0], full = eyeOf(0.125)![0], open = t(0.125, x => lg(x) > 0.63 * lg(full)), close = t(1.4, x => lg(x) < 0.37 * lg(full));
  check(first > 1 && lg(first) < 0.05 * lg(full) && open > 0.5 && open < 1.5 && close < open && close > 0.1, `walking in, the eye opens over about ${open.toFixed(1)} s (not in one frame) and it closes again in ${close.toFixed(2)} s`);
  const still: LookEye = { log: 0, snap: true }; check(eyeOf(0.125, 1, still, 0)![0] === full, 'the first frame (a level put up, a still) takes the reading outright'); }
{ const half = eyeOf(0.125, 0.5)!, full = eyeOf(0.125)!; check(half[0] > 1 && half[0] < full[0] && half[2] < full[2], 'the row at a half follows him half way'); }
process.exit(red ? 1 : 0);
