import { Vec3 } from "../core/math";
import { Light, Material } from "../scene/scene";

// ---------------------------------------------------------------------------
// Equipment: what the number keys select.
//
// Two items for now, and they are deliberately opposites. The pistol is loud,
// permanent and finite. The OCP is silent, temporary and free — it does not
// remove a guard, it removes the *light*, which is the resource this whole game
// is actually about.
// ---------------------------------------------------------------------------

export type SlotId = "none" | "pistol" | "ocp";

export interface SlotInfo {
  id: SlotId;
  label: string;
}

/**
 * Empty hands is a real slot, not the absence of one.
 *
 * It is also the starting state: a stealth game that opens with the gun already
 * out has told you what it thinks you should do with it.
 */
export const SLOTS: SlotInfo[] = [
  { id: "none", label: "HANDS" },
  { id: "pistol", label: "FIVE-SEVEN" },
  { id: "ocp", label: "OCP" },
];

/**
 * Seconds a light stays dark.
 *
 * Long enough to cross a room and do something, short enough that you cannot
 * simply switch the level off and walk through it. The real tension is that it
 * is *temporary* — you are on a clock the moment you fire it.
 */
export const OCP_DURATION = 18;

/** Recharge, so the OCP is a rhythm rather than a toggle. */
export const OCP_RECHARGE = 6;

/** How far from the ray's hit point a light can be and still count as struck. */
const OCP_HIT_RADIUS = 1.6;
/**
 * Tighter for a bullet than for the OCP pulse.
 *
 * The pulse is an area effect and can be generous; a bullet is a point and
 * should require actually hitting the fixture.
 */
export const SHOT_HIT_RADIUS = 0.55;

/**
 * Nearest live light to a point, ignoring any already out.
 *
 * Fixtures are geometry and lights are a separate list, so "did I hit that
 * lamp" is really "what light is nearest to where this landed".
 */
export function nearestLight(
  lights: Light[], count: number, at: Vec3, radius: number,
  skip?: { index: number }[],
): number {
  let best = -1;
  let bestD2 = radius * radius;
  for (let i = 0; i < count; i++) {
    // The moon is index 0 and hangs outside the building. Shooting it out would
    // be absurd, and it would also strand its dedicated key-light channel.
    if (i === 0) continue;
    if (skip?.some((d) => d.index === i)) continue;
    const l = lights[i];
    if (l.intensity <= 0) continue;
    const dx = l.pos.x - at.x, dy = l.pos.y - at.y, dz = l.pos.z - at.z;
    const d2 = dx * dx + dy * dy + dz * dz;
    if (d2 < bestD2) { bestD2 = d2; best = i; }
  }
  return best;
}

interface Disabled {
  /** Index into the scene's static light array. */
  index: number;
  intensity: number;
  /** The fixture's material and its original glow, restored with the light. */
  mat: number;
  emissive: [number, number, number] | null;
  remaining: number;
  pos: Vec3;
  /** Seconds until the next spark sputter from this fixture. */
  nextSpark: number;
}

export class Equipment {
  active = 0;
  /** 0..1; the OCP is unusable below 1. */
  ocpCharge = 1;

  private readonly disabled: Disabled[] = [];

  get slot(): SlotId {
    return SLOTS[this.active].id;
  }

  select(i: number): void {
    if (i >= 0 && i < SLOTS.length) this.active = i;
  }

  get ocpReady(): boolean {
    return this.ocpCharge >= 1;
  }

  /**
   * Fires the OCP at whatever the ray struck.
   *
   * Finds the nearest light to the impact point rather than tracing to a light
   * directly: fixtures are geometry and lights are a separate list, so "did I
   * hit that lamp" is really "did I hit something near that lamp".
   *
   * Returns the disabled light's index, or null if nothing was in range.
   */
  fireOCP(
    lights: Light[], staticCount: number, at: Vec3, materials?: Material[],
  ): number | null {
    if (!this.ocpReady) return null;
    const best = nearestLight(lights, staticCount, at, OCP_HIT_RADIUS, this.disabled);
    if (best < 0) return null;
    this.ocpCharge = 0;
    const mat = lights[best].emissiveMat ?? -1;
    const e = mat >= 0 && materials ? materials[mat].emissive : null;
    this.disabled.push({
      index: best,
      intensity: lights[best].intensity,
      mat,
      emissive: e ? [e.x, e.y, e.z] : null,
      remaining: OCP_DURATION,
      pos: lights[best].pos,
      nextSpark: 0,
    });
    return best;
  }

  /**
   * Advances timers. `restore` fires when a light comes back, with everything
   * needed to put both the light and its fixture back the way they were.
   */
  update(
    dt: number,
    restore: (
      index: number, intensity: number,
      mat: number, emissive: [number, number, number] | null,
    ) => void,
    spark?: (at: Vec3, burst: boolean) => void,
  ): void {
    if (this.ocpCharge < 1) {
      this.ocpCharge = Math.min(1, this.ocpCharge + dt / OCP_RECHARGE);
    }
    for (let i = this.disabled.length - 1; i >= 0; i--) {
      const d = this.disabled[i];
      d.remaining -= dt;
      if (d.remaining <= 0) {
        restore(d.index, d.intensity, d.mat, d.emissive);
        this.disabled.splice(i, 1);
        continue;
      }
      // A dead fixture sputters. It is also the only cue that a light is out
      // *temporarily* rather than simply being a dark part of the level, which
      // matters because the player is on a clock the moment they fire.
      d.nextSpark -= dt;
      if (spark && d.nextSpark <= 0) {
        const first = d.remaining > OCP_DURATION - 0.05;
        spark(d.pos, first);
        // Irregular on purpose; a metronome reads as an animation, not a fault.
        d.nextSpark = 0.45 + Math.random() * 1.5;
      }
    }
  }

  /**
   * Shoots a light out, permanently.
   *
   * The counterpart to the OCP, and deliberately its opposite: loud, costs a
   * round, and never comes back. The OCP buys you eighteen seconds and a clock;
   * a bullet buys you the room for good, and announces you doing it.
   *
   * Returns the light index and its fixture material, or null if nothing was
   * close enough to the impact.
   */
  shootOut(
    lights: Light[], staticCount: number, at: Vec3,
  ): { index: number; mat: number } | null {
    const i = nearestLight(lights, staticCount, at, SHOT_HIT_RADIUS, this.disabled);
    if (i < 0) return null;
    // Zeroed on the CPU-side copy too, so nearestLight stops finding it and the
    // gameplay light probe stops counting a light that no longer exists.
    lights[i].intensity = 0;
    return { index: i, mat: lights[i].emissiveMat ?? -1 };
  }

  /** The fixture material for a just-disabled light, or -1 if it has none. */
  matFor(lightIndex: number): number {
    const d = this.disabled.find((x) => x.index === lightIndex);
    return d ? d.mat : -1;
  }

  /** Seconds until the next light comes back, or null if none are out. */
  get soonestRestore(): number | null {
    let t: number | null = null;
    for (const d of this.disabled) t = t === null ? d.remaining : Math.min(t, d.remaining);
    return t;
  }

  get darkCount(): number {
    return this.disabled.length;
  }
}
