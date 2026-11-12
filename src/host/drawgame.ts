// A GAME IN THE MODULE, AS A PAGE THAT DRAWS IT HAS IT: started, handed a player's input, stepped, and read by what it leaves to be drawn (src/draw). The page
// holds its number and nothing of it.
import type { HandsOffPlan } from './benchScenes';
import { FaceError, type Fetched } from './face';
import { Drawn, drawRefused } from '../draw/drawn';
import { Face, HOLDS, type GameNo, type MomentNo } from '../draw/face';
import { Heard, Man, Section, View, Want } from '../draw/sections';
import { Commands } from '../draw/commands';
import { newHand, type Hand } from '../draw/hand';
import { cursorNow, LeadNow, leadNow, type Doubles } from '../draw/input';
import { HandTape, MARKS, hashOf, type PartOf, type TapeFigures, type TapeMark, type TapeRow } from '../tape/handtape';
import { sampleCursor, type CursorAt, type CursorCanvas, type CursorLens } from '../view/cursor';
import type { ActionRow, CursorView, DebriefView, EndView, FloorView, HandsSaid, KnobRow, MapView, OutcomeView, RunView, SavesTold, SavesView, ShownRow, Tally, TickView } from '../view/read';

/** what a start is handed beside the level (rust/game_start/src/handed.rs) */
export interface Start { level: Uint8Array; words: string; mannequin: Uint8Array; mannequinBin: Uint8Array; cast: Uint8Array; castLook: Uint8Array; seed: number; seats?: number; /** what the machine keeps, as its one text (empty: nothing) */ kept?: string; /** what a tape of the run says it is of */ named?: { level: string; build: string } }
/** the parts of a start that are the build's own, as a tape holds a walk's to its */
export const partsOf = (o: Start): PartOf[] => (['mannequin', 'mannequinBin', 'cast', 'castLook'] as const).map(name => ({ name, length: o[name].length, hash: hashOf(o[name]) }));

/** `face_start`'s: the rows that dress the picture are up */
const DRESS = 1, /** … and what `ready` makes is this page's to make */ LATER = 2;
/** `face_step`'s: the save controller */
const SAVES = 8;

/** lines of the light's table to a call (a millisecond or two), and how long calls go on between two turns of the event loop (ms) */
const READY = 512, READY_MS = 8;
/** bytes a tape may be: it ends there (an hour and a half and more of play) */
const TAPE_MOST = 48 * 1048576;
/** the game of a call was lost to a panic in it: its words, the call, and the tick */
export class GameLost extends Error { constructor(readonly words: string, readonly what: string, readonly tick: number) { super(`the game stopped in ${what}, at tick ${tick}: ${words}`); this.name = 'GameLost'; } }

export class DrawGame {
  /** the module */
  readonly face: Face;
  readonly drawn: Drawn;
  h = 0 as GameNo;
  /** what a player or a panel does that is not input: the next step takes it */
  readonly commands = new Commands();
  /** WHAT THE MODULE IS HANDED, kept as it is handed (put here before a start: that start is its first row); null: nobody keeps it */
  tape: HandTape | null = null;
  /** the last tape that ended: a run's lost to a panic, or past what is kept */
  ended: HandTape | null = null;
  /** ticks stepped since the start, and ms of the last one by part */
  ticks = 0;
  /** moves with everything done to the game: what was asked of it before is asked again */
  private done = 0;
  readonly ms = { step: 0, draw: 0 };
  private readonly hand = newHand();

  constructor(f: Fetched) {
    // (a trap, of a panic or of the stack run out, loses the game of that call, and nothing else of it is asked until it stands again)
    this.face = new Face(f.exports, { before: call => { if (this.lost && HOLDS.has(call)) throw this.lost; }, after: () => this.drawn?.fresh(),
      trapped: (words, call) => { this.lost = new GameLost(words, call, this.ticks); this.tape?.end(this.lost.message); this.ended = this.tape ?? this.ended; this.tape = null; return this.lost; } });
    const no = drawRefused(this.face); if (no) throw new FaceError(`${no} (tools/blitz/face-build.ts builds this checkout's)`);
    this.drawn = new Drawn(this.face, Want.all);
  }

  /** the panic the game was lost to, until it stands again */
  lost: GameLost | null = null;
  /** what a lost game would stand again from: a save's label, '' its run's first step, null nothing */
  kept(): string | null { return this.face.recoverWhat(this.h); }
  /** A GAME LOST TO A PANIC STANDS AGAIN, from the best save of it or from its run's first step: the save's label, '' for the start, null where nothing of it is kept */
  recover(newest: boolean): string | null {
    const r = this.face.recover(this.h, newest); if (!r) return null;
    this.lost = null; this.h = r.game as GameNo; this.between = 1; this.said = null; this.done++; this.draw(); return r.label;
  }

  /** A GAME, from its start, on what the machine keeps (`o.kept`: rust/game_start/src/host/kept.rs). What has been commanded by now is done to it at its first step */
  start(o: Start): void {
    this.lost = null;
    const kept = o.kept ?? '', seats = o.seats ?? 1;
    this.tape?.start({ build: o.named?.build ?? '', levelName: o.named?.level ?? '', level: o.level, words: o.words, parts: partsOf(o), seed: o.seed, seats, flags: DRESS, kept });
    if (this.h) { this.face.drop(this.h); this.h = 0 as GameNo; }
    this.h = this.face.start(o.level, o.words, o.mannequin, o.mannequinBin, o.cast, o.castLook, kept, o.seed, seats, DRESS | LATER);
    this.ticks = 0; this.done++; this.said = null; this.between = 1; this.tried = -1;
    // (room asked for and given back: the allocator keeps it, and a step seldom has to grow the memory)
    this.face.reserve(64 * 1048576);
    this.draw();
  }

  /** WHAT THE FIRST STEP WANTS AND A START LEAVES, made a slice at a time between turns of the event loop while the page has something else to wait for (the module's `ready`) */
  async ready(turn: () => Promise<void>): Promise<void> {
    for (let t0 = performance.now(); this.face.ready(this.h, READY);) if (performance.now() - t0 >= READY_MS) { await turn(); t0 = performance.now(); }
  }
  /** the floors' traffic map, two bytes a texel: worked out as it is asked */
  traffic(): Uint8Array<ArrayBuffer> { return this.face.traffic(this.h); }

  /** the next hand, to be written by whoever samples the player */
  numbersOf(): Hand { return this.hand; }
  /** ONE TICK on the hand as it stands: `keys` = the codes held, a line feed, the codes that went down */
  step(dt: number, keys: string): void {
    const t0 = performance.now();
    this.order();
    const T = this.tape; T?.step(dt, this.hand.all, keys);
    this.tick(dt, keys);
    if (T) { if (this.ticks % MARKS.every === 0) { T.mark(this.mark()); T.figures(this.figures()); } if (T.length > TAPE_MOST) { T.end('it was longer than a tape is kept'); this.ended = T; this.tape = null; } }
    const t1 = performance.now();
    this.draw();
    this.face.outClear(this.h);   // (what the tick told its host is in its sections now)
    this.ms.step = t1 - t0; this.ms.draw = performance.now() - t1;
  }
  private tick(dt: number, keys: string, told = SAVES): void {
    this.face.hand(this.h, 0, this.hand.input, this.hand.lens, keys);
    this.face.step(this.h, dt, told);
    this.ticks++; this.done++;
  }
  /** it is walked through a tape: nobody commands it but the tape */
  walked = false;
  private order(): void { if (this.commands.length) { const b = this.commands.take(); if (this.walked) return; this.tape?.order(b); this.face.commands(this.h, b); } }
  /** what the game has come to: the rolls of its die, the word of where everybody stands, and the hash of every part of it (`whole` false: without that, the dear one) */
  mark(whole = true): TapeMark {
    const of = (w: Uint32Array): number => hashOf(new Uint8Array(w.buffer));
    return { tick: this.ticks, die: this.face.dieN(this.h), stands: this.face.tickWord(this.h), state: whole ? of(this.face.stateWords(this.h)) : null };
  }
  /** who stands where, and the rounds the player has fired */
  figures(): TapeFigures { return { tick: this.ticks, shots: this.tally().stats.shots, men: this.map().men }; }
  /** A ROW OF A TAPE, handed over as it was: `seen` false, a tick nobody is shown (what it left to be drawn is not made, nor kept) */
  feed(row: TapeRow, seen = true): void {
    if (row.kind === 'step') { const t0 = performance.now(); this.hand.all.set(row.hand); this.tick(row.dt, row.keys); const t1 = performance.now(); if (seen) this.draw(); this.face.outClear(this.h); this.ms.step = t1 - t0; this.ms.draw = performance.now() - t1; }
    else if (row.kind === 'order') { this.face.commands(this.h, row.bytes); this.done++; if (seen) this.draw(); }
    else if (row.kind === 'hands') this.handsOff(row.plan);
  }
  /** A TICK STEPPED BY ANOTHER HAND (a networked game's walk): what it left is taken (`seen` false: nobody is shown it; `over`: one after the first before a frame) */
  stepped(seen: boolean, over: boolean): void { this.ticks++; this.done++; if (seen) { if (over) this.drawn.takeOver(this.h); else this.draw(); } this.face.outClear(this.h); }
  /** the shown seat's hand, for the picture's own view: a record handed for the seat after it is what is stepped */
  shownHand(seat: number, keys: string): void { this.face.hand(this.h, seat, this.hand.input, this.hand.lens, keys); }
  /** where the shown man is drawn ahead of the game, this tick of the page's (null: where the game has him); `still`: the game was not stepped in it */
  lead(L: { id: number; from: { pos: ArrayLike<number>; yaw: number }; to: { pos: ArrayLike<number>; yaw: number } } | null, still: boolean): void {
    if (still) this.face.drawHold(this.h);
    const N = this.leads;
    if (L) { N[LeadNow.man] = L.id; N[LeadNow.yaw] = L.from.yaw; N[LeadNow.toYaw] = L.to.yaw; for (let i = 0; i < 3; i++) { N[LeadNow.at + i] = L.from.pos[i]; N[LeadNow.to + i] = L.to.pos[i]; } }
    this.face.drawLead(this.h, L ? N : null);
  }
  private readonly leads = leadNow();
  /** A VIEWER'S OWN PICTURE of the run: with the fog off it, and through a tube or not whatever the run's was (null: as the run had it). The first call has the game keep what such a picture is made from */
  viewAs(bared: boolean, tube: boolean | null): void { this.as = [bared, tube]; if (this.lost) return; this.face.drawAs(this.h, bared, tube); this.draw(); }
  private as: [bared: boolean, tube: boolean | null] | null = null;
  /** the floor as it stands, for a map of it (tape/minimap.ts), and who and what is on it now: everybody, seen or not */
  floor(): FloorView { return this.face.floor(this.h); }
  map(): MapView { return this.face.map(this.h); }
  /** THE MOMENT THE GAME STANDS AT, whole, to come back to exactly (a walk of a tape's): its number, 0 where it cannot be held */
  moment(): MomentNo { return this.face.moment(this.h); }
  /** the game stands there again, at tick `ticks` */
  momentBack(k: MomentNo, ticks: number): boolean { const lost = this.lost; if (!this.face.momentBack(this.h, k)) return false; this.lost = null; if (lost) this.said = null;   // (a lost game's drawer is made again, and knows nothing of the page's look)
    if (this.as) this.face.drawAs(this.h, ...this.as);   // (a moment from before the picture was anybody's own)
    this.ticks = ticks; this.done++; return true; }
  /** ticks were stepped that nobody was shown: the picture is of where the game stands, and of nothing before it */
  show(): void { this.face.drawCut(this.h); this.draw(); }
  momentDrop(k: MomentNo): void { this.face.momentDrop(k); }
  /** THE FIRST TICKS, PLAYED AND THROWN AWAY: a browser compiles a function of the module as it is first called, and a game's first tick calls thousands of them. `hand` fills in what tick `i` of them is
   *  handed (the keys' text its answer); a tick, then a turn of the event loop. The game stands at its moment again after, and nobody was shown any of it */
  async warm(n: number, hand: (N: Hand, i: number) => string, turn: () => Promise<void>): Promise<void> {
    const k = this.face.moment(this.h); if (!k) return;
    const T = this.tape, at = this.ticks, told = this.walked ? 0 : SAVES; this.tape = null;   // (a tape keeps what was played)
    try { for (let i = 0; i < n; i++) { const keys = hand(this.numbersOf(), i); this.tick(1 / 60, keys, told); this.face.outClear(this.h); await turn(); } } catch (e) { if (!this.lost) throw e; /* (lost in them: it stands again below, and is lost in play if it is to be) */ }
    finally { this.momentBack(k, at); this.momentDrop(k); this.tape = T; this.draw(); }   // (…and its sections are of where it stands, before anybody asks which game they are of)
  }
  /** what has been commanded is done now, between two ticks, and what it changed is there to be read: the next step would do the same things first */
  tell(): void { this.order(); this.done++; this.draw(); }
  /** A FRAME IS DRAWN, `a` of the way from the tick before to the last */
  frame(a: number, focus: ArrayLike<number>): void { this.drawn.frame(this.h, a, focus); this.between = a; }
  private between = 1;
  /** the picture's clock: seconds of the game's, between its ticks as the frame is */
  get time(): number { return Math.max(0, this.drawn.u[this.drawn.at[Section.view] + View.tick] - 1 + this.between) / 60; }
  /** where a man is drawn standing, by his character's id; null = nothing is drawn of him */
  at(id: number): [number, number, number] | null { return this.of(id, Man.feet); }
  /** … and where his head is */
  head(id: number): [number, number, number] | null { return this.of(id, Man.head); }
  private of(id: number, word: Man): [number, number, number] | null {
    const { f, u, n, at } = this.drawn; for (let i = 0, o = at[Section.men]; i < n[Section.men]; i++, o += Man.row) if (u[o + Man.id] === id) return [f[o + word], f[o + word + 1], f[o + word + 2]];
    return null;
  }
  /** what this page says of its look, when it is another than it last said */
  look(v: Doubles<'PageLook'>): void {
    const was = this.said; if (was) { let i = 0; while (i < v.length && v[i] === was[i]) i++; if (i === v.length) return; }
    this.said = v.slice(); this.face.drawLook(this.h, v); this.draw();
  }
  private said: Float64Array | null = null;
  private told = { done: -1, view: null as TickView | null };
  /** WHAT A HUD IS TOLD of the game as it stands (src/view): read once a tick, or after what is done between two, however often it is asked */
  view(): TickView {
    const T = this.told; if (!T.view || T.done !== this.done) { T.view = this.face.tickView(this.h); T.done = this.done; }
    return T.view;
  }
  private cursorN = cursorNow(); private pointed = { done: -1, view: { under: null } as CursorView };
  /** WHAT THE CURSOR IS ON, through the camera as it stands: asked again when the cursor, the camera or the game has moved */
  cursor(input: CursorAt, cam: CursorLens, canvas: CursorCanvas): CursorView {
    const T = this.pointed, moved = sampleCursor(this.cursorN, input, cam, canvas);
    if (moved || T.done !== this.done) { T.view = this.face.cursorView(this.h, null, this.cursorN); T.done = this.done; }
    return T.view;
  }
  /** what the end card shows of the run, asked as it opens */
  debrief(): DebriefView { return this.face.debriefView(this.h); }
  /** …and beside that: how it ended, every man of it, the course's par, the map's floor and its marks */
  end(): EndView { return this.face.endView(this.h); }
  /** what a machine keeps of the run, asked as its try begins and as it ends */
  run(): RunView { return this.face.runView(this.h); }
  /** how the run ended, in a word, and why; null while it is being played */
  verdict(): OutcomeView | null { return this.face.verdict(this.h); }
  /** nobody's hands are on the seat: a hands-off player takes it (rust/game_start hands_off.rs), or with no plan it is the host's again */
  handsOff(plan: HandsOffPlan | null): void {
    this.tape?.hands(plan);
    if (plan) this.face.handsOff(this.h, 0, plan.seed, plan.cap ?? 240, !!plan.breaker, plan.route ?? []); else this.face.handsOn(this.h);
    this.handsKept.done = -1;
  }
  /** how he stands, null where nobody is hands off */
  hands(): HandsSaid | null {
    const K = this.handsKept; if (K.done !== this.done) { K.done = this.done; K.said = this.face.handsSaid(this.h); }
    return K.said;
  }
  private handsKept = { done: -1, said: null as HandsSaid | null };
  /** his timeline */
  handsTrace(): string { return this.face.handsTrace(this.h); }
  private tallied: { done: number; t: Tally } | null = null;
  /** what a measuring take notes of the game as its last tick left it */
  tally(): Tally {
    const T = this.tallied; if (T && T.done === this.done) return T.t;
    const t = this.face.tally(this.h); this.tallied = { done: this.done, t }; return t;
  }
  /** what the save controller tells the player: its tag while it is up, and each thing it has said since this was last asked */
  savesTold(): SavesTold { return this.face.savesTold(this.h); }
  /** this run's saves as a card lays them out, the one to go back to, would one by hand be made, and the mission's clock now */
  saves(): SavesView { return this.face.saves(this.h); }
  /** every knob a person may turn (`commands.knob` turns one) */
  knobs(): KnobRow[] { return this.face.knobs(this.h); }
  /** the panel's rows of the game's numbers, as the module lists them (game_start host/shown.rs) */
  shown(): ShownRow[] { return this.face.shown(); }
  /** the panel's actions, as the module lists them (game_start host/actions.rs) */
  actions(): ActionRow[] { return this.face.actions(this.h); }
  /** what those rows stand at now, as a row shows it */
  shownNow(): Float64Array { return this.face.shownNow(this.h); }
  private turned = new Map<string, number>();
  /** turn a knob, for the visit: a load or a restart stands a game whose knobs are another moment's, and this is turned again (`turnAgain`) */
  turn(name: string, value: number): void { this.turned.set(name, value); this.commands.knob(name, value); }
  turnAgain(): void { for (const [name, value] of this.turned) this.commands.knob(name, value); for (const [name, word] of this.picked) this.commands.act(name, 0, word); }
  private picked = new Map<string, string>();
  /** a row or a pick a start's record laid: noted, and turned again as one of the panel's */
  keptTurn(name: string, value: number): void { this.turned.set(name, value); }
  keptPick(name: string, word: string): void { this.picked.set(name, word); }
  /** what the panel and this machine have turned and picked, by name: what a play record says */
  get turnedNow(): Record<string, number | string> { return { ...Object.fromEntries(this.turned), ...Object.fromEntries(this.picked) }; }
  /** one of the panel's picks, for the visit: a restart or a load stands a game that has another, and this is picked again */
  pick(name: string, word: string): void { this.picked.set(name, word); this.commands.act(name, 0, word); }
  /** a name that crosses by its number */
  nameOf(i: number): string { return this.face.drawName(this.h, i); }
  private draw(): void {
    this.drawn.take(this.h); if (!this.tape) return;
    const { u, at } = this.drawn, run = u[at[Section.heard] + Heard.run]; if (run !== this.tried) { if (this.tried >= 0) this.tape.again(); this.tried = run; }
  }
  /** which try of the mission the tape is in */
  private tried = -1;
}
