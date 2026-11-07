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
// `--again [--played N] [--between <id>]`: the same start made a second time in this process, after N frames of the first game (and, with
// `--between`, after that other mission was stood and played N frames too): again.bin, again-s.bin.
// (THIS IS THE TYPESCRIPT START: the harness's word, which by default makes a stood game a puppet of the Rust module (tools/blitz/word.ts), is put off
// before the harness is first imported, on every path)
// (but `--again --played N` STEPS the first game, and only the Rust game steps: that run is asked for with TPS_GAME=rust in front, and says so)
{ const a = process.argv, i = a.indexOf('--played'), played = a.includes('--again') && i >= 0 ? +a[i + 1] : 0;
  if (!played) process.env.TPS_GAME = 'none'; else if (process.env.TPS_GAME !== 'rust') { process.stdout.write('start.ts: --again --played N steps the first game, which only the Rust game does: TPS_GAME=rust in front (tools/blitz/rust.sh)\n'); process.exit(64); } }
import { STAND } from '../qa/stand.ts';
import { writeFileSync, mkdirSync, readFileSync, existsSync } from 'node:fs';
import { resolve } from 'node:path';
const argv = process.argv.slice(2), arg = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv[i + 1]; };
const say = (s = '') => process.stdout.write(s + '\n');
const outDir = arg('--out', ''), SEED = +arg('--seed', '20942'), SEATS = +arg('--seats', '1'), SCN = arg('--scenario', ''), MISSION = arg('--mission', '');
let file = argv[0], spec: any = null, LINK = arg('--link', ''), BETWEEN: { file: string; link: string } | null = null;
if (STAND !== 'shipped') { say(`start.ts: the stand is '${STAND}': the Rust start is the page's, so this compare is too`); process.exit(64); }
if (MISSION || argv.includes('--missions')) {
  const { ROOT } = await import('../qa/headless.ts');   // (the page's stand on the switches: the look decides which file of a level the page loads)
  const { ALL_MISSIONS } = await import('../../src/scene/scenarioList.ts'), { scenarioQuery } = await import('../../src/scene/scenarios.ts'), { lookLevelFile } = await import('../../src/scene/look.ts');
  const row = (e: any) => { const look = e.level ? lookLevelFile(e.level) : ''; return { id: e.id, file: !e.level ? 'office.json' : existsSync(resolve(ROOT, 'levels', look)) ? look : e.level, link: scenarioQuery(e), listed: !e.offList }; };
  if (!MISSION) { for (const e of ALL_MISSIONS) { const r = row(e); say(`${r.id}\t${r.file}\t${r.link}\t${r.listed ? 'listed' : 'off the list'}`); } process.exit(0); }
  const e = ALL_MISSIONS.find((m: any) => m.id === MISSION); if (!e) { say(`start.ts: no mission '${MISSION}' (start.ts --missions lists them)`); process.exit(64); }
  const r = row(e); file = r.file; LINK = r.link;
  { const id = arg('--between', ''), b = id ? ALL_MISSIONS.find((m: any) => m.id === id) : null; if (id && !b) { say(`start.ts: no mission '${id}' to stand between`); process.exit(64); } if (b) BETWEEN = row(b); }
  if (outDir) { mkdirSync(outDir, { recursive: true }); writeFileSync(`${outDir}/mission.txt`, `${file}\n${LINK}\n`); }
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
if (SCN && !['roomclear', 'nightbreach', 'rescue', 'quiet'].includes(SCN)) { say(`start.ts: no scenario kind '${SCN}' (roomclear | nightbreach | rescue | quiet)`); process.exit(64); }
if (SCN && !spec && !LINK) { const deal = arg('--deal', ''); spec = { kind: SCN, squad: +arg('--squad', '4'), ...(deal !== '' ? { seed: +deal } : {}) }; }
dressAll(false);
/** ONE START: the level read from its file anew (a stand-up marks a level's boxes), stood, its mission started, its moving state at home */
async function stood(at = path, link = LINK, orNone = false): Promise<any> {
  const level = loadLevel(parseLevelData(readFileSync(at, 'utf8')));
  const W = await standUp({ level, seed: SEED, seats: SEATS }), g: any = W.game;
  if (link) {
    const { startFromQuery } = await import('../../src/game/scenarioRun.ts');
    const r = startFromQuery(g, link);
    if (r.why && orNone) return null;
    if (r.why) { say(`start.ts: the game refused the ${r.spec?.kind ?? 'start'}: ${r.why}`); process.exit(3); }
  } else if (spec) {
    const { startScenario } = await import('../../src/game/scenarioRun.ts');
    const why = startScenario(g, spec);
    if (why) { say(`start.ts: the game refused the ${spec.kind}: ${why}`); process.exit(3); }
  }
  g.sim.settle(g);
  return W;
}
// (with --again both states are A START'S EXPORT, as a host takes one: every weak table lists the keys of the game being walked and no other game's,
//  export.ts startExport. Taken plain, the second game's tables list the first game's men for as long as it is in memory.)
const startExport: (g: any, knobs: Record<string, unknown>) => Uint8Array = argv.includes('--again') ? (await import('./export.ts')).startExport : exportGame;
const taken = (g: any, state: string, block: string) => { const w: Uint32Array = g.sim.w; writeFileSync(`${outDir}/${state}`, startExport(g, knobRegistry())); writeFileSync(`${outDir}/${block}`, new Uint8Array(w.buffer, w.byteOffset, w.byteLength).slice()); };
const W = await stood(), g: any = W.game;
taken(g, 'c1.bin', 's0.bin');
say(`start.ts: ${path} stood on the page's stand, die seed ${SEED}${SEATS > 1 ? `, ${SEATS} seats` : ''}${LINK ? `, started from ${LINK}` : spec ? `, ${spec.kind} with ${spec.squad}${spec.seed !== undefined ? `, deal ${spec.seed}` : ''}` : ''}: ${g.guards.length} guards, ${g.level.boxes.length} boxes, ${g.doors.list.length} doors; die n=${g.dice?.n ?? '?'}; state in ${outDir}/c1.bin`);
// --again [--played N] [--between <id>]: THE SAME START A SECOND TIME IN THIS PROCESS, after the first game was stepped N frames with an idle hand
// (default 0) and, with --between, after that other mission was stood here and stepped N frames too: its state in again.bin and its block in
// again-s.bin. A game's state lives on the Game and a start writes all of it, so the second start is the first start; what differs between c1.bin
// and again.bin is something the process kept from a game before (tools/qa/fresh.ts is what puts it back; tools/blitz/start-levels.ts --again holds it).
if (argv.includes('--again')) {
  const N = +arg('--played', '0'), play = (w: any) => { for (let i = 0; i < N; i++) w.step(1 / 60); };
  play(W);
  let between = '';
  if (BETWEEN) { const at = resolve(ROOT, 'levels', BETWEEN.file), O = await stood(at, BETWEEN.link, true); if (O) play(O); between = O ? `, then ${arg('--between', '')} stood${N ? ' and played as long' : ''}` : `, then ${arg('--between', '')} asked for and refused by the game`; }
  const W2 = await stood(); taken(W2.game, 'again.bin', 'again-s.bin');
  say(`start.ts: stood again in the same process${N ? ` after ${N} frames of the first game` : ''}${between}: state in ${outDir}/again.bin`);
}
process.exit(0);
