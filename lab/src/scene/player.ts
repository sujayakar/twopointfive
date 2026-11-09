// THE ONE AT THE CONTROLS: what keys and pointer ask, turned into what a body can do. It is stopped by what stands, steps up what is low enough, falls where there is no floor, opens doors it walks
// into, and pushes what is loose (the bodies see it as they see anyone). Stepped once a tick by what was asked that tick.
import { type V3, add, sub, mul, norm, clamp } from '../math';
import { Person } from './people';
import type { Solid } from './solid';
import type { World } from './world';
export interface Asked { /** right and forward, −1 to 1, as the camera sees them */ x: number; z: number; run: boolean; low: boolean; up: boolean; aim: V3 | null; fire: boolean }
export const NOTHING: Asked = { x: 0, z: 0, run: false, low: false, up: false, aim: null, fire: false };
const STEP = 0.40, R = 0.24;
export class Player {
  readonly body: Person; asked: Asked = NOTHING; private y: number; private fall = 0; private since = 1; shots: { at: V3; dir: V3; far: number; age: number }[] = []; /** strikes this tick */ hit: { at: V3; dir: V3 } | null = null; torch = true;
  constructor(w: World, private solid: Solid, at: V3, private doors: { leaf: { at: V3; half: V3 }[]; want: number; open: number; hinge: V3 }[]) {
    this.body = new Person(w, [{ at }, { at: add(at, [0, 0, 1]) }], 1, [0.10, 0.72, 0.62], 90, { torch: true }); this.body.drive = { go: [0, 0, 0], aim: null, ready: 0, low: 0 }; this.y = at[1]; this.body.floorAt = x => this.floor(x);
  }
  /** stand it somewhere */
  put(at: V3) { this.y = at[1]; this.fall = 0; this.safe = [...at]; this.body.stand(at); }
  private floor(x: V3): number { const f = this.solid.floor([x[0], this.y, x[2]], 0.05, STEP + 0.25, 0.75); return f ?? this.y - 0.3; }
  /** `yaw` the camera's */
  step(dt: number, yaw: number) {
    const b = this.body, a = this.asked, s = Math.sin(yaw), c = Math.cos(yaw), l = Math.hypot(a.x, a.z), m = l > 1 ? 1 / l : 1, way: V3 = [(a.x * c - a.z * s) * m, 0, (-a.x * s - a.z * c) * m]; const speed = a.low ? 1.25 : a.run && !a.up ? 4.3 : a.up ? 1.7 : 2.2; b.drive!.go = mul(way, speed); b.drive!.aim = a.aim; b.drive!.ready = a.up || a.fire || this.since < 0.8 ? 1 : 0; b.drive!.low = a.low ? 1 : 0;
    const was: V3 = [...b.pos]; b.step(dt); this.since += dt;
    // no farther than what stands allows; and down to the floor, or up a step
    let p: V3 = [b.pos[0], this.y, b.pos[2]]; p = this.solid.push(p, R, a.low ? 1.15 : 1.72, STEP); const f = this.solid.floor(p, 0.13, STEP + 0.02, 30); if (f !== null && this.y - f < 0.45 && this.fall === 0) this.y = f; else { this.fall += 9.81 * dt; this.y -= this.fall * dt; if (f !== null && this.y <= f) { this.y = f; b.land(this.fall); this.fall = 0; } if (this.y < -8) { p = [...this.safe]; this.y = this.safe[1]; this.fall = 0; } } if (this.fall === 0) this.safe = [p[0], this.y, p[2]];
    b.vel = mul(sub([p[0], 0, p[2]], [was[0], 0, was[2]]), 1 / dt); const lag = this.y - b.pos[1]; b.pos = [p[0], Math.abs(lag) > 1.2 ? this.y : b.pos[1] + lag * (1 - Math.exp(-dt * (lag < -0.4 ? 30 : 13))), p[2]];
    // a door it walks into opens
    for (const d of this.doors) { const q = d.leaf[0]; if (!q || d.open > 0.9) continue; const to = sub(q.at, b.pos); if (Math.hypot(to[0], to[2]) < 0.95 && Math.abs(to[1] - 1) < 1.2 && l > 0.2 && (to[0] * way[0] + to[2] * way[2]) > 0.05) d.want = 1; }
    // a shot
    this.hit = null; for (const q of this.shots) q.age += dt; this.shots = this.shots.filter(q => q.age < 0.5); if (a.fire && this.since > 0.11) { this.since = 0; const z = b.muzzle, aim = a.aim ? norm(sub(a.aim, z.at)) : z.dir, along = clamp(aim[0] * z.dir[0] + aim[1] * z.dir[1] + aim[2] * z.dir[2], -1, 1), dir = along > 0.8 ? aim : z.dir, far = this.solid.ray(z.at, dir, 60, true); this.shots.push({ at: [...z.at], dir, far, age: 0 }); b.recoil(2.4); this.hit = { at: add(z.at, mul(dir, far)), dir }; }
  }
  private safe: V3 = [0, 0, 0];
}
