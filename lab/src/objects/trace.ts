// Rays through a level without a GPU: what is met, its normal and color. For sketches, and for asking what a level blocks and sends back.
import { type V3, add, sub, mul, dot, cross, norm, apply } from '../math';
import { F, boundsOf, type Piece } from '../scene/world';
import { MATERIALS } from '../scene/materials';
import { settle } from '../scene/faces';
import { enters, outside } from './shape';
import { build } from './rules';
import type { Made } from './object';
export interface Leaf { at: V3; n: V3; t: V3; b: V3; r: number; wide: number; c: V3; through: V3; kind: number; wood: boolean }
interface N { mn: ArrayLike<number>; mx: ArrayLike<number>; a?: N; b?: N; l?: number[] }
/** boxes in a tree, for rays */
export class Tree {
  private root: N | null; looked = 0;
  constructor(items: { mn: ArrayLike<number>; mx: ArrayLike<number>; k: number }[]) { this.root = items.length ? this.make(items) : null; }
  private make(l: { mn: ArrayLike<number>; mx: ArrayLike<number>; k: number }[]): N { const mn = [0, 1, 2].map(a => Math.min(...l.map(i => i.mn[a]!))), mx = [0, 1, 2].map(a => Math.max(...l.map(i => i.mx[a]!))); if (l.length <= 3) return { mn, mx, l: l.map(i => i.k) }; const a = [0, 1, 2].reduce((p, q) => mx[q]! - mn[q]! > mx[p]! - mn[p]! ? q : p), s = l.slice().sort((p, q) => p.mn[a]! + p.mx[a]! - q.mn[a]! - q.mx[a]!), m = s.length >> 1; return { mn, mx, a: this.make(s.slice(0, m)), b: this.make(s.slice(m)) }; }
  walk(o: V3, d: V3, far: () => number, f: (k: number) => void) { const st: N[] = this.root ? [this.root] : [], inv = d.map(x => 1 / (Math.abs(x) < 1e-9 ? 1e-9 : x)); while (st.length) { const n = st.pop()!; this.looked++; let lo = 0, hi = far(); for (let a = 0; a < 3; a++) { const p = (n.mn[a]! - o[a]!) * inv[a]!, q = (n.mx[a]! - o[a]!) * inv[a]!; lo = Math.max(lo, Math.min(p, q)); hi = Math.min(hi, Math.max(p, q)); } if (lo > hi) continue; if (n.l) for (const k of n.l) f(k); else st.push(n.a!, n.b!); } }
}
/** what a piece sends back: its material's color by its tint */
export const sends = (p: Piece): V3 => { const q = MATERIALS[p.mat]!, t = p.tint ?? [1, 1, 1]; return [0, 1, 2].map(a => Math.min(0.94, q.base[a]! * Math.min(1, t[a]!) ** 2 * (q.metal ? 0.75 : 1) + (q.glow ? 0.6 * t[a]! : 0))) as V3; };
export interface Met { t: number; k: number }
export interface Traced { tree: Tree; pieces: Piece[]; leaves: Leaf[]; meet(o: V3, d: V3, far: number, any?: boolean): Met; /** normal (toward the ray), what is sent back, and what a leaf lets through */ at(m: Met, o: V3, d: V3): { x: V3; n: V3; c: V3; through: V3 | null } }
/** what a leaf sends back off its upper side, and through (wgsl/surface.wgsl leafSurf, as a whole) */
function leafColors(tint: V3, tone: number, kind: number, wood: boolean): { c: V3; through: V3 } { if (wood) { const k = 0.7 + 0.5 * tone; return { c: [0.16 * k, 0.115 * k, 0.075 * k], through: [0, 0, 0] }; } const c = tint.map(t => t * t * (0.58 + 0.7 * tone) * (kind === 1 ? 0.7 : 1)) as V3, k = kind === 1 || c[0] > c[1] ? 1.5 : 1.45 * (1 - Math.exp(-20 * c[0])), th: V3 = [c[0] * k, c[1] * k * 0.96, c[2] * k * 0.6], most = Math.max(...c.map((v, i) => v + th[i]!)), s = Math.min(1, 0.88 / Math.max(1e-3, most)); return { c: mul(c, s), through: mul(th, s) }; }
/** what stands, as a scene would have it (faces in one plane parted) */
export function traced(m: Made, level: number): Traced {
  const b = build(m, level); settle(b.w.pieces); const leaves: Leaf[] = b.w.instances.flatMap(i => b.w.clusters[i.cluster]!.leaves.map(l => { const n = apply(i.rot, l.n), t = apply(i.rot, l.t); return { at: add(i.at, apply(i.rot, mul(l.at, i.scale))), n, t, b: cross(n, t), r: l.size * i.scale, wide: l.wood ? 0.022 : b.w.clusters[i.cluster]!.kind === 1 ? 0.06 : 0.46, ...leafColors(i.tint, l.tone, b.w.clusters[i.cluster]!.kind, !!l.wood), kind: b.w.clusters[i.cluster]!.kind, wood: !!l.wood }; })); return tracedOf([...b.w.pieces, ...b.w.moving].filter(p => p.at[1] > -50), leaves);
}
export function tracedOf(pieces: Piece[], leaves: Leaf[]): Traced {
  const P = pieces.length, tree = new Tree([...pieces.map((p, k) => ({ ...boundsOf(p), k })), ...leaves.map((l, k) => ({ mn: l.at.map(x => x - l.r), mx: l.at.map(x => x + l.r), k: k + P }))]);
  const meet = (o: V3, d: V3, far: number, any = false): Met => { let t = far, k = -1; tree.walk(o, d, () => any && k >= 0 ? -1 : t, i => { if (any && k >= 0) return; if (i < P) { if (any && pieces[i]!.flags & (F.NOSHADOW | F.GLASS)) return; const e = enters(pieces[i]!, o, d); if (e > 1e-4 && e < t) { t = e; k = i; } } else { const l = leaves[i - P]!, den = dot(l.n, d); if (Math.abs(den) < 1e-6) return; const e = dot(sub(l.at, o), l.n) / den; if (e < 1e-4 || e >= t) return; const x = sub(add(o, mul(d, e)), l.at), u = dot(x, l.t) / l.r, v = dot(x, l.b) / l.r; if (Math.abs(u) < 1 && Math.abs(v) < l.wide * (1 - u * u) * (1 + 0.35 * u)) { t = e; k = i; } } }); return { t, k }; };
  const at = (m: Met, o: V3, d: V3) => { const x = add(o, mul(d, m.t)); if (m.k >= P) { const l = leaves[m.k - P]!, under = dot(l.n, d) > 0; return { x, n: under ? mul(l.n, -1) : l.n, c: under && l.kind === 0 && !l.wood ? mul(l.c, 1.45) : l.c, through: l.wood ? null : l.through }; } const p = pieces[m.k]!, e = 4e-4, n = norm([0, 1, 2].map(a => { const q: V3 = [...x]; q[a]! += e; const r: V3 = [...x]; r[a]! -= e; return outside(p, q) - outside(p, r); }) as V3); return { x, n, c: sends(p), through: null }; };
  return { tree, pieces, leaves, meet, at };
}
