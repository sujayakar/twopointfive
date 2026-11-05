// THE FURNITURE'S STATE, AS COLUMNS — where every chair's, carton's, bin's and plant's own numbers are kept (game/props.ts reads and writes them), in
// TypeScript and in the Rust module alike, and how they move between the two.
//
// THE LAYOUT IS ONE, BYTE FOR BYTE (rust/sim/src/props.rs Props, which says why it is what it is: dense rows in the game's own order, a column a number as
// narrow as its values, hot before cold, every column whole vectors on a 16-byte boundary). The bytes are the `props` part of the game's block
// (sim/state.ts, sim/colstore.ts ColStore, which every such store is) from the moment the furniture is made, and the furniture's step runs over them
// with no copy in or out, in the module (props.rs update, the row RUST.props on) or in props.ts. A column is read as `d[b8 + O.x + row]` (the
// doubles) or `u[b4 + O.flags + row]` (the words), so the step and everything that reads a chair through `Prop.x` — the men who walk round it,
// a blast, a checkpoint file, the digest — reads the same values whichever side stepped.
//
// ROWS AND IDS. Rows 0 .. n are the things in the level now, in the order of `PropSystem.props` (which this keeps: row k IS props[k]); rows n .. total the
// ones a blast took out, kept for the level put back. A thing's id never moves; `row[id]` and `id[row]` say who is where. `leave` and `home` are the two
// writes that move rows, and nothing else does.
//
// A truth is a bit of `flags` (PT), a man is his id (an i32: read `| 0`), a sound (PropEvent.sound) its place in PROP_SOUNDS.
//
// (Imports sim.ts, state.ts and colstore.ts alone: like the rest of src/sim, this is below the game.)
import { sim, f64, type SimExports } from './sim';
import { simRoom, P_PROPS, type SimState } from './state';
import { ColStore, colsOf, colDiff, type ColsOf } from './colstore';

/** loose things of furniture a level can have (rust/sim/src/props.rs CAP; a level of more is not loaded: game/levelfit.ts) */
export const PROP_CAP = 128;
/** …and the boxes all of a level's furniture is made of, at most (the Rust module's copy of the box world lists them as one list of leaves:
 *  rust/sim/src/world.rs MAX_LEAVES, held to this number by scene/boxMirror.ts as it boots; game/levelfit.ts says so as a level loads). 256 until
 *  2026-11-04, when the fullest level stood at 220. */
export const PROP_PARTS = 512;
/** THE COLUMNS in the part's order (props.rs COLS; tools/qa/rust-check.ts holds the names and propIo() the offsets to the module's own): the doubles — the
 *  hot ones first — then the words */
export const PROP_F64 = ['x', 'z', 'yaw', 'vx', 'vz', 'w', 'cs', 'sn', 'idle', 'pushT', 'noiseT'] as const;
export const PROP_U32 = ['flags', 'who', 'id', 'row'] as const;
/** the head in front of the columns: the system's clock (a double), then the rows in the level now and the things in all (two words) */
export const PROP_HEAD = 16;
/** a thing's truths (the bits of `flags`: props.rs F_ASLEEP, F_DIRTY, F_ROLLING) */
export const PT = { asleep: 1, dirty: 2, rolling: 4 } as const;
/** PropEvent.sound as an event of the module's names it (its place here) */
export const PROP_SOUNDS = ['scrape', 'thud', 'roll'] as const;

/** where each column starts for columns `cap` rows long (sim/colstore.ts colsOf) */
export type PropCols = ColsOf<(typeof PROP_F64)[number], (typeof PROP_U32)[number]>;
export const propCols = (cap: number): PropCols => colsOf(PROP_HEAD, PROP_F64, PROP_U32, cap);
/** the furniture's bit in the head of a game's block (sim/state.ts: one a family, never reused), and how long its part is */
const PROP_BIT = 32;
simRoom(P_PROPS, propCols(PROP_CAP).bytes);

/** THE COLUMNS OF ONE SET OF FURNITURE (a level's, or a lone stage's), over its game's block `st`: sim/colstore.ts's store, and what its rows mean. `cap`
 *  is PROP_CAP, so that the bytes are the module's part as they stand; a level of more things than that is not loaded, and says so. */
export class PropStore extends ColStore<(typeof PROP_F64)[number], (typeof PROP_U32)[number]> {
  /** the maths the kept cos and sin were worked out under (math/exact.ts mathsEpoch: the host's, game/props.ts keeps it fresh) */
  epoch = -1;
  constructor(st: SimState, readonly total: number) {
    if (total > PROP_CAP) throw new Error(`a level of ${total} loose things of furniture: the game holds ${PROP_CAP} (sim/propshome.ts PROP_CAP, rust/sim/src/props.rs CAP)`);   // (said as the level loads: there is no other step for the furniture of a bigger one)
    super(st, P_PROPS, PROP_BIT, 'furniture', PROP_HEAD, PROP_F64, PROP_U32, PROP_CAP);
    const d = this.d, u = this.u, O = this.O, B = this.b8, W = this.b4;
    u[W + 2] = total; u[W + 3] = total;
    // (a thing as it is made: in its own row, asleep, to be placed, its clocks long past, the cos and sin of a yaw of 0)
    for (let i = 0; i < total; i++) { u[W + O.id + i] = i; u[W + O.row + i] = i; u[W + O.flags + i] = PT.asleep | PT.dirty; d[B + O.noiseT + i] = -10; d[B + O.pushT + i] = -10; d[B + O.cs + i] = 1; }
  }
  /** the system's clock */
  get time(): number { return this.d[this.b8]; } set time(v: number) { this.d[this.b8] = v; }
  /** rows in the level now */
  get n(): number { return this.u[this.b4 + 2]; }
  /** where thing `id` is */
  rowOf(id: number): number { return this.u[this.b4 + this.O.row + id]; }
  private swap(a: number, b: number): void {   // (rows a and b change places, every column; the two tables follow)
    if (a === b) return;
    this.swapCols(a, b, PROP_F64, SWAPPED);
    const u = this.u, O = this.O, W = this.b4; u[W + O.row + u[W + O.id + a]] = a; u[W + O.row + u[W + O.id + b]] = b;
  }
  /** A THING LEAVES THE LEVEL (PropSystem.remove): its row goes behind the live ones, those after it closing up in their order. → false if it was not in */
  leave(id: number): boolean {
    const n = this.n; let r = this.rowOf(id); if (r >= n) return false;
    for (; r < n - 1; r++) this.swap(r, r + 1);
    this.u[this.b4 + 2] = n - 1; return true;
  }
  /** THE LEVEL PUT BACK (PropSystem.reset): every thing in the level again, each in the row of its own id */
  homeRows(): void {
    for (let i = 0; i < this.total; i++) this.swap(i, this.rowOf(i));
    this.u[this.b4 + 2] = this.total;
  }
}
/** the word columns a row carries with it when it moves (`row` is the table that says where a thing went) */
const SWAPPED = ['flags', 'who', 'id'] as const;

// ---- the hand-over of one step (rust/sim/src/props.rs: its tables and their columns, by the same numbers)
/** the level's facts of a thing, written by row */
export const PD = { hx: 0, hz: 1, radius: 2, height: 3, mass: 4, invMass: 5, invI: 6, decel: 7, visc: 8, rest: 9, chair: 10 } as const;
/** a man, in and out (x and z come back as where the furniture leaves him) */
export const PK = { x: 0, z: 1, vx: 2, vz: 3, radius: 4, who: 5, mass: 6, quiet: 7 } as const;
/** a door frame */
export const PY = { cx: 0, cz: 1, ux: 2, uz: 3, half: 4 } as const;
/** a noise */
export const PE = { prop: 0, sound: 1, roll: 2, x: 3, y: 4, z: 5, who: 6, level: 7, speed: 8, rolling: 9 } as const;
export const PROP_EV = 10;
/** the step's numbers, in and out */
export const PP = { dt: 0, props: 1, contacts: 2, leaves: 3, ways: 4, flags: 5, friction: 6, casDecel: 7, casVisc: 8, casSpin: 9, casStart: 10, casStop: 11, events: 12, wrong: 13,
  swept: 14, guided: 15, pairs: 16, walls: 17, held: 18, substeps: 19 } as const;
export const PF = { enabled: 1, castor: 2 } as const;
export const PROP_PARS = 24;

/** where the hand-over's tables are in the module's memory (indices into its doubles), and how much each holds: asked once */
export interface PropIo { x: SimExports; def: number; con: number; leaf: number; way: number; ev: number; par: number; maxContacts: number; maxLeaves: number; maxWays: number; maxEvents: number }
let IO: PropIo | null = null;
export function propIo(): PropIo {
  if (IO) return IO;
  const x = sim(), n = (o: object) => Object.keys(o).length, O = propCols(PROP_CAP), names = [...PROP_F64, ...PROP_U32];
  // (the layout this file works out is the module's own: every column where props.rs has it, the part as long)
  const off = names.filter((k, i) => x.props_cap(100 + i) !== (i < PROP_F64.length ? O[k] * 8 : O[k] * 4));
  if (x.props_cap(0) !== PROP_CAP || x.props_cap(13) !== O.bytes || off.length || x.props_cap(100 + names.length) !== 0 || x.props_cap(6) !== PROP_PARS || x.props_cap(9) !== n(PD) || x.props_cap(10) !== n(PK) || x.props_cap(11) !== n(PY) || x.props_cap(12) !== PROP_EV || n(PE) !== PROP_EV)
    throw new Error(`the Rust module lays the furniture out as ${x.props_cap(0)} rows in ${x.props_cap(13)} bytes and src/sim/propshome.ts as ${PROP_CAP} in ${O.bytes}${off.length ? ` (columns not where the module has them: ${off.join(', ')})` : ''}, or the hand-over's tables are not the same width: bun run tools/sim/build.ts`);
  return IO = { x, def: x.props_addr(0) / 8, con: x.props_addr(2) / 8, leaf: x.props_addr(3) / 8, way: x.props_addr(4) / 8, ev: x.props_addr(5) / 8, par: x.props_addr(6) / 8,
    maxContacts: x.props_cap(2), maxLeaves: x.props_cap(3), maxWays: x.props_cap(4), maxEvents: x.props_cap(5) };
}
/** the module's memory as doubles, for the hand-over (never kept) */
export const propMem = f64;

/** COUNTS FOR THE CHECKS (the host's: no game reads them): steps the module took and the TypeScript took, noises the module listed, steps answered both
 *  ways (collision.ts BOTH) and how many of those differed, with the first in words; and what the module's steps met (leaves that swept a thing, frames
 *  that guided one, things knocking each other, a thing against a static, men held back, substeps), so a check can say its run was a fair one */
export const PROPS_STEPPED = { there: 0, here: 0, noises: 0, asked: 0, differ: 0, first: '', swept: 0, guided: 0, pairs: 0, walls: 0, held: 0, substeps: 0 };

/** WHERE TWO TAKES OF A STORE'S BYTES DIFFER, in words (`bytes()` of the same store, twice): the clock, the counts, then column by column and row by
 *  row — the doubles by their value (a NaN is a NaN), the words by theirs. → at most `most` lines, none if they are the same state */
export function propDiff(S: PropStore, a: Uint8Array, b: Uint8Array, most = 4): string[] {
  const O = S.O, da = new Float64Array(a.buffer, a.byteOffset, 1), db = new Float64Array(b.buffer, b.byteOffset, 1), ua = new Uint32Array(a.buffer, a.byteOffset, 4), ub = new Uint32Array(b.buffer, b.byteOffset, 4), out: string[] = [];
  if (!Object.is(da[0], db[0])) out.push(`the clock ${da[0]} | ${db[0]}`);
  if (ua[2] !== ub[2] || ua[3] !== ub[3]) out.push(`${ua[2]} of ${ua[3]} things in the level | ${ub[2]} of ${ub[3]}`);
  return colDiff(O, PROP_F64, PROP_U32, a, b, S.total, (u, r) => `thing ${u[O.id + r]}`, most, out);
}
