// THE TITLE, LEFT ALONE: after a few seconds with nobody touching anything the camera cuts round a loop of framings of the floor behind it, under a black veil, while the guards walk their
// beats through them. The first key, click, wheel tick or real mouse movement puts a fresh floor and the view back, and the world still.
import type { FollowCamera } from './camera';
import { DEG, smoothstep, v3, type Vec3 } from '../math/vec';

/** seconds of nobody before it starts; a shot's hold; how far the yaw drifts and the distance closes across a hold; the first veil's seconds */
const IDLE = 6, HOLD = 7, DRIFT = 5, PUSH_IN = 0.08, VEIL_UP = 0.3;
/** a framing: the floor point looked at, the yaw it is cut to (degrees), the distance it opens at; null = on the first guard, from behind, the way he is going */
interface Shot { at: Vec3 | null; yaw: number; dist: number }
const SHOTS: readonly Shot[] = [
  { at: [7.6, 0, 17.6], yaw: -22, dist: 19 }, { at: null, yaw: -60, dist: 12 }, { at: [15.5, 0, 25.6], yaw: -30, dist: 18 }, { at: [23, 0, 19.4], yaw: -24, dist: 18 },
  { at: [9.9, 0, 7.7], yaw: -30, dist: 13.5 }, { at: [18.5, 0, 8.4], yaw: -52, dist: 11 }, { at: [34.4, 0, 20.3], yaw: -32, dist: 15 },
];

export interface ReelDeps {
  /** may it run: the title is up, with no screen over it */
  may(): boolean;
  /** the world is let go, behind black, with nobody playing */
  begin(): void;
  /** where the first guard stands */
  man(): Vec3 | null;
  /** a fresh floor, and the view the title had */
  back(): void;
  /** the veil's black, 0 to 1, and whether the reel is on */
  veil(black: number, on: boolean): void;
}
export class Reel {
  /** the world runs for it */
  live = false;
  private on = false; private black = 0; private full = false; private idle = 0; private shot = -1; private t = 0; private yaw = 0;
  private was: { yaw: number; pitch: number; distance: number } | null = null;
  private seen: Vec3 | null = null; private way: [number, number] = [0, 0];
  constructor(private cam: FollowCamera, private d: ReelDeps) {
    for (const ev of ['keydown', 'mousedown', 'pointerdown', 'wheel', 'touchstart']) window.addEventListener(ev, () => this.poke(), { capture: true, passive: true });
    // (a mouse that is moved, not one that is knocked: a few pixels inside a third of a second)
    let x: number | null = null, y = 0, far = 0, at = 0;
    window.addEventListener('mousemove', e => { if (x !== null) { const d = Math.abs(e.clientX - x) + Math.abs(e.clientY - y), now = performance.now(); far = now - at > 300 ? d : far + d; at = now; if (far >= 3) { far = 0; this.poke(); } } x = e.clientX; y = e.clientY; }, { capture: true, passive: true });
  }
  /** somebody is here */
  poke(): void { this.idle = 0; if (this.on) this.stop(); }
  private stop(): void {
    const { cam, was } = this, ran = this.live;
    this.on = this.live = this.full = false; this.black = 0; this.shot = -1; this.was = null; this.d.veil(0, false);
    if (!ran || !was) return;
    cam.yaw = was.yaw; cam.pitch = was.pitch; cam.distance = cam.desiredDistance = was.distance; this.d.back();
  }
  /** a display frame, `dt` seconds of the wall's */
  frame(dt: number): void {
    if (!this.d.may()) { this.idle = 0; if (this.on) this.stop(); return; }
    dt = Math.min(dt, 0.25);   // (a tab that comes back arrives with one enormous frame)
    if (!this.on) { if ((this.idle += dt) >= IDLE) { this.on = true; this.black = 0; this.full = false; } return; }
    if (this.live) { this.d.veil(this.black, true); return; }
    this.black = Math.min(1, this.black + dt / VEIL_UP); this.d.veil(this.black, true);
    if (this.black < 1) return;
    if (!this.full) { this.full = true; return; }   // (one presented frame wholly black, and only then does anything behind it move)
    const c = this.cam; this.was = { yaw: c.yaw, pitch: c.pitch, distance: c.desiredDistance };
    this.d.begin(); this.live = true; this.cut();
  }
  private cut(): void {
    this.shot = (this.shot + 1) % SHOTS.length; this.t = 0; this.black = Math.max(this.black, 0.999);
    const s = SHOTS[this.shot], c = this.cam, at = s.at ?? this.seen ?? SHOTS[0].at!, [x, z] = this.way;
    this.yaw = !s.at && Math.hypot(x, z) > 0.005 ? Math.atan2(-x, -z) / DEG : s.yaw;
    c.yaw = this.yaw * DEG; c.distance = c.desiredDistance = s.dist; c.target = v3.copy(at); c.desired = v3.copy(at);
  }
  /** a tick of the world it runs: what the camera follows, null while it is not live */
  step(dt: number): Vec3 | null {
    if (!this.live) return null;
    const man = this.d.man(); if (man) { if (this.seen) this.way = [man[0] - this.seen[0], man[2] - this.seen[2]]; this.seen = v3.copy(man); }
    const s = SHOTS[this.shot], u = Math.min(1, (this.t += dt) / HOLD), c = this.cam;
    c.yaw = (this.yaw + (this.shot % 2 ? -1 : 1) * DRIFT * u) * DEG; c.desiredDistance = s.dist * (1 - PUSH_IN * smoothstep(0, 1, u));
    this.black = HOLD - this.t < 0.35 || this.t < 0.1 ? Math.min(1, this.black + dt / 0.3) : Math.max(0, this.black - dt / 0.45);
    if (this.t >= HOLD) this.cut();
    return SHOTS[this.shot].at ?? this.seen ?? SHOTS[0].at;
  }
}
