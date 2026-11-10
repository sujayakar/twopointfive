// A CLUMP OF PAMPAS GRASS: long narrow blades that rise from one crown and arch over to the ground all round, dead ones in a skirt under them, and above them stalks with silver plumes. A blade is a chain of lenses that bends; a plume
// is hairs in two steps along a stem that nods. Coarser: fewer blades, each of longer (and so wider) lenses, and fewer, longer hairs.
import { type V3, type Rot, add, sub, mul, norm, cross, dot, rng, mix } from '../math';
import type { Cluster, Leaf, World } from '../scene/world';
import { M } from '../scene/materials';
import type { Made, ObjectKind, Put } from './object';
const GREEN: V3 = [0.315, 0.400, 0.205], STRAW: V3 = [0.600, 0.500, 0.300], SILVER: V3 = [0.840, 0.800, 0.755], CULM: V3 = [0.62, 0.59, 0.36];
const face = (n: V3, t: V3): V3 => norm(sub(n, mul(t, dot(n, t)))), CROWN = 0.08;
/** how many of the blades, and of how many lenses each, a level has; and as much for hairs */
const LONGER = [1, 1.005, 1.025, 1.045], KEEP = [1, 0.71, 0.49, 0.285], LENSES = [7, 5, 4, 3], WIDER = [0.145, 0.206, 0.273, 0.42], HAIRS = [1, 0.62, 0, 0];
/** a wedge of the fountain, its foot at 0 0 0, o = +x, metres */
interface Blade { foot: V3; a: number; lean: number; bend: number; long: number; roll: number; tone: number; rank: number }
const blades = (seed: number, n: number, dead: boolean): Blade[] => { const r = rng(2100 + seed); return Array.from({ length: n }, (_, i): Blade => { const q = (i + r()) / n, a = (r() - 0.5) * 1.15, d = 0.11 * Math.sqrt(r()); if (dead) return { foot: [0.05 + 0.06 * r(), -0.05 + 0.10 * r(), 0.16 * (r() - 0.5)], a: a * 1.5, lean: 0.35 + 0.8 * r(), bend: 1.2 + 1.2 * r(), long: 0.34 + 0.40 * r(), roll: (r() - 0.5) * 1.2, tone: 0.1 + 0.7 * r(), rank: r() }; return { foot: [Math.cos(a) * d, -0.05, Math.sin(a) * d], a, lean: 0.06 + 0.40 * q + 0.14 * (r() - 0.5), bend: 1.25 + 0.95 * q + 0.7 * (r() - 0.5), long: (1.38 - 0.20 * q) * (0.72 + 0.36 * r()), roll: (r() - 0.5) * 1.1, tone: 0.12 + 0.50 * r() + 0.2 * (1 - q), rank: r() }; }); };
/** a blade, along it: it leans out, bends more and more, and near its end hangs */
function lenses(b: Blade, n: number, half: number, put: (at: V3, t: V3, n: V3, size: number, tone: number) => void) {
  const N = 24, o: V3 = [Math.cos(b.a), 0, Math.sin(b.a)], side: V3 = [-o[2], 0, o[0]], pts: V3[] = [b.foot]; for (let i = 0; i < N; i++) { const k = (i + 0.5) / N, th = b.lean + b.bend * Math.pow(k, 1.6), s = 0.10 * b.roll * k; pts.push(add(pts[i]!, mul(norm(add(add(mul(o, Math.sin(th)), [0, Math.cos(th), 0]), mul(side, s))), b.long / N))); }
  const at = (k: number): V3 => { const x = Math.max(0, Math.min(N - 1e-6, k * N)), i = Math.floor(x); return add(mul(pts[i]!, 1 - x + i), mul(pts[i + 1]!, x - i)); }, span = half * 2 / b.long;
  for (let i = 0; i < n; i++) { const k0 = (1 - span) * i / (n - 1), p = at(k0), q = at(Math.min(1, k0 + span)), m = at(k0 + span / 2), d = sub(q, p), l = Math.hypot(...d), t = mul(d, 1 / l), c = add(mul(add(p, q), 0.25), mul(m, 0.5)), up = face([0, 1, 0.001], t), w = cross(t, up), a = b.roll * (0.5 + k0); if (c[1] < -CROWN + 0.012) c[1] = -CROWN + 0.012; put(c, t, norm(add(mul(up, Math.cos(a)), mul(w, Math.sin(a)))), l / 2 * (i === n - 1 ? 1 : 1.02), b.tone + 0.08 * (i / n - 0.5)); }
}
const SCALE = 0.5, KINDS = 4, WEDGES = 9, PER = 52, wedges = Array.from({ length: KINDS }, (_, i) => blades(i, PER, false)), skirts = [0, 1].map(i => blades(50 + i, 24, true));
/** in its own frame +y goes out, and its middle is half a unit out from the crown */
function wedge(bs: Blade[], level: number): Cluster { const leaves: Leaf[] = [], into = (v: V3): V3 => [v[2], v[0], v[1]]; bs.filter(b => b.rank < KEEP[level]!).map(b => ({ ...b, long: b.long * LONGER[level]! })).forEach(b => lenses(b, b.long < 0.75 ? Math.max(2, LENSES[level]! - 3) : LENSES[level]!, WIDER[level]! * Math.min(1, b.long / (b.long < 0.75 ? 0.75 : 1.1)), (at, t, n, size, tone) => leaves.push({ at: add(mul(into(at), 1 / SCALE), [0, -0.95, 0]), t: into(t), n: into(n), size: size / SCALE, tone: Math.max(0, Math.min(1, tone)) }))); return { kind: 1, leaves }; }
/** a plume from 0 up, one unit 0.22 m; it nods to +x. Its hairs stand up off the stem, long in its lower half, and their ends hang */
const PLUME = 0.22;
function plume(seed: number, level: number): Cluster {
  const r = rng(2500 + seed), leaves: Leaf[] = [], stem = (k: number): V3 => [0.26 * k * k + 0.10 * k * k * k, 2.0 * k - 0.10 * k * k, 0], push = (a: V3, b: V3, tone: number, k = 1, up = 0.2) => { const d = sub(b, a), t = norm(d), q = r() * 3.14; leaves.push({ at: mul(add(a, b), 0.5), t, n: face([Math.cos(q), up, Math.sin(q)], t), size: Math.hypot(...d) / 2 * k, tone: Math.max(0, Math.min(1, tone)) }); };
  if (level > 1) { const n = level === 2 ? 24 : 12, long = level === 2 ? 0.98 : 1.38; for (let i = 0; i < n; i++) { const k = level === 2 ? 0.58 * (i + 0.5) / n : 0.30 * (i + 0.5) / n, a = i * 2.39996 + 0.5 * r(), o: V3 = [Math.cos(a) + 0.22, 0, Math.sin(a)], from = stem(k), top = stem(Math.min(1, k + long / 2)), l = long * (0.88 + 0.2 * r()), d = norm(add(sub(top, from), mul(o, (level === 2 ? 0.27 : 0.31) * (0.5 + 1.0 * r())))); push(add(from, mul(o, 0.035)), add(add(from, mul(o, 0.035)), mul(d, l)), 0.56 + 0.2 * (r() - 0.5) + 0.1 * k, 1, 2.2); } return { kind: 1, leaves }; }
  const N = 168, keep = HAIRS[level]!; for (let i = 0; i < N; i++) { const k = 0.01 + 0.96 * Math.pow((i + r()) / N, 0.92), a = i * 2.39996 + r(), o: V3 = [Math.cos(a) + 0.22, 0, Math.sin(a)], long = (0.55 * Math.pow(Math.sin(Math.PI * Math.min(1, 0.14 + 0.80 * k)), 0.75) + 0.07) * (0.65 + 0.6 * r()), open = (0.34 - 0.13 * k) * (0.5 + 0.9 * r()), from = stem(k), d = norm(add(mul(o, open), [0.15 * k, 1, 0])), mid = add(from, mul(d, long * 0.50)), end = add(mid, mul(norm(add(d, add(mul(o, 0.30), [0, -0.25 - 0.6 * r(), 0]))), long * 0.52)), tone = 0.36 + 0.46 * r() + 0.16 * k; if (r() > keep) continue;
    if (level === 0) { push(from, mid, tone - 0.06, 1.10); push(add(mid, mul(sub(from, mid), 0.15)), end, tone + 0.06, 1.0); } else push(add(from, mul(sub(end, from), 0.02)), end, tone, 1.06); }
  return { kind: 1, leaves };
}
const kept = new WeakMap<World, Map<number, number[]>>(), grown = new Map<number, Cluster[]>();
/** the wedges, two skirts, three plumes */
const clusters = (w: World, level: number): number[] => { let m = kept.get(w); if (!m) kept.set(w, m = new Map()); let l = m.get(level); if (!l) { let g = grown.get(level); if (!g) grown.set(level, g = [...wedges.map(b => wedge(b, level)), ...skirts.map(b => wedge(b, level)), ...[0, 1, 2].map(i => plume(i, level))]); m.set(level, l = g.map(c => { w.clusters.push(c); return w.clusters.length - 1; })); } return l; };
/** a stalk: its points from the crown to where the plume begins, which way the plume leans */
interface Stalk { pts: V3[]; y: V3; nod: V3; scale: number; kind: number; phase: number; tint: V3 }
const STALKS: Stalk[] = (() => { const r = rng(31), N = 15; return Array.from({ length: N }, (_, i): Stalk => { const a = i * 2.39996 + 0.4 * (r() - 0.5), q = Math.sqrt((i + 0.5) / N), lean = 0.05 + 0.22 * q, o: V3 = [Math.cos(a), 0, Math.sin(a)], high = (1.56 - 0.24 * q) * (0.90 + 0.16 * r()), root: V3 = [o[0] * 0.10 * q, 0.04, o[2] * 0.10 * q], pts: V3[] = []; for (let k = 0; k <= 3; k++) { const t = k / 3; pts.push(add(root, [o[0] * high * lean * t * (0.7 + 0.3 * t), high * t, o[2] * high * lean * t * (0.7 + 0.3 * t)])); } return { pts, y: norm(sub(pts[3]!, pts[2]!)), nod: norm([o[0] + 0.6, 0, o[2] + 0.2]), scale: PLUME * (0.86 + 0.28 * r()), kind: i % 3, phase: r() * 6.28, tint: mul(mix(SILVER, [0.82, 0.74, 0.70], r()), 0.95 + 0.08 * r()) }; }); })();
const BRIGHT = [1, 0.98, 0.995, 1.035], PALE = [1, 0.97, 0.97, 1.02];
function put(p: Put, level: number) {
  const ix = clusters(p.w, level), dice = rng(p.seed * 23 + 9), mat = M['lamp shade']!, v = BRIGHT[level]!;
  const spread = (n: number, first: number, kinds: number, turn: number, tint: V3, sway: number) => Array.from({ length: n }, (_, i) => { const a = turn + i * 6.28318 / n, o: V3 = [Math.cos(a), 0, Math.sin(a)], z: V3 = [0, 1, 0], x = cross(o, z), at: V3 = [o[0] * 0.95 * SCALE, CROWN, o[2] * 0.95 * SCALE], k = v * (0.93 + 0.14 * dice()); return { cluster: ix[first + (i * 3) % kinds]!, at, rot: [x, o, z] as Rot, scale: SCALE, base: add(at, [0, -0.6, 0]), sway: sway * (0.9 + 0.2 * dice()), phase: dice() * 6.28, tint: mul(tint, k) }; });
  p.from({ pieces: [], instances: [...spread(WEDGES, 0, KINDS, 0.2, GREEN, 0.055), ...spread(6, KINDS, 2, 0.6, STRAW, 0.04),
    ...STALKS.map(s => { const at = s.pts[3]!, x = norm(sub(s.nod, mul(s.y, dot(s.nod, s.y)))), k = PALE[level]! * (0.97 + 0.06 * dice()); return { cluster: ix[KINDS + 2 + s.kind]!, at, rot: [x, s.y, cross(x, s.y)] as Rot, scale: s.scale, base: add(at, [0, -0.32, 0]), sway: 0.10, phase: s.phase, tint: mul(s.tint, k) }; })] });
  // the crown they come from, and the stalks
  p.bar([0, 0, 0], [0, CROWN, 0], 0.17, 0.11, M.soil!, [1.6, 1.4, 1.1]); for (const s of STALKS) { const top = add(s.pts[3]!, mul(s.y, s.scale * (level < 2 ? 1.1 : 0.4))), q = level === 0 ? [...s.pts, top] : level === 1 ? [s.pts[0]!, s.pts[2]!, top] : [s.pts[0]!, top]; for (let i = 0; i + 1 < q.length; i++) { const a = 0.0052 - 0.0030 * i / (q.length - 1), b = 0.0052 - 0.0030 * (i + 1) / (q.length - 1); p.bar(q[i]!, q[i + 1]!, a, b, mat, CULM); if (level === 0 && i && i < 3) p.ball(q[i]!, [a * 1.25, a * 1.1, a * 1.25], mat, mul(CULM, 0.92)); } }
}
export const reeds: ObjectKind = { name: 'reeds', title: 'Pampas grass', make: (): Made => ({
  name: 'reeds', title: 'Pampas grass', size: [2.0, 2.12, 2.0], on: 'floor', close: { at: [0.1, 1.5, 0.1], far: 3.6 }, read: 0, moves: { kind: 'sways' }, crowd: 49,
  levels: [
    { shape: 'leaves', error: 0, less: '', put(p) { put(p, 0); } },
    { shape: 'leaves', error: 0.021, less: 'a third of the blades, two of five hairs', put(p) { put(p, 1); } },
    { shape: 'leaves', error: 0.042, less: 'more than half the blades; a plume is 24 long hairs', put(p) { put(p, 2); } },
    { shape: 'leaves', error: 0.080, less: 'three blades of four; a plume is twelve long hairs', put(p) { put(p, 3); } },
  ],
}) };
