// A MAN A LEVEL'S SCRIPT TOLD TO HAVE A SMOKE LIGHTS ONE (src/game/smokebreak.ts SMOKEBREAK / updateSmokeBreak; the arm is character.ts smokeLayer; the
// order is director.ts OrderPatch.smoke → GuardScript.smoke; the first such man is the ledger's night man, levels/tut-quiet.script.ts).
//
// The maintainer, 10-22: "maybe we should make him stop a bit further away and then turn around for a cigarette break", "cigarette break animation (with
// real smoke!) would be sick".
//
//   bun run tools/qa/smokebreak-check.ts [--verbose]      exit 0 = all pass, 1 = a FAIL line
//   bun run tools/qa/probes.ts smokebreak
//
// The harness has no GPU and no smoke solver: what is proven HERE is what the game files (the lights) and where the animator puts the hand — never a smoke seen.
//   lights    the ledger's court man told to smoke where he stands, twenty seconds: he lights it once (one flame, at his mouth), the end glows
//             from then on and brighter at a draw, he draws and breathes out
//   hand      … and his LEFT hand goes up to his mouth and comes down again; his right, the torch's, stays where it was carried
//   who       not before he has got where he was sent; not once his script lets him go (the hand is given back at once); not a dead man
import { standUp, ROOT } from './headless.ts';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const DT = 1 / 60;

export async function smokeBreakCheck(o: { verbose?: boolean } = {}): Promise<boolean> {
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { smokeBreakStats } = await import(`${ROOT}/src/game/smokebreak.ts`);
  const { order, endGuardScript } = await import(`${ROOT}/src/game/squad.ts`);
  /** he is at it: told to, up, not in Sam's arm, and where he was sent */
  const smoking = (gd: any): boolean => { const s = gd.script; return !!s?.smoke && gd.state !== 'dead' && !gd.held && gd.char.alive && (s.goal === null || !!s.arrived); };
  const quiet = async <T>(fn: () => T | Promise<T>): Promise<T> => { if (o.verbose) return await fn(); const l = console.log, e = console.error, w = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = l; console.error = e; console.warn = w; } };
  let fails = 0, checks = 0;
  const verdict = (ok: boolean, label: string, detail: string) => { checks++; if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
  const flat = (a: ArrayLike<number>, b: ArrayLike<number>) => Math.hypot(a[0] - b[0], a[2] - b[2]);

  /** the ledger stood up, the court's man found */
  async function rig() {
    const lvl = loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', 'tut-quiet.json'), 'utf8')));
    const W: any = await quiet(() => standUp({ level: lvl, seed: 7 }));
    const g = W.game;
    g.aiEnabled = false;   // (nobody's eyes or ears are this file's question: the men stand)
    const gd = g.guards.find((x: any) => x.route?.name === 'court') ?? g.guards[1];
    const step = (n = 1, each?: () => void) => { for (let i = 0; i < n; i++) { W.step(DT); each?.(); } };
    const flashes = () => g.engine.lights.lights.filter((l: any) => l.group === 'transient' && l.owner === gd.char.id);
    return { W, g, gd, step, flashes };
  }

  try {
    // ---- lights, hand
    {
      const A = await rig(), c = A.gd.char;
      order(A.gd, { goal: null, upper: 'torch', light: true }); A.step(90);   // (the torch carried as the script will have it carried, before he is told)
      const rest = { l: c.bones.handL ? [...c.bones.handL] : null, r: c.bones.handR ? [...c.bones.handR] : null };
      order(A.gd, { smoke: true });
      let up = 9, down = 0, rMove = 0, flameAt: number[] | null = null, mouth: number[] | null = null, maxK = 0, minK = 9, asked = new Set<number>(), lit = 0;
      A.step(20 * 60, () => { const h = c.bones.handL, hd = c.bones.head; if (!h || !hd) return;
        const d = Math.hypot(h[0] - hd[0], h[1] - hd[1], h[2] - hd[2]); up = Math.min(up, d); down = Math.max(down, d);
        if (rest.r && c.bones.handR) rMove = Math.max(rMove, Math.hypot(c.bones.handR[0] - rest.r[0], c.bones.handR[1] - rest.r[1], c.bones.handR[2] - rest.r[2]));
        if (c.anim.smoke !== null) asked.add(c.anim.smoke);
        for (const l of A.flashes()) { if (l.range > 2 && !flameAt) { flameAt = [...l.pos]; mouth = [...hd]; } if (l.range <= 2) { maxK = Math.max(maxK, l.peakIntensity); minK = Math.min(minK, l.peakIntensity); lit++; } } });
      const T = smokeBreakStats(A.g);
      verdict(T.lit === 1 && T.flames === 1 && T.draws >= 1 && T.breaths >= 2 && T.embers > 20 && !!flameAt && !!mouth && Math.hypot(flameAt![0] - mouth![0], flameAt![1] - mouth![1], flameAt![2] - mouth![2]) < 0.3 && maxK > minK,
        'lights — told to smoke, he lights it once at his mouth, the end glows and is brighter at a draw, he draws and breathes out',
        `${T.lit} lit, ${T.flames} flame${flameAt && mouth ? ` ${Math.hypot(flameAt[0] - mouth[0], flameAt[1] - mouth[1], flameAt[2] - mouth[2]).toFixed(2)} m from his head joint` : ' (none seen)'}, ${T.embers} glows of the end (${minK === 9 ? '-' : minK} to ${maxK}), ${T.draws} draws, ${T.breaths} breathed out in twenty seconds`);
      verdict(up < 0.26 && down > 0.5 && rMove < 0.12 && asked.has(1) && asked.has(0),
        'hand — his left hand goes up to his mouth and comes down again, and the torch\'s hand stays where it was',
        `the left wrist from ${down.toFixed(2)} m off his head joint to ${up.toFixed(2)} m · the right moved ${rMove.toFixed(2)} m · the animator was asked ${[...asked].sort().join(' and ')}`);
      // who: let go
      endGuardScript(A.gd, 'patrol'); A.step(1); const asked1 = c.anim.smoke, on1 = smoking(A.gd); const n1 = T.embers; A.step(90);
      const hd = c.bones.head, h = c.bones.handL, back = h && hd ? Math.hypot(h[0] - hd[0], h[1] - hd[1], h[2] - hd[2]) : 0;
      // who: on his way
      const B = await rig(); B.step(10); const p = B.gd.char.pos; order(B.gd, { goal: [p[0] + 2.5, 0, p[2] - 0.4], upper: 'torch', smoke: true });
      let early = 0; B.step(45, () => { if (B.gd.char.anim.smoke !== null) early++; }); const far = flat(B.gd.char.pos, B.gd.script.goal);
      const litWalking = smokeBreakStats(B.g).lit; B.step(9 * 60); const litThere = smokeBreakStats(B.g).lit, there = !!B.gd.script.arrived;
      // who: dead
      B.gd.state = 'dead'; B.gd.char.alive = false; B.step(2); const deadAsk = B.gd.char.anim.smoke;
      verdict(asked1 === null && !on1 && T.embers === n1 && back > 0.45 && early === 0 && litWalking === 0 && (!there || litThere === 1) && deadAsk === null,
        'who — not on his way there, not once his script lets him go, not a dead man',
        `let go: the hand asked ${asked1 === null ? 'nothing' : asked1} the frame after, no glow filed since, the wrist back ${back.toFixed(2)} m off his head in a second and a half · walking (${far.toFixed(1)} m to go after 45 frames): asked ${early} frames, ${litWalking} lit; ${there ? `there, ${litThere} lit` : 'he never got there (not judged)'} · dead: asked ${deadAsk === null ? 'nothing' : deadAsk}`);
    }
  } catch (e) { fails++; throw e; }
  console.log(`\nsmokebreak-check: ${checks - fails}/${checks} pass`);
  return fails === 0;
}

if (import.meta.main) { const ok = await smokeBreakCheck({ verbose: process.argv.includes('--verbose') }); process.exit(ok ? 0 : 1); }
