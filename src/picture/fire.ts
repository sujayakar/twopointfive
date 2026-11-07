// AN OPEN FIRE, AS THE PICTURE HAS IT. The lamp, its flicker, the coals and the embers are the rules' and reach the page as lights, boxes and events; each fire's flame is a record
// the composite draws (the `fires` section, closed forms of the game's clock). What is the picture's alone is here: which fires are near enough to draw, and the REAL GAS over
// them, filed on the page's own solver (hot, as wide as the drum, climbing and leaning in the wind), because a solver drawing, the graphics preset on and where the view looks are
// the page's to say. No rule hears of it (a LOOK source). `gas` and `soot` ease by this book, which a load or a restart starts over.
import { lookConfig } from '../render/frame';
import type { FireRecord } from '../render/passes';
import type { Particles } from '../render/particles';
import type { SmokeSystemLike, WindGpuSource } from '../smoke/types';
import type { Engine } from '../engine';
import type { Drawn } from '../draw/drawn';
import { Fire, FireTable, Section, View } from '../draw/sections';

/** integer hash to [0, 1) (rules: fire h01) */
function h01(x: number): number { x = Math.imul((x ^ 0x2545F491) >>> 0, 0x9E3779B1) >>> 0; x ^= x >>> 15; x = Math.imul(x, 0x85EBCA77) >>> 0; x ^= x >>> 13; return (x >>> 8) / 16777216; }

const enum Smoke { on, up, radius, cells, density, temp, speed, push, jitter, every, viewR, lean }
const enum Gas { on, up, radius, cells, density, temp, speed, push, jitter, every, viewR, keep, ease }
const enum Column { up, radius, density, sootEase, puffEvery, puffSecs, puffUp, puffRadius, puffOff, puffDensity, puffTemp, puffSpeed, puffPush, puffExpand }

/** a frame's book: by a fire's place among the level's, the tick it last had its gas kept up, how far it has eased to it */
interface Book { n: number; time: number; smoke: number[]; gas: number[]; gasMix: number[]; sootMix: number[]; kept: number }
const fresh = (): Book => ({ n: 0, time: 0, smoke: [], gas: [], gasMix: [], sootMix: [], kept: -1 });
const blank = (): FireRecord & { d: number } => ({ pos: [0, 0, 0], r: 0, h: 0, heat: 0, seed: 0, time: 0, wind: [0, 0], flick: 1, bed: 0, fog: 0, gas: 0, soot: 0, d: 0 });

export class FirePicture {
  private book = fresh(); private rows: (FireRecord & { d: number })[] = []; private list: (FireRecord & { d: number })[] = []; private look: number[] = [];
  constructor(private E: Engine, private D: Drawn, private smoke: SmokeSystemLike, private particles: Particles | null) {}
  /** a load, a restart: what was filed is gone with the solver */
  cut(): void { this.book = fresh(); }
  /** the flame's own middle `up` metres over its mouth, as the picture leans it: how far downwind it stands (x, z), and its slope there */
  private axis(up: number, wx: number, wz: number): [number, number, number, number] {
    const D = this.D, t = D.at[Section.fireTable], H = D.f[t + FireTable.height] * D.f[t + FireTable.tall], lean = D.f[t + FireTable.lean], y = Math.max(up, 0) / H, m = Math.min(y, 1.5);
    const b = lean * (m * Math.sqrt(m) + 0.735 * Math.max(y - 1.5, 0)), sl = lean * (y < 1.5 ? 1.5 * Math.sqrt(m) : 0.735) / H;
    return [wx * b, wz * b, wx * sl, wz * sl];
  }
  /** a tick has run */
  tick(dt: number, focus: ArrayLike<number>, heat: readonly WindGpuSource[]): void {
    const D = this.D, f = D.f, u = D.u, E = this.E, n = D.n[Section.fires], time = u[D.at[Section.view] + View.tick] / 60, was = time - Math.max(dt, 0);
    this.particles?.setHeat(heat);
    if (!D.n[Section.fireTable]) { if (this.book.kept) { this.book.kept = 0; this.smoke.keepHot?.(0); } return; }
    let P = this.book; if (was < P.time - 1e-9) P = this.book = fresh();
    P.n++; P.time = time;
    const t = D.at[Section.fireTable], B = t + FireTable.smoke, G = t + FireTable.gas, C = t + FireTable.column, height = f[t + FireTable.height];
    const drawing = this.smoke.drawing?.() === true && E.settings.fireSmoke !== false, row = lookConfig.fireSmoke, fit = lookConfig.fireSmoke > 0 && lookConfig.fireFit > 0;
    const soot: [number, number, number] = [f[t + FireTable.soot], f[t + FireTable.soot + 1], f[t + FireTable.soot + 2]];
    for (let i = 0, o = D.at[Section.fires]; i < n; i++, o += Fire.row) {
      if (!u[o + Fire.lit]) continue;
      const ix = u[o + Fire.ix], mx = f[o + Fire.pos], my = f[o + Fire.pos + 1], mz = f[o + Fire.pos + 2], heat01 = f[o + Fire.heat], si = Math.floor(f[o + Fire.seed] * 65535);
      const wx = f[o + Fire.blows], wz = f[o + Fire.blows + 2], wl = Math.hypot(wx, wz), lx = f[o + Fire.wind], lz = f[o + Fire.wind + 1], d = Math.hypot(mx - focus[0], mz - focus[2]);
      const near = f[B + Smoke.on] !== 0 && drawing && d < f[B + Smoke.viewR];
      if (near) {
        const every = f[B + Smoke.every], slot = Math.floor(time / every), goes = P.smoke[ix] === P.n - 1, lean = f[B + Smoke.lean]; P.smoke[ix] = P.n;
        if (slot !== Math.floor(was / every) || !goes) {
          const sk = wl > 8 ? 8 / wl : 1, L = row > 0, A = L && fit ? this.axis(height * f[C + Column.up], lx, lz) : null;
          this.smoke.emit({ pos: [mx + (A ? A[0] : 0), my + height * (L ? f[C + Column.up] : f[B + Smoke.up]), mz + (A ? A[1] : 0)], dir: A ? [A[2], 1, A[3]] : [wx * sk * lean, 1, wz * sk * lean], speed: f[B + Smoke.speed], radius: L ? f[C + Column.radius] : f[B + Smoke.radius], minVoxels: f[B + Smoke.cells],
            density: f[B + Smoke.density] * heat01 * (L ? f[C + Column.density] : 1), temperature: f[B + Smoke.temp] * heat01, ttl: every, age: 0, kind: 'cloud', push: f[B + Smoke.push], jitter: f[B + Smoke.jitter], lattice: 'coarse', look: true, ...(A ? { tint: soot } : {}) });
        }
        // the fire puffs: a short burst of thicker, hotter gas that pushes the air round it apart, a few a second
        if (row > 0) { const pe = f[C + Column.puffEvery], ps = Math.floor(time / pe);
          if (ps !== Math.floor(was / pe) || !goes) {
            const a = Math.min(2, row), off = f[C + Column.puffOff], ox = (h01(si + ps * 31 + 11) - 0.5) * 2 * off, oz = (h01(si + ps * 31 + 12) - 0.5) * 2 * off, k = 0.7 + 0.6 * h01(si + ps * 31 + 13), pu = f[C + Column.puffUp], A = fit ? this.axis(height * pu, lx, lz) : null;
            this.smoke.emit({ pos: [mx + ox + (A ? A[0] : 0), my + height * pu, mz + oz + (A ? A[1] : 0)], dir: A ? [A[2] + ox, 1, A[3] + oz] : [wx * lean + ox, 1, wz * lean + oz], speed: f[C + Column.puffSpeed] * k, radius: f[C + Column.puffRadius], minVoxels: f[B + Smoke.cells], density: f[C + Column.puffDensity] * heat01 * a * k, temperature: f[C + Column.puffTemp] * heat01 * k,
              ttl: f[C + Column.puffSecs], age: 0, kind: 'cloud', push: f[C + Column.puffPush], expand: f[C + Column.puffExpand], attack: 0.06, jitter: 0.9, lattice: 'coarse', look: true, ...(A ? { tint: soot } : {}) });
          } }
      }
      P.sootMix[ix] = Math.min(1, Math.max(0, (P.sootMix[ix] ?? 0) + (fit && near ? dt : -dt) * f[C + Column.sootEase]));
      // the flame as gas: a short source on the solver's fine lattice at the coals; the picture eases to it
      const goes = drawing && f[G + Gas.on] !== 0 && d < f[G + Gas.viewR];
      if (goes) { const every = f[G + Gas.every], slot = Math.floor(time / every), kept = P.gas[ix] === P.n - 1; P.gas[ix] = P.n;
        if (slot !== Math.floor(was / every) || !kept) this.smoke.emit({ pos: [mx, my + height * f[G + Gas.up], mz], dir: [0, 1, 0], speed: f[G + Gas.speed], radius: f[G + Gas.radius], minVoxels: f[G + Gas.cells], density: f[G + Gas.density] * heat01, temperature: f[G + Gas.temp] * heat01, ttl: every, age: 0, kind: 'cloud', push: f[G + Gas.push], jitter: f[G + Gas.jitter], lattice: 'fine', look: true }); }
      P.gasMix[ix] = Math.min(1, Math.max(0, (P.gasMix[ix] ?? 0) + (goes ? dt : -dt) * f[G + Gas.ease]));
    }
    const hot = f[G + Gas.on] !== 0 && P.gas.some(x => x === P.n) ? f[G + Gas.keep] : 0;
    if (hot !== P.kept) { P.kept = hot; this.smoke.keepHot?.(hot); }
  }
  /** before a frame is drawn: every fire near the view, four at most, nearest first */
  frame(focus: ArrayLike<number>): void {
    const D = this.D, f = D.f, u = D.u, E = this.E, L = this.list; L.length = 0;
    if (D.n[Section.fireTable]) { const t = D.at[Section.fireTable], P = this.book;
      for (let i = 0, o = D.at[Section.fires]; i < D.n[Section.fires]; i++, o += Fire.row) {
        const r = this.rows[i] ??= blank(), ix = u[o + Fire.ix], lit = u[o + Fire.lit] !== 0;
        for (let k = 0; k < 3; k++) (r.pos as number[])[k] = f[o + Fire.pos + k];
        (r.wind as number[])[0] = f[o + Fire.wind]; (r.wind as number[])[1] = f[o + Fire.wind + 1];
        r.d = Math.hypot(f[o + Fire.pos] - focus[0], f[o + Fire.pos + 2] - focus[2]); if (!(r.d < f[t + FireTable.drawR])) continue;
        r.r = f[o + Fire.r]; r.h = f[o + Fire.h]; r.heat = f[o + Fire.heat]; r.seed = f[o + Fire.seed]; r.time = f[o + Fire.time]; r.flick = f[o + Fire.flick]; r.bed = f[o + Fire.bed]; r.fog = u[o + Fire.fog]; r.gas = lit ? P.gasMix[ix] ?? 0 : 0; r.soot = lit ? P.sootMix[ix] ?? 0 : 0;
        L.push(r);
      }
      if (L.length) { for (let k = 0; k < 16; k++) this.look[k] = f[t + FireTable.look + k]; E.fireLook = this.look; }
      const cap = f[t + FireTable.max]; if (L.length > cap) { L.sort((a, b) => a.d - b.d); L.length = cap; }   // (which ones, not in what order: the composite orders them ray by ray)
    }
    E.fires = L;
  }
}
