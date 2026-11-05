// THE LEAVES OFF THEIR HINGES, AS COLUMNS — where each loose slab's own numbers are kept (game/doorblast.ts reads and writes them through `Door.slab` and
// steps the ones in the air), in TypeScript and in the Rust module alike, and how they move between the two.
//
// THE LAYOUT IS ONE, BYTE FOR BYTE (rust/sim/src/slabs.rs Slabs), and it is the furniture's kind of layout (sim/propshome.ts, props.rs: dense rows, a
// column a number as narrow as its values, hot before cold, every column whole vectors on a 16-byte boundary). The bytes are the `slabs` part of the
// game's block from the moment the doors are made (sim/state.ts, sim/colstore.ts), and a slab in the air is stepped over them, in the module (slabs.rs
// step, the row RUST.slabs on) or in game/doorblast.ts. A column is read as `d[b8 + O.px + row]` (the doubles) or `u[b4 + O.flags + row]` (the words).
//
// ROWS. Rows 0 .. fly are the slabs in the air, IN THE ORDER OF THEIR DOORS (the order they are stepped in: a slab's rays meet the ones stepped before it
// where this tick left them); rows fly .. n the ones lying at rest, which nothing steps. `door[row]` is whose leaf a row is; `row[door]` where a door's
// slab is PLUS ONE, 0 = that door has none — so a part of zeros is no slab at all. `enter`, `rest` and `leave` are the three writes that move rows, and
// nothing else does: no rule in the module moves one.
//
// A truth is a bit of `flags` (SLF); who is blamed is his id (an i32: read `| 0`).
//
// (Imports sim.ts, state.ts and colstore.ts alone: like the rest of src/sim, this is below the game.)
import { sim, f64, type SimExports } from './sim';
import { simRoom, P_SLABS, type SimState } from './state';
import { ColStore, colsOf, colDiff, type ColsOf } from './colstore';

/** slabs a level can have and still keep them in the module: one a door (rust/sim/src/slabs.rs CAP = doors.rs CAP) */
export const SLAB_CAP = 64;
/** THE COLUMNS in the part's order (slabs.rs COLS; tools/qa/rust-check.ts holds the names and slabIo() the offsets to the module's own): the doubles —
 *  the eleven a slab in the air reads every substep first — then the words */
export const SLAB_F64 = ['px', 'py', 'pz', 'vx', 'vy', 'vz', 'a', 'spin', 'tip', 'tipV', 't', 'ang', 'wedged', 'bw', 'bl', 'bx', 'by'] as const;
export const SLAB_U32 = ['flags', 'who', 'door', 'row'] as const;
/** the head in front of the columns: slabs in all, and how many of them are in the air (two words; two more spare) */
export const SLAB_HEAD = 16;
/** a slab's truths (the bits of `flags`: slabs.rs F_REST, F_LANDED, F_BACK, F_BITS): lying at rest · slammed flat · its top fell the way side −1 names ·
 *  it is the lock rail of a leaf gunfire took apart (bw, bl, bx, by are its numbers) */
export const SLF = { rest: 1, landed: 2, back: 4, bits: 8 } as const;

/** A SLAB'S NUMBERS OUT OF ITS ROW (game/doors.ts DoorSlab is this): what `load` gives and `save` takes. `wedged`: the slab's own clock before which a
 *  slab with no clear spot in reach is not searched for again (absent = 0) */
export interface SlabNow { p: [number, number, number]; v: [number, number, number]; a: number; spin: number; tip: number; tipV: number; side: 1 | -1; rest: boolean; landed: boolean; who: number; t: number; ang: number; bits?: [number, number, number, number]; wedged?: number }

/** where each column starts for columns `cap` rows long (sim/colstore.ts colsOf) */
export type SlabCols = ColsOf<(typeof SLAB_F64)[number], (typeof SLAB_U32)[number]>;
export const slabCols = (cap: number): SlabCols => colsOf(SLAB_HEAD, SLAB_F64, SLAB_U32, cap);
/** the slabs' bit in the head of a game's block (sim/state.ts: one a family, never reused), and how long their part is */
const SLAB_BIT = 64;
simRoom(P_SLABS, slabCols(SLAB_CAP).bytes);
/** the word columns a row carries with it when it moves (`row` is the table that says where a door's slab went) */
const SWAPPED = ['flags', 'who', 'door'] as const;

/** THE COLUMNS OF ONE SET OF DOORS' SLABS (a level's, or a lone door's), over its game's block `st`: sim/colstore.ts's store, and what its rows mean. `cap`
 *  is SLAB_CAP, so that the bytes are the module's part as they stand; a level of more doors than that is not loaded (the doors' own columns say so first:
 *  sim/doorshome.ts). */
export class SlabStore extends ColStore<(typeof SLAB_F64)[number], (typeof SLAB_U32)[number]> {
  constructor(st: SimState, readonly doors: number) {
    if (doors > SLAB_CAP) throw new Error(`a level of ${doors} doors: the game holds ${SLAB_CAP} leaves off their hinges (sim/slabshome.ts SLAB_CAP, rust/sim/src/slabs.rs CAP)`);
    super(st, P_SLABS, SLAB_BIT, 'leaves off their hinges', SLAB_HEAD, SLAB_F64, SLAB_U32, SLAB_CAP);
  }
  /** slabs in all, and how many of them are in the air */
  get n(): number { return this.u[this.b4]; }
  get fly(): number { return this.u[this.b4 + 1]; }
  /** where door `door`'s slab is, or −1 */
  rowOf(door: number): number { return this.u[this.b4 + this.O.row + door] - 1; }
  /** whose leaf row `r` is */
  doorAt(r: number): number { return this.u[this.b4 + this.O.door + r]; }
  private swap(a: number, b: number): void {   // (rows a and b change places, every column; the table follows)
    if (a === b) return;
    this.swapCols(a, b, SLAB_F64, SWAPPED);
    const u = this.u, O = this.O, W = this.b4; u[W + O.row + u[W + O.door + a]] = a + 1; u[W + O.row + u[W + O.door + b]] = b + 1;
  }
  /** A LEAF COMES OFF (Door.slab set): a row for door `door`'s slab — among the ones in the air, in its door's place, or behind them if it lies at rest
   *  from the start. Its numbers are `save`'s to write. → the row */
  enter(door: number, flying: boolean): number {
    if (this.rowOf(door) >= 0) this.leave(door);
    const u = this.u, O = this.O, W = this.b4, n = u[W], fly = u[W + 1];
    u[W + O.door + n] = door; u[W + O.row + door] = n + 1; u[W] = n + 1;
    if (!flying) return n;
    this.swap(n, fly); u[W + 1] = fly + 1;   // (the first one at rest goes to the end, and this one stands last of those in the air …)
    let r = fly; while (r > 0 && u[W + O.door + r - 1] > door) { this.swap(r, r - 1); r--; }   // (… then in its door's place among them)
    return r;
  }
  /** A SLAB IN THE AIR CAME TO REST: its row goes behind the ones still in the air, those after it closing up in their order (first of the ones at rest) */
  rest(door: number): void {
    const fly = this.fly; let r = this.rowOf(door); if (r < 0 || r >= fly) return;
    for (; r < fly - 1; r++) this.swap(r, r + 1);
    this.u[this.b4 + 1] = fly - 1;
  }
  /** A DOOR IS HUNG AGAIN (Door.slab = null): its slab's row is given up, and is zeros again. → false if it had none */
  leave(door: number): boolean {
    if (this.rowOf(door) < 0) return false;
    this.rest(door);
    const n = this.n, r = this.rowOf(door); this.swap(r, n - 1);
    const d = this.d, u = this.u, O = this.O, B = this.b8, W = this.b4, z = n - 1;
    for (const k of SLAB_F64) d[B + O[k] + z] = 0;
    u[W + O.flags + z] = 0; u[W + O.who + z] = 0; u[W + O.door + z] = 0; u[W + O.row + door] = 0; u[W] = n - 1;
    return true;
  }
  /** a slab's numbers out of row `r` */
  load(r: number): SlabNow {
    const d = this.d, u = this.u, O = this.O, B = this.b8 + r, W = this.b4 + r, f = u[W + O.flags];
    const s: SlabNow = { p: [d[B + O.px], d[B + O.py], d[B + O.pz]], v: [d[B + O.vx], d[B + O.vy], d[B + O.vz]], a: d[B + O.a], spin: d[B + O.spin], tip: d[B + O.tip], tipV: d[B + O.tipV],
      side: f & SLF.back ? -1 : 1, rest: (f & SLF.rest) !== 0, landed: (f & SLF.landed) !== 0, who: u[W + O.who] | 0, t: d[B + O.t], ang: d[B + O.ang], wedged: d[B + O.wedged] };
    if (f & SLF.bits) s.bits = [d[B + O.bw], d[B + O.bl], d[B + O.bx], d[B + O.by]];
    return s;
  }
  /** … and into it (the row stays where it is: `rest` moves a slab that has come to rest) */
  save(r: number, s: SlabNow): void {
    const d = this.d, u = this.u, O = this.O, B = this.b8 + r, W = this.b4 + r, b = s.bits;
    d[B + O.px] = s.p[0]; d[B + O.py] = s.p[1]; d[B + O.pz] = s.p[2]; d[B + O.vx] = s.v[0]; d[B + O.vy] = s.v[1]; d[B + O.vz] = s.v[2];
    d[B + O.a] = s.a; d[B + O.spin] = s.spin; d[B + O.tip] = s.tip; d[B + O.tipV] = s.tipV; d[B + O.t] = s.t; d[B + O.ang] = s.ang; d[B + O.wedged] = s.wedged ?? 0;
    d[B + O.bw] = b ? b[0] : 0; d[B + O.bl] = b ? b[1] : 0; d[B + O.bx] = b ? b[2] : 0; d[B + O.by] = b ? b[3] : 0;
    u[W + O.flags] = (s.rest ? SLF.rest : 0) | (s.landed ? SLF.landed : 0) | (s.side < 0 ? SLF.back : 0) | (b ? SLF.bits : 0); u[W + O.who] = s.who;
  }
}

// ---- the hand-over of one slab's step (rust/sim/src/slabs.rs PAR, by the same numbers)
export const SP = { dt: 0, row: 1, height: 2, thick: 3, hu: 4, closed: 5, fcx: 6, fcz: 7, but: 8, ask: 9, scrape: 10, slam: 11, slide: 12, bounce: 13, tip: 14,
  landed: 16, landX: 17, landZ: 18, rested: 19, nudged: 20, nudgeMax: 21, laid: 22, rays: 23, gathers: 24, substeps: 25 } as const;
export const SLAB_PARS = 32;
/** the step's answers (slabs.rs STEPPED, HOST, WRONG) */
export const SLAB_STEPPED = 0, SLAB_HOST = 1;

/** where the hand-over is in the module's memory (an index into its doubles): asked once */
export interface SlabIo { x: SimExports; par: number }
let IO: SlabIo | null = null;
export function slabIo(): SlabIo {
  if (IO) return IO;
  const x = sim(), O = slabCols(SLAB_CAP), names = [...SLAB_F64, ...SLAB_U32];
  // (the layout this file works out is the module's own: every column where slabs.rs has it, the part as long)
  const off = names.filter((k, i) => x.slabs_cap(100 + i) !== (i < SLAB_F64.length ? O[k] * 8 : O[k] * 4));
  if (x.slabs_cap(0) !== SLAB_CAP || x.slabs_cap(13) !== O.bytes || off.length || x.slabs_cap(100 + names.length) !== 0 || x.slabs_cap(1) !== SLAB_PARS)
    throw new Error(`the Rust module lays the slabs out as ${x.slabs_cap(0)} rows in ${x.slabs_cap(13)} bytes and src/sim/slabshome.ts as ${SLAB_CAP} in ${O.bytes}${off.length ? ` (columns not where the module has them: ${off.join(', ')})` : ''}, or the hand-over is not the same length: bun run tools/sim/build.ts`);
  return IO = { x, par: x.slabs_addr(0) / 8 };
}
/** the module's memory as doubles, for the hand-over (never kept) */
export const slabMem = f64;

/** COUNTS FOR THE CHECKS (the host's: no game reads them): steps of a slab the module took, the TypeScript took, the module handed back (none in
 *  play: a ray that is no number), steps answered both ways and how many of those differed, with the first in words; and what the module's steps asked */
export const SLABS_STEPPED = { there: 0, here: 0, host: 0, asked: 0, differ: 0, first: '', rays: 0, gathers: 0, substeps: 0, landed: 0, rested: 0 };

/** WHERE TWO TAKES OF A STORE'S BYTES DIFFER, in words (`bytes()` of the same store, twice). → at most `most` lines, none if they are the same state */
export function slabDiff(S: SlabStore, a: Uint8Array, b: Uint8Array, most = 4): string[] {
  const O = S.O, ua = new Uint32Array(a.buffer, a.byteOffset, O.bytes >>> 2), ub = new Uint32Array(b.buffer, b.byteOffset, O.bytes >>> 2), out: string[] = [];
  if (ua[0] !== ub[0] || ua[1] !== ub[1]) out.push(`${ua[0]} slabs, ${ua[1]} in the air | ${ub[0]}, ${ub[1]}`);
  for (let k = 0; k < S.doors && out.length < most; k++) if (ua[O.row + k] !== ub[O.row + k]) out.push(`door ${k}'s row (plus one) ${ua[O.row + k]} | ${ub[O.row + k]}`);
  return colDiff(O, SLAB_F64, SWAPPED, a, b, Math.max(ua[0], ub[0]), (u, r) => `door ${u[O.door + r]}`, most, out);
}
