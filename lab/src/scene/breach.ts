// A HOLE IN WHAT STANDS. A box that a jagged ring cuts through becomes the wedges round the ring: each is the box again with three more planes (two that run out from the hole's middle, one along its edge), so
// what is left is of the same kind as everything else, and nothing that draws or traces need know. What was inside the ring falls out as rubble: many-sided prisms, cut from the same boxes, that are bodies.
import { type V3, type Rot, add, sub, mul, dot } from '../math';
import { type Piece, type World, boundsOf, F } from './world';

export interface Breach { gone: Piece[]; come: Piece[]; /** each chunk: its pieces, as they were in the wall */ rubble: Partial<Piece>[][]; /** what it took with it */ lamps: number[]; lo: V3; hi: V3; at: V3; /** the way it was blown */ n: V3; /** across, and up */ u: V3; v: V3; r: number; /** how thick what it went through is, all told */ thick: number }
type P2 = [number, number]; type Half = { n: P2; d: number };
const clip = (poly: P2[], h: Half): P2[] => { const out: P2[] = []; poly.forEach((a, i) => { const b = poly[(i + 1) % poly.length]!, da = a[0] * h.n[0] + a[1] * h.n[1] - h.d, db = b[0] * h.n[0] + b[1] * h.n[1] - h.d; if (da <= 0) out.push(a); if ((da < 0 && db > 0) || (da > 0 && db < 0)) { const t = da / (da - db); out.push([a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t]); } }); return out; };
const area = (p: P2[]) => Math.abs(p.reduce((s, a, i) => { const b = p[(i + 1) % p.length]!; return s + a[0] * b[1] - a[1] * b[0]; }, 0)) / 2;
const dice = (seed: number) => { let s = (seed * 2654435761 + 12345) >>> 0; return () => { s ^= s << 13; s >>>= 0; s ^= s >>> 17; s ^= s << 5; s >>>= 0; return s / 4294967296; }; };
/** which of its ways is along d, and is it square to it */
const along = (r: Rot, d: V3): number => { for (let k = 0; k < 3; k++) if (Math.abs(dot(r[k]!, d)) > 0.999) return k; return -1; };
/** the part of a box inside some half planes of (u, v): as small a box as holds it, with those planes */
function part(p: Piece, c: V3, u: V3, v: V3, rect: P2[], halves: Half[], more: Partial<Piece>): Partial<Piece> | null {
  let poly = rect; for (const h of halves) { poly = clip(poly, h); if (poly.length < 3) return null; } if (area(poly) < 2e-4) return null;
  const k = along(p.rot, u), m = along(p.rot, v), q = sub(p.at, c), cu = dot(q, u), cv = dot(q, v), us = poly.map(a => a[0]), vs = poly.map(a => a[1]), u0 = Math.min(...us), u1 = Math.max(...us), v0 = Math.min(...vs), v1 = Math.max(...vs); if (u1 - u0 < 0.004 || v1 - v0 < 0.004) return null;
  const shift = add(mul(u, (u0 + u1) / 2 - cu), mul(v, (v0 + v1) / 2 - cv)), at = add(p.at, shift), half: V3 = [...p.half]; half[k] = (u1 - u0) / 2; half[m] = (v1 - v0) / 2; const local = (w: V3): V3 => [dot(p.rot[0], w), dot(p.rot[1], w), dot(p.rot[2], w)], moved = local(shift);
  const planes: [V3, number][] = (p.planes ?? []).map(([n, d]) => [n, d - dot(n, moved)]); for (const h of halves) { const w = add(mul(u, h.n[0]), mul(v, h.n[1])), off = h.d - dot(w, sub(at, c)), reach = Math.abs(h.n[0]) * half[k]! + Math.abs(h.n[1]) * half[m]!; if (off < reach - 1e-5) planes.push([local(w), off]); }
  if (planes.length > 8) return null;
  /* what is painted on it stays where it was */ const marks = p.marks?.map(m => { const f = m.face, a: [number, number] = f < 2 ? [f === 1 ? -moved[2] : moved[2], moved[1]] : f < 4 ? [moved[0], moved[2]] : [f === 4 ? -moved[0] : moved[0], moved[1]]; return { ...m, at: [m.at[0] - a[0], m.at[1] - a[1]] as [number, number] }; }).filter(m => { const f = m.face, hx = f < 2 ? half[2] : half[0], hy = f < 2 || f > 3 ? half[1] : half[2]; return Math.abs(m.at[0]) < hx + m.size[0] / 2 && Math.abs(m.at[1]) < hy + m.size[1] / 2; }).slice(0, 2);
  return { at, rot: p.rot, half, mat: p.mat, flags: (p.flags & ~(0xfff << 16)) | F.BROKEN, tint: p.tint, seed: p.seed, thing: p.thing, ...(planes.length ? { planes } : {}), ...(marks?.length ? { marks } : {}), ...more };
}
/** blow a hole of about r where a ray along d came to a wall at `hit`. `most`: how deep it goes */
export function breach(w: World, hit: V3, d: V3, r: number, seed: number, most = 0.5): Breach | null {
  const stands = (p: Piece) => !(p.flags & (F.WATER | F.FAR | F.MESH | F.MOVES | F.OPEN | F.BEND)), near = w.pieces.filter(p => { if (!stands(p)) return false; const b = boundsOf(p); for (let k = 0; k < 3; k++) if (b.mn[k]! > hit[k]! + r + 0.6 || b.mx[k]! < hit[k]! - r - 0.6) return false; return true; });
  // the wall: the thin box the ray met
  let wall: Piece | null = null, n: V3 = d, best = 0.3; for (const p of near) { if (p.quad || p.curves || p.flags & F.GLASS) continue; const least = p.half.indexOf(Math.min(...p.half)), a = p.rot[least]!, f = Math.abs(dot(a, d)), q = sub(hit, p.at); if (p.half[least]! > 0.3 || f < 0.3 || Math.abs(a[1]) > 0.5) continue; let off = 0; for (let k = 0; k < 3; k++) off = Math.max(off, Math.abs(dot(p.rot[k]!, q)) - p.half[k]!); if (off < best && p.half[(least + 1) % 3]! > 0.15 && p.half[(least + 2) % 3]! > 0.15) { best = off; wall = p; n = dot(a, d) < 0 ? mul(a, -1) : a; } }
  if (!wall) return null; const ways = wall.rot.filter(a => Math.abs(dot(a, n)) < 0.5), v = Math.abs(ways[0]![1]) > Math.abs(ways[1]![1]) ? ways[0]! : ways[1]!, u: V3 = [n[1] * v[2] - n[2] * v[1], n[2] * v[0] - n[0] * v[2], n[0] * v[1] - n[1] * v[0]], rnd = dice(seed), c = hit;
  // the ring
  const K = 10, turn = rnd() * 6.283, ring: P2[] = Array.from({ length: K }, (_, i) => { const a = turn + (i + 0.5 * (rnd() - 0.5)) / K * 2 * Math.PI, l = r * (0.82 + 0.36 * rnd()); return [Math.cos(a) * l, Math.sin(a) * l * 1.12]; });
  const inRing = (q: P2) => { for (let i = 0; i < K; i++) { const a = ring[i]!, b = ring[(i + 1) % K]!; if (a[0] * q[1] - a[1] * q[0] >= 0 && q[0] * b[1] - q[1] * b[0] >= 0) return (b[0] - a[0]) * (q[1] - a[1]) - (b[1] - a[1]) * (q[0] - a[0]) > 0; } return false; };
  const wedges = (wide: number): Half[][] => ring.map((a0, i) => { const b0 = ring[(i + 1) % K]!, a: P2 = [a0[0] * wide, a0[1] * wide], b: P2 = [b0[0] * wide, b0[1] * wide], la = Math.hypot(...a), lb = Math.hypot(...b), e: P2 = [b[1] - a[1], a[0] - b[0]], le = Math.hypot(...e), lap = 0.0015; return [{ n: [a[1] / la, -a[0] / la], d: lap }, { n: [-b[1] / lb, b[0] / lb], d: lap }, { n: [-e[0] / le, -e[1] / le], d: -(e[0] * a[0] + e[1] * a[1]) / le }]; });
  const gone: Piece[] = [], come: Piece[] = [], cut: { p: Piece; rect: P2[]; wide: number }[] = [], lamps: number[] = []; let d0 = 1e9, d1 = -1e9, deep = 0;
  for (const p of near) {
    const q = sub(p.at, c), k = along(p.rot, u), m = along(p.rot, v), h = along(p.rot, n), square = k >= 0 && m >= 0 && h >= 0 && k !== m, cn = dot(q, n), hn = [0, 1, 2].reduce((s, a) => s + Math.abs(dot(p.rot[a]!, n)) * p.half[a]!, 0), hu = [0, 1, 2].reduce((s, a) => s + Math.abs(dot(p.rot[a]!, u)) * p.half[a]!, 0), hv = [0, 1, 2].reduce((s, a) => s + Math.abs(dot(p.rot[a]!, v)) * p.half[a]!, 0);
    if (cn - hn > most || cn + hn < -0.12 || (hn > 0.6 && p !== wall)) continue; const cu = dot(q, u), cv = dot(q, v), rect: P2[] = [[cu - hu, cv - hv], [cu + hu, cv - hv], [cu + hu, cv + hv], [cu - hu, cv + hv]], middle = inRing([cu, cv]), little = Math.max(hu, hv) < r * 0.6;
    const pane = p.flags & F.GLASS, whole = pane || p.quad || p.curves || !square || p.flags & F.GLOW || (p.lamp ?? -1) >= 0 || little; let over = 0; for (const t of wedges(1)) { let poly = rect; const inner: Half[] = [t[0]!, t[1]!, { n: [-t[2]!.n[0], -t[2]!.n[1]], d: -t[2]!.d }]; for (const g of inner) { poly = clip(poly, g); if (poly.length < 3) break; } if (poly.length >= 3) over += area(poly); } if (over < 1e-4) continue;
    if (whole) { if (middle || (pane && over > 0.01)) { gone.push(p); if ((p.lamp ?? -1) >= 0) lamps.push(p.lamp!); } continue; }
    gone.push(p); if (hn > 0.02) { d0 = Math.min(d0, cn - hn); d1 = Math.max(d1, cn + hn); deep += 2 * hn; } const wide = hn < 0.02 ? 1.06 : 0.93 + 0.14 * rnd(); cut.push({ p, rect, wide }); for (const t of wedges(wide)) { const x = part(p, c, u, v, rect, t, { stays: true }); if (x) come.push(x as Piece); }
  }
  if (!gone.length) return null;
  // the rubble: the ring's inside parted about points thrown into it
  const N = Math.round(9 + 5 * r), pts: P2[] = Array.from({ length: N }, (_, i) => { const a = i * 2.39996 + rnd() * 0.8, l = r * 0.92 * Math.sqrt((i + 0.35 + 0.3 * rnd()) / N); return [Math.cos(a) * l, Math.sin(a) * l * 1.1]; }), rubble: Partial<Piece>[][] = [];
  const round: Half[] = Array.from({ length: 6 }, (_, i) => { const a = i / 6 * 2 * Math.PI + turn; return { n: [Math.cos(a), Math.sin(a)], d: r * 0.86 }; });
  for (const [i, a] of pts.entries()) {
    const near2 = pts.map((b, j) => ({ j, l: Math.hypot(b[0] - a[0], b[1] - a[1]) })).filter(x => x.j !== i).sort((x, y) => x.l - y.l), halves: Half[] = []; let cell: P2[] = [[-2 * r, -2 * r], [2 * r, -2 * r], [2 * r, 2 * r], [-2 * r, 2 * r]];
    for (const h of [...near2.map(x => { const b = pts[x.j]!, e: P2 = [b[0] - a[0], b[1] - a[1]], l = Math.hypot(...e); return { n: [e[0] / l, e[1] / l] as P2, d: ((b[0] * b[0] + b[1] * b[1]) - (a[0] * a[0] + a[1] * a[1])) / (2 * l) - 0.006 }; }), ...round]) { const was = area(cell), next = clip(cell, h); if (next.length < 3) { cell = []; break; } if (was - area(next) > was * 0.004 && halves.length < 8) { halves.push(h); cell = next; } }
    if (cell.length < 3 || area(cell) < 0.004) continue; const chunk: Partial<Piece>[] = []; for (const { p, rect } of cut) { const x = part({ ...p, planes: undefined }, c, u, v, rect, halves, {}); if (x) chunk.push({ ...x, flags: (p.flags & F.CUT) | F.MOVES | F.BROKEN, rig: 'loose' }); } if (chunk.length) rubble.push(chunk.sort((x, y) => y.half![0] * y.half![1] * y.half![2] - x.half![0] * x.half![1] * x.half![2]).slice(0, 2));
  }
  const lo: V3 = [1e9, 1e9, 1e9], hi: V3 = [-1e9, -1e9, -1e9], reach = r * 1.4; for (let k = 0; k < 3; k++) { const e = reach * (Math.abs(u[k]!) + Math.abs(v[k]!)), a = c[k]! + n[k]! * Math.min(0, d0), b = c[k]! + n[k]! * Math.max(0, d1); lo[k] = Math.min(a, b) - e; hi[k] = Math.max(a, b) + e; }
  return { gone, come, rubble, lamps, lo, hi, at: c, n, u, v, r, thick: deep };
}
