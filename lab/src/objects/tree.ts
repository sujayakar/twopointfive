// A STREET TREE: the lab's dome of sprays on pale limbs. Coarser: fewer leaves, each as much larger; thin wood goes first.
import { mul } from '../math';
import type { Cluster, Piece, World } from '../scene/world';
import { domeTree, spray } from '../scene/plants';
import { thinned } from './ficus';
import { apart, type Made, type ObjectKind, type Put } from './object';
const H = 5.0, sprays = [0, 1, 2, 3].map(i => spray(720 + i)), kept = new WeakMap<World, Map<string, number[]>>();
const clusters = (w: World, keep: number, wood: boolean, k: number): number[] => { let m = kept.get(w); if (!m) kept.set(w, m = new Map()); const id = keep + (wood ? 'w' : ''); let l = m.get(id); if (!l) m.set(id, l = sprays.map((s): number => { const c: Cluster = keep === 1 && wood ? s : thinned(s, keep, wood, k); w.clusters.push(c); return w.clusters.length - 1; })); return l; };
const made = (seed: number) => apart(w => domeTree(w, { balls: [], tufts: [], sprays: [0, 1, 2, 3] }, [0, 0, 0], H, 60 + seed));
function put(p: Put, keep: number, twigs: boolean, thinnest: number, bright = 1, k = 1) { const t = made(p.seed), ix = clusters(p.w, keep, twigs, k); p.from({ pieces: t.pieces.filter(q => Math.max(q.half[0], q.half[2]) >= thinnest).map(q => q.at[1] < 0.02 ? { ...q, planes: [[[0, -1, 0], q.at[1]]] as Piece['planes'] } : q), instances: t.instances.map(i => ({ ...i, tint: mul(i.tint, bright) })) }, i => ix[i.cluster]!); }
export const tree: ObjectKind = { name: 'tree', title: 'Street tree', make: (): Made => ({
  name: 'tree', title: 'Street tree', size: [5.1, 5.6, 5.2], on: 'floor', close: { at: [0.8, 3.4, 1.4], far: 5 }, read: 0, moves: { kind: 'sways' }, crowd: 49,
  levels: [
    { shape: 'leaves', error: 0, less: '', put(p) { put(p, 1, true, 0); } },
    { shape: 'leaves', error: 0.04, less: 'three leaves of four, twigs', put(p) { put(p, 4, false, 0.01); } },
    { shape: 'leaves', error: 0.10, less: 'fifteen of sixteen, thin branches', put(p) { put(p, 16, false, 0.03, 1.02, 0.95); } },
  ],
}) };
