import { Vec3 } from "../core/math";
import { BOX_STRIDE_F32, FLAG_EMISSIVE } from "../scene/scene";

// ---------------------------------------------------------------------------
// Particles, as boxes.
//
// Everything else in this world is an oriented box, so particles are too — and
// at this camera distance a small cube reads better than a billboard would.
// They go through the dynamic box list, which means they are lit and shadowed
// like anything else: muzzle smoke catches the flash that made it, and sparks
// off a dead fluorescent light the ceiling around them.
//
// That also sets the budget. Dynamic boxes are tested linearly by every ray, so
// this is capped hard — see MAX_PARTICLES. Sparks are emissive-flagged, which
// makes them free of shadow-ray cost; smoke and blood are not, because being
// lit is the entire point of them.
// ---------------------------------------------------------------------------

/**
 * Hard cap on live particles.
 *
 * 120 dynamic boxes measured ~1.6ms of trace time with per-character group
 * rejection, so 48 is roughly 0.6ms at worst and usually far less, since
 * particles cluster tightly and one group AABB rejects the lot.
 */
export const MAX_PARTICLES = 48;

export interface ParticleMaterials {
  blood: number;
  spark: number;
  debris: number;
}

interface P {
  x: number; y: number; z: number;
  vx: number; vy: number; vz: number;
  /** Half-extent; particles are cubes. */
  h: number;
  h0: number;
  age: number;
  life: number;
  mat: number;
  flags: number;
  /** Fraction of velocity retained per second. */
  drag: number;
  gravity: number;
  /** Grows toward this multiple of h0 over the particle's life. */
  grow: number;
  /** Static yaw, so a cube does not read as axis-aligned with the world. */
  yaw: number;
  spin: number;
}

const TAU = Math.PI * 2;

export class Particles {
  private readonly pool: P[] = [];
  private count = 0;

  constructor(private mats: ParticleMaterials) {
    for (let i = 0; i < MAX_PARTICLES; i++) {
      this.pool.push({
        x: 0, y: 0, z: 0, vx: 0, vy: 0, vz: 0, h: 0, h0: 0,
        age: 0, life: 0, mat: 0, flags: 0, drag: 1, gravity: 0, grow: 1,
        yaw: 0, spin: 0,
      });
    }
  }

  get live(): number {
    return this.count;
  }

  private emit(): P | null {
    // Oldest-wins rather than dropping: a burst should always be visible, and
    // the particle it replaces is the one closest to expiring anyway.
    if (this.count < MAX_PARTICLES) return this.pool[this.count++];
    let worst = 0;
    let worstAge = -1;
    for (let i = 0; i < this.count; i++) {
      const r = this.pool[i].age / this.pool[i].life;
      if (r > worstAge) { worstAge = r; worst = i; }
    }
    return this.pool[worst];
  }

  // Box smoke is gone: muzzle and impact smoke are volumetric puffs now
  // (src/game/smoke.ts) — a medium the beams actually scatter through, where
  // a drifting grey cube only ever pretended. Sparks, blood and chips stay
  // boxes: they are objects, not air.

  /**
   * Bullet impact: a spray of dark red away from the surface.
   *
   * This is the hit indicator. At this camera distance a guard going down takes
   * 2.4 seconds, which is far too slow to answer "did I hit"; the spray answers
   * it on the same frame as the shot.
   */
  impact(pos: Vec3, away: Vec3, n = 9): void {
    for (let i = 0; i < n; i++) {
      const p = this.emit();
      if (!p) return;
      p.x = pos.x; p.y = pos.y; p.z = pos.z;
      const s = 1.4 + Math.random() * 2.6;
      p.vx = (away.x + (Math.random() - 0.5) * 1.1) * s;
      p.vy = (away.y + Math.random() * 0.8) * s;
      p.vz = (away.z + (Math.random() - 0.5) * 1.1) * s;
      p.h = p.h0 = 0.008 + Math.random() * 0.012;
      p.age = 0;
      p.life = 0.35 + Math.random() * 0.4;
      p.mat = this.mats.blood;
      p.flags = 0;
      p.drag = 1.2;
      p.gravity = 9.0;
      p.grow = 1;
      p.yaw = Math.random() * TAU;
      p.spin = (Math.random() - 0.5) * 9;
    }
  }

  /**
   * Bullet hitting the world: chips of the surface.
   *
   * Chips alone used to read as confetti, so this once carried two buoyant
   * dust boxes too. The dust half is now the volumetric impact puff spawned
   * alongside this call — real hanging air the beam can catch — and the chips
   * keep answering "the round hit *there*".
   */
  debris(pos: Vec3, normal: Vec3, n = 5): void {
    for (let i = 0; i < n; i++) {
      const p = this.emit();
      if (!p) return;
      p.x = pos.x + normal.x * 0.02;
      p.y = pos.y + normal.y * 0.02;
      p.z = pos.z + normal.z * 0.02;
      const s = 1.2 + Math.random() * 2.4;
      p.vx = (normal.x + (Math.random() - 0.5) * 1.3) * s;
      p.vy = (normal.y + Math.random() * 0.9) * s;
      p.vz = (normal.z + (Math.random() - 0.5) * 1.3) * s;
      p.h = p.h0 = 0.006 + Math.random() * 0.009;
      p.age = 0;
      p.life = 0.3 + Math.random() * 0.35;
      p.mat = this.mats.debris;
      p.flags = 0;
      p.drag = 1.0;
      p.gravity = 11;
      p.grow = 1;
      p.yaw = Math.random() * TAU;
      p.spin = (Math.random() - 0.5) * 12;
    }
  }

  /**
   * Electrical sparks off a dead fixture.
   *
   * Emissive, so they light the ceiling around them and cost no shadow rays —
   * and because a spark that is not itself a light source looks like a speck of
   * orange paint.
   */
  sparks(pos: Vec3, n = 6): void {
    for (let i = 0; i < n; i++) {
      const p = this.emit();
      if (!p) return;
      p.x = pos.x; p.y = pos.y; p.z = pos.z;
      const s = 1.0 + Math.random() * 2.4;
      p.vx = (Math.random() - 0.5) * 2 * s;
      p.vy = -Math.random() * 0.8 * s;
      p.vz = (Math.random() - 0.5) * 2 * s;
      p.h = p.h0 = 0.006 + Math.random() * 0.008;
      p.age = 0;
      p.life = 0.18 + Math.random() * 0.3;
      p.mat = this.mats.spark;
      p.flags = FLAG_EMISSIVE;
      p.drag = 1.6;
      p.gravity = 7.0;
      p.grow = 1;
      p.yaw = Math.random() * TAU;
      p.spin = (Math.random() - 0.5) * 14;
    }
  }

  update(dt: number): void {
    for (let i = this.count - 1; i >= 0; i--) {
      const p = this.pool[i];
      p.age += dt;
      if (p.age >= p.life) {
        // Swap-remove, so the live prefix stays contiguous for packing.
        this.pool[i] = this.pool[this.count - 1];
        this.pool[this.count - 1] = p;
        this.count--;
        continue;
      }
      const k = Math.exp(-p.drag * dt);
      p.vx *= k; p.vz *= k;
      p.vy = (p.vy - p.gravity * dt) * k;
      p.x += p.vx * dt;
      p.y += p.vy * dt;
      p.z += p.vz * dt;
      // Nothing here collides; particles simply stop at the floor. Real contact
      // would mean a ray cast per particle per frame for an effect that lasts
      // half a second.
      // Buoyant particles have no floor to hit and must not be pinned by it.
      if (p.gravity > 0 && p.y < 0.01) {
        p.y = 0.01; p.vy = 0; p.vx *= 0.3; p.vz *= 0.3;
      }
      p.yaw += p.spin * dt;
      const t = p.age / p.life;
      p.h = p.h0 * (1 + (p.grow - 1) * t);
    }
  }

  /** Packs live particles into `out` starting `offset` boxes in. */
  buildBoxes(out: Float32Array, offset: number): number {
    const capacity = Math.floor(out.length / BOX_STRIDE_F32) - offset;
    const n = Math.min(this.count, capacity);
    const u = new Uint32Array(out.buffer, out.byteOffset, out.length);
    for (let i = 0; i < n; i++) {
      const p = this.pool[i];
      const o = (offset + i) * BOX_STRIDE_F32;
      const c = Math.cos(p.yaw), s = Math.sin(p.yaw);
      out[o] = p.x; out[o + 1] = p.y; out[o + 2] = p.z;
      u[o + 3] = p.mat;
      out[o + 4] = p.h; out[o + 5] = p.h; out[o + 6] = p.h;
      u[o + 7] = p.flags;
      // Yaw only: a tumbling cube needs a full basis, and at three pixels
      // across nobody can tell the difference.
      out[o + 8] = c; out[o + 9] = 0; out[o + 10] = -s; out[o + 11] = 0;
      out[o + 12] = 0; out[o + 13] = 1; out[o + 14] = 0; out[o + 15] = 0;
      out[o + 16] = s; out[o + 17] = 0; out[o + 18] = c; out[o + 19] = 0;
    }
    return n;
  }
}
