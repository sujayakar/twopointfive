// A MOPED on its stand: a pressed steel frame one steps through, the engine under it, cast wheels, a sprung saddle, a carrier. The stand and the back wheel carry it, so the front wheel is off the ground: its fork turns on the head's
// own slanting axis and the wheel turns in the fork. What is round and hollow (tires, mudguards) is made of segments; what its levels must keep is the light through its wheels and under its frame.
import { type V3, type Rot, add, sub, mul, dot, cross, norm, mix, compose, rotX, rotY } from '../math';
import { M } from '../scene/materials';
import type { Live, Made, ObjectKind, Put } from './object';
const MINT: V3 = [0.46, 0.77, 0.67], CREAM: V3 = [0.95, 0.91, 0.82], TAN: V3 = [0.66, 0.40, 0.24], BLACK: V3 = [0.17, 0.17, 0.18], ONE: V3 = [1, 1, 1], ALLOY: V3 = [0.94, 0.94, 0.95], DULL: V3 = [0.80, 0.80, 0.81];
const R = 0.275, REAR: V3 = [0, R, -0.555], FRONT: V3 = [0, R + 0.032, 0.555], D: V3 = [0, 0.899, -0.438], BACK: V3 = [0, -0.438, -0.899];
/** along the steering axis, and along the fork's legs (which stand a little before it) */
const axis = (t: number): V3 => add(add(FRONT, mul(BACK, 0.036)), mul(D, t)), leg = (t: number, x: number): V3 => add(add(FRONT, mul(D, t)), [x, 0, 0]);
const turn = (k: V3, a: number): Rot => { const c = Math.cos(a), s = Math.sin(a), f = (v: V3): V3 => add(add(mul(v, c), mul(cross(k, v), s)), mul(k, dot(k, v) * (1 - c))); return [f([1, 0, 0]), f([0, 1, 0]), f([0, 0, 1])]; };
/** an arc of n segments about an axle, from angle a to b (0 ahead, a quarter turn up): its outside an ellipsoid (so it rounds toward its sides), its inside flat, its sides drawn in toward the axle */
function arc(p: Put, c: V3, r: number, inner: number, wide: number, round: number, lean: number, a: number, b: number, n: number, mat: number, tint: V3) {
  const h = (b - a) / n / 2, m = (r + inner) / 2, l = Math.hypot(1, lean); for (let i = 0; i < n; i++) { const t = a + h * (2 * i + 1), y: V3 = [0, Math.sin(t), Math.cos(t)], z: V3 = [0, -y[2], y[1]]; p.add({ at: add(c, mul(y, m)), half: [wide + 0.012, (r - inner) / 2, r * Math.sin(h) + 0.0015], rot: [[1, 0, 0], y, z], mat, tint, curves: [{ at: [0, -m, 0], r: [round, r, r] }], planes: [[[0, -Math.sin(h), Math.cos(h)], m * Math.sin(h) + 0.0004], [[0, -Math.sin(h), -Math.cos(h)], m * Math.sin(h) + 0.0004], [[1 / l, -lean / l, 0], (wide - lean * (r - inner) / 2) / l], [[-1 / l, -lean / l, 0], (wide - lean * (r - inner) / 2) / l]] }); }
}
/** a drum across (along x) */
const across = (p: Put, at: V3, r: number, half: number, mat: number, tint: V3) => p.add({ at, half: [half, r, r], mat, tint, quad: [0, 1 / r ** 2, 1 / r ** 2, 0] });
/** bars from place to place */
const path = (p: Put, ps: V3[], r: number, mat: number, tint: V3, k = 1) => { for (let i = 0; i + 1 < ps.length; i++) p.bar(ps[i]!, ps[i + 1]!, r * (k + (1 - k) * (1 - i / (ps.length - 1))), r * 0.999 * (k + (1 - k) * (1 - (i + 1) / (ps.length - 1))), mat, tint); };
const both = (f: (s: number) => void) => { f(1); f(-1); };
const TAU = 2 * Math.PI;
/** 0: tire, rim, six spokes, hub and its drum; 1: fewer segments; 2: the tire and three bars across it */
function wheel(p: Put, c: V3, fine: number) {
  const n = fine === 0 ? 18 : fine === 1 ? 12 : 10; arc(p, c, R, fine === 2 ? 0.192 : 0.212, fine === 2 ? 0.027 : 0.030, 0.082, 0.16, 0, TAU, n, fine === 2 ? M.print! : M.rubber!, fine === 2 ? [0.42, 0.42, 0.425] : ONE); if (fine < 2) arc(p, c, 0.224, 0.192, 0.0215, 0.12, 0.30, 0, TAU, n, M.steel!, ALLOY);
  const spoke = (t: number, r0: number, r1: number, w: number, d: number) => { const y: V3 = [0, Math.sin(t), Math.cos(t)], z: V3 = [0, -y[2], y[1]], k = 0.10, l = Math.hypot(1, k); p.add({ at: add(c, mul(y, (r0 + r1) / 2)), half: [d, (r1 - r0) / 2, w * 1.4], rot: [[1, 0, 0], y, z], mat: M.steel!, tint: ALLOY, planes: [[[0, k / l, 1 / l], w / l], [[0, k / l, -1 / l], w / l]] }); };
  if (fine < 2) for (let i = 0; i < 6; i++) spoke(0.3 + i * TAU / 6, 0.030, 0.200, 0.0165, 0.0115); else for (let i = 0; i < 3; i++) { const t = 0.3 + i * TAU / 6, y: V3 = [0, Math.sin(t), Math.cos(t)], z: V3 = [0, -y[2], y[1]]; p.add({ at: c, half: [0.0105 + i * 0.002, 0.198, 0.0165], rot: [[1, 0, 0], y, z], mat: M.steel!, tint: ALLOY }); }
  across(p, c, 0.048, 0.036, M.steel!, DULL); if (fine === 0) { across(p, add(c, [-0.014, 0, 0]), 0.064, 0.016, M.steel!, DULL); across(p, c, 0.0085, 0.088, M.chrome!, ONE); both(s => { across(p, add(c, [s * 0.083, 0, 0]), 0.0135, 0.0075, M.chrome!, ONE); across(p, add(c, [s * 0.0365, 0, 0]), 0.026, 0.0045, M.steel!, ALLOY); }); p.bar(add(c, [0, -0.19, 0.04]), add(c, [0.004, -0.165, 0.045]), 0.0035, 0.003, M.rubber!, ONE); }
}
/** what turns with the bars; inside it, what turns with the wheel */
function fork(p: Put, fine: number): () => void {
  let spin!: ReturnType<Put['joint']>; const G = 0.318;
  const steer = p.joint(axis(0.5), () => {
    spin = p.joint(FRONT, () => wheel(p, FRONT, fine));
    both(s => { if (fine < 2) { p.bar(leg(-0.012, s * 0.062), leg(0.24, s * 0.062), 0.0185, 0.0180, M.steel!, ALLOY); p.bar(leg(0.215, s * 0.062), leg(0.425, s * 0.062), 0.0235, 0.0225, fine ? M.gloss! : M.gloss!, MINT); } else p.bar(leg(-0.012, s * 0.062), leg(0.425, s * 0.062), 0.021, 0.021, M.gloss!, MINT); });
    arc(p, FRONT, G, 0.291, 0.052, 0.125, 0, 0.56, 2.95, fine === 0 ? 7 : fine === 1 ? 5 : 4, M.gloss!, MINT);
    // crown, stem, the clamp above
    const crown = mix(leg(0.418, 0), axis(0.418), 0.4), rot: Rot = [[1, 0, 0], D, cross([1, 0, 0], D)]; p.add({ at: crown, half: [0.086, 0.020, 0.036], rot, mat: M.gloss!, tint: MINT }); p.bar(axis(0.40), axis(0.665), 0.0175, 0.0172, M.chrome!, ONE);
    // the lamp: a shell, its rim, a lens
    const eye = add(axis(0.535), [0, 0, 0.150]), f: V3 = norm([0, -0.10, 1]), er: Rot = [[1, 0, 0], f, cross([1, 0, 0], f)]; p.add({ at: add(eye, mul(f, -0.045)), half: [0.076, 0.075, 0.076], rot: er, mat: M.gloss!, tint: MINT, curves: [{ at: [0, 0.052, 0], r: [0.0765, 0.128, 0.0765] }] }); p.add({ at: add(eye, mul(f, 0.036)), half: [0.0795, 0.0085, 0.0795], rot: er, mat: M.chrome!, tint: ONE, quad: [1 / 0.0795 ** 2, 0, 1 / 0.0795 ** 2, 0] }); p.add({ at: add(eye, mul(f, 0.036)), half: [0.070, 0.025, 0.070], rot: er, mat: M.film!, tint: [0.80, 0.82, 0.82], quad: [1 / 0.070 ** 2, 1 / 0.025 ** 2, 1 / 0.070 ** 2, 0] });
    if (fine === 0) { both(s => p.bar(add(eye, [s * 0.060, 0, -0.035]), leg(0.50, s * 0.052), 0.0095, 0.0093, M.steel!, DULL)); p.bar(leg(0.415, 0.052), leg(0.515, 0.052), 0.013, 0.0129, M.gloss!, MINT); p.bar(leg(0.415, -0.052), leg(0.515, -0.052), 0.013, 0.0129, M.gloss!, MINT); const u: V3 = [0, 0.940, -0.342], sr = rotX(-0.349), dial = (k: number, r: number, h: number, mat: number, tint: V3) => p.add({ at: add(add(eye, [0, 0.066, -0.052]), mul(u, k)), half: [r, h, r], rot: sr, mat, tint, quad: [1 / r ** 2, 0, 1 / r ** 2, 0] }); dial(0, 0.036, 0.022, M.film!, BLACK); dial(0.0205, 0.0368, 0.0045, M.chrome!, ONE); dial(0.0225, 0.0308, 0.0045, M.film!, [0.90, 0.88, 0.82]); p.add({ at: add(add(eye, [0.006, 0.066, -0.052]), mul(u, 0.0272)), half: [0.0016, 0.0018, 0.013], rot: compose(sr, rotY(0.6)), mat: M.print!, tint: [0.75, 0.12, 0.10] }); }
    else p.add({ at: add(eye, [0, 0.070, -0.054]), half: [0.035, 0.024, 0.035], rot: rotX(-0.349), mat: M.film!, tint: BLACK, quad: [1 / 0.035 ** 2, 0, 1 / 0.035 ** 2, 0] });
    // the bars: up, out and back to the hands
    const c = axis(0.655), B = (s: number): V3[] => [add(c, [0, 0.004, 0.012]), add(c, [s * 0.045, 0.006, 0.012]), add(c, [s * 0.075, 0.035, 0.004]), add(c, [s * 0.105, 0.090, -0.010]), add(c, [s * 0.140, 0.117, -0.022]), add(c, [s * 0.200, 0.122, -0.050]), add(c, [s * 0.325, 0.108, -0.135])];
    both(s => { const b = B(s); if (fine === 0) { path(p, b, 0.0112, M.chrome!, ONE); for (const k of [2, 3, 4]) p.ball(b[k]!, [0.0113, 0.0113, 0.0113], M.chrome!, ONE); } else path(p, fine === 1 ? b : [b[0]!, b[2]!, b[4]!, b[6]!], 0.0118, M.chrome!, ONE); const g0 = mix(b[5]!, b[6]!, 0.22), g1 = add(b[6]!, mul(norm(sub(b[6]!, b[5]!)), 0.008)); p.bar(g0, g1, 0.0172, 0.0165, M.rubber!, ONE); if (fine === 0) { p.ball(g1, [0.0185, 0.0185, 0.0185], M.rubber!, ONE); p.ball(g0, [0.0195, 0.0195, 0.0195], M.rubber!, ONE); p.ball(mix(b[5]!, b[6]!, 0.12), [0.0155, 0.018, 0.0155], M.steel!, DULL); } if (fine < 2) { const l0 = add(mix(b[5]!, b[6]!, 0.10), [0, -0.004, 0.018]); path(p, [l0, add(l0, [s * 0.030, -0.003, 0.016]), add(l0, [s * 0.120, -0.010, -0.028])], 0.0062, M.steel!, ALLOY); if (fine === 0) p.ball(add(l0, [s * 0.120, -0.010, -0.028]), [0.0085, 0.0085, 0.0085], M.steel!, ALLOY); path(p, [l0, add(l0, [s * -0.055, -0.004, 0.075]), [s * 0.085, 0.905, 0.385], [s * 0.050, 0.790, 0.405], [s * 0.046, 0.700, 0.375]].slice(0, fine ? 4 : 5) as V3[], fine ? 0.0040 : 0.0036, M.rubber!, ONE); } });
    if (fine === 0) across(p, add(c, [0, 0.005, 0.012]), 0.0195, 0.032, M.steel!, DULL);
    // a mirror on its stalk
    { const b = B(1), m0 = mix(b[5]!, b[6]!, 0.08), m2 = add(m0, [0.062, 0.150, 0.012]), k = norm([-0.10, 0.02, -1]), mr: Rot = [norm(cross(k, [0, 1, 0])), k, cross(norm(cross(k, [0, 1, 0])), k)]; path(p, fine === 0 ? [m0, add(m0, [0.012, 0.070, 0.020]), m2] : [m0, m2], 0.0052, M.chrome!, ONE); p.add({ at: m2, half: [0.056, 0.018, 0.044], rot: mr, mat: M.film!, tint: BLACK, quad: [1 / 0.056 ** 2, 1 / 0.018 ** 2, 1 / 0.044 ** 2, 0], planes: [[[0, 1, 0], 0.004]] }); if (fine === 0) p.add({ at: add(m2, mul(k, 0.004)), half: [0.0525, 0.0022, 0.0405], rot: mr, mat: M.mirror!, tint: ONE, quad: [1 / 0.0525 ** 2, 0, 1 / 0.0405 ** 2, 0] }); }
    if (fine === 0) both(s => { const at = (a: number, x: number): V3 => add(FRONT, [x, 0.300 * Math.sin(a), 0.300 * Math.cos(a)]); p.bar(add(FRONT, [s * 0.080, 0, 0]), at(2.70, s * 0.047), 0.0042, 0.0041, M.chrome!, ONE); p.bar(at(0.95, s * 0.047), leg(0.20, s * 0.066), 0.0042, 0.0041, M.chrome!, ONE); });
  });
  return () => { void spin; steer.rot = turn(D, angle.steer); spin.rot = rotX(angle.spin); p.pose(); };
}
const angle = { steer: 0, spin: 0 }, AT_REST = 0.27;
/** all levels move as one: the bars swing to and fro, the wheel runs on */
const live = (p: Put, set: () => void): Live => { const go = (t: number) => { angle.steer = AT_REST + 0.40 * Math.sin(0.9 * t); angle.spin = 2.6 * t; set(); }; void p; go(0); return { step: (_, t) => go(t) }; };
/** the beam one steps over, from the head down to the engine */
const HEAD = axis(0.475), FOOT: V3 = [0, 0.405, -0.13], beamRot = (): Rot => { const y = norm(sub(HEAD, FOOT)); return [[1, 0, 0], y, cross([1, 0, 0], y)]; };
const beam = (p: Put) => { const l = Math.hypot(...sub(HEAD, FOOT)) / 2; p.add({ at: mix(HEAD, FOOT, 0.5), half: [0.054, l + 0.03, 0.078], rot: beamRot(), mat: M.gloss!, tint: MINT, curves: [{ at: [0, 0, 0], r: [0.054, 0, 0.078] }, { at: [0, -0.10, 0], r: [0.0625, 0.60, 0.092] }] }); };
/** the shell under the saddle, over the engine and before the wheel */
const shell = (p: Put) => { const at: V3 = [0, 0.535, -0.325], cut = (n: V3, w: number): [V3, number] => { const k = Math.hypot(...n); return [mul(n, 1 / k), (w - dot(n, at)) / k]; }; p.add({ at, half: [0.066, 0.150, 0.215], mat: M.gloss!, tint: MINT, curves: [{ at: [0, -0.015, 0.01], r: [0.0665, 0.225, 0.330] }, { at: [0, -0.22, 0.02], r: [0, 0.372, 0.30] }], planes: [cut([0, -0.8, -0.6], -0.221), cut([0, 0.55, 0.835], 0.130), cut([0, -1, 0.15], -0.425)] }); };
/** the saddle: a broad seat and a narrow nose, round over, flat beneath, on a bright band and two springs */
function saddle(p: Put, fine: number) {
  const at: V3 = [0, 0.752, -0.312], part = (z: number, w: number, l: number, h: number, low: number, up: number) => { p.add({ at: add(at, [0, up, z]), half: [w, h, l], rot: rotX(0.04), mat: M.film!, tint: TAN, quad: [1 / w ** 2, 1 / h ** 2, 1 / l ** 2, 0], planes: [[[0, -1, 0], low]] }); };
  part(-0.066, 0.125, 0.114, 0.061, 0.020, 0.004); part(0.022, 0.090, 0.172, 0.0565, 0.018, 0.003);
  if (fine < 2) { const band = (z: number, w: number, l: number, mat: number, tint: V3, y: number, h: number) => p.add({ at: add(at, [0, y, z]), half: [w, h, l], mat, tint, quad: [1 / w ** 2, 0, 1 / l ** 2, 0] }); band(-0.066, 0.1215, 0.1110, M.chrome!, ONE, -0.0195, 0.0075); band(0.022, 0.0880, 0.1680, M.chrome!, ONE, -0.0172, 0.0075); if (fine === 0) { band(-0.066, 0.108, 0.098, M.plastic!, BLACK, -0.031, 0.0075); band(0.022, 0.070, 0.145, M.plastic!, BLACK, -0.029, 0.0075); } }
  if (fine === 0) both(s => { for (let k = 0; k < 5; k++) p.drum([s * 0.066, 0.663 + k * 0.0125, -0.405], 0.0185, 0.0040, M.steel!, ALLOY); p.drum([s * 0.066, 0.687, -0.405], 0.0115, 0.036, M.steel!, BLACK); p.box([s * 0.048, 0.652, -0.405], [0.034, 0.006, 0.014], M.steel!, DULL); });
  p.bar([0, 0.62, -0.285], [0, 0.735, -0.300], 0.0165, 0.0165, M.chrome!, ONE);
}
/** the carrier over the back wheel */
function carrier(p: Put, fine: number) {
  const y = 0.672, r = fine === 2 ? 0.009 : 0.0066; both(s => { path(p, fine === 0 ? [[s * 0.070, y - 0.030, -0.40], [s * 0.084, y, -0.46], [s * 0.088, y, -0.775], [s * 0.070, y, -0.81], [0, y, -0.818]] : [[s * 0.070, y - 0.02, -0.41], [s * 0.088, y, -0.48], [s * 0.084, y, -0.805]], r, M.chrome!, ONE); if (fine < 2) path(p, [[s * 0.087, y, -0.70], [s * 0.090, 0.30, -0.575]], 0.0058, M.chrome!, ONE); });
  if (fine === 0) { for (const x of [-0.044, 0, 0.044]) p.bar([x, y + 0.001, -0.475], [x, y + 0.001, -0.815], 0.0052, 0.0051, M.chrome!, ONE); for (const z of [-0.52, -0.64]) p.bar([-0.087, y - 0.003, z], [0.087, y - 0.003, z], 0.0058, 0.0057, M.chrome!, ONE); for (const x of [-0.07, 0.07]) p.ball([x, y, -0.81], [0.0067, 0.0067, 0.0067], M.chrome!, ONE); }
  else if (fine === 1) { for (const x of [-0.044, 0, 0.044]) p.bar([x, y + 0.001, -0.475], [x, y + 0.001, -0.81], 0.0052, 0.0051, M.chrome!, ONE); p.bar([-0.088, y, -0.805], [0.088, y, -0.805], 0.0068, 0.0067, M.chrome!, ONE); for (const z of [-0.52, -0.64]) p.bar([-0.087, y - 0.003, z], [0.087, y - 0.003, z], 0.0058, 0.0057, M.chrome!, ONE); }
  else { p.box([0, y, -0.645], [0.018, 0.0065, 0.165], M.steel!, DULL); p.bar([-0.088, y, -0.803], [0.088, y, -0.803], r, r * 0.99, M.chrome!, ONE); }
}
/** the engine and what hangs on it */
function engine(p: Put, fine: number) {
  const CASE: V3 = [0, 0.300, -0.125], cyl: V3 = norm([0, 0.22, 1]), at = (k: number): V3 => add(add(CASE, [0, 0.022, 0]), mul(cyl, k)), rot: Rot = [[1, 0, 0], cyl, cross([1, 0, 0], cyl)];
  across(p, CASE, 0.076, 0.058, M.steel!, DULL); across(p, [0, 0.285, -0.215], 0.058, 0.052, M.steel!, DULL); p.box([0, 0.356, -0.16], [0.030, 0.045, 0.050], M.steel!, DULL);
  if (fine === 0) { for (let k = 0; k < 7; k++) p.add({ at: at(0.088 + 0.0165 * k), half: [0.054, 0.0036, 0.050], rot, mat: M.steel!, tint: ALLOY, curves: [{ at: [0, 0, 0], r: [0.061, 0, 0.058] }] }); p.bar(at(0.05), at(0.200), 0.038, 0.0379, M.steel!, [0.55, 0.55, 0.56]); p.bar(at(0.200), at(0.222), 0.0125, 0.0085, M.film!, CREAM); p.bar(at(0.219), add(at(0.23), [0, 0.028, 0.004]), 0.0115, 0.0100, M.rubber!, ONE); path(p, [add(at(0.23), [0, 0.028, 0.004]), add(at(0.20), [0.015, 0.075, 0]), add(at(0.12), [0.02, 0.10, 0])], 0.0036, M.rubber!, ONE); }
  else p.add({ at: at(0.135), half: [0.052, 0.064, 0.050], rot, mat: M.steel!, tint: fine === 1 ? [0.70, 0.70, 0.71] : [0.66, 0.66, 0.67], curves: [{ at: [0, 0, 0], r: [0.061, 0, 0.058] }] });
  // the flywheel's cover on the left, the clutch's on the right
  across(p, [0.066, 0.300, -0.125], 0.070, 0.016, M.steel!, ALLOY); if (fine === 0) { across(p, [0.0835, 0.300, -0.125], 0.052, 0.0035, M.film!, BLACK); across(p, [0.087, 0.300, -0.125], 0.020, 0.0030, M.steel!, ALLOY); for (let k = 0; k < 3; k++) { const a = k * TAU / 3 + 0.4; across(p, [0.0825, 0.300 + 0.061 * Math.sin(a), -0.125 + 0.061 * Math.cos(a)], 0.0055, 0.0028, M.chrome!, ONE); } }
  if (fine < 2) { p.add({ at: [-0.066, 0.292, -0.155], half: [0.0155, 0.072, 0.098], mat: M.steel!, tint: ALLOY, quad: [0, 1 / 0.072 ** 2, 1 / 0.098 ** 2, 0] }); across(p, [-0.038, 0.385, -0.085], 0.030, 0.036, M.film!, BLACK); }
  // the exhaust: out of the cylinder, under and back, its silencer
  const S0: V3 = [0.098, 0.205, -0.145], S1: V3 = [0.118, 0.262, -0.735]; if (fine === 0) path(p, [add(at(0.15), [0.012, -0.045, 0]), [0.040, 0.235, 0.045], [0.070, 0.200, -0.005], [0.090, 0.195, -0.075], S0], 0.0145, M.chrome!, ONE); else if (fine === 1) path(p, [add(at(0.15), [0.012, -0.045, 0]), [0.040, 0.235, 0.045], [0.072, 0.200, -0.010], S0], 0.0150, M.chrome!, ONE); else path(p, [add(at(0.15), [0.012, -0.045, 0]), [0.050, 0.215, 0.028], S0], 0.016, M.chrome!, ONE);
  p.bar(S0, mix(S0, S1, 0.16), fine ? 0.026 : 0.0165, 0.0335, M.chrome!, ONE); p.bar(mix(S0, S1, 0.16), mix(S0, S1, 0.94), 0.0335, 0.0360, M.chrome!, ONE); p.bar(mix(S0, S1, 0.94), S1, 0.0360, fine === 2 ? 0.025 : 0.0170, M.chrome!, ONE); if (fine < 2) p.bar(S1, mix(S0, S1, 1.06), 0.0095, 0.0094, M.steel!, [0.3, 0.3, 0.3]); if (fine === 0) { p.bar(mix(S0, S1, 0.24), mix(S0, S1, 0.255), 0.0372, 0.0373, M.steel!, DULL); p.bar(mix(S0, S1, 0.80), [0.088, 0.272, -0.575], 0.008, 0.0079, M.steel!, BLACK); }
}
/** pedals, the frame's back fork, springs, stand: black or bright bars */
function under(p: Put, fine: number) {
  both(s => { p.bar([s * 0.060, 0.300, -0.205], [s * 0.079, R, -0.555], fine === 2 ? 0.017 : 0.0155, 0.0135, M.gloss!, MINT);
    // a spring: its chrome cover, its coils, its eye
    const a: V3 = [s * 0.082, 0.622, -0.455], b: V3 = [s * 0.084, 0.288, -0.562]; if (fine === 0) { p.bar(a, mix(a, b, 0.46), 0.0215, 0.0213, M.chrome!, ONE); p.bar(mix(a, b, 0.40), b, 0.0125, 0.0124, M.steel!, BLACK); for (let k = 0; k < 7; k++) p.bar(mix(a, b, 0.50 + 0.055 * k), mix(a, b, 0.522 + 0.055 * k), 0.0195, 0.0194, M.steel!, ALLOY); across(p, b, 0.0165, 0.013, M.rubber!, ONE); across(p, a, 0.0165, 0.014, M.chrome!, ONE); } else p.bar(a, b, 0.0195, 0.0185, fine === 1 ? M.chrome! : M.steel!, fine === 1 ? [0.86, 0.86, 0.86] : [0.80, 0.80, 0.80]);
    // the stand
    p.bar([s * 0.052, 0.262, -0.175], [s * 0.112, 0.016, -0.255], 0.0135, 0.0120, M.paint!, BLACK); p.bar([s * 0.112, 0.0125, -0.215], [s * 0.112, 0.0125, -0.285], 0.0125, 0.0124, M.paint!, BLACK); });
  if (fine < 2) p.bar([-0.082, 0.135, -0.216], [0.082, 0.135, -0.216], 0.0095, 0.0094, M.paint!, BLACK); if (fine < 2) p.bar([0.10, 0.045, -0.246], [0.135, 0.040, -0.335], 0.0075, 0.0070, M.paint!, BLACK);
  // pedals
  const P0: V3 = [0, 0.262, -0.075]; across(p, P0, 0.0155, 0.112, M.chrome!, ONE); both(s => { const c: V3 = add(P0, [s * 0.112, 0, 0]), e: V3 = add(c, [s * 0.010, s * 0.028, s * 0.128]); p.bar(c, e, 0.0115, 0.0085, M.chrome!, ONE); if (fine === 0) { across(p, add(e, [s * 0.045, 0, 0]), 0.0065, 0.048, M.chrome!, ONE); for (const z of [-1, 1]) p.box(add(e, [s * 0.052, 0, z * 0.0215]), [0.043, 0.0110, 0.0105], M.rubber!, ONE); both(x => p.box(add(e, [s * 0.052 + x * 0.0435, 0, 0]), [0.0026, 0.0075, 0.030], M.steel!, DULL)); } else p.box(add(e, [s * 0.050, 0, 0]), [0.046, 0.0105, fine === 1 ? 0.0300 : 0.024], M.rubber!, ONE); });
  // the chain's guard
  const C0: V3 = [-0.094, 0.290, -0.150], C1: V3 = [-0.094, 0.280, -0.545]; if (fine < 2) { const z = norm(sub(C0, C1)), y = cross(z, [1, 0, 0]); p.add({ at: mix(C0, C1, 0.5), half: [0.0105, 0.043, 0.215], rot: [[1, 0, 0], y, z], mat: M.gloss!, tint: MINT, curves: [{ at: [0, 0, -0.35], r: [0, 0.50, 0.570] }, { at: [0, 0, 0.10], r: [0, 0.050, 0.335] }] }); }
}
/** the back: its mudguard, lamp, plate */
function tail(p: Put, fine: number) {
  arc(p, REAR, 0.318, 0.291, 0.054, 0.130, 0, 0.74, 3.20, fine === 0 ? 7 : fine === 1 ? 5 : 4, M.gloss!, MINT);
  const at = (a: number, r: number): V3 => add(REAR, [0, r * Math.sin(a), r * Math.cos(a)]), L = 2.55, rot: Rot = [[1, 0, 0], norm([0, Math.sin(L), Math.cos(L)]), cross([1, 0, 0], norm([0, Math.sin(L), Math.cos(L)]))];
  p.add({ at: at(L, 0.330), half: [0.040, 0.026, 0.032], rot, mat: M.film!, tint: BLACK }); p.add({ at: at(L, 0.352), half: [0.036, 0.032, 0.029], rot, mat: M.film!, tint: [0.82, 0.10, 0.07], curves: [{ at: [0, -0.010, 0], r: [0.044, 0.042, 0.037] }] });
  { p.box([0, 0.335, -0.893], [0.062, 0.044, 0.0025], M.print!, [0.94, 0.94, 0.92], { rot: rotX(-0.10), ...(fine === 0 ? { marks: [{ face: 4, at: [0, 0] as [number, number], size: [0.112, 0.070] as [number, number], art: 'plate', color: ONE }] } : {}) }); p.bar(at(2.86, 0.312), [0, 0.345, -0.890], 0.011, 0.010, M.steel!, BLACK); }
}
/** what is on the shell: side panels, a badge, the filler's cap, the head's tube */
function trim(p: Put, fine: number) {
  const y = norm([0, 0.70, -0.71]), rot: Rot = [[1, 0, 0], y, cross([1, 0, 0], y)], c: V3 = [0, 0.552, -0.276]; both(s => { p.add({ at: add(c, [s * 0.0605, 0, 0]), half: [0.0125, 0.104, 0.056], rot, mat: M.gloss!, tint: CREAM, curves: [{ at: [s * -0.030, 0, 0], r: [0.0425, 0.150, 0.075] }, { at: [0, -0.02, 0], r: [0, 0.124, 0.058] }] }); if (fine === 0) p.add({ at: add(c, [s * 0.0725, -0.005, 0.0]), half: [0.0030, 0.045, 0.0095], rot, mat: M.chrome!, tint: ONE, quad: [0, 1 / 0.045 ** 2, 1 / 0.0095 ** 2, 0] }); });
  p.bar(axis(0.405), axis(0.580), 0.0335, 0.0325, M.gloss!, MINT); if (fine < 2) both(s => p.bar(axis(0.4925 + s * 0.088 - 0.006), axis(0.4925 + s * 0.088 + 0.006), 0.0365, 0.0364, M.chrome!, ONE));
  if (fine === 0) { const b = beamRot(), top = add(mix(HEAD, FOOT, 0.26), mul(b[2], 0.074)); p.add({ at: top, half: [0.027, 0.027, 0.012], rot: b, mat: M.chrome!, tint: ONE, quad: [1 / 0.027 ** 2, 1 / 0.027 ** 2, 0, 0] }); p.add({ at: add(top, mul(b[2], 0.012)), half: [0.0215, 0.0215, 0.006], rot: b, mat: M.chrome!, tint: ONE, quad: [1 / 0.0215 ** 2, 1 / 0.0215 ** 2, 1 / 0.006 ** 2, 0] }); both(s => p.add({ at: add(mix(HEAD, FOOT, 0.50), [s * 0.0525, 0, 0]), half: [0.0035, 0.135, 0.0105], rot: b, mat: M.gloss!, tint: CREAM, quad: [0, 1 / 0.135 ** 2, 1 / 0.0105 ** 2, 0] })); }
}
function put(p: Put, fine: number): Live { beam(p); shell(p); saddle(p, fine); carrier(p, fine); engine(p, fine); under(p, fine); tail(p, fine); trim(p, fine); wheel(p, REAR, fine); return live(p, fork(p, fine)); }
export const moped: ObjectKind = { name: 'moped', title: 'Moped', make: (): Made => ({
  name: 'moped', title: 'Moped', size: [0.70, 1.17, 1.78], on: 'floor', close: { at: [0.05, 0.50, 0.02], far: 5.5 }, read: 0, moves: { kind: 'works' }, crowd: 49,
  levels: [
    { shape: 'pieces', error: 0, less: '', put: p => put(p, 0) },
    { shape: 'pieces', error: 0.012, less: 'cables, fins, coils, stays, the small bright parts', put: p => put(p, 1) },
    { shape: 'pieces', error: 0.030, less: 'rims, levers, pipes: the frame, the wheels\' rings with three bars across, saddle, engine, bars', put: p => put(p, 2) },
  ],
}) };
