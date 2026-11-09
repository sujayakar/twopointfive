// The pool's waves, read from the shader that draws them: one list, so that what floats rides what is seen.
import text from '../wgsl/waves.wgsl' with { type: 'text' };
const TILE = Number(/WAVE_TILE = ([\d.]+)/.exec(text)![1]), W = [...text.matchAll(/vec4f\(([^)]*)\)/g)].slice(0, 7).map(m => m[1]!.split(',').map(Number)).map(([a, b, h, p]) => { const kx = a! * 2 * Math.PI / TILE, kz = b! * 2 * Math.PI / TILE; return [kx, kz, h!, p!, Math.sqrt(9.81 * Math.hypot(kx, kz)) * 0.62] as const; });
/** height over the mean, its slopes, and how fast it rises */
export function wave(x: number, z: number, t: number, out: Float64Array) { let h = 0, sx = 0, sz = 0, up = 0; for (const [kx, kz, a, p, w] of W) { const q = kx * x + kz * z - t * w + p, c = a * Math.cos(q); h += a * Math.sin(q); sx += kx * c; sz += kz * c; up -= w * c; } out[0] = h; out[1] = sx; out[2] = sz; out[3] = up; }
/** WHAT STIRS THE WATER: how far it stands off the wind's waves, on a grid of hand's breadths. What goes in lifts it and what comes out lets it fall by as much as it displaced; from there it runs as
 *  shallow water does, off the walls and back, and dies away. Floating things read it, and so does every ray. */
export class Ripples {
  readonly n: [number, number]; readonly dx: number; readonly lo: [number, number]; readonly h: Float32Array<ArrayBuffer>; private v: Float32Array; private d: Float32Array; private was: Float32Array; live = false; version = 0; private quiet = 0; private wet = false; private first = true; readonly any: boolean;
  constructor(readonly rect: readonly [number, number, number, number], readonly top: number) {
    const w = rect[2] - rect[0], l = rect[3] - rect[1]; this.any = w > 0.5 && l > 0.5; this.dx = this.any ? Math.max(0.06, w / 96, l / 96) : 1; this.n = this.any ? [Math.ceil(w / this.dx / 4) * 4, Math.ceil(l / this.dx)] : [4, 1]; this.lo = [rect[0], rect[1]]; const N = this.n[0] * this.n[1]; this.h = new Float32Array(N); this.v = new Float32Array(N); this.d = new Float32Array(N); this.was = new Float32Array(N);
  }
  /** something is in it here, this tick: so much of it (m³), so wide */
  sink(x: number, z: number, volume: number, r: number) {
    if (!this.any || volume <= 0) return; const { dx, n: [nx, nz], lo } = this, gx = (x - lo[0]) / dx - 0.5, gz = (z - lo[1]) / dx - 0.5, k = Math.max(1.2, r / dx), x0 = Math.max(0, Math.ceil(gx - k)), x1 = Math.min(nx - 1, Math.floor(gx + k)), z0 = Math.max(0, Math.ceil(gz - k)), z1 = Math.min(nz - 1, Math.floor(gz + k)); let s = 0;
    for (let j = z0; j <= z1; j++) for (let i = x0; i <= x1; i++) { const q = 1 - ((i - gx) ** 2 + (j - gz) ** 2) / (k * k); if (q > 0) s += q * q; } if (s <= 0) return; const a = volume / (s * dx * dx); this.wet = true;
    for (let j = z0; j <= z1; j++) for (let i = x0; i <= x1; i++) { const q = 1 - ((i - gx) ** 2 + (j - gz) ** 2) / (k * k); if (q > 0) this.d[i + nx * j]! += a * q * q; }
  }
  step(dt: number) {
    if (!this.live && !this.wet) return; const { h, v, d, was, n: [nx, nz], dx } = this, c = Math.min(2.2, 0.6 * dx / dt), k = c * c / (dx * dx) * dt, keep = Math.exp(-0.55 * dt), smooth = 0.02; let most = 0, sum = 0;
    if (!this.live) { let new_ = 0; for (let i = 0; i < d.length; i++) new_ = Math.max(new_, Math.abs(d[i]! - was[i]!)); if (this.first || new_ < 4e-4) { this.first = false; was.set(d); d.fill(0); this.wet = false; return; } this.live = true; this.quiet = 0; }   // (what only lies in it stirs nothing)
    this.wet = false; for (let i = 0; i < h.length; i++) { h[i]! += 0.6 * (d[i]! - was[i]!); was[i] = d[i]!; d[i] = 0; }
    for (let z = 0; z < nz; z++) for (let x = 0; x < nx; x++) { const i = x + nx * z, m = h[i]!, l = (x > 0 ? h[i - 1]! : m) + (x < nx - 1 ? h[i + 1]! : m) + (z > 0 ? h[i - nx]! : m) + (z < nz - 1 ? h[i + nx]! : m) - 4 * m, lv = (x > 0 ? v[i - 1]! : v[i]!) + (x < nx - 1 ? v[i + 1]! : v[i]!) + (z > 0 ? v[i - nx]! : v[i]!) + (z < nz - 1 ? v[i + nx]! : v[i]!) - 4 * v[i]!; v[i] = (v[i]! + k * l + smooth * lv) * keep; }
    for (let i = 0; i < h.length; i++) { const q = h[i]! + v[i]! * dt; h[i] = q > 0.11 ? 0.11 : q < -0.11 ? -0.11 : q; sum += h[i]!; const a = Math.abs(q); if (a > most) most = a; } sum /= h.length; for (let i = 0; i < h.length; i++) h[i]! -= sum;
    this.version++; this.quiet = most < 3e-4 ? this.quiet + 1 : 0; if (this.quiet > 60) { this.live = false; h.fill(0); v.fill(0); }
  }
  /** added to height, slopes and how fast it rises */
  at(x: number, z: number, out: Float64Array) {
    if (!this.live) return; const { n: [nx, nz], dx, lo, h, v } = this, gx = Math.min(nx - 1.001, Math.max(0, (x - lo[0]) / dx - 0.5)), gz = Math.min(nz - 1.001, Math.max(0, (z - lo[1]) / dx - 0.5)), i = gx | 0, j = gz | 0, fx = gx - i, fz = gz - j, o = i + nx * j, a = h[o]!, b = h[o + 1]!, c = h[o + nx]!, e = h[o + nx + 1]!;
    out[0]! += (a * (1 - fx) + b * fx) * (1 - fz) + (c * (1 - fx) + e * fx) * fz; out[1]! += ((b - a) * (1 - fz) + (e - c) * fz) / dx; out[2]! += ((c - a) * (1 - fx) + (e - b) * fx) / dx; out[3]! += v[o]!;
  }
}
export const rippleRows = (r: Ripples) => r.h.length / 4;
/** `cells` = how a shader reads one cell, i; `on` = is there anything */
export const rippleCode = (r: Ripples, cell: string, on: string) => `
fn ripAt(c: vec2i) -> f32 { let q = clamp(c, vec2i(0), vec2i(${r.n[0] - 1}, ${r.n[1] - 1})); let i = u32(q.x + ${r.n[0]} * q.y); return ${cell}; }
fn rippleHeight(p: vec2f) -> f32 { if (${on} < 0.5) { return 0.0; } let g = (p - vec2f(${r.lo[0].toFixed(3)}, ${r.lo[1].toFixed(3)})) / ${r.dx.toFixed(5)} - 0.5; let b = vec2i(floor(g)); let f = g - floor(g); return mix(mix(ripAt(b), ripAt(b + vec2i(1, 0)), f.x), mix(ripAt(b + vec2i(0, 1)), ripAt(b + vec2i(1, 1)), f.x), f.y); }
fn rippleSlope(p: vec2f) -> vec2f { if (${on} < 0.5) { return vec2f(0.0); } let e = ${r.dx.toFixed(5)}; return vec2f(rippleHeight(p + vec2f(e, 0.0)) - rippleHeight(p - vec2f(e, 0.0)), rippleHeight(p + vec2f(0.0, e)) - rippleHeight(p - vec2f(0.0, e))) / (2.0 * e); }
`;
