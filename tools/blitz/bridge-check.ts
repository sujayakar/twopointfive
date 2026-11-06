// THE BRIDGE ALONE: a live TypeScript game mid-mission, exported, imported in Rust and exported again, must be the same bytes. Needs no translated body.
//   TPS_PROFILE=shipped bun run tools/blitz/bridge-check.ts [--level compound.json] [--query 'levelFile=compound.json&roomclear&squad4'] [--ticks 600] [--play room-clear-four:1:2400] [--out x.bin]
// It writes the blob; `BRIDGE_BLOB=x.bin cargo test --lib bridge_round_trip` in the crate is the other half. Exit 1 if two exports with no tick between differ.
import { writeFileSync } from 'node:fs';
import { exportGame, report, SIZES, ROWS, SERIALS } from './export';
import { decode } from './paths';
const argv = process.argv.slice(2), arg = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv[i + 1]; };
const query = arg('--query', 'levelFile=compound.json&roomclear&squad4'), ticks = +arg('--ticks', '600'), out = arg('--out', '/tmp/bridge.bin');
const { knobRegistry } = await import('../../src/game/knobs.ts');
const reg = knobRegistry();
let g: any;
const moment = arg('--play', '');   // scn:seed:frame: a real play-through, stopped at that frame with the commander's team on the floor
if (moment) { const [scn, seed, frame] = moment.split(':'); const { Game } = await import('../../src/game/game.ts'); const { play } = await import('../qa/playthrough.ts'); class Stop extends Error {}; const was = Game.prototype.update; let n = 0; Game.prototype.update = function (this: any, ...a: unknown[]) { if (++n >= +frame) { g = this; throw new Stop(); } return (was as any).apply(this, a); } as any; const log0 = console.log; console.log = () => {}; try { await play(scn, +seed, 300, false, {}); } catch (e) { if (!(e instanceof Stop)) throw e; } console.log = log0; Game.prototype.update = was; if (!g) throw new Error('the run ended before that frame'); }
else { const { standUp, ROOT } = await import('../qa/headless.ts'); const lf = new URLSearchParams(query).get('levelFile'); let level: any;
  if (lf) { const { readFileSync } = await import('node:fs'); const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`), { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`), { withOfficeScript, isOfficeFamily } = await import(`${ROOT}/src/scene/office.ts`).catch(() => ({} as any)); const data = parseLevelData(readFileSync(`${ROOT}/levels/${lf}`, 'utf8')); level = isOfficeFamily?.(data) ? withOfficeScript(loadLevel(data)) : loadLevel(data); }
  const log = console.log; console.log = () => {}; const W = await standUp({ ...(level ? { level } : {}), ...(arg('--seats', '') ? { seats: +arg('--seats', '1') } : {}) }); g = W.game; if (query) { const { startFromQuery } = await import('../../src/game/scenarioRun.ts'); await startFromQuery(g, new URLSearchParams(query)); } console.log = log;
  const DT = 1 / 60; for (let i = 0; i < ticks; i++) { if (g.seats?.length > 1) g.stepSeats(DT, g.seats.map(() => undefined)); else g.step(DT, undefined); g.engine.lights.update(DT, g.time); g.engine.world.dynamics = g.dynBoxes; g.engine.world.upload(); } }
const t0 = performance.now(); const a = exportGame(g, reg); const ms = performance.now() - t0; const rep = report();
const b = exportGame(g, reg);
writeFileSync(out, a);
let same = a.length === b.length; if (same) for (let i = 0; i < a.length; i++) if (a[i] !== b[i]) { same = false; break; }
const rows = a.length < 40e6 ? decode(a).length : -1;
console.log(`${moment || query} after ${moment ? '' : ticks} ticks, game time ${g.time.toFixed(2)}: ${a.length} bytes, ${rows} leaves, ${ms.toFixed(1)} ms. Two exports ${same ? 'are the same' : 'DIFFER'}`);
console.log('largest:'); for (const [k, v] of [...SIZES].sort((p, q) => q[1] - p[1]).slice(0, +arg('--sizes', '40'))) console.log(`  ${(v / 2 / 1e6).toFixed(2)} MB ${k}`);
console.log('rows:'); for (const [k, v] of ROWS) console.log(`  ${v.n} ${k} (first ${v.first})`);
const R = [...rep].sort((x, y) => y[1].n - x[1].n); console.log(`what did not fit: ${R.length} kinds`); for (const [k, v] of R.slice(0, +arg('--show', '60'))) console.log(`  ${v.n}× ${k} (${v.eg})`);
writeFileSync(out + '.report.json', JSON.stringify(R.map(([k, v]) => ({ at: k, ...v }))));
writeFileSync(out + '.serials.json', JSON.stringify([...SERIALS].sort()));
process.exit(same ? 0 : 1);
