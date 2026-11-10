// WHAT IS SOLID, for what moves: how deep a point is in a shape and which way is out; what stands near a place; how far a ray goes. The same shapes the rays of light meet.
import { type V3, type Rot, add, sub, mul, dot } from '../math';
import { M } from './materials';
import { F, boundsOf, type Piece, type World } from './world';
export interface Shape { at: V3; rot: Rot; half: V3; quad?: [number, number, number, number]; planes?: [V3, number][] }
export const out = (r: Rot, v: V3): V3 => [r[0][0] * v[0] + r[1][0] * v[1] + r[2][0] * v[2], r[0][1] * v[0] + r[1][1] * v[1] + r[2][1] * v[2], r[0][2] * v[0] + r[1][2] * v[1] + r[2][2] * v[2]], into = (r: Rot, v: V3): V3 => [dot(r[0], v), dot(r[1], v), dot(r[2], v)];
/** the way out, after depth() */
export const N: V3 = [0, 0, 0];
/** how deep x is in a shape (under 0: inside), and which way is out (N) */
export function depth(s: Shape, x: V3): number {
  const l = into(s.rot, sub(x, s.at)); let d = -1e9, a = 0, b = 0, c = 0; for (let k = 0; k < 3; k++) { const e = Math.abs(l[k]!) - s.half[k]!; if (e > d) { d = e; a = k === 0 ? Math.sign(l[0]) : 0; b = k === 1 ? Math.sign(l[1]) : 0; c = k === 2 ? Math.sign(l[2]) : 0; } }
  if (s.quad) { const [A, B, C, D] = s.quad, gx = 2 * A * l[0], gy = 2 * B * l[1] + D, gz = 2 * C * l[2], g = Math.sqrt(gx * gx + gy * gy + gz * gz); if (g > 1e-9) { const e = (A * l[0] * l[0] + B * l[1] * l[1] + C * l[2] * l[2] + D * l[1] - 1) / g; if (e > d) { d = e; a = gx / g; b = gy / g; c = gz / g; } } }
  if (s.planes) for (const [n, w] of s.planes) { const e = n[0] * l[0] + n[1] * l[1] + n[2] * l[2] - w; if (e > d) { d = e; a = n[0]; b = n[1]; c = n[2]; } }
  const n = out(s.rot, [a, b, c]); N[0] = n[0]; N[1] = n[1]; N[2] = n[2]; return d;
}
/** where a ray comes into a shape, or −1 */
export function enter(s: Shape, a: V3, d: V3, far: number): number {
  const o = into(s.rot, sub(a, s.at)), v = into(s.rot, d); let lo = -1e9, hi = far; for (let k = 0; k < 3; k++) { const q = o[k]!, e = v[k]!, h = s.half[k]!; if (Math.abs(e) < 1e-9) { if (Math.abs(q) > h) return -1; continue; } const m = (-h - q) / e, n = (h - q) / e; lo = Math.max(lo, Math.min(m, n)); hi = Math.min(hi, Math.max(m, n)); }
  if (s.quad) { const [qa, qb, qc, qd] = s.quad, A = qa * v[0] * v[0] + qb * v[1] * v[1] + qc * v[2] * v[2], B = qa * o[0] * v[0] + qb * o[1] * v[1] + qc * o[2] * v[2] + 0.5 * qd * v[1], C = qa * o[0] * o[0] + qb * o[1] * o[1] + qc * o[2] * o[2] + qd * o[1] - 1, D = B * B - A * C; if (A > 1e-9) { if (D <= 0) return -1; const r = Math.sqrt(D); lo = Math.max(lo, (-B - r) / A); hi = Math.min(hi, (-B + r) / A); } }
  if (s.planes) for (const [n, w] of s.planes) { const den = dot(n, v), num = w - dot(n, o); if (Math.abs(den) < 1e-9) { if (num < 0) return -1; continue; } const t = num / den; if (den < 0) lo = Math.max(lo, t); else hi = Math.min(hi, t); }
  return lo <= hi && lo > 1e-4 && lo < far ? lo : -1;
}
export class Solid {
  private grid = new Map<number, Piece[]>(); private box = new Map<Piece, { mn: V3; mx: V3 }>();
  constructor(private w: World, private doors: { leaf: Piece[] }[]) {}
  private key(x: number, y: number, z: number) { return (x + 256) + 512 * ((y + 64) + 256 * (z + 256)); }
  /** call when the scene is made */
  settle() { this.put(this.w.pieces); return this; }
  private put(ps: Piece[]) { for (const p of ps) { if (p.flags & F.WATER || (p.mat === M.paleBark && Math.max(p.half[0], p.half[2]) < 0.07)) continue; const b = boundsOf(p); if (b.mx[0] < -40 || b.mn[0] > 40 || b.mx[2] < -40 || b.mn[2] > 40 || b.mx[1] < -2) continue; this.box.set(p, b as never); for (let x = Math.floor(Math.max(-40, b.mn[0])); x <= Math.floor(Math.min(40, b.mx[0])); x++) for (let y = Math.floor(Math.max(-2, b.mn[1])); y <= Math.floor(b.mx[1]); y++) for (let z = Math.floor(Math.max(-40, b.mn[2])); z <= Math.floor(Math.min(40, b.mx[2])); z++) { const k = this.key(x, y, z), l = this.grid.get(k); if (l) l.push(p); else this.grid.set(k, [p]); } } }
  /** what stands has changed */
  change(gone: Piece[], come: Piece[]) { const was = new Set(gone); for (const p of gone) { const b = this.box.get(p); if (!b) continue; this.box.delete(p); for (let x = Math.floor(b.mn[0]); x <= Math.floor(b.mx[0]); x++) for (let y = Math.floor(b.mn[1]); y <= Math.floor(b.mx[1]); y++) for (let z = Math.floor(b.mn[2]); z <= Math.floor(b.mx[2]); z++) { const k = this.key(x, y, z), l = this.grid.get(k); if (l) this.grid.set(k, l.filter(q => !was.has(q))); } } this.put(come); }
  /** what stands within r of c (and the leaves of doors) */
  near(c: V3, r: number): Piece[] { const got = new Set<Piece>(); for (let x = Math.floor(c[0] - r); x <= Math.floor(c[0] + r); x++) for (let y = Math.floor(c[1] - r); y <= Math.floor(c[1] + r); y++) for (let z = Math.floor(c[2] - r); z <= Math.floor(c[2] + r); z++) for (const p of this.grid.get(this.key(x, y, z)) ?? []) { const m = this.box.get(p)!; if (m.mn[0] < c[0] + r && m.mx[0] > c[0] - r && m.mn[1] < c[1] + r && m.mx[1] > c[1] - r && m.mn[2] < c[2] + r && m.mx[2] > c[2] - r) got.add(p); } for (const d of this.doors) { const p = d.leaf[0]!; if (Math.hypot(p.at[0] - c[0], p.at[1] - c[1], p.at[2] - c[2]) < Math.hypot(...p.half) + r) got.add(p); } return [...got]; }
  /** how far from a along d (a unit) till something stands, at most far. `clear`: panes do not stop it */
  ray(a: V3, d: V3, far: number, clear = false): number {
    let best = far; const seen = new Set<Piece>(), ask = (p: Piece) => { if (seen.has(p)) return; seen.add(p); if (clear && p.flags & F.GLASS) return; const t = enter(p, a, d, best); if (t >= 0 && t < best) best = t; }; let x = Math.floor(a[0]), y = Math.floor(a[1]), z = Math.floor(a[2]); const st = d.map(v => v > 0 ? 1 : -1), td = d.map(v => Math.abs(1 / (Math.abs(v) < 1e-9 ? 1e-9 : v))), tn = [0, 1, 2].map(k => { const f = a[k]! - Math.floor(a[k]!); return (d[k]! > 0 ? 1 - f : f) * td[k]!; }); for (const q of this.doors) ask(q.leaf[0]!);
    for (let i = 0; i < 200; i++) { for (const p of this.grid.get(this.key(x, y, z)) ?? []) ask(p); const m = Math.min(tn[0]!, tn[1]!, tn[2]!); if (m > best) break; if (tn[0] === m) { x += st[0]!; tn[0] += td[0]!; } else if (tn[1] === m) { y += st[1]!; tn[1] += td[1]!; } else { z += st[2]!; tn[2] += td[2]!; } } return best;
  }
  /** the floor under a place: the highest that a foot of radius r finds, from `up` over it down to `down` under; or null */
  floor(x: V3, r: number, up: number, down: number): number | null { let best: number | null = null; for (const [a, b] of [[0, 0], [r, 0], [-r, 0], [0, r], [0, -r]] as const) { const t = this.ray([x[0] + a, x[1] + up, x[2] + b], [0, -1, 0], up + down); if (t < up + down - 1e-6) { const y = x[1] + up - t; if (best === null || y > best) best = y; } } return best; }
  /** push an upright body (feet at x, radius r, so tall) out of what stands; what is lower than `step` is stepped over */
  push(x: V3, r: number, tall: number, step: number): V3 {
    const ps = this.near([x[0], x[1] + tall / 2, x[2]], tall / 2 + r); let p: V3 = [...x]; for (let pass = 0; pass < 4; pass++) { let moved = false; for (const s of ps) { const b = this.box.get(s); if (b && b.mx[1] < x[1] + step) continue; for (const h of [step + r * 0.5, tall * 0.55, tall - r]) { const c: V3 = [p[0], p[1] + h, p[2]], d = depth(s, c); if (d >= r || d < -0.6) continue; const l = Math.hypot(N[0], N[2]); if (l < 0.3) continue; const k = (r - d) / l; p = [p[0] + N[0] / l * k, p[1], p[2] + N[2] / l * k]; moved = true; } } if (!moved) break; } return p;
  }
}
void add; void mul;
