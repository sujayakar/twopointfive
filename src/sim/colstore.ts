// A FAMILY'S STATE AS COLUMNS, IN THE GAME'S BLOCK — what every store of that kind is (sim/propshome.ts PropStore: the furniture; sim/slabshome.ts
// SlabStore: the leaves off their hinges; sim/itemshome.ts ItemStore: the thrown things), kept once.
//
// THE BYTES ARE ONE LAYOUT, AND THEY ARE THE FAMILY'S PART OF THE GAME'S BLOCK (sim/state.ts) from the store's first line: a head, then a column a number
// — the doubles first, then the words — every column `cap` rows long (the family's struct in rust/sim/src says why the columns are in the order they
// are). The store is made over the game's SimState and has no array of its own; a column is read as `d[b8 + O.x + row]` (a double) or
// `u[b4 + O.flags + row]` (a word), `d` and `u` being the block as it stands NOW (bound in the module or parked: asked at every use). The family's step
// runs over those bytes whichever side takes it — the Rust module's (its row of RUST on) with no copy in or out, or its TypeScript twin — and everything
// that reads the family through its accessors reads the same values.
//
// What a store's ROWS mean (which are live, in what order, which tables say who is where) is the family's own: this file names bytes and swaps rows.
// (Imports state.ts alone: like the rest of src/sim, this is below the game.)
import { simPart, type SimState } from './state';

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
  /** where each column starts (`O.x + row` into the doubles from `b8`, `O.flags + row` into the words from `b4`) */
  O: ColsOf<F, U>;
  /** the game's block, which the columns are a part of for the game's life (never a view of it: the block moves) */
  readonly home: SimState;
  /** where the part starts in the block: an index into its doubles, and into its words */
  readonly b8: number; readonly b4: number;
  /** `cap`: rows a column — the module's own number, so that the bytes are the module's part as they stand */
  cap: number;
  /** `st`: the game's block (or a lone system's own); `part`, `bit`, `name`: which part of it the family's is (sim/state.ts P_…), its bit in the head and
   *  what it is called. The part is zeros until the family writes it. */
  constructor(st: SimState, part: number, bit: number, name: string, head: number, f64: readonly F[], u32: readonly U[], cap: number) {
    this.cap = cap; this.O = colsOf(head, f64, u32, cap); this.home = st; st.keep(bit, name);
    const at = simPart(part); this.b8 = at >>> 3; this.b4 = at >>> 2;
  }
  /** the doubles and the words the columns are in now (asked at every use: a block's views die when the block moves) */
  get d(): Float64Array { return this.home.d; }
  get u(): Uint32Array { return this.home.w; }
  /** rows a and b change places in these columns (the family's tables follow in the family's own hands) */
  protected swapCols(a: number, b: number, f64: readonly F[], u32: readonly U[]): void {
    const d = this.d, u = this.u, O = this.O, B = this.b8, W = this.b4;
    for (const k of f64) { const o = B + O[k]; const t = d[o + a]; d[o + a] = d[o + b]; d[o + b] = t; }
    for (const k of u32) { const o = W + O[k]; const t = u[o + a]; u[o + a] = u[o + b]; u[o + b] = t; }
  }
  /** the bytes of the part as they stand (every column, the head and the tables) */
  bytes(): Uint8Array { const u = this.u; return new Uint8Array(u.buffer, u.byteOffset + this.b4 * 4, this.O.bytes).slice(); }
  /** …and put back */
  put(b: Uint8Array): void { const u = this.u; new Uint8Array(u.buffer, u.byteOffset + this.b4 * 4, this.O.bytes).set(b); }
}

/** WHERE TWO TAKES OF A STORE'S BYTES DIFFER, column by column and row by row over the first `rows` rows — the doubles by their value (a NaN is a NaN,
 *  −0 is not 0), the words by theirs; `who(u, row)` names a row. → at most `most` lines, none if those columns are the same */
export function colDiff<F extends string, U extends string>(O: ColsOf<F, U>, f64: readonly F[], u32: readonly U[], a: Uint8Array, b: Uint8Array, rows: number, who: (u: Uint32Array, r: number) => string, most: number, out: string[] = []): string[] {
  const first = O[u32[0]], da = new Float64Array(a.buffer, a.byteOffset, first >>> 1), db = new Float64Array(b.buffer, b.byteOffset, first >>> 1), ua = new Uint32Array(a.buffer, a.byteOffset, O.bytes >>> 2), ub = new Uint32Array(b.buffer, b.byteOffset, O.bytes >>> 2);
  for (const k of f64) for (let r = 0; r < rows && out.length < most; r++) if (!Object.is(da[O[k] + r], db[O[k] + r])) out.push(`row ${r} (${who(ua, r)}).${k} ${da[O[k] + r]} | ${db[O[k] + r]}`);
  for (const k of u32) for (let r = 0; r < rows && out.length < most; r++) if (ua[O[k] + r] !== ub[O[k] + r]) out.push(`row ${r}.${k} ${ua[O[k] + r]} | ${ub[O[k] + r]}`);
  return out;
}
