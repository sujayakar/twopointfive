// A YOUNG BIRCH: one white stem that wanders a little, marked with dark scars and lenticels; thin limbs that rise steeply and bow over, and from them fine twigs that hang, with small pointed leaves that show the sky between them.
// Coarser: twigs and bark's marks go first; then leaves, fewer and larger, and the thin limbs.
import { type V3, type Rot, add, sub, mul, norm, cross, dot, rng } from '../math';
import type { Cluster, Leaf, World } from '../scene/world';
import { M } from '../scene/materials';
import type { Made, ObjectKind, Put } from './object';
const H = 4.85, WHITE: V3 = [0.885, 0.880, 0.885], TWIG: V3 = [0.44, 0.335, 0.28], DARK: V3 = [0.25, 0.23, 0.225];
/** the root of an old leaf's color and of a young one's */
const OLD: V3 = [0.222, 0.318, 0.088], YOUNG: V3 = [0.318, 0.385, 0.112];
const turned = (v: V3, axis: V3, a: number): V3 => { const c = Math.cos(a), s = Math.sin(a); return add(add(mul(v, c), mul(cross(axis, v), s)), mul(axis, dot(axis, v) * (1 - c))); };
const square = (n: V3, t: V3): V3 => norm(sub(n, mul(t, dot(n, t))));

// ---- a spray: the end of a limb. It grows along +y from −0.95 and falls over toward −x (x is up); twigs to both sides fall more than it does; leaves hang from them, each its own way
interface Kind { fall: number; seed: number; twigs: number; long: number }
const KINDS: Kind[] = [{ fall: 0.75, seed: 1, twigs: 8, long: 1.75 }, { fall: 1.05, seed: 2, twigs: 9, long: 1.80 }, { fall: 0.45, seed: 3, twigs: 8, long: 1.65 }, { fall: 0.15, seed: 4, twigs: 9, long: 1.70 }, { fall: 1.25, seed: 5, twigs: 8, long: 1.70 }];
/** one leaf: where its stalk is, its long way and face, half its length */
interface Blade { at: V3; t: V3; n: V3; size: number; tone: number }
interface Grew { blades: Blade[]; wood: Leaf[] }
function spray(k: Kind): Grew {
  const r = rng(4400 + k.seed), blades: Blade[] = [], twigs: Leaf[] = [];
  const wood = (a: V3, b: V3) => { const d = sub(b, a), t = norm(d); twigs.push({ at: mul(add(a, b), 0.5), t, n: square([0.4, t[0], t[1] + 0.1], t), size: Math.hypot(...d) / 2 * 1.05, tone: r(), wood: true }); };
  const leaf = (p: V3, d: V3, side: V3, size: number, tone: number) => { const t = norm(add(add(mul(d, 0.60), mul(side, 0.75)), [-0.42 + (r() - 0.5) * 0.9, (r() - 0.5) * 0.8, (r() - 0.5) * 0.8])), n = square([0.62 + (r() - 0.5) * 1.1, 0.25 + (r() - 0.5) * 1.3, (r() - 0.5) * 1.3], t); blades.push({ at: add(p, mul(t, size * 0.35)), n, t, size, tone }); };
  /** a way through space that falls over as it goes */
  const way = (from: V3, d0: V3, long: number, fall: number, n: number) => { const at: V3[] = [from], ds: V3[] = []; let d = norm(d0); for (let i = 0; i < n; i++) { ds.push(d); at.push(add(at[i]!, mul(d, long / n))); const axis = cross(d, [-1, 0, 0]), l = Math.hypot(...axis); if (l > 1e-3) d = norm(turned(d, mul(axis, 1 / l), Math.min(fall / n, Math.asin(Math.min(1, l)) + (d[0] > 0 ? 3 : 0)))); } ds.push(d); return (q: number) => { const x = Math.min(n - 1e-4, Math.max(0, q * n)), i = Math.floor(x); return { at: add(mul(at[i]!, 1 - x + i), mul(at[i + 1]!, x - i)), d: ds[i]! }; }; };
  const main = way([0, -0.95, 0], [0.04, 1, (r() - 0.5) * 0.15], k.long, k.fall, 14), across = (d: V3, a: number): V3 => { const u = norm(cross(d, [1, 0, 0.001])), v = cross(u, d); return add(mul(u, Math.cos(a)), mul(v, Math.sin(a))); }; for (let i = 0; i < 6; i++) wood(main(i / 6 * 0.98).at, main((i + 1) / 6 * 0.98).at);
  for (let i = 0; i < k.twigs; i++) { const q = 0.14 + 0.74 * (i + 0.3 + 0.4 * r()) / k.twigs, m = main(q), d = norm(add(mul(m.d, 0.85), mul(across(m.d, (i % 2 ? 0 : Math.PI) + (r() - 0.5) * 1.9), 0.55 + 0.35 * r()))), long = (0.66 - 0.32 * q) * (0.75 + 0.5 * r()), w = way(m.at, d, long, 0.5 + 0.8 * r(), 5), n = Math.round(2.5 + long * 7.5); wood(w(0).at, w(0.5).at); wood(w(0.5).at, w(0.98).at);
    for (let j = 0; j < n; j++) { const u = 0.16 + 0.84 * (j + 0.6) / n, x = w(u), end = j === n - 1; leaf(x.at, x.d, end ? [0, 0, 0] : across(x.d, (j % 2 ? 0 : Math.PI) + (r() - 0.5) * 1.4), (0.066 + 0.022 * r()) * (end ? 0.8 : 1), 0.22 + 0.45 * r() + 0.2 * u * q); } }
  for (let j = 0; j < 8; j++) { const u = 0.28 + 0.72 * (j + 0.5) / 8, x = main(u); leaf(x.at, x.d, j === 7 ? [0, 0, 0] : across(x.d, (j % 2 ? 0 : Math.PI) + (r() - 0.5)), 0.066 + 0.020 * r(), 0.3 + 0.45 * r()); }
  return { blades, wood: twigs };
}
/** a leaf is broad at its foot and ends in a point: two ovals that share the point. Coarser, one; then one for `keep` that stand together on a twig, in their midst */
function cluster(g: Grew, keep: number, twigs: boolean, big: number): Cluster {
  const leaves: Leaf[] = twigs ? [...g.wood] : [];
  if (keep === 0) for (const b of g.blades) for (const s of [-1, 1]) { const side = cross(b.n, b.t), t = norm(add(b.t, mul(side, s * 0.27))), tip = add(b.at, mul(b.t, b.size * 2)); leaves.push({ at: sub(tip, mul(t, b.size)), t, n: square(add(b.n, mul(side, s * 0.16)), t), size: b.size, tone: b.tone }); }
  else for (let i = 0; i < g.blades.length; i += keep) { const of = g.blades.slice(i, i + keep), one = of[of.length >> 1]!, mids = of.map(b => add(b.at, mul(b.t, b.size * 0.92))), mid = mul(mids.reduce((a, b) => add(a, b), [0, 0, 0] as V3), 1 / of.length), own = add(one.at, mul(one.t, one.size * 0.92)); leaves.push({ at: add(mul(mid, 0.6), mul(own, 0.4)), t: one.t, n: one.n, size: Math.sqrt(of.reduce((a, b) => a + b.size * b.size, 0)) * 1.22 * big, tone: of.reduce((a, b) => a + b.tone, 0) / of.length }); }
  return { kind: 0, leaves };
}

// ---- the tree, in numbers: made once
interface Bar { a: V3; b: V3; ra: number; rb: number; tint: V3; kind: 'stem' | 'limb' | 'branch' | 'twig' }
interface Spray { root: V3; y: V3; kind: number; scale: number; light: number; roll: number }
interface Grown { bars: Bar[]; sprays: Spray[]; stem: (y: number) => { at: V3; r: number }; scars: { y: number; a: number }[] }
function grow(seed: number): Grown {
  const r = rng(9100 + seed), bars: Bar[] = [], sprays: Spray[] = [], scars: Grown['scars'] = [], p1 = r() * 6.28, p2 = r() * 6.28;
  /** the stem wanders, less than its own thickness over a metre */
  const stem = (y: number) => { const k = y / H; return { at: [0.055 * (Math.sin(1.35 * y + p1) - Math.sin(p1)) * Math.min(1, y), y, 0.045 * (Math.sin(1.10 * y + p2) - Math.sin(p2)) * Math.min(1, y)] as V3, r: 0.0055 + 0.041 * Math.pow(1 - k, 0.92) }; };
  const N = 22; for (let i = 0; i < N; i++) { const y0 = H * i / N, y1 = H * (i + 1) / N, a = stem(y0), b = stem(y1), k = i / N, v = 0.965 + 0.07 * r(), warm = r(); const pale: V3 = mul([WHITE[0], WHITE[1] - 0.018 * warm, WHITE[2] - 0.04 * warm], v), t = Math.max(0, Math.min(1, (k - 0.60) / 0.22)), foot = Math.max(0, 1 - i / 1.4) * 0.22; bars.push({ a: i ? a.at : [a.at[0], 0, a.at[2]], b: b.at, ra: a.r, rb: b.r, tint: [0, 1, 2].map(c => (pale[c]! * (1 - t) + TWIG[c]! * t) * (1 - foot)) as V3, kind: 'stem' }); }
  const ribs = 34, spin = r() * 6.28;
  for (let i = 0; i < ribs; i++) {
    const k = (i + 0.3 + 0.4 * r()) / ribs, y = 0.92 + (H - 1.35) * Math.pow(k, 0.93), from = stem(y), a = spin + i * 2.39996 + (r() - 0.5) * 0.5, out: V3 = [Math.cos(a), 0, Math.sin(a)], side: V3 = [-Math.sin(a), 0, Math.cos(a)];
    // long below the middle, short at the top; the lowest a little shorter again
    const long = (0.30 + 0.86 * Math.pow(1 - k, 0.78) * (0.60 + 0.40 * Math.min(1, k * 4.5))) * (0.85 + 0.3 * r()), rise = 0.92 + 0.34 * k + (r() - 0.5) * 0.40, n = long > 0.9 ? 6 : long > 0.55 ? 4 : 3, r0 = Math.min(from.r * 0.52, 0.0045 + 0.0078 * long), bend = (r() - 0.5) * 0.5; scars.push({ y, a });
    let p: V3 = add(from.at, mul(out, from.r * 0.4)), d: V3 = norm(add(mul(out, Math.cos(rise)), [0, Math.sin(rise), 0])); const pts: V3[] = [p], ds: V3[] = [];
    for (let j = 0; j < n; j++) { const u = j / n; p = add(p, mul(d, long / n)); pts.push(p); ds.push(d); const down = (0.62 + 0.28 * (1 - k)) * (0.35 + 1.0 * u) / n * 1.5; d = norm(add(add(d, mul([0, -1, 0], down)), mul(side, bend / n))); }
    for (let j = 0; j < n; j++) { const q0 = j / n, q1 = (j + 1) / n, ra = r0 + (0.0022 - r0) * q0, rb = r0 + (0.0022 - r0) * q1, w = Math.max(0, Math.min(1, (ra - 0.0055) / 0.006)); bars.push({ a: pts[j]!, b: pts[j + 1]!, ra, rb, tint: [0, 1, 2].map(c => TWIG[c]! + (WHITE[c]! * 0.90 - TWIG[c]!) * w) as V3, kind: ra > 0.007 ? 'limb' : 'branch' }); }
    // sprays: one at its end, others up its length to both sides and above, each on a short twig
    const light = Math.max(0, Math.min(1, 0.15 + 0.65 * k)), many = Math.max(1, Math.round(long * 4.2));
    const put = (root: V3, y0: V3, s: number, l: number) => { const y = norm(y0); sprays.push({ root, y, kind: y[1] > 0.75 ? 3 : y[1] < -0.2 ? 2 : [0, 1, 4, 0, 1][Math.floor(r() * 5)]!, scale: s, light: l, roll: (r() - 0.5) * 1.3 }); };
    if (long > 0.6) for (let j = 0; j < 2; j++) { const x = n * (0.62 + 0.33 * (j + r()) / 2), e = Math.min(n - 1, Math.floor(x)), at = add(mul(pts[e]!, 1 - x + e), mul(pts[e + 1]!, x - e)); put(at, add(mul(ds[e]!, 0.30), [(r() - 0.5) * 0.4, -0.85, (r() - 0.5) * 0.4]), 0.24 + 0.08 * r(), light - 0.1); }
    put(pts[n]!, add(d, [0, 0.05, 0]), 0.30 + 0.06 * r(), light + 0.15);
    for (let j = 0; j < many; j++) { const q = 0.20 + 0.72 * (j + 0.2 + 0.6 * r()) / many, x = q * n, e = Math.min(n - 1, Math.floor(x)), at = add(mul(pts[e]!, 1 - x + e), mul(pts[e + 1]!, x - e)), s = j % 2 ? 1 : -1, y = norm(add(add(mul(ds[e]!, 0.55), mul(side, s * (0.65 + 0.5 * r()))), [0, -0.10 + 0.5 * (r() - 0.4), 0])), far = 0.10 + 0.12 * r(), tip = add(at, mul(y, far)); bars.push({ a: at, b: tip, ra: 0.0036, rb: 0.0024, tint: TWIG, kind: 'twig' }); put(tip, add(y, [0, -0.06, 0]), 0.27 + 0.08 * r(), light + 0.2 * (r() - 0.5)); }
  }
  // the leader, and what stands round it
  for (let i = 0; i < 4; i++) { const y = H - 0.42 + i * 0.14, from = stem(Math.min(H, y)), a = i * 2.4 + spin, o: V3 = [Math.cos(a), 0, Math.sin(a)]; sprays.push({ root: from.at, y: norm(i === 3 ? [0.05, 1, 0.04] : add(mul(o, 0.55), [0, 0.85, 0])), kind: 3, scale: i === 3 ? 0.25 : 0.27, light: 0.95, roll: 0 }); }
  return { bars, sprays, stem, scars };
}
const grown = new Map<number, Grown>(), treeOf = (seed: number): Grown => { let g = grown.get(seed); if (!g) grown.set(seed, g = grow(seed)); return g; };
// ---- levels
interface Step { keep: number; twigs: boolean; big: number; bright: number; thin: 'twig' | 'branch' | 'limb' | 'stem'; every: number }
const STEPS: Step[] = [{ keep: 0, twigs: true, big: 1, bright: 1, thin: 'twig', every: 1 }, { keep: 1, twigs: false, big: 1.04, bright: 0.965, thin: 'twig', every: 1 }, { keep: 4, twigs: false, big: 1.045, bright: 0.95, thin: 'branch', every: 1 }, { keep: 12, twigs: false, big: 1.10, bright: 0.93, thin: 'limb', every: 3 }];
const ORDER = ['twig', 'branch', 'limb', 'stem'], sprayKinds = KINDS.map(spray), kept = new WeakMap<World, Map<number, number[]>>();
const clusters = (w: World, level: number): number[] => { let m = kept.get(w); if (!m) kept.set(w, m = new Map()); let l = m.get(level); if (!l) { const s = STEPS[level]!; m.set(level, l = sprayKinds.map(c => { w.clusters.push(cluster(c, s.keep, s.twigs, s.big)); return w.clusters.length - 1; })); } return l; };
/** what makes it a birch close by: a dark moustache under every limb, lenticels that lie across, bands where the bark peels, dark cracks at the foot */
function bark(p: Put, g: Grown) {
  const r = rng(p.seed * 53 + 11), mat = M['lamp shade']!, on = (y: number, a: number, k: number): V3 => { const s = g.stem(y); return [s.at[0] + Math.cos(a) * s.r * k, y, s.at[2] + Math.sin(a) * s.r * k]; };
  /** a thin skin on the stem's round, w to both sides of the way a */
  const skin = (y: number, a: number, w: number, h: number, tint: V3, k = 1.014) => { const s = g.stem(y), R = s.r * k; p.drum(s.at, R, h, mat, tint, { planes: [[[-Math.cos(a), 0, -Math.sin(a)], -R * Math.cos(w)]] }); };
  for (const { y, a } of g.scars) { const R = g.stem(y).r; if (R < 0.014) continue; const w = 0.75 + 0.35 * r(), drop = R * (0.9 + 0.5 * r()), t = mul(DARK, 0.9 + 0.3 * r()); for (const s of [-1, 1]) for (let j = 0; j < 2; j++) p.bar(on(y + R * 0.75 - drop * j * 0.42, a + s * w * j / 2, 1.0), on(y + R * 0.75 - drop * (j ? 1 : 0.42), a + s * w * (j + 1) / 2, 1.0), R * [0.13, 0.10][j]!, R * [0.10, 0.03][j]!, mat, t); { const c = g.stem(y - R * 0.25), q = c.r * 1.02, h = R * (0.50 + 0.25 * r()), m = 0.55 + 0.35 * r(), e = Math.hypot(1, m), out: V3 = [Math.cos(a), 0, Math.sin(a)], T: V3 = [-Math.sin(a), 0, Math.cos(a)]; p.drum(c.at, q, h, mat, t, { planes: [[mul(out, -1), -q * 0.72], ...[-1, 1].map((s): [V3, number] => [[T[0] * s / e, m / e, T[2] * s / e], m * h / e])] }); } }
  for (let i = 0; i < 120; i++) { const y = 0.10 + 2.9 * Math.pow(r(), 1.1), big = r(); skin(y, r() * 6.2832, 0.14 + 0.62 * big * r(), 0.0013 + 0.0034 * big * big, mul([0.46, 0.39, 0.35], 0.80 + 0.45 * r())); }
  for (let i = 0; i < 12; i++) skin(0.30 + 2.6 * (i + r()) / 12, r() * 6.2832, 0.6 + 1.3 * r(), 0.005 + 0.016 * r() * r(), mul([0.84, 0.775, 0.715], 0.95 + 0.1 * r()), 1.009 + 0.001 * (i % 4));
  for (let i = 0; i < 16; i++) { const y = 0.015 + 0.16 * r() * r(), a = i * 2.39996 + r(), h = 0.03 + 0.11 * r(), k = 0.0025 + 0.003 * r(); p.bar(on(y, a, 0.98), on(y + h, a + 0.2 * (r() - 0.5), 0.975), k, k * 0.5, mat, mul(DARK, 0.8 + 0.4 * r())); }
}
function put(p: Put, level: number) {
  const s = STEPS[level]!, g = treeOf(p.seed), ix = clusters(p.w, level), dice = rng(p.seed * 71 + 5), mat = M['lamp shade']!;
  g.bars.forEach(b => { if (ORDER.indexOf(b.kind) < ORDER.indexOf(s.thin)) return; p.bar(b.a, b.b, b.ra, b.rb, mat, b.tint); });
  if (level === 0) bark(p, g);
  p.from({ pieces: [], instances: g.sprays.map(q => {
    const up: V3 = Math.abs(q.y[1]) > 0.96 ? norm([q.root[0] + 0.01, 0, q.root[2] + 0.02]) : [0, 1, 0], x = turned(square(up, q.y), q.y, q.roll), rot: Rot = [x, q.y, cross(x, q.y)], at = add(q.root, mul(q.y, 0.95 * q.scale)), v = 0.93 + 0.14 * dice(), k = Math.max(0, Math.min(1, q.light + 0.3 * (dice() - 0.5))), tint = [0, 1, 2].map(c => (OLD[c]! + (YOUNG[c]! - OLD[c]!) * k) * v * s.bright) as V3;
    // it swings about its root: what it is carried and how it turns undo each other there
    const lift = 2.76 * q.scale * (q.y[1] >= 0 ? Math.max(0.5, q.y[1]) : Math.min(-0.5, q.y[1])); return { cluster: ix[q.kind]!, at, rot, scale: q.scale, base: add(at, [0, -lift, 0]), sway: 0.10 + 0.05 * dice(), phase: dice() * 6.28, tint };
  }) });
}
export const birch: ObjectKind = { name: 'birch', title: 'Young birch', make: (): Made => ({
  name: 'birch', title: 'Young birch', size: [2.6, 5.3, 2.5], on: 'floor', close: { at: [0.25, 2.3, 0.4], far: 4 }, read: 0, moves: { kind: 'sways' }, crowd: 49,
  levels: [
    { shape: 'leaves', error: 0, less: '', put(p) { put(p, 0); } },
    { shape: 'leaves', error: 0.012, less: 'twigs, marks of the bark; a leaf is one oval', put(p) { put(p, 1); } },
    { shape: 'leaves', error: 0.03, less: 'the thinnest limbs; one leaf for four', put(p) { put(p, 2); } },
    { shape: 'leaves', error: 0.075, less: 'thin limbs; one leaf for twelve', put(p) { put(p, 3); } },
  ],
}) };
