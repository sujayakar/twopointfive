// The look prototype's "Blowing snow" row, the frame's side (src/render/lookveil.ts lookVeilFrame): the frame says WHERE the powder's field lies over the world
// and where the sun is, and nothing of any wind (the powder has none of its own: it rides the game's air, render/powderField.ts, whose own check is
// tools/qa/powder-field-page.cjs). And the two writings of the shader that draws it (shaders/composite.wgsl and its compact twin) carry the same powder.
// No GPU: the powder itself is never SEEN here.
//   bun run tools/qa/lookveil-check.ts   (exit 0 = all pass; LOOKVEIL_BREAK=1 hands the frame a plan twice as wide as the field's and must fail)
import { lookVeilFrame } from '../../src/render/lookveil';
import { POW_RES } from '../../src/render/powderField';
import { AIR_RES } from '../../src/render/airField';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
let red = 0; const check = (ok: boolean, w: string) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${w}`); if (!ok) red++; };
const BREAK = process.env.LOOKVEIL_BREAK === '1';
const near = (a: number, b: number) => Math.abs(a - b) < 1e-6;
const sun = { kind: 2, enabled: true, intensity: 6, dir: [-0.5, -0.7071, 0.5], color: [1, 0.9, 0.7] }, bulb = { kind: 0, enabled: true, intensity: 26, dir: [0, -1, 0], color: [1, 0.6, 0.3] };
const plan = { x: -20, z: -14, w: 40, d: 28 };
const frame = (amount = 1, lights: any[] = [bulb, sun], world: typeof plan | null = plan) => lookVeilFrame(world && BREAK ? { ...world, w: world.w * 2 } : world, amount, lights);

check(frame(1, [bulb, sun], null) === null && frame(0) === null && lookVeilFrame({ ...plan, w: 0 }, 1, [sun]) === null, 'no field, a plan of no size, or none of it drawn: no fields (the frame\'s stay zeros and the shader skips it)');
{ // a texel's middle lands on its own place in the world: the shader reads the field at (q − corner) × (1 ∕ size)
  const f = frame()!, w = Math.round(plan.w * AIR_RES) * POW_RES / AIR_RES, d = Math.round(plan.d * AIR_RES) * POW_RES / AIR_RES;
  const uv = (x: number, z: number) => [(x - f[1][0]) * f[0][0], (z - f[1][1]) * f[0][1]];
  const a = uv(plan.x + 0.05, plan.z + 0.05), b = uv(plan.x + plan.w - 0.05, plan.z + plan.d - 0.05);
  check(!!f && near(a[0], 0.5 / w) && near(a[1], 0.5 / d) && near(b[0], 1 - 0.5 / w) && near(b[1], 1 - 0.5 / d), `the field's first cell (a ${1 / POW_RES} m cell) is read at its own middle and its last at its own: ${w} × ${d} cells over a ${plan.w} × ${plan.d} m plan`); }
{ const half = frame(0.5)!, more = frame(3)!; check(!!half && near(half[0][2], 0.5) && near(more[0][2], 1), 'the amount drawn is the caller\'s, never over 1'); }
{ const day = frame(), dark = frame(1, [bulb]), low = frame(1, [{ ...sun, dir: [-0.99, -0.1, 0] }]), off = frame(1, [{ ...sun, enabled: false }]);
  check(!!day && !!dark && !!low && !!off && near(day[1][2], 0.5 / 0.7071) && near(day[1][3], -0.5 / 0.7071) && dark[1][2] === 0 && dark[1][3] === 0 && off[1][2] === 0 && near(low[1][2], 0.99 / 0.25) && Math.abs(day[0][3] - 6 * 0.7071 * (0.2126 + 0.7152 * 0.9 + 0.0722 * 0.7)) < 1e-4 && dark[0][3] === 0 && Math.abs(low[0][3] - 6 * 0.1 * (0.2126 + 0.7152 * 0.9 + 0.0722 * 0.7)) < 1e-4
    && frame(1, [{ ...sun, transient: true }])?.[0][3] === 0,
    'its lit side and its shade lean to the sun that is on (the way to it along the ground per metre of height); with none, or with it off, there is neither; a sun on the horizon is held to four metres a metre; and the frame says what that sun puts on level ground, by its colour and its true height (a lightning stroke is not the sun), so a lamp\'s pool is not shaded as the sun\'s'); }
{ // the shader's two writings
  const cut = (f: string) => { const s = readFileSync(resolve(import.meta.dir, '../../src/render/shaders', f), 'utf8'); const a = s.indexOf('// ---- THE LOOK PROTOTYPE — LOOSE SNOW IN THE AIR'), b = s.indexOf('fn lookShadeGain('); const c = s.indexOf('if (LOOK) { if (frame.veilA.z > 0.0'); return a < 0 || c < 0 ? '' : s.slice(a, s.lastIndexOf('/**', b)) + s.slice(c, s.indexOf('var shadeG = 1.0;', c)) + s.slice(b, s.indexOf('\n}', b)); };
  const a = cut('composite.wgsl'), b = cut('compact/composite.wgsl');
  check(a.length > 2000 && a === b && /fn lookPowder\(/.test(a) && /textureSampleLevel\(powderTex, linSamp, \(q - frame\.veilB\.xy\) \* frame\.veilA\.xy/.test(a) && /max\(POWDER_ALBEDO \* [^;]*, ground\)/.test(a) && !/veilDensity|snowSparkleHash\(c \+ vec2i\(k/.test(a),
    'the kernel and its compact twin carry the same powder and the same eye, word for word; the powder is READ from the field (no pattern of the shader\'s own), and it is never drawn darker than what is under it'); }
process.exit(red ? 1 : 0);
