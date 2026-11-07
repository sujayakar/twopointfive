// A PAR ON A ROOM CLEAR — the check of the par clock as the page reads it (the level block `scenarios.par`; today levels/killhouse.json alone).
// THE CLOCK IS THE RUST GAME'S (rust/game_rules/src/par: the tick keeps it; src/game/par.ts only reads its record). Its rules are held natively, over three
// seeds each, in rust/game_start/tests/par.rs (the arithmetic, armed and waiting, an order, a stride, either player of two, the end said once, a man
// lost); this check holds what a host sees of it through the bridge: the record on the page's copy, the row (the Rust game words it; the page words nothing), the log, the file's rules, a game stood again from the copy, and that off moves nobody.
//   bun run tools/qa/par-check.ts
// What it holds (the game stood up by the harness, the kill house's room clear staged):
//   1. the level carries a par (the arithmetic is the native test's: rust/game_start/tests/par.rs)
//   2. the file: the block is read, and the validator refuses one out of order, one with an unknown key and one on a level with no room clear
//   3. the clock: armed and NOT running while Sam stands on his mark with nothing ordered; the objective row leads with the three bars; a step past
//      PAR_START_M starts it; in other games an order, and a round fired, start it with Sam still on his mark
//   4. the end: the last defender down = 'taken' = a result at the room clear's own end time, said ONCE on the HUD's log, the row worded with the medal; a
//      teammate lost before the end is counted and priced; a restart starts over; a run that is wiped has no result
//   5. the clock moves nobody: two games on one die, the same orders and keys, the level as it is and the same level WITHOUT ITS BLOCK, digest for
//      digest every half second while the run is on (at the end the game with the clock says its line, and the two logs differ by it) — and without
//      the block nothing is armed and the row is the room clear's own. (There is no switch to ask.)
//   (the clock has no slice of its own in a save any more: it is the Rust game's state, and a kept game holds it with the rest)
//   PAR_BREAK=1  every game is stood on the level without its block, where the lines above want a clock: the check must FAIL (exit 1) — the proof that it can
// exit 0 = all hold · 1 = one does not
import { resolve } from 'node:path';
import { readFileSync } from 'node:fs';

const FILE = 'killhouse.json';

export async function parCheck(): Promise<boolean> {
  const { standUp, ROOT } = await import('./headless.ts');
  const { PAR_START_M } = await import(`${ROOT}/src/game/par.ts`);
  const { ROOMCLEAR } = await import(`${ROOT}/src/game/roomclear.ts`);
  const { parseLevelData, validateLevelData } = await import(`${ROOT}/src/scene/levelData.ts`), { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { killGuard, fireWeapon } = await import(`${ROOT}/src/game/combat.ts`), { WEAPONS } = await import(`${ROOT}/src/game/weapons.ts`);
  const { digestGame, hashDigest } = await import(`${ROOT}/src/game/digest.ts`);
  const { knobsCopy, knobsBack } = await import(`${ROOT}/src/game/knobs.ts`);
  const brk = process.env.PAR_BREAK === '1', knobs0 = knobsCopy();

  let failed = false, checks = 0;
  const line = (ok: boolean, s: string) => { checks++; if (!ok) failed = true; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${s}`); };
  const quiet = async <T>(f: () => Promise<T> | T): Promise<T> => { const l = console.log, w = console.warn; console.log = console.warn = () => { }; try { return await f(); } finally { console.log = l; console.warn = w; } };
  const text = readFileSync(resolve(ROOT, 'levels', FILE), 'utf8');
  /** the level as its file has it, or the same level without its par block */
  const levelOf = (par = true) => { const d = JSON.parse(text); if (!par) delete d.scenarios.par; return loadLevel(parseLevelData(JSON.stringify(d))); };
  /** a Game on the kill house with its room clear staged (the authored placement, the structure's own start) */
  async function stage(par = true, dice: number | null = 5) {
    ROOMCLEAR.seed = 0; ROOMCLEAR.way = 0;
    const W: any = await quiet(() => standUp({ uploadEvery: 4, level: levelOf(par && !brk), seed: dice }));
    const g = W.game, rc = g.roomClear;
    const why = await quiet(() => rc.start(0)); if (why) throw new Error(`the room clear did not start on ${FILE}: ${why}`);
    const step = (n = 1) => quiet(() => { for (let i = 0; i < n; i++) W.step(1 / 60); });
    await step(3);
    const foes = () => g.guards.filter((x: any) => x.unit && x.team === 'hostile' && x.state !== 'dead');
    const drop = (list: any[], by: 'player' | 'guard' = 'player') => quiet(() => { for (const h of list) killGuard(g, h, [1, 0, 0], false, 'shot', by); });
    const order = () => quiet(() => g.orders.give('fallIn', g.orders.targetAt(g.player.char.pos), 'all'));
    const nudge = (m: number) => quiet(() => { const p = g.player.char.pos; g.teleportPlayer([p[0] + m, 0, p[2]]); });
    return { W, g, rc, par: g.par, step, foes, drop, order, nudge, row: () => g.objectiveText() as string };
  }

  try {
    // 1. the arithmetic
    const D = levelOf().scenarios!.par!;
    line(!!D && D.gold < D.silver && D.silver < D.bronze && D.lossSecs > 0, `${FILE} carries a par: gold ${D.gold} · silver ${D.silver} · bronze ${D.bronze} · a man lost ${D.lossSecs} s`);

    // 2. the file's rules
    { const bad = (edit: (d: any) => void) => { const d = JSON.parse(text); edit(d); return validateLevelData(d).filter((e: string) => /par/.test(e)); };
      line(validateLevelData(JSON.parse(text)).length === 0, 'the level file validates with its block');
      line(bad(d => { d.scenarios.par.silver = d.scenarios.par.gold; }).length > 0, 'refused: silver not past gold');
      line(bad(d => { d.scenarios.par.platinum = 30; }).length > 0, 'refused: a key the block does not have');
      line(bad(d => { delete d.scenarios.roomClear; }).length > 0, 'refused: a par on a level with no room clear'); }

    // 3. the clock starts on a step
    { const A = await stage(); await A.step(90);
      line(!!A.par.data && !A.par.started && A.par.secs === 0 && A.par.result() === null, 'staged: armed, and a second and a half on the mark with nothing ordered has not started the clock');
      line(/^par \d:\d\d \/ \d:\d\d \/ \d:\d\d · \S/.test(A.row()), `the objective row leads with the three bars before the start: '${A.row().slice(0, 44)}…'`);
      await A.nudge(PAR_START_M * 0.6); await A.step(2);
      line(!A.par.started, `a shuffle of ${(PAR_START_M * 0.6).toFixed(1)} m is not a start`);
      await A.nudge(PAR_START_M * 0.8); await A.step(1); const t0 = A.g.time; await A.step(120);
      line(A.par.started && Math.abs(A.par.secs - (A.g.time - t0)) < 0.05, `past ${PAR_START_M} m from the mark the clock runs (${A.par.secs.toFixed(2)} s after two seconds)`);
      line(/^⏱ 0:0[12] → gold \d:\d\d · \S/.test(A.row()), `the clock leads the row, then the medal still on, then the room clear's words: '${A.row().slice(0, 40)}…'`);
      // a game stood again from the page's copy (a file loaded on it, a resync) takes the clock up where it was: the whole record is on the wire
      { const was = A.par.t0, H = A.g.rustHost; if (H?.puppetTakesOver) { await quiet(() => H.puppetTakesOver('par-check: the game is stood again from the copy')); await A.step(2);
          line(A.par.started && A.par.t0 === was, `stood again from the copy mid-run, the clock runs on from the same start (${A.par.t0.toFixed(2)})`); }
        else line(false, 'this host has no way to stand the game again from the copy: the check of it did not run'); }

      // 4. the end
      const said = () => A.g.messages.filter((m: any) => /^par ▸/.test(m.text)).length;
      await A.drop(A.foes()); await A.step(4);
      const R = A.par.result();
      line(A.rc.end === 'taken' && !!R && R.lost === 0 && Math.abs(R.secs - (A.rc.endAt - t0)) < 0.05 && ['gold', 'silver', 'bronze', 'none'].includes(R.medal), `every defender down: taken, ${R?.secs.toFixed(1)} s, nobody lost, ${R?.medal}`);
      const secs = A.par.secs; await A.step(120);
      line(said() === 1 && A.par.secs === secs && A.par.result()?.secs === R?.secs, 'the result is said once on the HUD log and the clock holds at the end');
      { const H = A.g.rustHost; await quiet(() => H?.puppetTakesOver?.('par-check: the game is stood again from the copy, after the end')); await A.step(4);
        line(said() === 1 && A.par.result()?.secs === R?.secs, 'stood again from the copy after the end, the result stands and is not said a second time'); }
      line(/^structure taken · \d:\d\d · (GOLD|SILVER|BRONZE|no medal)/.test(A.row()), `the row at the end: '${A.row().slice(0, 44)}…'`);
      await quiet(() => A.g.restartEncounter()); await A.step(5);
      line(!!A.par.data && !A.par.started && A.par.result() === null && /^par /.test(A.row()), 'a restart: armed again, clock not started, no result');
      // the page's words about the best time ride the result where THIS page shows it (par.shown: the card, the debrief), never the row the game words,
      // which every machine of one game reads the same
      await A.nudge(PAR_START_M + 0.5); await A.step(30); await A.drop(A.foes()); await A.step(4);
      const R2 = A.par.result();
      A.par.best('best 0:59');
      line(!!R2 && A.par.shown === `${R2.line} · best 0:59` && !/best/.test(A.row()),
        `the page's best-time words ride what this page shows ('${A.par.shown}'), and the game's row carries none`); }

    // Sam's first round starts it, standing on his mark
    { const F = await stage(); await F.step(30); const before = F.par.started;
      await quiet(() => fireWeapon(F.g, F.g.player.char, [0, 1.2, 13.2], true, { weapon: WEAPONS.fn2000 })); await F.step(2);
      line(!before && F.par.started, 'a round fired from the mark with nothing ordered starts the clock'); }

    // an order starts it; a man lost is priced; wiped = no time
    { const B = await stage(); await B.step(30); const why = await B.order(); await B.step(2);
      line(!why && B.par.started, `an order ('fall in') with Sam still on his mark starts the clock${why ? ` — refused: ${why}` : ''}`);
      const ops = B.g.nightBreach.ops; await B.drop([ops[0]], 'guard'); await B.step(30); await B.drop(B.foes()); await B.step(4);
      const R = B.par.result();
      line(!!R && R.lost === 1 && R.of === ops.length && R.penalty === D.lossSecs && Math.abs(R.total - R.secs - D.lossSecs) < 1e-9 && /1 lost/.test(R.line), `a teammate down at the end is priced: ${R?.line}`);
      const C = await stage(); await C.order(); await C.step(10); await C.drop(C.g.nightBreach.ops, 'guard'); await quiet(() => { C.g.player.down = true; }); await C.step(30);
      line(C.rc.end !== 'taken' && C.par.result() === null, `a run that is not taken (${C.rc.end ?? 'no end'}) has no result`); }

    // 5. the clock moves nobody
    { const play = async (on: boolean) => { const S = await stage(on); const hs: string[] = [];
        await S.step(20); await S.order();
        S.W.input.keys.add('KeyD');
        for (let i = 0; i < 20; i++) { await S.step(30); hs.push(hashDigest(digestGame(S.g, true))); }
        S.W.input.keys.clear(); await S.drop(S.foes()); await S.step(30); hs.push(hashDigest(digestGame(S.g, true)));
        return { S, hs, started: S.par.started as boolean, res: S.par.result(), armed: S.par.data !== null, row: S.row(), said: S.g.messages.some((m: any) => /^par ▸/.test(m.text)) as boolean }; };
      const on = await play(true), off = await play(false);
      const at = on.hs.findIndex((h, i) => h !== off.hs[i]);
      line((at < 0 || at === 20) && on.hs.length === 21, `with the block and without it the games digest the same over ten seconds of the run (20 digests${at < 0 || at === 20 ? '' : `; first differs at ${at}`}); the end's own digest ${at === 20 ? 'differs, by the line the game with the clock says' : 'is the same too'}`);
      line(on.started && !!on.res, `with the block that run was timed and ranked (${on.res?.medal})`);
      line(!off.armed && off.res === null && !/par|⏱|GOLD|SILVER|BRONZE|medal/.test(off.row) && !off.said, `without the block nothing is armed and the row is the room clear's own: '${off.row.slice(0, 40)}…'`); }
  } catch (e) { line(false, `threw: ${(e as Error).stack ?? e}`); }
  finally { knobsBack(knobs0); }
  console.log(failed ? `par-check: FAILED (${checks} checks)` : `par-check: all ${checks} hold`);
  return !failed;
}

if (import.meta.main) { const ok = await parCheck(); process.exit(ok ? 0 : 1); }
