// THE TYPESCRIPT SIDE OF A TWO-SEAT GAME, tick by tick: tools/qa/seats-run.ts's `standTwo` (a game of two stood up headless, both seats driven by scripted hands,
// every tick `g.stepSeats(1/60, [record, record])` and what the headless host does round it), written out as mission.ts writes a mission, so that the same
// runner steps the Rust game from the same start with the same two records and compares after every tick.
//   bun run --preload ./tools/blitz/expose.ts tools/blitz/seats.ts <two-roomclear|two-nightbreach|two-compound>:<seed> --out DIR [--ticks N] [--keep "…"] [--every N] [--open "…"] [--asks]
// Run by run.sh for a name `two-*`. The games (seats-run.ts's own): two-roomclear = `roomclear&squad4` on the page's floor (40 s), two-nightbreach =
// `nightbreach` (30 s), two-compound = `roomclear&squad4` on levels/compound.json (30 s).
// It writes what mission.ts writes, with one difference: h<k>.bin, THE HOST'S HALF as tick k opens, begins with the count of seats and a record a seat
// (the Game's `cmd_buf` holds one player's record and is not what a game of two is stepped from), then every camera and Input row as before.
// What the host does round a step here (seats-run.ts `tick`): the lamps' update after the tick, the box world listed again after EVERY tick (host.txt's
// last column is therefore 1 from the second tick), each hand's camera moved and its Input's frame ended. The cameras and Inputs of the hands are the
// host's own: they feed the next tick's records and nothing else.
import '../qa/stand.ts';
import { writeFileSync, mkdirSync, appendFileSync } from 'node:fs';
const { exportGame, exportPart, fieldAt, bookOf } = await import('./host.ts');
const argv = process.argv.slice(2), arg = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv[i + 1]; };
const say = (s = '') => process.stdout.write(s + '\n');
const GAMES: Record<string, { query: string; level: string | null; secs: number }> = {
  'two-roomclear': { query: 'roomclear&squad4', level: null, secs: 40 }, 'two-nightbreach': { query: 'nightbreach', level: null, secs: 30 }, 'two-compound': { query: 'roomclear&squad4', level: 'compound', secs: 30 } };
const outDir = arg('--out', ''), EVERY = +arg('--every', '0');
const [name, seedS] = (argv[0] ?? '').split(':'), G = GAMES[name], seed = +(seedS ?? 9);
if (!G || !outDir) { say(`seats.ts <${Object.keys(GAMES).join('|')}>:<seed> --out DIR [--ticks N] [--keep "100,200-210"] [--every N] [--open "559"]`); process.exit(64); }
const TICKS = Math.min(+arg('--ticks', '1000000'), Math.round(G.secs * 60));
const ranges = (s: string) => s.split(',').filter(Boolean).map(x => x.split('-').map(Number)).map(([a, b]) => [a, b ?? a]);
const KEEP = ranges(arg('--keep', '')), OPEN = ranges(arg('--open', ''));
const inR = (R: number[][], k: number) => R.some(([a, b]) => k >= a && k <= b), keep = (k: number) => inR(KEEP, k) || (EVERY > 0 && k % EVERY === 0);
mkdirSync(outDir, { recursive: true });
const { knobRegistry } = await import('../../src/game/knobs.ts');
const { dressAll } = await import('../../src/game/dress.ts');
const { simLayout, SIM_PARTS } = await import('../../src/sim/state.ts');
const { sim } = await import('../../src/sim/sim.ts');
const { standTwo, DT } = await import('../qa/seats-run.ts');
const reg = knobRegistry();
const words = (g: any): Uint32Array => g.sim.w;
const block = (g: any): Uint8Array => { const w = words(g); return new Uint8Array(w.buffer, w.byteOffset, w.byteLength).slice(); };
let LAY: { off: number; len: number }[] | null = null;
function hashes(w: Uint32Array): number[] {
  LAY ??= simLayout().parts.map(p => ({ off: p.off >>> 2, len: p.len >>> 2 }));
  return LAY.map(({ off, len }) => { let n = Math.min(off + len, w.length); while (n > off && w[n - 1] === 0) n--; let h = 0x811c9dc5; for (let i = off; i < n; i++) h = Math.imul(h ^ w[i], 0x01000193); return h >>> 0; });
}
for (const f of ['host.txt', 'gives.txt', 'hostwrites.txt']) writeFileSync(`${outDir}/${f}`, '');
writeFileSync(`${outDir}/post.txt`, `parts ${SIM_PARTS.join(' ')}\n`);
const log = console.log; console.log = () => {};
const P = await standTwo(G.query, seed, G.level), g = P.g;
console.log = log;
dressAll(false);
if (!g.dice) throw new Error('seats.ts: the game of two has no die of its own');
function hostHalf(cmds: any[]): Uint8Array { const bk: any = bookOf(g), L: [string, any][] = [];
  const cb = fieldAt(g, reg, ['cmd_buf']); L.push(['u32', cmds.length]); for (const c of cmds) L.push([cb.ty, c]);
  for (const [id, ty] of [['CameraId', 'crate::camera::FollowCamera'], ['InputId', 'crate::input::Input']]) { const R: any[] = bk.listOf(id); L.push(['u32', R.length]); for (const row of R) L.push([ty, row]); }
  return exportPart(g, reg, L); }
// (`--asks`: every ask of the Game's own seat field made inside a tick, by where it came from, and the first tick with one: seats.ts SEAT_ASKS)
const { SEAT_ASKS } = await import('../../src/game/seats.ts');
const ASKS = argv.includes('--asks'); if (ASKS) SEAT_ASKS.sites = new Map(); let firstAsk = 0;
let k = 0, threw: unknown = null;
for (let i = 0; i < TICKS && !threw; i++) {
  const cmds = P.sample(i); k = i + 1;
  const step = g.stepSeats;
  g.stepSeats = function (this: any, dt: number, c: unknown) {
    if (k === 1) { g.sim.settle(g); writeFileSync(`${outDir}/c1.bin`, exportGame(g, reg)); writeFileSync(`${outDir}/s0.bin`, block(g)); }
    else if (inR(OPEN, k)) writeFileSync(`${outDir}/c${k}.bin`, exportGame(g, reg));
    writeFileSync(`${outDir}/h${k}.bin`, hostHalf(cmds));
    appendFileSync(`${outDir}/host.txt`, `${dt} ${g.probe?.pending?.length ?? 0} ${g.time} ${g.dice?.n ?? -1} ${k === 1 ? 0 : 1}\n`);
    try { return step.call(this, dt, c); } finally {
      appendFileSync(`${outDir}/post.txt`, `${k} ${g.dice?.n ?? -1} ${g.dice?.s ?? -1} ${g.time} ${hashes(words(g)).join(' ')}\n`);
      if (keep(k)) { writeFileSync(`${outDir}/a${k}.bin`, exportGame(g, reg)); writeFileSync(`${outDir}/s${k}.bin`, block(g)); } }
  };
  try { P.tick(cmds); } catch (e) { threw = e; } finally { delete g.stepSeats; }
  if (!firstAsk && SEAT_ASKS.misses) firstAsk = k;
}
if (threw) say(`THE TYPESCRIPT TICK ${k} THREW: ${(threw as any)?.stack ?? threw}`);
{ const x: any = sim(), at = x.knob_addr(0), len = x.knob_len(0) * 8; writeFileSync(`${outDir}/knobs.bin`, new Uint8Array(x.memory.buffer).slice(at, at + len)); }
if (ASKS) for (const [site, n] of [...SEAT_ASKS.sites!].sort((a, b) => b[1] - a[1])) say(`   ${n} × ${site}`);
say(`TypeScript: ${name} (${G.query}${G.level ? ` on ${G.level}` : ''}) seed ${seed}, two seats, ${k} ticks; time ${g.time.toFixed(4)}; die n=${g.dice?.n}; ${g.guards.length} guards; the Game's own seat fields asked inside a tick ${SEAT_ASKS.misses} times (${[...SEAT_ASKS.missed].join(' ')})${firstAsk ? `, first in tick ${firstAsk}` : ''}`);
say(`ticks ${threw ? k - 1 : k}; resident ${(process.memoryUsage().rss / 1e9).toFixed(2)} GB`);
writeFileSync(`${outDir}/seats.txt`, '2\n');
writeFileSync(`${outDir}/ticks.txt`, `${threw ? k - 1 : k}\n`);
process.exit(0);
