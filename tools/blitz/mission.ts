// THE TYPESCRIPT SIDE OF A WHOLE MISSION, tick by tick, WITHOUT a whole state a tick (a state grows by the frame's boxes: 35 KB a tick, gigabytes a mission).
//   bun run --preload ./tools/blitz/expose.ts tools/blitz/mission.ts <mission>:<seed> --out DIR [--ticks N] [--keep "100,200-210"] [--every N] [--open "559"]
// Run by run.sh. It stands the mission up headless on THE PAGE'S STAND (no TPS_PROFILE word: tools/qa/stand.ts), plays it with tools/qa/playthrough.ts's scripted
// player, and writes into DIR:
//   every tick   h<k>.bin    THE HOST'S HALF as tick k opens: the record `cmd_buf`, every camera and Input row (the bridge's own writers)
//                host.txt    a line: dt, the probe's pending, time, the die's count, how often the host listed the box world since the last tick
//                post.txt    after the tick, a line: `k die_n die_s time <a hash of each part of rust/sim's block, in SIM_PARTS' order>` (line 1 names the parts)
//   tick 1       c1.bin, s0.bin (the state and the block both sides start from)
//   --keep / --every   a<k>.bin, s<k>.bin: the whole state and the block as tick k returns, for diff.ts
//   --open       c<k>.bin: the whole state as tick k opens (`diff.ts a<k-1>.bin c<k>.bin` then shows what the HOST wrote between two ticks)
//   gives.txt        the test driver's orders, given between ticks through the menu's own call: a line for the tick that opens next
//   hostwrites.txt   what else the host writes into the game: `<k> chron_save "<label>"` (the autosave's chronicle row, the tick's last thing), `<k> room_at x y z`
//                    (the driver's own question that floods a room and keeps it)
// What is done to the game so that two sides can be stepped from one start (each a harness matter): every DRESS row is put down before the start (dress.ts says
// that is the same game to the bit); the game has ITS OWN DIE from the first tick, so its word and count are in the block; `sim.settle` is called before the
// start is taken, so every pose is in the pool (the bridge carries no floats of a pose that is not at home).
import '../qa/stand.ts';
import { writeFileSync, mkdirSync, appendFileSync } from 'node:fs';
const { exportGame, exportPart, fieldAt, bookOf } = await import('./host.ts');
const argv = process.argv.slice(2), arg = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv[i + 1]; };
const say = (s = '') => process.stdout.write(s + '\n');
const outDir = arg('--out', ''), TICKS = +arg('--ticks', '1000000'), DIE = +arg('--die', '20942'), EVERY = +arg('--every', '0');
if (!argv[0] || argv[0].startsWith('--') || !outDir) { say('mission.ts <mission>:<seed> --out DIR [--ticks N] [--keep "100,200-210"] [--every N] [--open "559"]'); process.exit(64); }
const ranges = (s: string) => s.split(',').filter(Boolean).map(x => x.split('-').map(Number)).map(([a, b]) => [a, b ?? a]);
const KEEP = ranges(arg('--keep', '')), OPEN = ranges(arg('--open', ''));
const inR = (R: number[][], k: number) => R.some(([a, b]) => k >= a && k <= b), keep = (k: number) => inR(KEEP, k) || (EVERY > 0 && k % EVERY === 0);
const [scn, seedS, frameS] = argv[0].split(':'); const seed = +(seedS ?? 1), frame = +(frameS ?? 1);
mkdirSync(outDir, { recursive: true });
const { knobRegistry } = await import('../../src/game/knobs.ts');
const { Game } = await import('../../src/game/game.ts');
const { play, entryOf } = await import('../qa/playthrough.ts');
// A TUTORIAL MISSION (levels/tut-<name>.json with its level script) is played by its own scripted player, tools/qa/lesson-run.ts, through the input.
// That harness feeds the player's light meter itself before every tick (meter.txt, a line a tick) and picks a row of the nearby list between ticks
// (`<k> select_verb <kind> <verb>` in hostwrites.txt). It is run here WITHOUT its model of the sky's light on the other men's meters (a hook over the game's
// own light asks, which the Rust game has no word for), so a lesson that leans on the moon may stop short of its end: how far it went is said.
const LESSON = scn.startsWith('tut-');
if (!LESSON && !entryOf(scn)) { say(`mission.ts: the scripted player has no mission '${scn}' (tools/qa/playthrough.ts plays the room clears, the rescues and the raids: bun run tools/qa/playthrough.ts --scn x names them)`); process.exit(64); }
const { newDice } = await import('../../src/math/dice.ts');
const { dressAll } = await import('../../src/game/dress.ts');
const { simLayout, SIM_PARTS } = await import('../../src/sim/state.ts');
const { sim } = await import('../../src/sim/sim.ts');
const reg = knobRegistry();
class Stop extends Error {}
const words = (g: any): Uint32Array => g.sim.w;
const block = (g: any): Uint8Array => { const w = words(g); return new Uint8Array(w.buffer, w.byteOffset, w.byteLength).slice(); };
// A HASH OF EACH PART OF THE BLOCK, over its words up to the last that is not zero (a parked block may be shorter than the Rust slot; the zeros say nothing).
// By name, so that the runner can leave one part out of its alarm and say which part parted.
let LAY: { off: number; len: number }[] | null = null;
function hashes(w: Uint32Array): number[] {
  LAY ??= simLayout().parts.map(p => ({ off: p.off >>> 2, len: p.len >>> 2 }));
  return LAY.map(({ off, len }) => { let n = Math.min(off + len, w.length); while (n > off && w[n - 1] === 0) n--; let h = 0x811c9dc5; for (let i = off; i < n; i++) h = Math.imul(h ^ w[i], 0x01000193); return h >>> 0; });
}
let k = 0, n = 0, uploads = 0; const gives: string[] = []; 
for (const f of ['host.txt', 'gives.txt', 'hostwrites.txt', ...(LESSON ? ['meter.txt'] : [])]) writeFileSync(`${outDir}/${f}`, '');
const { SCHEMA } = await import('./export.ts'); const wordAt = (ty: string, w: unknown) => w === undefined ? -1 : SCHEMA.types[ty].words.indexOf(w);
writeFileSync(`${outDir}/post.txt`, `parts ${SIM_PARTS.join(' ')}\n`);
function tapGives(g: any) { const O = g.orders, was = O.give; O.give = function (this: any, id: string, t: any, who: any, opts: any) {
  if (!g.ticking) { const T = t ?? this.targetAt(g.aimPoint), w = who === undefined ? '-' : typeof who === 'object' ? `g${g.guards.indexOf(who)}` : String(who);
    const tail = T.kind === 'door' ? ` door ${g.doors.list.indexOf(T.door)} ${T.side} book${bookOf(g).ids.get('DoorId')?.get(T.door)}` : T.kind === 'none' && T.man ? ` man ${g.guards.indexOf(T.man)}` : T.kind === 'foe' ? ` man ${g.guards.indexOf(T.foe)}` : (T.wall || T.thing) ? ' UNSUPPORTED-wall-or-thing' : '';
    const l = `${k + 1} ${id} ${w} ${T.kind} ${T.at.join(' ')}${tail}${opts ? ' UNSUPPORTED-opts' : ''}`; gives.push(l); appendFileSync(`${outDir}/gives.txt`, l + '\n'); }
  return was.call(this, id, t, who, opts); }; }
function hostHalf(g: any): Uint8Array { const bk: any = bookOf(g), P: [string, any][] = [];
  const cb = fieldAt(g, reg, ['cmd_buf']); P.push([cb.ty, cb.v]);
  for (const [id, ty] of [['CameraId', 'crate::camera::FollowCamera'], ['InputId', 'crate::input::Input']]) { const L: any[] = bk.listOf(id); P.push(['u32', L.length]); for (const row of L) P.push([ty, row]); }
  return exportPart(g, reg, P); }
const was = Game.prototype.update;
Game.prototype.update = function (this: any, ...a: any[]) {
  if (++n < frame) return (was as any).apply(this, a);
  const g = this, step = g.stepSeats; k++;
  if (k === 1) { const W = g.engine.world, up = W.upload; W.upload = function (this: any, ...x: any[]) { uploads++; return up.apply(this, x); }; }
  if (k === 1) tapGives(g);
  // THE TEST DRIVER'S OWN QUESTIONS THAT WRITE THE GAME: `g.fight.roomAt(p)` asked between ticks floods a room nobody had asked for and keeps it in `g.fight.rooms`.
  // Each such ask that grew the list is a line of hostwrites.txt for the tick that opens next.
  if (k === 1 && g.fight?.roomAt) { const F = g.fight, ask = F.roomAt; F.roomAt = function (this: any, p: any) { const n0 = F.rooms.length; const r = ask.call(this, p); if (!g.ticking && F.rooms.length !== n0) appendFileSync(`${outDir}/hostwrites.txt`, `${k + 1} room_at ${p[0]} ${p[1]} ${p[2]}\n`); return r; }; }
  if (k === 1 && LESSON) { const pick = g.selectVerb; g.selectVerb = function (this: any, kind: string, verb?: string) { if (!g.ticking) appendFileSync(`${outDir}/hostwrites.txt`, `${k + 1} select_verb ${wordAt('crate::game::InteractableKind', kind)} ${wordAt('crate::game::Verb', verb)} ${kind} ${verb}\n`); return pick.call(this, kind, verb); }; }
  if (k === 1) dressAll(false);
  if (k === 1 && !g.dice) g.dice = newDice(DIE);
  let err: unknown = null;
  g.stepSeats = function (this: any, dt: number, cmds: unknown) {
    if (k === 1) g.sim.settle(g);
    if (k === 1) { writeFileSync(`${outDir}/c1.bin`, exportGame(g, reg)); writeFileSync(`${outDir}/s0.bin`, block(g));
      // (A LESSON'S TESTS: the bridge does not carry a step's `done`, a record by its `is` word; the runner reads them from this file)
      if (LESSON) writeFileSync(`${outDir}/lesson.json`, JSON.stringify((g.level.scenarios?.lesson?.steps ?? []).map((s: any) => s.done)));
    }
    else if (inR(OPEN, k)) writeFileSync(`${outDir}/c${k}.bin`, exportGame(g, reg));
    writeFileSync(`${outDir}/h${k}.bin`, hostHalf(g));
    if (LESSON) appendFileSync(`${outDir}/meter.txt`, `${g.player.visibility}\n`);
    appendFileSync(`${outDir}/host.txt`, `${dt} ${g.probe?.pending?.length ?? 0} ${g.time} ${g.dice?.n ?? -1} ${k === 1 ? 0 : uploads}\n`); uploads = 0;
    const saves0 = g.chronicle.counts.saves;
    try { return step.call(this, dt, cmds); } finally {
      // THE HOST'S CHECKPOINT: the autosave controller (savestate.ts, the host's) writes a chronicle row and a count as the tick's last thing
      if (g.chronicle.counts.saves !== saves0) { const E = g.chronicle.events, e = E[E.length - 1]; appendFileSync(`${outDir}/hostwrites.txt`, `${k} chron_save ${JSON.stringify(e?.kind === 'checkpoint' ? (e.label ?? e.data?.label ?? '') : 'NOT-LAST')}\n`); }
      appendFileSync(`${outDir}/post.txt`, `${k} ${g.dice?.n ?? -1} ${g.dice?.s ?? -1} ${g.time} ${hashes(words(g)).join(' ')}\n`);
      if (keep(k)) { writeFileSync(`${outDir}/a${k}.bin`, exportGame(g, reg)); writeFileSync(`${outDir}/s${k}.bin`, block(g)); } }
  };
  try { (was as any).apply(this, a); } catch (e) { err = e; } finally { delete g.stepSeats; }
  if (err) { say(`THE TYPESCRIPT TICK ${k} THREW: ${(err as any)?.stack ?? err}`); throw new Stop(); }
  if (k >= TICKS) { say(`TypeScript: ${scn} seed ${seed}, ticks ${frame}..${frame + k - 1}, stand ${process.env.TPS_PROFILE}; time ${g.time.toFixed(4)}; die n=${g.dice?.n}; ${g.guards.length} guards`); throw new Stop(); }
} as any;
const log = console.log, er = console.error, wn = console.warn; let res: any = null;
let les: any = null;
try { if (LESSON) les = await (await import('../qa/lesson-run.ts')).runLesson(scn, { seed, sky: false, glow: false, floor: 0 }); else res = await play(scn, seed, 300, false, {}); } catch (e) { if (!(e instanceof Stop)) { console.log = log; throw e; } } finally { console.log = log; console.error = er; console.warn = wn; }
// (the module's knob table as the TypeScript game wrote it: the runner's check of the generated `knobs_to_rust`)
{ const x: any = sim(), at = x.knob_addr(0), len = x.knob_len(0) * 8; writeFileSync(`${outDir}/knobs.bin`, new Uint8Array(x.memory.buffer).slice(at, at + len)); }
if (les) { for (const l of les.lines.slice(-8)) say('   ' + l); say(`TypeScript: the lesson ${scn} seed ${seed} at tick ${k}: ${les.code === 0 ? 'every step went by and the run ended' : `STOPPED (${les.why})`}; ${les.steps.length} steps gone by: ${les.steps.map((s: any) => s.id).join(' ')}`); }
if (res) say(`TypeScript: ${scn} seed ${seed} PLAYED OUT at tick ${k} on the stand ${process.env.TPS_PROFILE}: ${res.end} (${res.why}); t ${res.t}; team ${res.team} of ${res.of}; hostiles ${res.hostiles} of ${res.hostilesOf}`);
say(`ticks ${k}; the driver's orders ${gives.length}; the die's count ${k ? 'in post.txt' : '-'}; resident ${(process.memoryUsage().rss / 1e9).toFixed(2)} GB`);
writeFileSync(`${outDir}/ticks.txt`, `${k}\n`);
process.exit(0);
