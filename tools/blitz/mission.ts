// THE TYPESCRIPT SIDE OF A WHOLE MISSION, tick by tick, WITHOUT a whole state a tick (a state grows by the frame's boxes: 35 KB a tick, gigabytes a mission).
//   bun run --preload ./tools/blitz/expose.ts tools/blitz/mission.ts <mission>:<seed> --out DIR [--ticks N] [--keep "100,200-210"] [--every N] [--open "559"] [--wall] [--dress all|rows]
// Run by run.sh. It stands the mission up headless on THE PAGE'S STAND (no TPS_PROFILE word: tools/qa/stand.ts), plays it with tools/qa/playthrough.ts's scripted
// player, and writes into DIR:
//   every tick   h<k>.bin    THE HOST'S HALF as tick k opens: a record a seat, every camera and Input row (the bridge's own writer, export.ts hostHalf)
//                host.txt    a line: dt, the probe's pending, time, the die's count, how often the host listed the box world since the last tick
//                post.txt    after the tick, a line: `k die_n die_s time <a hash of each part of rust/sim's block, in SIM_PARTS' order>` (line 1 names the parts)
//   tick 1       c1.bin, s0.bin (the state and the block both sides start from)
//   --keep / --every   a<k>.bin, s<k>.bin: the whole state and the block as tick k returns, for diff.ts
//   --open       c<k>.bin: the whole state as tick k opens (`diff.ts a<k-1>.bin c<k>.bin` then shows what the HOST wrote between two ticks)
//   charges.txt      a line whenever the number of SET wall charges changed in a tick: `<k> <how many are set>` (where to keep every tick: a set
//                    charge's boxes are on the frame's list and a blown one's flash is warmed, blastLook.ts)
//   gives.txt        the test driver's orders, given between ticks through the menu's own call: a line for the tick that opens next
//   between.txt      ONE ORDERED RECORD of the driver's orders, writes and calls of a rule before each tick, in the order it made them (below, `between`)
//   hostwrites.txt   what else the host writes into the game: `<k> chron_save "<label>"` (the autosave's chronicle row, the tick's last thing), `<k> chron_load`
//                    (a checkpoint put back before tick k opened: the state as it opens in L<k>.bin, the block in Ls<k>.bin), `<k> room_at x y z`
//                    (the driver's own question that floods a room and keeps it)
// What is done to the game so that two sides can be stepped from one start (each a harness matter): every DRESS row is put down before the start (dress.ts says
// that is the same game to the bit) unless `--dress` raises them; the game has ITS OWN DIE from the first tick, so its word and count are in the block; `sim.settle` is called before the
// start is taken, so every pose is in the pool (the bridge carries no floats of a pose that is not at home).
import '../qa/stand.ts';
import { writeFileSync, mkdirSync, appendFileSync, readFileSync } from 'node:fs';
const { exportGame, bookOf } = await import('./host.ts');
const { hostHalf } = await import('./export.ts');
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
// `--die tool`: the die a tally rolls for this mission and seed (tools/qa/playthrough.ts under the harness's word, PLAY.md): the stand is the word's own
// (`TPS_GAME=ts`: one draw of the scripted player's seeded Math.random as the game is stood, the dress rows down), so the scripted player's own draws
// after it are the tally's too and the game played here is the very game the tally played. The die is written to die.txt.
if (arg('--die', '') === 'tool') { process.env.TPS_GAME = 'ts'; process.env.RH_DIE_SAY = `${outDir}/die.txt`; }
if (argv.includes('--wall')) process.env.PT_WALL = '1';   // (`--wall`, below; playthrough.ts reads the word as it loads)
const { play, entryOf } = await import('../qa/playthrough.ts');
// A TUTORIAL MISSION (levels/tut-<name>.json with its level script) is played by its own scripted player, tools/qa/lesson-run.ts, through the input.
// That harness feeds the player's light meter itself before every tick (meter.txt, a line a tick) and picks a row of the nearby list between ticks
// (`<k> select_verb <kind> <verb>` in hostwrites.txt). It is run here WITHOUT its model of the sky's light on the other men's meters (a hook over the game's
// own light asks, which the Rust game has no word for), so a lesson that leans on the moon may stop short of its end: how far it went is said.
const LESSON = scn.startsWith('tut-');
// `--wall`: the scripted commander takes a held room by a WALL CHARGE where a partition allows (tools/qa/wallcharge-commander.ts, its word PT_WALL=1): the
// order is given on a wall (a floor target that carries the wall: the gives line ends `wall …`) and the commander BLOWS the charge by the module's own call
// between ticks (`<k> wall_blow <the wall's index>` in hostwrites.txt). The stand is the page's either way.
// (the word itself is set above, before the scripted player's file is read)
// (…and one lesson's own player hooks the same asks itself, tools/qa/lessons/tut-squad.ts: its word for "the estimate as every other tool has it")
if (LESSON) process.env.TUT_SQUAD_SKY = '0';
if (!LESSON && !entryOf(scn)) { say(`mission.ts: the scripted player has no mission '${scn}' (tools/qa/playthrough.ts plays the room clears, the rescues and the raids: bun run tools/qa/playthrough.ts --scn x names them)`); process.exit(64); }
const { newDice } = await import('../../src/math/dice.ts');
const { dressAll, DRESS } = await import('../../src/game/dress.ts');
const { simLayout, SIM_PARTS } = await import('../../src/sim/state.ts');
const { sim } = await import('../../src/sim/sim.ts');
const reg = knobRegistry();
// `--dress <all | row,row,…>`: THE DRESSING ROWS AS THE PAGE HAS THEM (dress.ts DRESS, every row up in the page), or only the rows named, up from the first
// tick; without the word every row is down on both sides, the stand of a host that draws nothing. The bridge carries the table, so the Rust game
// reads the same rows. What a row's step does on a stand with no GPU is README's "The dressing rows up".
const DRESS_WORD = arg('--dress', '');
function dressStand() { dressAll(false); if (!DRESS_WORD) return;
  if (DRESS_WORD === 'all') { dressAll(true); return; }
  for (const r of DRESS_WORD.split(',')) { if (!(r in DRESS)) { say(`mission.ts: --dress knows ${Object.keys(DRESS).join(', ')}, not '${r}'`); process.exit(64); } (DRESS as any)[r] = true; } }
class Stop extends Error {}
const CALLS = (process.env.BLITZ_CALL ?? '').split(';').map(x => x.trim()).filter(Boolean).map(x => ({ w: x.split(' '), at: 0, down: 0 })); let shotLast = false, fragSaid = 0;
const await0 = CALLS.length ? await import('../../src/game/qrf.ts') : (null as any);
let selAfter = 0;   // the highlighted row of the nearby list as the last tick left it
// A SCRIPTED CASE (`--case pie | orbit | boot`): a write BETWEEN TWO TICKS that the missions' seeds never make, so that a place mended by reading is
// reached by a run. Each is a line of gives.txt (the runner makes the same write through the Rust game's own setter; with BLITZ_CASE_RAW=1 it makes the
// write as the Rust made it BEFORE the mend, to show the run parting without it):
//   pie    a man slicing a doorway on the path he began on has his path replaced by a NEW EMPTY ARRAY and his goal dropped, as frag.ts does when the
//          grenade he fled is gone (`gd.path = []; gd.pathGoal = null`): guards.ts:2025 `gd.path !== P.pathRef`            `<k> !path_new <guard>`
//   orbit  a man backing off in a standoff has his goal written again as ANOTHER ARRAY of the same numbers, as goTo does (`v3.copy`):
//          guards.ts:1163 `gd.pathGoal !== S.orbitGoal`                                                                      `<k> !path_goal_same <guard>`
//   boot   Sam's word (F) twice at once on a level with a breach plan ('go on set'): the pair stacks, then goes in by the boot (breach.ts:220, the loud boot) `<k> !f`
const CASE = arg('--case', ''); let caseN = 0, caseSaw = '';
function caseStep(g: any) {
  if (!CASE) return;
  const w = (l: string) => { appendFileSync(`${outDir}/gives.txt`, l + '\n'); between(l, 'give'); appendFileSync(`${outDir}/case.txt`, l + '\n'); caseN++; };
  const alive = (gd: any) => gd.state !== 'dead';
  if (CASE === 'pie' && caseN < 1) { const i = g.guards.findIndex((gd: any) => alive(gd) && gd.pieing && gd.pieing.phase !== 'cross' && gd.pieing.pathRef === gd.path && gd.path.length > 0);
    if (i >= 0) { const gd = g.guards[i]; gd.path = []; gd.pathGoal = null; w(`${k} !path_new ${i}`); } }
  else if (CASE === 'orbit' && caseN < 1) { const i = g.guards.findIndex((gd: any) => alive(gd) && gd.standoff?.orbitGoal && gd.pathGoal === gd.standoff.orbitGoal);
    if (i >= 0) { const gd = g.guards[i]; gd.pathGoal = [gd.pathGoal[0], gd.pathGoal[1], gd.pathGoal[2]]; w(`${k} !path_goal_same ${i}`); } }
  else if (CASE === 'boot') { const N = g.nightBreach; if (N && N.state !== caseSaw) { caseSaw = N.state; appendFileSync(`${outDir}/case.txt`, `# tick ${k}: the drill is '${caseSaw}'\n`); } if (caseN >= 2) return;
    if (N?.active && N.drill && k > 30) { if (N.order()) w(`${k} !f`); } }   // (the second word at once: 'go on set', before the scripted player's first order of its own calls the pair off)
}
const words = (g: any): Uint32Array => g.sim.w;
const block = (g: any): Uint8Array => { const w = words(g); return new Uint8Array(w.buffer, w.byteOffset, w.byteLength).slice(); };
// A HASH OF EACH PART OF THE BLOCK, over its words up to the last that is not zero (a parked block may be shorter than the Rust slot; the zeros say nothing).
// By name, so that the runner can leave one part out of its alarm and say which part parted.
let LAY: { off: number; len: number }[] | null = null;
function hashes(w: Uint32Array): number[] {
  LAY ??= simLayout().parts.map(p => ({ off: p.off >>> 2, len: p.len >>> 2 }));
  return LAY.map(({ off, len }) => { let n = Math.min(off + len, w.length); while (n > off && w[n - 1] === 0) n--; let h = 0x811c9dc5; for (let i = off; i < n; i++) h = Math.imul(h ^ w[i], 0x01000193); return h >>> 0; });
}
let k = 0, n = 0, uploads = 0, loadsSeen = 0, chargesSet = 0, chronSeen: any = null; const gives: string[] = [];
for (const f of ['host.txt', 'gives.txt', 'hostwrites.txt', 'charges.txt', 'meter.txt', 'between.txt']) writeFileSync(`${outDir}/${f}`, '');
// ONE ORDERED RECORD OF WHAT THE DRIVER DID BETWEEN TWO TICKS (between.txt): every order, every write and every call of a rule the scripted player made
// before tick k, IN THE ORDER IT MADE THEM, a line each: `<k> give <a gives.txt line's rest>` · `<k> write <a hostwrites.txt line's rest>` (the lines a
// stepper replays BEFORE the tick: not the autosave's, not a checkpoint put back) · `<k> call <rule>` · `<k> meter <v>`. gives.txt and hostwrites.txt
// are written as before (a stepper that reads those two replays the orders first, then the writes); a stepper that reads this file replays the lines of a
// tick as they stand and nothing else before the tick.
const between = (l: string, kind: string) => { const i = l.indexOf(' '); appendFileSync(`${outDir}/between.txt`, `${l.slice(0, i)} ${kind}${l.slice(i)}\n`); };
const give1 = (l: string) => { gives.push(l); appendFileSync(`${outDir}/gives.txt`, l + '\n'); between(l, 'give'); };
const write1 = (l: string) => { appendFileSync(`${outDir}/hostwrites.txt`, l + '\n'); between(l, 'write'); };
// THE SCRIPTED PLAYER'S OWN ASK BEFORE EVERY STEP (tools/qa/playthrough.ts Player.step: `fragStepOff(g)`, where a player who read a grenade's cue would
// go). The ask WRITES: it makes the flee's guess of where the grenade comes to rest (frag.ts restOf, stamped with the time; kept 0.1 s, a settled one for
// good) and fills the memo of the walls its burst would open (wallsFor), both of which the next tick's own flee reads. It is a function of a module, not
// a method of an object, so there is nothing to hang a tap on: the line is written for every tick the scripted player steps (it asks before every step
// unless PT_STAND), and the driver's source is read once to see that the ask still stands where this was read.
const FRAG_ASK = (() => { if (LESSON || process.env.PT_STAND) return false;
  const src = readFileSync(new URL('../qa/playthrough.ts', import.meta.url), 'utf8'), asks = src.split('fragStepOff(g)').length - 1, steps = src.split('W.step(DT)').length - 1;
  if (asks !== 1 || steps !== 3 || !src.includes('const clear = STAND ? null : fragStepOff(g)')) { say(`mission.ts: the scripted player's step is not as it was read (fragStepOff(g) x${asks}, W.step(DT) x${steps}): read its calls between ticks again (between.txt)`); process.exit(3); }
  return true; })();
const { SCHEMA } = await import('./export.ts'); const wordAt = (ty: string, w: unknown) => w === undefined ? -1 : SCHEMA.types[ty].words.indexOf(w);
writeFileSync(`${outDir}/post.txt`, `parts ${SIM_PARTS.join(' ')}\n`);
// A LINE A TICK of host.txt and post.txt, by the tick's number. The host can step the game TWICE in one frame (a checkpoint put back: the save controller
// loads, steps, loads again and steps again); the step that stands is the last, so a second line for the same tick takes the first one's place.
const hostL: string[] = [], postL: string[] = [], meterL: string[] = [];
function row(file: string, L: string[], line: string) { const head = file === 'post.txt' ? `parts ${SIM_PARTS.join(' ')}\n` : '';
  if (L.length >= k) { L[k - 1] = line; writeFileSync(`${outDir}/${file}`, head + L.join('\n') + '\n'); } else { L.push(line); appendFileSync(`${outDir}/${file}`, line + '\n'); } }
/** a floor target's wall, as a gives line carries it: ` wall <index> <the box's id in the book> <alongX> <u> <side> <ok> <outer: -1 none> <why, as hex>` */
const wallTail = (g: any, W: any) => ` wall ${W.index} ${bookOf(g).ids.get('BoxId')?.get(W.box)} ${W.alongX ? 1 : 0} ${W.u} ${W.side} ${W.ok ? 1 : 0} ${W.outer === undefined ? -1 : W.outer ? 1 : 0} x${Buffer.from(String(W.why ?? ''), 'utf8').toString('hex')}`;
function tapGives(g: any) { const O = g.orders, was = O.give; O.give = function (this: any, id: string, t: any, who: any, opts: any) {
  if (!g.ticking) { const T = t ?? this.targetAt(g.aimPoint), w = who === undefined ? '-' : typeof who === 'object' ? `g${g.guards.indexOf(who)}` : String(who);
    const tail = T.kind === 'door' ? ` door ${g.doors.list.indexOf(T.door)} ${T.side} book${bookOf(g).ids.get('DoorId')?.get(T.door)}` : T.kind === 'none' && T.man ? ` man ${g.guards.indexOf(T.man)}` : T.kind === 'foe' ? ` man ${g.guards.indexOf(T.foe)}` : T.thing ? ' UNSUPPORTED-thing' : T.wall ? wallTail(g, T.wall) : '';
    const l = `${k + 1} ${id} ${w} ${T.kind} ${T.at.join(' ')}${tail}${opts ? ' UNSUPPORTED-opts' : ''}`; give1(l); }
  return was.call(this, id, t, who, opts); }; }
const was = Game.prototype.update;
Game.prototype.update = function (this: any, ...a: any[]) {
  if (++n < frame) return (was as any).apply(this, a);
  const g = this, step = g.stepSeats; k++;
  if (k === 1) { const W = g.engine.world, up = W.upload; W.upload = function (this: any, ...x: any[]) { uploads++; return up.apply(this, x); }; }
  if (k === 1) tapGives(g);
  // BLITZ_CALL="<k> restart_encounter|reset_guards [start]": the rule called before tick k on BOTH sides (the runner reads the same word), to hold the
  // Rust game's restart to the TypeScript's. With `start` the Rust game does not call its own rule: it takes the TypeScript's state after the call as a
  // NEW START (the way a host that plays the Rust game follows a rule a tool called on its puppet). The scripted player knows nothing of it and plays
  // on with the men it knew: a run for the ticks after the call.
  // The tick may be a WORD, found as the run goes: `fight` = the first tick after one in which a round was fired with a hostile alert (a restart
  // MID-FIGHT), `qrf` = the first tick with the response team's cast deployed and a man of it up (qrf.ts `men`). The rule `shift_enter` is no call: the
  // PLAYER'S OWN RESTART (Shift+Enter, player.ts → Game.restart) pressed on the host's Input before the tick, so each side restarts inside its own tick.
  // The call is a line of between.txt (`<k> call <rule>`: the runner makes it there, in the driver's order); the tick it fell on is said at the end.
  // Several calls in one run: `BLITZ_CALL="300 qrf_panel; qrf restart_encounter"` (a `;` between them), each on its own tick. `qrf_panel` asks for the
  // response team as the sandbox's panel does (qrf.ts requestQrf 'panel').
  for (const C of CALLS) { if (C.at) continue; const w = C.w[0], rule = C.w[1];
    if (/^\d+$/.test(w)) { if (k === +w) C.at = k; }
    else if (w === 'fight') { if (k > 60 && shotLast && g.guards.some((x: any) => x.team === 'hostile' && x.state === 'alert')) C.at = k; }
    else if (w === 'qrf') { if (g.qrf?.men?.some((x: any) => x.state !== 'dead' && g.guards.includes(x))) C.at = k; }
    else if (w === 'down') { C.down = g.player.down ? C.down + 1 : 0; if (C.down >= 30) C.at = k; }   // (the player half a second down: with PT_MORTAL=1, the death the scripted player can die)
    else throw new Error(`BLITZ_CALL: a tick is a number, fight, qrf or down, not ${w}`);
    if (C.at === k) {
      if (FRAG_ASK && fragSaid !== k) { between(`${k} frag_step_off`, 'call'); fragSaid = k; }   // (the scripted player's ask stood before this call: Player.step asks, then steps)
      if (rule === 'restart_encounter') g.restartEncounter(); else if (rule === 'reset_guards') g.resetGuards(); else if (rule === 'restart') g.restart();
      else if (rule === 'qrf_panel') (await0.requestQrf)(g, 'panel');
      else if (rule === 'shift_enter' || rule === 'enter') { const I = a.find((x: any) => x?.pressed instanceof Set && x?.keys instanceof Set); if (!I) throw new Error('BLITZ_CALL shift_enter: no Input among the frame\'s arguments'); if (rule === 'shift_enter') I.keys.add('ShiftLeft'); I.pressed.add('Enter'); }   // (`enter`: Enter alone, the down card's restart)
      else throw new Error(`BLITZ_CALL: no rule ${rule}`);
      if (rule !== 'shift_enter' && rule !== 'enter' && C.w[2] !== 'start') between(`${k} ${rule}`, 'call');
      appendFileSync(`${outDir}/call.txt`, `${k} ${C.w.slice(1).join(' ')}; ${g.guards.length} guards, ${g.guards.filter((x: any) => x.team === 'hostile' && x.state === 'alert').length} hostile alert, qrf ${g.qrf?.state ?? '-'} with ${g.qrf?.men?.length ?? 0} men, time ${g.time}\n`); } }
  // THE TEST DRIVER'S OWN QUESTIONS THAT WRITE THE GAME: `g.fight.roomAt(p)` asked between ticks floods a room nobody had asked for and keeps it in `g.fight.rooms`.
  // Each such ask that grew the list is a line of hostwrites.txt for the tick that opens next.
  if (k === 1 && g.fight?.roomAt) { const F = g.fight, ask = F.roomAt; F.roomAt = function (this: any, p: any) { const n0 = F.rooms.length; const r = ask.call(this, p); if (!g.ticking && F.rooms.length !== n0) write1(`${k + 1} room_at ${p[0]} ${p[1]} ${p[2]}`); return r; }; }
  // THE DRIVER'S OWN WRITE OF THE CARRIED COUNT: the scripted commander of `--wall` gives itself a wall charge where the kit issued none
  // (wallcharge-commander.ts `C.setCarried(1)`), between ticks and BEFORE the order it then gives on the wall. It is a line of gives.txt, in the
  // orders' own sequence (`<k> !wall_carried <n | null>`): the order after it is refused without it.
  if (k === 1 && g.wallCharge?.setCarried) { const C = g.wallCharge; for (const f of ['setCarried', 'raiseCarried'] as const) { const was = C[f]; C[f] = function (this: any, n: any) { if (!g.ticking) { const l = `${k + 1} !wall_${f === 'setCarried' ? 'carried' : 'raised'} ${n}`; give1(l); } return was.call(this, n); }; } }
  if (k === 1 && g.wallCharge?.blow) { const C = g.wallCharge, blow = C.blow; C.blow = function (this: any, c?: any, ...r: any[]) { if (!g.ticking && c) write1(`${k + 1} wall_blow ${c.wall.index}`); return arguments.length ? blow.call(this, c, ...r) : blow.call(this); }; }
  if (k === 1 && LESSON) { const pick = g.selectVerb; g.selectVerb = function (this: any, kind: string, verb?: string) { if (!g.ticking) write1(`${k + 1} select_verb ${wordAt('crate::game::InteractableKind', kind)} ${wordAt('crate::game::Verb', verb)} ${kind} ${verb}`); return pick.call(this, kind, verb); }; }
  // THE SCRIPTED PLAYER'S WORD TO THE HOSTAGES / THE MAN IN HAND, given by the module's own call between ticks (playthrough.ts: `R.ending.word(g.player.char)`
  // once the room is held and men of theirs are still up, `X.word(...)` on a snatch that ends in the van): `<k> rescue_word|snatch_word player|none`.
  if (k === 1) for (const [name, E] of [['rescue_word', g.rescue?.ending], ['snatch_word', g.snatch?.out]] as [string, any][]) if (E?.word) { const word = E.word; E.word = function (this: any, by: any) { if (!g.ticking) write1(`${k + 1} ${name} ${by === g.player.char ? 'player' : by ? 'UNSUPPORTED-man' : 'none'}`); return word.call(this, by); }; }
  // A LESSON'S PLAYER ASKS THE WORDS OF A SET DOOR CHARGE between ticks (tools/qa/lessons/tut-kit.ts, tut-solo.ts: `doorTools.closeWords(c)`, who stands too
  // close). The answer is KEPT, stamped with the time, and answers every ask of the next quarter second (doortools.ts closeAt), the tick's own among
  // them: a call line of the ordered record, `<k> call door_close_words <the charge's place in the set list>`.
  if (k === 1 && g.doorTools?.closeWords) { const D = g.doorTools, cw = D.closeWords; D.closeWords = function (this: any, c: any) { if (!g.ticking) between(`${k + 1} door_close_words ${D.set.indexOf(c)}`, 'call'); return cw.call(this, c); }; }
  if (k === 1) dressStand();
  caseStep(g);
  if (k === 1 && (!g.dice || /^\d+$/.test(arg('--die', '')))) g.dice = newDice(DIE);   // (`--die N` seeds the die of a game stood WITH one too: a tutorial's)
  let err: unknown = null;
  g.stepSeats = function (this: any, dt: number, cmds: unknown) {
    if (k === 1) g.sim.settle(g);
    if (k === 1) { writeFileSync(`${outDir}/c1.bin`, exportGame(g, reg)); writeFileSync(`${outDir}/s0.bin`, block(g)); }
    else if (inR(OPEN, k)) writeFileSync(`${outDir}/c${k}.bin`, exportGame(g, reg));
    // THE HOST PUT A CHECKPOINT BACK between the last tick and this one (savestate.ts, the host's save controller: a lesson's player went down and the
    // rails asked for it): the whole game is the saved one again. No stepper can follow that from its own state, so the state and the block as THIS tick
    // opens are written (L<k>.bin, Ls<k>.bin) and the runner takes them as a new start before it steps tick k.
    if (g.chronicle !== chronSeen) { chronSeen = g.chronicle; if (g.chronicle.counts.loads === 0) loadsSeen = 0; }   // (a chronicle made anew by a restart counts from nothing: no checkpoint was put back)
    if (g.chronicle.counts.loads !== loadsSeen || CALLS.some(C => C.w[2] === 'start' && C.at === k)) { loadsSeen = g.chronicle.counts.loads; if (k > 1) { appendFileSync(`${outDir}/hostwrites.txt`, `${k} chron_load\n`); writeFileSync(`${outDir}/L${k}.bin`, exportGame(g, reg)); writeFileSync(`${outDir}/Ls${k}.bin`, block(g)); } }
    // THE SCRIPTED PLAYER PUT THE HIGHLIGHT ON ANOTHER ROW of the nearby list between the last tick and this one (playthrough.ts:865 writes
    // `g.player.nearSel` and calls pickHover for a door's own row; `g.selectVerb` does the same by kind and verb): `<k> near_sel <row>`, unless the
    // pick was written as a lesson's `select_verb` already.
    if (k > 1 && !LESSON && !CALLS.some(C => C.at === k) && g.player.nearSel !== selAfter) write1(`${k} near_sel ${g.player.nearSel}`);
    if (FRAG_ASK && fragSaid !== k) between(`${k} frag_step_off`, 'call');
    between(`${k} ${g.player.visibility}`, 'meter');
    writeFileSync(`${outDir}/h${k}.bin`, hostHalf(g, reg, cmds as any[]));
    row('meter.txt', meterL, `${g.player.visibility}`);   // (a mission's scripted player writes the meter before every tick too, playthrough.ts:779: the same number every tick, until a restart makes the player anew)
    row('host.txt', hostL, `${dt} ${g.probe?.pending?.length ?? 0} ${g.time} ${g.dice?.n ?? -1} ${k === 1 ? 0 : uploads}`); uploads = 0;
    const chron0 = g.chronicle, saves0 = chron0.counts.saves;
    try { return step.call(this, dt, cmds); } finally {
      // THE HOST'S CHECKPOINT: the autosave controller (savestate.ts, the host's) writes a chronicle row and a count as the tick's last thing
      // (a restart inside the tick makes the chronicle anew: its count is then 0 where the old one's was not, and that is no save)
      if (g.chronicle === chron0 ? g.chronicle.counts.saves !== saves0 : g.chronicle.counts.saves > 0) { const E = g.chronicle.events, e = E[E.length - 1]; appendFileSync(`${outDir}/hostwrites.txt`, `${k} chron_save ${JSON.stringify(e?.kind === 'checkpoint' ? (e.label ?? e.data?.label ?? '') : 'NOT-LAST')}\n`); }
      selAfter = g.player.nearSel;
      shotLast = (g.events as any[]).some((e: any) => (e?.kind === 'shot' || e?.kind === 'guardShot') && e.from && e.time >= g.time - 0.05);   // (a round fired in the tick just run: the events are kept half a second)
      row('post.txt', postL, `${k} ${g.dice?.n ?? -1} ${g.dice?.s ?? -1} ${g.time} ${hashes(words(g)).join(' ')}`);
      { const set = (g.seats?.length ? g.seats : [{ view: g }]).reduce((x: number, st: any) => x + (st.view?.wallCharge?.set?.length ?? 0), 0); if (set !== chargesSet) { chargesSet = set; appendFileSync(`${outDir}/charges.txt`, `${k} ${set}\n`); } }
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
for (const C of CALLS) say(`BLITZ_CALL ${C.w.join(' ')}: ${C.at ? `before tick ${C.at}` : 'NEVER REACHED (no such tick in this run)'}`);
say(`ticks ${k}; the driver's orders ${gives.length}; the die's count ${k ? 'in post.txt' : '-'}; resident ${(process.memoryUsage().rss / 1e9).toFixed(2)} GB`);
writeFileSync(`${outDir}/ticks.txt`, `${k}\n`);
process.exit(0);
