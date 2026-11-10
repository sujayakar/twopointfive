// A CITY BICYCLE on its kickstand: made upright, its front turned a little on the steering axis, then leaned as one. A wheel is a ring of segments cut by one round curve, so its outline is round however few
// they are. Coarser: fewer segments; the spokes of one side and the other become one between them, as thick as both; what is small gives its color to what holds it.
import { type V3, type Rot, add, sub, mul, norm, dot, apply, compose, rotX, rotZ, along } from '../math';
import { F, type Piece } from '../scene/world';
import { M } from '../scene/materials';
import { Put, apart, type Made, type ObjectKind } from './object';
const PAINT: V3 = [0.17, 0.49, 0.46], LEATHER: V3 = [0.62, 0.37, 0.20], WOOD: V3 = [0.76, 0.56, 0.35], ALLOY: V3 = [0.92, 0.92, 0.93], DARK: V3 = [0.20, 0.20, 0.21], BLACK: V3 = [1, 1, 1], /** brushed alloy, where it is wide: as bright as steel, and no mirror */ SATIN: V3 = [0.76, 0.765, 0.78], SPOKE: V3 = [0.70, 0.70, 0.72], CREAM: V3 = [0.92, 0.89, 0.80], RED: V3 = [0.85, 0.07, 0.05], AMBER: V3 = [1.0, 0.50, 0.08];
// ---- where things are, upright: x to its left, y up, z forward; the cranks turn over z = 0
const R = 0.345, TUBE = 0.0195, RIM = 0.306, REAR: V3 = [0, R, -0.447], FRONT: V3 = [0, R, 0.640], BB: V3 = [0, 0.285, 0], LEAN = 0.15, STEER = 0.24, TURN = Math.PI * 2, SPOKES = 28;
/** up the steering axis, and forward square to it: a place by how far up, forward, aside */
const UP: V3 = [0, Math.sin(1.204), -Math.cos(1.204)], FWD: V3 = [0, Math.cos(1.204), Math.sin(1.204)], FOOT = sub(FRONT, mul(FWD, 0.055)), fork = (s: number, r = 0, x = 0): V3 => add(add(add(FOOT, mul(UP, s)), mul(FWD, r)), [x, 0, 0]);
const SEAT: V3 = norm([0, 0.940, -0.342]), seat = (s: number, x = 0, z = 0): V3 => add(add(BB, mul(SEAT, s)), [x, 0, z]);
/** round, its axis to the side */
const AXLE: Rot = [[0, 0, 1], [1, 0, 0], [0, 1, 0]];
const disc = (p: Put, at: V3, r: number, half: number, mat: number, tint: V3, more: Partial<Piece> = {}) => p.add({ at, half: [r, half, r], mat, tint, rot: AXLE, quad: [1 / r ** 2, 0, 1 / r ** 2, 0], ...more });
/** bars end to end, a ball where two meet */
const pipe = (p: Put, pts: V3[], r: number, mat: number, tint: V3) => { for (let i = 0; i + 1 < pts.length; i++) { p.bar(pts[i]!, pts[i + 1]!, r, r, mat, tint); if (i) p.ball(pts[i]!, [r, r, r], mat, tint); } };
/** a smooth way through some places, `n` steps between each two */
function curve(pts: V3[], n: number): V3[] { const out: V3[] = []; for (let i = 0; i + 1 < pts.length; i++) { const a = pts[Math.max(0, i - 1)]!, b = pts[i]!, c = pts[i + 1]!, d = pts[Math.min(pts.length - 1, i + 2)]!; for (let k = 0; k < n; k++) { const t = k / n; out.push([0, 1, 2].map(x => 0.5 * (2 * b[x]! + (c[x]! - a[x]!) * t + (2 * a[x]! - 5 * b[x]! + 4 * c[x]! - d[x]!) * t * t + (3 * b[x]! - a[x]! - 3 * c[x]! + d[x]!) * t ** 3)) as V3); } } out.push(pts[pts.length - 1]!); return out; }
/** a flat bar from a to b: so wide to the side, so thick */
const flat = (p: Put, a: V3, b: V3, wide: number, thick: number, mat: number, tint: V3) => p.add({ at: mul(add(a, b), 0.5), half: [thick, Math.hypot(...sub(b, a)) / 2, wide], rot: along(sub(b, a), [1, 0, 0]), mat, tint });
const both = (f: (s: number) => void) => { f(1); f(-1); };
/** part of a ring about a hub: `n` segments from a0 to a1 (0 is over the hub, then forward), their middle `rc` from it, so wide and deep; cut by a curve about the hub, or round as a tube */
function ring(p: Put, hub: V3, n: number, a0: number, a1: number, rc: number, half: [number, number], mat: number, tint: V3, o: { tube?: number; curve?: V3 } = {}) {
  const d = (a1 - a0) / n / 2, e = Math.abs(d) + 0.0004; for (let i = 0; i < n; i++) { const a = a0 + (2 * i + 1) * d, r: V3 = [0, Math.cos(a), Math.sin(a)], t: V3 = [0, -Math.sin(a), Math.cos(a)], w = rc * Math.sin(e);
    p.add({ at: add(hub, mul(r, rc)), half: [half[0], (rc + half[1]) * Math.tan(e), half[1]], rot: [[1, 0, 0], t, mul(r, -1)], mat, tint, planes: [[[0, Math.cos(e), Math.sin(e)], w], [[0, -Math.cos(e), Math.sin(e)], w]], quad: [1 / half[0] ** 2, 0, o.tube ? 1 / o.tube ** 2 : 0, 0] /* (its flat sides are a curve's, so that no segment shares a face's plane with the next) */, ...(o.curve ? { curves: [{ at: [0, 0, rc] as V3, r: o.curve }] } : {}) }); }
}
interface Wheel { hub: V3; flange: number; wide: number; shell: number }
const BACK: Wheel = { hub: REAR, flange: 0.037, wide: 0.027, shell: 0.031 }, FORE: Wheel = { hub: FRONT, flange: 0.033, wide: 0.030, shell: 0.025 };
/** both ends of a spoke: pairs lead and trail in turn, one of each pair from either flange */
function spoke(q: Wheel, k: number): [V3, V3] { const a = (k + 0.5) / SPOKES * TURN, s = k % 2 ? 1 : -1, b = a + (Math.floor(k / 2) % 2 ? 1.15 : -1.15), f = q.flange - 0.003; return [add(q.hub, [s * q.wide, Math.cos(b) * f, Math.sin(b) * f]), add(q.hub, [s * 0.003, Math.cos(a) * RIM, Math.sin(a) * RIM])]; }
/** a place by the hub, as far from its axle as it should be */
const outTo = (hub: V3, v: V3, r: number): V3 => { const k = r / Math.hypot(v[1] - hub[1], v[2] - hub[2]); return [v[0], hub[1] + (v[1] - hub[1]) * k, hub[2] + (v[2] - hub[2]) * k]; };
/** every `per` of them as one between them */
function spokes(p: Put, q: Wheel, per: number, r: number, mat: number, tint: V3) { for (let k = 0; k < SPOKES; k += per) { let a: V3 = [0, 0, 0], b: V3 = [0, 0, 0]; for (let i = 0; i < per; i++) { const [x, y] = spoke(q, k + i); a = add(a, mul(x, 1 / per)); b = add(b, mul(y, 1 / per)); } p.bar(outTo(q.hub, a, per > 1 ? 0.022 : q.flange - 0.003), outTo(q.hub, b, RIM), r, r, mat, tint); } }
function wheel(p: Put, q: Wheel, lv: number) {
  const h = q.hub, at = (x: number): V3 => add(h, [x, 0, 0]);
  if (lv < 3) { const n = [32, 13, 10][lv]!; ring(p, h, n, 0, TURN, R - TUBE, [TUBE, TUBE], M.rubber!, BLACK, lv < 2 ? { tube: TUBE, curve: [0.0825, R, R] } : { curve: [0.0825, R, R] }); ring(p, h, n, 0, TURN, [0.3095, 0.3105, 0.3110][lv]!, [[0.0125, 0.0125, 0.0135][lv]!, [0.0085, 0.0085, 0.0105][lv]!], M.paint!, SATIN, { curve: [0, 0.3185, 0.3185] }); }
  else ring(p, h, 8, 0, TURN, 0.3240, [0.0195, 0.0215], M.plastic!, [0.385, 0.385, 0.39], { curve: [0.1, R, R] });
  spokes(p, q, lv < 2 ? 1 : lv === 2 ? 2 : 4, [0.00115, 0.0012, 0.0028, 0.0080][lv]!, M.print!, SPOKE);
  if (lv < 2) { both(s => disc(p, at(s * q.wide), q.flange, 0.0022, M.paint!, SATIN)); disc(p, h, q.shell, q.wide - 0.0005, M.paint!, SATIN); disc(p, h, 0.006, 0.066, M.steel!, BLACK); } else disc(p, h, q.flange, q.wide + 0.004, M.paint!, SATIN);
  if (lv === 0) both(s => disc(p, at(s * 0.048), 0.0115, 0.0075, M.steel!, BLACK));
  for (const k of [2, 16]) { /* (a reflector, on two spokes) */ const on = (lv < 3 ? [k, k + 1] : [0, 1, 2, 3].map(i => k - k % 4 + i)).map(i => spoke(q, i)), m = (t: number) => mul(on.reduce((v, [a, b]) => add(v, add(a, mul(sub(b, a), t))), [0, 0, 0] as V3), 1 / on.length); p.add({ at: m(0.68), half: [0.0215, 0.0185, 0.0065], rot: along(sub(m(1), m(0)), [1, 0, 0]), mat: M.film!, tint: AMBER }); }
}
/** what does not turn with the handlebar */
function body(p: Put, lv: number) {
  const G = M.paint!, tube = (a: V3, b: V3, r0: number, r1 = r0) => p.bar(a, b, r0, r1, G, PAINT), low = fork(0.430), high = fork(0.507), fine = lv === 0;
  tube(BB, seat(0.545), 0.0145); tube(seat(0.505), high, 0.0142); tube(BB, low, 0.0160); tube(fork(0.402), fork(0.535), 0.0188); disc(p, BB, 0.0215, 0.036, G, PAINT);
  if (fine) { p.ball(low, [0.0198, 0.0198, 0.0198], G, PAINT); p.ball(high, [0.0196, 0.0196, 0.0196], G, PAINT); p.ball(seat(0.505), [0.0175, 0.0175, 0.0175], G, PAINT); p.bar(seat(0.535), seat(0.552), 0.0168, 0.0168, M.steel!, BLACK); }
  const bend: V3 = [0.031, 0.306, -0.150], crook = seat(0.395, 0.030, -0.080);
  both(s => { const x = (v: V3): V3 => [v[0] * s, v[1], v[2]];
    if (lv < 2) { tube(x([0.024, 0.287, -0.012]), x(bend), 0.0098, 0.0092); tube(x(bend), x([0.057, 0.343, -0.434]), 0.0092, 0.0066); tube(x(add(seat(0.490), [0.010, 0, 0])), x(crook), 0.0078); tube(x(crook), x([0.057, 0.352, -0.441]), 0.0078, 0.0060); } else { tube(x([0.024, 0.288, -0.02]), x([0.055, 0.343, -0.434]), 0.0105, 0.0085); tube(x(add(seat(0.485), [0.010, 0, 0])), x([0.055, 0.352, -0.441]), 0.0088, 0.0074); }
    if (fine) { p.ball(x(bend), [0.0093, 0.0093, 0.0093], G, PAINT); p.ball(x(crook), [0.0079, 0.0079, 0.0079], G, PAINT); disc(p, x([0.057, 0.346, -0.444]), 0.019, 0.0028, G, PAINT); } });
  // mudguard
  ring(p, REAR, [13, 8, 5, 4][lv]!, -1.95, 1.05, 0.3685, [0.0265, 0.0065], G, PAINT, { curve: [0.130, 0.375, 0.375] });
  if (lv < 2) { const tip = -1.62, e: V3 = [0, R + Math.cos(tip) * 0.368, REAR[2] + Math.sin(tip) * 0.368]; both(s => p.bar([s * 0.062, 0.347, -0.450], [s * 0.026, e[1], e[2]], 0.0024, 0.0024, M.steel!, BLACK)); }
  // rack
  const deck = 0.768, bars = fine ? [-0.044, 0, 0.044] : [-0.033, 0.033]; if (lv < 3) { for (const x of bars) p.box([x, deck, -0.475], [fine ? 0.0175 : 0.0265, 0.0055, 0.195], M.print!, WOOD); } else p.box([0, deck, -0.475], [0.056, 0.0055, 0.195], M.print!, WOOD);
  if (lv < 2) { for (const z of [-0.315, -0.475, -0.635]) p.bar([-0.066, deck - 0.010, z], [0.066, deck - 0.010, z], 0.0048, 0.0048, M.steel!, DARK); both(s => p.bar([s * 0.034, deck - 0.010, -0.290], add(seat(0.440), [s * 0.013, 0, -0.005]), 0.0042, 0.0042, M.steel!, DARK)); }
  both(s => { p.bar([s * 0.064, deck - 0.010, -0.545], [s * 0.066, 0.350, -0.452], [0.0046, 0.0046, 0.0092, 0.0110][lv]!, [0.0046, 0.0046, 0.0092, 0.0110][lv]!, M.steel!, DARK); if (lv < 2) { p.bar([s * 0.064, deck - 0.010, -0.400], [s * 0.066, 0.350, -0.447], 0.0042, 0.0042, M.steel!, DARK); } });
  if (lv < 3) p.box([0, deck - 0.028, -0.668], [0.034, 0.020, 0.011], M.film!, RED); if (fine) p.box([0, deck - 0.016, -0.655], [0.012, 0.005, 0.020], M.steel!, DARK);
  // saddle
  const S: V3 = [0, 0.932, -0.262], cut: [V3, number][] = [[[0, -1, 0], 0.016]]; p.bar(seat(0.50), seat(0.672), 0.0128, 0.0128, M.steel!, BLACK);
  const pad: [number, number, number][] = lv < 2 ? (fine ? [[-0.055, 0.086, 0.074], [-0.028, 0.084, 0.080], [0.002, 0.071, 0.080], [0.032, 0.056, 0.078], [0.062, 0.044, 0.076], [0.098, 0.035, 0.074], [0.128, 0.031, 0.070]] : [[-0.045, 0.088, 0.085], [0.010, 0.066, 0.092], [0.095, 0.036, 0.104]]) : [[-0.038, 0.087, 0.090], [0.080, 0.042, 0.118]]; for (const [z, wide, long] of pad) p.ball(add(S, [0, 0.001, z]), [wide, 0.030 + 0.07 * Math.abs(z - 0.04) * (z < 0.04 ? 1 : 0.15), long], M.plastic!, LEATHER, { planes: cut });
  if (lv < 2) both(s => { p.bar(add(S, [s * 0.046, -0.052, -0.075]), add(S, [s * 0.046, -0.010, -0.075]), 0.0145, 0.0145, M.steel!, [0.6, 0.6, 0.6]); p.bar(add(S, [s * 0.046, -0.050, -0.070]), add(S, [s * 0.020, -0.028, 0.160]), 0.0036, 0.0036, M.steel!, BLACK); });
  if (fine) { both(s => { for (let i = 0; i < 4; i++) p.drum(add(S, [s * 0.046, -0.0475 + i * 0.0105, -0.075]), 0.0185, 0.0027, M.steel!, ALLOY); }); disc(p, seat(0.645), 0.0165, 0.030, M.steel!, ALLOY); }
  // cranks, chain
  const A = -0.46, dir: V3 = [0, Math.sin(A), Math.cos(A)], ring0 = 0.088, cog = 0.037, X = -0.0455;
  disc(p, [X, BB[1], BB[2]], 0.083, 0.0045, M.paint!, SATIN); disc(p, [X, BB[1], BB[2]], 0.0925, 0.0012, M.steel!, DARK); if (fine) { for (let i = 0; i < 5; i++) { const a = i / 5 * TURN + 0.3; disc(p, [X, BB[1] + Math.cos(a) * 0.054, Math.sin(a) * 0.054], 0.0185, 0.0062, M.rubber!, DARK); } disc(p, [X - 0.0045, BB[1], BB[2]], 0.027, 0.006, M.steel!, ALLOY); } disc(p, BB, 0.010, 0.079, M.steel!, ALLOY);
  both(s => { const c: V3 = [s * 0.0745, BB[1], BB[2]], e = add(add(c, mul(dir, -s * 0.170)), [s * 0.006, 0, 0]); flat(p, add(c, mul(dir, s * 0.013)), add(e, mul(dir, -s * 0.010)), 0.0065, 0.0115, M.paint!, SATIN);
    const t = add(e, [s * 0.062, 0, 0]); if (fine) { for (const z of [-0.026, 0.026]) p.box(add(t, [0, 0, z]), [0.043, 0.0125, 0.0115], M.rubber!, BLACK); both(k => p.box(add(t, [0, 0, k * 0.0385]), [0.026, 0.0065, 0.0025], M.film!, AMBER)); disc(p, add(t, [s * -0.008, 0, 0]), 0.0085, 0.050, M.steel!, ALLOY); both(k => p.box(add(t, [k * 0.0430, 0, 0]), [0.0035, 0.0095, 0.027], M.steel!, DARK)); } else p.box(add(t, [s * -0.006, 0, 0]), [0.049, 0.0115, 0.0385], M.plastic!, [0.30, 0.275, 0.26]); });
  { const run = (k: number, r0 = ring0, r1 = cog) => { /* where a straight run leaves one and meets the other */ const d = sub(REAR, BB), l = Math.hypot(d[1], d[2]), b = Math.asin((r0 - r1) / l), u = norm(d), nrm: V3 = [0, k * u[2] * -1, k * u[1]], q: V3 = add(mul(nrm, Math.cos(b)), mul(u, Math.sin(b))); return [add([X, BB[1], BB[2]], mul(q, r0)), add([X, REAR[1], REAR[2]], mul(q, r1))] as const; };
    for (const k of [1, -1]) { const [a, b] = run(k); flat(p, a, b, 0.0028, lv < 3 ? 0.0042 : 0.0055, M.steel!, DARK); } if (lv < 3) disc(p, [X, REAR[1], REAR[2]], cog + 0.004, 0.0022, M.steel!, DARK);
    // the guard over it
    { const [a, b] = run(1, 0.108, 0.064), d = norm(sub(b, a)), down: V3 = [0, d[2], -d[1]], e = add(a, mul(sub(b, a), 0.95)), q = (v: V3, k: number): V3 => add(add(v, mul(down, k)), [-0.018, 0, 0]); p.add({ at: q(mul(add(a, e), 0.5), 0.025), half: [0.025, Math.hypot(...sub(e, a)) / 2, 0.0018], rot: along(d, [1, 0, 0]), mat: G, tint: PAINT }); disc(p, [X - 0.018, BB[1], BB[2]], 0.108, 0.0035, G, PAINT, { planes: [[norm([-0.25, 0, -1]), -0.038]] });
      if (fine) disc(p, q(e, 0.025), 0.0248, 0.0035, G, PAINT); if (lv < 3) p.bar([-0.040, 0.318, -0.20], add(q(a, 0.04), [0, 0, -0.19]), 0.0042, 0.0042, M.steel!, DARK); } }
  // the stand
  p.bar([0.038, 0.290, -0.085], [0.2035, 0.041, -0.1525], lv < 2 ? 0.0082 : 0.0095, lv < 2 ? 0.0078 : 0.009, M.paint!, SATIN); if (lv < 2) { p.add({ at: [0.2045, (0.0062 + 0.2045 * Math.sin(LEAN)) / Math.cos(LEAN), -0.153], half: [0.0165, 0.0062, 0.0165], mat: M.rubber!, tint: BLACK, rot: rotZ(LEAN), quad: [1 / 0.0165 ** 2, 0, 1 / 0.0165 ** 2, 0] }); p.box([0.020, 0.296, -0.085], [0.034, 0.010, 0.016], M.steel!, DARK); }
}
/** the handlebar's way, from its clamp to a grip */
const WAY: V3[] = [[0, 0, 0], [0.060, 0.006, 0.014], [0.125, 0.030, 0.034], [0.185, 0.046, 0.034], [0.232, 0.050, 0.008], [0.262, 0.047, -0.040], [0.280, 0.041, -0.095]];
/** what turns with the handlebar */
function front(p: Put, lv: number) {
  const G = M.paint!, C = M.steel!, fine = lv === 0; wheel(p, FORE, lv);
  both(s => { if (lv < 2) { const k = [fork(0.386, 0, s * 0.047), fork(0.150, 0.003, s * 0.053), fork(0.062, 0.019, s * 0.054), fork(-0.004, 0.0545, s * 0.054)]; p.bar(k[0]!, k[1]!, 0.0125, 0.0108, G, PAINT); p.bar(k[1]!, k[2]!, 0.0108, 0.0090, G, PAINT); p.bar(k[2]!, k[3]!, 0.0090, 0.0072, G, PAINT); if (fine) { p.ball(k[1]!, [0.0109, 0.0109, 0.0109], G, PAINT); p.ball(k[2]!, [0.0091, 0.0091, 0.0091], G, PAINT); disc(p, k[3]!, 0.015, 0.0035, G, PAINT); } } else p.bar(fork(0.386, 0, s * 0.047), fork(0, 0.050, s * 0.054), 0.0122, 0.0085, G, PAINT); });
  p.bar(fork(0.386, 0, 0.060), fork(0.386, 0, -0.060), 0.0158, 0.0158, G, PAINT); if (fine) { both(s => p.ball(fork(0.386, 0, s * 0.052), [0.016, 0.0158, 0.0158], G, PAINT)); p.bar(fork(0.394), fork(0.407), 0.0225, 0.0215, C, BLACK); p.bar(fork(0.531), fork(0.549), 0.0215, 0.0185, C, BLACK); }
  ring(p, FRONT, [10, 6, 4, 3][lv]!, -1.95, 0.45, 0.3685, [0.0265, 0.0065], G, PAINT, { curve: [0.130, 0.375, 0.375] });
  if (lv < 2) { const tip = -1.70, e: V3 = [0, R + Math.cos(tip) * 0.368, FRONT[2] + Math.sin(tip) * 0.368]; both(s => p.bar([s * 0.060, 0.346, 0.638], [s * 0.026, e[1], e[2]], 0.0024, 0.0024, C, BLACK)); }
  // stem and bar
  const top = fork(0.672), clamp = add(top, [0, 0.004, 0.062]); p.bar(fork(0.535), add(top, mul(UP, 0.008)), 0.0125, 0.0125, C, BLACK); p.bar(top, clamp, 0.0135, 0.0135, C, BLACK); disc(p, clamp, 0.0185, 0.0195, C, BLACK); if (lv < 2) { p.ball(add(top, mul(UP, 0.006)), [0.0145, 0.0145, 0.0145], C, BLACK); }
  both(s => { const at = (x: number, y: number, z: number): V3 => add(clamp, [s * x, y, z]), path = WAY.map(([x, y, z]) => at(x, y, z));
    if (fine) pipe(p, curve(path, 2), 0.0112, C, BLACK); else if (lv < 3) { const q = curve(path, 2); for (let i = 0; i + 1 < q.length; i++) p.bar(q[i]!, q[i + 1]!, 0.0112, 0.0112, C, BLACK); } else for (let i = 0; i + 1 < path.length; i++) p.bar(path[i]!, path[i + 1]!, 0.0112, 0.0112, C, BLACK);
    const a = at(0.278, 0.0415, -0.088), b = at(0.302, 0.030, -0.212); p.bar(a, b, 0.0158, 0.0172, M.plastic!, LEATHER); if (fine) { p.ball(b, [0.0172, 0.0172, 0.0172], M.plastic!, LEATHER); p.bar(a, add(a, mul(norm(sub(b, a)), 0.006)), 0.0180, 0.0180, M.plastic!, LEATHER); }
    if (lv > 1) p.bar(at(0.285, 0.030, -0.045), at(0.334, 0.004, -0.190), 0.0062, 0.0056, C, BLACK); if (lv === 2) { const q = fork(0.44, s < 0 ? 0.012 : 0, s * 0.03), k = at(0.170, -0.035, 0.090); p.bar(at(0.250, 0.040, -0.015), k, 0.0032, 0.0032, M.rubber!, BLACK); p.bar(k, q, 0.0032, 0.0032, M.rubber!, BLACK); }
    if (lv < 2) { p.bar(at(0.262, 0.040, -0.030), at(0.306, 0.022, -0.066), 0.0050, 0.0052, C, BLACK); p.bar(at(0.306, 0.022, -0.066), at(0.334, 0.004, -0.190), 0.0052, 0.0046, C, BLACK); } if (fine) { p.ball(at(0.334, 0.004, -0.190), [0.0065, 0.0065, 0.0065], C, BLACK); p.ball(at(0.306, 0.022, -0.066), [0.0054, 0.0054, 0.0054], C, BLACK); p.ball(at(0.258, 0.044, -0.030), [0.0165, 0.0165, 0.0165], C, ALLOY);
      const c = s > 0 ? [at(0.258, 0.036, -0.018), at(0.222, 0.010, 0.066), at(0.110, -0.060, 0.100), at(0.030, -0.150, 0.060), add(fork(0.44), [s * 0.024, 0, 0.012])] : [at(0.258, 0.036, -0.018), at(0.215, 0.000, 0.075), at(0.120, -0.085, 0.108), at(0.066, -0.200, 0.080), fork(0.340, 0.010, s * 0.056), fork(0.10, 0.012, s * 0.064), fork(0.02, 0.040, s * 0.056)]; pipe(p, curve(c, 3), 0.0024, M.rubber!, BLACK); } });
  if (lv < 2) { p.ball(add(clamp, [-0.098, 0.044, 0.026]), [0.027, 0.018, 0.027], M.chrome!, BLACK, { planes: [[[0, -1, 0], 0.006]] }); if (fine) { p.drum(add(clamp, [-0.098, 0.034, 0.026]), 0.0245, 0.0045, M.plastic!, DARK); p.add({ at: fork(0.478, 0.0185), half: [0.0105, 0.0150, 0.0035], mat: M.brass!, tint: BLACK, rot: along(UP, [1, 0, 0]).map((v, i, l) => i === 0 ? [1, 0, 0] : i === 2 ? FWD : l[1]) as Rot, quad: [1 / 0.0105 ** 2, 1 / 0.015 ** 2, 1 / 0.0035 ** 2, 0] }); } }
  // lamp
  const L = fork(0.455, 0.085), ax = along([0, -0.10, 1]); if (lv < 3) { p.add({ at: L, half: [0.036, 0.052, 0.036], mat: M.chrome!, tint: ALLOY, rot: ax, quad: [1 / 0.036 ** 2, 1 / 0.052 ** 2, 1 / 0.036 ** 2, 0], planes: [[[0, 1, 0], 0.016]] }); } if (lv < 2) { p.add({ at: add(L, mul(ax[1], 0.0185)), half: [0.0375, 0.0045, 0.0375], mat: C, tint: BLACK, rot: ax, quad: [1 / 0.0375 ** 2, 0, 1 / 0.0375 ** 2, 0] }); p.bar(fork(0.392, 0.012), add(L, mul(UP, -0.030)), 0.0052, 0.0052, M.steel!, DARK); }
  if (lv < 3) p.add({ at: add(L, mul(ax[1], 0.0205)), half: [0.0320, 0.0095, 0.0320], mat: M.film!, tint: CREAM, rot: ax, quad: [1 / 0.032 ** 2, 1 / 0.0095 ** 2, 1 / 0.032 ** 2, 0] });
}
/** turned about a line */
const about = (q: Piece, r: Rot, o: V3): Piece => ({ ...q, at: add(o, apply(r, sub(q.at, o))), rot: compose(r, q.rot) });
function turnOn(axis: V3, a: number): Rot { const c = Math.cos(a), s = Math.sin(a), k = norm(axis), f = (v: V3): V3 => { const d = dot(k, v), x: V3 = [k[1] * v[2] - k[2] * v[1], k[2] * v[0] - k[0] * v[2], k[0] * v[1] - k[1] * v[0]]; return [0, 1, 2].map(i => v[i]! * c + x[i]! * s + k[i]! * d * (1 - c)) as V3; }; return [f([1, 0, 0]), f([0, 1, 0]), f([0, 0, 1])]; }
const made = (f: (p: Put) => void) => apart(w => f(new Put(w, 'bicycle', [0, 0, 0], 0))).pieces;
const HEAD = turnOn(UP, STEER), LEANED = rotZ(-LEAN);
/** how far under a wheel's hub it touches, its axle pointing so */
const drop = (axle: V3) => Math.sqrt(R * R * (1 - axle[1] ** 2) + (0.0825 * axle[1]) ** 2);
/** leaned, and pitched so that both wheels touch */
const [POSE, SHIFT] = ((): [Rot, V3] => { const hub = (r: Rot, f: V3) => apply(r, f), foreHub = add(FOOT, apply(HEAD, sub(FRONT, FOOT))), low = (r: Rot): [number, number] => [hub(r, REAR)[1] - drop(r[0]), hub(r, foreHub)[1] - drop(apply(r, HEAD[0]))]; let r = LEANED; for (let i = 0; i < 4; i++) { const [a, b] = low(r); r = compose(rotX((b - a) / (FRONT[2] - REAR[2])), r); } return [r, [-0.045, -low(r)[0], -0.085]]; })();
/** what is thinner than 3 mm is a square wire, on its edge */
const wire = (q: Piece): Piece => { if (!q.quad || q.planes || q.quad[1] !== 0 || q.half[0] > 0.0031 || q.half[0] !== q.half[2]) return q; const h = q.half[0] * 0.89, [x, y, z] = q.rot, { quad: _, ...rest } = q; void _; return { ...rest, flags: q.flags & ~F.QUAD, half: [h, q.half[1], h], rot: [mul(add(x, z), Math.SQRT1_2), y, mul(sub(z, x), Math.SQRT1_2)] }; };
function put(p: Put, lv: number) { const all = [...made(q => { body(q, lv); wheel(q, BACK, lv); }), ...made(q => front(q, lv)).map(q => about(q, HEAD, FOOT))].map(wire); p.from({ pieces: all.map(q => { const r = about(q, POSE, [0, 0, 0]); return { ...r, at: add(r.at, SHIFT) }; }) }); }
export const bicycle: ObjectKind = { name: 'bicycle', title: 'City bicycle', make: (): Made => ({
  name: 'bicycle', title: 'City bicycle', size: [0.67, 1.07, 1.80], on: 'floor', close: { at: [0.05, 0.62, 0.05], far: 4.0 }, read: 0, moves: { kind: 'loose', dense: 1150, rough: 0.6, floats: false }, crowd: 64,
  levels: [
    { shape: 'pieces', error: 0, less: '', put(p) { put(p, 0); } },
    { shape: 'pieces', error: 0.010, less: 'cables, joints, coils, more than half the segments', put(p) { put(p, 1); } },
    { shape: 'pieces', error: 0.020, less: 'two spokes are one; springs, wires, bell', put(p) { put(p, 2); } },
    { shape: 'pieces', error: 0.036, less: 'four spokes are one; rims are in the tires\' color; lamps, cables', put(p) { put(p, 3); } },
  ],
}) };
