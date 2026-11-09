// LEVELS OF A MESH. It is cut into clusters of at most 128 triangles. A few clusters that lie together are one group: the group's inside is simplified to half, its rim held, and
// what is left is cut into clusters again, until little is left. So every cluster has an error of its own (metres; 0 at the bottom) and that of the group which takes its place.
// For ANY error e, the clusters with error <= e < parent's error cover the whole surface once, without a crack: a cut. The raster takes one by the pixel, a ray by its footprint.
import { Pool, Simplifier } from './simplify';
import type { Mesh } from './mesh';

export const CLUSTER = 128, VERTS = 256, GROUP = 12;
export type Ball = { x: number; y: number; z: number; r: number };
export interface Cluster {
  /** vertex ids of the pool, three a triangle */ tri: Uint32Array;
  /** its own error and the ball that is judged by; the same of the group it was simplified in (Infinity: nothing is coarser) */ error: number; ball: Ball; parentError: number; parentBall: Ball;
  /** what it fills */ bound: Ball; level: number;
}
export interface Dag { pool: Pool; clusters: Cluster[]; levels: number; /** of the finest */ triangles: number }

/** items → parts that lie together, of about equal numbers: halved along the longest way, again and again. `turn` turns the axes, so that cuts of one level are not those of the next */
export function parts(center: Float32Array, n: number, many: number, turn: number): Uint32Array[] {
  const a = turn * 1.9, b = turn * 2.7 + 0.4, ca = Math.cos(a), sa = Math.sin(a), cb = Math.cos(b), sb = Math.sin(b), c = new Float32Array(n * 3), idx = new Uint32Array(n);
  for (let i = 0; i < n; i++) { const x = center[i * 3]!, y = center[i * 3 + 1]!, z = center[i * 3 + 2]!, x1 = ca * x - sa * z, z1 = sa * x + ca * z; c[i * 3] = cb * x1 - sb * y; c[i * 3 + 1] = sb * x1 + cb * y; c[i * 3 + 2] = z1; idx[i] = i; }
  const out: Uint32Array[] = [], nth = (lo: number, hi: number, k: number, ax: number) => {      // quickselect
    let l = lo, r = hi - 1; while (l < r) { const p = c[idx[(l + r) >> 1]! * 3 + ax]!; let i = l, j = r; while (i <= j) { while (c[idx[i]! * 3 + ax]! < p) i++; while (c[idx[j]! * 3 + ax]! > p) j--; if (i <= j) { const t = idx[i]!; idx[i] = idx[j]!; idx[j] = t; i++; j--; } } if (k <= j) r = j; else if (k >= i) l = i; else return; }
  };
  const go = (lo: number, hi: number, m: number) => {
    if (m <= 1) { out.push(idx.slice(lo, hi)); return; }
    let best = 0, wide = -1; for (let k = 0; k < 3; k++) { let mn = Infinity, mx = -Infinity; for (let i = lo; i < hi; i++) { const v = c[idx[i]! * 3 + k]!; if (v < mn) mn = v; if (v > mx) mx = v; } if (mx - mn > wide) { wide = mx - mn; best = k; } }
    const left = m >> 1, at = lo + Math.round((hi - lo) * left / m); nth(lo, hi, at, best); go(lo, at, left); go(at, hi, m - left);
  };
  if (n > 0) go(0, n, Math.max(1, Math.min(many, n))); return out;
}
function ballOf(pool: Pool, tri: Uint32Array): Ball {
  let x0 = Infinity, y0 = Infinity, z0 = Infinity, x1 = -Infinity, y1 = -Infinity, z1 = -Infinity; const P = pool.pos;
  for (const v of tri) { const x = P[v * 3]!, y = P[v * 3 + 1]!, z = P[v * 3 + 2]!; if (x < x0) x0 = x; if (y < y0) y0 = y; if (z < z0) z0 = z; if (x > x1) x1 = x; if (y > y1) y1 = y; if (z > z1) z1 = z; }
  const x = (x0 + x1) / 2, y = (y0 + y1) / 2, z = (z0 + z1) / 2; let r = 0; for (const v of tri) r = Math.max(r, (P[v * 3]! - x) ** 2 + (P[v * 3 + 1]! - y) ** 2 + (P[v * 3 + 2]! - z) ** 2); return { x, y, z, r: Math.sqrt(r) };
}
/** a ball round balls */
export function around(bs: Ball[]): Ball {
  let x0 = Infinity, y0 = Infinity, z0 = Infinity, x1 = -Infinity, y1 = -Infinity, z1 = -Infinity;
  for (const b of bs) { x0 = Math.min(x0, b.x - b.r); y0 = Math.min(y0, b.y - b.r); z0 = Math.min(z0, b.z - b.r); x1 = Math.max(x1, b.x + b.r); y1 = Math.max(y1, b.y + b.r); z1 = Math.max(z1, b.z + b.r); }
  const x = (x0 + x1) / 2, y = (y0 + y1) / 2, z = (z0 + z1) / 2; let r = 0; for (const b of bs) r = Math.max(r, Math.hypot(b.x - x, b.y - y, b.z - z) + b.r); return { x, y, z, r: r * 1.0001 };
}
function cutUp(pool: Pool, tri: Uint32Array, turn: number): Uint32Array[] {
  const n = tri.length / 3, P = pool.pos, c = new Float32Array(n * 3);
  for (let t = 0; t < n; t++) for (let k = 0; k < 3; k++) c[t * 3 + k] = (P[tri[t * 3]! * 3 + k]! + P[tri[t * 3 + 1]! * 3 + k]! + P[tri[t * 3 + 2]! * 3 + k]!) / 3;
  const take = (ids: Uint32Array) => { const out = new Uint32Array(ids.length * 3); ids.forEach((t, i) => { out[i * 3] = tri[t * 3]!; out[i * 3 + 1] = tri[t * 3 + 1]!; out[i * 3 + 2] = tri[t * 3 + 2]!; }); return out; };
  const done: Uint32Array[] = []; for (const ids of parts(c, n, Math.ceil(n / CLUSTER), turn)) { const t = take(ids); if (new Set(t).size > VERTS) { const h = t.length / 6 | 0; done.push(t.slice(0, h * 3), t.slice(h * 3)); } else done.push(t); }
  return done;
}
/** `least`: where to stop, in triangles */
export function buildDag(m: Mesh, least = 24): Dag {
  const nv = m.pos.length / 3, pool = new Pool(nv * 2 + 64); pool.n = nv; pool.pos.set(m.pos); pool.nrm.set(m.nrm); for (let i = 0; i < nv * 4; i++) pool.col[i] = m.col[i]!;
  const simp = new Simplifier(), clusters: Cluster[] = [], none: Ball = { x: 0, y: 0, z: 0, r: 0 };
  let live: Cluster[] = cutUp(pool, m.tri, 0.31).map(tri => { const b = ballOf(pool, tri); return { tri, error: 0, ball: b, parentError: Infinity, parentBall: none, bound: b, level: 0 }; }); clusters.push(...live);
  let owner = new Int32Array(0), level = 0;
  for (;;) {
    const total = live.reduce((a, c) => a + c.tri.length / 3, 0); if (total <= least || level > 40) break; level++;
    const cen = new Float32Array(live.length * 3); live.forEach((c, i) => { cen[i * 3] = c.bound.x; cen[i * 3 + 1] = c.bound.y; cen[i * 3 + 2] = c.bound.z; });
    const groups = parts(cen, live.length, Math.round(live.length / GROUP), level * 0.77 + 0.1);
    // whose each vertex is: one group's, or several's (−2)
    if (owner.length < pool.n) owner = new Int32Array(pool.n * 2); owner.fill(-1, 0, pool.n); groups.forEach((ids, g) => { for (const i of ids) for (const v of live[i]!.tri) { const o = owner[v]!; if (o === -1) owner[v] = g; else if (o !== g) owner[v] = -2; } });
    const next: Cluster[] = []; let made = 0;
    for (const ids of groups) {
      const of = [...ids].map(i => live[i]!); let n = 0; for (const c of of) n += c.tri.length; const all = new Uint32Array(n); n = 0; for (const c of of) { all.set(c.tri, n); n += c.tri.length; }
      const was = all.length / 3, o = owner, s = simp.run(all, pool, v => o[v] === -2, Math.max(Math.min(least, was >> 1), was >> 1)), now = s.tri.length / 3;
      if (now > was * 0.85) { next.push(...of); continue; }           // it would not go: perhaps in other company, next time
      made++; const worst = Math.max(...of.map(c => c.error)), error = Math.max(worst * 1.0001, Math.hypot(worst, s.error)) + 1e-9, ball = around(of.map(c => c.ball));
      for (const c of of) { c.parentError = error; c.parentBall = ball; }
      for (const tri of cutUp(pool, s.tri, level * 1.3 + 0.2)) { const c: Cluster = { tri, error, ball, parentError: Infinity, parentBall: none, bound: ballOf(pool, tri), level }; next.push(c); clusters.push(c); }
    }
    live = next; if (!made) break;
  }
  return { pool, clusters, levels: level, triangles: m.tri.length / 3 };
}
/** the clusters of the cut at error e */
export const cutAt = (d: Dag, e: number) => d.clusters.filter(c => c.error <= e && c.parentError > e);
/** the error whose cut has at most so many triangles */
export function errorFor(d: Dag, triangles: number): number {
  const es = [...new Set(d.clusters.map(c => c.error))].sort((a, b) => a - b), count = (e: number) => cutAt(d, e).reduce((a, c) => a + c.tri.length / 3, 0);
  let lo = 0, hi = es.length - 1; while (lo < hi) { const k = (lo + hi) >> 1; if (count(es[k]!) <= triangles) hi = k; else lo = k + 1; } return es[lo]!;
}
