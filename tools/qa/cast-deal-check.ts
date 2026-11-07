// A TEAMMATE'S FACE IS HIS PLACE ON THE ROSTER, NOT HIS ID: for every kind of mission with teammates, the cast variant the staged game hands a teammate
// (game/cast.ts castVariantOf) is the one for his place (operatorAt), Sam is the first operator, and no two of the team wear one variant. A rescue stands its
// captors up before the team, which used to move every teammate's id, and his face with it.
//   bun run tools/qa/cast-deal-check.ts        exit 0 = all pass, 1 = a FAIL
//   ROSTER_BREAK=1 …                           = the mission's man taken by the old deal (his id): must fail
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');
const { standUp } = await import(`${ROOT}/tools/qa/headless.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { castVariantOf, operatorAt } = await import(`${ROOT}/src/game/cast.ts`);
const { castSide } = await import(`${ROOT}/src/anim/gltf.ts`);
const { rosterOf } = await import(`${ROOT}/src/game/loadout.ts`);
const { knobsCopy, knobsBack } = await import(`${ROOT}/src/game/knobs.ts`);

const brk = process.env.ROSTER_BREAK === '1';
let failed = false, checks = 0;
const line = (ok: boolean, s: string) => { checks++; if (!ok) failed = true; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${s}`); };
const quiet = async <T>(f: () => Promise<T> | T): Promise<T> => { const l = console.log, w = console.warn, e = console.error; console.log = console.warn = console.error = () => { }; try { return await f(); } finally { console.log = l; console.warn = w; console.error = e; } };

// [what it is, the level file (null = the office), the mission's query, teammates]
const RUNS: [string, string | null, string, number][] = [
  ['room clear, the four', null, 'roomclear&squad4', 4],
  ['room clear, the pair', null, 'roomclear', 2],
  ['hostage rescue (office), the pair', null, 'rescue', 2],
  ['hostage rescue (office), four', null, 'rescue&squad=4', 4],
  ['the registry (rescue), four', 'registry.json', 'rescue&squad=4', 4],
  ['the consulate (rescue), four', 'consulate.json', 'rescue&squad=4', 4],
  ['quiet, as two', 'lights-out.json', 'quiet&squad=1', 1],
];
const knobs0 = knobsCopy();
for (const [what, file, query, n] of RUNS) {
  const level = file ? loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8'))) : undefined;
  const W: any = await quiet(() => standUp({ uploadEvery: 4, cast: true, level, seed: 3 }));
  const g = W.game, res = await quiet(() => startFromQuery(g, query));
  await quiet(() => { for (let i = 0; i < 20; i++) W.step(1 / 60); });
  const team = g.guards.filter((x: any) => x.unit && x.team === 'operative'), cast = g.ch.cast;
  if (res.why || !cast || team.length !== n) { line(false, `${what}: staged with ${team.length} teammates of ${n}${res.why ? ` (${res.why})` : ''}${cast ? '' : ' — no cast loaded'}`); knobsBack(knobs0); continue; }
  const ops = castSide(cast, 'operator'), roster = rosterOf(n), off: string[] = [];
  team.forEach((gd: any, i: number) => { const game = (brk ? ops[1 + gd.char.id % (ops.length - 1)] : castVariantOf(g, gd.char))?.name, place = operatorAt(cast, i)?.name;
    if (game !== place) off.push(`${gd.callsign} (id ${gd.char.id}) is ${game}, his place deals ${place}`);
    if (roster[i]?.name !== gd.callsign) off.push(`place ${i + 1} on the roster is ${roster[i]?.name}, ${gd.callsign} in the mission`); });
  if (castVariantOf(g, g.player.char) !== ops[0]) off.push('Sam is not the first operator');
  const names = new Set([castVariantOf(g, g.player.char)?.name, ...team.map((gd: any) => (brk ? ops[1 + gd.char.id % (ops.length - 1)] : castVariantOf(g, gd.char))?.name)]);
  if (names.size !== Math.min(n + 1, ops.length)) off.push(`two of the team wear one variant (${[...names].join(' ')})`);
  line(off.length === 0, `${what}: each of ${n} wears his place's variant (ids ${team.map((gd: any) => gd.char.id).join(' ')})${off.length ? '\n    ' + off.join('\n    ') : ''}`);
  knobsBack(knobs0);
}
console.log(failed ? `=== cast-deal-check: FAIL ===` : `=== cast-deal-check: ALL PASS (${checks}) ===`);
process.exit(failed ? 1 : 0);
