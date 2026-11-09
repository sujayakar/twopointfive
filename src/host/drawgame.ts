// A GAME IN THE MODULE, AS A PAGE THAT DRAWS IT HAS IT: started, handed a player's input, stepped, and read by what it leaves to be drawn (src/draw). The page
// holds its number and nothing of it.
import type { HandsOffPlan } from './benchScenes';
import { FaceError, type Face, type Fetched } from './face';
import { Drawn, drawRefused, type DrawFace } from '../draw/drawn';
import { Heard, Man, Section, View, Want } from '../draw/sections';
import { Commands } from '../draw/commands';
import { HAND } from '../draw/hand';
import { HandTape, MARKS, hashOf, type PartOf, type TapeFigures, type TapeMark, type TapeRow } from '../tape/handtape';
import type { SaveRow } from '../ui/runcard';
import type { RunMark, RunVerdict } from '../view/runend';
import { CURSOR_DOUBLES, sampleCursor, type CursorAt, type CursorCanvas, type CursorLens } from '../view/cursor';
import { readCursorView, readDebriefView, readRunView, readTickView, type CursorView, type DebriefView, type RunView, type TickView } from '../view/read';

export interface HandsOffSaid { t: number; end: 'taken' | 'sam' | 'cap' | 'spent' | null; given: number; shotAt: number; up: number; of: number; left: number }
/** how a run ended, as the module's record of it says (game::verdict::verdict_lines) */
export interface Verdict { won: boolean; word: string; why: string; secs: number; approach: string | null; lost: number; of: number; foesDown: number; foesOf: number; hostages: { alive: number; of: number } | null; void: boolean; clean: boolean }
/** what a measuring take notes: each man by what he is at and whether he carries a light, the reaction force, the alarm, the run's saves, the mission's stage */
export interface Tally { men: [state: string, lit: 0 | 1][]; qrf: string; escalation: number; autosaves: number; manual: number; loads: number; stage: string; alarm: boolean; alerts: number; /** a room clear is staged */ clear: boolean; /** the kind of mission its verdict is for */ kind: string | null; /** the mission's own counts */ stats: Record<string, number | boolean | string | null> }
/** what a start is handed beside the level (rust/game_start/src/handed.rs PARTS) */
export interface Start { level: Uint8Array; words: string; mannequin: Uint8Array; mannequinBin: Uint8Array; cast: Uint8Array; castLook: Uint8Array; seed: number; seats?: number; /** what a tape of the run says it is of */ named?: { level: string; build: string } }
/** the parts of a start that are the build's own, as a tape holds a walk's to its */
export const partsOf = (o: Start): PartOf[] => (['mannequin', 'mannequinBin', 'cast', 'castLook'] as const).map(name => ({ name, length: o[name].length, hash: hashOf(o[name]) }));

/** `face_start`'s: the rows that dress the picture are up */
const DRESS = 1;
/** `face_step`'s: the save controller */
const SAVES = 8;

/** bytes a tape may be: it ends there (an hour and a half and more of play) */
const TAPE_MOST = 48 * 1048576;
/** the save controller with nothing to tell */
const QUIET = { tag: '', lines: [] as string[] };

/** the calls a lost game may still be asked, or is stood again by */
const STILL = new Set(['face_alloc', 'face_free', 'face_out_ptr', 'face_out_len', 'face_panic_ptr', 'face_panic_len', 'face_panic_clear', 'face_mem', 'face_recover', 'face_recover_what', 'face_drop', 'face_start', 'face_moment_back', 'face_moment_drop']);

/** a row of the panel that does something to the game */
export type ActionRow = { name: string; label: string; section: string; title: string } & ({ kind: 'press' } | { kind: 'each'; words: string[] } | { kind: 'choice'; words: string[]; now: string } | { kind: 'count'; lo: number; hi: number; step: number; start: number } | { kind: 'reload'; word: string; words: string[] });
/** a row of the panel over a number of the game: `scale` is what the row shows the number at, `kept` stays turned on this machine, `adv` waits behind 'all knobs', `def` is the number as the game ships it (NaN: a number a level sets) */
export interface ShownRow { name: string; label: string; section: string; title: string; lo: number; hi: number; step: number; scale: number; def: number; kept: boolean; adv: boolean }
/** the game of a call was lost to a panic in it: its words, the call, and the tick */
export class GameLost extends Error { constructor(readonly words: string, readonly what: string, readonly tick: number) { super(`the game stopped in ${what}, at tick ${tick}: ${words}`); this.name = 'GameLost'; } }

export class DrawGame {
  readonly X: Face;
  readonly memory: WebAssembly.Memory;
  readonly drawn: Drawn;
  h = 0;
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
  private hand = 0; private keys = 0; private numbers = new Float64Array(0);
  private static readonly KEYS = 2048;
  private enc = new TextEncoder(); private dec = new TextDecoder();

  constructor(f: Fetched) {
    this.memory = f.exports.memory as WebAssembly.Memory; this.X = this.guarded(f.exports as unknown as Face);
    const no = drawRefused(this.X as unknown as DrawFace); if (no) throw new FaceError(`${no} (tools/blitz/face-build.ts builds this checkout's)`);
    this.drawn = new Drawn(this.X as unknown as DrawFace, this.memory, Want.all);
    this.hand = this.X.face_alloc(HAND.count * 8) >>> 0; this.keys = this.X.face_alloc(DrawGame.KEYS) >>> 0;
  }

  private handed<T>(b: Uint8Array, f: (p: number, n: number) => T): T { const p = this.X.face_alloc(b.length) >>> 0; new Uint8Array(this.memory.buffer, p, b.length).set(b); try { return f(p, b.length); } finally { this.X.face_free(p, b.length); } }
  private answer(): string { const p = this.X.face_out_ptr() >>> 0, n = this.X.face_out_len() >>> 0; return this.dec.decode(new Uint8Array(this.memory.buffer, p, n)); }
  /** every call of the face: its memory may have grown, so the sections' views are made again where it has; a trap (a panic, the stack run out) loses the game of that call, and nothing else of it is called until it stands again */
  private guarded(raw: Face): Face {
    const out: Face = {};
    for (const k of Object.keys(raw)) { const f = raw[k]; out[k] = typeof f !== 'function' || !k.startsWith('face_') ? f : (...a: number[]): number => {
      if (this.lost && !STILL.has(k)) throw this.lost;
      try { const n = f(...a); this.drawn?.fresh(); return n; } catch (e) { if (!(e instanceof WebAssembly.RuntimeError) && !(e instanceof RangeError)) throw e;
        let words = ''; try { const p = raw.face_panic_ptr() >>> 0, n = raw.face_panic_len() >>> 0; words = this.dec.decode(new Uint8Array(this.memory.buffer, p, n)); raw.face_panic_clear(); } catch { /* the trap's own words, then */ }
        this.lost = new GameLost(words || String((e as Error).message), k, this.ticks); this.tape?.end(this.lost.message); this.ended = this.tape ?? this.ended; this.tape = null; throw this.lost; } }; }
    return out;
  }
  /** the panic the game was lost to, until it stands again */
  lost: GameLost | null = null;
  /** what a lost game would stand again from: a save's label, '' its run's first step, null nothing */
  kept(): string | null { const w = this.X.face_recover_what(this.h); return w === 0 ? null : w === 1 ? '' : this.answer(); }
  /** A GAME LOST TO A PANIC STANDS AGAIN, from the best save of it or from its run's first step: the save's label, '' for the start, null where nothing of it is kept */
  recover(newest: boolean): string | null {
    const h = this.X.face_recover(this.h, newest ? 1 : 0); if (!h) return null;
    this.lost = null; this.h = h; this.between = 1; const label = this.answer(); this.said = null; this.done++; this.draw(); return label;
  }

  /** A GAME, from its start; what has been commanded by now is done to it before it is made */
  start(o: Start): void {
    this.lost = null;
    const said = this.commands.take().slice(), seats = o.seats ?? 1;
    this.tape?.start({ build: o.named?.build ?? '', levelName: o.named?.level ?? '', level: o.level, words: o.words, parts: partsOf(o), seed: o.seed, seats, flags: DRESS, commands: said });
    const parts = [o.level, this.enc.encode(o.words), o.mannequin, o.mannequinBin, o.cast, o.castLook, said], n = 4 + parts.reduce((s, p) => s + 4 + p.length, 0);
    const at = this.X.face_alloc(n) >>> 0, b = new Uint8Array(this.memory.buffer, at, n), d = new DataView(this.memory.buffer, at, n); let q = 4;
    d.setUint32(0, parts.length, true);
    for (const p of parts) { d.setUint32(q, p.length, true); b.set(p, q + 4); q += 4 + p.length; }
    if (this.h) { this.X.face_drop(this.h); this.h = 0; }
    try { this.h = this.X.face_start(at, n, -1, o.seed, seats, DRESS); } finally { this.X.face_free(at, n); }
    if (!this.h) throw new FaceError('no room for another game in this module');
    this.ticks = 0; this.done++; this.said = null; this.between = 1; this.tried = -1;
    // (room asked for and given back: the allocator keeps it, and a step seldom has to grow the memory)
    this.X.face_reserve(64 * 1048576);
    this.draw();
  }

  /** the numbers of the next hand, to be written by whoever samples the player */
  numbersOf(): Float64Array { if (this.numbers.buffer !== this.memory.buffer) this.numbers = new Float64Array(this.memory.buffer, this.hand, HAND.count); return this.numbers; }
  /** ONE TICK on the hand as it stands: `keys` = the codes held, a line feed, the codes that went down */
  step(dt: number, keys: string): void {
    const t0 = performance.now();
    this.order();
    const T = this.tape; T?.step(dt, this.numbersOf(), keys);
    this.tick(dt, keys);
    if (T) { if (this.ticks % MARKS.every === 0) { T.mark(this.mark()); T.figures(this.figures()); } if (T.length > TAPE_MOST) { T.end('it was longer than a tape is kept'); this.ended = T; this.tape = null; } }
    const t1 = performance.now();
    this.draw();
    this.X.face_out_clear(this.h);   // (what the tick told its host is in its sections now)
    this.ms.step = t1 - t0; this.ms.draw = performance.now() - t1;
  }
  private tick(dt: number, keys: string, told = SAVES): void {
    const n = this.enc.encodeInto(keys, new Uint8Array(this.memory.buffer, this.keys, DrawGame.KEYS)).written;
    this.X.face_hand(this.h, 0, this.hand, HAND.count * 8, this.keys, n);
    this.X.face_step(this.h, dt, told);
    this.ticks++; this.done++;
  }
  /** it is walked through a tape: nobody commands it but the tape */
  walked = false;
  private order(): void { if (this.commands.length) { const b = this.commands.take(); if (this.walked) return; this.tape?.order(b); this.command(b); } }
  private command(b: Uint8Array): void { this.handed(b, (p, m) => { if (!this.X.face_commands(this.h, p, m)) throw new Error(this.answer()); }); }
  /** what the game has come to: the rolls of its die and its block's hash, and the whole of it where that is asked (a check's: it is a frame's worth of time) */
  mark(whole = false): TapeMark {
    const at = this.X.face_part_hashes(this.X.face_slot(this.h)) >>> 0, n = this.X.face_parts(), die = this.X.face_die_n(this.h), parts = hashOf(new Uint8Array(this.memory.buffer, at, n * 4));
    let state: number | null = null; if (whole) { this.X.face_state_words(this.h); state = hashOf(this.bytes()); }
    return { tick: this.ticks, die, parts, state };
  }
  /** who stands where, and the rounds the player has fired */
  figures(): TapeFigures { return { tick: this.ticks, shots: Number(this.tally().stats.shots ?? 0), men: this.map().men }; }
  /** A ROW OF A TAPE, handed over as it was: `seen` false, a tick nobody is shown (what it left to be drawn is not made, nor kept) */
  feed(row: TapeRow, seen = true): void {
    if (row.kind === 'step') { const t0 = performance.now(); this.numbersOf().set(row.hand); this.tick(row.dt, row.keys); const t1 = performance.now(); if (seen) this.draw(); this.X.face_out_clear(this.h); this.ms.step = t1 - t0; this.ms.draw = performance.now() - t1; }
    else if (row.kind === 'order') { this.command(row.bytes); this.done++; if (seen) this.draw(); }
    else if (row.kind === 'hands') this.handsTo(row.plan);
  }
  /** A TICK STEPPED BY ANOTHER HAND (a networked game's walk): what it left is taken (`seen` false: nobody is shown it; `over`: one after the first before a frame) */
  stepped(seen: boolean, over: boolean): void { this.ticks++; this.done++; if (seen) { if (over) this.drawn.takeOver(this.h); else this.draw(); } this.X.face_out_clear(this.h); }
  /** the shown seat's hand, for the picture's own view: a record handed for the seat after it is what is stepped */
  shownHand(seat: number, keys: string): void { const n = this.enc.encodeInto(keys, new Uint8Array(this.memory.buffer, this.keys, DrawGame.KEYS)).written; this.X.face_hand(this.h, seat, this.hand, HAND.count * 8, this.keys, n); }
  /** where the shown man is drawn ahead of the game, this tick of the page's (null: where the game has him); `still`: the game was not stepped in it */
  lead(L: { id: number; from: { pos: ArrayLike<number>; yaw: number }; to: { pos: ArrayLike<number>; yaw: number } } | null, still: boolean): void {
    if (still) this.X.face_draw_hold(this.h);
    if (!L) { this.X.face_draw_lead(this.h, 0, 0); return; }
    const N = new Float64Array([L.from.pos[0], L.from.pos[1], L.from.pos[2], L.from.yaw, L.to.pos[0], L.to.pos[1], L.to.pos[2], L.to.yaw]);
    this.handed(new Uint8Array(N.buffer), p => this.X.face_draw_lead(this.h, L.id, p));
  }
  /** A VIEWER'S OWN PICTURE of the run: with the fog off it, and through a tube or not whatever the run's was (null: as the run had it). The first call has the game keep what such a picture is made from */
  viewAs(bared: boolean, tube: boolean | null): void { this.as = [bared ? 1 : 0, tube === null ? 0 : tube ? 1 : 2]; if (this.lost) return; this.X.face_draw_as(this.h, ...this.as); this.draw(); }
  private as: [number, number] | null = null;
  /** the floor as it stands, for a map of it (tape/minimap.ts), and who and what is on it now: everybody, seen or not */
  floor(): { bounds: [number, number, number, number]; boxes: number[] } { this.X.face_floor(this.h); return JSON.parse(this.answer()); }
  map(): { men: { id: number; x: number; z: number; aim: number; team: string; kind: 'player' | 'man' | 'civ'; down: boolean; name: string; state: string }[]; doors: { hx: number; hz: number; ux: number; uz: number; width: number; locked: boolean; broken: boolean; off: boolean; name: string }[] } { this.X.face_map(this.h); return JSON.parse(this.answer()); }
  /** THE MOMENT THE GAME STANDS AT, whole, to come back to exactly (a walk of a tape's): its number, 0 where it cannot be held */
  moment(): number { return this.X.face_moment(this.h); }
  /** the game stands there again, at tick `ticks` */
  momentBack(k: number, ticks: number): boolean { const lost = this.lost; if (!this.X.face_moment_back(this.h, k)) return false; this.lost = null; if (lost) this.said = null;   // (a lost game's drawer is made again, and knows nothing of the page's look)
    if (this.as) this.X.face_draw_as(this.h, ...this.as);   // (a moment from before the picture was anybody's own)
    this.ticks = ticks; this.done++; return true; }
  /** ticks were stepped that nobody was shown: the picture is of where the game stands, and of nothing before it */
  show(): void { this.X.face_draw_cut(this.h); this.draw(); }
  momentDrop(k: number): void { this.X.face_moment_drop(k); }
  /** THE FIRST TICKS, PLAYED AND THROWN AWAY: a browser compiles a function of the module as it is first called, and a game's first tick calls thousands of them. `hand` fills in what tick `i` of them is
   *  handed (the keys' text its answer); a tick, then a turn of the event loop. The game stands at its moment again after, and nobody was shown any of it */
  async warm(n: number, hand: (N: Float64Array, i: number) => string, turn: () => Promise<void>): Promise<void> {
    const k = this.X.face_moment(this.h); if (!k) return;
    const T = this.tape, at = this.ticks, told = this.walked ? 0 : SAVES; this.tape = null;   // (a tape keeps what was played)
    try { for (let i = 0; i < n; i++) { const keys = hand(this.numbersOf(), i); this.tick(1 / 60, keys, told); this.X.face_out_clear(this.h); await turn(); } } catch (e) { if (!this.lost) throw e; /* (lost in them: it stands again below, and is lost in play if it is to be) */ }
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
  look(v: Float64Array): void {
    const was = this.said; if (was) { let i = 0; while (i < v.length && v[i] === was[i]) i++; if (i === v.length) return; }
    this.said = v.slice(); const at = this.X.face_alloc(v.length * 8) >>> 0;
    new Float64Array(this.memory.buffer, at, v.length).set(v);
    try { this.X.face_draw_look(this.h, at, v.length); } finally { this.X.face_free(at, v.length * 8); }
    this.draw();
  }
  private said: Float64Array | null = null;
  /** the last answer's bytes, kept */
  private bytes(): Uint8Array { const p = this.X.face_out_ptr() >>> 0, n = this.X.face_out_len() >>> 0; return new Uint8Array(this.memory.buffer.slice(p, p + n)); }
  private told = { done: -1, view: null as TickView | null };
  /** WHAT A HUD IS TOLD of the game as it stands (src/view): read once a tick, or after what is done between two, however often it is asked */
  view(): TickView {
    const T = this.told; if (!T.view || T.done !== this.done) { this.X.face_tick_view(this.h); T.view = readTickView(this.bytes()); T.done = this.done; }
    return T.view;
  }
  private cursorN = new Float64Array(CURSOR_DOUBLES); private pointed = { done: -1, view: { under: null } as CursorView };
  /** WHAT THE CURSOR IS ON, through the camera as it stands: asked again when the cursor, the camera or the game has moved */
  cursor(input: CursorAt, cam: CursorLens, canvas: CursorCanvas): CursorView {
    const T = this.pointed, moved = sampleCursor(this.cursorN, input, cam, canvas);
    if (moved || T.done !== this.done) { T.view = this.handed(new Uint8Array(this.cursorN.buffer), p => { this.X.face_cursor_view(this.h, -1, p); return readCursorView(this.bytes()); }); T.done = this.done; }
    return T.view;
  }
  /** what the end card shows of the run, asked as it opens */
  debrief(): DebriefView { this.X.face_debrief_view(this.h); return readDebriefView(this.bytes()); }
  /** …and beside that: how it ended, every man of it, the course's par, the map's floor and its marks */
  end(): { verdict: RunVerdict | null; par: { medal: string; line: string } | null; quiet: { holder: string } | null; marks: RunMark[]; floor: { w: number; h: number; cell: number; rows: string } } { this.X.face_end(this.h); return JSON.parse(this.answer()); }
  /** what a machine keeps of the run, asked as its try begins and as it ends */
  run(): RunView { this.X.face_run_view(this.h); return readRunView(this.bytes()); }
  /** how the run ended, in a word, and why; null while it is being played */
  verdict(): Verdict | null {
    const r = this.X.face_verdict(this.h); if (!r) return null;
    const [word, why, secs, way, men, foes, kept, voided, clean] = this.answer().split('\n'), two = (s: string): number[] => s.split(' ').map(Number), [lost, of] = two(men), [foesDown, foesOf] = two(foes), [alive, held] = two(kept);
    return { won: r === 2, word, why, secs: Number(secs), approach: way === '-' ? null : way, lost, of, foesDown, foesOf, hostages: kept === '-' ? null : { alive, of: held }, void: voided === 'true', clean: clean === 'true' };
  }
  /** nobody's hands are on the seat: a hands-off player takes it (rust/game_start hands_off.rs), or with no plan it is the host's again */
  handsOff(plan: HandsOffPlan | null): void { this.handsTo(this.enc.encode(plan ? [plan.seed, plan.cap ?? 240, plan.breaker ? 1 : 0, ...plan.route ?? []].join('\n') : '')); }
  private handsTo(b: Uint8Array): void { this.tape?.hands(b); this.handed(b, (p, n) => this.X.face_hands_off(this.h, 0, p, n)); this.handsKept.done = -1; }
  /** how he stands, null where nobody is hands off */
  hands(): HandsOffSaid | null {
    const K = this.handsKept; if (K.done === this.done) return K.said;
    this.X.face_hands_said(this.h); const s = this.answer(); K.done = this.done; return K.said = s ? JSON.parse(s) as HandsOffSaid : null;
  }
  private handsKept = { done: -1, said: null as HandsOffSaid | null };
  /** his timeline */
  handsTrace(): string { this.X.face_hands_trace(this.h); return this.answer(); }
  private tallied: { done: number; t: Tally } | null = null;
  /** what a measuring take notes of the game as its last tick left it */
  tally(): Tally {
    const T = this.tallied; if (T && T.done === this.done) return T.t;
    this.X.face_tally(this.h); const t = JSON.parse(this.answer()) as Tally; this.tallied = { done: this.done, t }; return t;
  }
  /** what the save controller tells the player: its tag while it is up, and each thing it has said since this was last asked */
  savesTold(): { tag: string; lines: string[] } { if (!this.X.face_saves_told(this.h)) return QUIET; const [tag, ...lines] = this.answer().split('\n'); return { tag, lines }; }
  /** this run's saves as a card lays them out, the one to go back to, would one by hand be made, and the mission's clock now */
  saves(): { elapsed: number; hand: boolean; offer: string; rows: SaveRow[] } {
    this.X.face_saves(this.h); const p = this.X.face_out_ptr() >>> 0, n = this.X.face_out_len() >>> 0, b = this.memory.buffer.slice(p, p + n), d = new DataView(b); let at = 12;
    const text = (): string => { const len = d.getUint32(at, true); at += 4 + len; return this.dec.decode(new Uint8Array(b, at - len, len)); };
    const offer = text(), count = d.getUint32(at, true), rows: SaveRow[] = []; at += 4;
    for (let i = 0; i < count; i++) { const key = text(), label = text(), says = d.getUint32(at, true); rows.push({ key, label, slot: says & 1 ? 'manual' : 'auto', regret: says & 2 ? 'regretted' : null, missionT: d.getFloat64(at + 4, true), savedAt: d.getFloat64(at + 12, true) }); at += 20; }
    return { elapsed: d.getFloat64(0, true), hand: d.getUint32(8, true) !== 0, offer, rows };
  }
  /** every knob a person may turn (`commands.knob` turns one) */
  knobs(): { name: string; yes: boolean; value: number }[] {
    this.X.face_knobs(this.h); const p = this.X.face_out_ptr() >>> 0, n = this.X.face_out_len() >>> 0, b = this.memory.buffer.slice(p, p + n), d = new DataView(b), out: { name: string; yes: boolean; value: number }[] = [];
    for (let at = 0; at < n;) { const w = d.getUint32(at, true), len = w & 0x7fffffff; out.push({ name: this.dec.decode(new Uint8Array(b, at + 4, len)), yes: w >>> 31 === 1, value: d.getFloat64(at + 4 + len, true) }); at += 12 + len; }
    return out;
  }
  /** the panel's rows of the game's numbers, as the module lists them (game_run shown.rs) */
  shown(): ShownRow[] {
    this.X.face_shown(); const p = this.X.face_out_ptr() >>> 0, n = this.X.face_out_len() >>> 0, b = this.memory.buffer.slice(p, p + n), d = new DataView(b), out: ShownRow[] = []; let at = 0;
    const text = (): string => { const len = d.getUint32(at, true), t = this.dec.decode(new Uint8Array(b, at + 4, len)); at += 4 + len; return t; }, num = (): number => { const v = d.getFloat64(at, true); at += 8; return v; };
    while (at < n) { const name = text(), label = text(), section = text(), title = text(), lo = num(), hi = num(), step = num(), scale = num(), def = num(), bits = d.getUint32(at, true); at += 4; out.push({ name, label, section, title, lo, hi, step, scale, def, kept: (bits & 1) !== 0, adv: (bits & 2) !== 0 }); }
    return out;
  }
  /** the panel's actions, as the module lists them (game_run actions.rs) */
  actions(): ActionRow[] {
    this.X.face_actions(this.h); const p = this.X.face_out_ptr() >>> 0, n = this.X.face_out_len() >>> 0, b = this.memory.buffer.slice(p, p + n), d = new DataView(b), out: ActionRow[] = []; let at = 0;
    const u = (): number => { const v = d.getUint32(at, true); at += 4; return v; }, text = (): string => { const len = u(), t = this.dec.decode(new Uint8Array(b, at, len)); at += len; return t; }, num = (): number => { const v = d.getFloat64(at, true); at += 8; return v; }, words = (): string[] => { const k = u(), w: string[] = []; for (let i = 0; i < k; i++) w.push(text()); return w; };
    while (at < n) { const name = text(), label = text(), section = text(), title = text(), kind = u(), head = { name, label, section, title };
      if (kind === 0) out.push({ ...head, kind: 'press' });
      else if (kind === 1) out.push({ ...head, kind: 'each', words: words() });
      else if (kind === 2) { const all = words(); out.push({ ...head, kind: 'choice', words: all, now: text() }); }
      else if (kind === 3) out.push({ ...head, kind: 'count', lo: num(), hi: num(), step: num(), start: num() });
      else { const word = text(); out.push({ ...head, kind: 'reload', word, words: words() }); } }
    return out;
  }
  /** what those rows stand at now, as a row shows it */
  shownNow(): Float64Array { this.X.face_shown_now(this.h); const p = this.X.face_out_ptr() >>> 0, n = this.X.face_out_len() >>> 0; return new Float64Array(this.memory.buffer.slice(p, p + n)); }
  private turned = new Map<string, number>();
  /** turn a knob, for the visit: a load or a restart stands a game whose knobs are another moment's, and this is turned again (`turnAgain`) */
  turn(name: string, value: number): void { this.turned.set(name, value); this.commands.knob(name, value); }
  turnAgain(): void { for (const [name, value] of this.turned) this.commands.knob(name, value); for (const [name, word] of this.picked) this.commands.act(name, 0, word); }
  private picked = new Map<string, string>();
  /** what the panel and this machine have turned and picked, by name: what a play record says */
  get turnedNow(): Record<string, number | string> { return { ...Object.fromEntries(this.turned), ...Object.fromEntries(this.picked) }; }
  /** one of the panel's picks, for the visit: a restart or a load stands a game that has another, and this is picked again */
  pick(name: string, word: string): void { this.picked.set(name, word); this.commands.act(name, 0, word); }
  /** a name that crosses by its number */
  nameOf(i: number): string { this.X.face_draw_name(this.h, i); return this.answer(); }
  private draw(): void {
    this.drawn.take(this.h); if (!this.tape) return;
    const { u, at } = this.drawn, run = u[at[Section.heard] + Heard.run]; if (run !== this.tried) { if (this.tried >= 0) this.tape.again(); this.tried = run; }
  }
  /** which try of the mission the tape is in */
  private tried = -1;
}
