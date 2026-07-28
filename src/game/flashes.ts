import { Vec3, v3 } from "../core/math";
import { LIGHT_SPHERE, Light } from "../scene/scene";

/**
 * Short-lived lights: muzzle flashes, and later grenade detonations.
 *
 * These are ordinary scene lights with a lifetime, so they light the room
 * through the same path as everything else — next-event estimation, ReSTIR
 * reuse, and an indirect bounce. That is the entire point: a gunshot in a dark
 * office should throw light onto the ceiling and back off the far wall, which
 * an additive screen-space sprite cannot do.
 *
 * ---------------------------------------------------------------------------
 * Why the duration is a free choice again
 *
 * These lights are traced into their own signal, which is spatially filtered
 * but never temporally accumulated. So the renderer no longer has an opinion
 * about how long a flash lasts: it appears and disappears exactly with the
 * light.
 *
 * That was not true before. While transients shared the steady signal, a flash
 * shorter than a few frames was swallowed by the 48-frame history, and one long
 * enough to survive left a glow hanging for nearly a second afterwards. Two
 * workarounds came and went — stretching the envelope, then braking the
 * denoiser's history — before splitting the signal made both unnecessary.
 * The duration below is now chosen for how it looks, not for what the
 * accumulator can represent.
 * ---------------------------------------------------------------------------
 */

/** Muzzle flash defaults, tuned against the scene's other lights. */
export const MUZZLE = {
  /** Seconds. ~3 frames at 60Hz — short enough to read as a snap. */
  duration: 0.05,
  /**
   * Inverse-square units, like every other light here. A gunshot genuinely does
   * out-blast a room's practicals by orders of magnitude, and the AgX highlight
   * rolloff keeps that from simply clipping to white.
   */
  intensity: 5200,
  /** Slightly warm and unsaturated — burning propellant, not a fire. */
  color: v3(1.0, 0.86, 0.62),
  /** Small but not a point, so the shadows it throws have a believable edge. */
  radius: 0.09,
};

/** Grenade detonation: bigger, slower, and much warmer. */
export const DETONATION = {
  duration: 0.35,
  intensity: 42000,
  color: v3(1.0, 0.72, 0.36),
  radius: 0.7,
};

interface Transient {
  light: Light;
  /** Seconds elapsed. */
  age: number;
  life: number;
  peak: number;
}

export class Flashes {
  private readonly items: Transient[] = [];
  /** Reused across frames; callers must not retain it. */
  private readonly out: Light[] = [];

  get count(): number {
    return this.items.length;
  }

  /**
   * Adds a flash at `pos`.
   *
   * Emitted as a sphere light rather than a spot: a muzzle flash lights the
   * shooter and the walls beside them as much as whatever they are pointing at,
   * and that side-spill is most of what sells it.
   */
  spawn(
    pos: Vec3,
    spec: { duration: number; intensity: number; color: Vec3; radius: number } = MUZZLE,
  ): void {
    this.items.push({
      light: {
        pos: v3(pos.x, pos.y, pos.z),
        kind: LIGHT_SPHERE,
        dir: v3(0, -1, 0),
        radius: spec.radius,
        color: v3(spec.color.x, spec.color.y, spec.color.z),
        intensity: 0,
        cosInner: -1,
        cosOuter: -1,
      },
      age: 0,
      life: spec.duration,
      peak: spec.intensity,
    });
  }

  update(dt: number): void {
    for (let i = this.items.length - 1; i >= 0; i--) {
      const it = this.items[i];
      it.age += dt;
      if (it.age >= it.life) {
        this.items.splice(i, 1);
        continue;
      }
      // Fast attack, sharp decay. A symmetric envelope reads as a soft pulse;
      // the asymmetry is what makes it read as a bang.
      const u = it.age / it.life;
      const env = u < 0.15 ? u / 0.15 : Math.pow(1 - (u - 0.15) / 0.85, 4.0);
      it.light.intensity = it.peak * env;
    }
  }

  lights(): Light[] {
    this.out.length = 0;
    for (const it of this.items) this.out.push(it.light);
    return this.out;
  }
}
