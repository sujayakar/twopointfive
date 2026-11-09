// A CAFE TABLE WITH A PARASOL: a canopy of eight panels, two by two, whose ribs bow a little, a valance in scallops, the wooden frame under it; a round table of marble on iron legs, its pole through it and into a heavy foot.
// Plates of canvas are thin and cut where they meet. What coarser levels must keep is the canopy's colors and its edge: nearly all that the game's camera sees.
import { type V3, type Rot, add, mul, sub, dot, cross, norm, mix, rotY } from '../math';
import { F } from '../scene/world';
import { M } from '../scene/materials';
import type { Made, ObjectKind, Put } from './object';
const RED: V3 = [0.84, 0.25, 0.16], CREAM: V3 = [0.95, 0.90, 0.80], WOOD: V3 = [0.72, 0.50, 0.32], PALE: V3 = [0.80, 0.60, 0.41], IRON: V3 = [0.235, 0.26, 0.25], MARBLE: V3 = [0.955, 0.945, 0.925], ONE: V3 = [1, 1, 1];
const N = 8, A = Math.PI / N, C = Math.cos(A), R = 1.25, RIM = 2.03, TOP = 0.735, TALL = 0.165, SCALLOP = 0.034;
/** a rib from above the pole out: how far off the axis, how high */
const RIB: [number, number][] = [[0, 2.475], [0.66, 2.302], [R, RIM]];
const out = (a: number): V3 => [Math.sin(a), 0, Math.cos(a)], at = (a: number, r: number, y: number): V3 => [Math.sin(a) * r, y, Math.cos(a) * r];
const colorOf = (i: number) => i % 2 ? CREAM : RED;
/** one panel of the canopy between two heights, cut where it meets its neighbors; it sags between its ribs, so its normal leans in from them */
function panel(p: Put, i: number, [r0, y0]: [number, number], [r1, y1]: [number, number], thick: number, sag: number, bow: [number, number], tint = colorOf(i)) {
  const a = i * 2 * A, e = out(a), t: V3 = [Math.cos(a), 0, -Math.sin(a)], lo = add(mul(e, r1 * C), [0, y1, 0]), hi = add(mul(e, r0 * C), [0, y0, 0]), s = norm(sub(hi, lo)), z = cross(t, s), rot: Rot = [t, s, z], mid = sub(mix(lo, hi, 0.5), mul(z, thick)), l = Math.hypot(...sub(hi, lo)) / 2;
  const planes = [1, -1].map(k => { const n = sub(mul(t, k * C), mul(e, Math.sin(A))); return [[dot(n, t), dot(n, s), dot(n, z)], -dot(n, mid)] as [V3, number]; });
  p.add({ at: mid, half: [r1 * Math.sin(A), l + 0.0012, thick], rot, mat: M.canvas!, tint, planes, ...(sag ? { flags: F.BEND, bend: [sag, -bow[1], -sag, -bow[1], sag, -bow[0], -sag, -bow[0]] } : {}) });
}
/** along side i, so far along it (−1 to 1), so far down from the rim: the valance hangs outward a little */
const FLARE = 0.16, side = (i: number, u: number, down: number): V3 => { const a = i * 2 * A, e = out(a), t: V3 = [Math.cos(a), 0, -Math.sin(a)]; return add(add(mul(e, R * C + down * Math.sin(FLARE) + 0.001), mul(t, u * R * Math.sin(A))), [0, RIM - 0.002 - down * Math.cos(FLARE), 0]); };
const hung = (i: number): Rot => { const a = i * 2 * A, e = out(a), t: V3 = [Math.cos(a), 0, -Math.sin(a)], y: V3 = [-e[0] * Math.sin(FLARE), Math.cos(FLARE), -e[2] * Math.sin(FLARE)]; return [t, y, cross(t, y)]; };
function scallops(p: Put, n: number) { const w = R * Math.sin(A) / n, rho = (w * w + SCALLOP ** 2) / (2 * SCALLOP); for (let i = 0; i < N; i++) for (let k = 0; k < n; k++) p.add({ at: side(i, (2 * k + 1 - n) / n, TALL / 2), half: [w + 0.0004, TALL / 2, 0.0026], rot: hung(i), mat: M.canvas!, tint: colorOf(i), curves: [{ at: [0, rho - TALL / 2, 0], r: [rho, rho, 0] }], flags: F.BEND, bend: [-0.16, 0.10, 0.16, 0.10, -0.06, 0, 0.06, 0] }); }
/** the scallops as one band, as deep as they are in the mean */
const band = (p: Put, thick = 0.0026) => { const tall = TALL - SCALLOP * 0.36; for (let i = 0; i < N; i++) p.add({ at: side(i, 0, tall / 2), half: [R * Math.sin(A) + 0.004, tall / 2, thick], rot: hung(i), mat: M.canvas!, tint: colorOf(i) }); };
/** the little roof over the vent: eight panels of its own, a hand over the others, with a hem */
function cap(p: Put, fine: boolean) {
  const r = 0.315, y = 2.428, t = r * Math.sin(A); for (let i = 0; i < N; i++) { panel(p, i, [0, y + 0.108], [r, y], 0.0026, 0.05, [0, 0], CREAM); if (fine) { const a = i * 2 * A, e = out(a); p.add({ at: add(mul(e, r * C + 0.001), [0, y - 0.0165, 0]), half: [t + 0.0012, 0.0150, 0.0024], rot: rotY(a), mat: M.canvas!, tint: CREAM }); } }
}
/** the same in one piece: a pyramid */
const hat = (p: Put) => { const r = 0.315 * C, h = 0.054, n = norm([1, r / (2 * h), 0]); p.add({ at: [0, 2.428 + h, 0], half: [0.315, h, 0.315], mat: M.canvas!, tint: CREAM, planes: Array.from({ length: N }, (_, i) => { const e = out(i * 2 * A); return [[e[0] * n[0], n[1], e[2] * n[0]], n[1] * h] as [V3, number]; }) }); };
/** a bar with square edges from a to b: `w` across (level), `h` the other way */
const lath = (p: Put, a: V3, b: V3, w: number, h: number, tint: V3) => { const y = norm(sub(b, a)), x = norm(cross(y, [0, 1, 0])); p.add({ at: mix(a, b, 0.5), half: [w, Math.hypot(...sub(b, a)) / 2, h], rot: [x, y, cross(x, y)], mat: M.print!, tint }); };
const under = 0.0175, ribAt = (k: number, j: number): V3 => at((2 * k + 1) * A, RIB[j]![0], RIB[j]![1] - under);
/** an iron leg: an S from under the top, in to a waist beside the pole, out to its foot */
const LEG: [number, number][] = [[0.300, 0.708], [0.232, 0.690], [0.165, 0.640], [0.116, 0.565], [0.094, 0.480], [0.098, 0.390], [0.132, 0.295], [0.190, 0.195], [0.262, 0.105], [0.345, 0.030]], LEGS = [0, 1, 2, 3].map(k => Math.PI / 4 + k * Math.PI / 2);
const leg = (p: Put, a: number, at10: number[], thick: (k: number) => number) => { for (let k = 0; k + 1 < at10.length; k++) { const [r0, y0] = LEG[at10[k]!]!, [r1, y1] = LEG[at10[k + 1]!]!; p.bar(at(a, r0, y0), at(a, r1, y1), thick(at10[k]!), thick(at10[k + 1]!) * 1.001, M.paint!, IRON); } };
/** a ring that lies level */
const ring = (p: Put, r: number, y: number, n: number, thick: number, mat: number, tint: V3) => { for (let k = 0; k < n; k++) p.bar(at(k * 2 * Math.PI / n, r, y), at((k + 1) * 2 * Math.PI / n, r, y), thick, thick * 0.999, mat, tint); };
const foot = (p: Put) => { p.drum([0, 0.0275, 0], 0.255, 0.0275, M.paint!, IRON); };
export const parasol: ObjectKind = { name: 'parasol', title: 'Café table with a parasol', make: (): Made => ({
  name: 'parasol', title: 'Café table with a parasol', size: [2.37, 2.64, 2.37], on: 'floor', close: { at: [0.06, 0.74, 0.06], far: 5.2 }, read: 0, moves: { kind: 'still' }, crowd: 49,
  levels: [
    { shape: 'pieces', error: 0, less: '', put(p) {
      for (let i = 0; i < N; i++) for (let j = 0; j < 2; j++) panel(p, i, RIB[j]!, RIB[j + 1]!, 0.0026, j ? 0.12 : 0.07, j ? [0.06, 0] : [0, -0.07]);
      scallops(p, 4); cap(p, true);
      // its frame: ribs under the seams, stretchers to the runner, the crown's hub, ends that hold the cloth
      for (let k = 0; k < N; k++) { const a = (2 * k + 1) * A; lath(p, at(a, 0.045, RIB[0]![1] - 0.031), ribAt(k, 1), 0.0085, 0.0125, WOOD); lath(p, ribAt(k, 1), at(a, R - 0.02, RIM - 0.006), 0.0085, 0.0118, WOOD); const g = at(a, 0.60, 2.283); lath(p, at(a, 0.040, 1.815), g, 0.0075, 0.0105, PALE); p.ball(g, [0.011, 0.011, 0.011], M.brass!, ONE); p.ball(at(a, R + 0.006, RIM - 0.006), [0.013, 0.013, 0.013], M.print!, WOOD); }
      p.drum([0, 2.428, 0], 0.052, 0.027, M.print!, WOOD); p.drum([0, 1.806, 0], 0.050, 0.030, M.print!, WOOD); p.drum([0, 1.764, 0], 0.037, 0.016, M.print!, PALE); p.drum([0, 1.812, 0], 0.0512, 0.0045, M.brass!, ONE);
      p.drum([0, 2.556, 0], 0.016, 0.026, M.print!, WOOD); p.ball([0, 2.610, 0], [0.030, 0.034, 0.030], M.print!, WOOD); p.drum([0, 2.546, 0], 0.028, 0.0075, M.print!, PALE);
      // the pole in two lengths, a ferrule between, a pin on its chain that holds the runner; the cord that lifts it
      p.drum([0, 1.23, 0], 0.0235, 1.19, M.print!, WOOD); p.drum([0, 1.17, 0], 0.0262, 0.052, M.brass!, ONE); for (const y of [1.122, 1.218]) p.drum([0, y, 0], 0.0282, 0.0055, M.brass!, ONE); p.bar([-0.045, 1.738, 0], [0.045, 1.738, 0], 0.0042, 0.0041, M.brass!, ONE); p.ball([0.047, 1.738, 0], [0.0095, 0.0095, 0.0095], M.brass!, ONE);
      p.bar([0.0295, 0.985, 0.012], [0.041, 1.78, 0.018], 0.0038, 0.0037, M.canvas!, CREAM); p.box([0.032, 0.98, 0.012], [0.008, 0.033, 0.0075], M.brass!, ONE); p.bar([0.034, 0.952, 0.012], [0.052, 0.944, 0.012], 0.0048, 0.0035, M.brass!, ONE); p.bar([0.034, 1.008, 0.012], [0.052, 1.016, 0.012], 0.0048, 0.0035, M.brass!, ONE); p.ball([0.040, 0.98, 0.012], [0.013, 0.024, 0.017], M.canvas!, CREAM);
      // the table: marble in a band of brass
      p.drum([0, TOP + 0.0015, 0], 0.391, 0.0110, M.film!, MARBLE); p.drum([0, TOP - 0.003, 0], 0.400, 0.0130, M.brass!, ONE); p.drum([0, TOP + 0.0125, 0], 0.038, 0.0042, M.brass!, ONE); p.drum([0, TOP - 0.022, 0], 0.335, 0.0065, M.paint!, IRON); p.drum([0, TOP - 0.038, 0], 0.052, 0.030, M.paint!, IRON);
      for (const a of LEGS) { leg(p, a, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9], k => k < 2 ? 0.0125 : 0.0135 + 0.0012 * Math.abs(k - 5)); p.ball(at(a, 0.338, 0.0165), [0.027, 0.0165, 0.027], M.paint!, IRON); p.ball(at(a, 0.094, 0.482), [0.0165, 0.0165, 0.0165], M.brass!, ONE); }
      ring(p, 0.079, 0.452, 12, 0.0085, M.paint!, IRON); ring(p, 0.150, 0.262, 16, 0.0075, M.paint!, IRON);
      // the foot: an iron plate with a step, a tube, a screw to hold the pole
      foot(p); p.ball([0, 0.055, 0], [0.205, 0.034, 0.205], M.paint!, IRON); p.drum([0, 0.19, 0], 0.0315, 0.15, M.paint!, IRON); p.drum([0, 0.335, 0], 0.0355, 0.012, M.paint!, IRON); p.bar([0.025, 0.285, 0.010], [0.070, 0.285, 0.028], 0.0065, 0.0065 * 0.99, M.steel!, ONE); p.add({ at: [0.076, 0.285, 0.0304], half: [0.0048, 0.021, 0.013], rot: rotY(-0.38), mat: M.steel!, tint: ONE, quad: [0, 1 / 0.021 ** 2, 1 / 0.013 ** 2, 0] });
    } },
    { shape: 'pieces', error: 0.013, less: 'scallops, what is turned or brass, the ribs\' bow', put(p) {
      for (let i = 0; i < N; i++) for (let j = 0; j < 2; j++) panel(p, i, RIB[j]!, RIB[j + 1]!, 0.0026, j ? 0.12 : 0.07, j ? [0.06, 0] : [0, -0.07]);
      band(p); cap(p, true);
      for (let k = 0; k < N; k++) { const a = (2 * k + 1) * A; lath(p, at(a, 0.05, 2.43), at(a, R - 0.02, RIM - 0.008), 0.0085, 0.0118, WOOD); }
      for (let k = 0; k < N; k++) { const a = (2 * k + 1) * A; lath(p, at(a, 0.040, 1.815), at(a, 0.60, 2.268), 0.0075, 0.0105, PALE); }
      p.drum([0, 1.80, 0], 0.048, 0.042, M.print!, WOOD); p.ball([0, 2.580, 0], [0.028, 0.064, 0.028], M.print!, WOOD); p.drum([0, 1.23, 0], 0.0238, 1.19, M.print!, WOOD);
      p.drum([0, TOP + 0.0015, 0], 0.391, 0.0110, M.film!, MARBLE); p.drum([0, TOP - 0.004, 0], 0.400, 0.0140, M.brass!, ONE); for (const a of LEGS) leg(p, a, [0, 2, 4, 6, 9], () => 0.0155);
      foot(p); p.drum([0, 0.19, 0], 0.033, 0.155, M.paint!, IRON); p.ball([0, 0.055, 0], [0.205, 0.034, 0.205], M.paint!, IRON);
    } },
    { shape: 'pieces', error: 0.028, less: 'stretchers, ribs, the legs\' bend: eight panels and their valance, the pole, a table', put(p) {
      for (let i = 0; i < N; i++) panel(p, i, [0, 2.50], [R + 0.004, RIM + 0.006], 0.004, 0.10, [0, 0]);
      band(p, 0.004); hat(p); p.drum([0, 1.304, 0], 0.026, 1.30, M.print!, WOOD);
      p.drum([0, TOP, 0], 0.400, 0.0155, M.film!, MARBLE); for (const a of LEGS) p.bar(at(a, 0.15, 0.715), at(a, 0.30, 0.01), 0.016, 0.0155, M.paint!, IRON); foot(p);
    } },
  ],
}) };
