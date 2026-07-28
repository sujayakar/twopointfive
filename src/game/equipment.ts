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

/** How far the pulse carries. */
const OCP_RANGE = 22;
/**
 * How far the cursor can sit from under a lamp and still catch it.
 *
 * Generous, because the player is pointing at a floor position roughly under
 * the fixture rather than at the fixture itself.
 */
const OCP_CONE_RADIUS = 2.2;
/**
 * Tighter for a bullet than for the OCP pulse.
 *
 * The pulse is an area effect and can be generous; a bullet is a point and
 * should require actually hitting the fixture.
 */
export const SHOT_HIT_RADIUS = 0.55;

/** How far a round will reach for a fixture; matches the world trace. */
const SHOT_RANGE = 45;

/**
 * Light nearest to where the player is pointing, measured on the floor plane.
 *
 * Targeting here has been wrong twice, in two different ways, and both came
 * from trying to turn a top-down cursor into a 3D direction.
 *
 * First the OCP traced a pulse with a fixed vertical rise and looked for lamps
 * near the impact, so it always met the ceiling about six metres out whatever
 * the player pointed at. Replacing that with perpendicular distance to a ray
 * aimed at ceiling height fixed the OCP and looked right — but only because
 * its catch radius is generous. The fixtures in this level sit at 1.13, 1.4,
 * 2.35 and 3.06 metres; a ray aimed at a hardcoded 3.0 passes over a metre
 * clear of most of them, which is why bullets, with a much tighter radius,
 * could only ever hit the single true ceiling light.
 *
 * So height is not guessed at all now. The camera is overhead and aiming lands
 * on the floor, so the honest question is "which lamp is the player pointing
 * at", and that is a horizontal distance. Range is still measured from the
 * shooter in 3D, and `blocked` keeps them from shooting through a wall.
 */
export function nearestLightToAim(
  lights: Light[], count: number, origin: Vec3, aimX: number, aimZ: number,
  maxDist: number, radius: number,
  skip?: { index: number }[],
  blocked?: (at: Vec3) => boolean,
): number {
  let best = -1;
  let bestScore = Infinity;
  for (let i = 0; i < count; i++) {
    // The moon is index 0 and hangs outside the building. Shooting it out would
    // be absurd, and it would also strand its dedicated key-light channel.
    if (i === 0) continue;
    if (skip?.some((d) => d.index === i)) continue;
    const l = lights[i];
    if (l.intensity <= 0) continue;
    // How far the cursor is from being under the lamp.
    const ox = l.pos.x - aimX, oz = l.pos.z - aimZ;
    const off = Math.hypot(ox, oz);
    if (off > radius) continue;
    const dx = l.pos.x - origin.x, dy = l.pos.y - origin.y, dz = l.pos.z - origin.z;
    const dist = Math.hypot(dx, dy, dz);
    if (dist > maxDist) continue;
    // Prefer the best-aimed, then the nearest.
    const score = off + dist * 0.02;
    if (score < bestScore && !blocked?.(l.pos)) { bestScore = score; best = i; }
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
   * Fires the OCP at whatever the player is pointing at.
   *
   * Generous on purpose: the pulse is an area effect, and the camera is far
   * enough out that demanding pixel accuracy on a ceiling fixture would be
   * frustrating rather than skilful. The bullet is the precise one.
   *
   * Returns the disabled light's index, or null if nothing was in range.
   */
  fireOCP(
    lights: Light[], staticCount: number, origin: Vec3,
    aimX: number, aimZ: number,
    materials?: Material[],
    blocked?: (at: Vec3) => boolean,
  ): number | null {
    if (!this.ocpReady) return null;
    const best = nearestLightToAim(
      lights, staticCount, origin, aimX, aimZ, OCP_RANGE, OCP_CONE_RADIUS,
      this.disabled, blocked,
    );
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
   * Takes the aim point for the same reason the OCP does. This used to search
   * around the bullet's impact point, which sounds right and cannot work: the
   * shot is fired nearly level, at torso height, so it lands on a wall well
   * below or beside the fixture. No round ever landed near a lamp, which is
   * why shooting lights out never worked even after the OCP started to.
   *
   * The radius is far tighter than the OCP's. The pulse is an area effect and
   * can afford to be generous; a bullet should mean you actually pointed at it.
   *
   * Returns the light index and its fixture material, or null if nothing was
   * close enough to where the player was aiming.
   */
  shootOut(
    lights: Light[], staticCount: number, origin: Vec3,
    aimX: number, aimZ: number,
    blocked?: (at: Vec3) => boolean,
  ): { index: number; mat: number } | null {
    const i = nearestLightToAim(
      lights, staticCount, origin, aimX, aimZ, SHOT_RANGE, SHOT_HIT_RADIUS,
      this.disabled, blocked,
    );
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
