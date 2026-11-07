// THE FOG'S FRAME RECORD REACHES A PAGE.
//
// The picture draws the fog of war from one record, `game.fogOfWar.frame` (render_fogofwar::FogFrame: is the fog on, the eyes, the lit and thinned cells),
// which the engine holds as `engine.fogFrame`. A page is sent only the fields on the page's list (tools/wire/fields.json `pageTick`); a headless tool is
// sent every field unless it asks for the page's list (TPS_GAME_LIST=page). The record's 26 fields were once on the every-tick list alone: every headless
// check read a full record and every page drew no fog at all, from the day the page became the Rust game. This stands a game ON THE PAGE'S LIST and
// holds the record to what the same game reads when it is sent whole.
//
//   bun run tools/blitz/fogpage-check.ts          exit 0 all pass · 1 a line failed
//   FOGPAGE_BREAK=1 …                             the page's game is read before its first tick (the record as made): lines 1 and 2 must fail
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { standUp, ROOT } from '../qa/headless.ts';

const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);

const BREAK = process.env.FOGPAGE_BREAK === '1', DT = 1 / 60, TICKS = 30;
let fails = 0; const line = (ok: boolean, what: string, detail: string) => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
const log = console.log, quiet = async <T>(f: () => T | Promise<T>): Promise<T> => { console.log = () => { }; try { return await f(); } finally { console.log = log; } };
/** the frame record of the compound's room clear with four, `ticks` in, on a game sent the page's list or every field */
const read = async (list: 'page' | 'whole', ticks: number) => { const was = process.env.TPS_GAME_LIST; if (list === 'page') process.env.TPS_GAME_LIST = 'page'; else delete process.env.TPS_GAME_LIST;
  try { const W: any = await quiet(() => standUp({ level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', 'compound.json'), 'utf8'))), seed: 1 }));
    const why = (await quiet(() => startFromQuery(W.game, '?levelFile=compound.json&roomclear&squad4')) as any)?.why; if (why) throw new Error(`start refused: ${why}`);
    for (let i = 0; i < ticks; i++) W.step(DT);
    const f = W.game.fogOfWar.frame; return { gameOn: !!W.game.fogOfWar.on, on: !!f.on, full: !!f.full, count: f.count | 0, lightCount: f.lightCount | 0, thinCount: f.thinCount | 0 }; }
  finally { if (was === undefined) delete process.env.TPS_GAME_LIST; else process.env.TPS_GAME_LIST = was; } };

console.log(`\n=== fogpage: the fog's frame record on the page's list${BREAK ? ' — FOGPAGE_BREAK: read before the first tick' : ''} ===`);
const page = await read('page', BREAK ? 0 : TICKS), whole = await read('whole', TICKS), say = (r: typeof page) => `game's fog ${r.gameOn ? 'on' : 'off'} · frame on ${r.on} · eyes ${r.count} · lights ${r.lightCount} · thinned cells ${r.thinCount}`;
line(whole.gameOn && page.on && page.count > 0, 'a room clear\'s fog is on in the record the picture draws from, on a game sent the page\'s list', say(page));
line(JSON.stringify(page) === JSON.stringify(whole), 'the record on the page\'s list is the record of the same game sent whole', `page: ${say(page)}\n    whole: ${say(whole)}`);
console.log(fails ? `=== fogpage: ${fails} FAILED ===` : '=== fogpage: ALL PASS ===');
process.exit(fails ? 1 : 0);
