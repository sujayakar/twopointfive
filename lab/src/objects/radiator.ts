// AN OLD RADIATOR of cast iron: ten sections of three columns, each a casting with rounded heads, on four feet; a valve with its wheel and pipe at one end, a lockshield at the other. What its levels must keep is the
// dark between the columns: a section that loses its gaps takes their mean.
import { type V3, mul, rotX } from '../math';
import { M } from '../scene/materials';
import type { Made, ObjectKind, Put } from './object';
const ENAMEL: V3 = [0.51, 0.565, 0.48], BLACK: V3 = [0.15, 0.145, 0.145], ONE: V3 = [1, 1, 1];
const N = 10, STEP = 0.058, HALF = 0.0238, DEEP = 0.064, LOW = 0.118, TOP = 0.742, xOf = (i: number) => (i - (N - 1) / 2) * STEP - 0.012, X0 = xOf(0) - HALF, X1 = xOf(N - 1) + HALF, HUB = 0.165, HIGH = 0.690;
/** the head of a section, where its columns meet: an egg on its side, flat where the next one is joined to it */
const head = (p: Put, x: number, y: number, h: number, half = HALF) => p.add({ at: [x, y, 0], half: [half, h, 0.0985], mat: M.paint!, tint: ENAMEL, quad: [1 / (half * 1.27) ** 2, 1 / h ** 2, 1 / 0.0985 ** 2, 0] });
const column = (p: Put, x: number, z: number, rx = 0.0208, rz = 0.0225) => p.add({ at: [x, (LOW + TOP) / 2, z], half: [rx, (TOP - LOW) / 2 - 0.045, rz], mat: M.paint!, tint: ENAMEL, quad: [1 / rx ** 2, 0, 1 / rz ** 2, 0] });
const along = (p: Put, a: number, b: number, y: number, r: number, mat: number, tint: V3, z = 0) => p.bar([a, y, z], [b, y, z], r, r * 0.999, mat, tint);
/** a six-sided nut round the x axis */
const nut = (p: Put, x: number, y: number, r: number, half: number, mat: number, tint: V3) => p.add({ at: [x, y, 0], half: [half, r * 1.16, r * 1.16], mat, tint, planes: [0, 1, 2, 3, 4, 5].map(k => [[0, Math.sin(k * Math.PI / 3 + 0.2), Math.cos(k * Math.PI / 3 + 0.2)], r] as [V3, number]), curves: [{ at: [0, 0, 0], r: [half * 1.9, r * 1.26, r * 1.26] }] });
/** a section as one slab, round above and below by planes that touch its heads; the dark between its columns is in its tint */
const slab = (p: Put, x: number, half: number, tint: V3, mat: number, less = 0) => { const mid = (LOW + TOP) / 2, touch = (a: number, h: number, y: number): [V3, number][] => [1, -1].map(s => { const ny = Math.sin(a) / h, nz = Math.cos(a) / 0.0985, l = Math.hypot(ny, nz); return [[0, ny / l, s * nz / l], (1 + ny * (y - mid)) / l] as [V3, number]; }); p.add({ at: [x, mid, 0], half: [half, (TOP - LOW) / 2 - 0.001 - less, 0.0865], mat, tint, planes: [...touch(0.62, 0.048, TOP - 0.048 - less), ...touch(1.12, 0.048, TOP - 0.048 - less), ...touch(-0.85, 0.042, LOW + 0.042 + less)] }); };
const feet = (p: Put, fat = 1) => { for (const i of [0, N - 1]) for (const s of [-1, 1]) p.bar([xOf(i), LOW + 0.03, s * 0.062], [xOf(i), 0.022, s * 0.074], 0.0225 * fat, 0.0135 * fat, M.paint!, ENAMEL); };
const pads = (p: Put) => { for (const i of [0, N - 1]) for (const s of [-1, 1]) p.ball([xOf(i), 0.0165, s * 0.0775], [0.0205, 0.0165, 0.0250], M.paint!, ENAMEL); };
/** what hides the hole in the floor */
const plate = (p: Put, x: number) => p.add({ at: [x, 0.006, 0], half: [0.032, 0.006, 0.032], mat: M.chrome!, tint: [0.9, 0.9, 0.9], curves: [{ at: [0, -0.006, 0], r: [0.032, 0.0125, 0.032] }] });
/** a slab is flat where columns were round: duller, and of the mean of its face and the dark between */
const SPREAD = 1.022;
const PIPE = X1 + 0.072, BACK = X0 - 0.060;
/** the valve's wheel, domed */
const knob = (p: Put, r: number) => p.add({ at: [PIPE, HUB + 0.084, 0], half: [r, 0.0135, r], mat: M.film!, tint: BLACK, quad: [1 / r ** 2, 0, 1 / r ** 2, 0], curves: [{ at: [0, -0.004, 0], r: [r + 0.0025, 0.0195, r + 0.0025] }] });
export const radiator: ObjectKind = { name: 'radiator', title: 'Cast iron radiator', make: (): Made => ({
  name: 'radiator', title: 'Cast iron radiator', size: [0.73, 0.74, 0.205], on: 'floor', close: { at: [0.30, 0.24, 0.04], far: 3.0 }, read: 0, moves: { kind: 'still' }, crowd: 64,
  levels: [
    { shape: 'pieces', error: 0, less: '', put(p) {
      for (let i = 0; i < N; i++) { const x = xOf(i); for (const z of [-DEEP, 0, DEEP]) column(p, x, z); head(p, x, TOP - 0.048, 0.048); head(p, x, LOW + 0.042, 0.042); }
      along(p, X0 + 0.004, X1 - 0.004, HIGH, 0.0285, M.paint!, ENAMEL); along(p, X0 + 0.004, X1 - 0.004, HUB, 0.0285, M.paint!, ENAMEL);
      // a collar round each end's hubs, plugs in those no pipe comes to, an air valve in one
      for (const y of [HIGH, HUB]) { along(p, X0 - 0.006, X0 + 0.010, y, 0.0250, M.paint!, ENAMEL); along(p, X1 - 0.010, X1 + 0.006, y, 0.0250, M.paint!, ENAMEL); } nut(p, X0 - 0.011, HIGH, 0.0165, 0.0075, M.paint!, ENAMEL); nut(p, X1 + 0.011, HIGH, 0.0165, 0.0075, M.paint!, ENAMEL);
      along(p, X1 + 0.015, X1 + 0.029, HIGH, 0.0070, M.brass!, ONE); p.box([X1 + 0.033, HIGH, 0], [0.0052, 0.0042, 0.0042], M.brass!, ONE, { rot: rotX(0.5) }); p.bar([X1 + 0.023, HIGH, 0.005], [X1 + 0.023, HIGH - 0.004, 0.017], 0.0027, 0.0024, M.brass!, ONE);
      // feet: cast with the end sections, waisted, on a pad
      feet(p); pads(p); for (const i of [0, N - 1]) for (const s of [-1, 1]) { p.ball([xOf(i), 0.098, s * 0.066], [0.0238, 0.034, 0.0285], M.paint!, ENAMEL); }
      // the valve: a union, its body, the stem and its wheel; the pipe into the floor
      nut(p, X1 + 0.0165, HUB, 0.0215, 0.0105, M.brass!, ONE); along(p, X1 + 0.02, X1 + 0.052, HUB, 0.0145, M.brass!, ONE); p.ball([PIPE, HUB + 0.001, 0], [0.0265, 0.0245, 0.0235], M.brass!, ONE); p.drum([PIPE, HUB - 0.026, 0], 0.0165, 0.018, M.brass!, ONE); p.add({ at: [PIPE, HUB - 0.046, 0], half: [0.0235, 0.0095, 0.0235], mat: M.brass!, tint: ONE, planes: [0, 1, 2, 3, 4, 5].map(k => [[Math.sin(k * Math.PI / 3 + 0.3), 0, Math.cos(k * Math.PI / 3 + 0.3)], 0.0200] as [V3, number]) });
      p.drum([PIPE, HUB + 0.031, 0], 0.0160, 0.012, M.brass!, ONE); p.drum([PIPE, HUB + 0.046, 0], 0.0118, 0.010, M.brass!, ONE); p.drum([PIPE, HUB + 0.064, 0], 0.0068, 0.014, M.steel!, ONE);
      knob(p, 0.036); for (let k = 0; k < 8; k++) { const a = k * Math.PI / 4; p.bar([PIPE + 0.0335 * Math.sin(a), HUB + 0.0725, 0.0335 * Math.cos(a)], [PIPE + 0.0335 * Math.sin(a), HUB + 0.0895, 0.0335 * Math.cos(a)], 0.0065, 0.0062, M.film!, BLACK); } p.ball([PIPE, HUB + 0.0975, 0], [0.0085, 0.0045, 0.0085], M.brass!, ONE);
      p.drum([PIPE, (HUB - 0.043) / 2, 0], 0.0135, (HUB - 0.049) / 2, M.paint!, ENAMEL); plate(p, PIPE);
      // the other end: a lockshield under its cap
      nut(p, X0 - 0.0165, HUB, 0.0215, 0.0105, M.brass!, ONE); along(p, X0 - 0.042, X0 - 0.02, HUB, 0.0145, M.brass!, ONE); p.ball([BACK, HUB, 0], [0.0225, 0.0225, 0.0215], M.brass!, ONE); p.drum([BACK, HUB + 0.030, 0], 0.0145, 0.016, M.brass!, ONE); p.ball([BACK, HUB + 0.046, 0], [0.0145, 0.010, 0.0145], M.brass!, ONE); p.drum([BACK, HUB - 0.026, 0], 0.0165, 0.014, M.brass!, ONE); p.drum([BACK, (HUB - 0.027) / 2, 0], 0.0135, (HUB - 0.033) / 2, M.paint!, ENAMEL); plate(p, BACK);
    } },
    { shape: 'pieces', error: 0.010, less: 'the middle columns but at its ends, nuts, the wheel\'s lobes', put(p) {
      for (let i = 0; i < N; i++) { const x = xOf(i); for (const z of [-DEEP, DEEP]) column(p, x, z, 0.0208, 0.0225); head(p, x, TOP - 0.048, 0.048); }
      for (const i of [0, N - 1]) column(p, xOf(i), 0); p.box([xOf(0) / 2 + xOf(N - 1) / 2, 0.42, 0], [(X1 - X0) / 2 - 0.06, 0.27, 0.012], M.paint!, ENAMEL); for (const y of [HIGH, HUB]) along(p, X0 + 0.004, X1 + 0.004, y, 0.0285, M.paint!, ENAMEL);
      for (let i = 0; i < N; i++) head(p, xOf(i), LOW + 0.042, 0.042); feet(p, 1.06); pads(p); plate(p, PIPE); plate(p, BACK);
      along(p, X1 - 0.01, PIPE, HUB, 0.0175, M.brass!, ONE); p.ball([PIPE, HUB, 0], [0.0265, 0.030, 0.0235], M.brass!, ONE); p.drum([PIPE, HUB / 2 + 0.002, 0], 0.0145, HUB / 2, M.paint!, ENAMEL); p.drum([PIPE, HUB + 0.050, 0], 0.012, 0.025, M.brass!, ONE); knob(p, 0.038);
      along(p, BACK, X0 + 0.01, HUB, 0.0175, M.brass!, ONE); p.drum([BACK, HUB + 0.012, 0], 0.020, 0.038, M.brass!, ONE); p.drum([BACK, HUB / 2 + 0.002, 0], 0.0145, HUB / 2, M.paint!, ENAMEL);
    } },
    { shape: 'pieces', error: 0.018, less: 'columns and heads: a section is one slab', put(p) {
      for (let i = 0; i < N; i++) slab(p, xOf(i), HALF - 0.0034, mul(ENAMEL, SPREAD), M.plaster!); along(p, X0 + 0.004, X1 + 0.004, HUB, 0.0285, M.paint!, ENAMEL); feet(p, 1.08); pads(p); plate(p, PIPE); plate(p, BACK);
      p.drum([PIPE, HUB / 2 + 0.002, 0], 0.0150, HUB / 2, M.paint!, ENAMEL); p.drum([BACK, HUB / 2 + 0.002, 0], 0.0145, HUB / 2, M.paint!, ENAMEL); along(p, X1 - 0.01, PIPE + 0.016, HUB, 0.021, M.brass!, ONE); along(p, BACK - 0.012, X0 + 0.01, HUB, 0.021, M.brass!, ONE); p.drum([BACK, HUB + 0.032, 0], 0.0150, 0.024, M.brass!, ONE); p.drum([PIPE, HUB + 0.045, 0], 0.013, 0.026, M.brass!, ONE); knob(p, 0.038);
    } },
    { shape: 'pieces', error: 0.045, less: 'sections: one block, the valve one post', put(p) {
      slab(p, (X0 + X1) / 2, (X1 - X0) / 2 - 0.002, mul(ENAMEL, 0.87), M.plaster!, 0.006); feet(p, 1.15); p.drum([PIPE, 0.125, 0], 0.021, 0.125, M.paint!, [0.52, 0.46, 0.33]); p.drum([BACK, 0.105, 0], 0.017, 0.105, M.paint!, [0.64, 0.58, 0.38]); along(p, BACK, PIPE, HUB, 0.019, M.brass!, ONE);
    } },
  ],
}) };

