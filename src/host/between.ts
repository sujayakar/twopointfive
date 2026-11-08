// BETWEEN TWO TICKS, the page's half: how far from the tick before to the last a drawn frame stands, and where the camera does. What the game has (the boxes, the
// men, the lights) stands there in the module (src/draw/drawn.ts `frame`); this is the display's clock and the host's own camera.
import type { Vec3 } from '../math/vec';

/** frames the lag looks back over, how fast one longer than the display needs is let go (ticks a frame), and a move longer than this in one tick is a cut (metres) */
const WINDOW = 32, EASE = 0.002, SNAP = 1.5, TURN = 0.5;

/** A PICTURE THAT IS TO MOVE EVENLY STANDS A CONSTANT TIME BEHIND ITS FRAMES, far enough that no frame asks for a tick that has not run: the most any of the last
 *  WINDOW frames owed the next tick. A display at the tick rate owes nothing and draws the last tick; one at twice it stands half a tick behind. */
export class Lag {
  lag = 0;
  private owed = new Float64Array(WINDOW); private n = 0; private i = 0;
  /** `owed` = the time owed the next tick ÷ the tick, 0 ≤ owed < 1: how far from the tick before to the last this frame stands (1 = the last) */
  at(owed: number): number {
    const r = this.owed; r[this.i] = owed; this.i = (this.i + 1) % WINDOW; if (this.n < WINDOW) this.n++;
    let most = 0; for (let i = 0; i < this.n; i++) if (r[i] > most) most = r[i];
    if (owed > this.lag) this.lag = owed; else if (this.lag > most) this.lag = Math.max(most, this.lag - EASE);
    const a = 1 - (this.lag - owed);
    return a > 1 - 1e-6 ? 1 : a < 0 ? 0 : a;
  }
  reset(): void { this.lag = 0; this.n = 0; this.i = 0; }
}

export interface Pose { target: Vec3; yaw: number; pitch: number; distance: number }
/** the camera as its last two ticks' steps left it */
export class CameraPair {
  private a = new Float64Array(6); private b = new Float64Array(6); private have = 0;
  private out: Pose = { target: [0, 0, 0], yaw: 0, pitch: 0, distance: 0 };
  capture(c: Pose): void { const t = this.a; this.a = this.b; this.b = t; t[0] = c.target[0]; t[1] = c.target[1]; t[2] = c.target[2]; t[3] = c.yaw; t[4] = c.pitch; t[5] = c.distance; this.have = Math.min(2, this.have + 1); }
  cut(): void { this.have = 0; }
  /** between the two, or null: the last as it is (nothing to blend, a cut, or `now` is no longer where the last tick left it) */
  at(k: number, now: Pose): Pose | null {
    const p = this.a, c = this.b; if (this.have < 2 || k >= 1) return null;
    if (now.target[0] !== c[0] || now.target[1] !== c[1] || now.target[2] !== c[2] || now.yaw !== c[3] || now.pitch !== c[4] || now.distance !== c[5]) return null;
    let dy = (c[3] - p[3]) % (2 * Math.PI); if (dy > Math.PI) dy -= 2 * Math.PI; else if (dy < -Math.PI) dy += 2 * Math.PI;
    if (Math.hypot(c[0] - p[0], c[1] - p[1], c[2] - p[2]) > SNAP || Math.abs(c[5] - p[5]) > SNAP || Math.abs(dy) > TURN || Math.abs(c[4] - p[4]) > TURN) return null;
    const o = this.out, t = o.target; for (let i = 0; i < 3; i++) t[i] = p[i] + (c[i] - p[i]) * k;
    o.yaw = p[3] + dy * k; o.pitch = p[4] + (c[4] - p[4]) * k; o.distance = p[5] + (c[5] - p[5]) * k;
    return o;
  }
}
