import { Vec3 } from "../core/math";

// ---------------------------------------------------------------------------
// Volumetric smoke puffs.
//
// The particle system already throws lit smoke *boxes*; these are different —
// they are regions of participating medium, visible only where light actually
// scatters through them: a torch beam cutting a gunshot's smoke, a muzzle
// flash blooming from inside its own cloud. The two layers are deliberately
// complementary: boxes read at any lighting, the medium only exists in light,
// and together they cover both the "there is smoke" and the "the air is
// thick" halves of the effect.
//
// Puffs travel to the shader as a fixed-size uniform array (MAX_PUFFS slots,
// radius <= 0 marks an empty slot) and modulate the density term of the
// volumetric march in pathtrace.wgsl. There is no occlusion or self-shadowing
// in the medium — at these sizes and lifetimes the eye reads drift, growth
// and fade, not transport accuracy.
// ---------------------------------------------------------------------------

/** Must match MAX_PUFFS in common.wgsl. */
export const MAX_PUFFS = 8;

/** Floats per puff across the two vec4 arrays the shader sees. */
const FLOATS = MAX_PUFFS * 8;

interface Puff {
  x: number; y: number; z: number;
  vx: number; vy: number; vz: number;
  /** Radius at birth and at end of life; growth eases between them. */
  r0: number;
  r1: number;
  /** Peak density multiplier; fades with age. */
  strength: number;
  age: number;
  life: number;
  /** Noise offset so two puffs never share internal structure. */
  seed: number;
}

export class Smoke {
  private puffs: Puff[] = [];
  /** Packed shader block, reused every frame. */
  readonly packed = new Float32Array(FLOATS);

  /** A pistol shot's muzzle cloud: small, quick, thrown forward. */
  muzzle(pos: Vec3, dir: Vec3): void {
    this.spawn({
      x: pos.x + dir.x * 0.3, y: pos.y + dir.y * 0.3, z: pos.z + dir.z * 0.3,
      vx: dir.x * 0.55, vy: 0.35 + dir.y * 0.2, vz: dir.z * 0.55,
      r0: 0.25, r1: 1.15, strength: 1.5, age: 0, life: 2.4,
      seed: Math.random() * 100,
    });
  }

  /** Dust kicked off an impact point: slower, larger, longer-lived. */
  impact(at: Vec3): void {
    this.spawn({
      x: at.x, y: at.y, z: at.z,
      vx: 0, vy: 0.28, vz: 0,
      r0: 0.3, r1: 1.6, strength: 1.7, age: 0, life: 3.2,
      seed: Math.random() * 100,
    });
  }

  /** A detonation's cloud: the big one. */
  detonation(at: Vec3): void {
    this.spawn({
      x: at.x, y: at.y + 0.4, z: at.z,
      vx: 0, vy: 0.5, vz: 0,
      r0: 0.9, r1: 3.4, strength: 2.6, age: 0, life: 5.5,
      seed: Math.random() * 100,
    });
  }

  private spawn(p: Puff): void {
    if (this.puffs.length < MAX_PUFFS) {
      this.puffs.push(p);
      return;
    }
    // Oldest-fraction-wins, same policy as the particle pool: a fresh burst
    // must always be visible, and the puff it evicts was fading anyway.
    let worst = 0;
    let worstFrac = -1;
    for (let i = 0; i < this.puffs.length; i++) {
      const frac = this.puffs[i].age / this.puffs[i].life;
      if (frac > worstFrac) { worstFrac = frac; worst = i; }
    }
    this.puffs[worst] = p;
  }

  update(dt: number): void {
    for (let i = this.puffs.length - 1; i >= 0; i--) {
      const p = this.puffs[i];
      p.age += dt;
      if (p.age >= p.life) {
        this.puffs.splice(i, 1);
        continue;
      }
      // Smoke decelerates as it spreads; the drag constant is a look choice.
      const drag = Math.exp(-dt * 1.4);
      p.vx *= drag; p.vy *= drag; p.vz *= drag;
      p.x += p.vx * dt;
      p.y += p.vy * dt;
      p.z += p.vz * dt;
    }

    // Repack. Slot order does not matter to the shader; empty slots carry
    // radius 0 which it skips.
    this.packed.fill(0);
    for (let i = 0; i < this.puffs.length; i++) {
      const p = this.puffs[i];
      const t = p.age / p.life;
      // Fast early growth that settles — smoke expands hardest right away.
      const grow = 1 - (1 - t) * (1 - t);
      const radius = p.r0 + (p.r1 - p.r0) * grow;
      // Density thins as the same material fills a growing volume; the extra
      // (1-t) makes the tail fade out rather than pop at end of life.
      const strength = p.strength * (1 - t) * (1 - t * 0.5);
      const base = i * 4;
      this.packed[base + 0] = p.x;
      this.packed[base + 1] = p.y;
      this.packed[base + 2] = p.z;
      this.packed[base + 3] = radius;
      const pbase = MAX_PUFFS * 4 + i * 4;
      this.packed[pbase + 0] = strength;
      this.packed[pbase + 1] = t;
      this.packed[pbase + 2] = p.seed;
    }
  }
}
