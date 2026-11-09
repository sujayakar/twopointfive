// FOLIAGE AS A MEDIUM: one grid for a whole scene, filled from its leaves. A cell knows how thick they stand in each of its eighths and their mean color, and where it is thick it has a
// slot in which the light there is kept. Rays that are blurred anyway step through it as through smoke; leaves that are seen read their light from it. An empty cell says how far off the
// nearest that is not is, so that rays go past in strides.
import { type V3, add, apply, mul } from '../math';
import type { World } from './world';
export const FOL = 0.4;
export interface Foliage { lo: V3; n: [number, number, number]; /** four words a cell: eight densities · slot + 1, or (top bit) how many cells to the next · color and kind */ cells: Uint32Array<ArrayBuffer>; /** a place among the leaves of each slot */ at: Float32Array<ArrayBuffer>; slots: number }
export const SLOT_ROWS = 3;
export function growFoliage(w: World): Foliage {
  if (!w.instances.length) return { lo: [0, -90, 0], n: [1, 1, 1], cells: new Uint32Array([0, 0, 0x80000001, 0]), at: new Float32Array(4), slots: 1 };
  const lo: V3 = [1e9, 1e9, 1e9], hi: V3 = [-1e9, -1e9, -1e9]; for (const i of w.instances) { const e = i.scale * (w.clusters[i.cluster]!.kind === 1 ? 2.1 : 1.22); for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, i.at[k]! - e); hi[k] = Math.max(hi[k]!, i.at[k]! + e); } }
  const n = [0, 1, 2].map(k => Math.ceil((hi[k]! - lo[k]! + 2 * FOL) / FOL)) as [number, number, number], N = n[0] * n[1] * n[2], S = FOL / 2, o: V3 = [lo[0] - FOL, lo[1] - FOL, lo[2] - FOL]; const area = new Float32Array(N * 8), col = new Float32Array(N * 4), mid = new Float32Array(N * 4), grass = new Float32Array(N);
  for (const i of w.instances) { const c = w.clusters[i.cluster]!, tint = i.tint; for (const l of c.leaves) { if (l.wood) continue; const p = add(i.at, apply(i.rot, mul(l.at, i.scale))), a = l.size * l.size * 4 * (c.kind === 1 ? 0.06 : 0.46) * (2 / 3) * i.scale * i.scale, g = [0, 1, 2].map(k => Math.floor((p[k]! - o[k]!) / S)), q = g.map((v, k) => Math.max(0, Math.min(n[k]! - 1, v >> 1))), cell = q[0]! + n[0] * (q[1]! + n[1] * q[2]!), sub = (g[0]! & 1) + 2 * (g[1]! & 1) + 4 * (g[2]! & 1); area[cell * 8 + sub]! += a; for (let k = 0; k < 3; k++) { col[cell * 4 + k]! += tint[k]! * a; mid[cell * 4 + k]! += p[k]! * a; } col[cell * 4 + 3]! += a; if (c.kind === 1) grass[cell]! += a; } }
  const cells = new Uint32Array(N * 4), at: number[] = [], thick = (a: number) => Math.min(255, Math.round(0.5 * 0.75 * a / (S * S * S) * 8)); let slots = 0;
  for (let i = 0; i < N; i++) { const a = col[i * 4 + 3]!; if (a < 0.0015) continue; let lo4 = 0, hi4 = 0; for (let s = 0; s < 4; s++) { lo4 |= thick(area[i * 8 + s]!) << (8 * s); hi4 |= thick(area[i * 8 + 4 + s]!) << (8 * s); } if (!lo4 && !hi4) continue; const k = (v: number) => Math.max(0, Math.min(255, Math.round(v / a * 255))); cells[i * 4] = lo4 >>> 0; cells[i * 4 + 1] = hi4 >>> 0; cells[i * 4 + 2] = ++slots; cells[i * 4 + 3] = (k(col[i * 4]!) | (k(col[i * 4 + 1]!) << 8) | (k(col[i * 4 + 2]!) << 16) | (grass[i]! > a / 2 ? 1 << 24 : 0)) >>> 0; at.push(mid[i * 4]! / a, mid[i * 4 + 1]! / a, mid[i * 4 + 2]! / a, 0); }
  // how far to the next that is not empty, in cells (the larger of x, y, z), by sweeps
  const far = new Uint8Array(N); for (let i = 0; i < N; i++) far[i] = cells[i * 4 + 2] ? 0 : 255; for (let pass = 0; pass < 2; pass++) { const f = pass === 0, span = (m: number) => f ? [0, m, 1] : [m - 1, -1, -1]; const [z0, z1, dz] = span(n[2]!) as [number, number, number], [y0, y1, dy] = span(n[1]!) as [number, number, number], [x0, x1, dx] = span(n[0]!) as [number, number, number]; for (let z = z0; z !== z1; z += dz) for (let y = y0; y !== y1; y += dy) for (let x = x0; x !== x1; x += dx) { const i = x + n[0] * (y + n[1] * z); if (far[i] === 0) continue; let b = far[i]!; for (let a = -1; a <= 1; a++) for (let c = -1; c <= 1; c++) for (let e = -1; e <= 1; e++) { const X = x + e, Y = y + c, Z = z + a; if (X < 0 || Y < 0 || Z < 0 || X >= n[0] || Y >= n[1] || Z >= n[2]) continue; const v = far[X + n[0] * (Y + n[1] * Z)]! + 1; if (v < b) b = v; } far[i] = b; } }
  for (let i = 0; i < N; i++) if (!cells[i * 4 + 2]) cells[i * 4 + 2] = (0x80000000 | Math.max(1, Math.min(64, far[i]!))) >>> 0;
  return { lo: o, n, cells, at: Float32Array.from(at.length ? at : [0, 0, 0, 0]), slots: Math.max(1, slots) };
}
export const foliageCode = (f: Foliage, cellRow: number, slotRow: number) => `
const FOL_LO = vec3f(${f.lo.map(v => v.toFixed(4)).join(', ')}); const FOL_N = vec3i(${f.n.join(', ')}); const FOL_C = ${FOL}; const FOL_CELLS = ${cellRow}u; const FOL_SLOTS = ${slotRow}u; const FOL_MANY = ${f.slots}u;
fn folCell(c: vec3i) -> vec4u { return bitcast<vec4u>(extra[FOL_CELLS + u32(c.x + FOL_N.x * (c.y + FOL_N.y * c.z))]); }
fn folThick(r: vec4u, l: vec3f) -> f32 { let s = u32(dot(step(vec3f(0.5), l), vec3f(1.0, 2.0, 4.0))); return f32((select(r.x, r.y, s > 3u) >> (8u * (s & 3u))) & 255u) * 0.125; }
/** what leaves of this cell send back off one side, and through (grass: more) */
fn folColors(r: vec4u) -> array<vec3f, 2> { let t = vec3f(f32(r.w & 255u), f32((r.w >> 8u) & 255u), f32((r.w >> 16u) & 255u)) / 255.0; var a = t * t * 1.15; var h = leafThrough(t * t, ((r.w >> 24u) & 1u) != 0u); let k = leafKeeps(a + h); return array<vec3f, 2>(a * k, h * k); }
struct Fol { L: vec3f, T: vec3f };
/** a ray through the foliage to t1: what it gathers, and what gets through. \`lit\`: the light is asked for too */
fn folWalk(ro: vec3f, rd: vec3f, t1: f32, lit: bool) -> Fol {
  var o: Fol; o.T = vec3f(1.0); if (FOL_MANY < 2u) { return o; } let inv = safeInv(rd); let hi = FOL_LO + vec3f(FOL_N) * FOL_C; let a = (FOL_LO - ro) * inv; let b = (hi - ro) * inv; var t = max(max3(min(a, b)), 0.0) + 1e-3; let te = min(min3(max(a, b)), t1) - 1e-3; let big = max3(abs(rd)); var T = 1.0; var tau = 0.0;
  for (var i = 0u; i < 96u; i++) {
    if (t >= te || T < 0.015) { break; } let x = ro + rd * t; let g = (x - FOL_LO) / FOL_C; let c = clamp(vec3i(floor(g)), vec3i(0), FOL_N - 1); let r = folCell(c); let e = (FOL_LO + (vec3f(c) + select(vec3f(0.0), vec3f(1.0), rd > vec3f(0.0))) * FOL_C - ro) * inv; let out = max(min3(e), t) + 2e-3;
    if ((r.z & 0x80000000u) != 0u) { t = max(out, t + f32((r.z & 255u) - 1u) * FOL_C / big); continue; }
    let z = min(out, te); let l = z - t; let f0 = g - vec3f(c); let step2 = rd * (l / FOL_C); let s = 0.5 * l * (folThick(r, f0 + step2 * 0.25) + folThick(r, f0 + step2 * 0.75)) * frame.misc2.z; if (s > 0.0) { let k = exp(-s); if (lit) { o.L += folSent(r, rd) * (T * (1.0 - k)); } T *= k; tau += s; } t = z + 1e-4;
  }
  o.T = exp(-vec3f(1.0, 0.86, 1.1) * tau); return o;
}
fn folSlot(s: u32, k: u32) -> vec4u { return bitcast<vec4u>(extra[FOL_SLOTS + (s - 1u) * ${SLOT_ROWS}u + k]); }
/** what a cell's leaves send along −rd */
fn folSent(r: vec4u, rd: vec3f) -> vec3f { let q = folSlot(r.z, 0u); let J = vec3f(unpack2x16float(q.x), unpack2x16float(q.y).x); let sun = unpack2x16float(q.y).y; if (sun <= 0.0 || max3(frame.sunE.xyz) <= 0.0) { return J; } let c = folColors(r); var s: Surf; s.albedo = c[0]; s.through = c[1]; return J + leafPhase(s, rd) * frame.sunE.xyz * sun; }
/** the light (all but the sun and the lamps) that lands on a leaf at x from the side n looks to; w = is there any kept. The six ways of a cell, mixed over the eight cells round x. open: how much of that side its own twig leaves open; the rest sees that twig's leaves */
fn folLight(x: vec3f, n: vec3f, open: f32) -> vec4f {
  if (FOL_MANY < 2u) { return vec4f(0.0); } let g = (x - FOL_LO) / FOL_C - 0.5; let b = vec3i(floor(g)); let f = g - floor(g); var E = vec3f(0.0); var G = vec3f(0.0); var ws = 0.0; let n2 = n * n;
  for (var k = 0; k < 8; k++) { let d = vec3i(k & 1, (k >> 1) & 1, k >> 2); let c = b + d; if (any(c < vec3i(0)) || any(c >= FOL_N)) { continue; } let r = folCell(c); if ((r.z & 0x80000000u) != 0u) { continue; } let w3 = select(1.0 - f, f, d == vec3i(1)); let w = w3.x * w3.y * w3.z + 1e-3;
    let q0 = folSlot(r.z, 0u); let q1 = folSlot(r.z, 1u); let q2 = folSlot(r.z, 2u); if (f32(q2.w) < 1.0) { continue; } let J = vec3f(unpack2x16float(q0.x), unpack2x16float(q0.y).x); let sv = unpack2x16float(q0.y).y; let cc = folColors(r); G += (J * PI + (cc[0] + cc[1]) * frame.sunE.xyz * (0.25 * sv)) * w;
    let px = vec3f(unpack2x16float(q0.z), unpack2x16float(q0.w).x); let nx = vec3f(unpack2x16float(q0.w).y, unpack2x16float(q1.x)); let py = vec3f(unpack2x16float(q1.y), unpack2x16float(q1.z).x); let ny = vec3f(unpack2x16float(q1.z).y, unpack2x16float(q1.w)); let pz = vec3f(unpack2x16float(q2.x), unpack2x16float(q2.y).x); let nz = vec3f(unpack2x16float(q2.y).y, unpack2x16float(q2.z));
    E += (select(nx, px, n.x > 0.0) * n2.x + select(ny, py, n.y > 0.0) * n2.y + select(nz, pz, n.z > 0.0) * n2.z) * w; ws += w; }
  if (ws <= 0.0) { return vec4f(0.0); } return vec4f(E / ws * open + G / ws * (1.0 - open), 1.0);
}
fn folHas(x: vec3f) -> bool { if (FOL_MANY < 2u) { return false; } let g = (x - FOL_LO) / FOL_C - 0.5; let b = vec3i(floor(g)); for (var k = 0; k < 8; k++) { let c = b + vec3i(k & 1, (k >> 1) & 1, k >> 2); if (any(c < vec3i(0)) || any(c >= FOL_N)) { continue; } let r = folCell(c); if ((r.z & 0x80000000u) == 0u) { return true; } } return false; }
`;
