// THE BOX WORLD'S COPY IN THE RUST MODULE — what src/scene/collision.ts hands the systems that have moved to Rust (docs/agents/rust.md, "the box world's copy").
//
// WHOSE THE BOXES ARE. A box is a TypeScript object, and every rule that moves, hides, cuts or flags one writes that object. While those rules are TypeScript the
// objects are THE TRUTH and the module holds a copy (rust/sim/src/world.rs): flat tables a question there can walk without leaving the module. This file writes
// the copy and answers the one question a copy raises — is it the boxes as they are NOW:
//   · the copy answers only INSIDE A TICK (Game.stepSeats opens and shuts it). Outside one — the page's own questions, a tool's, a snapshot put back, a level
//     edited in the viewer — the TypeScript answers as it always did, and nobody has to say what they wrote.
//   · as a tick opens, EVERY static is compared with its row, value for value, and a row that differs is written again (about 25 µs for a level's 1,200
//     boxes). So whatever happened between ticks is in the copy before the first question, with no list of who may have written.
//   · inside the tick, a rule that writes a static's shape or flags and asks again before the tick ends says so (StaticCollision.wrote), and the leaves — which
//     swing every tick and are swapped for lists of a question's own — are copied by list (the list, its length, and a count that every write of a leaf moves:
//     `leafWrote`, said where a leaf is written).
//   · THE PANES THAT ARE NO LONGER WHOLE (a piece out, a piece about to go, toughened glass coming down, a pane gone) are a table of the copy too
//     (rust/sim/src/glass.rs): the rule that breaks glass is TypeScript and its state is the truth; every such pane's row is written as a tick opens and again
//     for every question asked while the glass is being worked on and the first one after (`glassBegin`, `glassEnd`: the rule that breaks a pane asks
//     lines of its own between its writes), and a burst's own view of the glass is written before the first question after it is put on or taken off
//     (`viewed`).
//   · a snapshot put back (game/rollback.ts) writes into the same objects: every copy is shut there and then (`mirrorsShut`), and made afresh, grid and
//     all, as its game's next tick opens (math/side.ts PUT_BACKS).
// tools/qa/rust-check.ts holds it: with the check's own switch every question of a played mission is answered both ways and the two compared, and every row of
// the copy is compared with its box at each question — so a write that forgot to say so is a counted miss, not a wrong game.
//
// A LEVEL TOO BIG FOR A TABLE IS NOT LOADED, and nothing in play outgrows one: game/levelfit.ts counts a stood game against every table here and refuses
// one over (stoodOver, the last statement of Game's constructor); the one rule that adds statics in play holds itself to the room the level left
// (game/wallbreak.ts WallBreak.room). Behind both, a table outgrown is a throw by name (`outgrown`): there is nobody else to answer.
//
// The copy is a cache of the host's: in no snapshot, never compared, a game's own (HostOwned). The module holds WORLDS of them — a game a world, the least
// recently stepped one giving way.
import { HostOwned, PUT_BACKS } from '../math/side';
import { mathsEpoch } from '../math/exact';
import { sim, simWhyNot, f64, u32, i32, u8, type SimExports } from '../sim/sim';
import { PROP_CAP, PROP_PARTS } from '../sim/propshome';
import { cutConfig, MAX_CUT_PLANES } from './cut';
import { shapeCut, type Box } from './boxes';
import type { SimState } from '../sim/state';

// rust/sim/src/world.rs: a row's doubles, the kinds, the tables by number
const REC = 16, R_COS = 6, R_SIN = 7, R_YAW = 8, R_ROT = 9, R_CUT = 13, R_CUTN = 14, R_KIND = 15, K_ROT = 1, K_ROUND = 2, K_CUT = 4, K_WALK = 8, F_GLASS = 1 << 17;
const T_REC = 0, T_LEAF = 1, T_CUT = 2, T_PANE_T = 3, T_FLAGS = 4, T_LEAF_FLAGS = 5, T_START = 6, T_ITEMS = 7, T_TILTED = 8, T_PANE_BI = 9, T_WORLDS = 10, T_PT = 12, T_LIST = 13, T_THING = 14, T_THING_LIVE = 15,
  T_GLASS = 16, T_GLASS_W = 17, T_GLASS_OF = 18, T_VIEW = 19, T_VIEW_BI = 20, T_SNOW = 21;
// rust/sim/src/glass.rs: a broken pane's row (GD doubles, GW words), where its parts are, its flags; a view's doubles a pane; glassPattern.ts's piece ids
const GD = 80, GW = 64, G_SITE = 9, G_SITES = 8, G_DICE_AT = 72, GW_FLAGS = 1, GW_COLS = 2, GW_ROWS = 3, GW_DSEED = 4, GW_N = 5, GW_SEED = 13, GW_OPEN = 21, GW_GOING = 42, GW_CRATER = 63, GW_BITS = 21,
  G_GONE = 1, G_DICE = 2, G_RUN = 4, VD = 4, FRAGS_PER_SITE = 84, SECTORS = 12;
// …the furniture's tables: which list of the leaves its parts are, the doubles kept of a thing by its id (x, z, yaw, radius), the words of a thing in the level (id, where its parts start, how many)
const THING_SET = 2, TH = 4, TL = 3;
// what a question may owe before the module answers it (BoxMirror.owes): the statics compared, the panes' rows written, the view's rows written
const OWE_BOXES = 1, OWE_GLASS = 2, OWE_VIEW = 4;
/** SHEETS OF GLASS A LEVEL MAY HAVE (rust/sim/src/world.rs MAX_GLASS and MAX_VIEW: a row a pane that is no longer whole, a row a pane a burst's view names
 *  — so no level that loads can break more than the copy holds; game/levelfit.ts says so as the level loads, and `boot` holds the number to the module's) */
export const PANE_CAP = 256;
const PLANE = 4, CUT_ROW = MAX_CUT_PLANES * PLANE;

/** who holds each of the module's worlds, and a count that says who stepped last (the host's: which game has which table is no part of any game) */
const HELD: (BoxMirror | null)[] = [];
const CLOCK = { n: 0 };
/** the copies made afresh in this process, counted: a copy's number (`gridDrop`) is no other copy's */
const COPIES = { n: 0 };
/** how many times a leaf has been written (the host's count; a copy of the leaves is good while it stands where it stood when the copy was made) */
export const LEAVES = { n: 0 };
/** A LEAF WAS WRITTEN — said by the three places that write one: a door placing its leaf (game/doors.ts Door.place), a door putting its leaf back whole
 *  (Door.reset), a slab posed off its hinges (game/doorblast.ts poseSlab). Whoever calls those, and when, is no part of the rule. */
export function leafWrote(): void { LEAVES.n++; }
/** how many times the furniture has been written: a thing placed (game/props.ts Prop.place — the one place a part's box is written), a thing taken out of
 *  the level or the level put back (the list the parts are copied in changed). The host's count, as LEAVES is. */
export const THINGS = { n: 0 };
export function thingWrote(): void { THINGS.n++; }
/** the furniture as the copy reads it (game/props.ts PropSystem): every thing by its id, and the ones in the level now in the furniture's own order */
export interface Furniture { readonly all: readonly FurnitureThing[]; readonly props: readonly FurnitureThing[] }
export interface FurnitureThing { readonly id: number; readonly x: number; readonly z: number; readonly yaw: number; readonly radius: number; readonly boxes: readonly Box[] }
/** A PANE THAT IS NO LONGER WHOLE, as the copy reads it (game/glassbreak.ts PaneState is one): which static, its cracks' sites, the pieces that are out
 *  (a byte a piece id), the pieces queued to leave, the dice of toughened glass coming down */
export interface PaneSite { readonly u: number; readonly v: number; readonly rot: number; readonly n: number; readonly seed: number; readonly rho: readonly number[]; readonly rz: number; readonly merge: number }
/** (the dice: `clumps` in the order they let go — by key, then by number — `next` of them said to be out, and `out` a byte a clump by its number, set as each goes) */
export interface PaneDice { readonly u: number; readonly v: number; readonly cols: number; readonly rows: number; readonly seed: number; readonly hu: number; readonly hv: number; readonly next: number; readonly out: Uint8Array; readonly clumps: readonly { readonly k: number; readonly key: number }[] }
export interface PaneRow { readonly bi: number; readonly gone: boolean; readonly run: boolean; readonly sites: readonly PaneSite[]; readonly open: Uint8Array; readonly pending: readonly { readonly fid: number }[]; readonly dice: PaneDice | null }
/** A BURST'S VIEW OF THE GLASS for the judgment in hand (glass.rs VIEW_*): `blown` = what a blast has just taken is no glass to it; `gives` = the panes a
 *  burst would break are counted out — which statics, and four doubles each: the pane's nearest point to the burst and how far round it the glass gives
 *  (below nought: the whole pane) */
export const VIEW_NONE = 0, VIEW_BLOWN = 1, VIEW_GIVES = 2;
/** THE ONE SENTENCE FOR A LEVEL THE GAME WILL NOT LOAD, wherever it is said (here; game/levelfit.ts levelOver and stoodOver; game/civilians.ts join): with the
 *  level's name where whoever says it knows one — a level loaded from a file has one (scene/levelLoad.ts levelNameOf), a generator's and a bare box world none. */
export const tooBig = (name: string | null | undefined, why: string): string => `${name ? `the level '${name}'` : 'this level'} is too big to load: ${why}`;
/** …and how a reader knows it (tools/qa/copyfit-check.ts) */
export const isTooBig = (said: string): boolean => /^(this level|the level '.*?') is too big to load: /.test(said);
export interface PaneView { readonly kind: number; readonly bi: readonly number[]; readonly at: readonly number[] }
/** THE GAME'S GLASS, for the copy (game/glassbreak.ts GlassBreak): every pane that is no longer whole, handed over one by one, and the view in hand (read
 *  only when a question is about to be asked of the module under it: the game may work its rows out then and not before) */
export interface PaneSource { /** (a tool's made-up glass, a world with no game behind it: tools/qa/item-ident.ts, slab-ident.ts) */ each?(to: (p: PaneRow) => void): void; readonly view: PaneView | null;
  /** …or the game's block, where the rule is the module's own (rust/sim/src/panes.rs): a line or a ray there reads each pane's row where it stands, nothing is copied, and a view is its numbers (`at`: the burst's point, reach, power, whether a man counts on it) */
  readonly home?: SimState | null;
  /** …and the level's sheets of glass, three numbers each (a static, how far it stands from a door's opening, which face has the sky: rust/sim/src/world.rs Sheet) */
  readonly sheets?: readonly number[] }
const NO_SHEETS: readonly number[] = [];
/** A SNAPSHOT WAS PUT BACK (game/rollback.ts): no copy stands. Every game's is shut where it is — a put-back inside a tick (a checkpoint loaded by the
 *  step's own last lines) leaves the TypeScript answering for what is left of it — and is made afresh as its next tick opens (PUT_BACKS). */
export function mirrorsShut(): void { for (const h of HELD) if (h) h.shut(); }

/** THE FLAGS THE COPY KEEPS (scene/boxes.ts BoxFlag): the ones a question of the CPU's is asked by — NoShadow and Dynamic of the low byte, and everything from
 *  bit 16 up (SoundSeal, Glass, Thin, Bar: the CPU's own). The rest of the low byte says how a box is DRAWN (Gloss comes and goes with the rain) and the second
 *  byte whose it is: a write to those is no news here. A question asked by any other flag is answered by the TypeScript (collision.ts). */
export const MIRRORED = (2 | 8 | ~0xffff) >>> 0;
/** THE NUMBERS THE HOST SHARES WITH THE SENSES' RUST, under the names they have there (light.rs LAMP: doubles a lamp, MAX_LAMPS: lamps a part of a
 *  question carries · smoke.rs BLOB: doubles a blob, MAX_BLOBS: the blobs its table holds, which is the most a game keeps — sense.ts CpuSmoke.emit ·
 *  hear.rs: what the straight lines said, and the way round · sight.rs: the question's bits and the answer's). tools/qa/rust-check.ts reads the Rust
 *  sources and is red where one of them is another number there. */
/** …and what a search across the floor answers that is no count of waypoints (rust/sim/src/nav.rs; the same check reads them): no way · the host's own
 *  search · the search waits for the prices it has listed (find_priced) */
export const NAV = { NO_WAY: -1, ASK_HOST: -2, NEED: -3 } as const;
export const FACE = { LAMP: 15, MAX_LAMPS: 256, BLOB: 6, MAX_BLOBS: 64, GRAZE_A: 1, GRAZE_B: 2, OVER: 4, FAR: 8, WALLED: 16, SEALED: 0, ROUTED: 1, ROUTED_DOOR: 3, Q_LENS: 1, Q_UNSEEN: 2, Q_SMOKED: 4, A_VISIBLE: 1, A_IN_CONE: 2 } as const;
const kindOf = (b: Box, cutOn: boolean): number => (b.rot ? K_ROT : 0) | (b.round === true ? K_ROUND : 0) | (b.cut !== undefined && (cutOn || shapeCut(b)) && b.cut.length > 0 ? K_CUT : 0);   // (boxes.ts cutMet: a shape's record whatever the cut-pieces switch says; a piece of trim's only with that switch, as the picture)
/** a walker's rule for a pane (collision.ts glassWalkable), or none */
type Walk = ((bi: number) => boolean) | null;
/** …a static's kind: with the pane rule's answer for it as it stands (the leaves are nobody's panes) */
const kindAt = (b: Box, i: number, cutOn: boolean, walk: Walk): number => walk !== null && (b.flags & F_GLASS) !== 0 && walk(i) ? kindOf(b, cutOn) | K_WALK : kindOf(b, cutOn);
/** is row `o` (flags at `fi`) box b as it stands — every double by its bits (a −0 is not a 0, a NaN is itself) */
function same(m: Float64Array, fl: Uint32Array, o: number, fi: number, b: Box, kind: number, cut0: number): boolean {
  const c = b.c, h = b.h;
  if (!Object.is(m[o], c[0]) || !Object.is(m[o + 1], c[1]) || !Object.is(m[o + 2], c[2]) || !Object.is(m[o + 3], h[0]) || !Object.is(m[o + 4], h[1]) || !Object.is(m[o + 5], h[2])
    || !Object.is(m[o + R_YAW], b.yaw) || fl[fi] !== ((b.flags & MIRRORED) >>> 0) || m[o + R_KIND] !== kind) return false;
  if (kind === 0) return true;
  if (kind & K_ROT) { const q = b.rot!; if (!Object.is(m[o + R_ROT], q[0]) || !Object.is(m[o + R_ROT + 1], q[1]) || !Object.is(m[o + R_ROT + 2], q[2]) || !Object.is(m[o + R_ROT + 3], q[3])) return false; }
  if (kind & K_CUT) {
    const cut = b.cut!, n = cut.length < MAX_CUT_PLANES ? cut.length : MAX_CUT_PLANES; if (m[o + R_CUTN] !== n) return false;
    let p = cut0 + m[o + R_CUT] * CUT_ROW;
    for (let k = 0; k < n; k++, p += PLANE) { const pl = cut[k], v = pl.n; if (!Object.is(m[p], v[0]) || !Object.is(m[p + 1], v[1]) || !Object.is(m[p + 2], v[2]) || !Object.is(m[p + 3], pl.d)) return false; }
  }
  return true;
}
/** write box b into row `o`; its cut record, if it has one, into record `cutRow`. The cos and sin of its yaw are worked out HERE, by the maths in Math's place —
 *  the two numbers the TypeScript's own walk would work out — so the module asks for neither. */
function put(m: Float64Array, fl: Uint32Array, o: number, fi: number, b: Box, kind: number, cut0: number, cutRow: number): void {
  const c = b.c, h = b.h, yaw = b.yaw;
  m[o] = c[0]; m[o + 1] = c[1]; m[o + 2] = c[2]; m[o + 3] = h[0]; m[o + 4] = h[1]; m[o + 5] = h[2];
  m[o + R_COS] = Math.cos(yaw); m[o + R_SIN] = Math.sin(yaw); m[o + R_YAW] = yaw; m[o + R_KIND] = kind; fl[fi] = (b.flags & MIRRORED) >>> 0;
  if (kind & K_ROT) { const q = b.rot!; m[o + R_ROT] = q[0]; m[o + R_ROT + 1] = q[1]; m[o + R_ROT + 2] = q[2]; m[o + R_ROT + 3] = q[3]; }
  if (kind & K_CUT) {
    const cut = b.cut!, n = cut.length < MAX_CUT_PLANES ? cut.length : MAX_CUT_PLANES; m[o + R_CUT] = cutRow; m[o + R_CUTN] = n;
    let p = cut0 + cutRow * CUT_ROW;
    for (let k = 0; k < n; k++, p += PLANE) { const pl = cut[k], v = pl.n; m[p] = v[0]; m[p + 1] = v[1]; m[p + 2] = v[2]; m[p + 3] = pl.d; }
  }
}

export class BoxMirror extends HostOwned {
  /** the module, from the first tick this game opens with a row on (a game is made on runtimes that cannot run it too) */
  x!: SimExports;
  /** which of the module's worlds is this game's (−1 = none now: another game took it) */
  slot = -1;
  /** THE COPY STANDS: inside a tick or a window, proven as it opened. Only then may a question go to the module — and a sense's question does whenever it
   *  stands (collision.ts THE SENSES: nothing else answers one), with what a question owes paid first (`owes`). */
  live = false;
  /** …and the line tests are to be asked there (the copy stands, RUST.lines was on as the tick opened, and no static has been written since it was compared) */
  lines = false;
  /** …and a ray into the boxes (the same, by RUST.rays: a row of its own, so that the lines' row can be on without it) */
  rays = false;
  /** …and a circle against the boxes (the same, by RUST.circles) */
  circles = false;
  /** THE NEXT QUESTION OWES SOMETHING before the module may answer it (`again` pays): the statics compared (one was written), the panes' rows written (the
   *  glass was worked on, or is being), the view's rows written. While the glass is being worked on it stays true after it is paid. */
  owes = false;
  /** THE GAME'S GLASS while the copy stands (collision.ts glassRule: game/glassbreak.ts hangs itself there; handed in at `open` as the walk is, and let go
   *  at `shut`), or none: no row stands for a pane */
  glass: PaneSource | null = null;
  /** the level's sheets the copy holds (written with the statics, and again if the game's list is a new one) */
  private sheetsIn: readonly number[] | null = null;
  /** the game's block the copy's lines and rays read a pane in */
  private block: SimState | null = null;
  private owed = 0; private glassBusy = 0; private glassAngles = false; private viewLengths = false; private glassD = 0; private glassW = 0; private glassOf = 0; private viewD = 0; private viewBi = 0; private glassN = 0; private readonly glassBis: number[] = [];
  private rowLines = false; private rowRays = false; private rowCircles = false; private walk: Walk = null;
  /** where this game's eyes of a sight pass are written (an index into the doubles) */
  eyes = 0;
  /** where a pushed circle ends (an index into the doubles), and the statics a gather listed (into the words) with how many it can list */
  pt = 0; list = 0; listCap = 0;
  /** the module's scratch (an index into the doubles), and the tables a sense's question is written to: the lamps and the blobs (into the doubles, and how
   *  many each holds), the nav grid's shut cells (into the bytes, and how many cells of floor a search holds: nav.rs MAX_FLOOR) and a way's waypoints (into the doubles). One set a process: each is scratch
   *  of the one question that wrote it. */
  scratch = 0; scratchLen = 0; lamps = 0; lampCap = 0; blobs = 0; blobCap = 0; wind = 0; windCap = 0; navShut = 0; navCap = 0; way = 0; navWant = 0; navPrice = 0;
  /** one question's lamps and blobs as they are written (collision.ts lampsBegin … lampsOn, blobsBegin … blobsAlong): the leaves' list to ask with, how many so far */
  lampSet = -2; lampN = 0; blobSet = -2; blobN = 0;
  /** WHAT A SMOKE GRID THAT STEPS OVER THIS COPY GOES BY (rust/sim/src/bricks.rs; game/sense.ts SmokeGrid): the copy's NUMBER, given anew whenever every
   *  row of it is written afresh (the world newly this game's, another maths, a snapshot put back), so a grid whose faces were asked of an earlier copy
   *  asks them all again; and whether a grid has stepped over this one (`gridCopy`), from when on a row that is written is said to it (`grid_dirty`). */
  private copy = 0; private gridOver = false;
  private cellsWrote = true;
  /** the grid of the tick that is open, for the compare a question owes after the grid was written inside it (`again`): the box world's own arrays, held for the
   *  length of the tick and let go as it shuts */
  private gridOf: { cells: number[][]; tiltedAt: number[]; W: number; H: number; cs: number } | null = null;
  private used = 0;
  private readonly caps = { boxes: 0, leaves: 0, cuts: 0, panes: 0, cells: 0, items: 0, tilted: 0, things: 0, glass: 0, view: 0 };
  /** how much each table of the copy holds, by name (rows; the module is asked here if nobody has yet, and every number is 0 where there is no module: no
   *  copy, and nothing to hold a level to): what a level is read against — game/levelfit.ts fitRows, tools/qa/copyfit-check.ts */
  get tables(): Readonly<Record<string, number>> { if (!this.x && !simWhyNot()) this.boot(); return this.caps; }
  // the furniture's copy: where its two tables are, the count of writes it was copied at (−1 = not in this tick), whether it stood then, and whether
  // every row is owed (the world newly this game's, another maths, a snapshot put back: a row's cos and sin are compared by nobody)
  private thing = 0; private thingLive = 0; private thingsAt = -1; private thingsOf: Furniture | null = null; private thingsOk = false; private thingsAll = true;
  // the level's snow: where its table is (a byte a static), and how many of its bytes this copy has written (−1 = not known: what stands behind them may be another game's)
  private snowAt0 = 0; private snowLen = -1;
  // where the tables are: an index into the doubles (f64) or the words (u32 / i32) of the module's memory
  private rec = 0; private leaf = 0; private cut = 0; private paneT = 0; private flags = 0; private leafFlags = 0; private start = 0; private items = 0; private tilted = 0; private paneBi = 0;
  private boxes = 0; private nTilted = 0; private epoch = -1; private putBacks = -1;
  // a static's cut record (−1 = none), the records given back, and the next never used
  private cutOf!: Int32Array; private readonly cutFree: number[] = []; private cutNext = 0;
  // the game's own list of leaves (the array that stood in collision.ts dynamicBoxes as the tick opened), how long it was when copied, and the count of leaf
  // writes it was copied at (−1 = not in this tick)
  private own: Box[] | null = null; private ownLen = 0; private ownAt = -1;

  private boot(): void {
    const x = this.x = sim(), c = this.caps;
    c.boxes = x.world_cap(T_REC); c.leaves = x.world_cap(T_LEAF); c.cuts = x.world_cap(T_CUT); c.panes = x.world_cap(T_PANE_T); c.cells = x.world_cap(T_START); c.items = x.world_cap(T_ITEMS);
    c.tilted = x.world_cap(T_TILTED); this.listCap = x.world_cap(T_LIST); c.things = x.world_cap(T_THING); c.glass = x.world_cap(T_GLASS); c.view = x.world_cap(T_VIEW);
    if (c.glass !== PANE_CAP || c.view !== PANE_CAP) throw new Error(`the Rust module holds ${c.glass} broken panes and a view of ${c.view}: src/scene/boxMirror.ts says ${PANE_CAP} of each`);
    if (c.leaves !== PROP_PARTS || c.things !== PROP_CAP) throw new Error(`the Rust module's copy lists ${c.leaves} leaves and ${c.things} things of furniture: src/sim/propshome.ts says ${PROP_PARTS} boxes and ${PROP_CAP} things`);
    this.scratch = x.sim_scratch() / 8; this.scratchLen = x.sim_scratch_len(); this.lamps = x.light_lamps() / 8; this.lampCap = x.light_cap(); this.blobs = x.smoke_blobs() / 8; this.blobCap = x.smoke_cap(); this.wind = x.grid_wind() / 8; this.windCap = x.grid_wind_cap();
    this.navShut = x.nav_addr(0); this.navCap = x.nav_cap(0); this.way = x.nav_addr(1) / 8; this.navWant = x.nav_addr(2) / 4; this.navPrice = x.nav_addr(3) / 8;
    this.cutOf = new Int32Array(c.boxes).fill(-1);
    if (!HELD.length) for (let i = x.world_cap(T_WORLDS); i > 0; i--) HELD.push(null);
  }

  /** this game's world in the module: its own still, or a free one, or the one stepped longest ago (whose game copies afresh when it steps again). True = the
   *  world is newly this game's and nothing in it is ours. */
  private take(): boolean {
    if (!this.x) this.boot();
    this.used = ++CLOCK.n;
    if (this.slot >= 0 && HELD[this.slot] === this) return false;
    let s = 0; for (let i = 0; i < HELD.length; i++) { const h = HELD[i]; if (!h) { s = i; break; } if (h.used < HELD[s]!.used) s = i; }
    const old = HELD[s]; if (old) { old.slot = -1; old.shut(); }
    HELD[s] = this; this.slot = s;
    const x = this.x, at = (t: number, by: number) => x.world_addr(s, t) / by;
    this.rec = at(T_REC, 8); this.leaf = at(T_LEAF, 8); this.cut = at(T_CUT, 8); this.paneT = at(T_PANE_T, 8); this.flags = at(T_FLAGS, 4); this.leafFlags = at(T_LEAF_FLAGS, 4);
    this.start = at(T_START, 4); this.items = at(T_ITEMS, 4); this.tilted = at(T_TILTED, 4); this.paneBi = at(T_PANE_BI, 4); this.pt = at(T_PT, 8); this.list = at(T_LIST, 4); this.eyes = x.fog_eyes(s) / 8;
    this.thing = at(T_THING, 8); this.thingLive = at(T_THING_LIVE, 4); this.snowAt0 = at(T_SNOW, 1);
    this.glassD = at(T_GLASS, 8); this.glassW = at(T_GLASS_W, 4); this.glassOf = at(T_GLASS_OF, 4); this.viewD = at(T_VIEW, 8); this.viewBi = at(T_VIEW_BI, 4);
    return true;
  }

  /** A TICK OPENS: the copy is made the boxes as they stand (see the head). → `live`. */
  open(boxes: Box[], cells: number[][], tiltedAt: number[], W: number, H: number, cs: number, leaves: Box[], walk: Walk, glass: PaneSource | null, lines: boolean, circles: boolean, rays: boolean): void {
    this.walk = walk; this.glass = glass;
    const fresh = this.take(), all = fresh || this.epoch !== mathsEpoch || this.putBacks !== PUT_BACKS.n;   // (another maths in Math's place: every cos and sin kept here was the other's)
    this.epoch = mathsEpoch; this.putBacks = PUT_BACKS.n;
    if (all) { this.sheetsIn = null; this.cutOf.fill(-1); this.cutFree.length = 0; this.cutNext = 0; this.boxes = 0; this.gridDrop(); this.thingsAll = true; this.snowLen = -1;   // (…and it is another copy to a smoke grid: every face is asked again)
      u32().fill(0, this.glassOf, this.glassOf + this.caps.boxes); this.glassN = 0; this.glassBis.length = 0; }   // (whose broken panes these rows were is not known: none stands)
    if (all || this.cellsWrote) { this.cellsWrote = false; this.grid(cells, tiltedAt, W, H, boxes.length); }
    this.statics(boxes, all);
    this.x.world_shape(this.slot, W, H, cs, boxes.length, this.nTilted);
    this.gridOf = { cells, tiltedAt, W, H, cs };
    this.panes();
    this.ownAt = -1; this.thingsAt = -1; this.own = leaves; this.owes = false; this.owed = 0; this.rowLines = lines; this.rowRays = rays; this.rowCircles = circles;
    this.live = true; this.lines = lines; this.rays = rays; this.circles = circles;
  }
  /** the tick is over: from here the TypeScript answers */
  shut(): void { if (this.block) this.block.moved = null; this.block = null; if (this.live && this.slot >= 0) this.x.panes_world(this.slot, -1); this.live = this.lines = this.rays = this.circles = this.owes = false; this.owed = 0; this.own = null; this.walk = null; this.glass = null; this.gridOf = null; }   // (and nothing of the game is held from here: a world outlives its game)

  /** a tick has opened on this box world (StaticCollision.tickOpen says so): from then on it is a level in play, and what `outgrown` has to say is another thing */
  played = false;
  /** THE COPY HAS A TABLE TOO SMALL FOR THE BOX WORLD AS IT STANDS, and there is no other answer to a question of this world. Before a tick has opened on
   *  it, that is A LEVEL TOO BIG TO LOAD, and is said so — a game's first questions come as it is made (the paths' bake), before the whole count is read
   *  (game/levelfit.ts stoodOver, the last statement of Game's constructor), and a box world stood up with no game has no other refusal. After one, it is no
   *  case of play: the one rule that adds statics holds to the room the level left (game/wallbreak.ts WallBreak.room), so it is a mistake there or in
   *  somebody who grew the box world behind it, and is said by name. */
  private outgrown(what: string, has: number, cap: number): never {
    throw new Error(this.played
      ? `the box world${this.of ? ` of the level '${this.of}'` : ''} has outgrown the Rust module's copy of it in play: ${has} ${what}, and the copy holds ${cap} (game/wallbreak.ts refuses a piece over the room the level left: something else grew it)`
      : tooBig(this.of, `${has} ${what} (${cap}: what the Rust module's copy of the box world holds — scene/boxMirror.ts, game/levelfit.ts)`));
  }
  /** the name of the level this box world is, where whoever stood it up knew one (a game does: StaticCollision's second argument; a box world stood up bare has none) */
  of: string | null = null;
  /** the grid: cell c's statics in the order they were filed, one run after another; and the tilted statics' own list */
  private grid(cells: number[][], tiltedAt: number[], W: number, H: number, boxes: number): void {
    const n = W * H, c = this.caps; if (n > c.cells) this.outgrown('cells of the floor grid', n, c.cells); if (tiltedAt.length > c.tilted * 5) this.outgrown('tilted statics', tiltedAt.length / 5, c.tilted);
    if (boxes > c.boxes) this.outgrown('statics', boxes, c.boxes);
    let total = 0; for (let i = 0; i < n; i++) total += cells[i].length; if (total > c.items) this.outgrown('entries of the grid\'s cells', total, c.items);
    const w = u32(), st = this.start, it = this.items; let at = 0;
    for (let i = 0; i < n; i++) { const cell = cells[i]; w[st + i] = at; for (let k = 0; k < cell.length; k++) w[it + at++] = cell[k]; }
    w[st + n] = at;
    const t = i32(), t0 = this.tilted; for (let i = 0; i < tiltedAt.length; i++) t[t0 + i] = tiltedAt[i];
    this.nTilted = tiltedAt.length / 5;
  }
  /** every static against its row; a row that differs (or every row, `all`) written again */
  private statics(boxes: Box[], all: boolean): void {
    const n = boxes.length, m = f64(), fl = u32(), cutOn = cutConfig.on, f0 = this.flags, cut0 = this.cut, cutOf = this.cutOf, walk = this.walk;
    // (the smoke grid keeps which faces between its cells are open: where a row is written, or goes, the faces round where it stood and where it stands
    //  are asked again — nothing to say when every row is written, for then the grid's faces are all unknown already)
    const grid = !all && this.gridOver;
    for (let i = n; i < this.boxes; i++) { if (cutOf[i] >= 0) { this.cutFree.push(cutOf[i]); cutOf[i] = -1; } if (grid) this.gridDirty(m, this.rec + i * REC); }   // (the list was cut back: its records are free again)
    const was = this.boxes; this.boxes = n;
    for (let i = 0, o = this.rec; i < n; i++, o += REC) {
      const b = boxes[i], kind = kindAt(b, i, cutOn, walk);
      if (!all && i < was && same(m, fl, o, f0 + i, b, kind, cut0)) continue;   // (a row past the list as it was is no row of this list's, whatever was left written there: a wall taken off the end and put back is written, and said to the grid)
      if (grid) { this.gridDirty(m, o); const c = b.c, h = b.h, e = h[0] + h[1] + h[2]; this.x.grid_dirty(this.slot, c[0] - e, c[2] - e, c[0] + e, c[2] + e); }
      let row = cutOf[i];
      if (kind & K_CUT) { if (row < 0) { row = this.cutFree.length ? this.cutFree.pop()! : this.cutNext < this.caps.cuts ? this.cutNext++ : -1; if (row < 0) this.outgrown('cut and shaped statics', this.caps.cuts + 1, this.caps.cuts); cutOf[i] = row; } }
      else if (row >= 0) { this.cutFree.push(row); cutOf[i] = -1; }
      put(m, fl, o, f0 + i, b, kind, cut0, row);
    }
  }
  /** the plan's rectangle round row `o` as the module has it (its three half sides together: its reach in plan whichever way it is turned) */
  private gridDirty(m: Float64Array, o: number): void { const e = m[o + 3] + m[o + 4] + m[o + 5]; this.x.grid_dirty(this.slot, m[o] - e, m[o + 2] - e, m[o] + e, m[o + 2] + e); }
  /** THE COPY'S NUMBER, for a step of a smoke grid over it (collision.ts gridStep): from here on a row that is written is said to the grid */
  gridCopy(): number { this.gridOver = true; return this.copy; }
  /** this is another copy to a smoke grid: one that steps over it next knows no face and remembers no leaf (and nothing is said to a grid until one does) */
  gridDrop(): void { this.copy = ++COPIES.n; this.gridOver = false; }

  /** A STATIC WAS WRITTEN INSIDE THE TICK (its shape, its flags, its cut; a pane a walker now goes through): the module is asked nothing more until the statics
   *  have been compared again — which the next question does (`again`), so a blast that writes forty boxes pays for one compare, and one nobody asks after
   *  pays for none. The grid is not compared: only addStatic and truncateStatics change it, and they say so themselves. */
  wrote(): void { if (this.live) this.debt(OWE_BOXES); }
  private debt(what: number): void { this.owed |= what; if (!this.owes) { this.owes = true; this.lines = this.rays = this.circles = false; } }
  /** …or THE GLASS IS BEING WORKED ON (game/glassbreak.ts: a blow, a blast, a step of what is coming down — a piece out, a piece queued to leave). The
   *  rule asks lines and rays of its own between its writes, so from `glassBegin` to `glassEnd` EVERY question owes the panes' rows written again, and
   *  so does the first one after; no static is compared for it. Nobody has to say which write changed what. */
  glassBegin(): void { this.glassBusy++; if (this.live) this.debt(OWE_GLASS); }
  glassEnd(): void { this.glassBusy--; if (this.live) this.debt(OWE_GLASS); }
  /** …the glass was written with no question of its own between (a pane made another glass, every pane made whole): its rows are owed */
  glassWrote(): void { if (this.live) this.debt(OWE_GLASS); }
  /** …and the glass itself was hung, or taken down, under a standing copy (collision.ts glassRule): its rows are owed like any other write of it */
  glassIs(G: PaneSource | null): void { if (!this.live) return; this.glass = G; this.debt(OWE_GLASS); }
  /** a static's write is still owed (the furniture's check stands aside for it, as it did for every debt before the glass had its own) */
  get owesBoxes(): boolean { return (this.owed & OWE_BOXES) !== 0; }
  /** THE LEVEL'S SNOW (game/snowBog.ts: what snow each box of the level is, a byte a box; rust/sim/src/world.rs `snow`), for the rule that asks where a
   *  thrown thing bogs down (items.rs bog_at). Compared with the table at each walk that may meet snow and written where it differs, as the statics are
   *  (a few hundred bytes); a static added since (a wall's pieces) is no snow. Where each box is and how it is turned are the statics' own rows, which
   *  every tick proves. */
  snow(kinds: Uint8Array): void {
    const b = u8(), o = this.snowAt0, n = Math.min(kinds.length, this.caps.boxes);
    for (let i = 0; i < n; i++) if (b[o + i] !== kinds[i]) b[o + i] = kinds[i];
    if (this.snowLen !== n) { b.fill(0, o + n, o + this.caps.boxes); this.snowLen = n; }
  }
  /** …the compare a question owes after `wrote`: the copy stands again, and the rows are back as the tick opened with them */
  again(boxes: Box[], walk: Walk): void {
    const owed = this.owed | (this.glassBusy > 0 ? OWE_GLASS : 0); this.owes = false; this.owed = 0; if (!this.live) return;
    this.walk = walk;
    if (owed & OWE_BOXES) {
      const G = this.gridOf!;
      if (this.cellsWrote) { this.cellsWrote = false; this.grid(G.cells, G.tiltedAt, G.W, G.H, boxes.length); }   // (…the grid too, where it was the grid: what a tick's open does)
      this.statics(boxes, false);
      this.x.world_shape(this.slot, G.W, G.H, G.cs, boxes.length, this.nTilted);
    }
    if (owed & OWE_GLASS) this.panes(); else if (owed & OWE_VIEW) this.viewIn();
    if (this.glassBusy > 0) this.owes = true;   // (the glass is still being worked on: the next question writes its rows again)
    this.lines = this.rowLines; this.rays = this.rowRays; this.circles = this.rowCircles;
  }
  /** …or the GRID was (a static added, the list cut back): the same debt, and the compare that pays it copies the cells again first. (Until the count of
   *  questions the TypeScript took — sim.ts FELL — this shut the copy for what was left of the tick: a wall holed was a tick whose every later question,
   *  the senses' and the rules' steps among them, the TypeScript answered.) Outside a tick nothing is owed: the next open copies the cells. */
  wroteGrid(): void { this.cellsWrote = true; this.wrote(); }

  /** THE PANES THAT ARE NO LONGER WHOLE, written whole: a row each in the order the game hands them over, which row is whose static's, how many. Every row
   *  is written every time (a row is a few hundred numbers and a level's broken panes are few): nobody has to say WHICH pane changed, and a put-back or a
   *  pane made whole again is no case. More panes than the table holds cannot be (a row a sheet of glass, and a level of more sheets is not loaded:
   *  game/levelfit.ts levelOver), and is said by name (`outgrown`). */
  private panes(): void {
    const w = u32(), of = this.glassOf, was = this.glassBis, G = this.glass;
    for (let r = 0; r < this.glassN; r++) w[of + was[r]] = 0;
    const home = G !== null ? G.home ?? null : null;
    if (home !== null) { this.glassN = 0; was.length = 0; this.glassAngles = true; this.x.world_glass(this.slot, 0); this.x.panes_world(this.slot, home.bind()); (this.block = home).moved = s => { if (this.live) this.x.panes_world(this.slot, s); };
      const sh = G!.sheets ?? NO_SHEETS; if (sh !== this.sheetsIn) { const m = f64(), at = this.x.sim_scratch() / 8; for (let i = 0; i < sh.length; i++) m[at + i] = sh[i]; this.x.world_sheets(this.slot, sh.length / 3); this.sheetsIn = sh; }
      this.viewIn(); return; }   // (the rows stand in the game's block: the rule's own)
    this.x.panes_world(this.slot, -1); if (this.block) this.block.moved = null; this.block = null;
    let n = 0;
    if (G !== null && G.each) { const m = f64(), cap = this.caps.glass; G.each(p => { if (n < cap) this.pane(m, w, this.glassD + n * GD, this.glassW + n * GW, p); n++; }); }
    if (n > this.caps.glass) this.outgrown('panes of glass no longer whole', n, this.caps.glass);
    this.glassAngles = false; for (let r = 0, o = this.glassW + GW_FLAGS; r < n; r++, o += GW) if (!(w[o] & G_GONE)) { this.glassAngles = true; break; }   // (a pane that is gone is open everywhere: no piece to find)
    was.length = n; for (let r = 0, o = this.glassW; r < n; r++, o += GW) w[of + (was[r] = w[o])] = r + 1;
    this.glassN = n; this.x.world_glass(this.slot, n);
    this.viewIn();
  }
  /** one pane's row, its doubles from `d` and its words from `o` (glass.rs names the places) */
  private pane(m: Float64Array, w: Uint32Array, d: number, o: number, p: PaneRow): void {
    const S = p.sites, ns = S.length < G_SITES ? S.length : G_SITES, D = p.dice;
    w[o] = p.bi; w[o + GW_FLAGS] = (p.gone ? G_GONE : 0) | (D !== null ? G_DICE : 0) | (p.run ? G_RUN : 0) | (ns << 8);
    for (let i = 0, q = d; i < ns; i++, q += G_SITE) { const s = S[i], rho = s.rho;
      m[q] = s.u; m[q + 1] = s.v; m[q + 2] = s.rot; m[q + 3] = rho[0]; m[q + 4] = rho[1]; m[q + 5] = rho[2]; m[q + 6] = rho[3]; m[q + 7] = s.rz; m[q + 8] = s.merge;
      w[o + GW_N + i] = s.n; w[o + GW_SEED + i] = s.seed; }
    if (D !== null) { const q = d + G_DICE_AT; w[o + GW_COLS] = D.cols; w[o + GW_ROWS] = D.rows; w[o + GW_DSEED] = D.seed;
      // (the last clump OUT in the order they go — glass.rs dice_out. `out` is what the rule answers by, and a clump is marked out before `next` counts it,
      //  while its fragment is handed on and whoever hears it asks lines of their own: so `out` is read, from `next` on)
      let last = D.next; while (last < D.clumps.length && D.out[D.clumps[last].k] === 1) last++;
      m[q] = D.u; m[q + 1] = D.v; m[q + 2] = D.hu; m[q + 3] = D.hv; m[q + 4] = last > 0 ? D.clumps[last - 1].key : -Infinity; m[q + 5] = last > 0 ? D.clumps[last - 1].k : -1; }
    // a bit a piece that is out (only the ids of the sites there are can be set), and a bit a piece queued to leave, with a bit a site whose crater is
    const open = p.open, bits = ns * FRAGS_PER_SITE;
    for (let k = 0, at = 0; k < GW_BITS; k++) { let x = 0; for (let j = 0; j < 32 && at < bits; j++, at++) if (open[at] === 1) x |= 1 << j; w[o + GW_OPEN + k] = x; w[o + GW_GOING + k] = 0; }
    let crater = 0;
    for (const q of p.pending) { const fid = q.fid; w[o + GW_GOING + (fid >>> 5)] |= 1 << (fid & 31); if (fid % FRAGS_PER_SITE < SECTORS) crater |= 1 << ((fid / FRAGS_PER_SITE) | 0); }
    w[o + GW_CRATER] = crater;
  }
  /** A BURST'S VIEW WAS PUT ON OR TAKEN OFF (game/glassbreak.ts seeThrough): the next question owes its rows written, and none is read before one is
   *  asked (a judgment whose lines never reach the module costs the game no rows). A view names a level's sheets of glass at most, which the table
   *  holds (levelfit.ts): one of more is said by name (`outgrown`). */
  viewed(): void { if (this.live) this.debt(OWE_VIEW); }
  private viewIn(): void {
    const V = this.glass !== null ? this.glass.view : null; this.viewLengths = false;
    if (V === null) { this.x.world_view(this.slot, VIEW_NONE, 0); return; }
    if (this.glass!.home) { const a = V.at; this.x.panes_view(this.slot, V.kind, a[0], a[1], a[2], a[3], a[4], a[5]); return; }
    const n = V.bi.length; if (n > this.caps.view) this.outgrown('panes of glass a burst\'s view names', n, this.caps.view);
    const w = u32(), m = f64(), bi = V.bi, at = V.at;
    for (let k = 0; k < n; k++) w[this.viewBi + k] = bi[k];
    for (let k = 0, e = n * VD; k < e; k++) { m[this.viewD + k] = at[k]; if (k % VD === VD - 1 && at[k] >= 0) this.viewLengths = true; }   // (a pane that gives near its nearest point: a length is worked out)
    this.x.world_view(this.slot, V.kind, n);
  }
  /** may a pane's open question be asked of the module now: the game's glass is in the copy, rows and view */
  get glassIn(): boolean { return this.glass !== null; }
  /** …and does no answer of it need an angle or a length worked out (every broken pane is gone, no view gives near a point): then it is the same
   *  answer under the engine's maths as under the game's own */
  get glassPlain(): boolean { return !this.glassAngles && !this.viewLengths; }
  /** THE CHECK'S QUESTION of the panes' copy (collision.ts BOTH): how many of its rows are not the game's glass now. 0, or a write of the glass went unsaid. */
  stalePanes(): number {
    const G = this.glass; if (G === null) return this.glassN;
    if (G.home || !G.each) return 0;   // (no copy: the row is the state)
    const w = u32(), m = f64(), keepW = w.slice(this.glassW, this.glassW + this.glassN * GW), keepD = m.slice(this.glassD, this.glassD + this.glassN * GD), wasN = this.glassN;
    let n = 0, bad = 0; const tw = new Uint32Array(GW), td = new Float64Array(GD);
    G.each(p => { const r = n++; if (r >= wasN) { bad++; return; }
      // (the row as it would be written now, on a row of this check's own: the copy is not touched)
      tw.fill(0); td.fill(0); this.pane(td, tw, 0, 0, p);
      const S = p.sites, ns = S.length < G_SITES ? S.length : G_SITES; let same = true;
      for (let k = 0; same && k < GW; k++) { if (k >= GW_N && k < GW_OPEN && (k - GW_N) % G_SITES >= ns) continue; if ((k === GW_COLS || k === GW_ROWS || k === GW_DSEED) && p.dice === null) continue; same = keepW[r * GW + k] === tw[k]; }
      for (let i = 0; same && i < ns * G_SITE; i++) same = Object.is(keepD[r * GD + i], td[i]);
      if (p.dice !== null) for (let i = G_DICE_AT; same && i < G_DICE_AT + 6; i++) same = Object.is(keepD[r * GD + i], td[i]);
      if (!same) bad++; });
    return bad + (n < wasN ? wasN - n : 0);
  }

  /** THE LEAVES A QUESTION IS ASKED WITH (collision.ts dynamicBoxes): which of the module's lists holds them. The game's own list — the array that stood there
   *  as the tick opened — is list 0, copied at the tick's first question with it and again after a leaf is written (`leafWrote`). Any other array is a list a question
   *  swapped in for itself (a door's own leaf taken out, a ghost leaf put in); such arrays are scratch that is filled anew between questions, so it is copied
   *  into list 1 at EVERY question it is asked with — a few leaves. −1 = no leaves (nothing is copied); −2 = more than a list holds (the TypeScript answers). */
  leaves(dyn: Box[]): number {
    const n = dyn.length; if (n === 0) return -1;
    const own = dyn === this.own; if (own && this.ownAt === LEAVES.n && this.ownLen === n) return 0;
    if (n > this.caps.leaves) return -2;
    const s = own ? 0 : 1, m = f64(), fl = u32(), cutOn = cutConfig.on, cut0 = this.cut, base = s * this.caps.leaves;
    for (let i = 0, o = this.leaf + base * REC; i < n; i++, o += REC) { const b = dyn[i]; put(m, fl, o, this.leafFlags + base + i, b, kindOf(b, cutOn), cut0, this.caps.cuts + base + i); }
    this.x.world_leaves(this.slot, s, n);
    if (own) { this.ownAt = LEAVES.n; this.ownLen = n; }
    return s;
  }

  /** THE FURNITURE'S PARTS, FOR A QUESTION THAT MEETS THEM (the furniture's ray: lines.rs ray_things; the thrown things' walk: items.rs): every part box
   *  of every thing in the level is list THING_SET of the leaves, a thing's parts together and the things in the furniture's own order, with where each
   *  thing stands by its id beside them. Copied as the leaves are — at the tick's first question that asks for them, and again after a thing is written
   *  (`thingWrote`) — but row by row: a part is compared with its row and written only where it differs, so a chair shoved across a room costs its own
   *  five boxes a tick, not the level's two hundred. → does the copy stand (false: no tick, or more furniture than the tables hold — the TypeScript answers) */
  things(P: Furniture): boolean {
    if (!this.live) return false;
    if (this.thingsAt === THINGS.n && this.thingsOf === P) return this.thingsOk;   // (…of THIS furniture: a check stands a second set up over one box world)
    this.thingsAt = THINGS.n; this.thingsOf = P; this.thingsOk = false;
    const all = P.all, L = P.props, c = this.caps; if (all.length > c.things) return false;
    let parts = 0; for (let r = 0; r < L.length; r++) parts += L[r].boxes.length; if (parts > c.leaves) return false;
    const m = f64(), w = u32(), cutOn = cutConfig.on, cut0 = this.cut, base = THING_SET * c.leaves, every = this.thingsAll;
    for (let i = 0, o = this.thing; i < all.length; i++, o += TH) { const p = all[i]; m[o] = p.x; m[o + 1] = p.z; m[o + 2] = p.yaw; m[o + 3] = p.radius; }
    let at = 0;
    for (let r = 0, o = this.thingLive; r < L.length; r++, o += TL) {
      const p = L[r], B = p.boxes; w[o] = p.id; w[o + 1] = at; w[o + 2] = B.length;
      for (let j = 0; j < B.length; j++, at++) {
        const b = B[j], kind = kindOf(b, cutOn), row = this.leaf + (base + at) * REC, fi = this.leafFlags + base + at;
        if (every || !same(m, w, row, fi, b, kind, cut0)) put(m, w, row, fi, b, kind, cut0, c.cuts + base + at);
      }
    }
    this.x.world_leaves(this.slot, THING_SET, parts); this.x.world_things(this.slot, L.length);
    this.thingsAll = false; return this.thingsOk = true;
  }
  /** part `at` of the copied list → its box (what the furniture's ray names) */
  thingBox(P: Furniture, at: number): Box | null { for (const p of P.props) { if (at < p.boxes.length) return p.boxes[at]; at -= p.boxes.length; } return null; }
  /** THE CHECK'S QUESTION of the furniture's copy (collision.ts thingsBoth): how many of its rows are not the furniture now. 0, or a write went unsaid. */
  staleThings(P: Furniture): number {
    const m = f64(), w = u32(), cutOn = cutConfig.on, base = THING_SET * this.caps.leaves; let bad = 0, at = 0;
    for (let i = 0, o = this.thing; i < P.all.length; i++, o += TH) { const p = P.all[i]; if (!Object.is(m[o], p.x) || !Object.is(m[o + 1], p.z) || !Object.is(m[o + 2], p.yaw) || !Object.is(m[o + 3], p.radius)) bad++; }
    for (let r = 0, o = this.thingLive; r < P.props.length; r++, o += TL) {
      const p = P.props[r]; if (w[o] !== p.id || w[o + 1] !== at || w[o + 2] !== p.boxes.length) bad++;
      for (let j = 0; j < p.boxes.length; j++, at++) if (!same(m, w, this.leaf + (base + at) * REC, this.leafFlags + base + at, p.boxes[j], kindOf(p.boxes[j], cutOn), this.cut)) bad++;
    }
    return bad;
  }

  /** where the glass the last line crossed is listed: which static (into the words) and how far along the line it is entered (into the doubles) */
  get glassBi(): number { return this.paneBi; }
  get glassT(): number { return this.paneT; }

  // ---- A RULE OF THE MODULE'S WRITES THE LEAVES (the doors' step, rust/sim/src/doors.rs place). While a rule that moves a box runs in the module, the ROW it
  // writes is the original and the Box object the picture draws FOLLOWS: the host writes the object from the row before any TypeScript runs again, so a box and
  // its row are one outside a call into the module whichever side wrote. (The other way round is as it was: a TypeScript rule writes the object and says so.)
  /** may a rule of the module's step over this world now, with the game's own leaves `dyn` in list 0 as they stand (copied here if a leaf was written since) */
  rules(dyn: Box[]): boolean {
    if (!this.live || (this.owes && (this.owed !== 0 || this.glassBusy === 0)) || dyn !== this.own) return false;   // (while the glass is worked on `owes` stands after everything owed is paid: collision.ts rulesWorld has just paid it)
    if (dyn.length !== 0) return this.leaves(dyn) === 0;
    // (a level with no leaf at all: list 0 is the game's own, and empty — said to the module once, as a list of leaves is copied once)
    if (this.ownAt !== LEAVES.n || this.ownLen !== 0) { this.x.world_leaves(this.slot, 0, 0); this.ownAt = LEAVES.n; this.ownLen = 0; }
    return true;
  }
  /** leaf i of list 0 as the module's rule left it → its box: the centre (a new array, as Door.place makes one), the yaw, the flags the copy keeps. → the yaw */
  followLeaf(i: number, b: Box): number {
    const m = f64(), o = this.leaf + i * REC, yaw = m[o + R_YAW];
    b.c = [m[o], m[o + 1], m[o + 2]]; b.yaw = yaw; b.flags ^= (b.flags & MIRRORED) ^ u32()[this.leafFlags + i];
    return yaw;
  }
  /** a leaf's row as leafRow read it → its box, the same writes as followLeaf (for a caller that had to read the rows BEFORE something else could ask the
   *  module and have them copied from the boxes again). → the yaw */
  followRow(b: Box, r: readonly number[]): number { b.c = [r[0], r[1], r[2]]; b.yaw = r[3]; b.flags ^= (b.flags & MIRRORED) ^ r[4]; return r[3]; }
  /** leaf i's row of list 0: its centre, its yaw, its flags */
  leafRow(i: number): number[] { const m = f64(), o = this.leaf + i * REC; return [m[o], m[o + 1], m[o + 2], m[o + R_YAW], u32()[this.leafFlags + i]]; }
  /** the game's own leaves are being placed box by box while what the rule answered is heard (game/doors.ts takeBack): a question asked meanwhile, and the
   *  first one after, copies list 0 from the boxes as they stand */
  leavesTurned(): void { this.ownAt = -1; }

  /** THE CHECK'S QUESTION (tools/qa/rust-check.ts, through collision.ts BOTH): how many rows of the copy are not their box now — the statics, and list `set`
   *  of the leaves. 0 inside a tick, or a write went unsaid. */
  stale(boxes: Box[], dyn: Box[], set: number): number {
    const m = f64(), fl = u32(), cutOn = cutConfig.on; let bad = boxes.length === this.boxes ? 0 : 1;
    for (let i = 0, o = this.rec; i < boxes.length && i < this.boxes; i++, o += REC) if (!same(m, fl, o, this.flags + i, boxes[i], kindAt(boxes[i], i, cutOn, this.walk), this.cut)) bad++;
    if (set >= 0) { const base = set * this.caps.leaves; for (let i = 0, o = this.leaf + base * REC; i < dyn.length; i++, o += REC) if (!same(m, fl, o, this.leafFlags + base + i, dyn[i], kindOf(dyn[i], cutOn), this.cut)) bad++; }
    return bad;
  }
}
