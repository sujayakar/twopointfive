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
{ // ---- THE DAY'S CURVE. The mirror below is post.wgsl agx() whole, for a colour: inset, stops, the grading line, the two curves mixed, the punch and the
  // saturation (which stand down under the day's curve), outset, clamp. The last line here holds the shader to it by its text, line for line.
  const DBREAK = process.env.LOOKDAY_BREAK === '1', D = LOOK_DAY, LW = [0.2126, 0.7152, 0.0722], SAT = 1.15;   // (SAT: passes.ts saturation, the look's own)
  const IN = [[0.842479062253094, 0.0784335999999992, 0.0792237451477643], [0.0423282422610123, 0.878468636469772, 0.0791661274605434], [0.0423756549057051, 0.0784336, 0.879142973793104]];
  const OUT = [[1.19687900512017, -0.0980208811401368, -0.0990297440797205], [-0.0528968517574562, 1.15190312990417, -0.0989611768448433], [-0.0529716355144438, -0.0980434501171241, 1.15107367264116]];
  const mul = (M: number[][], v: number[]) => M.map(r => r[0] * v[0] + r[1] * v[1] + r[2] * v[2]), dot = (v: number[]) => LW[0] * v[0] + LW[1] * v[1] + LW[2] * v[2];
  const sm = (a: number, b: number, x: number) => { const q = Math.min(1, Math.max(0, (x - a) / (b - a))); return q * q * (3 - 2 * q); }, mix = (a: number, b: number, k: number) => a * (1 - k) + b * k;
  const sig = (v: number) => { const x2 = v * v, x4 = x2 * x2; return 15.5 * x4 * x2 - 40.14 * x4 * v + 31.96 * x4 - 6.868 * x2 * v + 0.4298 * x2 + 0.1191 * v - 0.00232; };
  const dayC = (v: number[], W: number) => { const g = dot(v), d = (g - W) * 16.5, u = d + D.toe, b = d * D.slope + 0.5 * (u - Math.sqrt(u * u + 0.5625)) * (D.under - D.slope), c = D.chroma - (D.chroma - D.slope) * D.highs * sm(-1.5, 0, d), k = D.knee;
    return v.map(vc => { const x = 2 ** (b + (vc - g) * 16.5 * c), y = x < k ? x : k + (1 - k) * (1 - Math.exp(-Math.max(x - k, 0) / (1 - k))); return Math.pow(Math.min(1, Math.max(0, y)), 1 / 2.2); }); };
  /** a colour of exposed linear light → what the screen is sent, 0‥255 a channel. `c` = the Contrast row's [contrast, stop]; `w` = the day's curve [how far in, its white's stop] */
  const tone = (rgb: number[], c: readonly number[], w: readonly number[]) => { let v = mul(IN, rgb.map(x => Math.max(x, 1e-10))).map(x => (Math.min(4.026069, Math.max(K.minEv, Math.log2(x))) - K.minEv) / K.stops);
    if (c[0] !== 1) v = v.map(x => Math.min(1, Math.max(0, c[1] + (x - c[1]) * c[0])));
    v = w[0] > 0 ? (() => { const a = v.map(sig), b = dayC(v, w[1]); return a.map((x, i) => mix(x, b[i], w[0])); })() : v.map(sig);
    const lum = dot(v), pw = w[0] > 0 ? mix(1.15, 1, w[0]) : 1.15, st = w[0] > 0 ? mix(SAT, 1, w[0]) : SAT;
    v = v.map(x => lum + st * (Math.pow(Math.max(x, 0), pw) - lum));
    return mul(OUT, v).map(x => 255 * Math.min(1, Math.max(0, x))); };
  const shown = (code: number) => Math.pow(code / 255, 2.2), under = (a: number, b: number) => Math.log2(shown(a) / shown(b));   // (the screen's own linear light, and stops between two codes on it)
  // a clear midday as game/hours.ts has it: the sun 6.0 at 56 degrees, the sky's zenith 0.3 0.45 0.8 (π × it on level ground), exposure 1.7
  const noon = { kind: 2, enabled: true, intensity: 6.0, dir: [0.145, -0.829, -0.54], color: [1, 0.97, 0.9] }, sky = [0.3, 0.45, 0.8].map(x => x * Math.PI), sunE = noon.color.map(x => x * 6.0 * 0.829), E = sky.map((x, i) => x + sunE[i]);
  const g = lookGrade(0.6, dot(sky), [noon], 1.7, 1.35), d0 = lookDay(g[1], 1), w: [number, number] = [d0[0], DBREAK ? K.grey : d0[1]], off = [0, 0], one = [1, g[1]];
  const lit = (albedo: number[], light: number[] = E) => albedo.map((a, i) => a * light[i] / Math.PI * 1.7), grey = (a: number) => [a, a, a];
  const A = [0.25, 0.47, 0.66, 0.85], dv = A.map(a => tone(lit(grey(a)), one, w)[1]), nv = A.map(a => tone(lit(grey(a)), g, off)[1]);
  check(w[0] === 1 && Math.abs(dv[0] - 148) <= 8 && Math.abs(dv[1] - 192) <= 8 && Math.abs(dv[2] - 218) <= 8 && Math.abs(dv[3] - 235) <= 6 && dv[3] - dv[0] >= 80 && nv[3] - nv[0] <= 70,
    `at a clear midday the curve is all the day's, and sunlit ground of 0.25 / 0.47 / 0.66 / 0.85 goes to the screen at ${dv.map(v => v.toFixed(0)).join(' / ')} of 255 (a camera's 148 / 192 / 218 / 235), ${(dv[3] - dv[0]).toFixed(0)} apart end to end where the night's curve with the Contrast row's turn sends ${nv.map(v => v.toFixed(0)).join(' / ')}, ${(nv[3] - nv[0]).toFixed(0)} apart`);
  // a shadow on the 0.47 stone that holds an eighth of its light, one under a roof that holds a thirty-second, a void that holds a thousandth: how far
  // under the stone each is on the SCREEN (its 2.2 taken off), by the day's curve and by the night's with the turn
  const stone = lit(grey(0.47)), dim = (n: number) => stone.map(x => x / 2 ** n);
  const dU = [3, 5, 10].map(n => under(tone(stone, one, w)[1], tone(dim(n), one, w)[1])), nU = [3, 5].map(n => under(tone(stone, g, off)[1], tone(dim(n), g, off)[1]));
  check(dU[0] > 2.4 && dU[0] < 3.3 && dU[1] < nU[1] - 0.8 && dU[2] >= 10,
    `on the screen a shadow three stops under the stone beside it is ${dU[0].toFixed(2)} stops under it and one five under is ${dU[1].toFixed(2)}, where the night's curve with the turn has them ${nU[0].toFixed(2)} and ${nU[1].toFixed(2)} under (it steepens as it goes down: the deeper the shade the further it is crushed); a void ten stops under is ${dU[2].toFixed(2)} under: the toe takes the dark end to black`);
  // a paint keeps its colour, and shade keeps the colour of the light that fills it: the stops between red and blue on the screen against those in the scene
  { const paint = lit([0.8, 0.45, 0.2]), shade = lit(grey(0.47), sky.map(x => x * 0.4)), rb = (c: number[]) => under(c[0], c[2]), scene = (c: number[]) => Math.log2(c[0] / c[2]);
    const pD = rb(tone(paint, one, w)), pN = rb(tone(paint, g, off)), sD = -rb(tone(shade, one, w)), sN = -rb(tone(shade, g, off));
    check(Math.abs(pD - scene(paint)) < 0.3 && Math.abs(sD + scene(shade)) < 0.3 && sN > sD + 0.3,
      `a sunlit paint whose red is ${scene(paint).toFixed(2)} stops over its blue is on the screen ${pD.toFixed(2)} over (the night's curve: ${pN.toFixed(2)}); stone in shade under a roof, filled by a sky ${(-scene(shade)).toFixed(2)} stops bluer than it is red, is ${sD.toFixed(2)} bluer on the screen, where the night's curve with the turn makes it ${sN.toFixed(2)}`); }
  // how far it comes in is asked of the light: none where sunlit ground sits in the night curve's middle, all of it in its flat top; the row scales it
  { const dusk = lookGrade(0.25, Math.PI * 0.17, [{ ...noon, intensity: 2.2, dir: [0.9, -0.375, 0.2], color: [1, 0.5, 0.26] }], 2.0, 1.35)[1];
    check(lookDay(dusk, 1)[0] === 0 && lookDay(g[1], 0)[0] === 0 && lookDay(g[1], 0.5)[0] === 0.5 && Math.abs(lookDay((D.from + D.full) / 2, 1)[0] - 0.5) < 1e-9 && lookDay(0.9, 7)[0] === 1 && dusk < D.from && g[1] > D.full,
      `sunlit ground sits at ${g[1].toFixed(3)} of the night curve's range at midday and ${dusk.toFixed(3)} at dusk: the day's curve is all in at the first and not in at the second (it comes in from ${D.from} to ${D.full}); the row at 0 is none of it at any hour, at a half is half`); }
  // part way in (a grey day, the row at a half): the two curves are MIXED, and a brighter thing is never sent to the screen darker, for a grey and for a paint
  { let bad = 0, n = 0; for (const k of [0.25, 0.5, 0.75, 1]) for (const col of [[1, 1, 1], [0.8, 0.45, 0.2], [0.3, 0.45, 0.8]]) { let last = -1;
      for (let s = -12; s <= 3; s += 0.125) { const c = tone(col.map(x => x * 2 ** s), [1 + 0.35 * (1 - k), g[1]], [k, w[1]]), y = dot(c); n++; if (y < last - 0.25) bad++; last = Math.max(last, y); } }   // (a quarter of a code: the night curve's own fit wanders by a few hundredths of a code under one code of black, at any weight and with none)
    check(bad === 0 || DBREAK, `at a quarter, a half, three quarters and all the way in, over fifteen stops of a grey, a warm paint and the sky's blue (${n} steps), the screen's brightness never falls by a quarter of a code as the light rises (${bad} falls)`); }
  // the one exposure model is still told sunlit ground's brightness when the turn has stood down to 1 (it rolls a brought-up room's highlights off about it)
  { const f1 = lookGradeFrame([1, g[1]], 1.7, true), f0 = lookGradeFrame([1, g[1]], 1.7), f2 = lookGradeFrame(g, 1.7);
    check(!!f1 && f1[0] === 1 && !!f2 && Math.abs(f1[1] - f2[1]) < 1e-12 && f0 === null, 'with the day\'s curve all in and the turn at 1 the frame still hands the exposure model the radiance of sunlit ground, with nothing to undo; with no curve and no turn it hands nothing, as before'); }
  { const post = readFileSync(resolve(import.meta.dir, '../../src/render/shaders/post.wgsl'), 'utf8'), eng = readFileSync(resolve(import.meta.dir, '../../src/engine.ts'), 'utf8'), gb = readFileSync(resolve(import.meta.dir, '../../src/render/gbuffer.ts'), 'utf8'), pa = readFileSync(resolve(import.meta.dir, '../../src/render/passes.ts'), 'utf8');
    const lines = ['fn dayCurve(v: vec3f) -> vec3f {\n  let g = dot(v, vec3f(0.2126, 0.7152, 0.0722));\n  let d = (g - post.gradeW) * 16.5;', '  let u = d + post.gradeToe;', '  let b = d * post.gradeSl + 0.5 * (u - sqrt(u * u + 0.5625)) * (post.gradeSu - post.gradeSl);', '  let c = post.gradeCs - (post.gradeCs - post.gradeSl) * post.gradeHi * smoothstep(-1.5, 0.0, d);', '  let x = exp2(b + (v - g) * 16.5 * c);\n  let k = post.gradeKn;\n  let y = select(k + (1.0 - k) * (1.0 - exp(-max(x - k, vec3f(0.0)) / (1.0 - k))), x, x < vec3f(k));', '  return pow(clamp(y, vec3f(0.0), vec3f(1.0)), vec3f(1.0 / 2.2));\n}',
      '  if (post.gradeD > 0.0) { v = mix(agxContrast(v), dayCurve(v), post.gradeD); } else { v = agxContrast(v); }', '    v = pow(v, vec3f(mix(1.15, 1.0, post.gradeD)));\n    v = lum + mix(post.saturation, post.gradeSr, post.gradeD) * (v - lum);\n  } else {\n    v = pow(v, vec3f(1.15));\n    v = lum + post.saturation * (v - lum);\n  }'];
    const miss = lines.filter(l => !post.includes(l)).length;
    check(miss === 0 && eng.includes('const d = lookConfig.gradeDay > 0 ? lookDay(g0[1], lookConfig.gradeDay) : null, g: [number, number] = d && d[0] > 0 ? [1 + (g0[0] - 1) * (1 - d[0]), g0[1]] : g0;') && eng.includes('P.gradeC = 1; P.gradeD = 0;')
      && /LOOK_DEFAULTS = \{[^}]*\bday: 0 \}/.test(gb) && pa.includes('d[14] = this.gradeD; d[15] = this.gradeW; d[16] = this.gradeSl; d[17] = this.gradeCs;') && pa.includes('d[19] = this.gradeSu; d[20] = this.gradeToe; d[21] = this.gradeHi; d[22] = this.gradeSat;') && pa.includes('saturation = 1.15;'),
      `the shader's day curve is the mirror's, line for line (${lines.length - miss} of ${lines.length} lines found), it is run only by as much as the frame says and not at all at 0, the night curve's punch and saturation stand down under it, the Contrast row's turn stands down by as much as the curve stands in, and the row opens at 0`); }
}
process.exit(red ? 1 : 0);
