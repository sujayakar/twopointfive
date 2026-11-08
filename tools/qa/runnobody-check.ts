// A MAN WHO HAS RUN IS NOBODY'S ENEMY ON THE BOOKS (src/game/runner.ts RUNNER.nobody; runnerNobody from the drop, runnerMute from the shout). His own script runs
// and nothing else of his does, but every module that walks the roster from outside still met a calm living man in state 'patrol'. The readers mended under the key:
// fogofwar.ts tellCalls (a teammate's 'contact' on him), doortools.ts ('room clear' held back), morale.ts hasMate (a mate beside him was not alone),
// suppression.ts (his bark under fire, and a mate's 'got us pinned' with him as the other man). NO LINE HERE for two
// of the clauses: darkdoor.ts mateBy (the same clause as hasMate, asked only inside the dark room's own move) and fogofwar.ts's 'heard' call (five seconds of his
// run past a teammate with his back turned put no footfall of his in that teammate's ear in this staging, key on or off).
//   bun run tools/qa/runnobody-check.ts        (or: bun run tools/qa/probes.ts runnobody)        exit 0 = every line PASS, 1 = a FAIL line
// Headless, the block with its room clear staged, every other man parked blind and deaf out of the way; one rifleman is broken through the runner's own seam
// and kept where he stands (his legs run on the spot: the lines are about what the others make of him). The key is settled (RUNNER.nobody), and so are the
// rules round it: no line asks any of them off and nothing here writes them. (Until 2026-11-07 every line had a half with the key off, the staging held
// 'runs no matter what' and a shot's word off, and two more lines asked a recording from before the key and the page's ?runnobody=0. And a line `room clear`
// asked the lead to call a room clear with the broken man gone low in it: it was staged with 'runs no matter what' held off, and on the page such a man
// runs out of the room, never low in its corner: a build from before that rule, out by ruling.)
//   contact    a teammate with his eyes on him: no 'contact' called on him
//   company    a mate three metres from him with a line is alone (morale.ts hasMate)
//   bark       rounds past his head: he says nothing
//   pinned     the same rounds past him and a mate beside him: the mate is pinned and does not shout 'got us pinned' (he would be the other of the 'us')
//   (contact and company each carry a man who has NOT broken: the old answer)
// RUNNOBODY_BREAK=1: the man is never broken (the runner's seam is not called), so every line that wants the others to make nothing of him — red.
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
type Vec3 = [number, number, number];
const BREAK = process.env.RUNNOBODY_BREAK === '1';

export async function runnobodyCheck(): Promise<boolean> {
  const { standUp, ROOT } = await import('./headless.ts');
  const { runnerBreak, runnerNobody, runnerMute } = await import(`${ROOT}/src/game/runner.ts`);
  const { hasMate } = await import(`${ROOT}/src/game/morale.ts`);
  const { noteRound } = await import(`${ROOT}/src/game/suppression.ts`);
  const { FOG } = await import(`${ROOT}/src/game/fogofwar.ts`);
  const { PICTURE } = await import(`${ROOT}/src/game/picture.ts`);
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
  const DT = 1 / 60;
  let failed = false; const line = (ok: boolean, s: string) => { if (!ok) failed = true; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${s}`); };
  const quiet = async <R>(fn: () => Promise<R> | R): Promise<R> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => {}; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
  const tell0 = PICTURE.tellSecs, cone0 = FOG.cone, own0 = FOG.own;
  const HIS: Vec3 = [32.0, 0, 4.9], MATE: Vec3 = [29.6, 0, 6.2], FAR: Vec3 = [8.6, 0, 13.9];
  const level = () => loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', 'block.json'), 'utf8')));
  /** the block; one rifleman (`A`) at his place, a second defender (`B`) and a teammate (`op`) to stand by him, everybody else parked.
   *  `broken`: A is broken now and a second run on (the gun is down, he is the runner's); he is held on his spot from then on */
  const world = async (broken = true) => {
    const W: any = await quiet(async () => { const X: any = await standUp({ seed: 3, level: level() }); startFromQuery(X.game, 'levelFile=block.json&roomclear&squad=4'); return X; });
    const g = W.game; PICTURE.tellSecs = 1e9; FOG.cone = false; FOG.own = true;   // AFTER standUp: the stand-up line would otherwise write them back
    g.godMode = true;
    const hs = g.guards.filter((x: any) => x.team === 'hostile' && x.state !== 'dead'), A = hs.find((x: any) => x.weapon.def.drop === 'rifle') ?? hs[0], B = hs.find((x: any) => x !== A), op = g.guards.find((x: any) => x.team === 'operative');
    const park = (o: any, p?: Vec3) => { o.hold = true; o.pinned = true; o.dazzledUntil = 1e9; o.heardUpTo = 1e12; if (p) { o.char.pos = [...p]; o.char.vel = [0, 0, 0]; o.path = []; o.pathGoal = null; } };
    for (const o of g.guards) if (o !== A) park(o, o.team === 'hostile' ? [5 + hs.indexOf(o), 0, 30] : undefined);
    const keeps: (() => void)[] = []; const stand = (o: any, p: Vec3, face?: Vec3) => keeps.push(() => { o.char.pos = [...p]; o.char.vel = [0, 0, 0]; o.path = []; o.pathGoal = null; if (face) o.char.bodyYaw = o.char.aimYaw = Math.atan2(face[0] - p[0], face[2] - p[2]); });
    const step = (n = 1, each?: () => void) => { for (let i = 0; i < n; i++) { g.player.visibility = 0.02; for (const k of keeps) k(); W.step(DT); each?.(); } };
    A.char.pos = [...HIS]; A.char.vel = [0, 0, 0]; A.path = []; A.pathGoal = null; A.script = null; A.dazzledUntil = 1e9; A.heardUpTo = 1e12; g.teleportPlayer(FAR); step(5);
    let broke = true; if (broken && !BREAK) { broke = runnerBreak(g, A); step(60); } stand(A, HIS);
    return { W, g, A, B, op, hs, step, stand, park, broke, fled: A.fled as boolean };
  };
  try {
    // ------------------------------------------------------------ contact
    { const calls = async (broken = true) => { const s = await world(broken), { g, A, op } = s; const at = MATE;
        op.dazzledUntil = 0; s.stand(op, at, HIS); g.fogOfWar.callLog.length = 0; s.step(180);
        return { fled: s.fled, n: g.fogOfWar.callLog.filter((c: any) => c.of === A.callsign && c.kind === 'contact').length as number, alive: A.state !== 'dead' }; };
      const b = await calls(), c = await calls(false);
      line(b.fled && b.n === 0 && c.n >= 1,
        `a teammate three metres off with a line to a man who has dropped his gun, three seconds: he says nothing (${b.n}) · on a man who has NOT broken he calls contact: ${c.n} (he lives through it: ${b.alive})`); }
    // ------------------------------------------------------------ company
    { const company = async (broken = true) => { const s = await world(broken), { g, A, B } = s; s.stand(B, MATE, HIS); s.step(20);
        return { mate: hasMate(g, B) as boolean, nobody: runnerNobody(A) as boolean }; };
      const b = await company(), c = await company(false);
      line(!b.mate && b.nobody && c.mate && !c.nobody,
        `a mate three metres from him with a line is alone (${!b.mate}) · beside a man who has NOT broken he has his mate (${c.mate})`); }
    // ------------------------------------------------------------ bark
    { const bark = async () => { const s = await world(), { g, A } = s; const t0 = A.sup.barkT, c = A.char;
        for (let i = 0; i < 12; i++) { const C: Vec3 = c.bones.chest ?? [c.pos[0], c.pos[1] + 1.1, c.pos[2]]; noteRound(g, g.player.char, [C[0] + 0.3, C[1], C[2] + 8], [C[0] + 0.3, C[1], C[2] - 6], null); s.step(6); }
        s.step(120); return { barked: A.sup.barkT !== t0 || !!A.sup.pending, v: A.sup.v as number, mute: runnerMute(A) as boolean }; };
      const b = await bark();
      line(!b.barked && b.mute, `twelve rounds a hand's breadth past his chest: he says nothing (${!b.barked})`); }
    // ------------------------------------------------------------ pinned (the mate's line, with him as the other man of the 'us')
    { const pinned = async () => { const s = await world(), { g, A, B } = s; s.stand(B, MATE, HIS); B.hold = false; B.pinned = false; B.script = null; let us = false; const all = new Set<string>();
        const past = (c: any) => { const C: Vec3 = c.bones.chest ?? [c.pos[0], c.pos[1] + 1.1, c.pos[2]]; noteRound(g, g.player.char, [C[0] + 0.3, C[1], C[2] + 8], [C[0] + 0.3, C[1], C[2] - 6], null); };
        for (let i = 0; i < 30 && !us; i++) { past(A.char); past(B.char); s.step(4, () => { const b = B.bubble; if (b) { all.add(b.text); if (/got us pinned/.test(b.text)) us = true; } }); }
        return { us, bPinned: g.time - B.sup.pinnedAt < 60, aPinned: g.time - A.sup.pinnedAt < 60, all: [...all].join(' | ') }; };
      const b = await pinned();
      line(!b.us && b.bPinned, `rounds past him and past a mate three metres off: the mate is pinned (${b.bPinned}) and says his own line, not that they've got US pinned ("${b.all}")`); }
  } catch (err) { line(false, `threw: ${(err as Error)?.stack ?? err}`); }
  finally { PICTURE.tellSecs = tell0; FOG.cone = cone0; FOG.own = own0; }
  console.log(failed ? '=== runnobody-check: FAIL ===' : '=== runnobody-check: ALL PASS ===');
  return !failed;
}
if (import.meta.main) process.exit((await runnobodyCheck()) ? 0 : 1);
