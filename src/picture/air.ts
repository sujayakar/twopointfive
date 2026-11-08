// THE AIR AS ONE SIMULATED FIELD, fed: render/airField.ts is a small fluid simulation of the air over the whole level floor; this tells it, once a tick, THE MAP (one byte a cell:
// SOLID where a wall, a piece of furniture or a door's leaf stands in the wind, otherwise the share of the ambient wind forced there) and THE WIND (the sources the tick left as
// they blow this tick, the storm's as the ambient, and whatever shoved the air: a blast's puff). It is stepped while something reads it (`want`, the snow's flakes and wisps):
// a level whose air nothing rides pays nothing. Still air is not stepped either. Picture only: nothing comes back from it.
import { BoxFlag, type Box } from '../scene/boxes';
import { WORLD } from '../scene/world';
import { snowPaintOf } from '../render/snowField';
import { AirField } from '../render/airField';
import type { Engine } from '../engine';
import type { Drawn } from '../draw/drawn';
import type { DrawWinds } from '../draw/winds';
import { Air, Leaf, Rect, Section, Word } from '../draw/sections';

/** cells a metre (render/airField.ts AIR_RES and particles.wgsl AIR_CELLS_PER_M say the same), local sources a tick (AIR_SRC_MAX) */
export const AIR_CELLS = 5, AIR_SOURCES = 16;
export const AIR_SIM = {
  ground: 0.55,      // the storm's vector at the ground, as a share (wind_field.wgsl's height profile at y = 0)
  minTop: 0.5,       // m over the floor a box must top to stand in the wind …
  maxFoot: 0.45,     // … and m it must reach down to (a lintel, a canopy, a wall light: air passes under)
  roofShare: 0,      // share of the ambient forced under a roof (0: indoors the air moves only as it is driven in)
  leafStep: 0.04,    // rad: a leaf must have swung this far to be drawn again
  ventOut: 0.5,      // of a ceiling diffuser's speed where its jet meets the floor: how fast the air spreads out across the floor
  gustK: 0.7,        // × a door's / pane's / wall vent's own speed, as driven air at the ground
  rate: 5,           // 1/s: how hard a source pulls the air at its centre
  sigmaMin: 0.35, sigmaMax: 2.5,   // m: a source's footprint
  puffRate: 14,      // 1/s: … and a puff (a blast's shove) pulls harder: it is over in a third of a second
  settle: 6,         // s the field goes on being stepped after the last source has stopped
};

const CAND = 8;   // floats a candidate source: distance from the view, x, z, vx, vz, sigma, out, rate
const HIDDEN = BoxFlag.NoPrimary | BoxFlag.NoShadow;
/** the flags `stands` reads: a box that only changes its gloss or its glow draws nothing again */
const READ = BoxFlag.NoPrimary | BoxFlag.NoShadow | BoxFlag.Glass;
interface AirStats { calm: number; speed: number; wind: [number, number]; sources: number }

/** draw one box's footprint solid into `m`. A box thinner than a cell is padded to one, a hair over half a cell either side of its middle, so one lying exactly on a cell edge
 *  takes both neighbours and a wall does not leak; a yawed one to a line whose cells share edges */
export function stamp(m: Uint8Array, w: number, h: number, cx: number, cz: number, hx: number, hz: number, yaw: number): void {
  const cell = 1 / AIR_CELLS, c = Math.cos(yaw), s = Math.sin(yaw), square = Math.abs(c * s) < 0.02;
  const need = cell * (square ? 0.5 : 0.75) + 1e-4, ex = Math.max(hx, need), ez = Math.max(hz, need);
  const rx = Math.abs(c) * ex + Math.abs(s) * ez, rz = Math.abs(s) * ex + Math.abs(c) * ez, ox = WORLD.min[0], oz = WORLD.min[2];
  const i0 = Math.max(0, Math.floor((cx - rx - ox) * AIR_CELLS)), i1 = Math.min(w - 1, Math.floor((cx + rx - ox) * AIR_CELLS));
  const j0 = Math.max(0, Math.floor((cz - rz - oz) * AIR_CELLS)), j1 = Math.min(h - 1, Math.floor((cz + rz - oz) * AIR_CELLS));
  for (let j = j0; j <= j1; j++) for (let i = i0; i <= i1; i++) {
    const px = ox + (i + 0.5) * cell - cx, pz = oz + (j + 0.5) * cell - cz, lx = c * px - s * pz, lz = s * px + c * pz;
    if (Math.abs(lx) <= ex && Math.abs(lz) <= ez) m[j * w + i] = 255;
  }
}

/** THE MAP: what stands in the wind, of the statics as told, the panes the air goes through and the leaves where they hang */
export class AirMap {
  readonly w = Math.round((WORLD.max[0] - WORLD.min[0]) * AIR_CELLS); readonly h = Math.round((WORLD.max[2] - WORLD.min[2]) * AIR_CELLS);
  readonly base = new Uint8Array(this.w * this.h); readonly mask = new Uint8Array(this.w * this.h);
  private key = NaN; private leafKey = NaN;
  constructor(private D: Drawn, private boxes: readonly Box[]) {}
  private open(): Set<number> { const D = this.D, s = new Set<number>(); for (let i = 0, o = D.at[Section.openPanes]; i < D.n[Section.openPanes]; i++, o += Word.row) s.add(D.u[o + Word.w]); return s; }
  /** does this static stand in the wind at the ground */
  private stands(b: Box, i: number, open: Set<number>): boolean {
    if (b.rot || (b.flags & HIDDEN) === HIDDEN || (b.flags & BoxFlag.NoShadow) !== 0) return false;   // (tilted props, what a blast took, decals and glow cards)
    const y0 = b.c[1] - b.h[1], y1 = b.c[1] + b.h[1];
    if (!(y1 >= AIR_SIM.minTop) || !(y0 <= AIR_SIM.maxFoot)) return false;
    if (b.flags & BoxFlag.Glass) return !open.has(i);
    return snowPaintOf(b) === null;   // (placed snow is ground the wind runs over, however high the bank)
  }
  /** the map is as the level stands now; true = it is another than it was */
  redraw(): boolean {
    const D = this.D, f = D.f, K = AIR_SIM, boxes = this.boxes, open = this.open();
    let key = boxes.length * 31 + open.size | 0, changed = false;
    for (const i of open) key = (Math.imul(key, 33) + i) | 0;
    for (let i = 0; i < boxes.length; i++) { const b = boxes[i]; key = (Math.imul(key, 33) + (b.flags & READ) + (b.rot ? 7919 : 0) + Math.round((b.c[0] + b.h[0] * 3 + b.c[2] * 7 + b.h[2] * 13 + b.c[1] * 17 + b.h[1] * 19 + b.yaw * 23) * 50)) | 0; }
    if (key !== this.key) {
      this.key = key; this.leafKey = NaN; changed = true;
      const { w, h, base } = this, ox = WORLD.min[0], oz = WORLD.min[2], cell = 1 / AIR_CELLS, under = Math.max(0, Math.min(254, Math.round(K.roofShare * 254)));
      const c0 = D.at[Section.ceilings], nc = D.n[Section.ceilings];
      for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) {
        const x = ox + (i + 0.5) * cell, z = oz + (j + 0.5) * cell; let r = false;
        for (let k = 0, o = c0; k < nc; k++, o += Rect.row) if (x > f[o + Rect.r] && z > f[o + Rect.r + 1] && x < f[o + Rect.r + 2] && z < f[o + Rect.r + 3]) { r = true; break; }
        base[j * w + i] = r ? under : 254;
      }
      for (let i = 0; i < boxes.length; i++) { const b = boxes[i]; if (this.stands(b, i, open)) stamp(base, w, h, b.c[0], b.c[2], b.h[0], b.h[2], b.yaw); }
    }
    const n = D.n[Section.doorBoxes], l0 = D.at[Section.doorBoxes]; let lk = n;
    for (let i = 0, o = l0; i < n; i++, o += Leaf.row) lk = (Math.imul(lk, 33) + Math.round(f[o + Leaf.yaw] / K.leafStep)) | 0;
    if (lk !== this.leafKey) {
      this.leafKey = lk; changed = true; this.mask.set(this.base);
      for (let i = 0, o = l0; i < n; i++, o += Leaf.row) { if (!(f[o + Leaf.c + 1] + f[o + Leaf.h + 1] >= K.minTop)) continue;   // (a leaf off its hinges lies on the floor: the doorway is open)
        stamp(this.mask, this.w, this.h, f[o + Leaf.c], f[o + Leaf.c + 2], f[o + Leaf.h], f[o + Leaf.h + 2], f[o + Leaf.yaw]); }
    }
    return changed;
  }
}

export class AirPicture {
  readonly map: AirMap;
  readonly stats: AirStats = { calm: 0, speed: 0, wind: [0, 0], sources: 0 };
  private src = new Float32Array(AIR_SOURCES * 8); private cand = new Float32Array(32 * CAND);
  private puffs: { x: number; z: number; out: number; sigma: number; rate: number; t: number; secs: number }[] = [];
  private wanted = false; private sent = false; private quiet = 1e9; private fresh = true; private lastT = -1;
  constructor(private E: Engine, private D: Drawn, boxes: readonly Box[], private winds: DrawWinds) { this.map = new AirMap(D, boxes); this.attach(); }
  /** the page's engine gets its field. A device that cannot build it leaves the engine without one, and every reader back on the wind as a formula */
  private attach(): void {
    const E = this.E, dev = E.device, scoped = typeof dev.pushErrorScope === 'function' && typeof dev.popErrorScope === 'function';
    try {
      if (scoped) { dev.pushErrorScope('validation'); dev.pushErrorScope('out-of-memory'); }
      const field = new AirField(dev, WORLD.max[0] - WORLD.min[0], WORLD.max[2] - WORLD.min[2]); E.air = field;
      if (scoped) { const drop = (err: GPUError | null) => { if (err && E.air === field) { E.air = null; console.warn('air field: not built on this device', err.message); } }; void dev.popErrorScope().then(drop, () => {}); void dev.popErrorScope().then(drop, () => {}); }
    } catch (err) { if (scoped) { try { void dev.popErrorScope(); void dev.popErrorScope(); } catch { /* the scopes went with the throw */ } } console.warn('air field: not built on this device', err); }
  }
  /** is there a field on this page for anything to ride */
  get live(): boolean { return !!this.E.air && this.E.settings.airSim !== false; }
  /** a reader asks for the air this tick: the map is drawn and the field stepped */
  want(): void { this.wanted = true; }
  /** the map goes to the field again: the field holds another moment's (a load, a game put back) */
  cut(): void { this.sent = false; this.fresh = true; }
  /** A PUFF: the air at (x, z) is driven outward for `secs`, `out` m/s at one `sigma` from its middle, running down to nothing over that time */
  puff(x: number, z: number, out: number, sigma: number, secs: number, rate = AIR_SIM.puffRate): void {
    if (!(out > 0) || !(secs > 0)) return;
    if (this.puffs.length >= AIR_SOURCES) this.puffs.shift();
    this.puffs.push({ x, z, out, sigma, rate, t: 0, secs });
  }
  /** the sources as the field's local ones (at most AIR_SOURCES records of 8: x, z, vx, vz, sigma, out, rate, 0), nearest the view first; returns how many */
  private sources(focus: ArrayLike<number>, ambient: [number, number]): number {
    const K = AIR_SIM, out = this.src, fx = focus[0], fz = focus[2]; ambient[0] = 0; ambient[1] = 0;
    let cand = this.cand, m = 0;
    const push = (d: number, x: number, z: number, vx: number, vz: number, sg: number, o: number, rate = K.rate) => {
      if ((m + 1) * CAND > cand.length) { const big = new Float32Array(cand.length * 2); big.set(cand); cand = this.cand = big; }
      const k = m++ * CAND; cand[k] = d; cand[k + 1] = x; cand[k + 2] = z; cand[k + 3] = vx; cand[k + 4] = vz; cand[k + 5] = sg; cand[k + 6] = o; cand[k + 7] = rate;
    };
    for (const s of this.winds.active.list) {
      if (!(s.speed > 0)) continue;
      if (s.kind === 'storm') { ambient[0] += s.dir[0] * s.speed * K.ground; ambient[1] += s.dir[2] * s.speed * K.ground; continue; }
      const floorY = s.floorY !== undefined && s.floorY > -100 ? s.floorY : 0, up = s.pos[1] - floorY;
      if (s.dir[1] < -0.5) {   // a ceiling diffuser: where its axis meets the floor, the air spreads out
        const t = up / -s.dir[1]; if (!(t < s.dMax)) continue;
        const x = s.pos[0] + s.dir[0] * t, z = s.pos[2] + s.dir[2] * t, q = t / Math.max(s.reach, 1e-3), sp = s.speed / (1 + q * q), sg = Math.max(K.sigmaMin, Math.min(K.sigmaMax, s.r0 + t * s.tanSpread));
        push(Math.hypot(x - fx, z - fz), x, z, s.dir[0] * sp * K.gustK, s.dir[2] * sp * K.gustK, sg, sp * K.ventOut);
      } else {                 // a gust through a door or a pane, a wall vent: driven air along its heading, less the higher over the ground it blows
        const hl = Math.hypot(s.dir[0], s.dir[2]); if (hl < 0.2) continue;
        const sg = Math.max(K.sigmaMin, Math.min(K.sigmaMax, s.r0 + 0.35 * s.reach * s.tanSpread)), k = K.gustK * Math.max(0.2, Math.min(1, 1 - (up - 1.5) / 1.5)), x = s.pos[0] + s.dir[0] / hl * sg, z = s.pos[2] + s.dir[2] / hl * sg;
        push(Math.hypot(x - fx, z - fz), x, z, s.dir[0] * s.speed * k, s.dir[2] * s.speed * k, sg, 0);
      }
    }
    for (const p of this.puffs) { const k = 1 - p.t / p.secs; if (k > 0) push(Math.hypot(p.x - fx, p.z - fz), p.x, p.z, 0, 0, Math.max(K.sigmaMin, Math.min(K.sigmaMax, p.sigma)), p.out * k, p.rate); }
    const n = Math.min(m, AIR_SOURCES);
    for (let i = 0; i < n; i++) {
      if (m > AIR_SOURCES) { let best = i; for (let j = i + 1; j < m; j++) if (cand[j * CAND] < cand[best * CAND]) best = j; if (best !== i) for (let q = 0; q < CAND; q++) { const t = cand[i * CAND + q]; cand[i * CAND + q] = cand[best * CAND + q]; cand[best * CAND + q] = t; } }
      const k = i * CAND, o = i * 8; out[o] = cand[k + 1]; out[o + 1] = cand[k + 2]; out[o + 2] = cand[k + 3]; out[o + 3] = cand[k + 4]; out[o + 4] = cand[k + 5]; out[o + 5] = cand[k + 6]; out[o + 6] = cand[k + 7]; out[o + 7] = 0;
    }
    return n;
  }
  /** once a tick */
  tick(dt: number, time: number, focus: ArrayLike<number>): void {
    const E = this.E, air = E.air, S = this.stats;
    if (!this.live || !air) { this.sent = false; this.wanted = false; this.fresh = true; this.puffs.length = 0; E.airWind[0] = E.airWind[1] = 0; E.airOn = false; return; }
    const M = this.map;
    if (air.w !== M.w || air.h !== M.h) { E.airOn = false; return; }
    if (this.fresh || time < this.lastT) { air.reset(); this.fresh = false; this.sent = false; this.quiet = 1e9; this.puffs.length = 0; E.powder?.reset(); }
    this.lastT = time;
    const wanted = this.wanted; this.wanted = false;
    const n = this.sources(focus, S.wind);
    if (this.puffs.length) { for (const p of this.puffs) p.t += Math.max(0, dt); this.puffs = this.puffs.filter(p => p.t < p.secs); }   // (this tick's share is in the sources above)
    // the ambient before its gusting: what a reader calls "the wind's own speed"
    let calm = 0; const air0 = this.D.at[Section.air];
    for (const s of this.winds.active.list) if (s.kind === 'storm' && s.speed > 0) calm = Math.max(calm, s.speed / Math.max(this.D.f[air0 + Air.stormGust], 0.2) * AIR_SIM.ground);
    S.calm = calm; S.sources = n; S.speed = Math.hypot(S.wind[0], S.wind[1]);
    this.quiet = S.speed > 0 || n > 0 ? 0 : this.quiet + Math.max(0, dt);
    if (!wanted) { E.airOn = false; return; }
    if (M.redraw() || !this.sent) { air.setCells(M.mask); this.sent = true; }
    air.setSources?.(this.src, n);
    E.airWind[0] = S.wind[0]; E.airWind[1] = S.wind[1];
    E.airOn = this.quiet < AIR_SIM.settle;
  }
}
