// A SWISS CHEESE PLANT IN A POT: a thick stem tied to a stake, long stalks from it, and on each a blade as long as a forearm, cut from its edge almost to its rib, with holes along the rib; two young blades are whole and paler,
// one is still rolled. A blade is fingers: each a long oval from the rib with a forked blunt end; between two at the rib a filler leaves a hole, or none. Coarser: a finger is one oval; then a blade is nine; then four.
import { type V3, ID, add, sub, mul, norm, cross, dot, rng, mix } from '../math';
import type { Cluster, Leaf, World } from '../scene/world';
import { M } from '../scene/materials';
import type { Made, ObjectKind, Put } from './object';
const GREEN: V3 = [0.170, 0.278, 0.098], LATER: V3 = [0.205, 0.320, 0.105], YOUNG: V3 = [0.262, 0.400, 0.112], STALK: V3 = [0.40, 0.54, 0.27], STEM: V3 = [0.36, 0.43, 0.22], CLAY: V3 = [0.80, 0.79, 0.765], TOP = 0.265;
/** one blade: where its stalk joins the stem, where the blade begins, which way its tip and its face look, how long it is, how old (0 a whole young one, 1 full), how far it droops */
interface Blade { from: V3; at: V3; tip: V3; face: V3; long: number; age: number; droop: number; seed: number }
/** the stem leans a little as it climbs; each stalk leaves it on the other side of the one below, and round */
const stemAt = (k: number): V3 => [0.015 + 0.035 * Math.sin(k * 2.6), TOP - 0.01 + 0.40 * k, 0.01 + 0.03 * Math.sin(k * 3.4 + 1)];
const BLADES: Blade[] = (() => {
  const r = rng(77), out: Blade[] = [], N = 11; for (let i = 0; i < N; i++) {
    const k = i / (N - 1), a = 0.55 + i * 2.39996 + (r() - 0.5) * 0.4, o: V3 = [Math.cos(a), 0, Math.sin(a)], from = stemAt(0.08 + 0.92 * k), young = i >= N - 2, reach = (young ? 0.14 : 0.36 - 0.14 * k) * (0.92 + 0.16 * r()), up = (young ? 0.32 : 0.15 + 0.24 * k) * (0.9 + 0.2 * r()), at = add(from, [o[0] * reach, up, o[2] * reach]);
    // the face looks up and out, the tip hangs out and down: older leaves more
    const tilt = young ? 0.6 : 0.26 + 0.24 * (1 - k) + 0.16 * (r() - 0.5), side: V3 = [-o[2], 0, o[0]], yaw = (r() - 0.5) * 0.7, w: V3 = norm(add(mul(o, Math.cos(yaw)), mul(side, Math.sin(yaw)))), tip = norm(add(mul(w, Math.cos(tilt)), [0, -Math.sin(tilt), 0])), f0 = norm(add(add(mul(w, Math.sin(tilt)), [0, Math.cos(tilt), 0]), mul(side, (r() - 0.5) * 0.55))), face = norm(sub(f0, mul(tip, dot(f0, tip))));
    out.push({ from, at, tip, face, long: young ? (i === N - 1 ? 0.20 : 0.24) : (0.355 + 0.06 * Math.sin(k * 3.6)) * (0.94 + 0.12 * r()), age: young ? 0 : 1, droop: young ? 0.10 : 0.22 + 0.14 * r(), seed: i });
  } return out;
})();
/** where the fingers of a half begin on the rib and end at the edge, of a blade one long */
const FINGERS: [number, number, number][] = [[0.01, 0.235, -0.255], [0.025, 0.42, -0.135], [0.075, 0.49, 0.045], [0.17, 0.495, 0.235], [0.275, 0.455, 0.415], [0.39, 0.39, 0.575], [0.51, 0.295, 0.725], [0.635, 0.18, 0.86]];
/** a blade's leaves about its own beginning, one long: x to its side, y to its tip, z its face */
function blade(b: Blade, level: number): Leaf[] {
  const r = rng(500 + b.seed), leaves: Leaf[] = [], tone0 = 0.36 + 0.16 * r();
  /** the blade is no plane: it droops to its tip, rises a little from the rib and falls to its edges */
  const on = (x: number, y: number, h = 0): V3 => { const q = Math.abs(x); return [x * (1 - 0.10 * q), y * (1 - 0.10 * y * b.droop), 0.16 * q - (0.42 + 0.3 * b.droop) * q * q - b.droop * y * Math.abs(y) * 0.8 + h]; };
  const oval = (x0: number, y0: number, x1: number, y1: number, h: number, tone: number, k = 1) => { const a = on(x0, y0, h), c = on(x1, y1, h), m = on((x0 + x1) / 2, (y0 + y1) / 2, h), d = sub(c, a), t = norm(d), sx = -(y1 - y0), sy = x1 - x0, l = Math.hypot(sx, sy) || 1, e = 0.05, s = sub(on((x0 + x1) / 2 + sx / l * e, (y0 + y1) / 2 + sy / l * e, h), on((x0 + x1) / 2 - sx / l * e, (y0 + y1) / 2 - sy / l * e, h)); let n = norm(cross(t, s)); if (n[2] < 0) n = mul(n, -1); leaves.push({ at: add(mul(add(a, c), 0.3), mul(m, 0.4)), t, n: norm(sub(n, mul(t, dot(n, t)))), size: Math.hypot(...d) / 2 * k, tone: Math.max(0, Math.min(1, tone)) }); };
  const jit = () => (r() - 0.5) * 2;
  if (!b.age) { // a young blade, whole: a heart
    for (const s of [-1, 1]) { oval(0, 1.0, s * 0.215, -0.23, 0.002 + 0.002 * s, tone0 + 0.05 * jit(), level > 2 ? 1.1 : 1.0); if (level < 3) { oval(s * 0.02, -0.06, s * 0.385, -0.17, 0.005, tone0, 1.1); oval(s * 0.08, 0.02, s * 0.42, 0.22, 0.006, tone0, 1.05); } } if (level < 2) oval(0, -0.05, 0, 0.85, 0.008, tone0, 0.9); return leaves;
  }
  for (const s of [-1, 1]) FINGERS.forEach(([y0, ex, ey], i) => {
    const wob = 1 + 0.06 * jit(), x1 = ex * wob, y1 = ey + 0.02 * jit(), P = (k: number, across = 0): [number, number] => { const dx = x1, dy = y1 - y0, l = Math.hypot(dx, dy); /* it bends to the tip as it goes */ return [s * (dx * k + across * -dy / l - 0.02 * k * k), y0 + dy * k + across * dx / l + 0.05 * k * k * (i ? 1 : 0.3)]; }, tone = tone0 + 0.10 * jit(), h = (i % 2) * 0.003;
    if (level === 0) {
      const fork = 0.019 + 0.004 * jit(), cut = 0.04 * jit(); oval(...P(0.09), ...P(0.59), h, tone, 1.02); oval(...P(0.28 + cut), ...P(0.82), h + 0.003, tone + 0.02 * jit(), 1.03); oval(...P(0.50, -0.010), ...P(0.99, -fork), h + 0.005, tone + 0.02 * jit(), 1.0); oval(...P(0.52, 0.010), ...P(0.99, fork), h + 0.006, tone + 0.02 * jit(), 1.0);
    } else if (level === 1) { oval(...P(0.08), ...P(0.64), h, tone, 1.02); oval(...P(0.36), ...P(1.0), h + 0.004, tone, 1.02); }
    else if (level === 2) { oval(...P(0.02), ...P(0.66), h, tone, 1.065); if (i > 0 && i < 7) oval(...P(0.38), ...P(1.0), h + 0.004, tone, 1.065); }
    else oval(...P(0.02), ...P(1.0), h, tone, 0.893);
    // toward the next finger: past a hole by the rib, or from the rib
    if (i < FINGERS.length - 1) { const [ny, nx, nyy] = FINGERS[i + 1]!, g = r(), hole = g < 0.7 && i > 0, m0 = (y0 + ny) / 2, f0 = hole ? 0.31 + 0.04 * jit() : 0.0, f = hole ? f0 + 0.20 : 0.46, X = s * (x1 + nx) / 2, Y = (y1 + nyy) / 2 - m0; if (level < 2) oval(X * f0, m0 + Y * f0, X * f, m0 + Y * f, -0.002, tone + 0.03 * jit(), 1.1); }
  });
  // along the rib, and the tip
  const n = [6, 4, 2, 1][level]!, long = [0.23, 0.31, 0.46, 0.8][level]!; for (let i = 0; i < n; i++) { const a = -0.09 + (0.75 - (level > 1 ? long - 0.23 : 0)) * i / Math.max(1, n - 0.6); oval(0, a, 0, a + long, 0.008, tone0 + 0.03 * jit(), [0.94, 0.9, 0.9, 0.7][level]!); } oval(0, 0.58, 0, 1.0, 0.009, tone0 + 0.02, 1.0); if (!level) for (const s of [-1, 1]) oval(s * 0.01, 0.66, s * 0.045, 0.97, 0.007, tone0 + 0.02, 1.0);
  return leaves;
}
/** a blade nods on its knee: the spray's foot is there, its middle straight above (so that what it turns about is where it hangs) */
const wide = (b: Blade) => b.long * 1.42, MIDDLE = 0.95;
const cluster = (b: Blade, level: number): Cluster => { const side = cross(b.tip, b.face), turn = (v: V3): V3 => add(add(mul(side, v[0]), mul(b.tip, v[1])), mul(b.face, v[2])), k = wide(b); return { kind: 0, leaves: blade(b, level).map(l => ({ ...l, at: sub(mul(turn(l.at), b.long / k), [0, MIDDLE, 0]), n: turn(l.n), t: turn(l.t), size: l.size * b.long / k })) }; };
const kept = new WeakMap<World, Map<number, number[]>>(), grown = new Map<number, Cluster[]>(), BRIGHT = [1, 1, 1, 1];
const clusters = (w: World, level: number): number[] => { let m = kept.get(w); if (!m) kept.set(w, m = new Map()); let l = m.get(level); if (!l) { let g = grown.get(level); if (!g) grown.set(level, g = BLADES.map(b => cluster(b, level))); m.set(level, l = g.map(c => { w.clusters.push(c); return w.clusters.length - 1; })); } return l; };
function wood(p: Put, level: number) {
  const mat = M['lamp shade']!, round = (pts: V3[], r0: number, r1: number, tint: V3, balls: boolean) => { for (let i = 0; i + 1 < pts.length; i++) { const a = r0 + (r1 - r0) * i / (pts.length - 1), b = r0 + (r1 - r0) * (i + 1) / (pts.length - 1); p.bar(pts[i]!, pts[i + 1]!, a, b, mat, tint); if (balls && i) p.ball(pts[i]!, [a, a, a], mat, tint); } };
  // the pot, its rim, its earth
  p.bar([0, 0, 0], [0, 0.275, 0], 0.118, 0.152, M.plastic!, CLAY); if (level < 2) { p.bar([0, 0.262, 0], [0, 0.292, 0], 0.157, 0.159, M.plastic!, CLAY); } p.drum([0, level < 2 ? 0.284 : 0.268, 0], 0.146, 0.0095, M.soil!, [1.1, 1.1, 1.1]);
  // the stake, the stem
  if (level < 3) { p.drum([0.002, 0.28 + 0.22, -0.034], 0.021, 0.25, M.canvas!, [0.44, 0.38, 0.27]); }
  const n = level < 2 ? 6 : 2; round(Array.from({ length: n + 1 }, (_, i) => stemAt(i / n)), 0.019, 0.013, STEM, level === 0);
  // a stalk: it leaves the stem along it, bends out, and has a knee where the blade begins
  for (const b of BLADES) { if (level > 2) { p.bar(add(b.from, [0, -0.03, 0]), b.at, 0.0062, 0.0045, mat, STALK); continue; } const N = level === 0 ? 6 : level === 1 ? 3 : 2, d = sub(b.at, b.from), pts: V3[] = []; for (let i = 0; i <= N; i++) { const t = i / N, bow = Math.sin(Math.PI * t) * 0.16; pts.push(add(b.from, [d[0] * t - d[0] * bow, d[1] * t + Math.hypot(d[0], d[2]) * bow - 0.02 * (1 - t), d[2] * t - d[2] * bow])); } round(pts, b.age ? 0.0072 : 0.0055, b.age ? 0.0046 : 0.0036, b.age ? STALK : mul(STALK, 1.15), level === 0); if (level === 0) { p.ball(sub(b.at, mul(b.face, 0.006)), [0.0062, 0.0062, 0.0062], mat, STALK); } }
  // the next blade, still rolled; and roots that hang in the air
  if (level < 2) { const a = stemAt(1), c = add(a, [0.02, 0.17, -0.012]), e = add(c, [0.012, 0.15, 0.006]); p.bar(a, c, 0.006, 0.0075, mat, mul(STALK, 1.1)); p.bar(c, e, 0.0085, 0.0022, mat, [0.55, 0.68, 0.30]); }
  if (level === 0) for (const [k, x, z, l] of [[0.22, 0.05, 0.03, 0.10], [0.40, -0.045, 0.035, 0.15], [0.55, 0.03, -0.05, 0.12]] as const) { const a = stemAt(k), b = add(a, [x, -0.01, z]), c = add(b, [x * 0.35, -l, z * 0.35]); round([a, b, c], 0.0036, 0.0022, [0.42, 0.33, 0.23], true); }
}
function put(p: Put, level: number) {
  const ix = clusters(p.w, level), dice = rng(p.seed * 13 + 5);
  p.from({ pieces: [], instances: BLADES.map((b, k) => { const v = (0.95 + 0.10 * dice()) * BRIGHT[level]!, up = wide(b) * MIDDLE, at = add(b.at, [0, up, 0]); return { cluster: ix[k]!, at, rot: ID, scale: wide(b), base: add(b.at, [0, -1.9 * up, 0]), sway: 0.035 + 0.015 * dice(), phase: dice() * 6.28, tint: mul(b.age ? mix(GREEN, LATER, k / BLADES.length) : YOUNG, v) }; }) });
  wood(p, level);
}
export const monstera: ObjectKind = { name: 'monstera', title: 'Swiss cheese plant in a pot', make: (): Made => ({
  name: 'monstera', title: 'Swiss cheese plant in a pot', size: [1.16, 1.01, 1.24], on: 'floor', close: { at: [0.02, 0.72, 0.05], far: 3.4 }, read: 0, moves: { kind: 'sways' }, crowd: 64,
  levels: [
    { shape: 'leaves', error: 0, less: '', put(p) { put(p, 0); } },
    { shape: 'leaves', error: 0.012, less: 'holes; a finger is two ovals, its end whole; the stem\'s joints', put(p) { put(p, 1); } },
    { shape: 'leaves', error: 0.030, less: 'cuts: a blade is ten ovals; roots, the rolled one', put(p) { put(p, 2); } },
    { shape: 'leaves', error: 0.060, less: 'a blade is five ovals; stalks are straight', put(p) { put(p, 3); } },
  ],
}) };
