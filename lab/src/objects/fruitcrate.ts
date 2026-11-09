// A MARKET CRATE of mandarins: rough slats on corner posts, fruit heaped over its rim, a paper price on a stick. Near, each fruit with its stem's end; then those that can be seen; then the heap
// as a few lumps; then a box with a dome on it.
import { type V3, type Rot, add, mul, apply, compose, norm, cross, rng, rotX, rotY, rotZ } from '../math';
import { M } from '../scene/materials';
import type { Piece } from '../scene/world';
import type { Made, ObjectKind, Put } from './object';
const X = 0.25, Z = 0.165, RIM = 0.188, POST = 0.206, THICK = 0.0042;
const WOOD: V3 = [0.90, 0.765, 0.59], GRAIN: V3 = [0.80, 0.625, 0.44], PEEL: V3 = [0.905, 0.532, 0.122], PAPER: V3 = [1.0, 0.985, 0.94], STICK: V3 = [0.78, 0.62, 0.36], LEAF: V3 = [0.30, 0.50, 0.17], CALYX: V3 = [0.47, 0.56, 0.25];
/** (what is printed comes out paler than it was drawn) */
const INK: V3 = [0.86, 0.84, 0.81];
const tone = (c: V3, k: number): V3 => [c[0] * k, c[1] * k, c[2] * k];
/** the long sides' slats: middle, half height */
const SLATS: [number, number][] = [[0.046, 0.0235], [0.106, 0.0225], [0.164, 0.0235]];
/** the heap's rows: how many, how high, how far back, how far to the right */
const ROWS: [number, number, number, number][] = [...[0, 1, 2, 3, 4].map(k => [k % 2 ? 6 : 7, 0.156, (k - 2) * 0.0585, 0] as [number, number, number, number]), [5, 0.200, -0.092, 0.0335], [6, 0.204, -0.032, -0.004], [6, 0.204, 0.028, 0.002], [5, 0.200, 0.088, -0.0335], [3, 0.247, -0.052, -0.018], [4, 0.250, 0.004, 0.004], [2, 0.246, 0.058, 0.020]];
interface Fruit { at: V3; half: V3; rot: Rot; tint: V3; top: boolean; tier: number }
/** every fruit: what shows between the slats, then three tiers, each in the hollows of the one under it */
function fruits(seed: number): Fruit[] {
  const q = rng(seed * 977 + 5), out: Fruit[] = [], R = 0.0335, put = (x: number, y: number, z: number, tier: number) => {
    const k = 0.94 + 0.11 * q(), j = 0.004, pole = norm([(q() - 0.5) * 1.5, 1, (q() - 0.5) * 1.5 + (tier ? 0.15 : 0)]), side = norm(cross(pole, [0.3, 0, 1])), ripe = q(), green = q() < 0.08 ? 0.07 : 0;
    out.push({ at: [x + (q() - 0.5) * j, y + (q() - 0.5) * j, z + (q() - 0.5) * j], half: [R * k, R * k * (0.82 + 0.05 * q()), R * k], rot: [side, pole, cross(side, pole)], tint: [PEEL[0] * (0.96 + 0.05 * q()) - green, PEEL[1] * (0.90 + 0.17 * ripe) + green * 0.6, PEEL[2] * (0.75 + 0.6 * ripe)], top: tier > 0, tier });
  };
  for (const y of [0.054, 0.104]) for (const s of [-1, 1]) for (let i = 0; i < 7; i++) put((i - 3) * 0.0655 + (y > 0.08 ? 0.012 : -0.008), y, s * 0.117, 0);
  const row = (n: number, z: number, y: number, tier: number, dx = 0) => { for (let i = 0; i < n; i++) put((i - (n - 1) / 2) * 0.067 + dx, y, z, tier); };
  ROWS.forEach(([n, y, z, dx], k) => { for (let i = 0; i < n; i++) { const corner = k < 5 && k % 4 === 0 && (i === 0 || i === n - 1); put((i - (n - 1) / 2) * 0.067 * (corner ? 0.955 : 1) + dx, y + (corner ? 0.006 : 0), z * (corner ? 0.93 : 1), k < 5 ? 1 : k < 9 ? 2 : 3); } });
  return out;
}
const ball = (p: Put, f: Fruit, more: Partial<Piece> = {}) => p.ball(f.at, f.half, M.print!, f.tint, { rot: f.rot, ...more });
/** half a leaf, folded at its rib: its edge an arc */
function leaf(p: Put, at: V3, yaw: number, tilt: number, long: number) {
  const r = compose(rotY(yaw), rotZ(tilt)), w = long * 0.42, R = (long * long + w * w) / (2 * w);
  for (const s of [-1, 1]) { const q = compose(r, rotX(s * -0.28)); p.add({ at: add(at, apply(q, [s * 0.0012, 0, s * w / 2])), rot: q, half: [long, 0.0007, w / 2 + 0.0004], mat: M.print!, tint: s > 0 ? LEAF : tone(LEAF, 0.9), curves: [{ at: [0, 0, s * (w / 2 - R)], r: [R, 0, R] }] }); }
}
/** where the price stands: the card's middle, and how it is turned */
const CARD: V3 = [0.052, 0.352, -0.062], TURN: Rot = compose(rotY(-0.10), rotX(-0.30)), CW = 0.083, CH = 0.066;
function price(p: Put, fine: boolean) {
  const on = (v: V3) => add(CARD, apply(TURN, v));
  p.add({ at: CARD, rot: TURN, half: [CW, CH, 0.0011], mat: M.print!, tint: PAPER, marks: [{ face: 5, at: [0, 0], size: [2 * CW - 0.012, 2 * CH - 0.012], art: 'tag0', color: INK }] });
  p.bar(on([0, CH + 0.012, -0.0052]), on([0, -0.215, -0.0052]), 0.0042, 0.0038, M.print!, STICK);
  if (!fine) return;
  for (const s of [-1, 1]) p.add({ at: on([0, CH + 0.008, s * 0.0055 - 0.0035]), rot: compose(TURN, rotX(s * 0.10)), half: [0.0058, 0.021, 0.0026], mat: M.print!, tint: [0.86, 0.17, 0.13] });
  p.bar(on([-0.0075, CH + 0.010, -0.0035]), on([0.0075, CH + 0.010, -0.0035]), 0.0044, 0.0043, M.steel!, [1, 1, 1]);
}
/** the wood's figure, as wide bands of a wave far larger than the board */
const grain = (face: number, q: () => number, tint: V3): Piece['marks'] => [{ face, at: [(q() - 0.5) * 0.3, (q() - 0.5) * 0.04], size: [1.1 + 0.6 * q(), 0.10 + 0.04 * q()], art: 'wave', color: [GRAIN[0] * tint[0] / WOOD[0], GRAIN[1] * tint[1] / WOOD[1], GRAIN[2] * tint[2] / WOOD[2]] }];
const label = (s: number, y = 0.001) => ({ face: s > 0 ? 1 : 0, at: [s * 0.006, y] as [number, number], size: [0.128, 0.084] as [number, number], art: 'p57', color: [0.97, 0.77, 0.66] as V3 });
function wood(p: Put, fine: boolean) {
  const q = rng(p.seed * 131 + 9), board = (at: V3, half: V3, more: Partial<Piece> = {}, faces: number[] = []) => { const k = 0.93 + 0.10 * q(), w = 0.97 + 0.06 * q(), t: V3 = [WOOD[0] * k, WOOD[1] * k, WOOD[2] * k * w]; return p.add({ at, half, mat: M['lamp shade']!, tint: t, ...more, ...(fine && faces.length ? { marks: [...faces.flatMap(f => grain(f, q, t)!), ...(more.marks ?? [])] } : {}) }); };
  for (const s of [-1, 1]) {
    SLATS.forEach(([y, h], i) => { board([0, y, s * (Z - THICK)], [X + 0.0015 * ((i + (s > 0 ? 1 : 0)) % 2), h, THICK], { rot: rotZ((q() - 0.5) * 0.010) }, [s > 0 ? 5 : 4]); });
    for (const [y, h] of [[0.0735, 0.0500], [0.1655, 0.0215]] as const) board([s * (X - 0.0115), y, 0], [0.0065, h, Z - 2 * THICK - 0.0005], y < 0.1 ? { marks: [label(s)] } : {}, fine ? [s > 0 ? 1 : 0] : []);
    for (const z of [-1, 1]) p.add({ at: [s * (X - 0.029), POST / 2 + 0.003, z * (Z - 2 * THICK - 0.0125)], half: [0.0110, POST / 2 - 0.003, 0.0110], mat: M['lamp shade']!, tint: tone(WOOD, 0.93 + 0.06 * q()) });
  }
  p.box([0, 0.020, 0], [X - 0.020, 0.0045, Z - 2 * THICK - 0.002], M['lamp shade']!, tone(WOOD, 0.9));
  for (const s of [-1, 1]) p.box([s * 0.16, 0.0082, 0], [0.016, 0.0082, 0.112], M['lamp shade']!, tone(WOOD, 0.88)); /* (it stands on these two, set in: a low wide thing on its corners rocks) */
  if (!fine) return;
  for (const s of [-1, 1]) { for (const [y, h] of SLATS) for (const z of [-1, 1]) for (const d of [-1, 1]) p.drum([s * (X - 0.030 + d * 0.004), y + d * h * 0.5, z * (Z + 0.0006)], 0.0030, 0.0010, M.steel!, [0.62, 0.60, 0.58], { rot: rotX(Math.PI / 2) }); }
}
/** a side from far off: its boards, and what is dark between them */
const SIDE: V3 = [0.810, 0.690, 0.531];
const heapTint = (k: number): V3 => tone([0.892, 0.541, 0.139], k);
export const fruitcrate: ObjectKind = { name: 'fruitcrate', title: 'Fruit crate', make: (): Made => ({
  name: 'fruitcrate', title: 'Fruit crate', size: [0.503, 0.44, 0.334], on: 'floor', close: { at: [0.06, 0.20, 0.04], far: 1.9 }, read: 2.6, moves: { kind: 'loose', dense: 480, rough: 0.6, floats: true }, crowd: 64,
  levels: [
    { shape: 'pieces', error: 0, less: '', put(p) {
      wood(p, true); price(p, true); const fs = fruits(p.seed), q = rng(p.seed * 71 + 3);
      p.box([0, 0.085, 0], [X - 0.034, 0.060, 0.098], M.print!, heapTint(0.80));
      for (const f of fs) { ball(p, f); if (f.top && q() < 0.72) { const at = add(f.at, mul(f.rot[1], f.half[1] - 0.0006)); p.add({ at, rot: f.rot, half: [0.0062, 0.0011, 0.0062], mat: M.print!, tint: CALYX, quad: [1 / 0.0062 ** 2, 0, 1 / 0.0062 ** 2, 0] }); p.add({ at: add(at, mul(f.rot[1], 0.0022)), rot: f.rot, half: [0.0022, 0.0016, 0.0022], mat: M.print!, tint: [0.55, 0.45, 0.26], quad: [1 / 0.0022 ** 2, 0, 1 / 0.0022 ** 2, 0] }); } }
      const tops = fs.filter(f => f.tier > 1); for (const [i, yaw, tilt, long] of [[18, 0.9, 0.30, 0.029], [21, 2.6, 0.25, 0.026], [13, -0.5, 0.40, 0.031], [28, 1.5, 0.28, 0.027], [23, -2.2, 0.30, 0.030], [30, -1.2, 0.20, 0.028]] as const) { const f = tops[i % tops.length]!, pole = add(f.at, mul(f.rot[1], f.half[1] + 0.002)); p.bar(pole, add(pole, apply(rotY(yaw), [0.004, 0.006, 0])), 0.0017, 0.0014, M.print!, [0.40, 0.42, 0.20]); leaf(p, add(pole, apply(compose(rotY(yaw), rotZ(tilt)), [long, 0.005, 0])), yaw, tilt, long); }
    } },
    { shape: 'pieces', error: 0.010, less: 'what shows between the slats, stems and leaves, nails, battens, the peg', put(p) { wood(p, false); price(p, false); p.box([0, 0.083, 0], [X - 0.034, 0.060, Z - 2 * THICK - 0.012], M.print!, heapTint(0.80)); for (const f of fruits(p.seed)) if (f.top) ball(p, f); } },
    { shape: 'pieces', error: 0.022, less: 'fruit: the heap is lumps; slats: a board a side', put(p) {
      for (const s of [-1, 1]) { p.box([0, 0.105, s * (Z - THICK)], [X, 0.083, THICK], M['lamp shade']!, SIDE); p.box([s * (X - 0.0115), 0.105, 0], [0.0065, 0.081, Z - 2 * THICK - 0.001], M['lamp shade']!, tone(WOOD, 0.935), { marks: [label(s, -0.030)] }); }
      p.box([0, 0.082, 0], [X - 0.019, 0.080, Z - 2 * THICK - 0.002], M.print!, heapTint(0.78)); price(p, false);
      const q = rng(p.seed * 37 + 1); for (let i = 0; i < 5; i++) for (let k = 0; k < 3; k++) { const u = (i - 2) / 2, v = k - 1; p.ball([u * 0.180, 0.212 - 0.042 * u * u - 0.028 * v * v, v * 0.094], [0.056, 0.048, 0.060], M.print!, heapTint(0.91 + 0.08 * q())); }
    } },
    { shape: 'pieces', error: 0.040, less: 'all but a box, a dome and the price', put(p) { p.box([0, 0.091, 0], [X - 0.010, 0.089, Z - 0.008], M['lamp shade']!, [0.806, 0.679, 0.517]); p.add({ at: [0, 0.16, 0], half: [X - 0.026, 0.122, Z - 0.027], mat: M.print!, tint: [0.800, 0.463, 0.116], quad: [1 / 0.34 ** 2, 1 / 0.122 ** 2, 1 / 0.225 ** 2, 0] }); p.add({ at: [CARD[0], 0.28, CARD[2] + 0.012], half: [0.004, 0.035, 0.004], mat: M.print!, tint: STICK }); p.add({ at: CARD, rot: TURN, half: [CW, CH, 0.0011], mat: M.print!, tint: PAPER, marks: [{ face: 5, at: [0, 0], size: [2 * CW - 0.012, 2 * CH - 0.012], art: 'tag0', color: INK }] }); } },
  ],
}) };
