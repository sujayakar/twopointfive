// The tour: camera, floor, doors and hour as a pure function of the tick. The page's C key, and the fixed run the benchmark plays.
import { type Lab, TICK } from './lab';
import type { V3 } from './math';
export function tourStep(lab: Lab, act: { level(i: number): void; doors(v: 'auto'): void }) {
  const t = lab.tick * TICK, c = lab.camera.want, legs = lab.building.place.legs, g = legs[Math.floor(t / 14) % legs.length]!;
  c.yaw = 0.62 + t * 0.045; c.pitch = 0.78 + 0.14 * Math.sin(t * 0.11); c.dist = g.dist; c.look = [...g.look] as V3; if (g.level !== lab.level) act.level(g.level); if (!lab.doorsAuto) act.doors('auto'); lab.hour = 7.5 + ((t * 0.12) % 14.5);
}
export const plainAct = (lab: Lab) => ({ level(i: number) { lab.level = i; lab.wantCut = lab.building.levels[i]!.cut; }, doors() { lab.doorsAuto = true; } });
