// THE LAMPS' REACH MASKS, KEPT BY THE PICTURE. A mask says which 1 m cells of the level a light can reach past the full-height walls (render/reach.ts has the
// rules); only the picture reads one (the light set's rows: the direct pass and the reservoirs). Until 11-05 the masks, the wall raster they are decided on
// and the queue that decides them again after a wall changes all lived in the Game and were worked in its tick: a blown wall then cost the GAME's frame 10 to
// 42 ms of picture work, a headless run and a server paid for masks nobody drew, and every row of it rode in the rollback snapshot.
// WHAT LIVES HERE: all of it. The masks themselves, a light at a time (no light object carries one: render/lights.ts asks `row`); the raster; the lights
// waiting to be decided again and the walls' changes noted but not yet read. The engine keeps one of these and takes ONE STEP OF IT A DRAWN FRAME (`tick`).
// A stand with no picture has none: the game's three tells below go nowhere and no mask is ever made.
// WHAT THE GAME TELLS IT, and nothing else passes between them: `boot` (the level stands: its doors, and which cells are under a roof), `changed` (walls
// changed in this rect, or `null`: they are the authored walls again), `dir` and `sun` (this directional light wants a mask). It READS the box world's
// statics and the light set's lights and writes nothing of the game's. The engine hands it to the light set's upload each frame (lights.ts `upload`): no
// object a snapshot walks holds it.
// NOBODY'S STATE: a HostOwned (math/side.ts), so a snapshot neither copies nor compares it. After a snapshot is put back the walls may be another moment's
// and nobody told it: it reads the count of put-backs (math/side.ts PUT_BACKS, the host's) at each step, and after one it reads the walls again and decides
// the masks again from where they stand, only where the raster differs. Any put-back, a rollback's as well as a save's, with no caller to remember it.
// A WIDE SUN'S MASK IS OF ITS DISC (`sun`), and the disc is the game's to change and to put back: each step looks at the radius such a mask was made for
// against the light's own, and makes it again where they differ (so the hour's tell is a hint, and a radius that went back with a snapshot is followed).
// It imports nothing from src/game.
import { HostOwned, PUT_BACKS } from '../math/side';
import { type Vec3, v3 } from '../math/vec';
import { type Box, BoxFlag, rayBox } from '../scene/boxes';
import { WORLD } from '../scene/world';
import type { RtLight } from './lights';
import { buildWallRaster, reachMaskFor, reachMaskSome, reachMaskForDirectionalSteps, wallRasterDiff, wallRasterDiffSteps, rasterRect, REACH_WORK, REACH_PACE, type ReachDoor, type ReachOnly, type ReachJob } from './reach';
import { whole, clock } from './steps';

type Rect = [number, number, number, number];
/** a light waiting to have its mask decided again: the rects walls changed in within its reach, the raster's changed sub-cells, and the mask part-decided */
interface Asked { l: RtLight; rects: Rect[]; boxes: Rect[]; job?: ReachJob; /** a directional light's mask part-decided (a few cells a step, on the clock), and the mask it is being decided again from */ dirJob?: { it: Generator<void, Uint32Array | null, void>; prev: Uint32Array | undefined }; restarts: number }
/** what a directional light's mask was made for: its disc's radius (under 0 = one direction), the way it shone, and the walls it was made on */
interface Made { r: number; d: Vec3; on: Uint8Array | null }
/** A STEP'S ALLOWANCE OF TIME (ms) where the frame has the planners' pace (render/planPace.ts; `tickTimed`): the masks waiting are decided for this long
 *  a drawn frame and carried on at the next. Its own allowance, beside the planners': about a wall's worth of masks is 25 to 45 ms of work cold in a
 *  browser, so a dozen or two frames. Where the frame has no pace (a stand with no clock to keep, `?planpace=old`, a page's first frames) the step is
 *  the count of sight-line steps it always was (REACH_PACE.steps). */
export const KEEP_MS = 2;
/** sight-line steps a lamp's mask is given between two looks at the clock */
const SLICE = 4000;

export class ReachKeep extends HostOwned {
  private readonly world: { readonly statics: Box[] }; private readonly lights: { readonly lights: RtLight[] };
  /** a lamp's mask (none = it may reach anywhere) and a directional light's (the cells with a clear way toward it: sent only with the reservoirs) */
  private readonly point = new WeakMap<RtLight, Uint32Array>(); private readonly dirs = new WeakMap<RtLight, Uint32Array>();
  /** what a directional light's mask was made for: the disc's radius (`sun`; under 0 = one direction, `dir`) and the way it shone */
  private readonly made = new WeakMap<RtLight, Made>();
  /** the full-height walls the masks stand on, and the level's as it booted (its masks come straight back when the walls are the authored ones again) */
  private solid: Uint8Array | null = null; private solidBoot: Uint8Array | null = null; private readonly bootMask = new Map<RtLight, { m: Uint32Array; made?: Made }>();
  private doors: ReachDoor[] = []; private interior: Uint8Array | null = null;
  /** the lamps that carry a mask, and the lights whose masks wait to be decided again */
  private masked: RtLight[] = []; private queue: Asked[] = [];
  /** walls' changes noted and not yet read into the raster: the ones that wait `hold` steps (the blast's own frame is the long one), the ones read at the
   *  next step, and `stale`: the walls are to be read again whatever was noted (a snapshot put back, known by the count of them) */
  private held: Rect[] = []; private hold = 0; private now: Rect[] = []; private stale = false; private putBacks = PUT_BACKS.n;
  /** a reading of the walls in hand (on a budget of time: the raster as it was built when the reading began, and the scan that sets it against the last) */
  private reading: { rects: Rect[]; solid: Uint8Array; it: Generator<void, Rect | null, void> } | null = null;

  constructor(world: { readonly statics: Box[] }, lights: { readonly lights: RtLight[] }) { super(); this.world = world; this.lights = lights; }

  /** a lamp's own mask (undefined = anywhere) */
  of(l: RtLight): Uint32Array | undefined { return this.point.get(l); }
  /** a directional light's mask (undefined = it has none) */
  dirOf(l: RtLight): Uint32Array | undefined { return this.dirs.get(l); }
  /** the row the light set sends for a light: a directional light's own mask only where `dirMasks` (the reservoirs), else the lamp's, else none (anywhere) */
  row(l: RtLight, dirMasks: boolean): Uint32Array | undefined { return (dirMasks ? this.dirs.get(l) : undefined) ?? this.point.get(l); }
  /** a mask its maker decided itself (the picture's own lights: daylight at an opening, engine.ts); undefined = none */
  give(l: RtLight, m: Uint32Array | undefined): void { if (m) this.point.set(l, m); else this.point.delete(l); }
  /** lights still waiting, and changes still to be read (none before a level stands) */
  get pending(): number { return this.solid ? this.queue.length + this.held.length + this.now.length + (this.reading ? 1 : 0) + (this.stale || this.putBacks !== PUT_BACKS.n ? 1 : 0) : 0; }
  /** what it stands on, as it is (a tool's: the raster, the cells under a roof, the lamps that carry a mask, the lights waiting and the notes not yet read) */
  peek(): { solid: Uint8Array | null; interior: Uint8Array | null; doors: readonly ReachDoor[]; lamps: readonly RtLight[]; queue: readonly { l: RtLight }[]; held: readonly unknown[]; now: readonly unknown[] } {
    return { solid: this.solid, interior: this.interior, doors: this.doors, lamps: this.masked, queue: this.queue, held: this.held, now: this.now }; }

  /** THE LEVEL STANDS: every static interior light it built (fixture area lights, analytic lamps, the beacon spots) gets the cells it can light past the
   *  full-height walls, street lamps included; torches, flashes and the rail light keep none (anywhere). Each directional light gets its own (the moon).
   *  A few tens of ms, once. `interior` = the cells under a roof (reach.ts buildInteriorCells: the game keeps it too, for its own rules). */
  boot(doors: ReachDoor[], interior: Uint8Array): void {
    const solid = buildWallRaster(this.world.statics, doors);
    this.doors = doors; this.interior = interior; this.solid = this.solidBoot = solid;
    this.masked = []; this.queue = []; this.bootMask.clear(); this.held = []; this.now = []; this.hold = 0; this.stale = false; this.putBacks = PUT_BACKS.n; this.reading = null;
    for (const l of this.lights.lights) {
      if (l.ttl >= 0 || l.kind === 2 || l.group === 'player' || l.group === 'flashlights') continue;   // transient / the moon / lights that MOVE (Sam's rail light, the guards' torches): anywhere. Exterior lamp heads above the roof DO get a mask, but only its interior cells (reach.ts)
      this.masked.push(l); this.point.set(l, reachMaskFor(solid, l.pos, l.kind === 3 || l.kind === 1 ? l.dir : [0, 0, 0], l.radius, interior));
    }
    for (const l of this.lights.lights) if (l.kind === 2) this.dir(l);
  }

  /** WALLS CHANGED inside `rect` [x0, z0, x1, z1]: noted, and read into the raster `hold` steps on with every rect noted since (hold < 0, or the old pace:
   *  at the next step). Every masked light whose range reaches a rect is then asked again, and only the cells whose line to the light passes the change are
   *  decided again (reach.ts ReachOnly). `null` = THE AUTHORED WALLS AGAIN (the wall engine's reset, which has just put them back): the raster and the masks
   *  the level booted with, at once, and nothing left waiting. It reads nothing: a hole made again after it (a checkpoint's) is noted and read as any other.
   *  A stale mask in between only keeps a light out of cells it could now reach. */
  changed(rect: Rect | null, hold = -1): void {
    if (!this.solid) return;
    if (!rect) { this.asBooted(); return; }
    if (hold >= 0 && !REACH_PACE.whole) { if (!this.held.length) this.hold = hold; this.held.push(rect); return; }
    this.now.push(rect);
  }

  private raster(): Uint8Array { return buildWallRaster(this.world.statics, this.doors); }
  /** the raster and the masks the level booted with, and nothing waiting */
  private asBooted(): void {
    this.solid = this.solidBoot; this.queue.length = 0; this.held.length = 0; this.now.length = 0; this.reading = null;
    for (const [l, b] of this.bootMask) { if (l.kind === 2) { this.dirs.set(l, b.m); if (b.made) this.made.set(l, { ...b.made }); } else this.point.set(l, b.m); }   // (a light with no entry was never decided again: its mask IS the authored level's. A directional light's goes back with what it was made for, and the next step makes it again if its disc is another now)
    this.bootMask.clear();
  }
  private ask(l: RtLight, r: Rect, box: Rect): void {
    const q = this.queue.find(x => x.l === l);
    if (!q) { this.queue.push({ l, rects: [r], boxes: [box], restarts: 0 }); return; }
    if (q.job?.st || q.dirJob) { q.job = undefined; q.dirJob = undefined; q.restarts++; }   // part-decided, and now there is more to decide: it starts over (a light this change does not reach carries on: it would not have been asked for it)
    q.rects.push(r); if (!q.boxes.includes(box)) q.boxes.push(box);
  }
  private askNear(r: Rect, box: Rect): void {
    for (const l of this.masked) { const dx = Math.max(r[0] - l.pos[0], 0, l.pos[0] - r[2]), dz = Math.max(r[1] - l.pos[2], 0, l.pos[2] - r[3]); if (Math.hypot(dx, dz) > l.range + 1) continue; this.ask(l, r, box); }
    for (const l of this.lights.lights) if (l.kind === 2 && this.dirs.has(l)) this.ask(l, r, box);
  }
  /** the noted rects read: the raster built again and set against the last, the lights in reach of each rect asked (the raster's changed box theirs together) */
  private read(rects: Rect[]): void {
    if (!this.solid) return;
    const solid = this.raster(); this.readEnds(rects, solid, wallRasterDiff(this.solid, solid));
  }
  /** … its end: `box` = the sub-cells that changed (a lamp's cells are decided again only where their own sight lines touch them), null = none did */
  private readEnds(rects: Rect[], solid: Uint8Array, box: Rect | null): void { if (!box) return; this.solid = solid; for (const r of rects) this.askNear(r, box); }
  /** … on a budget of time: the raster is built when the reading begins (the walls as they stand then; a change noted meanwhile is its own reading
   *  after), and the scan against the last is stepped until `late`. True when the reading has ended. */
  private readSome(rects: Rect[] | null, late: () => boolean): boolean {
    if (!this.solid) return true;
    if (!this.reading) { if (!rects) return true; const solid = this.raster(); this.reading = { rects, solid, it: wallRasterDiffSteps(this.solid, solid) }; if (late()) return false; }
    const R = this.reading; let r: IteratorResult<void, Rect | null>;
    try { r = R.it.next(); while (!r.done && !late()) r = R.it.next(); } catch (e) { this.reading = null; throw e; }
    if (!r.done) return false;
    this.reading = null; this.readEnds(R.rects, R.solid, r.value); return true;
  }
  /** the walls read again with nothing known of where they changed (a snapshot put back). The authored walls: the raster the level booted with (and its
   *  sight lines' jump field) and the masks it booted with, at once. Any others: the lights in reach of the raster's own changed box are asked, from the
   *  masks as they stand (one rect round every change: two holes far apart cost the lamps between them a wide wedge each). */
  private reread(): void {
    if (!this.solid) return;
    this.held.length = 0; this.now.length = 0; this.reading = null;
    const solid = this.raster(), boot = this.solidBoot;
    let same = !!boot && boot.length === solid.length; for (let i = 0; same && i < solid.length; i++) if (boot![i] !== solid[i]) same = false;
    if (same) { this.asBooted(); return; }
    const box = wallRasterDiff(this.solid, solid); if (!box) return;
    this.solid = solid;
    for (const q of this.queue) if (q.job?.st || q.dirJob) { q.job = undefined; q.dirJob = undefined; q.restarts++; }   // (the raster changed under every part-decided mask)
    this.askNear(rasterRect(box), box);
  }

  /** ONE STEP ON THE COUNT, a drawn frame's where the frame has no pace (a tool's stand, which keeps no clock; the page with `?planpace=old`; a page's
   *  first frames): the walls read where a note is due, then the waiting masks decided again in the order they were asked, `steps` sight-line steps of
   *  them (about 2 ms warm: a mask too big for a step's share carries on where it stopped at the next, reach.ts ReachJob, and is put in whole when its
   *  last cell is decided; a directional light's rays are a step's share by themselves), at most `n` lights finished. Returns the lights finished.
   *  `REACH_PACE.whole` (the page's ?reachpace=old) = the pace before the shares: each mask whole, one light at least and more until `wholeCells` cells
   *  are decided. No clock can change what a step on the count does. */
  tick(n = 8, steps = REACH_PACE.steps): number { return this.step(false, n, steps); }
  /** ONE STEP ON THE CLOCK, a drawn frame's where the frame has the planners' pace: the same work for KEEP_MS. The walls' reading in the scan's own
   *  steps; a lamp's mask in slices of sight-line steps; a directional light's a decided cell at a time; each carried on at the next step where it
   *  stopped. One scan step and one slice or cell at least, so a slow machine still ends and a reading never keeps the masks waiting. Under
   *  `REACH_PACE.whole` it is the count's step (the old pace keeps its frames). */
  tickTimed(n = 8): number { return this.step(!REACH_PACE.whole, n, REACH_PACE.steps); }
  /** the clock a step on the clock looks at (a check scripts one) */
  timeNow: () => number = clock;

  private step(byTime: boolean, n: number, steps: number): number {
    const putBack = this.putBacks !== PUT_BACKS.n; this.putBacks = PUT_BACKS.n;
    if (!this.solid) return 0;
    if (putBack) this.stale = true;   // (a snapshot was put back since the last step: the walls may be another moment's)
    if (this.stale) { this.stale = false; this.reread(); }
    // A DIRECTIONAL LIGHT'S MASK KNOWS WHAT IT WAS MADE FOR (the walls, the way it shone, its disc): one that is not waiting to be decided again and whose
    // disc is another now (the weather turned; a snapshot put the old one back) or whose walls are not these (made on a holed level since put back as
    // authored) is made again here, whole. Rare, and never on a blast's frames: a blast asks the light, and a light that waits is the queue's.
    for (const l of this.lights.lights) { if (l.kind !== 2) continue; const k = this.made.get(l); if (!k || this.queue.some(q => q.l === l)) continue;
      if ((k.r >= 0 && k.r !== l.radius) || k.on !== this.solid) { this.made.delete(l); if (k.r >= 0) this.sun(l, l.radius); else this.dir(l); } }
    const old = REACH_PACE.whole, t0 = byTime ? this.timeNow() : 0, late = (): boolean => this.timeNow() - t0 >= KEEP_MS;
    if (this.reading && !byTime) { const R = this.reading; this.reading = null; this.readEnds(R.rects, R.solid, whole(R.it)); }   // (a reading in hand and no clock to keep: ended here)
    if (byTime) {   // THE WALLS, a little a step: a reading in hand goes on; a note that is due begins one (with every rect noted since). The masks asked before it carry on below
      const due = !this.reading && (this.now.length > 0 || (this.held.length > 0 && this.hold-- <= 0));
      if (this.reading || due) this.readSome(due ? [...this.held.splice(0), ...this.now.splice(0)] : null, late);
    } else {
      if (this.now.length) this.read(this.now.splice(0));
      if (this.held.length && this.hold-- <= 0) { this.read(this.held.splice(0)); return 0; }   // (the noted changes read into the raster: this step's share; the masks from the next. While a note waits, masks asked before it carry on below)
    }
    let done = 0, worked = false; const c0 = REACH_WORK.steps, k0 = REACH_WORK.cells;
    while (done < n && this.queue.length) {
      const left = old ? Infinity : steps - (REACH_WORK.steps - c0);
      if (byTime ? worked && late() : left <= 0 || (old && done > 0 && REACH_WORK.cells - k0 >= REACH_PACE.wholeCells)) break;
      const q = this.queue[0], l = q.l;
      const boot = () => { if (!this.bootMask.has(l)) { const m = l.kind === 2 ? this.dirs.get(l) : this.point.get(l), k = this.made.get(l); if (m) this.bootMask.set(l, { m, made: k ? { ...k } : undefined }); } };   // what the level as authored gave it (masks are replaced, never written in place)
      if (l.kind === 2) {
        const prev = this.dirs.get(l), k = this.made.get(l), only = prev ? { prev, rects: q.rects } : undefined;
        if (!only) { this.queue.shift(); continue; }   // (no mask to decide again: nothing asked it, and nothing is owed)
        if (byTime && q.restarts < 2) {   // a few cells a step. The job reads the statics as they stood when it began and is begun again if the light is asked again (ask); the third go is whole, as below
          const J = q.dirJob ??= { it: this.dirLightSteps(l.dir, k ? k.r : -1, only), prev };
          let r: IteratorResult<void, Uint32Array | null>;
          try { r = J.it.next(); worked = true; while (!r.done && !late()) r = J.it.next(); } catch (e) { q.dirJob = undefined; throw e; }
          if (!r.done) break;
          q.dirJob = undefined;
          if (this.dirs.get(l) !== J.prev) { q.restarts++; continue; }   // (its mask was made anew under the job: decided again from that one)
          boot(); if (r.value) { this.dirs.set(l, r.value); if (k) this.made.set(l, { ...k, on: this.solid }); }
          this.queue.shift(); done++; continue;
        }
        if (!byTime && !old && REACH_WORK.steps !== c0) break;
        q.dirJob = undefined; boot(); this.dir(l, only); worked = true;
        this.queue.shift(); done++; if (old || byTime) continue; break;
      }
      const job = q.job ??= { budget: left }, prev = this.point.get(l), only = prev ? { prev, rects: q.rects, boxes: q.boxes, maxDist: l.range + 1.5 } : undefined, dir: Vec3 = l.kind === 3 || l.kind === 1 ? l.dir : [0, 0, 0];
      let m: Uint32Array | null;   // (walls changing faster than a big mask can be decided in shares: the third go takes it in one, as every mask once was)
      if (byTime) do { job.budget = q.restarts >= 2 ? Infinity : SLICE; m = reachMaskSome(this.solid, l.pos, dir, l.radius, this.interior, only, job); worked = true; } while (!m && !late());
      else { job.budget = q.restarts >= 2 ? Infinity : left; m = reachMaskSome(this.solid, l.pos, dir, l.radius, this.interior, only, job); }
      if (!m) break;
      boot(); this.point.set(l, m); this.queue.shift(); done++;
    }
    return done;
  }

  /** Give a DIRECTIONAL light (kind 2) its mask: the outdoor cells, plus every indoor cell with an unblocked static path toward it (reach.ts
   *  reachMaskForDirectionalSteps over the level's shadow-casting statics, glass passing), over its disc if it was made for one. A few ms to a few tens.
   *  `only` = decided again behind what changed (the queue's own ask on the count: the light keeps its place in the queue and what it was made for);
   *  without it the mask is made whole for ONE direction, of the walls as they stand, and the light waits for nothing. */
  dir(l: RtLight, only?: ReachOnly): void {
    if (l.kind !== 2) return;
    const k = this.made.get(l), m = whole(this.dirLightSteps(l.dir, only && k ? k.r : -1, only)); if (!m) return;
    this.dirs.set(l, m);
    if (only) { if (k) this.made.set(l, { ...k, on: this.solid }); }
    else { this.made.set(l, { r: -1, d: [l.dir[0], l.dir[1], l.dir[2]], on: this.solid }); this.unask(l); }
  }
  /** … and a WIDE one (the hour's sun: the overcast's disc is a soft box some twenty degrees across), which reaches cells its centre does not: the union over
   *  the centre and four directions on the disc's rim (a cell wrongly dark there would get no candidate at all; one wrongly lit only costs a ray). Asked
   *  twice for the same disc the same way on the same walls, it is made once. */
  sun(l: RtLight, radius: number): void {
    if (l.kind !== 2) return;
    const d0 = l.dir, was = this.made.get(l);
    if (was && was.r === radius && was.on === this.solid && was.d[0] === d0[0] && was.d[1] === d0[1] && was.d[2] === d0[2] && this.dirs.has(l)) return;
    const m = whole(this.dirLightSteps(d0, radius)); if (!m) return;
    this.dirs.set(l, m); this.made.set(l, { r: radius, d: [d0[0], d0[1], d0[2]], on: this.solid }); this.unask(l);
  }
  /** a mask made whole just now is of the walls as they stand: the light waits for nothing */
  private unask(l: RtLight): void { const i = this.queue.findIndex(q => q.l === l); if (i >= 0) this.queue.splice(i, 1); }
  /** what a whole derivation gives a directional light now, for what its mask was made for (a tool's: nothing is kept) */
  wholeOf(l: RtLight): Uint32Array | null { const k = this.made.get(l); return l.kind === 2 && k ? whole(this.dirLightSteps(l.dir, k.r)) : null; }

  /** A DIRECTIONAL LIGHT'S MASK IN STEPS, a stop a decided cell: along `d0`, and for a disc (`radius` over 0.05) the union with four directions on its
   *  rim. One reading of the statics serves all five. `only` = decided again behind what changed: each direction keeps `only.prev` away from the rects,
   *  so the union is the old mask there and the five's own answers by the hole. */
  private *dirLightSteps(d0: Vec3, radius: number, only?: ReachOnly): Generator<void, Uint32Array | null, void> {
    if (!this.interior) return null;
    const S = this.scene(), interior = this.interior;
    const toLight = (d: Vec3): Vec3 => { const len = Math.hypot(d[0], d[1], d[2]) || 1; return [-d[0] / len, -d[1] / len, -d[2] / len]; };
    const acc = yield* reachMaskForDirectionalSteps(toLight(d0), interior, S.blocked, only, S.free);
    if (radius > 0.05) {
      const u = v3.normalize(v3.cross(d0, Math.abs(d0[1]) < 0.9 ? [0, 1, 0] : [1, 0, 0])), w = v3.cross(d0, u), k = Math.tan(radius);
      for (let i = 0; i < 4; i++) { const a = i * Math.PI / 2, c = Math.cos(a), s = Math.sin(a);
        const m = yield* reachMaskForDirectionalSteps(toLight(v3.normalize([d0[0] + (u[0] * c + w[0] * s) * k, d0[1] + (u[1] * c + w[1] * s) * k, d0[2] + (u[2] * c + w[2] * s) * k])), interior, S.blocked, only, S.free);
        for (let j = 0; j < acc.length; j++) acc[j] |= m[j]; }
    }
    return acc;
  }
  /** the statics as rays and looks ask them: AS THEY STAND NOW (the list is copied: a piece appended or cut off under a job in hand must not move its
   *  indices), bucketed on the 1 m grid once, each box's turn worked out once */
  private scene(): { blocked: (from: Vec3, dir: Vec3, dist: number) => boolean; free: (p: Vec3) => Vec3 | null } {
    // ~10 k short rays: bucket every SHADOW-CASTING static (NoPrimary ones too: the ceiling slab is exactly what stops the moon; glass passes light) on the 1 m
    // grid once, then march each ray cell by cell testing the 3 × 3 buckets round it and stop at the first hit (indoors that is the ceiling within a metre or
    // two): ~30 ms, not the 0.4 s of a brute-force raycast over every static
    const W = WORLD.gridW, H = WORLD.gridH, cs = WORLD.gridCell, statics = this.world.statics.slice();
    const buckets: number[][] = Array.from({ length: W * H }, () => []), cosY = new Float64Array(statics.length), sinY = new Float64Array(statics.length);   // (a box's turn, worked out once a box and not once a look: a look asks every box of its cell)
    statics.forEach((b, i) => {
      if (b.flags & (BoxFlag.NoShadow | BoxFlag.Glass | BoxFlag.Dynamic)) return;
      const yw = b.yaw ?? 0; cosY[i] = Math.cos(yw); sinY[i] = Math.sin(yw);
      const r = Math.hypot(b.h[0], b.h[2]);   // yawed footprint bound
      const x0 = Math.max(0, Math.floor((b.c[0] - r - WORLD.min[0]) / cs)), x1 = Math.min(W - 1, Math.floor((b.c[0] + r - WORLD.min[0]) / cs));
      const z0 = Math.max(0, Math.floor((b.c[2] - r - WORLD.min[2]) / cs)), z1 = Math.min(H - 1, Math.floor((b.c[2] + r - WORLD.min[2]) / cs));
      for (let z = z0; z <= z1; z++) for (let x = x0; x <= x1; x++) buckets[z * W + x].push(i);
    });
    const seen = new Uint32Array(statics.length); let stamp = 0;
    const blocked = (from: Vec3, dir: Vec3, dist: number): boolean => {
      stamp++;
      const steps = Math.ceil(dist / 0.5);
      let lastCell = -1;
      for (let s = 0; s <= steps; s++) {
        const t = Math.min(dist, s * 0.5);
        const cx = Math.floor((from[0] + dir[0] * t - WORLD.min[0]) / cs), cz = Math.floor((from[2] + dir[2] * t - WORLD.min[2]) / cs);
        const cell = cz * W + cx; if (cell === lastCell) continue; lastCell = cell;
        for (let dz = -1; dz <= 1; dz++) for (let dx = -1; dx <= 1; dx++) {
          const x = cx + dx, z = cz + dz; if (x < 0 || z < 0 || x >= W || z >= H) continue;
          for (const i of buckets[z * W + x]) { if (seen[i] === stamp) continue; seen[i] = stamp; if (rayBox(from, dir, statics[i], dist)) return true; }
        }
      }
      return false;
    };
    // a look that fell inside a solid (reach.ts, the header of reachMaskForDirectional): out through the nearest side face of that box that is still in the look's own
    // cell and in no other box. Upright boxes, turned or not (the walls): a tilted, round or cut box is read as open air, as it was.
    const solidAt = (x: number, y: number, z: number): number => {   // (the box's index, or -1)
      const cx = Math.floor((x - WORLD.min[0]) / cs), cz = Math.floor((z - WORLD.min[2]) / cs); if (cx < 0 || cz < 0 || cx >= W || cz >= H) return -1;
      for (const i of buckets[cz * W + cx]) { const b = statics[i]; if (b.rot || b.round === true || (b.cut !== undefined && b.cut.length > 0) || Math.abs(y - b.c[1]) >= b.h[1]) continue;
        const c = cosY[i], s = sinY[i], dx = x - b.c[0], dz = z - b.c[2];
        if (Math.abs(c * dx - s * dz) < b.h[0] && Math.abs(s * dx + c * dz) < b.h[2]) return i; }
      return -1;
    };
    const free = (p: Vec3): Vec3 | null => {
      const bi = solidAt(p[0], p[1], p[2]); if (bi < 0) return p;
      const b = statics[bi], x0 = Math.floor((p[0] - WORLD.min[0]) / cs) * cs + WORLD.min[0], z0 = Math.floor((p[2] - WORLD.min[2]) / cs) * cs + WORLD.min[2], E = 0.02;
      const c = cosY[bi], s = sinY[bi], dx = p[0] - b.c[0], dz = p[2] - b.c[2], lx = c * dx - s * dz, lz = s * dx + c * dz;   // (the box's own frame, as editor/lint.ts insideBox turns it)
      let best: Vec3 | null = null, bd = Infinity;
      for (const [q0, q1, n0, n1] of [[-(b.h[0] + E), lz, -1, 0], [b.h[0] + E, lz, 1, 0], [lx, -(b.h[2] + E), 0, -1], [lx, b.h[2] + E, 0, 1]]) {
        const wx = b.c[0] + c * q0 + s * q1, wz = b.c[2] - s * q0 + c * q1, dd = Math.abs(q0 - lx) + Math.abs(q1 - lz);
        // (… with air beyond it: a cabinet 2 cm off a wall put the look ON the wall's face, and a ray that starts on a face does not hit it)
        if (dd < bd && wx > x0 && wx < x0 + cs && wz > z0 && wz < z0 + cs && solidAt(wx, p[1], wz) < 0 && solidAt(wx + (c * n0 + s * n1) * E, p[1], wz + (-s * n0 + c * n1) * E) < 0) { bd = dd; best = [wx, p[1], wz]; }
      }
      return best;
    };
    return { blocked, free };
  }
}
