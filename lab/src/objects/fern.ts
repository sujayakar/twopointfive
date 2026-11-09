// A FERN IN A POT: fronds that rise from a crown and arch over the rim, each a stem with leaflets in two rows that grow smaller to its tip; young ones stand paler and upright in the middle, two are still curled.
// Coarser: a leaflet's lobes are one; then one leaflet stands for two; then five leaves along it are a frond.
import { type V3, type Rot, add, sub, mul, norm, cross, dot, rng, rotY, apply, compose } from '../math';
import type { Cluster, Leaf, World } from '../scene/world';
import { M } from '../scene/materials';
import type { Made, ObjectKind, Put } from './object';
const POT: V3 = [0.80, 0.52, 0.40], TOP = 0.1475;
const GREEN: V3 = [0.235, 0.350, 0.105];
/** a frond grows along +y from −0.95, and bends toward +x by `bend` in all; `pairs` of leaflets */
interface Frond { bend: number; side: number; pairs: number; seed: number }
const FRONDS: Frond[] = [{ bend: 0.75, side: 0.10, pairs: 24, seed: 1 }, { bend: 1.35, side: -0.12, pairs: 26, seed: 2 }, { bend: 1.75, side: 0.16, pairs: 26, seed: 3 }, { bend: 2.10, side: -0.06, pairs: 28, seed: 4 }, { bend: 1.55, side: 0.05, pairs: 25, seed: 5 }];
const LONG = 1.9;
/** along the stem: where it is and which way it goes, 0 to 1 */
function stem(f: Frond): (k: number) => { at: V3; d: V3 } { const N = 48, at: V3[] = [[0, -0.95, 0]], dir: V3[] = []; for (let i = 0; i <= N; i++) { const k = i / N, a = f.bend * Math.pow(k, 1.45), s = f.side * k * k * 1.5, d = norm([Math.sin(a), Math.cos(a), s]); dir.push(d); at.push(add(at[i]!, mul(d, LONG / N))); } return k => { const x = Math.min(0.9999, Math.max(0, k)) * N, i = Math.floor(x), t = x - i; return { at: add(mul(at[i]!, 1 - t), mul(at[i + 1]!, t)), d: norm(add(mul(dir[i]!, 1 - t), mul(dir[i + 1]!, t))) }; }; }
/** the side of a frond that looks to the light (it bends away from it) */
const upOf = (d: V3): V3 => norm([-d[1], d[0], 0]);
/** how much brighter a level's leaves are, where they shade each other more */
const BRIGHT = [1, 0.99, 0.97, 0.965]; const MID = 0.425, THIN = 0.56, FAR = 1.06;
function frond(f: Frond, level: number): Cluster {
  const r = rng(900 + f.seed), leaves: Leaf[] = [], way = stem(f), face = (n: V3, t: V3): V3 => norm(sub(n, mul(t, dot(n, t))));
  const put = (at: V3, t: V3, n: V3, size: number, tone: number, wood = false) => leaves.push({ at, t, n: face(n, t), size, tone, ...(wood ? { wood } : {}) });
  if (level < 2) { const n = level ? 4 : 10; for (let i = 0; i < n; i++) { const a = way(i / n * 0.97).at, b = way((i + 1) / n * 0.97).at, d = sub(b, a), t = norm(d); put(mul(add(a, b), 0.5), t, [t[1], -t[0], 0.4], Math.hypot(...d) / 2 * 1.06, r(), true); } }
  if (level === 3) { for (let i = 0; i < 5; i++) { const q = (i + 0.55) / 5, w = way(0.17 + 0.83 * q), wide = [0.23, 0.275, 0.27, 0.235, 0.17][i]!; put(w.at, w.d, upOf(w.d), wide * FAR, 0.51); } return { kind: 0, leaves }; }
  for (let i = 0; i < f.pairs; i++) for (const s of [-1, 1]) { if (level === 2 && i % 2 !== (s > 0 ? 0 : 1)) continue;
    const k = 0.17 + 0.81 * (i + 0.5 + s * 0.22) / f.pairs, w = way(k), up = upOf(w.d), out = mul(norm(cross(up, w.d)), s);
    // long in the middle of the frond, short at its foot, none at its tip; each leans forward, and hangs a little
    const q = (k - 0.17) / 0.81, long = 0.25 * Math.pow(Math.sin(Math.PI * Math.pow(0.13 + 0.87 * q, 0.72)), 0.8) * (0.92 + 0.16 * r()) + 0.015, a = r(), b = r(), d = norm(add(add(out, mul(w.d, 0.30 + 0.14 * a)), mul(up, 0.06 - 0.26 * b))), n = norm(add(up, add(mul(out, (a - 0.5) * 0.5), mul(w.d, (b - 0.5) * 0.5)))), tone = 0.30 + 0.42 * r();
    if (level === 0) for (let j = 0; j < 2; j++) put(add(add(w.at, mul(d, long * [0.31, 0.69][j]!)), mul(up, -0.10 * long * j)), norm(add(d, mul(up, -0.16 * j))), n, long * [0.325, 0.285][j]!, tone);
    else if (level === 1) put(add(w.at, mul(d, long * 0.47)), d, n, long * MID, tone);
    else put(add(w.at, mul(d, long * 0.50)), d, n, long * THIN, tone);
  }
  return { kind: 0, leaves };
}
/** a pot of fired clay in its saucer */
function pot(p: Put, level: number) {
  const side = (y0: number, y1: number, r0: number, r1: number, tint = POT) => p.bar([0, y0, 0], [0, y1, 0], r0, r1, M['lamp shade']!, tint); if (level < 2) { side(0, 0.016, 0.088, 0.098, mul(POT, 0.96)); side(0.004, 0.112, 0.0715, 0.0965); side(0.106, 0.146, 0.1045, 0.1075, mul(POT, 1.02)); p.drum([0, 0.1415, 0], 0.0975, 0.0062, M.soil!, [1.15, 1.15, 1.15]); }
  else { side(0, 0.112, 0.082, 0.096); side(0.106, 0.146, 0.1045, 0.1075, mul(POT, 1.02)); p.drum([0, 0.1415, 0], 0.0975, 0.0062, M.soil!, [1.15, 1.15, 1.15]); }
}
/** a young frond, still curled: a pale stem that rises and rolls up */
function curled(p: Put, root: V3, high: number, a: number, level: number) {
  const o: V3 = [Math.cos(a), 0, Math.sin(a)], pts: V3[] = [], R = high * 0.17, n = level ? 5 : 16; for (let i = 0; i <= 2; i++) pts.push(add(root, [o[0] * high * 0.05 * i, high * 0.32 * i, o[2] * high * 0.05 * i])); const c = add(pts[2]!, mul(o, R)); for (let i = 1; i <= n; i++) { const t = i / n * (level ? 5 : 9.2), r = R * Math.pow(0.80, t), q = Math.PI - t; pts.push(add(c, add(mul(o, Math.cos(q) * r), [0, Math.sin(q) * r, 0]))); }
  for (let i = 0; i + 1 < pts.length; i++) { const k = i / pts.length, r = high * 0.032 * (1 - 0.45 * k); p.bar(pts[i]!, pts[i + 1]!, r, r * 0.97, M['lamp shade']!, [0.52, 0.60, 0.26]); if (!level && i) p.ball(pts[i]!, [r, r, r], M['lamp shade']!, [0.52, 0.60, 0.26]); }
}
/** one frond as it stands: its root, which way it grows and bends, how long it is, which of the kinds, how old (1 to 0) */
interface Grown { root: V3; y: V3; x: V3; scale: number; kind: number; age: number; way: number }
/** the crown: outer fronds long and low, inner ones short and upright */
const CROWN: Grown[] = (() => { const r = rng(338), out: Grown[] = [], N = 25; for (let i = 0; i < N; i++) { const inner = i / (N - 1), a = i * 2.39996 + (r() - 0.5) * 0.5, lean = 0.64 - 0.52 * inner + (r() - 0.5) * 0.14, o: V3 = [Math.cos(a), 0, Math.sin(a)], y = norm(add(mul(o, Math.sin(lean)), [0, Math.cos(lean), 0])), x = norm(sub(o, mul(y, dot(o, y)))), long = (0.58 - 0.18 * inner) * (0.86 + 0.26 * r()); out.push({ root: [o[0] * 0.034 * (1 - inner * 0.6), TOP + 0.004, o[2] * 0.034 * (1 - inner * 0.6)], y, x, scale: long / LONG, kind: inner > 0.78 ? 0 : inner > 0.45 ? (i % 2 ? 4 : 1) : [2, 3, 1, 2][i % 4]!, age: 1 - inner, way: ((a % 6.2832) + 6.2832) % 6.2832 }); } return out; })();
/** a third of the crown is one spray: the wind moves it as one, and its leaves know what shades them */
interface Bunch { at: V3; y: V3; of: Grown[] }
const THIRDS = 3, WIDE = 0.42, BUNCHES: Bunch[] = Array.from({ length: THIRDS }, (_, k) => { const a = (k + 0.5) / THIRDS * 6.2832, lean = 0.40, y: V3 = [Math.sin(lean) * Math.cos(a), Math.cos(lean), Math.sin(lean) * Math.sin(a)]; return { at: add([0, TOP + 0.004, 0], mul(y, 0.95 * WIDE)), y, of: CROWN.filter(g => Math.floor(g.way / 6.2832 * THIRDS) === k) }; });
const frame = (b: Bunch): Rot => { const x = norm(cross(b.y, [0.31, 0.2, 0.93])); return [x, b.y, cross(x, b.y)]; };
/** a bunch's leaves, in its own frame, one long */
function bunch(b: Bunch, level: number, fronds: Cluster[]): Cluster { const R = frame(b), into = (v: V3): V3 => [dot(R[0], v), dot(R[1], v), dot(R[2], v)]; return { kind: 0, leaves: b.of.flatMap((g, n) => { const z = cross(g.x, g.y), turn = (v: V3): V3 => add(add(mul(g.x, v[0]), mul(g.y, v[1])), mul(z, v[2])), mid = add(g.root, mul(g.y, 0.95 * g.scale)), light = 0.30 + 0.50 * Math.pow(1 - g.age, 1.6) + 0.10 * ((n * 0.618) % 1); return fronds[g.kind]!.leaves.map(l => ({ ...l, at: mul(into(sub(add(mid, mul(turn(l.at), g.scale)), b.at)), 1 / WIDE), n: into(turn(l.n)), t: into(turn(l.t)), size: l.size * g.scale / WIDE, tone: l.wood ? l.tone : Math.min(1, light + (l.tone - 0.4)) })); }) }; }
const kept = new WeakMap<World, Map<number, number[]>>(), grown = new Map<number, Cluster[]>();
const clusters = (w: World, level: number): number[] => { let m = kept.get(w); if (!m) kept.set(w, m = new Map()); let l = m.get(level); if (!l) { let g = grown.get(level); if (!g) { const fronds = FRONDS.map(f => frond(f, level)); grown.set(level, g = BUNCHES.map(b => bunch(b, level, fronds))); } m.set(level, l = g.map(c => { w.clusters.push(c); return w.clusters.length - 1; })); } return l; };
function put(p: Put, level: number) {
  const ix = clusters(p.w, level), dice = rng(p.seed * 17 + 3), big = p.seed === 1 ? 1 : 0.92 + 0.14 * dice(), yaw = p.seed === 1 ? 0 : p.seed * 1.7, about = rotY(yaw);
  p.from({ pieces: [], instances: BUNCHES.map((b, k) => { const at = add(mul(apply(about, sub(b.at, [0, TOP, 0])), big), [0, TOP, 0]), v = (0.95 + 0.10 * dice()) * BRIGHT[level]!, tint = mul(GREEN, v); return { cluster: ix[k]!, at, rot: compose(about, frame(b)), scale: WIDE * big, base: add(at, [0, -2.76 * WIDE, 0]), sway: 0.045 + 0.02 * dice(), phase: dice() * 6.28, tint }; }) });
  pot(p, level); if (level < 2) for (const [x, z, h, a] of [[0.030, 0.022, 0.115, 0.6], [-0.026, 0.030, 0.085, 2.3]] as const) curled(p, [x, TOP, z], h, a, level);
}
export const fern: ObjectKind = { name: 'fern', title: 'Fern in a pot', make: (): Made => ({
  name: 'fern', title: 'Fern in a pot', size: [0.95, 0.53, 0.84], on: 'floor', close: { at: [0.02, 0.32, 0.08], far: 3.2 }, read: 0, moves: { kind: 'sways' }, crowd: 64,
  levels: [
    { shape: 'leaves', error: 0, less: '', put(p) { put(p, 0); } },
    { shape: 'leaves', error: 0.012, less: 'lobes: a leaflet is one leaf', put(p) { put(p, 1); } },
    { shape: 'leaves', error: 0.025, less: 'stems, the curled ones; one leaflet for two', put(p) { put(p, 2); } },
    { shape: 'leaves', error: 0.050, less: 'leaflets: five leaves a frond, along it', put(p) { put(p, 3); } },
  ],
}) };
