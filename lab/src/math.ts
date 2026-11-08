export type V3 = [number, number, number];
export const add = (a: V3, b: V3): V3 => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
export const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
export const mul = (a: V3, s: number): V3 => [a[0] * s, a[1] * s, a[2] * s];
export const dot = (a: V3, b: V3) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
export const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
export const len = (a: V3) => Math.hypot(a[0], a[1], a[2]);
export const norm = (a: V3): V3 => { const l = len(a) || 1; return [a[0] / l, a[1] / l, a[2] / l]; };
export const mix = (a: V3, b: V3, t: number): V3 => [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t];
export const clamp = (x: number, a: number, b: number) => Math.min(b, Math.max(a, x));
export const smooth = (a: number, b: number, x: number) => { const t = clamp((x - a) / (b - a), 0, 1); return t * t * (3 - 2 * t); };
/** a small seeded generator: the same scene every time */
export const rng = (seed: number) => () => { seed |= 0; seed = (seed + 0x6D2B79F5) | 0; let t = Math.imul(seed ^ (seed >>> 15), 1 | seed); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; };

/** a rotation, as the three local axes in world space */
export type Rot = [V3, V3, V3];
export const ID: Rot = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
export const rotY = (a: number): Rot => { const c = Math.cos(a), s = Math.sin(a); return [[c, 0, -s], [0, 1, 0], [s, 0, c]]; };
export const rotX = (a: number): Rot => { const c = Math.cos(a), s = Math.sin(a); return [[1, 0, 0], [0, c, s], [0, -s, c]]; };
export const rotZ = (a: number): Rot => { const c = Math.cos(a), s = Math.sin(a); return [[c, s, 0], [-s, c, 0], [0, 0, 1]]; };
/** v in the rotation's frame → world */
export const apply = (r: Rot, v: V3): V3 => [r[0][0] * v[0] + r[1][0] * v[1] + r[2][0] * v[2], r[0][1] * v[0] + r[1][1] * v[1] + r[2][1] * v[2], r[0][2] * v[0] + r[1][2] * v[1] + r[2][2] * v[2]];
/** a then b's frame: world = a · b · local */
export const compose = (a: Rot, b: Rot): Rot => [apply(a, b[0]), apply(a, b[1]), apply(a, b[2])];
/** the frame whose y runs from a to b */
export const along = (dir: V3, hint: V3 = [0, 0, 1]): Rot => { const y = norm(dir); let x = cross(y, hint); if (len(x) < 1e-4) x = cross(y, [1, 0, 0]); x = norm(x); return [x, y, cross(x, y)]; };

export type M4 = Float32Array;
export const mat = (): M4 => new Float32Array(16);
export function perspective(fov: number, aspect: number, near: number, far: number): M4 {
  const f = 1 / Math.tan(fov / 2), m = mat(); m[0] = f / aspect; m[5] = f; m[10] = far / (near - far); m[11] = -1; m[14] = near * far / (near - far); return m;
}
export function view(eye: V3, r: V3, u: V3, back: V3): M4 {
  const m = mat(); m.set([r[0], u[0], back[0], 0, r[1], u[1], back[1], 0, r[2], u[2], back[2], 0, -dot(r, eye), -dot(u, eye), -dot(back, eye), 1]); return m;
}
export function mmul(a: M4, b: M4): M4 { const o = mat(); for (let c = 0; c < 4; c++) for (let r = 0; r < 4; r++) { let s = 0; for (let k = 0; k < 4; k++) s += a[k * 4 + r]! * b[c * 4 + k]!; o[c * 4 + r] = s; } return o; }
