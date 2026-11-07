// A MAN WHO HAS DROPPED HIS GUN RUNS FOR IT NO MATTER WHAT (src/game/runner.ts RUNNER.anyway; from a recorded game on the block: a man who dropped it in the
// east room took the corner for good with the fire exit two shut doors away — a sound he had placed in the only doorway and a mate's word about a man
// since shot walled the room off like men in his eyes. The maintainer: 'i think in this case he could have pathed to the exit?' — 'if a guy gives up he
// should drop his gun (including pistol) and just run for it no matter what').
//   bun run tools/qa/runanyway-check.ts        (or: bun run tools/qa/probes.ts runanyway)        exit 0 = every line PASS, 1 = a FAIL line
// Headless, the block with its room clear staged, every other man parked blind and deaf out of the way. The one man (a rifleman where the floor has one)
// is put where the recorded man stood (32.0, 4.9) and his picture is given what the recorded man's held: an enemy believed IN the room's only doorway
// (33.3, 12.0) and one believed inside the room (29.8, 8.3), neither true; nobody tells anybody anything else (PICTURE.tellSecs, PICTURE.shot: a parked
// man's blind burst is no word to him). The key is settled (RUNNER.anyway), and so are the nerve rule, the runner and the pictures (the run is planned on
// his picture, a shot is a word to him): no line asks any of them off and nothing here writes them. (Until 2026-11-07 every line had a half with the key
// off; two lines asked the run planned on the true places, PICTURE.run off; the staging held a shot's word off; and two more lines asked a recording from
// before the key and the page's ?runanyway=0.)
//   on         he runs for the outside — by the room's door and the fire exit — and is gone
//   free       an idea that is NOT on his way out: never the second look
//   settles    gone low with every try spent and the way open: he is up and gone
//   both guns  a rifleman: his rifle AND the pistol off his thigh lie on the floor and his thigh is drawn empty
//   checkpoint a run for it rides the runner's checkpoint slice onto a fresh roster as one
// RUNANYWAY_BREAK=1: his picture is given nothing (no enemy believed in his way), so he never takes the second look — red.
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
type Vec3 = [number, number, number];
const BREAK = process.env.RUNANYWAY_BREAK === '1';

export async function runanywayCheck(): Promise<boolean> {
  const { standUp, ROOT } = await import('./headless.ts');
  const { RUNNER_STATS, runnerBreak, runnerOf, runnerBare, runnerSnapshot, runnerRestore } = await import(`${ROOT}/src/game/runner.ts`);
  const { PICTURE, pictureTell } = await import(`${ROOT}/src/game/picture.ts`);
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
  const DT = 1 / 60, f1 = (x: number) => Number.isFinite(x) ? x.toFixed(1) : '—', at = (p: number[] | null | undefined) => p ? `(${f1(p[0])}, ${f1(p[2])})` : '—';
  let failed = false; const line = (ok: boolean, s: string) => { if (!ok) failed = true; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${s}`); };
  const quiet = async <R>(fn: () => Promise<R> | R): Promise<R> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => {}; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
  const tell0 = PICTURE.tellSecs, stats0 = { ...RUNNER_STATS };
  const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
  const HIS: Vec3 = [32.0, 0, 4.9], DOOR: Vec3 = [33.3, 0, 12.0], INSIDE: Vec3 = [29.8, 0, 8.3], FAR: Vec3 = [8.6, 0, 13.9];
  const level = () => loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', 'block.json'), 'utf8')));
  /** the block, the one man where the recorded man stood with nothing in his head; everybody else parked */
  const world = async () => {
    const W: any = await quiet(async () => { const X: any = await standUp({ seed: 3, level: level() }); startFromQuery(X.game, 'levelFile=block.json&roomclear&squad=4'); return X; });
    const g = W.game; PICTURE.tellSecs = 1e9;   // AFTER standUp: the stand-up line would otherwise write it back
    for (const k of Object.keys(RUNNER_STATS)) (RUNNER_STATS as any)[k] = 0;
    g.godMode = true; const vis = 0.02;
    const hs = g.guards.filter((x: any) => x.team === 'hostile' && x.state !== 'dead'), A = hs.find((x: any) => x.weapon.def.drop === 'rifle') ?? hs[0], op = g.guards.find((x: any) => x.team === 'operative');
    for (const o of g.guards) { if (o === A) continue; o.hold = true; o.pinned = true; o.dazzledUntil = 1e9; o.heardUpTo = 1e12; if (o.team === 'hostile') { o.char.pos = [5 + hs.indexOf(o), 0, 30]; o.char.vel = [0, 0, 0]; o.path = []; o.pathGoal = null; } }
    const step = (n = 1, each?: () => void) => { for (let i = 0; i < n; i++) { g.player.visibility = vis; W.step(DT); each?.(); } };
    A.char.pos = [...HIS]; A.char.vel = [0, 0, 0]; A.path = []; A.pathGoal = null; A.script = null; A.dazzledUntil = 1e9; A.heardUpTo = 1e12; g.teleportPlayer(FAR); step(5);
    /** he breaks now; `secs` on: what the runner made of him */
    const run = (secs = 25) => {
      const rifle = A.weapon.def.drop === 'rifle', broke = runnerBreak(g, A); let bold = false, first: Vec3 | null = null, low = false, goneAt = NaN, lowAt: Vec3 | null = null; const t0 = g.time;
      step(secs * 60, () => { const R: any = runnerOf(A); if (!R) return; if (R.bold) bold = true; if (!first && A.fled && R.goal) first = [...R.goal] as Vec3;
        if (R.phase === 'low') { low = true; lowAt = R.goal ? [...R.goal] as Vec3 : null; } if (isNaN(goneAt) && (R.phase === 'gone' || !g.guards.includes(A))) goneAt = g.time - t0; });
      const mine = (kind: string) => g.items.items.filter((it: any) => it.kind === kind && it.who === A.char.id).length;
      return { broke, bold, rifle, rifles: mine('rifle'), pistols: mine('pistol'), bare: runnerBare(A) as boolean, first: first as Vec3 | null, low, lowAt: lowAt as Vec3 | null, goneAt, end: [...A.char.pos] as Vec3, here: g.guards.includes(A) as boolean, phase: (runnerOf(A) as any)?.phase as string | undefined, stats: { ...RUNNER_STATS } };
    };
    return { W, g, A, op, step, run };
  };
  /** the recorded man's picture: an enemy believed in the doorway, one believed in the room */
  const guesses = (s: any) => { if (BREAK) return; pictureTell(s.g, s.A, 'player', DOOR); pictureTell(s.g, s.A, s.op, INSIDE); };

  try {
    // ------------------------------------------------------------ the recorded case
    { const s = await world(); guesses(s); const R = s.run(15);
      line(R.broke && R.bold && R.stats.bold >= 1 && !R.low && !!R.first && flat(R.first, [38.3, 0, 14.3]) < 2.5 && R.goneAt < 20 && !R.here,
        `the recorded man's picture (an enemy believed in the doorway, one believed in the room): he runs for the outside at ${at(R.first)} — by the room's door and the fire exit — and is gone ${f1(R.goneAt)} s on (the second look ${R.stats.bold}, the corner ${R.stats.low})`);
      line(R.rifle && R.rifles === 1 && R.pistols === 1 && R.bare,
        `a rifleman (${R.rifle}) who has given up: his rifle and his pistol are both on the floor (${R.rifles} and ${R.pistols}) and his thigh is drawn empty (${R.bare})`); }
    // ------------------------------------------------------------ a way the whole picture allows
    { const s = await world(); pictureTell(s.g, s.A, s.op, INSIDE); const b = s.run(15);
      line(!b.bold && b.stats.bold === 0 && !!b.first && !b.here,
        `an idea that is not on his way out: out at ${at(b.first)} and gone ${f1(b.goneAt)} s on, never the second look (${b.stats.bold})`); }
    // ------------------------------------------------------------ he never settles
    { const spent = async () => { const s = await world(), { g, A } = s; runnerBreak(g, A); s.step(60);   // (a second on: the gun is down and he is running)
        const R: any = runnerOf(A); R.phase = 'low'; R.tries = 99; R.t = 0; R.goal = [...A.char.pos]; R.script.arrived = true; R.face = A.char.bodyYaw; A.path = []; A.pathGoal = null;   // (…put low where he is, every try at the way out written off as spent)
        let goneAt = NaN; const t0 = g.time; s.step(20 * 60, () => { if (isNaN(goneAt) && ((runnerOf(A) as any)?.phase === 'gone' || !g.guards.includes(A))) goneAt = g.time - t0; });
        return { goneAt, phase: (runnerOf(A) as any)?.phase as string | undefined, here: g.guards.includes(A) as boolean }; };
      const b = await spent();
      line(!b.here && b.goneAt < 20,
        `a man gone low with every try at the way out spent, and the way open: he never settles — up at the next look and gone ${f1(b.goneAt)} s on`); }
    // ------------------------------------------------------------ the checkpoint
    { const s = await world(); guesses(s); runnerBreak(s.g, s.A); s.step(120); const R0: any = runnerOf(s.A), snap = JSON.parse(JSON.stringify(runnerSnapshot(s.g) ?? null));   // (the runner's own slice: the block's staged units keep a whole checkpoint off this floor)
      const s2 = await world(); runnerRestore(s2.g, snap); const R1: any = runnerOf(s2.g.guards[s.g.guards.indexOf(s.A)]);
      line(!!R0?.bold && R0.phase === 'out' && !!R1?.bold && R1.phase === 'out' && !!R0?.bare === !!R1?.bare && (s.A.weapon.def.drop !== 'rifle' || !!R1?.bare), `two seconds into a run for it (${R0?.phase}, no matter what: ${!!R0?.bold}, both guns down: ${!!R0?.bare}), the runner's checkpoint slice written and read onto a fresh roster: ${R1?.phase}, ${!!R1?.bold}, ${!!R1?.bare}`); }
  } catch (err) { line(false, `threw: ${(err as Error)?.stack ?? err}`); }
  finally { PICTURE.tellSecs = tell0; Object.assign(RUNNER_STATS, stats0); }
  console.log(failed ? '=== runanyway-check: FAIL ===' : '=== runanyway-check: ALL PASS ===');
  return !failed;
}
if (import.meta.main) process.exit((await runanywayCheck()) ? 0 : 1);
