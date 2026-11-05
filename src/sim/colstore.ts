// A FAMILY'S STATE AS COLUMNS, IN TYPESCRIPT OR IN THE RUST MODULE — what every store of that kind is (sim/propshome.ts PropStore: the furniture;
// sim/slabshome.ts SlabStore: the leaves off their hinges), kept once.
//
// THE BYTES ARE ONE LAYOUT, WHEREVER THEY STAND: a head, then a column a number — the doubles first, then the words — every column `cap` rows long (the
// family's struct in rust/sim/src says why the columns are in the order they are). While the family's row of RUST is off the bytes are an array of the
// game's own (`own`); while it is on they are the family's part of the game's block of the module's memory (sim/state.ts), and its step runs there with
// no copy in or out: moving home is one copy of the bytes (`moveIn`, `moveOut`: the tenant's two moves). Either way a column is read as
// `d[b8 + O.x + row]` (a double) or `u[b4 + O.flags + row]` (a word), so the TypeScript step and everything that reads the family through its accessors
// reads the same values with the row on or off.
//
// ONE ARRAY IS THE FIELD. A snapshot (game/rollback.ts) takes every typed array it meets whole, so a second view of the same bytes kept beside the
// first would be the same bytes copied twice a snapshot. The words are a view asked for here (`wordsOf`); a view holds nothing of its own, so it is no
// state of any game: the few asked for last are kept (two games in a process, two stores a game), and one not among them is made again.
//
// What a store's ROWS mean (which are live, in what order, which tables say who is where) is the family's own: this file moves bytes and swaps rows.
// (Imports state.ts for its type alone: like the rest of src/sim, this is below the game.)
import type { SimState } from './state';

const KEPT = 4;
const FOR: (Float64Array | null)[] = [null, null, null, null], VIEW: (Uint32Array | null)[] = [null, null, null, null];
let next = 0;
/** the words of `d`'s bytes */
export function wordsOf(d: Float64Array): Uint32Array {
  for (let i = 0; i < KEPT; i++) if (FOR[i] === d) return VIEW[i]!;
  const i = next; next = (next + 1) % KEPT; FOR[i] = d;
  return VIEW[i] = new Uint32Array(d.buffer, d.byteOffset, d.byteLength >>> 2);
}

/** where each column starts for columns `cap` rows long behind a head of `head` bytes: a double column as an index into doubles, a word column as an
 *  index into words, from the part's start; `bytes` the part's length */
export type ColsOf<F extends string, U extends string> = Record<F | U | 'bytes', number>;
export function colsOf<F extends string, U extends string>(head: number, f64: readonly F[], u32: readonly U[], cap: number): ColsOf<F, U> {
  const o: Record<string, number> = {}; let at = head;
  for (const k of f64) { o[k] = at / 8; at += cap * 8; }
  for (const k of u32) { o[k] = at / 4; at += cap * 4; }
  o.bytes = at; return o as ColsOf<F, U>;
}

/** THE COLUMNS OF ONE FAMILY OF ONE GAME: doubles named `F`, words named `U`. */
export class ColStore<F extends string, U extends string> {
  /** where each column starts (`O.x + row` into the doubles from `b8`, `O.flags + row` into the words from `b4`): asked at every use, as the arrays are
   *  (`widen` lays the columns out anew) */
  O: ColsOf<F, U>;
  /** the game's own array of them while they are not at home in the module */
  private own: Float64Array;
  /** the game's block of the module's memory while the columns are at home there, else null */
  home: SimState | null = null;
  /** where the part starts where the columns stand now: an index into the doubles, and into the words (0 in `own`) */
  b8 = 0; b4 = 0;
  /** `cap`: rows a column (the module's own number where the bytes are to be the module's part as they stand; a level of more rows than that keeps
   *  wider columns and stays in TypeScript) */
  cap: number;
  constructor(head: number, f64: readonly F[], u32: readonly U[], cap: number) {
    this.cap = cap; this.O = colsOf(head, f64, u32, cap); this.own = new Float64Array(this.O.bytes >>> 3);
  }
  /** the doubles and the words the columns are in now (asked at every use: a block's views die when the block moves) */
  get d(): Float64Array { const h = this.home; return h !== null ? h.d : this.own; }
  get u(): Uint32Array { const h = this.home; return h !== null ? h.w : wordsOf(this.own); }
  /** rows a and b change places in these columns (the family's tables follow in the family's own hands) */
  protected swapCols(a: number, b: number, f64: readonly F[], u32: readonly U[]): void {
    const d = this.d, u = this.u, O = this.O, B = this.b8, W = this.b4;
    for (const k of f64) { const o = B + O[k]; const t = d[o + a]; d[o + a] = d[o + b]; d[o + b] = t; }
    for (const k of u32) { const o = W + O[k]; const t = u[o + a]; u[o + a] = u[o + b]; u[o + b] = t; }
  }
  /** MORE ROWS THAN THE COLUMNS HOLD (a family whose things come and go: the thrown things' pool): columns `cap` rows long in their place, the head and
   *  every column's rows carried over. The module's part is the module's own length, so wider columns are no part of its block: the bytes come out of
   *  it first (the family's tenant then no longer `wants` it home) and are the game's own array from here on. */
  protected widen(head: number, f64: readonly F[], u32: readonly U[], cap: number): void {
    if (cap <= this.cap) return;
    if (this.home !== null) this.moveOut(this.home, this.b8 << 3);
    const was = this.own, O0 = this.O, n0 = this.cap, O1 = colsOf(head, f64, u32, cap), now = new Float64Array(O1.bytes >>> 3);
    const u0 = new Uint32Array(was.buffer, was.byteOffset, was.byteLength >>> 2), u1 = new Uint32Array(now.buffer);   // (views of this one move: not the kept ones)
    u1.set(u0.subarray(0, head >>> 2));
    for (const k of f64) now.set(was.subarray(O0[k], O0[k] + n0), O1[k]);
    for (const k of u32) u1.set(u0.subarray(O0[k], O0[k] + n0), O1[k]);
    this.own = now; this.O = O1; this.cap = cap;
  }
  /** the bytes of the part as they stand (every column, the head and the tables) */
  bytes(): Uint8Array { const u = this.u; return new Uint8Array(u.buffer, u.byteOffset + this.b4 * 4, this.O.bytes).slice(); }
  /** …and put back */
  put(b: Uint8Array): void { const u = this.u; new Uint8Array(u.buffer, u.byteOffset + this.b4 * 4, this.O.bytes).set(b); }
  /** THE TENANT'S TWO MOVES (sim/state.ts simTenant `into` and `outOf`): the bytes into the part of block `st` that starts at byte `at`, and out of it
   *  again, the part left zeros (a part nobody is at home in is zeros: rust/sim/src/state.rs) */
  moveIn(st: SimState, at: number): void { const b = this.bytes(); this.home = st; this.b8 = at >>> 3; this.b4 = at >>> 2; this.put(b); }
  moveOut(st: SimState, at: number): void {
    if (this.home === null) return;
    const b = this.bytes(); st.w.fill(0, at >>> 2, (at + this.O.bytes) >>> 2);
    this.home = null; this.b8 = 0; this.b4 = 0; this.put(b);
  }
}

/** WHERE TWO TAKES OF A STORE'S BYTES DIFFER, column by column and row by row over the first `rows` rows — the doubles by their value (a NaN is a NaN,
 *  −0 is not 0), the words by theirs; `who(u, row)` names a row. → at most `most` lines, none if those columns are the same */
export function colDiff<F extends string, U extends string>(O: ColsOf<F, U>, f64: readonly F[], u32: readonly U[], a: Uint8Array, b: Uint8Array, rows: number, who: (u: Uint32Array, r: number) => string, most: number, out: string[] = []): string[] {
  const first = O[u32[0]], da = new Float64Array(a.buffer, a.byteOffset, first >>> 1), db = new Float64Array(b.buffer, b.byteOffset, first >>> 1), ua = new Uint32Array(a.buffer, a.byteOffset, O.bytes >>> 2), ub = new Uint32Array(b.buffer, b.byteOffset, O.bytes >>> 2);
  for (const k of f64) for (let r = 0; r < rows && out.length < most; r++) if (!Object.is(da[O[k] + r], db[O[k] + r])) out.push(`row ${r} (${who(ua, r)}).${k} ${da[O[k] + r]} | ${db[O[k] + r]}`);
  for (const k of u32) for (let r = 0; r < rows && out.length < most; r++) if (ua[O[k] + r] !== ub[O[k] + r]) out.push(`row ${r}.${k} ${ua[O[k] + r]} | ${ub[O[k] + r]}`);
  return out;
}
