// FEWER TRIANGLES, by pulling edges together where that moves the surface least (quadric error, Garland and Heckbert). Vertices that are locked stay where they are, so that what
// lies beside this patch still fits it. The error that comes back is in metres.
// What is lost of the LOOK counts too: a spot of another color, or a bump, as wide as the edge and so strong, is as bad as so many metres of distance. The raster goes by both, rays by distance alone.
// A vertex carries means over the surface it stands for: its normal (the mean vector), its color, and how much normals that are gone spread about the one that is left.

/** vertices, growing: every level of a mesh adds its own */
export class Pool {
  n = 0; pos: Float32Array; nrm: Float32Array; col: Float32Array; spread: Float32Array;
  constructor(cap = 1024) { this.pos = new Float32Array(cap * 3); this.nrm = new Float32Array(cap * 3); this.col = new Float32Array(cap * 4); this.spread = new Float32Array(cap); }
  private grow() { const up = (a: Float32Array) => { const b = new Float32Array(a.length * 2); b.set(a); return b; }; this.pos = up(this.pos); this.nrm = up(this.nrm); this.col = up(this.col); this.spread = up(this.spread); }
  add() { if (this.n === this.spread.length) this.grow(); return this.n++; }
}

/** a patch is left as it is from where a collapse costs so many times (squared) what the cheap quarter did: one hard place must not spoil all that stands above it */
const DEAR = 9;
/** metres of error a metre of edge, for a color off by 1 and a normal off by a radian */
const COLOR = 0.6, NORMAL = 0.25;
const PULL = 1e-3;                                                  // how hard a vertex is held near its edge's middle, against the planes: keeps flats even

/** scratch for one patch at a time, kept between patches */
export class Simplifier {
  /** `ends`: vertices only ever go to where one of the two was, so that whatever else is known of each stays true of it */
  constructor(private ends = false) {} private pick = 0;
  /** where triangles of two kinds meet, the line between them is kept: its vertices go only along it, and those where three meet stay. Each knows its two neighbors on the line (−1: none) */
  private seamA = new Int32Array(0); private seamB = new Int32Array(0); private seam = new Uint8Array(0); private kinds = false;
  /** which vertex each that is gone went into (by their ids in the pool) */ fate = new Map<number, number>();
  private cap = 0; private mark = new Int32Array(0); private stamp = 0; private localOf = new Int32Array(0);
  private V = new Float64Array(0); private Q = new Float64Array(0); private N = new Float64Array(0); private C = new Float64Array(0); private S = new Float64Array(0);
  private ver = new Int32Array(0); private gone = new Uint8Array(0); private locked = new Uint8Array(0); private was = new Int32Array(0); private moved = new Uint8Array(0);
  private T = new Int32Array(0); private dead = new Uint8Array(0); private rings: Int32Array[] = []; private ringN = new Int32Array(0);
  private heapKey = new Float64Array(0); private heapA = new Int32Array(0); private heapB = new Int32Array(0); private heapVA = new Int32Array(0); private heapVB = new Int32Array(0); private heapN = 0;
  private at = new Float64Array(3); /** what a vertex says that its neighbors do not, as error a metre of edge; and the last cost's part that is distance */ private O = new Float64Array(0); private geo = 0; private near = new Int32Array(64);

  private room(tris: number) {
    if (tris <= this.cap) return; this.cap = tris * 2; const v = this.cap * 3;
    this.V = new Float64Array(v * 3); this.Q = new Float64Array(v * 11); this.N = new Float64Array(v * 3); this.C = new Float64Array(v * 4); this.S = new Float64Array(v); this.O = new Float64Array(v);
    this.ver = new Int32Array(v); this.gone = new Uint8Array(v); this.locked = new Uint8Array(v); this.was = new Int32Array(v); this.moved = new Uint8Array(v); this.ringN = new Int32Array(v);
    this.seamA = new Int32Array(v); this.seamB = new Int32Array(v); this.seam = new Uint8Array(v); this.T = new Int32Array(this.cap * 3); this.dead = new Uint8Array(this.cap);
  }
  private heap(n: number) {
    if (this.heapKey.length >= n) return; const k = n * 2, up = <A extends Float64Array | Int32Array>(a: A, b: A) => { b.set(a as never); return b; };
    this.heapKey = up(this.heapKey, new Float64Array(k)); this.heapA = up(this.heapA, new Int32Array(k)); this.heapB = up(this.heapB, new Int32Array(k)); this.heapVA = up(this.heapVA, new Int32Array(k)); this.heapVB = up(this.heapVB, new Int32Array(k));
  }
  private push(key: number, a: number, b: number) {
    this.heap(this.heapN + 1); let i = this.heapN++; const K = this.heapKey, A = this.heapA, B = this.heapB, VA = this.heapVA, VB = this.heapVB, va = this.ver[a]!, vb = this.ver[b]!;
    while (i > 0) { const p = (i - 1) >> 1; if (K[p]! <= key) break; K[i] = K[p]!; A[i] = A[p]!; B[i] = B[p]!; VA[i] = VA[p]!; VB[i] = VB[p]!; i = p; }
    K[i] = key; A[i] = a; B[i] = b; VA[i] = va; VB[i] = vb;
  }
  /** takes the least off the heap into slot heapN */
  private pop() {
    const K = this.heapKey, A = this.heapA, B = this.heapB, VA = this.heapVA, VB = this.heapVB, n = --this.heapN, k0 = K[0]!, a0 = A[0]!, b0 = B[0]!, va0 = VA[0]!, vb0 = VB[0]!;
    const k = K[n]!, a = A[n]!, b = B[n]!, va = VA[n]!, vb = VB[n]!; let i = 0;
    for (;;) { let c = 2 * i + 1; if (c >= n) break; if (c + 1 < n && K[c + 1]! < K[c]!) c++; if (K[c]! >= k) break; K[i] = K[c]!; A[i] = A[c]!; B[i] = B[c]!; VA[i] = VA[c]!; VB[i] = VB[c]!; i = c; }
    if (n > 0) { K[i] = k; A[i] = a; B[i] = b; VA[i] = va; VB[i] = vb; }
    K[n] = k0; A[n] = a0; B[n] = b0; VA[n] = va0; VB[n] = vb0;
  }
  /** where the two would best be one (at), and what that costs (squared metres); Infinity where it may not be */
  private cost(a: number, b: number): number {
    const L = this.locked, V = this.V, Q = this.Q, at = this.at, la = L[a]!, lb = L[b]!; if (la && lb) return Infinity;
    const qa = a * 11, qb = b * 11, A = a * 3, B = b * 3, w = Q[qa + 10]! + Q[qb + 10]!; if (w <= 0) return Infinity;
    const xx = Q[qa]! + Q[qb]!, xy = Q[qa + 1]! + Q[qb + 1]!, xz = Q[qa + 2]! + Q[qb + 2]!, yy = Q[qa + 3]! + Q[qb + 3]!, yz = Q[qa + 4]! + Q[qb + 4]!, zz = Q[qa + 5]! + Q[qb + 5]!, bx = Q[qa + 6]! + Q[qb + 6]!, by = Q[qa + 7]! + Q[qb + 7]!, bz = Q[qa + 8]! + Q[qb + 8]!, c = Q[qa + 9]! + Q[qb + 9]!;
    let x: number, y: number, z: number; this.pick = lb ? 1 : 0;
    if (this.ends) { const at2 = (v: number) => { const X = V[v]!, Y = V[v + 1]!, Z = V[v + 2]!; return xx * X * X + yy * Y * Y + zz * Z * Z + 2 * (xy * X * Y + xz * X * Z + yz * Y * Z) + 2 * (bx * X + by * Y + bz * Z) + c; }; if (!la && !lb) this.pick = at2(A) <= at2(B) ? 0 : 1; if (this.kinds) { const sa = this.seam[a]!, sb = this.seam[b]!; if (sa && sb && !(this.seamA[a] === b || this.seamB[a] === b || this.seamA[b] === a || this.seamB[b] === a)) return Infinity; if (sa && !sb) { if (lb) return Infinity; this.pick = 0; } else if (sb && !sa) { if (la) return Infinity; this.pick = 1; } } const v = this.pick ? B : A; x = V[v]!; y = V[v + 1]!; z = V[v + 2]!; } else if (la) { x = V[A]!; y = V[A + 1]!; z = V[A + 2]!; } else if (lb) { x = V[B]!; y = V[B + 1]!; z = V[B + 2]!; } else {
      const mx = (V[A]! + V[B]!) / 2, my = (V[A + 1]! + V[B + 1]!) / 2, mz = (V[A + 2]! + V[B + 2]!) / 2, l = PULL * w;
      const a11 = xx + l, a22 = yy + l, a33 = zz + l, r1 = l * mx - bx, r2 = l * my - by, r3 = l * mz - bz;
      const c11 = a22 * a33 - yz * yz, c12 = xz * yz - xy * a33, c13 = xy * yz - xz * a22, det = a11 * c11 + xy * c12 + xz * c13;
      if (Math.abs(det) < 1e-30) { x = mx; y = my; z = mz; } else {
        x = (c11 * r1 + c12 * r2 + c13 * r3) / det; y = (c12 * r1 + (a11 * a33 - xz * xz) * r2 + (xy * xz - a11 * yz) * r3) / det; z = (c13 * r1 + (xy * xz - a11 * yz) * r2 + (a11 * a22 - xy * xy) * r3) / det;
        // not farther than the edge is long
        const e = (V[A]! - V[B]!) ** 2 + (V[A + 1]! - V[B + 1]!) ** 2 + (V[A + 2]! - V[B + 2]!) ** 2; if ((x - mx) ** 2 + (y - my) ** 2 + (z - mz) ** 2 > e) { x = mx; y = my; z = mz; }
      }
    }
    at[0] = x; at[1] = y; at[2] = z;
    const q = xx * x * x + yy * y * y + zz * z * z + 2 * (xy * x * y + xz * x * z + yz * y * z) + 2 * (bx * x + by * y + bz * z) + c;
    const e = (V[A]! - V[B]!) ** 2 + (V[A + 1]! - V[B + 1]!) ** 2 + (V[A + 2]! - V[B + 2]!) ** 2;
    this.geo = Math.max(0, q / w); const o = Math.max(la ? 0 : this.O[a]!, lb ? 0 : this.O[b]!); return this.geo + (1e-6 + o * o) * e;                           // (on a flat the short edges go first)
  }
  /** would moving v to `at` turn a triangle over (those with `other` in them go away)? */
  private turns(v: number, other: number): boolean {
    const r = this.rings[v]!, n = this.ringN[v]!, T = this.T, V = this.V, [X, Y, Z] = this.at as unknown as [number, number, number], v3 = v * 3;
    for (let i = 0; i < n; i++) {
      const t = r[i]!; if (this.dead[t]) continue; let a = T[t * 3]!, b = T[t * 3 + 1]!, c = T[t * 3 + 2]!; if (a === other || b === other || c === other) continue;
      if (b === v) { const s = a; a = b; b = c; c = s; } else if (c === v) { const s = a; a = c; c = b; b = s; }
      const b3 = b * 3, c3 = c * 3, ex = V[c3]! - V[b3]!, ey = V[c3 + 1]! - V[b3 + 1]!, ez = V[c3 + 2]! - V[b3 + 2]!;
      const px = V[v3]! - V[b3]!, py = V[v3 + 1]! - V[b3 + 1]!, pz = V[v3 + 2]! - V[b3 + 2]!, qx = X - V[b3]!, qy = Y - V[b3 + 1]!, qz = Z - V[b3 + 2]!;
      const nx = ey * pz - ez * py, ny = ez * px - ex * pz, nz = ex * py - ey * px, mx = ey * qz - ez * qy, my = ez * qx - ex * qz, mz = ex * qy - ey * qx;
      const d = nx * mx + ny * my + nz * mz, ln = nx * nx + ny * ny + nz * nz, lm = mx * mx + my * my + mz * mz;
      if (lm < 1e-30 || (ln > 1e-30 && d < 0.25 * Math.sqrt(ln * lm))) return true;
      // no slivers either: the height over the far edge against its length
      const e2 = ex * ex + ey * ey + ez * ez; if (lm < 4e-4 * e2 * e2 && lm < ln) return true;
    }
    return false;
  }
  /** the vertices round v, into near */
  private round(v: number): number {
    const r = this.rings[v]!, T = this.T; let n = 0;
    for (let i = 0; i < this.ringN[v]!; i++) { const t = r[i]!; if (this.dead[t]) continue; for (let k = 0; k < 3; k++) { const u = T[t * 3 + k]!; if (u === v) continue; let has = false; for (let j = 0; j < n; j++) if (this.near[j] === u) { has = true; break; } if (!has) { if (n === this.near.length) { const b = new Int32Array(n * 2); b.set(this.near); this.near = b; } this.near[n++] = u; } } }
    return n;
  }
  private addRing(v: number, t: number) { let r = this.rings[v]!, n = this.ringN[v]!; if (n === r.length) { const b = new Int32Array(n * 2); b.set(r); r = this.rings[v] = b; } r[n] = t; this.ringN[v] = n + 1; }

  /**
   * `tri`: vertex ids of the pool. `isLocked(v)`: vertices that others share. Simplifies to about `target` triangles; new vertices join the pool.
   * Returns the triangles left and, of the worst place, the error as it is seen and the distance alone (metres, root mean square over what lies round it).
   */
  run(tri: Uint32Array, pool: Pool, isLocked: (v: number) => boolean, target: number, /** each triangle's kind */ kind?: Int32Array): { tri: Uint32Array; error: number; apart: number; kind?: Int32Array } {
    const nt = tri.length / 3; this.room(nt); if (this.mark.length < pool.n) { const cap = Math.max(pool.n * 2, 1024); this.mark = new Int32Array(cap); this.localOf = new Int32Array(cap); this.stamp = 0; }
    this.stamp++; const { V, Q, N, C, S, T, was, locked, ver, gone, moved, dead } = this; let nv = 0, cx = 0, cy = 0, cz = 0;
    for (let i = 0; i < 3 && i < tri.length; i++) { const p = tri[i]! * 3; cx += pool.pos[p]! / 3; cy += pool.pos[p + 1]! / 3; cz += pool.pos[p + 2]! / 3; }
    for (let i = 0; i < tri.length; i++) {
      const g = tri[i]!; let l: number;
      if (this.mark[g] === this.stamp) l = this.localOf[g]!; else {
        l = nv++; this.mark[g] = this.stamp; this.localOf[g] = l; was[l] = g; locked[l] = isLocked(g) ? 1 : 0; ver[l] = 0; gone[l] = 0; moved[l] = 0; this.ringN[l] = 0; if (!this.rings[l]) this.rings[l] = new Int32Array(12);
        V[l * 3] = pool.pos[g * 3]! - cx; V[l * 3 + 1] = pool.pos[g * 3 + 1]! - cy; V[l * 3 + 2] = pool.pos[g * 3 + 2]! - cz;
        for (let k = 0; k < 3; k++) N[l * 3 + k] = pool.nrm[g * 3 + k]!; for (let k = 0; k < 4; k++) C[l * 4 + k] = pool.col[g * 4 + k]!; S[l] = pool.spread[g]!;
        Q.fill(0, l * 11, l * 11 + 11);
      }
      T[i] = l;
    }
    const edgeN = new Map<number, number>();
    for (let t = 0; t < nt; t++) {
      dead[t] = 0; const a = T[t * 3]!, b = T[t * 3 + 1]!, c = T[t * 3 + 2]!; this.addRing(a, t); this.addRing(b, t); this.addRing(c, t);
      const A = a * 3, B = b * 3, D = c * 3, ux = V[B]! - V[A]!, uy = V[B + 1]! - V[A + 1]!, uz = V[B + 2]! - V[A + 2]!, vx = V[D]! - V[A]!, vy = V[D + 1]! - V[A + 1]!, vz = V[D + 2]! - V[A + 2]!;
      let x = uy * vz - uz * vy, y = uz * vx - ux * vz, z = ux * vy - uy * vx; const l = Math.hypot(x, y, z); if (l < 1e-20) continue; x /= l; y /= l; z /= l; const w = l / 6, d = -(x * V[A]! + y * V[A + 1]! + z * V[A + 2]!);
      for (const v of [a, b, c]) { const q = v * 11; Q[q]! += w * x * x; Q[q + 1]! += w * x * y; Q[q + 2]! += w * x * z; Q[q + 3]! += w * y * y; Q[q + 4]! += w * y * z; Q[q + 5]! += w * z * z; Q[q + 6]! += w * d * x; Q[q + 7]! += w * d * y; Q[q + 8]! += w * d * z; Q[q + 9]! += w * d * d; Q[q + 10]! += w; }
      for (let k = 0; k < 3; k++) { const p = T[t * 3 + k]!, q = T[t * 3 + (k + 1) % 3]!, key = p < q ? p * nv + q : q * nv + p; edgeN.set(key, (edgeN.get(key) ?? 0) + 1); }
    }
    // the rim of an open mesh: a plane stands up along it, so that it goes only along itself
    for (const [key, n] of edgeN) {
      if (n !== 1) continue; const a = Math.floor(key / nv), b = key % nv; if (locked[a] && locked[b]) continue; const t = this.triOn(a, b); if (t < 0) continue;
      const A = a * 3, B = b * 3, i0 = T[t * 3]! * 3, i1 = T[t * 3 + 1]! * 3, i2 = T[t * 3 + 2]! * 3, ux = V[i1]! - V[i0]!, uy = V[i1 + 1]! - V[i0 + 1]!, uz = V[i1 + 2]! - V[i0 + 2]!, vx = V[i2]! - V[i0]!, vy = V[i2 + 1]! - V[i0 + 1]!, vz = V[i2 + 2]! - V[i0 + 2]!, nx = uy * vz - uz * vy, ny = uz * vx - ux * vz, nz = ux * vy - uy * vx;
      const ex = V[B]! - V[A]!, ey = V[B + 1]! - V[A + 1]!, ez = V[B + 2]! - V[A + 2]!; let x = ey * nz - ez * ny, y = ez * nx - ex * nz, z = ex * ny - ey * nx; const l = Math.hypot(x, y, z); if (l < 1e-20) continue; x /= l; y /= l; z /= l; const w = (ex * ex + ey * ey + ez * ez), d = -(x * V[A]! + y * V[A + 1]! + z * V[A + 2]!);
      for (const v of [a, b]) { const q = v * 11; Q[q]! += w * x * x; Q[q + 1]! += w * x * y; Q[q + 2]! += w * x * z; Q[q + 3]! += w * y * y; Q[q + 4]! += w * y * z; Q[q + 5]! += w * z * z; Q[q + 6]! += w * d * x; Q[q + 7]! += w * d * y; Q[q + 8]! += w * d * z; Q[q + 9]! += w * d * d; }
    }
    this.kinds = !!kind; if (this.ends) this.fate = new Map();
    if (kind) {
      this.seam.fill(0, 0, nv); this.seamA.fill(-1, 0, nv); this.seamB.fill(-1, 0, nv); const of = new Map<number, number>(), many = new Uint8Array(nv); for (let t = 0; t < nt; t++) for (let k = 0; k < 3; k++) { const p = T[t * 3 + k]!, q = T[t * 3 + (k + 1) % 3]!, key = p < q ? p * nv + q : q * nv + p, o = of.get(key); if (o === undefined) { of.set(key, t); continue; } if (kind[o] === kind[t]) continue;
        for (const [v, w] of [[p, q], [q, p]] as const) { this.seam[v] = 1; if (many[v] === 0) this.seamA[v] = w; else if (many[v] === 1) this.seamB[v] = w; many[v]!++; }
        // (and a plane stands up along it)
        const A = p * 3, B = q * 3, i0 = T[t * 3]! * 3, i1 = T[t * 3 + 1]! * 3, i2 = T[t * 3 + 2]! * 3, ux = V[i1]! - V[i0]!, uy = V[i1 + 1]! - V[i0 + 1]!, uz = V[i1 + 2]! - V[i0 + 2]!, vx = V[i2]! - V[i0]!, vy = V[i2 + 1]! - V[i0 + 1]!, vz = V[i2 + 2]! - V[i0 + 2]!, nx = uy * vz - uz * vy, ny = uz * vx - ux * vz, nz = ux * vy - uy * vx, ex = V[B]! - V[A]!, ey = V[B + 1]! - V[A + 1]!, ez = V[B + 2]! - V[A + 2]!; let x = ey * nz - ez * ny, y = ez * nx - ex * nz, z = ex * ny - ey * nx; const l = Math.hypot(x, y, z); if (l < 1e-20) continue; x /= l; y /= l; z /= l; const w2 = 0.5 * (ex * ex + ey * ey + ez * ez), d = -(x * V[A]! + y * V[A + 1]! + z * V[A + 2]!); for (const v of [p, q]) { const o2 = v * 11; Q[o2]! += w2 * x * x; Q[o2 + 1]! += w2 * x * y; Q[o2 + 2]! += w2 * x * z; Q[o2 + 3]! += w2 * y * y; Q[o2 + 4]! += w2 * y * z; Q[o2 + 5]! += w2 * z * z; Q[o2 + 6]! += w2 * d * x; Q[o2 + 7]! += w2 * d * y; Q[o2 + 8]! += w2 * d * z; Q[o2 + 9]! += w2 * d * d; }
      }
      for (let v = 0; v < nv; v++) if (this.seam[v] && many[v] !== 2) { locked[v] = 1; this.seamA[v] = this.seamB[v] = -1; }
    }
    for (let v = 0; v < nv; v++) this.O[v] = locked[v] ? 0 : this.says(v);
    this.heapN = 0; for (const key of edgeN.keys()) { const a = Math.floor(key / nv), b = key % nv, c = this.cost(a, b); if (c < Infinity) this.push(c, a, b); }
    let left = nt, worst = 0, off = 0, plain = Infinity; const third = nt * 0.72;       // (what a quarter of them cost says what is dear)
    while (left > target && this.heapN > 0) {
      this.pop(); const s = this.heapN, a = this.heapA[s]!, b = this.heapB[s]!; if (gone[a] || gone[b] || ver[a] !== this.heapVA[s] || ver[b] !== this.heapVB[s]) continue;
      const key = this.heapKey[s]!, c0 = this.cost(a, b); if (c0 === Infinity) continue; const keep = this.pick ? b : a, drop = this.pick ? a : b, apart = this.geo;
      if (left <= third) { if (plain === Infinity) plain = worst; if (key > plain * DEAR) break; }
      // one hole more or less would be made where the two have other than two neighbors in common
      let shared = 0, faces = 0; { const n = this.round(a), mine = this.near.slice(0, n), m = this.round(b); for (let i = 0; i < n; i++) for (let j = 0; j < m; j++) if (mine[i] === this.near[j]) shared++; const r = this.rings[a]!; for (let i = 0; i < this.ringN[a]!; i++) { const t = r[i]!; if (!dead[t] && (T[t * 3] === b || T[t * 3 + 1] === b || T[t * 3 + 2] === b)) faces++; } }
      if (shared !== faces || faces > 2) continue;
      if (!locked[a] && this.turns(a, b)) continue; if (!locked[b] && this.turns(b, a)) continue;
      const [X, Y, Z] = [this.at[0]!, this.at[1]!, this.at[2]!];
      // what each stood for, and what it knew that its neighbors do not say
      const wa = Q[keep * 11 + 10]!, wb = Q[drop * 11 + 10]!, w = wa + wb; const la = this.odd(keep), lb = this.odd(drop);
      S[keep] = (wa * (S[keep]! + (locked[keep] ? 0 : la)) + wb * (S[drop]! + lb)) / w;
      if (this.ends) { /* (what is kept is as it was) */ } else if (!locked[keep]) { for (let k = 0; k < 3; k++) { N[keep * 3 + k] = (wa * N[keep * 3 + k]! + wb * N[drop * 3 + k]!) / w; } for (let k = 0; k < 4; k++) C[keep * 4 + k] = (wa * C[keep * 4 + k]! + wb * C[drop * 4 + k]!) / w; const l = Math.hypot(N[keep * 3]!, N[keep * 3 + 1]!, N[keep * 3 + 2]!) || 1; for (let k = 0; k < 3; k++) N[keep * 3 + k]! /= l; V[keep * 3] = X; V[keep * 3 + 1] = Y; V[keep * 3 + 2] = Z; moved[keep] = 1; } else S[keep] = pool.spread[was[keep]!]!;
      this.O[keep] = Math.max(this.O[keep]!, this.O[drop]!); for (let k = 0; k < 11; k++) Q[keep * 11 + k]! += Q[drop * 11 + k]!;
      if (this.ends) this.fate.set(was[drop]!, was[keep]!); if (this.kinds && this.seam[drop]) { const other = this.seamA[drop] === keep ? this.seamB[drop]! : this.seamA[drop]!; if (this.seamA[keep] === drop) this.seamA[keep] = other; else if (this.seamB[keep] === drop) this.seamB[keep] = other; if (other >= 0) { if (this.seamA[other] === drop) this.seamA[other] = keep; else if (this.seamB[other] === drop) this.seamB[other] = keep; } }
      const r = this.rings[drop]!, n = this.ringN[drop]!; gone[drop] = 1;
      for (let i = 0; i < n; i++) { const t = r[i]!; if (dead[t]) continue; let has = false; for (let k = 0; k < 3; k++) if (T[t * 3 + k] === keep) has = true; if (has) { dead[t] = 1; left--; continue; } for (let k = 0; k < 3; k++) if (T[t * 3 + k] === drop) T[t * 3 + k] = keep; this.addRing(keep, t); }
      { const q = this.rings[keep]!; let m = 0; for (let i = 0; i < this.ringN[keep]!; i++) if (!dead[q[i]!]) q[m++] = q[i]!; this.ringN[keep] = m; }
      ver[keep]!++; if (key > worst) worst = key; if (apart > off) off = apart;
      const m = this.round(keep), ring = this.near.slice(0, m); for (let i = 0; i < m; i++) { const c = this.cost(keep, ring[i]!); if (c < Infinity) this.push(c, keep, ring[i]!); }
    }
    // what is left faces as its triangles do: their vector area is that of all they stand for, so light falls on them as it did
    for (let v = 0; v < nv; v++) { if (this.ends || gone[v] || locked[v]) continue; let x = 0, y = 0, z = 0; const r = this.rings[v]!; for (let i = 0; i < this.ringN[v]!; i++) { const t = r[i]!; if (dead[t]) continue; const a = T[t * 3]! * 3, b = T[t * 3 + 1]! * 3, c = T[t * 3 + 2]! * 3, ux = V[b]! - V[a]!, uy = V[b + 1]! - V[a + 1]!, uz = V[b + 2]! - V[a + 2]!, wx = V[c]! - V[a]!, wy = V[c + 1]! - V[a + 1]!, wz = V[c + 2]! - V[a + 2]!; x += uy * wz - uz * wy; y += uz * wx - ux * wz; z += ux * wy - uy * wx; } const l = Math.hypot(x, y, z); if (l < 1e-30) continue; N[v * 3] = x / l; N[v * 3 + 1] = y / l; N[v * 3 + 2] = z / l; moved[v] = 1; }
    const left2 = kind ? new Int32Array(left) : undefined; let lo = 0; const out = new Uint32Array(left * 3), to = new Int32Array(nv).fill(-1); let o = 0;
    for (let t = 0; t < nt; t++) { if (dead[t]) continue; if (left2) left2[lo++] = kind![t]!; for (let k = 0; k < 3; k++) { const l = T[t * 3 + k]!; let g = to[l]!; if (g < 0) { if (!moved[l]) g = was[l]!; else { g = pool.add(); pool.pos[g * 3] = V[l * 3]! + cx; pool.pos[g * 3 + 1] = V[l * 3 + 1]! + cy; pool.pos[g * 3 + 2] = V[l * 3 + 2]! + cz; for (let q = 0; q < 3; q++) pool.nrm[g * 3 + q] = N[l * 3 + q]!; for (let q = 0; q < 4; q++) pool.col[g * 4 + q] = C[l * 4 + q]!; pool.spread[g] = S[l]!; } to[l] = g; } out[o++] = g; } }
    return { tri: out, error: Math.sqrt(worst), apart: Math.sqrt(off), ...(left2 ? { kind: left2 } : {}) };
  }
  /** what it says that its neighbors do not: of color, and of the way it faces */
  private says(v: number): number {
    const n = this.round(v), C = this.C; if (n < 3) return 0; let c = 0; for (let k = 0; k < 3; k++) { let m = 0; for (let i = 0; i < n; i++) m += C[this.near[i]! * 4 + k]!; c = Math.max(c, Math.abs(Math.sqrt(m / n) - Math.sqrt(C[v * 4 + k]!))); }
    return COLOR * c + NORMAL * Math.sqrt(2 * this.odd(v));
  }
  /** how far a vertex's normal is from what its neighbors' would make of it (1 − cos) */
  private odd(v: number): number {
    const n = this.round(v), N = this.N; let x = 0, y = 0, z = 0; for (let i = 0; i < n; i++) { const u = this.near[i]! * 3; x += N[u]!; y += N[u + 1]!; z += N[u + 2]!; }
    const l = Math.hypot(x, y, z); if (l < 1e-9) return 0; return Math.max(0, 1 - (x * N[v * 3]! + y * N[v * 3 + 1]! + z * N[v * 3 + 2]!) / l);
  }
  private triOn(a: number, b: number): number { const r = this.rings[a]!, T = this.T; for (let i = 0; i < this.ringN[a]!; i++) { const t = r[i]!; if (T[t * 3] === b || T[t * 3 + 1] === b || T[t * 3 + 2] === b) return t; } return -1; }
}
