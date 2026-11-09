// THE AIR, at two sizes. Here the coarse one: cells of about a metre, everywhere, solved on the CPU once a tick, so that whatever moves by it is the same on every machine
// and a run repeats. The fine one is the smoke's bricks, which begin as this, border on this, and are drawn to it as a whole.
//   The weather gives the free wind: at the rim, along the top, and slowly (over four seconds) to what lies open to the sky, as the wind above mixes down. Gusts come in upwind and go through. Between two cells is an opening, measured once: walls and panes shut it, a
// door's leaf shuts it while it is shut, leaves hold the air back. Air is not squeezed: what flows into a cell flows out. So lee, draughts and still rooms are found, not told.
import { type V3, sub, dot } from '../math';
import { F, boundsOf, type Piece, type World } from './world';
import type { Place } from './place';
/** the mean wind at strength 1, m/s */
export const BREEZE = 2.6;
/** something that moves the air: within r of at it takes on this speed, as fast as `grip` (1/s) */
export interface AirPush { at: V3; r: number; v: V3; grip: number }
const H: V3 = [1, 0.85, 1];
export class Air {
  way: V3 = [1, 0, 0]; strength = 1; gone = 0; readonly n: [number, number, number]; readonly o: V3; readonly cells: number; readonly pushes: AirPush[] = []; readonly packed: Uint32Array<ArrayBuffer>; version = 0;
  // (a layer of cells that are not there lies round the rest, so that no step asks where the edge is)
  private sy: number; private sz: number; private list: Int32Array; private red: Int32Array; private black: Int32Array;
  /** speed through each cell's near face of x, y, z; how open it is, to air and to pressure; a cell's own speed; pressure */
  private f: Float32Array[]; private open: Float32Array[]; private gate: Float32Array[]; private c: Float32Array[]; private was: Float32Array[]; private p: Float32Array; private div: Float32Array; private over: Float32Array; private drag: Float32Array; private held: Float32Array; private sky: Float32Array; private base: Uint8Array[]; private doorFaces: { get: () => number; faces: [number, number, number][] }[] = []; private odd = false; private doorKey = '-';
  constructor(place: Place, w: World, doors: { leaf: Piece[]; open: number }[]) {
    const b = place.air ?? [-24, 0, -20, 24, 11.05, 20]; this.o = [b[0], b[1], b[2]]; this.n = [0, 1, 2].map(k => Math.round((b[k + 3]! - b[k]!) / H[k]!)) as [number, number, number]; const [nx, ny, nz] = this.n; this.cells = nx * ny * nz; this.sy = nx + 2; this.sz = (nx + 2) * (ny + 2); const N = this.sz * (nz + 2), arr = () => new Float32Array(N), tri = () => [arr(), arr(), arr()];
    this.f = tri(); this.open = tri(); this.gate = tri(); this.c = tri(); this.was = tri(); this.p = arr(); this.div = arr(); this.over = arr(); this.drag = arr(); this.held = arr(); this.sky = arr(); this.base = [new Uint8Array(N), new Uint8Array(N), new Uint8Array(N)]; this.packed = new Uint32Array(this.cells * 2);
    const list: number[] = [], red: number[] = [], black: number[] = []; for (let z = 0; z < nz; z++) for (let y = 0; y < ny; y++) for (let x = 0; x < nx; x++) { const i = this.ix(x, y, z); list.push(i); ((x + y + z) & 1 ? black : red).push(i); } this.list = Int32Array.from(list); this.red = Int32Array.from(red); this.black = Int32Array.from(black); this.measure(w, doors); this.reset();
  }
  private ix(x: number, y: number, z: number) { return x + 1 + (y + 1) * this.sy + (z + 1) * this.sz; }
  /** what stands between neighbouring cells: five ways across each face, each shut by what it meets */
  private measure(w: World, doors: { leaf: Piece[]; open: number }[]) {
    const OFF = [[0, 0], [-0.3, -0.3], [0.3, -0.3], [-0.3, 0.3], [0.3, 0.3]];
    const cover = (p: Piece, a: number, to: (i: number, bit: number) => void) => {
      const b = boundsOf(p), lo = [0, 1, 2].map(k => Math.max(0, Math.floor((b.mn[k]! - this.o[k]!) / H[k]! - (k === a ? 0.5 : 0)))), hi = [0, 1, 2].map(k => Math.min(this.n[k]! - 1, Math.floor((b.mx[k]! - this.o[k]!) / H[k]! + (k === a ? 0.5 : 0)))), u = (a + 1) % 3, v = (a + 2) % 3;
      for (let z = lo[2]!; z <= hi[2]!; z++) for (let y = lo[1]!; y <= hi[1]!; y++) for (let x = lo[0]!; x <= hi[0]!; x++) { const g = [x, y, z]; if (g[a] === 0) continue; OFF.forEach(([s, t], bit) => {
        const from: V3 = [0, 0, 0]; for (let k = 0; k < 3; k++) from[k] = this.o[k]! + (g[k]! + 0.5) * H[k]!; from[u]! += s! * H[u]!; from[v]! += t! * H[v]!; from[a]! -= H[a]!; const q = sub(from, p.at); let l0 = 0, l1 = H[a]!; for (let k = 0; k < 3; k++) { const e = dot(p.rot[k]!, q), d = p.rot[k]![a]!, h = p.half[k]! * (p.quad && p.quad[k] !== 0 ? 0.8 : 1); if (Math.abs(d) < 1e-9) { if (Math.abs(e) > h) return; continue; } const m = (-h - e) / d, n = (h - e) / d; l0 = Math.max(l0, Math.min(m, n)); l1 = Math.min(l1, Math.max(m, n)); } if (l0 <= l1) to(this.ix(x, y, z), 1 << bit); }); }
    };
    for (const p of w.pieces) { if (p.flags & (F.WATER | F.FAR | F.OPEN) || Math.max(...p.half) < 0.22) continue; for (let a = 0; a < 3; a++) cover(p, a, (i, bit) => { this.base[a]![i]! |= bit; }); }
    for (const d of doors) { const faces = new Map<string, [number, number, number]>(); for (let a = 0; a < 3; a++) cover(d.leaf[0]!, a, (i, bit) => { if (this.base[a]![i]! & bit) return; const k = a + ':' + i, f = faces.get(k); if (f) f[2]++; else faces.set(k, [a, i, 1]); }); this.doorFaces.push({ get: () => d.open, faces: [...faces.values()] }); }
    // leaves hold it back, by their area in a cubic metre
    for (const i of w.instances) { const c = w.clusters[i.cluster]!; let area = 0; for (const l of c.leaves) area += l.size * l.size * (l.wood ? 0.05 : 0.9); const g = [0, 1, 2].map(k => Math.floor((i.at[k]! - this.o[k]!) / H[k]!)); if (g.some((q, k) => q < 0 || q >= this.n[k]!)) continue; this.drag[this.ix(g[0]!, g[1]!, g[2]!)]! += 0.2 * area * i.scale * i.scale / (H[0] * H[1] * H[2]); }
  }
  /** how open each face is now */
  private gates() {
    const key = this.doorFaces.map(d => d.get().toFixed(2)).join(); if (key === this.doorKey) return; this.doorKey = key; const bits = (m: number) => (m & 1) + ((m >> 1) & 1) + ((m >> 2) & 1) + ((m >> 3) & 1) + ((m >> 4) & 1), [nx, ny, nz] = this.n, S = [1, this.sy, this.sz];
    for (let a = 0; a < 3; a++) { const o = this.open[a]!, b = this.base[a]!; for (let i = 0; i < o.length; i++) o[i] = 1 - bits(b[i]!) / 5; } for (const d of this.doorFaces) { const shut = 1 - Math.min(1, d.get() * 1.4); for (const [a, i, n] of d.faces) this.open[a]![i] = Math.max(0, this.open[a]![i]! - n / 5 * shut); }
    // the rim's sides are told their speed (no pressure there); over the top it comes and goes; the ground is shut
    for (let a = 0; a < 3; a++) this.gate[a]!.fill(0); for (let z = 0; z < nz; z++) for (let y = 0; y < ny; y++) for (let x = 0; x < nx; x++) { const i = this.ix(x, y, z); if (y === 0) this.open[1]![i] = 0; if (x > 0) this.gate[0]![i] = this.open[0]![i]! / H[0] ** 2; if (y > 0) this.gate[1]![i] = this.open[1]![i]! / H[1] ** 2; if (z > 0) this.gate[2]![i] = this.open[2]![i]! / H[2] ** 2; if (y === ny - 1) { this.open[1]![i + this.sy] = 1; this.gate[1]![i + this.sy] = 1 / H[1] ** 2; } if (x === nx - 1) this.open[0]![i + 1] = 1; if (z === nz - 1) this.open[2]![i + this.sz] = 1; if (x === 0) this.open[0]![i] = 1; if (z === 0) this.open[2]![i] = 1; }
    for (let z = 0; z < nz; z++) for (let x = 0; x < nx; x++) { let s = 1; for (let y = ny - 1; y >= 0; y--) { const i = this.ix(x, y, z); if (y < ny - 1) s *= this.open[1]![i + this.sy]!; this.sky[i] = s * (y >= ny - 2 ? 2 : 0.25); } }
    for (const i of this.list) { let d = 0, wall = 0; for (let a = 0; a < 3; a++) { d += this.gate[a]![i]! + this.gate[a]![i + S[a]!]!; wall += 2 - this.open[a]![i]! - this.open[a]![i + S[a]!]!; } this.over[i] = d > 0.02 ? 1.6 / d : 0; this.held[i] = this.drag[i]! + 0.02 * wall; }
  }
  /** the weather: the wind where nothing stands, m/s, at height h. `gone` = how far it has gone, so that turning it up does not jump */
  private free(x: number, z: number): [number, number] { const w = this.way, a = x * w[0] + z * w[2] - this.gone, c = z * w[0] - x * w[2], g = Math.max(0.1, 0.85 + 0.42 * Math.sin(a * 0.31 + 1.3 * Math.sin(c * 0.17)) + 0.25 * Math.sin(a * 0.83 + c * 0.35 + 2.1) + 0.14 * Math.sin(a * 1.9 - c * 0.6 + 0.7)), side = (0.4 + 0.6 * g) * (0.22 * Math.sin(a * 0.9 + c * 0.7) + 0.14 * Math.sin(a * 1.7 - c * 1.3 + 2.0)), k = BREEZE * this.strength; return [k * (w[0] * g - w[2] * side), k * (w[2] * g + w[0] * side)]; }
  private low(h: number) { return 0.55 + 0.45 * Math.min(1, Math.max(0, h) / 5); }
  private rim() {
    const [nx, ny, nz] = this.n, [fx, , fz] = this.f; for (let z = 0; z < nz; z++) { const a = this.free(this.o[0], this.o[2] + z + 0.5)[0], b = this.free(this.o[0] + nx, this.o[2] + z + 0.5)[0]; for (let y = 0; y < ny; y++) { const k = this.low(this.o[1] + (y + 0.5) * H[1]); fx![this.ix(0, y, z)] = a * k; fx![this.ix(nx, y, z)] = b * k; } }
    for (let x = 0; x < nx; x++) { const a = this.free(this.o[0] + x + 0.5, this.o[2])[1], b = this.free(this.o[0] + x + 0.5, this.o[2] + nz)[1]; for (let y = 0; y < ny; y++) { const k = this.low(this.o[1] + (y + 0.5) * H[1]); fz![this.ix(x, y, 0)] = a * k; fz![this.ix(x, y, nz)] = b * k; } }
  }
  /** begin as the free wind would be, found to fit what stands */
  reset() { const [nx, ny, nz] = this.n; this.gates(); this.p.fill(0); for (let z = 0; z < nz; z++) for (let x = 0; x < nx; x++) { const v = this.free(this.o[0] + x + 0.5, this.o[2] + z + 0.5); for (let y = 0; y < ny; y++) { const i = this.ix(x, y, z), k = this.low(this.o[1] + (y + 0.5) * H[1]); this.f[0]![i] = v[0] * k; this.f[1]![i] = 0; this.f[2]![i] = v[1] * k; } } this.rim(); for (let k = 0; k < 6; k++) this.press(16); this.middles(); this.pack(); }
  blow(wind: V3, dt: number) {
    const s = Math.hypot(wind[0], wind[2]); if (s > 1e-6) this.way = [wind[0] / s, 0, wind[2] / s]; this.strength = s; this.gone += s * BREEZE * dt;
    // every second tick, by twice as much: it changes slowly
    this.odd = !this.odd; if (this.odd) return; dt *= 2; this.gates(); this.rim(); const { n: [nx, ny, nz], list, sy, sz } = this, [cx, cy, cz] = this.c as [Float32Array, Float32Array, Float32Array], [wx, wy, wz] = this.was as [Float32Array, Float32Array, Float32Array], [ox, oy, oz] = this.open as [Float32Array, Float32Array, Float32Array], held = this.held; wx.set(cx); wy.set(cy); wz.set(cz);
    // carried along: each cell takes the speed of where its air was, held back by leaves and by what it passes close to
    const kx = dt / H[0], ky = dt / H[1], kz = dt / H[2]; for (let z = 1, m = 0; z <= nz; z++) for (let y = 1; y <= ny; y++) for (let x = 1; x <= nx; x++, m++) {
      const i = list[m]!, vx = wx[i]!, vy = wy[i]!, vz = wz[i]!, sp = vx * vx + vy * vy + vz * vz; if (sp < 1e-5) continue; let dx = -vx * kx, dy = -vy * ky, dz = -vz * kz;
      // (not through what is shut, nor out of the rim)
      if (dx < 0 ? x === 1 || ox[i]! < 0.4 : x === nx || ox[i + 1]! < 0.4) dx = 0; if (dy < 0 ? y === 1 || oy[i]! < 0.4 : y === ny || oy[i + sy]! < 0.4) dy = 0; if (dz < 0 ? z === 1 || oz[i]! < 0.4 : z === nz || oz[i + sz]! < 0.4) dz = 0;
      const X = dx < 0 ? -1 : 1, Y = dy < 0 ? -sy : sy, Z = dz < 0 ? -sz : sz, a = Math.min(1, Math.abs(dx)), b = Math.min(1, Math.abs(dy)), e = Math.min(1, Math.abs(dz)), g000 = (1 - a) * (1 - b) * (1 - e), g100 = a * (1 - b) * (1 - e), g010 = (1 - a) * b * (1 - e), g110 = a * b * (1 - e), g001 = (1 - a) * (1 - b) * e, g101 = a * (1 - b) * e, g011 = (1 - a) * b * e, g111 = a * b * e, j = i + X, k = i + Y, l = j + Y, q = 1 / (1 + dt * Math.sqrt(sp) * held[i]!);
      cx[i] = q * (wx[i]! * g000 + wx[j]! * g100 + wx[k]! * g010 + wx[l]! * g110 + wx[i + Z]! * g001 + wx[j + Z]! * g101 + wx[k + Z]! * g011 + wx[l + Z]! * g111); cy[i] = q * (wy[i]! * g000 + wy[j]! * g100 + wy[k]! * g010 + wy[l]! * g110 + wy[i + Z]! * g001 + wy[j + Z]! * g101 + wy[k + Z]! * g011 + wy[l + Z]! * g111); cz[i] = q * (wz[i]! * g000 + wz[j]! * g100 + wz[k]! * g010 + wz[l]! * g110 + wz[i + Z]! * g001 + wz[j + Z]! * g101 + wz[k + Z]! * g011 + wz[l + Z]! * g111);
    }
    for (let z = 0; z < nz; z++) for (let x = 0; x < nx; x++) { const v = this.free(this.o[0] + x + 0.5, this.o[2] + z + 0.5); for (let y = 0; y < ny; y++) { const i = this.ix(x, y, z), q = this.sky[i]! * dt, k = this.low(this.o[1] + (y + 0.5) * H[1]); if (q > 0) { cx[i]! += (v[0] * k - cx[i]!) * q; cz[i]! += (v[1] * k - cz[i]!) * q; } } }
    for (const u of this.pushes) { const g = [0, 1, 2].map(k => (u.at[k]! - this.o[k]!) / H[k]! - 0.5), vol = Math.min(1, 4.19 * u.r ** 3 / (H[0] * H[1] * H[2])), b = g.map(Math.floor); for (let m = 0; m < 8; m++) { const x = b[0]! + (m & 1), y = b[1]! + ((m >> 1) & 1), z = b[2]! + (m >> 2); if (x < 0 || y < 0 || z < 0 || x >= nx || y >= ny || z >= nz) continue; const i = this.ix(x, y, z), k = (1 - Math.abs(g[0]! - x)) * (1 - Math.abs(g[1]! - y)) * (1 - Math.abs(g[2]! - z)) * vol * (1 - Math.exp(-dt * u.grip)); cx[i]! += (u.v[0] - cx[i]!) * k; cy[i]! += (u.v[1] - cy[i]!) * k; cz[i]! += (u.v[2] - cz[i]!) * k; } }
    // the faces take what changed
    for (let a = 0; a < 3; a++) { const f = this.f[a]!, g = this.gate[a]!, c = this.c[a]!, w = this.was[a]!, st = a === 0 ? 1 : a === 1 ? sy : sz; for (let m = 0; m < list.length; m++) { const i = list[m]!; if (g[i]! > 0) f[i]! += 0.5 * (c[i]! - w[i]! + c[i - st]! - w[i - st]!); else if (this.open[a]![i]! === 0) f[i] = 0; } }
    this.press(12); this.middles(); this.pack();
  }
  /** nothing is squeezed: pressure till what flows in flows out (over-relaxed, two colors, from the last answer) */
  private press(times: number) {
    const { list, p, div, over, sy, sz, n: [, ny] } = this, [fx, fy, fz] = this.f as [Float32Array, Float32Array, Float32Array], [ox, oy, oz] = this.open as [Float32Array, Float32Array, Float32Array], [gx, gy, gz] = this.gate as [Float32Array, Float32Array, Float32Array];
    for (let m = 0; m < list.length; m++) { const i = list[m]!; div[i] = (fx[i + 1]! * ox[i + 1]! - fx[i]! * ox[i]!) / H[0] + (fy[i + sy]! * oy[i + sy]! - fy[i]! * oy[i]!) / H[1] + (fz[i + sz]! * oz[i + sz]! - fz[i]! * oz[i]!) / H[2]; }
    for (let it = 0; it < times; it++) for (let h = 0; h < 2; h++) { const L = h ? this.black : this.red; for (let m = 0; m < L.length; m++) { const i = L[m]!, w = over[i]!; if (w === 0) continue; p[i] = p[i]! * -0.6 + w * (gx[i]! * p[i - 1]! + gx[i + 1]! * p[i + 1]! + gy[i]! * p[i - sy]! + gy[i + sy]! * p[i + sy]! + gz[i]! * p[i - sz]! + gz[i + sz]! * p[i + sz]! - div[i]!); } }
    for (let m = 0; m < list.length; m++) { const i = list[m]!, q = p[i]!; if (gx[i]! > 0) fx[i]! -= (q - p[i - 1]!) / H[0]; if (gy[i]! > 0) fy[i]! -= (q - p[i - sy]!) / H[1]; if (gz[i]! > 0) fz[i]! -= (q - p[i - sz]!) / H[2]; } const [nx, , nz] = this.n; for (let z = 0; z < nz; z++) for (let x = 0; x < nx; x++) { const i = this.ix(x, ny - 1, z); fy[i + sy]! += p[i]! / H[1]; }
    // (the answer is a good first guess for the next, which is nearly the same question: what is asked for is only what has changed)
    for (let m = 0; m < list.length; m++) p[list[m]!]! *= 0;
  }
  private middles() { const { list, sy, sz } = this, S = [1, sy, sz]; for (let a = 0; a < 3; a++) { const c = this.c[a]!, f = this.f[a]!, o = this.open[a]!, s = S[a]!; for (let m = 0; m < list.length; m++) { const i = list[m]!; c[i] = 0.5 * (f[i]! * o[i]! + f[i + s]! * o[i + s]!); } } }
  /** for the shaders: millimetres a second, and which of the far faces are open */
  private pack() { const { list, packed, sy, sz } = this, [cx, cy, cz] = this.c as [Float32Array, Float32Array, Float32Array], [ox, oy, oz] = this.open as [Float32Array, Float32Array, Float32Array]; for (let m = 0; m < list.length; m++) { const i = list[m]!; packed[m * 2] = ((cx[i]! * 1000) & 0xffff) | ((cy[i]! * 1000) << 16); packed[m * 2 + 1] = ((cz[i]! * 1000) & 0xffff) | ((ox[i + 1]! > 0.3 ? 0x10000 : 0) | (oy[i + sy]! > 0.3 ? 0x20000 : 0) | (oz[i + sz]! > 0.3 ? 0x40000 : 0)); } this.version++; }
  /** the air's speed at a place, m/s: between the eight cells round it, and not from the far side of what is shut */
  at(x: V3): V3 {
    const { n: [nx, ny, nz], sy, sz } = this, [cx, cy, cz] = this.c as [Float32Array, Float32Array, Float32Array], [ox, oy, oz] = this.open as [Float32Array, Float32Array, Float32Array], gx = Math.min(nx - 1.001, Math.max(0, (x[0] - this.o[0]) / H[0] - 0.5)), gy = Math.min(ny - 1.001, Math.max(0, (x[1] - this.o[1]) / H[1] - 0.5)), gz = Math.min(nz - 1.001, Math.max(0, (x[2] - this.o[2]) / H[2] - 0.5)), ix = gx | 0, iy = gy | 0, iz = gz | 0, fx = gx - ix, fy = gy - iy, fz = gz - iz, hx = fx > 0.5 ? 1 : 0, hy = fy > 0.5 ? 1 : 0, hz = fz > 0.5 ? 1 : 0, base = this.ix(ix, iy, iz), own = base + hx + hy * sy + hz * sz;
    const okx = ox[own + 1 - hx]! >= 0.3, oky = oy[own + sy - hy * sy]! >= 0.3, okz = oz[own + sz - hz * sz]! >= 0.3; let X = 0, Y = 0, Z = 0, W = 0;
    for (let m = 0; m < 8; m++) { const dx = m & 1, dy = (m >> 1) & 1, dz = m >> 2; if ((dx !== hx && !okx) || (dy !== hy && !oky) || (dz !== hz && !okz)) continue; const w = (dx ? fx : 1 - fx) * (dy ? fy : 1 - fy) * (dz ? fz : 1 - fz); if (w < 1e-4) continue; const i = base + dx + dy * sy + dz * sz; X += cx[i]! * w; Y += cy[i]! * w; Z += cz[i]! * w; W += w; }
    return W < 1e-4 ? [cx[own]!, cy[own]!, cz[own]!] : [X / W, Y / W, Z / W];
  }
}
export const airRows = (cells: number) => Math.ceil(cells / 2);
export const airCode = (a: Air) => `
fn airCell(c: vec3i) -> vec4f { let i = u32(c.x + ${a.n[0]} * (c.y + ${a.n[1]} * c.z)); let r = bitcast<vec4u>(extra[u32(frame.pool2.w) + (i >> 1u)]); let v = bitcast<vec2i>(select(r.xy, r.zw, (i & 1u) == 1u)); return vec4f(vec3f(vec3i((v.x << 16u) >> 16u, v.x >> 16u, (v.y << 16u) >> 16u)) * 0.001, f32(v.y >> 16u)); }
/** the air's speed at a place, m/s: the coarse air, not read from the far side of what is shut */
fn windAt(x: vec3f) -> vec3f {
  let g = clamp((x - vec3f(${a.o.map(v => v.toFixed(3)).join(', ')})) / vec3f(${H.map(v => v.toFixed(3)).join(', ')}) - 0.5, vec3f(0.0), vec3f(${a.n.map(v => (v - 1.001).toFixed(3)).join(', ')})); let b = vec3i(g); let f = g - vec3f(b); let h = vec3i(select(vec3f(0.0), vec3f(1.0), f > vec3f(0.5))); let own = airCell(b + h); var s = vec3f(0.0); var ws = 0.0;
  for (var m = 0; m < 8; m++) { let d = vec3i(m & 1, (m >> 1) & 1, m >> 2); let w3 = select(1.0 - f, f, d == vec3i(1)); let w = w3.x * w3.y * w3.z; if (w < 1e-3) { continue; } var ok = true; for (var k = 0u; k < 3u; k++) { if (d[k] != h[k]) { var lo = b + h; if (h[k] == 1) { lo[k] -= 1; } if ((u32(airCell(lo).w) & (1u << k)) == 0u) { ok = false; } } } if (!ok) { continue; } s += airCell(b + d).xyz * w; ws += w; }
  return select(own.xyz, s / max(ws, 1e-4), ws > 1e-3);
}`;
