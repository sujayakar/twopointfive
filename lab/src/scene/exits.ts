// The signs over the ways out. They run on their own batteries: in a blackout they are what is left.
import { type V3, rotY, apply, add, mul } from '../math';
import { type World, F } from './world';
import { M } from './materials';
export function exitSign(w: World, at: V3, yaw: number) {
  const r = rotY(yaw), n = apply(r, [0, 0, 1]), P = (x: number, y: number, z: number): V3 => add(at, apply(r, [x, y, z])), rad: V3 = [1.6, 16, 6.5]; w.as('exit sign', () => {
    const lamp = w.lamp({ kind: 'rect', at: P(0, 0, 0.034), u: mul(apply(r, [1, 0, 0]), 0.17), v: [0, 0.08, 0], n, radiance: rad, name: 'exit', on: 1 }); w.add({ at: P(0, 0, 0.012), half: [0.19, 0.10, 0.014], rot: r, mat: M.plastic!, tint: [0.2, 0.2, 0.21], flags: F.CUT });
    w.add({ at: P(0, 0, 0.028), half: [0.17, 0.08, 0.003], rot: r, mat: M.glow!, tint: [Math.sqrt(0.10), 1, Math.sqrt(0.40)], flags: F.CUT | F.GLOW | F.NOSHADOW, lamp, seed: 40, marks: [{ face: 5, at: [0, 0], size: [0.30, 0.14], art: 'exit', color: [1, 1, 1] }] }); });
}
