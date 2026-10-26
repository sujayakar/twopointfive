// A SAVE FROM A BUILD THAT HAD PLAN MODE STILL LOADS (src/game/savestate.ts: the `plan` slice went out with the mode; SAVE_VERSION did not move, so a
// file written before carries a `plan` key no reader asks for).
//   bun run tools/qa/saveplan-check.ts            (also `probes.ts saveplan`, in family checkpoints)
//   TPS_PROFILE=shipped bun run tools/qa/saveplan-check.ts      — the same lines on the page's stand
// PASS / FAIL each, headless: the solo night on the office (no file holds a teammate: savestate.ts canSave), saved two seconds in. The same file is
// then loaded twice into that game: as it is written today, and with the `plan` slice a plan mode build wrote laid in (the mode up, two men picked,
// a chain of two stops under way, the camera's look and aim — more than any such file held: the page wrote the slice idle).
//   1. the file with the old slice is not refused (canLoad) and its load does not throw.
//   2. the ten seconds after it are the ten seconds after the file without it: one digest, sampled once a second under one seed.
//   3. a file written after that load has no `plan` key.
// SAVEPLAN_BREAK=1 = the second load is of a file with a guard moved a metre, not of the one with the old slice: line 2 must fail.
//   exit 0 = all hold · 1 = one does not
import { standUp, ROOT } from './headless.ts';
import { digestRun, seeded } from './checkpoint-probes.ts';
const { saveState, loadState, canLoad } = await import(`${ROOT}/src/game/savestate.ts`);
const { knobsCopy, knobsBack } = await import(`${ROOT}/src/game/knobs.ts`);
const DT = 1 / 60, SEED = 5, RUN_SEED = 90210, SECS = 10, BREAK = process.env.SAVEPLAN_BREAK === '1', VERBOSE = process.argv.includes('--verbose');

export async function savePlanCheck(): Promise<boolean> {
  let fails = 0;
  const line = (ok: boolean, what: string, got: string) => { console.log(`  ${ok ? 'PASS' : 'FAIL'}  ${what} — ${got}`); if (!ok) fails++; };
  const knobs0 = knobsCopy(), rnd0 = Math.random;
  console.log(`A save from a build that had plan mode still loads (src/game/savestate.ts)${BREAK ? ' — SAVEPLAN_BREAK=1: the second file is another state, line 2 must fail' : ''}`);
  try {
    const quiet = async <X,>(f: () => X | Promise<X>): Promise<X> => { if (VERBOSE) return await f(); const l = console.log, w = console.warn; console.log = console.warn = () => { }; try { return await f(); } finally { console.log = l; console.warn = w; } };
    Math.random = seeded(SEED); const W: any = await quiet(() => standUp()), g = W.game;
    await quiet(() => { for (let i = 0; i < 120; i++) W.step(DT); });
    const file = saveState(g, { label: 'saveplan', kind: 'harness', force: true }), json = JSON.stringify(file), had = 'plan' in file;
    // the slice as a plan mode build wrote it (planmode.ts planSnapshot, as it was). A page of those builds wrote one into every file, idle: no file
    // holds a teammate (savestate.ts canSave), so the mode was never up in one. Asked here with everything in it all the same: the mode up, a pick, a
    // chain under way, the look and the aim — men by names nobody on this floor has
    const names = ['Red One', 'Red Two'], p0 = g.player.char.pos, at = (k: number): number[] => [p0[0] + k, 0, p0[2] + 1];
    const old = JSON.parse(json); old.plan = { up: true, pick: names, showT: 1.5, look: at(0), aim: at(1), stood: true, back: { from: at(2), t0: 0.4 },
      chains: [{ who: names, phase: 'go', spent: 0, t: 0.8, steps: [{ t: { kind: 'floor', at: at(2) }, look: [1, 0] }, { t: { kind: 'floor', at: at(3) }, wait: true }] }] };
    if (BREAK) { delete old.plan; old.guards[0].pos[0] += 1; }
    // 1
    const refused = canLoad(g, old); let threw = ''; const run = async (s: any) => { await quiet(() => loadState(g, s)); return await quiet(() => digestRun(W, SECS, RUN_SEED)); };
    const A = await run(JSON.parse(json)); let B: any = null; try { B = await run(old); } catch (e) { threw = String(e).slice(0, 160); }
    line(!had && refused === null && !threw, '1. a file carrying the old `plan` slice is not refused and its load does not throw', `today's file has the key: ${had} · canLoad: ${refused ?? 'yes'} · the load: ${threw || 'ran'}`);
    // 2
    line(!!B && A.hash === B.hash, '2. the ten seconds after it are the ten seconds after the same file without the slice', `without ${A.hash} · with ${B?.hash ?? 'none'} (${SECS} s, a sample a second, seed ${RUN_SEED})`);
    // 3
    { await quiet(() => loadState(g, JSON.parse(JSON.stringify(old)))); const after = saveState(g, { label: 'saveplan', kind: 'harness', force: true });
      line(!('plan' in after), '3. a file written after that load has no `plan` key', `keys: ${Object.keys(after).length}, 'plan' among them: ${'plan' in after}`); }
  } finally { knobsBack(knobs0); Math.random = rnd0; }
  console.log(fails ? `=== saveplan-check: ${fails} FAIL ===` : '=== saveplan-check: ALL PASS ===');
  return !fails;
}

if (import.meta.main) process.exit((await savePlanCheck()) ? 0 : 1);
