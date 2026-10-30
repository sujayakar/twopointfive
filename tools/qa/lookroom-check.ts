// The look prototype's Room contrast row (src/render/shaders/composite.wgsl, the line under the day's contrast being given back to a place under a roof, and
// lookShadeGain's z; render/frame.ts lookConfig.roomContrast → frame.gradeCfg.w; the panel's `roomc` row, ?look=roomc:0 puts it off). The Contrast row turns the
// day's stops about sunlit ground and the whole turn is taken back under a roof, so the yard had the row and a room had none (the maintainer, 10-30, on a GPU:
// "lighting still looks very flat indoors during midday (see flat shading on chair)"). With this row a room is turned by the same contrast about its own level.
// No GPU: the lines are mirrored here as numbers, with the tone map's turn as a power about its stop. The picture is never SEEN here.
//   bun run tools/qa/lookroom-check.ts   (exit 0 = all pass; LOOKROOM_BREAK=1 leaves a room without the turn and must fail)
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { LOOK_DEFAULTS } from '../../src/render/gbuffer';
import { lookConfig } from '../../src/render/frame';
import { lookEyeFrame } from '../../src/render/lookeye';
let red = 0; const check = (ok: boolean, w: string) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${w}`); if (!ok) red++; };
const BREAK = process.env.LOOKROOM_BREAK === '1', ROOT = resolve(import.meta.dir, '../..');
const c = readFileSync(resolve(ROOT, 'src/render/shaders/composite.wgsl'), 'utf8'), c2 = readFileSync(resolve(ROOT, 'src/render/shaders/compact/composite.wgsl'), 'utf8');
const ROOM_LEVEL = +(c.match(/\nconst ROOM_LEVEL = ([0-9.]+);/)?.[1] ?? NaN);
const sm = (a: number, b: number, x: number) => { const t = Math.min(1, Math.max(0, (x - a) / (b - a))); return t * t * (3 - 2 * t); };
const C = lookConfig.gradeC, X = 1 / C, P = 1.559;   // (the Contrast row as the page has it, and the radiance of sunlit ground it turns about at the pass's midday)
/** composite.wgsl under lookShadeGain: what a surface sending `l` back (after the shade gain) is handed to the tone map as, in a place `roof` under a roof whose level is `m` */
const sent = (l: number, roof: number, m: number, r: number, under = roof) => {
  let d = l; if (roof > 0 && l > 0 && l < P) d *= Math.min(Math.pow(l / P, (X - 1) * roof), 8);
  if (r > 0 && under > 0 && m > 0 && l > 0 && !BREAK) d *= Math.pow(Math.min(4, Math.max(0.125, l / m)), (1 - X) * Math.min(r, 1) * under);
  return d; };
const shown = (d: number) => P * Math.pow(d / P, C);   // (post.wgsl agx's turn, as light: C times the stops from P)
const K = LOOK_DEFAULTS.roomc;
{ let mono = true, same = true, lo = 1e9, hi = 0, step = 0;
  for (const m of [0.02, 0.05, 0.12, 0.3, 0.6].map(v => v * P)) for (const roof of [0.3, 1]) for (const r of [0.5, K, 1, 4]) { let last = -1;
    for (let i = 1; i <= 4000; i++) { const l = i / 2000 * P, d = sent(l, roof, m, r); if (d < last - 1e-12) mono = false; last = d;
      if (sent(l, roof, m, 0) !== sent(l, roof, 0, r) || sent(l, roof, m, r, 0) !== sent(l, roof, m, 0) || sent(l, 0, m, r, 0) !== l) same = false;
      const f = d / sent(l, roof, m, 0); lo = Math.min(lo, f); hi = Math.max(hi, f); }
    step = Math.max(step, Math.abs(sent(P * 0.9999, roof, m, r) / sent(P, roof, m, r) - 1)); }
  check(same, 'with the row at 0, out from under a roof, or with no level, a pixel is sent on exactly as it was');
  check(mono, 'a surface sending more light back is never shown darker than one sending less, from black to twice sunlit ground, at every height of the row and with the row asked for 4');
  check(step < 1e-3, 'and there is no step where a pixel is as bright as sunlit ground, where the give-back stops');
  check(BREAK || (lo > 0.58 && lo < 0.6 && hi > 1.42 && hi < 1.44), `the row never takes a pixel under ${lo.toFixed(2)} of what it was or over ${hi.toFixed(2)} times it (three stops under the room's level to two over)`); }
{ // a shadow's edge in a porch: the pixel just in full light and the one just out of it carry one weight for this row, whatever the give-back's two spans say
  let rim = true; for (const open of [1.5, 2.2, 2.6, 3, 4, 8]) { const under = sm(2, 3, open), lit = sent(0.5 * P, sm(2, 4, open), 0.1 * P, 1, under), shade = sent(0.5 * P, sm(1.25, 2.5, open), 0.1 * P, 1, under);
    if (lit / sent(0.5 * P, sm(2, 4, open), 0.1 * P, 0) !== shade / sent(0.5 * P, sm(1.25, 2.5, open), 0.1 * P, 0) && Math.abs(lit / sent(0.5 * P, sm(2, 4, open), 0.1 * P, 0) / (shade / sent(0.5 * P, sm(1.25, 2.5, open), 0.1 * P, 0)) - 1) > 1e-12) rim = false;
    if (open <= 2 && lit !== sent(0.5 * P, sm(2, 4, open), 0.1 * P, 0)) rim = false; }
  check(rim, 'the row turns a pixel in full light and its neighbour in shade by one weight (no rim round a sun patch from this row), and by none where the place holds half the open air\'s light or more'); }
{ // two faces of one thing in a room, well under sunlit ground: one sending 3 times the other's light back, the room's level between them
  const m = 0.08 * P, a = m / Math.sqrt(3), b = m * Math.sqrt(3), ratio = (r: number) => shown(sent(b, 1, m, r)) / shown(sent(a, 1, m, r)), yard = shown(0.3 * P) / shown(0.1 * P);
  check(Math.abs(ratio(0) - 3) < 0.01, `at 0 a room is shown with the light as it is: a face sending 3 times another's light back is shown ${ratio(0).toFixed(2)} times brighter, where two such faces in the yard's shade are ${yard.toFixed(2)} apart`);
  check(Math.abs(ratio(1) - yard) < 0.01 && ratio(K) > ratio(0) + 0.5 * (K - 0.01) && ratio(K) <= yard + 0.01, `at 1 a room has the yard's contrast (${ratio(1).toFixed(2)}), and at the row's own ${K} it is ${ratio(K).toFixed(2)}`);
  check(Math.abs(shown(sent(m, 1, m, 1)) / shown(sent(m, 1, m, 0)) - 1) < 1e-9, 'a surface at the room\'s own level is shown where it was: the room is turned about it, neither brought up nor taken down'); }
{ // the level, composite.wgsl lookShadeGain's z, on lookeye-check's stand (the shade row at 0.7 by day; open air's mean radiance 0.1; open ground gets 1)
  const shade: [number, number, number, number] = [0.7, 0.1, 6, 1];
  const level = (here: number, eye: readonly number[] | null) => { const e = eye ?? [0, 0, 0, 0], out = Math.max(e[2], 1);
    const open = Math.max(shade[1] / Math.max(here, 1e-4), 1), sam = Math.max(e[0], 1), roof = sm(1.25, 2.5, open), own = 1 + (open - 1) * roof, samHere = 1 + (sam - 1) * roof;
    const dark = own > sam && e[1] > 0 ? own * Math.pow(sam / own, e[1]) : Math.max(own, samHere), place = Math.min(Math.max(shade[2], e[3]), Math.max(1, Math.pow(dark, shade[0])));
    return ROOM_LEVEL * shade[1] / dark * place / (1 + (place / out - 1) / dark); };
  const room = lookEyeFrame({ log: 0, snap: true }, shade, 0.125, 1, 1 / 60, lookConfig.glare)!;   // (Sam in a room an eighth as bright as the open)
  check(ROOM_LEVEL > 0 && Math.abs(level(0.033, room) / level(0.0125, room) - 1) < 1e-9 && level(0.0125, room) < shade[1] && level(0.0125, room) > 0.02 * shade[1],
    `in Sam's room the window's end and the middle have ONE level (${(level(0.0125, room) / shade[1]).toFixed(2)} of the open air's light), so the fall-off from the window is turned about it with everything else`);
  check(level(0.004, null) < level(0.0125, null) && level(0.0125, null) < level(0.033, null), 'with Sam out of doors each room is turned about its own level, a darker room\'s the lower'); }
{ const eng = readFileSync(resolve(ROOT, 'src/engine.ts'), 'utf8'), panel = readFileSync(resolve(ROOT, 'src/ui/lookpanel.ts'), 'utf8');
  const fn = (t: string) => { const a = t.indexOf('fn lookShadeGain('), b = t.indexOf('\n}\n', a); return a < 0 ? '' : t.slice(a, b); };
  const line = 'if (frame.gradeCfg.x > 0.0 && frame.gradeCfg.w > 0.0 && sg.w > 0.0 && sg.z > 0.0 && lr > 0.0) { dif *= pow(clamp(lr / sg.z, 0.125, 4.0), (1.0 - frame.gradeCfg.x) * min(frame.gradeCfg.w, 1.0) * sg.w); }';
  check(fn(c).length > 500 && fn(c) === fn(c2) && c.includes(line) && c2.includes(line) && c.includes('shadeG = sg.x; dif *= shadeG; let lr = luminance(dif);\n') && c2.includes('shadeG = sg.x; dif *= shadeG; let lr = luminance(dif);\n')
    && fn(c).includes('let roomC = frame.gradeCfg.w > 0.0 && frame.gradeCfg.x > 0.0 && frame.gradeCfg.z > 0.0;') && fn(c).includes('if (lit && !roomC && !(frame.gradeCfg.z > 0.0 && frame.gradeCfg.x > 0.0 && l0 * out > 0.0 && l0 * out < frame.gradeCfg.y)) { return vec4f(out, 0.0, 0.0, 0.0); }')
    && fn(c).includes('let under = smoothstep(2.0, 3.0, open);') && fn(c).includes('if (lit) { return vec4f(out, smoothstep(2.0, 4.0, open), level, under); }') && fn(c).includes('Epix / frame.shadeCfg.w), roof, level, under);') && (fn(c).match(/diceMeanRadiance\(/g) ?? []).length === 0 && fn(c).includes('var level = 0.0; if (roomC) { level = ROOM_LEVEL * frame.shadeCfg.y / dark * place / (1.0 + (place / out - 1.0) / dark); }') && c2.includes(`\nconst ROOM_LEVEL = ${c.match(/\nconst ROOM_LEVEL = ([0-9.]+);/)?.[1]};`),
    'both writings of the kernel carry the mirrored lines, the level as it is mirrored and one number for where a room\'s middle sits; with the row at 0 (or ?footing=0) a pixel in full light returns as it did, with it up every pixel takes the walled read, and the row has one roof weight for a pixel in light and in shade');
  check(/lookConfig\.footing, lookConfig\.roomContrast\] : undefined/.test(eng) && /\['roomc', 'Room contrast', /.test(panel) && lookConfig.roomContrast === K && K > 0 && K <= 1,
    'the engine hands the row to the frame and the panel has the row, at the number the page starts with'); }
process.exit(red ? 1 : 0);
