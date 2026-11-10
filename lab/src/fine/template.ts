// SHAPES THAT FIT. A scene has thousands of boxes of as many sizes and few kinds of edges: so there are few shapes, and each fits any box. A vertex knows how much it goes with each side of the box (s, −1 to 1)
// and where it is from there in metres (o): its place is s · half + o. Edges, piping and chips keep their size; what lies between them is stretched. A body's vertices know their two bones instead.
import { rng, type V3 } from '../math';
import { Simplifier, Pool } from '../mesh/simplify';
import { parts, CLUSTER } from '../mesh/dag';
import { normalsOf, split4, type Mesh } from '../mesh/mesh';
import { ball, noise3, fbm3 } from '../mesh/shapes';
import { solid, lighten } from '../mesh/made';

export const PLAIN = 0, FITS = 1, SKINNED = 2;
export interface Shape {
  name: string; kind: number; mesh: Mesh;
  /** FITS: three a vertex; how far from a side its edges reach (thinner boxes have thinner ones); half of the box it was made as */ s?: Float32Array; reach?: number; half?: V3;
  /** its curvature goes with the box: errors too, and normals are the fitted shape's */ round?: boolean;
  /** levels made by hand, coarser than what simplifying the finest gives; each with what it is off */ coarse?: { mesh: Mesh; s: Float32Array; error: number }[];
  /** SKINNED: two bones a vertex and how much it goes with the first; each bone's rest: rows world → its own */ bones?: Uint8Array; weight?: Float32Array; rest?: Float32Array;
}
type Made = { mesh: Mesh; s: Float32Array; error: number };
const flat = (n: number) => new Uint8Array(n * 4).fill(255);
/** a box with its edges cut by c (44 triangles: each face keeps its own normal, the cuts run from one to the next), or sharp (12) */
function plainBox(h: V3, c: number, error: number): Made {
  const pos: number[] = [], nrm: number[] = [], s: number[] = [], tri: number[] = [], at: Record<string, number> = {}; for (let a = 0; a < 3; a++) for (const d of [-1, 1]) for (const u of [-1, 1]) for (const v of [-1, 1]) { const q = [0, 0, 0], n = [0, 0, 0], k = (a + 1) % 3, l = (a + 2) % 3, sg = [0, 0, 0]; q[a] = d * h[a]!; q[k] = u * (h[k]! - c); q[l] = v * (h[l]! - c); n[a] = d; sg[a] = d; sg[k] = u; sg[l] = v; at[`${a},${sg}`] = pos.length / 3; pos.push(...q); nrm.push(...n); s.push(...sg); }
  const id = (a: number, sg: number[]) => at[`${a},${sg}`]!, push = (x: number, y: number, z: number, out: number[]) => { const p = (i: number) => [pos[i * 3]!, pos[i * 3 + 1]!, pos[i * 3 + 2]!], A = p(x), B = p(y), C = p(z), n = [(B[1]! - A[1]!) * (C[2]! - A[2]!) - (B[2]! - A[2]!) * (C[1]! - A[1]!), (B[2]! - A[2]!) * (C[0]! - A[0]!) - (B[0]! - A[0]!) * (C[2]! - A[2]!), (B[0]! - A[0]!) * (C[1]! - A[1]!) - (B[1]! - A[1]!) * (C[0]! - A[0]!)]; if (n[0]! * out[0]! + n[1]! * out[1]! + n[2]! * out[2]! < 0) tri.push(x, z, y); else tri.push(x, y, z); };
  for (let a = 0; a < 3; a++) for (const d of [-1, 1]) { const k = (a + 1) % 3, l = (a + 2) % 3, g = (u: number, v: number) => { const q = [0, 0, 0]; q[a] = d; q[k] = u; q[l] = v; return id(a, q); }, o = [0, 0, 0]; o[a] = d; push(g(-1, -1), g(1, -1), g(1, 1), o); push(g(-1, -1), g(1, 1), g(-1, 1), o); }
  if (c > 0) { for (let a = 0; a < 3; a++) for (const u of [-1, 1]) for (const v of [-1, 1]) { const k = (a + 1) % 3, l = (a + 2) % 3, q = (e: number) => { const g = [0, 0, 0]; g[a] = e; g[k] = u; g[l] = v; return g; }, o = [0, 0, 0]; o[k] = u; o[l] = v; push(id(k, q(-1)), id(k, q(1)), id(l, q(1)), o); push(id(k, q(-1)), id(l, q(1)), id(l, q(-1)), o); } for (const x of [-1, 1]) for (const y of [-1, 1]) for (const z of [-1, 1]) push(id(0, [x, y, z]), id(1, [x, y, z]), id(2, [x, y, z]), [x, y, z]); }
  return { mesh: { pos: new Float32Array(pos), nrm: new Float32Array(nrm), col: flat(pos.length / 3), tri: new Uint32Array(tri) }, s: new Float32Array(s), error };
}
/** a drum of n sides, its caps apart from its side */
function plainDrum(h: V3, n: number, error: number): Made {
  const pos: number[] = [], nrm: number[] = [], s: number[] = [], tri: number[] = []; for (const y of [-1, 1]) for (let i = 0; i < n; i++) { const a = i / n * 2 * Math.PI, c = Math.cos(a), d = Math.sin(a); pos.push(c * h[0], y * h[1], d * h[2]); nrm.push(c, 0, d); s.push(c, y, d); }
  for (let i = 0; i < n; i++) { const a = i, b = (i + 1) % n; tri.push(a, a + n, b, b, a + n, b + n); } for (const y of [-1, 1]) { const b = pos.length / 3; for (let i = 0; i < n; i++) { const a = i / n * 2 * Math.PI; pos.push(Math.cos(a) * h[0], y * h[1], Math.sin(a) * h[2]); nrm.push(0, y, 0); s.push(Math.cos(a), y, Math.sin(a)); } for (let i = 1; i + 1 < n; i++) if (y > 0) tri.push(b, b + i + 1, b + i); else tri.push(b, b + i, b + i + 1); }
  return { mesh: { pos: new Float32Array(pos), nrm: new Float32Array(nrm), col: flat(pos.length / 3), tri: new Uint32Array(tri) }, s: new Float32Array(s), error };
}
const step = (a: number, b: number, x: number) => { const t = Math.max(0, Math.min(1, (x - a) / (b - a))); return t * t * (3 - 2 * t); }, white = (): [number, number, number, number] => [1, 1, 1, 1];
const rbox = (x: number, y: number, z: number, h: V3, r: number) => { const a = Math.abs(x) - h[0] + r, b = Math.abs(y) - h[1] + r, c = Math.abs(z) - h[2] + r; return Math.min(Math.max(a, b, c), 0) + Math.hypot(Math.max(a, 0), Math.max(b, 0), Math.max(c, 0)) - r; };
/** what goes with a side within `reach` of it, and between the sides in proportion */
function fit(name: string, mesh: Mesh, h: V3, reach: number, more: Partial<Shape> = {}): Shape { const s = new Float32Array(mesh.pos.length); for (let i = 0; i < s.length; i++) { const k = i % 3; s[i] = Math.max(-1, Math.min(1, mesh.pos[i]! / (h[k]! - reach))); } return { name, kind: FITS, mesh, s, reach, half: h, ...more }; }

/** a box with its edges round by r, worn and chipped so much */
export function box(name: string, seed: number, r: number, worn: number, chips: number): Shape {
  const o = rng(seed)() * 60, reach = Math.max(0.012, r * 2.2 + worn * 2), h: V3 = [0.2, 0.2, 0.2];
  const m = solid((x, y, z) => { const edge = [x, y, z].map((v, k) => Math.abs(v) - h[k]! + reach).sort((a, b) => b - a), near = step(0, reach, edge[1]!); let d = rbox(x, y, z, h, r) + near * worn * (0.5 + fbm3(x * 14 + o, y * 14, z * 14, 3)); if (chips > 0) d += near * chips * step(0.20, 0.42, fbm3(x * 11 + o, y * 11 + 4, z * 11, 3)); return d; }, h, 0.011, 2, () => 0, white, 0.0003);
  return fit(name, m, h, reach, { coarse: [plainBox(h, r * 0.7 + worn, r * 0.35 + worn), plainBox(h, 0, r * 0.7 + worn + chips * 0.5)] });
}
/** a cushion: round, full on its wide sides, with piping */
export function cushionBox(seed: number): Shape {
  const o = rng(seed)() * 60, h: V3 = [0.25, 0.25, 0.25], reach = 0.05, m = solid((x, y, z) => { const a = Math.abs(x) - h[0], b = Math.abs(y) - h[1], c = Math.abs(z) - h[2]; let d = rbox(x, y, z, h, 0.035); for (const [p, q, e] of [[a, b, c], [b, c, a], [c, a, b]] as [number, number, number][]) if (e < -0.01) d = Math.min(d, Math.hypot(p + 0.006, q + 0.03) - 0.0065 + Math.max(0, e + 0.03) * 0.3); return d; }, h, 0.013, 2, (x, y, z) => 0.0012 * fbm3(x * 22 + o, y * 22, z * 22, 3), white, 0.0005);
  return fit('cushion', m, h, reach, { coarse: [plainBox(h, 0.026, 0.009), plainBox(h, 0, 0.02)] });
}
/** a drum: its rims round by r, a ring round each end */
export function drum(name: string, r: number, ring: number): Shape {
  const h: V3 = [0.2, 0.2, 0.2], f = (x: number, y: number, z: number) => { const q = Math.hypot(x, z) - h[0] + r, b = Math.abs(y) - h[1] + r; let d = Math.min(Math.max(q, b), 0) + Math.hypot(Math.max(q, 0), Math.max(b, 0)) - r; if (ring > 0) d = Math.min(d, Math.hypot(Math.hypot(x, z) - h[0], Math.abs(y) - h[1] + 0.02) - ring); return d; }, m = solid(f, h, 0.009, 1, () => 0, white, 0.00008);
  // (its round goes with the box all the way: only what stands off it, and its rims, are in metres)
  const s = new Float32Array(m.pos.length); for (let i = 0; i < s.length; i += 3) { const x = m.pos[i]!, z = m.pos[i + 2]!, l = Math.hypot(x, z), k = l < h[0] - 0.02 ? l / h[0] : 1; s[i] = l > 1e-6 ? x / l * k : 0; s[i + 2] = l > 1e-6 ? z / l * k : 0; s[i + 1] = Math.max(-1, Math.min(1, m.pos[i + 1]! / (h[1] - 0.03))); }
  return { name, kind: FITS, mesh: m, s, reach: 0.02, half: h, round: true, coarse: [plainDrum(h, 28, 0.0018 + r * 0.4 + ring * 0.6), plainDrum(h, 16, 0.0045 + r * 0.4 + ring), plainDrum(h, 9, 0.013 + r * 0.4 + ring)] };
}
export function round(name: string, n: number, rough: number): Shape { const b = ball(n), h: V3 = [0.2, 0.2, 0.2], pos = b.pos.map((v, i) => { const j = i - i % 3, k = 1 + rough * fbm3(b.pos[j]! * 3, b.pos[j + 1]! * 3, b.pos[j + 2]! * 3, 4); return v * 0.2 * k; }) as Float32Array, col = new Uint8Array(pos.length / 3 * 4).fill(255), s = pos.map(v => v / 0.2) as Float32Array; return { name, kind: FITS, mesh: { pos, nrm: normalsOf(pos, b.tri), col, tri: b.tri }, s, reach: 0.02, half: h, round: true }; }
/** a limb of a tree: ridged along, its ends open into the next; all of it goes with the box */
export function limb(seed: number): Shape {
  const o = rng(seed)() * 50, make = (A: number, H: number, error: number): Made => { const pos: number[] = [], tri: number[] = [], col: number[] = []; for (let j = 0; j <= H; j++) for (let i = 0; i < A; i++) { const a = i / A * 2 * Math.PI, y = j / H * 2 - 1, c = Math.cos(a), q = Math.sin(a), ridge = Math.abs(noise3(c * 3.2 + o, y * 2.2 + 0.4 * noise3(c * 5, y * 7, q * 5), q * 3.2)), k = A < 10 ? 1 : 0.96 + 0.07 * ridge - 0.03 * fbm3(c * 8, y * 14 + o, q * 8, 3) + 0.04 * Math.max(0, noise3(c * 1.5 + o, y * 1.3, q * 1.5)); pos.push(c * 0.2 * k, y * 0.2, q * 0.2 * k); const b = Math.min(255, 255 * (0.86 + 0.5 * ridge)); col.push(b, b, b, 255); }
    for (let j = 0; j < H; j++) for (let i = 0; i < A; i++) { const a = j * A + i, b = j * A + (i + 1) % A, c = a + A, d = b + A; tri.push(a, c, b, b, c, d); } for (const [j, y] of [[0, -1], [H, 1]] as const) { const c = pos.length / 3; pos.push(0, y * 0.2, 0); col.push(220, 220, 220, 255); for (let i = 0; i < A; i++) { const a = j * A + i, b = j * A + (i + 1) % A; if (y < 0) tri.push(c, a, b); else tri.push(c, b, a); } }
    const P = new Float32Array(pos), T = new Uint32Array(tri); return { mesh: { pos: P, nrm: normalsOf(P, T), col: new Uint8Array(col), tri: T }, s: P.map(v => v / 0.2) as Float32Array, error }; }, m = make(64, 40, 0);
  return { name: 'limb', kind: FITS, mesh: m.mesh, s: m.s, reach: 0.02, half: [0.2, 0.2, 0.2], round: true, coarse: [make(24, 10, 0.004), make(12, 4, 0.009), make(7, 1, 0.017)] };
}
/** a flat slab that lies on a face, its top at +half y: a paver, a board, a panel of concrete, a sheet of roofing */
export function slab(name: string, seed: number, r: number, unevenness: number, pits: number, grain: 'stone' | 'wood' | 'cast' | 'felt'): Shape {
  const o = rng(seed)() * 90, h: V3 = [0.3, 0.006, 0.3], reach = 0.02; const m = solid((x, y, z) => rbox(x, y, z, h, r) + 0.0015 * step(-0.02, 0, Math.max(Math.abs(x) - h[0], Math.abs(z) - h[2])) * (0.5 + fbm3(x * 12 + o, 1, z * 12, 3)), h, 0.013, 2, (x, y, z) => y < 0 ? 0 : grain === 'wood' ? unevenness * fbm3(x * 5 + o, 0, z * 110, 3) : unevenness * fbm3(x * 24 + o, 0, z * 24, 3) - pits * step(0.36, 0.52, noise3(x * 55 + o, 3, z * 55)),
    (x, y, z) => { const b = fbm3(x * 3 + o, 5, z * 3, 3), p = step(0.36, 0.52, noise3(x * 55 + o, 3, z * 55)), v = grain === 'wood' ? 0.86 + 0.26 * fbm3(x * 4 + o, 0, z * 120, 3) : 0.94 + 0.08 * b - (pits > 0 ? 0.12 * p : 0); void y; return [v, v, v, 1]; }, 0.0004);
  const out = fit(name, m, h, reach); for (let i = 1; i < out.s!.length; i += 3) out.s![i] = Math.max(-1, Math.min(1, m.pos[i]! / h[1])) ; return { ...out, half: h, coarse: [plainBox(h, 0.0025, 0.0012 + unevenness), plainBox(h, 0, 0.0028 + unevenness + pits)] };
}

// ---------------------------------------------------------------- levels, and how they lie for the GPU
export interface Packed { name: string; data: Uint32Array; levels: { triangles: number; error: number }[]; kind: number }
const oct = (x: number, y: number, z: number): number => { const s = Math.abs(x) + Math.abs(y) + Math.abs(z) || 1; let a = x / s, b = z / s; if (y < 0) { const p = (1 - Math.abs(b)) * (a < 0 ? -1 : 1), q = (1 - Math.abs(a)) * (b < 0 ? -1 : 1); a = p; b = q; } const q = (v: number) => Math.round((v * 0.5 + 0.5) * 65535) & 0xffff; return (q(a) | (q(b) << 16)) >>> 0; };
/** every level whole, each half the one before; all rows counted from the shape's own first, so that it may lie anywhere, or in a file */
export function pack(sh0: Shape, least = 12): Packed {
  const sh = { ...sh0 }, by = sh.coarse ?? [], own: { tri: Uint32Array; error: number }[] = [], stop = by.length ? by[0]!.error : Infinity, fineTri = sh.mesh.tri; if (by.length) { const cat = (get: (x: { mesh: Mesh; s?: Float32Array }) => ArrayLike<number>) => [{ mesh: sh0.mesh, s: sh0.s }, ...by].flatMap(x => Array.from(get(x as never))); let off = sh0.mesh.pos.length / 3; for (const b of by) { own.push({ tri: b.mesh.tri.map(v => v + off), error: b.error }); off += b.mesh.pos.length / 3; } sh.s = new Float32Array(cat(x => x.s!)); sh.mesh = { pos: new Float32Array(cat(x => x.mesh.pos)), nrm: new Float32Array(cat(x => x.mesh.nrm)), col: new Uint8Array(cat(x => x.mesh.col)), tri: fineTri }; }
  const m = sh.mesh, nv = m.pos.length / 3, pool = new Pool(nv + 64); pool.n = nv; pool.pos.set(m.pos); pool.nrm.set(m.nrm); for (let i = 0; i < nv * 4; i++) pool.col[i] = (m.col[i]! / 255) ** 2;
  const simp = new Simplifier(true), levels: { tri: Uint32Array; error: number }[] = [{ tri: m.tri, error: 0 }]; for (;;) { const l = levels[levels.length - 1]!, n = l.tri.length / 3; if (n <= least || levels.length > 14) break; const r = simp.run(l.tri, pool, () => false, Math.max(least, n >> 1)); if (r.tri.length / 3 > n * 0.86) break; const error = Math.hypot(l.error, r.error) + 1e-6; if (error >= stop * 0.9) break; levels.push({ tri: r.tri, error }); } levels.push(...own);
  // where vertices are from what they go with
  const o = new Float32Array(nv * 3); for (let i = 0; i < nv * 3; i++) o[i] = sh.kind === FITS ? m.pos[i]! - sh.s![i]! * sh.half![i % 3]! : m.pos[i]!; const mn: V3 = [1e9, 1e9, 1e9], mx: V3 = [-1e9, -1e9, -1e9]; for (let i = 0; i < o.length; i++) { mn[i % 3] = Math.min(mn[i % 3]!, o[i]!); mx[i % 3] = Math.max(mx[i % 3]!, o[i]!); } const st = mn.map((v, k) => Math.max(1e-9, (mx[k]! - v) / 65535)) as V3;
  const rows: number[] = [], F = new Float32Array(1), U = new Uint32Array(F.buffer), f = (x: number) => { F[0] = x; return U[0]!; }, put = (...w: number[]) => { const r = rows.length / 4; for (let i = 0; i < w.length; i++) rows.push(w[i]! >>> 0); while (rows.length % 4) rows.push(0); return r; };
  const cut = levels.map(l => { const n = l.tri.length / 3, c = new Float32Array(n * 3); for (let t = 0; t < n; t++) for (let k = 0; k < 3; k++) c[t * 3 + k] = (m.pos[l.tri[t * 3]! * 3 + k]! + m.pos[l.tri[t * 3 + 1]! * 3 + k]! + m.pos[l.tri[t * 3 + 2]! * 3 + k]!) / 3; return parts(c, n, Math.ceil(n / CLUSTER), 0.3); }), many = cut.reduce((a, c) => a + c.length, 0);
  let big = 0; for (let i = 0; i < m.pos.length; i += 3) big = Math.max(big, Math.hypot(m.pos[i]!, m.pos[i + 1]!, m.pos[i + 2]!)); const bones = sh.rest ? sh.rest.length / 12 : 0;
  put(levels.length, sh.kind | (sh.round ? 16 : 0), 4 + levels.length, bones); put(f(mn[0]), f(mn[1]), f(mn[2]), f(sh.reach ?? 0)); put(f(st[0]), f(st[1]), f(st[2]), f(sh.half ? Math.max(...sh.half) : 1)); put(f(big), many, 4 + levels.length + many, 0);
  let at = 0; levels.forEach((l, i) => { put(f(l.error), at, cut[i]!.length, l.tri.length / 3); at += cut[i]!.length; }); const clusterAt = rows.length; for (let i = 0; i < many * 4; i++) rows.push(0); if (sh.rest) { for (const x of sh.rest) rows.push(f(x)); while (rows.length % 4) rows.push(0); }
  let ci = 0; levels.forEach((l, li) => cut[li]!.forEach(ids => {
    const n = ids.length, tri = new Uint32Array(n * 3); ids.forEach((t, i) => { for (let k = 0; k < 3; k++) tri[i * 3 + k] = l.tri[t * 3 + k]!; }); const local = new Map<number, number>(), list: number[] = [], taken = new Set<number>(), first: number[] = [], uses = new Map<number, number>(); for (const v of tri) uses.set(v, (uses.get(v) ?? 0) + 1);
    for (let t = 0; t < n; t++) { let best = -1, least = 1e9; for (let k = 0; k < 3; k++) { const v = tri[t * 3 + k]!; if (!taken.has(v) && uses.get(v)! < least) { least = uses.get(v)!; best = k; } } for (let k = 0; k < 3; k++) uses.set(tri[t * 3 + k]!, uses.get(tri[t * 3 + k]!)! - 1); if (best < 0) best = 0; else { taken.add(tri[t * 3 + best]!); local.set(tri[t * 3 + best]!, t); } first.push(best); list.push(tri[t * 3 + best]!); }
    for (const v of tri) if (!local.has(v)) { local.set(v, list.length); list.push(v); } const vr = rows.length / 4, q8 = (x: number) => Math.max(0, Math.min(255, Math.round((x / 1.25 * 0.5 + 0.5) * 254)));
    for (const v of list) { const p = [0, 1, 2].map(k => Math.round((o[v * 3 + k]! - mn[k]!) / st[k]!)), L = Math.hypot(m.nrm[v * 3]!, m.nrm[v * 3 + 1]!, m.nrm[v * 3 + 2]!) || 1, c = [0, 1, 2].map(k => m.col[v * 4 + k]!); let hi = 0, w = (c[0]! | (c[1]! << 8) | (c[2]! << 16)) >>> 0; if (sh.kind === FITS) { hi = q8(sh.s![v * 3]!) | (q8(sh.s![v * 3 + 1]!) << 8); w = ((c[0]! >> 3) | ((c[1]! >> 2) << 5) | ((c[2]! >> 3) << 11) | (q8(sh.s![v * 3 + 2]!) << 16)) >>> 0; } else if (sh.kind === SKINNED) { hi = sh.bones![v * 2]! | (sh.bones![v * 2 + 1]! << 7); w = (w | (Math.round(sh.weight![v]! * 255) << 24)) >>> 0; } rows.push((p[0]! | (p[1]! << 16)) >>> 0, (p[2]! | (hi << 16)) >>> 0, oct(m.nrm[v * 3]! / L, m.nrm[v * 3 + 1]! / L, m.nrm[v * 3 + 2]! / L), w); }
    const tr = rows.length / 4; for (let t = 0; t < n; t++) rows.push((t | (local.get(tri[t * 3 + (first[t]! + 1) % 3]!)! << 8) | (local.get(tri[t * 3 + (first[t]! + 2) % 3]!)! << 16)) >>> 0); while (rows.length % 4) rows.push(0);
    const c = clusterAt + ci++ * 4; rows[c] = vr; rows[c + 1] = tr; rows[c + 2] = n; rows[c + 3] = list.length; if (list.length > 256) throw new Error(sh.name + ': a cluster with ' + list.length + ' vertices');
  }));
  return { name: sh.name, data: new Uint32Array(rows), levels: levels.map(l => ({ triangles: l.tri.length / 3, error: l.error })), kind: sh.kind };
}
void split4; void lighten;
