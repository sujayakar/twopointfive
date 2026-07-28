import { Box, boxBounds } from "./scene";

// ---------------------------------------------------------------------------
// Binned-SAH BVH built on the CPU and flattened into a GPU storage buffer.
//
// Node layout (32 bytes):
//   bmin : vec3f   leftFirst : u32
//   bmax : vec3f   count     : u32
//
// count == 0  -> interior node; leftFirst is the left child index, right = left+1
// count  > 0  -> leaf; leftFirst is the first primitive index
//
// Primitives are reordered so each leaf covers a contiguous range, which lets
// the traversal loop read boxes sequentially and lets us drop the indirection
// table entirely.
// ---------------------------------------------------------------------------

const BINS = 12;
/**
 * One primitive per leaf.
 *
 * Measured, not assumed: at 1152x720 / 2 bounces the trace runs 15.07ms at
 * leaf<=4, 14.00ms at leaf<=2 and 13.12ms at leaf<=1. Ray-OBB is cheap enough
 * here that maximising cull quality beats minimising tree depth, and the node
 * count (1063 for 532 boxes, 34KB) is irrelevant at this scale.
 */
const DEFAULT_MAX_LEAF_SIZE = 1;

/** Must match the traversal stack depth in common.wgsl. */
export const TRAVERSAL_STACK_DEPTH = 32;
export const BVH_NODE_STRIDE_F32 = 8;

interface BuildNode {
  min: Float32Array;
  max: Float32Array;
  left: number;
  first: number;
  count: number;
}

export interface BVH {
  /** Flattened nodes, ready to upload. */
  nodes: Float32Array<ArrayBuffer>;
  nodeCount: number;
  /** Permutation of the original box array in leaf order. */
  order: Uint32Array;
  maxDepth: number;
}

const setEmpty = (min: Float32Array, max: Float32Array) => {
  min[0] = min[1] = min[2] = Infinity;
  max[0] = max[1] = max[2] = -Infinity;
};

const growPoint = (min: Float32Array, max: Float32Array, p: Float32Array, o: number) => {
  for (let i = 0; i < 3; i++) {
    if (p[o + i] < min[i]) min[i] = p[o + i];
    if (p[o + i] > max[i]) max[i] = p[o + i];
  }
};

const growBounds = (
  min: Float32Array,
  max: Float32Array,
  omin: Float32Array,
  omax: Float32Array,
  o: number,
) => {
  for (let i = 0; i < 3; i++) {
    if (omin[o + i] < min[i]) min[i] = omin[o + i];
    if (omax[o + i] > max[i]) max[i] = omax[o + i];
  }
};

function surfaceArea(min: Float32Array, max: Float32Array): number {
  const dx = max[0] - min[0];
  const dy = max[1] - min[1];
  const dz = max[2] - min[2];
  if (dx < 0 || dy < 0 || dz < 0) return 0;
  return 2 * (dx * dy + dy * dz + dz * dx);
}

export function buildBVH(boxes: Box[], maxLeafSize = DEFAULT_MAX_LEAF_SIZE): BVH {
  const n = boxes.length;
  if (n === 0) {
    const nodes = new Float32Array(BVH_NODE_STRIDE_F32);
    return { nodes, nodeCount: 1, order: new Uint32Array(0), maxDepth: 0 };
  }

  // Precompute per-primitive world AABBs and centroids as flat arrays.
  const pmin = new Float32Array(n * 3);
  const pmax = new Float32Array(n * 3);
  const cent = new Float32Array(n * 3);
  for (let i = 0; i < n; i++) {
    const b = boxBounds(boxes[i]);
    pmin[i * 3 + 0] = b.min.x; pmin[i * 3 + 1] = b.min.y; pmin[i * 3 + 2] = b.min.z;
    pmax[i * 3 + 0] = b.max.x; pmax[i * 3 + 1] = b.max.y; pmax[i * 3 + 2] = b.max.z;
    cent[i * 3 + 0] = (b.min.x + b.max.x) * 0.5;
    cent[i * 3 + 1] = (b.min.y + b.max.y) * 0.5;
    cent[i * 3 + 2] = (b.min.z + b.max.z) * 0.5;
  }

  const index = new Uint32Array(n);
  for (let i = 0; i < n; i++) index[i] = i;

  // Worst case for a binary BVH over n prims is 2n-1 nodes.
  const nodes: BuildNode[] = [];
  const makeNode = (first: number, count: number): number => {
    const min = new Float32Array(3);
    const max = new Float32Array(3);
    setEmpty(min, max);
    for (let i = first; i < first + count; i++) {
      growBounds(min, max, pmin, pmax, index[i] * 3);
    }
    nodes.push({ min, max, left: -1, first, count });
    return nodes.length - 1;
  };

  const root = makeNode(0, n);
  let maxDepth = 0;

  // Explicit stack; recursion would blow up on large levels.
  const stack: Array<{ node: number; depth: number }> = [{ node: root, depth: 0 }];
  const binMin = new Float32Array(BINS * 3);
  const binMax = new Float32Array(BINS * 3);
  const binCount = new Int32Array(BINS);
  const leftArea = new Float32Array(BINS);
  const leftCount = new Int32Array(BINS);

  while (stack.length > 0) {
    const { node: ni, depth } = stack.pop()!;
    const node = nodes[ni];
    if (depth > maxDepth) maxDepth = depth;
    if (node.count <= maxLeafSize) continue;

    // Split along the widest centroid axis.
    const cmin = new Float32Array([Infinity, Infinity, Infinity]);
    const cmax = new Float32Array([-Infinity, -Infinity, -Infinity]);
    for (let i = node.first; i < node.first + node.count; i++) {
      growPoint(cmin, cmax, cent, index[i] * 3);
    }
    let axis = 0;
    let extent = cmax[0] - cmin[0];
    for (let a = 1; a < 3; a++) {
      const e = cmax[a] - cmin[a];
      if (e > extent) { extent = e; axis = a; }
    }
    if (extent < 1e-7) continue; // all centroids coincide — leave as a leaf

    const scale = BINS / extent;
    binCount.fill(0);
    for (let b = 0; b < BINS; b++) {
      binMin[b * 3] = binMin[b * 3 + 1] = binMin[b * 3 + 2] = Infinity;
      binMax[b * 3] = binMax[b * 3 + 1] = binMax[b * 3 + 2] = -Infinity;
    }

    for (let i = node.first; i < node.first + node.count; i++) {
      const pi = index[i];
      let b = Math.floor((cent[pi * 3 + axis] - cmin[axis]) * scale);
      if (b < 0) b = 0;
      if (b >= BINS) b = BINS - 1;
      binCount[b]++;
      const sub = binMin.subarray(b * 3, b * 3 + 3);
      const sax = binMax.subarray(b * 3, b * 3 + 3);
      growBounds(sub, sax, pmin, pmax, pi * 3);
    }

    // Sweep left-to-right accumulating area*count, then right-to-left to find
    // the cheapest split plane.
    const accMin = new Float32Array(3);
    const accMax = new Float32Array(3);
    setEmpty(accMin, accMax);
    let cnt = 0;
    for (let b = 0; b < BINS - 1; b++) {
      cnt += binCount[b];
      growBounds(accMin, accMax, binMin, binMax, b * 3);
      leftCount[b] = cnt;
      leftArea[b] = surfaceArea(accMin, accMax);
    }

    setEmpty(accMin, accMax);
    cnt = 0;
    let bestCost = Infinity;
    let bestBin = -1;
    for (let b = BINS - 1; b > 0; b--) {
      cnt += binCount[b];
      growBounds(accMin, accMax, binMin, binMax, b * 3);
      const rightArea = surfaceArea(accMin, accMax);
      const lc = leftCount[b - 1];
      if (lc === 0 || cnt === 0) continue;
      const cost = leftArea[b - 1] * lc + rightArea * cnt;
      if (cost < bestCost) { bestCost = cost; bestBin = b; }
    }
    if (bestBin < 0) continue;

    // A split must beat the cost of keeping this node as a leaf.
    const parentArea = surfaceArea(node.min, node.max);
    if (parentArea > 0 && bestCost >= parentArea * node.count) continue;

    // In-place partition around the chosen plane.
    let lo = node.first;
    let hi = node.first + node.count - 1;
    while (lo <= hi) {
      const pi = index[lo];
      let b = Math.floor((cent[pi * 3 + axis] - cmin[axis]) * scale);
      if (b < 0) b = 0;
      if (b >= BINS) b = BINS - 1;
      if (b < bestBin) {
        lo++;
      } else {
        index[lo] = index[hi];
        index[hi] = pi;
        hi--;
      }
    }

    const leftCountFinal = lo - node.first;
    if (leftCountFinal === 0 || leftCountFinal === node.count) continue;

    const li = makeNode(node.first, leftCountFinal);
    const ri = makeNode(lo, node.count - leftCountFinal);
    node.left = li;
    node.count = 0;
    stack.push({ node: ri, depth: depth + 1 });
    stack.push({ node: li, depth: depth + 1 });
  }

  const out = new Float32Array(nodes.length * BVH_NODE_STRIDE_F32);
  const u32 = new Uint32Array(out.buffer);
  for (let i = 0; i < nodes.length; i++) {
    const nd = nodes[i];
    const o = i * BVH_NODE_STRIDE_F32;
    out[o + 0] = nd.min[0];
    out[o + 1] = nd.min[1];
    out[o + 2] = nd.min[2];
    u32[o + 3] = nd.count === 0 ? nd.left : nd.first;
    out[o + 4] = nd.max[0];
    out[o + 5] = nd.max[1];
    out[o + 6] = nd.max[2];
    u32[o + 7] = nd.count;
  }

  return { nodes: out, nodeCount: nodes.length, order: index, maxDepth };
}
