// WHO IS TOLD WHEN THE MISSION'S THING IS TAKEN (src/game/takecheck.ts; the maintainer, 10-12: "predictably getting a guard to come by when you pull the objective
// is a bit contrived. makes sense for the server rack in the office but not the officer files").
//   bun run tools/qa/takecheck-check.ts            (also `probes.ts takecheck`, in families `quiet` and `missions`)
//   TAKECHECK_BREAK=1 bun run tools/qa/takecheck-check.ts     the thing is never taken (the pull's own function is not called): must fail
// The rule is settled on (TAKE.wired): the check asks the page's side alone and writes no switch. Until 2026-11-07 every spot was also played with the rule
// off ("a man is sent after every take, as the build was") and the plain night with it off: out.
// PASS / FAIL each, headless. What it holds:
//   - at a spot of a level with a quiet block, the thing taken and twelve seconds played (until the man goes, where one does): a man is sent to look at
//     the holder (task 'rack', seven seconds after the take) exactly where the level says the holder is WIRED, and nowhere else; `mission.sweepAt` says the same.
//     An ordinary run asks the police post's first holder and the relay station's wired rack; QA_SWEEP=all asks every spot of every level (23, four minutes)
//   - the police post's case file (the play note's own mission): nobody is sent; the relay station's middle rack: a man is
//   - the office's plain night (no quiet mission): a man is sent
//   - a man questioned afterwards says 'somebody's been sent to look' where somebody was, and only there
import { readFileSync, readdirSync } from 'node:fs';
import { resolve } from 'node:path';
import { standUp, ROOT } from './headless.ts';
import { swept } from './sweep.ts';

const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { startFromQuery, quitScenario } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { drivePulled } = await import(`${ROOT}/src/game/mission.ts`);
const { takeTold } = await import(`${ROOT}/src/game/takecheck.ts`);
const D = await import(`${ROOT}/src/game/dialogue.ts`);

const DT = 1 / 60, BREAK = process.env.TAKECHECK_BREAK === '1';
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const hushed = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };

/** the levels that carry a quiet block, by file */
function quietLevels(): string[] {
  return readdirSync(resolve(ROOT, 'levels')).filter(f => f.endsWith('.json') && !f.endsWith('-gwot.json')).filter(f => { try { return !!JSON.parse(readFileSync(resolve(ROOT, 'levels', f), 'utf8')).scenarios?.quiet; } catch { return false; } }).sort();
}
async function stand(seed: number, file: string | null) {
  Math.random = seeded(0x7a4e + seed * 7919);
  return await hushed(() => standUp({ uploadEvery: 4, ...(file ? { level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8'))) } : {}) })) as any;
}
/** the spots an ordinary run plays (tools/qa/sweep.ts) */
const asked = (file: string, k: number): boolean => swept(0, 1) === 1 || (file === 'police-post.json' && k === 0) || (file === 'quiet-relay.json' && k === 0);
/** the thing taken now (the pull's own function, as the last frame of the hands' work calls it), then `secs` played with Sam where nobody sees or hears him
 *  again: was a man sent to the holder, and when */
function takeAndWait(W: any, secs = 12): { sent: number; who: string; sweep: number; drive: boolean; free: number } {
  const g = W.game; g.godMode = true; g.playerInvisible = true;
  const free = g.guards.filter((x: any) => x.state === 'patrol' && !x.hold && !x.held && !x.script && !x.unit && x.team === 'guard' && !x.task).length;   // (mission.ts sendRackCheck's own test of a calm staff man free to go)
  g.mission.stage = 'drive'; const t0 = g.time; if (!BREAK) drivePulled(g); const sweep = g.mission.sweepAt > 0 ? g.mission.sweepAt - t0 : -1;
  let sent = -1, who = '';
  for (let i = 0; i < Math.round(secs / DT); i++) { W.step(DT); const m = g.guards.find((x: any) => x.task?.kind === 'rack'); if (m) { sent = g.time - t0; who = m.callsign; break; } }
  const staff = g.guards.find((x: any) => !x.unit && x.state !== 'dead' && x.team === 'guard');
  const drive = !!staff && !!D.gather(g, staff).intel.find((it: any) => it.kind === 'drive')?.live;
  return { sent, who, sweep, drive, free };
}

export async function takeCheck(): Promise<boolean> {
  console.log('\n=== takecheck: a man is sent to look at an emptied holder only where something would tell them it is empty ===');
  let all = true; const verdict = (ok: boolean, label: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
  try {
    // (1) every quiet level, every spot
    const rows: string[] = []; let okOn = true, okWords = true, spots = 0, wiredN = 0; let post = '', relay = '';
    for (const file of quietLevels().filter(f => swept(0, 1) === 1 || f === 'police-post.json' || f === 'quiet-relay.json')) {
      const W = await stand(3, file), g = W.game, Q = g.level.scenarios.quiet; const squad = /lights-out|train-silent/.test(file) ? '&squad=1' : '';
      for (let k = 0; k < Q.spots.length; k++) {
        if (!asked(file, k)) continue;
        const P = Q.spots[k]; let on: ReturnType<typeof takeAndWait> | null = null;
        { let found = false; for (let pin = 0; pin < 400 && !found; pin++) { await hushed(() => startFromQuery(g, `?quiet=${pin}${squad}`)); if (g.quiet.active && g.quiet.deal.spot === k) found = true; else await hushed(() => quitScenario(g)); }
          if (!found) { rows.push(`${file} '${P.holder}': no deal reaches this spot`); okOn = false; continue; }
          for (let i = 0; i < 12; i++) W.step(DT);
          on = takeAndWait(W); await hushed(() => quitScenario(g)); }
        if (!on) continue; spots++; if (P.wired) wiredN++;
        const wantOn = !!P.wired;
        const onOk = wantOn ? on.sent >= 6.5 && on.sent <= 8 && on.sweep > 0 : on.sent < 0 && on.sweep < 0;
        const wordsOk = wantOn ? on.drive : !on.drive;
        okOn &&= onOk; okWords &&= wordsOk;
        const row = `${file} · ${P.holder}${P.wired ? ' (wired)' : ''}: ${on.sent < 0 ? 'nobody sent' : `${on.who} sent at ${on.sent.toFixed(1)} s`}${on.drive ? ' · questioned: \'somebody\'s been sent\'' : ''}${onOk && wordsOk ? '' : '   ← WRONG'}`;
        rows.push(row); if (file === 'police-post.json') post += (post ? ' · ' : '') + `${P.holder}: ${on.sent < 0 ? 'nobody' : on.who}`; if (file === 'quiet-relay.json') relay += (relay ? ' · ' : '') + `${P.holder}: ${on.sent < 0 ? 'nobody' : on.who}`;
      }
    }
    verdict(okOn && spots > 0, 'a man is sent to the emptied holder exactly where the level says it is wired, seven seconds after the take, and nowhere else', `${spots} spots asked, on ${quietLevels().length} levels with a quiet block, ${wiredN} wired\n    ${rows.join('\n    ')}`);
    verdict(okWords, 'a man questioned afterwards says \'somebody\'s been sent to look\' where somebody was, and only there', `${spots} spots`);
    verdict(!!post && !/: (?!nobody)/.test(post) && wiredN >= 1 && /the middle rack: (?!nobody)/.test(relay), 'the police post\'s holders send nobody; the relay station\'s middle rack, on the station\'s network, does', `police post — ${post || 'not played'}\n    relay station — ${relay || 'not played'}`);

    // (2) the office's plain night: no quiet mission, the rack drops off the network
    { const W = await stand(5, null), g = W.game;
      for (let i = 0; i < 30; i++) W.step(DT);
      const told = takeTold(g), r = takeAndWait(W);
      verdict(told && r.sent >= 6.5 && r.sent <= 8, 'the office\'s server rack on the plain night sends its man', r.sent < 0 ? 'nobody sent' : `${r.who} sent at ${r.sent.toFixed(1)} s`); }
  } catch (e) { verdict(false, 'the check threw', e instanceof Error ? e.stack ?? e.message : String(e)); }
  console.log(`=== takecheck: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}

if (import.meta.main) process.exit(await takeCheck() ? 0 : 1);
