// THE TAPE OF A GAME IN THE MODULE: what the module was handed, in order, so that handing it over again plays the same run (host/drawgame.ts writes one as it
// goes and is fed one back). Nothing of the picture is on it, and nothing here knows the game: a tape is the bytes of five calls.
//
//   START what the game was started from · STEP one tick's hand · ORDER commands, at the next step or between two · HANDS a hands-off player's plan, or none ·
//   MARK what the game came to by a tick, to hold a walk to · END why the tape stopped
//
// A hand is kept as it was handed (src/draw/hand.ts), its matrices too, and never made again from a camera: a walk is exact in any browser. A step carries only
// the numbers that moved since the step before, each as four bytes where four hold it.

/** `version`: a tape of another one is refused */
export const HANDTAPE = { magic: 0x31485054, version: 1 };
const START = 1, STEP = 2, ORDER = 3, HANDS = 4, MARK = 5, END = 6;
const DT = 1, KEYS = 2, WIDE = 4;

/** a part of a start that is the build's own: what it should be, for a walk to hand over its own and be held to it */
export interface PartOf { name: string; length: number; hash: number }
/** what a game was started from */
export interface TapeStart { build: string; levelName: string; level: Uint8Array; words: string; parts: PartOf[]; seed: number; seats: number; flags: number; commands: Uint8Array }
/** what the game came to by a tick: rolls made of its die, the hash of its parts' hashes, and the hash of its whole state where that was asked (the dearer one) */
export interface TapeMark { tick: number; die: number; parts: number; state: number | null }
/** ticks between two marks, and between two that have the whole state */
export const MARKS = { every: 60, whole: 600 };
export type TapeRow = { kind: 'step'; tick: number; dt: number; hand: Float64Array; keys: string } | { kind: 'order'; tick: number; bytes: Uint8Array }
  | { kind: 'hands'; tick: number; plan: Uint8Array } | { kind: 'mark'; mark: TapeMark } | { kind: 'end'; tick: number; why: string };

/** FNV-1a over bytes (a part's hash) */
export function hashOf(b: Uint8Array): number { let h = 0x811c9dc5; for (let i = 0; i < b.length; i++) h = Math.imul(h ^ b[i], 0x01000193); return h >>> 0; }

export class HandTape {
  /** ticks on the tape */
  ticks = 0;
  /** bytes on the tape */
  get length(): number { return this.sent + this.n; }
  private b = new Uint8Array(65536); private d = new DataView(this.b.buffer); private n = 0; private sent = 0;
  private was: Float64Array; private keys = ''; private dt = NaN; private over = false; private enc = new TextEncoder();

  /** `count`: the numbers of a hand */
  constructor(readonly count: number) { this.was = new Float64Array(count).fill(NaN); this.u32(HANDTAPE.magic); this.u32(HANDTAPE.version); this.u32(count); }

  private room(k: number): void { if (this.n + k <= this.b.length) return; let size = this.b.length * 2; while (size < this.n + k) size *= 2; const b = new Uint8Array(size); b.set(this.b.subarray(0, this.n)); this.b = b; this.d = new DataView(b.buffer); }
  private u8(v: number): void { this.room(1); this.b[this.n++] = v; }
  private u32(v: number): void { this.room(4); this.d.setUint32(this.n, v, true); this.n += 4; }
  private f64(v: number): void { this.room(8); this.d.setFloat64(this.n, v, true); this.n += 8; }
  private bytes(v: Uint8Array): void { this.u32(v.length); this.room(v.length); this.b.set(v, this.n); this.n += v.length; }
  private text(v: string): void { this.bytes(this.enc.encode(v)); }

  start(s: TapeStart): void {
    this.u8(START); this.text(s.build); this.text(s.levelName); this.bytes(s.level); this.text(s.words); this.u32(s.parts.length);
    for (const p of s.parts) { this.text(p.name); this.u32(p.length); this.u32(p.hash); }
    this.f64(s.seed); this.u32(s.seats); this.u32(s.flags); this.bytes(s.commands);
  }
  /** one tick's hand, before it is stepped */
  step(dt: number, hand: Float64Array, keys: string): void {
    if (this.over) return;
    const { count, was } = this; let moved0 = 0, moved1 = 0, wide0 = 0, wide1 = 0;
    for (let i = 0; i < count; i++) { const v = hand[i]; if (Object.is(v, was[i])) continue; was[i] = v; const bit = 1 << (i & 31), wide = !Object.is(Math.fround(v), v) || v !== v;
      if (i < 32) { moved0 |= bit; if (wide) wide0 |= bit; } else { moved1 |= bit; if (wide) wide1 |= bit; } }
    const says = (Object.is(dt, this.dt) ? 0 : DT) | (keys === this.keys ? 0 : KEYS) | (wide0 || wide1 ? WIDE : 0);
    this.room(32 + count * 8); this.u8(STEP); this.u8(says); this.u32(moved0); this.u32(moved1); if (says & WIDE) { this.u32(wide0); this.u32(wide1); }
    if (says & DT) this.f64(this.dt = dt);
    if (says & KEYS) this.text(this.keys = keys);
    for (let i = 0; i < count; i++) { const bit = 1 << (i & 31); if (!((i < 32 ? moved0 : moved1) & bit)) continue;
      if ((i < 32 ? wide0 : wide1) & bit) { this.d.setFloat64(this.n, was[i], true); this.n += 8; } else { this.d.setFloat32(this.n, was[i], true); this.n += 4; } }
    this.ticks++;
  }
  /** commands the game took, at the next step or between two */
  order(bytes: Uint8Array): void { if (!this.over) { this.u8(ORDER); this.bytes(bytes); } }
  /** a hands-off player took the seat, or with no plan gave it back */
  hands(plan: Uint8Array): void { if (!this.over) { this.u8(HANDS); this.bytes(plan); } }
  mark(m: TapeMark): void { if (this.over) return; this.u8(MARK); this.u32(m.tick); this.f64(m.die); this.u32(m.parts); this.u8(m.state === null ? 0 : 1); if (m.state !== null) this.u32(m.state); }
  /** nothing after this is on the tape */
  end(why: string): void { if (this.over) return; this.u8(END); this.text(why); this.over = true; }

  /** the tape as it stands */
  whole(): Uint8Array { if (this.sent) throw new Error('part of this tape was taken'); return this.b.slice(0, this.n); }
  /** what has come onto the tape since the last time (a keeper's) */
  take(): Uint8Array { const out = this.b.slice(0, this.n); this.sent += this.n; this.n = 0; return out; }
}

/** where a walk is on its tape */
export interface TapePlace { readonly at: number; readonly ticks: number; readonly hand: Float64Array; readonly keys: string; readonly dt: number }

/** why a tape cannot be walked here */
export class TapeRefused extends Error { }

/** A TAPE READ BACK: its start, then its rows in the order they were written. A tape cut off in a row ends at the row before */
export class HandWalk {
  readonly start: TapeStart;
  /** ticks read */
  ticks = 0;
  private d: DataView; private at = 0; private dec = new TextDecoder();
  private hand: Float64Array; private keys = ''; private dt = 1 / 60;

  /** this walk's place, to come back to (`back`) with the game's moment of the same tick */
  place(): TapePlace { return { at: this.at, ticks: this.ticks, hand: this.hand.slice(), keys: this.keys, dt: this.dt }; }
  back(p: TapePlace): void { this.at = p.at; this.ticks = p.ticks; this.hand.set(p.hand); this.keys = p.keys; this.dt = p.dt; }

  /** `count`: the numbers of this build's hand */
  constructor(private b: Uint8Array, count: number) {
    this.d = new DataView(b.buffer, b.byteOffset, b.byteLength);
    if (b.length < 12 || this.u32() !== HANDTAPE.magic) throw new TapeRefused('this is no tape of a run');
    const version = this.u32(), numbers = this.u32();
    if (version !== HANDTAPE.version) throw new TapeRefused(`this tape is of version ${version}, and this build walks version ${HANDTAPE.version}`);
    if (numbers !== count) throw new TapeRefused(`this tape's hand has ${numbers} numbers, and this build's has ${count}`);
    this.hand = new Float64Array(count).fill(NaN);
    try {
      if (this.u8() !== START) throw new RangeError();
      const build = this.text(), levelName = this.text(), level = this.bytes(), words = this.text(), parts: PartOf[] = [];
      for (let i = this.u32(); i > 0; i--) parts.push({ name: this.text(), length: this.u32(), hash: this.u32() });
      this.start = { build, levelName, level, words, parts, seed: this.f64(), seats: this.u32(), flags: this.u32(), commands: this.bytes() };
    } catch (e) { if (e instanceof RangeError) throw new TapeRefused('this tape has no start'); throw e; }
  }

  private u8(): number { return this.d.getUint8(this.at++); }
  private u32(): number { const v = this.d.getUint32(this.at, true); this.at += 4; return v; }
  private f64(): number { const v = this.d.getFloat64(this.at, true); this.at += 8; return v; }
  private bytes(): Uint8Array { const n = this.u32(); if (this.at + n > this.b.length) throw new RangeError(); this.at += n; return this.b.slice(this.at - n, this.at); }
  private text(): string { return this.dec.decode(this.bytes()); }

  /** the parts a walk would hand over, held to the tape's: the first that is another's, null where all are its own */
  other(parts: readonly PartOf[]): string | null { for (const p of this.start.parts) { const mine = parts.find(q => q.name === p.name); if (!mine || mine.length !== p.length || mine.hash !== p.hash) return p.name; } return null; }

  /** the next row, null at the end. A step's hand is this walk's own, written over by the next */
  next(): TapeRow | null { const from = this.at; if (from >= this.b.length) return null; try { return this.row(); } catch (e) { if (!(e instanceof RangeError)) throw e; this.at = this.b.length; return null; } }
  private row(): TapeRow {
    const kind = this.u8(), tick = this.ticks;
    if (kind === ORDER) return { kind: 'order', tick, bytes: this.bytes() };
    if (kind === HANDS) return { kind: 'hands', tick, plan: this.bytes() };
    if (kind === MARK) { const at = this.u32(), die = this.f64(), parts = this.u32(); return { kind: 'mark', mark: { tick: at, die, parts, state: this.u8() ? this.u32() : null } }; }
    if (kind === END) return { kind: 'end', tick, why: this.text() };
    if (kind !== STEP) throw new TapeRefused(`this tape has a row of no kind (${kind}) at tick ${tick}`);
    const says = this.u8(), moved0 = this.u32(), moved1 = this.u32(), wide0 = says & WIDE ? this.u32() : 0, wide1 = says & WIDE ? this.u32() : 0, { hand } = this;
    if (says & DT) this.dt = this.f64();
    if (says & KEYS) this.keys = this.text();
    for (let i = 0; i < hand.length; i++) { const bit = 1 << (i & 31); if (!((i < 32 ? moved0 : moved1) & bit)) continue;
      if ((i < 32 ? wide0 : wide1) & bit) hand[i] = this.f64(); else { hand[i] = this.d.getFloat32(this.at, true); this.at += 4; } }
    this.ticks++;
    return { kind: 'step', tick, dt: this.dt, hand, keys: this.keys };
  }
}
