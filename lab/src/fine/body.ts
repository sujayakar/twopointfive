// A BODY in one skin: the pieces a person is made of, melted into one surface that bends with them. Each vertex goes with the piece it is nearest, and near a seam with the next one too; what it is made of is that piece's.
import { World, type Piece } from '../scene/world';
import { Person } from '../scene/people';
import type { V3 } from '../math';
import { fbm3 } from '../mesh/shapes';
import { solid } from '../mesh/made';
import { M } from '../scene/materials';
import { SKINNED, type Shape } from './template';

/** how far from a piece's surface, about, in its own frame */
export function far(p: Piece, x: number, y: number, z: number, round = 0.004): number {
  const h = p.half, r = Math.min(round, Math.min(...h) * 0.5), a = Math.abs(x) - h[0] + r, b = Math.abs(y) - h[1] + r, c = Math.abs(z) - h[2] + r; let d = Math.min(Math.max(a, b, c), 0) + Math.hypot(Math.max(a, 0), Math.max(b, 0), Math.max(c, 0)) - r;
  if (p.quad) { const [A, B, C, D] = p.quad, f = A * x * x + B * y * y + C * z * z + D * y - 1, g = Math.hypot(2 * A * x, 2 * B * y + D, 2 * C * z); d = Math.max(d, f / Math.max(g, 1e-3 + 2 / Math.max(...h))); }
  for (const [n, w] of p.planes ?? []) d = Math.max(d, n[0] * x + n[1] * y + n[2] * z - w);
  for (const q of p.curves ?? []) { const e = [x - q.at[0], y - q.at[1], z - q.at[2]]; let f = -1, g = 0; for (let k = 0; k < 3; k++) if (q.r[k]! > 0) { const u = e[k]! / q.r[k]!; f += u * u; g += (2 * u / q.r[k]!) ** 2; } d = Math.max(d, f / Math.max(Math.sqrt(g), 0.3)); }
  return d;
}
const into = (p: Piece, x: number, y: number, z: number): V3 => { const d = [x - p.at[0], y - p.at[1], z - p.at[2]]; return [0, 1, 2].map(k => p.rot[k]![0] * d[0]! + p.rot[k]![1] * d[1]! + p.rot[k]![2] * d[2]!) as V3; };
/** the one body everybody has (its pieces in the order they are made) */
export function body(): Shape {
  const w = new World(), p = new Person(w, [{ at: [0, 0, 0], wait: 99, face: 0 }, { at: [0, 0, 1], wait: 99, face: 0 }], 1, [1, 0.5, 0.1], 0); for (let i = 0; i < 150; i++) p.step(1 / 60); const B = p.body, n = B.length, reach = B.map(q => Math.hypot(...q.half)), smin = (a: number, b: number, k: number) => { const h = Math.max(k - Math.abs(a - b), 0) / k; return Math.min(a, b) - h * h * k * 0.25; };
  let lo: V3 = [9, 9, 9], hi: V3 = [-9, -9, -9]; B.forEach((q, i) => { for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, q.at[k]! - reach[i]!); hi[k] = Math.max(hi[k]!, q.at[k]! + reach[i]!); } }); const c: V3 = [(lo[0] + hi[0]) / 2, (lo[1] + hi[1]) / 2, (lo[2] + hi[2]) / 2], h: V3 = [(hi[0] - lo[0]) / 2, (hi[1] - lo[1]) / 2, (hi[2] - lo[2]) / 2];
  const field = (x: number, y: number, z: number) => { let d = 9; const X = x + c[0], Y = y + c[1], Z = z + c[2]; for (let i = 0; i < n; i++) { const q = B[i]!, e = Math.hypot(X - q.at[0], Y - q.at[1], Z - q.at[2]) - reach[i]!; if (e > d + 0.03) continue; const l = into(q, X, Y, Z); d = smin(d, far(q, l[0], l[1], l[2], 0.006), 0.010); } return d; };
  const two = (x: number, y: number, z: number): [number, number, number] => { let a = 0, b = 0, fa = 9, fb = 9; const X = x + c[0], Y = y + c[1], Z = z + c[2]; for (let i = 0; i < n; i++) { const q = B[i]!, l = into(q, X, Y, Z), f = far(q, l[0], l[1], l[2], 0.006); if (f < fa) { b = a; fb = fa; a = i; fa = f; } else if (f < fb) { b = i; fb = f; } } const same = B[a]!.mat === B[b]!.mat; return [a, b, same ? Math.min(1, 0.5 + 0.5 * (fb - fa) / 0.025) : 1]; };
  const cloth = M.cloth!, mesh = solid(field, h, 0.012, 1, (x, y, z) => { const [a] = two(x, y, z); return B[a]!.mat === cloth ? 0.0016 * fbm3(x * 26, y * 18, z * 26, 3) : 0; }, (x, y, z) => { const [a] = two(x, y, z), v = B[a]!.mat === cloth ? 0.97 + 0.12 * fbm3(x * 30, y * 22, z * 30, 3) : 1; return [v, v, v, 1]; }, 0.0005);
  const nv = mesh.pos.length / 3, bones = new Uint8Array(nv * 2), weight = new Float32Array(nv); for (let i = 0; i < nv; i++) { const [a, b, k] = two(mesh.pos[i * 3]!, mesh.pos[i * 3 + 1]!, mesh.pos[i * 3 + 2]!); bones[i * 2] = a; bones[i * 2 + 1] = b; weight[i] = k; }
  // at rest, a point of the skin in each piece's own frame: rows over (x, y, z, 1) of the skin's
  const rest = new Float32Array(n * 12); B.forEach((q, i) => { for (let k = 0; k < 3; k++) { const r = q.rot[k]!; rest.set([r[0], r[1], r[2], r[0] * (c[0] - q.at[0]) + r[1] * (c[1] - q.at[1]) + r[2] * (c[2] - q.at[2])], i * 12 + k * 4); } });
  return { name: 'body', kind: SKINNED, mesh, bones, weight, rest };
}
