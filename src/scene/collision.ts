// CPU-side spatial helpers over the static boxes: circle push-out, line-of-sight, nav grid + A*.
import { Vec3, v3, quat, hyp2, hyp3 } from '../math/vec';
import { Box, BoxFlag, rayBox, cutMet } from './boxes';
import { cutClip, CUT_CLIP, MAX_CUT_PLANES } from './cut';
import { WORLD } from './world';
import { BoxMirror, MIRRORED, SENSES, FACE, NAV, senseRows, type Furniture, type PaneSource } from './boxMirror';
import { RUST, FELL, RULES_ROW, fell, callers, ruleRed, f64, u32, u8, simWhyNot } from '../sim/sim';
import type { SimState } from '../sim/state';
import { EXACT } from '../math/exact';

/** The parameter interval [t0, t1] over which the LINE a + t·rd (rd unit) lies inside box b — t0 may be negative, behind a — or null if the line misses it; and `depth`:
 *  how far INSIDE the box the origin a sits (its least distance to a face plane; ≤ 0 = on the surface or outside). This is the whole slab picture segmentBlocked judges
 *  a static by, where rayBox() only ever reports an ENTRY ahead of the origin and answers null from inside (boxes.ts: the rule the cursor's pick and every emitter
 *  embedded in its own fixture lean on) — which let a segment born ON a wall's face, or a hair inside it, run clean through that wall (W8.3). */
export function lineBox(a: Vec3, rd: Vec3, b: Box): { t0: number; t1: number; depth: number } | null {
  let ox: number, oy: number, oz: number, dx: number, dy: number, dz: number;
  if (b.rot) { const qi = quat.conj(b.rot); const o = quat.rotate(qi, v3.sub(a, b.c)); const d = quat.rotate(qi, rd); ox = o[0]; oy = o[1]; oz = o[2]; dx = d[0]; dy = d[1]; dz = d[2]; }
  else {
    const cs = Math.cos(b.yaw), sn = Math.sin(b.yaw); const px = a[0] - b.c[0], pz = a[2] - b.c[2];
    ox = cs * px - sn * pz; oy = a[1] - b.c[1]; oz = sn * px + cs * pz;                 // world → box local (Rᵀ), rayBox's own convention
    dx = cs * rd[0] - sn * rd[2]; dy = rd[1]; dz = sn * rd[0] + cs * rd[2];
  }
  const h = b.h; let t0 = -Infinity, t1 = Infinity;
  const o = [ox, oy, oz], d = [dx, dy, dz];
  for (let ax = 0; ax < 3; ax++) {
    if (Math.abs(d[ax]) < 1e-12) { if (Math.abs(o[ax]) > h[ax]) return null; continue; }   // parallel to this pair of faces: inside the slab for every t, or never
    let ta = (-h[ax] - o[ax]) / d[ax], tb = (h[ax] - o[ax]) / d[ax]; if (ta > tb) { const t = ta; ta = tb; tb = t; }
    if (ta > t0) t0 = ta; if (tb < t1) t1 = tb;
    if (t0 > t1) return null;
  }
  let depth = Math.min(h[0] - Math.abs(ox), h[1] - Math.abs(oy), h[2] - Math.abs(oz));
  if (b.round === true) {   // ROUND THINGS (scene/round.ts): the interval inside the CYLINDER about the box's own y (radius h[0]), and the origin's depth under its side or an end
    const qa = dx * dx + dz * dz, qb = ox * dx + oz * dz, qc = ox * ox + oz * oz - h[0] * h[0];
    if (qa < 1e-18) { if (qc > 0) return null; }
    else { const disc = qb * qb - qa * qc; if (disc < 0) return null; const s = Math.sqrt(disc), ta = (-qb - s) / qa, tb = (-qb + s) / qa; if (ta > t0) t0 = ta; if (tb < t1) t1 = tb; if (t0 > t1) return null; }
    depth = Math.min(h[0] - Math.sqrt(ox * ox + oz * oz), h[1] - Math.abs(oy));
  } else if (b.cut !== undefined && cutMet(b)) {   // CUT BOXES (scene/cut.ts): the interval inside the CUT solid, and the origin's depth under its nearest face — a cut plane included
    if (!cutClip(b.cut, ox, oy, oz, dx, dy, dz, t0, t1)) return null;
    t0 = CUT_CLIP.t0; t1 = CUT_CLIP.t1;
    for (let k = 0; k < b.cut.length && k < MAX_CUT_PLANES; k++) { const c = b.cut[k]; if (c.n[0] === 0 && c.n[1] === 0 && c.n[2] === 0) break; depth = Math.min(depth, c.d - (c.n[0] * ox + c.n[1] * oy + c.n[2] * oz)); }   // (the planes cutClip read: ≤ MAX_CUT_PLANES, ended by a zero normal)
  }
  return { t0, t1, depth };
}

/** How far past a box's own bounds the early rejects still count it as near (m). The bounds themselves are exact to a float's last bits (~1e-14 m at these sizes);
 *  what the pad has to cover is the slab test's own slack ALONG the line when the direction is within a hair of a box axis: a slab parameter is a difference good to
 *  ~3e-14 over a direction component the test accepts down to 1e-12, so an interval's end can sit up to ~0.03 m from the geometry's. */
const SEG_PAD = 0.05;
/** Can the segment from (a0, a1, a2) along the UNIT direction (r0, r1, r2) for `len` metres be said, without the slab test, to come nowhere near box b? True only
 *  when it provably stays clear of the box grown by `pad` — outside its height span (a yawed box's, exact), or outside the sphere round its centre that holds it
 *  whatever its yaw or tilt (|h0| + |h1| + |h2| ≥ its half diagonal). Both slab tests answer 'no' for such a box (rayBox: no entry within the length; lineBox: no
 *  interval reaching the segment, even one let run `shrink` past an end — the caller folds a negative shrink into the pad), so skipping them changes no answer.
 *  NaN anywhere makes every comparison false: not clear, the slab test decides as it always did. */
function segClear(a0: number, a1: number, a2: number, r0: number, r1: number, r2: number, len: number, b: Box, pad: number): boolean {
  const c = b.c, h = b.h; const hy = h[1] < 0 ? -h[1] : h[1];
  const m1 = c[1] - a1;
  if (!b.rot) { const e1 = r1 * len; if (e1 > 0 ? (m1 - hy > e1 + pad || m1 + hy < -pad) : (m1 - hy > pad || m1 + hy < e1 - pad)) return true; }
  const R = (h[0] < 0 ? -h[0] : h[0]) + hy + (h[2] < 0 ? -h[2] : h[2]) + pad;
  const m0 = c[0] - a0, m2 = c[2] - a2; const s = m0 * r0 + m1 * r1 + m2 * r2;
  if (s < -R || s > len + R) return true;
  return m0 * m0 + m1 * m1 + m2 * m2 - s * s > R * R;
}
/** rayBox's answer to 'does the ray enter this yawed box within tmax' (hit !== null — its t is > 0 whenever there is one): rayBox's own statements over the same
 *  scalars, stopping where it would start building the normal. For segmentBlockedDynamic, which only ever asked that. */
function rayBoxHits(a0: number, a1: number, a2: number, r0: number, r1: number, r2: number, b: Box, tmax: number): boolean {
  const cs = Math.cos(b.yaw), sn = Math.sin(b.yaw);
  const px = a0 - b.c[0], py = a1 - b.c[1], pz = a2 - b.c[2];
  const lx = cs * px - sn * pz, lz = sn * px + cs * pz, ly = py;
  const dx = cs * r0 - sn * r2, dz = sn * r0 + cs * r2, dy = r1;
  const ix = 1 / (dx || 1e-20), iy = 1 / (dy || 1e-20), iz = 1 / (dz || 1e-20);
  const h = b.h; let t0 = 0, t1 = tmax; let entered = false;
  { let ta = (-h[0] - lx) * ix, tb = (h[0] - lx) * ix; if (ta > tb) { const tmp = ta; ta = tb; tb = tmp; } if (ta > t0) { t0 = ta; entered = true; } if (tb < t1) t1 = tb; if (t0 > t1) return false; }
  { let ta = (-h[1] - ly) * iy, tb = (h[1] - ly) * iy; if (ta > tb) { const tmp = ta; ta = tb; tb = tmp; } if (ta > t0) { t0 = ta; entered = true; } if (tb < t1) t1 = tb; if (t0 > t1) return false; }
  { let ta = (-h[2] - lz) * iz, tb = (h[2] - lz) * iz; if (ta > tb) { const tmp = ta; ta = tb; tb = tmp; } if (ta > t0) { t0 = ta; entered = true; } if (tb < t1) t1 = tb; if (t0 > t1) return false; }
  return entered;
}

/** WHAT THE WALK GRID CALLS FLOOR, FEET CROSS (`on`: raised by the page — src/host/profile.ts pageSwitches, ?underfoot=0 — off here and on the frozen stand,
 *  TPS_UNDERFOOT=1). The grid's bake asks what stands between BAKE_Y (0.25 m) and a shoulder; a walking body was asked from FEET_Y (0.2 m), so a thing whose top
 *  lies in between was floor to every route and a wall to every foot: the bottom layer of a pile of sacks whose upper layers a blast took (0.2 m: what a wall
 *  charge left in its own hole — the men ordered through it walked on the spot at the wall's line), the low step of a bank of snow. On, a circle test that begins
 *  at FEET_Y or above and under BAKE_Y begins at BAKE_Y: men, the player and everybody else who walks are stopped by what the bake calls blocked, and by nothing less.
 *  Nothing is re-baked and nothing drawn changes; a test that begins lower (a body laid down, 0.05) is as it was. The two heights are not knobs: the bake
 *  below spells its own 0.25, and a band moved from under it would part the two again. (circleTouches takes the same band only so that it stays
 *  collideCircle's own test, as its comment promises: nobody asks it from 0.2 today.) */
export const UNDERFOOT = { on: false };
export const FEET_Y = 0.2, BAKE_Y = 0.25;
const footBand = (y0: number) => UNDERFOOT.on && y0 >= FEET_Y && y0 < BAKE_Y ? BAKE_Y : y0;

const shotPanes: { bi: number; t: number }[] = [];   // shotBlocked's scratch (the panes one line of fire crosses)
/** THE CHECK'S SWITCH OVER THE RUST ROWS OF THIS FILE (tools/qa/rust-check.ts raises it; nothing else does): on = every question a row sends to the module is
 *  ALSO answered here, the two compared, and every row of the module's copy compared with its box (boxMirror.ts stale) — `asked` questions, `differ` of them
 *  answered differently, `stale` rows that were not their box, and the first of either in words. The game plays on the TypeScript's answer while it is on. */
export const BOTH = { on: false, asked: 0, differ: 0, stale: 0, first: '', things: 0 };
/** A QUESTION ASKED OUTSIDE A TICK IS ASKED OF THE MODULE TOO (the window below): `on` = it is (off = outside a tick this file's own walk answers, as it
 *  did while a row could only be asked inside a tick); `proofs` = how many times a question outside every window had the copy proven for itself alone,
 *  which is what a caller that asks many should open a window to save. Never a rule of play: the same answers, by `rust-check --both`. */
export const ANYTIME = { on: true, proofs: 0, windows: 0, who: null as Record<string, number> | null };   // (`who`: a tool's table of who asked alone, when it wants one)
/** the same check's count of the senses' questions the module answered, a row each (RUST.sight … paths; boxMirror.ts SENSES): a row that is on and never asked proves nothing */
export const SENSED = { sight: 0, light: 0, smoke: 0, sound: 0, paths: 0 };
/** a blob of the smoke as a question carries it (sense.ts Blob's six; rust/sim/src/smoke.rs BLOB), and the model's numbers that go with them (SENSE.smoke's) */
export interface BlobRow { x: number; y0: number; z: number; r2: number; age: number; mass: number }
export interface SmokeNumbers { sigma: number; spread: number; widest: number; rise: number; riseRate: number }
/** …and of the times a light question's lamps did not fit the module's table and a part was sent on ahead (RUST.light: `lamp`) */
export const LAMPS = { parts: 0 };
/** …and of the rays the module answered (RUST.rays: raycast) */
export const RAYS = { n: 0, host: 0 };
/** …and of the searches with a penalty the module answered (RUST.paths: NavGrid.findPath; rust/sim/src/nav.rs find_priced): how many, how many times one
 *  came back for prices, and how many cells were priced for them. `waits` = one is waiting for prices NOW: the module has one set of tables, so a search
 *  asked from inside a price is an error, not a question */
export const PRICED = { n: 0, needs: 0, cells: 0, waits: false };
/** eyes one question of a sight pass can name (four bits each, and the count above them: rust/sim/src/fog.rs) */
const SIGHT_EYES = 7;
const bothPanes: { bi: number; t: number }[] = [];   // (the check's: the panes the module listed for the question in hand)
/** THE WIND THE COARSE GRID'S SMOKE DRIFTS BY (`gridStep`; game/wind.ts stormDrift): the vector (m/s), its rise with height (`low` of it at the ground,
 *  `gain` more by `fullAt` metres up) and the share of it each square metre of the plan has (0 under a roof … 1 in the open) */
export interface GridWind { x: number; z: number; low: number; gain: number; fullAt: number; share: (x: number, z: number) => number }
/** a sense's row by its bit (boxMirror.ts SENSES), for the count of questions the TypeScript took */
function senseName(bit: number): string { for (const [k, v] of Object.entries(SENSES)) if (v === bit) return k; return 'a sense'; }
export class StaticCollision {
  private cells: number[][];        // per 1m cell: static box indices whose footprint overlaps and that matter for movement
  private W = WORLD.gridW; private H = WORLD.gridH; private cs = WORLD.gridCell;
  nav!: NavGrid;
  /** shared sound-propagation policy (audio engine + guard hearing) */
  sound!: SoundPaths;

  constructor(public boxes: Box[]) {
    this.cells = Array.from({ length: this.W * this.H }, () => []);
    boxes.forEach((b, i) => {
      if (b.flags & (BoxFlag.NoShadow | BoxFlag.Dynamic)) return;
      const cs = Math.abs(Math.cos(b.yaw)), sn = Math.abs(Math.sin(b.yaw));
      const ex = cs * b.h[0] + sn * b.h[2], ez = sn * b.h[0] + cs * b.h[2];
      const x0 = Math.max(0, Math.floor((b.c[0] - ex) / this.cs)), x1 = Math.min(this.W - 1, Math.floor((b.c[0] + ex) / this.cs));
      const z0 = Math.max(0, Math.floor((b.c[2] - ez) / this.cs)), z1 = Math.min(this.H - 1, Math.floor((b.c[2] + ez) / this.cs));
      for (let z = z0; z <= z1; z++) for (let x = x0; x <= x1; x++) this.cells[z * this.W + x].push(i);
      if (b.rot && x0 <= x1 && z0 <= z1) this.tiltedAt.push(i, x0, x1, z0, z1);
    });
    this.stamp = new Int32Array(boxes.length);
    this.trigYaw = new Float64Array(boxes.length).fill(NaN); this.trigCos = new Float64Array(boxes.length); this.trigSin = new Float64Array(boxes.length);
    this.nav = new NavGrid(this); this.sound = new SoundPaths(this);
  }

  /** WALL BREAKING (game/wallbreak.ts): a static appended at RUNTIME — a piece standing in for a holed wall or a chewed crate. It joins `boxes` (the level's own
   *  list: every index a raycast reports stays an index into it) and is registered in the cells of `footprint`'s XZ extent (default: its own; the wall breaker
   *  hands the AUTHORED box, so a piece slot re-shaped inside that footprint later needs no re-registration — every query re-tests the box itself). Returns its
   *  index. The nav grid is NOT touched here: the caller rebakes the cells it changed (NavGrid.rebake). */
  addStatic(b: Box, footprint: Box = b): number {
    const i = this.boxes.length; this.boxes.push(b); this.mirror.wroteGrid();
    if (this.stamp.length <= i) { const s = new Int32Array(Math.max(i + 1, this.stamp.length * 2)); s.set(this.stamp); this.stamp = s; }
    const f = footprint; const cs = Math.abs(Math.cos(f.yaw)), sn = Math.abs(Math.sin(f.yaw));
    const ex = cs * f.h[0] + sn * f.h[2], ez = sn * f.h[0] + cs * f.h[2];
    const x0 = Math.max(0, Math.floor((f.c[0] - ex) / this.cs)), x1 = Math.min(this.W - 1, Math.floor((f.c[0] + ex) / this.cs));
    const z0 = Math.max(0, Math.floor((f.c[2] - ez) / this.cs)), z1 = Math.min(this.H - 1, Math.floor((f.c[2] + ez) / this.cs));
    for (let z = z0; z <= z1; z++) for (let x = x0; x <= x1; x++) this.cells[z * this.W + x].push(i);
    return i;
  }
  /** …and the level's list cut back to its first `n` statics (the authored ones: a restart): every later index leaves the cells. */
  truncateStatics(n: number) {
    if (this.boxes.length <= n) return;
    for (const cell of this.cells) { let w = 0; for (let k = 0; k < cell.length; k++) if (cell[k] < n) cell[w++] = cell[k]; cell.length = w; }
    this.boxes.length = n; this.mirror.wroteGrid();
  }

  /** THE RUST MODULE'S COPY OF THIS WORLD (scene/boxMirror.ts, which says whose the boxes are and how the copy is kept): the host's, in no snapshot. */
  readonly mirror = new BoxMirror();
  /** A TICK OPENS (Game.stepSeats): with a row of RUST on that asks the box world, the copy is made the boxes as they stand and the questions of this tick go
   *  to the module; with none, nothing happens here at all. */
  tickOpen(): void { this.noFit = false; this.stand(); }
  /** …the copy made the boxes as they stand, for whoever opens a stretch of questions: a tick, a window, a question alone (below) */
  private stand(): void { const M = this.mirror, S = EXACT.on ? senseRows() : 0;   // (the senses answer under the game's own maths alone: senseSet)
     if (RUST.lines || RUST.rays || RUST.circles || RUST.fog || S || ((RUST.doors || RUST.props || RUST.slabs || RUST.items) && EXACT.on)) M.open(this.boxes, this.cells, this.tiltedAt, this.W, this.H, this.cs, this.dynamicBoxes, this.glassWalkable, this.glassPanes, RUST.lines, RUST.circles, RUST.fog, S, RUST.rays); else if (M.live) M.shut(); }
  /** THE WORLD A RULE OF THE MODULE'S MAY STEP OVER NOW (the doors' step: game/doors.ts; the furniture's: game/props.ts): the copy, standing, with the game's own leaves `dyn` in it as they
   *  are — or null (no tick, no copy, a level too big for it, another list in the leaves' place): the TypeScript steps. */
  rulesWorld(dyn: Box[]): BoxMirror | null { if (ruleRed(RULES_ROW)) return null; const M = this.ready(); if (M.rules(dyn)) return M;
    if (FELL.on) fell(RULES_ROW, !M.live ? this.fellWhy() : 'another list in the leaves\' place, or leaves that do not fit a list'); return null; }
  /** …and THE WORLD A RULE MAY ASK THE STATICS OF ALONE (the furniture's settle as a level is stood up, game/props.ts: no leaf is asked for, and the game's
   *  own leaves need not be in it yet): the copy, standing with what it owed paid — or null (no window on it: the TypeScript's, and counted by the caller) */
  rulesStatics(): BoxMirror | null { const M = this.ready(); return M.live ? M : null; }
  /** …and THE PANE RULE AS A RULE'S STEP TAKES IT (a slab's and a thrown thing's rays ask it; `paneAsk` below): 0 or 1 to hand the module, or −1 — the step
   *  is the TypeScript's, and counted as its own here, once for every rule that asks */
  rulesPanes(): number { const ask = this.paneAsk; if (ask < 0) this.paneFell(RULES_ROW, ask); return ask; }
  /** why the copy is not answering now, for the count of questions the TypeScript took (sim.ts FELL) */
  private fellWhy(): string { const M = this.mirror; return M.live ? 'the row was not up when the tick opened' : !M.fit ? 'the level does not fit the copy' : 'outside a tick'; }
  /** the foot of a span as the statics are asked it (footBand), for a rule that asks them in the module */
  footOf(y0: number): number { return footBand(y0); }
  /** …and shuts */
  tickShut(): void { if (this.mirror.live) this.mirror.shut(); }
  /** A WINDOW ON THE COPY, FOR QUESTIONS ASKED OUTSIDE A TICK. The module answers a row's questions only over a copy that has been PROVEN (every static
   *  compared with its row: tickOpen), and a proof holds only while nobody writes a box without saying so. A tick is one such stretch. `window(fn)` is
   *  another: the copy is proven once, `fn`'s questions go to the module, and it shuts when `fn` returns, however it returns. Inside it the tick's rule
   *  holds: a rule that writes a static says `wrote()`. Already inside a tick or a window, `fn` just runs. A tick that opens and shuts inside `fn` shuts
   *  the window with it (a tick's writers answer to the tick alone); what `fn` asks after that is asked alone, below.
   *  A QUESTION OUTSIDE EVERY WINDOW IS ITS OWN (`lone`): the copy is proven for it and shut behind it. That is always right and costs a proof a
   *  question (about 25 ns a static), so code that asks many in a row opens a window (a level's deal, the camera's step of a frame, the sound's
   *  probe, a tool's driver). A window is proven by the first question asked in it: one that asks nothing costs nothing. */
  window<T>(fn: () => T): T {
    const M = this.mirror; if (M.live || this.windowed || !ANYTIME.on) return fn();
    this.windowed = true; this.noFit = false;
    try { return fn(); } finally { this.windowed = false; if (M.live) M.shut(); }
  }
  /** inside `window(fn)`, outside a tick: the copy is proven by the first question `fn` asks and stands until `fn` returns (a window nobody asks in costs nothing) */
  private windowed = false;
  /** the copy did not stand when it was last asked for, because the level does not fit it: nobody asks again until the next tick or window opens (a
   *  level that does not fit is the TypeScript's, slower and never wrong; a failed copy a question would be slower than that) */
  private noFit = false;
  /** THE COPY MADE READY FOR A QUESTION OF THE MODULE'S, where every asker passes: inside a window that no question has proven yet it is proven now, and
   *  a debt of the tick (a static written and said, the grid written) is paid */
  private ready(): BoxMirror { const M = this.mirror; if (!M.live && this.windowed) this.lone(); if (M.owes) M.again(this.boxes, this.glassWalkable); return M; }
  /** a question outside a tick: is the copy now standing for it ALONE (true: the caller shuts it behind the answer; false with the copy standing: a
   *  window's, which shuts it) */
  private lone(): boolean { if (!ANYTIME.on || this.noFit) return false; this.stand(); if (!this.mirror.live) { if (!this.mirror.fit) this.noFit = true; return false; }
    if (this.windowed) { ANYTIME.windows++; return false; }
    ANYTIME.proofs++; if (ANYTIME.who || FELL.who) { const W = ANYTIME.who ?? (ANYTIME.who = {}), st = callers(4, 2, /scene\/(collision|boxMirror)\.ts/, 3); W[st] = (W[st] ?? 0) + 1; }   // (who asked alone, wherever the count asks who: TPS_FELL=who, ?fell=who)
    return true; }
  /** A STATIC WAS WRITTEN INSIDE A TICK — its place, its size, its flags, its cut — by a rule that may be asked about again before the tick ends (a wall holed,
   *  a pane out, a bar down, a piece of furniture gone): say so, and the copy has it. Between ticks nobody need say anything. */
  wrote(): void { this.mirror.wrote(); }

  /** De-duplication of box indices met in more than one grid cell during ONE query (gather / collideCircle / segmentBlocked / raycast): a box counts as seen when
   *  stamp[bi] === the query's id. One array for the object's life instead of a Set per call — these run hundreds of times a frame (every guard's eyes, every sound,
   *  every prop substep). The queries never nest (none calls another that stamps), so one counter serves them all; only indices baked into `cells` are ever stamped,
   *  and those are < the length the array was sized with. */
  private stamp: Int32Array; private stampId = 0;
  /** A QUESTION ASKED BY THE PICTURE (an overlay, a check's own read): `fn` run with the query stamps and the walk grid's search tables swapped for the asker's
   *  own (`own`: an empty object the asker keeps, one a game), and no sound's route kept (SoundPaths.asking), so the rays and the searches it makes write nothing of the game's — a game that is drawn and
   *  the same game not drawn hold the same state. Not to be nested, and `fn` must add no box. */
  aside<T>(own: { s?: object; q?: object }, fn: () => T): T {
    type Q = { stamp: Int32Array; id: number; cell: Int32Array<ArrayBuffer>; seg: Int32Array<ArrayBuffer>; yaw: Float64Array; cos: Float64Array; sin: Float64Array };
    let q = own.q as Q | undefined;   // (a table shorter than the game's: boxes added since — an empty one is a clean one, for any id above 0 and any yaw)
    if (!q || q.stamp.length < this.stamp.length || q.yaw.length < this.trigYaw.length) { const n = this.trigYaw.length;
      q = { stamp: new Int32Array(this.stamp.length), id: q?.id ?? 0, cell: q?.cell ?? new Int32Array(this.cellStamp.length), seg: q?.seg ?? new Int32Array(this.segCells.length), yaw: new Float64Array(n).fill(NaN), cos: new Float64Array(n), sin: new Float64Array(n) }; }
    const mine: Q = { stamp: this.stamp, id: this.stampId, cell: this.cellStamp, seg: this.segCells, yaw: this.trigYaw, cos: this.trigCos, sin: this.trigSin };
    const put = (t: Q) => { this.stamp = t.stamp; this.stampId = t.id; this.cellStamp = t.cell; this.segCells = t.seg; this.trigYaw = t.yaw; this.trigCos = t.cos; this.trigSin = t.sin; };
    const asked = this.sound.asking; put(q); this.sound.asking = true;
    try { return this.nav.aside(own, fn); }
    finally { this.sound.asking = asked; own.q = { stamp: this.stamp, id: this.stampId, cell: this.cellStamp, seg: this.segCells, yaw: this.trigYaw, cos: this.trigCos, sin: this.trigSin } satisfies Q; put(mine); }
  }
  /** segmentBlocked's own: a grid cell already scanned (or passed as nowhere near) by this query (the stamp's id), and each static's cos / sin beside the exact yaw they were taken at */
  /** statics appended after the bake (a wall's standing pieces) get trig slots too; a slot is keyed by the yaw it was filled for, so a re-used index is never stale */
  private growTrig() {
    const n = Math.max(this.boxes.length, this.trigYaw.length * 2); const y = new Float64Array(n).fill(NaN), c = new Float64Array(n), s = new Float64Array(n);
    y.set(this.trigYaw); c.set(this.trigCos); s.set(this.trigSin); this.trigYaw = y; this.trigCos = c; this.trigSin = s;
  }
  private cellStamp = new Int32Array(WORLD.gridW * WORLD.gridH); private segCells = new Int32Array(WORLD.gridW * WORLD.gridH); private tiltedAt: number[] = []; private trigYaw!: Float64Array; private trigCos!: Float64Array; private trigSin!: Float64Array;
  /** Indices of the movement-relevant static boxes whose grid cells overlap the XZ rectangle [x0,x1]×[z0,z1] and whose vertical span overlaps
   *  [y0, y1], written once each into `out` (no allocation — the prop solver calls this per body per substep). Returns the count (capped at out.length). */
  gather(x0: number, z0: number, x1: number, z1: number, y0: number, y1: number, out: Int32Array): number {
    if (!this.mirror.live && RUST.circles && this.lone()) { try { return this.gatherAsk(x0, z0, x1, z1, y0, y1, out); } finally { this.mirror.shut(); } }
    return this.gatherAsk(x0, z0, x1, z1, y0, y1, out);
  }
  private gatherAsk(x0: number, z0: number, x1: number, z1: number, y0: number, y1: number, out: Int32Array): number {
    const M = this.ready();
    if (!M.circles || out.length > M.listCap) { if (FELL.on && RUST.circles) fell('circles', M.circles ? 'a list longer than the module\'s' : this.fellWhy()); return this.gatherHere(x0, z0, x1, z1, y0, y1, out); }
    // RUST.circles, inside a tick (rust/sim/src/collide.rs): the module lists them, and the list is read into `out`
    const n = M.x.collide_gather(M.slot, x0, z0, x1, z1, y0, y1, out.length), w = u32(), at = M.list;
    if (BOTH.on) { const t = this.gatherHere(x0, z0, x1, z1, y0, y1, out); let ok = t === n; for (let i = 0; ok && i < n; i++) ok = out[i] === w[at + i]; this.compared(M, -1, ok, 'gather', [x0, y0, z0], [x1, y1, z1], out.length, n); return t; }
    for (let i = 0; i < n; i++) out[i] = w[at + i];
    return n;
  }
  /** gather as this file answers it */
  gatherHere(x0: number, z0: number, x1: number, z1: number, y0: number, y1: number, out: Int32Array): number {
    const id = ++this.stampId; let n = 0;
    const cx0 = Math.max(0, Math.floor(x0 / this.cs)), cx1 = Math.min(this.W - 1, Math.floor(x1 / this.cs));
    const cz0 = Math.max(0, Math.floor(z0 / this.cs)), cz1 = Math.min(this.H - 1, Math.floor(z1 / this.cs));
    for (let z = cz0; z <= cz1; z++) for (let x = cx0; x <= cx1; x++) {
      const cell = this.cells[z * this.W + x];
      for (let k = 0; k < cell.length; k++) {
        const bi = cell[k]; if (this.stamp[bi] === id) continue; this.stamp[bi] = id;
        const b = this.boxes[bi]; if (b.c[1] + b.h[1] < y0 || b.c[1] - b.h[1] > y1 || (b.flags & BoxFlag.NoShadow)) continue;   // (NoShadow at QUERY time: a static the destruction lane has knocked off its desk — flags raised at runtime; nothing baked into the cells carries it)
        if (this.glassWalkable && (b.flags & BoxFlag.Glass) && this.glassWalkable(bi)) continue;   // (GLASS BREAKING: a pane open along a walker's whole height is no obstacle to a shoved chair either — the movement rule, glassbreak.ts walkable)
        if (n < out.length) out[n++] = bi;
      }
    }
    return n;
  }

  /** The statics a circle of radius r at (x, z) TOUCHES between heights y0 and y1 — collideCircle's own test (the same boxes skipped: runtime-hidden ones,
   *  walked-through glass), asked without moving anything. Indices into `out`; returns the count, or -1 if `out` was too short to hold them all. */
  circleTouches(x: number, z: number, r: number, y0: number, y1: number, out: Int32Array): number {
    if (!this.mirror.live && RUST.circles && this.lone()) { try { return this.circleTouchesAsk(x, z, r, y0, y1, out); } finally { this.mirror.shut(); } }
    return this.circleTouchesAsk(x, z, r, y0, y1, out);
  }
  private circleTouchesAsk(x: number, z: number, r: number, y0: number, y1: number, out: Int32Array): number {
    const M = this.ready();
    if (!M.circles || out.length > M.listCap) { if (FELL.on && RUST.circles) fell('circles', M.circles ? 'a list longer than the module\'s' : this.fellWhy()); return this.circleTouchesHere(x, z, r, y0, y1, out); }
    // RUST.circles, inside a tick: gathered and tested in the module; `out` is left as this file's own two steps leave it (the touching ones, then what the gather had put past them)
    const v = M.x.collide_touches(M.slot, x, z, r, footBand(y0), y1, out.length), full = v < 0, n = full ? out.length : v & 0xffff, k = full ? -1 : v >>> 16, w = u32(), at = M.list;
    if (BOTH.on) { const t = this.circleTouchesHere(x, z, r, y0, y1, out); let ok = t === k; for (let i = 0; ok && i < n; i++) ok = out[i] === w[at + i]; this.compared(M, -1, ok, 'circleTouches', [x, y0, z], [r, y1, 0], out.length, k); return t; }
    for (let i = 0; i < n; i++) out[i] = w[at + i];
    return k;
  }
  /** circleTouches as this file answers it */
  circleTouchesHere(x: number, z: number, r: number, y0: number, y1: number, out: Int32Array): number {
    y0 = footBand(y0); const n = this.gatherHere(x - r, z - r, x + r, z + r, y0, y1, out); if (n >= out.length) return -1;
    let k = 0;
    for (let j = 0; j < n; j++) {
      const b = this.boxes[out[j]]; const c = Math.cos(b.yaw), s = Math.sin(b.yaw), dx = x - b.c[0], dz = z - b.c[2];
      if (b.round === true) { const rr = r + b.h[0]; if (dx * dx + dz * dz < rr * rr) out[k++] = out[j]; continue; }   // (ROUND THINGS: a circle against a circle, as collideCircle)
      const lx = c * dx - s * dz, lz = s * dx + c * dz, ex = lx - Math.max(-b.h[0], Math.min(b.h[0], lx)), ez = lz - Math.max(-b.h[2], Math.min(b.h[2], lz));
      if (ex * ex + ez * ez < r * r) out[k++] = out[j];
    }
    return k;
  }
  /** Push a circle (XZ) of radius r out of static boxes overlapping the vertical span [y0, y1]. Mutates p. */
  collideCircle(p: Vec3, r: number, y0: number, y1: number, iterations = 3): boolean {
    if (!this.mirror.live && RUST.circles && this.lone()) { try { return this.collideCircleAsk(p, r, y0, y1, iterations); } finally { this.mirror.shut(); } }
    return this.collideCircleAsk(p, r, y0, y1, iterations);
  }
  private collideCircleAsk(p: Vec3, r: number, y0: number, y1: number, iterations = 3): boolean {
    const M = this.ready();
    if (!M.circles) { if (FELL.on && RUST.circles) fell('circles', this.fellWhy()); return this.collideCircleHere(p, r, y0, y1, iterations); }
    // RUST.circles, inside a tick: pushed out in the module, and where it ends read back
    const hit = M.x.collide_circle(M.slot, p[0], p[2], r, footBand(y0), y1, iterations) !== 0, m = f64(), at = M.pt;
    if (BOTH.on) { const x = m[at], z = m[at + 1], t = this.collideCircleHere(p, r, y0, y1, iterations); this.compared(M, -1, t === hit && Object.is(p[0], x) && Object.is(p[2], z), 'collideCircle', p, [r, y0, y1], iterations, 0); return t; }
    p[0] = m[at]; p[2] = m[at + 1];
    return hit;
  }
  /** collideCircle as this file answers it */
  collideCircleHere(p: Vec3, r: number, y0: number, y1: number, iterations: number): boolean {
    let touched = false; y0 = footBand(y0);
    for (let it = 0; it < iterations; it++) {
      let any = false;
      const x0 = Math.max(0, Math.floor((p[0] - r) / this.cs)), x1 = Math.min(this.W - 1, Math.floor((p[0] + r) / this.cs));
      const z0 = Math.max(0, Math.floor((p[2] - r) / this.cs)), z1 = Math.min(this.H - 1, Math.floor((p[2] + r) / this.cs));
      const id = ++this.stampId;   // (a fresh id per iteration: a box pushed out of in pass 0 must be looked at again in pass 1)
      for (let z = z0; z <= z1; z++) for (let x = x0; x <= x1; x++) for (const bi of this.cells[z * this.W + x]) {
        if (this.stamp[bi] === id) continue; this.stamp[bi] = id;
        const b = this.boxes[bi];
        if (b.c[1] + b.h[1] < y0 || b.c[1] - b.h[1] > y1 || (b.flags & BoxFlag.NoShadow)) continue;   // (a runtime-hidden static: see gather)
        if (this.glassWalkable && (b.flags & BoxFlag.Glass) && this.glassWalkable(bi)) continue;   // (GLASS BREAKING: a pane whose cells along a walker's height have ALL fallen out is walked through — glassbreak.ts walkable; the nav grid is NOT rebaked)
        const dx = p[0] - b.c[0], dz = p[2] - b.c[2];
        if (b.round === true) {   // ROUND THINGS (scene/round.ts): a body is pushed off the CIRCLE of radius h[0], straight out from its centre
          const rr = r + b.h[0], d2r = dx * dx + dz * dz; if (d2r >= rr * rr) continue;
          if (d2r > 1e-10) { const d = Math.sqrt(d2r), pen = rr - d; p[0] += dx / d * pen; p[2] += dz / d * pen; } else p[0] += rr;   // (dead centre: out along +x)
          any = true; touched = true; continue;
        }
        // circle center in box local space
        const c = Math.cos(b.yaw), s = Math.sin(b.yaw);
        const lx = c * dx - s * dz, lz = s * dx + c * dz;
        const qx = Math.max(-b.h[0], Math.min(b.h[0], lx)), qz = Math.max(-b.h[2], Math.min(b.h[2], lz));
        let ddx = lx - qx, ddz = lz - qz; let d2 = ddx * ddx + ddz * ddz;
        if (d2 >= r * r) continue;
        let nx: number, nz: number, pen: number;
        if (d2 > 1e-10) { const d = Math.sqrt(d2); nx = ddx / d; nz = ddz / d; pen = r - d; }
        else {
          // center inside the rectangle: push out along the axis of least penetration
          const px = b.h[0] - Math.abs(lx), pz = b.h[2] - Math.abs(lz);
          if (px < pz) { nx = Math.sign(lx) || 1; nz = 0; pen = px + r; } else { nx = 0; nz = Math.sign(lz) || 1; pen = pz + r; }
        }
        // back to world
        const wx = c * nx + s * nz, wz = -s * nx + c * nz;
        p[0] += wx * pen; p[2] += wz * pen; any = true; touched = true;
      }
      if (!any) break;
    }
    return touched;
  }

  /** Keep a circle of radius r on the world's floor rectangle (WORLD.min / max in XZ — the ground slab simply ends there: no kerb, no wall). Guards are held inside
   *  by the nav grid, which stops at the same edge; a body moved by hand (the player) needs this. Mutates p; true if it had to move it. */
  clampToWorld(p: Vec3, r: number): boolean {
    const x = Math.min(WORLD.max[0] - r, Math.max(WORLD.min[0] + r, p[0])), z = Math.min(WORLD.max[2] - r, Math.max(WORLD.min[2] + r, p[2]));
    const moved = x !== p[0] || z !== p[2]; p[0] = x; p[2] = z; return moved;
  }

  /** GLASS BREAKING (game/glassbreak.ts): is the point p on glass static bi inside a cell that has FALLEN OUT? null = panes never open (the game as it was).
   *  raycast lets a round through such a point (on to whatever stands beyond) and the solid segment test lets sound / a step / a line of fire through it.
   *  A RULE HUNG ALONE HAS NO ROWS: the TypeScript answers whatever may meet a pane (`paneAsk` −1) — a check's made-up closure, a mode's own. */
  get glassOpen(): ((bi: number, p: Vec3) => boolean) | null { return this.paneRule; }
  set glassOpen(f: ((bi: number, p: Vec3) => boolean) | null) { this.glassRule(f, null); }
  /** GLASS BREAKING, THE RULE AND WHAT IT ANSWERS BY, HUNG TOGETHER (game/glassbreak.ts hangs itself; scene/boxMirror.ts PaneSource): `rows` are the panes
   *  that are no longer whole and the burst's view in hand, as the copy can hold them, so a line or a ray of the module's judges a pane where it meets it.
   *  One call, so the module never judges by rows that were another rule's. `rows` is a FUNCTION, as every hook the game hangs here is: the collision
   *  keeps no object of the game's, so whatever walks a game's state finds its glass where it always was. */
  glassRule(rule: ((bi: number, p: Vec3) => boolean) | null, rows: (() => PaneSource) | null): void { this.paneRule = rule; this.paneRows = rule === null ? null : rows; this.mirror.glassIs(this.glassPanes); }
  get glassPanes(): PaneSource | null { return this.paneRows === null ? null : this.paneRows(); }
  private paneRule: ((bi: number, p: Vec3) => boolean) | null = null;
  private paneRows: (() => PaneSource) | null = null;   // (handed to the copy each time it is stood up: a world outlives its game, and holds nothing of it once shut)
  /** …the glass is being worked on, from here to there (a blow, a blast, a step of what is coming down): the copy's panes are written again before each of
   *  the module's answers meanwhile, and before the first one after */
  glassBegin(): void { this.mirror.glassBegin(); }
  glassEnd(): void { this.mirror.glassEnd(); }
  /** …or it was written with no question of its own between (a pane made another glass, every pane made whole) */
  glassWrote(): void { this.mirror.glassWrote(); }
  /** …and a burst's view of the glass was put on or taken off */
  glassViewed(): void { this.mirror.viewed(); }
  /** THE PANE RULE AS A QUESTION OF THE MODULE'S TAKES IT: 0 = the game has none (every pane is shut) · 1 = it has one and the copy holds it · −1 = it has
   *  one the module cannot judge by (a rule hung with no rows behind it; more broken panes, or a view of more panes, than the copy's tables) · −2 = IDLE:
   *  the engine's maths stand in Math's place and a broken pane's pieces have to be found — by angles, and the module's are the game's own. That is the
   *  rule of every row that works a length or an angle out (sim.ts OWN_MATHS), kept for the pane's question where it is asked, since the rows that ask
   *  it (`lines`, `rays`) answer boxes under either maths. Below nought the TypeScript answers whatever may meet a pane; only −1 is a question the
   *  module should have had (`paneFell`). A line that passes glass meets none, and asks with 0 whatever this says. */
  get paneAsk(): number { const M = this.mirror; return this.paneRule === null ? 0 : !EXACT.on && !M.glassPlain ? -2 : M.glassIn ? 1 : -1; }
  /** …a question the TypeScript took for it, counted (sim.ts FELL) where the module should have had it: never where the pane's question is idle */
  paneFell(row: string, ask: number): void {
    if (FELL.on && ask === -1) fell(row, `a pane's rule the module cannot judge by: ${this.paneRows === null ? 'no glass stands behind glassOpen' : 'more broken panes, or a view of more panes, than the copy holds'}`); }
  /** GLASS BREAKING: is glass static bi open along a walker's WHOLE height (every cell reaching below WALK_CLEAR_H gone)? Movement (collideCircle) and the props'
   *  gather then ignore it — the one movement rule, chosen so a man's capsule and solidBlocked never disagree about a pane he could step through; a pane holed only
   *  at head height stays the wall it was to his feet. null = never. (The nav grid is baked once at construction and is NOT rebaked: A* still routes round the frame.) */
  glassWalkable: ((bi: number) => boolean) | null = null;

  /** Extra oriented boxes (door leaves) that block sight / sound / projectiles but are not part of the baked grid.
   *  Character movement (collideCircle) and the nav bake deliberately ignore them — doors have their own contact model. */
  dynamicBoxes: Box[] = [];

  /** True if the segment a→b crosses one of the dynamic occluders (door leaves) — statics ignored. With `needFlags`, only occluders carrying those flag bits count. */
  segmentBlockedDynamic(a: Vec3, b: Vec3, needFlags = 0): boolean {
    if (!this.mirror.live && RUST.lines && this.lone()) { try { return this.segmentBlockedDynamicAsk(a, b, needFlags); } finally { this.mirror.shut(); } }
    return this.segmentBlockedDynamicAsk(a, b, needFlags);
  }
  private segmentBlockedDynamicAsk(a: Vec3, b: Vec3, needFlags = 0): boolean {
    const d0 = b[0] - a[0], d1 = b[1] - a[1], d2 = b[2] - a[2]; const len = hyp3(d0, d1, d2); if (len < 1e-4) return false; const il = 1 / len;
    const M = this.ready();
    if (M.lines) {   // RUST.lines, inside a tick: the same loop over the module's copy of these leaves
      const set = len < 1e6 && ((needFlags | 0) & ~MIRRORED) === 0 ? M.leaves(this.dynamicBoxes) : -2;
      if (set !== -2) {
        const r = set >= 0 && M.x.lines_leaves(M.slot, a[0], a[1], a[2], b[0], b[1], b[2], len, needFlags, set) !== 0;
        if (!BOTH.on) return r;
        const t = this.dynamicHit(a, d0 * il, d1 * il, d2 * il, len, needFlags); this.compared(M, set, t === r, 'leaves', a, b, needFlags, 0); return t;
      }
    }
    if (FELL.on && RUST.lines) fell('lines', M.lines ? 'the leaves alone: a flag or a length the copy does not keep' : this.fellWhy());
    return this.dynamicHit(a, d0 * il, d1 * il, d2 * il, len, needFlags);
  }
  /** THE FURNITURE'S RAY, ASKED OF THE MODULE (game/props.ts PropSystem.raycast: a round, a beam, a piece of debris, a mark, a thrown thing's plan): the
   *  nearest part of any thing of `P` the ray meets within tmax, over the copy's own list of the furniture's parts as they stand (lines.rs ray_things;
   *  the parts are copied where one was written since: boxMirror.ts things). Inside a tick or a window the copy stands; outside both it is proven for
   *  this one question and shut behind it (`lone`). → the hit (which row of `P.props`, how far along, the face's normal, the part's box), null = nothing
   *  met, or UNDEFINED = the module cannot answer it now (no copy standing, or more furniture than it holds): the caller answers, and says so. */
  thingsRay(P: Furniture, ro: Vec3, rd: Vec3, tmax: number): { row: number; t: number; n: Vec3; box: Box } | null | undefined {
    if (!this.mirror.live && this.lone()) { try { return this.thingsAsk(P, ro, rd, tmax); } finally { this.mirror.shut(); } }
    return this.thingsAsk(P, ro, rd, tmax);
  }
  private thingsAsk(P: Furniture, ro: Vec3, rd: Vec3, tmax: number): { row: number; t: number; n: Vec3; box: Box } | null | undefined {
    const M = this.ready(); if (!M.live || !M.things(P)) return undefined;
    const k = M.x.lines_ray_things(M.slot, ro[0], ro[1], ro[2], rd[0], rd[1], rd[2], tmax);
    if (k < 0) return null;
    const m = f64(), at = M.scratch, box = M.thingBox(P, m[at + 4]);
    if (box === null) throw new Error(`the Rust module's ray into the furniture met part ${m[at + 4]} of its list, which is no part of the furniture as it stands (${P.props.length} things)`);
    return { row: k, t: m[at], n: [m[at + 1], m[at + 2], m[at + 3]], box };
  }
  /** …AND ANSWERED BOTH WAYS (the check's: BOTH; PropSystem.raycast calls this with what its own walk answered): the module is asked the same ray over
   *  its copy of the furniture's parts, the two answers are compared, and the copy with the furniture as it stands. */
  thingsBoth(P: Furniture, ro: Vec3, rd: Vec3, tmax: number, t: { t: number; n: Vec3; prop: unknown; box: Box } | null): void {
    const M = this.mirror; if (!M.live || M.owesBoxes || !M.things(P)) return;   // (a debt is paid by the game's own questions, never by the check's)
    const k = M.x.lines_ray_things(M.slot, ro[0], ro[1], ro[2], rd[0], rd[1], rd[2], tmax), m = f64(), at = M.scratch;
    const ok = k < 0 ? t === null : t !== null && P.props[k] === t.prop && Object.is(m[at], t.t) && Object.is(m[at + 1], t.n[0]) && Object.is(m[at + 2], t.n[1]) && Object.is(m[at + 3], t.n[2]) && M.thingBox(P, m[at + 4]) === t.box;
    const st = M.staleThings(P); BOTH.asked++; BOTH.things++; BOTH.stale += st; if (!ok) BOTH.differ++;
    if ((st || !ok) && !BOTH.first) BOTH.first = `the furniture's ray [${ro.join(', ')}] → [${rd.join(', ')}] (${tmax}, ${k}): ${ok ? '' : 'the two answers differ; '}${st} row(s) of the copy not the furniture`;
  }
  private compared(M: BoxMirror, set: number, ok: boolean, what: string, a: Vec3, b: Vec3, p: number, q: number): void {
    const st = M.stale(this.boxes, this.dynamicBoxes, set) + M.stalePanes(); BOTH.asked++; BOTH.stale += st; if (!ok) BOTH.differ++;
    if ((st || !ok) && !BOTH.first) BOTH.first = `${what} [${a.join(', ')}] → [${b.join(', ')}] (${p}, ${q}): ${ok ? '' : 'the two answers differ; '}${st} row(s) of the copy not their box or their pane`;
  }
  /** segmentBlockedDynamic's loop over the leaves for a segment already reduced to origin + unit direction (r0, r1, r2) + length — the same floats the array form
   *  made (d · (1 / len)), so segmentBlocked hands its own down instead of deriving them twice. A leaf the segment provably cannot touch (segClear) is passed
   *  without the slab test; the rest get rayBox's own arithmetic (rayBoxHits) with no result object. */
  private dynamicHit(a: Vec3, r0: number, r1: number, r2: number, len: number, needFlags: number): boolean {
    const dyn = this.dynamicBoxes; const a0 = a[0], a1 = a[1], a2 = a[2];
    const uu = r0 * r0 + r2 * r2, su = Math.sqrt(uu), lu = len * uu;   // (the segment on the floor plan: a + t · (r0, r2), t in [0, len])
    for (let i = 0; i < dyn.length; i++) {
      const db = dyn[i]; if ((db.flags & needFlags) !== needFlags) continue;
      if (db.rot || db.round === true || (db.cut !== undefined && cutMet(db))) { if (segClear(a0, a1, a2, r0, r1, r2, len, db, SEG_PAD)) continue; const hit = rayBox(a, [r0, r1, r2], db, len); if (hit && hit.t >= 0) return true; continue; }
      // an upright leaf stands inside the cylinder of radius |h0| + |h2| round its centre: a segment whose floor-plan line passes further off than that (the cross
      // product, no division), or whose stretch of it ends before / starts after it, cannot enter the leaf — rayBox would say no
      const c = db.c, h = db.h; const R = (h[0] < 0 ? -h[0] : h[0]) + (h[2] < 0 ? -h[2] : h[2]) + SEG_PAD;
      const m0 = c[0] - a0, m2 = c[2] - a2; const cr = m0 * r2 - m2 * r0; if (cr * cr > R * R * uu) continue;
      const s = m0 * r0 + m2 * r2, e = R * su; if (s < -e || s > lu + e) continue;
      if (rayBoxHits(a0, a1, a2, r0, r1, r2, db, len)) return true;
    }
    return false;
  }

  /** True if the segment a→b is blocked by a static box or a dynamic occluder (ignores NoShadow decals). `through` = flags of statics the segment PASSES: by default
   *  glass (BoxFlag.Glass) — this is the floor's line-of-SIGHT test (eyes, torch cones, the light between a man and a lamp, the dazzle), and a pane is nothing to any
   *  of those; pass 0 for the SOLID question (sound, a step, a lunge, an arm's reach: SoundPaths and the movement / reach gates do). Panes never break unless glass breaking is on (a round
   *  stars the glass and stops in it), so both answers hold for the whole encounter. One model: they see you through the conference-room glass and do not hear
   *  you through it.
   *  A static counts when the segment runs INSIDE it beyond the first `shrink` and enters it before the last `shrink` (lineBox's slab interval: t1 > shrink and
   *  t0 < len − shrink) — so a segment that merely starts or ends on a surface (an eye at a wall, a body on the floor, a lamp on its panel) is not blocked by that
   *  surface, but one BORN ON A FACE AND HEADING IN (an impact's sound on the wall it struck, a listener behind it) meets the wall's full thickness and is: the old
   *  entry-distance test (rayBox: t > shrink, and null from inside) let 288 of 318 such segments over the office's wall faces straight through, and made a→b and
   *  b→a disagree for every one of them. The one exemption kept from that rule, on purpose: a box the ORIGIN is embedded in deeper than `shrink` is skipped — a light
   *  inside its fixture's housing, a fixture box asked from its own centre which side is open (game.ts area lights) — it is not in the way of what it emits. (The
   *  END has no such exemption: a target buried in a box is behind that box's face, as it always was — the one asymmetry left, and only for endpoints inside solids.) */
  segmentBlocked(a: Vec3, b: Vec3, shrink = 0.02, through: number = BoxFlag.Glass, panes: { bi: number; t: number }[] | null = null): boolean {
    if (!this.mirror.live && RUST.lines && this.lone()) { try { return this.segmentBlockedAsk(a, b, shrink, through, panes); } finally { this.mirror.shut(); } }
    return this.segmentBlockedAsk(a, b, shrink, through, panes);
  }
  private segmentBlockedAsk(a: Vec3, b: Vec3, shrink = 0.02, through: number = BoxFlag.Glass, panes: { bi: number; t: number }[] | null = null): boolean {
    const M = this.ready();
    if (!M.lines) { if (FELL.on && RUST.lines) fell('lines', this.fellWhy()); return this.segmentBlockedHere(a, b, shrink, through, panes); }
    // RUST.lines, inside a tick (rust/sim/src/lines.rs: the walk below, statement for statement, over the module's copy of the boxes). The length is worked out
    // HERE, by the maths in Math's place, and handed over. A pane crossed where it is open is passed there, by the game's own rule over the copy's panes
    // (glass.rs); where a line of fire is counting panes, the module lists the glass in the way that is not open — nothing solid being in the way.
    const a0 = a[0], a1 = a[1], a2 = a[2], d0 = b[0] - a0, d1 = b[1] - a1, d2 = b[2] - a2;
    const len = hyp3(d0, d1, d2); if (len < 1e-4) return false;
    const set = len < 1e6 && ((through | 0) & ~MIRRORED) === 0 ? M.leaves(this.dynamicBoxes) : -2;   // (a length that is no number or no length of this world's, a flag the copy does not keep: this file's own answer)
    const ask = (through & BoxFlag.Glass) !== 0 ? 0 : this.paneAsk;   // (a line that passes glass is stopped by no pane and asks none)
    if (ask < 0) { this.paneFell('lines', ask); return this.segmentBlockedHere(a, b, shrink, through, panes); }
    if (set === -2) { if (FELL.on) fell('lines', len < 1e6 && ((through | 0) & ~MIRRORED) === 0 ? 'the leaves do not fit a list' : 'a flag or a length the copy does not keep'); return this.segmentBlockedHere(a, b, shrink, through, panes); }
    const r = M.x.lines_blocked(M.slot, a0, a1, a2, b[0], b[1], b[2], len, shrink, through, set, ask | (panes ? 2 : 0));
    if (r === -2) { if (FELL.on) fell('lines', 'more glass on one line than the module lists'); return this.segmentBlockedHere(a, b, shrink, through, panes); }
    const n0 = panes ? panes.length : 0, hit = r === -1;
    if (r > 0) { const wb = u32(), wt = f64(), b0 = M.glassBi, t0 = M.glassT; for (let i = 0; i < r; i++) panes!.push({ bi: wb[b0 + i], t: wt[t0 + i] }); }
    if (BOTH.on) {
      const mine = bothPanes; mine.length = 0; if (panes) { for (let i = n0; i < panes.length; i++) mine.push(panes[i]); panes.length = n0; }
      const t = this.segmentBlockedHere(a, b, shrink, through, panes);
      let ok = t === hit;
      if (ok && !t && panes) { ok = panes.length - n0 === mine.length; for (let i = 0; ok && i < mine.length; i++) ok = panes[n0 + i].bi === mine[i].bi && Object.is(panes[n0 + i].t, mine[i].t); }
      this.compared(M, set, ok, 'line', a, b, shrink, through); return t;
    }
    return hit;
  }
  /** IS THE LINE TEST THIS CLASS'S OWN: a check that puts its own `segmentBlocked` or `solidBlocked` over one instance is asking for ITS lines, which the
   *  module does not walk — so a whole question of the game's that the module would answer from its own lines (the fog's pass, a sense) is not asked there */
  private get ownLines(): boolean { return this.segmentBlocked === StaticCollision.prototype.segmentBlocked && this.solidBlocked === StaticCollision.prototype.solidBlocked; }
  /** RUST.fog — A SIGHT PASS OPENS (game/fogofwar.ts update): may its questions go to the module (rust/sim/src/fog.rs)? Inside a tick with the row on, where
   *  the game's own maths stand in Math's place (the module works the lengths out itself, and they are those: under an engine's maths the TypeScript's lengths
   *  are that engine's), with no more eyes than a question names, and the line test this class's own (`ownLines`). True = the caller writes its eyes (`sightEye`) and asks with `eyesSee`. */
  sightOpen(n: number): boolean { const M = this.ready(); const ok = M.sight && EXACT.on && n <= SIGHT_EYES && this.ownLines;
    if (!ok && FELL.on && RUST.fog) fell('fog', M.sight ? (!EXACT.on ? 'the engine\'s maths' : n > SIGHT_EYES ? 'more eyes than a question names' : 'a check\'s own line test') : this.fellWhy()); return ok; }
  /** eye `k` of the pass: where it is, where it looks in plan and that look's length (or 1), the cosine of half its field, its near field, and whether it sees all round */
  sightEye(k: number, pos: Vec3, look: Vec3, lookLen: number, cosHalf: number, near: number, full: boolean): void {
    const m = f64(), o = this.mirror.eyes + k * 9;
    m[o] = pos[0]; m[o + 1] = pos[1]; m[o + 2] = pos[2]; m[o + 3] = look[0]; m[o + 4] = look[2]; m[o + 5] = lookLen; m[o + 6] = cosHalf; m[o + 7] = near; m[o + 8] = full ? 1 : 0;
  }
  /** WHICH EYES HAVE A LINE: the `n` eyes named four bits each in `order` are tried in that order until `want` have one. `kind` 0 = to the point `a`; 1 = to a
   *  man down or gone (`a` his chest: only an eye over him has one); 3 = to a living man (`a` his chest, `b` his head, asked behind a blocked chest).
   *  The answer: how many × 2²⁸ + those eyes, four bits each in the order found; −1 = the module does not answer this one (the caller walks its own lines). */
  eyesSee(order: number, n: number, want: number, kind: number, through: number, a: Vec3, b: Vec3): number {
    const M = this.ready();
    if (!M.sight || ((through | 0) & ~MIRRORED) !== 0 || !(through & BoxFlag.Glass)) { if (FELL.on && RUST.fog) fell('fog', M.sight ? 'a line that may have to ask a pane, or a flag the copy does not keep' : this.fellWhy()); return -1; }   // (a line that stops at glass may have to ask the pane: this file's)
    const set = M.leaves(this.dynamicBoxes); if (set === -2) { if (FELL.on) fell('fog', 'the leaves do not fit a list'); return -1; }
    const r = M.x.fog_see(M.slot, set, order, n, want, kind, through, a[0], a[1], a[2], b[0], b[1], b[2]);
    if (r === -1) { if (FELL.on) fell('fog', 'the module handed the pass back (a length that is no length of this world\'s, or more eyes than a question names)'); return -1; }
    return r >>> 0;
  }
  // ---- THE SENSES IN THE MODULE (RUST.sight / light / smoke / sound / paths: rust/sim/src/sight.rs, light.rs, smoke.rs, hear.rs, nav.rs). Each is a whole
  // question of the game's (an eye on a man, the lamps on a point, the smoke along a line, a sound's way, a way across the floor) asked once where the
  // TypeScript asked its lines one by one; each answers null / a number below zero where the module does not, and the caller's own statements then do.
  /** MAY ROW `bit` (boxMirror.ts SENSES) BE ASKED THERE NOW: inside a tick with the row on, the copy standing, and the game's own maths in Math's place (the
   *  module works its lengths out itself, and they are those), and the line test this class's own (`ownLines`). → which of the module's lists holds the leaves to ask with (−1 = there are none), or −2 = no. */
  senseSet(bit: number): number { const M = this.ready(); const ok = (M.senses & bit) !== 0 && EXACT.on && this.ownLines, set = ok ? M.leaves(this.dynamicBoxes) : -2;
    if (set === -2 && FELL.on && (senseRows() & bit) !== 0) fell(senseName(bit), ok ? 'the leaves do not fit a list' : (M.senses & bit) !== 0 ? (!EXACT.on ? 'the engine\'s maths' : 'a check\'s own line test') : this.fellWhy()); return set; }
  /** the check's line for a sense's question answered both ways (BOTH): were the two answers one, and is the copy its boxes */
  senseCompared(set: number, ok: boolean, what: string, a: Vec3, b: Vec3): void { this.compared(this.mirror, set, ok, what, a, b, 0, 0); }
  /** RUST.sight — game/guards.ts eyeContact as one question: the eye `E` on a man's `chest` and `head`. `unseen` = nothing is seen whatever the lines say (he
   *  is hidden, down or dead; the eye is out), `smoked` = the smoke between them hides him. null = the caller's own statements. */
  eyeContact(E: { pos: Vec3; look: Vec3; range: number; halfDeg: number; nearM: number; nearDeg: number; vHalfDeg?: number }, chest: Vec3, head: Vec3, unseen: boolean, smoked: boolean, through: number): { visible: boolean; dist: number; inCone: boolean } | null {
    if (((through | 0) & ~MIRRORED) !== 0 || !(through & BoxFlag.Glass)) { if (FELL.on && RUST.sight) fell('sight', (through & BoxFlag.Glass) ? 'a flag the copy does not keep' : 'a line that stops at glass may have to ask the pane'); return null; }   // (the caller's)
    if (!this.mirror.live && RUST.sight && this.lone()) { try { return this.eyeContactAsk(E, chest, head, unseen, smoked, through); } finally { this.mirror.shut(); } }   // (an eye asked between ticks: a tool's, a soak's watcher)
    return this.eyeContactAsk(E, chest, head, unseen, smoked, through);
  }
  private eyeContactAsk(E: { pos: Vec3; look: Vec3; range: number; halfDeg: number; nearM: number; nearDeg: number; vHalfDeg?: number }, chest: Vec3, head: Vec3, unseen: boolean, smoked: boolean, through: number): { visible: boolean; dist: number; inCone: boolean } | null {
    const set = this.senseSet(SENSES.sight); if (set === -2) return null;   // (senseSet has counted it)
    const M = this.mirror, e = E.pos, l = E.look, v = E.vHalfDeg;
    const r = M.x.sight_contact(M.slot, set, through, (v !== undefined ? FACE.Q_LENS : 0) | (unseen ? FACE.Q_UNSEEN : 0) | (smoked ? FACE.Q_SMOKED : 0), e[0], e[1], e[2], l[0], l[1], l[2], E.range, E.halfDeg, E.nearM, E.nearDeg, v === undefined ? 0 : v,
      chest[0], chest[1], chest[2], head[0], head[1], head[2]);
    if (r === -1) { if (FELL.on) fell('sight', 'the module handed the eye back (a line too long to be one)'); return null; }
    SENSED.sight++; return { visible: (r & FACE.A_VISIBLE) !== 0, dist: f64()[M.scratch], inCone: (r & FACE.A_IN_CONE) !== 0 };
  }
  /** RUST.light — guards.ts lightEstimateAt's lamps: `lampsBegin` the question (→ may it go to the module), then a `lamp` each that passed the caller's
   *  own gates, then `lampsOn`, which leaves the six faces in `acc` (false anywhere = the caller's own loop, from the start).
   *  IN PARTS: the module's table holds `lampCap` lamps; a lamp past that sends the table as a PART and begins it again, and the module keeps the six
   *  faces from part to part (light.rs SUM), so any number of lamps in reach of a point is one sum in one order.
   *  `shade`: the smoke's shadow (sense.ts SENSE.bounce) — each lamp no wall hides is dimmed along the line to it by the smoke of `shade.grid`, and
   *  where that grid cannot answer (or none `stands`: sense.ts SmokeGrid.along) by `shade.blobs`, written here for the module's own smoke.rs under
   *  the model's numbers `K` (its table holds them all: the game keeps no more, sense.ts CpuSmoke.emit, and more is an error by name) */
  private readonly lampQ: { on: boolean; P: Vec3 | null; sim: SimState | null; stands: number; K: SmokeNumbers | null; nb: number; parts: number } = { on: false, P: null, sim: null, stands: 0, K: null, nb: 0, parts: 0 };
  lampsBegin(P: Vec3, shade: { grid: { sim: SimState; stands: boolean }; blobs: readonly BlobRow[] } | null = null, K: SmokeNumbers | null = null): boolean {
    const M = this.mirror, Q = this.lampQ; this.lampsOver(); M.lampSet = this.senseSet(SENSES.light); M.lampN = 0; if (M.lampSet === -2) return false;
    if (shade && K) { this.blobsFit(shade.blobs.length); M.blobN = 0; for (const b of shade.blobs) this.blob(b.x, b.y0, b.z, b.r2, b.age, b.mass);
      Q.sim = shade.grid.sim; Q.stands = shade.grid.stands ? 1 : 0; Q.K = K; Q.nb = M.blobN; }
    Q.P = P; Q.on = true; return true;
  }
  /** no question about the light stands (and nothing of the last one is held) */
  private lampsOver(): void { const Q = this.lampQ; Q.on = false; Q.P = null; Q.sim = null; Q.K = null; Q.stands = 0; Q.nb = 0; Q.parts = 0; }
  /** the lamps written so far, asked as a part of the question (false = the module handed it back: the caller's own loop, and the question is over) */
  private lampsPart(): boolean {
    const M = this.mirror, Q = this.lampQ, P = Q.P!, st = Q.sim, K = Q.K, more = Q.parts > 0 ? 1 : 0;   // (the grid's block is bound a part: a call that reads state says where it stands NOW, sim/state.ts)
    const r = st && K ? M.x.light_estimate(M.slot, M.lampSet, M.lampN, P[0], P[1], P[2], K.sigma, st.bind(), st.stamp, Q.stands, more, Q.nb, K.spread, K.widest, K.rise, K.riseRate)
      : M.x.light_estimate(M.slot, M.lampSet, M.lampN, P[0], P[1], P[2], 0, 0, 0, 0, more, 0, 0, 0, 0, 0);
    Q.parts++; M.lampN = 0;
    if (r === -1) { if (FELL.on) fell('light', 'the module handed the lamps back (a line too long to be one)'); this.lampsOver(); return false; }
    return true;
  }
  lamp(pos: Vec3, range: number, radius: number, kind: number, dir: Vec3, outerDeg: number, innerDeg: number, color: Vec3, intensity: number): boolean {
    const M = this.mirror;
    if (M.lampN >= M.lampCap) { if (!this.lampQ.on) return false;   // (no question stands: the grid's sweep writes its lamps here too, and takes no more than the table — a level's limit)
      LAMPS.parts++; if (!this.lampsPart()) return false; }
    const m = f64(), o = M.lamps + M.lampN++ * FACE.LAMP;
    m[o] = pos[0]; m[o + 1] = pos[1]; m[o + 2] = pos[2]; m[o + 3] = range; m[o + 4] = radius; m[o + 5] = kind; m[o + 6] = dir[0]; m[o + 7] = dir[1]; m[o + 8] = dir[2]; m[o + 9] = outerDeg; m[o + 10] = innerDeg;
    m[o + 11] = color[0]; m[o + 12] = color[1]; m[o + 13] = color[2]; m[o + 14] = intensity;
    return true;
  }
  /** …the last part asked, and the six faces of the whole question read */
  lampsOn(acc: number[]): boolean {
    const M = this.mirror; if (!this.lampQ.on || !this.lampsPart()) return false; this.lampsOver();
    const m = f64(), at = M.scratch; for (let k = 0; k < 6; k++) acc[k] = m[at + k];
    SENSED.light++; return true;
  }
  /** RUST.smoke — game/sense.ts CpuSmoke.trans: `begin` (→ may it go to the module, with this many blobs), a `blob` each, then the transmittance along
   *  a → b under the model's numbers (−1 = the caller's own loop). */
  blobsBegin(n: number): boolean { const M = this.mirror; M.blobSet = this.senseSet(SENSES.smoke); M.blobN = 0; if (M.blobSet === -2) return false;   // (senseSet has counted it)
    this.blobsFit(n); return true; }
  /** a game keeps no more blobs than the module's table holds (sense.ts CpuSmoke.emit holds its knob to the table), so more is no content: a mistake, by name */
  private blobsFit(n: number): void { const cap = this.mirror.blobCap; if (n > cap) throw new Error(`${n} blobs of smoke and the Rust module's table holds ${cap}: a game keeps no more than that (sense.ts CpuSmoke.emit), so somebody else filled the list`); }
  blob(x: number, y0: number, z: number, r2: number, age: number, mass: number): void { const M = this.mirror, m = f64(), o = M.blobs + M.blobN++ * FACE.BLOB; m[o] = x; m[o + 1] = y0; m[o + 2] = z; m[o + 3] = r2; m[o + 4] = age; m[o + 5] = mass; }
  blobsAlong(a: Vec3, b: Vec3, sigma: number, spread: number, widest: number, rise: number, riseRate: number): number {
    const M = this.mirror; if (M.x.smoke_trans(M.slot, M.blobSet, M.blobN, a[0], a[1], a[2], b[0], b[1], b[2], sigma, spread, widest, rise, riseRate) === -1) { if (FELL.on) fell('smoke', 'the module handed the line back (a line too long to be one)'); return -1; }
    SENSED.smoke++; return f64()[M.scratch];
  }
  // ---- THE GAME'S OWN SMOKE ON THE COARSE GRID (game/sense.ts SENSE.grid, SmokeGrid; rust/sim/src/bricks.rs). No row of RUST: the grid has no TypeScript
  // twin, so where the module does not run, or the level's plan is larger than a grid stands on, the blobs answer as they did. The state is a part of a
  // block in the module (sim/state.ts: the game's own, or a tool's), so a step copies nothing: it names the block and this world's copy.
  /** the emitters handed over since the last step (x y z radius rate ttl each) join the list and `n` steps of a tenth of a second are taken, over the boxes and
   *  leaves as they stand. The copy is the grid's to stand up: where no row of RUST has it standing for this tick, it is made the boxes as they are for
   *  the step alone and let go again, with no question of the game's sent to it (so nobody who opens a tick has to know a grid is there). False = nothing
   *  was done (the module does not run here, more leaves than a list holds, or no grid stands on this level): the state is as it was, the emitters not
   *  taken.
   *  `wind`: the smoke under the open sky drifts by this vector (m/s; sense.ts SENSE.drift), each square metre of the plan by the share of it `share`
   *  gives (0 under a roof … 1 in the open) and by its rise with height (`low` of it at the ground, `gain` more by `fullAt` metres up), and
   *  spreads of its own by less there (`K.inWind` of its spread where the wind blows whole). None, or a plan of more squares than the table holds: still air, the step as it was. */
  gridStep(st: SimState, pending: readonly number[], n: number, K: { decay: number; spread: number; rise: number; riseRate: number; gone: number; inWind: number },
    glow: { K: { albedo: number; share: number; passes: number; sweep: number }; lamps: (col: StaticCollision) => boolean } | null = null,
    wind: GridWind | null = null): boolean {
    const M = this.ready();
    const mine = !M.live;
    if (mine) { if (simWhyNot()) return false; M.open(this.boxes, this.cells, this.tiltedAt, this.W, this.H, this.cs, this.dynamicBoxes, this.glassWalkable, this.glassPanes, false, false, false, 0); if (!M.live) return false; }
    try {
      const set = M.leaves(this.dynamicBoxes); if (set === -2) return false;
      // (THE LIGHT THAT COMES ROUND, rust/sim/src/glow.rs: the step sweeps it from the level's steady lamps, which `glow.lamps` writes as for a question
      // about the light. More lamps than a question carries: the field stands as it was.)
      let lamps = -1; if (glow) { M.lampN = 0; this.lampsOver(); if (glow.lamps(this)) lamps = M.lampN; }
      // (the emitters go in the scratch, written last: the step takes them once it knows a grid stands here, so one that does nothing has taken none.
      // A step carries as many as the scratch holds, 170, the LATEST of them: the grid's own list holds 64, and `pending` is kept to that many by
      // whoever fills it, sense.ts SmokeGrid.emit)
      const x = M.x, m = f64(), at = M.scratch, all = pending.length - pending.length % 6, e = Math.min(all, M.scratchLen - M.scratchLen % 6); for (let i = 0, from = all - e; i < e; i++) m[at + i] = pending[from + i];
      // (THE WIND, bricks.rs 3b: its share a square metre, row by row, two squares to a brick's side — the plan as the grid lays it, bricks.rs step)
      let wx = 0, wz = 0, low = 1, gain = 0, fullAt = 1;
      if (wind && (wind.x !== 0 || wind.z !== 0)) {
        const nx = 2 * Math.ceil(this.W * this.cs / 2), nz = 2 * Math.ceil(this.H * this.cs / 2);
        if (nx * nz <= M.windCap) { for (let z = 0, o = M.wind; z < nz; z++) for (let i = 0; i < nx; i++) m[o++] = wind.share(i + 0.5, z + 0.5); wx = wind.x; wz = wind.z; low = wind.low; gain = wind.gain; fullAt = wind.fullAt; }
      }
      const G = glow?.K, g = st.bind();
      return x.grid_step(g, st.stamp, M.slot, M.gridCopy(), set, n, e / 6, K.decay, K.spread, K.rise, K.riseRate, K.gone, lamps, G ? G.albedo : 0, G ? G.share : 0, G ? G.passes : 0, G ? G.sweep : 0, wx, wz, low, gain, fullAt, K.inWind) !== -1;
    } finally { if (mine) M.shut(); }
  }
  /** segmentBlocked as this file answers it (and the whole of it wherever the module does not: outside a tick, with RUST.lines off) */
  segmentBlockedHere(a: Vec3, b: Vec3, shrink: number, through: number, panes: { bi: number; t: number }[] | null): boolean {
    // (the CPU-frame lane, second pass: the same walk, the same boxes in the same order and lineBox's own statements over scalars — so every answer, every pane noted
    //  and every glassOpen asked is what it was — minus the work that could not change them: the direction and length derived once for the leaves and the statics,
    //  a grid cell scanned once per query (cellStamp: everything in a cell seen before is already stamped), a static's cos / sin kept by its exact yaw, and a box the
    //  segment provably cannot touch passed before the slab test (segClear). tools/qa/segment-ident.ts holds the old code beside this one over random segments.)
    const a0 = a[0], a1 = a[1], a2 = a[2]; const d0 = b[0] - a0, d1 = b[1] - a1, d2 = b[2] - a2;
    const len = hyp3(d0, d1, d2); if (len < 1e-4) return false;
    const il = 1 / len; const r0 = d0 * il, r1 = d1 * il, r2 = d2 * il;
    if (this.dynamicHit(a, r0, r1, r2, len, 0)) return true;
    // walk grid cells along the segment (coarse: sample every 0.5 m, collect candidate boxes)
    const id = ++this.stampId; const steps = Math.ceil(len / 0.5) + 1;
    if (this.boxes.length > this.trigYaw.length) this.growTrig();
    const cs = this.cs, W = this.W, H = this.H, cells = this.cells, stamp = this.stamp, boxes = this.boxes, tYaw = this.trigYaw, tCos = this.trigCos, tSin = this.trigSin;
    const skip = through | BoxFlag.NoShadow;   // (+ NoShadow: a static hidden at runtime by the destruction lane is nothing to sight or sound either)
    const pad = SEG_PAD + (shrink < 0 ? -shrink : 0); let rdv: Vec3 | null = null;
    const uu = r0 * r0 + r2 * r2, su = Math.sqrt(uu), lu = len * uu, e1 = r1 * len;   // (the segment on the floor plan, a + t · (r0, r2) for t in [0, len], and its rise)
    // WHICH CELLS: the old walk sampled the segment every half metre and scanned the nine cells round each sample — a generous net. A box in the way has a point on
    // the segment (or within a negative shrink of its ends), and that point's own cell holds the box (the cells were filled from its footprint), so the cells that
    // matter are the ones within `pad` of the segment on the floor plan; the rest of the net adds candidates but never an answer. For a segment wholly inside the
    // grid (past its edge a box is filed in the clamped border cells, away from where it is) those are found directly: column by column along its longer floor axis,
    // the rows its stretch across that column (widened by q > pad both ways) covers — a superset of every cell within pad of it, a fraction of the net. Off when the
    // panes are being listed (their order in the list is the scan's) and when the segment is upright or leaves the grid: those take the old walk, cell for cell in
    // its order. With the direct cells a pane's glassOpen question may be asked in another order or not at all (a wall found first) — it is a question
    // (glassbreak.ts openAt reads, writes nothing). A TILTED static is filed by its yawed footprint alone, so it can be in the way from a cell it is not filed in:
    // the few a level has are asked first, each exactly when the old walk would have met it (one of its half-metre samples within a cell of the box's filed cells).
    const lxz = Math.sqrt(d0 * d0 + d2 * d2), b0 = b[0], b2 = b[2], gx = W * cs - pad - 0.25, gz = H * cs - pad - 0.25, g0 = pad + 0.25;
    const near = panes === null && lxz > 1e-6 && pad < 0.5 && a0 > g0 && a0 < gx && b0 > g0 && b0 < gx && a2 > g0 && a2 < gz && b2 > g0 && b2 < gz;
    const list = this.segCells; let n = 0;
    if (near) {
      const tl = this.tiltedAt;
      for (let k = 0; k < tl.length; k += 5) {
        const bi = tl[k]; stamp[bi] = id;
        const bx = boxes[bi]; if (bx.flags & skip) continue;
        if (segClear(a0, a1, a2, r0, r1, r2, len, bx, pad)) continue;
        const x0 = tl[k + 1] - 1, x1 = tl[k + 2] + 1, z0 = tl[k + 3] - 1, z1 = tl[k + 4] + 1; let met = false;
        for (let i = 0; i <= steps; i++) {
          const t = (i / steps) * len; const x = Math.floor((a0 + r0 * t) / cs), z = Math.floor((a2 + r2 * t) / cs);
          if (x >= x0 && x <= x1 && z >= z0 && z <= z1) { met = true; break; }
        }
        if (!met) continue;
        const s = lineBox(a, rdv ??= [r0, r1, r2], bx); if (!s || s.depth > shrink) continue;
        if (s.t1 > shrink && s.t0 < len - shrink) {
          if (this.glassOpen && (bx.flags & BoxFlag.Glass)) { const te = Math.max(0, s.t0); if (this.glassOpen(bi, [a0 + r0 * te, a1 + r1 * te, a2 + r2 * te])) continue; }
          return true;
        }
      }
      const q = pad + 1e-3; const xMajor = (d0 < 0 ? -d0 : d0) >= (d2 < 0 ? -d2 : d2);
      const am = xMajor ? a0 : a2, bm = xMajor ? b0 : b2, an = xMajor ? a2 : a0, slope = xMajor ? d2 / d0 : d0 / d2;
      const lo = am < bm ? am : bm, hi = am < bm ? bm : am, M = xMajor ? W : H, N = xMajor ? H : W;
      let c0 = Math.floor((lo - q) / cs), c1 = Math.floor((hi + q) / cs); if (c0 < 0) c0 = 0; if (c1 > M - 1) c1 = M - 1;
      const fwd = bm >= am;   // (from a's end: a blocked line usually meets its wall early)
      for (let j = c0; j <= c1; j++) {
        const c = fwd ? j : c1 - (j - c0);
        let s0 = c * cs - q, s1 = (c + 1) * cs + q; if (s0 < lo) s0 = lo; if (s1 > hi) s1 = hi;
        const n0 = an + (s0 - am) * slope, n1 = an + (s1 - am) * slope;
        let q0 = Math.floor(((n0 < n1 ? n0 : n1) - q) / cs), q1 = Math.floor(((n0 < n1 ? n1 : n0) + q) / cs); if (q0 < 0) q0 = 0; if (q1 > N - 1) q1 = N - 1;
        if (xMajor) for (let r = q0; r <= q1; r++) list[n++] = r * W + c; else for (let r = q0; r <= q1; r++) list[n++] = c * W + r;
      }
    } else {
      const cstamp = this.cellStamp; let lx = NaN, lz = NaN;
      for (let i = 0; i <= steps; i++) {
        const t = (i / steps) * len; const x = Math.floor((a0 + r0 * t) / cs), z = Math.floor((a2 + r2 * t) / cs);
        if (x === lx && z === lz) continue; lx = x; lz = z;   // (the same cell as the last sample: its nine were scanned then)
        for (let dz = -1; dz <= 1; dz++) for (let dx = -1; dx <= 1; dx++) {
          const cx = x + dx, cz = z + dz; if (cx < 0 || cz < 0 || cx >= W || cz >= H) continue;
          const ci = cz * W + cx; if (cstamp[ci] === id) continue; cstamp[ci] = id;   // (a cell scanned once per query: everything in it is stamped the first time)
          list[n++] = ci;
        }
      }
    }
    for (let j = 0; j < n; j++) {
      const cell = cells[list[j]];
      {
        for (let k = 0; k < cell.length; k++) {
          const bi = cell[k]; if (stamp[bi] === id) continue; stamp[bi] = id;
          const bx = boxes[bi]; if (bx.flags & skip) continue;
          let t0 = -Infinity, t1 = Infinity, depth: number;
          if (bx.rot || bx.round === true || (bx.cut !== undefined && cutMet(bx))) {   // (CUT BOXES: a cut static takes lineBox, which holds the cut rule — the scalar copy below is the whole box's; ROUND THINGS: lineBox holds the round too)
            if (segClear(a0, a1, a2, r0, r1, r2, len, bx, pad)) continue; const s = lineBox(a, rdv ??= [r0, r1, r2], bx); if (!s) continue; t0 = s.t0; t1 = s.t1; depth = s.depth; }
          else {
            // nowhere near the segment — outside the box's height span, or its floor-plan line / stretch clear of the cylinder (|h0| + |h2|) that holds the box at any
            // yaw: lineBox would miss it or put its interval off both ends (a negative shrink is in the pad)
            const c = bx.c, h = bx.h; const hy = h[1] < 0 ? -h[1] : h[1], m1 = c[1] - a1;
            if (e1 > 0 ? (m1 - hy > e1 + pad || m1 + hy < -pad) : (m1 - hy > pad || m1 + hy < e1 - pad)) continue;
            const R = (h[0] < 0 ? -h[0] : h[0]) + (h[2] < 0 ? -h[2] : h[2]) + pad;
            const m0 = c[0] - a0, m2 = c[2] - a2; const cr = m0 * r2 - m2 * r0; if (cr * cr > R * R * uu) continue;
            const sa = m0 * r0 + m2 * r2, ea = R * su; if (sa < -ea || sa > lu + ea) continue;
            // lineBox over scalars: its statements in its order
            const yaw = bx.yaw; let co: number, sn: number;
            if (tYaw[bi] === yaw) { co = tCos[bi]; sn = tSin[bi]; } else { co = Math.cos(yaw); sn = Math.sin(yaw); tYaw[bi] = yaw; tCos[bi] = co; tSin[bi] = sn; }
            const qx = a0 - c[0], qz = a2 - c[2];
            const ox = co * qx - sn * qz, oy = a1 - c[1], oz = sn * qx + co * qz;
            const ex = co * r0 - sn * r2, ez = sn * r0 + co * r2;
            const h0 = h[0], h1 = h[1], h2 = h[2];
            if (Math.abs(ex) < 1e-12) { if (Math.abs(ox) > h0) continue; }
            else { let ta = (-h0 - ox) / ex, tb = (h0 - ox) / ex; if (ta > tb) { const tt = ta; ta = tb; tb = tt; } if (ta > t0) t0 = ta; if (tb < t1) t1 = tb; if (t0 > t1) continue; }
            if (Math.abs(r1) < 1e-12) { if (Math.abs(oy) > h1) continue; }
            else { let ta = (-h1 - oy) / r1, tb = (h1 - oy) / r1; if (ta > tb) { const tt = ta; ta = tb; tb = tt; } if (ta > t0) t0 = ta; if (tb < t1) t1 = tb; if (t0 > t1) continue; }
            if (Math.abs(ez) < 1e-12) { if (Math.abs(oz) > h2) continue; }
            else { let ta = (-h2 - oz) / ez, tb = (h2 - oz) / ez; if (ta > tb) { const tt = ta; ta = tb; tb = tt; } if (ta > t0) t0 = ta; if (tb < t1) t1 = tb; if (t0 > t1) continue; }
            depth = Math.min(h0 - Math.abs(ox), h1 - Math.abs(oy), h2 - Math.abs(oz));
          }
          if (depth > shrink) continue;                        // the origin is EMBEDDED in this box: not an occluder of its own contents (see above)
          if (t1 > shrink && t0 < len - shrink) {              // inside it past the first `shrink`, entered before the last: in the way …
            if (this.glassOpen && (bx.flags & BoxFlag.Glass)) { const te = Math.max(0, t0); if (this.glassOpen(bi, [a0 + r0 * te, a1 + r1 * te, a2 + r2 * te])) continue; }   // … unless it is a pane crossed where a cell has fallen out (GLASS BREAKING: sound, a step, a line of fire through the hole)
            if (panes && (bx.flags & BoxFlag.Glass)) { panes.push({ bi, t: Math.max(0, t0) }); continue; }   // (shotBlocked, GLASS PENETRATION: a pane in the way is noted — where the line enters it — and judged afterwards; everything else still ends the walk)
            return true;
          }
        }
      }
    }
    return false;
  }
  /** segmentBlocked with nothing passed through: the SOLID test (sound, movement, reach) — an intact pane blocks it. */
  solidBlocked(a: Vec3, b: Vec3, shrink = 0.02): boolean { return this.segmentBlocked(a, b, shrink, 0); }
  /** The line of FIRE a→b: is there anything a round would stop in — every static (a glass pane included: the round stars it and is spent in it, combat.ts
   *  fireWeapon → raycast) and every leaf. Loose furniture is not asked (concealment, not cover: fireWeapon lets the round through to whoever stands behind it).
   *  Today that is exactly the solid walk; it is named apart because it is a different QUESTION from a step or a sound's straight run — the guards' 'can I hit
   *  him from here' (guards.ts clearShot, the tactics' cover points) must never again be answered by the line of SIGHT, which passes glass: a man who sees Sam
   *  through the conference glazing planted on that line and put round after round into the pane. */
  shotBlocked(a: Vec3, b: Vec3, shrink = 0.02, energy?: number): boolean {
    const pass = this.firePass ? this.firePass() : 0;   // (WINDOW BARS: 0 unless that rule is live — then a standing bar is nothing to the line, as it is to the round)
    const P = this.glassPen; if (!P || !P.live()) return this.segmentBlocked(a, b, shrink, pass);   // (the switch off: the solid walk, as it was)
    // GLASS PENETRATION (game/glasspen.ts): the same walk, but the panes across the line are collected instead of ending it, and — nothing else being in the way —
    // the question becomes the ROUND's: has a round of `energy` (the asker's gun; the model's cautious default without one) enough for every pane in turn? One
    // rule with combat.ts fireWeapon, so a man shoots through a window exactly when his round would get through it.
    const panes = shotPanes; panes.length = 0;
    if (this.segmentBlocked(a, b, shrink, pass, panes)) return true;
    if (!panes.length) return false;
    panes.sort((x, y) => x.t - y.t);
    const d = v3.sub(b, a); return !P.passes(a, v3.scale(d, 1 / v3.len(d)), panes, energy);
  }
  /** GLASS PENETRATION (game/glasspen.ts installs it; null = rounds stop in glass, the game as it was): `live` = the switch as of now, `passes` = would a round of
   *  `energy` leaving `a` along unit `rd` get through these panes (static index + metres along the line to where it enters each, nearest first). */
  glassPen: { live(): boolean; passes(a: Vec3, rd: Vec3, panes: { bi: number; t: number }[], energy?: number): boolean } | null = null;
  /** WINDOW BARS (game/winbars.ts installs it; null = a bar is a wall to a line of fire, the game as it was): the flags of the statics a line of FIRE passes as of
   *  now — BoxFlag.Bar while that rule is live (a round that meets a standing bar breaks it and flies on: combat.ts fireWeapon), else 0. */
  firePass: (() => number) | null = null;

  /** Closest static hit along a ray (t, box index) or null. Solid: an intact glass pane IS hit (a round, a canister, a spark, a shin all meet it); a broken one is not. */
  raycast(ro: Vec3, rd: Vec3, tmax: number): { t: number; index: number; n: Vec3 } | null {
    if (!(tmax > 0)) return null;   // (A RAY OF NO LENGTH MEETS NOTHING, said once for both walks: a box is met where the ray ENTERS it, at t ≥ 0 and nearer than tmax, so none is; the walk below gave the same null, by a loop of no steps)
    if (!this.mirror.live && RUST.rays && this.lone()) { try { return this.raycastAsk(ro, rd, tmax); } finally { this.mirror.shut(); } }
    return this.raycastAsk(ro, rd, tmax);
  }
  private raycastAsk(ro: Vec3, rd: Vec3, tmax: number): { t: number; index: number; n: Vec3 } | null {
    const M = this.ready();
    if (!M.rays || !(tmax < 1e6)) { if (FELL.on && RUST.rays) fell('rays', M.rays ? 'a ray a thousand kilometres long or longer' : this.fellWhy()); return this.raycastHere(ro, rd, tmax); }
    // RUST.rays, inside a tick (rust/sim/src/lines.rs ray: the walk below, statement for statement, over the module's copy of the boxes and of whichever
    // leaves stand in dynamicBoxes now). A pane met where it is open is passed there (glass.rs, the game's own rule over the copy's panes), and the ray
    // flies on to what stands beyond, as the walk below does.
    const ask = this.paneAsk; if (ask < 0) { this.paneFell('rays', ask); return this.raycastHere(ro, rd, tmax); }
    const set = M.leaves(this.dynamicBoxes); if (set === -2) { if (FELL.on) fell('rays', 'the leaves do not fit a list'); return this.raycastHere(ro, rd, tmax); }
    const k = M.x.lines_ray(M.slot, ro[0], ro[1], ro[2], rd[0], rd[1], rd[2], tmax, set, ask);
    if (k === -2) { RAYS.host++; if (FELL.on) fell('rays', 'the module handed the ray back'); return this.raycastHere(ro, rd, tmax); }
    let hit: { t: number; index: number; n: Vec3 } | null = null;
    if (k >= 0) { const m = f64(), at = M.scratch; hit = { t: m[at], index: k, n: [m[at + 1], m[at + 2], m[at + 3]] }; }
    RAYS.n++;
    if (BOTH.on) { const t = this.raycastHere(ro, rd, tmax);
      this.compared(M, set, hit === null ? t === null : t !== null && t.index === hit.index && Object.is(t.t, hit.t) && Object.is(t.n[0], hit.n[0]) && Object.is(t.n[1], hit.n[1]) && Object.is(t.n[2], hit.n[2]), 'raycast', ro, rd, tmax, k); return t; }
    return hit;
  }
  /** raycast as this file answers it */
  raycastHere(ro: Vec3, rd: Vec3, tmax: number): { t: number; index: number; n: Vec3 } | null {
    let best = tmax, bi = -1; let bn: Vec3 = [0, 1, 0];
    this.dynamicBoxes.forEach((db, i) => { const hit = rayBox(ro, rd, db, best); if (hit && hit.t < best) { best = hit.t; bi = 1e6 + i; bn = hit.n; } });   // index ≥ 1e6: dynamic occluder
    const id = ++this.stampId; const steps = Math.ceil(tmax / 0.5) + 1;
    for (let i = 0; i <= steps; i++) {
      const t = (i / steps) * tmax; if (t > best + 1.5) break;
      const x = Math.floor((ro[0] + rd[0] * t) / this.cs), z = Math.floor((ro[2] + rd[2] * t) / this.cs);
      for (let dz = -1; dz <= 1; dz++) for (let dx = -1; dx <= 1; dx++) {
        const cx = x + dx, cz = z + dz; if (cx < 0 || cz < 0 || cx >= this.W || cz >= this.H) continue;
        const cell = this.cells[cz * this.W + cx];
        for (let k = 0; k < cell.length; k++) {
          const b = cell[k]; if (this.stamp[b] === id) continue; this.stamp[b] = id;
          if (this.boxes[b].flags & BoxFlag.NoShadow) continue;   // (a runtime-hidden static: see gather)
          const hit = rayBox(ro, rd, this.boxes[b], best); if (hit && hit.t < best) { if (this.glassOpen && (this.boxes[b].flags & BoxFlag.Glass) && this.glassOpen(b, v3.mad(ro, rd, hit.t))) continue; best = hit.t; bi = b; bn = hit.n; }   // (GLASS BREAKING: a round through the missing part of a pane flies on to what stands beyond) (rayBox answers null from INSIDE a box — boxes.ts: a round or a spark born inside a static leaves it freely, by that rule)
        }
      }
    }
    return bi >= 0 ? { t: best, index: bi, n: bn } : null;
  }
}

/** How a sound at `src` reaches a listener at `dst`. One policy for both the audio engine (apparent direction,
 *  distance, muffling) and the guards' hearing:
 *  - direct: unobstructed straight line (a jamb grazing one side ray muffles it a touch);
 *  - routed: walled off, but the walkable space (nav grid) connects them — it arrives from the first bend/doorway,
 *    at the path's length, muffled a little, or a lot if a closed door leaf sits across that route;
 *  - sealed / too far to search: a dull remnant through the mass.
 *  All fields are defined in every case, so consumers just read them. */
export interface Propagation {
  direct: boolean; routed: boolean;
  pathLen: number;      // metres travelled (always finite: straight, path length, or a padded straight distance when sealed)
  via: Vec3;            // the last point of the route still in sight of the listener — the doorway / corner it comes through (the source itself unless routed)
  apparent: Vec3;       // where it seems to come from: through `via`, pushed out to the travelled distance (the source itself unless routed)
  muffle: number;       // 0 clear .. 1 through solid mass → drives low-pass / gain
  carry: number;        // 0..1 multiplier on how far it is heard (AI hearing radii)
}
export class SoundPaths {
  private cache = new Map<string, { t: number; r: Propagation }>(); private now = 0;
  /** up while a picture asks (Collision.aside): no answer is kept, whatever the asker says — the cache is left as it was found */
  asking = false;
  constructor(private col: StaticCollision) {}
  tick(time: number) { this.now = time; if (this.cache.size > 512) this.cache.clear(); }
  /** the level changed under the cached routes (a wall holed, cover gone — game/wallbreak.ts): every answer is asked afresh */
  invalidate() { this.cache.clear(); }
  /** `keep` false = the answer is not kept for the next asker (a question that is not the simulation's own — game/cues.ts overheard — leaves the cache as it found it) */
  propagate(src: Vec3, dst: Vec3, keep = true): Propagation {
    if (PRICED.waits) throw new Error('a sound was asked for from inside the price of a search: its way round is a search, and the module has one search\'s tables (rust/sim/src/nav.rs find_priced)');   // (every time, not only when the answer is not remembered)
    return RUST.sound && !this.col.mirror.live ? this.col.window(() => this.propagateAsk(src, dst, keep)) : this.propagateAsk(src, dst, keep); }
  private propagateAsk(src: Vec3, dst: Vec3, keep: boolean): Propagation {
    const r = this.propagateThere(src, dst, keep, !BOTH.on); if (r === null) return this.propagateHere(src, dst, keep);
    if (!BOTH.on) return r;
    const t = this.propagateHere(src, dst, keep);
    this.col.senseCompared(this.col.senseSet(SENSES.sound), t === r || (t.direct === r.direct && t.routed === r.routed && Object.is(t.pathLen, r.pathLen) && Object.is(t.muffle, r.muffle) && Object.is(t.carry, r.carry) && same3(t.via, r.via) && same3(t.apparent, r.apparent)), 'sound', src, dst);
    return t;
  }
  /** RUST.sound — the same answer from the module (rust/sim/src/hear.rs), in two questions with this file's own memory of routed answers between them: what
   *  the straight lines say, and — walled off, near enough, and not remembered — the way round by the floor (rust/sim/src/nav.rs over the grid as it stands
   *  now). null = `propagateHere`'s, from the start (nothing was written). `write` false = the answer is not remembered (the check's second asker). */
  private propagateThere(src: Vec3, dst: Vec3, keep: boolean, write: boolean): Propagation | null {
    const col = this.col, set = col.senseSet(SENSES.sound); if (set === -2) return null;   // (senseSet has counted it)
    const M = col.mirror, x = M.x, ask = col.paneAsk; if (ask < 0) { col.paneFell('sound', ask); return null; }
    const c = x.hear_direct(M.slot, set, ask, src[0], src[1], src[2], dst[0], dst[1], dst[2]); if (c === -1) { if (FELL.on) fell('sound', 'the module handed the straight lines back (one too long to be a line)'); return null; }
    const straight = f64()[M.scratch]; if (c !== FACE.WALLED) SENSED.sound++;
    if (c < FACE.OVER) { let graze = 0; if (c & FACE.GRAZE_A) graze += 0.1; if (c & FACE.GRAZE_B) graze += 0.1; return { direct: true, routed: false, pathLen: straight, via: v3.copy(src), apparent: v3.copy(src), muffle: graze, carry: 1 }; }
    if (c === FACE.OVER) return { direct: true, routed: false, pathLen: straight, via: v3.copy(src), apparent: v3.copy(src), muffle: 0.15, carry: 0.9 };
    const sealed = (): Propagation => ({ direct: false, routed: false, pathLen: straight * 1.5, via: v3.copy(src), apparent: v3.copy(src), muffle: 1, carry: 0.25 });
    if (c === FACE.FAR) return sealed();
    const nav = col.nav, cs = nav.cell;
    const key = `${Math.floor(src[0] / cs)},${Math.floor(src[2] / cs)}>${Math.floor(dst[0] / cs)},${Math.floor(dst[2] / cs)}`;
    const hit = this.cache.get(key); if (hit && Math.abs(this.now - hit.t) < 0.5) return hit.r;
    if (!nav.there(M)) { if (FELL.on) fell('sound', 'a floor of more cells than the module\'s table'); return null; }
    const k = x.hear_route(M.slot, set, ask, BoxFlag.SoundSeal, nav.W, nav.H, cs, src[0], src[1], src[2], dst[0], dst[1], dst[2], straight); if (k === -1) { if (FELL.on) fell('sound', 'the module handed the way round back (a line too long to be one, a way of more waypoints than its table)'); return null; }
    let r: Propagation; SENSED.sound++;
    if (k === FACE.SEALED) r = sealed();
    else {
      const m = f64(), at = M.scratch, L = m[at], via: Vec3 = [m[at + 1], m[at + 2], m[at + 3]];
      const dir = v3.normalize([via[0] - dst[0], 0, via[2] - dst[2]]);
      const muffle = k === FACE.ROUTED_DOOR ? 0.85 : 0.35;
      r = { direct: false, routed: true, pathLen: L, via, apparent: [dst[0] + dir[0] * L, src[1], dst[2] + dir[2] * L], muffle, carry: 1 - muffle * 0.8 };
    }
    if (write && keep && !this.asking) this.cache.set(key, { t: this.now, r }); return r;
  }
  /** propagate as this file answers it (and the whole of it wherever the module does not) */
  propagateHere(src: Vec3, dst: Vec3, keep = true): Propagation {
    const a: Vec3 = [src[0], Math.max(src[1], 0.9), src[2]], b: Vec3 = [dst[0], Math.max(dst[1], 0.9), dst[2]];
    const straight = v3.dist(a, b);
    // (solidBlocked throughout: sound does not pass a glass pane — it goes round through the door like round any wall, or through the opening once the pane is broken)
    if (!this.col.solidBlocked(a, b)) {
      // clear line; rays offset half a metre sideways catch a jamb / corner partly in the way and soften the transition at doorways
      let graze = 0;
      if (straight > 1) { const d = v3.sub(b, a); const side = v3.normalize([-d[2], 0, d[0]]); if (this.col.solidBlocked(v3.mad(a, side, 0.5), b)) graze += 0.1; if (this.col.solidBlocked(v3.mad(a, side, -0.5), b)) graze += 0.1; }
      return { direct: true, routed: false, pathLen: straight, via: v3.copy(src), apparent: v3.copy(src), muffle: graze, carry: 1 };
    }
    // blocked at chest height but clear at 2 m: a counter, cubicle partition or cabinet is in the way, not a wall — sound goes
    // straight over the top, barely dulled (without this, a step behind the reception desk 'routed' 9 m around and read as silence)
    const a2: Vec3 = [src[0], 2.0, src[2]], b2: Vec3 = [dst[0], 2.0, dst[2]];
    if (!this.col.solidBlocked(a2, b2)) return { direct: true, routed: false, pathLen: straight, via: v3.copy(src), apparent: v3.copy(src), muffle: 0.15, carry: 0.9 };
    const sealed = (): Propagation => ({ direct: false, routed: false, pathLen: straight * 1.5, via: v3.copy(src), apparent: v3.copy(src), muffle: 1, carry: 0.25 });
    if (straight > 28) return sealed();                    // far and walled off: not worth a search
    const nav = this.col.nav; const cs = nav.cell;
    const key = `${Math.floor(src[0] / cs)},${Math.floor(src[2] / cs)}>${Math.floor(dst[0] / cs)},${Math.floor(dst[2] / cs)}`;
    const hit = this.cache.get(key); if (hit && Math.abs(this.now - hit.t) < 0.5) return hit.r;
    const path = nav.findPath(dst, src, 6000);            // listener → source: the first legs tell us where it comes from
    let r: Propagation;
    if (!path || !path.length) r = sealed();
    else {
      let L = 0; let prev: Vec3 = [dst[0], 0, dst[2]];
      let via: Vec3 = [path[0][0], 1.0, path[0][2]];                                  // farthest waypoint the listener can still see: the corner / doorway it spills out of
      let doorInWay = false; let pa: Vec3 = [dst[0], 1.0, dst[2]]; let sightLost = false;
      for (const q of path) {
        L += v3.distXZ(q, prev); prev = q;
        const pb: Vec3 = [q[0], 1.0, q[2]];
        if (!sightLost) { if (!this.col.solidBlocked(b, pb)) via = pb; else sightLost = true; }   // ('sight' here is the sound's straight run to the listener: a pane in the way means it has not spilled out yet)
        if (!doorInWay && this.col.segmentBlockedDynamic(pa, pb, BoxFlag.SoundSeal)) doorInWay = true;
        pa = pb;
      }
      const lastLeg = v3.distXZ(src, prev); L += lastLeg; L = Math.max(L, straight);
      // the last leg stops short of the source so a sound emitted at a door leaf (creak, slam) is not muffled by its own leaf
      const s1: Vec3 = [src[0], 1.0, src[2]]; const end: Vec3 = lastLeg > 0.3 ? v3.mad(s1, v3.normalize(v3.sub(pa, s1)), 0.25) : pa;
      if (!doorInWay && this.col.segmentBlockedDynamic(pa, end, BoxFlag.SoundSeal)) doorInWay = true;   // a (near-)closed leaf across any leg muffles it a lot
      const dir = v3.normalize([via[0] - dst[0], 0, via[2] - dst[2]]);
      const muffle = doorInWay ? 0.85 : 0.35;
      r = { direct: false, routed: true, pathLen: L, via, apparent: [dst[0] + dir[0] * L, src[1], dst[2] + dir[2] * L], muffle, carry: 1 - muffle * 0.8 };
    }
    if (keep && !this.asking) this.cache.set(key, { t: this.now, r }); return r;
  }
}

/** 2D navigation grid (0.5 m) over the level for guard path finding. */
export class NavGrid {
  cell = 0.5; W: number; H: number; blocked: Uint8Array;
  /** findPath's tables, blank between searches (see there) */
  private scratch: NavScratch | null = null;
  constructor(private col: StaticCollision) {
    this.W = Math.round(WORLD.gridW * WORLD.gridCell / this.cell); this.H = Math.round(WORLD.gridH * WORLD.gridCell / this.cell);
    this.blocked = new Uint8Array(this.W * this.H);
    const p: Vec3 = [0, 0, 0];
    col.window(() => {   // (a circle a cell, thousands of them and nothing written between: one proof of the module's copy for the lot — StaticCollision.window)
    for (let z = 0; z < this.H; z++) for (let x = 0; x < this.W; x++) {
      p[0] = (x + 0.5) * this.cell; p[1] = 0; p[2] = (z + 0.5) * this.cell;
      const q: Vec3 = [p[0], 0, p[2]];
      // blocked if a guard-sized circle at the cell center overlaps geometry between knee and shoulder height
      // clearance must be >= the guards' movement collision radius (0.3) or steering can dead-lock on prop corners the grid calls free
      if (col.collideCircle(q, 0.34, 0.25, 1.5, 1)) { const moved = v3.distXZ(q, p); if (moved > 0.005) this.blocked[z * this.W + x] = 1; }
    }
    });
  }
  /** WALL BREAKING (game/wallbreak.ts): the level changed inside the world rectangle [x0, x1] × [z0, z1] — re-ask the bake's own question (the constructor's
   *  statement, cell for cell) of every nav cell whose centre lies within the agent's clearance of it. Returns the cells that changed as [index, nowBlocked]
   *  pairs flattened (opened = 0, closed = 1), so the caller can tell the AI which floor appeared. */
  rebake(x0: number, z0: number, x1: number, z1: number): number[] {
    const pad = 0.34 + this.cell; const changed: number[] = []; const p: Vec3 = [0, 0, 0];
    const cx0 = clampi(Math.floor((x0 - pad) / this.cell), 0, this.W - 1), cx1 = clampi(Math.floor((x1 + pad) / this.cell), 0, this.W - 1);
    const cz0 = clampi(Math.floor((z0 - pad) / this.cell), 0, this.H - 1), cz1 = clampi(Math.floor((z1 + pad) / this.cell), 0, this.H - 1);
    for (let z = cz0; z <= cz1; z++) for (let x = cx0; x <= cx1; x++) {
      p[0] = (x + 0.5) * this.cell; p[1] = 0; p[2] = (z + 0.5) * this.cell;
      const q: Vec3 = [p[0], 0, p[2]];
      const b = this.col.collideCircle(q, 0.34, 0.25, 1.5, 1) && v3.distXZ(q, p) > 0.005 ? 1 : 0;
      const i = z * this.W + x; if (this.blocked[i] !== b) { this.blocked[i] = b; changed.push(i, b); }
    }
    return changed;
  }
  idx(x: number, z: number) { return z * this.W + x; }
  /** WHAT SHUTS A CELL: the statics the bake's own question meets at the cell (wx, wz) lies in — the constructor's circle and heights, to the letter (keep the
   *  three numbers with its line). Count into `out`, -1 = `out` too short. A caller asks it to judge a cell as it WOULD be with some of them gone. */
  cellBlockers(wx: number, wz: number, out: Int32Array): number { const x = Math.floor(wx / this.cell), z = Math.floor(wz / this.cell); if (x < 0 || z < 0 || x >= this.W || z >= this.H) return 0; return this.col.circleTouches((Math.floor(wx / this.cell) + 0.5) * this.cell, (Math.floor(wz / this.cell) + 0.5) * this.cell, 0.34, 0.25, 1.5, out); }
  isBlocked(wx: number, wz: number) { const x = Math.floor(wx / this.cell), z = Math.floor(wz / this.cell); if (x < 0 || z < 0 || x >= this.W || z >= this.H) return true; return this.blocked[this.idx(x, z)] === 1; }

  /** findPath for a PICTURE's question (game/attention.ts attentionWay, asked by an overlay): the same search on the ASKER'S tables (`own.s`, made at its first
   *  search and kept by it), the grid's own left as they were — so a game that is drawn and the same game not drawn hold the same state, word for word. */
  findPathAside(own: { s?: object }, a: Vec3, b: Vec3, maxIter = 6000): Vec3[] | null { return this.aside(own, () => this.findPath(a, b, maxIter)); }
  /** `fn` with every search of it on the asker's own tables (`own.s`, kept between asks) and none of the grid's: what a picture asks writes nothing of the game's */
  aside<T>(own: { s?: object }, fn: () => T): T {
    const mine = this.scratch; this.scratch = (own.s as NavScratch | undefined) ?? null;
    try { return fn(); } finally { own.s = this.scratch ?? undefined; this.scratch = mine; }
  }
  /** A* from a to b (world coords). Returns world-space waypoints (cell centers), smoothed by greedy line-of-walk.
   *  `penalty` (optional): a cost MULTIPLIER for stepping INTO cell (ix, iz) — 1 = plain floor, 6 = six times dearer to walk through, 20 = all but a wall
   *  (values under 1 are read as 1, so the straight-line heuristic stays admissible). It bends the route round places a man has reason to avoid without
   *  forbidding them: a doorway his side has just seen become a kill zone, the corridor a colleague is already pressing up (game/tactics.ts — the k-th
   *  alternative route is simply A* again with the earlier routes' cells made dear). The smoothing then never straightens a leg back ACROSS dear cells the
   *  search walked round (a merge is only taken if it crosses no more penalised cells than the raw cells it replaces). Absent, nothing here differs by one
   *  instruction from the search it always was. */
  findPath(a: Vec3, b: Vec3, maxIter = 6000, penalty?: (ix: number, iz: number) => number): Vec3[] | null {
    if (PRICED.waits) throw new Error('a way was asked for from inside the price of another search: the module has one search\'s tables (rust/sim/src/nav.rs find_priced)');
    return RUST.paths && !this.col.mirror.live ? this.col.window(() => this.findPathAsk(a, b, maxIter, penalty)) : this.findPathAsk(a, b, maxIter, penalty);
  }
  private findPathAsk(a: Vec3, b: Vec3, maxIter: number, penalty?: (ix: number, iz: number) => number): Vec3[] | null {
    if (!BOTH.on || !penalty) {
      const r = this.findPathThere(a, b, maxIter, penalty); if (r === undefined) return this.findPathHere(a, b, maxIter, penalty);
      if (!BOTH.on) return r;
      const t = this.findPathHere(a, b, maxIter); let ok = (t === null) === (r === null) && (t === null || t.length === r!.length);
      for (let i = 0; ok && t && i < t.length; i++) ok = same3(t[i], r![i]);
      this.col.senseCompared(this.col.senseSet(SENSES.paths), ok, 'path', a, b);
      return t;
    }
    // (the check's, with a penalty. The module asks the caller's function ONCE a cell; this file's search asks it at EVERY visit, as it does with the row
    //  off. What is compared: the way; the cells each side priced, in the order it first asked for them (the module must come to need exactly the cells
    //  this file's search reads, when it reads them); and EVERY price this file's search was given against the one the module was given for that cell,
    //  which is the rule the module's once-a-cell rests on: a penalty gives a cell one price all through a search.)
    const W = this.W, first = new Map<number, number>(), there: number[] = [], here: number[] = [], met = new Set<number>(); let onePrice = true;
    const r = this.findPathThere(a, b, maxIter, (ix, iz) => { const i = iz * W + ix, v = penalty(ix, iz); there.push(i); first.set(i, v); return v; });
    const t = this.findPathHere(a, b, maxIter, (ix, iz) => { const i = iz * W + ix, v = penalty(ix, iz), was = first.get(i);
      if (!met.has(i)) { met.add(i); here.push(i); }
      if (was === undefined) first.set(i, v); else if (!Object.is(was, v)) onePrice = false;
      return v; });
    if (r === undefined) return t;
    let ok = onePrice && (t === null) === (r === null) && (t === null || t.length === r!.length) && here.length === there.length;
    for (let i = 0; ok && t && i < t.length; i++) ok = same3(t[i], r![i]);
    for (let i = 0; ok && i < here.length; i++) ok = here[i] === there[i];
    this.col.senseCompared(this.col.senseSet(SENSES.paths), ok, onePrice ? 'priced path' : 'priced path (the penalty gave one cell two prices in one search)', a, b);
    return t;
  }
  /** THE GRID AS IT STANDS, WRITTEN TO THE MODULE for one question of its search (rust/sim/src/nav.rs). False = it is larger than the module's table. */
  there(M: BoxMirror): boolean { if (this.blocked.length > M.navCap) return false; u8().set(this.blocked, M.navShut); return true; }
  /** RUST.paths — findPath from the module: the same search and smoothing over the grid as it stands now. undefined = `findPathHere`'s.
   *  WITH A PENALTY the module's search stops wherever it comes to a cell it has no price for and says which (eight at most at a step of the search; a
   *  leg's cells in the smoothing); `priceThese` asks the caller's function for exactly those, once each, and the search goes on. So the function is
   *  called for the cells this file's own search reads, in the order it first reads them (ONCE a cell where this file's reads it at every visit: every
   *  penalty in the game gives a cell the same price all through one search), and the module calls nothing. A search that waits is scratch of this one
   *  call: it is resumed to its end here, or dropped if the price throws. */
  private findPathThere(a: Vec3, b: Vec3, maxIter: number, penalty?: (ix: number, iz: number) => number): Vec3[] | null | undefined {
    const col = this.col, set = col.senseSet(SENSES.paths); if (set === -2) return undefined;   // (senseSet has counted it)
    if ((maxIter >>> 0) !== maxIter) { if (FELL.on) fell('paths', 'a cap on the search that is no whole number'); return undefined; }
    const M = col.mirror; if (!this.there(M)) { if (FELL.on) fell('paths', 'a floor of more cells than the module\'s table'); return undefined; }
    const x = M.x, ask = col.paneAsk; if (ask < 0) { col.paneFell('paths', ask); return undefined; }
    let n: number;
    if (!penalty) n = x.nav_path(M.slot, set, ask, this.W, this.H, this.cell, a[0], a[2], b[0], b[2], maxIter);
    else {
      n = x.nav_priced(M.slot, set, ask, this.W, this.H, this.cell, a[0], a[2], b[0], b[2], maxIter);
      if (n === NAV.NEED) { PRICED.waits = true;
        try { while (n === NAV.NEED) { PRICED.needs++; this.priceThese(M, x.nav_need(), penalty); n = x.nav_resume(); } }
        finally { PRICED.waits = false; if (n === NAV.NEED) x.nav_drop(); } }
      if (n !== NAV.ASK_HOST) PRICED.n++;
    }
    if (n === NAV.ASK_HOST) { if (FELL.on) fell('paths', 'the module handed the search back (a line too long to be one, a point that is no number, a way of more waypoints than its table)'); return undefined; }
    SENSED.paths++; if (n === NAV.NO_WAY) return null;
    const m = f64(), out: Vec3[] = []; for (let i = 0, o = M.way; i < n; i++, o += 2) out.push([m[o], 0, m[o + 1]]);
    return out;
  }
  /** the prices the module's search has asked for (`k` cells, listed at the module's own table), from the caller's function, in the order asked */
  priceThese(M: BoxMirror, k: number, penalty: (ix: number, iz: number) => number): void {
    const W = this.W, want = M.navWant, price = M.navPrice;
    for (let i = 0; i < k; i++) { const c = u32()[want + i]; const v = penalty(c % W, (c / W) | 0); f64()[price + c] = v; }   // (the views are taken anew at every cell: the function may ask the module questions of its own)
    PRICED.cells += k;
  }
  /** findPath as this file answers it (and the whole of it wherever the module does not) */
  findPathHere(a: Vec3, b: Vec3, maxIter = 6000, penalty?: (ix: number, iz: number) => number): Vec3[] | null {
    const W = this.W, H = this.H;
    const sx = clampi(Math.floor(a[0] / this.cell), 0, W - 1), sz = clampi(Math.floor(a[2] / this.cell), 0, H - 1);
    let gx = clampi(Math.floor(b[0] / this.cell), 0, W - 1), gz = clampi(Math.floor(b[2] / this.cell), 0, H - 1);
    if (this.blocked[this.idx(gx, gz)]) { const alt = this.nearestFree(gx, gz, b[0], b[2]); if (!alt) return null; gx = alt[0]; gz = alt[1]; }   // (a goal by a wall resolves to ITS side of the wall — see nearestFree)
    const start = this.idx(sx, sz), goal = this.idx(gx, gz);
    if (start === goal) return [[(gx + 0.5) * this.cell, 0, (gz + 0.5) * this.cell]];
    // (the search's four per-cell tables are the grid's own, kept between calls in their blank state — every cell a search wrote is listed in `seen` and put
    // back as it ends — and the open list is a typed array with its own count: the same values read and written in the same order as when each call made and
    // filled five fresh arrays the size of the level, so the same pops, the same ties, the same path. A search inside a search (a penalty that asks for a
    // path) or one after a penalty threw takes fresh tables, as it always did: that is this file's search alone. While the MODULE's priced search waits for
    // prices, a way or a sound asked from inside a price is an error, `findPath` and `propagate` say so, and no penalty in the game asks one.)
    let S = this.scratch; if (!S || S.busy || S.g.length !== W * H) { const fresh = navScratch(W * H); if (!S || !S.busy) this.scratch = fresh; S = fresh; }   // (nothing of the LEVEL is kept: `blocked` is read afresh by every search, and a grid of another size takes new tables)
    S.busy = true;
    const gScore = S.g, came = S.came, closed = S.closed, f = S.f, blocked = this.blocked; let open = S.open, seen = S.seen, nOpen = 0, nSeen = 0;
    { const x = start % W, z = (start / W) | 0; const dx = Math.abs(x - gx), dz = Math.abs(z - gz); gScore[start] = 0; f[start] = (dx + dz) + (Math.SQRT2 - 2) * Math.min(dx, dz); }
    open[nOpen++] = start; seen[nSeen++] = start;
    let iter = 0;
    while (nOpen && iter++ < maxIter) {
      // pop min f (linear scan; grids are small)
      let mi = 0, mf = f[open[0]]; for (let i = 1; i < nOpen; i++) { const fi = f[open[i]]; if (fi < mf) { mi = i; mf = fi; } }
      const cur = open[mi]; open[mi] = open[--nOpen];
      if (cur === goal) break;
      if (closed[cur]) continue; closed[cur] = 1;
      const cx = cur % W, cz = (cur / W) | 0;
      if (nOpen + 8 > open.length) { const o = new Int32Array(open.length * 2); o.set(open); open = S.open = o; }
      if (nSeen + 8 > seen.length) { const o = new Int32Array(seen.length * 2); o.set(seen); seen = S.seen = o; }
      for (let dz = -1; dz <= 1; dz++) for (let dx = -1; dx <= 1; dx++) {
        if (!dx && !dz) continue; const nx = cx + dx, nz = cz + dz;
        if (nx < 0 || nz < 0 || nx >= W || nz >= H) continue;
        const ni = nz * W + nx; if (blocked[ni] || closed[ni]) continue;
        if (dx && dz && (blocked[cz * W + cx + dx] || blocked[(cz + dz) * W + cx])) continue; // no corner cutting
        const step = dx && dz ? Math.SQRT2 : 1;
        const ng = gScore[cur] + (penalty ? step * Math.max(1, penalty(nx, nz)) : step);
        if (ng < gScore[ni]) { const hx = Math.abs(nx - gx), hz = Math.abs(nz - gz); gScore[ni] = ng; came[ni] = cur; f[ni] = ng + ((hx + hz) + (Math.SQRT2 - 2) * Math.min(hx, hz)); open[nOpen++] = ni; seen[nSeen++] = ni; }
      }
    }
    const cells: number[] = []; const found = came[goal] >= 0; if (found) for (let c = goal; c >= 0 && c !== start; c = came[c]) cells.push(c);
    for (let i = 0; i < nSeen; i++) { const c = seen[i]; gScore[c] = Infinity; f[c] = Infinity; came[c] = -1; closed[c] = 0; }
    S.busy = false;
    if (!found) return null;
    cells.reverse();
    // greedy smoothing (a raw cell's centre is made when it is asked for: the test point in one array of the call's own, a Vec3 of its own only for a waypoint kept)
    const n = cells.length, q: Vec3 = [0, 0, 0];
    const dear = penalty ? cells.map(c => penalty(c % W, (c / W) | 0) > 1) : null;   // (with a penalty: which raw cells the search paid extra for — a merged leg may not cross more dear cells than the ones it stands in for)
    const out: Vec3[] = []; let anchor: Vec3 = [a[0], 0, a[2]]; let i = 0;
    while (i < n) {
      let j = n - 1;
      for (; j > i; j--) { const c = cells[j]; q[0] = ((c % W) + 0.5) * this.cell; q[2] = (((c / W) | 0) + 0.5) * this.cell; if (this.walkable(anchor, q) && (!dear || this.dearAlong(anchor, q, penalty!) <= countTrue(dear, i, j))) break; }
      const c = cells[j], p: Vec3 = [((c % W) + 0.5) * this.cell, 0, (((c / W) | 0) + 0.5) * this.cell];
      out.push(p); anchor = p; i = j + 1;
    }
    return out;
  }
  /** distinct cells along the straight leg a → b (sampled at half a cell) whose penalty is above plain floor — the smoothing's guard against cutting back across them */
  private dearAlong(a: Vec3, b: Vec3, penalty: (ix: number, iz: number) => number): number {
    const dx = b[0] - a[0], dz = b[2] - a[2]; const n = Math.ceil(hyp2(dx, dz) / (this.cell * 0.5)) + 1;
    let last = -1, count = 0;
    for (let k = 0; k <= n; k++) {
      const t = k / n; const ix = Math.floor((a[0] + dx * t) / this.cell), iz = Math.floor((a[2] + dz * t) / this.cell); const i = this.idx(ix, iz);
      if (i === last) continue; last = i;
      if (ix >= 0 && iz >= 0 && ix < this.W && iz < this.H && penalty(ix, iz) > 1) count++;
    }
    return count;
  }

  /** The free cell nearest (by ring) to the blocked cell (gx, gz). With the world point (wx, wz) that asked: among the free cells of the winning ring (and, failing
   *  that, the next), the first one the point reaches in a STRAIGHT clear line at waist height — its own side of whatever wall or partition put it in the blocked band.
   *  A sound made a hand off a wall (its cell is in the 0.34 m clearance band the bake blocks along every wall) used to resolve to whichever free cell the ring met
   *  first in scan order: 36 of 202 such points over the office landed in the room BEHIND the wall, and the guards' hearing then measured its path — and heard it
   *  come — from the wrong room; a man's goal by a wall could send him round into the next office the same way. No clear candidate in those two rings (a point buried
   *  in a solid has no side) = the ring order's first, exactly as before. */
  private nearestFree(gx: number, gz: number, wx?: number, wz?: number): [number, number] | null {
    let fallback: [number, number] | null = null; let extra = 1;   // rings searched past the winning one for a same-side cell
    const from: Vec3 | null = wx === undefined || wz === undefined ? null : [wx, 1.0, wz];
    for (let r = 1; r < 8; r++) {
      for (let dz = -r; dz <= r; dz++) for (let dx = -r; dx <= r; dx++) {
        if (Math.max(Math.abs(dx), Math.abs(dz)) !== r) continue; const x = gx + dx, z = gz + dz;
        if (x < 0 || z < 0 || x >= this.W || z >= this.H || this.blocked[this.idx(x, z)]) continue;
        if (!from) return [x, z];
        fallback ??= [x, z];
        if (!this.col.solidBlocked(from, [(x + 0.5) * this.cell, 1.0, (z + 0.5) * this.cell])) return [x, z];
      }
      if (fallback && extra-- <= 0) return fallback;
    }
    return fallback;
  }
  /** Centre of the walkable cell nearest (by ring) to a world point — the point's own cell if it is free — or null if nothing within ~3.5 m is. What a body put
   *  down by hand (a teleport) stands on when the asked-for spot is inside something; unlike findPath(p, p) it does not need a way OUT of the blocked cell. */
  nearestFreePoint(wx: number, wz: number): Vec3 | null {
    const x = clampi(Math.floor(wx / this.cell), 0, this.W - 1), z = clampi(Math.floor(wz / this.cell), 0, this.H - 1);
    const c = this.blocked[this.idx(x, z)] ? this.nearestFree(x, z, wx, wz) : [x, z] as [number, number];
    return c ? [(c[0] + 0.5) * this.cell, 0, (c[1] + 0.5) * this.cell] : null;
  }

  /** Conservative straight-line walkability (samples cells along the segment with the agent radius). */
  walkable(a: Vec3, b: Vec3): boolean {
    const dx = b[0] - a[0], dz = b[2] - a[2]; const len = hyp2(dx, dz); const n = Math.ceil(len / (this.cell * 0.5)) + 1;
    const px = -dz / (len || 1) * 0.22, pz = dx / (len || 1) * 0.22;
    for (let i = 0; i <= n; i++) {
      const t = i / n; const x = a[0] + dx * t, z = a[2] + dz * t;
      if (this.isBlocked(x, z) || this.isBlocked(x + px, z + pz) || this.isBlocked(x - px, z - pz)) return false;
    }
    return true;
  }
}

function clampi(x: number, a: number, b: number) { return x < a ? a : x > b ? b : x; }
/** two points, the same to the bit */
function same3(a: Vec3, b: Vec3): boolean { return Object.is(a[0], b[0]) && Object.is(a[1], b[1]) && Object.is(a[2], b[2]); }
interface NavScratch { busy: boolean; g: Float32Array; f: Float32Array; came: Int32Array; closed: Uint8Array; open: Int32Array; seen: Int32Array }
function navScratch(n: number): NavScratch { return { busy: false, g: new Float32Array(n).fill(Infinity), f: new Float32Array(n).fill(Infinity), came: new Int32Array(n).fill(-1), closed: new Uint8Array(n), open: new Int32Array(1024), seen: new Int32Array(1024) }; }
/** how many of flags[i‥j] are set (findPath's smoothing: the dear raw cells a merged leg would replace) */
function countTrue(flags: boolean[], i: number, j: number): number { let n = 0; for (let k = i; k <= j; k++) if (flags[k]) n++; return n; }
