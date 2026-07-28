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
  smoke: number;
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

  /**
   * Muzzle smoke: a small puff pushed along the barrel, rising and expanding.
   *
   * Not emissive, deliberately. The muzzle flash is a real light for ~3 frames,
   * and smoke that is lit by it — bright at the instant of the shot, then dim
   * grey drifting in the dark — is worth far more than smoke that glows on its
   * own.
   */
  smoke(pos: Vec3, dir: Vec3, n = 5): void {
    for (let i = 0; i < n; i++) {
      const p = this.emit();
      if (!p) return;
      const spread = 0.28;
      p.x = pos.x; p.y = pos.y; p.z = pos.z;
      // A short muzzle blast, then it stops travelling and rises. Heavy
      // horizontal drag kills the forward push in about a fifth of a second,
      // and negative gravity is buoyancy — smoke hangs and climbs, it does not
      // keep flying down the barrel.
      p.vx = dir.x * (0.45 + Math.random() * 0.35) + (Math.random() - 0.5) * spread;
      p.vy = 0.15 + Math.random() * 0.2;
      p.vz = dir.z * (0.45 + Math.random() * 0.35) + (Math.random() - 0.5) * spread;
      p.h = p.h0 = 0.012 + Math.random() * 0.014;
      p.age = 0;
      p.life = 0.7 + Math.random() * 0.6;
      p.mat = this.mats.smoke;
      p.flags = 0;
      p.drag = 4.2;
      p.gravity = -0.85;
      p.grow = 3.6;
      p.yaw = Math.random() * TAU;
      p.spin = (Math.random() - 0.5) * 1.5;
    }
  }

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
   * Bullet hitting the world: chips of the surface plus a small dust puff.
   *
   * Two behaviours from one call, because a wall hit reads wrong with either
   * alone — chips without dust look like confetti, dust without chips looks
   * like the round hit fog. The dust is buoyant like muzzle smoke, so it hangs
   * where the round struck instead of sliding down the wall.
   */
  debris(pos: Vec3, normal: Vec3, n = 7): void {
    for (let i = 0; i < n; i++) {
      const p = this.emit();
      if (!p) return;
      const dust = i >= n - 2;
      p.x = pos.x + normal.x * 0.02;
      p.y = pos.y + normal.y * 0.02;
      p.z = pos.z + normal.z * 0.02;
      const s = dust ? 0.25 + Math.random() * 0.3 : 1.2 + Math.random() * 2.4;
      p.vx = (normal.x + (Math.random() - 0.5) * 1.3) * s;
      p.vy = (normal.y + Math.random() * 0.9) * s;
      p.vz = (normal.z + (Math.random() - 0.5) * 1.3) * s;
      p.h = p.h0 = dust ? 0.014 + Math.random() * 0.014 : 0.006 + Math.random() * 0.009;
      p.age = 0;
      p.life = dust ? 0.5 + Math.random() * 0.4 : 0.3 + Math.random() * 0.35;
      p.mat = dust ? this.mats.smoke : this.mats.debris;
      p.flags = 0;
      p.drag = dust ? 3.4 : 1.0;
      p.gravity = dust ? -0.5 : 11;
      p.grow = dust ? 2.8 : 1;
      p.yaw = Math.random() * TAU;
      p.spin = (Math.random() - 0.5) * (dust ? 1.2 : 12);
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
