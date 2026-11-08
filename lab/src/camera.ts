import { type V3, add, mul, sub, norm, cross, clamp } from './math';
/** looks down at a point, as the game does; eased, and only by the tick, so a run repeats */
export class Camera {
  look: V3 = [0, 3.0, 0]; yaw = 0.62; pitch = 0.94; dist = 44; fov = 0.40;
  want = { look: [0, 3.0, 0] as V3, yaw: 0.62, pitch: 0.94, dist: 44 };
  step(dt: number) { const k = 1 - Math.exp(-dt * 7), w = this.want; w.pitch = clamp(w.pitch, 0.12, 1.5); w.dist = clamp(w.dist, 4, 110); this.look = add(this.look, mul(sub(w.look, this.look), k)); this.yaw += (w.yaw - this.yaw) * k; this.pitch += (w.pitch - this.pitch) * k; this.dist += (w.dist - this.dist) * k; }
  snap() { const w = this.want; w.pitch = clamp(w.pitch, 0.12, 1.5); this.look = [...w.look] as V3; this.yaw = w.yaw; this.pitch = w.pitch; this.dist = w.dist; }
  frame(aspect: number) {
    const cp = Math.cos(this.pitch), back: V3 = [Math.sin(this.yaw) * cp, Math.sin(this.pitch), Math.cos(this.yaw) * cp], fwd = mul(back, -1), right = norm(cross(fwd, [0, 1, 0])), up = cross(right, fwd), t = Math.tan(this.fov / 2);
    return { pos: add(this.look, mul(back, this.dist)), right: mul(right, t * aspect), up: mul(up, t), fwd };
  }
  ray(aspect: number, u: number, v: number) { const f = this.frame(aspect), x = u * 2 - 1, y = v * 2 - 1; return { from: f.pos, dir: norm(add(f.fwd, add(mul(f.right, x), mul(f.up, -y)))) }; }
}
