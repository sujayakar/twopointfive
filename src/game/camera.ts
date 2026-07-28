import {
  Mat4, Vec3, damp, mat4Identity, mat4Invert, mat4LookAt, mat4Mul, mat4Perspective, v3,
} from "../core/math";

/**
 * Fixed-angle follow camera. The reference look is a high three-quarter view
 * with a narrow FOV from far away — that keeps verticals nearly parallel so it
 * reads as isometric, while still giving enough perspective for depth.
 */
export class Camera {
  /** Rotation about Y. 45 degrees puts the world's axes on the screen diagonals. */
  yaw = Math.PI * 0.25;
  /** Elevation above the horizon. */
  pitch = Math.PI * 0.33;
  distance = 23;
  fovY = (34 * Math.PI) / 180;

  pos: Vec3 = v3(0, 10, 10);
  target: Vec3 = v3(0, 1, 0);

  viewProj: Mat4 = mat4Identity();
  prevViewProj: Mat4 = mat4Identity();
  invViewProj: Mat4 = mat4Identity();

  private smoothTarget: Vec3 = v3(0, 1, 0);

  /**
   * @param focus  where the camera should look (player position, biased toward
   *               the aim point so you can see a little further ahead).
   */
  update(dt: number, focus: Vec3, aspect: number): void {
    this.smoothTarget = v3(
      damp(this.smoothTarget.x, focus.x, 0.0015, dt),
      damp(this.smoothTarget.y, focus.y, 0.0015, dt),
      damp(this.smoothTarget.z, focus.z, 0.0015, dt),
    );
    this.target = this.smoothTarget;

    const cp = Math.cos(this.pitch);
    const sp = Math.sin(this.pitch);
    this.pos = v3(
      this.target.x + Math.sin(this.yaw) * cp * this.distance,
      this.target.y + sp * this.distance,
      this.target.z + Math.cos(this.yaw) * cp * this.distance,
    );

    this.prevViewProj = this.viewProj;
    const view = mat4LookAt(this.pos, this.target, v3(0, 1, 0));
    const proj = mat4Perspective(this.fovY, aspect, 0.1);
    this.viewProj = mat4Mul(proj, view);
    this.invViewProj = mat4Invert(this.viewProj);
  }

  /** Ground-plane forward/right for camera-relative movement. */
  groundBasis(): { forward: Vec3; right: Vec3 } {
    const f = v3(-Math.sin(this.yaw), 0, -Math.cos(this.yaw));
    const r = v3(-f.z, 0, f.x);
    return { forward: f, right: r };
  }

  /**
   * Unprojects a device-pixel screen position onto the horizontal plane at
   * height `planeY`. This is how the character knows where the cursor is.
   */
  screenToGround(px: number, py: number, w: number, h: number, planeY: number): Vec3 {
    const ndcX = (px / w) * 2 - 1;
    const ndcY = 1 - (py / h) * 2;
    const m = this.invViewProj;
    const apply = (x: number, y: number, z: number): Vec3 => {
      const ox = m[0] * x + m[4] * y + m[8] * z + m[12];
      const oy = m[1] * x + m[5] * y + m[9] * z + m[13];
      const oz = m[2] * x + m[6] * y + m[10] * z + m[14];
      const ow = m[3] * x + m[7] * y + m[11] * z + m[15];
      const inv = ow !== 0 ? 1 / ow : 1;
      return v3(ox * inv, oy * inv, oz * inv);
    };
    // Unproject on the NEAR plane (reverse-Z puts it at ndc z = 1). The far
    // plane is at z = 0 and this projection is infinite, so unprojecting there
    // divides by ~0 and yields a garbage direction — which is what made the
    // character's aim drift away from the cursor.
    const nearPt = apply(ndcX, ndcY, 1);
    const dir = v3(nearPt.x - this.pos.x, nearPt.y - this.pos.y, nearPt.z - this.pos.z);
    if (Math.abs(dir.y) < 1e-6) return v3(this.pos.x, planeY, this.pos.z);
    const t = (planeY - this.pos.y) / dir.y;
    if (t < 0) return v3(this.pos.x, planeY, this.pos.z);
    return v3(this.pos.x + dir.x * t, planeY, this.pos.z + dir.z * t);
  }
}
