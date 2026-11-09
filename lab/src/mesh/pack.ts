// MESHES AS THE SHADERS READ THEM: rows of four words, to be laid somewhere in one buffer (`base` = the row where they start). What each row holds is in wgsl/mesh.wgsl.
import { type Cluster, type Dag, CLUSTER, cutAt, errorFor, parts, around, type Ball } from './dag';
import type { V3, Rot } from '../math';

/** a level a ray may meet: the most it stands off the finest (metres at scale 1), how many triangles, and how much its normals no longer say (mean of 1 − cos) */
export interface Level { error: number; triangles: number; spread: number; root: number }
export interface Packed { header: number; levels: Level[]; clusters: number; triangles: number; ball: Ball; mn: V3; mx: V3; depth: number }
/** one mesh somewhere in an object: child = scale⁻¹ · rotᵀ · (object − at) */
export interface Child { mesh: Packed; at: V3; rot: Rot; scale: number; tint: V3 }
/** an object: levels fine to coarse, each a list of children. Most have one level of one child; a floor's are its pebbles, and their relief as one mesh */
export interface ObjectLevel { error: number; children: Child[] }
export interface PackedObject { row: number; levels: ObjectLevel[]; mn: V3; mx: V3; most: number }

/** rows of an entry; how wide the picking tree is; how many levels rays get; triangles in a leaf of their trees */
export const ENTRY_ROWS = 5, WIDE = 8, RATIOS = 11, LEAF = 8;

const oct = (x: number, y: number, z: number): number => { const s = Math.abs(x) + Math.abs(y) + Math.abs(z) || 1; let a = x / s, b = z / s; if (y < 0) { const p = (1 - Math.abs(b)) * (a < 0 ? -1 : 1), q = (1 - Math.abs(a)) * (b < 0 ? -1 : 1); a = p; b = q; } const q = (v: number) => Math.round((v * 0.5 + 0.5) * 65535) & 0xffff; return (q(a) | (q(b) << 16)) >>> 0; };

export class Store {
  f = new Float32Array(1 << 16); u = new Uint32Array(this.f.buffer); rows = 0;
  /** by kind, rows */ readonly weigh: Record<string, number> = {};
  constructor(readonly base: number) {}
  /** room for n rows; returns the first, counted from the store's own start */
  take(n: number, kind: string): number { while ((this.rows + n) * 4 > this.f.length) { const f = new Float32Array(this.f.length * 2); new Uint32Array(f.buffer).set(this.u); this.f = f; this.u = new Uint32Array(f.buffer); } const at = this.rows; this.rows += n; this.weigh[kind] = (this.weigh[kind] ?? 0) + n; return at; }
  get data(): Float32Array<ArrayBuffer> { return this.f.slice(0, this.rows * 4) as Float32Array<ArrayBuffer>; }
}

// ---------------------------------------------------------------- trees for rays: binary; a node's two children lie side by side
type Box = Float64Array;                                             // mn, mx
const area = (b: ArrayLike<number>, o = 0) => { const x = b[o + 3]! - b[o]!, y = b[o + 4]! - b[o + 1]!, z = b[o + 5]! - b[o + 2]!; return x < 0 ? 0 : x * y + y * z + z * x; };
interface TreeNode { box: Box; a?: TreeNode; b?: TreeNode; first: number; count: number }
/** a tree over boxes (6 numbers each), `leaf` at most in a leaf; `order` comes out as the leaves have them */
export function tree(boxes: Float64Array, n: number, leaf: number): { root: TreeNode; order: Uint32Array; nodes: number } {
  const order = new Uint32Array(n), BINS = 10; for (let i = 0; i < n; i++) order[i] = i; let nodes = 0;
  const binN = new Int32Array(BINS), bin = new Float64Array(BINS * 6), right = new Float64Array(BINS);
  const make = (lo: number, hi: number): TreeNode => {
    nodes++; const box: Box = new Float64Array([1e30, 1e30, 1e30, -1e30, -1e30, -1e30]), c = [1e30, 1e30, 1e30, -1e30, -1e30, -1e30];
    for (let i = lo; i < hi; i++) { const o = order[i]! * 6; for (let k = 0; k < 3; k++) { const a = boxes[o + k]!, b = boxes[o + 3 + k]!, m = a + b; if (a < box[k]!) box[k] = a; if (b > box[3 + k]!) box[3 + k] = b; if (m < c[k]!) c[k] = m; if (m > c[3 + k]!) c[3 + k] = m; } }
    if (hi - lo <= leaf) return { box, first: lo, count: hi - lo };
    let best = Infinity, axis = -1, cut = 0;
    for (let k = 0; k < 3; k++) {
      const w = c[3 + k]! - c[k]!; if (w < 1e-12) continue; const s = BINS / w * 0.9999; binN.fill(0); for (let b = 0; b < BINS; b++) { bin.fill(1e30, b * 6, b * 6 + 3); bin.fill(-1e30, b * 6 + 3, b * 6 + 6); }
      for (let i = lo; i < hi; i++) { const o = order[i]! * 6, b = ((boxes[o + k]! + boxes[o + 3 + k]! - c[k]!) * s) | 0, q = b * 6; binN[b]!++; for (let j = 0; j < 3; j++) { if (boxes[o + j]! < bin[q + j]!) bin[q + j] = boxes[o + j]!; if (boxes[o + 3 + j]! > bin[q + 3 + j]!) bin[q + 3 + j] = boxes[o + 3 + j]!; } }
      const acc = [1e30, 1e30, 1e30, -1e30, -1e30, -1e30]; let m = 0; for (let b = BINS - 1; b > 0; b--) { const q = b * 6; m += binN[b]!; for (let j = 0; j < 3; j++) { acc[j] = Math.min(acc[j]!, bin[q + j]!); acc[3 + j] = Math.max(acc[3 + j]!, bin[q + 3 + j]!); } right[b] = m ? m * area(acc) : 0; }
      acc.fill(1e30, 0, 3); acc.fill(-1e30, 3, 6); m = 0; for (let b = 0; b < BINS - 1; b++) { const q = b * 6; m += binN[b]!; for (let j = 0; j < 3; j++) { acc[j] = Math.min(acc[j]!, bin[q + j]!); acc[3 + j] = Math.max(acc[3 + j]!, bin[q + 3 + j]!); } if (!m || m === hi - lo) continue; const cost = m * area(acc) + right[b + 1]!; if (cost < best) { best = cost; axis = k; cut = b; } }
    }
    let mid = (lo + hi) >> 1;
    if (axis >= 0) { const s = BINS / (c[3 + axis]! - c[axis]!) * 0.9999, at = (i: number) => { const o = order[i]! * 6; return ((boxes[o + axis]! + boxes[o + 3 + axis]! - c[axis]!) * s) | 0; }; let i = lo, j = hi - 1; while (i <= j) { if (at(i) <= cut) i++; else { const t = order[i]!; order[i] = order[j]!; order[j] = t; j--; } } if (i > lo && i < hi) mid = i; }
    return { box, first: 0, count: 0, a: make(lo, mid), b: make(mid, hi) };
  };
  return { root: make(0, n), order, nodes };
}
/** lays a tree down; what a leaf says is `leafAt(first, count)`. Returns the root's row (from the store's start) */
function lay(s: Store, root: TreeNode, kind: string, leafAt: (first: number, count: number) => [number, number], at = -1): number {
  if (at < 0) at = s.take(2, kind); s.f.set(root.box.subarray(0, 3), at * 4); s.f.set(root.box.subarray(3, 6), at * 4 + 4);
  if (root.a && root.b) { const c = s.take(4, kind); lay(s, root.a, kind, leafAt, c); lay(s, root.b, kind, leafAt, c + 2); s.u[at * 4 + 3] = s.base + c; s.u[at * 4 + 7] = 0; }
  else { const [a, b] = leafAt(root.first, root.count); s.u[at * 4 + 3] = a; s.u[at * 4 + 7] = b; }
  return at;
}
// (a leaf's second word: 1..255 = so many triangles (or children) from the first word on; 0 = an inner node, its children at the first word and two rows on)

/** a whole mesh: vertices, triangles, what the raster picks clusters by, and a tree for each level a ray may ask for */
export function packMesh(s: Store, d: Dag): Packed {
  const P = d.pool, mn: V3 = [1e30, 1e30, 1e30], mx: V3 = [-1e30, -1e30, -1e30];
  for (const c of d.clusters) for (const v of c.tri) for (let k = 0; k < 3; k++) { const p = P.pos[v * 3 + k]!; if (p < mn[k]!) mn[k] = p; if (p > mx[k]!) mx[k] = p; }
  const st = mn.map((v, k) => Math.max(1e-9, (mx[k]! - v) / 65535)) as V3, header = s.take(4 + RATIOS, 'headers'), q = (v: number, k: number) => Math.round((P.pos[v * 3 + k]! - mn[k]!) / st[k]!), at = (v: number, k: number) => mn[k]! + q(v, k) * st[k]!;
  // ---- each cluster: its triangles in the order of a small tree over them, its vertices once each (triangle i begins with a vertex that begins no other, so that one number tells a triangle)
  const n = d.clusters.length, rec = s.take(n, 'clusters'), idx = s.take(n * CLUSTER / 4, 'triangles'), sub: number[] = [], cone: Float64Array[] = [];
  const local = new Int32Array(P.n).fill(-1), vx = new Float64Array(256), vy = new Float64Array(256), vz = new Float64Array(256), uses = new Int32Array(256), begins = new Int32Array(256), of = new Int32Array(256), L = new Int32Array(CLUSTER * 3), boxes = new Float64Array(CLUSTER * 6), fn = new Float64Array(CLUSTER * 3);
  d.clusters.forEach((c, ci) => {
    const m = c.tri.length / 3; let nv = 0;
    for (let i = 0; i < m * 3; i++) { const g = c.tri[i]!; let v = local[g]!; if (v < 0) { v = local[g] = nv++; of[v] = g; uses[v] = 0; begins[v] = -1; vx[v] = at(g, 0); vy[v] = at(g, 1); vz[v] = at(g, 2); } L[i] = v; uses[v]!++; }
    for (let t = 0; t < m; t++) { const i = L[t * 3]!, j = L[t * 3 + 1]!, k = L[t * 3 + 2]!, o = t * 6; boxes[o] = Math.min(vx[i]!, vx[j]!, vx[k]!) - st[0] * 0.01; boxes[o + 1] = Math.min(vy[i]!, vy[j]!, vy[k]!) - st[1] * 0.01; boxes[o + 2] = Math.min(vz[i]!, vz[j]!, vz[k]!) - st[2] * 0.01; boxes[o + 3] = Math.max(vx[i]!, vx[j]!, vx[k]!) + st[0] * 0.01; boxes[o + 4] = Math.max(vy[i]!, vy[j]!, vy[k]!) + st[1] * 0.01; boxes[o + 5] = Math.max(vz[i]!, vz[j]!, vz[k]!) + st[2] * 0.01; }
    const tr = tree(boxes, m, LEAF); sub.push(lay(s, tr.root, 'ray trees', (f, k) => [ci * CLUSTER + f, k]));
    // who begins which triangle: greedily, the vertex with the fewest triangles left. Those that begin one come first, in the triangles' order; a triangle left without gets a copy
    const list: number[] = [], first = new Uint8Array(m), slot = new Int32Array(nv).fill(-1);
    for (let t = 0; t < m; t++) { const q = tr.order[t]! * 3; let best = -1, least = 1e9; for (let k = 0; k < 3; k++) { const v = L[q + k]!; if (begins[v]! < 0 && uses[v]! < least) { least = uses[v]!; best = k; } } for (let k = 0; k < 3; k++) uses[L[q + k]!]!--; if (best >= 0) { begins[L[q + best]!] = t; slot[L[q + best]!] = t; } else best = 0; first[t] = best; list.push(L[q + best]!); }
    for (let v = 0; v < nv; v++) if (slot[v]! < 0) { slot[v] = list.length; list.push(v); }
    if (list.length > 256) throw new Error('a cluster with ' + list.length + ' vertices');
    const vb = s.take(list.length, 'vertices'); list.forEach((lv, i) => { const v = of[lv]!, o = (vb + i) * 4, N = P.nrm, l = Math.hypot(N[v * 3]!, N[v * 3 + 1]!, N[v * 3 + 2]!) || 1, b = (k: number) => Math.max(0, Math.min(255, Math.round(P.col[v * 4 + k]!))); s.u[o] = (q(v, 0) | (q(v, 1) << 16)) >>> 0; s.u[o + 1] = (q(v, 2) | (Math.min(65535, Math.round(P.spread[v]! * 65535)) << 16)) >>> 0; s.u[o + 2] = oct(N[v * 3]! / l, N[v * 3 + 1]! / l, N[v * 3 + 2]! / l); s.u[o + 3] = (b(0) | (b(1) << 8) | (b(2) << 16) | (b(3) << 24)) >>> 0; });
    for (let t = 0; t < m; t++) { const o = tr.order[t]! * 3, f = first[t]!; s.u[idx * 4 + ci * CLUSTER + t] = (t | (slot[L[o + (f + 1) % 3]!]! << 8) | (slot[L[o + (f + 2) % 3]!]! << 16)) >>> 0; }
    s.u[(rec + ci) * 4] = s.base + vb; s.u[(rec + ci) * 4 + 1] = m; s.u[(rec + ci) * 4 + 2] = list.length; s.u[(rec + ci) * 4 + 3] = c.level;
    // the cone its faces look into: from behind all of them none is seen
    let ax = 0, ay = 0, az = 0, fc = 0; for (let t = 0; t < m; t++) { const i = L[t * 3]!, j = L[t * 3 + 1]!, k = L[t * 3 + 2]!, ux = vx[j]! - vx[i]!, uy = vy[j]! - vy[i]!, uz = vz[j]! - vz[i]!, wx = vx[k]! - vx[i]!, wy = vy[k]! - vy[i]!, wz = vz[k]! - vz[i]!, x = uy * wz - uz * wy, y = uz * wx - ux * wz, z = ux * wy - uy * wx, l = Math.sqrt(x * x + y * y + z * z); ax += x; ay += y; az += z; if (l > 1e-20) { fn[fc++] = x / l; fn[fc++] = y / l; fn[fc++] = z / l; } }
    const l = Math.hypot(ax, ay, az) || 1; ax /= l; ay /= l; az /= l; let least = 1; for (let i = 0; i < fc; i += 3) least = Math.min(least, fn[i]! * ax + fn[i + 1]! * ay + fn[i + 2]! * az); cone.push(new Float64Array([ax, ay, az, least <= 0.05 ? 2 : Math.sqrt(1 - least * least)]));
    for (let v = 0; v < nv; v++) local[of[v]!] = -1;
  });
  // ---- what the raster picks by: a tree eight wide over every level's clusters, the levels side by side on top
  let deepest = 0; interface Wide { bound: Ball; lod: Ball; parent: number; cluster: number; kids: Wide[] }
  const leafOf = (c: Cluster, i: number): Wide => ({ bound: c.bound, lod: c.parentBall, parent: c.parentError, cluster: i, kids: [] });
  const up = (kids: Wide[]): Wide => ({ bound: around(kids.map(k => k.bound)), lod: around(kids.map(k => k.lod)), parent: Math.max(...kids.map(k => k.parent)), cluster: -1, kids });
  const grow = (ws: Wide[], turn: number): Wide[] => { while (ws.length > WIDE) { const c = new Float32Array(ws.length * 3); ws.forEach((w, i) => { c[i * 3] = w.bound.x; c[i * 3 + 1] = w.bound.y; c[i * 3 + 2] = w.bound.z; }); ws = parts(c, ws.length, Math.ceil(ws.length / WIDE), turn).map(ids => up([...ids].map(i => ws[i]!))); turn += 0.7; } return ws; };
  let tops: Wide[] = []; for (let l = 0; l <= d.levels; l++) { const ws: Wide[] = []; d.clusters.forEach((c, i) => { if (c.level === l) ws.push(leafOf(c, i)); }); if (ws.length) tops.push(...(ws.length > 1 ? [up(grow(ws, l))] : ws)); }
  while (tops.length > WIDE) { const next: Wide[] = []; for (let i = 0; i < tops.length; i += WIDE) { const k = tops.slice(i, i + WIDE); next.push(k.length > 1 ? up(k) : k[0]!); } tops = next; }
  const put = (kids: Wide[], depth: number): number => {
    deepest = Math.max(deepest, depth); const r = s.take(kids.length * ENTRY_ROWS, 'picking');
    kids.forEach((w, i) => { const o = (r + i * ENTRY_ROWS) * 4, B = w.bound, L = w.lod; s.f.set([B.x, B.y, B.z, B.r, L.x, L.y, L.z, L.r], o); s.f[o + 8] = Math.min(w.parent, 1e30);
      if (w.cluster >= 0) { const c = d.clusters[w.cluster]!; s.f[o + 9] = c.error; s.u[o + 10] = w.cluster; s.u[o + 11] = 0; s.f.set([c.ball.x, c.ball.y, c.ball.z, c.ball.r], o + 12); s.f.set(cone[w.cluster]!, o + 16); }
      else { const at = put(w.kids, depth + 1); s.f[o + 9] = 0; s.u[o + 10] = s.base + at; s.u[o + 11] = w.kids.length; } });
    return r;
  };
  const top = put(tops, 1);
  // ---- levels for rays: cuts at a half, a quarter … of the triangles, each a tree over its clusters, whose own small trees are shared
  const index = new Map(d.clusters.map((c, i) => [c, i])), levels: Level[] = [];
  for (let k = 0; k < RATIOS; k++) {
    const e = k === 0 ? 0 : errorFor(d, d.triangles / 2 ** k); if (k > 0 && e <= levels[levels.length - 1]!.error) continue; const cs = cutAt(d, e), tris = cs.reduce((a, c) => a + c.tri.length / 3, 0); if (k > 0 && tris > levels[levels.length - 1]!.triangles * 0.8) continue;
    const boxes = new Float64Array(cs.length * 6); cs.forEach((c, i) => { const r = sub[index.get(c)!]! * 4; boxes.set(s.f.subarray(r, r + 3), i * 6); boxes.set(s.f.subarray(r + 4, r + 7), i * 6 + 3); });
    const tr = tree(boxes, cs.length, 1), root = lay(s, tr.root, 'ray trees', () => [0, 0]);
    // (a leaf of the upper tree is a copy of its cluster's own root)
    const fill = (node: TreeNode, row: number) => { if (node.a && node.b) { const c = s.u[row * 4 + 3]! - s.base; fill(node.a, c); fill(node.b, c + 2); } else { const from = sub[index.get(cs[tr.order[node.first]!]!)!]! * 4; for (let w = 0; w < 8; w++) s.u[row * 4 + w] = s.u[from + w]!; } }; fill(tr.root, root);
    let sp = 0, sn = 0; for (const c of cs) for (const v of c.tri) { sp += P.spread[v]!; sn++; }
    levels.push({ error: e, triangles: tris, spread: sp / sn, root: s.base + root });
  }
  const h = header * 4, whole = around(d.clusters.filter(c => c.level === 0).map(c => c.bound)); s.f.set(mn, h); s.u[h + 3] = levels.length; s.f.set(st, h + 4); s.u[h + 7] = s.base + rec; s.u[h + 8] = s.base + idx; s.u[h + 9] = s.base + top; s.u[h + 10] = tops.length; s.u[h + 11] = n; s.f.set([whole.x, whole.y, whole.z, whole.r], h + 12);
  levels.forEach((l, i) => { s.f[h + 16 + i * 4] = l.error; s.u[h + 17 + i * 4] = l.root; s.u[h + 18 + i * 4] = l.triangles; s.f[h + 19 + i * 4] = l.spread; });
  return { header: s.base + header, levels, clusters: n, triangles: d.triangles, ball: whole, mn, mx, depth: deepest };
}
/** an object: its children's rows, a tree over each level's, and its header */
export function packObject(s: Store, levels: ObjectLevel[], /** the width from which a ray meets cells in its place (0: never) */ gone = 0): PackedObject {
  const header = s.take(3 + levels.length, 'objects'), mn: V3 = [1e30, 1e30, 1e30], mx: V3 = [-1e30, -1e30, -1e30]; let bits = 7, most = 0, start = 0; const total = levels.reduce((a, l) => a + l.children.length, 0), kids = s.take(total * 4, 'children');
  for (const l of levels) for (const c of l.children) while ((1 << (bits - 7)) < c.mesh.clusters) bits++;
  levels.forEach((l, li) => {
    const n = l.children.length, boxes = new Float64Array(n * 6);
    l.children.forEach((c, i) => {
      const o = (kids + (start + i) * 4) * 4, k = 1 / c.scale; for (let a = 0; a < 3; a++) { const x = c.rot[a]!; s.f[o + a * 4] = x[0] * k; s.f[o + a * 4 + 1] = x[1] * k; s.f[o + a * 4 + 2] = x[2] * k; s.f[o + a * 4 + 3] = -(x[0] * c.at[0] + x[1] * c.at[1] + x[2] * c.at[2]) * k; }
      const q = (x: number) => Math.max(0, Math.min(255, Math.round(x * 255))); s.u[o + 12] = c.mesh.header; s.u[o + 13] = (q(c.tint[0]) | (q(c.tint[1]) << 8) | (q(c.tint[2]) << 16)) >>> 0; s.f[o + 14] = c.scale; s.u[o + 15] = 0;
      for (let a = 0; a < 3; a++) { let lo = c.at[a]!, hi = c.at[a]!; for (let b = 0; b < 3; b++) { const m = c.rot[b]![a]! * c.scale, p = m * c.mesh.mn[b]!, r = m * c.mesh.mx[b]!; lo += Math.min(p, r); hi += Math.max(p, r); } boxes[i * 6 + a] = lo; boxes[i * 6 + 3 + a] = hi; if (lo < mn[a]!) mn[a] = lo; if (hi > mx[a]!) mx[a] = hi; }
    });
    // (leaves name children by where they lie: laid again in the tree's order)
    const tr = tree(boxes, n, 2), was = s.u.slice((kids + start * 4) * 4, (kids + (start + n) * 4) * 4); tr.order.forEach((src, i) => s.u.set(was.subarray(src * 16, src * 16 + 16), (kids + (start + i) * 4) * 4)); const first = start, root = lay(s, tr.root, 'ray trees', (f, k) => [first + f, k]);
    const h = (header + 3 + li) * 4; s.f[h] = l.error; s.u[h + 1] = s.base + root; s.u[h + 2] = first; s.u[h + 3] = n; start += n; most = Math.max(most, n);
  });
  const h = header * 4; s.u[h] = levels.length; s.u[h + 1] = bits; s.u[h + 2] = s.base + kids; s.u[h + 3] = total; s.f[h + 8] = gone; s.f.set([(mn[0] + mx[0]) / 2, (mn[1] + mx[1]) / 2, (mn[2] + mx[2]) / 2, Math.hypot(mx[0] - mn[0], mx[1] - mn[1], mx[2] - mn[2]) / 2], h + 4);
  if (total > 1 << (31 - bits)) throw new Error('too many children'); return { row: s.base + header, levels, mn, mx, most };
}
