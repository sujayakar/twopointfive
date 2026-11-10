// A CLUMP OF LAVENDER IN FLOWER: crooked grey wood from one root, on each end of it a tuft of narrow grey-green leaves, and out of that straight thin stalks that fan to a dome, each with a spike of
// violet whorls (close together, and often one more a little way down). Stalks and spikes of a tuft have one wind, so that they stay together.
// Coarser: a whorl is one leaf of its color, then a spike is two, then one for several; stalks thin out; narrow leaves are fewer and broader.
import { type V3, type Rot, add, sub, mul, norm, cross, dot, rng, rotY, apply, compose } from '../math';
import type { Cluster, Leaf, Piece, World } from '../scene/world';
import { M } from '../scene/materials';
import type { Made, ObjectKind, Put } from './object';
const LEAF: V3 = [0.465, 0.525, 0.420], STALK: V3 = [0.40, 0.450, 0.315], VIOLET: V3 = [0.375, 0.300, 0.590], WOOD: V3 = [0.40, 0.37, 0.34];
/** one kind of tuft: how far out its foot is (along +x), how high, how it leans outward */
interface Tuft { at: V3; y: V3; seed: number; inner: number }
const kind = (far: number, high: number, lean: number, seed: number): Tuft => ({ at: [far, high, 0], y: norm([Math.sin(lean), Math.cos(lean), 0]), seed, inner: 1 - far / 0.172 });
const KINDS: Tuft[] = [kind(0.010, 0.150, 0.05, 51), kind(0.064, 0.140, 0.24, 52), kind(0.070, 0.132, 0.34, 53), kind(0.120, 0.112, 0.52, 54), kind(0.112, 0.120, 0.60, 55), kind(0.172, 0.066, 0.86, 56), kind(0.164, 0.074, 0.98, 57), kind(0.168, 0.058, 0.76, 58)];
/** tufts as they stand: one in the middle, rings of five, nine and eleven round it; each a little larger or smaller */
const TUFTS: { kind: number; way: number; big: number }[] = (() => { const r = rng(4107), out: { kind: number; way: number; big: number }[] = []; for (const [n, first, many, a0] of [[1, 0, 1, 0.3], [5, 1, 2, 0.9], [9, 3, 2, 0.2], [11, 5, 3, 0.5]] as const) for (let i = 0; i < n; i++) out.push({ kind: first + (i + (i === n - 1 && n % many === 1 ? 1 : 0)) % many, way: a0 + (i + 0.30 * (r() - 0.5)) * 6.2832 / n, big: 0.92 + 0.15 * r() }); return out; })();
/** a way across, whatever d is */
const across = (d: V3, a: number): V3 => { const x = norm(cross(d, Math.abs(d[1]) < 0.9 ? [0, 1, 0] : [1, 0, 0])), z = cross(x, d); return add(mul(x, Math.cos(a)), mul(z, Math.sin(a))); };
const face = (n: V3, t: V3): V3 => norm(sub(n, mul(t, dot(n, t))));
interface Parts { leaves: Leaf[]; stalks: Leaf[]; blooms: Leaf[] }
/** how far a level goes: leaves kept (one of so many) and their length; whorls; parts of a stalk and how many stalks are kept */
interface Grain { leaf: number; leafLong: number; oval: boolean; bloom: 'florets' | 'whorls' | 'halves' | 'one'; big: number; stalk: number; stalks: number }
const GRAINS: Grain[] = [
  { leaf: 1, leafLong: 1, oval: false, bloom: 'florets', big: 1, stalk: 11, stalks: 1 },
  { leaf: 4, leafLong: 0.685, oval: true, bloom: 'whorls', big: 1.26, stalk: 9, stalks: 0.75 },
  { leaf: 16, leafLong: 1.25, oval: true, bloom: 'halves', big: 1.24, stalk: 3, stalks: 0.25 },
  { leaf: 64, leafLong: 2.48, oval: true, bloom: 'one', big: 1.04, stalk: 0, stalks: 0 },
];
/** a tuft's parts in the object's own frame (metres) */
function grow(u: Tuft, g: Grain): Parts {
  const r = rng(u.seed * 977), P: Parts = { leaves: [], stalks: [], blooms: [] }; let count = 0;
  // leafy shoots: each a little stem with narrow leaves in crossed pairs, long at its foot, short and upright at its tip
  const shoots = 11; for (let i = 0; i < shoots; i++) {
    const a = i * 2.39996 + r(), open = (i === 0 ? 0.1 : 0.35 + 0.75 * Math.sqrt(i / shoots)) * (0.9 + 0.2 * r()), d0 = norm(add(u.y, add(mul(across(u.y, a), open), [0, 0.18, 0]))), long = 0.085 + 0.060 * r(), root = add(u.at, mul(d0, -0.03)), bend = 0.30, n = 11;
    const at = (k: number): V3 => add(add(root, mul(d0, long * k)), [0, long * bend * k * k, 0]), dir = (k: number): V3 => norm(add(d0, [0, 2 * bend * k, 0]));
    for (let j = 0; j < n; j++) for (const s of [0, 1]) for (const twin of [0, 1]) { if (twin && j > n - 4) continue;
      const k = 0.10 + 0.90 * (j + 0.3 * r()) / (n - 1), d = dir(k), p = at(k), b = i + j * 1.5708 + s * 3.1416 + twin * 0.9 + (r() - 0.5) * 0.5, o = across(d, b), spread = (1.05 - 0.72 * k) * (0.85 + 0.3 * r()), t = norm(add(add(d, mul(o, spread)), [0, 0.10 - 0.16 * r() * (1 - k), 0])), size = (0.0265 - 0.011 * k * k) * (0.84 + 0.32 * r()) * (twin ? 0.72 : 1), tone = Math.min(1, Math.max(0, 0.12 + 0.62 * k + 0.30 * (r() - 0.5))), n0 = face(add(mul(o, -0.5), add(mul(d, 0.9), [0, 0.5, 0])), t), spin = r();
      if (count++ % g.leaf !== (g.leaf >> 1)) continue;
      const z = size * g.leafLong, f = g.oval ? face(add(n0, mul(cross(n0, t), (spin - 0.5) * 1.6)), t) : face(add(n0, mul(cross(n0, t), (spin - 0.5) * 1.2)), t); P.leaves.push({ at: add(p, mul(t, g.oval ? size * 0.92 : size * 0.95)), t, n: f, size: z, tone });
    }
  }
  // stalks: straight, a little bowed; from within the tuft
  const stalks = 8; for (let i = 0; i < stalks; i++) {
    const a = i * 2.39996 + 1.3 + 0.5 * r(), open = 0.40 * Math.sqrt((i + 0.4) / stalks) * (0.85 + 0.3 * r()), d0 = norm(add(u.y, mul(across(u.y, a), open))), long = (0.255 + 0.075 * u.inner) * (0.82 + 0.32 * r()), root = add(u.at, mul(d0, 0.01 + 0.02 * r())), up = 0.16 + 0.12 * r(), at = (k: number): V3 => add(add(root, mul(d0, long * k)), [0, long * up * (k * k - 0.25 * k), 0]), way = (k: number): V3 => norm(sub(at(k + 0.02), at(k - 0.02))), tone = 0.35 + 0.4 * r(), keep = (u.seed * 7 + i * 5) % 8 / 8 < g.stalks;
    if (g.stalk && keep) for (let j = 0; j < g.stalk; j++) { const p = at(0.02 + 0.98 * j / g.stalk), q = at(0.02 + 0.98 * (j + 1) / g.stalk), e = sub(q, p), t = norm(e); P.stalks.push({ at: mul(add(p, q), 0.5), t, n: face(across(t, a + j * 1.9), t), size: Math.hypot(...e) / 2 * 1.22, tone }); }
    const spike = (0.036 + 0.026 * r()) * (0.8 + 0.2 * u.inner), end = way(1), from = at(1), fat = 0.0043 + 0.0009 * r(), whorls = Math.max(5, Math.round(spike / 0.0062)), age = r(), more = r() < 0.62, low = 0.016 + 0.012 * r();
    const ring = (c: V3, w: number, m: number, size: number) => { for (let q = 0; q < m; q++) { const o = across(end, q * 6.2832 / m + w * 0.55 + (r() - 0.5) * 0.3), lip = r() < 0.26, t = norm(add(mul(o, lip ? 1.0 : 0.72), mul(end, lip ? 0.45 : 0.85))), z = size * (lip ? 1.16 : 0.92 + 0.16 * r()), tone = lip ? 0.82 + 0.18 * r() : 0.10 + 0.34 * r() + 0.10 * age; P.blooms.push({ at: add(add(c, mul(o, fat * 0.30)), mul(t, z * 0.72)), t, n: face(add(end, add(mul(o, -0.2), mul(cross(o, end), (r() - 0.5) * 1.2))), t), size: z, tone }); } };
    if (g.bloom === 'florets') { for (let w = 0; w < whorls; w++) { const k = (w + 0.35) / whorls, thin = Math.sin(Math.PI * Math.min(1, 0.26 + 0.60 * k)); ring(add(from, mul(end, spike * k * 0.93)), w, w >= whorls - 1 ? 4 : 8, fat * (0.62 + 0.40 * thin)); } if (more) ring(sub(from, mul(way(0.95), low)), 1, 6, fat * 0.86); }
    else { const m = g.bloom === 'whorls' ? 5 : g.bloom === 'halves' ? 2 : 1, z = (0.0065 + spike / m * 0.45) * g.big; for (let w = 0; w < m; w++) { const o = across(end, w * 1.7 + a); P.blooms.push({ at: add(from, mul(end, spike * (w + 0.5) / m)), t: norm(add(end, mul(across(end, w * 2.4 + i), 0.15))), n: o, size: z, tone: 0.40 }); } if (m === 5 && more) P.blooms.push({ at: sub(from, mul(way(0.95), low - 0.003)), t: across(end, a), n: end, size: 0.0085, tone: 0.40 }); }
  }
  return P;
}
/** parts in an instance's frame */
const into = (ls: Leaf[], R: Rot, at: V3, scale: number): Leaf[] => ls.map(l => { const e = mul(sub(l.at, at), 1 / scale), q = (v: V3): V3 => [dot(R[0], v), dot(R[1], v), dot(R[2], v)]; return { ...l, at: q(e), n: q(l.n), t: q(l.t), size: l.size / scale }; });
const frame = (y: V3): Rot => { const x = norm(cross(y, [0.31, 0.2, 0.93])); return [x, y, cross(x, y)]; };
/** what of a kind of tuft is one instance: leaves, stalks, blooms */
interface Sort { cluster: Cluster; at: V3; rot: Rot; scale: number; tint: V3; what: 0 | 1 | 2 }
const made = new Map<number, Sort[][]>(), BRIGHT: V3[] = [[1, 1, 1], [0.93, 1, 0.925], [0.935, 1, 0.93], [0.875, 1, 0.90]];
function sorts(level: number): Sort[][] { let all = made.get(level); if (all) return all; const g = GRAINS[level]!, b = BRIGHT[level]!; all = KINDS.map(u => { const s: Sort[] = [], p = grow(u, g), R = frame(u.y), push = (ls: Leaf[], kind: number, at: V3, scale: number, tint: V3, what: 0 | 1 | 2) => { if (ls.length > 1) s.push({ cluster: { kind, leaves: into(ls, R, at, scale) }, at, rot: R, scale, tint, what }); };
  push(p.leaves, g.oval ? 0 : 1, u.at, g.oval ? 0.175 : 0.162, mul(LEAF, b[0] * (g.oval ? Math.sqrt(0.7) : 1)), 0); push(p.stalks, 1, u.at, 0.21, mul(STALK, b[1]), 1); const mid = mul(p.blooms.reduce((q, l) => add(q, l.at), [0, 0, 0] as V3), 1 / Math.max(1, p.blooms.length)); push(p.blooms, 0, mid, 0.29, mul(VIOLET, b[2]), 2); return s; }); made.set(level, all); return all; }
const kept = new WeakMap<World, Map<number, number[][]>>();
const clusters = (w: World, level: number): number[][] => { let m = kept.get(w); if (!m) kept.set(w, m = new Map()); let l = m.get(level); if (!l) m.set(level, l = sorts(level).map(k => k.map(s => { w.clusters.push(s.cluster); return w.clusters.length - 1; }))); return l; };
/** crooked grey wood from the root to each tuft */
function wood(p: Put, place: (k: number) => (v: V3) => V3, big: number, level: number) {
  const bar = (a: V3, b: V3, ra: number, rb: number) => { const q = p.bar(a, b, ra * big, rb * big, M.bark!, WOOD); if (Math.min(a[1], b[1]) < 0.02) q.planes = [[[-q.rot[0][1], -q.rot[1][1], -q.rot[2][1]], q.at[1] - p.at[1]]] as Piece['planes']; };
  if (level < 3) TUFTS.forEach((f, k) => { if (level === 2 && k % 3) return; const u = KINDS[f.kind]!, r = rng(k * 31 + 7), to = place(k), root = to([u.at[0] * 0.10, 0.004, 0]), mid = to([u.at[0] * 0.55 + 0.02 * (r() - 0.5), u.at[1] * (0.36 + 0.1 * r()), 0.03 * (r() - 0.5)]), end = to(add(u.at, mul(u.y, 0.012))); if (level < 2) { bar(root, mid, 0.0095, 0.007); bar(mid, end, 0.0068, 0.0042); if (!level) p.ball(mid, [0.0074 * big, 0.0074 * big, 0.0074 * big], M.bark!, WOOD); } else bar(root, end, 0.009, 0.005); });
  const foot = level < 2 ? 0.022 : 0.026; bar([0.003, 0, 0.002], [0.004, 0.036, -0.002], foot, foot * 0.62);
}
function put(p: Put, level: number) {
  const ix = clusters(p.w, level), dice = rng(p.seed * 23 + 5), big = p.seed === 1 ? 1 : 0.90 + 0.18 * dice(), yaw = p.seed === 1 ? 0 : p.seed * 2.1, turns = TUFTS.map(t => rotY(yaw - t.way)), place = (k: number) => (v: V3): V3 => mul(apply(turns[k]!, v), big * TUFTS[k]!.big), phase = TUFTS.map(() => dice() * 6.28), shade = TUFTS.map(() => 0.95 + 0.10 * dice());
  p.from({ pieces: [], instances: TUFTS.flatMap((t, k) => sorts(level)[t.kind]!.map((s, j) => { const at = place(k)(s.at), z = big * t.big, d = s.what === 2 ? (s.at[1] - KINDS[t.kind]!.at[1]) * z : 0; /* (a spike goes as its stalk turns about its foot) */ return { cluster: ix[t.kind]![j]!, at, rot: compose(turns[k]!, s.rot), scale: s.scale * z, base: [at[0], at[1] - 0.30 - 2.91 * d, at[2]] as V3, sway: s.what ? 0.050 : 0.030, phase: phase[k]! + (s.what ? 0 : 0.7), tint: mul(s.tint, s.what ? 1 : shade[k]!) }; })) });
  wood(p, place, big, level);
}
export const lavender: ObjectKind = { name: 'lavender', title: 'Lavender', make: (): Made => ({
  name: 'lavender', title: 'Lavender', size: [0.95, 0.66, 0.92], on: 'floor', close: { at: [0.06, 0.36, 0.12], far: 1.9 }, read: 0, moves: { kind: 'sways' }, crowd: 64,
  levels: [
    { shape: 'leaves', error: 0, less: '', put(p) { put(p, 0); } },
    { shape: 'leaves', error: 0.018, less: 'florets: a whorl is one; three narrow leaves of four; some stalks', put(p) { put(p, 1); } },
    { shape: 'leaves', error: 0.032, less: 'whorls: a spike is two leaves; most narrow leaves and stalks', put(p) { put(p, 2); } },
    { shape: 'leaves', error: 0.060, less: 'stalks; a spike is one leaf', put(p) { put(p, 3); } },
  ],
}) };
