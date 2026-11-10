// A YOUNG SCOTS PINE, some thirteen years old: a whorl of branches for each year up a straight stem, each branch a node for each year too, with a side branch to the left and right of it. Needles stay a few
// years, so they stand only on the outer, younger wood, as bottle brushes that point up; within it is open, and one sees the stem, grey below and orange above.
// A shoot of one year is an instance: needles round a twig, from its foot (the instance's place) along +y. Coarser: needles are fewer and broader, till a shoot is two of them; thin wood goes first and leaves its color.
import { type V3, type Rot, add, sub, mul, norm, cross, dot, rng, rotY, apply, compose } from '../math';
import type { Cluster, Leaf, Piece, World } from '../scene/world';
import { M } from '../scene/materials';
import type { Made, ObjectKind, Put } from './object';
const S = (oval: boolean) => oval ? 0.29 : 0.168, GREEN: V3 = [0.308, 0.386, 0.246];
const across = (d: V3, a: number): V3 => { const x = norm(cross(d, Math.abs(d[1]) < 0.9 ? [0, 1, 0] : [1, 0, 0])), z = cross(x, d); return add(mul(x, Math.cos(a)), mul(z, Math.sin(a))); };
const face = (n: V3, t: V3): V3 => norm(sub(n, mul(t, dot(n, t))));
/** how long the kinds of shoot are, metres; a few of each */
const LONGS = [0.10, 0.155, 0.215, 0.29], EACH = 3;
interface Grain { needles: number; oval: boolean; long: number; twig: boolean; bright: number; /** the wood that went leaves its color */ brown: number }
const GRAINS: Grain[] = [
  { needles: 1, oval: false, long: 1, twig: true, bright: 1, brown: 1 },
  { needles: 4.4, oval: true, long: 0.768, twig: false, bright: 0.945, brown: 1.015 },
  { needles: 8.4, oval: true, long: 1.065, twig: false, bright: 0.94, brown: 1.04 },
  { needles: 30, oval: true, long: 1.45, twig: false, bright: 0.935, brown: 1.09 },
];
/** a shoot of one year: pairs of needles in a spiral, the lower spread wide, the upper close about the bud */
function shoot(kind: number, seed: number, g: Grain): Cluster {
  const r = rng(5200 + kind * 17 + seed), z0 = S(g.oval), long = LONGS[kind]!, n0 = Math.round(15 + 114 * long), leaves: Leaf[] = [], bend = across([0, 1, 0], r() * 6.28), at = (k: number): V3 => add([0, long * k, 0], mul(bend, 0.06 * long * k * k)), put = (p: V3, t: V3, n: V3, size: number, tone: number, wood = false) => leaves.push({ at: mul(p, 1 / z0), t, n: face(n, t), size: size / z0, tone, ...(wood ? { wood } : {}) });
  if (g.twig) { for (const [a, b] of [[-0.06, 0.5], [0.5, 1.02]] as const) { const p = at(a), q = at(b), t = norm(sub(q, p)); put(mul(add(p, q), 0.5), t, across(t, 1 + seed), Math.hypot(...sub(q, p)) / 2 * 1.04, 0.75 + 0.2 * r(), true); } }
  const few = Math.max(2, Math.round(n0 / g.needles));
  for (let i = 0; i < (g.needles === 1 ? n0 : few); i++) { const k = (i + 0.5 * r() + 0.25) / (g.needles === 1 ? n0 : few), a = i * 2.39996 + (r() - 0.5) * 0.5, o = across([0, 1, 0], a), tip = k > 0.86 ? (k - 0.86) / 0.14 : 0, spread = (0.96 - 0.30 * k - 0.45 * tip) * (0.82 + 0.36 * r()), t = norm(add(mul(o, Math.sin(spread)), [0, Math.cos(spread), 0])), size = (0.0490 - 0.006 * tip) * (0.82 + 0.34 * r()) * (kind === 0 ? 0.86 : 1), tone = 0.30 + 0.5 * r(), n = add(across(t, r() * 6.28), [0, 0.3, 0]); if (g.oval) { const z = size * g.long; put(add(at(k), mul(t, size * 0.98)), t, n, z, 0.55); } else put(add(at(k), mul(t, size * 0.98)), t, n, size, tone); }
  return { kind: g.oval ? 0 : 1, leaves };
}
/** a shoot as it stands: its foot, which way, what kind, how old, whose branch; how deep in the crown */
interface Grown { at: V3; y: V3; kind: number; age: number; branch: number; lit: number }
interface Wood { a: V3; b: V3; ra: number; rb: number; what: 0 | 1 | 2; high: number }
interface Tree { shoots: Grown[]; wood: Wood[]; top: number }
const trees = new Map<number, Tree>();
function grow(seed: number): Tree {
  const had = trees.get(seed); if (had) return had; const r = rng(seed * 911 + 17), shoots: Grown[] = [], wood: Wood[] = [], YEARS = [0.12, 0.18, 0.24, 0.30, 0.36, 0.40, 0.43, 0.45, 0.47, 0.47, 0.44, 0.40, 0.34], N = YEARS.length; let branch = 0;
  const kindOf = (l: number) => { let best = 0; LONGS.forEach((v, i) => { if (Math.abs(v - l) < Math.abs(LONGS[best]! - l)) best = i; }); return best; };
  // the stem: a hair out of true from year to year
  const stem: V3[] = [[0, 0, 0]]; { let d: V3 = [0.012, 1, 0.01]; for (const y of YEARS) { d = norm(add(d, [0.03 * (r() - 0.5), 0.5, 0.03 * (r() - 0.5)])); stem.push(add(stem[stem.length - 1]!, mul(d, y * (1.0 + 0.08 * r())))); } } const high = stem[N]![1], thick = (y: number) => 0.0055 + 0.043 * Math.pow(Math.max(0, 1 - y / high), 0.92);
  for (let i = 0; i < N; i++) wood.push({ a: stem[i]!, b: stem[i + 1]!, ra: thick(stem[i]![1]) * (i ? 1 : 1.18), rb: thick(stem[i + 1]![1]), what: 0, high: stem[i]![1] / high });
  /** an axis: grown for `age` years from `from` toward d, `long` in all, rising by `rise`; its needles on the last `green` years; its side branches */
  const axis = (from: V3, d0: V3, age: number, long: number, order: number, r0: number, rise: number, b: number) => {
    const w = Array.from({ length: age }, (_, s) => (1.22 - 0.42 * s / Math.max(1, age - 1)) * (0.85 + 0.3 * r())), all = w.reduce((p, q) => p + q, 0), green = 2; let at = from, d = d0;
    for (let s = 0; s < age; s++) { const l = long * w[s]! / all, left = age - s, r1 = r0 * (1 - 0.72 * s / age), r2 = r0 * (1 - 0.72 * (s + 1) / age); d = norm(add(d, [0.10 * (r() - 0.5), rise * (s > age - 4 ? 0.22 : s === 0 ? 0 : 0.05), 0.10 * (r() - 0.5)])); const to = add(at, mul(d, l));
      if (left <= green) shoots.push({ at, y: d, kind: kindOf(l), age: left, branch: b, lit: Math.min(1, Math.hypot(to[0], to[2]) / 1.1) }); if (left > 1 || order === 0 && age > 1 && s < age - 1) wood.push({ a: at, b: to, ra: Math.max(0.0028, r1), rb: Math.max(0.0024, r2), what: order ? 2 : 1, high: at[1] / high });
      at = to; const j = left - 1; if (!j || order > 1) continue;
      // side branches of this node: to the left and right, now and then one above; those near the stem of old branches have died of shade
      if (order === 0 && j >= 6 && r() < 0.7) continue; if (order === 1 && (j > 1 || age < 3)) continue; const n = order === 0 ? (r() < 0.22 ? 3 : 2) : 1, right = norm(cross(d, [0, 1, 0])), over = cross(right, d);
      for (let q = 0; q < n; q++) { const side = q === 2 ? 0 : (q ? 1 : -1) * (order === 1 && n === 1 && s % 2 ? -1 : 1), open = order === 0 ? 0.86 + 0.2 * r() : 0.72 + 0.2 * r(), out = norm(add(add(mul(d, Math.cos(open)), mul(right, side * Math.sin(open))), mul(over, q === 2 ? 0.55 : 0.10 + 0.22 * r()))), rest = long * w.slice(s + 1).reduce((p, v) => p + v, 0) / all; const ll = order === 0 ? rest * (0.56 + 0.2 * r()) : Math.min(rest * 0.7, 0.10 + 0.06 * r()); if (ll < 0.05) continue; axis(at, out, order === 0 ? j : 1, ll, order + 1, Math.max(0.0028, r2 * 0.55), rise * 0.8, b); } }
  };
  const REACH = [0, 0.28, 0.53, 0.76, 0.95, 1.10, 1.20, 1.26, 1.28, 1.22, 1.02], TILT = [0, 0.62, 0.80, 0.95, 1.08, 1.18, 1.26, 1.33, 1.40, 1.46, 1.52];
  for (let i = 3; i < N; i++) { const age = N - i, at = stem[i]!, n = age > 8 ? 4 : 5 + (r() < 0.3 ? 1 : 0), a0 = r() * 6.28; for (let k = 0; k < n; k++) { const a = a0 + (k + 0.34 * (r() - 0.5)) * 6.2832 / n, tilt = TILT[age]! + 0.16 * (r() - 0.5), d = norm([Math.sin(tilt) * Math.cos(a), Math.cos(tilt), Math.sin(tilt) * Math.sin(a)]), y = at[1] - 0.012 * k; axis([at[0], y, at[2]], d, age, REACH[age]! * (0.84 + 0.30 * r()), 0, thick(y) * 0.22 + 0.0025, 0.55 + 0.45 * Math.min(1, age / 6), branch++); } }
  // the leader: this year's and last year's shoots, and the crown of young ones round its bud
  shoots.push({ at: stem[N - 1]!, y: norm(sub(stem[N]!, stem[N - 1]!)), kind: 3, age: 1, branch, lit: 1 }, { at: add(stem[N - 1]!, mul(sub(stem[N]!, stem[N - 1]!), 0.28)), y: norm(sub(stem[N]!, stem[N - 1]!)), kind: 3, age: 1, branch, lit: 1 }, { at: add(stem[N - 2]!, mul(sub(stem[N - 1]!, stem[N - 2]!), 0.35)), y: norm(sub(stem[N - 1]!, stem[N - 2]!)), kind: 3, age: 2, branch, lit: 1 }, { at: stem[N - 2]!, y: norm(sub(stem[N - 1]!, stem[N - 2]!)), kind: 3, age: 2, branch, lit: 1 }); const made = { shoots, wood, top: high }; trees.set(seed, made); return made;
}
const kept = new WeakMap<World, Map<number, number[]>>(), kinds = new Map<number, Cluster[]>();
const clusters = (w: World, level: number): number[] => { let m = kept.get(w); if (!m) kept.set(w, m = new Map()); let l = m.get(level); if (!l) { let g = kinds.get(level); if (!g) kinds.set(level, g = LONGS.flatMap((_, k) => Array.from({ length: EACH }, (_, s) => shoot(k, s, GRAINS[level]!)))); m.set(level, l = g.map(c => { w.clusters.push(c); return w.clusters.length - 1; })); } return l; };
const frame = (y: V3, a: number): Rot => { const x = across(y, a); return [x, y, cross(x, y)]; };
function put(p: Put, level: number) {
  const t = grow(p.seed), ix = clusters(p.w, level), g = GRAINS[level]!, dice = rng(p.seed * 53 + 9), yaw = p.seed === 1 ? 0 : p.seed * 1.3, about = rotY(yaw);
  const phases = new Map<number, number>(), phase = (b: number) => { let v = phases.get(b); if (v === undefined) phases.set(b, v = (b * 2.39996) % 6.28); return v; };
  p.from({ pieces: [], instances: t.shoots.map(s => { const v = 0.94 + 0.12 * dice(), a = dice() * 6.28, pick = Math.floor(dice() * EACH), warm = s.age === 1 ? [1.07, 1.05, 0.98] : s.age === 2 ? [0.98, 0.98, 1.0] : [0.99, 0.92, 0.88]; const at = apply(about, s.at), k = (0.82 + 0.18 * s.lit) * v * g.bright * (g.oval ? Math.sqrt(0.7) : 1); return { cluster: ix[s.kind * EACH + pick]!, at, rot: compose(about, frame(s.y, a)), scale: S(g.oval), base: [at[0], at[1] - 1, at[2]] as V3, sway: 0.028 + 0.006 * at[1], phase: phase(s.branch), tint: [GREEN[0] * warm[0]! * k * g.brown, GREEN[1] * warm[1]! * k, GREEN[2] * warm[2]! * k * Math.sqrt(g.brown)] as V3 }; }) });
  const thin = [0, 0.0036, 0.0054, 0.012][level]!;
  for (const w of t.wood) { if (Math.max(w.ra, w.rb) < thin) continue; const a = apply(about, w.a), b = apply(about, w.b), up = Math.min(1, Math.max(0, (w.what ? 0.25 + w.high * 0.75 : w.high * 1.35 - 0.12))), young = Math.max(0, (w.high - 0.80) / 0.2), tint: V3 = [0.55 + 0.19 * up - 0.15 * young, 0.545 + 0.06 * up - 0.01 * young, 0.55 - 0.045 * up], q = p.bar(a, b, w.ra, w.rb, M.paleBark!, w.what === 2 ? mul(tint, 0.88) : tint); if (w.a[1] < 0.02) q.planes = [[[-q.rot[0][1], -q.rot[1][1], -q.rot[2][1]], q.at[1] - p.at[1]]] as Piece['planes']; if (w.what < 2 && level < 2 && w.rb > 0.006) p.ball(b, [w.rb * 1.04, w.rb * 1.04, w.rb * 1.04], M.paleBark!, tint); }
}
export const pine: ObjectKind = { name: 'pine', title: 'Young pine', make: (): Made => ({
  name: 'pine', title: 'Young pine', size: [2.4, 4.88, 2.45], on: 'floor', close: { at: [0.65, 2.5, 0.95], far: 3.6 }, read: 0, moves: { kind: 'sways' }, crowd: 16,
  levels: [
    { shape: 'leaves', error: 0, less: '', put(p) { put(p, 0); } },
    { shape: 'leaves', error: 0.02, less: 'three needles of four (the fourth is broad), twigs', put(p) { put(p, 1); } },
    { shape: 'leaves', error: 0.05, less: 'two broad leaves of three; thin wood', put(p) { put(p, 2); } },
    { shape: 'leaves', error: 0.12, less: 'a shoot is two leaves; all wood but the stem and the thick of the branches', put(p) { put(p, 3); } },
  ],
}) };
