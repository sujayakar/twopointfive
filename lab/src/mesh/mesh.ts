// A MESH, as data, and the few things done to one whole: normals, areas, kinds of check. Nothing here knows about the GPU.

/** triangles with shared vertices: position, mean normal (its shortness is how much the normals under it spread), color (rgb, and a: how open) */
export interface Mesh { pos: Float32Array; nrm: Float32Array; col: Uint8Array; tri: Uint32Array }

/** smooth normals from the triangles, each by its area */
export function normalsOf(pos: Float32Array, tri: Uint32Array): Float32Array {
  const n = new Float32Array(pos.length);
  for (let t = 0; t < tri.length; t += 3) {
    const a = tri[t]! * 3, b = tri[t + 1]! * 3, c = tri[t + 2]! * 3;
    const ux = pos[b]! - pos[a]!, uy = pos[b + 1]! - pos[a + 1]!, uz = pos[b + 2]! - pos[a + 2]!, vx = pos[c]! - pos[a]!, vy = pos[c + 1]! - pos[a + 1]!, vz = pos[c + 2]! - pos[a + 2]!;
    const x = uy * vz - uz * vy, y = uz * vx - ux * vz, z = ux * vy - uy * vx;
    for (const k of [a, b, c]) { n[k]! += x; n[k + 1]! += y; n[k + 2]! += z; }
  }
  for (let i = 0; i < n.length; i += 3) { const l = Math.hypot(n[i]!, n[i + 1]!, n[i + 2]!) || 1; n[i]! /= l; n[i + 1]! /= l; n[i + 2]! /= l; }
  return n;
}

/** each triangle into four, with the middles of the edges shared */
export function split4(m: { pos: Float32Array; tri: Uint32Array }): { pos: Float32Array; tri: Uint32Array } {
  const nv = m.pos.length / 3, nt = m.tri.length / 3, mids = new Map<number, number>(), tri = new Uint32Array(nt * 12);
  let pos = new Float32Array((nv + nt * 1.5 + 16) * 3 | 0), n = nv; pos.set(m.pos);
  const mid = (a: number, b: number) => {
    const k = a < b ? a * nv + b : b * nv + a, got = mids.get(k); if (got !== undefined) return got;
    if ((n + 1) * 3 > pos.length) { const p = new Float32Array(pos.length * 1.5 | 0); p.set(pos); pos = p; }
    for (let i = 0; i < 3; i++) pos[n * 3 + i] = (pos[a * 3 + i]! + pos[b * 3 + i]!) / 2;
    mids.set(k, n); return n++;
  };
  for (let t = 0, o = 0; t < nt; t++, o += 12) {
    const a = m.tri[t * 3]!, b = m.tri[t * 3 + 1]!, c = m.tri[t * 3 + 2]!, ab = mid(a, b), bc = mid(b, c), ca = mid(c, a);
    tri.set([a, ab, ca, ab, b, bc, ca, bc, c, ab, bc, ca], o);
  }
  return { pos: pos.slice(0, n * 3), tri };
}

/** how many edges have other than two triangles on them (0: closed) */
export function openEdges(tri: Uint32Array): number {
  const seen = new Map<number, number>(); let top = 0; for (const v of tri) if (v > top) top = v;
  for (let t = 0; t < tri.length; t += 3) for (let k = 0; k < 3; k++) { const a = tri[t + k]!, b = tri[t + (k + 1) % 3]!, key = Math.min(a, b) * (top + 1) + Math.max(a, b); seen.set(key, (seen.get(key) ?? 0) + 1); }
  let open = 0; for (const n of seen.values()) if (n !== 2) open++; return open;
}
