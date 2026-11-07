// THE TYPESCRIPT START OF A LEVEL FILE, exported by the bridge: what rust/game_start's own start is held to (rust/game_start/README.md, "The compare").
//   bun run --preload ./tools/blitz/expose.ts tools/blitz/start.ts <level file> --out DIR [--seed N] [--seats 2] [--scenario roomclear|nightbreach|rescue|quiet [--squad N] [--deal N] | --link '?words']
//   bun run --preload ./tools/blitz/expose.ts tools/blitz/start.ts --mission <id> --out DIR [--seed N]     a row of the mission list, as a click on it starts it
//   bun run --preload ./tools/blitz/expose.ts tools/blitz/start.ts --missions                              the list's rows: id, the file the page loads, the link
// Stands the level headless on the page's stand through the one stand-up (src/host/standup.ts standGame), with the game's own die seeded BEFORE the Game is
// made (the men's construction dice are its first draws), every dressing row down (a host that draws nothing), `sim.settle` called, and writes into DIR:
//   c1.bin   the whole state, as the first tick would open
//   s0.bin   rust/sim's block then
// With `--scenario` the mission is started by the game's own `startScenario` (a spec of that kind, `--squad` teammates, `--deal` its seed) before the
// state is taken; with `--link '?words'` it is started AS THE PAGE STARTS A MISSION, by the game's own `startFromQuery` on those words of an
// address (the row's own hour and weather, the fog of war, a quiet mission's rung and cameras, then the mission). `--mission <id>` is a row of the
// mission list (scene/scenarioList.ts): the level file the page loads for it (the look's, scene/look.ts lookLevelFile) and the link a click on the
// row loads (scenarios.ts scenarioQuery), both written to DIR/mission.txt for the Rust side. No tick is run.
import { STAND } from '../qa/stand.ts';
import { writeFileSync, mkdirSync, readFileSync, existsSync } from 'node:fs';
import { resolve } from 'node:path';
const argv = process.argv.slice(2), arg = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv[i + 1]; };
const say = (s = '') => process.stdout.write(s + '\n');
const outDir = arg('--out', ''), SEED = +arg('--seed', '20942'), SEATS = +arg('--seats', '1'), SCN = arg('--scenario', ''), MISSION = arg('--mission', '');
let file = argv[0], spec: any = null, LINK = arg('--link', '');
if (STAND !== 'shipped') { say(`start.ts: the stand is '${STAND}': the Rust start is the page's, so this compare is too`); process.exit(64); }
if (MISSION || argv.includes('--missions')) {
  const { ROOT } = await import('../qa/headless.ts');   // (the page's stand on the switches: the look decides which file of a level the page loads)
  const { ALL_MISSIONS } = await import('../../src/scene/scenarioList.ts'), { scenarioQuery } = await import('../../src/scene/scenarios.ts'), { lookLevelFile } = await import('../../src/scene/look.ts');
  const row = (e: any) => { const look = e.level ? lookLevelFile(e.level) : ''; return { id: e.id, file: !e.level ? 'office.json' : existsSync(resolve(ROOT, 'levels', look)) ? look : e.level, link: scenarioQuery(e), listed: !e.offList }; };
  if (!MISSION) { for (const e of ALL_MISSIONS) { const r = row(e); say(`${r.id}\t${r.file}\t${r.link}\t${r.listed ? 'listed' : 'off the list'}`); } process.exit(0); }
  const e = ALL_MISSIONS.find((m: any) => m.id === MISSION); if (!e) { say(`start.ts: no mission '${MISSION}' (start.ts --missions lists them)`); process.exit(64); }
  const r = row(e); file = r.file; LINK = r.link;
  if (outDir) { mkdirSync(outDir, { recursive: true }); writeFileSync(`${outDir}/mission.txt`, `${file}\n${LINK}\n`); }
}
if (!file || (file.startsWith('--') && !MISSION) || !outDir) { say("start.ts <level file> --out DIR [--seed N] [--scenario <kind> [--squad N] [--deal N]] · start.ts --mission <id> --out DIR · start.ts --missions"); process.exit(64); }
mkdirSync(outDir, { recursive: true });
// (THIS IS THE TYPESCRIPT START: the harness's word, which by default makes a stood game a puppet of the Rust module (tools/blitz/word.ts), is put off)
process.env.TPS_GAME = 'none';
const { exportGame } = await import('./host.ts');
const { knobRegistry } = await import('../../src/game/knobs.ts');
const { dressAll } = await import('../../src/game/dress.ts');
const { parseLevelData } = await import('../../src/scene/levelData.ts');
const { loadLevel } = await import('../../src/scene/levelLoad.ts');
const { standUp, ROOT } = await import('../qa/headless.ts');
const path = existsSync(file) ? file : resolve(ROOT, 'levels', file);
const level = loadLevel(parseLevelData(readFileSync(path, 'utf8')));
dressAll(false);
const W = await standUp({ level, seed: SEED, seats: SEATS });
const g: any = W.game;
if (SCN && !['roomclear', 'nightbreach', 'rescue', 'quiet'].includes(SCN)) { say(`start.ts: no scenario kind '${SCN}' (roomclear | nightbreach | rescue | quiet)`); process.exit(64); }
if (SCN && !spec && !LINK) { const deal = arg('--deal', ''); spec = { kind: SCN, squad: +arg('--squad', '4'), ...(deal !== '' ? { seed: +deal } : {}) }; }
if (LINK) {
  const { startFromQuery } = await import('../../src/game/scenarioRun.ts');
  const r = startFromQuery(g, LINK);
  if (r.why) { say(`start.ts: the game refused the ${r.spec?.kind ?? 'start'}: ${r.why}`); process.exit(3); }
} else if (spec) {
  const { startScenario } = await import('../../src/game/scenarioRun.ts');
  const why = startScenario(g, spec);
  if (why) { say(`start.ts: the game refused the ${spec.kind}: ${why}`); process.exit(3); }
}
g.sim.settle(g);
const w: Uint32Array = g.sim.w;
writeFileSync(`${outDir}/c1.bin`, exportGame(g, knobRegistry()));
writeFileSync(`${outDir}/s0.bin`, new Uint8Array(w.buffer, w.byteOffset, w.byteLength).slice());
say(`start.ts: ${path} stood on the page's stand, die seed ${SEED}${SEATS > 1 ? `, ${SEATS} seats` : ''}${LINK ? `, started from ${LINK}` : spec ? `, ${spec.kind} with ${spec.squad}${spec.seed !== undefined ? `, deal ${spec.seed}` : ''}` : ''}: ${g.guards.length} guards, ${level.boxes.length} boxes, ${g.doors.list.length} doors; die n=${g.dice?.n ?? '?'}; state in ${outDir}/c1.bin`);
process.exit(0);
