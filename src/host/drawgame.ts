// A GAME IN THE MODULE, AS A PAGE THAT DRAWS IT HAS IT: started, handed a player's input, stepped, and read by what it leaves to be drawn (src/draw). The page
// holds its number and nothing of it.
import { FaceError, type Face, type Fetched } from './face';
import { ALL, Drawn, drawRefused, type DrawFace } from '../draw/drawn';
import { Commands } from '../draw/commands';
import { HAND } from '../draw/hand';

/** what a start is handed beside the level (rust/game_start/src/handed.rs PARTS) */
export interface Start { level: Uint8Array; words: string; mannequin: Uint8Array; mannequinBin: Uint8Array; cast: Uint8Array; castLook: Uint8Array; seed: number; seats?: number }

/** `face_start`'s flags: the rows that dress the picture are up · the game works its own senses out */
const DRESS = 1, SENSES = 2;
/** `face_step`'s: the box world listed · the lights' envelopes · a rails level's view · the save controller */
const LISTED = 1, LIGHTS = 2, VIEW = 4, SAVES = 8;

export class DrawGame {
  readonly X: Face;
  readonly memory: WebAssembly.Memory;
  readonly drawn: Drawn;
  h = 0;
  /** what a player or a panel does that is not input: the next step takes it */
  readonly commands = new Commands();
  /** ticks stepped since the start, and ms of the last one by part */
  ticks = 0;
  readonly ms = { step: 0, draw: 0 };
  private hand = 0; private keys = 0; private numbers = new Float64Array(0);
  private static readonly KEYS = 2048;
  private enc = new TextEncoder(); private dec = new TextDecoder();

  constructor(f: Fetched) {
    this.X = f.exports as unknown as Face; this.memory = f.exports.memory as WebAssembly.Memory;
    const no = drawRefused(this.X as unknown as DrawFace); if (no) throw new FaceError(`${no} (tools/blitz/face-build.ts builds this checkout's)`);
    this.drawn = new Drawn(this.X as unknown as DrawFace, this.memory, ALL);
    this.hand = this.X.face_alloc(HAND.count * 8) >>> 0; this.keys = this.X.face_alloc(DrawGame.KEYS) >>> 0;
  }

  private handed<T>(b: Uint8Array, f: (p: number, n: number) => T): T { const p = this.X.face_alloc(b.length) >>> 0; new Uint8Array(this.memory.buffer, p, b.length).set(b); try { return f(p, b.length); } finally { this.X.face_free(p, b.length); } }
  private answer(): string { const p = this.X.face_out_ptr() >>> 0, n = this.X.face_out_len() >>> 0; return this.dec.decode(new Uint8Array(this.memory.buffer, p, n)); }
  /** a call that may stop on a panic: its words, with the file and line */
  private call<T>(what: string, f: () => T): T {
    try { return f(); } catch (e) {
      let words = ''; try { const p = this.X.face_panic_ptr() >>> 0, n = this.X.face_panic_len() >>> 0; words = this.dec.decode(new Uint8Array(this.memory.buffer, p, n)); } catch { /* the trap's own words, then */ }
      throw new Error(words ? `the game stopped in ${what}: ${words}` : `${what}: ${String((e as Error)?.message ?? e)}`); }
  }

  start(o: Start): void {
    const parts = [o.level, this.enc.encode(o.words), o.mannequin, o.mannequinBin, o.cast, o.castLook, new Uint8Array(0)], n = 4 + parts.reduce((s, p) => s + 4 + p.length, 0);
    const at = this.X.face_alloc(n) >>> 0, b = new Uint8Array(this.memory.buffer, at, n), d = new DataView(this.memory.buffer, at, n); let q = 4;
    d.setUint32(0, parts.length, true);
    for (const p of parts) { d.setUint32(q, p.length, true); b.set(p, q + 4); q += 4 + p.length; }
    if (this.h) { this.X.face_drop(this.h); this.h = 0; }
    try { this.h = this.call('the start', () => this.X.face_start(at, n, -1, o.seed, o.seats ?? 1, DRESS | SENSES)); } finally { this.X.face_free(at, n); }
    if (!this.h) throw new FaceError('no room for another game in this module');
    this.ticks = 0; this.said = null;
    // (room asked for and given back: the allocator keeps it, and a step seldom has to grow the memory)
    this.X.face_reserve(64 * 1048576);
    this.draw();
  }

  /** the numbers of the next hand, to be written by whoever samples the player */
  numbersOf(): Float64Array { if (this.numbers.buffer !== this.memory.buffer) this.numbers = new Float64Array(this.memory.buffer, this.hand, HAND.count); return this.numbers; }
  /** ONE TICK on the hand as it stands: `keys` = the codes held, a line feed, the codes that went down */
  step(dt: number, keys: string): void {
    const t0 = performance.now();
    if (this.commands.length) this.handed(this.commands.take(), (p, m) => { if (!this.call('a command', () => this.X.face_commands(this.h, p, m))) throw new Error(this.answer()); });
    const n = this.enc.encodeInto(keys, new Uint8Array(this.memory.buffer, this.keys, DrawGame.KEYS)).written;
    this.X.face_hand(this.h, 0, this.hand, HAND.count * 8, this.keys, n);
    this.call('a tick', () => this.X.face_step(this.h, dt, LISTED | LIGHTS | VIEW | SAVES));
    this.ticks++;
    const t1 = performance.now();
    this.draw();
    this.X.face_out_clear(this.h);   // (what the tick told its host is in its sections now)
    this.ms.step = t1 - t0; this.ms.draw = performance.now() - t1;
  }
  /** A FRAME IS DRAWN, `a` of the way from the tick before to the last */
  frame(a: number, focus: ArrayLike<number>): void { this.call('drawing', () => this.drawn.frame(this.h, a, focus)); }
  /** what this page says of its look, when it is another than it last said */
  look(v: Float64Array): void {
    const was = this.said; if (was) { let i = 0; while (i < v.length && v[i] === was[i]) i++; if (i === v.length) return; }
    this.said = v.slice(); const at = this.X.face_alloc(v.length * 8) >>> 0;
    new Float64Array(this.memory.buffer, at, v.length).set(v);
    try { this.call('the look', () => this.X.face_draw_look(this.h, at, v.length)); } finally { this.X.face_free(at, v.length * 8); }
    this.draw();
  }
  private said: Float64Array | null = null;
  /** a name that crosses by its number */
  nameOf(i: number): string { this.X.face_draw_name(this.h, i); return this.answer(); }
  private draw(): void { this.call('drawing', () => this.drawn.take(this.h)); }
}
