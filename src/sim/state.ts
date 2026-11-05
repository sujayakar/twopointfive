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
// it always was (a rule that lives only in the module is at home when its own switch is on: either way the system says so itself, `wants`). Which, for THIS
// game, is a bit in the block's head — state like the rest — and as a tick opens every system whose wish and bit disagree is
// moved (`settle`: the two functions a system registers, `into` and `outOf`). So a row may be raised or dropped under a running game; a snapshot holds where
// things were when it was taken; and a checkpoint FILE, which must load with every row off (a browser with no WebAssembly plays the TypeScript), is written
// from and read into the same fields as ever, through the system's own accessors.
//
// (Imports sim.ts and src/math alone: like the rest of src/sim, this is below the game.)
import { sim, u32 } from './sim';

/** THE PARTS OF A BLOCK, in the order of rust/sim/src/state.rs PARTS (tools/qa/rust-check.ts holds the two lists to each other) */
export const SIM_PARTS = ['head', 'dice', 'doors', 'falls', 'props', 'slabs', 'items', 'men', 'smoke', 'poses'] as const;
export const P_DICE = 1, P_DOORS = 2, P_FALLS = 3, P_PROPS = 4, P_SLABS = 5, P_ITEMS = 6, P_MEN = 7, P_SMOKE = 8, P_POSES = 9;   // (0 is the head; the pools after the parts of a fixed length, the largest last: a parked block is one run of bytes to its last word in use, `span`)
/** the head's words (state.rs Head): the layout's word, the homes */
const H_LAYOUT = 0, H_HOMES = 1;

/** words a piece: a block is taken in pieces of this many (4 KB), and a piece that has not changed since the last take is shared with it */
const PIECE = 1024;
/** a block as a snapshot keeps it: its pieces in order (none: the game has no block) */
export type SimCopy = readonly Uint32Array[];
const NO_BLOCK: SimCopy = Object.freeze([]) as SimCopy;
const NO_WORDS = new Uint32Array(0), NO_DOUBLES = new Float64Array(0);

/** what the started module says of a block: its length in words, where each part starts (bytes), how many slots */
interface Layout { words: number; games: number; word: number; off: number[]; len: number[]; pool: boolean[] }
let L: Layout | null = null;
function layout(): Layout {
  if (L) return L;
  const x = sim(), n = x.state_parts(), off: number[] = [], len: number[] = [], pool: boolean[] = [];
  for (let i = 0; i < n; i++) { off.push(x.state_part(i, 0)); len.push(x.state_part(i, 1)); pool.push(x.state_part(i, 2) !== 0); }
  return L = { words: x.state_len() >>> 2, games: x.state_games(), word: x.state_layout() >>> 0, off, len, pool };
}
/** where part `p` of a block starts, in bytes from the block's own start (`st.d[simPart(P) >> 3]` is its first double) */
export function simPart(p: number): number { return layout().off[p]; }
/** what the module says of its blocks, for a check: the layout's word, a block's bytes, the slots, each part's start and length */
export function simLayout(): { word: number; bytes: number; games: number; parts: { name: string; off: number; len: number; pool: boolean }[] } {
  const l = layout(); return { word: l.word, bytes: l.words * 4, games: l.games, parts: l.off.map((off, i) => ({ name: SIM_PARTS[i] ?? `part${i}`, off, len: l.len[i], pool: l.pool[i] })) };
}

/** A SYSTEM THAT KEEPS STATE IN THE BLOCK while it wants to, and in its TypeScript objects (if it has any) while it does not */
export interface SimTenant {
  /** what it is called (a part's name, for a message) */
  readonly name: string;
  /** its bit in the head's homes (one a system, never reused) */
  readonly bit: number;
  /** SHOULD THIS GAME'S STATE OF THE SYSTEM BE IN THE BLOCK NOW: the system's own reason, whole. A system with a TypeScript twin says its row of RUST and
   *  that this game has such state at all (the die: `RUST.dice` and a die of its own; a game with none gets no block for it); a rule that lives only in the
   *  module says its own switch. */
  wants(g: never): boolean;
  /** move this game's state of the system into the block (the bit goes up after) */
  into(g: never, st: SimState): void;
  /** …and out of it, back into the TypeScript objects (the bit goes down after) */
  outOf(g: never, st: SimState): void;
  /** as a tick opens with the system at home: what has come to the game since is moved in too (a man who arrived mid-game) */
  tend?(g: never, st: SimState): void;
}
const TENANTS: SimTenant[] = [];
/** register a system (a top-level statement of its module) */
export function simTenant(t: SimTenant): void {
  if (TENANTS.some(o => o.bit === t.bit || o.name === t.name)) throw new Error(`a tenant of the module's state is registered twice: ${t.name} (bit ${t.bit})`);
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
  /** views of pieces of the block handed out by `f32` (the host's: made again whenever the block moves; never part of a game's state) */
  private subs = new Map<number, Float32Array>();

  /** this game has a block (some system's state is at home in the module) */
  get open(): boolean { return this.w.length !== 0; }

  /** WHAT TABLES MADE FROM THE BLOCK GO BY: a number no other block of any game has had, given anew whenever the block's bytes were written wholesale or it
   *  changed place (a take put back, parked, brought into a slot, born, shut). A family that keeps tables DERIVED from its state (never in the block: the
   *  block is state and nothing else) keeps the stamp they were made at, and makes them again when it differs. A family's own writes do not change it. */
  stamp = 0;
  /** the slot the block stands in NOW, −1 = parked or none: for a caller that must not move it (anim/skeleton.ts: a question about a pose) */
  get where(): number { return this.slot; }
  /** THE SLOT, for a call into the module: the block is bound first if it was parked (a game with no block gets one) */
  bind(): number {
    const s = this.slot;
    if (s >= 0) { this.used = ++CLOCK.n; if (this.w.length === 0) this.view(); return s; }   // (a view over a buffer the module's memory grew out of is empty)
    return this.bring();
  }
  private bring(): number {
    const l = layout();
    if (this.own === null) this.born();
    let s = -1, oldest = Infinity;
    for (let i = 0; i < l.games; i++) { const h = HELD[i]; if (!h || h.slot !== i) { s = i; break; } if (h.used < oldest) { oldest = h.used; s = i; } }
    const was = HELD[s]; if (was && was.slot === s) was.park();
    const from = this.w, to = u32(), at = sim().state_addr(s) >>> 2;
    for (let i = 0; i < l.off.length; i++) { const o = l.off[i] >>> 2; to.set(from.subarray(o, o + this.live(i)), at + o); }
    this.own = null; this.slot = s; HELD[s] = this; this.used = this.stamp = ++CLOCK.n; SIM_STATE_TALLY.bound++;
    this.view();
    return s;
  }
  /** a block for a game that had none: zeros under the layout's word, parked */
  private born(): void { this.own = new ArrayBuffer(this.span(true)); new Uint32Array(this.own)[H_LAYOUT] = layout().word; this.stamp = ++CLOCK.n; this.view(); }
  /** out of the module into a buffer of its own (another game needs the slot) */
  private park(): void {
    const l = layout(), from = this.w, own = new ArrayBuffer(this.span(false)), to = new Uint32Array(own);
    for (let i = 0; i < l.off.length; i++) { const o = l.off[i] >>> 2; to.set(from.subarray(o, o + this.live(i)), o); }
    this.own = own;
    if (HELD[this.slot] === this) HELD[this.slot] = null;
    this.slot = -1; SIM_STATE_TALLY.parked++; this.stamp = ++CLOCK.n;
    this.view();
  }
  /** HOW MANY BYTES A PARKED BLOCK NEEDS: up to the end of the last word that is state (a pool is sized for the largest game, and a parked game does not
   *  pay for the room it does not use). `empty`: of a block of zeros. A whole number of doubles. */
  private span(empty: boolean): number {
    const l = layout(); let end = 0;
    for (let i = 0; i < l.off.length; i++) { const e = (l.off[i] >>> 2) + (empty ? (l.pool[i] ? 2 : l.len[i] >>> 2) : this.live(i)); if (e > end) end = e; }
    return ((end + 1) >>> 1) << 3;
  }
  /** a parked block made long enough for `words` words (a take put back into it may hold more of a pool than it does) */
  private room(words: number): void {
    if (this.own === null || this.own.byteLength >= words * 4) return;
    const own = new ArrayBuffer(((words + 1) >>> 1) << 3); new Uint32Array(own).set(this.w); this.own = own; this.view();
  }
  private view(): void {
    const l = layout(); this.subs.clear();
    if (this.slot >= 0) { const m = sim().memory.buffer, at = sim().state_addr(this.slot); this.w = new Uint32Array(m, at, l.words); this.d = new Float64Array(m, at, l.words >>> 1); }
    else if (this.own) { this.w = new Uint32Array(this.own); this.d = new Float64Array(this.own); }
    else { this.w = NO_WORDS; this.d = NO_DOUBLES; }
  }
  /** no block any more: the slot is free and what was in use of it is zeros again (no system's state is at home here) */
  private shut(): void {
    if (this.slot >= 0) { const l = layout(), w = this.w; for (let i = l.off.length - 1; i >= 0; i--) { const o = l.off[i] >>> 2; w.fill(0, o, o + this.live(i)); } if (HELD[this.slot] === this) HELD[this.slot] = null; }
    this.slot = -1; this.own = null; this.stamp = ++CLOCK.n; this.view();
  }
  /** HOW MANY WORDS OF PART `i` ARE THE GAME'S STATE: all of it — or, of a pool (rust/sim/src/state.rs: a part sized for the largest game, its first word
   *  the bytes of it in use), only those. What lies past them is carried, compared and folded by nobody. */
  private live(i: number): number { const l = layout(); return liveOf(l, i, this.w[l.off[i] >>> 2]); }
  /** `n` singles of the block from byte `at`, as an array: for a READ or a WRITE now. The array is this object's (made again when the block moves), so it
   *  is never kept in a field of the game: ask again. */
  f32(at: number, n: number): Float32Array {
    let v = this.subs.get(at);
    if (v === undefined || v.length !== n) { if (this.slot >= 0 && this.w.length === 0) this.view(); v = new Float32Array(this.w.buffer, this.w.byteOffset + at, n); this.subs.set(at, v); }
    return v;
  }

  /** AS A TICK OPENS: every system whose wish (`wants`: its row of RUST, its own switch) and whose home in this game disagree is moved (Game.stepSeats); a system at home may tend what has
   *  come since (a man who arrived); and a block is BOUND for the length of the tick, so an array `f32` gave is the block's until the tick ends. */
  settle(g: object): void {
    for (let i = 0; i < TENANTS.length; i++) {
      const t = TENANTS[i], have = this.w.length !== 0 && (this.w[H_HOMES] & t.bit) !== 0, want = t.wants(g as never);
      if (want === have) { if (have && t.tend) t.tend(g as never, this); continue; }
      if (want) { if (this.w.length === 0) this.born(); t.into(g as never, this); this.w[H_HOMES] |= t.bit; }
      else { t.outOf(g as never, this); this.w[H_HOMES] &= ~t.bit; if (this.w[H_HOMES] === 0) this.shut(); }
    }
    if (this.w.length !== 0) { if (this.slot < 0) this.bring(); else this.used = ++CLOCK.n; }   // (a game that ticks is not the one to park, though it asks nothing by `bind` all tick)
  }

  // ---- what the rollback snapshot does with a block (src/game/rollback.ts)
  /** the block as it stands, in pieces: each part's live words from its start, PIECE at a time, the parts in order. `was`: an earlier take — a piece that is
   *  still that to the bit is shared, and when every one is, `was` itself comes back */
  take(was?: SimCopy): SimCopy {
    const w = this.w;
    if (w.length === 0) return NO_BLOCK;
    const l = layout(), out: Uint32Array[] = []; let shared = was !== undefined;
    // (`kw`: where this part's pieces start in `was`, which is walked a part at a time by its own lengths, as `put` walks a take: a pool before this
    //  part that holds a piece more or fewer than it did then does not put this part's pieces out of step with the ones they are compared with)
    for (let i = 0, kw = 0; i < l.off.length; i++) {
      const o0 = l.off[i] >>> 2, n = this.live(i), nw = was !== undefined && kw < was.length ? Math.ceil(liveOf(l, i, was[kw][0]) / PIECE) : 0;
      for (let o = 0, p = 0; o < n; o += PIECE, p++) {
        const e = o + PIECE < n ? o + PIECE : n, c = p < nw ? was![kw + p] : undefined;
        let same = c !== undefined && c.length === e - o;
        if (same) for (let j = o0 + o, q = 0, m = e - o; q < m; j++, q++) if (w[j] !== c![q]) { same = false; break; }
        if (same) out.push(c!); else { shared = false; out.push(w.slice(o0 + o, o0 + e)); }
      }
      kw += nw;
    }
    return shared && was!.length === out.length ? was! : out;
  }
  /** the block is `c` to the bit (what `take(c) === c` says, with nothing copied) */
  same(c: SimCopy): boolean {
    const w = this.w; if (w.length === 0) return c.length === 0;
    const l = layout(); let k = 0;
    for (let i = 0; i < l.off.length; i++) {
      const o0 = l.off[i] >>> 2, n = this.live(i);
      for (let o = 0; o < n; o += PIECE, k++) {
        const e = o + PIECE < n ? o + PIECE : n, a = c[k]; if (a === undefined || a.length !== e - o) return false;
        for (let j = o0 + o, q = 0, m = e - o; q < m; j++, q++) if (w[j] !== a[q]) return false;
      }
    }
    return k === c.length;
  }
  /** a take written back into this game's block, wherever it stands (none: the game had no block then, and has none now). A pool's first piece says how
   *  much of it the take holds, so the pieces say where they go; a parked block is made long enough first. */
  put(c: SimCopy): void {
    if (c.length === 0) { if (this.w.length !== 0) this.shut(); return; }
    if (this.w.length === 0) this.born();
    const l = layout(); let need = 0;
    for (let i = 0, k = 0; i < l.off.length; i++) { const n = liveOf(l, i, c[k][0]); k += Math.ceil(n / PIECE); if ((l.off[i] >>> 2) + n > need) need = (l.off[i] >>> 2) + n; }
    this.room(need);
    const w = this.w;
    for (let i = 0, k = 0; i < l.off.length; i++) { const o0 = l.off[i] >>> 2, n = liveOf(l, i, c[k][0]); for (let o = 0; o < n; o += c[k++].length) w.set(c[k], o0 + o); }
    this.stamp = ++CLOCK.n;
  }
  /** the block a part at a time, each under its name, as its live words (fresh views, for a walk that reads them at once: never kept) */
  parts(): { name: string; words: Uint32Array }[] {
    if (this.w.length === 0) return [];
    const l = layout(), w = this.w;
    return l.off.map((off, i) => ({ name: SIM_PARTS[i] ?? `part${i}`, words: w.subarray(off >>> 2, (off >>> 2) + this.live(i)) }));
  }
}

/** how many words of part `i` are state, given the part's first word (the block's own, or a take's first piece of it) */
function liveOf(l: Layout, i: number, first: number): number {
  const all = l.len[i] >>> 2; if (!l.pool[i]) return all;
  const n = (first + 3) >>> 2; return n < 2 ? 2 : n > all ? all : n;
}

/** how many bytes a take holds in all, and how many are its own: in no piece that `shared` (everything an earlier snapshot holds) has */
export function simCopyBytes(c: SimCopy, shared?: ReadonlySet<unknown> | null): { bytes: number; own: number } {
  let bytes = 0, own = 0;
  for (let p = 0; p < c.length; p++) { bytes += c[p].byteLength; if (!shared || !shared.has(c[p])) own += c[p].byteLength; }
  return { bytes, own };
}
