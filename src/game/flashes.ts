import { Vec3, v3 } from "../core/math";
import { LIGHT_SPHERE, Light } from "../scene/scene";

/**
 * Short-lived lights: muzzle flashes, and later canister bursts.
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

/**
 * Flashbang: the brightest thing in the level, by a long way, and white.
 *
 * Not BURST with the numbers turned up — the colour is the point. A canister
 * burst is burning propellant and reads warm; a flashbang is a magnesium
 * charge and reads as a bleach-white wall of light with no hue at all. Making
 * it warm makes it look like an explosion instead of a stun charge.
 *
 * Duration is chosen for legibility rather than physics: the real device is a
 * millisecond, which at 60 Hz would land inside a single frame and read as one
 * stuttered white frame. Seven frames is long enough to see the room light up
 * and the shadows swing before it goes.
 */
export const FLASHBANG = {
  duration: 0.12,
  /**
   * ~35x a muzzle flash. It genuinely should out-blast every practical in the
   * building at once; the AgX highlight rolloff is what keeps that from being
   * a flat white rectangle.
   */
  intensity: 180000,
  color: v3(1.0, 0.98, 0.95),
  /** Large enough that the shadows it throws have a visibly soft edge. */
  radius: 0.45,
};

/** Canister burst: bigger, slower, and much warmer. */
export const BURST = {
  duration: 0.35,
  intensity: 42000,
  color: v3(1.0, 0.72, 0.36),
  radius: 0.7,
};

/**
 * Mean of the decay envelope over one frame, rather than a point sample of it.
 *
 * This is what makes every shot look the same. A flash lasts ~3 frames, so
 * point-sampling its envelope at whatever instant each frame happens to fall on
 * is close to sampling it at random: catch the peak and it is blinding, miss it
 * and the same shot is a dud. There is no frame rate at which that stops being
 * luck, because the rise of a real muzzle flash is shorter than any frame.
 *
 * Averaging over the frame's own interval instead means a flash delivers the
 * same energy however the frames happen to land — which is also just what a
 * light that exists for part of a frame physically does.
 *
 * Envelope is (1-u)^4: no attack ramp, because a muzzle flash rises in
 * microseconds. The ramp that used to be here existed to help the temporal
 * denoiser track the flash, and transient lights are no longer accumulated.
 *
 *   mean over [u0,u1] of (1-u)^4  =  ((1-u0)^5 - (1-u1)^5) / (5 (u1-u0))
 */
function meanEnvelope(u0: number, u1: number): number {
  if (u1 <= u0) return Math.pow(1 - u0, 4);
  const a = Math.pow(1 - u0, 5);
  const b = Math.pow(1 - u1, 5);
  return (a - b) / (5 * (u1 - u0));
}

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
      const u0 = it.age / it.life;
      it.age += dt;
      if (u0 >= 1) {
        this.items.splice(i, 1);
        continue;
      }
      const u1 = Math.min(it.age / it.life, 1);
      it.light.intensity = it.peak * meanEnvelope(u0, u1);
    }
  }

  lights(): Light[] {
    this.out.length = 0;
    for (const it of this.items) this.out.push(it.light);
    return this.out;
  }
}
