// What stands near a place, for the few rays the CPU asks (a torch held past a wall).
import { type V3, sub, dot } from '../math';
import { type Piece, boundsOf, F } from './world';
const G = 2, key = (x: number, y: number, z: number) => Math.floor(x / G) + 512 * (Math.floor(y / G) + 64 + 512 * (Math.floor(z / G) + 256));
export class Near {
  private cells = new Map<number, Piece[]>();
  constructor(pieces: Piece[]) { this.change([], pieces); }
  /** what stands has changed */
  change(gone: Piece[], come: Piece[]) { if (gone.length) { const was = new Set(gone); for (const [k, l] of this.cells) if (l.some(p => was.has(p))) this.cells.set(k, l.filter(p => !was.has(p))); } for (const p of come) { if (p.mesh !== undefined || p.flags & (F.GLASS | F.WATER)) continue; const b = boundsOf(p); if (b.mx[0] - b.mn[0] > 60) continue; if (b.mn[0] > 25 || b.mx[0] < -25 || b.mn[2] > 20 || b.mx[2] < -20 || b.mx[1] < -2) continue; for (let x = Math.floor(b.mn[0] / G); x <= Math.floor(b.mx[0] / G); x++) for (let y = Math.floor(b.mn[1] / G); y <= Math.floor(b.mx[1] / G); y++) for (let z = Math.floor(b.mn[2] / G); z <= Math.floor(b.mx[2] / G); z++) { const k = key(x * G, y * G, z * G); let l = this.cells.get(k); if (!l) this.cells.set(k, l = []); l.push(p); } } }
  /** how far from a along d (a unit) before something stands, at most far (short rays only) */
  reach(a: V3, d: V3, far: number, also: Piece[] = []): number {
    let best = far; const seen = new Set<Piece>(also); for (const t of [0, far / 2, far]) for (const p of this.cells.get(key(a[0] + d[0] * t, a[1] + d[1] * t, a[2] + d[2] * t)) ?? []) seen.add(p);
    for (const p of seen) { const o = sub(a, p.at); let lo = 0, hi = best; for (let k = 0; k < 3; k++) { const q = dot(p.rot[k]!, o), v = dot(p.rot[k]!, d), h = p.half[k]!; if (Math.abs(v) < 1e-9) { if (Math.abs(q) > h) { hi = -1; break; } continue; } const u = (-h - q) / v, w = (h - q) / v; lo = Math.max(lo, Math.min(u, w)); hi = Math.min(hi, Math.max(u, w)); } if (lo <= hi && lo > 1e-4) best = lo; }
    return best;
  }
}
