// SHAPES WITHOUT ARTISTS: pebbles by cutting and wearing down, a broken column by a lathe, chisels and weather. All of it from a seed; nothing is loaded.
import { rng, type V3, norm, cross } from '../math';
import { type Mesh, normalsOf, split4 } from './mesh';

// ---- noise: value noise in three dimensions, fast
const PERM = (() => { const p = new Uint8Array(512), r = rng(1234), a = Array.from({ length: 256 }, (_, i) => i); for (let i = 255; i > 0; i--) { const j = Math.floor(r() * (i + 1)), t = a[i]!; a[i] = a[j]!; a[j] = t; } for (let i = 0; i < 512; i++) p[i] = a[i & 255]!; return p; })();
/** gradient noise, about −1 to 1 */
export function noise3(x: number, y: number, z: number): number {
  const X = Math.floor(x), Y = Math.floor(y), Z = Math.floor(z), fx = x - X, fy = y - Y, fz = z - Z, u = fx * fx * fx * (fx * (fx * 6 - 15) + 10), v = fy * fy * fy * (fy * (fy * 6 - 15) + 10), w = fz * fz * fz * (fz * (fz * 6 - 15) + 10), i = X & 255, j = Y & 255, k = Z & 255, P = PERM;
  const A = P[i]! + j, B = P[i + 1]! + j, AA = P[A & 511]! + k, AB = P[(A + 1) & 511]! + k, BA = P[B & 511]! + k, BB = P[(B + 1) & 511]! + k;
  const g = (h: number, a: number, b: number, c: number) => { const q = h & 15, s = q < 8 ? a : b, t = q < 4 ? b : q === 12 || q === 14 ? a : c; return ((q & 1) ? -s : s) + ((q & 2) ? -t : t); };
  const x1 = g(P[AA & 511]!, fx, fy, fz), x2 = g(P[BA & 511]!, fx - 1, fy, fz), x3 = g(P[AB & 511]!, fx, fy - 1, fz), x4 = g(P[BB & 511]!, fx - 1, fy - 1, fz), y1 = g(P[(AA + 1) & 511]!, fx, fy, fz - 1), y2 = g(P[(BA + 1) & 511]!, fx - 1, fy, fz - 1), y3 = g(P[(AB + 1) & 511]!, fx, fy - 1, fz - 1), y4 = g(P[(BB + 1) & 511]!, fx - 1, fy - 1, fz - 1);
  const a = x1 + u * (x2 - x1), b = x3 + u * (x4 - x3), c = y1 + u * (y2 - y1), d = y3 + u * (y4 - y3), e = a + v * (b - a), f = c + v * (d - c); return e + w * (f - e);
}
export function fbm3(x: number, y: number, z: number, octaves: number): number { let s = 0, a = 0.5, f = 1; for (let i = 0; i < octaves; i++) { s += a * noise3(x * f, y * f + i * 17.3, z * f - i * 9.1); f *= 2.03; a *= 0.5; } return s; }
const smin = (a: number, b: number, k: number) => { const h = Math.max(k - Math.abs(a - b), 0) / k; return Math.min(a, b) - h * h * k * 0.25; }, smax = (a: number, b: number, k: number) => -smin(-a, -b, k), clamp01 = (x: number) => Math.max(0, Math.min(1, x)), step = (a: number, b: number, x: number) => { const t = clamp01((x - a) / (b - a)); return t * t * (3 - 2 * t); };

/** a ball of 20 · 4^n triangles */
export function ball(n: number): { pos: Float32Array; tri: Uint32Array } {
  const t = (1 + Math.sqrt(5)) / 2, v = [[-1, t, 0], [1, t, 0], [-1, -t, 0], [1, -t, 0], [0, -1, t], [0, 1, t], [0, -1, -t], [0, 1, -t], [t, 0, -1], [t, 0, 1], [-t, 0, -1], [-t, 0, 1]];
  let m = { pos: new Float32Array(v.flat()), tri: new Uint32Array([0, 11, 5, 0, 5, 1, 0, 1, 7, 0, 7, 10, 0, 10, 11, 1, 5, 9, 5, 11, 4, 11, 10, 2, 10, 7, 6, 7, 1, 8, 3, 9, 4, 3, 4, 2, 3, 2, 6, 3, 6, 8, 3, 8, 9, 4, 9, 5, 2, 4, 11, 6, 2, 10, 8, 6, 7, 9, 8, 1]) };
  const round = (p: Float32Array) => { for (let i = 0; i < p.length; i += 3) { const l = Math.hypot(p[i]!, p[i + 1]!, p[i + 2]!); p[i]! /= l; p[i + 1]! /= l; p[i + 2]! /= l; } };
  round(m.pos); for (let i = 0; i < n; i++) { m = split4(m) as typeof m; round(m.pos); } return m;
}

/**
 * A PEBBLE about 1 across (half extents about 0.5): a block cut by planes, worn until its edges are round (a soft minimum over the planes), with a little unevenness, pits, and on some a band of another stone.
 * `worn` 0 = broken stone, 1 = from a river.
 */
export function pebble(seed: number, n: number, worn = 0.8): Mesh {
  const r = rng(seed), dir = (): V3 => { for (;;) { const v: V3 = [r() * 2 - 1, r() * 2 - 1, r() * 2 - 1], l = Math.hypot(...v); if (l > 0.1 && l <= 1) return norm(v); } };
  const squash: V3 = [1, 0.5 + 0.25 * r(), 0.68 + 0.3 * r()], planes: [V3, number][] = []; for (let i = 0; i < 13; i++) planes.push([dir(), 0.66 + 0.40 * r()]); const ox = r() * 50, soft = 0.10 + 0.42 * worn, band = dir(), bandAt = (r() - 0.5) * 0.5, bandWide = 0.02 + 0.05 * r(), banded = r() < 0.35;
  const { pos: d, tri } = ball(n), nv = d.length / 3, pos = new Float32Array(nv * 3), col = new Uint8Array(nv * 4);
  for (let i = 0; i < nv; i++) {
    const x = d[i * 3]!, y = d[i * 3 + 1]!, z = d[i * 3 + 2]!; let k = 1.0;                  // how far out the stone goes this way
    for (const [p, w] of planes) { const c = p[0] * x + p[1] * y + p[2] * z; if (c > 0.05) k = smin(k, w / c, soft); }
    const px = x * k, py = y * k, pz = z * k, f = fbm3(px * 2.2 + ox, py * 2.2, pz * 2.2, 3), g = fbm3(px * 9 + ox, py * 9 + 3, pz * 9, 4), h = fbm3(px * 38, py * 38 + ox, pz * 38, 3), pit = step(0.22, 0.5, noise3(px * 15 + ox, py * 15, pz * 15 + 7)), b = px * band[0] + py * band[1] + pz * band[2] - bandAt + 0.05 * f, inBand = banded ? 1 - step(bandWide * 0.6, bandWide, Math.abs(b)) : 0;
    const s = 0.5 * (k + 0.07 * f + (0.018 + 0.030 * (1 - worn)) * g + 0.006 * h - 0.028 * pit * (1.1 - worn * 0.7) + 0.012 * inBand);
    pos[i * 3] = x * s * squash[0]; pos[i * 3 + 1] = y * s * squash[1]; pos[i * 3 + 2] = z * s * squash[2];
    const speck = step(0.15, 0.4, noise3(px * 26 + 4, py * 26, pz * 26 + ox)), v = 0.80 + 0.22 * f + 0.14 * g + 0.10 * h - 0.18 * pit - 0.16 * speck, q = [v, v * 0.985, v * 0.96]; for (let c = 0; c < 3; c++) col[i * 4 + c] = 255 * clamp01(q[c]! + (0.98 - q[c]!) * inBand * 0.85); col[i * 4 + 3] = 255 * clamp01(1 - 0.5 * pit);
  }
  return { pos, nrm: normalsOf(pos, tri), col, tri };
}

// ---------------------------------------------------------------- the column
/** surface nets: the zero of a signed field over a grid, as triangles. Blocks far from it are passed over (`field` must not grow faster than distance, about) */
export function nets(field: (x: number, y: number, z: number) => number, lo: V3, hi: V3, cell: number): { pos: Float32Array; tri: Uint32Array } {
  const B = 8, n = [0, 1, 2].map(k => Math.ceil((hi[k]! - lo[k]!) / cell / B) * B) as V3, N = n.map(v => v + 1), vert = new Map<number, Int32Array>(), pos: number[] = [], tri: number[] = [], key = (x: number, y: number, z: number) => x + N[0]! * (y + N[1]! * z), val = new Map<number, Float32Array>(), bn = n.map(v => v / B);
  const bkey = (x: number, y: number, z: number) => x + bn[0]! * (y + bn[1]! * z), M = B + 1;
  for (let bz = 0; bz < bn[2]!; bz++) for (let by = 0; by < bn[1]!; by++) for (let bx = 0; bx < bn[0]!; bx++) {
    const h = B * cell / 2, c = field(lo[0] + bx * B * cell + h, lo[1] + by * B * cell + h, lo[2] + bz * B * cell + h); if (Math.abs(c) > h * 1.9 + cell) continue;
    const v = new Float32Array(M * M * M); let i = 0, neg = false, posi = false; for (let z = 0; z < M; z++) for (let y = 0; y < M; y++) for (let x = 0; x < M; x++) { const f = field(lo[0] + (bx * B + x) * cell, lo[1] + (by * B + y) * cell, lo[2] + (bz * B + z) * cell); v[i++] = f; if (f < 0) neg = true; else posi = true; } if (neg && posi) val.set(bkey(bx, by, bz), v);
  }
  const E = [[0, 1], [2, 3], [4, 5], [6, 7], [0, 2], [1, 3], [4, 6], [5, 7], [0, 4], [1, 5], [2, 6], [3, 7]] as const;
  for (const [bk, v] of val) {
    const bx = bk % bn[0]!, by = Math.floor(bk / bn[0]!) % bn[1]!, bz = Math.floor(bk / bn[0]! / bn[1]!);
    for (let z = 0; z < B; z++) for (let y = 0; y < B; y++) for (let x = 0; x < B; x++) {
      const at = (i: number) => v[x + (i & 1) + M * (y + ((i >> 1) & 1) + M * (z + (i >> 2)))]!; let mask = 0; for (let i = 0; i < 8; i++) if (at(i) < 0) mask |= 1 << i; if (mask === 0 || mask === 255) continue;
      // corners inside that do not touch along an edge of the cell are sheets of their own, each with its vertex
      const part = new Int32Array(8).fill(-1), gx = bx * B + x, gy = by * B + y, gz = bz * B + z; for (let i = 0; i < 8; i++) { if (!(mask >> i & 1) || part[i]! >= 0) continue; const id = pos.length / 3, todo = [i]; part[i] = id; while (todo.length) { const c = todo.pop()!; for (const k of [1, 2, 4]) { const q = c ^ k; if (mask >> q & 1 && part[q]! < 0) { part[q] = id; todo.push(q); } } }
        let sx = 0, sy = 0, sz = 0, m = 0; for (const [a, b] of E) { const fa = at(a), fb = at(b); if ((fa < 0) === (fb < 0) || part[fa < 0 ? a : b] !== id) continue; const t = fa / (fa - fb); sx += (a & 1) + t * ((b & 1) - (a & 1)); sy += ((a >> 1) & 1) + t * (((b >> 1) & 1) - ((a >> 1) & 1)); sz += (a >> 2) + t * ((b >> 2) - (a >> 2)); m++; }
        pos.push(lo[0] + (gx + sx / m) * cell, lo[1] + (gy + sy / m) * cell, lo[2] + (gz + sz / m) * cell); }
      vert.set(key(gx, gy, gz), part);
    }
  }
  for (const [bk, v] of val) {
    const bx = bk % bn[0]!, by = Math.floor(bk / bn[0]!) % bn[1]!, bz = Math.floor(bk / bn[0]! / bn[1]!);
    for (let z = 0; z < B; z++) for (let y = 0; y < B; y++) for (let x = 0; x < B; x++) {
      const gx = bx * B + x, gy = by * B + y, gz = bz * B + z, f0 = v[x + M * (y + M * z)]!;
      for (let a = 0; a < 3; a++) {
        const f1 = v[x + (a === 0 ? 1 : 0) + M * (y + (a === 1 ? 1 : 0) + M * (z + (a === 2 ? 1 : 0)))]!; if ((f0 < 0) === (f1 < 0)) continue; const u = (a + 1) % 3, w = (a + 2) % 3, g = [gx, gy, gz], q: number[] = [];
        for (const [i, j] of [[0, 0], [-1, 0], [-1, -1], [0, -1]] as const) { const c = [...g]; c[u]! += i; c[w]! += j; const p = vert.get(key(c[0]!, c[1]!, c[2]!)); if (!p) break; /* which corner of that cell the edge's inner end is */ const e = [...g]; if (f0 >= 0) e[a]!++; q.push(p[(e[0]! - c[0]!) | ((e[1]! - c[1]!) << 1) | ((e[2]! - c[2]!) << 2)]!); } if (q.length < 4) continue;
        if (f0 < 0) tri.push(q[0]!, q[1]!, q[2]!, q[0]!, q[2]!, q[3]!); else tri.push(q[0]!, q[2]!, q[1]!, q[0]!, q[3]!, q[2]!);
      }
    }
  }
  return largest(new Float32Array(pos), new Uint32Array(tri));
}
/** the largest connected part alone: crumbs that the noise cut loose are left out */
function largest(pos: Float32Array, tri: Uint32Array): { pos: Float32Array; tri: Uint32Array } {
  const nv = pos.length / 3, up = new Int32Array(nv).map((_, i) => i), find = (i: number): number => { while (up[i] !== i) { up[i] = up[up[i]!]!; i = up[i]!; } return i; };
  for (let t = 0; t < tri.length; t += 3) { const a = find(tri[t]!); up[find(tri[t + 1]!)] = a; up[find(tri[t + 2]!)] = a; }
  const size = new Int32Array(nv); for (let t = 0; t < tri.length; t += 3) size[find(tri[t]!)]!++; let best = 0; for (let i = 0; i < nv; i++) if (size[i]! > size[best]!) best = i;
  const to = new Int32Array(nv).fill(-1), p: number[] = [], q: number[] = []; for (let t = 0; t < tri.length; t += 3) { if (find(tri[t]!) !== best) continue; for (let k = 0; k < 3; k++) { const v = tri[t + k]!; if (to[v]! < 0) { to[v] = p.length / 3; p.push(pos[v * 3]!, pos[v * 3 + 1]!, pos[v * 3 + 2]!); } q.push(to[v]!); } }
  return { pos: new Float32Array(p), tri: new Uint32Array(q) };
}

const FLUTES = 20, R = 0.30;
/** the column's plain shape: plinth, two rings with a hollow between, a fluted shaft that is broken off; `drum` is the piece that fell, along x, broken at both ends */
function columnField(drum: boolean) {
  const flute = (x: number, z: number, taper: number) => { const a = Math.atan2(z, x), s = Math.PI * 2 / FLUTES, k = Math.round(a / s) * s, r0 = R * taper, cx = Math.cos(k) * r0 * 1.035, cz = Math.sin(k) * r0 * 1.035; return Math.hypot(x - cx, z - cz) - r0 * 0.128; };
  const shaft = (x: number, y: number, z: number) => { const taper = 1 - 0.035 * y, r = Math.hypot(x, z); return smax(r - R * taper, -flute(x, z, taper), 0.004); };
  const torus = (x: number, y: number, z: number, rad: number, tube: number, at: number) => Math.hypot(Math.hypot(x, z) - rad, y - at) - tube;
  const chip = (x: number, y: number, z: number) => { const n = fbm3(x * 5.5, y * 5.5 + 3, z * 5.5, 3); return 0.024 * step(0.08, 0.34, n); };
  if (drum) return (x: number, y: number, z: number) => { const br = (s: number) => 0.055 * fbm3(y * 6 + s, z * 6, 2 + s, 4) + 0.10 * fbm3(y * 1.7, z * 1.7, 8 + s, 2); let d = shaft(y, x + 0.4, z); d = smax(d, x - 0.46 - 0.35 * y + br(1.7), 0.006); d = smax(d, -x - 0.40 + 0.2 * z + br(6.1), 0.006); return d + chip(x, y, z) * 0.9; };
  return (x: number, y: number, z: number) => {
    const bx = Math.abs(x) - 0.46, bz = Math.abs(z) - 0.46, by = Math.abs(y - 0.075) - 0.075, box = Math.min(Math.max(bx, by, bz), 0) + Math.hypot(Math.max(bx, 0), Math.max(by, 0), Math.max(bz, 0)) - 0.012;
    let d = smax(shaft(x, y - 0.42, z), 0.40 - y, 0.01); const brk = 0.075 * fbm3(x * 5 + 1.3, z * 5, 4, 4) + 0.12 * fbm3(x * 1.6, z * 1.6, 1.1, 2); d = smax(d, y - 1.46 - 0.5 * x - 0.16 * z + brk, 0.006);
    d = Math.min(d, Math.max(Math.hypot(x, z) - R * 1.08, Math.abs(y - 0.25) - 0.17));
    d = smin(d, Math.max(Math.hypot(x, z) - R * 1.16, Math.abs(y - 0.40) - 0.018) - 0.004, 0.012); d = smin(d, torus(x, y, z, R * 1.13, 0.052, 0.335), 0.004); d = smin(d, Math.max(Math.hypot(x, z) - R * 1.17 + 0.05 * (1 - ((y - 0.268) / 0.05) ** 2), Math.abs(y - 0.268) - 0.05), 0.004); d = smin(d, torus(x, y, z, R * 1.27, 0.072, 0.205), 0.004); d = smin(d, box, 0.004);
    return d + chip(x, y, z);
  };
}
/** what the weather and the years leave in the stone: how far in, at a place, and its color. Nothing steeper or finer than vertices `apart` metres from each other can hold: what is finer is the material's */
function weather(x: number, y: number, z: number, up: number, apart: number): { deep: number; col: [number, number, number, number] } {
  const a = fbm3(x * 14, y * 14, z * 14, 4), b = fbm3(x * 60 + 5, y * 60, z * 60, 4), soft = apart * 22, c = apart < 0.0016 ? noise3(x * 240, y * 240, z * 240) : 0, bed = noise3(x * 1.5, y * 46 + 2 * noise3(x * 3, y * 3, z * 3), z * 1.5);
  const pits = step(0.30 - soft, 0.52 + soft, noise3(x * 85 + 2, y * 60, z * 85)) * (0.5 + 0.5 * step(-0.1, 0.4, noise3(x * 9, y * 9, z * 9 + 3))), w = noise3(x * 4.2 + 9, y * 4.2, z * 4.2), crack = Math.abs(noise3(x * 3.1 + 0.35 * noise3(x * 18, y * 18, z * 18), y * 2.2, z * 3.1 + 5)), split = (1 - step(0.004, 0.018 + apart * 6, crack)) * step(0.0, 0.25, w);
  const deep = 0.0035 * a + 0.0013 * b + 0.00035 * c + 0.0012 * Math.max(0, bed) + 0.0048 * pits + 0.010 * split;
  const moss = step(0.10, 0.34, fbm3(x * 5 + 3, y * 5, z * 5, 4) + 0.30 * up - 0.16 - 0.25 * y) * 0.9, lich = step(0.26 - soft * 0.5, 0.33 + soft * 0.5, noise3(x * 22 + 7, y * 22, z * 22) + 0.10 * b) * step(0.0, 0.3, noise3(x * 2.5, y * 2.5 + 4, z * 2.5)), v = 0.80 + 0.14 * a + 0.12 * b + 0.07 * c - 0.10 * Math.max(0, bed) - 0.34 * pits - 0.4 * split;
  let col: [number, number, number] = [v, v * 0.955, v * 0.87]; const to = (k: number, q: [number, number, number]) => { col = col.map((p, i) => p + (q[i]! - p) * k) as typeof col; }; to(lich * 0.75, [0.83, 0.74, 0.42]); to(moss * 0.85, [0.36 + 0.1 * b, 0.44 + 0.14 * b, 0.20]);
  return { deep: deep - moss * 0.0016 * (1 + 2 * c), col: [col[0], col[1], col[2], 1 - 0.55 * pits - 0.5 * split] };
}
/** the hero: the standing part, or the fallen drum. `finer` = how often its net is split in four (each doubles the detail a way) */
export function column(drum: boolean, cell = 0.011, finer = 2): Mesh {
  const f = columnField(drum); let m = nets(f, drum ? [-0.66, -0.36, -0.36] : [-0.52, -0.04, -0.52], drum ? [0.74, 0.36, 0.36] : [0.52, 1.95, 0.52], cell);
  const e = 0.0008, onto = (p: Float32Array, times: number) => { for (let i = 0; i < p.length; i += 3) for (let k = 0; k < times; k++) { const x = p[i]!, y = p[i + 1]!, z = p[i + 2]!, d = f(x, y, z); let gx = f(x + e, y, z) - d, gy = f(x, y + e, z) - d, gz = f(x, y, z + e) - d; const l = Math.hypot(gx, gy, gz); if (l < 1e-9) break; gx /= l; gy /= l; gz /= l; const s = d / Math.max(l / e, 0.4) * 0.9; p[i] = x - gx * s; p[i + 1] = y - gy * s; p[i + 2] = z - gz * s; } };
  onto(m.pos, 2); for (let k = 0; k < finer; k++) { m = split4(m); if (k < 2) onto(m.pos, 1); }
  const n0 = normalsOf(m.pos, m.tri), nv = m.pos.length / 3, pos = new Float32Array(m.pos.length), col = new Uint8Array(nv * 4);
  for (let i = 0; i < nv; i++) { const x = m.pos[i * 3]!, y = m.pos[i * 3 + 1]!, z = m.pos[i * 3 + 2]!, w = weather(drum ? x + 9 : x, y, z, n0[i * 3 + 1]!, cell / 2 ** finer); for (let k = 0; k < 3; k++) pos[i * 3 + k] = m.pos[i * 3 + k]! - n0[i * 3 + k]! * w.deep; for (let k = 0; k < 4; k++) col[i * 4 + k] = 255 * clamp01(w.col[k]!); }
  return { pos, nrm: normalsOf(pos, m.tri), col, tri: m.tri };
}
void cross;
