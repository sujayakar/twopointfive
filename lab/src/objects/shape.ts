// The true form of a piece, as the shaders meet it: its box, its quadric, its planes, its curves.
import { type V3, sub, dot, add, apply, mul, norm } from '../math';
import { boundsOf, type Piece, type World } from '../scene/world';
import { out } from '../scene/solid';
/** how far outside x is (under 0: inside), roughly in metres: the worst of all it is cut by */
export function outside(p: Piece, x: V3): number {
  const e = sub(x, p.at), l: V3 = [dot(p.rot[0], e), dot(p.rot[1], e), dot(p.rot[2], e)]; let d = Math.max(Math.abs(l[0]) - p.half[0], Math.abs(l[1]) - p.half[1], Math.abs(l[2]) - p.half[2]);
  const quad = (q: readonly number[], v: V3, lin = 0) => { const f = q[0]! * v[0] * v[0] + q[1]! * v[1] * v[1] + q[2]! * v[2] * v[2] + lin * v[1] - 1, g = Math.hypot(2 * q[0]! * v[0], 2 * q[1]! * v[1] + lin, 2 * q[2]! * v[2]); if (g > 1e-9) d = Math.max(d, f / g); else if (f > 0) d = 1; };
  if (p.quad) quad(p.quad, l, p.quad[3]); for (const [n, w] of p.planes ?? []) d = Math.max(d, dot(n, l) - w); for (const c of p.curves ?? []) quad(c.r.map(r => r > 0 ? 1 / (r * r) : 0), sub(l, c.at)); return d;
}
/** where a ray comes in, or −1 */
export function enters(p: Piece, o: V3, d: V3): number {
  const e = sub(o, p.at), a: V3 = [dot(p.rot[0], e), dot(p.rot[1], e), dot(p.rot[2], e)], v: V3 = [dot(p.rot[0], d), dot(p.rot[1], d), dot(p.rot[2], d)]; let lo = -1e9, hi = 1e9;
  for (let k = 0; k < 3; k++) { if (Math.abs(v[k]!) < 1e-9) { if (Math.abs(a[k]!) > p.half[k]!) return -1; continue; } const m = (-p.half[k]! - a[k]!) / v[k]!, n = (p.half[k]! - a[k]!) / v[k]!; lo = Math.max(lo, Math.min(m, n)); hi = Math.min(hi, Math.max(m, n)); }
  const quad = (q: readonly number[], x: V3, lin = 0): boolean => { const A = q[0]! * v[0] * v[0] + q[1]! * v[1] * v[1] + q[2]! * v[2] * v[2], B = q[0]! * x[0] * v[0] + q[1]! * x[1] * v[1] + q[2]! * x[2] * v[2] + 0.5 * lin * v[1], C = q[0]! * x[0] * x[0] + q[1]! * x[1] * x[1] + q[2]! * x[2] * x[2] + lin * x[1] - 1, D = B * B - A * C; if (A > 1e-9) { if (D <= 0) return false; const r = Math.sqrt(D); lo = Math.max(lo, (-B - r) / A); hi = Math.min(hi, (-B + r) / A); } else if (Math.abs(A) <= 1e-9 && Math.abs(B) < 1e-9 && C > 0) return false; return true; };
  if (p.quad && !quad(p.quad, a, p.quad[3])) return -1; for (const c of p.curves ?? []) if (!quad(c.r.map(r => r > 0 ? 1 / (r * r) : 0), sub(a, c.at))) return -1;
  for (const [n, w] of p.planes ?? []) { const den = dot(n, v), num = w - dot(n, a); if (Math.abs(den) < 1e-9) { if (num < 0) return -1; continue; } const t = num / den; if (den < 0) lo = Math.max(lo, t); else hi = Math.min(hi, t); }
  return lo <= hi && lo > 0 ? lo : -1;
}
/** how far it truly reaches: rays from the six sides, a grid of them */
export function extent(p: Piece): { mn: V3; mx: V3 } {
  const b = boundsOf(p), mn: V3 = [...b.mn], mx: V3 = [...b.mx], square = p.rot.every(r => r.some(x => Math.abs(x) > 0.9999)); if (!p.quad && !p.planes?.length && !p.curves?.length) { for (let k = 0; k < 3; k++) { mn[k]! += 1e-4; mx[k]! -= 1e-4; } return { mn, mx }; } void square;
  const N = 15; for (let k = 0; k < 3; k++) { const u = (k + 1) % 3, v = (k + 2) % 3; for (const s of [1, -1]) { let best = 1e9; const d: V3 = [0, 0, 0]; d[k] = s; for (let i = 0; i < N; i++) for (let j = 0; j < N; j++) { const o: V3 = [0, 0, 0]; o[k] = s > 0 ? b.mn[k]! - 1 : b.mx[k]! + 1; o[u] = b.mn[u]! + (b.mx[u]! - b.mn[u]!) * (i + 0.5) / N; o[v] = b.mn[v]! + (b.mx[v]! - b.mn[v]!) * (j + 0.5) / N; const t = enters(p, o, d); if (t >= 0 && t < best) best = t; } if (best < 1e8) { if (s > 0) mn[k] = b.mn[k]! - 1 + best; else mx[k] = b.mx[k]! + 1 - best; } } } return { mn, mx };
}
/** of two faces in one plane: is there a place where both are truly there, and nothing covers it? */
export function fightSeen(all: Piece[], a: { p: Piece; at: V3; n: V3; u: V3; v: V3; hu: number; hv: number }, b: { p: Piece; at: V3; u: V3; v: V3; hu: number; hv: number }): boolean {
  const N = 6; for (let i = 0; i < N; i++) for (let j = 0; j < N; j++) { const x = add(add(a.at, mul(a.u, a.hu * ((i + 0.5) / N * 2 - 1))), mul(a.v, a.hv * ((j + 0.5) / N * 2 - 1))); if (outside(a.p, x) > 5e-4 || outside(b.p, x) > 2.5e-3) continue; const e = sub(x, b.at); if (Math.abs(dot(e, b.u)) > b.hu || Math.abs(dot(e, b.v)) > b.hv) continue; const y = add(x, mul(a.n, 0.0025)); if (all.some(p => p !== a.p && p !== b.p && Math.abs(p.at[0] - y[0]) < 3 && outside(p, y) < 0)) continue; return true; } return false;
}
/** the leaves of a world, where they are: middle and half length */
export const leavesOf = (w: World): { at: V3; r: number }[] => w.instances.flatMap(i => w.clusters[i.cluster]!.leaves.filter(l => !l.wood).map(l => ({ at: add(i.at, apply(i.rot, mul(l.at, i.scale))), r: l.size * i.scale })));
void out; void norm;
