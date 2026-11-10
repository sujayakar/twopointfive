import type { Camera } from './room';
import { add, cross, normalize, scale, type Vec3 } from './vec';

/** the eye, and three axes such that a pixel's ray is forward + right · x + up · y, x and y from −1 to 1 */
export interface Eye {
  eye: Vec3;
  forward: Vec3;
  right: Vec3;
  up: Vec3;
}

export function eyeOf(c: Camera, aspect: number): Eye {
  const back: Vec3 = [
    Math.cos(c.pitch) * Math.sin(c.yaw),
    Math.sin(c.pitch),
    Math.cos(c.pitch) * Math.cos(c.yaw),
  ];
  const forward = scale(back, -1);
  const right = normalize(cross(forward, [0, 1, 0]));
  const up = cross(right, forward);
  const half = Math.tan((c.lens * Math.PI) / 360);
  return {
    eye: add(c.target, scale(back, c.distance)),
    forward,
    right: scale(right, half * aspect),
    up: scale(up, half),
  };
}

export function orbit(c: Camera, dx: number, dy: number): void {
  c.yaw -= dx * 0.006;
  c.pitch = Math.max(-1.5, Math.min(1.5, c.pitch + dy * 0.006));
}
export function pan(c: Camera, dx: number, dy: number): void {
  const e = eyeOf(c, 1);
  const size = c.distance * 0.0016;
  c.target = add(c.target, add(scale(normalize(e.right), -dx * size), scale(normalize(e.up), dy * size)));
}
export function dolly(c: Camera, wheel: number): void {
  c.distance = Math.max(0.05, Math.min(200, c.distance * Math.exp(wheel * 0.001)));
}
