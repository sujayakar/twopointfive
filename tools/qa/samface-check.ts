#!/usr/bin/env bun
// SAM THROWS A BREAKER WHICHEVER WAY IT FACES (src/game/workthings.ts WORK.samFace, asked by player.ts for his own row and by lighting.ts for his OCP).
// His row at the mains panel asked for a clear line to a point a hand's width toward -z of the panel's mark: right where the panel hangs facing that way or
// along a wall that runs that way, and inside the wall where it hangs facing the other way (the two charges, the glasshouse, the mousehole, the yards and
// four of the tutorials), where he was never offered the row and only the squad's order could throw it.
//   bun run tools/qa/samface-check.ts
// Lines: front (stood in front of the two charges' panel: the row is offered, and his key throws the mains) · asitwas (a panel that faces the old way: the row is offered).
// `probes.ts samface` (in family `squadorders`) runs it. Exit 0 = every line passed, 1 = a line failed.
import { resolve } from 'node:path';
import { readFileSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);

type Vec3 = [number, number, number];
const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
let fails = 0;
export async function samfaceCheck(): Promise<boolean> {
fails = 0; const rnd = Math.random;
try {
const line = (ok: boolean, what: string) => { console.log(`  ${ok ? 'PASS' : 'FAIL'}  ${what}`); if (!ok) fails++; };

/** a room clear with the four staged on a level; Sam cannot be hurt or seen. The panel's real face is read off its box, as workthings.ts reads it. */
async function stage(file: string) {
  Math.random = seeded(0x5a3f);
  const level = loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8')));
  const W: any = await standUp({ uploadEvery: 4, level }), g = W.game; g.godMode = true; g.playerInvisible = true;
  const { why } = startFromQuery(g, new URLSearchParams(`levelFile=${file}&roomclear=1&squad4`)); if (why) throw new Error(`start refused — ${why}`);
  const run = (secs: number, keys: string[] = []) => { for (let f = 0, n = Math.round(secs / DT); f < n; f++) { if (f === 0) for (const k of keys) W.input.pressed.add(k); W.step(DT); } };
  run(DT);
  const B = g.level.breaker, box = g.engine.world.statics[B.boxes[0]], bp: Vec3 = B.pos, dx = bp[0] - box.c[0], dz = bp[2] - box.c[2];
  const n: Vec3 = Math.abs(dx) > Math.abs(dz) ? [Math.sign(dx) || 1, 0, 0] : [0, 0, Math.sign(dz) || -1];
  const at = (m: number, side = 0): Vec3 => [bp[0] + n[0] * m - n[2] * side, 0, bp[2] + n[2] * m + n[0] * side];
  const rowAt = (p: Vec3): boolean => { g.teleportPlayer(p); run(3 * DT); return g.nearby.some((r: any) => r.kind === 'breaker' && r.verb === 'breakerOff'); };
  return { W, g, run, n, at, rowAt };
}

console.log('samface-front — the two charges, stood in front of the panel');
{ const { g, run, n, at, rowAt } = await stage('defuse.json');
  line(n[2] === 1, `the panel hangs facing the other way from the office's (its face looks along ${n[0] ? 'x' : 'z'} ${n[0] + n[2] > 0 ? '+' : '-'})`);
  line(rowAt(at(0.6)), 'in front of it: his row is offered');
  line(rowAt(at(0.7, 0.4)) && rowAt(at(0.7, -0.4)), '…and from a step to either side of it');
  g.teleportPlayer(at(0.6)); run(3 * DT); const sel = g.selectVerb('breaker', 'breakerOff'); run(0.5, ['Space', 'KeyE']);
  line(sel && g.blackout.active, 'his key throws it: the mains are off');
}
console.log('samface-asitwas — a panel that faces the old way');
{ const { n, at, rowAt } = await stage('warehouse.json');
  line(n[2] === -1, 'the warehouse: the panel faces the way the office\'s does');
  line(rowAt(at(0.6)), 'in front of it the row is offered');
}
} finally { Math.random = rnd; }
console.log(fails ? `\n=== samface: ${fails} FAILED ===` : '\n=== samface: ALL PASS ===');
return fails === 0;
}
if (import.meta.main) process.exit((await samfaceCheck()) ? 0 : 1);
