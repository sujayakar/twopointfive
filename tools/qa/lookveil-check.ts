// The look prototype's "Blowing snow" row, the frame's side (src/render/lookveil.ts lookVeilFrame; game/snowFx.ts veilWind gives it the storm's air): the veil
// takes the wind's way and speed, comes and goes with its strength, runs on by the wind summed over the frames, and leans its light to the sun that is up.
// And the two writings of the shader that draws it (shaders/composite.wgsl and its compact twin) carry the same veil. No GPU: the veil itself is never SEEN here.
//   bun run tools/qa/lookveil-check.ts   (exit 0 = all pass; LOOKVEIL_BREAK=1 hands every frame a still wind's strength and must fail)
import { LOOK_VEIL, lookVeilFrame } from '../../src/render/lookveil';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
let red = 0; const check = (ok: boolean, w: string) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${w}`); if (!ok) red++; };
const BREAK = process.env.LOOKVEIL_BREAK === '1';
const near = (a: number, b: number) => Math.abs(a - b) < 1e-6;
const sun = { kind: 2, enabled: true, intensity: 6, dir: [-0.5, -0.7071, 0.5] }, bulb = { kind: 0, enabled: true, intensity: 26, dir: [0, -1, 0] };
const frame = (w: [number, number] | null, amount = 1, dt = 1 / 60, drift: [number, number] = [0, 0], lights: any[] = [bulb, sun]) => lookVeilFrame(w && BREAK ? [w[0] * 0.01, w[1] * 0.01] : w, amount, dt, drift, lights);

check(frame(null) === null && frame([0, 0]) === null && frame([0.2, 0.1]) === null, `still air, and a breeze under ${LOOK_VEIL.from} m/s, has no veil`);
{ const lull = frame([0.55, 0.2])!, quiet = frame([0.8, 0.3])!, gust = frame([1.08, 0.4])!;
  check(!!lull && !!quiet && !!gust && lull[0][2] > 0 && lull[0][2] < quiet[0][2] && quiet[0][2] < gust[0][2] && gust[0][2] === 1, `the veil thins in a lull and fills in a gust: ${lull?.[0][2].toFixed(2)} at 0.59 m/s, ${quiet?.[0][2].toFixed(2)} at the snow's own 0.85, ${gust?.[0][2].toFixed(2)} at 1.15`);
  check(!!quiet && near(quiet[0][0], 0.8 * LOOK_VEIL.speedK) && near(quiet[0][1], 0.3 * LOOK_VEIL.speedK), `it runs the wind's own way at ${LOOK_VEIL.speedK} times its speed (${(0.854 * LOOK_VEIL.speedK).toFixed(1)} m/s in the snow's weather)`); }
{ const half = frame([1.08, 0.4], 0.5)!; check(!!half && near(half[0][2], 0.5), 'the row at a half is half the veil'); }
{ // the drift: summed, so a change of wind never jumps the pattern
  const d: [number, number] = [0, 0]; for (let i = 0; i < 60; i++) frame([0.8, 0.3], 1, 1 / 60, d);
  const one = [d[0], d[1]]; for (let i = 0; i < 60; i++) frame([1.6, 0.6], 1, 1 / 60, d);
  check(near(one[0], 0.8 * LOOK_VEIL.speedK) && near(one[1], 0.3 * LOOK_VEIL.speedK) && near(d[0], 3 * one[0]) && near(d[1], 3 * one[1]), `in a second of the snow's wind the streamers run ${Math.hypot(one[0], one[1]).toFixed(2)} m; a second at twice the wind adds twice that to where they were`);
  const was = [d[0], d[1]]; frame([0.8, 0.3], 1, 5, d); frame([0.1, 0], 1, 1 / 60, d);
  check(near(d[0] - was[0], 0.8 * LOOK_VEIL.speedK * 0.1 + 0.1 * LOOK_VEIL.speedK / 60) && frame([0.8, 0.3], 1, 0, d)?.[1][0] === d[0], 'a frame of five seconds moves them a tenth of a second\'s worth, a breeze too soft to show still carries them on, and the frame is handed where they are'); }
{ const day = frame([0.8, 0.3]), dark = frame([0.8, 0.3], 1, 1 / 60, [0, 0], [bulb]), low = frame([0.8, 0.3], 1, 1 / 60, [0, 0], [{ ...sun, dir: [-0.99, -0.1, 0] }]), off = frame([0.8, 0.3], 1, 1 / 60, [0, 0], [{ ...sun, enabled: false }]);
  check(!!day && !!dark && !!low && !!off && near(day[1][2], 0.5 / 0.7071) && near(day[1][3], -0.5 / 0.7071) && dark[1][2] === 0 && dark[1][3] === 0 && off[1][2] === 0 && near(low[1][2], 0.99 / 0.25),
    'its lit side and its shade lean to the sun that is on (the way to it along the ground per metre of height); with none, or with it off, there is neither; a sun on the horizon is held to four metres a metre'); }
{ // the shader's two writings
  const cut = (f: string) => { const s = readFileSync(resolve(import.meta.dir, '../../src/render/shaders', f), 'utf8'); const a = s.indexOf('// ---- THE LOOK PROTOTYPE — BLOWING SNOW'), b = s.indexOf('fn lookShadeGain('); const c = s.indexOf('if (LOOK) { if (frame.veilA.z > 0.0'); return a < 0 || c < 0 ? '' : s.slice(a, s.lastIndexOf('/**', b)) + s.slice(c, s.indexOf('var shadeG = 1.0;', c)); };
  const a = cut('composite.wgsl'), b = cut('compact/composite.wgsl');
  check(a.length > 2000 && a === b && /fn veilDensity\(/.test(a) && /max\(VEIL_ALBEDO \* [^;]*, ground\)/.test(a), 'the kernel and its compact twin carry the same veil, word for word, and neither draws it darker than the snow under it'); }
process.exit(red ? 1 : 0);
