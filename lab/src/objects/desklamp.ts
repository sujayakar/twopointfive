// A JOINTED DESK LAMP: a heavy foot, two arms with springs, a shade that is truly hollow (a ring of staves cut from one cone). It looks about: it turns on its foot, its arms bow and rise, its head nods.
// Coarser: fewer staves, then a full cone; the springs' coils become the bar they are as thick as; each pair of rods one.
import { type V3, type Rot, ID, add, sub, mul, norm, along, rotX, rotY, compose } from '../math';
import { type Piece, taper } from '../scene/world';
import { M } from '../scene/materials';
import { facets } from './bicycle';
import type { Live, Made, ObjectKind, Put } from './object';
const HUE: V3[] = [[0.93, 0.62, 0.10], [0.93, 0.62, 0.10], [0.925, 0.62, 0.115], [0.90, 0.63, 0.245]], ONE: V3 = [1, 1, 1], STEEL: V3 = [0.93, 0.93, 0.94], DARK: V3 = [0.24, 0.24, 0.25], COIL: V3 = [0.76, 0.76, 0.78], WHITE: V3 = [0.97, 0.96, 0.93];
// ---- where its joints are at rest; its foot's middle is a little behind the origin
const Z = -0.06, NECK: V3 = [0, 0.062, Z], HIP: V3 = [0, 0.104, Z], LOW = 0.34, ELBOW: V3 = add(HIP, mul([0, Math.cos(0.40), -Math.sin(0.40)], LOW)), UP = 0.33, HEAD: V3 = add(ELBOW, mul([0, Math.sin(0.30), Math.cos(0.30)], UP));
/** the way the shade looks, at rest */
const LOOK: V3 = norm([0, -0.74, 0.67]);
/** round, its axis to the side */
const ACROSS: Rot = [[0, 0, 1], [1, 0, 0], [0, 1, 0]];
const pin = (p: Put, at: V3, r: number, half: number, mat: number, tint: V3) => p.add({ at, half: [r, half, r], mat, tint, rot: ACROSS, quad: [1 / r ** 2, 0, 1 / r ** 2, 0] });
/** a smooth way through some places */
const smooth = (pts: V3[]): V3[] => [...pts.slice(0, -1).flatMap((b, i): V3[] => { const a = pts[Math.max(0, i - 1)]!, c = pts[i + 1]!, d = pts[Math.min(pts.length - 1, i + 2)]!; return [b, [0, 1, 2].map(x => (9 * (b[x]! + c[x]!) - a[x]! - d[x]!) / 16) as V3]; }), pts[pts.length - 1]!];
const both = (f: (s: number) => void) => { f(1); f(-1); };
/** a hollow cone: n staves, so thick; their outside is the one cone */
function staves(p: Put, a: V3, b: V3, ra: number, rb: number, n: number, thick: number, mat: number, tint: V3) {
  const t = taper(a, b, ra, rb), l = t.half[1] * 2, k = (rb - ra) / l, r0 = (ra + rb) / 2 - thick, d = Math.PI / n + 0.002, g = Math.hypot(1, k);
  for (let i = 0; i < n; i++) { const f = (i + 0.5) / n * Math.PI * 2, c = Math.cos(f), s = Math.sin(f), side = (x: number): [V3, number] => [[Math.cos(f + x), 0, Math.sin(f + x)], 0]; p.add({ ...t, mat, tint, planes: [[[-c / g, k / g, -s / g], -r0 / g], side(Math.PI / 2 + d), side(-Math.PI / 2 - d)] }); }
}
/** a square rod: so wide to the side, so thick */
const rod = (p: Put, a: V3, b: V3, wide: number, thick: number, mat: number, tint: V3) => p.add({ at: mul(add(a, b), 0.5), half: [thick, Math.hypot(...sub(b, a)) / 2, wide], rot: along(sub(b, a), [1, 0, 0]), mat, tint });
const alongArm = (a: V3, b: V3, k: number, out = 0, x = 0): V3 => { const d = norm(sub(b, a)); return add(add(a, mul(sub(b, a), k)), [x, -d[2] * out, d[1] * out]); };
/** a coiled spring from a to b: its ends are wire, its coils rings of two sizes */
function spring(p: Put, a: V3, b: V3, r: number, coils: number) {
  const d = sub(b, a), at = (k: number): V3 => add(a, mul(d, k)); p.bar(a, b, r * 0.30, r * 0.30, M.steel!, STEEL); p.bar(at(0.125), at(0.875), r * 0.78, r * 0.78, M.steel!, [0.62, 0.62, 0.64]);
  const R = along(d, [1, 0, 0]), l = Math.hypot(...d) * 0.72; for (let i = 0; i < coils; i++) p.add({ at: at(0.14 + 0.72 * (i + 0.5) / coils), half: [r, l / coils * 0.30, r], mat: M.steel!, tint: STEEL, rot: R, quad: [1 / r ** 2, 0, 1 / r ** 2, 0] });
}
function foot(p: Put, lv: number) {
  const G = M.paint!, PAINT = HUE[lv]!, fine = lv === 0;
  if (lv < 2) p.drum([0, 0.0025, Z], 0.087, 0.0025, M.rubber!, ONE);
  if (lv < 3) { p.drum([0, 0.011, Z], 0.095, 0.008, G, PAINT); p.bar([0, 0.0185, Z], [0, 0.0275, Z], 0.0945, 0.076, G, PAINT); } else p.bar([0, 0.0005, Z], [0, 0.0275, Z], 0.097, 0.086, G, PAINT);
  if (lv < 2) p.ball([0, 0.026, Z], [0.074, 0.0095, 0.074], G, PAINT, { planes: [[[0, -1, 0], 0]] }); p.bar([0, 0.028, Z], [0, 0.070, Z], lv < 2 ? 0.0165 : 0.0175, lv < 2 ? 0.0125 : 0.0135, lv < 2 ? M.chrome! : G, lv < 2 ? ONE : [0.8, 0.62, 0.3]);
  if (fine) { p.drum([0, 0.0365, Z], 0.0235, 0.0035, M.chrome!, STEEL); p.drum([0.046, 0.0345, Z + 0.030], 0.0092, 0.0035, M.plastic!, DARK); p.drum([0.046, 0.0385, Z + 0.030], 0.0066, 0.0035, M.chrome!, STEEL); }
  // its cord, and the plug it ends in
  { const pts: V3[] = [[0, 0.010, Z - 0.088], [0.006, 0.006, Z - 0.125], [0.030, 0.0045, Z - 0.160], [0.072, 0.0045, Z - 0.168], [0.112, 0.0045, Z - 0.150], [0.146, 0.0045, Z - 0.118], [0.170, 0.0045, Z - 0.074], [0.172, 0.0065, Z - 0.030], [0.165, 0.0105, Z - 0.004]], q = fine ? smooth(pts) : lv === 1 ? pts : lv === 2 ? pts.filter((_, i) => i % 2 === 0) : [pts[0]!, pts[3]!, pts[6]!, pts[8]!], r = 0.0036;
    for (let i = 0; i + 1 < q.length; i++) { p.bar(q[i]!, q[i + 1]!, r, r, M.rubber!, ONE); if (fine && i) p.ball(q[i]!, [r, r, r], M.rubber!, ONE); }
    const e = pts[8]!, d = norm([-0.22, 0.02, 1]), x = along(d, [0, 1, 0]); p.add({ at: add(e, mul(d, 0.020)), half: [0.0140, 0.0180, 0.0105], mat: M.rubber!, tint: [1.2, 1.2, 1.2], rot: x }); if (lv < 2) both(s => { const k = add(add(e, mul(d, 0.036)), mul(x[0], s * 0.0068)); p.bar(k, add(k, mul(d, 0.019)), 0.0024, 0.0022, M.brass!, ONE); }); if (fine) p.bar(add(e, mul(d, -0.012)), add(e, mul(d, 0.004)), 0.0050, 0.0085, M.rubber!, [1.2, 1.2, 1.2]); }
}
function turret(p: Put, lv: number) { const G = M.paint!, PAINT = HUE[lv]!;
  p.drum([0, 0.074, Z], 0.0185, 0.0085, G, PAINT); if (lv < 2) { both(s => p.add({ at: [s * 0.0105, 0.096, Z], half: [0.0026, 0.0205, 0.0125], mat: G, tint: PAINT })); both(s => pin(p, [s * 0.0105, 0.1165, Z], 0.0125, 0.0045, G, PAINT)); } else p.add({ at: [0, 0.095, Z], half: [0.0148, 0.020, 0.0120], mat: G, tint: PAINT });
}
function lowArm(p: Put, lv: number) { const C = M.steel!, G = M.paint!, PAINT = HUE[lv]!, fine = lv === 0;
  if (lv < 3) both(s => rod(p, alongArm(HIP, ELBOW, 0, 0, s * 0.0185), alongArm(HIP, ELBOW, 1, 0, s * 0.0185), 0.0043, 0.0043, C, STEEL)); else rod(p, HIP, ELBOW, 0.0155, 0.0066, C, [0.88, 0.88, 0.90]);
  pin(p, HIP, lv < 2 ? 0.0090 : 0.0150, lv < 2 ? 0.0265 : 0.0330, lv < 2 ? M.steel! : M.plastic!, DARK); if (lv < 2) both(s => { pin(p, [s * 0.0285, HIP[1], Z], 0.0155, 0.0052, M.plastic!, DARK); if (fine) pin(p, [s * 0.0340, HIP[1], Z], 0.0065, 0.0024, C, STEEL); });
  // a lug behind the hip, and two springs from it up the arm
  const lug = alongArm(HIP, ELBOW, 0.02, -0.030), up = alongArm(HIP, ELBOW, 0.60, -0.006); if (lv < 2) { pin(p, lug, 0.0038, 0.0182, M.steel!, STEEL); pin(p, up, 0.0034, 0.0165, M.steel!, STEEL); both(s => p.add({ at: mul(add(add(HIP, [s * 0.0145, 0, 0]), add(lug, [s * 0.0145, 0, 0])), 0.5), half: [0.0060, 0.0190, 0.0022], rot: along(sub(lug, HIP), [1, 0, 0]), mat: G, tint: PAINT })); }
  both(s => { const a = add(lug, [s * 0.0085, 0, 0]), b = add(up, [s * 0.0085, 0, 0]); if (fine) spring(p, a, b, 0.0068, 20); else if (lv === 1) { p.bar(add(a, mul(sub(b, a), 0.14)), add(a, mul(sub(b, a), 0.86)), 0.0066, 0.0066, M.steel!, COIL); p.bar(a, b, 0.0022, 0.0022, M.steel!, STEEL); } }); if (lv === 2) p.add({ at: alongArm(lug, up, 0.5), half: [0.0125, Math.hypot(...sub(up, lug)) * 0.40, 0.0060], rot: along(sub(up, lug), [0, 0, 1]), mat: M.steel!, tint: COIL });
  if (lv < 3) both(s => p.add({ at: alongArm(HIP, ELBOW, 0.975, 0.004, s * 0.0185), half: [0.0130, 0.0215, 0.0061], rot: along(sub(ELBOW, HIP), [1, 0, 0]), mat: G, tint: PAINT }));
}
function upArm(p: Put, lv: number) { const C = M.steel!, G = M.paint!, PAINT = HUE[lv]!, fine = lv === 0;
  if (lv < 3) both(s => rod(p, alongArm(ELBOW, HEAD, -0.03, 0, s * 0.0115), alongArm(ELBOW, HEAD, 1, 0, s * 0.0115), 0.0040, 0.0040, C, STEEL)); else rod(p, ELBOW, HEAD, 0.0128, 0.0060, C, [0.88, 0.88, 0.90]);
  pin(p, ELBOW, lv < 2 ? 0.0080 : 0.0140, lv < 2 ? 0.0265 : 0.0325, lv < 2 ? M.steel! : M.plastic!, DARK); if (lv > 1) pin(p, alongArm(ELBOW, HEAD, 0.02, -0.010), 0.0190, lv === 2 ? 0.0188 : 0.0215, G, PAINT); if (lv < 2) { both(s => { pin(p, add(ELBOW, [s * 0.0280, 0, 0]), 0.0145, 0.0050, M.plastic!, DARK); if (fine) pin(p, add(ELBOW, [s * 0.0332, 0, 0]), 0.0060, 0.0022, C, STEEL); }); }
  const a = alongArm(ELBOW, HEAD, 0.05, -0.026), b = alongArm(ELBOW, HEAD, 0.52, -0.003); if (lv < 2) { pin(p, a, 0.0034, 0.0130, M.steel!, STEEL); both(s => p.add({ at: add(mul(add(ELBOW, a), 0.5), [s * 0.0065, 0, 0]), half: [0.0055, 0.0170, 0.0013], rot: along(sub(a, ELBOW), [1, 0, 0]), mat: G, tint: PAINT })); pin(p, b, 0.0032, 0.0110, M.steel!, STEEL); }
  if (fine) spring(p, a, b, 0.0062, 18); else if (lv === 2) p.bar(add(a, mul(sub(b, a), 0.10)), add(a, mul(sub(b, a), 0.90)), 0.0058, 0.0058, M.steel!, COIL); else if (lv === 1) { p.bar(add(a, mul(sub(b, a), 0.14)), add(a, mul(sub(b, a), 0.86)), 0.0060, 0.0060, M.steel!, COIL); p.bar(a, b, 0.0020, 0.0020, M.steel!, STEEL); }
  if (lv < 2) both(s => p.add({ at: alongArm(ELBOW, HEAD, 0.985, 0, s * 0.0115), half: [0.0105, 0.0165, 0.0058], rot: along(sub(HEAD, ELBOW), [1, 0, 0]), mat: G, tint: PAINT }));
}
function head(p: Put, lv: number) { const G = M.paint!, PAINT = HUE[lv]!, C = M.chrome!, fine = lv === 0, at = (k: number): V3 => add(add(HEAD, mul(LOOK, k)), mul([0, LOOK[2], -LOOK[1]], 0.038));      // (down the shade's axis; it hangs before the joint)
  const R = along(LOOK, [1, 0, 0]), drum = (k: number, r: number, half: number, mat: number, tint: V3, more: Partial<Piece> = {}) => p.add({ at: at(k), half: [r, half, r], mat, tint, rot: R, quad: [1 / r ** 2, 0, 1 / r ** 2, 0], ...more });
  pin(p, HEAD, lv < 2 ? 0.0070 : 0.0120, lv < 2 ? 0.0200 : 0.0250, lv < 2 ? M.steel! : M.plastic!, DARK); if (lv < 2) both(s => { pin(p, add(HEAD, [s * 0.0210, 0, 0]), 0.0125, 0.0045, M.plastic!, DARK); if (fine) pin(p, add(HEAD, [s * 0.0258, 0, 0]), 0.0052, 0.0020, C, STEEL); });
  p.bar(HEAD, at(0.004), lv < 2 ? 0.0068 : 0.0085, lv < 2 ? 0.0075 : 0.0085, G, PAINT);
  // neck, closed behind by a dome; fins round it
  p.bar(at(-0.022), at(0.034), 0.0300, 0.0345, G, PAINT); if (lv < 3) p.add({ at: at(-0.0215), half: [0.0298, 0.0200, 0.0298], mat: G, tint: PAINT, rot: R, quad: [1 / 0.0298 ** 2, 1 / 0.02 ** 2, 1 / 0.0298 ** 2, 0], planes: [[[0, 1, 0], 0]] }); if (fine) { for (let i = 0; i < 3; i++) drum(0.004 + i * 0.0085, 0.0352 + i * 0.0008, 0.0016, G, mul(PAINT, 0.97)); p.bar(at(-0.040), at(-0.054), 0.0080, 0.0062, M.plastic!, DARK); p.bar(at(-0.054), at(-0.060), 0.0046, 0.0040, C, STEEL); }
  if (lv === 1) p.bar(at(-0.038), at(-0.058), 0.0072, 0.0052, M.plastic!, DARK);
  const a = at(0.034), m = at(0.072), b = at(0.130);
  if (fine) { staves(p, a, m, 0.0345, 0.062, 20, 0.0017, G, PAINT); staves(p, m, b, 0.062, 0.084, 24, 0.0017, G, PAINT); staves(p, at(0.0345), m, 0.0328, 0.0602, 12, 0.0030, M['lamp shade']!, WHITE); staves(p, m, at(0.1285), 0.0602, 0.0818, 14, 0.0030, M['lamp shade']!, WHITE); }
  else if (lv === 1) { staves(p, a, m, 0.0345, 0.062, 8, 0.0055, G, PAINT); staves(p, m, b, 0.062, 0.084, 10, 0.0055, G, PAINT); }
  else { p.bar(a, m, 0.0345, 0.062, G, PAINT); p.bar(m, b, 0.062, 0.084, G, PAINT); if (lv === 2) drum(0.1285, 0.079, 0.0035, M['lamp shade']!, [0.62, 0.52, 0.36]); }
  if (lv < 2) { drum(0.040, 0.0150, 0.0180, M.plastic!, [0.9, 0.88, 0.82]); p.add({ at: at(0.076), half: [0.0245, 0.0270, 0.0245], mat: M.film!, tint: WHITE, rot: R, quad: [1 / 0.0245 ** 2, 1 / 0.027 ** 2, 1 / 0.0245 ** 2, 0] }); }
}
function put(p: Put, lv: number): Live {
  let n = 0; const add = p.add.bind(p); p.add = q => add(facets({ rot: ID, flags: 0, ...q }, 0.0166, lv ? 0.955 : 0.977, !lv, n++));
  foot(p, lv); let hip!: ReturnType<Put['joint']>, elbow!: ReturnType<Put['joint']>, nod!: ReturnType<Put['joint']>;
  const neck = p.joint(NECK, () => { turret(p, lv); hip = p.joint(HIP, () => { lowArm(p, lv); elbow = p.joint(ELBOW, () => { upArm(p, lv); nod = p.joint(HEAD, () => head(p, lv)); }); }); }); p.add = add;
  /** it looks about, slowly, and rests between */
  return { step(_, t) { const a = t * 0.9, bow = 0.16 * Math.sin(a) - 0.05 * Math.sin(2 * a), lift = 0.20 * Math.sin(a * 0.75) ; neck.rot = rotY(0.55 * Math.sin(a * 0.5)); hip.rot = rotX(bow); elbow.rot = rotX(-bow - lift); nod.rot = compose(rotX(lift * 0.7 + 0.12 * Math.sin(1.7 * a)), rotY(0)); p.pose(); } };
}
export const desklamp: ObjectKind = { name: 'desklamp', title: 'Desk lamp', make: (): Made => ({
  name: 'desklamp', title: 'Desk lamp', size: [0.27, 0.62, 0.55], on: 'floor', close: { at: [0, 0.42, 0.04], far: 2.9 }, read: 0, moves: { kind: 'works' }, crowd: 100,
  levels: [
    { shape: 'pieces', error: 0, less: '', put: p => put(p, 0) },
    { shape: 'pieces', error: 0.005, less: 'coils, the shade\'s lining; fewer staves', put: p => put(p, 1) },
    { shape: 'pieces', error: 0.012, less: 'joints\' plates, the bulb; the shade is full', put: p => put(p, 2) },
    { shape: 'pieces', error: 0.020, less: 'springs; each pair of rods is one; steps of the foot', put: p => put(p, 3) },
  ],
}) };
