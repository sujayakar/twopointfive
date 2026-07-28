/**
 * How visible the player is, from the light actually reaching them.
 *
 * Fed by the GPU light probe, which evaluates illuminance at the player's chest
 * against the same lights, BVH and occlusion test the frame was rendered with.
 * That equivalence is the point: in a stealth game the player has to be able to
 * trust their eyes, and a CPU-side approximation would eventually disagree with
 * the image about whether a given shadow is real.
 */

/**
 * Illuminance mapped to a 0..1 meter.
 *
 * These are the two numbers that decide whether the game feels fair, so they
 * are deliberately explicit rather than buried in a curve. `dark` is the level
 * the corridor floor sits at with only the moon on it; `lit` is roughly what
 * standing directly under a fluorescent reads. Both want re-checking with the
 * in-game readout after any change to light intensities or the exposure.
 */
export const VISIBILITY_RANGE = { dark: 0.30, lit: 4.0 };

/**
 * Measured with the probe at chest height, moon + practicals only, no
 * flashlight (illuminance in the renderer's arbitrary units, not real lux):
 *
 *   cubicle farm      0.41      darkest open floor
 *   dark corridor     0.58
 *   server room       0.91
 *   conference room   2.13
 *   corridor east     2.23      brightest static spot
 *   muzzle flash      3.47      at ~1m, for reference
 *
 * `dark` sits just under the darkest measured floor so genuinely dark corners
 * read as zero, and `lit` above the brightest static spot so there is headroom
 * for the flashlight and muzzle flashes to peg the meter. Re-measure after any
 * change to light intensities — the HUD readout prints raw illuminance for
 * exactly this purpose.
 */

/**
 * Smoothing time constants, in seconds.
 *
 * Asymmetric on purpose. Becoming lit should register almost immediately —
 * stepping into a beam is the player's own doing and hiding the consequence
 * feels like a cheat. Becoming dark again is slower, so a guard's torch
 * sweeping past you leaves a moment of exposure rather than flicking straight
 * back to safe. It also filters the probe's own frame-to-frame noise, which
 * matters because a single sample of a jittered area light is not smooth.
 */
const RISE_TAU = 0.05;
const FALL_TAU = 0.35;

export class Visibility {
  /** Smoothed 0..1. 0 is invisible, 1 is fully lit. */
  level = 0;
  /** Unsmoothed, for debugging and tuning. */
  raw = 0;
  /** Raw illuminance straight from the probe, before any mapping. */
  illuminance = 0;

  update(illuminance: number, dt: number): void {
    // The probe reports 0 before the first readback lands; treat that as dark
    // rather than letting a NaN or an undefined leak into the meter.
    const lux = Number.isFinite(illuminance) ? Math.max(illuminance, 0) : 0;
    this.illuminance = lux;

    const { dark, lit } = VISIBILITY_RANGE;
    // Log-ish response. Perceived brightness is roughly logarithmic, and a
    // linear map spends almost the whole meter on the brightest few percent of
    // the level while every interesting stealth decision happens near black.
    const t = (Math.log(lux + dark) - Math.log(dark)) / (Math.log(lit + dark) - Math.log(dark));
    this.raw = Math.min(Math.max(t, 0), 1);

    const tau = this.raw > this.level ? RISE_TAU : FALL_TAU;
    // Frame-rate independent exponential smoothing.
    const a = 1 - Math.exp(-dt / tau);
    this.level += (this.raw - this.level) * a;
  }

  /** Coarse band, for the HUD and eventually for guard detection ranges. */
  get band(): "hidden" | "dim" | "exposed" {
    if (this.level < 0.25) return "hidden";
    if (this.level < 0.6) return "dim";
    return "exposed";
  }

  /** Twenty-cell bar, the readout this is actually tuned against. */
  meter(): string {
    const filled = Math.round(this.level * 20);
    return "[" + "|".repeat(filled) + "·".repeat(20 - filled) + "]";
  }
}
