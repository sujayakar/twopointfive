// THE TAPE OF A GAME IN THE MODULE: what the module was handed, in order, so that handing it over again plays the same run (host/drawgame.ts writes one as it
// goes and is fed one back). Nothing of the picture is on it, and nothing here knows the game: a tape is the bytes of five calls.
//
//   START what the game was started from · STEP one tick's hand · ORDER commands, at the next step or between two · HANDS a hands-off player's plan, or none ·
//   MARK what the game came to by a tick, to hold a walk to · FIGURES who stood where by it · TRY the mission began again · END why the tape stopped
//
// A hand is kept as it was handed (src/draw/hand.ts: the input's numbers, then the lens's), its matrices too, and never made again from a camera: a walk is exact in any browser. A step carries only
// the numbers that moved since the step before, each as four bytes where four hold it.

import { HAND_NUMBERS } from '../draw/hand';
import { HAND } from '../draw/input';
import type { HandsOffPlan } from '../host/benchScenes';

/** `version`: a tape of another one is refused, and one whose hand has other numbers (`HAND`) */
export const HANDTAPE = { magic: 0x31485054, version: 4 };
const START = 1, STEP = 2, ORDER = 3, HANDS = 4, MARK = 5, END = 6, TRY = 7, FIGURES = 8;
const DT = 1, KEYS = 2, WIDE = 4;
/** a step's numbers, each with a bit of two words */
const count = HAND_NUMBERS;
if (count > 64) throw new Error('a tape has 64 bits to say which of a hand\u2019s numbers moved, and a hand has more');

/** a part of a start that is the build's own: what it should be, for a walk to hand over its own and be held to it */
export interface PartOf { name: string; length: number; hash: number }
/** what a game was started from */
export interface TapeStart { build: string; levelName: string; level: Uint8Array; words: string; parts: PartOf[]; seed: number; seats: number; flags: number; kept: string }
/** what the game came to by a tick: rolls made of its die, the word of where everybody stands, and the hash of every part of it (null: a tape made before every mark had it) */
export interface TapeMark { tick: number; die: number; stands: number; state: number | null }
/** WHO STANDS WHERE by a tick, and the rounds the player has fired: what came of the play, whatever the module that played it keeps its state in (a mark's hashes are of one module's own layout) */
export interface TapeFigures { tick: number; shots: number; men: { id: number; x: number; z: number; down: boolean }[] }
/** metres a man may stand from where he stood, and it is the same play */
const HAND_BREADTH = 0.1;
/** the first thing in which two tellings of who stands where are not the same play, in words; null: they are */
export function figuresPart(a: TapeFigures, b: TapeFigures): string | null {
  if (a.shots !== b.shots) return `${b.shots} rounds fired, ${a.shots} on the tape`;
  const men = new Map(b.men.map(m => [m.id, m]));
  for (const m of a.men) { const n = men.get(m.id); if (!n) return `man ${m.id} is not there`; men.delete(m.id);
    if (n.down !== m.down) return `man ${m.id} is ${n.down ? 'down' : 'up'}, ${m.down ? 'down' : 'up'} on the tape`;
    if (Math.hypot(n.x - m.x, n.z - m.z) > HAND_BREADTH) return `man ${m.id} stands at ${n.x} ${n.z}, at ${m.x} ${m.z} on the tape`; }
  for (const id of men.keys()) return `man ${id} is not on the tape`;
  return null;
}
/** ticks between two marks */
export const MARKS = { every: 60 };
export type TapeRow = { kind: 'step'; tick: number; dt: number; hand: Float64Array; keys: string } | { kind: 'order'; tick: number; bytes: Uint8Array }
  | { kind: 'hands'; tick: number; plan: HandsOffPlan | null } | { kind: 'mark'; mark: TapeMark } | { kind: 'figures'; figures: TapeFigures } | { kind: 'try'; tick: number } | { kind: 'end'; tick: number; why: string };

/** FNV-1a over bytes (a part's hash) */
export function hashOf(b: Uint8Array): number { let h = 0x811c9dc5; for (let i = 0; i < b.length; i++) h = Math.imul(h ^ b[i], 0x01000193); return h >>> 0; }

export class HandTape {
  /** ticks on the tape, and the seconds of play they are */
  ticks = 0; seconds = 0;
  /** the tick the try that is on began after */
  began = 0;
  /** bytes on the tape */
  get length(): number { return this.n; }
  private b = new Uint8Array(65536); private d = new DataView(this.b.buffer); private n = 0;
  private was = new Float64Array(count).fill(NaN); private keys = ''; private dt = NaN; private over = false; private enc = new TextEncoder();

  constructor() { this.u32(HANDTAPE.magic); this.u32(HANDTAPE.version); this.u32(count); this.u32(HAND); }

  private room(k: number): void { if (this.n + k <= this.b.length) return; let size = this.b.length * 2; while (size < this.n + k) size *= 2; const b = new Uint8Array(size); b.set(this.b.subarray(0, this.n)); this.b = b; this.d = new DataView(b.buffer); }
  private u8(v: number): void { this.room(1); this.b[this.n++] = v; }
  private u32(v: number): void { this.room(4); this.d.setUint32(this.n, v, true); this.n += 4; }
  private f64(v: number): void { this.room(8); this.d.setFloat64(this.n, v, true); this.n += 8; }
  private bytes(v: Uint8Array): void { this.u32(v.length); this.room(v.length); this.b.set(v, this.n); this.n += v.length; }
  private text(v: string): void { this.bytes(this.enc.encode(v)); }

  start(s: TapeStart): void {
    this.u8(START); this.text(s.build); this.text(s.levelName); this.bytes(s.level); this.text(s.words); this.u32(s.parts.length);
    for (const p of s.parts) { this.text(p.name); this.u32(p.length); this.u32(p.hash); }
    this.f64(s.seed); this.u32(s.seats); this.u32(s.flags); this.text(s.kept);
  }
  /** one tick's hand, before it is stepped */
  step(dt: number, hand: Float64Array, keys: string): void {
    if (this.over) return;
    const { was } = this; let moved0 = 0, moved1 = 0, wide0 = 0, wide1 = 0;
    for (let i = 0; i < count; i++) { const v = hand[i]; if (Object.is(v, was[i])) continue; was[i] = v; const bit = 1 << (i & 31), wide = !Object.is(Math.fround(v), v) || v !== v;
      if (i < 32) { moved0 |= bit; if (wide) wide0 |= bit; } else { moved1 |= bit; if (wide) wide1 |= bit; } }
    const says = (Object.is(dt, this.dt) ? 0 : DT) | (keys === this.keys ? 0 : KEYS) | (wide0 || wide1 ? WIDE : 0);
    this.room(32 + count * 8); this.u8(STEP); this.u8(says); this.u32(moved0); this.u32(moved1); if (says & WIDE) { this.u32(wide0); this.u32(wide1); }
    if (says & DT) this.f64(this.dt = dt);
    if (says & KEYS) { this.text(this.keys = keys); this.room(count * 8); }
    for (let i = 0; i < count; i++) { const bit = 1 << (i & 31); if (!((i < 32 ? moved0 : moved1) & bit)) continue;
      if ((i < 32 ? wide0 : wide1) & bit) { this.d.setFloat64(this.n, was[i], true); this.n += 8; } else { this.d.setFloat32(this.n, was[i], true); this.n += 4; } }
    this.ticks++; this.seconds += dt;
  }
  /** commands the game took, at the next step or between two */
  order(bytes: Uint8Array): void { if (!this.over) { this.u8(ORDER); this.bytes(bytes); } }
  /** a hands-off player took the seat, or with no plan gave it back */
  hands(plan: HandsOffPlan | null): void { if (!this.over) { this.u8(HANDS); this.text(plan ? [plan.seed, plan.cap ?? 240, plan.breaker ? 1 : 0, ...plan.route ?? []].join('\n') : ''); } }
  mark(m: TapeMark): void { if (this.over) return; this.u8(MARK); this.u32(m.tick); this.f64(m.die); this.u32(m.stands); this.u8(m.state === null ? 0 : 1); if (m.state !== null) this.u32(m.state); }
  figures(f: TapeFigures): void {
    if (this.over) return; this.u8(FIGURES); this.u32(f.tick); this.u32(f.shots); this.u32(f.men.length); this.room(f.men.length * 13);
    for (const m of f.men) { this.d.setUint32(this.n, m.id, true); this.d.setFloat32(this.n + 4, m.x, true); this.d.setFloat32(this.n + 8, m.z, true); this.b[this.n + 12] = m.down ? 1 : 0; this.n += 13; }
  }
  /** the mission began again, the tape goes on */
  again(): void { if (!this.over) { this.u8(TRY); this.began = this.ticks; } }
  /** nothing after this is on the tape */
  end(why: string): void { if (this.over) return; this.u8(END); this.text(why); this.over = true; }

  /** the tape as it stands, from byte `at` of it */
  whole(at = 0): Uint8Array { return this.b.slice(at, this.n); }
}

/** where a walk is on its tape */
export interface TapePlace { readonly at: number; readonly ticks: number; readonly hand: Float64Array; readonly keys: string; readonly dt: number }

/** why a tape cannot be walked here, as the player is told it */
export class TapeRefused extends Error { }
/** … one of another build's making */
export const ANOTHER = 'This replay is from a different version of the game, so it can\u2019t be played on this one.';

/** A TAPE READ BACK: its start, then its rows in the order they were written. A tape cut off in a row ends at the row before */
export class HandWalk {
  readonly start: TapeStart;
  /** ticks read */
  ticks = 0;
  private d: DataView; private at = 0; private dec = new TextDecoder();
  private hand: Float64Array; private keys = ''; private dt = 1 / 60;

  /** the hand of the last step read (this walk's own) */
  get last(): Float64Array { return this.hand; }
  /** this walk's place, to come back to (`back`) with the game's moment of the same tick */
  place(): TapePlace { return { at: this.at, ticks: this.ticks, hand: this.hand.slice(), keys: this.keys, dt: this.dt }; }
  back(p: TapePlace): void { this.at = p.at; this.ticks = p.ticks; this.hand.set(p.hand); this.keys = p.keys; this.dt = p.dt; }

  /** `any`: whatever its hand's numbers are (a tool's that reads what came of the run, and hands nothing over again) */
  constructor(private b: Uint8Array, any = false) {
    this.d = new DataView(b.buffer, b.byteOffset, b.byteLength);
    if (b.length < 12 || this.u32() !== HANDTAPE.magic) throw new TapeRefused('That file isn\u2019t a replay of this game.');
    const version = this.u32(), numbers = this.u32(), hand = version >= 3 ? this.u32() : 0;
    if (any ? version < 2 || version > HANDTAPE.version : version !== HANDTAPE.version || hand !== HAND) throw new TapeRefused(ANOTHER);
    this.hand = new Float64Array(numbers).fill(NaN);
    try {
      if (this.u8() !== START) throw new RangeError();
      const build = this.text(), levelName = this.text(), level = this.bytes(), words = this.text(), parts: PartOf[] = [];
      for (let i = this.u32(); i > 0; i--) parts.push({ name: this.text(), length: this.u32(), hash: this.u32() });
      this.start = { build, levelName, level, words, parts, seed: this.f64(), seats: this.u32(), flags: this.u32(), kept: this.text() };
    } catch (e) { if (e instanceof RangeError) throw new TapeRefused('This replay is cut off before it begins.'); throw e; }
  }

  private u8(): number { return this.d.getUint8(this.at++); }
  private u32(): number { const v = this.d.getUint32(this.at, true); this.at += 4; return v; }
  private f32(): number { const v = this.d.getFloat32(this.at, true); this.at += 4; return v; }
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
    if (kind === HANDS) { const said = this.text(), [seed, cap, breaker, ...route] = said.split('\n'); return { kind: 'hands', tick, plan: said ? { seed: Number(seed), cap: Number(cap), breaker: breaker !== '0', route } : null }; }
    if (kind === MARK) { const at = this.u32(), die = this.f64(), stands = this.u32(); return { kind: 'mark', mark: { tick: at, die, stands, state: this.u8() ? this.u32() : null } }; }
    if (kind === FIGURES) { const f: TapeFigures = { tick: this.u32(), shots: this.u32(), men: [] }; for (let n = this.u32(); n > 0; n--) f.men.push({ id: this.u32(), x: this.f32(), z: this.f32(), down: this.u8() === 1 }); return { kind: 'figures', figures: f }; }
    if (kind === TRY) return { kind: 'try', tick };
    if (kind === END) return { kind: 'end', tick, why: this.text() };
    if (kind !== STEP) throw new TapeRefused(`this tape has a row of no kind (${kind}) at tick ${tick}`);
    const says = this.u8(), moved0 = this.u32(), moved1 = this.u32(), wide0 = says & WIDE ? this.u32() : 0, wide1 = says & WIDE ? this.u32() : 0, { hand } = this;
    if (says & DT) this.dt = this.f64();
    if (says & KEYS) this.keys = this.text();
    for (let i = 0; i < hand.length; i++) { const bit = 1 << (i & 31); if (!((i < 32 ? moved0 : moved1) & bit)) continue;
      if ((i < 32 ? wide0 : wide1) & bit) hand[i] = this.f64(); else hand[i] = this.f32(); }
    this.ticks++;
    return { kind: 'step', tick, dt: this.dt, hand, keys: this.keys };
  }
}
