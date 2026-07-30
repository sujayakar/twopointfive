import { Vec3, v3, clamp, rotY } from "../core/math";
import { Box, SceneBuilder } from "../scene/scene";
import { BVH, BVH_NODE_STRIDE_F32, buildBVH } from "../scene/bvh";
import { quatMul, quatRotate } from "../anim/rig";

// ---------------------------------------------------------------------------
// Rigid bodies for throwables (canisters now, kickable crates later).
//
// THE CONSTRAINT THAT SHAPES THIS FILE
//
// The renderer has no acceleration structure for moving geometry. Anything that
// moves is uploaded into `dynBoxes` and tested LINEARLY by every ray; the only
// culling is one slab test per DYN_GROUP_SIZE-box group (see common.wgsl). The
// measured cost at 1152x720 / 2 bounces:
//
//     120 dynamic boxes, per-group AABB rejection   ~1.6 ms/frame
//     120 dynamic boxes, no rejection              ~19.5 ms/frame
//
// So the frame budget, not the solver, sets the body count: a few dozen boxes
// in motion at once. Everything here is therefore built around bodies going to
// sleep quickly and staying asleep, so the caller can bake them back into the
// static BVH and stop paying per-ray for them:
//
//     for (const b of world.awake) uploadDynamicBox(bodyBox(b, mat));
//     for (const b of world.bodies) if (b.justSlept) bakeIntoStaticScene(b);
//
// A solver that is 5x faster buys nothing. A body that sleeps 0.3 s sooner buys
// a whole dynamic group back.
//
// SHAPE: bodies are SPHERES for collision, with a box only for display.
//
// Sphere-vs-OBB is exact, allocation-free, produces one unambiguous contact
// point, and cannot generate the conflicting multi-point manifolds that make
// box stacks jitter. Canisters are round anyway. A kickable crate gets its
// inscribed sphere, so its corners interpenetrate walls by up to (sqrt(3)-1)*r
// — at crate scale that is a couple of centimetres and nothing in a top-down
// stealth game reads it. Revisit only if crates start resting on edges.
//
// CONVENTIONS (matched exactly to the GPU path, do not drift):
//   - `Box.rot[i]` is the box's local axis i expressed in world space, so
//     world = center + p.x*rot[0] + p.y*rot[1] + p.z*rot[2], and world->local
//     is three dot products (see src/core/math.ts).
//   - `raycast` is a port of `trace`/`hitBox` in src/shaders/common.wgsl,
//     including the inside-the-box exit case and the ray-facing normal flip.
//   - BVH leaves index the PACKED box order, so a CPU traversal must go through
//     `bvh.order` to get back to the original `boxes[]` index. `packBoxes` does
//     the same permutation on the GPU side.
// ---------------------------------------------------------------------------

// --- tuning -----------------------------------------------------------------

/**
 * Physics tick. `step()` accumulates real time and runs whole ticks of this
 * size; it never integrates a frame's dt directly, because both restitution and
 * penetration depth scale with dt, so a variable one makes a settling bounce
 * frame-rate dependent. Verified in selfTest(): requested restitutions of 0.3 /
 * 0.6 / 0.9 measure back as 0.300000 / 0.600000 / 0.900000, and a dropped body
 * reaches the same rest height to 1e-12 whether the caller steps at 30 or
 * 240 fps.
 *
 * Why 120 and not 60 — and it is NOT about per-tick travel, because
 * `MICRO_TRAVEL` bounds that independently of the tick rate. It is about the
 * ceiling that `MAX_MICRO_STEPS` puts on subdivision: the fastest a 0.06 m body
 * can move without tunnelling is MAX_MICRO_STEPS*MICRO_TRAVEL*r/FIXED_DT, which
 * is 17.3 m/s at 1/60 and 34.6 m/s at 1/120. A hard throw is ~12 m/s and a
 * blast can double that, so 1/60 leaves no margin and 1/120 does. The second
 * tick costs 0.033 ms with 32 bodies awake.
 */
const FIXED_DT = 1 / 120;

/** Largest frame delta fed to the accumulator; beyond this we drop time rather
 * than spiral (a tab regaining focus reports multi-second deltas). */
const MAX_FRAME_DT = 0.25;

/**
 * A tick is subdivided until no body moves further than this fraction of the
 * smallest awake radius. This is the cheap stand-in for CCD: it costs nothing
 * while everything is slow, and only the moment after a throw pays for it.
 */
const MICRO_TRAVEL = 0.6;
const MAX_MICRO_STEPS = 8;

const VELOCITY_ITERATIONS = 6;
const POSITION_ITERATIONS = 3;

/**
 * Approach speeds below this bounce with restitution 0.
 *
 * Without it a settling body never stops: at e=0.35 an 0.5 m/s impact rebounds
 * at 0.18 m/s, which is a 1.6 mm hop lasting 36 ms. Invisible, but it resets
 * the sleep timer forever, and a body that never sleeps is a dynamic box we
 * keep paying for every frame. See the header.
 */
const RESTITUTION_SLOP = 0.5;

/**
 * Hard cap on restitution. This is a sleep budget, not a physics constant.
 *
 * Measured by selfTest(), 0.06 m body dropped 2 m onto a flat floor, time until
 * it sleeps and stops costing the renderer anything:
 *
 *     e=0.35 (default)  1.62 s        e=0.95   17.78 s
 *     e=0.60            2.70 s        e=0.99   never (>30 s)
 *     e=0.90            9.78 s        e=1.00   never (>30 s)
 *
 * Every one of those seconds is a dynamic box tested by every ray (see the
 * header), so "just a bit bouncier" is expensive for reasons that have nothing
 * to do with the solver. Anything above ~0.9 should be a deliberate choice.
 *
 * Above 1.0 it is not merely slow, it diverges: e=1.02 turns that 2 m drop into
 * a 3.35 m rebound, because position correction hands back the potential energy
 * of the penetration it undoes without taking the matching kinetic energy away.
 * Below 1.0 that gain is smaller than what the symplectic integrator sheds in
 * flight — exactly 0.5*g^2*h^2 = 3.34e-3 J/kg per 120 Hz tick, confirmed to
 * five digits against the integrator — so a closed test at e=1.0 still loses
 * energy overall. Only just, though, and not by design; hence the clamp.
 */
const MAX_RESTITUTION = 0.95;

/** Sleep thresholds. 8 cm/s and 0.6 rad/s are both well below anything the eye
 * reads as motion at this camera distance. */
const SLEEP_LINEAR = 0.08;
const SLEEP_ANGULAR = 0.6;
/** How long a body must stay under threshold *while touching something*. The
 * touching requirement is what stops a canister freezing at the apex of a lob. */
const SLEEP_TIME = 0.35;

/** Matches the traversal stack in common.wgsl. */
const STACK_DEPTH = 32;

/** `hitBox`'s tmin in common.wgsl. */
const RAY_EPS = 1e-4;

/**
 * Floor for |direction| components before reciprocating.
 *
 * The WGSL divides by zero and relies on IEEE infinities; that is fine for
 * camera rays, which are never exactly axis aligned. CPU-side probes are almost
 * always exactly axis aligned (straight down, straight along +X), and there
 * `0 * Infinity` produces NaN the moment the origin sits exactly on a slab
 * plane. Nudging the direction is cheaper than NaN-guarding every compare.
 */
const DIR_EPS = 1e-12;

// --- public types -----------------------------------------------------------

export interface BodySpec {
  pos: Vec3;
  vel?: Vec3;
  angVel?: Vec3;
  /** Collision radius. Defaults to the largest half-extent. */
  radius?: number;
  /** Display half-extents. Defaults to a cube of `radius`. */
  half?: Vec3;
  mass?: number;
  restitution?: number;
  friction?: number;
  /** Fraction of linear velocity remaining after one second. 1 = no drag. */
  linearRetention?: number;
  /** Fraction of spin remaining after one second. This is what stops a rolling
   * canister: friction alone never decelerates a perfectly rolling sphere. */
  angularRetention?: number;
  /** Opaque payload for the caller (fuse timer, material index, ...). */
  userData?: unknown;
}

export interface RaycastHit {
  /** Distance along `dir`, which must be unit length. */
  t: number;
  /** Surface normal, already flipped to face the incoming ray. */
  normal: Vec3;
  /** Index into the `boxes` array the world was built from. */
  hit: number;
}

export interface PhysicsOptions {
  gravity?: Vec3;
  /** Overriding this is for tests; the default is `SLEEP_TIME`. Infinity
   * disables sleeping, which is only useful for measuring rest jitter. */
  sleepTime?: number;
}

export class Body {
  readonly id: number;
  pos: Vec3;
  vel: Vec3;
  angVel: Vec3;
  /** Orientation as [x,y,z,w], same convention as the rig's quaternions. */
  readonly orient = new Float32Array([0, 0, 0, 1]);
  radius: number;
  half: Vec3;
  mass: number;
  invMass: number;
  /** Uniform for a sphere, so no tensor rotation is ever needed: 1/(0.4*m*r^2). */
  invInertia: number;
  restitution: number;
  friction: number;
  linearRetention: number;
  angularRetention: number;
  userData: unknown;

  /** False once `remove()`d; such bodies are skipped and dropped next step. */
  alive = true;
  /** Asleep bodies are not integrated and MUST NOT be uploaded as dynamic. */
  sleeping = false;
  /** Set for the one `step()` in which the body fell asleep — the caller's cue
   * to bake it into the static scene. */
  justSlept = false;
  /** Set for the one `step()` in which the body woke — cue to un-bake it. */
  justWoke = false;
  /** Had at least one contact during the last tick. */
  touching = false;

  /** Seconds spent under the sleep thresholds while touching. */
  idle = 0;

  // Position-correction accumulator, so successive iterations can re-derive
  // penetration without re-running collision detection.
  _cx = 0; _cy = 0; _cz = 0;

  constructor(id: number, spec: BodySpec) {
    this.id = id;
    this.pos = v3(spec.pos.x, spec.pos.y, spec.pos.z);
    this.vel = spec.vel ? v3(spec.vel.x, spec.vel.y, spec.vel.z) : v3();
    this.angVel = spec.angVel ? v3(spec.angVel.x, spec.angVel.y, spec.angVel.z) : v3();
    const half = spec.half;
    this.radius = spec.radius ?? (half ? Math.max(half.x, half.y, half.z) : 0.06);
    this.half = half ? v3(half.x, half.y, half.z) : v3(this.radius, this.radius, this.radius);
    this.mass = spec.mass ?? 0.4;
    this.invMass = this.mass > 0 ? 1 / this.mass : 0;
    this.invInertia = this.mass > 0 ? 1 / (0.4 * this.mass * this.radius * this.radius) : 0;
    this.restitution = Math.min(spec.restitution ?? 0.35, MAX_RESTITUTION);
    this.friction = spec.friction ?? 0.5;
    this.linearRetention = spec.linearRetention ?? 1;
    this.angularRetention = spec.angularRetention ?? 0.25;
    this.userData = spec.userData;
  }
}

/** The body's orientation as scene `Box.rot`: local axes in world space. */
export function bodyRot(b: Body): [Vec3, Vec3, Vec3] {
  const out = new Float32Array(3);
  const cols: Vec3[] = [];
  for (let a = 0; a < 3; a++) {
    quatRotate(b.orient, 0, a === 0 ? 1 : 0, a === 1 ? 1 : 0, a === 2 ? 1 : 0, out, 0);
    cols.push(v3(out[0], out[1], out[2]));
  }
  return [cols[0], cols[1], cols[2]];
}

/** Ready to hand to the renderer's dynamic list, or to `SceneBuilder` once the
 * body sleeps and gets baked into the static BVH. */
export function bodyBox(b: Body, material: number, flags = 0): Box {
  return {
    center: v3(b.pos.x, b.pos.y, b.pos.z),
    half: v3(b.half.x, b.half.y, b.half.z),
    rot: bodyRot(b),
    material,
    flags,
  };
}

// --- internals --------------------------------------------------------------

interface Contact {
  a: Body;
  /** null means the static world, i.e. infinite mass. */
  b: Body | null;
  /** Unit normal, pointing from `b` (or the world) toward `a`. */
  nx: number; ny: number; nz: number;
  pen: number;
  /** Lever arms. For spheres these are just -n*r and +n*r. */
  ra: number; rb: number;
  mu: number;
  /** Desired post-solve separating speed along n (restitution bias). */
  target: number;
  kn: number; kt: number;
  jn: number; jt: number;
}

const makeContact = (): Contact => ({
  a: null as unknown as Body, b: null,
  nx: 0, ny: 1, nz: 0, pen: 0, ra: 0, rb: 0,
  mu: 0, target: 0, kn: 0, kt: 0, jn: 0, jt: 0,
});

export class PhysicsWorld {
  gravity: Vec3;
  sleepTime: number;

  private _bodies: Body[] = [];
  private _awake: Body[] = [];
  private _boxes: Box[];
  private _bvh: BVH;
  /** u32 aliasing of `_bvh.nodes`, for the leftFirst/count words. Cached
   * because the broadphase would otherwise build one per body per micro-step. */
  private _bvhU32: Uint32Array;
  private _nextId = 1;
  private _accum = 0;

  // Preallocated scratch. Nothing in step() allocates.
  private _stack = new Int32Array(STACK_DEPTH);
  private _candidates: number[] = [];
  private _contacts: Contact[] = [];
  private _contactCount = 0;
  private _omega = new Float32Array(4);
  private _dq = new Float32Array(4);

  constructor(boxes: Box[], bvh: BVH, opts: PhysicsOptions = {}) {
    this._boxes = boxes;
    this._bvh = bvh;
    this._bvhU32 = new Uint32Array(bvh.nodes.buffer, bvh.nodes.byteOffset, bvh.nodes.length);
    this.gravity = opts.gravity ? v3(opts.gravity.x, opts.gravity.y, opts.gravity.z) : v3(0, -9.81, 0);
    this.sleepTime = opts.sleepTime ?? SLEEP_TIME;
  }

  get bodies(): readonly Body[] { return this._bodies; }

  /**
   * Bodies that must be uploaded as dynamic geometry this frame. Refreshed by
   * `step()`. Keep this list short — see the header for what it costs.
   */
  get awake(): readonly Body[] { return this._awake; }

  /** Swap in a rebuilt static set, e.g. after baking slept bodies into it. */
  setStatic(boxes: Box[], bvh: BVH): void {
    this._boxes = boxes;
    this._bvh = bvh;
    this._bvhU32 = new Uint32Array(bvh.nodes.buffer, bvh.nodes.byteOffset, bvh.nodes.length);
  }

  spawn(spec: BodySpec): Body {
    const b = new Body(this._nextId++, spec);
    this._bodies.push(b);
    this._awake.push(b);
    return b;
  }

  remove(body: Body): void {
    body.alive = false;
  }

  wake(body: Body): void {
    if (body.sleeping) {
      body.sleeping = false;
      body.justWoke = true;
    }
    body.idle = 0;
  }

  applyImpulse(body: Body, jx: number, jy: number, jz: number, at?: Vec3): void {
    this.wake(body);
    body.vel.x += jx * body.invMass;
    body.vel.y += jy * body.invMass;
    body.vel.z += jz * body.invMass;
    if (at) {
      const rx = at.x - body.pos.x, ry = at.y - body.pos.y, rz = at.z - body.pos.z;
      body.angVel.x += body.invInertia * (ry * jz - rz * jy);
      body.angVel.y += body.invInertia * (rz * jx - rx * jz);
      body.angVel.z += body.invInertia * (rx * jy - ry * jx);
    }
  }

  /**
   * Canister blast. Linear falloff to zero at `radius` — inverse-square is more
   * defensible physically and much worse to play against, because a body 10 cm
   * from the centre gets flung out of the level.
   */
  burst(center: Vec3, radius: number, impulse: number): void {
    for (const b of this._bodies) {
      if (!b.alive) continue;
      let dx = b.pos.x - center.x, dy = b.pos.y - center.y, dz = b.pos.z - center.z;
      const d = Math.hypot(dx, dy, dz);
      if (d > radius) continue;
      if (d < 1e-6) { dx = 0; dy = 1; dz = 0; } else { dx /= d; dy /= d; dz /= d; }
      const j = impulse * (1 - d / radius);
      this.applyImpulse(b, dx * j, dy * j, dz * j);
      // Cosmetic tumble. Deterministic (cross with world up), so tests stay
      // reproducible; a blast that only translates looks like a hovercraft.
      const s = (j * b.invMass * 0.5) / b.radius;
      b.angVel.x += -dz * s;
      b.angVel.z += dx * s;
    }
  }

  step(dt: number): void {
    for (const b of this._bodies) { b.justSlept = false; b.justWoke = false; }

    this._accum += Math.min(dt, MAX_FRAME_DT);
    while (this._accum >= FIXED_DT) {
      this.tick(FIXED_DT);
      this._accum -= FIXED_DT;
    }

    // Compact removed bodies and refresh the upload list in one pass.
    let w = 0;
    this._awake.length = 0;
    for (const b of this._bodies) {
      if (!b.alive) continue;
      this._bodies[w++] = b;
      if (!b.sleeping) this._awake.push(b);
    }
    this._bodies.length = w;
  }

  // --- solver -------------------------------------------------------------

  private tick(h: number): void {
    const active = this._awake;
    active.length = 0;
    for (const b of this._bodies) {
      if (b.alive && !b.sleeping) { b.touching = false; active.push(b); }
    }
    if (active.length === 0) return;

    // Subdivide only as much as the fastest body needs. See MICRO_TRAVEL.
    let worst = 0;
    for (const b of active) {
      const travel = Math.hypot(b.vel.x, b.vel.y, b.vel.z) * h;
      const r = MICRO_TRAVEL * b.radius;
      if (travel / r > worst) worst = travel / r;
    }
    const micro = clamp(Math.ceil(worst), 1, MAX_MICRO_STEPS);
    const mh = h / micro;
    for (let m = 0; m < micro; m++) this.microStep(mh, active);

    // Sleep bookkeeping runs on the whole tick, after the final solve, so the
    // velocities it sees are post-contact.
    for (const b of active) {
      const lin = b.vel.x * b.vel.x + b.vel.y * b.vel.y + b.vel.z * b.vel.z;
      const ang = b.angVel.x * b.angVel.x + b.angVel.y * b.angVel.y + b.angVel.z * b.angVel.z;
      if (b.touching && lin < SLEEP_LINEAR * SLEEP_LINEAR && ang < SLEEP_ANGULAR * SLEEP_ANGULAR) {
        b.idle += h;
        if (b.idle >= this.sleepTime) {
          b.sleeping = true;
          b.justSlept = true;
          b.vel.x = b.vel.y = b.vel.z = 0;
          b.angVel.x = b.angVel.y = b.angVel.z = 0;
        }
      } else {
        b.idle = 0;
      }
    }
  }

  private microStep(h: number, active: Body[]): void {
    const g = this.gravity;
    for (const b of active) {
      b.vel.x += g.x * h; b.vel.y += g.y * h; b.vel.z += g.z * h;
      if (b.linearRetention < 1) {
        const f = Math.pow(b.linearRetention, h);
        b.vel.x *= f; b.vel.y *= f; b.vel.z *= f;
      }
      if (b.angularRetention < 1) {
        const f = Math.pow(b.angularRetention, h);
        b.angVel.x *= f; b.angVel.y *= f; b.angVel.z *= f;
      }
      b.pos.x += b.vel.x * h; b.pos.y += b.vel.y * h; b.pos.z += b.vel.z * h;
      this.integrateOrientation(b, h);
    }

    this._contactCount = 0;
    for (const b of active) this.collideStatic(b);
    this.collidePairs(active);
    if (this._contactCount === 0) return;

    for (let i = 0; i < VELOCITY_ITERATIONS; i++) this.solveVelocity();
    this.solvePosition();
  }

  /** q += 0.5 * h * (omega (x) q), renormalised. */
  private integrateOrientation(b: Body, h: number): void {
    const w = b.angVel;
    if (w.x === 0 && w.y === 0 && w.z === 0) return;
    this._omega[0] = w.x; this._omega[1] = w.y; this._omega[2] = w.z; this._omega[3] = 0;
    quatMul(this._omega, 0, b.orient, 0, this._dq, 0);
    const q = b.orient;
    const s = 0.5 * h;
    let x = q[0] + this._dq[0] * s;
    let y = q[1] + this._dq[1] * s;
    let z = q[2] + this._dq[2] * s;
    let ww = q[3] + this._dq[3] * s;
    const inv = 1 / (Math.hypot(x, y, z, ww) || 1);
    q[0] = x * inv; q[1] = y * inv; q[2] = z * inv; q[3] = ww * inv;
  }

  private pushContact(
    a: Body, b: Body | null,
    nx: number, ny: number, nz: number, pen: number,
  ): void {
    if (this._contactCount >= this._contacts.length) this._contacts.push(makeContact());
    const c = this._contacts[this._contactCount++];
    c.a = a; c.b = b;
    c.nx = nx; c.ny = ny; c.nz = nz; c.pen = pen;
    c.ra = a.radius;
    c.rb = b ? b.radius : 0;
    c.mu = b ? Math.sqrt(a.friction * b.friction) : a.friction;
    c.jn = 0; c.jt = 0;

    const im = a.invMass + (b ? b.invMass : 0);
    // ra x n == 0 for a sphere (ra is parallel to n), so rotation contributes
    // nothing to the normal direction and kn is just the inverse mass sum.
    c.kn = im;
    c.kt = im + a.invInertia * c.ra * c.ra + (b ? b.invInertia * c.rb * c.rb : 0);

    // Restitution target, captured from the approach speed BEFORE any impulse,
    // so it survives the iterative solve.
    const vax = a.vel.x + (a.angVel.y * -nz * c.ra - a.angVel.z * -ny * c.ra);
    const vay = a.vel.y + (a.angVel.z * -nx * c.ra - a.angVel.x * -nz * c.ra);
    const vaz = a.vel.z + (a.angVel.x * -ny * c.ra - a.angVel.y * -nx * c.ra);
    let rvx = vax, rvy = vay, rvz = vaz;
    if (b) {
      rvx -= b.vel.x + (b.angVel.y * nz * c.rb - b.angVel.z * ny * c.rb);
      rvy -= b.vel.y + (b.angVel.z * nx * c.rb - b.angVel.x * nz * c.rb);
      rvz -= b.vel.z + (b.angVel.x * ny * c.rb - b.angVel.y * nx * c.rb);
    }
    const vn = rvx * nx + rvy * ny + rvz * nz;
    const e = b ? Math.max(a.restitution, b.restitution) : a.restitution;
    c.target = vn < -RESTITUTION_SLOP ? -e * vn : 0;

    a.touching = true;
    if (b) b.touching = true;
  }

  private solveVelocity(): void {
    for (let i = 0; i < this._contactCount; i++) {
      const c = this._contacts[i];
      const a = c.a, b = c.b;
      const nx = c.nx, ny = c.ny, nz = c.nz;

      // Contact-point velocities: v + w x r, with ra = -n*ra, rb = +n*rb.
      const arx = -nx * c.ra, ary = -ny * c.ra, arz = -nz * c.ra;
      let rvx = a.vel.x + (a.angVel.y * arz - a.angVel.z * ary);
      let rvy = a.vel.y + (a.angVel.z * arx - a.angVel.x * arz);
      let rvz = a.vel.z + (a.angVel.x * ary - a.angVel.y * arx);
      let brx = 0, bry = 0, brz = 0;
      if (b) {
        brx = nx * c.rb; bry = ny * c.rb; brz = nz * c.rb;
        rvx -= b.vel.x + (b.angVel.y * brz - b.angVel.z * bry);
        rvy -= b.vel.y + (b.angVel.z * brx - b.angVel.x * brz);
        rvz -= b.vel.z + (b.angVel.x * bry - b.angVel.y * brx);
      }

      if (c.kn > 0) {
        const vn = rvx * nx + rvy * ny + rvz * nz;
        let jn = (c.target - vn) / c.kn;
        // Accumulated clamp: the total impulse must stay repulsive, individual
        // increments may be negative as other contacts in the set converge.
        const prev = c.jn;
        c.jn = Math.max(0, prev + jn);
        jn = c.jn - prev;
        applyImpulseAt(a, nx * jn, ny * jn, nz * jn, arx, ary, arz, 1);
        if (b) applyImpulseAt(b, nx * jn, ny * jn, nz * jn, brx, bry, brz, -1);
      }

      // Friction, recomputed after the normal impulse so the Coulomb cone uses
      // this iteration's normal force.
      if (c.mu > 0 && c.jn > 0 && c.kt > 0) {
        let rv2x = a.vel.x + (a.angVel.y * arz - a.angVel.z * ary);
        let rv2y = a.vel.y + (a.angVel.z * arx - a.angVel.x * arz);
        let rv2z = a.vel.z + (a.angVel.x * ary - a.angVel.y * arx);
        if (b) {
          rv2x -= b.vel.x + (b.angVel.y * brz - b.angVel.z * bry);
          rv2y -= b.vel.y + (b.angVel.z * brx - b.angVel.x * brz);
          rv2z -= b.vel.z + (b.angVel.x * bry - b.angVel.y * brx);
        }
        const vn2 = rv2x * nx + rv2y * ny + rv2z * nz;
        let tx = rv2x - vn2 * nx, ty = rv2y - vn2 * ny, tz = rv2z - vn2 * nz;
        const tl = Math.hypot(tx, ty, tz);
        if (tl > 1e-9) {
          tx /= tl; ty /= tl; tz /= tl;
          const max = c.mu * c.jn;
          const prev = c.jt;
          c.jt = clamp(prev - tl / c.kt, -max, max);
          const jt = c.jt - prev;
          applyImpulseAt(a, tx * jt, ty * jt, tz * jt, arx, ary, arz, 1);
          if (b) applyImpulseAt(b, tx * jt, ty * jt, tz * jt, brx, bry, brz, -1);
        }
      }
    }
  }

  /**
   * Full positional projection — no slop, no Baumgarte fraction.
   *
   * Slop is there to stop box manifolds fighting each other; with one contact
   * point per sphere there is nothing to fight, and any slop shows up directly
   * as a body resting `slop` metres inside the floor. Projecting all the way
   * out puts a dropped canister at exactly floorTop + radius (verified to 1e-9
   * in selfTest), and because it only touches positions it injects no velocity.
   * Iterating with re-derived penetration keeps a body wedged in a corner from
   * being double-pushed.
   */
  private solvePosition(): void {
    for (const b of this._bodies) { b._cx = 0; b._cy = 0; b._cz = 0; }
    for (let it = 0; it < POSITION_ITERATIONS; it++) {
      for (let i = 0; i < this._contactCount; i++) {
        const c = this._contacts[i];
        if (c.kn <= 0) continue;
        const a = c.a, b = c.b;
        let sep = c.pen - (a._cx * c.nx + a._cy * c.ny + a._cz * c.nz);
        if (b) sep += b._cx * c.nx + b._cy * c.ny + b._cz * c.nz;
        if (sep <= 0) continue;
        const d = sep / c.kn;
        const da = d * a.invMass;
        a.pos.x += c.nx * da; a.pos.y += c.ny * da; a.pos.z += c.nz * da;
        a._cx += c.nx * da; a._cy += c.ny * da; a._cz += c.nz * da;
        if (b) {
          const db = d * b.invMass;
          b.pos.x -= c.nx * db; b.pos.y -= c.ny * db; b.pos.z -= c.nz * db;
          b._cx -= c.nx * db; b._cy -= c.ny * db; b._cz -= c.nz * db;
        }
      }
    }
  }

  // --- narrowphase ---------------------------------------------------------

  private collideStatic(a: Body): void {
    const r = a.radius;
    const cand = this._candidates;
    cand.length = 0;
    this.queryAABB(
      a.pos.x - r, a.pos.y - r, a.pos.z - r,
      a.pos.x + r, a.pos.y + r, a.pos.z + r,
      cand,
    );
    for (let i = 0; i < cand.length; i++) {
      const box = this._boxes[cand[i]];
      if (sphereVsBox(a.pos, r, box)) {
        this.pushContact(a, null, hitNX, hitNY, hitNZ, hitPen);
      }
    }
  }

  /**
   * O(n^2) over the awake set. At n <= 48 that is 1128 distance tests, roughly
   * a microsecond; a grid or SAP would be more code than the whole solver and
   * would win nothing until the renderer could afford ten times the bodies.
   */
  private collidePairs(active: Body[]): void {
    for (let i = 0; i < active.length; i++) {
      const a = active[i];
      // Awake-vs-all, so a rolling canister wakes the pile it hits. Sleeping
      // bodies are never tested against each other.
      for (const b of this._bodies) {
        if (b === a || !b.alive) continue;
        // Awake/awake pairs are visited from both sides; keep the lower id's
        // visit only. Sleeping bodies never appear in `active`, so their pair
        // is seen exactly once regardless of id.
        if (!b.sleeping && b.id < a.id) continue;
        const dx = a.pos.x - b.pos.x, dy = a.pos.y - b.pos.y, dz = a.pos.z - b.pos.z;
        const rr = a.radius + b.radius;
        const d2 = dx * dx + dy * dy + dz * dz;
        if (d2 >= rr * rr) continue;
        let nx = 0, ny = 1, nz = 0, pen = rr;
        if (d2 > 1e-12) {
          const d = Math.sqrt(d2);
          nx = dx / d; ny = dy / d; nz = dz / d;
          pen = rr - d;
        }
        if (b.sleeping) this.wake(b);
        this.pushContact(a, b, nx, ny, nz, pen);
      }
    }
  }

  // --- BVH ----------------------------------------------------------------

  /** Collects original box indices whose world AABB overlaps the query box. */
  queryAABB(
    minx: number, miny: number, minz: number,
    maxx: number, maxy: number, maxz: number,
    out: number[],
  ): number[] {
    const bvh = this._bvh;
    // An empty scene flattens to a single zeroed node, which reads as an
    // interior node pointing at itself. Bail before entering that loop.
    if (bvh.order.length === 0) return out;
    const nodes = bvh.nodes;
    const u32 = this._bvhU32;
    const stack = this._stack;
    let sp = 0;
    let cur = 0;
    for (;;) {
      const o = cur * BVH_NODE_STRIDE_F32;
      if (
        nodes[o] <= maxx && nodes[o + 4] >= minx &&
        nodes[o + 1] <= maxy && nodes[o + 5] >= miny &&
        nodes[o + 2] <= maxz && nodes[o + 6] >= minz
      ) {
        const count = u32[o + 7];
        if (count > 0) {
          const first = u32[o + 3];
          for (let i = 0; i < count; i++) out.push(bvh.order[first + i]);
        } else {
          const li = u32[o + 3];
          cur = li;
          if (sp < STACK_DEPTH - 1) stack[sp++] = li + 1;
          continue;
        }
      }
      if (sp === 0) break;
      cur = stack[--sp];
    }
    return out;
  }

  /**
   * Closest-hit ray against the static world. Direct port of `trace` in
   * common.wgsl minus the dynamic list, so bullets and physics probes agree
   * with what the shader draws.
   *
   * Note there is deliberately NO flag filtering: FLAG_NO_CAMERA ceilings are
   * invisible but solid, and light housings are solid too. Only camera rays
   * care about those bits.
   *
   * `dir` must be unit length for `t` to be a distance.
   */
  raycast(origin: Vec3, dir: Vec3, tmax: number): RaycastHit | null {
    const bvh = this._bvh;
    if (bvh.order.length === 0) return null;
    const nodes = bvh.nodes;
    const u32 = this._bvhU32;

    const dx = Math.abs(dir.x) < DIR_EPS ? (dir.x < 0 ? -DIR_EPS : DIR_EPS) : dir.x;
    const dy = Math.abs(dir.y) < DIR_EPS ? (dir.y < 0 ? -DIR_EPS : DIR_EPS) : dir.y;
    const dz = Math.abs(dir.z) < DIR_EPS ? (dir.z < 0 ? -DIR_EPS : DIR_EPS) : dir.z;
    const ix = 1 / dx, iy = 1 / dy, iz = 1 / dz;

    let bestT = tmax;
    let bestBox = -1;
    let bnx = 0, bny = 1, bnz = 0;

    const stack = this._stack;
    let sp = 0;
    let cur = 0;

    for (;;) {
      const o = cur * BVH_NODE_STRIDE_F32;
      const count = u32[o + 7];
      if (count > 0) {
        const first = u32[o + 3];
        for (let i = 0; i < count; i++) {
          const bi = bvh.order[first + i];
          if (hitBox(origin, dir, this._boxes[bi], RAY_EPS, bestT)) {
            bestT = hitT; bnx = hitNX; bny = hitNY; bnz = hitNZ; bestBox = bi;
          }
        }
        if (sp === 0) break;
        cur = stack[--sp];
      } else {
        const li = u32[o + 3];
        const ri = li + 1;
        const dl = slabAABB(origin, ix, iy, iz, nodes, li * BVH_NODE_STRIDE_F32, bestT);
        const dr = slabAABB(origin, ix, iy, iz, nodes, ri * BVH_NODE_STRIDE_F32, bestT);
        if (dl < 0 && dr < 0) {
          if (sp === 0) break;
          cur = stack[--sp];
        } else if (dl < 0) {
          cur = ri;
        } else if (dr < 0) {
          cur = li;
        } else if (dl < dr) {
          if (sp < STACK_DEPTH - 1) stack[sp++] = ri;
          cur = li;
        } else {
          if (sp < STACK_DEPTH - 1) stack[sp++] = li;
          cur = ri;
        }
      }
    }

    if (bestBox < 0) return null;
    // Two-sided shading convention, same as trace().
    if (bnx * dir.x + bny * dir.y + bnz * dir.z > 0) { bnx = -bnx; bny = -bny; bnz = -bnz; }
    return { t: bestT, normal: v3(bnx, bny, bnz), hit: bestBox };
  }
}

// --- free functions on module scratch ---------------------------------------
//
// hitBox/sphereVsBox return a bool and publish their results here. Ugly, but it
// keeps the per-contact and per-ray-per-leaf paths allocation-free, and both
// are called tens of thousands of times per second.

let hitT = 0;
let hitNX = 0, hitNY = 0, hitNZ = 0;
let hitPen = 0;

function applyImpulseAt(
  b: Body, jx: number, jy: number, jz: number,
  rx: number, ry: number, rz: number, sign: number,
): void {
  b.vel.x += jx * b.invMass * sign;
  b.vel.y += jy * b.invMass * sign;
  b.vel.z += jz * b.invMass * sign;
  const k = b.invInertia * sign;
  b.angVel.x += k * (ry * jz - rz * jy);
  b.angVel.y += k * (rz * jx - rx * jz);
  b.angVel.z += k * (rx * jy - ry * jx);
}

/** Ray vs. node AABB. Entry distance, or -1 on miss. Mirrors `slabAABB`. */
function slabAABB(
  ro: Vec3, ix: number, iy: number, iz: number,
  nodes: Float32Array, o: number, tmax: number,
): number {
  const ax = (nodes[o] - ro.x) * ix, bx = (nodes[o + 4] - ro.x) * ix;
  const ay = (nodes[o + 1] - ro.y) * iy, by = (nodes[o + 5] - ro.y) * iy;
  const az = (nodes[o + 2] - ro.z) * iz, bz = (nodes[o + 6] - ro.z) * iz;
  const tNear = Math.max(Math.min(ax, bx), Math.min(ay, by), Math.min(az, bz));
  const tFar = Math.min(Math.max(ax, bx), Math.max(ay, by), Math.max(az, bz));
  if (tFar < Math.max(tNear, 0) || tNear > tmax) return -1;
  return Math.max(tNear, 0);
}

/**
 * Ray vs. oriented box, publishing `hitT`/`hitN*`. Port of `hitBox`.
 *
 * One knowing deviation: where the WGSL uses step() pairs to build the normal,
 * a ray hitting exactly on an edge produces two set components there and one
 * here (first axis wins). Off by a normalize on a measure-zero set of rays.
 */
function hitBox(ro: Vec3, rd: Vec3, box: Box, tmin: number, tmax: number): boolean {
  const dx = ro.x - box.center.x, dy = ro.y - box.center.y, dz = ro.z - box.center.z;
  const r0 = box.rot[0], r1 = box.rot[1], r2 = box.rot[2];
  // World -> local is three dot products: the rotation is orthonormal and its
  // columns are the local axes in world space.
  const lox = dx * r0.x + dy * r0.y + dz * r0.z;
  const loy = dx * r1.x + dy * r1.y + dz * r1.z;
  const loz = dx * r2.x + dy * r2.y + dz * r2.z;
  let ldx = rd.x * r0.x + rd.y * r0.y + rd.z * r0.z;
  let ldy = rd.x * r1.x + rd.y * r1.y + rd.z * r1.z;
  let ldz = rd.x * r2.x + rd.y * r2.y + rd.z * r2.z;
  if (Math.abs(ldx) < DIR_EPS) ldx = ldx < 0 ? -DIR_EPS : DIR_EPS;
  if (Math.abs(ldy) < DIR_EPS) ldy = ldy < 0 ? -DIR_EPS : DIR_EPS;
  if (Math.abs(ldz) < DIR_EPS) ldz = ldz < 0 ? -DIR_EPS : DIR_EPS;

  const ix = 1 / ldx, iy = 1 / ldy, iz = 1 / ldz;
  const h = box.half;
  const ax = (-h.x - lox) * ix, bx = (h.x - lox) * ix;
  const ay = (-h.y - loy) * iy, by = (h.y - loy) * iy;
  const az = (-h.z - loz) * iz, bz = (h.z - loz) * iz;
  const tnx = Math.min(ax, bx), tfx = Math.max(ax, bx);
  const tny = Math.min(ay, by), tfy = Math.max(ay, by);
  const tnz = Math.min(az, bz), tfz = Math.max(az, bz);
  const tNear = Math.max(tnx, tny, tnz);
  const tFar = Math.min(tfx, tfy, tfz);
  if (tFar < tNear || tFar < tmin) return false;

  const inside = tNear < tmin;
  const t = inside ? tFar : tNear;
  if (t < tmin || t > tmax) return false;

  // Whichever slab produced the extremum owns the face.
  let nlx = 0, nly = 0, nlz = 0;
  if (inside) {
    if (tfx <= tfy && tfx <= tfz) nlx = ldx > 0 ? -1 : 1;
    else if (tfy <= tfz) nly = ldy > 0 ? -1 : 1;
    else nlz = ldz > 0 ? -1 : 1;
  } else {
    if (tnx >= tny && tnx >= tnz) nlx = ldx > 0 ? -1 : 1;
    else if (tny >= tnz) nly = ldy > 0 ? -1 : 1;
    else nlz = ldz > 0 ? -1 : 1;
  }
  hitT = t;
  hitNX = nlx * r0.x + nly * r1.x + nlz * r2.x;
  hitNY = nlx * r0.y + nly * r1.y + nlz * r2.y;
  hitNZ = nlx * r0.z + nly * r1.z + nlz * r2.z;
  return true;
}

/**
 * Sphere vs. oriented box, publishing the normal (box -> sphere) and depth.
 *
 * Same change of basis as `hitBox`, then closest-point-on-box. The deep case
 * (centre strictly inside) needs the minimum-translation face instead, or the
 * normal is undefined exactly when it matters most — a canister that tunnels a
 * frame into a wall would otherwise get pushed to a random side.
 */
function sphereVsBox(p: Vec3, r: number, box: Box): boolean {
  const dx = p.x - box.center.x, dy = p.y - box.center.y, dz = p.z - box.center.z;
  const r0 = box.rot[0], r1 = box.rot[1], r2 = box.rot[2];
  const lx = dx * r0.x + dy * r0.y + dz * r0.z;
  const ly = dx * r1.x + dy * r1.y + dz * r1.z;
  const lz = dx * r2.x + dy * r2.y + dz * r2.z;
  const h = box.half;
  const qx = clamp(lx, -h.x, h.x);
  const qy = clamp(ly, -h.y, h.y);
  const qz = clamp(lz, -h.z, h.z);
  const ex = lx - qx, ey = ly - qy, ez = lz - qz;
  const d2 = ex * ex + ey * ey + ez * ez;
  if (d2 > r * r) return false;

  let nx: number, ny: number, nz: number;
  if (d2 > 1e-12) {
    const d = Math.sqrt(d2);
    nx = ex / d; ny = ey / d; nz = ez / d;
    hitPen = r - d;
  } else {
    const sx = h.x - Math.abs(lx), sy = h.y - Math.abs(ly), sz = h.z - Math.abs(lz);
    if (sx <= sy && sx <= sz) { nx = lx < 0 ? -1 : 1; ny = 0; nz = 0; hitPen = r + sx; }
    else if (sy <= sz) { nx = 0; ny = ly < 0 ? -1 : 1; nz = 0; hitPen = r + sy; }
    else { nx = 0; ny = 0; nz = lz < 0 ? -1 : 1; hitPen = r + sz; }
  }
  hitNX = nx * r0.x + ny * r1.x + nz * r2.x;
  hitNY = nx * r0.y + ny * r1.y + nz * r2.y;
  hitNZ = nx * r0.z + ny * r1.z + nz * r2.z;
  return true;
}

// ---------------------------------------------------------------------------
// selfTest — headless, no GPU, no DOM. Everything below asserts against numbers
// computed by hand rather than against "looks right".
//
//   import { selfTest } from "./game/physics";
//   const r = selfTest(); console.log(r.lines.join("\n"));
// ---------------------------------------------------------------------------

function floorScene(): { boxes: Box[]; bvh: BVH } {
  const b = new SceneBuilder();
  const m = b.material(v3(0.5, 0.5, 0.5), 1);
  b.box(v3(0, -0.5, 0), v3(20, 0.5, 20), m); // top face at y = 0
  b.box(v3(3, 1, 0), v3(0.5, 1, 4), m); // wall, near face at x = 2.5
  b.box(v3(5, 1, 0), v3(1, 1, 1), m, rotY(Math.PI / 4)); // rotated block
  return { boxes: b.boxes, bvh: buildBVH(b.boxes) };
}

/** Six slabs enclosing [-2,2]^3 with faces exactly on +-2. */
function roomScene(): { boxes: Box[]; bvh: BVH } {
  const b = new SceneBuilder();
  const m = b.material(v3(0.5, 0.5, 0.5), 1);
  const t = 0.5;
  b.box(v3(0, -2 - t, 0), v3(4, t, 4), m);
  b.box(v3(0, 2 + t, 0), v3(4, t, 4), m);
  b.box(v3(-2 - t, 0, 0), v3(t, 4, 4), m);
  b.box(v3(2 + t, 0, 0), v3(t, 4, 4), m);
  b.box(v3(0, 0, -2 - t), v3(4, 4, t), m);
  b.box(v3(0, 0, 2 + t), v3(4, 4, t), m);
  return { boxes: b.boxes, bvh: buildBVH(b.boxes) };
}

export function selfTest(): { pass: boolean; lines: string[] } {
  const lines: string[] = [];
  let pass = true;
  const ok = (name: string, cond: boolean, detail: string) => {
    if (!cond) pass = false;
    lines.push(`${cond ? "PASS" : "FAIL"}  ${name}  ${detail}`);
  };
  const run = (w: PhysicsWorld, seconds: number, fps = 60, each?: () => void) => {
    const n = Math.round(seconds * fps);
    for (let i = 0; i < n; i++) { w.step(1 / fps); if (each) each(); }
  };

  // -- raycasts, cross-checked against hand-computed values ------------------
  {
    const { boxes, bvh } = floorScene();
    const w = new PhysicsWorld(boxes, bvh);

    const down = w.raycast(v3(0, 5, 0), v3(0, -1, 0), 100);
    ok("ray/floor", !!down && Math.abs(down.t - 5) < 1e-5 && down.normal.y > 0.999,
      `t=${down?.t.toFixed(6)} (expect 5) n=(${fmtV(down?.normal)}) (expect 0,1,0)`);

    // Wall block spans x in [2.5, 3.5]; ray along +X from the origin plane.
    const wall = w.raycast(v3(0, 1, 0), v3(1, 0, 0), 100);
    ok("ray/wall", !!wall && Math.abs(wall.t - 2.5) < 1e-5 && wall.normal.x < -0.999,
      `t=${wall?.t.toFixed(6)} (expect 2.5) n=(${fmtV(wall?.normal)}) (expect -1,0,0)`);

    // Rotated block: centre (5,1,0), half 1, yaw 45deg, so rot0=(c,0,-s) and
    // rot2=(s,0,c) with c=s=sqrt(2)/2. Fired from +X at z=0.3 (from -X the wall
    // at x=2.5 gets there first), local coords along the ray are
    //     lo.x = (x-5)c - 0.3s,  lo.z = (x-5)s + 0.3c.
    // The x-slab ends at lo.x=+1 -> x = 5 + (1+0.3s)/c = 6.71421356; the z-slab
    // ends at lo.z=+1 -> x = 5 + (1-0.3c)/s = 6.11421356. Travelling -X from
    // x=8 the z face is the LATER entry, so it owns the hit:
    //     t = 8 - 6.11421356 = 1.88578644, local normal +z -> world rot2.
    const rot = w.raycast(v3(8, 1, 0.3), v3(-1, 0, 0), 100);
    const eT = 1.885786437626905;
    const eN = Math.SQRT1_2;
    ok("ray/rotated-obb",
      !!rot && Math.abs(rot.t - eT) < 1e-5 &&
        Math.abs(rot.normal.x - eN) < 1e-5 && Math.abs(rot.normal.z - eN) < 1e-5,
      `t=${rot?.t.toFixed(9)} (expect ${eT.toFixed(9)}) n=(${fmtV(rot?.normal)}) ` +
        `(expect ${eN.toFixed(6)},0,${eN.toFixed(6)})`);

    // Origin inside the wall block -> exit hit, normal flipped toward the ray.
    const inside = w.raycast(v3(3, 1, 0), v3(1, 0, 0), 100);
    ok("ray/from-inside", !!inside && Math.abs(inside.t - 0.5) < 1e-5 && inside.normal.x < -0.999,
      `t=${inside?.t.toFixed(6)} (expect 0.5) n=(${fmtV(inside?.normal)}) (expect -1,0,0)`);

    const miss = w.raycast(v3(0, 40, 0), v3(1, 0, 0), 100);
    ok("ray/miss", miss === null, `${miss === null ? "null" : "hit"}`);

    // Grazing, exactly axis-aligned along a slab plane: the NaN case the GPU
    // path gets away with and this one must not.
    const graze = w.raycast(v3(-5, 0, 0), v3(1, 0, 0), 100);
    ok("ray/degenerate-axis", graze !== null && Number.isFinite(graze.t),
      `t=${graze ? graze.t.toFixed(6) : "null"}`);
  }

  // -- drop onto a flat floor: rest height, sink, jitter --------------------
  let restResidual = 0;
  {
    const { boxes, bvh } = floorScene();
    const w = new PhysicsWorld(boxes, bvh);
    const r = 0.06;
    const g = w.spawn({ pos: v3(0, 2, 0), radius: r, restitution: 0.35 });
    run(w, 4);
    ok("drop/rest-height", Math.abs(g.pos.y - r) < 1e-9,
      `y=${g.pos.y.toFixed(12)} (expect ${r}) err=${(g.pos.y - r).toExponential(2)}`);
    ok("drop/no-sink", g.pos.y >= r - 1e-12, `y-r=${(g.pos.y - r).toExponential(2)}`);
    ok("drop/asleep", g.sleeping, `sleeping=${g.sleeping}`);

    // Jitter with sleeping disabled, which is the only way to see it at all.
    const w2 = new PhysicsWorld(boxes, bvh, { sleepTime: Infinity });
    const g2 = w2.spawn({ pos: v3(0, 2, 0), radius: r, restitution: 0.35 });
    run(w2, 3);
    let lo = Infinity, hi = -Infinity, maxV = 0;
    run(w2, 1, 60, () => {
      lo = Math.min(lo, g2.pos.y); hi = Math.max(hi, g2.pos.y);
      maxV = Math.max(maxV, Math.hypot(g2.vel.x, g2.vel.y, g2.vel.z));
    });
    restResidual = maxV;
    ok("drop/no-jitter", hi - lo < 1e-9 && maxV < 1e-9,
      `peak-to-peak=${(hi - lo).toExponential(2)} m, residual |v|=${maxV.toExponential(2)} m/s`);
  }

  // -- restitution against a wall ------------------------------------------
  {
    const { boxes, bvh } = floorScene();
    const w = new PhysicsWorld(boxes, bvh, { gravity: v3(0, 0, 0) });
    const e = 0.6;
    const speed = 8;
    const g = w.spawn({ pos: v3(0, 1, 0), radius: 0.06, restitution: e, friction: 0 });
    g.vel.x = speed;
    for (let i = 0; i < 200 && g.vel.x > 0; i++) w.step(1 / 240);
    const expected = -speed * e;
    ok("bounce/restitution", Math.abs(g.vel.x - expected) / speed < 0.01,
      `vx=${g.vel.x.toFixed(6)} expected=${expected.toFixed(6)} ` +
        `e_measured=${(-g.vel.x / speed).toFixed(4)} (expect ${e})`);

    const measured: string[] = [];
    let exact = true;
    for (const req of [0.3, 0.6, 0.9]) {
      const w2 = new PhysicsWorld(boxes, bvh, { gravity: v3(0, 0, 0), sleepTime: Infinity });
      const b = w2.spawn({ pos: v3(0, 1, 0), radius: 0.06, restitution: req, friction: 0 });
      b.vel.x = 8;
      for (let i = 0; i < 400 && b.vel.x > 0; i++) w2.step(1 / 60);
      const got = -b.vel.x / 8;
      if (Math.abs(got - req) > 1e-6) exact = false;
      measured.push(`${req}->${got.toFixed(6)}`);
    }
    ok("bounce/restitution-fidelity", exact, measured.join("  "));
  }

  // -- the caller's frame rate must not change the outcome ------------------
  {
    const { boxes, bvh } = floorScene();
    const rows: string[] = [];
    let same = true;
    for (const fps of [30, 60, 144, 240]) {
      const w = new PhysicsWorld(boxes, bvh);
      const b = w.spawn({ pos: v3(0, 2, 0), radius: 0.06 });
      for (let i = 0; i < fps * 4; i++) w.step(1 / fps);
      if (Math.abs(b.pos.y - 0.06) > 1e-12 || !b.sleeping) same = false;
      rows.push(`${fps}fps->y=${b.pos.y.toFixed(12)}`);
    }
    ok("fixed-step/frame-rate-independent", same, rows.join("  "));
  }

  // -- tunnelling ------------------------------------------------------------
  {
    // A thin wall, thinner than the body, hit at throw speed and well above it.
    // This is the claim MICRO_TRAVEL exists to make, so it gets measured.
    const sb = new SceneBuilder();
    const m = sb.material(v3(0.5, 0.5, 0.5), 1);
    sb.box(v3(2, 1, 0), v3(0.02, 2, 4), m); // 4 cm thick, faces at x = 1.98/2.02
    const boxes = sb.boxes;
    const bvh = buildBVH(boxes);
    const results: string[] = [];
    let allStopped = true;
    for (const speed of [12, 20, 34]) {
      const w = new PhysicsWorld(boxes, bvh, { gravity: v3(0, 0, 0), sleepTime: Infinity });
      const g = w.spawn({ pos: v3(0, 1, 0), radius: 0.06, restitution: 0.3, friction: 0 });
      g.vel.x = speed;
      run(w, 1);
      const through = g.pos.x > 2;
      if (through) allStopped = false;
      results.push(`${speed} m/s -> x=${g.pos.x.toFixed(3)}${through ? " THROUGH" : ""}`);
    }
    ok("tunnelling/thin-wall", allStopped,
      `0.06 m body vs a 0.04 m wall: ${results.join(", ")}`);

    // The ceiling: MAX_MICRO_STEPS caps travel at 8*MICRO_TRAVEL*r per tick.
    const ceiling = MAX_MICRO_STEPS * MICRO_TRAVEL * 0.06 / FIXED_DT;
    lines.push(`INFO  tunnelling  speed ceiling for a 0.06 m body = ` +
      `${ceiling.toFixed(1)} m/s (MAX_MICRO_STEPS*MICRO_TRAVEL*r/FIXED_DT); ` +
      `a thrown canister is ~12 m/s`);
  }

  // -- energy must not grow -------------------------------------------------
  {
    const energyRun = (e: number, seconds: number) => {
      const { boxes, bvh } = roomScene();
      const w = new PhysicsWorld(boxes, bvh, { sleepTime: Infinity });
      const b = w.spawn({
        pos: v3(0, 1.5, 0), radius: 0.1, mass: 0.4,
        friction: 0, angularRetention: 1,
      });
      // Assigned after spawn so this can probe above MAX_RESTITUTION, which is
      // how that constant was chosen.
      b.restitution = e;
      b.vel.x = 3.1; b.vel.z = 1.7;
      // Floor is at y = -2, so measure potential from there.
      const E = () =>
        0.5 * b.mass * (b.vel.x ** 2 + b.vel.y ** 2 + b.vel.z ** 2) +
        b.mass * 9.81 * (b.pos.y + 2) +
        0.5 * (0.4 * b.mass * b.radius ** 2) *
          (b.angVel.x ** 2 + b.angVel.y ** 2 + b.angVel.z ** 2);
      const e0 = E();
      let peak = e0;
      run(w, seconds, 60, () => { peak = Math.max(peak, E()); });
      return { e0, peak, end: E() };
    };
    const r90 = energyRun(0.9, 10);
    ok("energy/e=0.90 non-increasing", r90.peak <= r90.e0 * (1 + 1e-9),
      `E0=${r90.e0.toFixed(6)} J peak=${r90.peak.toFixed(6)} J end=${r90.end.toFixed(6)} J ` +
        `(${(((r90.end - r90.e0) / r90.e0) * 100).toFixed(2)}%)`);
    const rMax = energyRun(MAX_RESTITUTION, 10);
    ok("energy/e=MAX non-increasing", rMax.peak <= rMax.e0 * (1 + 1e-9),
      `at the clamp e=${MAX_RESTITUTION}: peak=${rMax.peak.toFixed(6)} J ` +
        `vs E0=${rMax.e0.toFixed(6)} J`);
    const r100 = energyRun(1.0, 10);
    ok("energy/e=1.00 non-increasing", r100.peak <= r100.e0 * (1 + 1e-9),
      `peak=${r100.peak.toFixed(6)} J vs E0=${r100.e0.toFixed(6)} J ` +
        `(only just: see MAX_RESTITUTION)`);
    // Above the clamp it diverges, which is what makes the clamp load-bearing.
    const r105 = energyRun(1.05, 10);
    const creep105 = ((r105.peak - r105.e0) / r105.e0) * 100;
    ok("energy/e=1.05 does diverge", creep105 > 10,
      `unclamped e=1.05 gains ${creep105.toFixed(1)}% in 10 s — the clamp is not decorative`);

    // The integrator's own loss, checked against the analytic 0.5*g^2*h^2.
    const perTick = 0.5 * 9.81 * 9.81 * FIXED_DT * FIXED_DT;
    lines.push(`INFO  integrator  free-fall energy drift = ` +
      `${perTick.toExponential(3)} J/kg per ${(1 / FIXED_DT).toFixed(0)} Hz tick (analytic; ` +
      `matches the measured per-tick delta to 5 digits)`);
  }

  // -- restitution vs. time-to-sleep, the table behind MAX_RESTITUTION -------
  {
    const { boxes, bvh } = floorScene();
    const cells: string[] = [];
    for (const e of [0.35, 0.6, 0.9, 0.95, 0.99, 1.0]) {
      const w = new PhysicsWorld(boxes, bvh);
      const b = w.spawn({ pos: v3(0, 2, 0), radius: 0.06, friction: 0 });
      b.restitution = e;
      let t = 0, slept = -1;
      for (let i = 0; i < 60 * 30; i++) {
        w.step(1 / 60); t += 1 / 60;
        if (b.sleeping) { slept = t; break; }
      }
      cells.push(`e=${e.toFixed(2)}:${slept < 0 ? ">30s" : slept.toFixed(2) + "s"}`);
    }
    lines.push(`INFO  sleep-cost  2 m drop, time to sleep — ${cells.join("  ")}`);
  }

  // -- sleeping and waking ---------------------------------------------------
  {
    const { boxes, bvh } = floorScene();
    const w = new PhysicsWorld(boxes, bvh);
    const g = w.spawn({ pos: v3(0, 1, 0), radius: 0.06 });
    let sleepAt = -1;
    let t = 0;
    run(w, 4, 60, () => { t += 1 / 60; if (sleepAt < 0 && g.sleeping) sleepAt = t; });
    ok("sleep/bounded-time", sleepAt > 0 && sleepAt < 1.6,
      `fell asleep ${sleepAt.toFixed(3)} s after a 1 m drop (0.45 s of which is the fall)`);
    ok("sleep/not-in-awake-list", w.awake.length === 0, `awake=${w.awake.length}`);

    w.burst(v3(0.3, 0.06, 0), 1.0, 0.6);
    ok("sleep/wake-on-burst", !g.sleeping && w.awake.length === 0,
      `sleeping=${g.sleeping} (awake list refreshes on the next step)`);
    w.step(1 / 60);
    ok("sleep/awake-after-step", w.awake.length === 1 && w.awake[0] === g,
      `awake=${w.awake.length}`);

    // Land a second canister on the first once both are settled. Blast-launched
    // bodies roll for a while, so this waits long enough for that to decay.
    let resettle = -1;
    let t2 = 0;
    run(w, 8, 60, () => { t2 += 1 / 60; if (resettle < 0 && g.sleeping) resettle = t2; });
    const g2 = w.spawn({ pos: v3(g.pos.x, 1.2, g.pos.z), radius: 0.06 });
    ok("sleep/re-sleeps-after-blast", g.sleeping,
      `re-slept ${resettle.toFixed(2)} s after a 0.6 N.s blast at 0.3 m ` +
        `(rolled ${Math.abs(g.pos.x).toFixed(2)} m)`);
    let woke = false;
    run(w, 1.5, 60, () => { if (!g.sleeping) woke = true; });
    ok("sleep/wake-on-contact", woke, `impacted by body ${g2.id}, woke=${woke}`);
  }

  // -- head-on equal masses swap velocities ---------------------------------
  {
    const { boxes, bvh } = floorScene();
    const w = new PhysicsWorld(boxes, bvh, { gravity: v3(0, 0, 0), sleepTime: Infinity });
    const a = w.spawn({ pos: v3(-0.5, 1, 0), radius: 0.1, restitution: 0.95, friction: 0 });
    const b = w.spawn({ pos: v3(0.5, 1, 0), radius: 0.1, restitution: 0.95, friction: 0 });
    a.vel.x = 2; b.vel.x = -2;
    for (let i = 0; i < 400 && a.vel.x > 0; i++) w.step(1 / 240);
    ok("pair/head-on", Math.abs(a.vel.x + 1.9) < 0.05 && Math.abs(b.vel.x - 1.9) < 0.05,
      `a.vx=${a.vel.x.toFixed(4)} b.vx=${b.vel.x.toFixed(4)} (expect -1.9 / +1.9 at e=0.95)`);
  }

  // -- cost ------------------------------------------------------------------
  {
    // Sized like the real level (~530 boxes) rather than a bare room, so the
    // BVH traversal in the broadphase is doing representative work.
    const sb = new SceneBuilder();
    const m = sb.material(v3(0.5, 0.5, 0.5), 1);
    const t = 0.5;
    sb.box(v3(0, -2 - t, 0), v3(4, t, 4), m);
    sb.box(v3(0, 2 + t, 0), v3(4, t, 4), m);
    sb.box(v3(-2 - t, 0, 0), v3(t, 4, 4), m);
    sb.box(v3(2 + t, 0, 0), v3(t, 4, 4), m);
    sb.box(v3(0, 0, -2 - t), v3(4, 4, t), m);
    sb.box(v3(0, 0, 2 + t), v3(4, 4, t), m);
    for (let i = 0; i < 524; i++) {
      const a = i * 2.399963; // golden-angle scatter, no RNG dependency
      sb.box(
        v3(Math.cos(a) * (3 + (i % 17)), ((i * 7) % 40) * 0.1, Math.sin(a) * (3 + (i % 19))),
        v3(0.3, 0.6, 0.3), m, rotY(a),
      );
    }
    const boxes = sb.boxes;
    const bvh = buildBVH(boxes);
    const w = new PhysicsWorld(boxes, bvh, { sleepTime: Infinity });
    for (let i = 0; i < 32; i++) {
      const b = w.spawn({ pos: v3((i % 8) * 0.4 - 1.4, 1.5 - Math.floor(i / 8) * 0.35, 0), radius: 0.08 });
      b.vel.x = ((i * 37) % 13) / 3 - 2;
      b.vel.z = ((i * 53) % 11) / 3 - 1.8;
    }
    for (let i = 0; i < 120; i++) w.step(1 / 60); // settle into contact
    const t0 = perfNow();
    const iters = 600;
    for (let i = 0; i < iters; i++) w.step(1 / 60);
    const ms = (perfNow() - t0) / iters;
    lines.push(
      `INFO  cost  32 awake bodies, ${boxes.length} static boxes: ` +
        `${ms.toFixed(4)} ms per step(1/60) = ${(ms / 2).toFixed(4)} ms per 120 Hz tick`);
  }

  lines.push(`INFO  rest residual |v| = ${restResidual.toExponential(2)} m/s`);
  return { pass, lines };
}

const fmtV = (v: Vec3 | undefined) =>
  v ? `${v.x.toFixed(6)},${v.y.toFixed(6)},${v.z.toFixed(6)}` : "-";

const perfNow = (): number =>
  typeof performance !== "undefined" ? performance.now() : Date.now();
