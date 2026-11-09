// CLOTH: a net of points that keep their distances, pressed by the air on every cell (by how squarely the cell meets it) and pulled by its own weight. Drawn as a plate
// a cell, whose normal runs smoothly from corner to corner. Stepped once a tick; no clock, no dice.
import { type V3, type Rot, add, sub, mul, cross, norm } from '../math';
import { World, F, type Piece, type Mark } from './world';
import type { Air } from './air';
export interface ClothSpec {
  /** where it hangs from, the way its width and its height run, and how big it is */ at: V3; u: V3; v: V3; width: number; height: number; nx: number; ny: number;
  /** which corners of cells are held */ held: (i: number, j: number) => boolean;
  mat: number; tint: V3; art?: string; back?: boolean; /** kg a square metre */ weight?: number; /** how it stands against folding, 0 to 1 */ stiff?: number; /** 1 = it ends in a point */ taper?: number; thick?: number; flags?: number; /** what it can not pass: n · x ≥ d */ wall?: [V3, number];
}
const RHO = 1.2, SUB = 2, PASSES = 5;
export class Cloth {
  readonly cells: Piece[] = []; private p: Float64Array; private was: Float64Array; private keep: Uint8Array; private ties: Int32Array; private rest: Float64Array; private k: Float64Array; private nx: number; private ny: number; private nrm: Float64Array; private t = 0;
  constructor(w: World, private s: ClothSpec) {
    const { nx, ny } = s, W = nx + 1, N = W * (ny + 1); this.nx = nx; this.ny = ny; this.p = new Float64Array(N * 3); this.keep = new Uint8Array(N); this.nrm = new Float64Array(N * 3); const n = norm(cross(s.u, s.v));
    for (let j = 0; j <= ny; j++) for (let i = 0; i <= nx; i++) { const q = (j * W + i) * 3, x = this.local(i, j), off = 0.012 * Math.sin(i * 1.7 + j * 1.1) * (this.keep[j * W + i] = s.held(i, j) ? 1 : 0, s.held(i, j) ? 0 : 1); const a = add(add(s.at, mul(s.u, x[0] + s.width / 2)), add(mul(s.v, s.height / 2 - x[1]), mul(n, off))); this.p.set(a, q); }
    this.was = this.p.slice(); const ties: number[] = [], rest: number[] = [], k: number[] = [], tie = (a: number, b: number, kk: number) => { if (kk <= 0) return; ties.push(a, b); const [ai, aj, bi, bj] = [a % W, Math.floor(a / W), b % W, Math.floor(b / W)], x = this.local(ai, aj), y = this.local(bi, bj); rest.push(Math.hypot(x[0] - y[0], x[1] - y[1])); k.push(kk); };
    const bendy = s.stiff ?? 0.06; for (let j = 0; j <= ny; j++) for (let i = 0; i <= nx; i++) { const a = j * W + i; if (i < nx) tie(a, a + 1, 1); if (j < ny) tie(a, a + W, 1); if (i < nx && j < ny) { tie(a, a + W + 1, 0.5); tie(a + 1, a + W, 0.5); } if (i + 2 <= nx) tie(a, a + 2, bendy); if (j + 2 <= ny) tie(a, a + 2 * W, bendy); }
    this.ties = Int32Array.from(ties); this.rest = Float64Array.from(rest); this.k = Float64Array.from(k);
    for (let j = 0; j < ny; j++) for (let i = 0; i < nx; i++) {
      const c0 = this.local(i, j), c1 = this.local(i + 1, j + 1), d0 = this.local(i + 1, j), d1 = this.local(i, j + 1), cx = (c0[0] + c1[0] + d0[0] + d1[0]) / 4, cy = (c0[1] + c1[1]) / 2, marks: Mark[] = []; if (s.art) for (const face of s.back ? [5, 4] : [5]) marks.push({ face, at: [face === 5 ? -cx : cx, -cy], size: [s.width, s.height], art: s.art, color: [1, 1, 1] });
      this.cells.push(w.add({ at: s.at, half: [(Math.abs(d0[0] - c0[0]) + Math.abs(c1[0] - d1[0])) / 4 * 1.03 + 0.001, s.height / ny / 2 * 1.03 + 0.001, s.thick ?? 0.0012], mat: s.mat, tint: s.tint, flags: (s.flags ?? F.CUT) | F.MOVES | F.BEND, rig: 'cloth', marks, bend: [0, 0, 0, 0, 0, 0, 0, 0], cell: [cx, cy] }));
    }
    this.pose();
  }
  /** in its own plane about its middle, x along the width and y up */
  private local(i: number, j: number): [number, number] { const s = this.s, t = j / s.ny, k = 1 - (s.taper ?? 0) * t * 0.98; return [(i / s.nx - 0.5) * s.width * k, (0.5 - t) * s.height]; }
  step(dt: number, t: number, air: Air) {
    const { nx, ny, p, was, keep, ties, rest, k, s } = this, W = nx + 1, N = W * (ny + 1), h = dt / SUB, weight = s.weight ?? 0.16, mid: V3 = [0, 0, 0]; this.t = t;
    for (let i = 0; i < N; i++) for (let a = 0; a < 3; a++) mid[a]! += p[i * 3 + a]! / N; const W0 = air.at(mid), speed = Math.hypot(...W0), acc = new Float64Array(N * 3), w0: V3 = speed > 1e-3 ? [W0[0] / speed, W0[1] / speed, W0[2] / speed] : [1, 0, 0];
    for (let sub = 0; sub < SUB; sub++) {
      acc.fill(0); const tt = t + sub * h;
      for (let j = 0; j < ny; j++) for (let i = 0; i < nx; i++) {
        const A = (j * W + i) * 3, B = A + 3, C = A + W * 3, D = C + 3, ux = p[D]! - p[A]!, uy = p[D + 1]! - p[A + 1]!, uz = p[D + 2]! - p[A + 2]!, vx = p[C]! - p[B]!, vy = p[C + 1]! - p[B + 1]!, vz = p[C + 2]! - p[B + 2]!;
        // the cell's face (its length is twice its area), and the air over it: the wind, what rolls along the cloth with it, less the cloth's own motion
        let X = uy * vz - uz * vy, Y = uz * vx - ux * vz, Z = ux * vy - uy * vx; const l = Math.hypot(X, Y, Z) || 1e-9, area = l / 2; X /= l; Y /= l; Z /= l; const mx = (p[A]! + p[D]!) / 2, my = (p[A + 1]! + p[D + 1]!) / 2, mz = (p[A + 2]! + p[D + 2]!) / 2, along = (mx - s.at[0]) * w0[0] + (mz - s.at[2]) * w0[2], roll = speed * (0.22 * Math.sin(along * 7.5 - tt * (4 + 2.6 * speed) + my * 2.1) + 0.14 * Math.sin(along * 13.0 - my * 5.3 - tt * (7 + 3.1 * speed) + 1.0));
        let sx = 0, sy = 0, sz = 0; for (const q of [A, B, C, D]) { sx += p[q]! - was[q]!; sy += p[q + 1]! - was[q + 1]!; sz += p[q + 2]! - was[q + 2]!; } sx /= 4 * h; sy /= 4 * h; sz /= 4 * h; const rx = W0[0] - sx, ry = W0[1] - sy, rz = W0[2] - sz, n = rx * X + ry * Y + rz * Z + roll, f = 0.5 * RHO * 1.3 * area * n * Math.abs(n), drag = 0.5 * RHO * 0.05 * area * Math.hypot(rx, ry, rz), m = weight * Math.max(area, 1e-5);
        const ax = (f * X + drag * rx) / m / 4, ay = (f * Y + drag * ry) / m / 4, az = (f * Z + drag * rz) / m / 4; for (const q of [A, B, C, D]) { acc[q]! += ax; acc[q + 1]! += ay; acc[q + 2]! += az; }
      }
      for (let i = 0; i < N; i++) { const q = i * 3; if (keep[i]) continue; const col = i % W, rw = Math.floor(i / W), share = (col > 0 && col < nx ? 1 : 2) * (rw > 0 && rw < ny ? 1 : 2); for (let a = 0; a < 3; a++) { const x = p[q + a]!, g = Math.max(-260, Math.min(260, acc[q + a]! * share * (share > 1 ? 1 : 1))) - (a === 1 ? 9.81 : 0); p[q + a] = x + (x - was[q + a]!) * 0.9985 + g * h * h; was[q + a] = x; } }
      for (let pass = 0; pass < PASSES; pass++) {
        for (let e = 0; e < rest.length; e++) { const a = ties[e * 2]!, b = ties[e * 2 + 1]!, wa = keep[a] ? 0 : 1, wb = keep[b] ? 0 : 1; if (wa + wb === 0) continue; const A = a * 3, B = b * 3, dx = p[B]! - p[A]!, dy = p[B + 1]! - p[A + 1]!, dz = p[B + 2]! - p[A + 2]!, d = Math.sqrt(dx * dx + dy * dy + dz * dz) || 1e-9, c = k[e]! * (d - rest[e]!) / d / (wa + wb); p[A]! += dx * c * wa; p[A + 1]! += dy * c * wa; p[A + 2]! += dz * c * wa; p[B]! -= dx * c * wb; p[B + 1]! -= dy * c * wb; p[B + 2]! -= dz * c * wb; }
        if (s.wall) { const [n, d] = s.wall; for (let i = 0; i < N; i++) { if (keep[i]) continue; const q = i * 3, o = p[q]! * n[0] + p[q + 1]! * n[1] + p[q + 2]! * n[2] - d; if (o < 0) for (let a = 0; a < 3; a++) { p[q + a]! -= n[a]! * o; was[q + a]! += (p[q + a]! - was[q + a]!) * 0.3; } } }
      }
    }
    this.pose();
  }
  pose() {
    const { nx, ny, p, nrm, s } = this, W = nx + 1; nrm.fill(0);
    for (let j = 0; j < ny; j++) for (let i = 0; i < nx; i++) { const A = (j * W + i) * 3, B = A + 3, C = A + W * 3, D = C + 3, ux = p[B]! + p[D]! - p[A]! - p[C]!, uy = p[B + 1]! + p[D + 1]! - p[A + 1]! - p[C + 1]!, uz = p[B + 2]! + p[D + 2]! - p[A + 2]! - p[C + 2]!, vx = p[A]! + p[B]! - p[C]! - p[D]!, vy = p[A + 1]! + p[B + 1]! - p[C + 1]! - p[D + 1]!, vz = p[A + 2]! + p[B + 2]! - p[C + 2]! - p[D + 2]!, X = uy * vz - uz * vy, Y = uz * vx - ux * vz, Z = ux * vy - uy * vx; for (const q of [A, B, C, D]) { nrm[q]! += X; nrm[q + 1]! += Y; nrm[q + 2]! += Z; } }
    const P = (q: number): V3 => [p[q]!, p[q + 1]!, p[q + 2]!], thin = s.thick ?? 0.0012;
    this.cells.forEach((c, n) => {
      const i = n % nx, j = Math.floor(n / nx), A = (j * W + i) * 3, B = A + 3, C = A + W * 3, D = C + 3, a = P(A), b = P(B), e = P(C), d = P(D), u = sub(add(b, d), add(a, e)), v = sub(add(a, b), add(e, d)), x = norm(u), z = norm(cross(u, v)), y = cross(z, x), rot: Rot = [x, y, z];
      c.at = mul(add(add(a, b), add(e, d)), 0.25); c.rot = rot; void thin; const out = c.bend!; [C, D, A, B].forEach((q, m) => { const nx2 = nrm[q]!, ny2 = nrm[q + 1]!, nz2 = nrm[q + 2]!, zz = Math.max(nx2 * z[0] + ny2 * z[1] + nz2 * z[2], 1e-9); out[m * 2] = Math.max(-1.5, Math.min(1.5, (nx2 * x[0] + ny2 * x[1] + nz2 * x[2]) / zz)); out[m * 2 + 1] = Math.max(-1.5, Math.min(1.5, (nx2 * y[0] + ny2 * y[1] + nz2 * y[2]) / zz)); });
    });
  }
}
