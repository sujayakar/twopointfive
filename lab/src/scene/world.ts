// THE WORLD, as data: pieces, leaf clusters, lamps, openings, marks. Nothing here knows about the GPU.
import { type V3, type Rot, ID, add, mul, apply, norm, cross, sub } from '../math';
import { buildBvh, type Bounds, type Bvh } from '../bvh';

/** what a piece is to a ray */
export const F = { GLASS: 1 << 8, WATER: 1 << 9, CUT: 1 << 10, GLOW: 1 << 11, NOSHADOW: 1 << 12, QUAD: 1 << 13, MOVES: 1 << 14, FAR: 1 << 15, /** grows on the terrace, out of doors: the section leaves it whole, or hides it whole when the terrace itself is hidden */ OPEN: 1 << 28 } as const;

/** ONE PIECE: a frame and half extents, cut by up to 8 planes and at most one quadric (A x² + B y² + C z² + D y ≤ 1) */
export interface Piece {
  at: V3; rot: Rot; half: V3; mat: number; flags: number;
  quad?: [number, number, number, number]; planes?: [V3, number][];
  tint?: V3; lamp?: number; marks?: Mark[]; seed?: number;
  /** what moves it, by name */ rig?: string;
}
/** a sign painted on one face: rect in metres about the face's middle, a cell of the atlas, a color */
export interface Mark { face: number; at: [number, number]; size: [number, number]; art: string; color: V3; turn?: number }
export interface Lamp { kind: 'rect' | 'sphere' | 'spot'; at: V3; u: V3; v: V3; radiance: V3; cos?: number; name: string; on: number; night?: boolean; n?: V3 }
/** where the sky comes in: a rectangle, its normal pointing in */
export interface Opening { at: V3; u: V3; v: V3; n: V3 }
export interface Leaf { at: V3; n: V3; t: V3; size: number; tone: number; wood?: boolean }
export interface Cluster { leaves: Leaf[]; kind: number }
/** one leaf cluster somewhere: world = at + rot · scale · local */
export interface Instance { cluster: number; at: V3; rot: Rot; scale: number; base: V3; sway: number; phase: number; tint: V3; /** out of doors: the section leaves it whole */ open?: boolean }

export class World {
  pieces: Piece[] = []; moving: Piece[] = []; lamps: Lamp[] = []; openings: Opening[] = []; clusters: Cluster[] = []; instances: Instance[] = []; plumes: { at: V3; r: number; h: number; k: number }[] = [];
  add(p: Partial<Piece> & { at: V3; half: V3; mat: number }): Piece {
    const full: Piece = { rot: ID, flags: 0, ...p }; if (full.quad) full.flags |= F.QUAD;
    (full.flags & F.MOVES ? this.moving : this.pieces).push(full); return full;
  }
  /** a box by its corners */
  box(a: V3, b: V3, mat: number, more: Partial<Piece> = {}) { return this.add({ at: mul(add(a, b), 0.5), half: [Math.abs(b[0] - a[0]) / 2, Math.abs(b[1] - a[1]) / 2, Math.abs(b[2] - a[2]) / 2], mat, ...more }); }
  cylinder(at: V3, r: number, h: number, mat: number, more: Partial<Piece> = {}) { return this.add({ at, half: [r, h, r], mat, quad: [1 / (r * r), 0, 1 / (r * r), 0], ...more }); }
  ellipsoid(at: V3, half: V3, mat: number, more: Partial<Piece> = {}) { return this.add({ at, half, mat, quad: [1 / half[0] ** 2, 1 / half[1] ** 2, 1 / half[2] ** 2, 0], ...more }); }
  /** a taper from a (radius ra) to b (radius rb) */
  taper(a: V3, b: V3, ra: number, rb: number, mat: number, more: Partial<Piece> = {}) { return this.add({ ...taper(a, b, ra, rb), mat, ...more }); }
  lamp(l: Lamp) { this.lamps.push(l); return this.lamps.length - 1; }
}
export function taper(a: V3, b: V3, ra: number, rb: number): { at: V3; half: V3; rot: Rot; quad: [number, number, number, number] } {
  const d = sub(b, a), l = Math.hypot(...d), y = norm(d); let x = cross(y, [0, 0, 1]); if (Math.hypot(...x) < 1e-3) x = cross(y, [1, 0, 0]); x = norm(x);
  const r0 = (ra + rb) / 2, k = (rb - ra) / l, big = Math.max(ra, rb);
  return { at: mul(add(a, b), 0.5), half: [big, l / 2, big], rot: [x, y, cross(x, y)], quad: [1 / (r0 * r0), -(k * k) / (r0 * r0), 1 / (r0 * r0), -2 * k / r0] };
}

export function boundsOf(p: Piece): Bounds {
  const mn: [number, number, number] = [0, 0, 0], mx: [number, number, number] = [0, 0, 0];
  for (let k = 0; k < 3; k++) { const e = Math.abs(p.rot[0][k]!) * p.half[0] + Math.abs(p.rot[1][k]!) * p.half[1] + Math.abs(p.rot[2][k]!) * p.half[2]; mn[k] = p.at[k]! - e - 1e-4; mx[k] = p.at[k]! + e + 1e-4; }
  return { mn, mx };
}
export const PIECE_FLOATS = 24;
/** a piece as the shaders read it: three rows world → local, half + flags, quadric, (material | marks, tint, planes, lamp | seed) */
export function packPiece(p: Piece, f: Float32Array, u: Uint32Array, o: number, planeAt: number, markAt: number) {
  for (let k = 0; k < 3; k++) { const r = p.rot[k]!; f[o + k * 4] = r[0]; f[o + k * 4 + 1] = r[1]; f[o + k * 4 + 2] = r[2]; f[o + k * 4 + 3] = -(r[0] * p.at[0] + r[1] * p.at[1] + r[2] * p.at[2]); }
  f.set(p.half, o + 12); u[o + 15] = (p.flags | (p.planes?.length ?? 0)) >>> 0;
  f.set(p.quad ?? [0, 0, 0, 0], o + 16);
  const t = p.tint ?? [1, 1, 1], q = (x: number) => Math.max(0, Math.min(255, Math.round(x * 255)));
  u[o + 20] = (p.mat | ((p.marks?.length ?? 0) << 8) | (markAt << 12)) >>> 0;
  u[o + 21] = (q(t[0]) | (q(t[1]) << 8) | (q(t[2]) << 16)) >>> 0;
  u[o + 22] = planeAt >>> 0; u[o + 23] = (((p.lamp ?? -1) & 0xff) | ((p.seed ?? 0) << 8)) >>> 0;
}
/** HOW OPEN each side of each leaf is, next to the fog its cluster is to a bounce: what its own neighbours keep from it, which no fog can tell. Found once */
export function ownShade(c: Cluster): [number, number][] {
  const L = c.leaves, n = L.length, far = 0.9, dirs: V3[] = [], K = 14; for (let i = 0; i < K; i++) { const u = (i + 0.5) / K, a = i * 2.39996, s = Math.sqrt(u); dirs.push([s * Math.cos(a), s * Math.sin(a), Math.sqrt(1 - u)]); }
  const wide = (l: Leaf) => l.wood ? 0.022 : c.kind === 1 ? 0.06 : 0.46, side = L.map(l => cross(l.n, l.t)); let area = 0; for (const l of L) if (!l.wood) area += l.size * l.size * 4 * wide(l) * (2 / 3); const sigma = c.kind === 1 ? 1.3 : 0.5 * area / 4.19 * 0.75;
  return L.map((l, i) => { const out: [number, number] = [1, 1]; for (let f = 0; f < 2; f++) { const z = mul(l.n, f ? -1 : 1), x = l.t, y = side[i]!; let seen = 0, fog = 0;
    for (const d0 of dirs) { const d: V3 = [x[0] * d0[0] + y[0] * d0[1] + z[0] * d0[2], x[1] * d0[0] + y[1] * d0[1] + z[1] * d0[2], x[2] * d0[0] + y[2] * d0[1] + z[2] * d0[2]], o = add(l.at, mul(z, 0.012)); let open = 1;
      for (let j = 0; j < n; j++) { if (j === i) continue; const m = L[j]!, e: V3 = [m.at[0] - o[0], m.at[1] - o[1], m.at[2] - o[2]], b = e[0] * d[0] + e[1] * d[1] + e[2] * d[2]; if (b < -m.size || b > far + m.size) continue; const r2 = e[0] * e[0] + e[1] * e[1] + e[2] * e[2] - b * b; if (r2 > m.size * m.size) continue;
        const den = m.n[0] * d[0] + m.n[1] * d[1] + m.n[2] * d[2]; if (Math.abs(den) < 1e-5) continue; const t = (m.n[0] * e[0] + m.n[1] * e[1] + m.n[2] * e[2]) / den; if (t < 1e-3 || t > far) continue; const q: V3 = [o[0] + d[0] * t - m.at[0], o[1] + d[1] * t - m.at[1], o[2] + d[2] * t - m.at[2]], u = (q[0] * m.t[0] + q[1] * m.t[1] + q[2] * m.t[2]) / m.size; if (Math.abs(u) >= 1) continue; const s = side[j]!, v = (q[0] * s[0] + q[1] * s[1] + q[2] * s[2]) / m.size; if (Math.abs(v) < wide(m) * (1 - u * u) * (1 + 0.35 * u)) { open = 0; break; } }
      seen += open; const bb = o[0] * d[0] + o[1] * d[1] + o[2] * d[2], cc = o[0] * o[0] + o[1] * o[1] + o[2] * o[2] - 1, h = bb * bb - cc; fog += h > 0 ? Math.exp(-sigma * Math.min(far, Math.max(0, -bb + Math.sqrt(h)))) : 1; }
    out[f] = Math.max(0.04, Math.min(1, seen / Math.max(fog, 1e-3))); } return out; });
}
/** leaves as the shaders read them: centre + size, normal and long axis packed, tone */
export function packCluster(c: Cluster): { bvh: Bvh; data: ArrayBuffer } {
  const b = c.leaves.map(l => { const e = l.size; return { mn: [l.at[0] - e, l.at[1] - e, l.at[2] - e], mx: [l.at[0] + e, l.at[1] + e, l.at[2] + e] } as Bounds; });
  const ao = ownShade(c), bvh = buildBvh(b, 3), data = new ArrayBuffer(c.leaves.length * 32), f = new Float32Array(data), u = new Uint32Array(data);
  bvh.order.forEach((src, i) => { const l = c.leaves[src]!, o = i * 8; f.set(l.at, o); f[o + 3] = l.size; u[o + 4] = oct(l.n); u[o + 5] = oct(l.t); f[o + 6] = l.tone; const a = ao[src]!; u[o + 7] = (l.wood ? 2 : c.kind) | (Math.round(a[0] * 255) << 8) | (Math.round(a[1] * 255) << 16); });
  return { bvh, data };
}
function oct(n: V3): number { const s = Math.abs(n[0]) + Math.abs(n[1]) + Math.abs(n[2]); let x = n[0] / s, y = n[2] / s; if (n[1] < 0) { const a = (1 - Math.abs(y)) * Math.sign(x || 1), b = (1 - Math.abs(x)) * Math.sign(y || 1); x = a; y = b; } const q = (v: number) => Math.round((v * 0.5 + 0.5) * 65535) & 0xffff; return (q(x) | (q(y) << 16)) >>> 0; }
export const worldOf = (i: Instance, v: V3): V3 => add(i.at, apply(i.rot, mul(v, i.scale)));
