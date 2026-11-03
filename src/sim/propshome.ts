// THE FURNITURE'S STATE, AS COLUMNS — where every chair's, carton's, bin's and plant's own numbers are kept (game/props.ts reads and writes them), in
// TypeScript and in the Rust module alike, and how they move between the two.
//
// THE LAYOUT IS ONE, BYTE FOR BYTE (rust/sim/src/props.rs Props, which says why it is what it is: dense rows in the game's own order, a column a number as
// narrow as its values, hot before cold, every column whole vectors on a 16-byte boundary). While the row RUST.props is off the bytes are an ArrayBuffer of
// the game's own (`PropStore.own`); while it is on they are the `props` part of the game's block of the module's memory (sim/state.ts), and the furniture's
// step runs there (props.rs update) with no copy in or out: moving home is one copy of the bytes. Either way a column is read as `d[b8 + O.x + row]` (the
// doubles) or `u[b4 + O.flags + row]` (the words), so the step in props.ts and everything that reads a chair through `Prop.x` — the men who walk round it,
// a blast, a checkpoint file, the digest — reads the same values with the row on or off.
//
// ROWS AND IDS. Rows 0 .. n are the things in the level now, in the order of `PropSystem.props` (which this keeps: row k IS props[k]); rows n .. total the
// ones a blast took out, kept for the level put back. A thing's id never moves; `row[id]` and `id[row]` say who is where. `leave` and `home` are the two
// writes that move rows, and nothing else does.
//
// A truth is a bit of `flags` (PT), a man is his id (an i32: read `| 0`), a sound (PropEvent.sound) its place in PROP_SOUNDS.
//
// (Imports sim.ts and state.ts alone: like the rest of src/sim, this is below the game.)
import { sim, f64, RUST, type SimExports } from './sim';
import { simTenant, simPart, P_PROPS, type SimState } from './state';

/** things a level can have and still keep them in the module (rust/sim/src/props.rs CAP; more: they stay in TypeScript, the row on or not) */
export const PROP_CAP = 128;
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

/** where each column starts for columns `cap` rows long: the doubles as an index into doubles, the words as an index into words, from the part's start */
export interface PropCols { x: number; z: number; yaw: number; vx: number; vz: number; w: number; cs: number; sn: number; idle: number; pushT: number; noiseT: number; flags: number; who: number; id: number; row: number; bytes: number }
export function propCols(cap: number): PropCols {
  const o: Record<string, number> = {}; let at = PROP_HEAD;
  for (const k of PROP_F64) { o[k] = at / 8; at += cap * 8; }
  for (const k of PROP_U32) { o[k] = at / 4; at += cap * 4; }
  o.bytes = at; return o as unknown as PropCols;
}

/** the words of a store's own array: a view of the same bytes, kept for the store that asked last (a view holds nothing of its own, so it is no state of any
 *  game: two games stepped in turn each ask it again, which is one small object a turn) */
let wordsFor: Float64Array | null = null, words: Uint32Array | null = null;
function wordsOf(d: Float64Array): Uint32Array {
  if (wordsFor !== d || words === null) { wordsFor = d; words = new Uint32Array(d.buffer, d.byteOffset, d.byteLength >>> 2); }
  return words;
}

/** THE COLUMNS OF ONE SET OF FURNITURE (a level's, or a lone stage's). */
export class PropStore {
  /** rows a column (PROP_CAP, so that the bytes are the module's part as they stand; a level of more things than that keeps wider columns and stays here) */
  readonly cap: number;
  /** where each column starts (`O.x + row` into the doubles from `b8`, `O.flags + row` into the words from `b4`) */
  readonly O: PropCols;
  /** the game's own array of them while they are not at home. ONE array is the field: a snapshot takes every typed array it meets whole, so a second view
   *  of the same bytes kept beside this one would be the same bytes copied twice a snapshot. The words are a view asked of it (`wordsOf`) */
  private readonly own: Float64Array;
  /** the game's block of the module's memory while the columns are at home there (RUST.props), else null */
  home: SimState | null = null;
  /** where the part starts where the columns stand now: an index into the doubles, and into the words (0 in `own`) */
  b8 = 0; b4 = 0;
  /** the maths the kept cos and sin were worked out under (math/exact.ts mathsEpoch: the host's, game/props.ts keeps it fresh) */
  epoch = -1;
  constructor(readonly total: number) {
    const cap = this.cap = Math.max(PROP_CAP, (total + 3) & ~3), O = this.O = propCols(cap);
    const d = this.own = new Float64Array(O.bytes >>> 3), u = wordsOf(d);
    u[2] = total; u[3] = total;
    // (a thing as it is made: in its own row, asleep, to be placed, its clocks long past, the cos and sin of a yaw of 0)
    for (let i = 0; i < total; i++) { u[O.id + i] = i; u[O.row + i] = i; u[O.flags + i] = PT.asleep | PT.dirty; d[O.noiseT + i] = -10; d[O.pushT + i] = -10; d[O.cs + i] = 1; }
  }
  /** the doubles and the words the columns are in now (asked at every use: a block's views die when the block moves) */
  get d(): Float64Array { const h = this.home; return h !== null ? h.d : this.own; }
  get u(): Uint32Array { const h = this.home; return h !== null ? h.w : wordsOf(this.own); }
  /** the system's clock */
  get time(): number { return this.d[this.b8]; } set time(v: number) { this.d[this.b8] = v; }
  /** rows in the level now */
  get n(): number { return this.u[this.b4 + 2]; }
  /** where thing `id` is */
  rowOf(id: number): number { return this.u[this.b4 + this.O.row + id]; }
  private swap(a: number, b: number): void {   // (rows a and b change places, every column; the two tables follow)
    if (a === b) return;
    const d = this.d, u = this.u, O = this.O, B = this.b8, W = this.b4;
    for (const k of PROP_F64) { const o = B + O[k]; const t = d[o + a]; d[o + a] = d[o + b]; d[o + b] = t; }
    for (const k of ['flags', 'who', 'id'] as const) { const o = W + O[k]; const t = u[o + a]; u[o + a] = u[o + b]; u[o + b] = t; }
    u[W + O.row + u[W + O.id + a]] = a; u[W + O.row + u[W + O.id + b]] = b;
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
  /** the bytes of the part as they stand (a check's: every column, the head and the tables) */
  bytes(): Uint8Array { const u = this.u; return new Uint8Array(u.buffer, u.byteOffset + this.b4 * 4, this.O.bytes).slice(); }
  /** …and put back */
  put(b: Uint8Array): void { const u = this.u; new Uint8Array(u.buffer, u.byteOffset + this.b4 * 4, this.O.bytes).set(b); }
}

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

/** what the tenant asks of a game: its furniture's columns (game/props.ts PropSystem.store) */
interface HasProps { props?: { store: PropStore } }
// the furniture's bytes move into a game's block as its tick opens with RUST.props on, and out again with it off (state.ts settle): one copy each way
simTenant({ name: 'props', bit: 32,
  wants(g: HasProps) { const S = g.props?.store; return RUST.props && !!S && S.total > 0 && S.cap === PROP_CAP; },
  into(g: HasProps, st: SimState) {
    const S = g.props!.store, b = S.bytes(), at = simPart(P_PROPS);
    S.home = st; S.b8 = at >>> 3; S.b4 = at >>> 2; S.put(b);
  },
  outOf(g: HasProps, st: SimState) {
    const S = g.props?.store; if (!S || S.home === null) return;
    const b = S.bytes(), at = simPart(P_PROPS); st.w.fill(0, at >>> 2, (at + S.O.bytes) >>> 2);   // (a part nobody is at home in is zeros: state.rs)
    S.home = null; S.b8 = 0; S.b4 = 0; S.put(b);
  } });

/** WHERE TWO TAKES OF A STORE'S BYTES DIFFER, in words (`bytes()` of the same store, twice): the clock, the counts, then column by column and row by
 *  row — the doubles by their value (a NaN is a NaN), the words by theirs. → at most `most` lines, none if they are the same state */
export function propDiff(S: PropStore, a: Uint8Array, b: Uint8Array, most = 4): string[] {
  const O = S.O, da = new Float64Array(a.buffer, a.byteOffset, O.flags >>> 1), db = new Float64Array(b.buffer, b.byteOffset, O.flags >>> 1), ua = new Uint32Array(a.buffer, a.byteOffset, O.bytes >>> 2), ub = new Uint32Array(b.buffer, b.byteOffset, O.bytes >>> 2), out: string[] = [];
  if (!Object.is(da[0], db[0])) out.push(`the clock ${da[0]} | ${db[0]}`);
  if (ua[2] !== ub[2] || ua[3] !== ub[3]) out.push(`${ua[2]} of ${ua[3]} things in the level | ${ub[2]} of ${ub[3]}`);
  for (const k of PROP_F64) for (let r = 0; r < S.total && out.length < most; r++) if (!Object.is(da[O[k] + r], db[O[k] + r])) out.push(`row ${r} (thing ${ua[O.id + r]}).${k} ${da[O[k] + r]} | ${db[O[k] + r]}`);
  for (const k of PROP_U32) for (let r = 0; r < S.total && out.length < most; r++) if (ua[O[k] + r] !== ub[O[k] + r]) out.push(`row ${r}.${k} ${ua[O[k] + r]} | ${ub[O[k] + r]}`);
  return out;
}
