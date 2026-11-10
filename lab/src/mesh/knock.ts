// A HOLE IN A FINE WALL. What a wall is faced with are whole things (bricks, pavers, boards), each placed by a row of its own: those whose middle is in the ring are marked as gone, and nothing that draws or traces meets
// them again. What lies behind is boxes, which break as boxes do (scene/breach.ts); the toothed rim comes by itself.
import type { Lab } from '../lab';
import type { Breach } from '../scene/breach';
import { F } from '../scene/world';

/** marks what a hole takes with it; returns how many */
export function knockOut(lab: Lab, b: Breach): number {
  const m = lab.building.world.meshes!, u = new Uint32Array(m.data.buffer, m.data.byteOffset, m.data.length), f = m.data; let many = 0;
  for (const p of lab.building.world.pieces) {
    if (!(p.flags & F.MESH) || p.mesh === undefined || Math.hypot(p.at[0] - b.at[0], p.at[1] - b.at[1], p.at[2] - b.at[2]) > b.r * 1.5 + Math.hypot(...p.half)) continue; const kids = u[p.mesh * 4 + 2]!, n = u[p.mesh * 4 + 3]!, s = p.quad![0];
    for (let i = 0; i < n; i++) {
      const o = (kids + i * 4) * 4; if (u[o + 13]! >>> 31) continue; const k2 = f[o]! ** 2 + f[o + 1]! ** 2 + f[o + 2]! ** 2, l = [0, 1, 2].map(a => -(f[o + a]! * f[o + 3]! + f[o + 4 + a]! * f[o + 7]! + f[o + 8 + a]! * f[o + 11]!) / k2), x = [0, 1, 2].map(a => p.at[a]! + s * (p.rot[0][a]! * l[0]! + p.rot[1][a]! * l[1]! + p.rot[2][a]! * l[2]!) - b.at[a]!);
      const along = x[0]! * b.n[0] + x[1]! * b.n[1] + x[2]! * b.n[2], du = x[0]! * b.u[0] + x[1]! * b.u[1] + x[2]! * b.u[2], dv = x[0]! * b.v[0] + x[1]! * b.v[1] + x[2]! * b.v[2]; if (along < -0.2 || along > b.thick + 0.25 || Math.hypot(du, dv / 1.12) > b.r * 1.16) continue;
      u[o + 13] = (u[o + 13]! | 0x80000000) >>> 0; many++; lab.scene.writeExtra(lab.scene.meshRow + kids + i * 4 + 3, new Uint8Array(u.slice(o + 12, o + 16).buffer));
    }
  }
  return many;
}
