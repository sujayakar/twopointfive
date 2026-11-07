// HOW SMOKE IS DRAWN ON THE LOOK PAGE (src/render/smokeLook.ts): the lab's tuned numbers are the page's own, and only there.
//   the table     it is the numbers that were handed back from the lab, row for row
//   the solver    the numbers of the motion and of a can in that paste are the build's own defaults (so nothing of the smoke itself moved)
//   where         each row lands where the renderer reads it; with the page's switch off nothing is written and a preset's step is its own
//   the step      a preset's march step in the table's proportion, never under a cell
//   the wiring    the page turns it on for the look alone (`?smokelook=0` off), before a preset is put on; the preset's one writer asks for the step;
//                 a benchmark asked for on the look page carries the word
// No GPU: nothing here says what the smoke looks like.
//   bun run tools/qa/smokelook-check.ts                     exit 0 = every line passed, 1 = one failed
//   SMOKELOOK_BREAK=1 bun run tools/qa/smokelook-check.ts   the paste is read with one row moved: must FAIL
import { readFileSync } from 'node:fs';
import { ROOT } from './headless.ts';
const { SMOKE_LOOK, smokeLook, smokeLookStep, putSmokeLook } = await import(`${ROOT}/src/render/smokeLook.ts`);
const { defaultSmokeParams } = await import(`${ROOT}/src/smoke/smoke.ts`);
const { lookPageWord } = await import(`${ROOT}/src/scene/look.ts`);
const BRK = process.env.SMOKELOOK_BREAK === '1'; let red = false;
const check = (ok: boolean, what: string, detail = '') => { red ||= !ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
console.log(`\n=== smokelook: how smoke is drawn on the look page${BRK ? ' (one row of the paste moved: must fail)' : ''} ===`);

// the lab's "copy them", as handed back (the rows a quality preset owns left out: marchLights, quarter, scaledFreely)
const PASTE = { params: { buoyancy: 1.6, weight: 0.03, velDamp: 0.3, densDecay: 0.14, tempDecay: 1.2, rbIters: 4, omega: 1.7, vortConf: 4, turbulence: 2.5, stirConf: 2, canisterDensity: 95, canisterDuration: 32, canisterRadius: 0.2, canisterTemp: 1.6, canisterSpeed: 1.9, densityScale: 1.3, fxDensityScale: 1, calmCells: 0.5 },
  draw: { creases: 0.6, selfK: 10.5, selfReach: 0.58, ahead: BRK ? 0 : 0.65, keep: 0, glowThrough: 0.15, blurBehind: 0, blurStart: 0, leanOnNew: 1, stepCells: 1 } } as const;
{ const off = Object.entries(PASTE.draw).filter(([k, v]) => (SMOKE_LOOK as Record<string, number>)[k] !== v).map(([k, v]) => `${k}: ${(SMOKE_LOOK as Record<string, number>)[k]} against ${v}`);
  check(!off.length && Object.keys(SMOKE_LOOK).length === Object.keys(PASTE.draw).length, 'the table is the lab\'s numbers, row for row', off.join(' · ') || `${Object.keys(SMOKE_LOOK).length} rows`); }
{ const D = defaultSmokeParams() as Record<string, unknown>, off = Object.entries(PASTE.params).filter(([k, v]) => D[k] !== v).map(([k, v]) => `${k}: ${D[k]} against ${v}`);
  check(!off.length, 'the motion\'s and the can\'s numbers in the paste are the solver\'s own defaults', off.join(' · ') || `${Object.keys(PASTE.params).length} numbers`); }

{ const stub = () => ({ settings: { smokeMultiScatter: 0, smokeDiffuseStart: 0.35, smokeTaaReact: 0.3 }, smokeDraw: { creases: 0, selfK: 0, selfReach: 0, ahead: 0 }, passes: { smokeDiffuse: 1, smokeDiffuseKeep: 0 } });
  const e = stub(); putSmokeLook(e as never); const L = SMOKE_LOOK;
  check(e.smokeDraw.creases === L.creases && e.smokeDraw.selfK === L.selfK && e.smokeDraw.selfReach === L.selfReach && e.smokeDraw.ahead === L.ahead && e.passes.smokeDiffuse === L.blurBehind && e.passes.smokeDiffuseKeep === L.keep
    && e.settings.smokeMultiScatter === L.glowThrough && e.settings.smokeDiffuseStart === L.blurStart && e.settings.smokeTaaReact === L.leanOnNew, 'each row lands where the renderer reads it', JSON.stringify(e)); }

{ const was = smokeLook.on; smokeLook.on = false; const plain = [1.5, 2].map(smokeLookStep) as number[]; smokeLook.on = true; const look = [1.5, 2, 1, 0.5].map(smokeLookStep) as number[]; smokeLook.on = was;
  check(plain[0] === 1.5 && plain[1] === 2, 'off the look page a preset\'s march step is its own', plain.join(', '));
  check(look[0] === 1 && Math.abs(look[1] - 4 / 3) < 1e-9 && look[2] === 1 && look[3] === 1, 'on it high\'s step is the tuned one, a cheaper preset\'s is longer in proportion, and none is under a cell', look.map(x => +x.toFixed(3)).join(', ')); }

{ const main = readFileSync(`${ROOT}/src/main.ts`, 'utf8'), q = readFileSync(`${ROOT}/src/ui/quality.ts`, 'utf8');
  const on = main.indexOf('smokeLook.on = lookConfig.on && q.get(\'smokelook\') !== \'0\''), preset = main.indexOf('new Quality('), put = main.indexOf('if (smokeLook.on) putSmokeLook(engine)');
  check(on > 0 && preset > on && put > 0, 'the page turns it on for the look alone, before the first preset is put on, and writes the table once', `switch at ${on}, first preset at ${preset}, table at ${put}`);
  check(/S\.volStepVoxels = smokeLookStep\(K\.volStep\)/.test(q) && (q.match(/S\.volStepVoxels = /g) ?? []).length === 1, 'the preset\'s one writer of the march step asks the table');
  check(lookPageWord('smokelook', '0'), 'a benchmark asked for on the look page carries ?smokelook'); }

console.log(red ? '\n=== smokelook: FAILED ===' : '\n=== smokelook: all pass ===');
process.exit(red ? 1 : 0);
