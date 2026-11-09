// A FICUS IN A POT: wood as pieces, a crown of leaves that sways. Coarser: fewer leaves, each as much larger, so that the crown is as thick; twigs go first.
import { mul } from '../math';
import type { Cluster, World } from '../scene/world';
import { M } from '../scene/materials';
import { domeTree, spray } from '../scene/plants';
import { apart, type Made, type ObjectKind, type Put } from './object';
const H = 2.0, POT = 0.40;
/** one leaf of `keep`, so many times as large: as much area in all */
export function thinned(c: Cluster, keep: number, wood: boolean, k = 1): Cluster { let n = 0; return { kind: c.kind, leaves: c.leaves.filter(l => l.wood ? wood : n++ % keep === (keep >> 1)).map(l => l.wood ? l : { ...l, size: l.size * Math.sqrt(keep) * k }) }; }
const sprays = [0, 1, 2, 3].map(i => spray(700 + i, 9, 10)), kept = new WeakMap<World, Map<string, number[]>>();
const clusters = (w: World, keep: number, wood: boolean): number[] => { let m = kept.get(w); if (!m) kept.set(w, m = new Map()); const k = keep + (wood ? 'w' : ''); let l = m.get(k); if (!l) m.set(k, l = sprays.map(s => { w.clusters.push(keep === 1 && wood ? s : thinned(s, keep, wood, keep > 4 ? 0.93 : 1)); return w.clusters.length - 1; })); return l; };
const tree = (seed: number) => apart(w => domeTree(w, { balls: [], tufts: [], sprays: [0, 1, 2, 3] }, [0, POT - 0.02, 0], H, 40 + seed));
function put(p: Put, keep: number, twigs: boolean, thinnest: number, bright = 1) {
  const t = tree(p.seed), ix = clusters(p.w, keep, twigs); p.from({ pieces: t.pieces.filter(q => Math.max(q.half[0], q.half[2]) >= thinnest), instances: t.instances.map(i => ({ ...i, tint: mul(i.tint, bright) })) }, i => ix[i.cluster]!);
  p.add({ at: [0, POT / 2 - 0.01, 0], half: [0.25, POT / 2 - 0.01, 0.25], mat: M.stone!, tint: [0.85, 0.55, 0.42], quad: [1 / 0.215 ** 2, -((0.07 / POT) ** 2) / 0.215 ** 2, 1 / 0.215 ** 2, -2 * (0.07 / POT) / 0.215] }); p.drum([0, POT - 0.012, 0], 0.235, 0.012, M.soil!, [1, 1, 1]); if (thinnest < 0.02) p.drum([0, POT - 0.012, 0], 0.268, 0.022, M.stone!, [0.85, 0.55, 0.42]);
}
export const ficus: ObjectKind = { name: 'ficus', title: 'Ficus in a pot', make: (): Made => ({
  name: 'ficus', title: 'Ficus in a pot', size: [1.95, 2.53, 2.0], on: 'floor', close: { at: [0.35, 1.75, 0.5], far: 3.2 }, read: 0, moves: { kind: 'sways' }, crowd: 100,
  levels: [
    { shape: 'leaves', error: 0, less: '', put(p) { put(p, 1, true, 0); } },
    { shape: 'leaves', error: 0.02, less: 'three leaves of four (the fourth is twice as long), twigs', put(p) { put(p, 4, false, 0.004); } },
    { shape: 'leaves', error: 0.05, less: 'fifteen leaves of sixteen, thin branches', put(p) { put(p, 16, false, 0.012, 1.035); } },
  ],
}) };
