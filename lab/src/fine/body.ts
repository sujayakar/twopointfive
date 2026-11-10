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
/** the surfaces a piece is bounded by, each as a distance (about) and which way it looks */
function bounds(p: Piece, x: number, y: number, z: number): [number, number, number, number][] {
  const h = p.half, out: [number, number, number, number][] = [[x - h[0], 1, 0, 0], [-x - h[0], -1, 0, 0], [y - h[1], 0, 1, 0], [-y - h[1], 0, -1, 0], [z - h[2], 0, 0, 1], [-z - h[2], 0, 0, -1]];
  if (p.quad) { const [A, B, C, D] = p.quad, gx = 2 * A * x, gy = 2 * B * y + D, gz = 2 * C * z, g = Math.max(1e-6, Math.hypot(gx, gy, gz)); out.push([(A * x * x + B * y * y + C * z * z + D * y - 1) / g, gx / g, gy / g, gz / g]); } for (const [n, w] of p.planes ?? []) out.push([n[0] * x + n[1] * y + n[2] * z - w, n[0], n[1], n[2]]);
  for (const q of p.curves ?? []) { let f = -1; const n: V3 = [0, 0, 0]; [x - q.at[0], y - q.at[1], z - q.at[2]].forEach((e, k) => { if (q.r[k]! > 0) { f += (e / q.r[k]!) ** 2; n[k] = 2 * e / q.r[k]! ** 2; } }); const g = Math.max(1e-6, Math.hypot(...n)); out.push([f / g, n[0] / g, n[1] / g, n[2] / g]); }
  return out;
}
/** puts the points of a net of a piece onto it, and those near where two or three of its surfaces meet onto the crease */
export function snap(p: Piece, pos: Float32Array, near: number) {
  for (let i = 0; i < pos.length; i += 3) for (let it = 0; it < 6; it++) {
    const b = bounds(p, pos[i]!, pos[i + 1]!, pos[i + 2]!), top = Math.max(...b.map(q => q[0])), on = b.filter(q => q[0] > top - near && q[0] > -near).sort((a, c) => c[0] - a[0]).filter((q, k, l) => l.findIndex(r => r[1] * q[1] + r[2] * q[2] + r[3] * q[3] > 0.97) === k).slice(0, 3); if (!on.length) break;
    // the least move that brings all of them to nothing: J Jt t = f
    const n = on.length, A = on.map(a => on.map(c => a[1] * c[1] + a[2] * c[2] + a[3] * c[3])), f = on.map(q => q[0]); for (let a = 0; a < n; a++) A[a]![a]! += 1e-4; for (let a = 0; a < n; a++) for (let c = a + 1; c < n; c++) { const k = A[c]![a]! / A[a]![a]!; for (let e = a; e < n; e++) A[c]![e]! -= k * A[a]![e]!; f[c]! -= k * f[a]!; } const t = new Array<number>(n).fill(0); for (let a = n - 1; a >= 0; a--) { let v = f[a]!; for (let c = a + 1; c < n; c++) v -= A[a]![c]! * t[c]!; t[a] = v / A[a]![a]!; }
    let m = 0; for (let k = 0; k < 3; k++) { let d = 0; on.forEach((q, a) => { d += q[k + 1]! * t[a]!; }); d = Math.max(-near * 2, Math.min(near * 2, d)); pos[i + k]! -= d; m += Math.abs(d); } if (m < 1e-6) break;
  }
}
const into = (p: Piece, x: number, y: number, z: number): V3 => { const d = [x - p.at[0], y - p.at[1], z - p.at[2]]; return [0, 1, 2].map(k => p.rot[k]![0] * d[0]! + p.rot[k]![1] * d[1]! + p.rot[k]![2] * d[2]!) as V3; };
/** the one body everybody has (its pieces in the order they are made) */
export function body(): Shape {
  const w = new World(), p = new Person(w, [{ at: [0, 0, 0], wait: 99, face: 0 }, { at: [0, 0, 1], wait: 99, face: 0 }], 1, [1, 0.5, 0.1], 0); for (let i = 0; i < 150; i++) p.step(1 / 60); const B = p.body, n = B.length, reach = B.map(q => Math.hypot(...q.half)), smin = (a: number, b: number, k: number) => { const h = Math.max(k - Math.abs(a - b), 0) / k; return Math.min(a, b) - h * h * k * 0.25; };
  let lo: V3 = [9, 9, 9], hi: V3 = [-9, -9, -9]; B.forEach((q, i) => { for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, q.at[k]! - reach[i]!); hi[k] = Math.max(hi[k]!, q.at[k]! + reach[i]!); } }); const c: V3 = [(lo[0] + hi[0]) / 2, (lo[1] + hi[1]) / 2, (lo[2] + hi[2]) / 2], h: V3 = [(hi[0] - lo[0]) / 2, (hi[1] - lo[1]) / 2, (hi[2] - lo[2]) / 2];
  const field = (x: number, y: number, z: number) => { let d = 9; const X = x + c[0], Y = y + c[1], Z = z + c[2]; for (let i = 0; i < n; i++) { const q = B[i]!, e = Math.hypot(X - q.at[0], Y - q.at[1], Z - q.at[2]) - reach[i]!; if (e > d + 0.03) continue; const l = into(q, X, Y, Z); d = smin(d, far(q, l[0], l[1], l[2], 0.006), 0.010); } return d; };
  const two = (x: number, y: number, z: number): [number, number, number] => { let a = 0, b = 0, fa = 9, fb = 9; const X = x + c[0], Y = y + c[1], Z = z + c[2]; for (let i = 0; i < n; i++) { const q = B[i]!, l = into(q, X, Y, Z), f = far(q, l[0], l[1], l[2], 0.006); if (f < fa) { b = a; fb = fa; a = i; fa = f; } else if (f < fb) { b = i; fb = f; } } const same = B[a]!.mat === B[b]!.mat; return [a, b, same ? Math.min(1, 0.5 + 0.5 * (fb - fa) / 0.025) : 1]; };
  const cloth = M.cloth!, mesh = solid(field, h, 0.0095, 1, (x, y, z) => { const [a] = two(x, y, z); return B[a]!.mat === cloth ? 0.0016 * fbm3(x * 26, y * 18, z * 26, 3) : 0; }, (x, y, z) => { const [a] = two(x, y, z), v = B[a]!.mat === cloth ? 0.97 + 0.12 * fbm3(x * 30, y * 22, z * 30, 3) : 1; return [v, v, v, 1]; }, 0.0005);
  const nv = mesh.pos.length / 3, bones = new Uint8Array(nv * 2), weight = new Float32Array(nv); for (let i = 0; i < nv; i++) { const [a, b, k] = two(mesh.pos[i * 3]!, mesh.pos[i * 3 + 1]!, mesh.pos[i * 3 + 2]!); bones[i * 2] = a; bones[i * 2 + 1] = b; weight[i] = k; }
  // at rest, a point of the skin in each piece's own frame: rows over (x, y, z, 1) of the skin's
  const rest = new Float32Array(n * 12); B.forEach((q, i) => { for (let k = 0; k < 3; k++) { const r = q.rot[k]!; rest.set([r[0], r[1], r[2], r[0] * (c[0] - q.at[0]) + r[1] * (c[1] - q.at[1]) + r[2] * (c[2] - q.at[2])], i * 12 + k * 4); } });
  return { name: 'body', kind: SKINNED, mesh, bones, weight, rest };
}
