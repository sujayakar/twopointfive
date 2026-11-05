// The look prototype's ONE EXPOSURE row (src/render/lookexpo.ts lookExpoFrame; composite.wgsl lookExpoGain, mirrored below; the place field it reads is
// render/placeField.ts, whose own check is tools/qa/place-field-page.cjs). The maintainer, 10-31, of the day rows mended a case at a time: "i worry that we're
// finding these cases one by one and not fixing them systematically", and of a store with one bulb in overcast: "compare to night and dusk". One gain a place,
// no cap on it, and nothing by the pixel but the Light in order row (11-04: a pixel holding more light than its place is shown as that light's place would be). No GPU: the lines are mirrored here as numbers and the picture is never SEEN here.
//   bun run tools/qa/lookexpo-check.ts   (exit 0 = all pass; LOOKEXPO_BREAK=1 caps a place's gain at six times, as the rows before had it, and must fail;
//                                         HOURS_BREAK=1 leaves the eyes' widest out of the every-hour model, as a night with no bound, and must fail;
//                                         PARTSHOW_BREAK=1 shows a lit particle with no gain, as the splat had it, and must fail)
import { readFileSync, existsSync } from 'node:fs';
import { resolve } from 'node:path';
import { LOOK_DEFAULTS } from '../../src/render/gbuffer';
import { lookConfig } from '../../src/render/frame';
import { lookExpoFrame, lookExpoBound, lookNightSkyE, LOOK_EXPO } from '../../src/render/lookexpo';
import { lookGrade, lookGradeFrame, LOOK_GRADE } from '../../src/render/lookgrade';
import { hourExposure, LUX_PER_UNIT, HOUR_SUN_LUX } from '../../src/render/photometry';
import { HOURS } from '../../src/game/hours';
import { PLACE, PLACE_W, PLACE_H, PLACE_BYTES, PLACE_WORDS, PLACE_NEAR } from '../../src/render/placeField';
import { placeMask, placeWall, placeRoof, doorStep, DOOR_STEPS, DOOR_PER_STOP } from '../../src/scene/placePlan';
import { makeBox, BoxFlag } from '../../src/scene/boxes';
let red = 0; const check = (ok: boolean, w: string) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${w}`); if (!ok) red++; };
const BREAK = process.env.LOOKEXPO_BREAK === '1', HOURS_BREAK = process.env.HOURS_BREAK === '1', ROOT = resolve(import.meta.dir, '../..');
const COMP = readFileSync(resolve(ROOT, 'src/render/shaders/composite.wgsl'), 'utf8'), COMMON = readFileSync(resolve(ROOT, 'src/render/shaders/place_common.wgsl'), 'utf8');
const KERNEL = COMMON + COMP, wnum = (name: string) => +(KERNEL.match(new RegExp(`\\nconst ${name} = ([0-9.]+);`))?.[1] ?? NaN);
const BAND = wnum('EXPO_BAND'), MOST = wnum('EXPO_MOST'), TOP = wnum('EXPO_TOP'), KEY = wnum('EXPO_KEY'), READ = wnum('EXPO_READ'), LAMP = wnum('EXPO_LAMP');   // (the kernel's own numbers: the mirror below runs on what the picture runs on)
type V4 = readonly [number, number, number, number];
const SHADE: V4 = [0.7, 0.1, 6, 1], C = lookConfig.gradeC, P = 1.559, GRADE: V4 = [1 / C, P, 1, 0.6], FLAT: V4 = [0, P, 1, 0.6];
const clamp = (x: number, a: number, b: number) => Math.min(b, Math.max(a, x));
/** place_common.wgsl lookGains on the CPU: a place's three gains (as it is shown now, as it is shown to eyes that are in it, as the open air is shown now).
 *  `place` = the levels the field holds (log2): at the pixel, under Sam (null = its eye has no answer yet) and in the open air on this map (the field's word
 *  for the cells under the sky; left out = what open ground would hold); null = this page has no field: `here` is then the cache at the pixel and Sam is
 *  taken to stand in the open. */
function gains(here: number, eye: V4, place: { u: number; ue: number | null; sky?: number; mid?: number } | null, shade = SHADE, grade = GRADE, widest: readonly [number, number] | null = null): [number, number, number] {
  let rs = 1, openL = shade[1], keyMost = KEY;
  const eyeDark = (ue: number) => Math.pow(clamp(openL / Math.max(ue, 1e-9) / BAND, 1, MOST), clamp(-eye[3] - 1, 0, 1));   // (lookEyeDark)
  if (place) { here = 2 ** place.u; if (place.sky !== undefined) { openL = 2 ** place.sky; if (place.mid !== undefined) keyMost = Math.max(2 ** (place.mid - place.sky) * BAND, 1); }
    if (place.ue !== null) rs = eyeDark(2 ** place.ue); }
  const expoFor = (h: number, rs: number) => {   // (lookExpoFor)
    const rp = clamp(openL / Math.max(h, 1e-9) / BAND, 1, MOST), a = eye[1], outG = Math.pow(rs, eye[2]), key = Math.pow(Math.min(Math.sqrt(rs), keyMost), a);
    let base = 1;
    if (rp >= rs) base = key * Math.pow(rs, a) * Math.pow(rp / rs, shade[0]);
    else { const q = rs / rp, over = outG > 1.001 ? 1 + (outG - 1) * (1 - Math.exp(-(q - 1) / (outG - 1))) : 1; base = key * Math.pow(rp * over, a); }
    if (BREAK) base = Math.min(base, 6);
    if (grade[0] > 0) base *= Math.pow(Math.max(rp / base, 1e-6), 1 - grade[0]);
    if (widest && widest[0] > 0 && !HOURS_BREAK) {   // (THE EYES' WIDEST: frame.placeCfg's z and w. The engine says them whenever the model is on; a line here that hands none asks the model's arithmetic without its bound, and one whose frame says a night's exposure over the hour's is held to that, as the shader holds it with the dim level under the read floor)
      const over = Math.max(eye[0], 1); let most = over;
      if (place) most *= clamp(Math.pow(widest[0] / (over * Math.max(h, READ)), shade[0]), 1, Math.max(widest[1], 1));
      base = Math.min(base, most); }
    else if (eye[0] > 1) base = Math.min(base, eye[0]);
    return base; };
  const g = expoFor(here, rs);
  return [g, Math.max(expoFor(here, eyeDark(here)), g), Math.min(expoFor(openL, rs), g)];
}
/** place_common.wgsl lookRoll: the shoulder's factor for light `l` that a gain `g` has multiplied */
function roll(l: number, g: number, shade = SHADE, grade = GRADE): number {
  let Pr = grade[1] > 0 ? grade[1] : shade[3]; Pr = Math.max(Pr, LAMP);   // (never about less than the night's white)
  if (!(Pr > 0 && g > 1 && l > Pr)) return 1;
  const room = (TOP - 1) * Pr, w = clamp((g - 1) * 2, 0, 1); return 1 - w + w * (Pr + room * (1 - Math.exp(-(l - Pr) / room))) / l;
}
/** place_common.wgsl lookShowBy as a factor: light `l0` (before any gain) of a place is shown × this. THE ONE RULE: exposed as for eyes in the place,
 *  rolled off there, taken down to the place's gain now, and never under what the open air shows the same light as. No light named = the place's gain. */
function gain(here: number, eye: V4, place: { u: number; ue: number | null; sky?: number; mid?: number } | null, shade = SHADE, grade = GRADE, l0 = 0, widest: readonly [number, number] | null = null): number {
  const [g, gf, go] = gains(here, eye, place, shade, grade, widest);
  if (!(l0 > 0)) return g;
  return Math.max(l0 * gf * roll(l0 * gf, gf, shade, grade) * (g / gf), l0 * go) / l0;
}
/** post.wgsl agx's turn, as light: C times the stops from sunlit ground */
const shown = (l: number, c = C) => P * Math.pow(l / P, c);
/** a pale floor in a place `dark` times under the open air, as the tone map is handed it: the sky-lit ground's radiance over `dark`, times the gain */
const white = (dark: number, g: number) => 0.2706 * SHADE[3] / dark * g;
const lvl = (dark: number) => Math.log2(SHADE[1] / dark);   // (the field's level in a place `dark` times under the open air)
const eyeAt = (amount = 1, glare = lookConfig.glare) => lookExpoFrame(SHADE, 0.7, amount, glare)!;
const OUT = eyeAt(), K = PLACE_W * PLACE_H;

check(lookExpoFrame(undefined, 0.7, 1) === null && lookExpoFrame([0, 0.1, 6, 1], 0.7, 1) === null && OUT[3] === -2 && eyeAt(0.25)[3] === -1.25 && OUT[0] === 1 && OUT[1] === 1 && BAND === LOOK_EXPO.band && MOST === LOOK_EXPO.most && KEY === LOOK_EXPO.key  && TOP === LOOK_EXPO.top,
  'no shade row or a night gives no model (no place takes a gain); by day the frame says the model with a number under 0 that carries the Eyes follow Sam row; and the page\'s four numbers are the kernel\'s');
check(gain(SHADE[1], OUT, { u: lvl(1), ue: lvl(1) }) === 1 && gain(SHADE[1] * 0.9, OUT, { u: lvl(1.1), ue: lvl(1) }) === 1 && gain(SHADE[1], OUT, null) === 1 && gain(SHADE[1], OUT, { u: lvl(1), ue: lvl(1) }, SHADE, FLAT) === 1,
  `with Sam out in the open the open air is sent on exactly as it is, out to where a place holds ${(1 / BAND).toFixed(2)} of its light, with the contrast row or without`);
{ // Sam in a windowless room 700 times under a grey day (a bulb at its real strength): his room is shown where the yard is with him out in it
  const u = lvl(700), g = gain(0, OUT, { u, ue: u }, SHADE, FLAT), was = Math.min(6, Math.pow(700, 0.7));
  check(Math.abs(white(700, g) / white(1, 1) - KEY / BAND) < 1e-9, `Sam in a room 700 times under the open air: its pale floor is shown ${(KEY / BAND).toFixed(2)} times as bright as the yard's with him out in it, the key over a place that just counts as the open air (× ${g.toFixed(0)}; the rows before stopped at × ${was.toFixed(0)} and left it ${(700 / was).toFixed(0)} times under)`);
  const half = gain(0, eyeAt(0.5), { u, ue: u }, SHADE, FLAT);
  check(half < g && half > Math.pow(700 / BAND, 0.7) - 1e-6 && Math.abs(half - KEY * Math.pow(700 / BAND, 0.5) * Math.pow(Math.pow(700 / BAND, 0.5), 0.7)) < 1e-6, `with the Eyes follow Sam row at a half the eye goes half the stops to his room and the rest is the shade row's share: × ${half.toFixed(0)}`); }
{ // one gain a place: with the field, two spots of one room (its level one number) take one gain whatever light each holds; a pixel's own light is not asked
  const u = lvl(8), a = COMMON.indexOf('fn lookExpoFor('), body = COMMON.slice(a, COMMON.indexOf('\n}\n', a));
  check(gain(0.033, OUT, { u, ue: u }) === gain(0.004, OUT, { u, ue: u }) && gain(0, OUT, { u, ue: u }, SHADE, GRADE, 0.01) === gain(0, OUT, { u, ue: u }) && body.length > 600 && !/Epix|\bl0\b|luminance|lookRoll/.test(body) && /^fn lookExpoFor\(here: f32, rs: f32, oa: vec3f\) -> f32 \{/.test(body) && COMP.includes('fn lookExpoGain(here: f32, own: f32, near: f32) -> f32 {'),
    'a room has one gain from its window to its far end, and the model\'s gain asks nothing of the pixel\'s own light (the shoulder is the one rule\'s, after it; the Light in order row\'s cap is lookExpoGain\'s, before it, where two places meet, and is nothing to eyes in the room): a shadow\'s edge and a man\'s shading keep the light\'s own proportions'); }
{ // the shoulder: light that is not the place's own, in a place shown hundreds of times brighter, to eyes that are in the place
  const u = lvl(500), sun = white(1, 1) * 6, g0 = gain(0, OUT, { u, ue: u }, SHADE, FLAT), g = gain(0, OUT, { u, ue: u }, SHADE, FLAT, sun), go = gains(0, OUT, { u, ue: u }, SHADE, FLAT)[2]; let mono = true, last = 0, under = true;
  for (let i = 0; i <= 400; i++) { const l0 = Math.pow(2, i / 20 - 14), w = l0 * gain(0, OUT, { u, ue: u }, SHADE, FLAT, l0); if (w < last * (1 - 1e-12)) mono = false; if (w > Math.max(TOP * P, l0 * go) + 1e-9) under = false; last = w; }
  check(BREAK || (sun * g0 > 100 * P && sun * g <= Math.max(TOP * P, sun * go) + 1e-9 && sun * g > P && sun * go < TOP * P && mono && under), `a sun patch in a room 500 times under the open air would be handed on at ${(sun * g0 / P).toFixed(0)} times sunlit ground's brightness; it is rolled off to ${(sun * g / P).toFixed(1)} times it (under ${TOP}, or what the open air shows of the same light where that is more), and a brighter light is still never shown darker than a dimmer one`);
  check(gain(SHADE[1], OUT, { u: lvl(1), ue: lvl(1) }, SHADE, FLAT, 1e6) === 1 && gain(0, OUT, { u, ue: u }, SHADE, FLAT, P / g0 * 0.99) === g0 && [COMP, readFileSync(resolve(ROOT, 'src/render/shaders/compact/composite.wgsl'), 'utf8')].every(t => (t.match(/ = lookShow\(/g) ?? []).length === 5 && !/lookRoll\(/.test(t) && !/\* shadeG\)?;? ?(spec|refl|color) \+=/.test(t) && /let ws = lookShow\(wetCoatSpec\(id, P, N, z, EdS, EdB, Ei\), shadeG\);/.test(t)) && /frame\.eyeCfg\.w < 0\.0 && P > 0\.0 && g > 1\.0 && l > P/.test(COMMON) && (COMMON.match(/lookRoll\(/g) ?? []).length === 2,
    'the open air is never rolled off, nor anything under sunlit ground\'s brightness; the diffuse, a mirrored yard, the gloss and a man\'s finish all go through the one rule and nothing else rolls a place\'s light off'); }
{ // A PLACE SEEN FROM ANOTHER IS THE PICTURE IT IS TO EYES IN IT, SHOWN DARKER (11-02: men white at their edges in a bulb-lit room seen from the yard at midday)
  const u = lvl(4000), IN = { u, ue: u }, YARD = { u, ue: lvl(1) }, w = white(4000, 1);   // a store with one bulb at midday: 4,000 times under the yard
  const [gIn] = gains(0, OUT, IN, SHADE, FLAT), [gOut, gFull, gOpen] = gains(0, OUT, YARD, SHADE, FLAT); let same = true, low = true, mono = true, last = 0;
  for (let i = 0; i <= 240; i++) { const k = Math.pow(2, i / 20), l = w * k;   // a light up to 4,096 times the room's pale floor: a bulb's glint on a wet coat, on a drum
    const from = l * gain(0, OUT, YARD, SHADE, FLAT, l), inside = l * gain(0, OUT, IN, SHADE, FLAT, l);
    if (from > l * gOpen * (1 + 1e-9) && Math.abs(from / (w * gOut) - inside / (w * gIn)) > 1e-6 * inside / (w * gIn)) same = false;   // (where the open air's own showing is not the larger: the same ratio to the floor as from inside)
    if (from > Math.max(TOP * P * gOut / gFull, l * gOpen) + 1e-9) low = false; if (from < last * (1 - 1e-12)) mono = false; last = from; }
  const was = 100 * w * gOut;   // (as it was: a glint 100 times the floor kept all of it, handed on at this)
  check(BREAK || (Math.abs(gFull - gIn) < 1e-9 && gOpen === 1 && gFull / gOut > 8 && same && low && mono && TOP * P * gOut / gFull < P * 0.6 && was > TOP * P * gOut / gFull * 2),
    `from the yard, a store ${(Math.log2(gFull / gOut)).toFixed(1)} stops under its own exposure: a glint in it is as many times its floor as it is from inside the room, and never over ${(TOP * gOut / gFull).toFixed(2)} of sunlit ground's brightness (a glint 100 times the floor was handed on at ${(was / P).toFixed(2)} of it, beside a floor at ${(w * gOut / P).toFixed(3)})`);
  const patch = P;   // a sun patch on that room's floor, through its door: as bright as the sunlit ground outside
  check(BREAK || (Math.abs(gain(0, OUT, YARD, SHADE, FLAT, patch) - 1) < 1e-9 && Math.abs(gain(0, OUT, YARD, SHADE, FLAT, w) / gOut - 1) < 1e-12 && /return c \* \(max\(full \* \(gains\.x \/ gains\.y\), l \* gains\.z\) \/ l\);/.test(COMMON)),
    'and light as bright as the open air\'s own is never shown darker than the open air shows it: a sun patch through that room\'s door is as bright as the ground it came in over, while the room\'s floor keeps the room\'s gain');
  const vol = readFileSync(resolve(ROOT, 'src/render/shaders/volumetrics.wgsl'), 'utf8'), vol2 = readFileSync(resolve(ROOT, 'src/render/shaders/compact/volumetrics.wgsl'), 'utf8'), passes = readFileSync(resolve(ROOT, 'src/render/passes.ts'), 'utf8');
  check([vol, vol2].every(t => (t.match(/if \(LOOK\) \{ let pk = airShow\(X, luminance\(Lin\)\); Lin \*= pk; Lin2 \*= pk; LinA \*= pk; \}/g) ?? []).length === 3 && t.includes('@group(1) @binding(5) var<uniform> placeBuf: PlaceBuf;') && /return lookShowBy\(vec3f\(l\), g\)\.x \/ l;/.test(t) && /if \(!\(frame\.shadeCfg\.x > 0\.0 && frame\.eyeCfg\.w < 0\.0\)\) \{ return 1\.0; \}/.test(t))
    && (passes.match(/volCommon \+ placeCommonSrc \+ /g) ?? []).length === 6 && !/volCommon \+ (?!placeCommonSrc|volPlainSrc)/.test(passes) && /fireSrc \+ placeCommonSrc \+ tuned\(compositeSrc\)/.test(passes) && /\{ binding: 5, visibility: CS, buffer: \{ type: 'uniform' \} \}/.test(passes) && (passes.match(/\{ binding: 5, resource: \{ buffer: this\.placeBuf \?\? this\.placeNone \} \}/g) ?? []).length === 2 && /this\.bgCompS = null; this\.bgVol = null; \}/.test(passes),
    'the air is shown by the same rule: each of the smoke march\'s three writings passes a step\'s light through it by the place the step is in, in both writings of the kernel, 1 off the look page and at night; every volumetric program that marches light (not the plain air\'s, which traces nothing and shows no place: vol_plain.wgsl) and the composite are built with the one text, and the march is handed the field'); }
{ // A LIT PARTICLE IS SHOWN BY ITS PLACE, as the floor it lies on is (11-04, the maintainer on his GPU: "GPU particles (like casings) look like they're not lit"):
  // the splat came after the composite with no gain, so brass in a room shown g times brighter was drawn at a g-th of the floor's share. The splat now
  // asks the three functions a point of the floor asks, in one place; the lines below hold that text. The arithmetic is this file's mirror of those
  // functions, so it says what the rule gives and what the splat gave before it, not that the shader was run. PARTSHOW_BREAK=1 shows a particle with
  // no gain, as it was, and must fail.
  const PB = process.env.PARTSHOW_BREAK === '1', rd = (f: string) => readFileSync(resolve(ROOT, f), 'utf8');
  const draw = rd('src/render/shaders/particles_draw.wgsl'), parts = rd('src/render/particles.ts'), eng = rd('src/engine.ts'), passes = rd('src/render/passes.ts'), field = rd('src/render/placeField.ts'), main = rd('src/main.ts'), vols = [rd('src/render/shaders/volumetrics.wgsl'), rd('src/render/shaders/compact/volumetrics.wgsl')];
  const u = lvl(4000), IN = { u, ue: u }, YARD = { u, ue: lvl(1) }, floor = white(4000, 1), brass = 0.6 * floor;   // a case that sends back 0.6 of what the pale floor under it does, deep in a store 4,000 times under the yard
  const surf = (l: number, place: typeof IN) => l * gain(0, OUT, place, SHADE, FLAT, l), show = (l: number, place: typeof IN) => PB ? l : surf(l, place);
  const rIn = show(brass, IN) / surf(floor, IN), rYard = show(brass, YARD) / surf(floor, YARD), gIn = gain(0, OUT, IN, SHADE, FLAT);
  check(Math.abs(rIn - 0.6) < 1e-9 && Math.abs(rYard - 0.6) < 1e-9 && gIn > 100,
    `brass deep in a store 4,000 times under the yard is 0.60 of the floor it lies on, to eyes in the store and from the yard, by the rule the floor is shown by (with no gain it was ${(0.6 / gIn).toFixed(4)} of it to eyes in the store: black)`);
  // … and where the Light in order row acts (a stride inside that store's open door, eyes in the yard): the floor hands the row the light that falls on
  // it, the lamps' whole and the bounce at its true strength; the splat has only what the particle sends on, in which the sim scaled the bounce, and puts
  // the bounce back as far as the place's own level says it (partShow). The row's level, lookGainsOwn's, mirrored: k = 1, the eyes in the open air.
  { const Sc = +(rd('src/settings.ts').match(/indirectScale: ([0-9.]+),/)?.[1] ?? NaN), open = SHADE[1], here = open / 4000, near = open / 8, eyes = open / BAND, PI = Math.PI;
    const rowLevel = (own: number) => { const o = Math.min(own, near, eyes); return near > here && o > here ? here * (o / here) : here; };
    const shownAt = (l: number, h: number) => l * gain(0, OUT, { u: Math.log2(h), ue: lvl(1) }, SHADE, FLAT, l);
    const partOwn = (l: number, alb: number, sc = Sc) => PB ? 0 : l * PI / alb + (sc / Math.max(sc, 0.05) - sc) * here;   // (partShow's two lines)
    const pair = (Ed: number, B: number, sc = Sc) => { const aF = 0.5, lF = aF * (Ed + sc * B) / PI, lC = 0.6 * (Ed + sc * B) / PI; return (shownAt(lC, rowLevel(partOwn(lC, 0.6, sc))) / shownAt(lF, rowLevel(Ed + sc * B / Math.max(sc, 0.05)))) / (0.6 / aF); };   // (the floor's: composite.wgsl lookShadeGain's hand, the lamps' light and the drawn bounce ÷ max(the scale, 0.05))
    const deep = pair(0, here), door = pair(open / 20, here), most = pair(0, 3 * here), scales = [0, 0.02, 0.05, 0.35, 1, 2].map(sc => Math.max(Math.abs(pair(open / 20, here, sc) - 1), Math.abs(pair(open / 200, here, sc) - 1)));
    check(Sc > 0 && Sc < 1 && Math.abs(deep - 1) < 1e-9 && Math.abs(door - 1) < 1e-9 && most >= 1 && most < Math.pow(3 / (1 + 2 * Sc), 1) + 1e-9 && most < 2 && Math.max(...scales) < 1e-9,
      `a case and the floor under it hand the Light in order row the same light, so they are shown in their own proportion: with the bounce at the place's own level (deep in the store, and a stride inside its open door in the door's daylight) to the bit; where the bounce at the case is three times its place's level and no lamp lights it, the case reads ${most.toFixed(2)} times over the floor's share, the most the splat's estimate is out by there (the scale is ${Sc}); and with the bounce at the place's level the two agree at every scale the bounce row can be set to, 0 to 2`); }
  const wrap = '  partAt = vec4f(0.0);\n  var o = vsKind(vi);\n  if (LOOK) { if (partAt.w > 0.0) { o.shade = partShow(partAt.xyz, o.shade, partAt.w); } }', lit = '  if (kind != P_SPARK) { partAt = vec4f(p.pos, select(1.0, clamp(luminance(unpackRgb8(p.color)), 0.05, 1.0), kind == P_DEBRIS || kind == P_DUST || kind == P_CASING || kind == P_SHEET)); }\n';
  const body = 'fn partShow(X: vec3f, c: vec3f, alb: f32) -> vec3f {\n  if (!(frame.shadeCfg.x > 0.0 && frame.eyeCfg.w < 0.0)) { return c; }\n  let l = luminance(c);\n  if (!(l > 0.0)) { return c; }\n  let q = (X.xz - RC_MIN.xz) / RC_SIZE.xz;\n  if (X.y >= CEIL_Y || any(q < vec2f(0.0)) || any(q >= vec2f(1.0))) { let oa = lookOpenAir(); return lookShowBy(c, lookGainsAt(oa.x, lookEyeRs(oa), oa)); }\n  let lit = l * 3.14159265 / alb;\n  let lv = lookPlaceLevels(X, vec3f(0.0, 1.0, 0.0), lit);\n  let own = lit + (frame.indirectScale / max(frame.indirectScale, 0.05) - frame.indirectScale) * lv.x;   // (the composite hands the row its drawn bounce ÷ max(the scale, 0.05): the same, for every scale the row can be set to)\n  return lookShowBy(c, lookGainsOwn(lv.x, own, lv.y, frame.placeCfg.x));\n}\n';
  const kinds = ['if (kind == P_SHEET) { return sheetVertex(', 'if (kind == P_RAIN) { return rainVertex(', 'kind == P_SHARD)) { return shardVertex('];
  check(!PB && draw.includes(body) && draw.includes(wrap) && (draw.match(/partShow\(/g) ?? []).length === 2 && (draw.match(/partAt = /g) ?? []).length === 2 && draw.includes(lit) && kinds.every(k => draw.indexOf(k) > draw.indexOf(lit)) && (draw.match(/@vertex/g) ?? []).length === 1 && draw.includes('@group(1) @binding(7) var<uniform> placeBuf: PlaceBuf;')
    && [rd('src/render/shaders/composite.wgsl'), rd('src/render/shaders/compact/composite.wgsl')].every(c => c.includes('let lv = lookPlaceLevels(P, N, Eown); return lookExpoGain(lv.x, Eown, lv.y);') && c.includes('let g = lookGainsOwn(here, own, near, frame.placeCfg.x);') && c.includes('return lookShowBy(c, lookNow.xyz);'))
    && vols.every(v => v.includes('airAt = at; airG = lookAirGains(X);') && !/airG = lookGains\(/.test(v)) && /fn lookAirGains\(X: vec3f\) -> vec3f \{/.test(COMMON)
    && parts.includes("pre + particlesCommon + refractCommon + placeCommonSrc + particlesDrawSrc, 'particlesDraw')") && parts.includes('preD = scenePrelude() + common + refractCommon + placeCommonSrc;') && parts.includes('const preD = scenePrelude() + commonSrc + refractSrc + placeSrc;') && main.includes("f('place_common.wgsl')]); return P.hotSwapAll(c, a, d, rf, pc); };")
    && parts.includes("{ binding: 7, visibility: VS, buffer: { type: 'uniform' } }") && parts.includes('{ binding: 7, resource: { buffer: this.placeBuf ?? this.placeNone } }') && /setPlace\(b: GPUBuffer \| null\) \{ if \(b !== this\.placeBuf\) \{ this\.placeBuf = b; this\.bgDraw = null; \} \}/.test(parts)
    && [parts, passes].every(s => s.includes('this.placeNone = placeNone(device);') && !/size: PLACE_BYTES/.test(s)) && (field.match(/size: PLACE_BYTES, usage: GPUBufferUsage\.UNIFORM \}/g) ?? []).length === 1
    && eng.includes('{ const pb = place ? place.buffer : null; this.passes.setPlace(pb); this.particles?.setPlace(pb); }'),
    'the splat asks what the composite asks of a point of the floor (its place off the floor plan, the Light in order row with the light on it, the one rule for what a place sends the eye), once, where the vertex stage hands its answer on, for every kind but a spark, whose light is its own; it is built with the place text in all three of its builds and handed the field each frame, the same zeros as the composite off the look page; and the smoke march asks the air\'s gains of the same text'); }
{ // the open air is what the cells under the open sky hold on this map: a walled yard is the open air
  const sky = lvl(1.7);
  check(gain(0, OUT, { u: sky, ue: sky, sky }) === 1 && gain(0, OUT, { u: lvl(1), ue: sky, sky }) === 1 && gain(0, OUT, { u: lvl(2), ue: sky, sky }) === 1 && Math.abs(gain(0, OUT, { u: lvl(700), ue: lvl(700), sky }, SHADE, FLAT) - KEY * 700 / 1.7 / BAND) < 1e-6 && /let sky = placeWord\(2\); if \(sky > -500\.0\) \{ openL = exp2\(sky\); let mid = placeWord\(3\);/.test(KERNEL),
    'a yard between walls that holds 1 / 1.7 of what open ground would is the open air (× 1), so is the brighter snow past its wall, and a room is counted from the yard');
  // the open air's MIDDLE: a room Sam stands in is shown as bright as the middle of the open air, not as its darkest tenth
  const mid = sky + Math.log2(1.6), room = { u: lvl(700), ue: lvl(700), sky, mid };
  check(Math.abs(gain(0, OUT, room, SHADE, FLAT) * 2 ** lvl(700) - 2 ** mid) < 1e-9 && gain(0, OUT, { u: sky, ue: sky, sky, mid }) === 1 && gain(0, OUT, { u: mid, ue: mid, sky, mid }) === 1 && Math.abs(gain(0, OUT, { u: lvl(700), ue: lvl(700), sky, mid: sky - 2 }, SHADE, FLAT) - 700 / 1.7 / BAND) < 1e-9,
    'with Sam in a dark room its floor is brought to the level the middle of the open air holds (the field\'s second word), the open air itself is still × 1 at both of its words, and a middle under the low word (a bad read) is no key at all');
  // never brighter than the night: the most any place is brought up is the night's exposure over the hour's
  const NIGHT = lookExpoFrame(SHADE, 0.7, 1, lookConfig.glare, 1 / 1152)!, pit = lvl(1e7);
  check(BREAK || (Math.abs(NIGHT[0] - 1152) < 1e-9 && Math.abs(gain(0, NIGHT, { u: pit, ue: pit, sky, mid }, SHADE, FLAT) - 1152) < 1e-9 && gain(0, NIGHT, { u: pit, ue: pit, sky, mid }) <= 1152 && Math.abs(gain(0, NIGHT, room, SHADE, FLAT) - gain(0, OUT, room, SHADE, FLAT)) < 1e-9 && lookExpoFrame(SHADE, 0.7, 1, lookConfig.glare, 1)![0] === 1 && KERNEL.includes('    let over = max(frame.eyeCfg.x, 1.0); var most = over;') && KERNEL.includes('    base = min(base, most);\n  }\n  return base;')),
    'a place is never brought up by more than the night\'s exposure over the hour\'s (× 1,152 on the grey day): a room with no light of its own is shown as the night shows it, a room under that bound is as it was, and a night says a bound of 1 (the eyes\' widest, below, is what brings a night\'s darkest places past it)');
  const u = lvl(1e6), far = gain(0, OUT, { u, ue: u }, SHADE, FLAT);
  check(BREAK || Math.abs(far - MOST * KEY) < 1e-6, `a place with no light at all in its probes is followed ${Math.log2(MOST)} stops down and no further (× ${MOST})`); }
{ // LIGHT KEEPS ITS ORDER WHERE TWO PLACES MEET (place_common.wgsl lookGainsOwn, the Light in order row): a pixel that holds more light than its place
  // is shown as a place holding its own light would be, as far as THE BRIGHTEST PLACE A STRIDE AWAY holds (the field's second block) and as far as the light
  // of the place the eyes are in. The maintainer's doorway, 11-04: a room thirty times under the open air, its door open on a yard's shade, a leaf in the
  // doorway's daylight holding four tenths of the open air's light. IN THE PICTURE'S OWN MEASURE: bounced light (the sky's on the shade outside, the room's
  // far end) is drawn × the bounce's scale and an opening's own light is drawn whole. LOOKEXPO_BREAK=1 puts the row down and the same lines are asked
  const kernel = (f: string) => readFileSync(resolve(ROOT, 'src/render/shaders', f), 'utf8'), cut = (t: string, name: string) => { const i = t.indexOf(`fn ${name}(`), j = t.indexOf('\n}\n', i); return i < 0 || j < 0 ? '' : t.slice(i, j); };
  const own = cut(COMMON, 'lookGainsOwn'), call = 'fn lookExpoGain(here: f32, own: f32, near: f32) -> f32 { let g = lookGainsOwn(here, own, near, frame.placeCfg.x); lookNow = vec4f(g, 1.0); return g.x; }';
  const hand = 'luminance(Ed) + luminance(Ei) / max(frame.indirectScale, 0.05)';
  const text = own.includes('var h = here; let oa = lookOpenAir(); let rs = lookEyeRs(oa);') && own.includes('if (k > 0.0 && near > h && oa.z > 0.5 && placeWord(2) > -500.0) {') && own.includes('let o = min(own, min(near, oa.x / (rs * EXPO_BAND)));') && own.includes('if (o > h) { h = max(h, 1e-9) * pow(o / max(h, 1e-9), min(k, 1.0)); }') && own.includes('return lookGainsAt(h, rs, oa);') && COMMON.includes('fn lookGains(here: f32) -> vec3f { let oa = lookOpenAir(); return lookGainsAt(here, lookEyeRs(oa), oa); }')
    && [kernel('composite.wgsl'), kernel('compact/composite.wgsl')].every(t => t.includes(call) && t.includes(hand) && (t.match(/return lookExpoGain\([^;]*?, Eown, [^;]*?\);/g) ?? []).length === 1 && t.includes('let lv = lookPlaceLevels(P, N, Eown); return lookExpoGain(lv.x, Eown, lv.y);'));
  const S = +(readFileSync(resolve(ROOT, 'src/settings.ts'), 'utf8').match(/indirectScale: ([0-9.]+),/)?.[1] ?? NaN), row = BREAK ? 0 : 1, sky = Math.log2(SHADE[1]), open = 2 ** sky;
  /** lookGainsOwn's level: a pixel holding light `own` (× the open air's) in a place at level `u`, the brightest place a stride away `nearBy` stops over it, the eyes in a place at level `ue` */
  const level = (u: number, own: number, nearBy: number, ue: number, eye: V4) => { const rs = Math.pow(clamp(open / 2 ** ue / BAND, 1, MOST), clamp(-eye[3] - 1, 0, 1)), h = 2 ** u, near = h * 2 ** nearBy, o = Math.min(own * open, near, open / (rs * BAND)); return Math.log2(row > 0 && near > h && o > h ? Math.max(h, 1e-9) * Math.pow(o / Math.max(h, 1e-9), row) : h); };
  /** what a pixel holding `own` and drawn with light `drawn` shows, in a place `dark` times under the open air with a place `nearBy` stops brighter a stride away, to eyes in a place `sam` times under the open air */
  const shows = (dark: number, own: number, drawn: number, nearBy: number, sam: number) => drawn * gains(0, OUT, { u: level(lvl(dark), own, nearBy, lvl(sam), OUT), ue: lvl(sam), sky }, SHADE, FLAT)[0];
  const by = (dark: number, sam: number) => gains(0, OUT, { u: lvl(dark), ue: lvl(sam), sky }, SHADE, FLAT)[0];   // (the place's gain alone)
  // the room is log2(30) stops under the yard; the leaf hangs a cell and a half in from the doorway (PLACE.near stops a cell under the yard's level there, and the doorway's own loss)
  const stops = Math.log2(30), atLeaf = stops - 1.5 * PLACE.near - 2 * PLACE.nearDoor;
  const far = shows(30, 1 / 30, S / 30, 0, 1), leaf = shows(30, 0.4, 0.4, atLeaf, 1), shade = shows(1, 1, S, 0, 1), was = 0.4 * by(30, 1);
  let mono = true, last = 0; for (let i = 0; i <= 400; i++) { const o = (1 / 30) * Math.pow(30 * 8, i / 400), v = shows(30, o, o, atLeaf, 1); if (v < last * (1 - 1e-9)) mono = false; last = v; }
  const inRoom = shows(30, 0.4, 0.4, atLeaf, 30), dim = shows(30, 1 / 60, S / 60, atLeaf, 1), deep = shows(30, 0.4, 0.4, 0, 1);
  const oneScale = shows(30, 0.4, 0.4 * S, atLeaf, 1);   // (the same leaf were an opening's light drawn on the bounce's scale, as the sky's on the shade is)
  check(text, 'the order rule is the place model\'s own (place_common.wgsl, by its words for the open air and the eyes), the two composite kernels hand it the pixel\'s light at its true strength (what is drawn whole, and the bounce over its scale) and the brightest place a stride away, and both of their callers pass them on');
  check(was > shade * 5 && leaf < shade * 1.9 && oneScale < shade && leaf < was / 5 && leaf > far && mono && Math.abs(inRoom - 0.4 * by(30, 30)) < 1e-12 && Math.abs(dim - (S / 60) * by(30, 1)) < 1e-12 && Math.abs(deep - was) < 1e-12,
    `from the yard, a leaf in the daylight of an opening into a room 30 times under the open air is shown under 1.9 times the shade outside and over the room's far end (far end ${far.toFixed(3)}, leaf ${leaf.toFixed(3)}, shade ${shade.toFixed(3)}; by the room's gain alone the leaf was ${was.toFixed(3)}, ${(was / shade).toFixed(1)} times the shade). What is left over the shade is not the exposure's: the leaf holds 0.4 of the shade's light and is drawn with more of it, an opening's light whole and the sky's × ${S}; on one scale the same leaf is ${oneScale.toFixed(3)}, under the shade. Also: what a pixel shows never falls as its light rises, a pixel darker than its room is as it was, DEEP IN THE ROOM (no brighter place a stride away) the same light is to the bit as it was, and with Sam in the room nothing moves`);
  // THE BRIGHTEST PLACE A STRIDE AWAY, the solver's rule written again (place_field.wgsl hand and placeSweep's last line, held by their text) on a made-up
  // plan: a yard at level 0 above a wall with one doorway, a room 9 stops under it below
  { const pf = kernel('place_field.wgsl'), W = 12, H = 30, wallJ = 4, doorI = 6; let yard = 0;
    const mask = (i: number, j: number) => (i < 0 || j < 0 || i >= W || j >= H ? 255 : j === wallJ ? (i === doorI ? 1 : 255) : 2);
    const near = new Float32Array(W * H).fill(-1000), d = BREAK ? 1e9 : PLACE.near, dd = BREAK ? 0 : PLACE.nearDoor;
    const sweeps = (n: number) => { for (let sweep = 0; sweep < n; sweep++) for (const par of [0, 1]) for (let j = 0; j < H; j++) for (let i = 0; i < W; i++) { if (((i + j) & 1) !== par) continue; const m = mask(i, j); if (m === 255) { near[j * W + i] = -1000; continue; }
      let v = j === wallJ ? (yard - 9) / 2 : j < wallJ ? yard : -9;   // (a doorway's own cell: the soft step between the two)
      for (const [di, dj] of [[1, 0], [-1, 0], [0, 1], [0, -1]]) { const mn = mask(i + di, j + dj); if (mn === 255) continue; const nv = near[(j + dj) * W + i + di]; if (nv < -500) continue; v = Math.max(v, nv - ((m & 3) === 1 || (mn & 3) === 1 ? dd : d)); }
      near[j * W + i] = v; } };
    const at = (i: number, j: number) => near[j * W + i];
    sweeps(80); const first = at(doorI, wallJ + 1), two = at(doorI, wallJ + 3), along = at(doorI + 2, wallJ + 1), far = at(0, wallJ + 1), deep = at(doorI, wallJ + 22), out = at(3, 0);
    yard = -8; sweeps(160); const dimFirst = at(doorI, wallJ + 1), dimOut = at(3, 0);   // the yard goes dim (dusk, a lamp put out): the block follows it down
    const textN = pf.includes('const PLACE_NEAR: i32 = PLACE_W * PLACE_H + 4;') && pf.includes('let i = c.y * PLACE_W + c.x; let door = (m & 3u) == 1u || (mn & 3u) == 1u;') && pf.includes('if (nv > -500.0) { o.z = nv - select(U.near, U.nearDoor + max(0.0, sn - sm), door); }') && pf.includes('field[PLACE_NEAR + i] = max(lv, max(max(h0.z, h1.z), max(h2.z, h3.z)));') && pf.includes('if (m == 255u) { field[i] = PLACE_SOLID; field[PLACE_NEAR + i] = PLACE_SOLID; return; }')
      && COMMON.includes('let i = PLACE_W * PLACE_H + 4 + q.y * PLACE_W + q.x; return placeBuf.v[i >> 2u][i & 3]; }') && PLACE_NEAR === PLACE_W * PLACE_H + PLACE_WORDS && PLACE.near === 0.5 && PLACE.nearDoor > 0 && PLACE.nearDoor < PLACE.near;
    check(textN && first === -2 * PLACE.nearDoor && two === first - 2 * PLACE.near && along === first - 2 * PLACE.near && far === Math.max(-9, first - 6 * PLACE.near) && deep === -9 && out === 0 && dimOut === -8 && dimFirst === -8 - 2 * PLACE.nearDoor,
      `the brightest place a stride away: the room's first cell inside its doorway holds the yard's level all but whole (${first}), each cell of floor further in ${PLACE.near} of a stop less (two cells in ${two}, two along the wall ${along}), never through the wall (six cells along it ${far}), and twenty-two cells in the room is its own again (${deep}); the yard is the yard's (${out}); and when the yard goes eight stops dim the block follows it down (${dimOut} outside, ${dimFirst} inside the door)`);
    // A DOOR OPENS BY DEGREES (scene/placePlan.ts placeMask `steps`; place_field.wgsl doorShut and hand, held by their text and by the two constants):
    // the same yard and room, the doorway's cells saying how open the door is, through a wall one cell deep and through one two cells deep
    const shutOf = (m: number) => { const st = (m >> 3) & 31; return m !== 255 && (m & 3) === 1 && st ? (DOOR_STEPS - st) / DOOR_PER_STOP : 0; };
    const inside = (deep: number, st: number): number => { const mk = (i: number, j: number) => (i < 0 || j < 0 || i >= W || j >= H ? 255 : j >= wallJ && j < wallJ + deep ? (i === doorI ? 1 | (st << 3) : 255) : 2), n = new Float32Array(W * H).fill(-1000);
      for (let sweep = 0; sweep < 80; sweep++) for (const par of [0, 1]) for (let j = 0; j < H; j++) for (let i = 0; i < W; i++) { if (((i + j) & 1) !== par) continue; const m = mk(i, j); if (m === 255) continue;
        let v = (m & 3) === 1 ? -1000 : j < wallJ ? 0 : -9;   // (the yard at 0, the room nine stops under; a doorway's cell has no level of its own and holds what it is handed)
        for (const [di, dj] of [[1, 0], [-1, 0], [0, 1], [0, -1]]) { const mn = mk(i + di, j + dj); if (mn === 255) continue; const nv = n[(j + dj) * W + i + di]; if (nv < -500) continue;
          v = Math.max(v, nv - ((m & 3) === 1 || (mn & 3) === 1 ? PLACE.nearDoor + (BREAK ? 0 : Math.max(0, shutOf(mn) - shutOf(m))) : PLACE.near)); }
        n[j * W + i] = v; }
      return n[(wallJ + deep) * W + doorI]; };
    const hops = (deep: number) => -(deep + 1) * PLACE.nearDoor, stops = (st: number) => (DOOR_STEPS - st) / DOOR_PER_STOP;
    const ok = [1, 2].every(deep => inside(deep, 0) === hops(deep) && inside(deep, DOOR_STEPS) === hops(deep) && [25, 19, 13, 7, 1].every(st => Math.abs(inside(deep, st) - (hops(deep) - stops(st))) < 1e-5));
    const numOf = (re: RegExp) => +(pf.match(re)?.[1] ?? NaN);
    const textD = numOf(/\nconst DOOR_STEPS: f32 = ([\d.]+);/) === DOOR_STEPS && numOf(/\nconst DOOR_PER_STOP: f32 = ([\d.]+);/) === DOOR_PER_STOP && DOOR_STEPS === 31
      && pf.includes('fn doorShut(m: u32) -> f32 { let s = (m >> 3u) & 31u; return select(0.0, (DOOR_STEPS - f32(s)) / DOOR_PER_STOP, m != 255u && (m & 3u) == 1u && s != 0u); }')
      && pf.includes('let sm = doorShut(m); let sn = doorShut(mn);') && pf.includes('if (v > -500.0) { o = vec3f(v * U.kappa, U.kappa, PLACE_SOLID); }') && pf.includes('if ((m & 3u) == 1u) { field[i] = PLACE_SOLID; field[PLACE_NEAR + i] = max(max(h0.z, h1.z), max(h2.z, h3.z)); return; }')
      && [1, 0.5, 0.25, 1 / 32, 0.02, 0].map(doorStep).join() === '31,25,19,1,0,0';
    check(ok && textD, `a door open by degrees hands the yard on dimmer by the share that is open, ONCE, however deep the wall: with the door wide the room's first cell holds what a doorway that says nothing gives it (${inside(1, DOOR_STEPS)} through a wall one cell deep, ${inside(2, DOOR_STEPS)} through two), half open a stop under that (${inside(1, 25).toFixed(3)}, ${inside(2, 25).toFixed(3)}), a quarter open two (${inside(1, 19).toFixed(3)}, ${inside(2, 19).toFixed(3)}), a thirty-second five (${inside(1, 1).toFixed(3)}); the map's steps are ${DOOR_PER_STOP} a stop in the plan and in the shader, and a share under about a thirty-fourth is shut`); } }
{ // order is kept and nothing steps: over every pair of Sam's place and a place, a darker place is never shown brighter than a lighter one
  let order = true, cont = true, worst = 0;
  for (const amount of [0, 0.5, 1]) for (const glare of [0, 0.1, 1]) for (const day of [1, 0.4]) for (const grade of [GRADE, FLAT]) for (const field of [true, false]) for (const sky of [undefined, lvl(1.7)]) for (const sam of [1, 1.3, 2, 3, 16, 100, 700, 4000, 40000]) {
    const e0 = eyeAt(amount, glare), eye: V4 = [e0[0], day, e0[2] * day, e0[3]], shade: V4 = [0.7 * day, 0.1, 6, 1]; let last = Infinity, lastG = NaN;
    for (let i = 0; i <= 3200; i++) { const dark = Math.pow(2, i / 200), g = gain(SHADE[1] / dark, eye, field ? { u: lvl(dark), ue: lvl(sam), sky } : null, shade, grade, white(dark, 1)), w = shown(white(dark, g), grade[0] > 0 ? 1 / grade[0] : 1);
      if (w > last * (1 + 1e-9)) { order = false; worst = Math.max(worst, w / last); if (process.env.LOOKEXPO_WHERE) console.log(JSON.stringify({ amount, glare, day, grade: grade[0], field, sam, dark, by: w / last })); } if (i > 0 && Math.abs(g / lastG - 1) > 0.02) cont = false; last = w; lastG = g; } }
  check(order, `a darker place is never shown brighter than a lighter one: from the open air to a place 65,536 times under it, with Sam in the open or in a place up to 40,000 times under it, in open country and in a walled yard, at every height of the eyes, glare and shade rows, by day and at dusk, with the place field and without${order ? '' : ` (worst × ${worst.toFixed(3)})`}`);
  check(cont, 'and the gain has no step anywhere along the way (under 2 % between places a two-hundredth of a stop apart)'); }
{ // a darker place than his, and a brighter one
  const us = lvl(16), sam = gain(0, OUT, { u: us, ue: us }, SHADE, FLAT), deep = gain(0, OUT, { u: lvl(16 * 64), ue: us }, SHADE, FLAT), yard = gain(0, OUT, { u: lvl(1), ue: us }, SHADE, FLAT), fromYard = gain(0, OUT, { u: lvl(512), ue: lvl(1) }, SHADE, FLAT);
  check(Math.abs(sam - KEY * 16 / BAND) < 1e-9 && Math.abs(deep / sam - Math.pow(64, 0.7)) < 1e-6, `a place darker than Sam's gets his gain and the shade row's share of its own extra dark: a store 6 stops under his room is shown ${(6 * (1 - 0.7)).toFixed(1)} stops under it`);
  check(Math.abs(fromYard - Math.pow(512 / BAND, 0.7)) < 1e-6 && Math.log2(512 / fromYard) < 3.5, `from the yard a room 9 stops under it is shown ${Math.log2(512 / fromYard).toFixed(1)} stops under it`);
  check(yard > KEY * 1.1 && yard < KEY * 1.3 && Math.abs(yard - KEY * Math.pow(16 / BAND, LOOK_EXPO.out * lookConfig.glare)) < 1e-9 && gain(0, eyeAt(1, 0), { u: lvl(1), ue: us }, SHADE, FLAT) === KEY, `from a room 16 times under it the yard is overexposed: by the key the whole picture is exposed up by (× ${KEY}) and a little more by the Glare outside row (× ${yard.toFixed(2)} in all)`);
  let keyOk = gain(0, OUT, { u: lvl(1), ue: lvl(1) }, SHADE, FLAT) === 1, lastK = 1; for (let i = 0; i <= 600; i++) { const r = Math.pow(2, i / 100), u = lvl(r * BAND), k = gain(0, OUT, { u, ue: u }, SHADE, FLAT) / r; if (k < lastK - 1e-9 || k > KEY + 1e-9 || k / lastK > 1.01) keyOk = false; lastK = k; }
  check(keyOk && Math.abs(lastK - KEY) < 1e-9 && /let key = pow\(min\(sqrt\(rs\), oa\.y\), a\);/.test(KERNEL), `the key comes in with how dark his place is, with no step: none in the open, all of it (× ${KEY}) from a place ${(KEY * KEY).toFixed(1)} times under`); }
{ // the day's contrast is the yard's, in the place's own exposure: a place is toned as the open air is and then shown base ∕ rp as bright
  const us = lvl(12), sam = gain(0, OUT, { u: us, ue: us }), yardWhite = shown(white(1, 1));
  check(Math.abs(shown(white(12, sam)) / yardWhite - KEY * Math.pow(BAND, -C)) < 1e-9 && Math.abs(shown(white(BAND, 1)) / yardWhite - Math.pow(BAND, -C)) < 1e-9, 'with the Contrast row up, Sam\'s room is toned as the open air is and then shown the key brighter: a pale floor in it sits the key over where one does in the darkest shade that still counts as the open air');
  const u = lvl(512), far = gain(0, OUT, { u, ue: lvl(1) }), flat = gain(0, OUT, { u, ue: lvl(1) }, SHADE, FLAT);
  check(Math.abs(shown(white(512, far)) / yardWhite - flat / (512 / BAND) * Math.pow(BAND, -C)) < 1e-9, `and a room seen from the yard is toned as the yard is and then shown as much darker as the shade row says (${Math.log2(512 / flat).toFixed(1)} stops for one 9 stops under)`);
  check(Math.abs(shown(white(12, sam) * 3) / shown(white(12, sam)) - Math.pow(3, C)) < 1e-6 && Math.abs(shown(white(512, far) * 3) / shown(white(512, far)) - Math.pow(3, C)) < 1e-6 && C > 1, `a face sending 3 times another's light back is ${Math.pow(3, C).toFixed(2)} times brighter in a room as in the yard (the Room contrast row has nothing left to do)`); }
{ // the two writings of the kernel, the field's numbers, and the rows
  const c = readFileSync(resolve(ROOT, 'src/render/shaders/composite.wgsl'), 'utf8'), c2 = readFileSync(resolve(ROOT, 'src/render/shaders/compact/composite.wgsl'), 'utf8'), pf = readFileSync(resolve(ROOT, 'src/render/shaders/place_field.wgsl'), 'utf8');
  const cut = (t: string, from: string, to: string) => { const a = t.indexOf(from), b = t.indexOf(to, a); return a < 0 || b < 0 ? '' : t.slice(a, b); };
  const part = (t: string) => cut(t, '\n// (EXPO_BAND, EXPO_KEY, EXPO_MOST, EXPO_TOP, ', '\nfn linearToSrgb(');
  const num = (t: string, re: RegExp) => +(t.match(re)?.[1] ?? NaN);
  check(part(c).length > 1500 && part(c) === part(c2) && COMMON.includes('  if (placeWord(1) < 0.5) { return vec2f(lookOpenAir().x); }') && [c, c2].every(t => t.includes('fn lookShadeGain(P: vec3f, N: vec3f, Eown: f32) -> f32 {\n  if (!(frame.eyeCfg.w < 0.0)) { return 1.0; }\n  let lv = lookPlaceLevels(P, N, Eown); return lookExpoGain(lv.x, Eown, lv.y);\n}') && !/fn lookExpoFor|fn lookRoll|fn placeCell|struct PlaceBuf|lookPlaceHere|lookPlaceField|fn lookPlaceLevels/.test(t) && t.includes('@group(1) @binding(17) var<uniform> placeBuf: PlaceBuf;')
      && !/gradeCfg\.w|ROOM_LEVEL|sg\.[xyzw]|Epix|frame\.shadeCfg\.z|frame\.eyeCfg\.[xyz]/.test(t) && (t.match(/lookShadeGain\(P, N, /g) ?? []).length === 1) && !existsSync(resolve(ROOT, 'src/render/lookeye.ts')),
    'both writings of the kernel carry the same lines, word for word; the pixel\'s place is the floor plan\'s and the field\'s (tools/qa/wallsides-check.ts holds that rule), it has one gain and it is the model\'s, with nothing left of the rows it replaced (no gain by the pixel\'s own light, no eye by a meter, no contrast given back by the caller, no read of the cache for the place); and neither holds a copy of the model (it is place_common.wgsl\'s, one text)');
  check([COMMON, pf].every(t => num(t, /\nconst PLACE_W: i32 = (\d+);/) === PLACE_W && num(t, /\nconst PLACE_H: i32 = (\d+);/) === PLACE_H) && num(COMMON, /struct PlaceBuf \{ v: array<vec4f, (\d+)> \};/) * 16 === PLACE_BYTES && PLACE_BYTES >= (2 * K + PLACE_WORDS) * 4 && PLACE_WORDS === 4 && /field\[k \+ 3\] = /.test(pf) && !/field\[k \+ 4\]/.test(pf),
    `the place field is ${PLACE_W} × ${PLACE_H} cells in the solver, the picture and the page's code, and the picture's buffer is the size the page makes (${PLACE_BYTES} bytes)`);
  const eng = readFileSync(resolve(ROOT, 'src/engine.ts'), 'utf8'), panel = readFileSync(resolve(ROOT, 'src/ui/lookpanel.ts'), 'utf8');
  check(/lookConfig\.on \? lookExpoFrame\(shadeCfg, /.test(eng) && !/\['expo', /.test(panel) && !('expo' in LOOK_DEFAULTS) && !('expo' in lookConfig) && /: null\) \?\? undefined\),/.test(eng) && !/lookEyeFrame|roomContrast|lookConfig\.expo\b/.test(eng) && !('roomc' in LOOK_DEFAULTS) && /const placeOn = !!shadeCfg && shadeCfg\[0\] > 0;/.test(eng), 'the model is the only way a place is brought up: the engine runs it wherever the shade row is up, it has no row of its own any more, and the rows it replaced are gone, the Room contrast row with them'); }
{ // ONE EXPOSURE AT EVERY HOUR (the look panel's `hours` and `dark` rows; render/lookexpo.ts lookExpoBound; place_common.wgsl lookExpoFor's last lines)
  const eng = readFileSync(resolve(ROOT, 'src/engine.ts'), 'utf8'), panel = readFileSync(resolve(ROOT, 'src/ui/lookpanel.ts'), 'utf8'), page = readFileSync(resolve(ROOT, 'src/main.ts'), 'utf8');
  // a night as the field holds it (its own measure, the night's units; read off the block's night): the yard 0.07 under a sky that puts 0.0818 on open ground,
  // a lit room 0.2, the far end of the weakest lit hall 0.031, a dim room 0.02, the unlit room the team starts in 0.0075
  const Y = 0.07, L = 0.2, H = 0.031, D = 0.02, U = 0.0075, SKY_E = 0.0818, N_SHADE: V4 = [0.7, 0.0149, 6, SKY_E], X = N_SHADE[0], NIGHT = lookExpoFrame(N_SHADE, 0.7, 1, lookConfig.glare, 1)!;
  const W1 = lookExpoBound(1, SKY_E), W0 = lookExpoBound(0, SKY_E), DIM = W1[0], NOGRADE: V4 = [0, 0, 1, 0.6], EYES = [L, Y, H, D, U, 1e-4];
  const at = (h: number, eye: number, w: readonly [number, number] | null = W1, f = NIGHT, open = Y, l0 = 0) => gain(0, f, { u: Math.log2(h), ue: Math.log2(eye), sky: Math.log2(open), mid: Math.log2(open) + 0.3 }, N_SHADE, NOGRADE, l0, w);
  const pc = readFileSync(resolve(ROOT, 'src/render/shaders/place_common.wgsl'), 'utf8');
  check(!('hours' in LOOK_DEFAULTS) && !('hours' in lookConfig) && LOOK_DEFAULTS.dark === 1 && !/'hours'/.test(panel) && /\['dark', 'Eyes open in the dark', /.test(panel) && /'order', 'dark'/.test(panel.match(/const LIVE[^\n]*/)![0]) && !/'portal'/.test(panel) && !('portal' in LOOK_DEFAULTS)
    && lookExpoFrame(N_SHADE, 0.7, 1, lookConfig.glare, 1) !== null && lookExpoFrame([0, 0.0149, 6, SKY_E], 0.7, 1, lookConfig.glare, 1) === null && lookExpoFrame([0.7, 0, 6, 0], 0.7, 1, lookConfig.glare, 1) === null && lookExpoFrame.length <= 5
    && !/everyHour|smoothstep\(0\.08, 0\.35, up\)\)/.test(eng.slice(eng.indexOf('function lookShade('), eng.indexOf('function lookShade(') + 400)) && /return \[Math\.min\(1, amount\), 0\.5 \* up, 0, /.test(eng) && !/everyHour/.test(eng) && /if \(placeOn\) \{ const own = hourExp < 1 \? lookConfig\.nightSky\?\.\(\) \?\? null : null;/.test(eng)
    && pc.includes('  P = max(P, EXPO_LAMP);') && !/if \(frame\.placeCfg\.z > 0\.0\)|else if \(frame\.eyeCfg\.x > 1\.0\)/.test(pc) && pc.includes('    let over = max(frame.eyeCfg.x, 1.0); var most = over;')
    && /eyesWidest = lookExpoBound\(lookConfig\.dark, lookNightSkyE\(hourExp, shadeCfg\?\.\[3\], own && lookExposure\(null, own\.zenith, own\.horizon\) >= 1 \? /.test(eng) && /, this\.settings\.nvAmount\); \}/.test(eng)
    && /lookExpoFrame\(shadeCfg, lookConfig\.shade, lookConfig\.eyes, lookConfig\.glare, hourExp\)/.test(eng) && /if \(placeOn\) this\.buildPlace\(\);/.test(eng) && /private buildPlace\(\): void \{ if \(this\.place \|\| this\.placeDead\) return; try \{ this\.place = new PlaceField\(/.test(eng) && /this\.placeDead = true; \} \}/.test(eng) && /lookConfig\.nightSky = \(\) => level\.sky;/.test(page),
    'the model asks no hour and has no row for it: the shade row has no fade into a night, the frame\'s day share is 1, the engine hands the frame the eyes\' widest by the night\'s sky and the goggles whenever the model is on, the shader\'s bound and its white are said with no test of a row, and a device that cannot build the field is asked once and no more (no place of a night is brought up there: the bound waits for a live field)');
  check(NIGHT[0] === 1 && NIGHT[1] === 1 && NIGHT[3] === -2 && DIM === LOOK_EXPO.dim && W1[1] === 2 ** LOOK_EXPO.dark && W0[1] === 1 && DIM < Y / 2 && DIM <= H && LOOK_EXPO.dimSky * SKY_E >= LOOK_EXPO.dim
    && lookExpoBound(1, null)[0] === LOOK_EXPO.dim && Math.abs(lookExpoBound(1, 0.0135)[0] - LOOK_EXPO.dimSky * 0.0135) < 1e-12 && lookExpoBound(1, 0)[0] > 0 && lookExpoBound(1, 0)[0] < 1e-6 && READ === LOOK_EXPO.read
    && lookNightSkyE(1, 0.05, 0.9) === 0.05 && lookNightSkyE(1, undefined, 0.9) === null && lookNightSkyE(0.5, 2.1, 0.08) === 0.08 && lookNightSkyE(0.5, 2.1, null) === null,
    `on a night the frame says the whole model (the day's share is 1) and a bound of 1; the dim level (${DIM} in the field's measure and the night's units) is under half the block's yard and under its weakest lit hall, and never over ${LOOK_EXPO.dimSky} of what the night's sky puts on open ground: the sky the frame draws on a night, the level's own while an hour is up, and none where the level is its own day`);
  check(EYES.every(eye => at(Y, eye) === 1 && at(L, eye) === 1 && at(H, eye) === 1) && Math.abs(at(DIM, U) - 1) < 1e-9 && EYES.every(eye => [Y, L, D, U, 1e-5].every(h => at(h, eye, W0) === 1 && at(h, eye, lookExpoBound(1, SKY_E, 0.5)) === 1 && at(h, eye, lookExpoBound(2, SKY_E, 1)) === 1))
    && lookExpoBound(1, SKY_E, 0.25)[1] === 2 && lookExpoBound(1, SKY_E, 0.5)[1] === 1,
    'a night\'s yard and its lit rooms, down to the far end of its weakest lit hall, are sent on exactly as the night drew them wherever the eyes are; with the Eyes open in the dark row at 0 so is every place of a night, and so it is from the goggles\' halfway down, where the picture becomes the tube\'s');
  { const dark = at(U, U), corner = at(U / 2, U), most = lookExpoBound(2, SKY_E);
    check(Math.abs(dark - Math.pow(DIM / U, X)) < 1e-9 && EYES.every(eye => Math.abs(at(U, eye) - dark) < 1e-9 && Math.abs(at(D, eye) - Math.pow(DIM / D, X)) < 1e-9) && corner > dark && corner * U / 2 < dark * U && Math.abs(at(1e-5, U) - 4) < 1e-9 && Math.abs(at(1e-5, U, most) - 16) < 1e-9 && dark > 2.5 && dark * U < DIM,
      `the unlit room the block's team starts in (${(Y / U).toFixed(0)} times under the yard) comes up by the shade row's share of its dark under the dim level (× ${dark.toFixed(2)}), the same from inside it, from the yard and from a lit room; a darker corner of it by more (× ${corner.toFixed(2)}) and still shown darker; a pit by two stops and no more with the row at 1, four at 2`);
    let ok = true, strict = true; for (const eye of EYES) { let last = Infinity; for (let h = 2; h > 2e-6; h *= 0.97) { const g = at(h, eye), shown = h * g;
        if (g < 1 || g > 4 + 1e-9 || (g > 1 && shown > DIM + 1e-9) || shown > last + 1e-12) ok = false;
        if (h < DIM && !(shown < last * 0.9995)) strict = false;   // (every step darker is shown darker: by the share's slope until the two stops are spent, by the place's own after)
        last = shown; } }
    check(ok && strict, 'from a lit room down to the read floor, with the eyes in a lit room, in the yard, in the weak hall, in a dim room, in the unlit one and in a pit: no place is shown under the night\'s exposure or over four times it, none is brought past the dim level, and two dark places a step apart are never drawn at one level, wherever the eyes are'); }
  { // one room at dusk and at night: the hour's exposure times the place's gain against the night's gain; and its light through the shoulder
    const hx = hourExposure('dusk', HOURS.dusk.sun.intensity), D_SHADE: V4 = [0.7, 0.115, 6, 0.6], DUSK = lookExpoFrame(D_SHADE, 0.7, 1, lookConfig.glare, hx)!, open = Math.log2(0.6), mid = 0.6 * 2 ** 0.3, DGRADE: V4 = [1 / 1.2, 0.3, 1, 0.6];
    const room = (h: number, f = DUSK, shade = D_SHADE, w: readonly [number, number] | null = W1, x = hx, l0 = 0, grade = NOGRADE) => gain(0, f, { u: Math.log2(h * x), ue: Math.log2(h * x), sky: open, mid: open + 0.3 }, shade, grade, l0 * x, w) * x;   // (a room holding `h` of the night's units, the eyes in it: what it, or light `l0` of the night's units in it, is shown by over the night's exposure)
    // the room lit by its own lamps, bound-held at dusk (its gain is the night's): a face the night shows at 0.6, the floor under the bulb at the night's white, a wall by the bulb at 3
    const through = [0.3, 0.6, LAMP, 3].map(l => [room(0.6, DUSK, D_SHADE, W1, hx, l, DGRADE), at(0.6, 0.6, W1, NIGHT, Y, l)]);
    check(hx < 0.02 && Math.abs(DUSK[0] - 1 / hx) < 1e-9 && [L, 0.6, 1.5, 4].every(h => Math.abs(room(h) - Math.min(1, mid / h)) < 1e-9) && Math.abs(room(L) - at(L, L)) < 1e-9 && Math.abs(room(U) - at(U, U)) < 1e-9 && Math.abs(room(D) - at(D, D)) < 1e-9
      && Math.abs(room(4) - mid / 4) < 1e-9 && through.every(([d, n], i) => n === 1 && (i < 3 ? Math.abs(d - 1) < 1e-9 : d < 1 && d > 0.85)) && DGRADE[1] < LAMP / 2 && LAMP === LOOK_EXPO.lamp && Math.abs(LAMP - LOOK_GRADE.albedo * (15 / LUX_PER_UNIT) / Math.PI) < 0.005,
      `at dusk a room lit by its own lamps is shown as the night shows it, or where the open air's middle is if that is darker, and never brighter than the night's picture; its light up to the night's white (${LAMP}: a pale floor under a 15 lux bulb) is shown at dusk to the number as at night, though dusk's own sunlit ground is a quarter of that, and a wall by the bulb at 3 a little under (${through[3][0].toFixed(2)} of the night's); and an unlit room is brought up as far at dusk as at night (× ${room(U).toFixed(2)} the night's exposure at both)`);
    // midday and the grey day, by the hours' own numbers: the dim level is at the field's read floor at midday, and the bound is the night's exposure as it ever was
    const mx = hourExposure('midday', HOURS.midday.sun.intensity), gx = hourExposure('overcast', HOURS.overcast.sun.intensity), DAY: V4 = [0.7, 0.3, 6, 1.9], MID = lookExpoFrame(DAY, 0.7, 1, lookConfig.glare, mx)!, MID0 = lookExpoFrame(DAY, 0.7, 1, lookConfig.glare, mx)!, GREY = lookExpoFrame(DAY, 0.7, 1, lookConfig.glare, gx)!;
    let same = true, held = true; for (let h = 5000; h > 1e-4; h *= 0.9) { for (const l of [0, 0.5, 3]) if (room(h, MID, DAY, W1, mx, l, GRADE) !== room(h, MID0, DAY, null, mx, l, GRADE)) same = false; const g = room(h, GREY, DAY, W1, gx); if (g > 4 + 1e-9 || (h >= DIM && g > 1 + 1e-9)) held = false; }
    check(same && held && DIM * mx < READ && MID.every((v, i) => v === MID0[i]) && GRADE[1] > LAMP && mx === Math.min(1, HOURS.midday.sun.intensity * LUX_PER_UNIT / HOUR_SUN_LUX.midday),
      `at midday the row moves no place and no light in one (the dim level in a midday's units, ${(DIM * mx).toExponential(1)}, is under the field's read floor, so a black room is not shown by its noise any further than it was; and sunlit ground is over the night's white), and on the grey day no place holding the dim level or more`); }
  check(gain(0.001, NIGHT, null, N_SHADE, NOGRADE, 0, W1) === 1 && gain(1e-6, NIGHT, null, N_SHADE, NOGRADE, 0, W1) === 1,
    'until the field is live no place of a night is shown past the night\'s exposure (the cache\'s read is not in the field\'s measure, and the bound takes no number from it)');
  { // a level under a blacker sky (the solo tutorial's: its sky puts 0.0135 on open ground, its yard holds about 0.012)
    const ys = 0.012, ws = lookExpoBound(1, 0.0135), a2 = (h: number, eye: number) => at(h, eye, ws, NIGHT, ys);
    check(ws[0] < ys / 2 && [ys, 0.02, 0.002].every(eye => a2(ys, eye) === 1 && a2(0.02, eye) === 1 && a2(L, eye) === 1) && Math.abs(a2(0.002, 0.002) - Math.pow(ws[0] / 0.002, X)) < 1e-9 && a2(0.002, 0.002) > 1.5,
      `under a sky six times blacker the dim level goes down with the yard (${ws[0].toFixed(4)}): that yard and a lit hall holding 0.02 are still sent on as the night drew them, from the yard, from the hall and from an unlit room, and the unlit room comes up by the same rule (× ${a2(0.002, 0.002).toFixed(2)})`); }
  { // light in the unlit room, from a thousandth of the night's white to twenty times it: what the eye is sent
    const g0 = at(U, U); let rises = true, plain = true, last = 0; for (let l = LAMP / 1000; l < LAMP * 20; l *= 1.05) { const g = at(U, U, W1, NIGHT, Y, l), shown = l * g; if (!(shown >= last) || shown < l - 1e-9) rises = false; if (l * g0 < LAMP && Math.abs(g - g0) > 1e-9) plain = false; last = shown; }
    check(rises && plain && COMMON.includes('  var P = select(frame.shadeCfg.w, frame.gradeCfg.y, frame.gradeCfg.y > 0.0);\n  P = max(P, EXPO_LAMP);'),
      'in a room brought up, light that stays under the night\'s white is brought up with it to the number, and from there up it never falls and never goes under what the night\'s own exposure shows; the shoulder is about the night\'s white wherever the hour\'s own is dimmer, at every hour'); }
  const bound = "  {\n    let over = max(frame.eyeCfg.x, 1.0); var most = over;\n    if (oa.z > 0.5) { most *= clamp(pow(frame.placeCfg.z / (over * max(here, EXPO_READ)), frame.shadeCfg.x), 1.0, max(frame.placeCfg.w, 1.0)); }";
  check(COMMON.includes(bound) && COMMON.includes('    base = min(base, most);\n  }\n  return base;') && !/else if \(frame\.eyeCfg\.x > 1\.0\)/.test(COMMON) && /eyesWidest\?\.\[0\] \?\? 0, eyesWidest\?\.\[1\] \?\? 0\]/.test(eng),
    'the kernel carries the eyes\' widest as the mirror has it, after every other line of a place\'s gain, with no test of a row and no second bound beside it'); }
{ // the map: what walls a place off, what a doorway is, and what a corridor is
  const R = 2.5, wallX = (x0: number, x1: number, z: number) => makeBox({ c: [(x0 + x1) / 2, 1.3, z], h: [(x1 - x0) / 2, 1.3, 0.1] });
  const crate = makeBox({ c: [5, 0.4, 5], h: [1, 0.4, 1] }), lintel = makeBox({ c: [5, 2.3, 8], h: [0.5, 0.2, 0.1] }), post = makeBox({ c: [8, 1.3, 5], h: [0.1, 1.3, 0.1] });
  const sill = makeBox({ c: [15, 0.45, 10], h: [0.6, 0.45, 0.1] }), pane = makeBox({ c: [15, 1.45, 10], h: [0.6, 0.55, 0.02], flags: BoxFlag.Glass | BoxFlag.NoShadow }), head = makeBox({ c: [15, 2.3, 10], h: [0.6, 0.3, 0.1] });
  const leaf = makeBox({ c: [30.5, 1, 20], h: [0.5, 1, 0.03], flags: BoxFlag.Dynamic }), man = makeBox({ c: [22, 0.9, 12], h: [0.25, 0.9, 0.2], flags: BoxFlag.Dynamic }), gone = makeBox({ c: [30, 1.3, 5], h: [3, 1.3, 0.1], flags: BoxFlag.NoPrimary | BoxFlag.NoShadow });
  // one long wall along z = 10 with a window in it (x 14.4‥15.6), a door's gap (x 20‥21, 1 m) and a cart gap (x 24‥26.4, 2.4 m); a corridor 1.2 m wide
  // between two walls (z 20 and z 21.4, x 4‥16) that opens at both ends; a shut door's leaf in a gap of a wall along z = 20 (x 30‥31)
  const slab = makeBox({ c: [10, 2.7, 5], h: [5, 0.1, 4] }), beam = makeBox({ c: [30, 2.5, 14], h: [3, 0.15, 0.15] });   // a ceiling over x 5‥15, z 1‥9; a beam over nothing much
  const boxes = [slab, beam, wallX(4, 14.4, 10), sill, pane, head, wallX(15.6, 20, 10), wallX(21, 24, 10), wallX(26.4, 36, 10), wallX(4, 16, 20), wallX(4, 16, 21.4), wallX(26, 30, 20), wallX(31, 36, 20), crate, lintel, post, gone];
  const m = placeMask(boxes, [leaf, man], undefined, [[30, 20, 31, 20]]), raw = (x: number, z: number) => m[Math.floor(z * R) * PLACE_W + Math.floor(x * R)], at = (x: number, z: number) => raw(x, z) === 255 ? 255 : raw(x, z) & 3;   // (255 = wall, 1 = a doorway's cell, 2 = any other free cell)
  check(placeWall(wallX(0, 4, 3)) === 255 && placeWall(leaf) === 255 && placeWall(lintel) === 0 && placeWall(post) === 0 && placeWall(man) === 0 && placeWall(gone) === 0 && placeWall(crate) > 0 && placeWall(crate) < 255
    && at(10, 10) === 255 && at(30.5, 20) === 255 && at(5, 5) === 2 && at(5, 8) === 2 && at(8, 5) === 2 && at(22, 12) === 2 && at(30, 5) === 2,
    'a wall and a shut door wall a place off (the door\'s line, told to the map: no leaf is stamped into it); a crate, a lintel on its own, a post, a man and a wall a blast has taken do not');
  check(placeWall(sill) > 0 && placeWall(sill) < 255 && placeWall(pane) > 0 && placeWall(pane) < 255 && placeWall(head) === 0 && (placeWall(sill) | placeWall(pane)) === 255 && at(15, 10) === 255 && at(14.6, 10) === 255 && at(15.4, 10) === 255,
    'a window is a wall: its sill and its pane fill the wall\'s height between them, so a room with windows is a place of its own and the light through them is the room\'s own reading');
  check(at(20.5, 10) === 1 && at(20.2, 10) === 1 && at(20.5, 9) === 2 && at(20.5, 11) === 2 && at(25, 10) === 2 && at(25.8, 10) === 2,
    'a 1 m gap in a wall is a doorway (its cells are marked, the floor each side is not), and a gap a cart goes through is not: the two sides are one place');
  check(at(10, 20.7) === 2 && at(5, 20.7) === 2 && at(15, 20.7) === 2 && at(10, 20) === 255 && at(10, 21.4) === 255,
    'a corridor as narrow as a doorway is no doorway along its length: it is one place from end to end');
  check(placeRoof(slab) && !placeRoof(beam) && !placeRoof(lintel) && !placeRoof(wallX(0, 4, 3)) && raw(10, 5) === 2 && raw(6, 2) === 2 && raw(20, 5) === 6 && raw(30, 14) === 6 && raw(20.5, 10) === 5 && raw(10, 10) === 255,
    'a ceiling slab is a roof over its cells and a beam or a lintel is not: every other free cell is under the open sky, which is what the field calls the open air'); }
process.exit(red ? 1 : 0);
