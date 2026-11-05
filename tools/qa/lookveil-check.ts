// The look prototype's "Blowing snow" row, the frame's side (src/render/lookveil.ts lookVeilFrame): the frame says WHERE the powder's field lies over the world
// and where the sun is, and nothing of any wind (the powder has none of its own: it rides the game's air, render/powderField.ts, whose own check is
// tools/qa/powder-field-page.cjs). And the two writings of the shader that draws it (shaders/composite.wgsl and its compact twin) carry the same powder.
// No GPU: the powder itself is never SEEN here.
//   bun run tools/qa/lookveil-check.ts   (exit 0 = all pass; LOOKVEIL_BREAK=1 hands the frame a plan twice as wide as the field's and must fail)
import { lookVeilFrame } from '../../src/render/lookveil';
import { POW_RES, POWDER_DRAW, POWDER_DRAW_OWN, POWDER_DRAW_STEP, powderDrawWord } from '../../src/render/powderField';
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
  check(a.length > 2000 && a === b && /fn lookPowder\(/.test(a) && /textureSampleLevel\(powderTex, linSamp, \(q - frame\.veilB\.xy\) \* frame\.veilA\.xy/.test(a) && /max\(POWDER_ALBEDO \* [^;]*, ground\)/.test(a) && /powderThreads\(q, t\.ba\)/.test(a) && !/veilDensity|snowSparkleHash\(c \+ vec2i\(k|airWind|frame\.wind/.test(a),
    'the kernel and its compact twin carry the same powder and the same eye, word for word; where the powder is, how much and which way it runs are READ from the field (its threads are drawn along the field\'s own velocity: the shader has no wind of its own), and it is never drawn darker than what is under it'); }
{ // THE LOOK PANEL'S SNOW WISPS ROWS: the table's numbers are the shader's own consts, an untouched table is the word 0, and a moved row is its own byte
  const src = readFileSync(resolve(import.meta.dir, '../../src/render/shaders/composite.wgsl'), 'utf8'), num = (n: string) => +(new RegExp(`const ${n}: f32 = ([0-9.]+);`).exec(src)?.[1] ?? NaN);
  const own = { gain: num('POWDER_GAIN'), thin: num('POWDER_THIN'), thread: num('POWDER_THREAD'), grain: num('POWDER_GRAIN') };
  check(Object.entries(own).every(([k, v]) => v === (POWDER_DRAW_OWN as Record<string, number>)[k] && v === (POWDER_DRAW as Record<string, number>)[k]) && powderDrawWord() === 0,
    `the drawing rows open on the shader's own numbers (${JSON.stringify(own)}) and the frame's word is 0: the picture as it was`);
  // a thread's reads are under a grain apart WHATEVER the grain's width (the step is said in grains: in metres over the grain's const it was 0.6 on the panel at any row and would have been 1.4 with a tuned width made the const)
  check(num('POWDER_STEP') > 0 && num('POWDER_STEP') < 1 && /let d = v \* \(POWDER_STEP \* run \/ max\(sp, 1e-3\)\);/.test(src) && !/POWDER_REACH/.test(src),
    `a thread's reads lie ${num('POWDER_STEP')} of a grain apart, under 1, and the step asks no width: no row of beads at any grain`);
  const units = [...src.matchAll(/powderRow\((\d+)u, POWDER_(\w+), ([0-9. /]+)\)/g)].map(m => [m[2].toLowerCase(), +m[1], m[3].includes('/') ? 1 / 255 : +m[3]] as const);
  const K = POWDER_DRAW_STEP as Record<string, number>, SH: Record<string, number> = { gain: 0, thin: 8, thread: 16, grain: 24 };
  check(units.length === 4 && units.every(([k, sh, u]) => SH[k] === sh && Math.abs(u - K[k]) < 1e-9), `the shader reads each row from its own byte at the table's step (${units.map(u => u.join(' ')).join(' · ')})`);
  const D = POWDER_DRAW as Record<string, number>, was = { ...D }; D.gain = 4.5; D.thin = 0; D.grain = 0.1;
  const w = powderDrawWord(); Object.assign(D, was);
  check((w & 255) === 90 && ((w >>> 8) & 255) === 1 && ((w >>> 16) & 255) === 0 && (w >>> 24) === 100 && powderDrawWord() === 0, `a moved row goes as its nearest step, a row put to nothing as one step (0 is the shader's own), an unmoved one as 0 (0x${w.toString(16)})`); }
{ // THE STEP'S PROGRAM IS COMPILED ONCE A DEVICE: a field made again (both snow rows put to nothing and back) asks for no second compile
  const G = globalThis as any; G.GPUTextureUsage ??= { TEXTURE_BINDING: 4, COPY_DST: 2, COPY_SRC: 1, STORAGE_BINDING: 8 }; G.GPUBufferUsage ??= { UNIFORM: 64, COPY_DST: 8 }; G.GPUShaderStage ??= { COMPUTE: 4 };
  const { PowderField } = await import('../../src/render/powderField');
  const fake = () => { let made = 0; const thing: any = new Proxy(function () { }, { get: (_t, k) => k === 'then' ? undefined : thing, apply: () => thing });
    const dev: any = new Proxy({}, { get: (_t, k) => k === 'createComputePipelineAsync' ? () => { made++; return Promise.resolve(thing); } : k === 'createShaderModule' ? () => ({ getCompilationInfo: () => Promise.resolve({ messages: [] }) }) : thing });
    return { dev, made: () => made }; };
  const A = fake(), B = fake(), air = { w: 200, h: 140, view: {} as any, cells: {} as any };
  const f1 = new PowderField(A.dev, air); await f1.whenReady; f1.destroy(); const f2 = new PowderField(A.dev, air); await f2.whenReady; const f3 = new PowderField(B.dev, air); await f3.whenReady;
  check(A.made() === 1 && B.made() === 1 && f2.ready && f3.ready, `a second field on one device compiles nothing more, another device compiles its own (${A.made()} and ${B.made()})`); }
process.exit(red ? 1 : 0);
