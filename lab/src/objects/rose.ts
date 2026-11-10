// A SHRUB ROSE IN FLOWER: canes that rise from one crown and arch out, shoots off them with leaves of five leaflets, and at the ends of the upper shoots double flowers with their buds. Petals lie along the rim of a cup, so that
// its edge is round. Coarser: a leaf is three leaflets and a flower ten petals; then a leaf is one and a flower four; then a shoot is four leaves and a flower two.
import { type V3, type Rot, ID, add, sub, mul, norm, cross, dot, rng, mix } from '../math';
import type { Cluster, Leaf, World } from '../scene/world';
import { M } from '../scene/materials';
import type { Made, ObjectKind, Put } from './object';
const OLD: V3 = [0.182, 0.285, 0.112], NEW: V3 = [0.262, 0.352, 0.118], BRONZE: V3 = [0.335, 0.262, 0.135], PINK: V3 = [0.640, 0.345, 0.420], BUD: V3 = [0.600, 0.240, 0.320], BARK: V3 = [0.40, 0.35, 0.25], CANE: V3 = [0.37, 0.42, 0.22];
const face = (n: V3, t: V3): V3 => norm(sub(n, mul(t, dot(n, t))));
/** a shoot grows along +y from −0.95: its stem, and leaves by turns round it, each of five leaflets on a stalk that arches out; 0.16 m a unit */
function shoot(seed: number, level: number, flowers: boolean): Cluster {
  const r = rng(1300 + seed), leaves: Leaf[] = [], lean: V3 = [(r() - 0.5) * 0.5, 0, (r() - 0.5) * 0.5], top = flowers ? 0.92 : 0.75, stem = (k: number): V3 => [lean[0] * k * (1 - 0.5 * k), -0.95 + (top + 0.95) * k, lean[2] * k * (1 - 0.5 * k)];
  const put = (at: V3, t: V3, n: V3, size: number, tone: number, wood = false) => leaves.push({ at, t, n: face(n, t), size, tone, ...(wood ? { wood } : {}) }), stick = (a: V3, b: V3) => { const d = sub(b, a), t = norm(d); put(mul(add(a, b), 0.5), t, [t[1], -t[0] + 0.01, 0.3], Math.hypot(...d) / 2 * 1.05, 0.5 + 0.3 * r(), true); };
  if (level < 2) for (let i = 0; i < 3; i++) stick(stem(i / 3), stem((i + 1) / 3));
  const N = flowers ? 8 : 9; for (let i = 0; i < N; i++) {
    const k = 0.10 + (flowers ? 0.84 : 0.90) * (i + 0.5) / N, from = stem(k), a = i * 2.39996 + (r() - 0.5) * 0.7, o: V3 = [Math.cos(a), 0, Math.sin(a)], side: V3 = [-o[2], 0, o[0]], young = k > 0.8, long = (young ? 0.46 : 0.74) * (0.85 + 0.3 * r()), rise = 0.55 + 0.5 * k + 0.2 * (r() - 0.5), hang = 0.55 + 0.35 * r(), tone0 = 0.22 + 0.38 * k + 0.16 * r(), tw = (r() - 0.5) * 0.9;
    /** along the stalk */
    const at = (q: number): V3 => add(from, add(mul(o, long * q * (1 - 0.12 * q)), [0, long * (rise * q * 0.72 - hang * q * q * 0.75), 0])), way = (q: number): V3 => norm(sub(at(q + 0.05), at(q - 0.05))), up = (q: number): V3 => { const d = way(q); return face(add([0, 1, 0], mul(side, tw)), d); };
    const leaflet = (q: number, s: number, size: number, dq = 0) => { const d = way(q), u = up(q), w = cross(u, d), t = s ? norm(add(add(mul(w, s * 0.92), mul(d, 0.50 + dq)), mul(u, -0.10 - 0.16 * r()))) : norm(add(d, mul(u, -0.12))), n = add(u, add(mul(w, s * (0.10 - 0.35 * r())), mul(d, 0.3 * (r() - 0.5)))); put(add(at(q), mul(t, size * 1.02)), t, n, size, Math.min(1, tone0 + 0.18 * (r() - 0.5))); };
    const z = young ? 0.72 : 1;
    if (level === 0) { stick(at(0), at(0.48)); stick(at(0.45), at(0.95)); leaflet(0.46, 1, 0.138 * z); leaflet(0.46, -1, 0.138 * z); leaflet(0.74, 1, 0.156 * z); leaflet(0.74, -1, 0.156 * z); leaflet(0.96, 0, 0.172 * z); }
    else if (level === 1) { leaflet(0.60, 1, 0.2025 * z, 0.1); leaflet(0.60, -1, 0.2025 * z, 0.1); leaflet(0.93, 0, 0.184 * z); }
    else if (level === 2) { const d = way(0.75), t = d, n = up(0.75); r(); put(at(0.84), t, n, 0.346 * z, tone0); }
    else if (i % 2 === 0) { const d = way(0.7), n = up(0.7); put(at(0.72), d, n, 0.492 * z, tone0 + 0.02); }
  }
  // what holds the flower: sepals, under it
  if (flowers && level === 0) { const c = stem(1); for (let i = 0; i < 5; i++) { const a = i * 1.2566, o: V3 = [Math.cos(a), 0, Math.sin(a)]; put(add(c, add(mul(o, 0.07), [0, -0.015, 0])), norm(add(o, [0, 0.25 - 0.5 * (i % 2), 0])), [0, 1, 0.01], 0.075, 0.4); } }
  return { kind: 0, leaves };
}
/** a flower: rings of petals, the outer ones flat and wide, the inner ones upright; a petal lies along its ring. 1 is its radius; it opens to +y. `open`: 1 full, 0 a cup */
function flower(seed: number, level: number, open: number): Cluster {
  const r = rng(1700 + seed), leaves: Leaf[] = [], ring = (n: number, at: number, y: number, tilt: number, size: number, tone: number, a0: number) => { for (let i = 0; i < n; i++) { const a = a0 + i * 6.2832 / n + (r() - 0.5) * 0.30, q = tilt + (r() - 0.5) * 0.22, o: V3 = [Math.cos(a), 0, Math.sin(a)], t: V3 = norm([-o[2], 0.12 * (r() - 0.5), o[0]]), w: V3 = add(mul(o, Math.cos(q)), [0, Math.sin(q), 0]), n0 = cross(t, w), n1 = n0[1] < 0 ? mul(n0, -1) : n0; leaves.push({ at: add(mul(o, at * (0.95 + 0.1 * r())), [0, y + 0.04 * (r() - 0.5), 0]), t, n: face(n1, t), size: size * (0.93 + 0.14 * r()), tone: Math.max(0, Math.min(1, tone + 0.16 * (r() - 0.5))) }); } };
  const c = 1 - open; // a cup is narrower and more upright
  if (level === 0) { ring(5, 0.66 - 0.16 * c, -0.10 + 0.14 * c, 0.02 + 0.75 * c, 0.56, 0.80, 0); ring(5, 0.36 - 0.06 * c, -0.20, 0.45 + 0.4 * c, 0.46, 0.60, 0); ring(5, 0.52 - 0.14 * c, 0.02 + 0.14 * c, 0.38 + 0.7 * c, 0.50, 0.72, 0.63); ring(4, 0.28, -0.08, 0.75, 0.36, 0.50, 0.3); ring(5, 0.38 - 0.10 * c, 0.12 + 0.12 * c, 0.75 + 0.5 * c, 0.40, 0.62, 0.2); ring(4, 0.25, 0.20 + 0.1 * c, 1.05 + 0.3 * c, 0.30, 0.50, 0.9); ring(3, 0.12, 0.23 + 0.1 * c, 1.25, 0.20, 0.40, 0.4); ring(2, 0.04, 0.20, 1.3, 0.16, 0.30, 0); }
  else if (level === 1) { ring(5, 0.52 - 0.14 * c, -0.07 + 0.12 * c, 0.16 + 0.75 * c, 0.66, 0.74, 0); ring(3, 0.30 - 0.06 * c, 0.10 + 0.1 * c, 0.75 + 0.4 * c, 0.56, 0.58, 0.5); ring(2, 0.10, 0.18, 1.1, 0.34, 0.45, 0); }
  else if (level === 2) { for (const s of [1, -1]) leaves.push({ at: [0, 0.02, s * 0.30], t: [1, 0, 0], n: face([0, 1, s * -0.30], [1, 0, 0]), size: 0.92 - 0.1 * c, tone: 0.60 }); leaves.push({ at: [0, 0.08, 0], t: [0, 0, 1], n: [0.95, 0.31, 0], size: 0.55, tone: 0.5 }); }
  else { leaves.push({ at: [0, 0.04, 0.26], t: [1, 0, 0], n: face([0, 1, -0.45], [1, 0, 0]), size: 1.0, tone: 0.58 }, { at: [0, 0.04, -0.26], t: [1, 0, 0], n: face([0, 1, 0.45], [1, 0, 0]), size: 1.0, tone: 0.58 }); }
  return { kind: 0, leaves };
}
/** a bud: petals wrapped to a point along +y, half a unit wide; and what lies on the ground */
function bud(level: number): Cluster { const leaves: Leaf[] = []; for (let i = 0; i < (level ? 2 : 4); i++) { const a = i * (level ? 1.571 : 1.5708), o: V3 = [Math.cos(a), 0, Math.sin(a)]; leaves.push({ at: level ? [0, 0.15, 0] : add(mul(o, 0.24), [0, 0.12, 0]), t: norm([-o[0] * (level ? 0 : 0.22), 1, -o[2] * (level ? 0 : 0.22)]), n: face(o, norm([-o[0] * 0.22, 1, -o[2] * 0.22])), size: level ? 1.05 : 0.85, tone: 0.5 }); } return { kind: 0, leaves: leaves.map(l => ({ ...l, n: face(l.n, l.t) })) }; }
function fallen(): Cluster { const r = rng(88), leaves: Leaf[] = []; for (let i = 0; i < 9; i++) { const a = r() * 6.28, d = 0.35 + 0.8 * r(), b = r() * 6.28; leaves.push({ at: [Math.cos(a) * d, 0.02 + (d < 0.95 ? (0.95 - d) * 0.14 : 0) - 0.03, Math.sin(a) * d], t: [Math.cos(b), 0, Math.sin(b)], n: face([0.15 * (r() - 0.5), 1, 0.15 * (r() - 0.5)], [Math.cos(b), 0, Math.sin(b)]), size: 0.15, tone: 0.4 + 0.5 * r() }); } return { kind: 0, leaves }; }
// ---- how this one grew: canes, what springs from them, where it flowers
interface Cane { pts: V3[]; r0: number; r1: number; old: boolean }
interface Sprig { foot: V3; y: V3; scale: number; kind: number; tint: V3; phase: number; sway: number; blooms: { at: V3; y: V3; scale: number; kind: number }[] }
const CANES: Cane[] = [], SPRIGS: Sprig[] = [], TWIGS: { a: V3; m: V3; b: V3; r: number }[] = [];
const MID: V3 = [0, 0.60, 0], RX = 0.50, RU = 0.44, RD = 0.42;
{
  const r = rng(4242), N = 9; for (let i = 0; i < N; i++) { const a = i * 2.39996 + 0.5 * (r() - 0.5), inner = i / (N - 1), out = 0.46 - 0.34 * inner + 0.06 * r(), high = 0.62 + 0.28 * inner + 0.06 * r(), o: V3 = [Math.cos(a), 0, Math.sin(a)], root: V3 = [o[0] * 0.035, 0.015, o[2] * 0.035], side: V3 = [-o[2], 0, o[0]], sw = (r() - 0.5) * 0.14, pts: V3[] = []; for (let k = 0; k <= 5; k++) { const t = k / 5; pts.push(add(root, add(add(mul(o, out * t * (0.55 + 0.45 * t)), mul(side, sw * Math.sin(t * 3.1))), [0, high * (t * (1.22 - 0.22 * t)), 0]))); } CANES.push({ pts, r0: 0.0085 - 0.002 * r(), r1: 0.0045, old: i % 3 === 0 }); }
  const near = (p: V3): V3 => { let best = 1e9, q: V3 = [0, 0, 0]; for (const c of CANES) for (let i = 1; i < c.pts.length; i++) for (let t = 0; t < 4; t++) { const x = add(mul(c.pts[i - 1]!, 1 - t / 4), mul(c.pts[i]!, t / 4)), e = sub(p, x), d = Math.hypot(...e) + Math.max(0, 0.06 - e[1]) * 1.5; if (d < best) { best = d; q = x; } } return q; };
  const S = 78; for (let i = 0; i < S; i++) {
    const deep = i % 5 === 4, v = 1 - (i + 0.5) / S * 1.72, q = Math.sqrt(Math.max(0, 1 - v * v)), a = i * 2.39996 + (r() - 0.5) * 0.5, d: V3 = [q * Math.cos(a), v, q * Math.sin(a)], k = deep ? 0.45 + 0.12 * r() : 0.77 + 0.32 * r() * r() + 0.03 * Math.sin(a * 3 + 1), mid = add(MID, [d[0] * RX * k, d[1] * (d[1] > 0 ? RU : RD) * k, d[2] * RX * k]), y = norm([d[0] * 0.9 + (r() - 0.5) * 0.4, 0.75 + 0.5 * Math.max(0, d[1]) + (r() - 0.5) * 0.3, d[2] * 0.9 + (r() - 0.5) * 0.4]), flowers = !deep && v > -0.28 && r() < (v > 0.1 ? 0.82 : 0.56), scale = (0.145 + 0.03 * r()) * (flowers ? 1.05 : 1), foot = sub(mid, mul(y, 0.95 * scale)), c = near(foot), kind = (flowers ? 3 : 0) + Math.floor(r() * 3);
    const tint0 = mix(OLD, NEW, Math.max(0, Math.min(1, (deep ? -0.1 : 0.15) + 0.55 * v + 0.35 * (r() - 0.5)))), tint = mul(!deep && !flowers && r() < 0.16 ? mix(tint0, BRONZE, 0.6) : tint0, 0.92 + 0.16 * r()), phase = r() * 6.28, sway = 0.065 + 0.03 * r(); TWIGS.push({ a: c, m: add(mul(add(c, foot), 0.5), [0, 0.02 + 0.03 * r(), 0]), b: foot, r: 0.0034 });
    const s: Sprig = { foot, y, scale, kind, tint, phase, sway, blooms: [] }; SPRIGS.push(s); if (!flowers) continue;
    // a flower at its end, one or two beside it, younger
    const tip = add(foot, mul(y, 1.87 * scale)), n = 1 + Math.floor(r() * 2.7); for (let b = 0; b < n; b++) { const w = r() * 6.28, e: V3 = norm(add(y, mul([Math.cos(w), 0.1, Math.sin(w)], b ? 1.0 : 0.35))), at = b ? add(tip, add(mul(e, 0.055), mul(y, -0.045 + 0.02 * r()))) : add(tip, mul(y, 0.012)), isBud = b === 2 || (b === 1 && r() < 0.35); s.blooms.push({ at, y: norm(add(e, mul([d[0], 0.35, d[2]], 0.5))), scale: isBud ? 0.0165 : (b ? 0.041 : 0.047) * (0.9 + 0.2 * r()), kind: isBud ? 4 : b ? 2 + Math.floor(r() * 2) : Math.floor(r() * 2.4) }); }
  }
}
const frame = (y: V3, turn: number): Rot => { const e = norm(cross(y, Math.abs(y[1]) < 0.95 ? [0, 1, 0] : [1, 0, 0])), f = cross(e, y), x = add(mul(e, Math.cos(turn)), mul(f, Math.sin(turn))); return [x, y, cross(x, y)]; };
const kept = new WeakMap<World, Map<number, number[]>>(), grown = new Map<number, Cluster[]>();
/** six shoots (three that flower), four flowers, a bud, what has fallen */
const clusters = (w: World, level: number): number[] => { let m = kept.get(w); if (!m) kept.set(w, m = new Map()); let l = m.get(level); if (!l) { let g = grown.get(level); if (!g) grown.set(level, g = [...[0, 1, 2, 3, 4, 5].map(i => shoot(i, level, i > 2)), flower(0, level, 1), flower(1, level, 0.85), flower(2, level, 0.5), flower(3, level, 0.15), bud(level), fallen()]); m.set(level, l = g.map(c => { w.clusters.push(c); return w.clusters.length - 1; })); } return l; };
const BRIGHT = [1, 0.985, 0.955, 0.94], BLOOM = [1, 0.99, 0.97, 0.96];
function put(p: Put, level: number) {
  const ix = clusters(p.w, level), dice = rng(p.seed * 19 + 7), mat = M['lamp shade']!;
  // (a flower goes with its shoot: it turns about the same foot by the same wind)
  p.from({ pieces: [], instances: SPRIGS.flatMap((s, i) => { const at = add(s.foot, mul(s.y, 0.95 * s.scale)), v = BRIGHT[level]! * (0.96 + 0.08 * dice()); return [{ cluster: ix[s.kind]!, at, rot: frame(s.y, i * 1.7), scale: s.scale, base: add(at, [0, -2.9 * (at[1] - s.foot[1]), 0]), sway: s.sway, phase: s.phase, tint: mul(s.tint, v) }, ...s.blooms.filter(b => level < 3 || b.kind < 4).map((b, j) => ({ cluster: ix[6 + b.kind]!, at: b.at, rot: frame(b.y, i + j * 2.1), scale: b.scale, base: add(b.at, [0, -2.9 * (b.at[1] - s.foot[1]), 0]), sway: s.sway, phase: s.phase, tint: mul(b.kind === 4 ? BUD : mix(PINK, BUD, b.kind > 1 ? 0.35 : 0), BLOOM[level]! * (0.95 + 0.08 * dice())) }))]; }) });
  if (level < 2) p.from({ pieces: [], instances: [{ cluster: ix[11]!, at: [0.01, 0.022, 0.02], rot: ID, scale: 0.15, base: [0.01, 0.022, 0.02], sway: 0, phase: 0, tint: mul(PINK, 0.95) }] });
  // the crown, its earth
  p.bar([0, 0, 0], [0, 0.034, 0], 0.155, 0.06, M.soil!, [1.1, 1.05, 1.0]); if (level < 2) { p.ball([0, 0.034, 0], [0.045, 0.022, 0.045], M.bark!, [0.95, 0.95, 0.95]); for (let i = 0; i < 7; i++) { const a = i * 2.39996, d = 0.10 + 0.035 * ((i * 0.618) % 1), q = 0.045 + 0.02 * ((i * 0.37) % 1); p.ball([Math.cos(a) * d, 0.006, Math.sin(a) * d], [q, 0.022, q * 0.8], M.soil!, [1.1, 1.05, 1.0], { planes: [[[0, -1, 0], 0.0055 - 0.0001 * i]] }); } }
  const step = [1, 1, 2, 5][level]!; for (const c of CANES) { const q = c.pts.filter((_, i) => i % step === 0 || i === c.pts.length - 1); for (let i = 0; i + 1 < q.length; i++) { const k0 = i / (q.length - 1), k1 = (i + 1) / (q.length - 1), ra = c.r0 + (c.r1 - c.r0) * k0, rb = c.r0 + (c.r1 - c.r0) * k1, tint = mix(c.old ? BARK : mix(BARK, CANE, 0.6), CANE, k0); p.bar(q[i]!, q[i + 1]!, ra, rb, mat, tint); if (level === 0 && i) p.ball(q[i]!, [ra, ra, ra], mat, tint); } }
  if (level < 2) for (const t of TWIGS) { if (level === 0) { p.bar(t.a, t.m, t.r, t.r * 0.9, mat, CANE); p.bar(t.m, t.b, t.r * 0.9, t.r * 0.75, mat, CANE); p.ball(t.m, [t.r * 0.9, t.r * 0.9, t.r * 0.9], mat, CANE); } else p.bar(t.a, t.b, t.r, t.r * 0.8, mat, CANE); }
  if (level === 0) { // stalks of flowers, and thorns
    for (const s of SPRIGS) if (s.blooms.length) p.bar(s.foot, add(s.foot, mul(s.y, 1.45 * s.scale)), 0.0026, 0.002, mat, mul(CANE, 1.05));
    const r = rng(5); for (const c of CANES) for (let i = 0; i < 6; i++) { const k = 0.06 + 0.8 * (i + r()) / 6, x = k * 5, n = Math.min(4, Math.floor(x)), at = add(mul(c.pts[n]!, 1 - (x - n)), mul(c.pts[n + 1]!, x - n)), a = r() * 6.28, rad = c.r0 + (c.r1 - c.r0) * k; p.bar(add(at, [Math.cos(a) * rad * 0.6, 0.002, Math.sin(a) * rad * 0.6]), add(at, [Math.cos(a) * (rad + 0.008), -0.003, Math.sin(a) * (rad + 0.008)]), 0.003, 0.0006, mat, [0.46, 0.30, 0.22]); }
  }
}
export const rose: ObjectKind = { name: 'rose', title: 'Shrub rose in flower', make: (): Made => ({
  name: 'rose', title: 'Shrub rose in flower', size: [1.25, 1.2, 1.25], on: 'floor', close: { at: [0.16, 0.85, 0.28], far: 3.0 }, read: 0, moves: { kind: 'sways' }, crowd: 64,
  levels: [
    { shape: 'leaves', error: 0, less: '', put(p) { put(p, 0); } },
    { shape: 'leaves', error: 0.0155, less: 'thorns, stalks; a leaf is three leaflets, a flower ten petals', put(p) { put(p, 1); } },
    { shape: 'leaves', error: 0.032, less: 'twigs; a leaf is one, a flower three petals', put(p) { put(p, 2); } },
    { shape: 'leaves', error: 0.070, less: 'buds; a shoot is five leaves, a flower two petals', put(p) { put(p, 3); } },
  ],
}) };
