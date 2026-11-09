// A CHAIR of turned wood with a cushion: pieces only. Its back is slats, so what its levels must keep is how much light comes through.
import { type V3, rotX } from '../math';
import { M } from '../scene/materials';
import type { Made, ObjectKind, Put } from './object';
const WOOD: V3 = [0.74, 0.53, 0.36], CLOTH: V3 = [0.36, 0.56, 0.60], W = 0.21, SEAT = 0.43, TOP = 0.86;
/** along the back's posts: how far back it leans at a height */
const lean = (y: number) => -0.20 - 0.16 * Math.max(0, y - SEAT);
const legs = (p: Put, fat = 1) => { for (const s of [-1, 1]) { p.bar([s * 0.195, SEAT - 0.02, 0.175], [s * 0.215, 0, 0.20], 0.021 * fat, 0.013 * fat, M.print!, WOOD); p.bar([s * 0.185, SEAT - 0.02, -0.185], [s * 0.205, 0, -0.235], 0.021 * fat, 0.013 * fat, M.print!, WOOD); } };
const posts = (p: Put, fat = 1) => { for (const s of [-1, 1]) p.bar([s * 0.185, SEAT + 0.005, lean(SEAT)], [s * 0.178, TOP, lean(TOP)], 0.018 * fat, 0.014 * fat, M.print!, WOOD); };
const seat = (p: Put) => { p.box([0, SEAT + 0.004, 0], [W + 0.006, 0.018, 0.208], M.print!, WOOD); p.box([0, SEAT + 0.042, 0.004], [W - 0.012, 0.019, 0.190], M.canvas!, CLOTH); p.ball([0, SEAT + 0.052, 0.004], [W - 0.02, 0.030, 0.182], M.canvas!, CLOTH); };
const slats = (p: Put, n: number, thick: number) => { for (let i = 0; i < n; i++) { const x = (i - (n - 1) / 2) * (0.29 / (n - 1)); p.add({ at: [x, 0.675, lean(0.675) - 0.004], half: [thick, 0.150, 0.0055], mat: M.print!, tint: WOOD, rot: rotX(-0.158) }); } };
export const chair: ObjectKind = { name: 'chair', title: 'Chair', make: (): Made => ({
  name: 'chair', title: 'Chair', size: [0.46, 0.86, 0.52], on: 'floor', close: { at: [0.02, 0.55, 0], far: 2.6 }, read: 0, moves: { kind: 'loose', dense: 520, rough: 0.6, floats: true },
  levels: [
    { shape: 'pieces', error: 0, less: '', put(p) {
      legs(p); posts(p); seat(p); slats(p, 5, 0.0155);
      for (const y of [0.525, 0.83]) for (let i = 0; i < 3; i++) { const x = (k: number) => -0.185 + k * 0.1233, z = (k: number) => lean(y) - (k === 1 || k === 2 ? 0.016 : 0); p.bar([x(i), y, z(i)], [x(i + 1), y, z(i + 1)], y > 0.7 ? 0.021 : 0.013, y > 0.7 ? 0.0208 : 0.0128, M.print!, WOOD); }
      for (const s of [-1, 1]) p.bar([s * 0.203, 0.20, 0.186], [s * 0.196, 0.20, -0.208], 0.010, 0.0099, M.print!, WOOD); p.bar([-0.20, 0.20, -0.005], [0.20, 0.20, -0.005], 0.010, 0.0099, M.print!, WOOD);
      for (const s of [-1, 1]) for (const z of [0.20, -0.235]) p.drum([s * (z > 0 ? 0.215 : 0.205), 0.003, z], 0.0145, 0.003, M.rubber!, [1, 1, 1]);
    } },
    { shape: 'pieces', error: 0.010, less: 'feet, the rails\' bend, round edges of the cushion', put(p) {
      legs(p); posts(p); p.box([0, SEAT + 0.004, 0], [W + 0.006, 0.018, 0.208], M.print!, WOOD); p.box([0, SEAT + 0.049, 0.004], [W - 0.014, 0.027, 0.186], M.canvas!, CLOTH); slats(p, 5, 0.0155);
      p.add({ at: [0, 0.83, lean(0.83) - 0.010], half: [0.185, 0.020, 0.017], mat: M.print!, tint: WOOD }); p.add({ at: [0, 0.525, lean(0.525) - 0.010], half: [0.185, 0.012, 0.011], mat: M.print!, tint: WOOD }); for (const s of [-1, 1]) p.bar([s * 0.203, 0.20, 0.186], [s * 0.196, 0.20, -0.208], 0.010, 0.0099, M.print!, WOOD); p.bar([-0.20, 0.20, -0.005], [0.20, 0.20, -0.005], 0.010, 0.0099, M.print!, WOOD);
    } },
    { shape: 'pieces', error: 0.030, less: 'stretchers; five slats are two as wide, so as much light comes through', put(p) {
      for (const s of [-1, 1]) for (const z of [0.188, -0.212]) p.add({ at: [s * 0.203, SEAT / 2 - 0.004, z], half: [0.0145, SEAT / 2 - 0.004, 0.0145], mat: M.print!, tint: WOOD });
      p.box([0, SEAT + 0.004, 0], [W + 0.006, 0.018, 0.208], M.print!, WOOD); p.box([0, SEAT + 0.048, 0.004], [W - 0.014, 0.026, 0.184], M.canvas!, CLOTH); for (const s of [-1, 1]) p.add({ at: [s * 0.182, 0.65, lean(0.65)], half: [0.0145, 0.21, 0.0145], mat: M.print!, tint: WOOD, rot: rotX(-0.158) });
      p.add({ at: [0, 0.83, lean(0.83) - 0.008], half: [0.168, 0.0205, 0.016], mat: M.print!, tint: WOOD }); for (const s of [-1, 1]) p.add({ at: [s * 0.085, 0.665, lean(0.665) - 0.004], half: [0.0385, 0.150, 0.0055], mat: M.print!, tint: WOOD, rot: rotX(-0.158) });
    } },
  ],
}) };
