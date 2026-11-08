// Ways on foot: between stops, round what stands there. A grid of the floor at a person's height, the shortest way over it, then pulled straight.
import type { V3 } from '../math';
import { type Piece, boundsOf, F } from './world';
const CELL = 0.125, WIDE = 0.27;
export function plan<S extends { at: V3 }>(pieces: Piece[], stops: S[]): S[] {
  const xs = stops.map(s => s.at[0]), zs = stops.map(s => s.at[2]), y0 = Math.min(...stops.map(s => s.at[1])), y1 = Math.max(...stops.map(s => s.at[1])); if (y1 - y0 > 0.5) return stops;
  const x0 = Math.min(...xs) - 3, z0 = Math.min(...zs) - 3, nx = Math.ceil((Math.max(...xs) + 3 - x0) / CELL), nz = Math.ceil((Math.max(...zs) + 3 - z0) / CELL), shut = new Uint8Array(nx * nz), lo = y0 + 0.28, hi = y0 + 1.75;
  for (const p of pieces) {
    if (p.flags & (F.WATER | F.FAR)) continue; const b = boundsOf(p); if (b.mx[1] < lo || b.mn[1] > hi) continue; const up = Math.abs(p.rot[1][1]) > 0.999, c = p.rot[0], d = p.rot[2];
    for (let i = Math.max(0, Math.floor((b.mn[0] - WIDE - x0) / CELL)); i <= Math.min(nx - 1, Math.ceil((b.mx[0] + WIDE - x0) / CELL)); i++) for (let k = Math.max(0, Math.floor((b.mn[2] - WIDE - z0) / CELL)); k <= Math.min(nz - 1, Math.ceil((b.mx[2] + WIDE - z0) / CELL)); k++) {
      const x = x0 + i * CELL - p.at[0], z = z0 + k * CELL - p.at[2]; let dx: number, dz: number; if (up) { dx = Math.abs(x * c[0] + z * c[2]) - p.half[0]; dz = Math.abs(x * d[0] + z * d[2]) - p.half[2]; if (p.quad && p.quad[0] && p.quad[2]) { const r = Math.hypot(x, z) - Math.max(p.half[0], p.half[2]); if (r < WIDE) shut[i + k * nx] = 1; continue; } } else { dx = Math.max(b.mn[0] - (x + p.at[0]), x + p.at[0] - b.mx[0]); dz = Math.max(b.mn[2] - (z + p.at[2]), z + p.at[2] - b.mx[2]); }
      if (Math.hypot(Math.max(0, dx), Math.max(0, dz)) < WIDE) shut[i + k * nx] = 1; }
  }
  const near = (p: V3): number => { const ci = Math.round((p[0] - x0) / CELL), ck = Math.round((p[2] - z0) / CELL); let best = -1, bd = 1e9; for (let r = 0; r < 14 && best < 0; r++) for (let i = ci - r; i <= ci + r; i++) for (let k = ck - r; k <= ck + r; k++) { if (i < 0 || k < 0 || i >= nx || k >= nz || shut[i + k * nx]) continue; const d = (i - ci) ** 2 + (k - ck) ** 2; if (d < bd) { bd = d; best = i + k * nx; } } return best; };
  const clear = (a: number, b: number) => { const ax = a % nx, az = Math.floor(a / nx), bx = b % nx, bz = Math.floor(b / nx), n = Math.ceil(Math.hypot(bx - ax, bz - az) * 2); for (let i = 0; i <= n; i++) { const t = n ? i / n : 0; if (shut[Math.round(ax + (bx - ax) * t) + Math.round(az + (bz - az) * t) * nx]) return false; } return true; };
  const way = (a: number, b: number): number[] => {
    const cost = new Float32Array(nx * nz).fill(1e9), from = new Int32Array(nx * nz).fill(-1), open: [number, number][] = [[0, a]], bx = b % nx, bz = Math.floor(b / nx); cost[a] = 0;
    while (open.length) { let m = 0; for (let i = 1; i < open.length; i++) if (open[i]![0] < open[m]![0]) m = i; const [, c] = open[m]!; open[m] = open[open.length - 1]!; open.pop(); if (c === b) break; const x = c % nx, z = Math.floor(c / nx);
      for (let dx = -1; dx < 2; dx++) for (let dz = -1; dz < 2; dz++) { const i = x + dx, k = z + dz; if ((!dx && !dz) || i < 0 || k < 0 || i >= nx || k >= nz) continue; const q = i + k * nx; if (shut[q]) continue; const g = cost[c]! + Math.hypot(dx, dz); if (g < cost[q]! - 1e-6) { cost[q] = g; from[q] = c; open.push([g + Math.hypot(i - bx, k - bz), q]); } } }
    if (from[b]! < 0 && a !== b) return []; const out = [b]; for (let c = b; c !== a;) { c = from[c]!; out.unshift(c); } const pulled = [out[0]!]; for (let i = 0; i < out.length - 1;) { let j = out.length - 1; while (j > i + 1 && !clear(out[i]!, out[j]!)) j--; pulled.push(out[j]!); i = j; } return pulled;
  };
  const at = stops.map(s => near(s.at)), out: S[] = []; stops.forEach((s, i) => { const a = at[i]!, n = (i + 1) % stops.length, b = at[n]!; const p: V3 = a >= 0 ? [x0 + (a % nx) * CELL, s.at[1], z0 + Math.floor(a / nx) * CELL] : s.at; out.push({ ...s, at: Math.hypot(p[0] - s.at[0], p[2] - s.at[2]) < CELL ? s.at : p }); if (a < 0 || b < 0) return; for (const c of way(a, b).slice(1, -1)) out.push({ at: [x0 + (c % nx) * CELL, s.at[1], z0 + Math.floor(c / nx) * CELL] } as S); });
  return out;
}
