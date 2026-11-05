// The look prototype's Contrast row, the frame's side (src/render/lookgrade.ts lookGrade; the tone map's side is post.wgsl agx, mirrored below): by day the
// picture's stops are turned about the stop sunlit ground sits on, so the snow keeps its place and what is darker goes down. No GPU: the picture is never SEEN here.
//   bun run tools/qa/lookgrade-check.ts   (exit 0 = all pass; LOOKGRADE_BREAK=1 turns the stops about middle grey and must fail)
// AND THE DAY'S CURVE (the Day curve row; lookgrade.ts LOOK_DAY, lookDay; post.wgsl dayCurve, mirrored below): LOOKDAY_BREAK=1 puts its white at middle
// grey and must fail. What the mirror says is the curve's arithmetic for a grey and for two channels a stop apart; what a level looks like under it is a page's.
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { LOOK_GRADE, LOOK_DAY, lookGrade, lookGradeFrame, lookDay } from '../../src/render/lookgrade';
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
{ // the day's contrast is the open air's: what the frame hands the one exposure model (place_common.wgsl lookExpoFor tones a place as the open air is and
  // then shows it as much darker as it is to be; that arithmetic is tools/qa/lookexpo-check.ts's)
  const g = grade(0.84, 2.4, [sun], 1.7, 1.4), f = lookGradeFrame(g, 1.7)!, snowR = 0.85 * (2.4 + 4.18 * 0.829 * 0.9659) / Math.PI;
  const common = readFileSync(resolve(import.meta.dir, '../../src/render/shaders/place_common.wgsl'), 'utf8');
  check(!!f && Math.abs(f[0] - 1 / 1.4) < 1e-12 && Math.abs(f[1] - snowR) < 0.02 * snowR && lookGradeFrame([1, 0.84], 1.7) === null && common.includes('  if (frame.gradeCfg.x > 0.0) { base *= pow(max(rp / base, 1e-6), 1.0 - frame.gradeCfg.x); }'),
    'the frame hands the model 1 over the contrast and the radiance of sunlit snow the turn is about, nothing with the row at 1, and the model tones a place by them before it is shown darker'); }
{ // ---- THE DAY'S CURVE
  const DBREAK = process.env.LOOKDAY_BREAK === '1', D = LOOK_DAY;
  /** post.wgsl agxContrast, and its own punch after (pow 1.15): a grey's stop → 0‥255 on the screen */
  const night = (v: number) => { const x2 = v * v, x4 = x2 * x2; return 255 * Math.pow(Math.max(0, 15.5 * x4 * x2 - 40.14 * x4 * v + 31.96 * x4 - 6.868 * x2 * v + 0.4298 * x2 + 0.1191 * v - 0.00232), 1.15); };
  /** post.wgsl dayCurve for one channel at stop v whose colour's brightness is at stop g (a grey: g = v), white at stop W: the screen's LINEAR light, and 0‥255 after the 2.2 and the punch */
  const sm = (a: number, b: number, x: number) => { const q = Math.min(1, Math.max(0, (x - a) / (b - a))); return q * q * (3 - 2 * q); };
  const dayLin = (v: number, g: number, W: number) => { const d = (g - W) * K.stops, u = d + D.toe, b = d * D.slope + 0.5 * (u - Math.sqrt(u * u + 0.5625)) * (D.under - D.slope), c = D.chroma - (D.chroma - D.slope) * D.highs * sm(-1.5, 0, d);
    const x = 2 ** (b + (v - g) * K.stops * c), k = D.knee; return Math.min(1, Math.max(0, x < k ? x : k + (1 - k) * (1 - Math.exp(-(x - k) / (1 - k))))); };
  const day = (v: number, W: number) => 255 * Math.pow(Math.pow(dayLin(v, v, W), 1 / 2.2), 1.15);
  // a clear midday as game/hours.ts has it: the sun 6.0 at 56 degrees, the sky's zenith 0.3 0.45 0.8 (π × its luminance on level ground), exposure 1.7
  const noon = { kind: 2, enabled: true, intensity: 6.0, dir: [0.145, -0.829, -0.54], color: [1, 0.97, 0.9] }, skyE = Math.PI * 0.443, E = skyE + 6.0 * 0.829 * 0.9659;
  const g = lookGrade(0.6, skyE, [noon], 1.7, 1.35), d = lookDay(g[1], 1), W = DBREAK ? K.grey : d[1];
  const sunlit = (albedo: number) => stopOf(albedo * E / Math.PI * 1.7), A = [0.25, 0.47, 0.66, 0.85];
  const dv = A.map(a => day(sunlit(a), W)), nv = A.map(a => night(turn(sunlit(a), g)));
  check(d[0] === 1 && Math.abs(dv[0] - 148) <= 8 && Math.abs(dv[1] - 192) <= 8 && Math.abs(dv[2] - 218) <= 8 && Math.abs(dv[3] - 235) <= 6 && dv[3] - dv[0] >= 78 && nv[3] - nv[0] <= 70,
    `at a clear midday the curve is all the day's, and sunlit ground of 0.25 / 0.47 / 0.66 / 0.85 is shown at ${dv.map(v => v.toFixed(0)).join(' / ')} of 255 (a camera's 148 / 192 / 218 / 235), ${(dv[3] - dv[0]).toFixed(0)} apart end to end where the night's curve with the Contrast row's turn has them at ${nv.map(v => v.toFixed(0)).join(' / ')}, ${(nv[3] - nv[0]).toFixed(0)} apart`);
  // a shadow on the 0.47 stone that holds an eighth of its light (three stops under), one under a roof that holds a thirty-second (five), and a void that
  // holds a thousandth (ten): how far under the stone each is SHOWN, in the screen's own stops
  const stone = sunlit(0.47), lin = (v: number) => Math.pow(v / 255, 2.2 / 1.15), under = (a: number, b: number) => Math.log2(lin(a) / lin(b));
  const dU = [3, 5, 10].map(n => under(day(stone, W), day(stone - n / K.stops, W))), nU = [3, 5].map(n => under(night(turn(stone, g)), night(turn(stone - n / K.stops, g))));
  check(dU[0] > 3 * D.slope - 0.05 && dU[0] < 3 && dU[1] > 5 * D.slope && dU[1] < nU[1] - 0.8 && dU[2] > 10 * D.slope + 1.5 && dU[2] > 9,
    `a shadow three stops under the stone beside it is shown ${dU[0].toFixed(2)} stops under it and one five under ${dU[1].toFixed(2)}, where the night's curve with the turn shows them ${nU[0].toFixed(2)} and ${nU[1].toFixed(2)} under (it steepens as it goes down, so the deeper the shade the further it is crushed); and a void ten stops under is shown ${dU[2].toFixed(2)} under: the toe takes the dark end back to black, where the top's slope alone would leave it ${(10 * D.slope).toFixed(1)} under, a grey veil`);
  // two channels a stop apart, well under the knee: they stay `chroma` of a stop apart on the screen's linear light, whatever the slope
  { const gS = stone - 2 / K.stops, a = dayLin(gS + 0.5 / K.stops, gS, W), b = dayLin(gS - 0.5 / K.stops, gS, W), gT = sunlit(0.85), a2 = dayLin(gT + 0.25 / K.stops, gT, W), b2 = dayLin(gT - 0.25 / K.stops, gT, W), top = Math.log2(a2 / b2) / 0.5;
    check(Math.abs(Math.log2(a / b) - D.chroma) < 1e-9 && D.chroma >= D.slope && top < D.chroma - 0.02 && top > D.slope - 0.25,
      `a colour's channels a stop apart are shown ${Math.log2(a / b).toFixed(2)} of a stop apart two stops under the stone (a paint keeps its colour; each channel by the slope alone would leave ${D.slope}), and ${top.toFixed(2)} at sunlit snow, where a share goes channel by channel and the shoulder closes them: snow in the sun is white`); }
  // how far it comes in is asked of the light: none where sunlit ground sits in the night curve's middle, all of it in its flat top; the row scales it
  { const dusk = lookGrade(0.25, Math.PI * 0.17, [{ ...noon, intensity: 2.2, dir: [0.9, -0.375, 0.2], color: [1, 0.5, 0.26] }], 2.0, 1.35)[1];
    check(lookDay(dusk, 1)[0] === 0 && lookDay(g[1], 0)[0] === 0 && lookDay(g[1], 0.5)[0] === 0.5 && Math.abs(lookDay((D.from + D.full) / 2, 1)[0] - 0.5) < 1e-9 && lookDay(0.9, 7)[0] === 1 && Math.abs(lookDay(g[1], 1)[1] - (g[1] + D.head / K.stops)) < 1e-12,
      `sunlit ground sits at ${g[1].toFixed(3)} of the night curve's range at midday and ${dusk.toFixed(3)} at dusk: the day's curve is all in at the first and not in at the second (it comes in from ${D.from} to ${D.full}); the row at 0 is none of it at any hour, at a half is half; its white is ${D.head} of a stop over sunlit snow`); }
  { const post = readFileSync(resolve(import.meta.dir, '../../src/render/shaders/post.wgsl'), 'utf8'), eng = readFileSync(resolve(import.meta.dir, '../../src/engine.ts'), 'utf8'), gb = readFileSync(resolve(import.meta.dir, '../../src/render/gbuffer.ts'), 'utf8'), pa = readFileSync(resolve(import.meta.dir, '../../src/render/passes.ts'), 'utf8');
    check(post.includes('  if (post.gradeD > 0.0) { v = mix(agxContrast(v), dayCurve(v), post.gradeD); } else { v = agxContrast(v); }') && post.includes('  let b = d * post.gradeSl + 0.5 * (u - sqrt(u * u + 0.5625)) * (post.gradeSu - post.gradeSl);') && post.includes('  let c = post.gradeCs - (post.gradeCs - post.gradeSl) * post.gradeHi * smoothstep(-1.5, 0.0, d);') && post.includes('  let x = exp2(b + (v - g) * 16.5 * c);') && post.includes('  let y = select(k + (1.0 - k) * (1.0 - exp(-(x - k) / (1.0 - k))), x, x < vec3f(k));')
      && eng.includes('const d = lookConfig.gradeDay > 0 ? lookDay(g0[1], lookConfig.gradeDay) : null, g: [number, number] = d && d[0] > 0 ? [1 + (g0[0] - 1) * (1 - d[0]), g0[1]] : g0;') && eng.includes('P.gradeC = 1; P.gradeD = 0;')
      && /LOOK_DEFAULTS = \{[^}]*\bday: 0 \}/.test(gb) && pa.includes('d[14] = this.gradeD; d[15] = this.gradeW; d[16] = this.gradeSl; d[17] = this.gradeCs;') && pa.includes('d[19] = this.gradeSu; d[20] = this.gradeToe; d[21] = this.gradeHi;'),
      'the tone map runs the day\'s curve only by as much as the frame says and not at all at 0, by the formula mirrored here; the Contrast row\'s turn stands down by as much as the curve stands in; and the row opens at 0'); }
}
process.exit(red ? 1 : 0);
