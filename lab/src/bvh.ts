// Boxes → a tree flattened eight times, once for each octant a ray can head into. Nodes are in the order such a ray should meet them, and each says where
// to go when it is missed, so a walk needs no stack and still goes near to far.
export type Bounds = { mn: [number, number, number]; mx: [number, number, number] };
export interface Bvh { order: number[]; nodes: ArrayBuffer; perOctant: number }
const BINS = 12;
type Node = { mn: number[]; mx: number[]; axis: number; a?: Node; b?: Node; first: number; count: number };

export function buildBvh(items: Bounds[], leaf = 2, octants = 8): Bvh {
  const order = items.map((_, i) => i), cen = items.map(b => [0, 1, 2].map(k => (b.mn[k]! + b.mx[k]!) / 2));
  const bound = (f: number, n: number) => { const mn = [1e9, 1e9, 1e9], mx = [-1e9, -1e9, -1e9]; for (let i = f; i < f + n; i++) { const b = items[order[i]!]!; for (let k = 0; k < 3; k++) { mn[k] = Math.min(mn[k]!, b.mn[k]!); mx[k] = Math.max(mx[k]!, b.mx[k]!); } } return { mn, mx }; };
  const area = (mn: number[], mx: number[]) => { const x = mx[0]! - mn[0]!, y = mx[1]! - mn[1]!, z = mx[2]! - mn[2]!; return x < 0 ? 0 : x * y + y * z + z * x; };
  let total = 0;
  const make = (first: number, count: number): Node => {
    total++;
    const { mn, mx } = bound(first, count), node: Node = { mn, mx, axis: 0, first, count };
    if (count <= leaf) return node;
    let best = Infinity, axis = -1, split = 0, lo = 0, scale = 0;
    for (let k = 0; k < 3; k++) {
      let a = 1e9, b = -1e9; for (let i = first; i < first + count; i++) { const c = cen[order[i]!]![k]!; a = Math.min(a, c); b = Math.max(b, c); }
      if (b - a < 1e-5) continue;
      const s = BINS / (b - a), bins = Array.from({ length: BINS }, () => ({ n: 0, mn: [1e9, 1e9, 1e9], mx: [-1e9, -1e9, -1e9] }));
      for (let i = first; i < first + count; i++) { const it = items[order[i]!]!, bin = bins[Math.min(BINS - 1, Math.floor((cen[order[i]!]![k]! - a) * s))]!; bin.n++; for (let q = 0; q < 3; q++) { bin.mn[q] = Math.min(bin.mn[q]!, it.mn[q]!); bin.mx[q] = Math.max(bin.mx[q]!, it.mx[q]!); } }
      for (let cut = 1; cut < BINS; cut++) {
        const side = (f: number, t: number) => { const p = [1e9, 1e9, 1e9], q = [-1e9, -1e9, -1e9]; let n = 0; for (let i = f; i < t; i++) { n += bins[i]!.n; for (let m = 0; m < 3; m++) { p[m] = Math.min(p[m]!, bins[i]!.mn[m]!); q[m] = Math.max(q[m]!, bins[i]!.mx[m]!); } } return n ? n * area(p, q) : Infinity; };
        const cost = side(0, cut) + side(cut, BINS); if (cost < best) { best = cost; axis = k; split = cut; lo = a; scale = s; }
      }
    }
    let mid = first + (count >> 1);
    if (axis >= 0) {
      let i = first, j = first + count - 1;
      while (i <= j) { if (Math.min(BINS - 1, Math.floor((cen[order[i]!]![axis]! - lo) * scale)) < split) i++; else { const t = order[i]!; order[i] = order[j]!; order[j] = t; j--; } }
      if (i > first && i < first + count) mid = i;
    }
    node.axis = Math.max(0, axis); node.count = 0; node.a = make(first, mid - first); node.b = make(mid, first + count - mid); return node;
  };
  const root = items.length ? make(0, items.length) : null, per = Math.max(1, total);
  const nodes = new ArrayBuffer(per * 32 * octants), f = new Float32Array(nodes), u = new Uint32Array(nodes);
  for (let o = 0; o < octants; o++) {
    let n = 0;
    const put = (x: Node) => {
      const at = (o * per + n++) * 8; f.set(x.mn, at); f.set(x.mx, at + 4);
      if (x.a && x.b) { u[at + 7] = 0; const flip = (o >> x.axis) & 1; put(flip ? x.b : x.a); put(flip ? x.a : x.b); } else u[at + 7] = (x.first << 4) | x.count;
      u[at + 3] = n;
    };
    if (root) put(root); else { f.set([1e9, 1e9, 1e9], o * 8); f.set([1e9, 1e9, 1e9], o * 8 + 4); u[o * 8 + 3] = 1; u[o * 8 + 7] = 0; }
  }
  return { order, nodes, perOctant: per };
}

/** new bounds in an old tree (one order): the same shape, refitted from the leaves up */
export function refitBvh(b: Bvh, bound: (item: number) => Bounds) {
  const f = new Float32Array(b.nodes), u = new Uint32Array(b.nodes);
  for (let i = b.perOctant - 1; i >= 0; i--) {
    const o = i * 8, leaf = u[o + 7]!, n = leaf & 15; let x0 = 1e9, y0 = 1e9, z0 = 1e9, x1 = -1e9, y1 = -1e9, z1 = -1e9;
    if (n) { for (let k = 0; k < n; k++) { const q = bound(b.order[(leaf >>> 4) + k]!); x0 = Math.min(x0, q.mn[0]); y0 = Math.min(y0, q.mn[1]); z0 = Math.min(z0, q.mn[2]); x1 = Math.max(x1, q.mx[0]); y1 = Math.max(y1, q.mx[1]); z1 = Math.max(z1, q.mx[2]); } }
    else { const a = (i + 1) * 8, c = u[a + 3]! * 8; for (const q of [a, c]) { x0 = Math.min(x0, f[q]!); y0 = Math.min(y0, f[q + 1]!); z0 = Math.min(z0, f[q + 2]!); x1 = Math.max(x1, f[q + 4]!); y1 = Math.max(y1, f[q + 5]!); z1 = Math.max(z1, f[q + 6]!); } }
    f[o] = x0; f[o + 1] = y0; f[o + 2] = z0; f[o + 4] = x1; f[o + 5] = y1; f[o + 6] = z1;
  }
}
