// The pool's waves, read from the shader that draws them: one list, so that what floats rides what is seen.
import text from '../wgsl/waves.wgsl' with { type: 'text' };
const TILE = Number(/WAVE_TILE = ([\d.]+)/.exec(text)![1]), W = [...text.matchAll(/vec4f\(([^)]*)\)/g)].slice(0, 7).map(m => m[1]!.split(',').map(Number)).map(([a, b, h, p]) => { const kx = a! * 2 * Math.PI / TILE, kz = b! * 2 * Math.PI / TILE; return [kx, kz, h!, p!, Math.sqrt(9.81 * Math.hypot(kx, kz)) * 0.62] as const; });
/** height over the mean, its slopes, and how fast it rises */
export function wave(x: number, z: number, t: number, out: Float64Array) { let h = 0, sx = 0, sz = 0, up = 0; for (const [kx, kz, a, p, w] of W) { const q = kx * x + kz * z - t * w + p, c = a * Math.cos(q); h += a * Math.sin(q); sx += kx * c; sz += kz * c; up -= w * c; } out[0] = h; out[1] = sx; out[2] = sz; out[3] = up; }
