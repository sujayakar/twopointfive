// THE SENSES ON THE CPU (`SENSE.cpu`; DEFAULT OFF — the page's `?cpusense`, the harness's `TPS_CPUSENSE=1`): everything the simulation takes off the renderer
// (sensors.ts: the light meters, the smoke along a sight line) worked out by the game itself instead, in the step that asks, from nothing but the game's own state.
// WHY: a value off the GPU arrives a drawn frame or two after it was asked and is the renderer's — so a fight on the page is its seed, its inputs AND a stream of
// readbacks (the run tape keeps them: a re-run needs the file). With this on, a fight is its seed and its inputs and nothing else: a machine with no GPU plays the
// very same one, which is what a server that will not trust its clients has to do. Engine groundwork; nothing networked.
//
// What stands in:
//   light   — nothing is asked of the probe and no answer is served, so every meter falls to the estimate it already had for 'no answer yet' (guards.ts
//             lightEstimateAt: the direct light of every lamp with the walls between, the hour's sun and sky, a stroke of lightning) — what the harness has always
//             run on. Sam's own meter, which had no stand-in (headless it never moved), reads the same estimate at his chest and head (game.ts lightMeter).
//             NOT in the estimate: bounced light, and smoke's own shadow. A place lit only by bounce reads darker to the men than the picture shows it.
//   the light model — `SENSE.bounce` (off): those two, on the coarse grid: a lamp is dimmed by the grid's smoke on the way, and the lamps' light off
//             the walls is kept a square metre of the plan and added to the estimate (rust/sim/src/glow.rs). Where no grid stands, the estimate above.
//   smoke   — `CpuSmoke` below: every emitter the game hands the solver also feeds a blob here (what was let out, spreading and thinning at the solver's own decay),
//             and a sight line's transmittance is the blobs' sum along it, answered in the step that asks. It is a MODEL of the solver's picture, not a copy: a
//             canister shuts a doorway and a room as the picture's does, the edges and the minute-old wisps will not match cell for cell.
//   smoke on a grid — `SENSE.grid` (off): the same emitters let out into half-metre cells that the walls and the doors hold (`SmokeGrid`; the cells are
//             stepped in the Rust module, rust/sim/src/bricks.rs), and a sight line is answered from the cells. Where no grid stands the blobs answer.
//   the solver's CPU-side fact the game reads to decide how much MORE to emit (`budget`: its emitter list ages by the drawn frame) is answered from a count of
//             the game's own, aged by the step — the page at any frame rate and a machine with no solver at all let out the same puffs.
//   A solver the picture has turned off or frozen takes no emitter; the game's own smoke does not ask: with this on, the fight is the same whatever is drawn.
// One model for presentation and AI is what this trades against: OFF, the men see by the renderer's own values; ON, by the stand-ins above. Whether the page plays
// on them is the maintainer's call — the switch ships off, and tools/qa/sense-check.ts holds it either way.
// (src/game is one import cycle: nothing here is built from another game module's export as the file loads.)
import type { Vec3 } from '../math/vec';
import type { SmokeEmitter, SmokeSystemLike } from '../smoke/types';
import { BOTH, type StaticCollision } from '../scene/collision';
import { SENSES } from '../scene/boxMirror';
import { sim, simWhyNot } from '../sim/sim';

export const SENSE = {
  /** the senses worked out by the game itself (see the head). Off = the renderer's values through Game.sensors, as ever. */
  cpu: false,
  /** THE GAME'S OWN SMOKE ON A COARSE GRID (`SmokeGrid` below; DEFAULT OFF — the page's `?smokegrid`, the harness's `TPS_SMOKEGRID=1`; read only under
   *  `cpu`): what the senses see through is density in half-metre cells that the walls and the doors hold, in place of a blob an emitter that no wall
   *  shapes. Off = the blobs, as they were. */
  grid: false,
  /** THE LIGHT MODEL (DEFAULT OFF — the page's `?cpulight`, the harness's `TPS_CPULIGHT=1`; it stands on the coarse grid, so it is read only where
   *  `grid` has one standing): the two things the estimate of the light on a man has never had (guards.ts lightEstimateAt). THE SMOKE'S SHADOW: a lamp
   *  no wall hides is dimmed by the grid's smoke between it and him. THE LIGHT THAT COMES ROUND: the lamps' light off the walls, one number a
   *  square metre of the plan, stepped with the grid (rust/sim/src/glow.rs) and added to the brightest face. Off = the estimate as it was. */
  bounce: false,
  /** the light that comes round, its numbers (glow.rs): how much of the light in a square leaves its surfaces again · how much of that a square hands
   *  across each open side · how many times a step relaxes it · the steps a sweep of the whole plan takes (a step is a tenth of a second) */
  glow: { albedo: 0.3, share: 0.1, passes: 2, sweep: 32 },
  smoke: {
    /** extinction per metre per unit of density (the renderer's: σt = 6/m per unit) */
    sigma: 6,
    /** what the smoke loses a second (the solver's densDecay) */
    decay: 0.14,
    /** how fast a blob widens: its width² grows by 2·spread·age (m²/s), up to `widest` metres */
    spread: 0.2, widest: 3.5,
    /** how far a blob's middle rises above where it was let out (m), and how fast (m/s) */
    rise: 1, riseRate: 0.3,
    /** an emitter that lets out less than this in its whole life (density × volume, unit·m³) is no blob: a muzzle's wisp, a spark's trail hide nobody */
    least: 0.05,
    /** a blob thinner than this (unit·m³) after its emitter has stopped is gone */
    gone: 0.02,
    /** the most blobs kept (the one with the least in it and still to come goes first) */
    cap: 48,
    /** the emitters a solver holds at once, and how many of those it keeps back from `budget` (smoke.ts MAX_EMITTERS, and its budget()'s reserve) */
    emitters: 64, reserve: 12,
  },
};

interface Blob { x: number; y0: number; z: number; r2: number; rate: number; ttl: number; age: number; mass: number; }
const PI15 = 5.568327996831708;   // π^1.5: a gaussian exp(−ρ²/r²) holds this much × r³

/** erf(x), Abramowitz & Stegun 7.1.26 (|error| < 1.5e-7) — on the game's own exp, so the same bits wherever it runs */
function erf(x: number): number {
  const s = x < 0 ? -1 : 1, a = x < 0 ? -x : x, t = 1 / (1 + 0.3275911 * a);
  return s * (1 - (((((1.061405429 * t - 1.453152027) * t) + 1.421413741) * t - 0.284496736) * t + 0.254829592) * t * Math.exp(-a * a));
}

/** THE GAME'S OWN SMOKE ON A COARSE GRID (SENSE.grid; the model is rust/sim/src/bricks.rs, and its head says what it is). This is the game's side of it:
 *  the state (the module's row of doubles, this game's own copy: what a snapshot takes; a file checkpoint keeps no smoke in the air), the seconds owed toward the next step,
 *  and the emitters handed over since the last one. The module is the only thing that steps it; where it cannot (the module does not run on this
 *  runtime, the level's plan is larger than a grid stands on) `stands` is false and the blobs answer, as they did. */
export class SmokeGrid {
  /** the state (bricks.rs LEN doubles) */
  st: Float64Array;
  /** how many times THIS file has written into `st` itself (a clear): the module's copy is old after one (boxMirror.ts gridSync) */
  writes = 0;
  /** seconds owed toward the next step (a step is a tenth of a second) */
  clock = 0;
  /** the emitters handed over since the last step: x y z, how wide (m), what each lets out a second (unit·m³), for how long */
  pending: number[] = [];
  /** did the last step find a grid standing on this level */
  stands = false;
  /** has anything been let out into it since it was last empty (a clear of an empty grid is nothing at all: a page's warm-up clears, a tape of it played
   *  again does not, and the two must step alike) */
  fed = false;
  constructor(len: number) { this.st = new Float64Array(len); }
  /** a grid for a game on this runtime, or none (the module does not run here) */
  static make(): SmokeGrid | null { return simWhyNot() ? null : new SmokeGrid(sim().grid_len()); }
  emit(x: number, y: number, z: number, radius: number, rate: number, ttl: number): void {
    const P = this.pending; if (P.length >= 6 * SENSE.smoke.emitters) P.splice(0, 6);   // (a game no step has reached for a long while keeps the latest)
    P.push(x, y, z, radius, rate, ttl); this.fed = true;
  }
  /** `dt` seconds of the game, inside its tick: the steps that have come due are taken (four at most: a game that stood still does not owe a hundred).
   *  `lamps`: the light that comes round is stepped too (SENSE.bounce), from the level's steady lamps as this writes them (guards.ts steadyLamps) */
  step(col: StaticCollision, dt: number, lamps: ((col: StaticCollision) => boolean) | null = null): void {
    this.clock += dt; if (this.clock < GRID_STEP) return;
    let left = this.clock, n = 0; while (left >= GRID_STEP && n < 4) { left -= GRID_STEP; n++; }
    this.stands = col.gridStep(this.st, this.writes, this.pending, n, SENSE.smoke, lamps ? { K: SENSE.glow, lamps } : null);
    if (this.stands) { this.clock = left >= GRID_STEP ? 0 : left; this.pending.length = 0; }
    else if (this.clock > 4 * GRID_STEP) this.clock = 4 * GRID_STEP;   // (nothing was done: the steps and the emitters wait, but no more of either than a call takes)
  }
  /** no smoke anywhere. The steps keep their beat (`clock` is the game's time, not the smoke's), and the light that comes round stays: it is the
   *  level's, not the smoke's. A NEW ENCOUNTER takes a new grid instead (Game.restartEncounter), so a file loaded into a running game and into a
   *  fresh one go on alike */
  clear(): void { if (!this.fed) return; this.st.fill(0, 0, GRID.nbx); this.st.fill(0, GRID.nbz + 1, GRID.glowD); this.writes++; this.pending.length = 0; this.fed = false; }   // (the smoke and its book: the plan it stands on and the light that comes round are not smoke)
  /** how much of the light along a → b comes through (1 = clear air); −1 = no grid stands (the caller's blobs answer) */
  along(col: StaticCollision, a: Vec3, b: Vec3): number { return this.stands ? col.gridAlong(this.st, this.writes, a, b, SENSE.smoke.sigma) : -1; }
  /** the light that comes round on a point (glow.rs): the bounced light of the nearest square metre of the plan whose middle the point has a clear line
   *  to — its own, else the ones beside it, nearest first — so a man is in the bounce of the room he is in and never of the one behind the wall. 0 off
   *  the plan, on a plan of more metres than the model holds, where no step has swept, and where no middle is in sight. Read from the state. */
  glowAt(col: StaticCollision, P: Vec3): number {
    const st = this.st, nx = st[GRID.nbx] * GRID.side, nz = st[GRID.nbz] * GRID.side; if (!(nx * nz <= GRID.glowMax)) return 0;
    const ix = Math.floor(P[0]), iz = Math.floor(P[2]), ax = P[0] - ix - 0.5, az = P[2] - iz - 0.5, sx = ax < 0 ? -1 : 1, sz = az < 0 ? -1 : 1, xFirst = Math.abs(ax) >= Math.abs(az);
    for (let k = 0; k < 4; k++) {
      const dx = k === 3 || k === (xFirst ? 1 : 2) ? sx : 0, dz = k === 3 || k === (xFirst ? 2 : 1) ? sz : 0, jx = ix + dx, jz = iz + dz;   // (its own · across the nearer edge · across the other · the corner's)
      if (!(jx >= 0 && jz >= 0 && jx < nx && jz < nz)) continue;
      if (!col.segmentBlocked(P, [jx + 0.5, GRID.chest, jz + 0.5])) return st[GRID.glowB + jz * nx + jx];
    }
    return 0;
  }
}
/** THE GRID'S OWN NUMBERS as rust/sim/src/bricks.rs has them (tools/qa/smokegrid-check.ts reads that file and is red where one differs): seconds a step, and
 *  the state's layout for whoever reads it: doubles in the head, emitters and doubles an emitter, bricks, cells a brick, m³ a cell, and where in the head
 *  the book of the smoke is kept (let out · given back with thin bricks · refused for want of a brick) */
export const GRID = { step: 0.1, head: 8, emitters: 64, emit: 7, bricks: 160, cells: 128, vol: 0.125, out: 4, given: 5, lost: 6,
  /** …the plan in squares (the head), a square's side (m), and the light that comes round (glow.rs), a number a square metre: the most metres it holds,
   *  the height it is read at, and where its four columns start: the lamps' light · the bounce · the way open toward +x · toward +z */
  nbx: 2, nbz: 3, side: 2, glowMax: 2048, chest: 1.2, glowD: 8 + 64 * 7 + 160 * 2 + 160 * 128, glowB: 8 + 64 * 7 + 160 * 2 + 160 * 128 + 2048, glowX: 8 + 64 * 7 + 160 * 2 + 160 * 128 + 2 * 2048,
  glowZ: 8 + 64 * 7 + 160 * 2 + 160 * 128 + 3 * 2048,
  /** …and, after them, the word of the lamps the plan was last swept whole under (the row's last double) */
  glowLamps: 8 + 64 * 7 + 160 * 2 + 160 * 128 + 4 * 2048 };
/** THE LIGHT MODEL'S FIELD as the estimate of the light on a man takes it (guards.ts lightEstimateAt): a game's own smoke with a grid standing. The grid
 *  holds the light that comes round; the smoke answers what dims a lamp on its way (the grid's cells, and the blobs where the grid cannot answer). */
export type LightField = CpuSmoke & { grid: SmokeGrid };
const GRID_STEP = GRID.step;

/** THE SMOKE AS THE GAME KNOWS IT (see the head): plain numbers, stepped with the game, nothing of the renderer's */
export class CpuSmoke {
  blobs: Blob[] = [];
  /** the same smoke on the coarse grid, where SENSE.grid has put one beside the blobs (game.ts senseStep): it is fed what the blobs are fed, and answers
   *  a sight line where it stands */
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
    if (this.blobs.length >= K.cap) {   // full: the one worth least goes — what it holds and what it has still to let out — and that may be the newcomer
      const worth = (b: Blob) => b.mass + b.rate * Math.max(0, b.ttl - b.age);
      let k = 0; for (let i = 1; i < this.blobs.length; i++) if (worth(this.blobs[i]) < worth(this.blobs[k])) k = i;
      if (worth(this.blobs[k]) >= rate * e.ttl) return;
      this.blobs.splice(k, 1);
    }
    this.blobs.push({ x: e.pos[0], y0: e.pos[1], z: e.pos[2], r2: r * r, rate, ttl: e.ttl, age: 0, mass: 0 });
  }
  /** how many more emitters there is room for, as the solver counts it (smoke.ts budget) — off the game's own count */
  budget(): number { const K = SENSE.smoke; return Math.max(0, K.emitters - K.reserve - this.lives.length); }
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
  /** how much of the light along a → b comes through (1 = clear air). `blocked(p, q)`: is there a wall between the two points (the smoke of the next room is not
   *  in this one's sight lines) */
  trans(a: Vec3, b: Vec3, blocked: (p: Vec3, q: Vec3) => boolean): number {
    if (!this.blobs.length) return 1;
    const K = SENSE.smoke, dx = b[0] - a[0], dy = b[1] - a[1], dz = b[2] - a[2], L = Math.sqrt(dx * dx + dy * dy + dz * dz);
    if (L < 1e-6) return 1;
    const ux = dx / L, uy = dy / L, uz = dz / L; let tau = 0;
    for (const bl of this.blobs) {
      if (bl.mass <= 0) continue;
      const s2 = Math.min(bl.r2 + 2 * K.spread * bl.age, K.widest * K.widest), s = Math.sqrt(s2), cy = bl.y0 + Math.min(K.rise, K.riseRate * bl.age);
      const px = bl.x - a[0], py = cy - a[1], pz = bl.z - a[2], t = px * ux + py * uy + pz * uz;   // the blob's middle, along the line
      const d2 = px * px + py * py + pz * pz - t * t;
      if (d2 > 9 * s2 || t < -3 * s || t > L + 3 * s) continue;
      const tc = t < 0 ? 0 : t > L ? L : t;
      if (blocked([bl.x, cy, bl.z], [a[0] + ux * tc, a[1] + uy * tc, a[2] + uz * tc])) continue;
      tau += K.sigma * bl.mass / (Math.PI * s2) * Math.exp(-Math.max(d2, 0) / s2) * 0.5 * (erf((L - t) / s) - erf(-t / s));
    }
    return tau <= 0 ? 1 : Math.exp(-tau);
  }
  /** `trans` in the game's own box world: a wall between a blob and the line is anything a sight line stops at (col.segmentBlocked). With RUST.smoke the sum
   *  is the module's (rust/sim/src/smoke.rs: trans's statements over the same blobs, written there for the one question). */
  along(col: StaticCollision, a: Vec3, b: Vec3): number {
    if (this.grid) { const t = this.grid.along(col, a, b); if (t !== -1) return t; }
    const B = this.blobs; if (!B.length) return 1;
    if (col.blobsBegin(B.length)) {
      for (let i = 0; i < B.length; i++) { const bl = B[i]; col.blob(bl.x, bl.y0, bl.z, bl.r2, bl.age, bl.mass); }
      const K = SENSE.smoke, r = col.blobsAlong(a, b, K.sigma, K.spread, K.widest, K.rise, K.riseRate);
      if (r !== -1) {
        if (!BOTH.on) return r;
        const t = this.trans(a, b, (p, q) => col.segmentBlocked(p, q)); col.senseCompared(col.senseSet(SENSES.smoke), Object.is(t, r), 'smoke', a, b); return t;
      }
    }
    return this.trans(a, b, (p, q) => col.segmentBlocked(p, q));
  }
}

/** THE GAME'S OWN SMOKE PUT IN FRONT OF THE SOLVER under SENSE.cpu: every emitter the solver is handed — by the game or by anything else that holds it (a panel
 *  button, the page's warm-up) — also feeds `own`, a clear clears both, and what the game asks back to decide how much more to emit is `own`'s answer. Done on the
 *  solver object itself, so nobody is left holding an unwatched one; the solver's own working is called as it was. Returns the undo. */
export function ownSmoke(real: SmokeSystemLike, own: CpuSmoke): () => void {
  const R = real as unknown as Record<string, unknown>, had: [string, PropertyDescriptor | undefined][] = [];
  const emit = real.emit, clearAll = real.clearAll;
  const set = (k: string, f: unknown) => { had.push([k, Object.getOwnPropertyDescriptor(R, k)]); Object.defineProperty(R, k, { value: f, writable: true, configurable: true, enumerable: false }); };
  set('emit', (e: SmokeEmitter) => { own.emit(e); emit.call(real, e); });
  set('clearAll', () => { own.clear(); clearAll?.call(real); });
  set('budget', () => own.budget());
  return () => { for (const [k, d] of had) { if (d) Object.defineProperty(R, k, d); else delete R[k]; } };
}
