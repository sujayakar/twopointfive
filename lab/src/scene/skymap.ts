// WHAT THE SKY REACHES: how high the topmost thing stands over each spot, found once. Rain wets and snow lies where nothing is over a place. And what has been trodden: feet press the snow down.
import type { Solid } from './solid';
import { boundsOf, type Piece } from './world';
export const SKY = { lo: [-24, -20] as const, n: [300, 250] as const, c: 0.16 }, TROD = { lo: [-20, -15] as const, n: [800, 600] as const, c: 0.05 };
export const SKY_ROWS = Math.ceil(SKY.n[0] * SKY.n[1] / 4), TROD_ROWS = Math.ceil(TROD.n[0] * TROD.n[1] / 16);
export function skyMap(solid: Solid, moving: Piece[]): Float32Array<ArrayBuffer> {
  const [nx, nz] = SKY.n, h = new Float32Array(SKY_ROWS * 4).fill(-50); for (let z = 0; z < nz; z++) for (let x = 0; x < nx; x++) { const t = solid.ray([SKY.lo[0] + (x + 0.5) * SKY.c, 34, SKY.lo[1] + (z + 0.5) * SKY.c], [0, -1, 0], 40); h[x + nx * z] = 34 - t; }
  for (const p of moving) { if (p.rig !== 'cloth') continue; const b = boundsOf(p); for (let z = Math.max(0, Math.floor((b.mn[2] - SKY.lo[1]) / SKY.c)); z <= Math.min(nz - 1, Math.floor((b.mx[2] - SKY.lo[1]) / SKY.c)); z++) for (let x = Math.max(0, Math.floor((b.mn[0] - SKY.lo[0]) / SKY.c)); x <= Math.min(nx - 1, Math.floor((b.mx[0] - SKY.lo[0]) / SKY.c)); x++) h[x + nx * z] = Math.max(h[x + nx * z]!, p.at[1]); } return h;
}
/** a print: cells pressed, and the spans that changed (byte offset, bytes) */
export function tread(g: Uint8Array, x: number, z: number, yaw: number, deep: number): [number, number][] {
  const [nx, nz] = TROD.n, c = Math.cos(yaw), s = Math.sin(yaw), out: [number, number][] = [], i0 = Math.floor((x - 0.2 - TROD.lo[0]) / TROD.c), i1 = Math.floor((x + 0.2 - TROD.lo[0]) / TROD.c), k0 = Math.floor((z - 0.2 - TROD.lo[1]) / TROD.c), k1 = Math.floor((z + 0.2 - TROD.lo[1]) / TROD.c);
  for (let k = Math.max(0, k0); k <= Math.min(nz - 1, k1); k++) { let lo = 1e9, hi = -1; for (let i = Math.max(0, i0); i <= Math.min(nx - 1, i1); i++) { const dx = TROD.lo[0] + (i + 0.5) * TROD.c - x, dz = TROD.lo[1] + (k + 0.5) * TROD.c - z, u = dx * c - dz * s, v = dx * s + dz * c, d = Math.max(Math.abs(u) / 0.075, Math.abs(v) / 0.165), w = Math.max(0, Math.min(1, (1.25 - d) / 0.4)) * deep; if (w <= 0) continue; const o = i + nx * k, was = g[o]!, now = Math.max(was, Math.round(w * 255)); if (now !== was) { g[o] = now; lo = Math.min(lo, o); hi = Math.max(hi, o); } } if (hi >= 0) { const a = lo & ~3, b = (hi | 3) + 1; out.push([a, b - a]); } } return out;
}
export const skyCode = (sky: number, trod: number) => `
fn skyCell(c: vec2i) -> f32 { let q = clamp(c, vec2i(0), vec2i(${SKY.n[0] - 1}, ${SKY.n[1] - 1})); let i = u32(q.x + ${SKY.n[0]} * q.y); return extra[${sky}u + (i >> 2u)][i & 3u]; }
/** does the sky reach it: 0 to 1 */
fn exposed(x: vec3f) -> f32 { let g = (x.xz - vec2f(${SKY.lo[0].toFixed(1)}, ${SKY.lo[1].toFixed(1)})) / ${SKY.c} - 0.5; if (any(g < vec2f(-1.0)) || any(g > vec2f(${SKY.n[0].toFixed(1)}, ${SKY.n[1].toFixed(1)}))) { return 1.0; } let b = floor(g); let f = g - b; var s = 0.0; for (var k = 0; k < 4; k++) { let h = skyCell(vec2i(b) + vec2i(k & 1, k >> 1u)); s += select(1.0 - f.x, f.x, (k & 1) == 1) * select(1.0 - f.y, f.y, k > 1) * select(0.0, 1.0, h < x.y + 0.10); } return s; }
fn trodCell(c: vec2i) -> f32 { if (any(c < vec2i(0)) || any(c >= vec2i(${TROD.n[0]}, ${TROD.n[1]}))) { return 0.0; } let i = u32(c.x + ${TROD.n[0]} * c.y); let w = bitcast<vec4u>(extra[${trod}u + (i >> 4u)])[(i >> 2u) & 3u]; return f32((w >> (8u * (i & 3u))) & 255u) * (1.0 / 255.0); }
/** how far the snow is trodden down there, and which way it slopes */
fn trodAt(p: vec2f) -> vec3f { let g = (p - vec2f(${TROD.lo[0].toFixed(1)}, ${TROD.lo[1].toFixed(1)})) / ${TROD.c} - 0.5; let b = vec2i(floor(g)); let f = g - floor(g); let a = trodCell(b); let c = trodCell(b + vec2i(1, 0)); let d = trodCell(b + vec2i(0, 1)); let e = trodCell(b + vec2i(1, 1)); return vec3f(mix(mix(a, c, f.x), mix(d, e, f.x), f.y), mix(c - a, e - d, f.y), mix(d - a, e - c, f.x)); }
/** SNOW, laid over any material the sky reaches from above; leaves carry it on their upper sides; feet press it down */
fn snowCover(s0: Surf, fp: f32) -> Surf {
  var s = s0; if (frame.more.x <= 0.0 || s.kind == 63u || s.kind == 8u) { return s; } let k = frame.more.x * exposed(s.x + s.ng * 0.2 + vec3f(0.0, 0.02, 0.0)); if (k <= 0.0) { return s; }
  if (s.kind == 14u || s.kind == 20u) { let a = k * smoothstep(0.15, 0.6, s.ng.y) * smoothstep(0.25, 0.6, vnoise(s.x.xz * 3.1 + s.x.y * 2.0)); s.albedo = mix(s.albedo, vec3f(0.84, 0.88, 0.93), a * 0.92); s.through = mix(s.through, vec3f(0.05), a); s.rough = mix(s.rough, 0.75, a); return s; }
  let broad = fbm(s.x.xz * 0.55 + 7.0); let fine = vnoise(s.x.xz * 5.0 + s.x.y); let a = smoothstep(0.30, 0.62, (s.ng.y + 0.35 * (broad - 0.5) + 0.12 * (fine - 0.5)) * mix(0.55, 1.0, k)) * smoothstep(0.0, 0.35, k); if (a <= 0.0) { return s; }
  let e = max(0.03, fp); let h0 = fbm(s.x.xz * 1.1); let hx = fbm((s.x.xz + vec2f(e, 0.0)) * 1.1); let hz = fbm((s.x.xz + vec2f(0.0, e)) * 1.1); var g = vec2f(hx - h0, hz - h0) / e * 0.16; let t = trodAt(s.x.xz); let up = smoothstep(0.75, 0.95, s.ng.y); g += vec2f(t.y, t.z) * (-1.1 * up); let press = t.x * up;
  s.albedo = mix(s.albedo, mix(vec3f(0.86, 0.895, 0.94), vec3f(0.62, 0.67, 0.74), press * 0.75), a); s.rough = mix(s.rough, mix(0.78, 0.6, press), a); s.f0 = mix(s.f0, 0.02, a); s.metal = mix(s.metal, 0.0, step(0.4, a)); s.n = normalize(mix(s.n, normalize(s.ng - vec3f(g.x, 0.0, g.y)), a)); if (a > 0.4) { s.kind = 26u; } return s;
}
`;
