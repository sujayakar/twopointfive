// A TREE OF BOXES over pieces: each node's box holds everything below it, so a ray that misses the box
// misses all of it. Split along the longest side, where the cost of looking is least (SAH).

import { turnVec, NO_TURN, type Vec3 } from './vec';
import type { PieceGiven } from './room';

export interface Bounds {
  low: Vec3;
  high: Vec3;
}
export interface TreeNode extends Bounds {
  /** a leaf's first piece, or an inner node's first child (the second follows it) */
  first: number;
  /** pieces in a leaf; 0 in an inner node */
  count: number;
}

export function boundsOf(piece: PieceGiven): Bounds {
  const turn = piece.turn ?? NO_TURN;
  const reach: Vec3 = [0, 0, 0];
  for (let axis = 0; axis < 3; axis++) {
    const edge: Vec3 = [0, 0, 0];
    edge[axis] = piece.half[axis]!;
    const turned = turnVec(turn, edge);
    for (let i = 0; i < 3; i++) reach[i]! += Math.abs(turned[i]!);
  }
  const hair = 1e-4;
  const at = (sign: number) =>
    piece.center.map((c, i) => c + sign * (reach[i]! + hair)) as Vec3;
  return { low: at(-1), high: at(1) };
}

const grow = (a: Bounds, b: Bounds): Bounds => ({
  low: a.low.map((x, i) => Math.min(x, b.low[i]!)) as Vec3,
  high: a.high.map((x, i) => Math.max(x, b.high[i]!)) as Vec3,
});
const EMPTY: Bounds = { low: [1e30, 1e30, 1e30], high: [-1e30, -1e30, -1e30] };
function area(b: Bounds): number {
  const [x, y, z] = b.high.map((h, i) => Math.max(0, h - b.low[i]!)) as Vec3;
  return x * y + y * z + z * x;
}

/** nodes and the order the pieces must stand in. Indices start at the offsets given. */
export function buildTree(bounds: Bounds[], nodeOffset = 0, pieceOffset = 0): { nodes: TreeNode[]; order: number[] } {
  const nodes: TreeNode[] = [];
  const order = bounds.map((_, i) => i);
  if (bounds.length === 0) return { nodes: [{ ...EMPTY, first: pieceOffset, count: 0 }], order };
  const middle = (i: number, axis: number) => bounds[i]!.low[axis]! + bounds[i]!.high[axis]!;

  const fill = (node: TreeNode, from: number, to: number) => {
    const all = order.slice(from, to);
    const box = all.reduce((b, i) => grow(b, bounds[i]!), EMPTY);
    Object.assign(node, box);
    const leaf = () => {
      node.first = pieceOffset + from;
      node.count = to - from;
    };
    if (all.length <= 2) return leaf();
    let best = { cost: area(box) * all.length, axis: -1, at: 0 };
    for (let axis = 0; axis < 3; axis++) {
      const sorted = [...all].sort((a, b) => middle(a, axis) - middle(b, axis));
      const right: number[] = [];
      let b = EMPTY;
      for (let k = sorted.length - 1; k > 0; k--) {
        b = grow(b, bounds[sorted[k]!]!);
        right[k] = area(b) * (sorted.length - k);
      }
      b = EMPTY;
      for (let k = 1; k < sorted.length; k++) {
        b = grow(b, bounds[sorted[k - 1]!]!);
        const cost = area(box) * 0.5 + area(b) * k + right[k]!;
        if (cost < best.cost) best = { cost, axis, at: k };
      }
    }
    if (best.axis < 0) return leaf();
    const sorted = [...all].sort((a, b) => middle(a, best.axis) - middle(b, best.axis));
    order.splice(from, all.length, ...sorted);
    const left: TreeNode = { ...EMPTY, first: 0, count: 0 };
    const rightNode: TreeNode = { ...EMPTY, first: 0, count: 0 };
    node.first = nodeOffset + nodes.length;
    node.count = 0;
    nodes.push(left, rightNode);
    fill(left, from, from + best.at);
    fill(rightNode, from + best.at, to);
  };
  const root: TreeNode = { ...EMPTY, first: 0, count: 0 };
  nodes.push(root);
  fill(root, 0, order.length);
  return { nodes, order };
}
