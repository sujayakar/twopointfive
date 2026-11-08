// THE PICTURE'S CAMERA OFF THE RUN'S OWN VIEW, paused or playing: a drag moves it over the floor, a right drag (or Shift + drag) turns and tilts it, Ctrl turns it as in play, the wheel zooms,
// W A S D slide it. Turned or zoomed only, it still follows where the run looked. Only the picture's: the game is handed the tape's hands.
import { FollowCamera } from '../host/camera';
import { clamp, DEG, type Vec3 } from '../math/vec';

export interface TapeView { look: Vec3; yaw: number; pitch: number; dist: number; fov: number }
const FLY = new Set(['KeyW', 'KeyA', 'KeyS', 'KeyD']);
/** its tilt and zoom, how far from where the run looked it may be taken, and the pixels a press may travel and still be a click */
const PITCH = [12 * DEG, 89 * DEG], DIST = [3, 60], FAR = 80, DRAG = 5;

export class FreeCam {
  /** null: the run's own view. `look` null: it follows where the run looked */
  set: { look: Vec3 | null; yaw: number; pitch: number; dist: number } | null = null;
  private scratch = new FollowCamera(); private held = new Set<string>();
  private drag: { id: number; x: number; y: number; u: number; v: number; turn: boolean; level: boolean; ctrl: boolean; left: boolean; moved: boolean; from: TapeView; grab: Vec3 | null } | null = null;
  private dragged = false;
  /** `run`: the run's own view as it stands · `busy`: the mouse and the keys are somebody else's · `turn`: radians for the pixels of a Ctrl turn */
  constructor(private canvas: HTMLCanvasElement, private run: () => TapeView, private aspect: () => number, private busy: () => boolean, private turn: (px: number) => number) { this.mouse(); }

  /** the view the next picture is drawn through */
  view(): TapeView { const r = this.run(), F = this.set; return F ? { look: F.look ? [F.look[0], F.look[1], F.look[2]] : r.look, yaw: F.yaw, pitch: F.pitch, dist: F.dist, fov: r.fov } : r; }
  /** the run's own view again */
  back(): void { const D = this.drag; this.drag = null; if (D?.moved) { this.dragged = D.left; this.canvas.style.cursor = ''; } this.set = null; }
  /** it goes to look at a spot */
  to(look: Vec3): void { this.take().look = this.near(look); }
  key(code: string, down: boolean): boolean { if (!FLY.has(code)) return false; if (down) this.held.add(code); else this.held.delete(code); return true; }
  letGo(): void { this.held.clear(); }
  /** the click just heard was the end of a drag */
  wasDrag(): boolean { const d = this.dragged; this.dragged = false; return d; }
  /** the keys' slide, `dt` seconds of the display's */
  frame(dt: number): void {
    const k = this.held; if (!k.size || this.busy()) return;
    const x = (k.has('KeyD') ? 1 : 0) - (k.has('KeyA') ? 1 : 0), z = (k.has('KeyW') ? 1 : 0) - (k.has('KeyS') ? 1 : 0); if (!x && !z) return;
    const v = this.view(), s = v.dist * 0.6 * Math.min(Math.max(dt, 0), 0.1) / Math.hypot(x, z), sy = Math.sin(v.yaw), cy = Math.cos(v.yaw);
    this.to([v.look[0] + (-sy * z + cy * x) * s, v.look[1], v.look[2] + (-cy * z - sy * x) * s]);
  }
  /** the floor point under a spot of the picture */
  floorAt(v: TapeView, s: number, t: number): Vec3 | null { const C = this.scratch; C.target = v.look; C.yaw = v.yaw; C.pitch = v.pitch; C.distance = v.dist; C.fov = v.fov; C.aspect = this.aspect(); C.rebuild(); return C.groundPoint(s, t, v.look[1]); }
  uv(e: MouseEvent): [number, number] { const r = this.canvas.getBoundingClientRect(); return [(e.clientX - r.left) / Math.max(1, r.width), (e.clientY - r.top) / Math.max(1, r.height)]; }

  private take(): NonNullable<FreeCam['set']> { if (!this.set) { const v = this.view(); this.set = { look: null, yaw: v.yaw, pitch: v.pitch, dist: v.dist }; } return this.set; }
  private near(p: Vec3): Vec3 { const c = this.run().look, dx = p[0] - c[0], dz = p[2] - c[2], d = Math.hypot(dx, dz), k = d > FAR ? FAR / d : 1; return [c[0] + dx * k, p[1], c[2] + dz * k]; }
  private mouse(): void {
    const cv = this.canvas, zoomed = (F: NonNullable<FreeCam['set']>, px: number): void => { F.dist = clamp(F.dist * Math.exp(clamp(px, -400, 400) * 0.0012), DIST[0], DIST[1]); };
    cv.addEventListener('pointerdown', e => { this.dragged = false; if (this.busy() || e.button > 2) return; const [u, v] = this.uv(e);
      this.drag = { id: e.pointerId, x: e.clientX, y: e.clientY, u, v, turn: e.button !== 0 || e.shiftKey || e.ctrlKey, level: e.ctrlKey && !e.shiftKey, ctrl: e.ctrlKey, left: e.button === 0, moved: false, from: this.view(), grab: null }; });
    const up = (e: PointerEvent): void => { const D = this.drag; if (!D || e.pointerId !== D.id) return; this.drag = null; if (D.moved) { this.dragged = D.left; try { cv.releasePointerCapture(D.id); } catch { /* it was let go of */ } cv.style.cursor = ''; } };
    cv.addEventListener('pointermove', e => { const D = this.drag;
      if (!D) { if (e.ctrlKey && !e.buttons && e.movementX && !this.busy()) this.take().yaw += this.turn(e.movementX); return; }
      if (e.pointerId !== D.id) return; if (!e.buttons) { up(e); return; }   // (the press ended somewhere this canvas did not hear of)
      const dx = e.clientX - D.x, dy = e.clientY - D.y;
      if (!D.moved) { if (Math.hypot(dx, dy) < DRAG) return; D.moved = true; D.from = this.view(); D.grab = D.turn ? null : this.floorAt(D.from, D.u, D.v); try { cv.setPointerCapture(D.id); } catch { /* it was let go of */ } cv.style.cursor = 'grabbing'; }
      const F = this.take();
      if (D.turn) { F.yaw = D.from.yaw + (D.ctrl ? this.turn(dx) : dx * 0.006); if (!D.level) F.pitch = clamp(D.from.pitch + dy * 0.005, PITCH[0], PITCH[1]); }
      else { const [u, v] = this.uv(e), q = D.grab && this.floorAt(D.from, u, v); if (D.grab && q) F.look = this.near([D.from.look[0] + D.grab[0] - q[0], D.from.look[1], D.from.look[2] + D.grab[2] - q[2]]); } });
    cv.addEventListener('pointerup', up); cv.addEventListener('pointercancel', up);
    cv.addEventListener('wheel', e => { if (this.busy() || !e.deltaY) return; zoomed(this.take(), e.deltaY * (e.deltaMode === 1 ? 33 : e.deltaMode === 2 ? 400 : 1));   // (a wheel that counts lines or pages, as pixels)
      const D = this.drag; if (D?.moved) { D.x = e.clientX; D.y = e.clientY; [D.u, D.v] = this.uv(e); D.from = this.view(); D.grab = D.turn ? null : this.floorAt(D.from, D.u, D.v); } }, { passive: true });
  }
}
