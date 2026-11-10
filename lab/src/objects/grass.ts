// A PATCH OF GRASS: tufts of blades on earth. Coarser: fewer blades, each wider.
import { type V3, rng, rotY, mul } from '../math';
import type { Cluster, World } from '../scene/world';
import { M } from '../scene/materials';
import { grassTuft, GRASS } from '../scene/plants';
import type { Made, ObjectKind, Put } from './object';
const S = 1.4, tufts = [0, 1, 2].map(i => grassTuft(760 + i, 90)), kept = new WeakMap<World, Map<number, number[]>>();
const clusters = (w: World, keep: number): number[] => { let m = kept.get(w); if (!m) kept.set(w, m = new Map()); let l = m.get(keep); if (!l) m.set(keep, l = tufts.map(t => { const c: Cluster = keep === 1 ? t : { kind: 0, leaves: t.leaves.filter((_, i) => i % keep === 0).map(b => ({ ...b, size: b.size * 0.92 })) }; w.clusters.push(c); return w.clusters.length - 1; })); return l; };
function put(p: Put, keep: number) { const q = rng(p.seed * 331), ix = clusters(p.w, keep); p.box([0, 0.012, 0], [S / 2, 0.012, S / 2], M.soil!, [1, 1, 1]); for (let i = 0; i < 70; i++) { const at: V3 = [(q() - 0.5) * (S - 0.2), 0.02, (q() - 0.5) * (S - 0.2)], s = 0.09 + 0.07 * q(), t = 0.8 + 0.4 * q(); p.w.instances.push({ cluster: ix[i % 3]!, at: p.P(...at), base: p.P(...at), rot: rotY(q() * 6.28 + p.yaw), scale: s, sway: 0.09, phase: q() * 6.28, tint: mul(GRASS, t) }); } }
export const grass: ObjectKind = { name: 'grass', title: 'Patch of grass', make: (): Made => ({
  name: 'grass', title: 'Patch of grass', size: [1.46, 0.27, 1.4], on: 'skin', close: { at: [0, 0.1, 0.2], far: 2.4 }, read: 0, moves: { kind: 'sways' }, crowd: 49,
  levels: [
    { shape: 'leaves', error: 0, less: '', put(p) { put(p, 1); } },
    { shape: 'leaves', error: 0.02, less: 'seven blades of eight, the eighth as wide as a leaf', put(p) { put(p, 8); } },
  ],
}) };
