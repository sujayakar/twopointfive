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
if (/^(solo|quiet)-/.test(argv[0])) for (const [k, v] of [['GLASSFX', 'on'], ['TPS_MOVES', '1'], ['TPS_TURN', '1'], ['TPS_CAST', '1'], ['TPS_OPENING', '1'], ['TPS_QRFSILENT', '1'], ['TPS_QRFSQUAD', '1']]) process.env[k] ??= v;   // (tools/qa/solo-night.ts's own stand words, before any game module loads)
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
// A QUIET MISSION OR THE PLAIN SOLO NIGHT (`solo-<plan>:<seed>`, `quiet-<plan>:<seed>`; plan = walk | sneak | careful | rush | loud): played by
// tools/qa/solo-night.ts's scripted Sam, one man on the floor through the input, mortal and seen (his meter fed every frame from the CPU's estimate of
// the light on him: the `meter` line of the record). `quiet-` is the level's quiet mission (`--quiet`, dealt off the seed), `solo-` the plain office
// after dark. BLITZ_SOLO="<more of solo-night.ts's options>" (`--unseen`, `--ghost`, `--god`, `--squad 1`, `--level kit-yard.json`, `--cap 120`).
// That tool is a program, not a function: it is given its own command line and loaded, and its exit is caught. Its stand words are set here before
// any module of the game loads, as it sets them itself.
const SOLO = /^(solo|quiet)-/.test(scn);
// `--wall`: the scripted commander takes a held room by a WALL CHARGE where a partition allows (tools/qa/wallcharge-commander.ts, its word PT_WALL=1): the
// order is given on a wall (a floor target that carries the wall: the gives line ends `wall …`) and the commander BLOWS the charge by the module's own call
// between ticks (`<k> wall_blow <the wall's index>` in hostwrites.txt). The stand is the page's either way.
// (the word itself is set above, before the scripted player's file is read)
// (…and one lesson's own player hooks the same asks itself, tools/qa/lessons/tut-squad.ts: its word for "the estimate as every other tool has it")
if (LESSON) process.env.TUT_SQUAD_SKY = '0';
if (!LESSON && !SOLO && !entryOf(scn)) { say(`mission.ts: the scripted player has no mission '${scn}' (tools/qa/playthrough.ts plays the room clears, the rescues and the raids: bun run tools/qa/playthrough.ts --scn x names them)`); process.exit(64); }
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
// `--hard`: THE PLAYER'S DIFFICULTY AT HARD, as a tally stands it (playthrough.ts `--set '{"DIFFICULTY":{"level":"hard"}}'` → setDifficulty('hard') before the
// game is stood, which lays the PRESSURE and DIGIN tables and the fog's rows; a lesson's player takes `hard`). `--set '<json>'`: any rows the scripted
// player's `--set` takes (a lever alone: '{"DIFFICULTY":{"fog":"<rung>"}}'). The start and knobs.bin carry the tables, so one option serves both sides.
const HARD = argv.includes('--hard'), SET: Record<string, Record<string, unknown>> = HARD ? { DIFFICULTY: { level: 'hard' } } : JSON.parse(arg('--set', '{}'));
// `--lever <key>=<rung>[,…]`: A LEVER MOVED ON THE STANDING GAME, before the first tick (difficulty.ts setLever, as the mission sheet's row moves it): for
// the fourth lever, "you're told" (`told=guided|standard|bare`), which is a lever only once the page's switches are up (TOLD.on, raised as the game is
// stood) and so is not one the scripted player's `--set` can name. The tables it lays are in the start and in knobs.bin.
const LEVER = arg('--lever', '');
class Stop extends Error {}
const CALLS = (process.env.BLITZ_CALL ?? '').split(';').map(x => x.trim()).filter(Boolean).map(x => ({ w: x.split(' '), at: 0, down: 0 })); let shotLast = false, fragSaid = 0;
const await0d: any = LEVER ? await import('../../src/game/difficulty.ts') : null;
const await0 = CALLS.length ? await import('../../src/game/qrf.ts') : (null as any);
let nvSaid = false, nvAfter = false; const noiseSaid = new WeakSet<object>();   // (the driver's own writes of Sam's goggles and of a staged report: said once)
let selAfter = 0;   // the highlighted row of the nearby list as the last tick left it
// A SCRIPTED CASE (`--case pie | orbit | boot`): a write BETWEEN TWO TICKS that the missions' seeds never make, so that a place mended by reading is
// reached by a run. Each is a line of gives.txt (the runner makes the same write through the Rust game's own setter; with BLITZ_CASE_RAW=1 it makes the
// write as the Rust made it BEFORE the mend, to show the run parting without it):
//   pie    a man slicing a doorway on the path he began on has his path replaced by a NEW EMPTY ARRAY and his goal dropped, as frag.ts does when the
//          grenade he fled is gone (`gd.path = []; gd.pathGoal = null`): guards.ts:2025 `gd.path !== P.pathRef`            `<k> !path_new <guard>`
//   orbit  a man backing off in a standoff has his goal written again as ANOTHER ARRAY of the same numbers, as goTo does (`v3.copy`):
//          guards.ts:1163 `gd.pathGoal !== S.orbitGoal`                                                                      `<k> !path_goal_same <guard>`
//   boot   Sam's word (F) twice at once on a level with a breach plan ('go on set'): the pair stacks, then goes in by the boot (breach.ts:220, the loud boot) `<k> !f`
//   keeper a man a press LEFT TO KEEP HIS ROOM, idle and out of contact, hears a fight outside it (the room fight's own write when a loud event reaches
//          him, roomfight.ts:661: `D.noiseAt`, `D.noiseT`, `D.noiseOut`), up to three times a run. With DIGIN.keeper off (`--hard`) he lets it lie
//          (digin.ts:705); with it on he starts to think of digging in. Nothing is written until a press has left such a man: case.txt says how many
//          men were left to keep a room, and after each write what the man did.                                    `<k> !keeper_hears <guard> <x> <y> <z>`
//   sticky the player's tube sends a STICKY CAMERA over the nearest hostile up (3 to 14 m off) by the module's own call (stickycam.ts fireCam), up to
//          three in a run, 300 ticks apart: a stuck camera is an eye of the fog, a wedge about its wall's normal that the fog's cone narrows
//          (stickycam.ts:195; at the page's numbers the cone, 120, is wider than the camera's 110: `--set '{"FOG":{"coneDeg":40}}'` beside the
//          difficulty's hard makes the read count).                                                     `<k> call sticky_fire <from x y z> <target x y z>`
const CASE = arg('--case', ''); let caseN = 0, caseSaw = '', caseAt = -1000, caseMan: any = null;
const caseMods: any = CASE.startsWith('keeper') ? { P: await import('../../src/game/pressure.ts') } : CASE === 'sticky' ? { S: await import('../../src/game/stickycam.ts') } : {};
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
  else if (CASE === 'keeper' || CASE === 'keeper-left') { const top = (globalThis as any).__tpsTop ?? {}, st = top['src/game/pressure.ts']?.STATES?.get(g), ds = top['src/game/digin.ts']?.states?.get(g), F = g.fight; if (!st || !F) return;
    const note = (t: string) => appendFileSync(`${outDir}/case.txt`, `# tick ${k}: ${t}\n`);
    if (caseMan) { const { D, M, t } = caseMan; caseMan = null; note(`the man who heard: phase '${M?.phase}', ${M?.knowT === t ? 'the noise taken as heard' : 'the noise NOT taken'}${M?.phase === 'idle' && M?.knowT === t ? ' (heard, and let lie)' : ''}; DIGIN.keeper ${(knobRegistry() as any).DIGIN.keeper}; still a keeper ${st.stay.has(D)}`); }
    const said = `${st.stay.size} left to keep a room, ${st.sent.size} rooms a man left`; if (said !== caseSaw) { caseSaw = said; note(said); }
    if (caseN >= 3 || k - caseAt < 180 || !ds || g.rescue?.active) return;
    // (`keeper-left`: no press of this run has left a man, so the press's own two writes are made for one: a defender holding a room with a mate of
    //  it up, idle and out of contact, is left to keep it against the room next door (pressure.ts:668 `st.stay.set`, :698 `st.sent.add`))
    if (CASE === 'keeper-left' && !st.stay.size && k >= 300) { const up = (X: any) => X.gd.state !== 'dead' && X.gd.team === 'hostile';
      for (const D of F.defs) { const gd = D.gd, M = ds.men.get(gd); if (!up(D) || D.mode !== 'hold' || !D.room || !D.room.entries.length || !M || M.phase !== 'idle' || g.time - gd.lastSeenT < 3 || g.time < gd.dazzledUntil) continue;
        if (!isNaN(ds.rooms.get(D.room)?.enteredAt ?? NaN) || !F.defs.some((X: any) => X !== D && up(X) && X.room === D.room)) continue;
        const T = F.rooms.find((R: any) => R !== D.room && R.entries.some((e: any) => D.room.entries.some((x: any) => x.door === e.door))) ?? F.rooms.find((R: any) => R !== D.room); if (!T) continue;
        st.stay.set(D, T); st.sent.add(D.room); w(`${k} !keeper_left ${g.guards.indexOf(gd)} ${F.rooms.indexOf(T)}`); caseN--; note(`guard ${g.guards.indexOf(gd)} is left to keep room ${D.room.id} against room ${T.id} (the press's own two writes: no press of this run left a man)`); break; } }
    for (const D of F.defs) { const gd = D.gd, M = ds.men.get(gd); if (gd.state === 'dead' || !M || M.phase !== 'idle' || !st.stay.has(D) || !caseMods.P.pressKeeps(g, D)) continue;
      if (g.time < gd.dazzledUntil || g.time - gd.lastSeenT < 3 || !D.room || !D.room.entries.length || !isNaN(ds.rooms.get(D.room)?.enteredAt ?? NaN)) continue;
      if (D.noiseAt && D.noiseOut && g.time - D.noiseT < 6 && D.noiseT > M.knowT) { note(`guard ${g.guards.indexOf(gd)}, a keeper, has heard a fight by the game's own rules (nothing written)`); caseMan = { D, M, t: D.noiseT }; caseAt = k; return; }
      const p = g.player.char.pos, at = [p[0], p[1], p[2]]; D.noiseAt = at; D.noiseT = g.time; D.noiseOut = true; caseAt = k; caseMan = { D, M, t: g.time };
      w(`${k} !keeper_hears ${g.guards.indexOf(gd)} ${at.join(' ')}`); note(`guard ${g.guards.indexOf(gd)}, left to keep room ${D.room.id}, hears a fight at ${at.map((x: number) => x.toFixed(2)).join(' ')}`); return; } }
  else if (CASE === 'sticky') { const pl = g.player, c = pl.char; if (caseN >= 3 || k < 120 || k - caseAt < 300 || pl.down) return;
    const near = g.guards.filter((x: any) => x.team === 'hostile' && x.state !== 'dead').map((x: any) => ({ x, d: Math.hypot(x.char.pos[0] - c.pos[0], x.char.pos[2] - c.pos[2]) })).filter((o: any) => o.d > 3 && o.d < 14).sort((a: any, b: any) => a.d - b.d)[0]; if (!near) return;
    const from = [c.pos[0], 1.3, c.pos[2]], to = [near.x.char.pos[0], caseMods.S.STICKY.aimY, near.x.char.pos[2]];
    if (FRAG_ASK && fragSaid !== k) { between(`${k} frag_step_off`, 'call'); fragSaid = k; }   // (the scripted player's ask stood before this call)
    caseMods.S.fireCam(g, pl, from, to); caseAt = k; caseN++; const l = `${k} sticky_fire ${from.join(' ')} ${to.join(' ')}`; between(l, 'call'); appendFileSync(`${outDir}/case.txt`, `${l}\n# tick ${k}: a camera over guard ${g.guards.indexOf(near.x)} at ${near.d.toFixed(1)} m; ${pl.cams.length} out; FOG.cone ${(knobRegistry() as any).FOG.cone} coneDeg ${(knobRegistry() as any).FOG.coneDeg}\n`); }
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
const FRAG_ASK = (() => { if (LESSON || SOLO || process.env.PT_STAND) return false;
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
    else if (w === 'qrfcast') { if (g.qrf?.cast && g.qrf.men?.some((x: any) => x.state !== 'dead')) C.at = k; }
    else if (w === 'down') { C.down = g.player.down ? C.down + 1 : 0; if (C.down >= 30) C.at = k; }   // (the player half a second down: with PT_MORTAL=1, the death the scripted player can die)
    else throw new Error(`BLITZ_CALL: a tick is a number, fight, qrf, qrfcast or down, not ${w}`);
    if (C.at === k) {
      if (FRAG_ASK && fragSaid !== k) { between(`${k} frag_step_off`, 'call'); fragSaid = k; }   // (the scripted player's ask stood before this call: Player.step asks, then steps)
      if (rule === 'restart_encounter') g.restartEncounter(); else if (rule === 'reset_guards') g.resetGuards(); else if (rule === 'restart') g.restart();
      else if (rule === 'qrf_panel') (await0.requestQrf)(g, 'panel');
      // (`qrf_stage`: the response team staged as tools/qa/qrf-squad-check.ts stages it: the sandbox's rows written (the team on, 14 s out, four men, no
      //  perimeter, the squad's own code for its rooms), the alarm placed where the player stands, the team asked for. `qrfcast` is then the tick its cast stands.)
      else if (rule === 'qrf_stage') { const T = g.tune, p = g.player.char.pos; T.qrf = true; T.qrfEta = 14; T.qrfSize = 4; T.perimeter = false; T.qrfSquad = true; g.alarm.placed = true; g.alarm.pos = [p[0], p[1], p[2]]; g.alarm.raisedAt = g.time; (await0.requestQrf)(g, 'panel'); }
      // (`qrf_stage_rack`: the same staging with the alarm AT THE MISSION'S RACK, the solo night's server room, whose door is locked: the team's
      //  squad code then stacks on a locked door with the building's keys. The record's line carries the place: `call qrf_stage_at x y z`.)
      else if (rule === 'qrf_stage_rack') { const T = g.tune, p = g.level.mission.rack.front; T.qrf = true; T.qrfEta = 14; T.qrfSize = 4; T.perimeter = false; T.qrfSquad = true; g.alarm.placed = true; g.alarm.pos = [p[0], 0, p[2]]; g.alarm.raisedAt = g.time; (await0.requestQrf)(g, 'panel'); between(`${k} qrf_stage_at ${g.alarm.pos.join(' ')}`, 'call'); }
      else if (rule === 'shift_enter' || rule === 'enter') { const I = a.find((x: any) => x?.pressed instanceof Set && x?.keys instanceof Set); if (!I) throw new Error('BLITZ_CALL shift_enter: no Input among the frame\'s arguments'); if (rule === 'shift_enter') I.keys.add('ShiftLeft'); I.pressed.add('Enter'); }   // (`enter`: Enter alone, the down card's restart)
      else throw new Error(`BLITZ_CALL: no rule ${rule}`);
      if (rule !== 'shift_enter' && rule !== 'enter' && rule !== 'qrf_stage_rack' && C.w[2] !== 'start') between(`${k} ${rule}`, 'call');
      appendFileSync(`${outDir}/call.txt`, `${k} ${C.w.slice(1).join(' ')}; ${g.guards.length} guards, ${g.guards.filter((x: any) => x.team === 'hostile' && x.state === 'alert').length} hostile alert, qrf ${g.qrf?.state ?? '-'} with ${g.qrf?.men?.length ?? 0} men, time ${g.time}\n`); } }
  // THE TEST DRIVER'S OWN QUESTIONS THAT WRITE THE GAME: `g.fight.roomAt(p)` asked between ticks floods a room nobody had asked for and keeps it in `g.fight.rooms`.
  // Each such ask that grew the list is a line of hostwrites.txt for the tick that opens next.
  if (k === 1 && g.fight?.roomAt) { const F = g.fight, ask = F.roomAt; F.roomAt = function (this: any, p: any) { const n0 = F.rooms.length; const r = ask.call(this, p); if (!g.ticking && F.rooms.length !== n0) write1(`${k + 1} room_at ${p[0]} ${p[1]} ${p[2]}`); return r; }; }
  // THE DRIVER'S OWN WRITE OF THE CARRIED COUNT: the scripted commander of `--wall` gives itself a wall charge where the kit issued none
  // (wallcharge-commander.ts `C.setCarried(1)`), between ticks and BEFORE the order it then gives on the wall. It is a line of gives.txt, in the
  // orders' own sequence (`<k> !wall_carried <n | null>`): the order after it is refused without it.
  if (k === 1 && g.wallCharge?.setCarried) { const C = g.wallCharge; for (const f of ['setCarried', 'raiseCarried'] as const) { const was = C[f]; C[f] = function (this: any, n: any) { if (!g.ticking) { const l = `${k + 1} !wall_${f === 'setCarried' ? 'carried' : 'raised'} ${n}`; give1(l); } return was.call(this, n); }; } }
  if (k === 1 && g.wallCharge?.blow) { const C = g.wallCharge, blow = C.blow; C.blow = function (this: any, c?: any, ...r: any[]) { if (!g.ticking && c) write1(`${k + 1} wall_blow ${c.wall.index}`); return arguments.length ? blow.call(this, c, ...r) : blow.call(this); }; }
  if (k === 1 && (LESSON || SOLO)) { const pick = g.selectVerb; g.selectVerb = function (this: any, kind: string, verb?: string) { if (!g.ticking) write1(`${k + 1} select_verb ${wordAt('crate::game::InteractableKind', kind)} ${wordAt('crate::game::Verb', verb)} ${kind} ${verb}`); return pick.call(this, kind, verb); }; }
  // THE SCRIPTED PLAYER'S WORD TO THE HOSTAGES / THE MAN IN HAND, given by the module's own call between ticks (playthrough.ts: `R.ending.word(g.player.char)`
  // once the room is held and men of theirs are still up, `X.word(...)` on a snatch that ends in the van): `<k> rescue_word|snatch_word player|none`.
  if (k === 1) for (const [name, E] of [['rescue_word', g.rescue?.ending], ['snatch_word', g.snatch?.out]] as [string, any][]) if (E?.word) { const word = E.word; E.word = function (this: any, by: any) { if (!g.ticking) write1(`${k + 1} ${name} ${by === g.player.char ? 'player' : by ? 'UNSUPPORTED-man' : 'none'}`); return word.call(this, by); }; }
  // A LESSON'S PLAYER ASKS THE WORDS OF A SET DOOR CHARGE between ticks (tools/qa/lessons/tut-kit.ts, tut-solo.ts: `doorTools.closeWords(c)`, who stands too
  // close). The answer is KEPT, stamped with the time, and answers every ask of the next quarter second (doortools.ts closeAt), the tick's own among
  // them: a call line of the ordered record, `<k> call door_close_words <the charge's place in the set list>`.
  if (k === 1 && g.doorTools?.closeWords) { const D = g.doorTools, cw = D.closeWords; D.closeWords = function (this: any, c: any) { if (!g.ticking) between(`${k + 1} door_close_words ${D.set.indexOf(c)}`, 'call'); return cw.call(this, c); }; }
  if (k === 1 && LEVER) { const Df = await0d; for (const kv of LEVER.split(',')) { const [key, rung] = kv.split('=');
      if (!Df.leversNow().some((L: any) => L.key === key && L.rungs.some((r: any) => r.id === rung))) { say(`mission.ts: --lever knows ${Df.leversNow().map((L: any) => `${L.key}=${L.rungs.map((r: any) => r.id).join('|')}`).join(', ')}, not '${kv}'`); process.exit(64); }
      Df.setLever(key, rung, false); } say(`mission.ts: the levers before tick 1: ${Df.leversLine(true)}; TOLD.rung ${(knobRegistry() as any).TOLD?.rung}`); }
  // THE SCRIPTED PLAYER'S OPTION WORDS (tools/qa/playthrough.ts): PT_BREAKER=sam cuts the mains by hand between ticks (`g.setBlackout(86400, 'hand')`),
  // PT_NV=1 then puts Sam's goggles down (`g.player.nv = true`), PT_NOISE=1 pushes one loud round's report onto the game's events where the team
  // starts. Each is a `call` line of the ordered record (`set_blackout_hand`, `nv_on`, `noise_shot x y z`).
  if (k === 1 && !LESSON && !SOLO) { const sb = g.setBlackout; g.setBlackout = function (this: any, secs: number, cause: string) { if (!g.ticking) { if (secs === 86400 && cause === 'hand') between(`${k + 1} set_blackout_hand`, 'call'); else between(`${k + 1} UNSUPPORTED set_blackout ${secs} ${cause}`, 'call'); } return sb.call(this, secs, cause); }; }
  if (!LESSON && !SOLO && k > 1) { if (g.player.nv && !nvSaid && !nvAfter) { nvSaid = true; between(`${k} nv_on`, 'call'); }
    for (const e of g.events as any[]) if (e?.ptNoise && !noiseSaid.has(e)) { noiseSaid.add(e); between(`${k} noise_shot ${e.pos[0]} ${e.pos[1]} ${e.pos[2]}`, 'call'); } }
  // (BLITZ_HOLD_SAM=<tick>: from that tick on the scripted player's keys are taken off the host's Input before every tick, so the player's man stands where he is
  //  whatever his script wants: a floor nobody walks, for a staged case)
  if (process.env.BLITZ_HOLD_SAM && k >= +process.env.BLITZ_HOLD_SAM) { const I = a.find((x: any) => x?.pressed instanceof Set && x?.keys instanceof Set); if (I) { I.keys.clear(); I.pressed.clear(); } }
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
    if (k > 1 && !LESSON && !SOLO && !CALLS.some(C => C.at === k) && g.player.nearSel !== selAfter) write1(`${k} near_sel ${g.player.nearSel}`);
    if (FRAG_ASK && fragSaid !== k) between(`${k} frag_step_off`, 'call');
    between(`${k} ${g.player.visibility}`, 'meter');
    writeFileSync(`${outDir}/h${k}.bin`, hostHalf(g, reg, cmds as any[]));
    row('meter.txt', meterL, `${g.player.visibility}`);   // (a mission's scripted player writes the meter before every tick too, playthrough.ts:779: the same number every tick, until a restart makes the player anew)
    row('host.txt', hostL, `${dt} ${g.probe?.pending?.length ?? 0} ${g.time} ${g.dice?.n ?? -1} ${k === 1 ? 0 : uploads}`); uploads = 0;
    const chron0 = g.chronicle, saves0 = chron0.counts.saves, loads0 = chron0.counts.loads;
    // (the save controller's CAPTURE asks the whistle for the player's breath, which makes its row: `<k> save_breath` where the row came with this tick's save)
    const breathOf = (globalThis as any).__tpsTop?.['src/game/whistle.ts']?.blowerOf, breath0 = !!breathOf?.has?.(g.player);
    try { return step.call(this, dt, cmds); } finally {
      // THE HOST'S CHECKPOINT: the autosave controller (savestate.ts, the host's) writes a chronicle row and a count as the tick's last thing
      // (a restart inside the tick makes the chronicle anew: its count is then 0 where the old one's was not, and that is no save)
      if (g.chronicle === chron0 ? g.chronicle.counts.saves !== saves0 : g.chronicle.counts.saves > 0) { const E = g.chronicle.events, e = E[E.length - 1]; appendFileSync(`${outDir}/hostwrites.txt`, `${k} chron_save ${JSON.stringify(e?.kind === 'checkpoint' ? (e.label ?? e.data?.label ?? '') : 'NOT-LAST')}\n`); if (!breath0 && breathOf?.has?.(g.player)) appendFileSync(`${outDir}/hostwrites.txt`, `${k} save_breath\n`); }
      // (a FILE save loaded INSIDE the step, savestate.ts loadInStep from player.ts's bare Enter once down: the game is the saved one as this tick
      //  returns. The runner does not compare this tick and takes the next one's opening state as a new start)
      if (g.chronicle.counts.loads > loads0) appendFileSync(`${outDir}/hostwrites.txt`, `${k} load_in_step\n`);
      selAfter = g.player.nearSel; nvAfter = !!g.player.nv;
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
let solo = '';
// HARNESS STAND-IN: A QUIET MISSION'S RUNG OF THE FOG (BLITZ_QUIETFOG=off|team|cones|own). On the page the rung rides the mission's link
// (`quietfog=<rung>`, quietfog.ts quietFogOfQuery) and a bare link plays the default's (QUIETFOG.def, 'team' in the page). solo-night.ts writes its own
// bare link, so the rung is given as THE DEFAULT, held at that value through the stand (which writes the page's default over it): the start then
// lays the same two rows of the fog and hands out the same cameras as the link's word would. What differs from the page: the row QUIETFOG.def itself.
if (SOLO && process.env.BLITZ_QUIETFOG) { const Q = (await import('../../src/game/quietfog.ts')).QUIETFOG, rung = process.env.BLITZ_QUIETFOG; Object.defineProperty(Q, 'def', { get: () => rung, set() { /* held */ }, enumerable: true, configurable: true }); }
try { if (SOLO) { const [kind, plan] = scn.split('-'), was = process.argv, exit = process.exit, lines: string[] = [];
    process.argv = [was[0], 'solo-night.ts', '--trace', String(seed), '--plan', plan, ...(kind === 'quiet' ? ['--quiet'] : []), ...(HARD ? ['--hard'] : []), ...(process.env.BLITZ_SOLO ?? '').split(' ').filter(Boolean)];
    (process as any).exit = () => { throw new Stop(); }; console.log = (...x: unknown[]) => { lines.push(x.join(' ')); };
    try { await import('../qa/solo-night.ts'); } finally { process.argv = was; (process as any).exit = exit; solo = [...lines.filter(l => /QRF|CONTROL|door /.test(l)).slice(0, 30), '…', ...lines.slice(-14)].join('\n   '); } }
  else if (LESSON) les = await (await import('../qa/lesson-run.ts')).runLesson(scn, { seed, sky: false, glow: false, floor: 0, ...(HARD ? { hard: true } : {}) }); else res = await play(scn, seed, 300, false, SET); } catch (e) { if (!(e instanceof Stop)) { console.log = log; throw e; } } finally { console.log = log; console.error = er; console.warn = wn; }
// (the module's knob table as the TypeScript game wrote it: the runner's check of the generated `knobs_to_rust`)
{ const x: any = sim(), at = x.knob_addr(0), len = x.knob_len(0) * 8; writeFileSync(`${outDir}/knobs.bin`, new Uint8Array(x.memory.buffer).slice(at, at + len)); }
if (SOLO) say(`TypeScript: ${scn} seed ${seed} (tools/qa/solo-night.ts ${process.env.BLITZ_SOLO ?? ''}) at tick ${k}; its last lines:\n   ${solo}`);
if (les) { for (const l of les.lines.slice(-8)) say('   ' + l); say(`TypeScript: the lesson ${scn} seed ${seed} at tick ${k}: ${les.code === 0 ? 'every step went by and the run ended' : `STOPPED (${les.why})`}; ${les.steps.length} steps gone by: ${les.steps.map((s: any) => s.id).join(' ')}`); }
if (res) say(`TypeScript: ${scn} seed ${seed} PLAYED OUT at tick ${k} on the stand ${process.env.TPS_PROFILE}: ${res.end} (${res.why}); t ${res.t}; team ${res.team} of ${res.of}; hostiles ${res.hostiles} of ${res.hostilesOf}`);
for (const C of CALLS) say(`BLITZ_CALL ${C.w.join(' ')}: ${C.at ? `before tick ${C.at}` : 'NEVER REACHED (no such tick in this run)'}`);
say(`ticks ${k}; the driver's orders ${gives.length}; the die's count ${k ? 'in post.txt' : '-'}; resident ${(process.memoryUsage().rss / 1e9).toFixed(2)} GB`);
writeFileSync(`${outDir}/ticks.txt`, `${k}\n`);
process.exit(0);
