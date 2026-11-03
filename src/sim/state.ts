// A GAME'S STATE IN THE RUST MODULE, FROM TYPESCRIPT — the one object a Game reaches it through (`game.sim`), and what the rollback snapshot, the saves kept in
// memory and the state's word know that state by (docs/agents/rust.md, "State in the module is state of the game").
//
// THE RULE (rust/sim/src/state.rs has the Rust half): whatever of a game lives in the module from one call to the next is ONE BLOCK of bytes a game — a head,
// then a part a family of the update — and nothing else in the module is state. A block holds indices, never addresses, so it is the same bytes wherever it
// stands. That is what makes it cheap to carry: nobody writes a line a system to take it, put it back, put it into another game or fold it into a word.
//
// WHERE A BLOCK IS. The module holds a fixed number of blocks (state_games: its memory never grows). A game's block is BOUND to one of those slots, or PARKED
// in a buffer of the game's own when more games are alive in the process than the module has slots — the one stepped longest ago gives way, and nothing is
// lost. Either way `st.w` and `st.d` are the block and nothing but the block (32-bit words, doubles), so:
//   · a READ or a WRITE from TypeScript is `st.d[i]` / `st.w[i]`: this game's, wherever it stands, and no call into the module;
//   · a CALL into the module that reads or writes state passes `st.bind()`, the slot (a parked block is brought in first).
// NO OBJECT OF THE GAME KEEPS A TYPED ARRAY OVER THE MODULE'S MEMORY (never `st.w` in a field, never a `subarray` of it): the snapshot would take such a view
// for an array of the game's own, and putting it into another game clones the buffer under it, which is the whole memory. State is reached through the
// SimState, every time. rollback.ts names any such view it meets (`Snapshot.simViews`) and tools/qa/rollback-check.ts is red on one.
//
// AT HOME, OR NOT. While a system has a TypeScript twin, its state lives in the block only when its row of RUST is on; else it is in the TypeScript objects, as
// it always was. Which, for THIS game, is a bit in the block's head — state like the rest — and as a tick opens every system whose row and bit disagree is
// moved (`settle`: the two functions a system registers, `into` and `outOf`). So a row may be raised or dropped under a running game; a snapshot holds where
// things were when it was taken; and a checkpoint FILE, which must load with every row off (a browser with no WebAssembly plays the TypeScript), is written
// from and read into the same fields as ever, through the system's own accessors.
//
// (Imports sim.ts and src/math alone: like the rest of src/sim, this is below the game.)
import { sim, RUST, u32 } from './sim';

/** THE PARTS OF A BLOCK, in the order of rust/sim/src/state.rs PARTS (tools/qa/rust-check.ts holds the two lists to each other) */
export const SIM_PARTS = ['head', 'dice'] as const;
export const P_DICE = 1;   // (0 is the head)
/** the head's words (state.rs Head): the layout's word, the homes */
const H_LAYOUT = 0, H_HOMES = 1;

/** words a piece: a block is taken in pieces of this many (4 KB), and a piece that has not changed since the last take is shared with it */
const PIECE = 1024;
/** a block as a snapshot keeps it: its pieces in order (none: the game has no block) */
export type SimCopy = readonly Uint32Array[];
const NO_BLOCK: SimCopy = Object.freeze([]) as SimCopy;
const NO_WORDS = new Uint32Array(0), NO_DOUBLES = new Float64Array(0);

/** what the started module says of a block: its length in words, where each part starts (bytes), how many slots */
interface Layout { words: number; games: number; word: number; off: number[]; len: number[] }
let L: Layout | null = null;
function layout(): Layout {
  if (L) return L;
  const x = sim(), n = x.state_parts(), off: number[] = [], len: number[] = [];
  for (let i = 0; i < n; i++) { off.push(x.state_part(i, 0)); len.push(x.state_part(i, 1)); }
  return L = { words: x.state_len() >>> 2, games: x.state_games(), word: x.state_layout() >>> 0, off, len };
}
/** where part `p` of a block starts, in bytes from the block's own start (`st.d[simPart(P) >> 3]` is its first double) */
export function simPart(p: number): number { return layout().off[p]; }
/** what the module says of its blocks, for a check: the layout's word, a block's bytes, the slots, each part's start and length */
export function simLayout(): { word: number; bytes: number; games: number; parts: { name: string; off: number; len: number }[] } {
  const l = layout(); return { word: l.word, bytes: l.words * 4, games: l.games, parts: l.off.map((off, i) => ({ name: SIM_PARTS[i] ?? `part${i}`, off, len: l.len[i] })) };
}

/** A SYSTEM THAT KEEPS STATE IN THE BLOCK while its row of RUST is on, and in its TypeScript objects while it is off */
export interface SimTenant {
  /** its row of RUST */
  readonly row: string;
  /** its bit in the head's homes (one a system, never reused) */
  readonly bit: number;
  /** this game has state of the system at all (a game with no die of its own has none to move: it gets no block for it) */
  has(g: never): boolean;
  /** move this game's state of the system into the block (the bit goes up after) */
  into(g: never, st: SimState): void;
  /** …and out of it, back into the TypeScript objects (the bit goes down after) */
  outOf(g: never, st: SimState): void;
}
const TENANTS: SimTenant[] = [];
/** register a system (a top-level statement of its module) */
export function simTenant(t: SimTenant): void {
  if (TENANTS.some(o => o.bit === t.bit || o.row === t.row)) throw new Error(`a tenant of the module's state is registered twice: ${t.row} (bit ${t.bit})`);
  TENANTS.push(t);
}

/** who holds each of the module's slots (the host's: which game stands in which slot is no part of any game), and a count that says who was bound last */
const HELD: (SimState | null)[] = [];
const CLOCK = { n: 0 };
/** counts for a check: blocks parked to make room, parked blocks brought back */
export const SIM_STATE_TALLY = { parked: 0, bound: 0 };

export class SimState {
  /** the block as 32-bit words and as doubles: all of it and nothing else, wherever it stands (empty: this game has no block) */
  w: Uint32Array = NO_WORDS;
  d: Float64Array = NO_DOUBLES;
  /** the module's slot the block is bound to (−1: parked in `own`, or no block) */
  private slot = -1;
  private own: ArrayBuffer | null = null;
  private used = 0;

  /** this game has a block (some system's state is at home in the module) */
  get open(): boolean { return this.w.length !== 0; }

  /** THE SLOT, for a call into the module: the block is bound first if it was parked (a game with no block gets one) */
  bind(): number {
    const s = this.slot;
    if (s >= 0) { this.used = ++CLOCK.n; if (this.w.buffer !== sim().memory.buffer) this.view(); return s; }
    return this.bring();
  }
  private bring(): number {
    const l = layout();
    if (this.own === null) this.born();
    let s = -1, oldest = Infinity;
    for (let i = 0; i < l.games; i++) { const h = HELD[i]; if (!h || h.slot !== i) { s = i; break; } if (h.used < oldest) { oldest = h.used; s = i; } }
    const was = HELD[s]; if (was && was.slot === s) was.park();
    u32().set(new Uint32Array(this.own!), sim().state_addr(s) >>> 2);
    this.own = null; this.slot = s; HELD[s] = this; this.used = ++CLOCK.n; SIM_STATE_TALLY.bound++;
    this.view();
    return s;
  }
  /** a block for a game that had none: zeros under the layout's word, parked */
  private born(): void { const l = layout(); this.own = new ArrayBuffer(l.words * 4); new Uint32Array(this.own)[H_LAYOUT] = l.word; this.view(); }
  /** out of the module into a buffer of its own (another game needs the slot) */
  private park(): void {
    const l = layout(), at = sim().state_addr(this.slot) >>> 2;
    this.own = u32().slice(at, at + l.words).buffer as ArrayBuffer;
    if (HELD[this.slot] === this) HELD[this.slot] = null;
    this.slot = -1; SIM_STATE_TALLY.parked++;
    this.view();
  }
  private view(): void {
    const l = layout();
    if (this.slot >= 0) { const m = sim().memory.buffer, at = sim().state_addr(this.slot); this.w = new Uint32Array(m, at, l.words); this.d = new Float64Array(m, at, l.words >>> 1); }
    else if (this.own) { this.w = new Uint32Array(this.own); this.d = new Float64Array(this.own); }
    else { this.w = NO_WORDS; this.d = NO_DOUBLES; }
  }
  /** no block any more: the slot is free and the bytes are gone (no system's state is at home here) */
  private shut(): void {
    if (this.slot >= 0) { this.w.fill(0); if (HELD[this.slot] === this) HELD[this.slot] = null; }
    this.slot = -1; this.own = null; this.view();
  }

  /** AS A TICK OPENS: every system whose row of RUST and whose home in this game disagree is moved (Game.stepSeats). Nearly always nothing. */
  settle(g: object): void {
    for (let i = 0; i < TENANTS.length; i++) {
      const t = TENANTS[i], have = this.w.length !== 0 && (this.w[H_HOMES] & t.bit) !== 0, want = (RUST as Record<string, boolean>)[t.row] === true && t.has(g as never);
      if (want === have) continue;
      if (want) { if (this.w.length === 0) this.born(); t.into(g as never, this); this.w[H_HOMES] |= t.bit; }
      else { t.outOf(g as never, this); this.w[H_HOMES] &= ~t.bit; if (this.w[H_HOMES] === 0) this.shut(); }
    }
  }

  // ---- what the rollback snapshot does with a block (src/game/rollback.ts)
  /** the block as it stands, in pieces. `was`: an earlier take — a piece that is still that to the bit is shared, and when every one is, `was` itself comes back */
  take(was?: SimCopy): SimCopy {
    const w = this.w, n = w.length;
    if (n === 0) return NO_BLOCK;
    const k = Math.ceil(n / PIECE), old = was !== undefined && was.length === k ? was : null;
    let out: Uint32Array[] | null = old ? null : [];
    for (let p = 0; p < k; p++) {
      const o = p * PIECE, e = o + PIECE < n ? o + PIECE : n, c = old ? old[p] : undefined;
      let same = c !== undefined && c.length === e - o;
      if (same) for (let j = o, q = 0; j < e; j++, q++) if (w[j] !== c![q]) { same = false; break; }
      if (same) { if (out) out.push(c!); } else { if (!out) out = old!.slice(0, p); out.push(w.slice(o, e)); }
    }
    return out ?? old!;
  }
  /** the block is `c` to the bit (what `take(c) === c` says, with nothing copied) */
  same(c: SimCopy): boolean {
    const w = this.w, n = w.length;
    if (c.length !== Math.ceil(n / PIECE)) return false;
    for (let p = 0, o = 0; p < c.length; p++) { const a = c[p], m = a.length; if (o + m > n) return false; for (let q = 0; q < m; q++) if (w[o + q] !== a[q]) return false; o += m; }
    return true;
  }
  /** a take written back into this game's block, wherever it stands (none: the game had no block then, and has none now) */
  put(c: SimCopy): void {
    if (c.length === 0) { if (this.w.length !== 0) this.shut(); return; }
    if (this.w.length === 0) this.born();
    const w = this.w; for (let p = 0, o = 0; p < c.length; o += c[p].length, p++) w.set(c[p], o);
  }
  /** the block a part at a time, each under its name, as words (fresh views, for a walk that reads them at once: never kept) */
  parts(): { name: string; words: Uint32Array }[] {
    if (this.w.length === 0) return [];
    const l = layout(), w = this.w;
    return l.off.map((off, i) => ({ name: SIM_PARTS[i] ?? `part${i}`, words: w.subarray(off >>> 2, (off + l.len[i]) >>> 2) }));
  }
}

/** how many bytes a take holds in all, and how many are its own: in no piece that `shared` (everything an earlier snapshot holds) has */
export function simCopyBytes(c: SimCopy, shared?: ReadonlySet<unknown> | null): { bytes: number; own: number } {
  let bytes = 0, own = 0;
  for (let p = 0; p < c.length; p++) { bytes += c[p].byteLength; if (!shared || !shared.has(c[p])) own += c[p].byteLength; }
  return { bytes, own };
}
