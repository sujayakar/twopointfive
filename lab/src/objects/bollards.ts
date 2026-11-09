// TWO STREET BOLLARDS of painted cast iron with a chain hung between them. Near, every link is a ring of wire; then a plate a link, as wide as its wire; then bars along the line it hangs in.
import { type V3, type Rot, add, sub, mul, norm, cross, rng } from '../math';
import { M } from '../scene/materials';
import type { Made, ObjectKind, Put } from './object';
const GREEN: V3 = [0.175, 0.315, 0.260], CREAM: V3 = [0.96, 0.915, 0.79], GOLD: V3 = [0.86, 0.635, 0.24], IRON: V3 = [0.245, 0.245, 0.25], APART = 0.68;
const tone = (c: V3, k: number): V3 => [c[0] * k, c[1] * k, c[2] * k];
/** what it is turned of: from a height to a height, how wide there */
const ring = (p: Put, x: number, y: number, r: number, h: number, tint: V3, mat = M.paint!) => p.add({ at: [x, y, 0], half: [r, h, r], mat, tint, quad: [1 / r ** 2, 1 / (h * 1.9) ** 2, 1 / r ** 2, 0] });
const cone = (p: Put, x: number, y0: number, y1: number, r0: number, r1: number, tint: V3, mat = M.paint!) => p.bar([x, y0, 0], [x, y1, 0], r0, r1, mat, tint);
/** eight-sided, a0 and a1 from its middle to its sides */
function eight(p: Put, x: number, y0: number, y1: number, a0: number, a1: number, tint: V3, mat = M.paint!) { const h = (y1 - y0) / 2, m = (a1 - a0) / (2 * h), l = Math.hypot(1, m), planes: [V3, number][] = []; for (let k = 0; k < 8; k++) { const t = (k + 0.5) * Math.PI / 4; planes.push([[Math.cos(t) / l, -m / l, Math.sin(t) / l], (a0 + a1) / 2 / l]); } const w = Math.max(a0, a1) / Math.cos(Math.PI / 8); p.add({ at: [x, y0 + h, 0], half: [w, h, w], mat, tint, planes }); }
/** a round bar, so wide all along */
const rod = (p: Put, a: V3, b: V3, r: number, mat: number, tint: V3) => p.bar(a, b, r, r * 0.9985, mat, tint);
/** one post: a foot bolted down, an eight-sided shaft with a pale band, a collar with an eye, a ball */
function post(p: Put, s: number) {
  const x = s * APART;
  p.drum([x, 0.0075, 0], 0.124, 0.0075, M.paint!, GREEN); for (let k = 0; k < 4; k++) { const a = (k + 0.5) * Math.PI / 2; p.drum([x + 0.108 * Math.cos(a), 0.0185, 0.108 * Math.sin(a)], 0.0085, 0.0055, M.steel!, [0.62, 0.61, 0.59]); }
  cone(p, x, 0.012, 0.040, 0.104, 0.099, GREEN); eight(p, x, 0.036, 0.150, 0.0905, 0.0860, GREEN); ring(p, x, 0.158, 0.0985, 0.0125, GREEN); ring(p, x, 0.178, 0.0865, 0.0090, GREEN);
  eight(p, x, 0.180, 0.720, 0.0715, 0.0575, GREEN); eight(p, x, 0.560, 0.655, 0.0632, 0.0608, CREAM, M.print!); ring(p, x, 0.551, 0.0735, 0.0075, GREEN); ring(p, x, 0.664, 0.0705, 0.0075, GREEN);
  cone(p, x, 0.716, 0.742, 0.0560, 0.0700, GREEN); ring(p, x, 0.752, 0.0790, 0.0130, GREEN); ring(p, x, 0.773, 0.0680, 0.0080, GOLD, M.print!); cone(p, x, 0.765, 0.803, 0.0600, 0.0480, GREEN); ring(p, x, 0.806, 0.0590, 0.0080, GREEN); cone(p, x, 0.806, 0.834, 0.0430, 0.0370, GREEN);
  p.ball([x, 0.884, 0], [0.0670, 0.0625, 0.0670], M.paint!, GREEN); ring(p, x, 0.884, 0.0690, 0.0060, GREEN); cone(p, x, 0.940, 0.956, 0.0210, 0.0120, GREEN); p.ball([x, 0.959, 0], [0.0125, 0.0110, 0.0125], M.paint!, GREEN);
}
/** the eye a chain hangs in, its middle */
const EYE: V3 = [APART - 0.104, 0.7515, 0], EYE_R = 0.0165;
function eye(p: Put, s: number, n: number) { const c: V3 = [s * EYE[0], EYE[1], 0]; rod(p, [s * (APART - 0.070), EYE[1], 0], [s * (EYE[0] + EYE_R - 0.002), EYE[1], 0], 0.0095, M.paint!, GREEN); wire(p, Array.from({ length: n }, (_, k) => { const a = k / n * 2 * Math.PI; return add(c, [EYE_R * Math.cos(a), EYE_R * Math.sin(a), 0]); }), 0.0058, GREEN, M.paint!); }
/** wire along a closed line */
function wire(p: Put, pts: V3[], r: number, tint: V3, mat: number) { const n = pts.length; for (let i = 0; i < n; i++) { const a = pts[i]!, b = pts[(i + 1) % n]!, d = mul(norm(sub(b, a)), r * 0.36); rod(p, sub(a, d), add(b, d), r, mat, tint); } }
/** the line it hangs in: places evenly far apart along it */
const END: V3 = [EYE[0] - EYE_R * 0.75, EYE[1] - EYE_R * 0.45, 0], SAG = 0.225;
function hang(n: number): V3[] { const N = 2000, pts: V3[] = [], far: number[] = [0], c = 0.79, y0 = c * (Math.cosh(END[0] / c) - 1); for (let i = 0; i <= N; i++) { const x = END[0] * (i / N * 2 - 1); pts.push([x, END[1] - SAG + SAG * (c * (Math.cosh(x / c) - 1)) / y0, 0]); if (i) far.push(far[i - 1]! + Math.hypot(pts[i]![0] - pts[i - 1]![0], pts[i]![1] - pts[i - 1]![1])); } const out: V3[] = []; let k = 0; for (let i = 0; i <= n; i++) { const want = far[N]! * i / n; while (k < N && far[k]! < want) k++; out.push(pts[k]!); } return out; }
const LINKS = 31, WIRE = 0.0056, WIDE = 0.0135;
/** where each link is: its ends along the line, and the way it is wide (they hang by turns, half a right angle to either side) */
function links(seed: number): { a: V3; b: V3; u: V3; w: V3 }[] { const q = rng(seed * 19 + 7), l = hang(LINKS); return Array.from({ length: LINKS }, (_, i) => { const a = l[i]!, b = l[i + 1]!, u = norm(sub(b, a)), side: V3 = [0, 0, 1], up = cross(side, u), t = (i % 2 ? 1 : -1) * Math.PI / 4 + (q() - 0.5) * 0.35 + 0.12; return { a, b, u, w: add(mul(side, Math.cos(t)), mul(up, Math.sin(t))) }; }); }
/** a link: two straights and two half turns. It reaches past the line's places by half its wire, so each goes through the next */
function link(p: Put, k: { a: V3; b: V3; u: V3; w: V3 }) { const m = mul(add(k.a, k.b), 0.5), reach = Math.hypot(...sub(k.b, k.a)) / 2 + WIRE * 0.98, c = reach - WIDE, pts: V3[] = [], R = WIDE / Math.cos(Math.PI / 8); for (const e of [1, -1]) for (let i = 0; i < 4; i++) { const t = (e > 0 ? 0 : Math.PI) + (1.5 - i) * Math.PI / 4; pts.push(add(m, add(mul(k.u, e * c + R * Math.cos(t)), mul(k.w, R * Math.sin(t))))); } wire(p, pts, WIRE, IRON, M.print!); }
const plate = (p: Put, k: { a: V3; b: V3; u: V3; w: V3 }) => { const rot: Rot = [k.u, k.w, cross(k.u, k.w)]; p.add({ at: mul(add(k.a, k.b), 0.5), half: [Math.hypot(...sub(k.b, k.a)) / 2 + 0.0045, 0.0155, 0.0070], rot, mat: M.print!, tint: IRON }); };
const stub = (p: Put, s: number, from = 0.070) => rod(p, [s * (APART - from), EYE[1], 0], [s * END[0], EYE[1] - 0.006, 0], 0.0115, M.paint!, GREEN);
function bars(p: Put, n: number, r: number, tint: V3 = IRON) { const l = hang(n); for (let i = 0; i < n; i++) { const d = mul(norm(sub(l[i + 1]!, l[i]!)), 0.0015); rod(p, sub(l[i]!, d), add(l[i + 1]!, d), r, M.print!, tint); } }
export const bollards: ObjectKind = { name: 'bollards', title: 'Bollards and chain', make: (): Made => ({
  name: 'bollards', title: 'Bollards and chain', size: [1.608, 0.97, 0.248], on: 'floor', close: { at: [0.55, 0.74, 0], far: 1.8 }, read: 0, moves: { kind: 'still' }, crowd: 49,
  levels: [
    { shape: 'pieces', error: 0, less: '', put(p) { for (const s of [-1, 1]) { post(p, s); eye(p, s, 8); } for (const k of links(p.seed)) link(p, k); } },
    { shape: 'pieces', error: 0.010, less: 'the holes of the links, the eyes\', bolts, the thinner mouldings', put(p) {
      for (const s of [-1, 1]) { const x = s * APART; p.drum([x, 0.0075, 0], 0.124, 0.0075, M.paint!, GREEN); eight(p, x, 0.012, 0.152, 0.0925, 0.0870, GREEN); ring(p, x, 0.162, 0.0975, 0.0155, GREEN); eight(p, x, 0.175, 0.730, 0.0715, 0.0570, GREEN); cone(p, x, 0.558, 0.656, 0.0660, 0.0635, CREAM, M.print!); p.drum([x, 0.551, 0], 0.0715, 0.0075, M.paint!, GREEN); p.drum([x, 0.6635, 0], 0.0690, 0.0075, M.paint!, GREEN); ring(p, x, 0.748, 0.0785, 0.0180, GREEN); cone(p, x, 0.766, 0.780, 0.0660, 0.0650, GOLD, M.print!); cone(p, x, 0.780, 0.830, 0.0570, 0.0420, GREEN); p.ball([x, 0.884, 0], [0.0675, 0.0630, 0.0675], M.paint!, GREEN); cone(p, x, 0.940, 0.968, 0.0190, 0.0090, GREEN); stub(p, s); }
      for (const k of links(p.seed)) plate(p, k);
    } },
    { shape: 'pieces', error: 0.022, less: 'links: a bar for four; mouldings', put(p) { for (const s of [-1, 1]) { const x = s * APART; cone(p, x, 0.002, 0.165, 0.110, 0.094, GREEN); cone(p, x, 0.160, 0.800, 0.0785, 0.0600, GREEN); cone(p, x, 0.562, 0.652, 0.0690, 0.0660, CREAM, M.print!); p.drum([x, 0.755, 0], 0.076, 0.027, M.paint!, [0.31, 0.375, 0.25]); p.ball([x, 0.878, 0], [0.068, 0.090, 0.068], M.paint!, GREEN); stub(p, s); } bars(p, 8, 0.0132); } },
    { shape: 'pieces', error: 0.040, less: 'all but two posts, their bands and the chain\'s line', put(p) { for (const s of [-1, 1]) { const x = s * APART; cone(p, x, 0.002, 0.800, 0.1000, 0.0610, GREEN); cone(p, x, 0.565, 0.650, 0.0738, 0.0705, CREAM, M.print!); p.ball([x, 0.860, 0], [0.068, 0.107, 0.068], M.paint!, GREEN); stub(p, s); } bars(p, 4, 0.0165, tone(IRON, 1.2)); } },
  ],
}) };

