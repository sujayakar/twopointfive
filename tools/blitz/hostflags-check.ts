// IS ANYTHING OF THE HOST'S HANDED TO THE RUST GAME WITH THE START? A page's start is taken inside its frame: the box world's copy may stand open, a question
// may be in flight, counts have counted. None of it is the game. The whole export is taken twice, at rest and with the host in the middle of its work
// (a window open and a copy standing, a sound asked aside, every count moved), and the bytes must be the same.
//   TPS_GAME=rust bun run tools/blitz/hostflags-check.ts [level.json?row …]     HOSTFLAGS_BREAK=1 = the game's clock is moved too: must be red
import { readFileSync } from 'node:fs';
const R = `${import.meta.dir}/../..`;
const { standUp } = await import(`${R}/tools/qa/headless.ts`), { startFromQuery } = await import(`${R}/src/game/scenarioRun.ts`);
const { loadLevel } = await import(`${R}/src/scene/levelLoad.ts`), { parseLevelData } = await import(`${R}/src/scene/levelData.ts`);
const { exportGame } = await import('./export'), { knobRegistry } = await import(`${R}/src/game/knobs.ts`), COL = await import(`${R}/src/scene/collision.ts`), MIR = await import(`${R}/src/scene/boxMirror.ts`);
let bad = 0;
for (const row of process.argv.length > 2 ? process.argv.slice(2) : ['pass.json?roomclear&squad4', 'compound.json?roomclear&squad4']) {
  const [file, q] = row.split('?'), W: any = await standUp({ level: loadLevel(parseLevelData(readFileSync(`${R}/levels/${file}`, 'utf8'))), seed: 3 } as any), r = startFromQuery(W.game, `?levelFile=${file}&${q}`); if (r.why) throw new Error(r.why);
  const g = W.game, reg = knobRegistry() as Record<string, unknown>, take = () => exportGame(g, reg).slice(), same = (a: Uint8Array, b: Uint8Array) => { if (a.length !== b.length) return `${a.length} bytes against ${b.length}`; let n = 0, at = -1; for (let i = 0; i < a.length; i++) if (a[i] !== b[i]) { if (at < 0) at = i; n++; } return n ? `${n} bytes differ, the first at ${at} of ${a.length}` : ''; };
  g.sim.settle(g); const rest = take(), again = same(rest, take()); if (again) { bad++; console.log(`FAIL ${row}: two exports at rest differ: ${again}`); continue; }
  const p = g.player.char.pos; let busy: Uint8Array | null = null;
  g.col.window(() => { g.col.raycast([p[0], p[1] + 1, p[2]], [1, 0, 0], 40); g.col.aside({}, () => { g.col.segmentBlocked([p[0], 1, p[2]], [p[0] + 4, 1, p[2] + 4]); MIR.LEAVES.n += 3; MIR.THINGS.n += 2; if (COL.ANYTIME) { COL.ANYTIME.proofs += 5; COL.ANYTIME.windows += 1; } if (COL.SENSED) COL.SENSED.sight += 7; if (process.env.HOSTFLAGS_BREAK) g.time += 1; busy = take(); }); });
  const d = same(rest, busy!); if (d) bad++; console.log(`${d ? 'FAIL' : 'ok  '} ${row}: ${rest.length} bytes${d ? `, ${d}` : ', the same with the host in the middle of its work'}`);
}
console.log(`=== hostflags-check: ${bad ? `${bad} FAIL` : 'all pass'} ===`); process.exit(bad ? 1 : 0);
