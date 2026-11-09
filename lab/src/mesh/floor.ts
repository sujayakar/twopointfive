// A FLOOR OF PEBBLES, a metre square at a time. Every square of a kind is the same; those that lie over its edges are the same for all kinds, so any kind fits any other.
// Near by it is its pebbles; farther off one relief, the top of them as a mesh with levels of its own. Both are levels of one object, each with its error in metres.
import { type V3, type Rot, rng, rotX, rotY, rotZ, compose, apply } from '../math';
import { normalsOf, type Mesh } from './mesh';
import { cutAt, errorFor, type Dag } from './dag';
import type { Child, Packed } from './pack';

export interface Stone { shape: number; at: V3; rot: Rot; scale: number; tint: V3 }
export const SIDE = 1;
/** what lies under the stones, as the relief's color */
const GROUND = [109, 95, 79, 255];
const TINTS: [V3, number][] = [[[0.70, 0.70, 0.71], 5], [[0.62, 0.61, 0.60], 5], [[0.52, 0.53, 0.56], 4], [[0.80, 0.75, 0.68], 4], [[0.74, 0.64, 0.54], 2], [[0.90, 0.89, 0.86], 2], [[0.40, 0.41, 0.44], 3], [[0.64, 0.55, 0.50], 1]];
/** the stones of one square, by kind */
export function stones(kind: number, shapes: number): Stone[] {
  type Disc = { x: number; z: number; r: number }; const grid = new Map<number, Disc[]>(), C = 0.07, cell = (v: number) => ((Math.floor(v / C) % 15) + 15) % 15, out: Stone[] = [];
  const far = (a: number) => { const d = Math.abs(a); return Math.min(d, SIDE - d); };
  const fits = (x: number, z: number, r: number) => { for (let i = -1; i <= 1; i++) for (let k = -1; k <= 1; k++) for (const q of grid.get(cell(((x + i * C) % SIDE + SIDE) % SIDE) * 15 + cell(((z + k * C) % SIDE + SIDE) % SIDE)) ?? []) { const dx = far(x - q.x), dz = far(z - q.z), m = (r + q.r) * 0.86; if (dx * dx + dz * dz < m * m) return false; } return true; };
  const put = (r0: () => number, x: number, z: number, r: number, y: number) => {
    const d = { x, z, r }, key = cell(x) * 15 + cell(z); grid.set(key, [...(grid.get(key) ?? []), d]);
    const all = TINTS.reduce((a, t) => a + t[1], 0); let pick = r0() * all, tint: V3 = TINTS[0]![0]; for (const [t, w] of TINTS) { if ((pick -= w) <= 0) { tint = t; break; } } const v = 0.92 + 0.16 * r0();
    out.push({ shape: Math.floor(r0() * shapes), at: [x - SIDE / 2, y, z - SIDE / 2], rot: compose(rotY(r0() * 6.283), compose(rotX((r0() - 0.5) * 0.6), rotZ((r0() - 0.5) * 0.6))), scale: r * 2.15, tint: [tint[0] * v, tint[1] * v, tint[2] * v] });
  };
  // two beds, the lower filling what the upper leaves; in each the shared ones first, whatever lies over an edge
  const edge = rng(4711), own = rng(100 + kind * 13);
  for (const bed of [[[0.026, 0.034, 900, 0.004], [0.018, 0.026, 2400, 0.002], [0.011, 0.018, 5200, -0.003]], [[0.018, 0.026, 1800, -0.016], [0.011, 0.018, 5200, -0.014]]] as const) {
    grid.clear(); for (const [r0, rim] of [[edge, true], [own, false]] as const) for (const [lo, hi, tries, y] of bed) for (let i = 0; i < tries * (rim ? 0.6 : 1); i++) {
      const r = lo + (hi - lo) * r0(), x = r0() * SIDE, z = r0() * SIDE, over = x - r * 1.1 < 0 || z - r * 1.1 < 0 || x + r * 1.1 > SIDE || z + r * 1.1 > SIDE; if (over !== rim || !fits(x, z, r)) continue; put(r0, x, z, r, y - 0.006 * r0() + (r - 0.02) * 0.15);
    }
  }
  return out;
}
/** the relief: the top of the stones over a grid of n × n, as a mesh (its own rim runs over the square's edge) */
export function relief(list: Stone[], dags: Dag[], n: number, under: number): Mesh {
  const N = n + 1, h = new Float32Array(N * N).fill(under), col = new Uint8Array(N * N * 4), nrm = new Float32Array(N * N * 3), step = SIDE / n; for (let i = 0; i < N * N; i++) { col.set(GROUND, i * 4); nrm[i * 3 + 1] = 1; }
  const coarse = dags.map(d => cutAt(d, errorFor(d, 1400))), q = (x: number) => Math.max(0, Math.min(255, Math.round(x)));
  for (const s of list) for (const sx of [-SIDE, 0, SIDE]) for (const sz of [-SIDE, 0, SIDE]) {
    const cx = s.at[0] + sx, cz = s.at[2] + sz, r = s.scale * 0.6; if (cx + r < -SIDE / 2 || cx - r > SIDE / 2 || cz + r < -SIDE / 2 || cz - r > SIDE / 2) continue; const P = dags[s.shape]!.pool;
    for (const c of coarse[s.shape]!) for (let t = 0; t < c.tri.length; t += 3) {
      const v = [0, 1, 2].map(k => { const i = c.tri[t + k]! * 3, w = apply(s.rot, [P.pos[i]! * s.scale, P.pos[i + 1]! * s.scale, P.pos[i + 2]! * s.scale]); return [w[0] + cx, w[1] + s.at[1], w[2] + cz] as V3; });
      const gx = v.map(p => (p[0] + SIDE / 2) / step), gz = v.map(p => (p[2] + SIDE / 2) / step), den = (gx[1]! - gx[0]!) * (gz[2]! - gz[0]!) - (gx[2]! - gx[0]!) * (gz[1]! - gz[0]!); if (Math.abs(den) < 1e-9) continue;
      for (let z = Math.max(0, Math.ceil(Math.min(...gz))); z <= Math.min(n, Math.floor(Math.max(...gz))); z++) for (let x = Math.max(0, Math.ceil(Math.min(...gx))); x <= Math.min(n, Math.floor(Math.max(...gx))); x++) {
        const b = ((x - gx[0]!) * (gz[2]! - gz[0]!) - (gx[2]! - gx[0]!) * (z - gz[0]!)) / den, e = ((gx[1]! - gx[0]!) * (z - gz[0]!) - (x - gx[0]!) * (gz[1]! - gz[0]!)) / den; if (b < 0 || e < 0 || b + e > 1) continue; const y = v[0]![1] * (1 - b - e) + v[1]![1] * b + v[2]![1] * e, o = z * N + x; if (y <= h[o]!) continue; h[o] = y;
        const i0 = c.tri[t]!, i1 = c.tri[t + 1]!, i2 = c.tri[t + 2]!; for (let k = 0; k < 3; k++) { const a = (P.col[i0 * 4 + k]! * (1 - b - e) + P.col[i1 * 4 + k]! * b + P.col[i2 * 4 + k]! * e) / 255; col[o * 4 + k] = q(255 * a * s.tint[k]!); } col[o * 4 + 3] = 230;
      }
    }
  }
  const pos = new Float32Array(N * N * 3), tri = new Uint32Array(n * n * 6); for (let z = 0; z < N; z++) for (let x = 0; x < N; x++) pos.set([x * step - SIDE / 2, h[z * N + x]!, z * step - SIDE / 2], (z * N + x) * 3);
  let o = 0; for (let z = 0; z < n; z++) for (let x = 0; x < n; x++) { const a = z * N + x, b = a + 1, c = a + N, d = c + 1; if (Math.abs(h[a]! - h[d]!) < Math.abs(h[b]! - h[c]!)) tri.set([a, c, d, a, d, b], o); else tri.set([a, c, b, b, c, d], o); o += 6; }
  return { pos, nrm: normalsOf(pos, tri), col, tri };
}
export const children = (list: Stone[], shapes: Packed[]): Child[] => list.map(s => ({ mesh: shapes[s.shape]!, at: s.at, rot: s.rot, scale: s.scale, tint: s.tint }));
