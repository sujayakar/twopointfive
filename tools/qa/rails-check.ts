// A TUTORIAL MISSION'S FRAME — the check behind src/game/rails.ts.
//   bun run tools/qa/rails-check.ts [--verbose]      (also `probes.ts rails`, family missions)
// Headless, on levels/tut-sky.json (the first tutorial mission). Nothing here looks at a picture. It asks the frame's core and no more (under a minute):
//   1. the level's lesson block is whole (lesson-check's own faults, the rails keys among them);
//   2. the first step: the player is parked out of the picture and out of every sense, the view rides the squad, the program is running and has had the step's cue
//      (its men are on the road); the command key is LOCKED (no order, the press counted as dropped) and so are the zoom and the walk keys; only the turn of the view is open;
//   3. ONE ACTION A STEP: the turn done, the zoom opens and the walk keys do not; then the walk keys shift the view and nobody's feet; then the command key
//      orders the squad, and the order reaches the program (the squad's smoke goes out);
//   4. A STEP IS DONE BY ITS DEED: nothing done for ten seconds on the first step, it is still up, and the squad has not left its trucks;
//   5. NOBODY IS LOST: a teammate who would go down is on a knee and up again after the level's seconds; a man of the other side goes down as ever;
//   6. the room fight goes on beside the program, and the scenario does not end as taken while a step is up.
import { readFileSync } from 'node:fs';
import { ROOT, standUp } from './headless.ts';
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { RAILS, RAILS_STATS } = await import(`${ROOT}/src/game/rails.ts`);
const { checkLesson } = await import(`${ROOT}/src/game/lesson.ts`);
const { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
const VERBOSE = process.argv.includes('--verbose'), LEVEL = 'tut-sky', DT = 1 / 60;
const YARD = [11.4, 0.02, 16.4];
type V = number[];
const flat = (a: V, b: V) => Math.hypot(a[0] - b[0], a[2] - b[2]);

async function rig(on: boolean, seed = 1) {
  const log = console.log, warn = console.warn, err = console.error; if (!VERBOSE) console.log = console.warn = console.error = () => { };
  let W: any, started: any;
  try { W = await standUp({ uploadEvery: 4, level: loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/${LEVEL}.json`, 'utf8'))), seed }); RAILS.on = on; started = startFromQuery(W.game, `?levelFile=${LEVEL}.json&roomclear=1&squad4`); }
  finally { console.log = log; console.warn = warn; console.error = err; }
  const g = W.game, I = W.input;
  const quiet = <T,>(f: () => T): T => { if (VERBOSE) return f(); const l = console.log, w = console.warn; console.log = console.warn = () => { }; try { return f(); } finally { console.log = l; console.warn = w; } };
  const run = (secs: number, keys: string[] = []) => quiet(() => { for (let i = 0; i < Math.round(secs * 60); i++) { for (const k of keys) I.keys.add(k); W.step(DT); } for (const k of keys) I.keys.delete(k); });
  const press = (k: string) => quiet(() => { I.pressed.add(k); I.keys.add(k); W.step(DT); I.keys.delete(k); W.step(DT); W.step(DT); });
  const command = (p: V) => { W.cursorAt(p); run(2 / 60); W.cursorAt(p); press('KeyB'); };
  /** the host's own turn and zoom of the view, as host/loop.ts does them: only when the frame has them open */
  const turn = (rad: number) => { for (let i = 0; i < 40; i++) { if (!g.rails.locks('camera')) W.cam.rotate(rad / 40); run(1 / 60); } };
  const zoom = () => { for (let i = 0; i < 20; i++) { if (!g.rails.locks('zoom')) W.cam.zoom(60); run(1 / 60); } };
  const squad = () => g.guards.filter((x: any) => x.unit && x.team === 'operative' && x.state !== 'dead');
  return { W, g, I, run, press, command, turn, zoom, squad, why: started?.why as string | undefined };
}

export async function railsCheck(): Promise<boolean> {
  let all = true; const keep = { on: RAILS.on, rnd: Math.random };
  const verdict = (ok: boolean, label: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
  try {
    // ---- 1: the block
    { const R = await rig(true), g = R.g, L = g.level.scenarios?.lesson, faults = checkLesson(g);
      verdict(!R.why && !!L?.rails && faults.length === 0, 'the level starts and its lesson block is whole', `${R.why ?? 'started'} · ${L?.steps.length ?? 0} steps · ${faults.length ? faults.join(' · ') : 'no faults'}`);
    }
    // ---- 2, 3
    { const R = await rig(true), g = R.g, d0 = RAILS_STATS.dropped; R.run(0.5);
      const park = g.level.scenarios.lesson.rails.sky.park, mid = R.squad().reduce((c: V, m: any) => [c[0] + m.char.pos[0] / 4, 0, c[2] + m.char.pos[2] / 4], [0, 0, 0]);
      const look = g.followPos(), n0 = g.orders.log.length, crew = g.guards.filter((x: any) => /^Tango (8|9|10)$/.test(x.callsign)).length;
      const p0 = [...g.player.char.pos]; R.command(YARD); R.run(0.3, ['KeyW']);
      const A = g.rails.allowed();
      verdict(g.rails.live && g.rails.skyOn && g.playerInvisible && flat(p0, park) < 0.6 && flat(look, mid) < 1.5 && !!g.director.run && !g.director.run.ended && crew === 3
        && g.orders.log.length === n0 && RAILS_STATS.dropped > d0 && g.rails.locks('zoom') && !g.rails.locks('camera') && g.rails.panned() === 0 && flat(g.player.char.pos, p0) < 0.05 && A.size === 1,
        'switch on, the first step: the player parked and unseen, the view on the squad, the program running with its men on the road; the command key, the zoom and the walk keys locked, the turn open',
        `live ${g.rails.live} · parked ${flat(p0, park).toFixed(2)} m off its place, unseen ${g.playerInvisible} · the view ${flat(look, mid).toFixed(2)} m off the squad's middle · the road's men ${crew} · orders ${n0} → ${g.orders.log.length} · presses dropped ${RAILS_STATS.dropped - d0} · open: ${[...A].join(' ')} · the view shifted ${g.rails.panned().toFixed(2)} m`);
      const at0 = R.squad().map((m: any) => [...m.char.pos]); R.run(10); const still = g.lesson.i, drift = Math.max(...R.squad().map((m: any, i: number) => flat(m.char.pos, at0[i])));
      verdict(still === 0 && drift < 1.5 && R.squad().length === 4, 'a step is done by its deed: ten seconds of nothing and the first step is still up — and the squad is still behind its trucks (nobody falls in on the parked player)', `step ${still + 1} after ten seconds · the man who moved most: ${drift.toFixed(2)} m`);
      R.turn(1.0); R.run(0.3); const s1 = g.lesson.i, z1 = !g.rails.locks('zoom'), pan1 = g.rails.locks('pan'); R.run(0.4, ['KeyD']); const shift0 = g.rails.panned();
      R.zoom(); R.run(1.0); const s2 = g.lesson.i; const feet = [...g.player.char.pos]; R.run(2.5, ['KeyD']); const shift1 = g.rails.panned(), s3 = g.lesson.i, moved = flat(g.player.char.pos, feet);
      const n1 = g.orders.log.length, sm0 = g.items?.list?.length ?? 0; R.command(YARD); R.run(1.5);
      const rows = g.director.rowsOf().map((r: any) => r.what).join('|'), smoke = /charge|Smoke out/.test(rows) || rows.includes('signal:order:move');
      verdict(s1 === 1 && z1 && pan1 && shift0 === 0 && s2 === 2 && shift1 > 5 && moved < 0.05 && s3 === 3 && g.orders.log.length === n1 + 1 && smoke,
        'one action a step: the turn opens the zoom (the walk keys stay shut), the zoom opens the walk keys (they shift the view, never his feet), then the command key orders the squad and the program hears it',
        `after the turn: step ${s1 + 1}, zoom ${z1 ? 'open' : 'locked'}, walk keys ${pan1 ? 'locked' : 'open'} (shift ${shift0.toFixed(2)} m) · after the zoom: step ${s2 + 1} · the walk keys: view ${shift1.toFixed(1)} m, feet ${moved.toFixed(2)} m, step ${s3 + 1} · orders ${n1} → ${g.orders.log.length} · the program ${smoke ? 'heard the order' : 'heard nothing'}`);
      void sm0; }
    // ---- 6, 8
    { const R = await rig(true), g = R.g; R.run(1.0);
      const mate = R.squad()[0], foe = g.guards.find((x: any) => x.team === 'hostile' && x.state !== 'dead'), secs = g.level.scenarios.lesson.rails.respawn, t0 = g.time;
      const { ROOMFIGHT } = await import(`${ROOT}/src/game/roomfight.ts`); const fightOn = ROOMFIGHT.on ? !!g.fight?.on : true;
      killGuard(g, mate, [1, 0, 0], false, 'shot', 'guard'); const knee = mate.state !== 'dead' && g.rails.isSpared(mate) && mate.staggerUntil > g.time;
      killGuard(g, foe, [1, 0, 0], false, 'shot', 'player'); const down = foe.state === 'dead';
      R.run(secs + 0.5); const up = mate.state !== 'dead' && !g.rails.isSpared(mate);
      const end0 = g.roomClear.end; for (const x of g.guards) if (x.team === 'hostile' && x.state !== 'dead') killGuard(g, x, [1, 0, 0], false, 'shot', 'player'); R.run(0.5);
      verdict(knee && down && up, 'nobody is lost: a teammate who would go down is on a knee and up again; a man of the other side goes down as ever', `the teammate: ${knee ? 'on a knee' : mate.state} then ${up ? `up after ${(g.time - t0).toFixed(1)} s` : 'not up'} (the level's ${secs} s) · the other man: ${foe.state}`);
      verdict(fightOn && !end0 && !g.roomClear.end && g.lesson.i < g.level.scenarios.lesson.steps.length, 'the room fight runs beside the program, and the scenario does not end as taken while a step is up',
        `the room fight ${fightOn ? 'on' : 'OFF'} · every man of the other side put down on step ${g.lesson.i + 1}: the end is ${g.roomClear.end ?? 'not called'}`); }
  } finally { RAILS.on = keep.on; Math.random = keep.rnd; }
  console.log(`=== rails: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}

if (import.meta.main) process.exit((await railsCheck()) ? 0 : 1);
