// A GAME IN THE MODULE, AS A PAGE THAT DRAWS IT HAS IT: started, handed a player's input, stepped, and read by what it leaves to be drawn (src/draw). The page
// holds its number and nothing of it.
import { FaceError, type Face, type Fetched } from './face';
import { Drawn, drawRefused, type DrawFace } from '../draw/drawn';
import { Man, Section, View, Want } from '../draw/sections';
import { Commands } from '../draw/commands';
import { HAND } from '../draw/hand';
import type { SaveRow } from '../ui/runcard';
import { CURSOR_DOUBLES, sampleCursor, type CursorAt, type CursorCanvas, type CursorLens } from '../view/cursor';
import { readCursorView, readDebriefView, readRunView, readTickView, type CursorView, type DebriefView, type RunView, type TickView } from '../view/read';

/** a hands-off player's plan: the die his beats are rolled on, the seconds he plays, doors by name to take first, the breaker thrown before any */
export interface HandsOffPlan { seed: number; cap?: number; route?: string[]; breaker?: boolean }
export interface HandsOffSaid { t: number; end: 'taken' | 'sam' | 'cap' | 'spent' | null; given: number; shotAt: number; up: number; of: number; left: number }
/** how a run ended, as the module's record of it says (game::verdict::verdict_lines) */
export interface Verdict { won: boolean; word: string; why: string; secs: number; approach: string | null; lost: number; of: number; foesDown: number; foesOf: number; hostages: { alive: number; of: number } | null; void: boolean; clean: boolean }
/** what a measuring take notes: each man by what he is at and whether he carries a light, the reaction force, the alarm, the run's saves, the mission's stage */
export interface Tally { men: [state: string, lit: 0 | 1][]; qrf: string; escalation: number; autosaves: number; manual: number; loads: number; stage: string; alarm: boolean; alerts: number; /** a room clear is staged */ clear: boolean; /** the kind of mission its verdict is for */ kind: string | null; /** the mission's own counts */ stats: Record<string, number | boolean | string | null> }
/** what a start is handed beside the level (rust/game_start/src/handed.rs PARTS) */
export interface Start { level: Uint8Array; words: string; mannequin: Uint8Array; mannequinBin: Uint8Array; cast: Uint8Array; castLook: Uint8Array; seed: number; seats?: number }

/** `face_start`'s: the rows that dress the picture are up */
const DRESS = 1;
/** `face_step`'s: a rails level's view · the save controller */
const VIEW = 4, SAVES = 8;

/** the save controller with nothing to tell */
const QUIET = { tag: '', lines: [] as string[] };

/** the calls a lost game may still be asked, or is stood again by */
const STILL = new Set(['face_alloc', 'face_free', 'face_out_ptr', 'face_out_len', 'face_panic_ptr', 'face_panic_len', 'face_panic_clear', 'face_mem', 'face_recover', 'face_recover_what', 'face_drop', 'face_start']);

/** the game of a call was lost to a panic in it: its words, the call, and the tick */
export class GameLost extends Error { constructor(readonly words: string, readonly what: string, readonly tick: number) { super(`the game stopped in ${what}, at tick ${tick}: ${words}`); this.name = 'GameLost'; } }

export class DrawGame {
  readonly X: Face;
  readonly memory: WebAssembly.Memory;
  readonly drawn: Drawn;
  h = 0;
  /** what a player or a panel does that is not input: the next step takes it */
  readonly commands = new Commands();
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
        throw this.lost = new GameLost(words || String((e as Error).message), k, this.ticks); } }; }
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
    const parts = [o.level, this.enc.encode(o.words), o.mannequin, o.mannequinBin, o.cast, o.castLook, new Uint8Array(0), this.commands.take().slice()], n = 4 + parts.reduce((s, p) => s + 4 + p.length, 0);
    const at = this.X.face_alloc(n) >>> 0, b = new Uint8Array(this.memory.buffer, at, n), d = new DataView(this.memory.buffer, at, n); let q = 4;
    d.setUint32(0, parts.length, true);
    for (const p of parts) { d.setUint32(q, p.length, true); b.set(p, q + 4); q += 4 + p.length; }
    if (this.h) { this.X.face_drop(this.h); this.h = 0; }
    try { this.h = this.X.face_start(at, n, -1, o.seed, o.seats ?? 1, DRESS); } finally { this.X.face_free(at, n); }
    if (!this.h) throw new FaceError('no room for another game in this module');
    this.ticks = 0; this.done++; this.said = null; this.between = 1;
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
    const n = this.enc.encodeInto(keys, new Uint8Array(this.memory.buffer, this.keys, DrawGame.KEYS)).written;
    this.X.face_hand(this.h, 0, this.hand, HAND.count * 8, this.keys, n);
    this.X.face_step(this.h, dt, VIEW | SAVES);
    this.ticks++; this.done++;
    const t1 = performance.now();
    this.draw();
    this.X.face_out_clear(this.h);   // (what the tick told its host is in its sections now)
    this.ms.step = t1 - t0; this.ms.draw = performance.now() - t1;
  }
  private order(): void { if (this.commands.length) this.handed(this.commands.take(), (p, m) => { if (!this.X.face_commands(this.h, p, m)) throw new Error(this.answer()); }); }
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
  /** what a machine keeps of the run, asked as its try begins and as it ends */
  run(): RunView { this.X.face_run_view(this.h); return readRunView(this.bytes()); }
  /** how the run ended, in a word, and why; null while it is being played */
  verdict(): Verdict | null {
    const r = this.X.face_verdict(this.h); if (!r) return null;
    const [word, why, secs, way, men, foes, kept, voided, clean] = this.answer().split('\n'), two = (s: string): number[] => s.split(' ').map(Number), [lost, of] = two(men), [foesDown, foesOf] = two(foes), [alive, held] = two(kept);
    return { won: r === 2, word, why, secs: Number(secs), approach: way === '-' ? null : way, lost, of, foesDown, foesOf, hostages: kept === '-' ? null : { alive, of: held }, void: voided === 'true', clean: clean === 'true' };
  }
  /** nobody's hands are on the seat: a hands-off player takes it (rust/game_run hands_off.rs), or with no plan it is the host's again */
  handsOff(plan: HandsOffPlan | null): void { const b = this.enc.encode(plan ? [plan.seed, plan.cap ?? 240, plan.breaker ? 1 : 0, ...plan.route ?? []].join('\n') : ''); this.handed(b, (p, n) => this.X.face_hands_off(this.h, 0, p, n)); this.handsKept.done = -1; }
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
  private turned = new Map<string, number>();
  /** turn a knob, for the visit: a load or a restart stands a game whose knobs are another moment's, and this is turned again (`turnAgain`) */
  turn(name: string, value: number): void { this.turned.set(name, value); this.commands.knob(name, value); }
  turnAgain(): void { for (const [name, value] of this.turned) this.commands.knob(name, value); }
  /** a name that crosses by its number */
  nameOf(i: number): string { this.X.face_draw_name(this.h, i); return this.answer(); }
  private draw(): void { this.drawn.take(this.h); }
}
