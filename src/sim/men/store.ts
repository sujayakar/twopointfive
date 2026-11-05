// THE MEN'S ROWS — where a man's columns stand, and who stands in which row. The columns themselves are generated (schema.ts → cols.ts, rust/sim/src/men.rs).
//
// THE BYTES ARE ONE LAYOUT WHEREVER THEY STAND. A game's men are the `men` part of the game's block of the module's memory as soon as its first tick opens (the tenant below: one
// copy, then every read and write is the block's, and a rule in Rust reads the same bytes with no copy). Until then, and for a man of no game at all (a menu's figure, the viewer's, a
// tool's), they are an array of the store's own, read through the same accessors. A column reads as `d[b8 + <its place> + row]` or `u[b4 + <its place> + row]` either way.
//
// A ROW IS A MAN'S FOR HIS LIFE. `enter` gives the lowest free row, zeros; `leave` frees it. The generation column counts each change of hands (odd: someone stands there), so a
// HANDLE, what a column of one man keeps of another (`handleOf`: generation and row in one word, never 0), is caught by name when the row has changed hands since (`rowOf`), and
// never read as the man who stands there now. A man's own object that has left holds GONE instead of a store, so reading him afterwards throws by name too.
//
// (Imports sim.ts and state.ts alone: like the rest of src/sim, this is below the game.)
import { sim, simWhyNot } from '../sim';
import { simTenant, simPart, P_MEN, type SimState } from '../state';
import { HostOwned } from '../../math/side';
import { MEN_F64, MEN_U32, MEN_HEAD, MEN_GEN, MEN_SCHEMA, type ManCols } from './cols';
import { MEN_ROWS } from './schema';

/** the part's length in bytes, and where each column starts (a double's as an index into the doubles, a word's into the words) */
const MEN_AT: readonly number[] = (() => { const out: number[] = []; let at = MEN_HEAD; for (const _ of MEN_F64) { out.push(at / 8); at += MEN_ROWS * 8; } for (const _ of MEN_U32) { out.push(at / 4); at += MEN_ROWS * 4; } return out; })();
const MEN_BYTES = MEN_HEAD + MEN_F64.length * MEN_ROWS * 8 + MEN_U32.length * MEN_ROWS * 4;
const NF = MEN_F64.length;

let held = false;
/** the module's table is this file's table, once a process: a built.ts of another schema is a mistake, said by name */
function holdSchema(): void {
  if (held || simWhyNot()) return;
  held = true;
  const x = sim(), names = [...MEN_F64, ...MEN_U32];
  const off = names.filter((_, i) => x.men_col(i) !== (i < NF ? MEN_AT[i] * 8 : MEN_AT[i] * 4));
  if (x.men_schema() >>> 0 !== MEN_SCHEMA || x.men_col(names.length) !== MEN_ROWS || off.length)
    throw new Error(`the Rust module lays the men out as schema ${(x.men_schema() >>> 0).toString(16)} in ${x.men_col(names.length)} rows and src/sim/men/cols.ts as ${MEN_SCHEMA.toString(16)} in ${MEN_ROWS}${off.length ? ` (columns not where the module has them: ${off.join(', ')})` : ''}: bun run tools/sim/men.ts, then bun run tools/sim/build.ts`);
}

/** the words of a store's own bytes. A view holds nothing of its own, and it is kept out of the store's fields so that a snapshot does not take the same bytes twice */
let ownFor: Float64Array | null = null, ownView: Uint32Array | null = null;
function ownWords(d: Float64Array): Uint32Array { if (ownFor !== d) { ownFor = d; ownView = new Uint32Array(d.buffer, d.byteOffset, d.byteLength >>> 2); } return ownView!; }

/** THE COLUMNS OF ONE GAME'S MEN (or of the men of no game: `loneMen`) */
export class MenStore {
  /** the store's own bytes while they are not at home in a game's block */
  private own: Float64Array | null = new Float64Array(MEN_BYTES >>> 3);
  /** the game's block while they are at home there */
  private home: SimState | null = null;
  /** where the part starts where the columns stand now: an index into the doubles, and into the words (0 in `own`) */
  b8 = 0; b4 = 0;
  constructor() { holdSchema(); }
  /** the doubles and the words the columns are in now: asked at every use, because a block's views die when the block moves */
  get d(): Float64Array { const h = this.home; return h !== null ? h.d : this.own!; }
  get u(): Uint32Array { const h = this.home; return h !== null ? h.w : ownWords(this.own!); }
  /** THE TENANT'S MOVE: the bytes into the part of block `st` that starts at byte `at`, once */
  moveIn(st: SimState, at: number): void {
    const from = this.own; if (from === null) return;
    new Uint32Array(st.w.buffer, st.w.byteOffset + at, MEN_BYTES >>> 2).set(ownWords(from));
    this.home = st; this.own = null; this.b8 = at >>> 3; this.b4 = at >>> 2;
  }
  private get gens(): number { return this.b4 + MEN_GEN; }
  /** A ROW FOR A NEW MAN: the lowest free one, zeros. A game cannot hold more men than there are rows (game/levelfit.ts `manrows`): past them is a mistake, said by name */
  enter(): number {
    const u = this.u, g = this.gens;
    for (let r = 0; r < MEN_ROWS; r++) if ((u[g + r] & 1) === 0) { u[g + r] += 1; return r; }
    throw new Error(`a game of more than ${MEN_ROWS} men: sim/men/schema.ts MEN_ROWS (game/levelfit.ts menHeld says a game cannot hold that many: who stood a man past it?)`);
  }
  /** his row is free again, and zeros */
  leave(r: number): void {
    const u = this.u, d = this.d, g = this.gens;
    if ((u[g + r] & 1) === 0) throw new Error(`row ${r} of the men left twice`);
    for (let c = 0; c < MEN_AT.length; c++) { if (c < NF) d[this.b8 + MEN_AT[c] + r] = 0; else if (MEN_AT[c] !== MEN_GEN) u[this.b4 + MEN_AT[c] + r] = 0; }
    u[g + r] += 1;
  }
  /** a word that names the man in row `r` for as long as he stands there: never 0, so a column of zeros is "nobody" */
  handleOf(r: number): number { return (this.u[this.gens + r] << 8 | r) >>> 0; }
  /** the row a handle names: whoever holds a handle to a man who has left is told so, by name */
  rowOf(h: number): number {
    const r = h & 255, g = h >>> 8, now = this.u[this.gens + r];
    if (g !== now) throw new Error(`a stale handle to a man: row ${r} is at generation ${now}, the handle was taken at ${g}`);
    return r;
  }
  /** how many men stand here now (for a check and a panel) */
  get standing(): number { const u = this.u, g = this.gens; let n = 0; for (let r = 0; r < MEN_ROWS; r++) n += u[g + r] & 1; return n; }
}

/** WHAT A MAN WHO HAS LEFT HOLDS in place of a store: asking it anything throws by name. The host's, one for all games: a snapshot keeps it as it is */
class Gone extends HostOwned {
  private no(): never { throw new Error('a man was read or written after he left the game (sim/men/store.ts): his row may be someone else\'s now'); }
  get d(): Float64Array { return this.no(); }
  get u(): Uint32Array { return this.no(); }
  b8 = 0; b4 = 0;
}
const GONE = new Gone() as unknown as MenStore;

/** a man's row and store, written into his object as he is made */
export function standMan(m: ManCols, S: MenStore): void { m.S = S; m.r = S.enter(); }
/** …and the same row for a second object of the same man (a Guard record and his Character) */
export function sameMan(m: ManCols, of: ManCols): void { m.S = of.S; m.r = of.r; }
/** he has left the game: his row is free, and every object of him (`m`, `also`) now throws by name when read */
export function manLeaves(m: ManCols, ...also: ManCols[]): void { const S = m.S; if (S !== GONE) S.leave(m.r); for (const x of [m, ...also]) { x.S = GONE; x.r = -1; } }

// THE MEN OF NO GAME (a menu's figure, the viewer's man, a tool's): one store until it is full, and then another. Nobody has their rows back: what is left of a full store goes
// with the last of its men.
let lone: MenStore | null = null, loneRows = 0;
/** the store for a man of no game */
export function loneMen(): MenStore {
  if (lone === null || loneRows === MEN_ROWS) { lone = new MenStore(); loneRows = 0; }
  loneRows++; return lone;
}

/** what the tenant asks of a game: its men's columns */
interface HasMen { men?: MenStore }
// a game's men move into its block as its first tick opens, once, and stay (state.ts settle)
simTenant({ name: 'men', bit: 256,
  wants(g: HasMen) { return !!g.men && !simWhyNot(); },
  into(g: HasMen, st: SimState) { g.men!.moveIn(st, simPart(P_MEN)); },
  outOf() { throw new Error('a game\'s men do not leave its block'); } });
