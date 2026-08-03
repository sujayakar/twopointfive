import { Vec3, v3, clamp } from "../core/math";
import { BOX_STRIDE_F32, Box } from "../scene/scene";
import { BVH } from "../scene/bvh";
import { Body, PhysicsWorld, bodyRot } from "./physics";
import { Smoke } from "./smoke";

// ---------------------------------------------------------------------------
// Smoke canisters: the physics module's first real caller.
//
// The canister is a rigid sphere for collision (physics.ts's whole design) and a
// small box for display, uploaded as one dynamic box like a particle. It is
// lobbed on the ballistic arc that lands it on the cursor point, bounces and
// rolls to rest, and pops its emitter the moment it settles (or after
// FUSE_MAX if it never does) — the cloud then follows the canister for
// EMIT_SECONDS. A settled body sleeps and costs the solver nothing; it is left
// in the dynamic list rather than baked into the static BVH because it is one
// box and the level restarts wipe it anyway.
// ---------------------------------------------------------------------------

/** Seconds after release before the emitter pops even if it is still moving. */
const FUSE_MAX = 1.4;
/** Seconds of sustained emission. */
// 30 s, not 8: long enough to actually cross a room and be used as cover
// rather than as a puff. The solver's global dissipation still thins it (see
// FluidTuning.dissipation), so this is the emitter's life, not the cloud's.
export const CANISTER_EMIT_SECONDS = 30;
/** Seconds the spent canister lingers after the cloud stops. */
const LINGER = 5;
/** Collision sphere and display box. */
const RADIUS = 0.075;
const HALF = v3(0.05, 0.075, 0.05);
const GRAVITY = 9.81;

interface Live {
  body: Body;
  fuse: number;
  /** -1 while unpopped, then seconds since the emitter started. */
  emitAge: number;
}

export class Canisters {
  readonly world: PhysicsWorld;
  private live: Live[] = [];

  constructor(boxes: Box[], bvh: BVH, private smoke: Smoke) {
    this.world = new PhysicsWorld(boxes, bvh);
  }

  /** Static-BVH AABB query — the fluid occupancy bake reads geometry through it. */
  query = (
    minx: number, miny: number, minz: number,
    maxx: number, maxy: number, maxz: number,
    out: number[],
  ): number[] => this.world.queryAABB(minx, miny, minz, maxx, maxy, maxz, out);

  get count(): number {
    return this.live.length;
  }

  /**
   * Throws from `from` on the lob that lands on the ground point `target`.
   *
   * Flight time grows with distance so a long throw arcs higher instead of
   * flying flat; the vertical speed then follows from where the arc has to
   * come down.
   */
  throw(from: Vec3, target: Vec3): void {
    const dx = target.x - from.x, dz = target.z - from.z;
    const d = clamp(Math.hypot(dx, dz), 0.8, 11);
    const inv = 1 / Math.max(Math.hypot(dx, dz), 1e-6);
    const T = clamp(0.28 + d / 13, 0.35, 1.05);
    const vh = d / T;
    const landY = RADIUS + 0.05;
    const vy = (landY - from.y + 0.5 * GRAVITY * T * T) / T;
    const body = this.world.spawn({
      pos: v3(from.x, from.y, from.z),
      vel: v3(dx * inv * vh, vy, dz * inv * vh),
      // A little end-over-end tumble; deterministic (no RNG in the sim's inputs).
      angVel: v3(dz * inv * 9, 0, -dx * inv * 9),
      radius: RADIUS,
      half: HALF,
      mass: 0.38,
      restitution: 0.3,
      friction: 0.55,
      angularRetention: 0.2,
    });
    this.live.push({ body, fuse: 0, emitAge: -1 });
  }

  /**
   * World-space direction the can is venting, from its orientation.
   *
   * The body's long axis is local +y (half-extents 0.05/0.075/0.05), so a can
   * that came to rest on its side vents sideways and one that landed upright
   * vents upward — both correct, and both for free from where it happened to
   * settle. This is the second column of the quaternion's rotation matrix.
   *
   * Flattened toward horizontal unless it really is standing on end: a jet
   * that hugs the floor is the reference behaviour, and a can resting at a
   * slight tilt should not fire its cloud at the ceiling.
   */
  private ventAxis(orient: Float32Array): Vec3 {
    const [x, y, z, w] = orient;
    let ax = 2 * (x * y - z * w);
    let ay = 1 - 2 * (x * x + z * z);
    let az = 2 * (x * w + y * z);
    const horiz = Math.hypot(ax, az);
    if (horiz > 0.15) {
      // Keep a little of the tilt so it is not perfectly flat, but bias hard
      // toward the ground plane.
      ay = Math.max(-0.15, Math.min(0.25, ay));
      const l = Math.hypot(ax, ay, az) || 1;
      return v3(ax / l, ay / l, az / l);
    }
    // Genuinely upright: let it vent up.
    const l = Math.hypot(ax, ay, az) || 1;
    return v3(ax / l, ay / l, az / l);
  }

  /**
   * Called when a canister settles and starts venting, with the vent's world
   * position. The renderer uses it to anchor the fine smoke lattice — the
   * canister is the one event that both lasts long enough to be worth 26 rows
   * and, being asleep, stays put for all 30 seconds of it.
   */
  onSettle: ((x: number, z: number) => void) | null = null;

  update(dt: number): void {
    this.world.step(dt);
    for (let i = this.live.length - 1; i >= 0; i--) {
      const g = this.live[i];
      if (g.emitAge < 0) {
        g.fuse += dt;
        if (g.body.sleeping || g.fuse >= FUSE_MAX) {
          g.emitAge = 0;
          const b = g.body;
          // Axis captured once, at settle: the can is asleep from here, so
          // re-reading its orientation every frame would only cost work.
          const axis = this.ventAxis(b.orient);
          this.smoke.canisterCloud(
            () => ({
              x: b.pos.x + axis.x * 0.12,
              y: b.pos.y + 0.14,
              z: b.pos.z + axis.z * 0.12,
            }),
            CANISTER_EMIT_SECONDS,
            axis,
          );
          this.onSettle?.(b.pos.x, b.pos.z);
        }
        continue;
      }
      g.emitAge += dt;
      if (g.emitAge > CANISTER_EMIT_SECONDS + LINGER) {
        this.world.remove(g.body);
        this.live.splice(i, 1);
      }
    }
  }

  /** Packs the canisters into the dynamic-box array like particles. */
  buildBoxes(out: Float32Array, offset: number, mat: number): number {
    const capacity = Math.floor(out.length / BOX_STRIDE_F32) - offset;
    const n = Math.min(this.live.length, capacity);
    const u = new Uint32Array(out.buffer, out.byteOffset, out.length);
    for (let i = 0; i < n; i++) {
      const b = this.live[i].body;
      const o = (offset + i) * BOX_STRIDE_F32;
      out[o] = b.pos.x; out[o + 1] = b.pos.y; out[o + 2] = b.pos.z;
      u[o + 3] = mat;
      out[o + 4] = b.half.x; out[o + 5] = b.half.y; out[o + 6] = b.half.z;
      u[o + 7] = 0;
      const [r0, r1, r2] = bodyRot(b);
      out[o + 8] = r0.x; out[o + 9] = r0.y; out[o + 10] = r0.z; out[o + 11] = 0;
      out[o + 12] = r1.x; out[o + 13] = r1.y; out[o + 14] = r1.z; out[o + 15] = 0;
      out[o + 16] = r2.x; out[o + 17] = r2.y; out[o + 18] = r2.z; out[o + 19] = 0;
    }
    return n;
  }

  reset(): void {
    for (const g of this.live) this.world.remove(g.body);
    this.live.length = 0;
    this.world.step(0);
  }
}
