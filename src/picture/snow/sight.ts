// WHETHER THE TEAM HAS EYES ON THE GROUND, as the picture asks it of a mark a hidden man made: the fog's own eyes (their row) walked over a plan of what stops a level line at eye height.
// The fog's mask is the GPU's and the rules' lines are the game's; this is the page's own reading of the same eyes and walls, for a print found or not.
import { BoxFlag, type Box } from '../../scene/boxes';
import { WORLD } from '../../scene/world';
import type { Drawn } from '../../draw/drawn';
import { Fog, Leaf, Section } from '../../draw/sections';
import { AIR_CELLS, stamp } from '../air';
import type { Sight } from './tracks';

const EYE = 1.4, THROUGH = BoxFlag.Glass | BoxFlag.NoShadow;

export class SnowSight implements Sight {
  private w = Math.round((WORLD.max[0] - WORLD.min[0]) * AIR_CELLS); private h = Math.round((WORLD.max[2] - WORLD.min[2]) * AIR_CELLS);
  private walls = new Uint8Array(this.w * this.h); private cells = new Uint8Array(this.w * this.h); private key = NaN;
  constructor(private D: Drawn, private boxes: readonly Box[]) {}
  get active(): boolean { const D = this.D; return D.n[Section.fog] > 0 && D.u[D.at[Section.fog] + Fog.on] !== 0; }
  /** what stops a line now */
  private map(): void {
    const D = this.D, f = D.f, n = D.n[Section.doorBoxes], l0 = D.at[Section.doorBoxes];
    let key = this.boxes.length * 31 + n | 0; for (let i = 0; i < this.boxes.length; i++) { const b = this.boxes[i]; key = (Math.imul(key, 33) + (b.flags & THROUGH) + Math.round((b.c[0] + b.h[0] * 3 + b.c[2] * 7 + b.h[2] * 13) * 50)) | 0; }
    if (key !== this.key) { this.key = key; this.walls.fill(0); for (const b of this.boxes) { if (b.rot || (b.flags & THROUGH) || b.c[1] + b.h[1] < EYE || b.c[1] - b.h[1] > EYE) continue; stamp(this.walls, this.w, this.h, b.c[0], b.c[2], b.h[0], b.h[2], b.yaw); } }
    this.cells.set(this.walls);
    for (let i = 0, o = l0; i < n; i++, o += Leaf.row) if (f[o + Leaf.c + 1] + f[o + Leaf.h + 1] >= EYE) stamp(this.cells, this.w, this.h, f[o + Leaf.c], f[o + Leaf.c + 2], f[o + Leaf.h], f[o + Leaf.h + 2], f[o + Leaf.yaw]);
  }
  private tickMapped = -1;
  private blocked(ax: number, az: number, bx: number, bz: number): boolean {
    const n = Math.ceil(Math.hypot(bx - ax, bz - az) * AIR_CELLS * 2) + 1, ox = WORLD.min[0], oz = WORLD.min[2];
    for (let i = 0; i <= n; i++) { const t = i / n, x = Math.floor((ax + (bx - ax) * t - ox) * AIR_CELLS), z = Math.floor((az + (bz - az) * t - oz) * AIR_CELLS); if (x < 0 || z < 0 || x >= this.w || z >= this.h) continue; if (this.cells[z * this.w + x]) return true; }
    return false;
  }
  /** a new tick: the map is read again before the first line of it */
  fresh(): void { this.tickMapped = -1; }
  /** 2 an eye has the spot (within its reach, or in its field with a clear line) · 1 in an eye's field and every such eye's line stopped · 0 in no eye's field */
  groundLook(x: number, z: number): 0 | 1 | 2 {
    const D = this.D, f = D.f, u = D.u, o = D.at[Section.fog], count = u[o + Fog.count]; let r: 0 | 1 = 0;
    if (this.tickMapped < 0) { this.map(); this.tickMapped = 0; }
    for (let i = 0; i < count; i++) {
      const e = o + Fog.eyes + i * 4, k = o + Fog.looks + i * 4, ex = f[e], ez = f[e + 2], dx = x - ex, dz = z - ez, d = Math.hypot(dx, dz); if (d < 0.4) return 2;
      const cos = f[k + 2], near = f[k + 3]; if (!(cos <= -2 || d < near || (dx * f[k] + dz * f[k + 1]) / d > cos)) continue;
      r = 1; const t = 1 - 0.12 / d; if (!this.blocked(ex, ez, ex + dx * t, ez + dz * t)) return 2;
    }
    return r;
  }
}
