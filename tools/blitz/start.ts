// THE TYPESCRIPT START OF A LEVEL FILE, exported by the bridge: what rust/game_start's own start is held to (rust/game_start/README.md, "The compare").
//   bun run --preload ./tools/blitz/expose.ts tools/blitz/start.ts <level file> --out DIR [--seed N] [--seats 2] [--scenario roomclear|nightbreach|rescue|quiet [--squad N] [--deal N]]
//   bun run --preload ./tools/blitz/expose.ts tools/blitz/start.ts --mission <id> --out DIR [--seed N]     a row of the mission list, as a click on it starts it
//   bun run --preload ./tools/blitz/expose.ts tools/blitz/start.ts --missions                              the list's rows: id, the file the page loads, the link
// Stands the level headless on the page's stand through the one stand-up (src/host/standup.ts standGame), with the game's own die seeded BEFORE the Game is
// made (the men's construction dice are its first draws), every dressing row down (a host that draws nothing), `sim.settle` called, and writes into DIR:
//   c1.bin   the whole state, as the first tick would open
//   s0.bin   rust/sim's block then
// With `--scenario` the mission is started by the game's own `startScenario` (a spec of that kind, `--squad` teammates, `--deal` its seed) before the
// state is taken. `--mission <id>` is a row of the mission list (scene/scenarioList.ts): the level file the page loads for it (the look's,
// scene/look.ts lookLevelFile) and the mission its link names (scenarios.ts scenarioQuery read by specOfQuery, as the page reads it), both written
// to DIR/mission.txt for the Rust side (the office's missions: on a copy of its file with its scenarios written in, below). The mission is started by `startScenario`, NOT by the page's `startFromQuery`, which first lays the row's
// hour and weather, the fog and a quiet mission's cameras off the address: the Rust game has no type for an address's words yet. No tick is run.
import { STAND } from '../qa/stand.ts';
import { writeFileSync, mkdirSync, readFileSync, existsSync } from 'node:fs';
import { resolve } from 'node:path';
const argv = process.argv.slice(2), arg = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv[i + 1]; };
const say = (s = '') => process.stdout.write(s + '\n');
const outDir = arg('--out', ''), SEED = +arg('--seed', '20942'), SEATS = +arg('--seats', '1'), SCN = arg('--scenario', ''), MISSION = arg('--mission', '');
let file = argv[0], spec: any = null;
if (STAND !== 'shipped') { say(`start.ts: the stand is '${STAND}': the Rust start is the page's, so this compare is too`); process.exit(64); }
if (MISSION || argv.includes('--missions')) {
  const { ROOT } = await import('../qa/headless.ts');   // (the page's stand on the switches: the look decides which file of a level the page loads)
  const { ALL_MISSIONS, specOfQuery } = await import('../../src/scene/scenarioList.ts'), { scenarioQuery } = await import('../../src/scene/scenarios.ts'), { lookLevelFile } = await import('../../src/scene/look.ts');
  const row = (e: any) => { const look = e.level ? lookLevelFile(e.level) : ''; return { id: e.id, file: !e.level ? 'office.json' : existsSync(resolve(ROOT, 'levels', look)) ? look : e.level, link: scenarioQuery(e), listed: !e.offList }; };
  if (!MISSION) { for (const e of ALL_MISSIONS) { const r = row(e); say(`${r.id}\t${r.file}\t${r.link}\t${r.listed ? 'listed' : 'off the list'}`); } process.exit(0); }
  const e = ALL_MISSIONS.find((m: any) => m.id === MISSION); if (!e) { say(`start.ts: no mission '${MISSION}' (start.ts --missions lists them)`); process.exit(64); }
  const r = row(e); file = r.file; spec = specOfQuery(r.link);
  // THE OFFICE's missions are not in its file: a TypeScript module holds them and they are attached by the level's name (scene/level.ts withOfficeScript).
  // Both starts are stood on a COPY of the bundled file with that block written in, as every other level carries its own (the loader takes a file's own block).
  if (!e.level) {
    if (!outDir) { say('start.ts: --mission of the office needs --out DIR (the copy of its file is written there)'); process.exit(64); }
    const { OFFICE_DATA, OFFICE_SCENARIOS } = await import('../../src/scene/level.ts');
    mkdirSync(outDir, { recursive: true }); file = resolve(outDir, 'office-with-scenarios.json');
    writeFileSync(file, JSON.stringify({ ...OFFICE_DATA, scenarios: OFFICE_SCENARIOS }));
  }
  if (outDir) { mkdirSync(outDir, { recursive: true }); writeFileSync(`${outDir}/mission.txt`, `${file}\n${spec.kind}\n${spec.squad}\n${spec.seed ?? ''}\n`); }
}
if (!file || (file.startsWith('--') && !MISSION) || !outDir) { say("start.ts <level file> --out DIR [--seed N] [--scenario <kind> [--squad N] [--deal N]] · start.ts --mission <id> --out DIR · start.ts --missions"); process.exit(64); }
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
const W = await standUp({ level, seed: SEED, seats: SEATS });
const g: any = W.game;
if (SCN && !['roomclear', 'nightbreach', 'rescue', 'quiet'].includes(SCN)) { say(`start.ts: no scenario kind '${SCN}' (roomclear | nightbreach | rescue | quiet)`); process.exit(64); }
if (SCN && !spec) { const deal = arg('--deal', ''); spec = { kind: SCN, squad: +arg('--squad', '4'), ...(deal !== '' ? { seed: +deal } : {}) }; }
if (spec) {
  const { startScenario } = await import('../../src/game/scenarioRun.ts');
  const why = startScenario(g, spec);
  if (why) { say(`start.ts: the game refused the ${spec.kind}: ${why}`); process.exit(3); }
}
g.sim.settle(g);
const w: Uint32Array = g.sim.w;
writeFileSync(`${outDir}/c1.bin`, exportGame(g, knobRegistry()));
writeFileSync(`${outDir}/s0.bin`, new Uint8Array(w.buffer, w.byteOffset, w.byteLength).slice());
say(`start.ts: ${path} stood on the page's stand, die seed ${SEED}${SEATS > 1 ? `, ${SEATS} seats` : ''}${spec ? `, ${spec.kind} with ${spec.squad}${spec.seed !== undefined ? `, deal ${spec.seed}` : ''}` : ''}: ${g.guards.length} guards, ${level.boxes.length} boxes, ${g.doors.list.length} doors; die n=${g.dice?.n ?? '?'}; state in ${outDir}/c1.bin`);
process.exit(0);
