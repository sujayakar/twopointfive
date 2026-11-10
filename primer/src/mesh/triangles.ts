// A TREE OF BOXES OVER TRIANGLES, as whole numbers for one buffer (wgsl/ray.wgsl reads it). Halved along the
// longest side: quick to make, and good enough for small triangles.

import type { Levels } from './clusters';
import type { Vec3 } from '../world/vec';

const NODE_WORDS = 8;
const TRIANGLE_WORDS = 18;
const LEAF = 4;
type Triangle = [number, number, number];

export function boundsOfTriangles(points: Vec3[], triangles: Triangle[]): { low: Vec3; high: Vec3 } {
  const low: Vec3 = [Infinity, Infinity, Infinity];
  const high: Vec3 = [-Infinity, -Infinity, -Infinity];
  for (const t of triangles) for (const i of t) points[i]!.forEach((v, k) => {
    low[k] = Math.min(low[k]!, v);
    high[k] = Math.max(high[k]!, v);
  });
  return { low, high };
}

/** appends the tree to `words` and gives its root's number */
export function triangleTree(levels: Levels, triangles: Triangle[], words: number[], asWord: (x: number) => number): number {
  const { points, normals } = levels;
  const order = triangles.map((_, i) => i);
  const middle = triangles.map((t) => [0, 1, 2].map((k) => points[t[0]]![k]! + points[t[1]]![k]! + points[t[2]]![k]!) as Vec3);
  // room for the nodes first, then the triangles in leaf order
  while (words.length % NODE_WORDS) words.push(0);
  const root = words.length / NODE_WORDS;
  const nodes: { low: Vec3; high: Vec3; first: number; count: number }[] = [];
  const add = () => nodes.push({ low: [0, 0, 0], high: [0, 0, 0], first: 0, count: 0 }) - 1;
  add();
  const fill = (index: number, from: number, to: number) => {
    const node = nodes[index]!;
    Object.assign(node, boundsOfTriangles(points, order.slice(from, to).map((i) => triangles[i]!)));
    if (to - from <= LEAF) {
      node.first = from;
      node.count = to - from;
      return;
    }
    const size = node.high.map((h, k) => h - node.low[k]!);
    const axis = size.indexOf(Math.max(...size));
    const sorted = order.slice(from, to).sort((a, b) => middle[a]![axis]! - middle[b]![axis]!);
    for (let i = 0; i < sorted.length; i++) order[from + i] = sorted[i]!;
    const half = (from + to) >> 1;
    const left = add();
    add();
    node.first = root + left;
    fill(left, from, half);
    fill(left + 1, half, to);
  };
  fill(0, 0, order.length);
  const start = (root + nodes.length) * NODE_WORDS;
  for (const n of nodes) {
    words.push(...n.low.map(asWord), n.count ? start + n.first * TRIANGLE_WORDS : n.first, ...n.high.map(asWord), n.count);
  }
  for (const i of order) {
    const t = triangles[i]!;
    for (const v of t) words.push(...points[v]!.map(asWord));
    for (const v of t) words.push(...normals[v]!.map(asWord));
  }
  return root;
}
