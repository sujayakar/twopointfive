// WHAT STANDS UNDER AND OVER A POINT, of the statics a page has told: a level's plan binned once, asked straight up and down. The picture's own plans ask this; no rule does.
import { BoxFlag, type Box } from '../scene/boxes';
import { WORLD } from '../scene/world';

const CELL = 2;

export class PlanRays {
  private cells: number[][]; private w: number; private h: number;
  constructor(private boxes: readonly Box[]) {
    this.w = Math.ceil((WORLD.max[0] - WORLD.min[0]) / CELL) + 1; this.h = Math.ceil((WORLD.max[2] - WORLD.min[2]) / CELL) + 1;
    this.cells = Array.from({ length: this.w * this.h }, () => []);
    for (let i = 0; i < boxes.length; i++) { const b = boxes[i]; if (b.flags & BoxFlag.NoShadow) continue;
      const r = Math.hypot(b.h[0], b.h[2]); const x0 = this.at(b.c[0] - r, 0), x1 = this.at(b.c[0] + r, 0), z0 = this.at(b.c[2] - r, 2), z1 = this.at(b.c[2] + r, 2);
      for (let z = z0; z <= z1; z++) for (let x = x0; x <= x1; x++) this.cells[z * this.w + x].push(i); }
  }
  private at(v: number, k: 0 | 2): number { return Math.max(0, Math.min((k === 0 ? this.w : this.h) - 1, Math.floor((v - WORLD.min[k]) / CELL))); }
  private cell(x: number, z: number): number[] { return this.cells[this.at(z, 2) * this.w + this.at(x, 0)]; }
  /** is (x, z) in its plan */
  private over(b: Box, x: number, z: number): boolean { if (b.cut) return false;   // (a cut thing's shape is no plan's: a car's bonnet is a box here only to the renderer)
    if (b.round) return Math.hypot(x - b.c[0], z - b.c[2]) < b.h[0]; const c = Math.cos(b.yaw), s = Math.sin(b.yaw), px = x - b.c[0], pz = z - b.c[2]; return Math.abs(c * px - s * pz) < b.h[0] && Math.abs(s * px + c * pz) < b.h[2]; }
  /** is the point inside a box, other than `not` */
  inside(x: number, y: number, z: number, not: number): boolean { for (const i of this.cell(x, z)) { const b = this.boxes[i]; if (i !== not && !b.rot && y > b.c[1] - b.h[1] && y < b.c[1] + b.h[1] && this.over(b, x, z)) return true; } return false; }
  /** how far down from (x, y, z) the first solid is, within `most`, or Infinity */
  down(x: number, y: number, z: number, most: number): number { let best = Infinity; for (const i of this.cell(x, z)) { const b = this.boxes[i]; if (b.rot || !this.over(b, x, z)) continue; const t = y - (b.c[1] + b.h[1]); if (t >= 0 && t < most && t < best) best = t; } return best; }
  /** does anything stand over (x, y, z), up to `top`. As a line of the game's: either end drawn in by `shrink`, and a box it starts well inside is no box in its way */
  covered(x: number, y: number, z: number, top: number, shrink = 0.02): boolean {
    const y0 = y + shrink + 1e-4, y1 = top - shrink;
    for (const i of this.cell(x, z)) { const b = this.boxes[i]; if (b.rot || !this.over(b, x, z)) continue;
      const lo = b.c[1] - b.h[1], hi = b.c[1] + b.h[1]; if (!(hi > y0 && lo < y1)) continue;
      const c = Math.cos(b.yaw), s = Math.sin(b.yaw), px = x - b.c[0], pz = z - b.c[2], depth = Math.min(b.h[0] - Math.abs(c * px - s * pz), b.h[1] - Math.abs(y - b.c[1]), b.h[2] - Math.abs(s * px + c * pz));
      if (depth > shrink) continue;
      return true; }
    return false;
  }
}
