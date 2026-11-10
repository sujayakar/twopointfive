// IVY ON A TRELLIS that stands by itself: a wooden trough of earth with two posts, between them a lattice of laths laid crosswise; out of the earth brown stems that climb and fork, and from them shoots whose five lobed leaves
// lie over each other like tiles, outward to both sides and over the top. The last part, to the right above, is not grown over yet. A few shoots hang over the trough.
// Coarser: a leaf is two ovals, then one; then one leaf for five, and half as many laths, twice as wide.
import { type V3, type Rot, add, sub, mul, norm, cross, dot, rng } from '../math';
import type { Cluster, Leaf, World } from '../scene/world';
import { M } from '../scene/materials';
import type { Made, ObjectKind, Put } from './object';
const W = 0.46, TALL = 1.90, Z = -0.06, RIM = 0.36, DEEP = 0.17, SOIL = 0.315;
const OLD: V3 = [0.152, 0.244, 0.084], YOUNG: V3 = [0.300, 0.385, 0.112], CEDAR: V3 = [0.600, 0.475, 0.365];
const square = (n: V3, t: V3): V3 => norm(sub(n, mul(t, dot(n, t))));

// ---- a shoot: it grows along +y from −0.95 (x is outward, off the lattice); leaves on long stalks to both sides, their points down, their faces out and a little up
interface Kind { seed: number; leaves: number; size: number; long: number; out: number; hang: number }
const KINDS: Kind[] = [{ seed: 1, leaves: 12, size: 0.196, long: 1.6, out: 0.30, hang: 0.7 }, { seed: 2, leaves: 11, size: 0.206, long: 1.5, out: 0.42, hang: 0.6 }, { seed: 3, leaves: 13, size: 0.186, long: 1.7, out: 0.24, hang: 0.8 }, { seed: 4, leaves: 12, size: 0.200, long: 1.6, out: 0.50, hang: 0.6 }, { seed: 5, leaves: 13, size: 0.135, long: 1.95, out: 0.20, hang: -0.15 }];
interface Blade { at: V3; t: V3; n: V3; size: number; tone: number }
interface Grew { blades: Blade[]; wood: Leaf[] }
function shoot(k: Kind): Grew {
  const r = rng(5200 + k.seed), blades: Blade[] = [], twigs: Leaf[] = [], wob = r() * 6.28, way = (q: number): V3 => [0.05 + k.out * 0.35 * q, -0.95 + k.long * q, 0.10 * Math.sin(q * 5 + wob) - 0.10 * Math.sin(wob)];
  const wood = (a: V3, b: V3) => { const d = sub(b, a), t = norm(d); twigs.push({ at: mul(add(a, b), 0.5), t, n: square([1, 0.1, 0.3], t), size: Math.hypot(...d) / 2 * 1.06, tone: 0.3 + 0.5 * r(), wood: true }); };
  for (let i = 0; i < 4; i++) wood(way(i / 4), way((i + 1) / 4));
  for (let i = 0; i < k.leaves; i++) { const q = 0.04 + 0.96 * (i + 0.6) / k.leaves, s = i === k.leaves - 1 ? 0.2 : i % 2 ? 1 : -1, size = k.size * (0.82 + 0.36 * r()) * (q > 0.82 ? 1 - 2.4 * (q - 0.82) : 1), p = way(q), far = 0.14 + 0.42 * r(), j = add(p, [k.out * (0.35 + 0.9 * r()), 0.10 * (r() - 0.2), s * far]), t = norm([0.18 + 0.35 * (r() - 0.4), -k.hang + 0.5 * (r() - 0.5), s * (0.55 + 0.5 * r()) + (r() - 0.5) * 0.35]), n = square([0.9, 0.42 + 0.5 * (r() - 0.5), s * 0.12 + (r() - 0.5) * 0.7], t); blades.push({ at: j, t, n, size, tone: 0.22 + 0.40 * r() + 0.25 * q * q }); if (i % 3 === 0) wood(p, j); }
  return { blades, wood: twigs };
}
/** a mat: shoots side by side on both faces of the lattice. Its frame is upright, its middle in the lattice, x out to the front */
const MAT = 0.36, MATS = 3, TUFT = 0.21, TUFTS = 2, SHOOTS = KINDS.map(shoot);
function mat(seed: number, many: number, wide: number, size: number): Grew {
  const r = rng(5300 + seed), blades: Blade[] = [], twigs: Leaf[] = []; for (let i = 0; i < many; i++) { const f = i % 2 ? -1 : 1, q = Math.sqrt((i + 0.5) / many), a = i * 2.39996 + seed, lean = (r() - 0.5) * 1.5, y: V3 = norm([f * 0.16, Math.cos(lean), Math.sin(lean)]), x = square([f, 0.1, (r() - 0.5) * 0.4], y), z = cross(x, y), k = size * (0.91 + 0.18 * r()), at: V3 = [f * size * (0.08 + 0.18 * r()), wide * q * Math.sin(a) + 0.10, wide * 1.06 * q * Math.cos(a)], into = (v: V3): V3 => add(add(mul(x, v[0]), mul(y, v[1])), mul(z, v[2])), g = SHOOTS[(i + seed) % 4]!, old = r() * 0.14 - 0.07; for (const b of g.blades) blades.push({ at: add(at, mul(into(b.at), k)), t: into(b.t), n: into(b.n), size: b.size * k, tone: b.tone + old }); for (const l of g.wood) twigs.push({ ...l, at: add(at, mul(into(l.at), k)), t: into(l.t), n: into(l.n), size: l.size * k }); }
  return { blades, wood: twigs };
}
const turn = (b: Blade, a: number): V3 => norm(add(mul(b.t, Math.cos(a)), mul(cross(b.n, b.t), Math.sin(a))));
/** (hidden: deep in) five lobes from where the stalk comes in, each a little folded down; coarser two, one, or one for `keep` */
function cluster(g: Grew, step: Step, hidden = (_: Blade) => false): Cluster {
  const leaves: Leaf[] = step.twigs ? [...g.wood] : [], lobe = (b: Blade, a: number, size: number, fold: number, k = 0.97) => { const t = turn(b, a), side = cross(b.n, b.t); leaves.push({ at: add(b.at, mul(t, size * k)), t, n: square(add(b.n, mul(side, fold)), t), size, tone: b.tone }); };
  for (const b of step.lobes > 1 ? g.blades : []) if (step.lobes === 5) { lobe(b, 0, b.size, 0); for (const s of [-1, 1]) { lobe(b, s * 0.74, b.size * 0.82, s * 0.14); if (!hidden(b)) lobe(b, s * 1.48, b.size * 0.58, s * 0.20); } } else for (const s of [-1, 1]) lobe(b, s * 0.56, b.size * 1.10 * step.big, s * 0.14, 0.80);
  if (step.lobes === 1) for (let i = 0; i < g.blades.length; i += step.keep) { const of = g.blades.slice(i, i + step.keep), one = of[of.length >> 1]!, mids = of.map(b => add(b.at, mul(b.t, b.size * 0.75))), mid = mul(mids.reduce((a, b) => add(a, b), [0, 0, 0] as V3), 1 / of.length), own = add(one.at, mul(one.t, one.size * 0.75)), size = Math.sqrt(of.reduce((a, b) => a + b.size * b.size, 0)) * 1.45 * step.big; leaves.push({ at: add(mul(mid, 0.6), mul(own, 0.4)), t: step.keep === 1 && i % 2 ? turn(one, Math.PI / 2) : one.t, n: one.n, size, tone: of.reduce((a, b) => a + b.tone, 0) / of.length }); }
  return { kind: 0, leaves };
}

/** every kind there is: shoots, mats, tufts */
const GREWN: Grew[] = [...SHOOTS, ...Array.from({ length: MATS }, (_, i) => mat(i, 12, 0.62, 0.47)), ...Array.from({ length: TUFTS }, (_, i) => mat(7 + i, 5, 0.34, 0.66))];
// ---- how it has grown
/** a band of the trellis, its leaves in its own frame, ROW long */
interface Row { at: V3; light: number; grew: Grew }
const ROW = 0.56;
interface Stem { a: V3; b: V3; ra: number; rb: number }
interface Shoot { root: V3; y: V3; x: V3; kind: number; scale: number; light: number }
interface Grown { stems: Stem[]; shoots: Shoot[]; rows: Row[] }
function grow(): Grown {
  const r = rng(8301), stems: Stem[] = [], shoots: Shoot[] = [];
  const put = (root: V3, y: V3, out: V3, kind: number, scale: number, light: number) => { const Y = norm(y), x = square(out, Y); shoots.push({ root, y: Y, x, kind, scale, light }); };
  // how high each stem has come; they lean toward the light, away from the bare side
  const high = [1.84, 1.90, 1.80, 1.62, 1.36, 1.06, 0.86]; high.forEach((top, i) => { const face = i % 3 === 1 ? -1 : 1, x0 = -0.36 + 0.72 * i / (high.length - 1) + (r() - 0.5) * 0.06, zOn = Z + face * 0.017; let p: V3 = [x0 * 0.86, SOIL - 0.01, Z + face * (0.05 + 0.03 * r())], h = (r() - 0.5) * 0.3; const steps = Math.round((top - SOIL) / 0.085), pts: V3[] = [p];
    for (let j = 0; j < steps; j++) { h += (r() - 0.5) * 0.75 - 0.35 * h; if (p[0] > 0.36) h -= 0.4; if (p[0] < -0.36) h += 0.4; p = [p[0] + Math.sin(h) * 0.088, p[1] + Math.cos(h) * 0.088, zOn + face * 0.006 * r()]; pts.push(p); }
    const n = pts.length - 1, r0 = 0.0055 + 0.0032 * r(); for (let j = 0; j < n; j++) stems.push({ a: pts[j]!, b: pts[j + 1]!, ra: r0 * (1 - 0.7 * j / n), rb: r0 * (1 - 0.7 * (j + 1) / n) });
    // shoots all the way up, to either side and either face; thinner at the top
    for (let j = n - 2; j <= n; j++) { const d = norm(sub(pts[j]!, pts[j - 1]!)), light = 0.45 + 0.35 * (j - n + 2) / 2 + 0.2 * (r() - 0.5); for (let e = 0; e < 2; e++) { const f = r() < 0.65 ? face : -face, s = (j + e) % 2 ? 1 : -1, lean = s * (0.35 + 0.75 * r()); put(add(pts[j]!, [0, -0.03 * r(), 0]), [Math.sin(lean) + d[0] * 0.3, Math.cos(lean) * 0.9, f * (0.10 + 0.22 * r())], [0.25 * s * r(), 0.1, f], Math.floor(r() * 4), 0.13 + 0.03 * r(), light); } }
    const end = pts[n]!; for (let e = 0; e < 2; e++) put(end, [(r() - 0.5) * 1.0, 1, face * 0.25 * r()], [0, 0.2, e ? -face : face], 4, 0.115 + 0.035 * r(), 0.85 + 0.15 * r());
  });
  // where it is thick, mats: in rows, each a little out of line, as high as the stems have come there
  const here = (x: number) => { const q = Math.max(0, Math.min(high.length - 1.001, (x + 0.36) / 0.72 * (high.length - 1))), i = Math.floor(q); return high[i]! + (high[i + 1]! - high[i]!) * (q - i); };
  for (let row = 0; row < 5; row++) for (let i = 0; i < 3; i++) { const x = -0.33 + 0.29 * i + (row % 2 ? 0.10 : 0.0) + 0.06 * (r() - 0.5), y = 0.54 + 0.285 * row + 0.05 * (r() - 0.5); if (Math.abs(x) > 0.37 || y > here(x) - 0.15) continue; const lean = 0.30 * (r() - 0.5), f = (row + i) % 2 ? 1 : -1, Y: V3 = [Math.sin(lean), Math.cos(lean), 0], k = MAT * (0.94 + 0.14 * r()), q = (y - SOIL) / (here(x) - SOIL); shoots.push({ root: sub([x, y, Z], mul(Y, 0.95 * k)), y: Y, x: [0, 0, f], kind: KINDS.length + Math.floor(r() * MATS), scale: k, light: 0.10 + 0.40 * q * q + 0.18 * (r() - 0.5) }); }
  // over the top, and round the posts
  const tuft = (root: V3, y: V3, out: V3, light: number) => put(root, y, out, KINDS.length + MATS + Math.floor(r() * TUFTS), TUFT * (0.88 + 0.24 * r()), light);
  for (let i = 0; i < 3; i++) tuft([-0.37 + 0.48 * (i + r() * 0.8) / 3, TALL - 0.13, Z], [(r() - 0.5) * 0.5, 0.9, (r() - 0.5) * 0.3], [0.3 * (r() - 0.5), 0, i % 2 ? 1 : -1], 0.55 + 0.3 * r());
  for (let i = 0; i < 4; i++) put([-0.42 + 0.54 * (i + r()) / 4, TALL - 0.02, Z], [(r() - 0.5) * 0.9, 0.75, (r() - 0.5) * 0.9], [0.3 * (r() - 0.5), 0.5, i % 2 ? 1 : -1], 4, 0.12 + 0.03 * r(), 0.7 + 0.3 * r());
  for (let i = 0; i < 5; i++) { const left = i < 4, y = left ? 0.46 + 1.15 * (i + r()) / 4 : 0.46 + 0.20 * r(), s = left ? -1 : 1; tuft([s * (W - 0.06), y, Z], [s * 0.12, 0.9, (r() - 0.5) * 0.5], [0.25 * (r() - 0.5), 0, i % 2 ? 1 : -1], 0.2 + 0.3 * r()); }
  for (let i = 0; i < 2; i++) put([(i < 2 ? -1 : 1) * (W - 0.03), 0.6 + 0.4 * i * r(), Z], [(i < 2 ? -1 : 1) * (0.5 + 0.4 * r()), 0.6, (r() - 0.5) * 0.8], [i < 2 ? -1 : 1, 0.1, (r() - 0.5) * 0.6], 4, 0.12 + 0.03 * r(), 0.6);
  // over the earth, and over the rim
  for (let i = 0; i < 6; i++) { const f = i % 3 === 2 ? -1 : 1; tuft([-0.36 + 0.72 * (i + r()) / 6, SOIL - 0.03, f > 0 ? 0.03 : -0.11], [(r() - 0.5) * 0.7, 0.8, f * 0.55], [0, 0.4, f], 0.08 + 0.25 * r()); }
  for (const [x, l] of [[-0.30, 0.2], [-0.20, 0.5], [-0.05, 0.3], [0.16, 0.6], [0.24, 0.35], [0.37, 0.2]] as const) { let a: V3 = [x, SOIL, DEEP - 0.06]; const over: V3[] = [a, [x + 0.01, RIM + 0.022, DEEP - 0.015], [x + 0.02, RIM + 0.008, DEEP + 0.028], [x + 0.025, RIM - 0.06, DEEP + 0.040]]; for (let j = 0; j < 3; j++) stems.push({ a: over[j]!, b: over[j + 1]!, ra: 0.0032, rb: 0.0028 }); put(over[3]!, [0.15 * (r() - 0.5), -0.95, 0.05], [0, 0.1, 1], l > 0.4 ? 4 : 2, 0.10 + 0.03 * r(), l + 0.3); put(over[2]!, [(r() - 0.5) * 1.5, -0.35, 0.3], [0, 0.6, 0.8], 0, 0.10 + 0.02 * r(), l); a = over[3]!; void a; }
  // what is thick is rows, a cluster each, so that its leaves know how dark it is between them; what hangs or reaches out free is a shoot each
  const free = shoots.filter(q => q.kind === KINDS.length - 1 || q.y[1] < 0), rows: Row[] = [0.46, 0.80, 1.20, 1.62].map((y, i) => ({ at: [0, y, Z + (i ? 0 : 0.06)], light: [0.10, 0.18, 0.32, 0.52][i]!, grew: { blades: [], wood: [] } })), rowOf = (y: number) => rows[y < 0.60 ? 0 : y < 1.00 ? 1 : y < 1.41 ? 2 : 3]!;
  for (const q of shoots) if (!free.includes(q)) { const g = GREWN[q.kind]!, z = cross(q.x, q.y), mid = add(q.root, mul(q.y, 0.95 * q.scale)), row = rowOf(mid[1]), into = (v: V3): V3 => add(add(mul(q.x, v[0]), mul(q.y, v[1])), mul(z, v[2])), place = (v: V3): V3 => mul(sub(add(mid, mul(into(v), q.scale)), row.at), 1 / ROW), k = q.scale / ROW, more = (q.light - row.light) * 0.55 + (r() - 0.5) * 0.16; for (const b of g.blades) row.grew.blades.push({ at: place(b.at), t: into(b.t), n: into(b.n), size: b.size * k, tone: Math.max(0, Math.min(1, b.tone + more)) }); for (const l of g.wood) row.grew.wood.push({ ...l, at: place(l.at), t: into(l.t), n: into(l.n), size: l.size * k }); }
  return { stems, shoots: free, rows };
}
let grown: Grown | undefined; const ivyOf = (): Grown => grown ??= grow();
// ---- levels
interface Step { lobes: number; keep: number; twigs: boolean; big: number; bright: V3; stems: number; every: number }
const STEPS: Step[] = [{ lobes: 5, keep: 1, twigs: true, big: 1, bright: [1, 1, 1], stems: 0, every: 1 }, { lobes: 2, keep: 1, twigs: false, big: 1.06, bright: [0.98, 0.986, 0.965], stems: 0.004, every: 1 }, { lobes: 1, keep: 1, twigs: false, big: 1.075, bright: [0.893, 0.938, 0.850], stems: 1, every: 1 }, { lobes: 1, keep: 5, twigs: false, big: 1.10, bright: [0.880, 0.912, 0.852], stems: 1, every: 2 }];
const kept = new WeakMap<World, Map<number, number[]>>();
const clusters = (w: World, level: number): number[] => { let m = kept.get(w); if (!m) kept.set(w, m = new Map()); let l = m.get(level); if (!l) m.set(level, l = [...SHOOTS.map(g => cluster(g, STEPS[level]!)), ...ivyOf().rows.map(q => cluster(q.grew, STEPS[level]!, /* (of what lies against the lattice nobody sees the small lobes) */ b => Math.abs(b.at[2] * ROW + q.at[2] - Z) < 0.040))].map(c => { w.clusters.push(c); return w.clusters.length - 1; })); return l; };
function trellis(p: Put, level: number, every: number) {
  const r = rng(77), mat = M.crumb!, board = (a: V3, b: V3, v = 0.92 + 0.16 * r()) => p.box(mul(add(a, b), 0.5), [(b[0] - a[0]) / 2, (b[1] - a[1]) / 2, (b[2] - a[2]) / 2], mat, mul(CEDAR, v)), fine = level < 2;
  // the trough: boards on corner posts, a rim, earth
  if (fine) { for (const z of [-DEEP, DEEP]) for (let i = 0; i < 3; i++) board([-W, 0.022 + 0.112 * i, z - 0.010], [W, 0.129 + 0.112 * i, z + 0.010]); for (const x of [-W + 0.015, W - 0.015]) for (let i = 0; i < 3; i++) board([x - 0.009, 0.0225 + 0.112 * i, -DEEP + 0.008], [x + 0.009, 0.1285 + 0.112 * i, DEEP - 0.008]); for (const x of [-1, 1]) for (const z of [-1, 1]) board([x * (W - 0.045) - 0.022, 0, z * (DEEP - 0.030) - 0.022], [x * (W - 0.045) + 0.022, RIM - 0.02, z * (DEEP - 0.030) + 0.022], 0.9); board([-W + 0.02, 0.03, -DEEP + 0.01], [W - 0.02, 0.05, DEEP - 0.01], 0.85); }
  else { board([-W, 0.012, -DEEP - 0.008], [W, RIM - 0.01, DEEP + 0.008], 0.96); for (const x of [-1, 1]) board([x * (W - 0.05) - 0.03, 0, -DEEP + 0.005], [x * (W - 0.05) + 0.03, 0.02, DEEP - 0.005], 0.8); }
  for (const z of [-1, 1]) board([-W - 0.022, RIM - 0.006, z * DEEP - 0.026], [W + 0.022, RIM + 0.015, z * DEEP + 0.026], 1.06); for (const x of [-1, 1]) board([x * W - 0.026, RIM - 0.0055, -DEEP + 0.026], [x * W + 0.0245, RIM + 0.0145, DEEP - 0.026], 1.03);
  p.box([0, SOIL - 0.03 + (fine ? 0 : 0.025), 0], [W - 0.012, 0.03, DEEP - 0.011], M.soil!, [1, 1, 1]);
  // posts, rails
  const post = W - 0.028; for (const x of [-1, 1]) { board([x * post - 0.020, 0.05, Z - 0.021], [x * post + 0.020, TALL, Z + 0.021], 0.97); if (fine) p.add({ at: [x * post, TALL + 0.012, Z], half: [0.027, 0.016, 0.028], mat, tint: mul(CEDAR, 1.05), planes: [[[0.6, 0.8, 0], 0.019], [[-0.6, 0.8, 0], 0.019], [[0, 0.8, 0.6], 0.0195], [[0, 0.8, -0.6], 0.0195]] }); }
  for (const y of [0.445, TALL - 0.045]) board([-post + 0.018, y - 0.020, Z - 0.0185], [post - 0.018, y + 0.020, Z + 0.0185], 1.0);
  // laths, laid crosswise, one layer on the other; their ends are in the frame
  const x1 = post - 0.012, y0 = 0.455, y1 = TALL - 0.055, mid = (y0 + y1) / 2, step = 0.128 * every, half = 0.0105 * every; for (const s of [-1, 1]) { const far = (x1 + (y1 - mid)) * Math.SQRT1_2, n = Math.floor(far / step); for (let i = -n; i <= n; i++) { const c = i * step + (s > 0 ? 0.02 : -0.013); /* x·s·√½ … the line where (−s x + y) √½ = c */ let lo = -9, hi = 9; const d: V3 = [Math.SQRT1_2 * s, Math.SQRT1_2, 0], o: V3 = [-s * c * Math.SQRT1_2, mid + c * Math.SQRT1_2, 0]; for (const [e, a, b] of [[0, -x1, x1], [1, y0, y1]] as const) { const u = (a - o[e]!) / d[e]!, v = (b - o[e]!) / d[e]!; lo = Math.max(lo, Math.min(u, v)); hi = Math.min(hi, Math.max(u, v)); } if (hi - lo < 0.06) continue; const at: V3 = [o[0] + d[0] * (lo + hi) / 2, o[1] + d[1] * (lo + hi) / 2, Z + s * 0.0046]; p.add({ at, half: [half, (hi - lo) / 2, 0.0044], rot: [[d[1], -d[0], 0], d, [0, 0, 1]], mat, tint: mul(CEDAR, 0.94 + 0.16 * r()) }); } }
}
function put(p: Put, level: number) {
  const s = STEPS[level]!, g = ivyOf(), ix = clusters(p.w, level), dice = rng(p.seed * 61 + 9); trellis(p, level, s.every);
  for (const b of g.stems) if (Math.max(b.ra, b.rb) >= s.stems) p.bar(b.a, b.b, b.ra, b.rb, M['lamp shade']!, [0.46, 0.385, 0.31]);
  const rows = g.rows.map((q, i) => { const v = 0.95 + 0.10 * dice(), tint = [0, 1, 2].map(c => (OLD[c]! + (YOUNG[c]! - OLD[c]!) * q.light) * v * s.bright[c]!) as V3; return { cluster: ix[KINDS.length + i]!, at: q.at, rot: [[1, 0, 0], [0, 1, 0], [0, 0, 1]] as Rot, scale: ROW, base: add(q.at, [0, -0.9, 0]), sway: 0.026 + 0.004 * i, phase: 0.9 * i + dice(), tint }; });
  p.from({ pieces: [], instances: [...rows, ...g.shoots.map(q => { const rot: Rot = [q.x, q.y, cross(q.x, q.y)], at = add(q.root, mul(q.y, 0.95 * q.scale)), k = Math.max(0, Math.min(1, q.light)), v = 0.92 + 0.16 * dice(), tint = [0, 1, 2].map(c => (OLD[c]! + (YOUNG[c]! - OLD[c]!) * k) * v * s.bright[c]!) as V3, lift = 2.76 * q.scale * (q.y[1] >= 0 ? Math.max(0.6, q.y[1]) : Math.min(-0.6, q.y[1])); return { cluster: ix[q.kind]!, at, rot, scale: q.scale, base: add(at, [0, -lift, 0]), sway: 0.16 + 0.06 * dice(), phase: dice() * 6.28, tint }; })] });
}
export const ivy: ObjectKind = { name: 'ivy', title: 'Ivy on a trellis', make: (): Made => ({
  name: 'ivy', title: 'Ivy on a trellis', size: [1.33, 2.17, 0.74], on: 'floor', close: { at: [0.05, 1.25, 0], far: 3 }, read: 0, moves: { kind: 'sways' }, crowd: 64,
  levels: [
    { shape: 'leaves', error: 0, less: '', put(p) { put(p, 0); } },
    { shape: 'leaves', error: 0.014, less: 'lobes: a leaf is two ovals; twigs, thin stems', put(p) { put(p, 1); } },
    { shape: 'leaves', error: 0.030, less: 'boards, stems; a leaf is one oval', put(p) { put(p, 2); } },
    { shape: 'leaves', error: 0.070, less: 'one leaf for five; half of the laths', put(p) { put(p, 3); } },
  ],
}) };
