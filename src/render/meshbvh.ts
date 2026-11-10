// Triangle BVH over the character mesh — the CPU half of mesh-traced character shadows (render/meshShadows.ts is the GPU half; the WGSL
// traversal lives in common.wgsl under FLAG_MESH_SHADOWS). Pure TypeScript, no GPU: the game builds the topology with it once.
//
// The scheme. Every mannequin is the same glTF mesh, so ONE tree topology serves all of them: it is built once over the rest-pose triangles
// (binned SAH on centroids, leaves of ≤ LEAF_MAX triangles, triangles reordered so every leaf — and every subtree — owns a contiguous run),
// flattened depth-first with skip pointers (left child = i + 1, `skip[i]` = the node after this subtree = where a ray goes when it misses
// node i or is done with it; a stackless walk). Per frame and per character the GPU re-skins the triangle corners and REFITS the boxes
// bottom-up, never rebuilding: a posed limb only ever moves boxes, the topology of a body does not change. What is skinned and refit is the
// man's own tree (wornTree below): this one less the parts he does not wear. A refit tree is looser than a rebuilt one for extreme poses (an arm stretched out overlaps the
// torso's box).

/** triangles per leaf at most (the GPU leaf loop and the packed count byte assume ≤ 255; 4 is the usual any-hit sweet spot) */
export const LEAF_MAX = 4;
/** characters that get a tree on the GPU (= BoxWorld's capsule character cap: the capsule list and the skin instances are the same list in the same order) */
export const MESH_MAX_CHARS = 8;
const BINS = 12;          // SAH candidate planes per axis
const MAX_DEPTH = 40;     // past this a range is split at its median instead of by SAH (bounds the refit's level loop; never reached on the mannequin)

export interface MeshBvh {
  numTris: number;            // T
  numNodes: number;           // N
  maxDepth: number;           // deepest node (root = 0)
  corners: Uint32Array;       // 3T vertex indices: triangle t of the BVH order is (corners[3t], corners[3t+1], corners[3t+2])
  triOrder: Uint32Array;      // T: BVH-ordered triangle → source triangle (indices[3s..])
  skip: Uint32Array;          // N: the node a ray visits next when it misses node i or has finished it (N = done)
  right: Int32Array;          // N: right child of an internal node (its left child is i + 1); −1 for a leaf
  triStart: Uint32Array;      // N: first triangle of a leaf (of the subtree, for internal nodes — subtrees own contiguous runs)
  triCount: Uint32Array;      // N: triangles in a leaf; 0 = internal node
  subtreeEnd: Uint32Array;    // N: one past the last triangle under node i (leaf: triStart + triCount)
  stats: { leaves: number; avgLeaf: number; maxLeaf: number; medianSplits: number };
}

/** Build the topology over rest-pose `positions` (3 per vertex) and `indices` (3 per triangle). ~13.7k triangles: a few tens of ms. */
export function buildMeshBvh(positions: Float32Array, indices: Uint32Array): MeshBvh {
  const T = (indices.length / 3) | 0;
  // per source triangle: centroid + bounds
  const cen = new Float64Array(T * 3), bmin = new Float64Array(T * 3), bmax = new Float64Array(T * 3);
  for (let t = 0; t < T; t++) {
    for (let a = 0; a < 3; a++) {
      const p0 = positions[indices[t * 3] * 3 + a], p1 = positions[indices[t * 3 + 1] * 3 + a], p2 = positions[indices[t * 3 + 2] * 3 + a];
      cen[t * 3 + a] = (p0 + p1 + p2) / 3; bmin[t * 3 + a] = Math.min(p0, p1, p2); bmax[t * 3 + a] = Math.max(p0, p1, p2);
    }
  }
  const ids = new Uint32Array(T); for (let t = 0; t < T; t++) ids[t] = t;
  const cap = 2 * T + 1;   // a binary tree over ≥ 1-triangle leaves never has more nodes
  const skip = new Uint32Array(cap), right = new Int32Array(cap).fill(-1), triStart = new Uint32Array(cap), triCount = new Uint32Array(cap), subtreeEnd = new Uint32Array(cap);
  let N = 0, maxDepth = 0, medianSplits = 0;
  const binCount = new Int32Array(BINS), binMin = new Float64Array(BINS * 3), binMax = new Float64Array(BINS * 3);
  const rightArea = new Float64Array(BINS), rightN = new Int32Array(BINS);
  const area = (mn: ArrayLike<number>, mx: ArrayLike<number>, o = 0) => { const dx = mx[o] - mn[o], dy = mx[o + 1] - mn[o + 1], dz = mx[o + 2] - mn[o + 2]; return dx < 0 ? 0 : 2 * (dx * dy + dy * dz + dz * dx); };

  const build = (start: number, end: number, d: number): number => {
    const node = N++; if (d > maxDepth) maxDepth = d;
    triStart[node] = start; subtreeEnd[node] = end;
    const count = end - start;
    if (count <= LEAF_MAX) { triCount[node] = count; skip[node] = N; return node; }
    // centroid bounds of the range
    const cmin = [Infinity, Infinity, Infinity], cmax = [-Infinity, -Infinity, -Infinity];
    for (let k = start; k < end; k++) { const t = ids[k]; for (let a = 0; a < 3; a++) { const c = cen[t * 3 + a]; if (c < cmin[a]) cmin[a] = c; if (c > cmax[a]) cmax[a] = c; } }
    let bestAxis = -1, bestBin = -1, bestCost = Infinity;
    if (d < MAX_DEPTH) {
      for (let axis = 0; axis < 3; axis++) {
        const ext = cmax[axis] - cmin[axis]; if (!(ext > 1e-9)) continue;
        const sc = BINS / ext;
        binCount.fill(0); binMin.fill(Infinity); binMax.fill(-Infinity);
        for (let k = start; k < end; k++) {
          const t = ids[k]; let b = Math.floor((cen[t * 3 + axis] - cmin[axis]) * sc); if (b >= BINS) b = BINS - 1; if (b < 0) b = 0;
          binCount[b]++;
          for (let a = 0; a < 3; a++) { if (bmin[t * 3 + a] < binMin[b * 3 + a]) binMin[b * 3 + a] = bmin[t * 3 + a]; if (bmax[t * 3 + a] > binMax[b * 3 + a]) binMax[b * 3 + a] = bmax[t * 3 + a]; }
        }
        // suffix sweep: bounds / counts of bins [i, BINS)
        const mn = [Infinity, Infinity, Infinity], mx = [-Infinity, -Infinity, -Infinity]; let n = 0;
        for (let i = BINS - 1; i > 0; i--) {
          n += binCount[i]; for (let a = 0; a < 3; a++) { mn[a] = Math.min(mn[a], binMin[i * 3 + a]); mx[a] = Math.max(mx[a], binMax[i * 3 + a]); }
          rightN[i] = n; rightArea[i] = n ? area(mn, mx) : 0;
        }
        // prefix sweep: split before bin i (left = [0, i), right = [i, BINS))
        mn[0] = mn[1] = mn[2] = Infinity; mx[0] = mx[1] = mx[2] = -Infinity; n = 0;
        for (let i = 1; i < BINS; i++) {
          n += binCount[i - 1]; for (let a = 0; a < 3; a++) { mn[a] = Math.min(mn[a], binMin[(i - 1) * 3 + a]); mx[a] = Math.max(mx[a], binMax[(i - 1) * 3 + a]); }
          if (n === 0 || rightN[i] === 0) continue;
          const cost = area(mn, mx) * n + rightArea[i] * rightN[i];
          if (cost < bestCost) { bestCost = cost; bestAxis = axis; bestBin = i; }
        }
      }
    }
    let mid: number;
    if (bestAxis >= 0) {
      // partition in place: centroid bin < bestBin goes left
      const ext = cmax[bestAxis] - cmin[bestAxis], sc = BINS / ext;
      let i = start, j = end - 1;
      while (i <= j) {
        const t = ids[i]; let b = Math.floor((cen[t * 3 + bestAxis] - cmin[bestAxis]) * sc); if (b >= BINS) b = BINS - 1; if (b < 0) b = 0;
        if (b < bestBin) i++; else { ids[i] = ids[j]; ids[j] = t; j--; }
      }
      mid = i;
      if (mid === start || mid === end) bestAxis = -1;   // (cannot happen with both counts > 0, but never recurse on an empty side)
    }
    if (bestAxis < 0) {
      // median split along the longest centroid axis (degenerate centroids, or the depth cap): always terminates, count halves
      medianSplits++;
      let axis = 0; if (cmax[1] - cmin[1] > cmax[axis] - cmin[axis]) axis = 1; if (cmax[2] - cmin[2] > cmax[axis] - cmin[axis]) axis = 2;
      const sub = Array.from(ids.subarray(start, end)).sort((p, q) => cen[p * 3 + axis] - cen[q * 3 + axis]);
      ids.set(sub, start);
      mid = (start + end) >> 1;
    }
    build(start, mid!, d + 1);              // left child = node + 1 by construction
    right[node] = build(mid!, end, d + 1);
    skip[node] = N;                         // everything under `node` has been numbered: the next number is the node after this subtree
    return node;
  };
  if (T > 0) build(0, T, 0);

  const corners = new Uint32Array(T * 3);
  for (let k = 0; k < T; k++) { const t = ids[k]; corners[k * 3] = indices[t * 3]; corners[k * 3 + 1] = indices[t * 3 + 1]; corners[k * 3 + 2] = indices[t * 3 + 2]; }
  let leaves = 0, maxLeaf = 0; for (let i = 0; i < N; i++) if (triCount[i] > 0) { leaves++; if (triCount[i] > maxLeaf) maxLeaf = triCount[i]; }
  return {
    numTris: T, numNodes: N, maxDepth, corners, triOrder: ids,
    skip: skip.slice(0, N), right: right.slice(0, N), triStart: triStart.slice(0, N), triCount: triCount.slice(0, N), subtreeEnd: subtreeEnd.slice(0, N),
    stats: { leaves, avgLeaf: leaves ? T / leaves : 0, maxLeaf, medianSplits },
  };
}

/** A MAN'S OWN TREE: the mesh's tree less what he does not wear. The cast's mesh is every variant's clothes at once (three times what any one man has on), so a tree over all of it is mostly
 *  parts that are not there. The leaves keep their order, a leaf keeps its worn triangles in theirs, a node with nothing worn under one of its children is that other child, and one
 *  with no more than a leaf's worth worn under it is a leaf: a ray meets the same triangles in the same order as in the whole tree. Numbered and linked as the whole tree is. */
export interface WornTree {
  numTris: number; numNodes: number;
  verts: Uint32Array;      // the mesh's vertices he wears, ascending
  corners: Uint32Array;    // 3 a triangle: places in `verts`
  nodes: Uint32Array;      // 4 a node: skip, triStart, triCount (0 = internal), right child (0 on a leaf; the left is the next node)
  leaves: number;
  order: Uint32Array;      // the leaves, then the internal nodes by how far they stand over their deepest leaf (the refit's order: a node's children always come in an earlier step)
  steps: Uint32Array;      // where each of those steps begins in `order`, and its end: step 0 = the leaves
}
export function wornTree(b: MeshBvh, vertexCount: number, worn: (vertex: number) => boolean): WornTree {
  const N = b.numNodes, kept = new Uint32Array(N), on = new Uint8Array(b.numTris);   // worn triangles under each node (children are numbered after their parent)
  for (let t = 0; t < b.numTris; t++) on[t] = worn(b.corners[t * 3]) ? 1 : 0;
  for (let i = N - 1; i >= 0; i--) {
    if (b.triCount[i] === 0) { kept[i] = kept[i + 1] + kept[b.right[i]]; continue; }
    for (let t = b.triStart[i], e = t + b.triCount[i]; t < e; t++) kept[i] += on[t];
  }
  const T = N ? kept[0] : 0, nodes = new Uint32Array(N * 4), height = new Uint8Array(N), tris = new Uint32Array(T); let n = 0, nt = 0, H = 0;
  const emit = (from: number): number => {
    let i = from; while (b.triCount[i] === 0 && (!kept[i + 1] || !kept[b.right[i]])) i = kept[i + 1] ? i + 1 : b.right[i];
    const at = n++, o = at * 4; nodes[o + 1] = nt;
    if (kept[i] <= LEAF_MAX) { for (let t = b.triStart[i]; t < b.subtreeEnd[i]; t++) if (on[t]) tris[nt++] = t; nodes[o + 2] = kept[i]; }
    else { const l = emit(i + 1), r = emit(b.right[i]); nodes[o + 3] = r; height[at] = 1 + Math.max(height[l], height[r]); }
    nodes[o] = n; H = Math.max(H, height[at] + 1);
    return at;
  };
  if (T) emit(0);
  const place = new Uint32Array(vertexCount), corners = new Uint32Array(T * 3); let V = 0;
  for (let k = 0; k < T; k++) for (let j = 0; j < 3; j++) place[b.corners[tris[k] * 3 + j]] = 1;
  for (let v = 0; v < vertexCount; v++) if (place[v]) V++;
  const verts = new Uint32Array(V); V = 0; for (let v = 0; v < vertexCount; v++) if (place[v]) { place[v] = V; verts[V++] = v; }
  for (let k = 0; k < T; k++) for (let j = 0; j < 3; j++) corners[k * 3 + j] = place[b.corners[tris[k] * 3 + j]];
  const steps = new Uint32Array(H + 1); for (let i = 0; i < n; i++) steps[height[i] + 1]++;
  for (let h = 0; h < H; h++) steps[h + 1] += steps[h];
  const order = new Uint32Array(n), next = steps.slice(); for (let i = 0; i < n; i++) order[next[height[i]]++] = i;
  return { numTris: T, numNodes: n, verts, corners, nodes: nodes.slice(0, n * 4), leaves: H ? steps[1] : 0, order, steps };
}

/** FNV-1a over the topology arrays (MeshShadows.digest). */
export function topologyDigest(b: MeshBvh): string {
  let h = 0x811c9dc5 >>> 0;
  const feed = (arr: ArrayLike<number>) => { for (let i = 0; i < arr.length; i++) { let v = arr[i] >>> 0; for (let k = 0; k < 4; k++) { h ^= v & 255; h = Math.imul(h, 0x01000193) >>> 0; v >>>= 8; } } };
  feed(b.skip); feed(b.triStart); feed(b.triCount); feed(b.corners);
  return h.toString(16).padStart(8, '0');
}

/** leaf boxes are grown by this much (m) so a hit lying exactly on a box face survives the slab test's rounding; parents inherit it */
export const LEAF_PAD = 1e-4;




