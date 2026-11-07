// THE TYPESCRIPT START OF A LEVEL FILE, exported by the bridge: what rust/game_start's own start is held to (rust/game_start/README.md, "The compare").
//   bun run --preload ./tools/blitz/expose.ts tools/blitz/start.ts <level file> --out DIR [--seed N] [--scenario roomclear|nightbreach|rescue|quiet [--squad N] [--deal N]]
// Stands the level headless on the page's stand through the one stand-up (src/host/standup.ts standGame), with the game's own die seeded BEFORE the Game is
// made (the men's construction dice are its first draws), every dressing row down (a host that draws nothing), `sim.settle` called, and writes into DIR:
//   c1.bin   the whole state, as the first tick would open
//   s0.bin   rust/sim's block then
// With `--scenario` the mission is started by the game's own `startScenario` (a spec of that kind, `--squad` teammates, `--deal` its seed) before the
// state is taken. No tick is run. The stand must be the page's: a run under TPS_PROFILE=frozen is refused, since the Rust start has only the page's values.
import { STAND } from '../qa/stand.ts';
import { writeFileSync, mkdirSync, readFileSync, existsSync } from 'node:fs';
import { resolve } from 'node:path';
const argv = process.argv.slice(2), arg = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv[i + 1]; };
const say = (s = '') => process.stdout.write(s + '\n');
const file = argv[0], outDir = arg('--out', ''), SEED = +arg('--seed', '20942'), SCN = arg('--scenario', '');
if (!file || file.startsWith('--') || !outDir) { say('start.ts <level file> --out DIR [--seed N] [--scenario <kind> [--squad N] [--deal N]]'); process.exit(64); }
if (STAND !== 'shipped') { say(`start.ts: the stand is '${STAND}': the Rust start is the page's, so this compare is too`); process.exit(64); }
mkdirSync(outDir, { recursive: true });
const { exportGame } = await import('./host.ts');
const { knobRegistry } = await import('../../src/game/knobs.ts');
const { dressAll } = await import('../../src/game/dress.ts');
const { parseLevelData } = await import('../../src/scene/levelData.ts');
const { loadLevel } = await import('../../src/scene/levelLoad.ts');
const { standUp, ROOT } = await import('../qa/headless.ts');
const path = existsSync(file) ? file : resolve(ROOT, 'levels', file);
const level = loadLevel(parseLevelData(readFileSync(path, 'utf8')));
dressAll(false);
const W = await standUp({ level, seed: SEED });
const g: any = W.game;
if (SCN) {
  if (!['roomclear', 'nightbreach', 'rescue', 'quiet'].includes(SCN)) { say(`start.ts: no scenario kind '${SCN}' (roomclear | nightbreach | rescue | quiet)`); process.exit(64); }
  const { startScenario } = await import('../../src/game/scenarioRun.ts');
  const deal = arg('--deal', ''), why = startScenario(g, { kind: SCN as any, squad: +arg('--squad', '4') as any, ...(deal !== '' ? { seed: +deal } : {}) });
  if (why) { say(`start.ts: the game refused the ${SCN}: ${why}`); process.exit(3); }
}
g.sim.settle(g);
const w: Uint32Array = g.sim.w;
writeFileSync(`${outDir}/c1.bin`, exportGame(g, knobRegistry()));
writeFileSync(`${outDir}/s0.bin`, new Uint8Array(w.buffer, w.byteOffset, w.byteLength).slice());
say(`start.ts: ${path} stood on the page's stand, die seed ${SEED}${SCN ? `, scenario ${SCN}` : ''}: ${g.guards.length} guards, ${level.boxes.length} boxes, ${g.doors.list.length} doors; die n=${g.dice?.n ?? '?'}; state in ${outDir}/c1.bin`);
process.exit(0);
