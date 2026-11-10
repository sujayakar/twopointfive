export type Vec3 = [number, number, number];
export type Quat = [number, number, number, number];

export const add = (a: Vec3, b: Vec3): Vec3 => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
export const sub = (a: Vec3, b: Vec3): Vec3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
export const scale = (a: Vec3, s: number): Vec3 => [a[0] * s, a[1] * s, a[2] * s];
export const dot = (a: Vec3, b: Vec3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
export const cross = (a: Vec3, b: Vec3): Vec3 => [
  a[1] * b[2] - a[2] * b[1],
  a[2] * b[0] - a[0] * b[2],
  a[0] * b[1] - a[1] * b[0],
];
export const length = (a: Vec3): number => Math.hypot(...a);
export const normalize = (a: Vec3): Vec3 => scale(a, 1 / (length(a) || 1));

export const NO_TURN: Quat = [0, 0, 0, 1];
export function turnAbout(axis: Vec3, radians: number): Quat {
  const a = normalize(axis);
  const s = Math.sin(radians / 2);
  return [a[0] * s, a[1] * s, a[2] * s, Math.cos(radians / 2)];
}
/** first b, then a */
export function turnBoth(a: Quat, b: Quat): Quat {
  return [
    a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1],
    a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0],
    a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3],
    a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2],
  ];
}
export function turnVec(q: Quat, v: Vec3): Vec3 {
  const u: Vec3 = [q[0], q[1], q[2]];
  const t = add(cross(u, v), scale(v, q[3]));
  return add(v, scale(cross(u, t), 2));
}
