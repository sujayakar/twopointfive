// The look prototype's Contrast row, the frame's side (src/render/lookgrade.ts lookGrade; the tone map's side is post.wgsl agx, mirrored below): by day the
// picture's stops are turned about the stop sunlit ground sits on, so the snow keeps its place and what is darker goes down. No GPU: the picture is never SEEN here.
//   bun run tools/qa/lookgrade-check.ts   (exit 0 = all pass; LOOKGRADE_BREAK=1 turns the stops about middle grey and must fail)
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { LOOK_GRADE, lookGrade } from '../../src/render/lookgrade';
let red = 0; const check = (ok: boolean, w: string) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${w}`); if (!ok) red++; };
const BREAK = process.env.LOOKGRADE_BREAK === '1', K = LOOK_GRADE;
const sun = { kind: 2, enabled: true, intensity: 4.18, dir: [0.145, -0.829, -0.54], color: [1, 0.97, 0.9] };
const grade = (up: number, E: number, lights: Parameters<typeof lookGrade>[2], exp: number, amount: number): [number, number] => { const g = lookGrade(up, E, lights, exp, amount); return BREAK ? [g[0], K.grey] : g; };
/** post.wgsl agx's grading line for a grey (highlights and shadows at rest): a stop v (0‥1 of the range) after the turn */
const turn = (v: number, g: readonly number[]) => Math.min(1, Math.max(0, g[1] + (v - g[1]) * g[0]));
const stopOf = (radiance: number) => (Math.log2(radiance) - K.minEv) / K.stops;
{ // the pass at midday under half cloud: the sky gives the ground 2.4, the sun 3.4 more, at an exposure of 1.7
  const g = grade(0.84, 2.4, [sun], 1.7, 1.4), snow = stopOf(0.85 * (2.4 + 4.18 * 0.829 * 0.9659) / Math.PI * 1.7), rifle = snow - 4 / K.stops, shadow = stopOf(0.85 * 2.4 / Math.PI * 1.7);
  check(g[0] === 1.4 && Math.abs(turn(snow, g) - snow) < 0.004, `by day the row is the contrast outright, and sunlit snow keeps its stop (${snow.toFixed(3)} of the range; the turn is about ${g[1].toFixed(3)})`);
  check(Math.abs((snow - turn(rifle, g)) * K.stops - 5.6) < 0.1 && turn(shadow, g) < shadow - 0.01, `a black rifle in the sun, four stops under the snow, is shown ${((snow - turn(rifle, g)) * K.stops).toFixed(1)} under it, and a shadow on the snow ${((snow - turn(shadow, g)) * K.stops).toFixed(1)} stops under it where it was ${((snow - shadow) * K.stops).toFixed(1)}`);
  const one = grade(0.84, 2.4, [sun], 1.7, 1); check(one[0] === 1 && turn(rifle, one) === rifle, 'the row at 1 is the tone map as it is'); }
{ const night = grade(0.03, 0.1, [{ ...sun, intensity: 0.06 }], 1, 1.4), dusk = grade(0.2, 0.7, [{ ...sun, intensity: 1.5, dir: [0.6, -0.37, -0.7] }], 2, 1.4);
  check(night[0] === 1 && dusk[0] > 1 && dusk[0] < 1.4, `a night is left alone whatever the row says, and the row fades in with the sky (dusk here: × ${dusk[0].toFixed(2)})`); }
{ const flash = grade(0.84, 2.4, [{ ...sun, transient: true, intensity: 400 }, sun], 1.7, 1.4), plain = grade(0.84, 2.4, [sun], 1.7, 1.4), off = grade(0.84, 2.4, [{ ...sun, enabled: false }], 1.7, 1.4), dark = grade(0.84, 1e-7, [], 1e-3, 1.4), hot = grade(0.84, 900, [sun], 40, 1.4);
  check(flash[1] === plain[1] && off[1] < plain[1] && dark[1] === K.grey && hot[1] === K.top, 'a lightning stroke is not the sun; with the sun off the turn is about the sky-lit ground; and it never leaves the span from middle grey to 0.9'); }
{ const s = readFileSync(resolve(import.meta.dir, '../../src/render/shaders/post.wgsl'), 'utf8');
  check(/v = post\.gradeP \+ \(v - post\.gradeP\) \* post\.gradeC/.test(s) && /if \(post\.gradeC != 1\.0 \|\| post\.gradeH != 0\.0 \|\| post\.gradeS != 0\.0\)/.test(s) && Math.abs(stopOf(0.18) - K.grey) < 0.001,
    'the tone map turns its stops about that stop, and with every row at rest it does not run the line at all'); }
process.exit(red ? 1 : 0);
