// AN OFFICE CHAIR: a cast star on five twin castors, a gas spring, a sling of sewn channels between two rails. The seat turns on the spring, the back leans on the seat. Its channels are round bars across,
// laid along a side view; coarser, they are slabs along the same line.
import { type V3, type Rot, add, sub, mul, norm, cross, rng, rotX, rotY } from '../math';
import { M } from '../scene/materials';
import type { Joint, Live, Made, ObjectKind, Put } from './object';
const CLOTH: V3 = [0.735, 0.33, 0.245], BLACK: V3 = [0.25, 0.25, 0.26], ALLOY: V3 = [0.98, 0.98, 0.98], W = 0.228, RAIL = 0.0115, X = W + RAIL - 0.001;
/** cloth and the rails beside it, from far off */
const FAR: V3 = [0.735, 0.39, 0.33];
const tone = (c: V3, k: number): V3 => [c[0] * k, c[1] * k, c[2] * k];
type YZ = [number, number];
/** the side view, as (y, z): the seat from its front edge back, the back from its hinge up */
const SEAT: YZ[] = [[0.402, 0.246], [0.428, 0.236], [0.447, 0.205], [0.4535, 0.150], [0.4500, 0.07], [0.4440, -0.02], [0.4435, -0.11], [0.4530, -0.190]], BACK: YZ[] = [[0.480, -0.2135], [0.545, -0.2165], [0.610, -0.2185], [0.690, -0.231], [0.780, -0.253], [0.870, -0.281], [0.940, -0.307], [0.975, -0.326]];
const HINGE: YZ = [0.457, -0.208], PIN: V3 = [0, ...HINGE];
/** so far along a line, and which way it goes there */
function along(l: YZ[], s: number): { at: V3; t: V3 } { let i = 0, left = s; for (; i + 2 < l.length; i++) { const d = Math.hypot(l[i + 1]![0] - l[i]![0], l[i + 1]![1] - l[i]![1]); if (left <= d) break; left -= d; } const a = l[i]!, b = l[i + 1]!, t = norm([0, b[0] - a[0], b[1] - a[1]]); return { at: [0, a[0] + t[1] * left, a[1] + t[2] * left], t }; }
const long = (l: YZ[]) => l.reduce((n, p, i) => i ? n + Math.hypot(p[0] - l[i - 1]![0], p[1] - l[i - 1]![1]) : 0, 0);
const frame = (t: V3): Rot => [[1, 0, 0], t, cross([1, 0, 0], t)];
/** sewn channels along a line: each a bar, flat oval */
function channels(p: Put, l: YZ[], n: number) { const L = long(l), step = L / n, h = step / 2 + 0.0018; for (let i = 0; i < n; i++) { const c = along(l, (i + 0.5) * step), a = along(l, (i + 0.12) * step).at, b = along(l, (i + 0.88) * step).at, t = norm(sub(b, a)), k = 0.985 + 0.03 * ((i * 7) % 5) / 4; p.add({ at: add(mul(add(a, b), 0.35), mul(c.at, 0.30)), half: [W + 0.0008 * (i % 2), h, 0.0205], rot: frame(t), mat: M.canvas!, tint: tone(CLOTH, k), quad: [0, 1 / (h * 1.16) ** 2, 1 / 0.0215 ** 2, 0] }); } }
/** the same as slabs: between such places on the line */
function slabs(p: Put, l: YZ[], cuts: [number, number][], thick: number, k = 1, wide = W, tint = CLOTH) { for (const [a, b] of cuts) { const u = along(l, a).at, v = along(l, b).at, t = norm(sub(v, u)); p.add({ at: mul(add(u, v), 0.5), half: [wide, Math.hypot(...sub(v, u)) / 2, thick], rot: frame(t), mat: M.canvas!, tint: tone(tint, k) }); } }
function rails(p: Put, l: YZ[], at: number[], r: number, mat = M.steel!, tint = ALLOY, from = 0) { const pts = at.map(i => l[i]!); for (const s of [-1, 1]) for (let i = from; i + 1 < pts.length; i++) { const a: V3 = [s * X, ...pts[i]!], b: V3 = [s * X, ...pts[i + 1]!], d = mul(norm(sub(b, a)), i + 2 < pts.length ? 0.0022 : 0); p.bar(a, add(b, d), r, r * 0.998, mat, tint); } }
/** one arm of the star: high and wide at the hub, its upper edges broken */
const arm = (i: number): { out: V3; yaw: number } => { const a = i * Math.PI * 0.4 + Math.PI / 2 + 0.16; return { out: [Math.cos(a), 0, Math.sin(a)], yaw: a }; }, TIP = 0.292;
function star(p: Put, fine: boolean) {
  for (let i = 0; i < 5; i++) {
    const { out } = arm(i), a = add(mul(out, 0.020), [0, 0.104, 0]), b = add(mul(out, TIP + 0.012), [0, 0.0735, 0]), x = norm(sub(b, a)), z = norm(cross(x, [0, 1, 0])), y = cross(z, x), h = Math.hypot(...sub(b, a)) / 2, side = 0.0075 / h, top = 0.0080 / h;
    const pl = (n: V3, w: number): [V3, number] => { const l = Math.hypot(...n); return [mul(n, 1 / l), w / l]; };
    p.add({ at: mul(add(a, b), 0.5), half: [h, 0.0245, 0.0255], rot: [x, y, z], mat: M.steel!, tint: ALLOY, planes: [pl([side, 0, 1], 0.0185), pl([side, 0, -1], 0.0185), pl([top, 1, 0], 0.0165), pl([top * 0.8, -1, 0], 0.0160), ...(fine ? [pl([side + top, 1, 1], 0.0275), pl([side + top, 1, -1], 0.0275)] : [])] });
  }
}
/** a castor under the tip of an arm: a hood over two wheels, trailing as it was last rolled */
function castor(p: Put, i: number, fine: boolean, q: () => number) {
  const { out } = arm(i), stem = mul(out, TIP), yaw = q() * 6.283, way: V3 = [Math.cos(yaw), 0, Math.sin(yaw)], axle: V3 = [-Math.sin(yaw), 0, Math.cos(yaw)], at = add(stem, add(mul(way, 0.017), [0, 0.0265, 0]));
  p.bar(add(stem, [0, 0.040, 0]), add(stem, [0, 0.068, 0]), 0.0072, 0.0071, M.steel!, tone(ALLOY, 0.9));
  const r: Rot = [way, axle, cross(way, axle)], wheel = (o: number, half: number, rad: number, c: V3, mat: number) => p.add({ at: add(at, mul(axle, o)), rot: r, half: [rad, half, rad], mat, tint: c, quad: [1 / rad ** 2, 0, 1 / rad ** 2, 0] });
  if (!fine) { wheel(0, 0.0250, 0.0265, tone(BLACK, 1.02), M.plastic!); return; }
  for (const s of [-1, 1]) { wheel(s * 0.0150, 0.0085, 0.0265, [1.35, 1.35, 1.35], M.rubber!); wheel(s * 0.0236, 0.0016, 0.0165, [0.47, 0.47, 0.48], M.plastic!); }
  p.add({ at: add(at, [0, 0.003, 0]), rot: r, half: [0.0285, 0.0062, 0.0285], mat: M.plastic!, tint: tone(BLACK, 1.1), quad: [1 / 0.0285 ** 2, 0, 1 / 0.0285 ** 2, 0] });
  p.add({ at: add(add(at, mul(way, -0.007)), [0, 0.0155, 0]), rot: [way, [0, 1, 0], cross(way, [0, 1, 0])], half: [0.0245, 0.0075, 0.0245], mat: M.plastic!, tint: tone(BLACK, 1.1), quad: [1 / 0.0245 ** 2, 1 / 0.0165 ** 2, 1 / 0.0245 ** 2, 0] });
}
function moves(p: Put, seat: Joint, back: () => Joint): Live { return { step(_, t) { seat.rot = rotY(0.62 * Math.sin(t * 0.55) + 0.12 * Math.sin(t * 1.3)); back().rot = rotX(-0.11 * (1 - Math.cos(t * 0.95))); p.pose(); } }; }
/** what turns: what the seat is made of, and (on it) the back */
function turns(p: Put, seat: () => void, back: () => void): Live { let b!: Joint; const s = p.joint([0, 0.36, 0], () => { seat(); b = p.joint(PIN, back); }); return moves(p, s, () => b); }
/** an armrest: a loop from the rail, a pad on it */
function rest(p: Put, s: number, fine: boolean) {
  const front: V3 = [s * 0.283, 0.640, 0.075], rear: V3 = [s * 0.283, 0.640, -0.085], low: V3 = [s * (X + 0.004), 0.446, 0.035], lowRear: V3 = [s * (X + 0.004), 0.440, -0.055];
  p.bar(low, front, 0.0125, 0.0100, M.steel!, ALLOY); p.bar(lowRear, rear, 0.0125, 0.0100, M.steel!, ALLOY);
  p.add({ at: [s * 0.283, 0.6495, -0.004], half: [0.0265, 0.0130, 0.130], mat: M.plastic!, tint: BLACK, ...(fine ? { curves: [{ at: [0, -0.030, 0], r: [0.045, 0.043, 0] }, { at: [0, 0, 0], r: [0.0305, 0, 0.1345] }] as { at: V3; r: V3 }[] } : {}) });
}
export const officechair: ObjectKind = { name: 'officechair', title: 'Office chair', make: (): Made => ({
  name: 'officechair', title: 'Office chair', size: [0.66, 0.99, 0.66], on: 'floor', close: { at: [0.04, 0.55, 0.02], far: 2.5 }, read: 0, moves: { kind: 'works' },
  levels: [
    { shape: 'pieces', error: 0, less: '', put(p) {
      const q = rng(p.seed * 53 + 11); star(p, true); for (let i = 0; i < 5; i++) castor(p, i, true, q);
      p.drum([0, 0.100, 0], 0.040, 0.026, M.steel!, ALLOY); p.drum([0, 0.129, 0], 0.033, 0.006, M.steel!, tone(ALLOY, 0.97)); p.drum([0, 0.190, 0], 0.0270, 0.058, M.plastic!, BLACK); p.drum([0, 0.265, 0], 0.0232, 0.030, M.plastic!, tone(BLACK, 0.95)); p.drum([0, 0.335, 0], 0.0142, 0.045, M.chrome!, [1, 1, 1]);
      return turns(p, () => {
        p.box([0, 0.394, -0.010], [0.062, 0.0215, 0.098], M.plastic!, BLACK, { planes: [[[0, -0.894, 0.447], 0.052], [[0, -0.894, -0.447], 0.052], [[0.70710678, -0.70710678, 0], 0.050], [[-0.70710678, -0.70710678, 0], 0.050]] }); p.drum([0, 0.366, 0], 0.029, 0.010, M.plastic!, tone(BLACK, 0.9));
        for (const z of [-0.078, 0.062]) for (const s of [-1, 1]) p.bar([s * 0.030, 0.404, z], [s * (X - 0.004), z > 0 ? 0.449 : 0.4425, z * 1.45], 0.0135, 0.0095, M.steel!, ALLOY);
        p.bar([0.05, 0.394, 0.045], [0.195, 0.384, 0.060], 0.0045, 0.0044, M.steel!, tone(ALLOY, 0.9)); p.add({ at: [0.215, 0.3835, 0.0625], half: [0.026, 0.0050, 0.0115], mat: M.plastic!, tint: BLACK }); p.bar([0, 0.383, 0.085], [0, 0.378, 0.125], 0.017, 0.0165, M.plastic!, tone(BLACK, 1.1));
        channels(p, SEAT, 10); rails(p, [...SEAT, HINGE], [0, 1, 2, 3, 5, 6, 8], RAIL); for (const s of [-1, 1]) { rest(p, s, true); p.bar([s * (X - 0.016), PIN[1], PIN[2]], [s * (X + 0.016), PIN[1], PIN[2]], 0.0185, 0.0184, M.plastic!, BLACK); }
        p.bar([-X, ...SEAT[0]!], [X, ...SEAT[0]!], 0.0105, 0.0104, M.steel!, ALLOY);
      }, () => {
        channels(p, BACK, 10); const l: YZ[] = [HINGE, ...BACK]; rails(p, l, [0, 1, 3, 4, 5, 6, 8], RAIL); p.bar([-X, ...BACK[7]!], [X, ...BACK[7]!], 0.0115, 0.0114, M.steel!, ALLOY); p.bar([-X, BACK[0]![0] - 0.004, BACK[0]![1] - 0.012], [X, BACK[0]![0] - 0.004, BACK[0]![1] - 0.012], 0.0085, 0.0084, M.steel!, ALLOY);
      });
    } },
    { shape: 'pieces', error: 0.010, less: 'the castors\' wheels apart, levers, the rails\' bends', put(p) {
      const q = rng(p.seed * 53 + 11); star(p, false); for (let i = 0; i < 5; i++) castor(p, i, false, q);
      p.drum([0, 0.104, 0], 0.040, 0.030, M.steel!, ALLOY); p.drum([0, 0.215, 0], 0.0262, 0.082, M.plastic!, BLACK); p.drum([0, 0.335, 0], 0.0142, 0.045, M.chrome!, [1, 1, 1]);
      return turns(p, () => {
        p.box([0, 0.393, -0.008], [0.056, 0.0205, 0.092], M.plastic!, BLACK); for (const z of [-0.078, 0.062]) p.bar([-X + 0.004, z > 0 ? 0.429 : 0.424, z * 1.3], [X - 0.004, z > 0 ? 0.429 : 0.424, z * 1.3], 0.0110, 0.0109, M.steel!, ALLOY);
        channels(p, SEAT, 10); rails(p, [...SEAT, HINGE], [0, 2, 4, 8], RAIL); for (const s of [-1, 1]) rest(p, s, false);
      }, () => { channels(p, BACK, 10); rails(p, [HINGE, ...BACK], [0, 3, 5, 8], RAIL); p.bar([-X, ...BACK[7]!], [X, ...BACK[7]!], 0.0115, 0.0114, M.steel!, ALLOY); });
    } },
    { shape: 'pieces', error: 0.030, less: 'channels are slabs; castors are balls', put(p) {
      for (let i = 0; i < 5; i++) { const { out } = arm(i); p.bar([0, 0.104, 0], add(mul(out, TIP), [0, 0.072, 0]), 0.026, 0.0185, M.steel!, ALLOY); p.ball(add(mul(out, TIP), [0, 0.030, 0]), [0.029, 0.030, 0.029], M.plastic!, BLACK); }
      p.drum([0, 0.23, 0], 0.027, 0.15, M.plastic!, tone(BLACK, 1.15));
      return turns(p, () => { p.box([0, 0.402, -0.008], [0.056, 0.0265, 0.092], M.plastic!, BLACK); slabs(p, SEAT, [[0.004, 0.062], [0.062, 0.436]], 0.0185, 1.0); rails(p, [...SEAT, HINGE], [1, 8], 0.0135); for (const s of [-1, 1]) rest(p, s, false); },
        () => { slabs(p, BACK, [[0, 0.22], [0.22, 0.40], [0.40, 0.517]], 0.0185, 1.0); rails(p, [HINGE, ...BACK], [0, 8], 0.0135); });
    } },
    { shape: 'pieces', error: 0.090, less: 'all but star, post, seat, back and arms', put(p) {
      for (let i = 0; i < 5; i++) { const { out } = arm(i); p.bar([0, 0.095, 0], add(mul(out, TIP + 0.02), [0, 0.040, 0]), 0.024, 0.030, M.steel!, tone(ALLOY, 0.85)); }
      p.drum([0, 0.24, 0], 0.030, 0.18, M.plastic!, tone(BLACK, 1.2));
      return turns(p, () => { p.add({ at: [0, 0.438, 0.02], half: [W + 0.016, 0.022, 0.215], mat: M.canvas!, tint: FAR }); for (const s of [-1, 1]) p.box([s * 0.262, 0.545, -0.004], [0.014, 0.110, 0.055], M.plastic!, tone(BLACK, 1.5)); },
        () => { slabs(p, BACK, [[-0.02, 0.517]], 0.020, 1, W + 0.016, FAR); });
    } },
  ],
}) };
