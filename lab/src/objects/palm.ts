// A DATE PALM: a thick stem clad in the stubs of old leaves, wound round it in spirals; a heart of brown fibre; and out of it fronds in a spiral, the young upright in the middle, the old low and hanging, each a keel with
// narrow stiff leaflets in two rows that stand in a V. Some old fronds have dried; fruit hangs on orange strands.
// Coarser: two short leaves stand for three leaflets, then one for seven; then five are a frond.
import { type V3, type Rot, add, sub, mul, norm, cross, dot, rng, rotY, apply } from '../math';
import type { Cluster, Leaf, World } from '../scene/world';
import { M } from '../scene/materials';
import type { Made, ObjectKind, Put } from './object';
const TOP: V3 = [0.10, 2.36, -0.04];
/** how long a frame is: leaves are kept within 1.2 of its middle, blades within 2.1 */
const frameOf = (level: number) => level ? 1.62 : 1.25;
const GREEN: V3 = [0.276, 0.347, 0.205], YELLOW: V3 = [0.390, 0.402, 0.182], DRY: V3 = [0.520, 0.425, 0.285];
const square = (n: V3, t: V3): V3 => norm(sub(n, mul(t, dot(n, t))));
/** a frond leaves the heart at `lean` from upright and bends by `bend` more; its leaflets stand up in a V by `vee` (dry ones hang) */
interface Frond { lean: number; bend: number; long: number; vee: number; pairs: number; leaflet: number; side: number; many: number; age: number }
const FRONDS: Frond[] = [
  { lean: 0.08, bend: 0.22, long: 1.30, vee: 1.00, pairs: 34, leaflet: 0.290, side: 0.05, many: 5, age: 0 },
  { lean: 0.22, bend: 0.46, long: 1.60, vee: 0.75, pairs: 38, leaflet: 0.365, side: -0.06, many: 4, age: 0.06 },
  { lean: 0.40, bend: 0.66, long: 1.74, vee: 0.62, pairs: 40, leaflet: 0.398, side: 0.08, many: 5, age: 0.15 },
  { lean: 0.60, bend: 0.82, long: 1.80, vee: 0.55, pairs: 41, leaflet: 0.408, side: -0.10, many: 5, age: 0.28 },
  { lean: 0.80, bend: 0.94, long: 1.82, vee: 0.48, pairs: 41, leaflet: 0.408, side: 0.06, many: 5, age: 0.40 },
  { lean: 1.00, bend: 0.98, long: 1.80, vee: 0.42, pairs: 41, leaflet: 0.408, side: 0.12, many: 5, age: 0.55 },
  { lean: 1.22, bend: 1.02, long: 1.76, vee: 0.34, pairs: 40, leaflet: 0.398, side: -0.08, many: 5, age: 0.70 },
  { lean: 1.46, bend: 0.98, long: 1.68, vee: 0.26, pairs: 40, leaflet: 0.387, side: 0.10, many: 4, age: 0.85 },
  { lean: 1.74, bend: 0.80, long: 1.52, vee: 0.12, pairs: 37, leaflet: 0.376, side: -0.12, many: 3, age: 1 },
  { lean: 2.02, bend: 0.62, long: 1.20, vee: -0.55, pairs: 27, leaflet: 0.312, side: 0.14, many: 3, age: 2 },
];
/** along the keel (outward is +x, the heart is the origin): where it is, its way, its upper side, its left */
function keel(f: Frond): (q: number) => { at: V3; d: V3; up: V3; left: V3 } {
  const N = 40, at: V3[] = [[0.05 * Math.sin(f.lean), 0, 0]], ds: V3[] = []; for (let i = 0; i <= N; i++) { const q = i / N, a = f.lean + f.bend * Math.pow(q, 1.35), z = f.side * q * q * 1.6, d = norm([Math.sin(a), Math.cos(a), z]); ds.push(d); at.push(add(at[i]!, mul(d, f.long / N))); }
  return q => { const x = Math.min(0.9999, Math.max(0, q)) * N, i = Math.floor(x), t = x - i, d = norm(add(mul(ds[i]!, 1 - t), mul(ds[i + 1]!, t))), up = norm([-d[1], d[0], 0]); return { at: add(mul(at[i]!, 1 - t), mul(at[i + 1]!, t)), d, up: square(up, d), left: cross(square(up, d), d) }; };
}
interface Leaflet { root: V3; t: V3; n: V3; long: number; tone: number }
function leaflets(f: Frond, seed: number): Leaflet[][] {
  const r = rng(6100 + seed), way = keel(f), rows: Leaflet[][] = [[], []];
  for (let s = 0; s < 2; s++) for (let i = 0; i < f.pairs; i++) {
    const u = (i + 0.5 + (s ? 0.25 : -0.25) + (r() - 0.5) * 0.3) / f.pairs, q = 0.15 + 0.845 * u, w = way(q), sg = s ? 1 : -1;
    // spines at the foot, longest before the middle, shorter to the tip; they lean forward, more toward the tip, and stand a little in ranks
    const long = f.leaflet * Math.min(1, 0.22 + 3.6 * u) * (1 - 0.50 * Math.pow(u, 2.2)) * (0.90 + 0.2 * r()), ahead = (f.age < 0.1 ? 0.62 : 0.98) - 0.50 * u + (r() - 0.5) * 0.16, vee = f.vee * (1 - 0.25 * u) + [0.20, -0.16, 0.02][i % 3]! + (r() - 0.5) * 0.26, out = add(mul(w.left, sg * Math.cos(vee)), mul(w.up, Math.sin(vee))), limp = f.age * 0.10 + (f.age > 1 ? 0.35 : 0) + 0.10 * r(), t = norm(add(add(mul(w.d, Math.cos(ahead)), mul(out, Math.sin(ahead))), [0, -limp, 0])), n = square(add(mul(w.up, Math.cos(vee)), mul(w.left, -sg * Math.sin(vee) + (r() - 0.5) * 0.6)), t);
    rows[s]!.push({ root: add(w.at, mul(w.left, sg * 0.004)), t, n, long, tone: 0.28 + 0.34 * r() });
  }
  return rows;
}
const ROWS = FRONDS.map((f, i) => leaflets(f, i));
/** how high a kind's frame is over the heart: blades are kept only over their foot, so the finest's is at the frond's lowest; the others' in its midst */
const REACH = FRONDS.map((f, i) => { const way = keel(f), ys = [...Array(21).keys()].map(k => way(k / 20).at[1]), tips = ROWS[i]!.flat().map(l => l.root[1] + l.t[1] * l.long); return { lo: Math.min(...ys, ...tips), hi: Math.max(...ys, ...tips) }; });
const frameAt = (kind: number, level: number) => level ? (REACH[kind]!.lo + REACH[kind]!.hi) / 2 : Math.min(-0.45, REACH[kind]!.lo + 0.03);
interface Step { bright: V3; big: number }
const STEPS: Step[] = [{ bright: [1, 1, 1], big: 1 }, { bright: [0.861, 0.844, 0.885], big: 0.96 }, { bright: [0.859, 0.842, 0.872], big: 1.005 }, { bright: [0.893, 0.876, 0.910], big: 0.945 }];
function frond(kind: number, level: number): Cluster {
  const f = FRONDS[kind]!, rows = ROWS[kind]!, way = keel(f), y = frameAt(kind, level), S = frameOf(level), leaves: Leaf[] = [], big = STEPS[level]!.big, put = (at: V3, t: V3, n: V3, size: number, tone: number) => leaves.push({ at: mul(sub(at, [0, y, 0]), 1 / S), t, n: square(n, t), size: size / S, tone }), mean = (v: V3[]) => mul(v.reduce((a, b) => add(a, b), [0, 0, 0] as V3), 1 / v.length);
  if (level === 0) {
    for (const row of rows) for (const l of row) put(add(l.root, mul(l.t, l.long * 0.485)), l.t, l.n, l.long / 2, l.tone);
    // the keel: narrow blades end over end, flat and on edge, thinner to the tip
    for (let i = 0; i < 7; i++) { const a = 0.10 + 0.86 * i / 7, b = Math.min(0.995, a + 0.86 / 7 * 1.9), p = way(a).at, e = way(b).at, d = sub(e, p), t = norm(d), m = way((a + b) / 2), half = Math.hypot(...d) / 2 * (1 - 0.35 * i / 7); for (const n of [m.up, m.left]) put(add(mul(add(p, e), 0.5), mul(m.up, -0.003)), t, n, half, 0.90); }
    return { kind: 1, leaves };
  }
  // (by turns they lean a little to either side: turned as the mean of those they stand for, they would all look the same way, and be brighter by lamp for it)
  if (level < 3) { const per = level === 1 ? 3 : 7; for (const row of rows) for (let i = 0; i < row.length; i += per) { const g = row.slice(i, i + per); if (g.length * 2 < per) continue; const root = mean(g.map(l => l.root)), long = g.reduce((a, l) => a + l.long, 0) / g.length, tone = g.reduce((a, l) => a + l.tone, 0) / g.length, full = Math.sqrt(g.length / per) * big, way1 = mean(g.map(l => l.t));
    if (level === 1) for (const k of [0, 1]) { const one = g[k ? g.length - 1 : 0]!, t = norm(add(way1, mul(one.t, 0.8))); put(add(root, mul(t, long * [0.26, 0.74][k]!)), t, add(one.n, mul(cross(one.n, t), k === ((i / per) & 1) ? 0.3 : -0.3)), long * 0.226 * full, tone); } else { const t = norm(way1); put(add(root, mul(t, long * 0.5)), t, add(mean(g.map(l => l.n)), mul(cross(mean(g.map(l => l.n)), t), (i / per) % 2 ? 0.5 : -0.5)), long * 0.478 * full, tone); } }
    if (level === 1) for (let i = 0; i < 4; i++) { const m = way(0.16 + 0.2 * i); put(m.at, m.d, m.up, 0.035, 0.9); } }
  else for (let i = 0; i < 5; i++) { const q = 0.16 + 0.84 * (i + 0.5) / 5, m = way(q), wide = f.leaflet * 0.925 * [0.62, 0.84, 0.82, 0.70, 0.5][i]!; put(m.at, m.d, m.up, wide * big, 0.45); }
  return { kind: 0, leaves };
}
const kept = new WeakMap<World, Map<number, number[]>>(), made = new Map<number, Cluster[]>();
const clusters = (w: World, level: number): number[] => { let m = kept.get(w); if (!m) kept.set(w, m = new Map()); let l = m.get(level); if (!l) { let c = made.get(level); if (!c) made.set(level, c = FRONDS.map((_, i) => frond(i, level))); m.set(level, l = c.map(x => { w.clusters.push(x); return w.clusters.length - 1; })); } return l; };
// ---- the stem
const axis = (y: number): V3 => { const k = y / TOP[1]; return [TOP[0] * k * k, y, TOP[2] * k * k]; }, girth = (y: number) => 0.150 + 0.050 * Math.exp(-y / 0.18) + 0.020 * Math.max(0, (y - 1.2) / 1.0);
const BARK: V3 = [0.585, 0.575, 0.575], TAN: V3 = [0.74, 0.645, 0.545], FIBRE: V3 = [0.56, 0.46, 0.37];
function stem(p: Put, level: number) {
  const r = rng(p.seed * 29 + 7), mat = M.paleBark!, n = level < 2 ? 9 : level === 2 ? 5 : 3, plump = level === 0 ? 0.80 : level === 1 ? 0.84 : 1.06;
  for (let i = 0; i < n; i++) { const y0 = 2.24 * i / n, y1 = 2.24 * (i + 1) / n; p.bar(i ? axis(y0) : [0, 0, 0], i ? axis(y1) : [0, y1, 0], girth(y0) * plump, girth(y1) * plump, mat, mul(BARK, level < 2 ? 0.80 : 0.94)); }
  if (level < 2) { const N = level ? 98 : 196; for (let i = 0; i < N; i++) {
    const k = (i + 0.5) / N, y = 0.075 + 2.06 * k, a = i * 2.39996 + (r() - 0.5) * 0.16, o: V3 = [Math.cos(a), 0, Math.sin(a)], side: V3 = [-Math.sin(a), 0, Math.cos(a)], g = girth(y), big = (level ? 1.32 : 1) * (0.88 + 0.24 * r()), w = 0.056 * big * g / 0.165, h = (0.062 + 0.04 * Math.max(0, k - 0.8) * 5) * big, lean = 0.24 + 0.12 * r() + 0.35 * Math.max(0, k - 0.82) * 5, up: V3 = norm(add(mul(o, Math.sin(lean)), [0, Math.cos(lean), 0])), z = cross(side, up), fresh = Math.max(0, Math.min(1, (k - 0.55) * 2.2)) , v = 0.84 + 0.3 * r(), c = [0, 1, 2].map(e => Math.min(1, (BARK[e]! + (TAN[e]! - BARK[e]!) * fresh) * v)) as V3;
    // broad where it clasps the stem, narrow where it was cut
    const m = 0.36, e = Math.hypot(1, m), zo = z[0] * o[0] + z[2] * o[2] > 0 ? z : mul(z, -1); p.add({ at: add(add(axis(y), mul(o, g * 0.93)), mul(up, h * 0.45)), half: [w, h, 0.021], mat, tint: c, rot: [cross(up, zo), up, zo], planes: [[[1 / e, m / e, 0], (w * 0.72) / e], [[-1 / e, m / e, 0], (w * 0.72) / e]] });
  } }
  p.bar([0, 0, 0], [0, 0.085, 0], 0.218, 0.185, mat, mul(BARK, level < 2 ? 0.84 : 0.92)); if (level < 2) p.bar([0, 0.004, 0], [0, 0.024, 0], 0.238, 0.216, mat, mul(BARK, 0.70));
  // the heart
  const heart = axis(2.24); p.ball(heart, [0.19, 0.20, 0.19], mat, FIBRE); if (level < 3) p.ball(add(heart, [0, 0.12, 0]), [0.11, 0.17, 0.11], mat, mul(FIBRE, 1.08));
}
/** dates, on orange strands that hang from a stalk between the fronds */
function fruit(p: Put, level: number) {
  const r = rng(p.seed * 37 + 3), mat = M['lamp shade']!, strand: V3 = [0.93, 0.63, 0.25]; for (let b = 0; b < 3; b++) { const R = rotY(0.95 + b * 2.35 + (r() - 0.5) * 0.4 + (p.seed === 1 ? 0 : p.seed)), P = (x: number, y: number, z = 0): V3 => add(TOP, apply(R, [x, y, z])), pts = [P(0.03, -0.02), P(0.24, 0.06), P(0.40, -0.02), P(0.485, -0.21)], long = 0.30 + 0.06 * r(); if (level < 2) for (let i = 0; i < 3; i++) p.bar(pts[i]!, pts[i + 1]!, 0.015 - 0.002 * i, 0.013 - 0.002 * i, mat, strand);
    if (level) { p.ball(P(0.49, -0.17 - long * 0.5), [0.074, 0.165, 0.074], mat, [0.69, 0.455, 0.19]); if (level > 1) p.bar(P(0.02, 0, 0), P(0.485, -0.22), 0.013, 0.013, mat, strand); continue; }
    for (let i = 0; i < 13; i++) { const a = i * 2.39996, k = Math.sqrt((i + 0.5) / 13), from = P(0.465 + 0.01 * Math.cos(a), -0.10 - 0.05 * r(), 0.01 * Math.sin(a)), mid = P(0.48 + 0.085 * k * Math.cos(a), -0.20 - 0.06 * r(), 0.085 * k * Math.sin(a)), end = P(0.49 + 0.105 * k * Math.cos(a), -0.16 - long * (1.05 - 0.25 * k) * (0.9 + 0.2 * r()), 0.105 * k * Math.sin(a)); p.bar(from, mid, 0.0045, 0.004, mat, strand); p.bar(mid, end, 0.004, 0.003, mat, strand); for (let j = 0; j < 3; j++) { const q = 0.14 + 0.84 * (j + r() * 0.6) / 2.6, c = add(mul(mid, 1 - q), mul(end, q)), t = r(); p.ball(add(c, [(r() - 0.5) * 0.018, 0, (r() - 0.5) * 0.018]), [0.0150, 0.0220, 0.0150], mat, [0.78 + 0.14 * t, 0.42 + 0.20 * t, 0.15 + 0.06 * t]); } } }
}
/** where every frond is: which kind, which way, how large */
interface Grown { kind: number; yaw: number; scale: number; tint: V3 }
function crown(seed: number): Grown[] {
  const r = rng(seed * 101 + 17), out: Grown[] = []; let i = 0; FRONDS.forEach((f, kind) => { for (let j = 0; j < f.many; j++, i++) { const v = 0.93 + 0.14 * r(), k = Math.max(0, Math.min(1, f.age > 1 ? 0 : 0.10 + 0.75 * Math.max(0, f.age - 0.65) + (f.age < 0.1 ? 0.25 : 0) + 0.2 * (r() - 0.5))), dry = f.age > 1; out.push({ kind, yaw: i * 2.39996 + 0.4 + (r() - 0.5) * 0.25 + (dry ? j * 2.1 : 0), scale: 0.95 + 0.10 * r(), tint: mul(dry ? DRY : [0, 1, 2].map(c => GREEN[c]! + (YELLOW[c]! - GREEN[c]!) * k) as V3, v) }); } }); return out;
}
function put(p: Put, level: number) {
  const ix = clusters(p.w, level), s = STEPS[level]!, dice = rng(p.seed * 13 + 2), all = crown(p.seed); stem(p, level); fruit(p, level);
  p.from({ pieces: [], instances: all.map(g => { const dy = frameAt(g.kind, level) * g.scale, at = add(TOP, [0, dy, 0]), hangs = FRONDS[g.kind]!.lean > 1.2 ? -1 : 1; return { cluster: ix[g.kind]!, at, rot: rotY(g.yaw), scale: frameOf(level) * g.scale, base: add(at, [0, -2.9 * dy, 0]), sway: hangs * (0.030 + 0.012 * dice()), phase: dice() * 6.28, tint: [0, 1, 2].map(c => g.tint[c]! * s.bright[c]!) as V3 }; }) });
  // the feet of the keels
  if (level < 2) for (const g of all) { const f = FRONDS[g.kind]!, way = keel(f), R: Rot = rotY(g.yaw), at = (q: number): V3 => add(TOP, mul(apply(R, way(q).at), g.scale)), c: V3 = f.age > 1 ? [0.62, 0.50, 0.38] : [0.55 + 0.10 * f.age, 0.595, 0.33 + 0.03 * f.age], k = f.age === 0 ? 0.7 : 1; p.bar(add(TOP, mul(apply(R, [0.02, -0.10, 0]), 1)), at(0.09), 0.030 * k, 0.019 * k, M['lamp shade']!, mul(c, 1.02)); if (level === 0) p.bar(at(0.085), at(0.20), 0.0185 * k, 0.0105 * k, M['lamp shade']!, c); }
}
export const palm: ObjectKind = { name: 'palm', title: 'Date palm', make: (): Made => ({
  name: 'palm', title: 'Date palm', size: [3.6, 3.92, 3.75], on: 'floor', close: { at: [0.2, 2.4, 0.4], far: 5 }, read: 0, moves: { kind: 'sways' }, crowd: 49,
  levels: [
    { shape: 'leaves', error: 0, less: '', put(p) { put(p, 0); } },
    { shape: 'leaves', error: 0.03, less: 'half of the stubs; two short leaves for three leaflets', put(p) { put(p, 1); } },
    { shape: 'leaves', error: 0.06, less: 'stubs, the feet of the keels; one leaf for seven leaflets', put(p) { put(p, 2); } },
    { shape: 'leaves', error: 0.13, less: 'leaflets: five leaves a frond, along it', put(p) { put(p, 3); } },
  ],
}) };
