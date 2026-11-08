// The tour: camera, floor, doors and hour as a pure function of the tick. The page's C key, and the fixed run the benchmark plays.
import { type Lab, TICK } from './lab';
import type { V3 } from './math';
export const LEGS: { name: string; look: V3; dist: number; level: number }[] = [{ name: 'roof', look: [1, 3, 0], dist: 52, level: 0 }, { name: 'garden', look: [7.5, 3.6, 0.5], dist: 32, level: 0 }, { name: 'upper floor', look: [-4, 3.4, -1], dist: 38, level: 1 }, { name: 'ground near', look: [2, 0.6, 0], dist: 30, level: 2 }, { name: 'ground', look: [0, 0.6, 0], dist: 46, level: 2 }];
export function tourStep(lab: Lab, act: { level(i: number): void; doors(v: 'auto'): void }) {
  const t = lab.tick * TICK, c = lab.camera.want, g = LEGS[Math.floor(t / 14) % LEGS.length]!;
  c.yaw = 0.62 + t * 0.045; c.pitch = 0.78 + 0.14 * Math.sin(t * 0.11); c.dist = g.dist; c.look = [...g.look] as V3; if (g.level !== lab.level) act.level(g.level); if (!lab.doorsAuto) act.doors('auto'); lab.hour = 7.5 + ((t * 0.12) % 14.5);
}
export const plainAct = (lab: Lab) => ({ level(i: number) { lab.level = i; lab.wantCut = lab.building.levels[i]!.cut; }, doors() { lab.doorsAuto = true; } });
