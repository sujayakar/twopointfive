// THE TYPESCRIPT SIDE OF A TWO-SEAT GAME, tick by tick: tools/qa/seats-run.ts's `standTwo` (a game of two stood up headless, both seats driven by scripted hands,
// every tick `g.stepSeats(1/60, [record, record])` and what the headless host does round it), written out as mission.ts writes a mission, so that the same
// runner steps the Rust game from the same start with the same two records and compares after every tick.
//   bun run --preload ./tools/blitz/expose.ts tools/blitz/seats.ts <two-roomclear|two-nightbreach|two-compound>:<seed> --out DIR [--ticks N] [--keep "…"] [--every N] [--open "…"] [--asks]
// Run by run.sh for a name `two-*`. The games (seats-run.ts's own): two-roomclear = `roomclear&squad4` on the page's floor (40 s), two-nightbreach =
// `nightbreach` (30 s), two-compound = `roomclear&squad4` on levels/compound.json (30 s).
// THREE GAMES FOR TWO RULES (blitz/coop-needs.md, needs 12 and 13; rust/game_start/tests/coop.rs holds them now):
//   two-lock0 | two-lock1   the page's floor, the hands idle: that seat's pair is given `stack` on the locked server door before tick 2 and `openClear` before
//                           tick 961, through the seat's own order book (a line of gives.txt ending `seat<N>`, which the runner gives as that seat)
//   two-full                the room clear on the barracks, 17 men as stood up, the scripted fight for 120 s. THE TYPESCRIPT'S PICTURE IS GIVEN ROOM here, a
//                           tool's stand-in and nothing of the game's: the skinned buffer holds 16 (anim/characters.ts MAX_CHARS) and the 17th man handed
//                           to it throws, so the hand-over past it is dropped and counted, and the fight plays on for the Rust side to be stepped beside
// and each writes coop.json: the orders it gave and what each answered, the tick each man of the ordered pair crossed the door's frame line, the most men
// handed to the picture in one frame and how many hand-overs were dropped.
// It writes what mission.ts writes: h<k>.bin, THE HOST'S HALF as tick k opens, is a record a seat (here two), then every camera and Input row
// (the bridge's own writer, export.ts hostHalf).
// What the host does round a step here (seats-run.ts `tick`): the lamps' update after the tick, each hand's camera moved and its Input's frame ended. The cameras and Inputs of the hands are the
// host's own: they feed the next tick's records and nothing else.
import '../qa/stand.ts';
import { writeFileSync, mkdirSync, appendFileSync } from 'node:fs';
const { exportGame } = await import('./host.ts');
const { hostHalf, bookOf, hostRowsText } = await import('./export.ts');
const argv = process.argv.slice(2), arg = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv[i + 1]; };
const say = (s = '') => process.stdout.write(s + '\n');
type Give = { tick: number; seat: number; id: string; door: string };
const lock = (seat: number): Give[] => [{ tick: 2, seat, id: 'stack', door: 'server' }, { tick: 961, seat, id: 'openClear', door: 'server' }];
const GAMES: Record<string, { query: string; level: string | null; secs: number; idle?: boolean; gives?: Give[]; room?: boolean }> = {
  'two-roomclear': { query: 'roomclear&squad4', level: null, secs: 40 }, 'two-nightbreach': { query: 'nightbreach', level: null, secs: 30 }, 'two-compound': { query: 'roomclear&squad4', level: 'compound', secs: 30 },
  'two-lock0': { query: 'roomclear&squad4', level: null, secs: 30, idle: true, gives: lock(0) }, 'two-lock1': { query: 'roomclear&squad4', level: null, secs: 30, idle: true, gives: lock(1) },
  'two-full': { query: 'roomclear&squad4', level: 'barracks', secs: 120, room: true } };
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
const P = await standTwo(G.query, seed, G.level, !!G.idle), g = P.g;
console.log = log;
// ---- what the co-op check reads (coop.json)
const COOP = { name, seed, seats: g.seats.length as number, men: g.seats.length + g.guards.length + (g.civilians?.length ?? 0), gave: [] as { tick: number; line: string; why: string | null }[],
  crossed: {} as Record<string, number>, door: null as null | { name: string; hinge: number[]; closedDir: number; width: number }, picture: { holds: 0, most: 0, mostTick: 0, dropped: 0, firstDropped: 0 } };
{ const { MAX_CHARS } = await import('../../src/anim/characters.ts'); COOP.picture.holds = MAX_CHARS;
  const C = g.chars, note = (i: number) => { if (i + 1 > COOP.picture.most) { COOP.picture.most = i + 1; COOP.picture.mostTick = k; } };
  if (G.room && C) { const si = C.setInstance.bind(C), sm = C.setMaterials.bind(C), fin = C.finish.bind(C);   // (THE PICTURE IS GIVEN ROOM: above)
    C.setInstance = (i: number, ...a: unknown[]) => { note(i); if (i < MAX_CHARS) return si(i, ...a); COOP.picture.dropped++; COOP.picture.firstDropped ||= k; };
    C.setMaterials = (i: number, ...a: unknown[]) => { if (i < MAX_CHARS) return sm(i, ...a); };
    C.finish = (n: number, ...a: unknown[]) => fin(Math.min(n, MAX_CHARS), ...a); }
  else if (C) { const si = C.setInstance.bind(C); C.setInstance = (i: number, ...a: unknown[]) => { note(i); return si(i, ...a); }; } }
/** the orders of this tick, given through the seat's own book between ticks, each a line of gives.txt as mission.ts writes the scripted commander's, with the seat */
function giveNow(tick: number) {
  for (const x of G.gives ?? []) if (x.tick === tick) { const v = g.seats[x.seat].view, d = g.doors.list.find((q: any) => q.def.name === x.door); if (!d) throw new Error(`seats.ts: no door '${x.door}' on this floor`);
    const T = v.orders.targetAt(d.frameCentre); if (T.kind !== 'door') throw new Error(`seats.ts: the ${x.door} door's frame is not a door target (${T.kind})`);
    const line = `${tick} ${x.id} - ${T.kind} ${T.at.join(' ')} door ${g.doors.list.indexOf(T.door)} ${T.side} book${bookOf(g).ids.get('DoorId')?.get(T.door)} seat${x.seat}`;
    const why = v.orders.give(x.id, T) ?? null; appendFileSync(`${outDir}/gives.txt`, line + '\n'); COOP.gave.push({ tick, line, why });
    COOP.door = { name: x.door, hinge: [...d.def.hinge], closedDir: d.def.closedDir, width: d.def.width }; } }
/** the tick each man of an ordered pair first stands on the far side of the door's frame line from where he stood when the order was given */
const sideOfDoor = (p: ArrayLike<number>) => { const D = COOP.door!; return Math.sign((p[0] - D.hinge[0]) * Math.sin(D.closedDir) - (p[2] - D.hinge[1]) * Math.cos(D.closedDir)); };
let pair: { gd: any; side: number }[] = [];
function watchPair() { if (!COOP.door || !G.gives) return; const s = G.gives[0].seat;
  if (!pair.length) pair = g.seats[s].view.orders.squad().map((gd: any) => ({ gd, side: sideOfDoor(gd.char.pos) }));
  for (const m of pair) { const now = sideOfDoor(m.gd.char.pos); if (now !== 0 && now !== m.side && COOP.crossed[m.gd.callsign] === undefined) COOP.crossed[m.gd.callsign] = k; } }
dressAll(false);
{ const w = argv.includes('--dress') ? argv[argv.indexOf('--dress') + 1] : ''; if (w === 'all') dressAll(true); else if (w) { const { DRESS } = await import('../../src/game/dress.ts'); for (const r of w.split(',')) (DRESS as any)[r] = true; } }   // (`--dress all|rows`: the dressing rows as the page has them, mission.ts)
if (G.room && argv.includes('--men')) { const { DRESS } = await import('../../src/game/dress.ts'); DRESS.men = true; }   // (`--men`: the men ARE handed to the TypeScript's picture, for its count; the Rust game hands nobody anything, and the picture's own state (g.rig_fits) then differs by design)
if (!g.dice) throw new Error('seats.ts: the game of two has no die of its own');
// (`--asks`: every ask of the Game's own seat field made inside a tick, by where it came from, and the first tick with one: seats.ts SEAT_ASKS)
const { SEAT_ASKS } = await import('../../src/game/seats.ts');
const ASKS = argv.includes('--asks'); if (ASKS) SEAT_ASKS.sites = new Map(); let firstAsk = 0;
const DIE_N = argv.includes('--die') && /^\d+$/.test(argv[argv.indexOf('--die') + 1] ?? '') ? +argv[argv.indexOf('--die') + 1] : null; const { newDice } = await import('../../src/math/dice.ts');
// BLITZ_CALL="<k> restart_encounter|reset_guards [start]" (mission.ts, the runner): the rule called before tick k of a game of two seats as well. With
// `start` the state after the call is written as a NEW START (L<k>.bin, Ls<k>.bin, a `chron_load` line), which the runner takes in place of calling.
const CALL = (process.env.BLITZ_CALL ?? '').split(' ');
let k = 0, threw: unknown = null;
for (let i = 0; i < TICKS && !threw; i++) {
  const cmds = P.sample(i); k = i + 1;
  const step = g.stepSeats;
  g.stepSeats = function (this: any, dt: number, c: unknown) {
    if (k === 1 && DIE_N !== null) g.dice = newDice(DIE_N);   // (`--die N`, as mission.ts: the start the Rust side imports carries the die)
    if (k === 1) { g.sim.settle(g); writeFileSync(`${outDir}/c1.bin`, exportGame(g, reg)); writeFileSync(`${outDir}/s0.bin`, block(g)); }
    else if (inR(OPEN, k)) writeFileSync(`${outDir}/c${k}.bin`, exportGame(g, reg));
    if (CALL[2] === 'start' && k === +CALL[0] && k > 1) { appendFileSync(`${outDir}/hostwrites.txt`, `${k} chron_load\n`); writeFileSync(`${outDir}/L${k}.bin`, exportGame(g, reg)); writeFileSync(`${outDir}/Ls${k}.bin`, block(g)); }
    writeFileSync(`${outDir}/h${k}.bin`, hostHalf(g, reg, cmds));
    // (`--start-only`, as mission.ts: the start alone for a kept run of an older layout of the wire; no tick is run)
    if (argv.includes('--start-only')) { writeFileSync(`${outDir}/hostrows.txt`, hostRowsText(g));   /* (which rows of the half are the HOST's own, handed at the stand: half-grow.ts gives them to a half kept before a host had them) */ const x: any = sim(), at = x.knob_addr(0), len = x.knob_len(0) * 8; writeFileSync(`${outDir}/knobs.bin`, new Uint8Array(x.memory.buffer).slice(at, at + len)); process.exit(0); }
    appendFileSync(`${outDir}/host.txt`, `${dt} ${g.probe?.pending?.length ?? 0} ${g.time} ${g.dice?.n ?? -1}\n`);
    try { return step.call(this, dt, c); } finally {
      appendFileSync(`${outDir}/post.txt`, `${k} ${g.dice?.n ?? -1} ${g.dice?.s ?? -1} ${g.time} ${hashes(words(g)).join(' ')}\n`);
      if (keep(k)) { writeFileSync(`${outDir}/a${k}.bin`, exportGame(g, reg)); writeFileSync(`${outDir}/s${k}.bin`, block(g)); } }
  };
  if (CALL[0] && k === +CALL[0]) { if (CALL[1] === 'restart_encounter') g.restartEncounter(); else if (CALL[1] === 'reset_guards') g.resetGuards(); else throw new Error(`BLITZ_CALL: no rule ${CALL[1]}`); }
  giveNow(k);
  try { P.tick(cmds); } catch (e) { threw = e; } finally { delete g.stepSeats; }
  watchPair();
  if (!firstAsk && SEAT_ASKS.misses) firstAsk = k;
}
if (threw) say(`THE TYPESCRIPT TICK ${k} THREW: ${(threw as any)?.stack ?? threw}`);
{ const x: any = sim(), at = x.knob_addr(0), len = x.knob_len(0) * 8; writeFileSync(`${outDir}/knobs.bin`, new Uint8Array(x.memory.buffer).slice(at, at + len)); }
if (ASKS) for (const [site, n] of [...SEAT_ASKS.sites!].sort((a, b) => b[1] - a[1])) say(`   ${n} × ${site}`);
say(`TypeScript: ${name} (${G.query}${G.level ? ` on ${G.level}` : ''}) seed ${seed}, two seats, ${k} ticks; time ${g.time.toFixed(4)}; die n=${g.dice?.n}; ${g.guards.length} guards; the Game's own seat fields asked inside a tick ${SEAT_ASKS.misses} times (${[...SEAT_ASKS.missed].join(' ')})${firstAsk ? `, first in tick ${firstAsk}` : ''}`);
say(`ticks ${threw ? k - 1 : k}; resident ${(process.memoryUsage().rss / 1e9).toFixed(2)} GB`);
writeFileSync(`${outDir}/coop.json`, JSON.stringify({ ...COOP, ticks: threw ? k - 1 : k, threw: threw ? String((threw as any)?.message ?? threw) : null, asks: SEAT_ASKS.misses, firstAsk }));
for (const x of COOP.gave) say(`   the order before tick ${x.tick}: ${x.line} -> ${x.why ?? 'given'}`);
if (G.room || COOP.picture.most > COOP.picture.holds) say(`   the picture holds ${COOP.picture.holds}; the most men handed to it in a frame ${COOP.picture.most} (tick ${COOP.picture.mostTick}); hand-overs dropped ${COOP.picture.dropped}${COOP.picture.firstDropped ? `, the first in tick ${COOP.picture.firstDropped}` : ''}`);
writeFileSync(`${outDir}/ticks.txt`, `${threw ? k - 1 : k}\n`);
process.exit(0);
