// THE GAME SENSES FOR ITSELF: how lit a point is and how much smoke is on a line are worked out by the simulation, the same on a page, a server and a bare
// machine (rust/game_rules/src/sense, guards; rust/sim/src/bricks.rs, glow.rs, light.rs). The picture is asked nothing. What is here is the table of the
// model's numbers, and what the page's own tools still hold of it (the error tool's grids).
// (src/game is one import cycle: nothing here is built from another game module's export as the file loads.)
import type { Vec3 } from '../math/vec';
import type { SmokeEmitter, SmokeSystemLike } from '../smoke/types';
import { type StaticCollision, type GridWind } from '../scene/collision';
import { FACE } from '../scene/boxMirror';
import { sim, simWhyNot } from '../sim/sim';
import { simPart, P_SMOKE, SimState } from '../sim/state';
import type { Game } from './game';
import { RUST_PASS, RUST_SURFACE, goneWithTick } from './crossing';

export const SENSE = {
  /** the light under a roof (glow.rs): the steps a sweep of the whole plan takes (a step is a tenth of a second) */
  glow: { sweep: 32 },
  smoke: {
    /** extinction per metre per unit of density (the renderer's: σt = 6/m per unit) */
    sigma: 6,
    /** what the smoke loses a second (the solver's densDecay) */
    decay: 0.14,
    /** how fast a blob widens: its width² grows by 2·spread·age (m²/s), up to `widest` metres */
    spread: 0.2, widest: 3.5,
    /** how far a blob's middle rises above where it was let out (m), and how fast (m/s) */
    rise: 1, riseRate: 0.3,
    /** how much of its own spread SIDEWAYS the grid's smoke keeps where the wind blows whole (`drift`; bricks.rs step 4; upward it keeps none: the
     *  wind is flat). With none of it the smoke cannot get round what the wind pushes it against, and a plume never widens */
    inWind: 0.15,
    /** an emitter that lets out less than this in its whole life (density × volume, unit·m³) is no blob: a muzzle's wisp, a spark's trail hide nobody */
    least: 0.05,
    /** a blob thinner than this (unit·m³) after its emitter has stopped is gone */
    gone: 0.02,
    /** the most blobs kept (the one with the least in it and still to come goes first) — and never more than the Rust module's table holds, whatever
     *  is written here (boxMirror.ts FACE.MAX_BLOBS; `emit`) */
    cap: 48,
    /** the emitters a solver holds at once, and how many of those it keeps back from `budget` (smoke.ts MAX_EMITTERS, and its budget()'s reserve) */
    emitters: 64, reserve: 12,
  },
};

interface Blob { x: number; y0: number; z: number; r2: number; rate: number; ttl: number; age: number; mass: number; }
const PI15 = 5.568327996831708;   // π^1.5: a gaussian exp(−ρ²/r²) holds this much × r³

/** THE GAME'S OWN SMOKE ON A COARSE GRID (SENSE.grid; the model is rust/sim/src/bricks.rs, and its head says what it is). This is the game's side of it.
 *  THE STATE IS A PART OF THE GAME'S BLOCK IN THE MODULE (sim/state.ts; bricks.rs Smoke): the cells, the emitters, the book, the light that comes round.
 *  The snapshot carries the block, so nothing here is copied in or out, and a put-back, another game or a parked block is the block's business. What this
 *  object keeps is the host's half of the beat: the seconds owed toward the next step and the emitters handed over since the last one. The module is the
 *  only thing that steps it; where no grid stands on the level (its plan is larger than one stands on, more leaves than a list holds) `stands` is false
 *  and the blobs answer, as they did. A file checkpoint keeps no smoke in the air. */
export class SmokeGrid {
  /** seconds owed toward the next step (a step is a tenth of a second) */
  clock = 0;
  /** the emitters handed over since the last step: x y z, how wide (m), what each lets out a second (unit·m³), for how long */
  pending: number[] = [];
  /** did the last step find a grid standing on this level */
  stands = false;
  /** has anything been let out into it since it was last empty (a clear of an empty grid is nothing at all: a page's warm-up clears, a tape of it played
   *  again does not, and the two must step alike) */
  fed = false;
  /** `sim`: the block the state is a part of: the game's own (`game.sim`), or one of a tool's making (`make`). The object, never a view of it. */
  constructor(readonly sim: SimState) { }
  /** A GRID THAT IS NO GAME'S (a tool that measures, a check): in a block of its own, which takes one of the module's slots while it stands there, so
   *  whoever makes one gives it back (`free`). None where the module does not run. */
  static make(): SmokeGrid | null {
    if (simWhyNot()) return null;
    const st = new SimState(); sim().grid_reset(st.bind(), 1); return new SmokeGrid(st);
  }
  /** a tool's own grid is done with: its block is no more, and the slot it stood in is free (never a game's grid: that one goes with its switches) */
  free(): void { this.sim.put([]); this.clock = 0; this.pending.length = 0; this.stands = false; this.fed = false; }
  /** where the state row starts in the block, in doubles (after the part's own first word: bricks.rs ROW) */
  private get row0(): number { return (simPart(P_SMOKE) >>> 3) + 1; }
  /** one double of the state row, by its place in it (GRID): a read now, wherever the block stands */
  at(i: number): number { return this.sim.d[this.row0 + i]; }
  /** the state row as far as it is in use (the part's first word says: up to the end of the highest brick that stands), for a READ NOW by a check or a
   *  tool. Never kept: the block moves. */
  row(): Float64Array { const st = this.sim, o = simPart(P_SMOKE), live = st.w[o >>> 2]; return st.d.subarray(this.row0, this.row0 + (live > 8 ? (live - 8) >>> 3 : 0)); }
  /** is there smoke for a step to move: an emitter handed over and not yet taken, or a brick standing (the block's own word of how much of the row is
   *  in use: past the cells' start). Read off the game's state, so the same on every machine and in a game a snapshot was just put into. */
  get smoky(): boolean { return this.pending.length > 0 || this.sim.w[simPart(P_SMOKE) >>> 2] > 8 + GRID.rho * 8; }
  emit(x: number, y: number, z: number, radius: number, rate: number, ttl: number): void {
    const P = this.pending; if (P.length >= 6 * SENSE.smoke.emitters) P.splice(0, 6);   // (a game no step has reached for a long while keeps the latest)
    P.push(x, y, z, radius, rate, ttl); this.fed = true;
  }
  /** `dt` seconds of the game, inside its tick: the steps that have come due are taken (four at most: a game that stood still does not owe a hundred).
   *  `wind`: the smoke under the open sky drifts by it (SENSE.drift; wind.ts stormDrift). Asked only while there is smoke to move (`smoky`). */
  step(col: StaticCollision, dt: number, wind: (() => GridWind | null) | null = null): void {
    this.clock += dt; if (this.clock < GRID_STEP) return;
    let left = this.clock, n = 0; while (left >= GRID_STEP && n < 4) { left -= GRID_STEP; n++; }
    this.stands = col.gridStep(this.sim, this.pending, n, SENSE.smoke, wind && this.smoky ? wind() : null);
    if (this.stands) { this.clock = left >= GRID_STEP ? 0 : left; this.pending.length = 0; }
    else if (this.clock > 4 * GRID_STEP) this.clock = 4 * GRID_STEP;   // (nothing was done: the steps and the emitters wait, but no more of either than a call takes)
  }
  /** no smoke anywhere. The steps keep their beat (`clock` is the game's time, not the smoke's), and the light that comes round stays: it is the
   *  level's, not the smoke's. A NEW ENCOUNTER takes a new grid instead (`renew`), so a file loaded into a running game and into a fresh one go on alike */
  clear(): void { if (!this.fed) return; sim().grid_clear(this.sim.bind()); this.pending.length = 0; this.fed = false; }   // (the smoke and its book: the plan it stands on and the light that comes round are not smoke)
  /** A NEW GRID WHERE THIS ONE STOOD: no smoke, on no plan, nothing swept, and its beat and the sweep's turn start now (a new encounter, Game.restartEncounter;
   *  the senses come to the game) */
  renew(): void { sim().grid_reset(this.sim.bind(), 1); this.clock = 0; this.pending.length = 0; this.stands = false; this.fed = false; }
  /** how much of the light along a → b comes through (1 = clear air); −1 = no grid stands, or it holds more smoke than bricks (the caller's blobs answer).
   *  Read from the state alone, so it answers outside a tick too. */
  along(a: Vec3, b: Vec3): number { if (!this.stands) return -1; const st = this.sim, g = st.bind(); return sim().grid_trans(g, st.stamp, a[0], a[1], a[2], b[0], b[1], b[2], SENSE.smoke.sigma); }
}
/** THE GRID'S OWN NUMBERS as rust/sim/src/bricks.rs has them (tools/qa/smokegrid-check.ts reads that file and is red where one differs): seconds a step, and
 *  the state row's layout for whoever reads it: doubles in the head, emitters and doubles an emitter, bricks, cells a brick, m³ a cell, and where in the head
 *  the book of the smoke is kept (let out · given back with thin bricks, and what the wind carried over the plan's edge · refused for want of a brick) */
export const GRID = { step: 0.1, head: 8, emitters: 64, emit: 7, bricks: 512, cells: 128, vol: 0.125, out: 4, given: 5,
  /** …the plan in squares (the head), a square's side (m), and where the cells start, last of the row: after the light under a roof (glow.rs), five numbers
   *  a square metre and three more */
  nbx: 2, nbz: 3, side: 2, rho: 8 + 64 * 7 + 512 * 2 + 5 * 2048 + 3 };
/** THE LIGHT MODEL'S FIELD as the estimate of the light on a man takes it (guards.ts lightEstimateAt): a game's own smoke with a grid standing. The grid
 *  holds the light that comes round; the smoke answers what dims a lamp on its way (the grid's cells, and the blobs where the grid cannot answer). */
export type LightField = CpuSmoke & { grid: SmokeGrid };
const GRID_STEP = GRID.step;

/** THE SMOKE AS THE GAME KNOWS IT (see the head): plain numbers, stepped with the game, nothing of the renderer's */
export class CpuSmoke {
  blobs: Blob[] = [];
  /** the same smoke on the coarse grid, where SENSE.grid has put one beside the blobs (as the game's tick opens: the tenant below SmokeGrid, by
   *  sim/state.ts settle; a tool puts its own): it is fed what the blobs are fed, and answers a sight line where it stands */
  grid: SmokeGrid | null = null;
  /** the seconds each emitter handed over has left to run, small or not: what `budget` counts */
  lives: number[] = [];
  /** an emitter handed to the solver: what it will let out, as one blob where it stands (a tracked source is taken where it was born) */
  emit(e: SmokeEmitter): void {
    if (e.look) return;   // (a look-only source — a man's breath in the cold, snowBreath.ts: it hides nobody and is no emitter `budget` counts)
    const K = SENSE.smoke, r = Math.max(e.radius, 0.05), rate = e.density * PI15 * r * r * r;
    if (e.ttl > 0) { this.lives.push(e.ttl); if (this.lives.length > 2 * K.emitters) this.lives.shift(); }
    if (!(rate * e.ttl >= K.least)) return;
    this.grid?.emit(e.pos[0], e.pos[1], e.pos[2], r, rate, e.ttl);
    if (this.blobs.length >= Math.min(K.cap, FACE.MAX_BLOBS)) {   // (no more than the module's table holds, whatever the knob says: a question about the smoke carries every blob at once — smoke.rs, and light.rs for a lamp's shadow) full: the one worth least goes — what it holds and what it has still to let out — and that may be the newcomer
      const worth = (b: Blob) => b.mass + b.rate * Math.max(0, b.ttl - b.age);
      let k = 0; for (let i = 1; i < this.blobs.length; i++) if (worth(this.blobs[i]) < worth(this.blobs[k])) k = i;
      if (worth(this.blobs[k]) >= rate * e.ttl) return;
      this.blobs.splice(k, 1);
    }
    this.blobs.push({ x: e.pos[0], y0: e.pos[1], z: e.pos[2], r2: r * r, rate, ttl: e.ttl, age: 0, mass: 0 });
  }
  step(dt: number): void {
    const K = SENSE.smoke, keep = Math.exp(-K.decay * dt);
    { const L = this.lives; let w = 0; for (let i = 0; i < L.length; i++) { const t = L[i] - dt; if (t > 0) L[w++] = t; } L.length = w; }
    for (let i = this.blobs.length - 1; i >= 0; i--) {
      const b = this.blobs[i];
      b.mass *= keep; if (b.age < b.ttl) b.mass += b.rate * Math.min(dt, b.ttl - b.age);
      b.age += dt;
      if (b.age >= b.ttl && b.mass < K.gone) this.blobs.splice(i, 1);
    }
  }
  clear(): void { this.blobs.length = 0; this.lives.length = 0; this.grid?.clear(); }
  /** HOW MUCH OF THE LIGHT ALONG a → b COMES THROUGH (1 = clear air), in the game's own box world: the grid's cells where a grid stands and can answer,
   *  else the blobs. */
  along(col: StaticCollision, a: Vec3, b: Vec3): number { return goneWithTick('CpuSmoke.along') as any; }
  /** …by THE BLOBS, whatever a grid would say (the error tool's own model of them; `along` where no grid answers). The Rust module's sum (rust/sim/src/
   *  smoke.rs) over these blobs, written there for the one question: each blob a gaussian that widens with its age and rises from where it was let out,
   *  its share of the line by the error function, and none of it counted through a wall — a wall between a blob's middle and the line's nearest point
   *  to it is anything a sight line stops at (the smoke of the next room is not in this one's sight lines). */
  blobsAlong(col: StaticCollision, a: Vec3, b: Vec3): number {
    if (!this.blobs.length) return 1;
    // (the blobs are one question in several calls: asked between ticks, the whole of it stands in one window over the module's copy, collision.ts window)
    return col.mirror.live ? this.blobsAsk(col, a, b) : col.window(() => this.blobsAsk(col, a, b));
  }
  private blobsAsk(col: StaticCollision, a: Vec3, b: Vec3): number {
    const B = this.blobs, K = SENSE.smoke; col.blobsBegin(B.length);
    for (let i = 0; i < B.length; i++) { const bl = B[i]; col.blob(bl.x, bl.y0, bl.z, bl.r2, bl.age, bl.mass); }
    return col.blobsAlong(a, b, K.sigma, K.spread, K.widest, K.rise, K.riseRate);
  }
}

/** THE GAME'S OWN SMOKE PUT IN FRONT OF THE SOLVER under SENSE.cpu: every emitter the solver is handed — by the game or by anything else that holds it (a panel
 *  button, the page's warm-up) — also feeds `own`, a clear clears both, and what the game asks back to decide how much more to emit is `own`'s answer. Done on the
 *  solver object itself, so nobody is left holding an unwatched one; the solver's own working is called as it was. Returns the undo. */
export function ownSmoke(real: SmokeSystemLike, own: CpuSmoke): () => void { return goneWithTick('ownSmoke') as any; }

/** a page's way to the Rust game by a call's name (host/rustgame.ts rustCall), handed in once by main.ts; null = no page: the call surface carries it */
export const SENSE_PAGE: { call: ((key: string, self: unknown, args: unknown[]) => unknown) | null } = { call: null };
/** FOR A TOOL THAT HOLDS THE SENSES TO THE PICTURE (host/modelerr.ts), between ticks: 0 the light on points (`xs`: whose carried lamps are left out, then
 *  x y z each) · 1 how much of the light along lines the smoke lets through (six numbers each) · 2 a canister's smoke let out at x y z, to the game and the
 *  picture alike · 3 the smoke cleared, the same */
export function senseTool(g: Game, what: 0 | 1 | 2 | 3, xs: number[]): number[] { if (SENSE_PAGE.call) return SENSE_PAGE.call('senseTool', undefined, [g, what, xs]) as number[]; { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('senseTool', undefined, [g, what, xs]); if (__r !== RUST_PASS) return __r as any; } } return goneWithTick('senseTool') as any; }
