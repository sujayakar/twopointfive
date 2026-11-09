// A PATCH OF GRAVEL: a skin of the floor. Near, every stone is one; farther off, the large ones on a ground of the mean color; far, the ground alone. What must not move between levels is its brightness.
import { type V3, rng, rotX, rotY, rotZ, compose } from '../math';
import { M } from '../scene/materials';
import type { Made, ObjectKind, Put } from './object';
const S = 1.5, N = 900;
interface Stone { at: V3; half: V3; a: number; b: number; c: number; tint: V3 }
function stones(seed: number): Stone[] {
  const q = rng(seed * 7919), out: Stone[] = [], grid: [number, number, number][] = [];
  for (let i = 0; i < N * 6 && out.length < N; i++) { const big = out.length < N * 0.12, r = big ? 0.022 + 0.020 * q() : 0.010 + 0.014 * q(), x = (q() - 0.5) * (S - 2 * r), z = (q() - 0.5) * (S - 2 * r); if (grid.some(([a, b, c]) => (a - x) ** 2 + (b - z) ** 2 < (0.82 * (c + r)) ** 2)) continue; grid.push([x, z, r]); const k = 0.72 + 0.5 * q(), w = q(), tint: V3 = [k * (0.96 + 0.10 * w), k * (0.95 + 0.04 * w), k * (0.95 - 0.06 * w)]; out.push({ at: [x, 0.012 + r * 0.30, z], half: [r, r * (0.50 + 0.25 * q()), r * (0.70 + 0.3 * q())], a: q() * 6.28, b: (q() - 0.5) * 0.7, c: (q() - 0.5) * 0.7, tint }); }
  return out;
}
const ground = (p: Put, mat: number, tint: V3, high = 0.012) => p.box([0, high / 2, 0], [S / 2, high / 2, S / 2], mat, tint);
const put = (p: Put, s: Stone) => p.ball(s.at, s.half, M.stone!, s.tint, { rot: compose(compose(rotY(s.a), rotX(s.b)), rotZ(s.c)) });
export const gravel: ObjectKind = { name: 'gravel', title: 'Patch of gravel', make: (): Made => ({
  name: 'gravel', title: 'Patch of gravel', size: [S, 0.06, S], on: 'skin', close: { at: [0, 0.02, 0.2], far: 2.2 }, read: 0, moves: { kind: 'still' }, crowd: 36,
  levels: [
    { shape: 'pieces', error: 0, less: '', put(p) { ground(p, M.gravel!, [0.62, 0.60, 0.58]); for (const s of stones(p.seed)) put(p, s); } },
    { shape: 'pieces', error: 0.015, less: 'stones under 3 cm', put(p) { ground(p, M.gravel!, [0.86, 0.85, 0.84], 0.016); for (const s of stones(p.seed)) if (s.half[0] > 0.0215) put(p, s); } },
    { shape: 'pieces', error: 0.030, less: 'every stone', put(p) { ground(p, M.gravel!, [0.90, 0.89, 0.88], 0.018); } },
  ],
}) };
