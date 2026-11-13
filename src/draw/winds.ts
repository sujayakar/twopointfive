// THE WIND'S LISTS, handed over: what blows goes to the smoke's solver, what the tick picked for the particles and the fires' hot air to the particle pool. The renderer takes each
// source as an object; these are made once and written over, so a tick makes none.
import type { Particles } from '../render/particles';
import type { SmokeSystem } from '../smoke/smoke';
import type { WindGpuSource } from '../smoke/types';
import type { Drawn } from './drawn';
import { WindFlag } from './bits';
import { WIND_KINDS } from './names';
import { Air, Rect, Section, Wind } from './sections';

const blank = (): WindGpuSource => ({ kind: 'jet', name: '', pos: [0, 0, 0], dir: [0, 0, 0], speed: 0, reach: 0, tanSpread: 0, r0: 0, dMax: 0, bound: { c: [0, 0, 0], r: 0 } });

/** one section's sources as the renderer takes them */
class WindList {
  readonly list: WindGpuSource[] = [];
  private made: WindGpuSource[] = [];
  private roofs: number[][][] = [];
  constructor(private D: Drawn, private section: number) {}
  read(): readonly WindGpuSource[] {
    const D = this.D, f = D.f, u = D.u, L = this.list, n = D.n[this.section]; L.length = n;
    for (let i = 0, o = D.at[this.section]; i < n; i++, o += Wind.row) {
      const s = L[i] = this.made[i] ??= blank(), says = u[o + Wind.says], fy = f[o + Wind.floorY];
      s.kind = WIND_KINDS[u[o + Wind.kind]];
      s.name = s.kind === 'storm' ? 'storm' : '';
      for (let k = 0; k < 3; k++) { s.pos[k] = f[o + Wind.pos + k]; s.dir[k] = f[o + Wind.dir + k]; (s.bound.c as number[])[k] = f[o + Wind.bound + k]; }
      s.bound.r = f[o + Wind.bound + 3];
      s.speed = f[o + Wind.speed]; s.reach = f[o + Wind.reach]; s.tanSpread = f[o + Wind.tanSpread]; s.r0 = f[o + Wind.r0]; s.dMax = f[o + Wind.dMax];
      s.floorY = fy === fy ? fy : undefined;
      s.kinds = u[o + Wind.kinds] || undefined;
      s.field = says & WindFlag.Field ? true : undefined; s.heat = says & WindFlag.Heat ? true : undefined;
      if (s.kind === 'storm') { s.interior = rect(f, o + Wind.interior, s.interior); s.annex = rect(f, o + Wind.annex, s.annex); s.roofs = u[o + Wind.roofs] ? this.roofsOf(i, u[o + Wind.roofsAt], u[o + Wind.roofs]) : undefined; }
    }
    return L;
  }
  private roofsOf(i: number, at: number, n: number): number[][] {
    const D = this.D, R = this.roofs[i] ??= []; R.length = n;
    for (let k = 0, o = D.at[Section.windRoofs] + at * Rect.row; k < n; k++, o += Rect.row) rect(D.f, o, R[k] ??= [0, 0, 0, 0]);
    return R;
  }
}
const rect = <T extends ArrayLike<number> | undefined>(f: ArrayLike<number>, o: number, to: T): [number, number, number, number] => { const r = (to ?? [0, 0, 0, 0]) as unknown as [number, number, number, number]; for (let k = 0; k < 4; k++) r[k] = f[o + k]; return r; };

export class DrawWinds {
  readonly active: WindList; readonly gusts: WindList; readonly heat: WindList;
  private knobs: [number, number, number] = [0, 0, 0];
  constructor(private D: Drawn, private particles: Particles | null, private smoke: SmokeSystem) { this.active = new WindList(D, Section.winds); this.gusts = new WindList(D, Section.gusts); this.heat = new WindList(D, Section.heat); }
  /** a tick has run */
  from(): void {
    const D = this.D, o = D.at[Section.air];
    for (let k = 0; k < 3; k++) this.knobs[k] = D.f[o + Air.knobs + k];
    this.particles?.setWind(this.gusts.read(), this.knobs);
    this.smoke.setWind(this.active.read(), D.f[o + Air.relax]);
  }
}
