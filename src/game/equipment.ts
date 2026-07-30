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
 * How far off the cursor a lamp can sit and still be caught, in world units at
 * the lamp's depth.
 *
 * Generous, because the pulse is an area effect and the camera is far enough
 * out that demanding pixel accuracy would be fiddly rather than skilful.
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
 * Light nearest the cursor, measured against the ray the camera casts through
 * it. Equivalently: the lamp whose drawn position on screen is closest to the
 * crosshair.
 *
 * Targeting here was wrong twice, both times by inventing a direction instead
 * of using the one the view already defines.
 *
 * First the OCP traced a pulse with a fixed vertical rise, so it always met
 * the ceiling about six metres out whatever the player pointed at. Then it
 * used the aim point on the ground plane, which fixed the OCP and looked
 * right. It was not right: everything above the floor draws at a screen
 * position offset from the point beneath it, so pointing at a lamp puts the
 * ground-plane aim somewhere else entirely. Measured over the 28 lights in
 * this level, that error runs to 2.99m and is worst on the ceiling fixtures.
 * Only 12 of 28 fell inside the bullet's 0.55m radius — while 24 fell inside
 * the OCP's 2.2m, which is exactly why the OCP appeared to work and shooting
 * a light out never did.
 *
 * The camera ray has no such error by construction. Range is still measured
 * from the shooter in 3D, and `blocked` keeps them from shooting through a
 * wall.
 */
export function nearestLightOnCursor(
  lights: Light[], count: number,
  eye: Vec3, cursorDir: Vec3, shooter: Vec3,
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
    // Perpendicular distance from the lamp to the cursor ray.
    const ex = l.pos.x - eye.x, ey = l.pos.y - eye.y, ez = l.pos.z - eye.z;
    const t = ex * cursorDir.x + ey * cursorDir.y + ez * cursorDir.z;
    if (t <= 0) continue;
    const px = ex - cursorDir.x * t, py = ey - cursorDir.y * t, pz = ez - cursorDir.z * t;
    const off = Math.hypot(px, py, pz);
    if (off > radius) continue;
    // Range belongs to the weapon, so it is measured from the muzzle.
    const dx = l.pos.x - shooter.x, dy = l.pos.y - shooter.y, dz = l.pos.z - shooter.z;
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
  /**
   * Starts on the pistol.
   *
   * Empty hands is still the more honest opening for a stealth game, but the
   * first thing a new player does is look for what they have; starting with
   * nothing in frame makes the demo look featureless. Slot 1 is one key away.
   */
  active = 1;
  /** 0..1; the OCP is unusable below 1. */
  ocpCharge = 1;

  private readonly disabled: Disabled[] = [];
  /** Lights shot out for good — remembered only so a restart can undo them. */
  private readonly shot: Array<{ index: number; intensity: number; mat: number }> = [];

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
    eye: Vec3, cursorDir: Vec3,
    materials?: Material[],
    blocked?: (at: Vec3) => boolean,
  ): number | null {
    if (!this.ocpReady) return null;
    const best = nearestLightOnCursor(
      lights, staticCount, eye, cursorDir, origin, OCP_RANGE, OCP_CONE_RADIUS,
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
   * Takes the cursor ray for the same reason the OCP does. This used to search
   * around the bullet's impact point, which sounds right and cannot work: the
   * shot is fired nearly level, at torso height, so it lands on a wall well
   * below or beside the fixture. No round ever landed near a lamp.
   *
   * The radius is far tighter than the OCP's. The pulse is an area effect and
   * can afford to be generous; a bullet should mean you actually pointed at it.
   *
   * Returns the light index and its fixture material, or null if nothing was
   * close enough to where the player was aiming.
   */
  shootOut(
    lights: Light[], staticCount: number, origin: Vec3,
    eye: Vec3, cursorDir: Vec3,
    blocked?: (at: Vec3) => boolean,
  ): { index: number; mat: number } | null {
    const i = nearestLightOnCursor(
      lights, staticCount, eye, cursorDir, origin, SHOT_RANGE, SHOT_HIT_RADIUS,
      this.disabled, blocked,
    );
    if (i < 0) return null;
    const mat = lights[i].emissiveMat ?? -1;
    this.shot.push({ index: i, intensity: lights[i].intensity, mat });
    // Zeroed on the CPU-side copy too, so nearestLight stops finding it and the
    // gameplay light probe stops counting a light that no longer exists.
    lights[i].intensity = 0;
    return { index: i, mat };
  }

  /**
   * Every light back on and the OCP full — the restart, which promises the
   * level's opening frame. `restore` puts both the light and its fixture back;
   * the CPU-side intensity of a shot-out lamp is the only original this class
   * had to remember, since shooting zeroed it.
   */
  reset(
    lights: Light[], materials: Material[],
    restore: (
      index: number, intensity: number,
      mat: number, emissive: [number, number, number] | null,
    ) => void,
  ): void {
    for (const d of this.disabled) restore(d.index, d.intensity, d.mat, d.emissive);
    this.disabled.length = 0;
    for (const s of this.shot) {
      lights[s.index].intensity = s.intensity;
      const e = s.mat >= 0 ? materials[s.mat].emissive : null;
      restore(s.index, s.intensity, s.mat, e ? [e.x, e.y, e.z] : null);
    }
    this.shot.length = 0;
    this.ocpCharge = 1;
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
