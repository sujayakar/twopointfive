// WHAT IS CARRIED: a person's limbs as soft boxes on their bones, for rays wider than a limb. Their parts are gone for such a ray (Piece.cls), and these stand for them. Kept in extra each frame.
import type { Lab } from '../lab';
import type { Person } from '../scene/people';
import { MATERIALS } from '../scene/materials';
import type { V3 } from '../math';
export const PEOPLE = 8, LIMBS = 14, LIMB_ROWS = 5, CARRY_ROWS = 1 + PEOPLE + PEOPLE * LIMBS * LIMB_ROWS;
/** each limb: the part whose frame it has, half its size there (or the part's own), how thick it is across x y z, what it is colored by, and from which level of the block on it stands for them */
const SPEC: { bone: string; half?: V3; at?: V3; tau: V3; of: string[]; level: number }[] = [
  { bone: 'chest', half: [0.20, 0.24, 0.20], at: [0, 0, -0.04], tau: [1.8, 1.5, 2.2], of: ['chest', 'plateF', 'plateB', 'pack', 'yoke'], level: 1 }, { bone: 'pelvis', half: [0.17, 0.17, 0.13], at: [0, 0.06, 0], tau: [1.8, 1.5, 2.0], of: ['pelvis', 'belly', 'belt'], level: 1 },
  { bone: 'head', half: [0.12, 0.17, 0.135], at: [0, 0.01, 0], tau: [1.6, 1.5, 1.6], of: ['helmet', 'head', 'jaw'], level: 0 }, { bone: 'recv', half: [0.03, 0.05, 0.33], at: [0, 0, 0.1], tau: [0.9, 1.0, 1.2], of: ['recv', 'rail', 'stock'], level: 0 },
  ...['L', 'R'].flatMap(s => [{ bone: 'arm' + s, tau: [2.2, 1.5, 2.2] as V3, of: ['arm' + s, 'biceps' + s, 'shoulder' + s], level: 0 }, { bone: 'fore' + s, tau: [2.0, 1.4, 2.0] as V3, of: ['fore' + s, 'hand' + s], level: 0 }, { bone: 'thigh' + s, tau: [2.2, 1.5, 2.2] as V3, of: ['thigh' + s, 'quad' + s], level: 0 }, { bone: 'shin' + s, tau: [2.0, 1.4, 2.0] as V3, of: ['shin' + s, 'calf' + s, 'cuff' + s], level: 0 }, { bone: 'boot' + s, tau: [2.0, 2.0, 2.2] as V3, of: ['boot' + s], level: 0 }]),
];
/** up to which class a part of a person is met as it is: what it is a part of is gone all at once */
export const partClass = (name: string): number => /^(chest|yoke|plate|pack|mag\d|strap|collar|neck|pelvis|seat|belly|belt|pouch|holster)/.test(name) ? 1 : 0;
export class Carry {
  readonly rows = new Float32Array(CARRY_ROWS * 4); private tint = new Map<Person, V3[]>();
  private colors(p: Person): V3[] { let c = this.tint.get(p); if (!c) this.tint.set(p, c = SPEC.map(s => { const t = [0, 0, 0] as V3; let w = 0; for (const n of s.of) { const q = p.part(n); if (!q) continue; const a = q.half[0] * q.half[1] + q.half[1] * q.half[2] + q.half[2] * q.half[0], m = MATERIALS[q.mat & 255]!, e = q.tint ?? [1, 1, 1]; for (let k = 0; k < 3; k++) t[k]! += m.base[k]! * e[k]! * e[k]! * a; w += a; } return t.map(v => Math.min(0.94, v / Math.max(w, 1e-6))) as V3; })); return c; }
  /** where they are now */
  frame(lab: Lab, row: number) {
    const f = this.rows, u = new Uint32Array(f.buffer), people = lab.building.people.slice(0, PEOPLE); f.fill(0); u[0] = people.length; u[1] = lab.litCells.many;
    people.forEach((p, i) => {
      f.set([p.pos[0], p.pos[1] + 0.95, p.pos[2], 1.25], (1 + i) * 4); const c = this.colors(p);
      SPEC.forEach((s, j) => { const q = p.part(s.bone)!, o = (1 + PEOPLE + (i * LIMBS + j) * LIMB_ROWS) * 4, d = s.at ?? [0, 0, 0], at = [0, 1, 2].map(k => q.at[k]! + q.rot[0][k]! * d[0] + q.rot[1][k]! * d[1] + q.rot[2][k]! * d[2]), h = s.half ?? q.half;
        for (let k = 0; k < 3; k++) { const r = q.rot[k]!; f.set(r, o + k * 4); f[o + k * 4 + 3] = -(r[0] * at[0]! + r[1] * at[1]! + r[2] * at[2]!); } f.set(h, o + 12); f[o + 15] = s.level; f.set(c[j]!, o + 16); u[o + 19] = s.tau.reduce((a, v, k) => a | (Math.round(v * 255 / 6.4) << (8 * k)), 0) >>> 0; });
    });
    lab.scene.writeExtra(row, new Uint8Array(f.buffer));
  }
}
