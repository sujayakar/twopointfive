// FACES THAT SHARE A PLANE, the same way out. Which of them is seen there is a matter of rounding: it changes with the ray, and flickers.
import { type V3, add, mul, dot, sub } from '../math';
import type { Piece } from './world';
/** nearer than this and a ray across the city can not tell them apart */
export const APART = 0.0015;
/** a flat face: its middle, the way out, its sides (with their half lengths) */
interface Face { p: Piece; i: number; k: number; sign: number; at: V3; n: V3; d: number; u: V3; v: V3; hu: number; hv: number }
export interface Fight { a: Face; b: Face; gap: number }
export function facesOf(p: Piece, i: number): Face[] {
  const out: Face[] = [], q = p.quad, round = q ? [0, 1, 2].map(k => q[k] !== 0) : [false, false, false];
  for (let k = 0; k < 3; k++) {
    if (q && (round[k] || round.every(x => x) || (q[3] !== 0 && k !== 1))) continue;      // a curved side has no plane
    const u = (k + 1) % 3, v = (k + 2) % 3, s = q ? 0.7 : 1; for (const sign of [-1, 1]) { const n = mul(p.rot[k]!, sign), at = add(p.at, mul(n, p.half[k]!)); out.push({ p, i, k, sign, at, n, d: dot(n, at), u: p.rot[u]!, v: p.rot[v]!, hu: p.half[u]! * s, hv: p.half[v]! * s }); }
  }
  return out;
}
const meet = (a: Face, b: Face) => { const d = sub(b.at, a.at), least = 1e-3; for (const x of [a.u, a.v, b.u, b.v]) { const r = a.hu * Math.abs(dot(a.u, x)) + a.hv * Math.abs(dot(a.v, x)) + b.hu * Math.abs(dot(b.u, x)) + b.hv * Math.abs(dot(b.v, x)); if (Math.abs(dot(d, x)) > r - least) return false; } return true; };
export function faceFights(pieces: Piece[]): Fight[] {
  const by = new Map<string, Face[]>(), out: Fight[] = []; pieces.forEach((p, i) => { for (const f of facesOf(p, i)) { const k = f.n.map(x => Math.round(x * 2e3) + 0).join(); let l = by.get(k); if (!l) by.set(k, l = []); l.push(f); } });
  for (const l of by.values()) { l.sort((a, b) => a.d - b.d); for (let i = 0; i < l.length; i++) for (let j = i + 1; j < l.length; j++) { const gap = l[j]!.d - l[i]!.d; if (gap >= APART - 1e-6) break; if (meet(l[i]!, l[j]!)) out.push({ a: l[i]!, b: l[j]!, gap }); } }
  return out;
}
/** Part them: of two faces in one plane, the piece made later comes out by a hair. Returns how many are left */
export function settle(pieces: Piece[]): number {
  for (let pass = 0; pass < 32; pass++) {
    const fights = faceFights(pieces); if (!fights.length) return 0; const out = new Map<string, { f: Face; by: number }>();
    for (const { a, b, gap } of fights) { const later = a.i > b.i ? a : b, by = APART * 1.02 + (later === b ? -gap : gap), k = `${later.i} ${later.k} ${later.sign}`, was = out.get(k); if (!was || was.by < by) out.set(k, { f: later, by }); }
    for (const { f, by } of out.values()) { const half = [...f.p.half] as [number, number, number]; half[f.k]! += by / 2; f.p.half = half; f.p.at = add(f.p.at, mul(f.n, by / 2)); }
  }
  return faceFights(pieces).length;
}

/** Which of a box's twelve edges stand free: nothing beside them in either face's plane, nor across the corner. Edge c·4 + i + 2j runs along axis c; i, j say which side of the next two axes */
export function freeEdges(pieces: Piece[]): Map<Piece, number> {
  const G = 2, grid = new Map<string, Piece[]>(), key = (x: number, y: number, z: number) => `${Math.floor(x / G)},${Math.floor(y / G)},${Math.floor(z / G)}`, big: Piece[] = [], out = new Map<Piece, number>();
  for (const p of pieces) { const e = [0, 1, 2].map(k => Math.abs(p.rot[0][k]!) * p.half[0] + Math.abs(p.rot[1][k]!) * p.half[1] + Math.abs(p.rot[2][k]!) * p.half[2]); if (Math.max(...e) > 24) { big.push(p); continue; } for (let x = Math.floor((p.at[0] - e[0]!) / G); x <= Math.floor((p.at[0] + e[0]!) / G); x++) for (let y = Math.floor((p.at[1] - e[1]!) / G); y <= Math.floor((p.at[1] + e[1]!) / G); y++) for (let z = Math.floor((p.at[2] - e[2]!) / G); z <= Math.floor((p.at[2] + e[2]!) / G); z++) { const k = `${x},${y},${z}`; let l = grid.get(k); if (!l) grid.set(k, l = []); l.push(p); } }
  const inside = (p: Piece, x: V3) => { const d = sub(x, p.at); for (let k = 0; k < 3; k++) if (Math.abs(dot(d, p.rot[k]!)) > p.half[k]! + 1e-4) return false; return true; }, taken = (x: V3, not: Piece) => { for (const p of grid.get(key(...x)) ?? []) if (p !== not && inside(p, x)) return true; for (const p of big) if (p !== not && inside(p, x)) return true; return false; }, D = 0.006;
  for (const p of pieces) {
    if (p.quad || p.planes?.length) continue; let mask = 0;
    for (let c = 0; c < 3; c++) { const A = (c + 1) % 3, B = (c + 2) % 3; if (Math.min(p.half[A]!, p.half[B]!) < 0.004) continue; for (let i = 0; i < 2; i++) for (let j = 0; j < 2; j++) {
      const sa = i ? 1 : -1, sb = j ? 1 : -1, na = mul(p.rot[A]!, sa), nb = mul(p.rot[B]!, sb), mid = add(add(p.at, mul(na, p.half[A]!)), mul(nb, p.half[B]!)); let free = 0; const N = 7;
      for (let t = 0; t < N; t++) { const x = add(mid, mul(p.rot[c]!, p.half[c]! * ((t + 0.5) / N * 2 - 1))); if (!taken(add(x, add(mul(na, D), mul(nb, -D))), p) && !taken(add(x, add(mul(na, -D), mul(nb, D))), p) && !taken(add(x, add(mul(na, D), mul(nb, D))), p)) free++; }
      if (free >= N - 2) mask |= 1 << (c * 4 + i + 2 * j); } }
    if (mask) out.set(p, mask);
  }
  return out;
}
