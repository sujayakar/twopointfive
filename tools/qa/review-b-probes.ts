// The 'review-b' probe family (run through probes.ts: `bun run tools/qa/probes.ts review-b | checkpoint-civilians (also rb-checkpoint-civilians) | rb-refuse-over | rb-tactics-reset | rb-gate-civilians |
// rb-laser-civilian | rb-whistle-load | rb-rain-annex | rb-glass-reset | rb-nocivilians [--verbose]`): the deep review's game-systems findings
// fixed by lane game-fixes-B (docs/internal/review-ledger.md pass 3 + pass 9), each held by a directed headless repro:
//   checkpoint-civilians  (G3.1 / G3.7) the cleaner told to stay down and the late worker knocked out are SAVED: a minute of play later the file loads to exactly
//                         that — her corner and her promise, his body where it lay (asleep), the card's counts and price unchanged but for the load; a civilian
//                         mid-run to a phone loads mid-run (same target, same spot, same path) and gets there; and right after a load whose abandoned future had
//                         run the event count on, a stun charge a metre from a working civilian FRIGHTENS him (the deafness: a fresh civilian's ear marker was
//                         stamped off the future's count) — with no second construction of the civilians on the load (the double reset). Also rides 'checkpoints'.
//   rb-refuse-over        (G3.2) a civilian killed under tune.civilianKillFails fails the run with Sam standing: canSave says so and saveState throws; a hand-made
//                         file whose stage is 'failed' (or 'done') is refused by canLoad / loadState — never poured over a living man as 'exfil'.
//   rb-tactics-reset      (G3.3) a doorway marked dangerous, then restartEncounter (and resetGuards alone): no dangerous doorway survives into the fresh shift.
//   rb-gate-civilians     (G3.4) the autosave gate names a frightened / running / telling / phoning civilian and one hiding from HIM untold; a complied one and
//                         a body are no objection; the queue of words due (a guard's radio a beat after her report) holds it too.
//   rb-laser-civilian     (G3.6) the cleaner stood in the corridor beam: the beam is drawn CUT at her shins and does not trip; gone again, the beam is whole.
//   rb-whistle-load       (G3.8) a whistle in the air at the save is not in the loaded game's event queue: nobody answers it a second time.
//   rb-rain-annex         (U9.9) the rain bed's gain: full out in the lot, a third inside the building AND inside the roofed stair tower (ANNEX).
//   rb-glass-reset        (U9.10) a pane starred, then restartEncounter: the glass's hit count / log go with the crack decals.
//   rb-nocivilians        (lead) ?nocivilians on a FRESH profile (no stored record): nobody but the shift — the URL flag is read whatever storage holds.
import { standUp, ROOT } from './headless.ts';
const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
const SS = await import(`${ROOT}/src/game/savestate.ts`);
const { saveState, loadState, canSave, canLoad } = SS;
const { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
const { rollcallState, raiseEscalation } = await import(`${ROOT}/src/game/guards.ts`);
const { markDanger, dangerDoors } = await import(`${ROOT}/src/game/tactics.ts`);
const { tryWhistle, WHISTLE_STATS } = await import(`${ROOT}/src/game/whistle.ts`);
const { SCORE } = await import(`${ROOT}/src/game/chronicle.ts`);
const { debriefOf } = await import('./debrief-read.ts');
const { glassState, crackNear, glassPanes } = await import(`${ROOT}/src/game/glass.ts`);
const { ANNEX } = await import(`${ROOT}/src/game/weather.ts`);

type Vec3 = [number, number, number];
const DT = 1 / 60;
const VERBOSE = process.argv.includes('--verbose');
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const f1 = (x: number) => (Number.isFinite(x) ? x.toFixed(1) : String(x)), f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : String(x));
const posS = (p: Vec3 | null | undefined) => p ? `(${f1(p[0])}, ${f1(p[2])})` : '—';
function verdict(ok: boolean, label: string, detail: string) { console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}: ${detail}`); return ok; }
let failures = 0;
export function reviewBFailures() { return failures; }
const steps = (W: any, secs: number, until?: () => boolean) => { const n = Math.max(1, Math.round(secs / DT)); for (let i = 0; i < n; i++) { W.step(DT); if (until?.()) return true; } return !until; };
/** the floor up off `seed`: god mode, Sam parked unseen on the verge, the men deaf to the staging unless asked, the radio check far off */
async function stage(seed: number) {
  Math.random = seeded(seed);
  const W = await standUp(); const { game } = W; const pl = game.player;
  game.godMode = true; game.saves.enabled = false;   // (no autosaves under the probes' feet: each case writes / loads its own)
  steps(W, 5 * DT);
  game.teleportPlayer([2.0, 0, 2.0]); game.playerInvisible = true; pl.crouch = true; pl.visibility = 0.02;
  rollcallState(game).nextAt = 1e9;
  const [K, N, R] = game.guards; const [M, O] = game.civilians;
  return { W, game, pl, K, N, R, M, O };
}
const civLine = (cv: any) => { const C = cv.civ; return cv.state === 'dead' ? `${cv.callsign} DOWN(${cv.downKind}) @${posS(cv.char.pos)}` : `${cv.callsign} ${C.state}${C.cause ? `[${C.cause}${C.armed ? ',armed' : ''}]` : ''}${C.target ? `→${C.target.name}` : ''}${C.complied ? ' complied' : ''}${C.reported ? ' reported' : ''} @${posS(cv.char.pos)} aw ${f2(cv.awareness)} heard ${cv.heardUpTo}`; };


// ================================================================================================================================ G3.2
export async function refuseOver() {
  console.log('\n=== rb-refuse-over: a run that is over with Sam standing (a civilian killed under civilianKillFails) cannot be saved; a file of a finished run cannot be loaded ===');
  let ok = true;
  const S = await stage(7373); const { W, game, M } = S;
  steps(W, 2); for (const gd of game.guards) gd.heardUpTo = 1e12;
  const good = saveState(game, { label: 'alive', kind: 'harness' });
  game.tune.civilianKillFails = true;
  killGuard(game, M, [0, 0, 1], false, 'shot'); steps(W, 3);
  const why = canSave(game); let threw = false; try { saveState(game, { force: true }); } catch { threw = true; }
  const gate = game.saves.gate(game);
  ok = verdict(game.mission.stage === 'failed' && !game.player.down && /run is over/.test(why ?? '') && threw && !!gate, 'the cleaner shot dead under civilianKillFails: mission failed, Sam up — canSave names it, saveState throws (forced too), the autosave gate is shut', `stage ${game.mission.stage}, down ${game.player.down}; canSave '${why}'; threw ${threw}; gate '${gate}'`) && ok;
  game.tune.civilianKillFails = false;
  for (const st of ['failed', 'done']) {
    const bad = JSON.parse(JSON.stringify(good)); bad.stage = st; bad.mission.stage = st;
    const wl = canLoad(game, bad); let lthrew = false; const stageBefore = game.mission.stage;
    try { loadState(game, bad); } catch { lthrew = true; }
    ok = verdict(/finished run/.test(wl ?? '') && lthrew && game.mission.stage === stageBefore, `a hand-made file with stage '${st}': canLoad names it, loadState throws, the running game untouched (never rewritten to 'exfil')`, `canLoad '${wl}'; threw ${lthrew}; stage still ${game.mission.stage}`) && ok;
  }
  { const fine = JSON.parse(JSON.stringify(good)); const wl = canLoad(game, fine); let lthrew = false; try { loadState(game, fine); } catch (e) { lthrew = true; if (VERBOSE) console.log((e as Error).message); } ok = verdict(!wl && !lthrew && game.mission.stage === 'infiltrate' && game.civilians[0].state !== 'dead', 'the same file with its honest stage loads (and the cleaner is alive again in it)', `canLoad ${wl ?? 'ok'}, threw ${lthrew}, stage ${game.mission.stage}, ${civLine(game.civilians[0])}`) && ok; }
  console.log(`=== rb-refuse-over: ${ok ? 'ALL PASS' : 'FAILURES above'} ===`); if (!ok) failures++;
  return ok;
}

// ================================================================================================================================ G3.3
export async function tacticsResetProbe() {
  console.log('\n=== rb-tactics-reset: a doorway marked dangerous does not survive restartEncounter / resetGuards into the fresh shift ===');
  let ok = true;
  const S = await stage(8484); const { W, game } = S;
  const d = game.doors.byName('storage') ?? game.doors.list[0];
  markDanger(game, d, 'guard', 'rounds', 'Kowalski'); steps(W, 2 * DT);
  const n0 = dangerDoors(game).length;
  game.restartEncounter(); steps(W, 2 * DT);
  const n1 = dangerDoors(game).length;
  ok = verdict(n0 === 1 && n1 === 0, 'markDanger → 1 dangerous doorway; restartEncounter → none', `${n0} → ${n1}`) && ok;
  markDanger(game, d, 'guard', 'body', 'Novak'); const n2 = dangerDoors(game).length; game.resetGuards(); const n3 = dangerDoors(game).length;
  ok = verdict(n2 === 1 && n3 === 0, 'and resetGuards alone (the tour\'s stops reset the men through it) drops it too', `${n2} → ${n3}`) && ok;
  console.log(`=== rb-tactics-reset: ${ok ? 'ALL PASS' : 'FAILURES above'} ===`); if (!ok) failures++;
  return ok;
}



// ================================================================================================================================ G3.8
export async function whistleLoad() {
  console.log('\n=== rb-whistle-load: a whistle in the air at the save is not put back by the load — nobody answers it twice ===');
  let ok = true;
  const S = await stage(2828); const { W, game, pl, K } = S;
  steps(W, 1);
  // Sam in the corridor six metres west of Kowalski, unseen but audible; he whistles; the same frame is saved (forced harness snapshot: the event is in the queue)
  game.playerInvisible = false; pl.visibility = 0.02; game.teleportPlayer([K.char.pos[0] - 6, 0, K.char.pos[2]]); steps(W, 2 * DT); pl.visibility = 0.02;
  const blew = tryWhistle(game);
  const inQueue = game.events.some((e: any) => e.kind === 'whistle');
  const s = saveState(game, { kind: 'harness', force: true });
  const fileHas = s.events.some((e: any) => e.kind === 'whistle');
  steps(W, 0.3); const res0 = WHISTLE_STATS.resolved, ans0 = WHISTLE_STATS.answered;
  loadState(game, JSON.parse(JSON.stringify(s))); W.cam.target = v3.copy(game.player.char.pos);
  const afterLoad = game.events.some((e: any) => e.kind === 'whistle');
  steps(W, 1.0);
  const res1 = WHISTLE_STATS.resolved, ans1 = WHISTLE_STATS.answered;
  ok = verdict(blew && inQueue && fileHas && !afterLoad && res1 === res0 && ans1 === ans0, 'blown, in the queue, in the file — and NOT in the loaded game\'s queue: no second resolution, no second answer over the next second', `blew ${blew}, queue ${inQueue}, file ${fileHas}, after load ${afterLoad}; resolved ${res0} → ${res1}, answered ${ans0} → ${ans1}; Kowalski ${game.guards[0].state} aw ${f2(game.guards[0].awareness)}`) && ok;
  console.log(`=== rb-whistle-load: ${ok ? 'ALL PASS' : 'FAILURES above'} ===`); if (!ok) failures++;
  return ok;
}


// ================================================================================================================================ U9.10
export async function glassReset() {
  console.log('\n=== rb-glass-reset: the glass\'s hit count / log go with the crack decals on restartEncounter ===');
  let ok = true;
  const S = await stage(1515); const { W, game } = S;
  const panes = glassPanes(game); const b = game.level.boxes[panes[0]];
  const n = crackNear(game, [b.c[0], b.c[1], b.c[2]], 0.5, 0); steps(W, DT);
  const GS = glassState(game); const hits0 = GS.hits, log0 = GS.log.length, decals0 = game.decals.list.filter((d: any) => d.kind === 'glassHit').length;
  game.restartEncounter(); steps(W, DT);
  const GS2 = glassState(game);
  ok = verdict(n >= 1 && hits0 >= 1 && log0 >= 1 && decals0 >= 1 && GS2.hits === 0 && GS2.log.length === 0 && GS2.lastHitT <= -100 && game.decals.list.length === 0, 'a pane starred (hits, log, decals) → restart → hits 0, log empty, decals gone', `${n} pane(s): hits ${hits0} / log ${log0} / stars ${decals0} → hits ${GS2.hits} / log ${GS2.log.length} / decals ${game.decals.list.length}`) && ok;
  console.log(`=== rb-glass-reset: ${ok ? 'ALL PASS' : 'FAILURES above'} ===`); if (!ok) failures++;
  return ok;
}

