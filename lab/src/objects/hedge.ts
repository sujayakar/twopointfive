// TWO METRES OF CLIPPED BOX HEDGE: stems in a row that fork into a thicket; a shell of small leaves, in crossed pairs up short shoots, cut to a soft-edged block by the shears; dark twigs and old leaves within.
// Patches of shell share a frame: +y out of the hedge, the shears' cut at y = CUT. Coarser: fewer leaves, each larger, still under the cut.
import { type V3, type Rot, add, sub, mul, norm, cross, dot, rng, rotY, apply, compose } from '../math';
import type { Cluster, Leaf, Piece, World } from '../scene/world';
import { M } from '../scene/materials';
import type { Made, ObjectKind, Put } from './object';
/** how long, high and deep it is cut, how round its edges are, how far up its skirt begins; a patch's size; the cut, in a patch */
const LONG = 2.0, HIGH = 0.86, DEEP = 0.56, ROUND = 0.045, FOOT = 0.035, S = 0.125, CUT = 0.30;
const DARK: V3 = [0.192, 0.286, 0.084], YOUNG: V3 = [0.288, 0.386, 0.106], OLD: V3 = [0.16, 0.22, 0.075], WOOD: V3 = [0.62, 0.58, 0.50];
const KINDS = 8, face = (n: V3, t: V3): V3 => norm(sub(n, mul(t, dot(n, t))));
/** fewer leaves, each larger: one of `keep`, `big` times as long; what would stand out over the cut is let down under it */
interface Grain { keep: number; big: number; twigs: boolean; bright: number; flat: number; over: number; wide: number }
const GRAINS: Grain[] = [
  { keep: 1, big: 1, twigs: true, bright: 1, flat: 0, over: 0.07, wide: 9 },
  { keep: 3, big: 1.66, twigs: false, bright: 0.965, flat: 0.25, over: 0.10, wide: 1.0 },
  { keep: 12, big: 3.45, twigs: false, bright: 0.895, flat: 0.5, over: 0.10, wide: 0.95 },
];
/** a patch of shell: shoots that rise to the cut, a pair of leaves every centimetre or so, each pair across the last; +z is the side that is up */
function patch(seed: number, g: Grain): Cluster {
  const r = rng(6100 + seed * 13), leaves: Leaf[] = [], shoots = 8; let n = 0;
  for (let i = 0; i < shoots; i++) {
    const a = i * 2.39996 + 0.5 * r() + seed, far = 0.70 * Math.sqrt((i + 0.4) / shoots), lean = 0.14 + 0.16 * r(), b = a + 1.6 * (r() - 0.5), d = norm([Math.cos(b) * lean, 1, Math.sin(b) * lean + 0.12]), top = CUT - 0.05 + 0.07 * r() + (r() < 0.035 ? 0.08 + 0.12 * r() : 0), stray = top > CUT + 0.05, foot: V3 = [Math.cos(a) * far - d[0] * 0.4, -0.58 + 0.25 * r(), Math.sin(a) * far - d[2] * 0.4], long = (top - foot[1]) / d[1], pairs = stray ? 4 : 3, x = norm(cross(d, [0.3, 0.1, 1])), z = cross(x, d), twist = r() * 3.14;
    if (g.twigs && i % 4 === 1) for (const [p, q] of [[0.1, 0.95]] as const) { const t = d, at = add(foot, mul(d, long * (p + q) / 2)); leaves.push({ at, t, n: face([t[1], -t[0], 0.4], t), size: long * (q - p) / 2 * 1.02, tone: 0.2 + 0.4 * r(), wood: true }); }
    for (let j = 0; j < pairs; j++) for (const s of [0, 1]) {
      const k = 0.46 + 0.54 * Math.pow((j + 0.35 * r()) / (pairs - 0.65), 0.85), w = twist + j * 1.5708 + s * 3.1416 + (r() - 0.5) * 0.7, o = add(mul(x, Math.cos(w)), mul(z, Math.sin(w))), up = stray ? 0.3 + 0.8 * k * k : 0.16 + 0.42 * k * k, t = norm(add(add(o, mul(d, up * (0.7 + 0.6 * r()))), [0, 0, 0.10])), size = (0.113 + 0.028 * r()) * (j === pairs - 1 ? 0.80 : 1), at0 = add(add(foot, mul(d, long * k)), mul(t, size * 0.92)), n0 = face(add(add(mul(d, 0.9), mul(o, -0.5 * up)), [(r() - 0.5) * 0.7, 0.25, 0.55 + (r() - 0.5) * 0.7]), t), tone = Math.min(1, Math.max(0, -0.16 + 0.70 * k * k + 0.40 * r() + (j === pairs - 1 ? 0.12 : 0))), cut = r();
      if (at0[1] + t[1] * size > CUT + 0.15 && cut < 0.7 && !stray) continue; // (the shears had it)
      const mine = stray && g.keep === 3 ? 2 : g.keep; if (n++ % mine !== (seed * 5) % mine) continue;
      const big = size * (stray && g.keep === 3 ? 1.35 : g.big), tf = norm([t[0], t[1] * (1 - g.flat), t[2]]), nf = face(add(n0, [0, g.flat * 0.5, 0]), tf), side = cross(nf, tf), reach = Math.abs(tf[1]) * big + 0.4 * Math.abs(side[1]) * big * 0.46; const rad = Math.hypot(at0[0], at0[2]), in1 = g.keep === 1 ? 1 : Math.min(1, (g.wide - 0.8 * big) / rad); leaves.push({ at: [at0[0] * in1, stray && g.keep < 5 ? at0[1] : Math.min(at0[1], CUT + g.over - reach), at0[2] * in1], t: tf, n: nf, size: big, tone });
    }
  }
  // behind them old dark leaves, broader, that close it
  let m = 0; for (let i = 0; i < 14; i++) { const a = i * 2.39996 + 0.8 * r(), far = 0.74 * Math.sqrt((i + 0.5) / 14), w = r() * 6.28, t = norm([Math.cos(w), 0.35 * (r() - 0.3), Math.sin(w)]), n1 = face([0.9 * (r() - 0.5), 1, 0.9 * (r() - 0.5)], t), y = -0.02 - 0.34 * r(), size = 0.165 + 0.04 * r(); if (m++ % g.keep === (seed * 3) % Math.min(g.keep, 12)) leaves.push({ at: [Math.cos(a) * far, y, Math.sin(a) * far], t, n: n1, size: size * Math.min(g.big, 2.6), tone: 0.06 + 0.24 * r() }); }
  return { kind: 0, leaves: leaves.length > 1 ? leaves : [...leaves, ...leaves] };
}
/** along the edges within, where one could see through: old dark leaves and twigs, any way */
function thicket(seed: number, g: Grain): Cluster {
  const r = rng(6900 + seed), leaves: Leaf[] = [], way = (): V3 => norm([r() - 0.5, r() - 0.5, r() - 0.5]); let n = 0;
  for (let i = 0; i < 18; i++) { const at = mul(way(), Math.cbrt(r()) * 0.72), t = way(), m = face(way(), t), size = 0.42 + 0.12 * r(); if (n++ % Math.min(g.keep, 4) === 0) leaves.push({ at: mul(at, 1 / (1 + 0.5 * (Math.min(g.big, 1.7) - 1))), t, n: m, size: size * Math.min(g.big, 1.7), tone: 0.1 * r() }); }
  if (g.twigs) for (let i = 0; i < 4; i++) { const t = norm(add(way(), [0, 0.6, 0])); leaves.push({ at: mul(way(), 0.5 * r()), t, n: face(way(), t), size: 0.5 + 0.4 * r(), tone: 0.2 + 0.5 * r(), wood: true }); }
  return { kind: 0, leaves };
}
/** where a patch is: on the cut, which way is out, which way is up in it; how much light it gets in a year */
interface Place { at: V3; y: V3; up: V3; lit: number; big: number }
const PLACES: Place[] = (() => {
  const out: Place[] = [], r = rng(811), hx = LONG / 2 - ROUND, hz = DEEP / 2 - ROUND, top = HIGH - ROUND, low = FOOT + 0.075, step = 0.128;
  const put = (inner: V3, n: V3, big = 1, deep = 0) => { const y0 = norm(add(n, [(r() - 0.5) * 0.10, (r() - 0.5) * 0.10, (r() - 0.5) * 0.10])), on = add(inner, mul(n, ROUND - deep - 0.004 + 0.008 * r())), lean = 1 + 0.05 * (0.5 - on[1] / HIGH), up = face([(r() - 0.5) * 0.9, 1, (r() - 0.5) * 0.9], y0); out.push({ at: [on[0] * lean, on[1], on[2] * lean], y: y0, up, lit: Math.max(0, n[1]) * 0.55 + 0.45 * on[1] / HIGH, big }); };
  /** rows over a face of the inner block, every other row shifted */
  const rows = (o0: V3, u0: V3, v0: V3, n: V3) => { const ku = 0.022 / Math.hypot(...u0), kv = 0.022 / Math.hypot(...v0), o = add(o0, add(mul(u0, ku), mul(v0, kv))), u = mul(u0, 1 - 2 * ku), v = mul(v0, 1 - 2 * kv), lu = Math.hypot(...u), lv = Math.hypot(...v), N = Math.round(lv / (step * 0.9)); for (let j = 0; j <= N; j++) { const m = Math.round(lu / step), odd = j % 2; for (let i = 0; i <= m - odd; i++) put(add(add(o, mul(u, Math.min(1, Math.max(0, (i + odd * 0.5 + 0.35 * (r() - 0.5)) / m)))), mul(v, Math.min(1, Math.max(0, (j + 0.35 * (r() - 0.5)) / N)))), n); } };
  for (const s of [1, -1]) { rows([-hx, low, s * hz], [2 * hx, 0, 0], [0, top - low, 0], [0, 0, s]); rows([s * hx, low, -hz], [0, 0, 2 * hz], [0, top - low, 0], [s, 0, 0]); } rows([-hx, top, -hz], [2 * hx, 0, 0], [0, 0, 2 * hz], [0, 1, 0]);
  const edge = (a: V3, b: V3, n: V3) => { const l = Math.hypot(...sub(b, a)), m = Math.round(l / (step * 0.76)); for (let i = 0; i <= m; i++) put(add(a, mul(sub(b, a), Math.min(1, Math.max(0, (i + 0.4 * (r() - 0.5)) / m)))), norm(n), 0.72, 0.014); };
  for (const s of [1, -1]) { edge([-hx, top, s * hz], [hx, top, s * hz], [0, 1, s]); edge([s * hx, top, -hz], [s * hx, top, hz], [s, 1, 0]); for (const q of [1, -1]) { edge([s * hx, low, q * hz], [s * hx, top, q * hz], [s, 0, q]); put([s * hx, top, q * hz], norm([s, 1, q]), 0.7, 0.018); } }
  return out;
})();
const WITHIN: V3[] = (() => { const r = rng(77), out: V3[] = []; for (let x = -0.89; x <= 0.90; x += 0.148) for (const z of [-0.185, 0.185]) out.push([x, 0.10 + 0.02 * r(), z], [x, 0.745, z * 0.95]); for (const x of [-0.895, 0.895]) { for (const z of [-0.18, 0.18]) for (const y of [0.26, 0.42, 0.58]) out.push([x, y, z]); out.push([x, 0.745, 0], [x, 0.11, 0]); } return out; })();
const grown = new Map<number, Cluster[]>(), kept = new WeakMap<World, Map<number, number[]>>();
const clusters = (w: World, level: number): number[] => { let m = kept.get(w); if (!m) kept.set(w, m = new Map()); let l = m.get(level); if (!l) { let g = grown.get(level); if (!g) grown.set(level, g = [...Array.from({ length: KINDS }, (_, i) => patch(i, GRAINS[level]!)), thicket(1, GRAINS[level]!), thicket(2, GRAINS[level]!)]); m.set(level, l = g.map(c => { w.clusters.push(c); return w.clusters.length - 1; })); } return l; };
/** stems in a row, each forks and forks again */
function wood(p: Put, level: number, yaw: Rot) {
  const r = rng(p.seed * 7 + 400), held = (v: V3): V3 => [Math.max(-0.945, Math.min(0.945, v[0])), Math.min(0.79, v[1]), Math.max(-0.215, Math.min(0.215, v[2]))], bar = (a0: V3, b0: V3, ra: number, rb: number) => { const a = held(a0), b = held(b0); if (Math.hypot(...sub(b, a)) < 0.02) return; const q = p.bar(apply(yaw, a), apply(yaw, b), ra, rb, M.bark!, WOOD); if (Math.min(a[1], b[1]) < 0.02) q.planes = [[[-q.rot[0][1], -q.rot[1][1], -q.rot[2][1]], q.at[1] - p.at[1]]] as Piece['planes']; };
  for (let i = 0; i < 10; i++) { const x = -0.92 + 0.2044 * i + 0.05 * (r() - 0.5), z = 0.06 * (r() - 0.5), fork: V3 = [x + 0.05 * (r() - 0.5), 0.17 + 0.08 * r(), z + 0.03 * (r() - 0.5)], n = level > 1 ? 1 : 5, a0 = r() * 6.28; bar([x, 0, z], fork, 0.013 + 0.005 * r(), 0.010); if (level > 2) continue;
    for (let j = 0; j < n; j++) { const a = a0 + j * 1.2566 + 0.5 * (r() - 0.5), to: V3 = [fork[0] + 0.10 * Math.cos(a) + (i === 0 ? -0.04 : i === 9 ? 0.04 : 0), fork[1] + (j % 2 ? 0.12 : 0.28) + 0.12 * r(), fork[2] + 0.14 * Math.sin(a)]; bar(fork, to, 0.0075, 0.005); if (level > 0) continue; for (let k = 0; k < 3; k++) { const b = a + (k - 1) * 0.9 + 0.4 * (r() - 0.5), up = k === 1 ? 0.24 : 0.05 + 0.10 * r(); bar(add(fork, mul(sub(to, fork), 0.55 + 0.42 * r() * (k === 1 ? 0.2 : 1))), add(to, [0.09 * Math.cos(b), up, 0.11 * Math.sin(b)]), 0.0042, 0.0025); } }
    if (level === 0) for (const s of [1, -1]) bar([x, 0.045 + 0.04 * r(), z], [x + (i === 0 ? -0.02 : i === 9 ? 0.02 : 0.08 * (r() - 0.5)), 0.10 + 0.05 * r(), s * 0.19], 0.0055, 0.003); }
}
function put(p: Put, level: number) {
  const ix = clusters(p.w, level), g = GRAINS[level]!, dice = rng(p.seed * 41 + 3), half = p.seed % 2 ? 0 : Math.PI, yaw = rotY(half);
  p.from({ pieces: [], instances: [...PLACES.map((q, i) => { const v = 0.90 + 0.20 * dice(), young = Math.min(1, Math.max(0, q.lit * 0.95 + 0.30 * (dice() - 0.5) - 0.12)), x = norm(cross(q.y, q.up)), R: Rot = [x, q.y, cross(x, q.y)], at = sub(q.at, mul(q.y, (CUT + 0.07) * S * q.big)), tint = [0, 1, 2].map(k => (DARK[k]! + (YOUNG[k]! - DARK[k]!) * young) * v * g.bright) as V3, A = apply(yaw, at); return { cluster: ix[Math.floor(dice() * KINDS)]!, at: A, rot: compose(yaw, R), scale: S * q.big, base: [A[0], A[1] - 0.50, A[2]] as V3, sway: 0.030 + 0.012 * dice(), phase: dice() * 6.28, tint }; }),
    ...WITHIN.map((at, i) => { const A = apply(yaw, at); return { cluster: ix[KINDS + i % 2]!, at: A, rot: rotY(dice() * 6.28), scale: 0.092, base: [A[0], A[1] - 0.4, A[2]] as V3, sway: 0.02, phase: dice() * 6.28, tint: mul(OLD, g.bright) }; })] });
  wood(p, level, yaw);
  p.box([0, 0.385, 0], [0.85, 0.32, 0.118], M.bark!, [0.35, 0.36, 0.33]); // (the thicket of bare twigs within, as dark as it is: only its darkness is ever seen)
}
export const hedge: ObjectKind = { name: 'hedge', title: 'Box hedge', make: (): Made => ({
  name: 'hedge', title: 'Box hedge', size: [2.10, 0.90, 0.65], on: 'floor', close: { at: [0.55, 0.62, 0.25], far: 2.2 }, read: 0, moves: { kind: 'sways' }, crowd: 36,
  levels: [
    { shape: 'leaves', error: 0, less: '', put(p) { put(p, 0); } },
    { shape: 'leaves', error: 0.029, less: 'two leaves of three (the third is longer), twigs', put(p) { put(p, 1); } },
    { shape: 'leaves', error: 0.065, less: 'eleven of twelve, most of the wood', put(p) { put(p, 2); } },
  ],
}) };
