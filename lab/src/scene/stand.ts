// WHAT STANDS ON WHAT. Cut by default: whatever is over a room the section can open goes with that room. And two questions asked of a finished scene: does everything
// stand on something, at every floor it can be cut to; and are two things in one place.
import { type V3, sub, dot, cross } from '../math';
import { World, F, boundsOf, type Piece } from './world';
import type { Place } from './place';
/** the section takes what is over a room with the room */
export function settleCut(w: World, place: Place) {
  const M = 0.45; for (const p of [...w.pieces, ...w.moving]) { if (p.flags & (F.CUT | F.FAR | F.OPEN)) continue; const b = boundsOf(p); if (b.mx[0] - b.mn[0] > 30) continue; for (const r of place.rooms) if (p.at[0] > r[0] - M && p.at[0] < r[3] + M && p.at[2] > r[2] - M && p.at[2] < r[5] + M && p.at[1] > r[1]) { p.flags |= F.CUT; break; } }
}
/** how far apart two pieces' boxes are (under 0: how deep in each other); round ones are taken a little smaller */
export function gap(a: Piece, b: Piece, shrink = 1): number {
  const h = (p: Piece): V3 => p.quad ? [0, 1, 2].map(k => p.half[k]! * (p.quad![k] !== 0 ? shrink : 1)) as V3 : p.half, A = h(a), B = h(b), d = sub(b.at, a.at); let best = -1e9; const axes: V3[] = [...a.rot, ...b.rot]; for (const x of a.rot) for (const y of b.rot) { const c = cross(x, y), l = Math.hypot(...c); if (l > 1e-3) axes.push([c[0] / l, c[1] / l, c[2] / l]); }
  for (const x of axes) { const r = A[0] * Math.abs(dot(a.rot[0], x)) + A[1] * Math.abs(dot(a.rot[1], x)) + A[2] * Math.abs(dot(a.rot[2], x)) + B[0] * Math.abs(dot(b.rot[0], x)) + B[1] * Math.abs(dot(b.rot[1], x)) + B[2] * Math.abs(dot(b.rot[2], x)), g = Math.abs(dot(d, x)) - r; if (g > best) best = g; } return best;
}
function pairs(ps: Piece[], near: number, f: (i: number, j: number) => void) {
  const bs = ps.map(boundsOf), big: number[] = [], G = 1.0, grid = new Map<number, number[]>(), key = (x: number, y: number, z: number) => (x + 2048) + 4096 * ((y + 512) + 1024 * (z + 2048));
  ps.forEach((_, i) => { const b = bs[i]!, c = [0, 1, 2].map(k => [Math.floor((b.mn[k]! - near) / G), Math.floor((b.mx[k]! + near) / G)] as const); if (c.some(([a, z]) => z - a > 24)) { big.push(i); return; } for (let x = c[0]![0]; x <= c[0]![1]; x++) for (let y = c[1]![0]; y <= c[1]![1]; y++) for (let z = c[2]![0]; z <= c[2]![1]; z++) { const k = key(x, y, z), l = grid.get(k); if (l) l.push(i); else grid.set(k, [i]); } });
  const meet = (i: number, j: number) => [0, 1, 2].every(k => bs[i]!.mn[k]! - near <= bs[j]!.mx[k]! && bs[j]!.mn[k]! - near <= bs[i]!.mx[k]!), seen = new Set<number>(), go = (i: number, j: number) => { if (i === j || !meet(i, j)) return; const a = Math.min(i, j), b = Math.max(i, j), k = a * ps.length + b; if (seen.has(k)) return; seen.add(k); f(a, b); };
  for (const l of grid.values()) for (let x = 0; x < l.length; x++) for (let y = x + 1; y < l.length; y++) go(l[x]!, l[y]!); for (const i of big) ps.forEach((_, j) => go(i, j));
}
/** what stands on nothing once the section is at `cut`: its pieces, by what they are part of */
export function floating(w: World, cut: number, near = 0.05): Piece[][] {
  const ps = [...w.pieces, ...w.moving.filter(p => p.rig === 'cloth' || p.rig === 'hung')].filter(p => !(p.flags & F.FAR) && !(p.flags & F.CUT && boundsOf(p).mn[1] > cut) && !(p.flags & F.OPEN && cut < 1e8) && !(p.rig === 'cloth' && cut < 1e8)), up = ps.map((_, i) => i), find = (i: number): number => { while (up[i] !== i) { up[i] = up[up[i]!]!; i = up[i]!; } return i; };
  pairs(ps, near, (i, j) => { const a = find(i), b = find(j); if (a !== b && gap(ps[i]!, ps[j]!) < near) up[a] = b; }); const ground = new Set<number>(); ps.forEach((p, i) => { const b = boundsOf(p); if (b.mx[0] - b.mn[0] > 15 || b.mn[1] < -0.3 || Math.max(...p.half) < 0.021) ground.add(find(i)); else if (Math.max(...p.half) < 0.4) for (const q of w.instances) if (Math.hypot(p.at[0] - q.at[0], p.at[1] - q.at[1], p.at[2] - q.at[2]) < q.scale * 1.4) { ground.add(find(i)); break; } }); const out = new Map<number, Piece[]>(); ps.forEach((p, i) => { const r = find(i); if (ground.has(r)) return; const l = out.get(r); if (l) l.push(p); else out.set(r, [p]); }); return [...out.values()];
}
/** things in each other */
export function clashes(w: World, deep = 0.02): { a: Piece; b: Piece; by: number }[] {
  const out: { a: Piece; b: Piece; by: number }[] = [], ps = [...w.pieces, ...w.moving.filter(p => p.rig === 'cloth')].filter(p => !(p.flags & (F.FAR | F.WATER))); pairs(ps, 0, (i, j) => { const a = ps[i]!, b = ps[j]!; if (a.thing === b.thing || !(a.thing && b.thing)) return; const g = gap(a, b, 0.75); if (g < -deep) out.push({ a, b, by: -g }); }); return out;
}
