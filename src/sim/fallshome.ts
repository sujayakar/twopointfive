// WHAT FALLS, AND WHAT IT LEAVES — the landings the simulation has scheduled for things that let go, and the LITTER map (m² of broken glass in each
// half-metre cell of the floor: what makes a footfall louder), as one flat store a game: in TypeScript, and at home in the Rust module's block while the row
// RUST.falls is on (rust/sim/src/falls.rs has the layout and the same five rules, statement for statement).
//
// THIS IS THE GAMEPLAY HALF OF WHAT BREAKS. The landings wait in LISTS, one for each kind of thing that falls, each with its own count and its own memory
// of when the floor was last told (so one kind's cap, gap or going inert never moves another's):
//   FALL_GLASS   fragments of a pane (game/shards.ts): when a fragment reaches the floor, whether the floor hears it, the glass it leaves there
//   FALL_PIECES  the pieces a thing that breaks throws (game/debris.ts, under DEBRIS.dress): when and where each comes down, and that the floor hears it
// The plates, the chunks and the spray a player sees are dressing and never write here.
//
// Every rule below is ONE function with two bodies — the module's (`there`, the row on and the store at home) and this file's (`here`) — and a third road
// for the checks (`FALLS.both`: both taken from the same store, everything they leave compared; the game goes on with this file's). No length or angle is
// worked out by either, so the row answers under either maths.
//
// (Imports sim.ts and state.ts alone: like the rest of src/sim, this is below the game.)
import { sim, f64, RUST, ruleFell, ruleRed } from './sim';
import { simTenant, simPart, P_FALLS, type SimState } from './state';

/** the floor's cells (rust/sim/src/falls.rs: the world's floor rectangle, 40 m × 28 m from the origin, in half-metre cells) and the landings that can wait */
export const FALL_CELL = 0.5, FALL_CELLS_X = 80, FALL_CELLS_Z = 56, FALL_CELL_MAX = 0.25, FALL_MAXP = 96;
/** the lists of landings (falls.rs L_…) */
export const FALL_GLASS = 0, FALL_PIECES = 1, FALL_LISTS = 2;
const NCELLS = FALL_CELLS_X * FALL_CELLS_Z;
/** a landing's columns (falls.rs P_…) */
export const FP = { t: 0, x: 1, y: 2, z: 3, name: 4, gain: 5, rate: 6, amount: 7, who: 8, event: 9, level: 10, dust: 11 } as const;
const NP = 12, DUE = 10;
/** where the store's parts start (doubles): the cells, each list's columns, then how many wait in each list · has glass been laid · when the floor was last
 *  told of each list's */
const LIST = NP * FALL_MAXP, O_P = NCELLS, O_N = NCELLS + FALL_LISTS * LIST, O_ANY = O_N + FALL_LISTS, O_LAST = O_ANY + 1;
export const FALL_LEN = O_LAST + FALL_LISTS + 1;
/** the sound a landing comes with, as its column holds it (its place here; none = −1, a landing the simulation only tells the floor of) */
export const FALL_SOUNDS = ['glassTinkle', 'glassShatter'] as const;
export type FallSound = typeof FALL_SOUNDS[number] | '';
/** is the floor told of a landing: no · only if it has a sound of its own · yes */
export const FALL_UNHEARD = 0, FALL_IF_SOUNDED = 1, FALL_HEARD = 2;
/** a landing whose time has come, as the rule that scheduled it takes it back */
export interface FallDue { name: FallSound; pos: [number, number, number]; gain: number; rate: number; heard: boolean; level: number; who: number;
  /** the dust it raises where it lands (a puff's scale; 0 = none) */
  dust: number }

/** COUNTS FOR THE CHECKS (the host's: no game reads them): the rules the module took and this file took, by name, and — under `both` — how many were
 *  answered both ways, how many differed, and the first that did in words */
export const FALLS = { both: false, there: 0, here: 0, asked: 0, differ: 0, first: '', dues: 0, steps: 0, crunches: 0 };

export class FallStore {
  own = new Float64Array(FALL_LEN);
  /** the game's block of the module's memory while the store is at home there (RUST.falls), else null */
  home: SimState | null = null;
  /** where the store starts in `home.d` (0 in `own`) */
  base = 0;
  constructor() { this.own.fill(-1e9, O_LAST, O_LAST + FALL_LISTS); }
  private get d(): Float64Array { const h = this.home; return h !== null ? h.d : this.own; }

  // ---- reads (no call: the store is read where it stands)
  /** landings waiting in the glass's list */
  get waiting(): number { return this.d[this.base + O_N]; }
  /** landings waiting in list `l` */
  waitingIn(l: number): number { return this.d[this.base + O_N + l]; }
  /** has any glass been laid since the map was cleared */
  get any(): boolean { return this.d[this.base + O_ANY] !== 0; }
  /** m² of glass in the cell under (x, z) */
  at(x: number, z: number): number { const k = cell(x, z); return k === NCELLS ? 0 : this.d[this.base + k]; }
  /** the cells that hold glass: [cell x, cell z, m²], row by row */
  cells(): [number, number, number][] { const d = this.d, o = this.base, out: [number, number, number][] = []; for (let k = 0; k < NCELLS; k++) if (d[o + k] !== 0) out.push([k % FALL_CELLS_X, (k / FALL_CELLS_X) | 0, d[o + k]]); return out; }
  /** landing i that waits in list `l`, by its columns' names */
  landing(i: number, l = FALL_GLASS): Record<keyof typeof FP, number> { const d = this.d, o = this.base + O_P + l * LIST, r = {} as Record<keyof typeof FP, number>; for (const k of Object.keys(FP) as (keyof typeof FP)[]) r[k] = d[o + FP[k] * FALL_MAXP + i]; return r; }

  // ---- the rules (`l`: the list a landing waits in; the glass's where it is not said)
  /** a landing put on list `l` — or, the list at `cap`, its glass laid now and its sound thickening the last landing of the list that has one */
  schedule(t: number, x: number, y: number, z: number, name: FallSound, gain: number, rate: number, amount: number, who: number, event: number, level: number, cap: number, l = FALL_GLASS, dust = 0): boolean {
    const n = (FALL_SOUNDS as readonly string[]).indexOf(name);
    return this.rule('schedule', () => sim().falls_schedule(this.home!.bind(), l, t, x, y, z, n, gain, rate, amount, who, event, level, dust, cap) !== 0, () => scheduleHere(this.d, this.base, l, t, x, y, z, n, gain, rate, amount, who, event, level, dust, cap));
  }
  /** shards.ts deposit: `amount` m² of glass laid at (x, z) */
  deposit(x: number, z: number, amount: number): void { this.rule('deposit', () => { sim().falls_deposit(this.home!.bind(), x, z, amount); return 0; }, () => { depositHere(this.d, this.base, x, z, amount); return 0; }); }
  /** the landings of list `l` whose time has come, in order: each one's glass laid and the floor told (one event a `gap`, at `level` unless the landing has its own) */
  due(time: number, level: number, gap: number, l = FALL_GLASS): FallDue[] {
    if (this.waitingIn(l) === 0) return NONE;
    FALLS.dues++;
    return this.rule('due', () => { const n = sim().falls_due(this.home!.bind(), l, time, level, gap), m = f64(), at = sim().sim_scratch() / 8, out: FallDue[] = [];
      for (let i = 0, o = at; i < n; i++, o += DUE) out.push({ name: FALL_SOUNDS[m[o]] ?? '', pos: [m[o + 1], m[o + 2], m[o + 3]], gain: m[o + 4], rate: m[o + 5], heard: m[o + 6] !== 0, level: m[o + 7], who: m[o + 8], dust: m[o + 9] });
      return out; }, () => dueHere(this.d, this.base, l, time, level, gap));
  }
  /** shards.ts step: a footfall at (x, z) of hearing level `level` → the level to file, and how much glass was underfoot (0‥1; 0 = no crunch) */
  step(x: number, z: number, level: number, crunchFull: number, grind: number, gain: number, add: number): { level: number; k: number } {
    FALLS.steps++;
    const r = this.rule('step', () => { const l = sim().falls_step(this.home!.bind(), x, z, level, crunchFull, grind, gain, add); return { level: l, k: f64()[sim().sim_scratch() / 8] }; }, () => stepHere(this.d, this.base, x, z, level, crunchFull, grind, gain, add));
    if (r.k > 0) FALLS.crunches++;
    return r;
  }
  /** may the floor be told of a landing of list `l` at `time` (one a `gap`, floor-wide)? If so the telling is remembered. */
  heard(time: number, gap: number, l = FALL_GLASS): boolean { return this.rule('heard', () => sim().falls_heard(this.home!.bind(), l, time, gap) !== 0, () => { const d = this.d, o = this.base + O_LAST + l; if (time - d[o] < gap) return false; d[o] = time; return true; }); }
  /** nothing waits in list `l`, and the floor's memory of its last landing is new — with the glass's list, no glass lies either; `listOnly`: the landings
   *  alone are dropped (the rule went inert) */
  clear(listOnly = false, l = FALL_GLASS): void {
    this.rule('clear', () => { sim().falls_clear(this.home!.bind(), l, listOnly ? 0 : 1, -1e9); return 0; }, () => { clearHere(this.d, this.base, l, listOnly); return 0; });
  }

  /** ONE RULE, there or here — or both ways for the checks (FALLS.both): the module's taken first with nothing of it kept, then this file's from the same
   *  store, and the two stores and answers compared to the bit */
  private rule<T>(what: string, there: () => T, here: () => T): T {
    if (!RUST.falls || this.home === null || ruleRed('falls')) { if (RUST.falls && this.home === null) ruleFell('falls', 'the store is not at home in a block of the module'); FALLS.here++; return here(); }
    if (!FALLS.both) { FALLS.there++; return there(); }
    const o = this.base, was = this.home.d.slice(o, o + FALL_LEN), a = there(), after = this.home.d.slice(o, o + FALL_LEN); this.home.d.set(was, o);
    const b = here(), d = this.home.d; let bad = JSON.stringify(a, nanSafe) === JSON.stringify(b, nanSafe) ? '' : `answers ${JSON.stringify(b)} here, ${JSON.stringify(a)} there`;
    for (let i = 0; i < FALL_LEN && !bad; i++) if (!Object.is(d[o + i], after[i])) bad = `double ${i} of the store ${d[o + i]} here, ${after[i]} there`;
    FALLS.asked++; if (bad) { FALLS.differ++; if (!FALLS.first) FALLS.first = `falls.${what}: ${bad}`; }
    return b;
  }
}
const NONE: FallDue[] = [];
/** a number as text that keeps what JSON drops (−0, NaN, the infinities) */
const nanSafe = (_k: string, v: unknown) => typeof v === 'number' && (!Number.isFinite(v) || Object.is(v, -0)) ? `#${Object.is(v, -0) ? '-0' : v}` : v;

// ---- this file's bodies of the rules (falls.rs, statement for statement, over the same doubles)
function cell(x: number, z: number): number { const ix = Math.floor(x / FALL_CELL), iz = Math.floor(z / FALL_CELL); return ix >= 0 && ix < FALL_CELLS_X && iz >= 0 && iz < FALL_CELLS_Z ? iz * FALL_CELLS_X + ix : NCELLS; }
function depositHere(d: Float64Array, o: number, x: number, z: number, amount: number): void {
  if (amount <= 0) return;
  const k = cell(x, z); if (k === NCELLS) return;
  d[o + k] = Math.min(FALL_CELL_MAX, d[o + k] + amount); d[o + O_ANY] = 1;
}
function scheduleHere(d: Float64Array, o: number, l: number, t: number, x: number, y: number, z: number, name: number, gain: number, rate: number, amount: number, who: number, event: number, level: number, dust: number, cap: number): boolean {
  if (!(l >= 0 && l < FALL_LISTS)) return false;
  const n = d[o + O_N + l], P = o + O_P + l * LIST;
  if (n >= cap || n >= FALL_MAXP) {
    depositHere(d, o, x, z, amount);
    if (gain > 0) for (let i = n - 1; i >= 0; i--) { const q = P + FP.gain * FALL_MAXP + i; if (d[q] > 0) { d[q] = Math.min(1, d[q] + gain * 0.2); break; } }
    return false;
  }
  const row = [t, x, y, z, name, gain, rate, amount, who, event, level, dust]; for (let k = 0; k < NP; k++) d[P + k * FALL_MAXP + n] = row[k];
  d[o + O_N + l] = n + 1; return true;
}
function dueHere(d: Float64Array, o: number, l: number, time: number, level: number, gap: number): FallDue[] {
  if (!(l >= 0 && l < FALL_LISTS)) return [];
  const n = d[o + O_N + l], P = o + O_P + l * LIST, M = FALL_MAXP, L = o + O_LAST + l, out: FallDue[] = []; let w = 0;
  for (let i = 0; i < n; i++) {
    if (d[P + FP.t * M + i] > time) { if (w !== i) for (let k = 0; k < NP; k++) d[P + k * M + w] = d[P + k * M + i]; w++; continue; }
    const x = d[P + FP.x * M + i], z = d[P + FP.z * M + i], gain = d[P + FP.gain * M + i], ev = d[P + FP.event * M + i];
    depositHere(d, o, x, z, d[P + FP.amount * M + i]);
    let heard = false; if ((ev === 2 || (ev === 1 && gain > 0)) && !(time - d[L] < gap)) { d[L] = time; heard = true; }
    const lv = d[P + FP.level * M + i];
    out.push({ name: FALL_SOUNDS[d[P + FP.name * M + i]] ?? '', pos: [x, d[P + FP.y * M + i], z], gain, rate: d[P + FP.rate * M + i], heard, level: lv !== 0 ? lv : level, who: d[P + FP.who * M + i], dust: d[P + FP.dust * M + i] });
  }
  for (let i = w; i < n; i++) for (let k = 0; k < NP; k++) d[P + k * M + i] = 0;
  d[o + O_N + l] = w; return out;
}
function stepHere(d: Float64Array, o: number, x: number, z: number, level: number, crunchFull: number, grind: number, gain: number, add: number): { level: number; k: number } {
  const c = cell(x, z); if (c === NCELLS) return { level, k: 0 };
  const a = d[o + c]; if (a === 0 || a !== a) return { level, k: 0 };
  const k = Math.min(1, a / crunchFull); if (k < 0.08) return { level, k: 0 };
  d[o + c] = a * grind;
  return { level: level * (1 + gain * k) + add * k, k };
}
function clearHere(d: Float64Array, o: number, l: number, listOnly: boolean): void {
  if (!(l >= 0 && l < FALL_LISTS)) return;
  const P = o + O_P + l * LIST; d.fill(0, P, P + LIST); d[o + O_N + l] = 0;
  if (listOnly) return;
  d[o + O_LAST + l] = -1e9;
  if (l === FALL_GLASS) { d.fill(0, o, o + NCELLS); d[o + O_ANY] = 0; }
}

/** what the tenant asks of a game: its falls (game.ts Game.falls) */
interface HasFalls { falls?: FallStore }
// the store moves into a game's block as its tick opens with RUST.falls on, and out again with it off (state.ts settle)
simTenant({ name: 'falls', bit: 8,
  wants(g: HasFalls) { return RUST.falls && !!g.falls; },
  into(g: HasFalls, st: SimState) { const S = g.falls!, base = simPart(P_FALLS) >>> 3; st.d.set(S.own, base); S.home = st; S.base = base; },
  outOf(g: HasFalls, st: SimState) { const S = g.falls; if (!S || S.home === null) return; const o = S.base; S.own.set(st.d.subarray(o, o + FALL_LEN)); st.d.fill(0, o, o + FALL_LEN); S.home = null; S.base = 0; } });
