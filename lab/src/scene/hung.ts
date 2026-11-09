// WHAT HANGS, and the wind moves: paper pinned at its top, cloth, pennants, bulbs on a string. A chain of plates on hinges, each with weight; the air presses on their
// faces, and they swing on. Stepped once a tick; no clock, no dice.
import { type V3, type Rot, rotX, rotY, compose, apply, add, mul, dot } from '../math';
import { World, F, type Piece, type Mark } from './world';
import { gustAt } from './plants';
const SIGN = apply(rotX(0.3), [0, -1, 0])[2]! > 0 ? 1 : -1;
export interface HungSpec { hinge: V3; yaw: number; width: number; height: number; n: number; mat: number; tint: V3; art?: string; back?: boolean; /** it hangs on a wall: it can only lift away from it */ wall?: boolean; stiff?: number; weight?: number; thick?: number; taper?: number; flags?: number; under?: number }
export class Hung {
  private a: number[]; private v: number[]; readonly segs: Piece[]; private r: Rot; private nrm: V3; private l: number; lamp = -1; bulb: Piece | null = null;
  constructor(private w: World, private s: HungSpec) {
    this.r = rotY(s.yaw); this.nrm = apply(this.r, [0, 0, 1]); this.l = s.height / s.n; this.a = Array(s.n).fill(s.wall ? 0.02 : 0); this.v = Array(s.n).fill(0);
    this.segs = Array.from({ length: s.n }, (_, i) => { const marks: Mark[] = []; if (s.art) for (const face of s.back ? [5, 4] : [5]) marks.push({ face, at: [0, -s.height / 2 + this.l * (i + 0.5)], size: [s.width, s.height], art: s.art, color: [1, 1, 1] }); return w.add({ at: s.hinge, half: [s.width / 2 * (1 - (s.taper ?? 0) * (i + 0.5) / s.n), this.l / 2 + 0.001, s.thick ?? 0.0015], mat: s.mat, tint: s.tint, flags: (s.flags ?? F.CUT) | F.MOVES, rig: 'hung', marks }); }); this.pose();
  }
  step(dt: number, t: number, wind: V3, calm: number) {
    const s = this.s, g = gustAt(s.hinge, t, wind), flutter = Math.sin(t * 9.1 + s.hinge[0] * 3.7 + s.hinge[2] * 2.1) * 0.5 + Math.sin(t * 15.7 + s.hinge[1] * 5.0 + s.hinge[0]) * 0.35, air = (dot(wind, this.nrm) * (1.2 + 1.6 * g) + flutter * Math.hypot(...wind) * (0.5 + g) * 0.9) * calm * 2.4, weight = s.weight ?? 0.25, stiff = s.stiff ?? 14;
    for (let k = 0; k < 2; k++) for (let i = 0; i < s.n; i++) { const h = dt / 2, a = this.a[i]!, up = i ? this.a[i - 1]! : 0, down = i + 1 < s.n ? this.a[i + 1]! : a, rel = air * Math.cos(a) - this.v[i]! * this.l * (i + 1) * 0.8, f = rel * Math.abs(rel) * 1.1 / weight; this.v[i]! += h * (-9.8 * 0.9 * Math.sin(a) / Math.max(0.12, this.l * (s.n - i) * 0.7) + f - stiff * (a - up) * (i ? 1 : 0.15) - stiff * (a - down) - this.v[i]! * 1.6); this.a[i] = a + h * this.v[i]!; if (s.wall && this.a[i]! < 0) { this.a[i] = 0; if (this.v[i]! < 0) this.v[i]! *= -0.15; } if (Math.abs(this.a[i]!) > 2.2) { this.a[i] = 2.2 * Math.sign(this.a[i]!); this.v[i] = 0; } }
    this.pose();
  }
  pose() { let p = this.s.hinge; this.segs.forEach((q, i) => { const a = this.a[i]! * SIGN, rot = compose(this.r, rotX(a)), d = apply(rot, [0, -this.l, 0]); q.at = add(p, mul(d, 0.5)); q.rot = rot; p = add(p, d); }); if (this.lamp >= 0) this.w.lamps[this.lamp]!.at = p; if (this.bulb) this.bulb.at = p; }
}
/** a small lamp on a short wire */
export function hungBulb(w: World, hinge: V3, drop: number, r: number, rad: V3, name: string): Hung { const h = new Hung(w, { hinge, yaw: 0.6, width: 0.006, height: drop, n: 1, mat: 0, tint: [0.05, 0.05, 0.05], stiff: 0.5, weight: 2.2, thick: 0.003 }), m = Math.max(...rad); h.lamp = w.lamp({ kind: 'sphere', at: hinge, u: [r, 0, 0], v: [0, 0, 0], radiance: rad, name, on: 1, night: true }); h.bulb = w.add({ at: hinge, half: [r, r * 1.25, r], mat: 12, tint: [Math.sqrt(rad[0] / m), Math.sqrt(rad[1] / m), Math.sqrt(rad[2] / m)], flags: F.CUT | F.GLOW | F.NOSHADOW | F.MOVES, lamp: h.lamp, seed: Math.round(m * 10), quad: [1 / r ** 2, 1 / (r * 1.25) ** 2, 1 / r ** 2, 0], rig: 'hung' }); h.pose(); return h; }
