// WHAT IS THROWN OR DROPPED, AS COLUMNS — where every can's, grenade's, magazine's and dropped gun's own numbers are kept (game/throwables.ts reads and
// writes them through `Item`), in TypeScript and in the Rust module alike, and how they move between the two.
//
// THE LAYOUT IS ONE, BYTE FOR BYTE (rust/sim/src/items.rs Items), and it is the furniture's kind of layout (sim/propshome.ts, props.rs: a column a number
// as narrow as its values, hot before cold, every column whole vectors on a 16-byte boundary). The bytes are the `items` part of the game's block from
// the moment the pool is made (sim/state.ts, sim/colstore.ts ColStore, which every such store is), and the pool is walked over them, in the module
// (items.rs update, the row RUST.items on) or in game/throwables.ts. A column is read as `d[b8 + O.px + row]` (the doubles) or `u[b4 + O.flags + row]`
// (the words).
//
// ROWS. Rows 0 .. n are the pool, IN THE GAME'S OWN ORDER: row k is `Throwables.items[k]`, the oldest first. That order is the rule's (a full pool gives
// up the first spent thing; the picture's boxes are listed in it; the step walks it from the newest down), so nothing sorts the rows. The pool's own
// writes move rows and nothing else does: `enter` (one more behind the last), `close` (a row given up, those behind it closing up in their order)
// and `lay` (every row written anew, when the game's list was changed from outside the pool). A thing that leaves the pool TAKES ITS NUMBERS WITH IT
// (`rowOut`: game/throwables.ts Item keeps them): whoever still holds it — a door op its can, a guard the gun he dropped — reads what it was when it left.
//
// MORE THAN THE RULE'S THIRTY-TWO. A pool is ITEM_CAP things by the game's own rule, which gives one up for each new one; it grows past that only when
// every one of them is a can still on its fuse or venting (never, in play). The columns have ITEM_ROWS rows for that, twice the rule's, and they are
// the module's part as they stand, always: a pool that grew is stepped where every pool is. At ITEM_ROWS live cans the oldest gives way (spawn), so
// no pool is ever more than its columns; a list handed in longer than that is a fault, said aloud (`room`).
// (Until 2026-11-04 the columns were ITEM_CAP rows, a pool that grew left the module, and it was stepped in TypeScript from there on.)
//
// A truth is a bit of `flags` (ITF), a kind its place in ITEM_KINDS, who threw it his id (an i32: read `| 0`), the thing of furniture it lies on that
// thing's id plus one (0: none).
//
// (Imports sim.ts, state.ts and colstore.ts alone: like the rest of src/sim, this is below the game.)
import { sim, f64, type SimExports } from './sim';
import { simRoom, P_ITEMS, type SimState } from './state';
import { ColStore, colsOf, colDiff } from './colstore';

/** things a pool holds by the game's rule (game/throwables.ts spawn gives one up at this many, unless every one is a live can) */
export const ITEM_CAP = 32;
/** …and the rows its columns have, in the module and here (rust/sim/src/items.rs CAP): what a pool of live cans may grow to */
export const ITEM_ROWS = 64;
/** THE COLUMNS in the part's order (items.rs COLS; tools/qa/rust-check.ts holds the names and itemIo() the offsets to the module's own): the doubles —
 *  the twelve read for every row on every tick first, then what a bounce reads and writes, then other rules' own — then the words */
export const ITEM_F64 = ['px', 'py', 'pz', 'vx', 'vy', 'vz', 'yaw', 'spin', 'tumble', 'tumbleRate', 'age', 'life', 'radius', 'restitution', 'friction', 'heading', 'onPose', 'fuse', 'emitT', 'fill'] as const;
export const ITEM_U32 = ['flags', 'kind', 'bounces', 'who', 'on', 'serial'] as const;
/** the head in front of the columns: things in the pool (a word; three more spare) */
export const ITEM_HEAD = 16;
/** a thing's truths (the bits of `flags`: items.rs F_SETTLED, F_FIRED, F_CARRIED): lying at rest · its fuse has run · in a hand */
export const ITF = { settled: 1, fired: 2, carried: 4 } as const;
/** WHAT A THING IS, by its place here (the `kind` column; items.rs K_SMOKE, K_FRAG, K_RIFLE are three of these places) */
export const ITEM_KINDS = ['smoke', 'mag', 'flash', 'frag', 'pistol', 'torch', 'rifle', 'ring', 'shell'] as const;

type F = (typeof ITEM_F64)[number];
type U = (typeof ITEM_U32)[number];
/** A COLUMN BY ITS PLACE in the two lists together (the doubles 0 .. 19, then the words): how a row is named out of the store (`rowOut`, `rowIn`) and
 *  how game/throwables.ts Item asks for one of its numbers */
export const IC = Object.fromEntries([...ITEM_F64, ...ITEM_U32].map((k, i) => [k, i])) as Record<F | U, number>;
const NF = ITEM_F64.length, NC = ITEM_F64.length + ITEM_U32.length;
/** where each column starts (sim/colstore.ts colsOf), the pool's bit in the head of a game's block (sim/state.ts: one a family, never reused), and how
 *  long its part is */
export const itemCols = () => colsOf(ITEM_HEAD, ITEM_F64, ITEM_U32, ITEM_ROWS);
const ITEM_BIT = 128;
simRoom(P_ITEMS, itemCols().bytes);

/** THE COLUMNS OF ONE POOL (a game's, a lone stage's), over its game's block `st`: sim/colstore.ts's store, and what its rows mean */
export class ItemStore extends ColStore<F, U> {
  /** where each column starts, by its place (IC): the doubles' as indices into the doubles, the words' into the words — `O` by number, made anew with it */
  at: number[];
  constructor(st: SimState) { super(st, P_ITEMS, ITEM_BIT, 'thrown things', ITEM_HEAD, ITEM_F64, ITEM_U32, ITEM_ROWS); this.at = this.places(); }
  private places(): number[] { const O = this.O; return [...ITEM_F64.map(k => O[k]), ...ITEM_U32.map(k => O[k])]; }
  /** `n` rows fit, or it is nobody's pool: the game's rule keeps a pool far under its rows (the header), and one past them is a fault */
  private room(n: number): void { if (n > this.cap) throw new Error(`a pool of ${n} thrown things: its columns hold ${this.cap} (sim/itemshome.ts ITEM_ROWS, rust/sim/src/items.rs CAP; game/throwables.ts spawn gives one up for each new one past ${ITEM_CAP})`); }
  /** things in the pool */
  get n(): number { return this.u[this.b4]; }
  /** ONE MORE THING, behind the last: its row (zeros: its numbers are the caller's to write) */
  enter(): number { const n = this.n; this.room(n + 1); this.u[this.b4] = n + 1; return n; }
  /** row `r`'s numbers, by their places (IC) */
  rowOut(r: number): number[] {
    const d = this.d, u = this.u, at = this.at, B = this.b8 + r, W = this.b4 + r, out: number[] = new Array(NC);
    for (let c = 0; c < NF; c++) out[c] = d[B + at[c]];
    for (let c = NF; c < NC; c++) out[c] = u[W + at[c]];
    return out;
  }
  /** …and into it */
  rowIn(r: number, v: readonly number[]): void {
    const d = this.d, u = this.u, at = this.at, B = this.b8 + r, W = this.b4 + r;
    for (let c = 0; c < NF; c++) d[B + at[c]] = v[c];
    for (let c = NF; c < NC; c++) u[W + at[c]] = v[c];
  }
  /** A ROW IS GIVEN UP: the rows behind it close up in their order, and the last is zeros again */
  close(r: number): void {
    const n = this.n; if (r < 0 || r >= n) return;
    const d = this.d, u = this.u, at = this.at, B = this.b8, W = this.b4;
    for (let c = 0; c < NF; c++) { const o = B + at[c]; d.copyWithin(o + r, o + r + 1, o + n); d[o + n - 1] = 0; }
    for (let c = NF; c < NC; c++) { const o = W + at[c]; u.copyWithin(o + r, o + r + 1, o + n); u[o + n - 1] = 0; }
    u[W] = n - 1;
  }
  /** THE POOL LAID OUT ANEW: these rows, in this order, and zeros behind them */
  lay(rows: readonly (readonly number[])[]): void {
    const was = this.n; this.room(rows.length);
    for (let r = 0; r < rows.length; r++) this.rowIn(r, rows[r]);
    const d = this.d, u = this.u, at = this.at, B = this.b8, W = this.b4;
    for (let r = rows.length; r < was; r++) { for (let c = 0; c < NF; c++) d[B + at[c] + r] = 0; for (let c = NF; c < NC; c++) u[W + at[c] + r] = 0; }
    u[W] = rows.length;
  }
}

// ---- the hand-over of one walk of the pool (rust/sim/src/items.rs PAR and EVS, by the same numbers)
/** in: the tick, the row to start at, whether the host has a rule for panes, whether the ground may be lying snow, a rifle's share for the furniture;
 *  out: the row handed back, how many events are listed, counts for the checks; in again, read where the ground may be snow: the rule's numbers
 *  (game/snowBog.ts SNOWBOG top, bankM, packed, old, dead, grip, stick), how many roofs are in force and, from `roof`, four numbers each */
export const IP = { dt: 0, from: 1, ask: 2, bog: 3, rifleShare: 4, row: 8, events: 9, rays: 10, thingRays: 11, substeps: 12, walked: 13, rested: 14,
  snowTop: 16, snowBank: 17, snowPacked: 18, snowOld: 19, snowDead: 20, snowGrip: 21, snowStick: 22, roofs: 23, roof: 24 } as const;
export const ITEM_PARS = 40;
/** the roofs a walk is handed at most (items.rs MAX_ROOFS; weather.ts keeps at most four) */
export const ITEM_ROOFS = 4;
/** an event: what (IEK), the row it is of, a number (a bounce: how fast into the face; an impulse: the id of the thing of furniture), where (three),
 *  a vector (a bounce: the face's normal; an impulse: the impulse) */
export const IE = { kind: 0, row: 1, a: 2, p: 3, v: 6 } as const;
export const ITEM_EV = 9;
export const IEK = { bounce: 0, impulse: 1, gone: 2 } as const;
/** the most one row lists (its time up; or, a substep, an impulse, a bounce and the floor's bounce — twice) */
export const ITEM_ROW_EVENTS = 6;
/** the walk's answers (items.rs DONE, HOST, WRONG) */
export const ITEMS_DONE = 0, ITEMS_HOST = 1;

/** where the hand-over's tables are in the module's memory (indices into its doubles), and how many events the list holds: asked once */
export interface ItemIo { x: SimExports; par: number; ev: number; maxEvents: number }
let IO: ItemIo | null = null;
export function itemIo(): ItemIo {
  if (IO) return IO;
  const x = sim(), O = itemCols(), names = [...ITEM_F64, ...ITEM_U32];
  // (the layout this file works out is the module's own: every column where items.rs has it, the part as long)
  const off = names.filter((k, i) => x.items_cap(100 + i) !== (i < ITEM_F64.length ? O[k] * 8 : O[k] * 4));
  if (x.items_cap(0) !== ITEM_ROWS || x.items_cap(13) !== O.bytes || off.length || x.items_cap(100 + names.length) !== 0 || x.items_cap(1) !== ITEM_PARS || x.items_cap(4) !== ITEM_ROOFS || x.items_cap(3) !== ITEM_EV || x.items_cap(2) !== ITEM_ROWS * ITEM_ROW_EVENTS)
    throw new Error(`the Rust module lays the thrown things out as ${x.items_cap(0)} rows in ${x.items_cap(13)} bytes and src/sim/itemshome.ts as ${ITEM_ROWS} in ${O.bytes}${off.length ? ` (columns not where the module has them: ${off.join(', ')})` : ''}, or the hand-over is not the same shape: bun run tools/sim/build.ts`);
  return IO = { x, par: x.items_addr(0) / 8, ev: x.items_addr(1) / 8, maxEvents: x.items_cap(2) };
}
/** the module's memory as doubles, for the hand-over (never kept) */
export const itemMem = f64;

/** COUNTS FOR THE CHECKS (the host's: no game reads them): rows the module walked, rows the TypeScript stepped, rows the module handed back (none in
 *  play: a ray that is no number), calls into the module, walks answered both ways and how many of those differed, with the first in words; what the module's walks asked and
 *  met; and the steps of a pool with something in it that began in the module (`stepsThere`) and that were this side's whole (`stepsHere`) */
export const ITEMS_STEPPED = { there: 0, here: 0, host: 0, walks: 0, asked: 0, differ: 0, first: '', rays: 0, thingRays: 0, substeps: 0, rested: 0, events: 0, stepsThere: 0, stepsHere: 0 };

/** WHERE TWO TAKES OF A STORE'S BYTES DIFFER, in words (`bytes()` of the same store, twice). → at most `most` lines, none if they are the same state */
export function itemDiff(S: ItemStore, a: Uint8Array, b: Uint8Array, most = 4): string[] {
  const O = S.O, ua = new Uint32Array(a.buffer, a.byteOffset, 4), ub = new Uint32Array(b.buffer, b.byteOffset, 4), out: string[] = [];
  if (ua[0] !== ub[0]) out.push(`${ua[0]} things in the pool | ${ub[0]}`);
  return colDiff(O, ITEM_F64, ITEM_U32, a, b, Math.max(ua[0], ub[0]), (u, r) => `serial ${u[O.serial + r]}`, most, out);
}
