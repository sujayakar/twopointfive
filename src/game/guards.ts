import { Vec3, angleDelta, clamp, lerpAngle, v3 } from "../core/math";
import { Rig } from "../anim/rig";
import { Character, CharacterMaterials } from "./character";
import { BOX_STRIDE_F32, LIGHT_SPOT, Light } from "../scene/scene";

// ---------------------------------------------------------------------------
// Patrolling guards.
//
// Deliberately not AI: they are moving light sources that are also moving
// shadow casters, which is the one thing a static scene cannot show off. A
// guard walking past a cubicle farm sweeps a hard-edged beam across twenty
// partitions and drags its own silhouette along the floor behind it, and every
// bit of that falls out of the tracer rather than being authored.
//
// The rig is shared state (Rig.computeWorld writes into rig.worldPos/worldRot),
// so a pose is only valid between the Character.update that produced it and the
// next one from anybody else. Everything here re-poses immediately before it
// reads. See the note on Guards.buildBoxes.
// ---------------------------------------------------------------------------

/** A closed loop of 2D waypoints. The last point connects back to the first. */
export interface PatrolRoute {
  waypoints: Array<[number, number]>;
  /** Ground speed in m/s. Defaults to a normal walking pace. */
  speed?: number;
}

/** The clip every guard walks. Formal, unhurried — reads as a patrol, not a chase. */
const CLIP = "Walk_Formal_Loop";
/** Stood still and looking. The pistol grip is layered over it as usual. */
const IDLE_CLIP = "Pistol_Idle_Loop";
/**
 * Death. The library has no takedown or grab clips at all, so this plus the hit
 * reaction is the whole vocabulary available for a guard going down.
 */
const DEATH_CLIP = "Death01";
/** Played first when taken down: the victim's half of the staged melee. */
const KNOCKBACK_CLIP = "Hit_Knockback";
const KNOCKBACK_LEN = 0.83;

/**
 * How long a guard stops and looks toward a gunshot.
 *
 * Long enough to read as "something happened over there" and to make the shot
 * feel loud, short enough that it is a beat rather than a state you have to
 * wait out. Deliberately not an investigation: nobody walks over, nobody
 * searches. A guard who hunts you needs pathfinding and a whole failure state
 * to go with it, and neither is what this demo is about.
 */
const ALERT_LEN = 3.2;
/** Beyond this a shot is not worth reacting to. */
const ALERT_RANGE = 34;
/** How quickly an alerted guard snaps around. Faster than a patrol turn. */
const ALERT_TURN = 0.0004;
/** Death01 runs 2.4s; after that the clock stops and the body stays put. */
const DEATH_SETTLE = 2.4;
/**
 * Ground speed Walk_Formal_Loop was authored for; playback rate is actual speed
 * over this, so the feet do not slide. Same number as CLIP_SPEED in player.ts.
 */
const CLIP_GROUND_SPEED = 1.55;

const DEFAULT_SPEED = 1.45;

/** Fraction of the turn remaining per second — matches the player's feel. */
const TURN_RATE = 0.0008;

/**
 * Speed floor while turning.
 *
 * Guards track their polyline exactly rather than cutting corners, so at a
 * right-angle waypoint the facing has to catch up with a direction that changed
 * instantly. Scaling speed by cos(heading error) makes them slow into the turn
 * and pivot, which is both what a person does and what keeps the feet from
 * skating sideways through the corner.
 */
const MIN_TURN_SPEED_SCALE = 0.3;

/** Beam is aimed slightly down: a guard sweeps the floor ahead, not the far wall. */
const BEAM_PITCH = -0.18;

/**
 * Guard weapon light. Cooler and weaker than the player's 240-unit warm beam,
 * so the two read as different lights and the player's own beam still dominates
 * the frame it is pointed at.
 */
const TORCH = {
  intensity: 170,
  radius: 0.06,
  color: v3(0.90, 0.94, 1.0),
  cosInner: Math.cos((14 * Math.PI) / 180),
  cosOuter: Math.cos((30 * Math.PI) / 180),
};

/** One guard walking one closed route, carrying one spot light. */
export class Guard {
  /** Feet position. y is always the floor. */
  readonly pos: Vec3 = v3(0, 0, 0);
  /** Facing, following the direction of travel. yaw 0 faces +Z. */
  yaw = 0;
  readonly character: Character;
  /** Rebuilt in place by update(); do not hold onto the Vec3s. */
  readonly light: Light;
  /** True once shot. A dead guard stops patrolling and its torch goes out. */
  dead = false;
  /** Seconds left standing still, facing a noise. */
  alertLeft = 0;
  /** Yaw to face while alerted. */
  private alertYaw = 0;

  /**
   * Stop and look toward a noise.
   *
   * Out of range, dead or already carried guards ignore it. Re-alerting an
   * already alerted guard restarts the clock and re-aims it, so a second shot
   * from somewhere else turns heads again rather than being swallowed.
   */
  hear(at: Vec3): boolean {
    if (this.dead || this.carried) return false;
    const dx = at.x - this.pos.x, dz = at.z - this.pos.z;
    if (dx * dx + dz * dz > ALERT_RANGE * ALERT_RANGE) return false;
    this.alertLeft = ALERT_LEN;
    this.alertYaw = Math.atan2(dx, dz);
    return true;
  }
  /** True while carried by the player; position is driven from outside. */
  carried = false;
  private knockbackLeft = 0;
  private deathTime = 0;
  /** True while the recoil one-shot is playing. */
  firing = false;
  /** True only on the frame a shot went off. Nothing fires guards yet. */
  justFired = false;
  /** Muzzle transform, latched by update() while this guard's pose is live. */
  private readonly muzzlePos: Vec3 = v3(0, 1.2, 0);
  private readonly muzzleDir: Vec3 = v3(0, 0, 1);

  private readonly wp: Array<[number, number]>;
  private readonly speed: number;
  /** Cumulative arc length at the end of each segment. */
  private readonly cum: number[] = [];
  private readonly total: number;
  /** Distance travelled around the loop. Position is a pure function of this. */
  private travelled = 0;

  constructor(rig: Rig, route: PatrolRoute) {
    if (route.waypoints.length < 2) {
      throw new Error("patrol route needs at least 2 waypoints");
    }
    this.wp = route.waypoints;
    this.speed = route.speed ?? DEFAULT_SPEED;

    let acc = 0;
    for (let i = 0; i < this.wp.length; i++) {
      const a = this.wp[i];
      const b = this.wp[(i + 1) % this.wp.length];
      const len = Math.hypot(b[0] - a[0], b[1] - a[1]);
      if (len < 1e-4) throw new Error(`patrol route has a degenerate leg at ${i}`);
      acc += len;
      this.cum.push(acc);
    }
    this.total = acc;

    this.character = new Character(rig);
    // Night vision is the player's signature; a guard wearing it reads as one.
    this.character.headgear = "cap";
    this.character.play(CLIP, 0);

    // Start already facing down the first leg, so nobody spawns mid-pirouette.
    this.yaw = this.legYaw(0);
    this.place();

    this.light = {
      pos: v3(0, 1, 0),
      kind: LIGHT_SPOT,
      dir: v3(0, 0, 1),
      radius: TORCH.radius,
      color: TORCH.color,
      intensity: TORCH.intensity,
      cosInner: TORCH.cosInner,
      cosOuter: TORCH.cosOuter,
    };
  }

  /** Index of the leg containing `travelled`, and how far into it we are. */
  private locate(): { leg: number; local: number } {
    for (let i = 0; i < this.cum.length; i++) {
      if (this.travelled < this.cum[i]) {
        return { leg: i, local: this.travelled - (i > 0 ? this.cum[i - 1] : 0) };
      }
    }
    return { leg: this.cum.length - 1, local: 0 };
  }

  private legYaw(leg: number): number {
    const a = this.wp[leg];
    const b = this.wp[(leg + 1) % this.wp.length];
    return Math.atan2(b[0] - a[0], b[1] - a[1]);
  }

  private place(): void {
    const { leg, local } = this.locate();
    const a = this.wp[leg];
    const b = this.wp[(leg + 1) % this.wp.length];
    const legLen = this.cum[leg] - (leg > 0 ? this.cum[leg - 1] : 0);
    const t = local / legLen;
    this.pos.x = a[0] + (b[0] - a[0]) * t;
    this.pos.z = a[1] + (b[1] - a[1]) * t;
  }

  /**
   * Fires, if the weapon is off cooldown. Sets `justFired` for this frame.
   *
   * Nothing calls this yet: guards carry the capability so a future alert state
   * has something to drive, but they do not decide to shoot on their own.
   */
  fire(): boolean {
    const went = this.character.fire();
    if (went) this.justFired = true;
    return went;
  }

  update(dt: number): void {
    this.justFired = false;

    if (this.dead) {
      // A takedown plays the knockback, then settles into the death clip's
      // final pose.
      //
      // Entered at DEATH_SETTLE rather than at 0, because Hit_Knockback already
      // puts the guard on the floor and Death01's frame 0 is standing. Playing
      // it from the top made a taken-down guard drop, spring back to its feet
      // and drop a second time. Both clips end prone, so the cross-fade is
      // between two similar resting poses and stays short.
      if (this.knockbackLeft > 0) {
        this.knockbackLeft -= dt;
        if (this.knockbackLeft <= 0) {
          this.character.play(DEATH_CLIP, 0.18, DEATH_SETTLE);
          // The clip is already at its end, so the settle clock is spent too.
          // Left running, it would advance past the end of a non-looping clip.
          this.deathTime = DEATH_SETTLE;
        }
      }
      // Let the death animation play out, then hold the final pose. The clock
      // stops rather than looping, so the body stays where it fell.
      const settling = this.deathTime < DEATH_SETTLE;
      this.deathTime += dt;
      // Rate 1, not 0. Character.update advances clip time by dt*rate, so the
      // rate that means "standing still" for a walk cycle also means the death
      // animation never plays — the guard just froze upright on its first frame.
      this.character.update(settling ? dt : 0, 1, false);
      return;
    }

    // Alerted: hold position, turn toward the noise, keep the idle playing.
    // The torch turns with the guard, which is the whole point — a room full
    // of beams all swinging to the same spot is the tell that the shot landed.
    if (this.alertLeft > 0) {
      this.alertLeft -= dt;
      this.yaw = lerpAngle(this.yaw, this.alertYaw, 1 - Math.pow(ALERT_TURN, dt));
      this.character.play(IDLE_CLIP, 0.25);
      this.character.update(dt, 1, true);
      this.firing = this.character.firing;
      this.syncLightAndMuzzle();
      return;
    }
    this.character.play(CLIP, 0.25);

    const { leg } = this.locate();
    const desired = this.legYaw(leg);
    this.yaw = lerpAngle(this.yaw, desired, 1 - Math.pow(TURN_RATE, dt));

    const scale = clamp(
      Math.cos(angleDelta(this.yaw, desired)),
      MIN_TURN_SPEED_SCALE,
      1,
    );
    const moveSpeed = this.speed * scale;

    this.travelled = (this.travelled + moveSpeed * dt) % this.total;
    this.place();

    // `aiming` layers the two-handed pistol grip over the walk, which is what
    // keeps the weapon forward instead of swinging with the stride.
    this.character.update(dt, moveSpeed / CLIP_GROUND_SPEED, true);
    this.firing = this.character.firing;

    this.syncLightAndMuzzle();
  }

  /**
   * Copies the live pose's lens and muzzle transforms onto this guard's light.
   *
   * Must run while this guard's pose is the one on the shared rig, which means
   * immediately after its own Character.update and before anybody else's.
   */
  private syncLightAndMuzzle(): void {
    const p = this.character.weaponLight(this.pos, this.yaw).pos;
    this.light.pos.x = p.x;
    this.light.pos.y = p.y;
    this.light.pos.z = p.z;
    const mz = this.character.muzzle(this.pos, this.yaw);
    this.muzzlePos.x = mz.pos.x;
    this.muzzlePos.y = mz.pos.y;
    this.muzzlePos.z = mz.pos.z;
    this.muzzleDir.x = mz.dir.x;
    this.muzzleDir.y = mz.dir.y;
    this.muzzleDir.z = mz.dir.z;
    const cp = Math.cos(BEAM_PITCH);
    this.light.dir.x = Math.sin(this.yaw) * cp;
    this.light.dir.y = Math.sin(BEAM_PITCH);
    this.light.dir.z = Math.cos(this.yaw) * cp;
  }

  /**
   * Moves this guard to the point on its own route closest to (x, z).
   *
   * For the scripted demo, which needs a guard to be somewhere specific when a
   * beat starts. Patrols wander, so a script that just looks for "the nearest
   * guard" finds a different one every run — in testing it ended up chasing one
   * twenty metres away through a wall. Snapping along the existing route keeps
   * the guard where it is supposed to be able to walk.
   */
  snapNearest(x: number, z: number): void {
    let bestT = 0;
    let bestD = Infinity;
    for (let t = 0; t < this.total; t += 0.25) {
      this.travelled = t;
      this.place();
      const d = (this.pos.x - x) ** 2 + (this.pos.z - z) ** 2;
      if (d < bestD) { bestD = d; bestT = t; }
    }
    this.travelled = bestT;
    this.place();
  }

  /**
   * Restores this guard's world transforms onto the shared rig.
   *
   * dt=0 advances no clock, so it re-runs sampling and produces exactly the
   * pose update() produced — it just has to be re-run because every other
   * Character shares the same Rig scratch buffers.
   */
  repose(): void {
    this.character.update(0, 1, !this.dead);
  }

  /**
   * One shot, one kill.
   *
   * Deliberately not a health system: this is a stealth game, and a guard who
   * survives being shot turns every encounter into a firefight. Returns false
   * if already dead so a second bullet does not restart the animation.
   */
  kill(melee = false): boolean {
    if (this.dead) return false;
    this.dead = true;
    this.deathTime = melee ? -KNOCKBACK_LEN : 0;
    this.knockbackLeft = melee ? KNOCKBACK_LEN : 0;
    this.character.play(melee ? KNOCKBACK_CLIP : DEATH_CLIP, 0.08);
    // The torch goes out rather than falling to the floor — a dropped light
    // would need a physics body, and that is exactly what we removed.
    this.light.intensity = 0;
    return true;
  }

  /**
   * Distance from `origin` along `dir` at which a bullet would hit this guard,
   * or null. A vertical capsule about the spine, which is a much better fit for
   * a standing figure than the 26 limb boxes it is drawn from — and far cheaper
   * than testing all of them.
   */
  hitScan(origin: Vec3, dir: Vec3, tmax: number): number | null {
    if (this.dead) return null;
    const R = 0.34;
    const LO = 0.15, HI = 1.75;
    // Closest approach between the ray and the guard's vertical axis, solved in
    // the XZ plane since the axis is vertical.
    const ox = origin.x - this.pos.x;
    const oz = origin.z - this.pos.z;
    const a = dir.x * dir.x + dir.z * dir.z;
    if (a < 1e-9) return null;
    const b = 2 * (ox * dir.x + oz * dir.z);
    const c = ox * ox + oz * oz - R * R;
    const disc = b * b - 4 * a * c;
    if (disc < 0) return null;
    const t = (-b - Math.sqrt(disc)) / (2 * a);
    if (t < 0 || t > tmax) return null;
    const y = origin.y + dir.y * t;
    if (y < this.pos.y + LO || y > this.pos.y + HI) return null;
    return t;
  }

  /** World position and direction of this guard's muzzle, latched by update(). */
  muzzle(): { pos: Vec3; dir: Vec3 } {
    return {
      pos: v3(this.muzzlePos.x, this.muzzlePos.y, this.muzzlePos.z),
      dir: v3(this.muzzleDir.x, this.muzzleDir.y, this.muzzleDir.z),
    };
  }

  buildBoxes(m: CharacterMaterials): { data: Float32Array<ArrayBuffer>; count: number } {
    // No extra rotation while carried. Death01 already ends horizontal —
    // measured 0.65m tall against 1.7m standing — so a body is lying flat by
    // the time it can be picked up, and pitching it further stands it back up
    // (measured 1.35m, i.e. upright over the shoulder).
    return this.character.buildBoxes(this.pos, this.yaw, m, !this.dead);
  }
}

/** The guards on patrol, driven as one. */
export class Guards {
  private readonly guards: Guard[];
  /** Reused across frames; excludes dead guards. */
  private readonly live: Light[] = [];

  constructor(rig: Rig, routes: PatrolRoute[]) {
    this.guards = routes.map((r) => new Guard(rig, r));
  }

  get count(): number {
    return this.guards.length;
  }

  /** Read-only view, for anything that needs an individual guard. */
  get all(): readonly Guard[] {
    return this.guards;
  }

  update(dt: number): void {
    for (const g of this.guards) g.update(dt);
  }

  /**
   * Tells every guard in earshot about a gunshot. Returns how many reacted.
   *
   * Broadcast rather than traced: sound goes around corners, and a shot in this
   * building is loud enough that everyone still standing hears it. Working out
   * who has line of sight would be both wrong and more expensive.
   */
  alert(at: Vec3): number {
    let n = 0;
    for (const g of this.guards) if (g.hear(at)) n++;
    return n;
  }

  /**
   * Appends every guard's limb boxes into `out`, starting `outOffset` boxes in,
   * and returns how many boxes were written.
   *
   * Each guard is re-posed immediately before it is packed. The rig is shared,
   * so whichever Character updated last owns rig.worldPos — without this, every
   * guard would render wearing the last guard's pose.
   *
   * NOTE for the caller: the same hazard applies across the player/guard
   * boundary. Pack the player *before* calling this, or re-pose the player
   * after it, or the player will come out wearing a guard's pose.
   */
  buildBoxes(out: Float32Array, outOffset: number, m: CharacterMaterials): number {
    const capacity = Math.floor(out.length / BOX_STRIDE_F32) - outOffset;
    let written = 0;
    for (const g of this.guards) {
      g.repose();
      const { data, count } = g.buildBoxes(m);
      if (written + count > capacity) break;
      out.set(
        data.subarray(0, count * BOX_STRIDE_F32),
        (outOffset + written) * BOX_STRIDE_F32,
      );
      written += count;
    }
    return written;
  }

  /**
   * One spot light per guard, refreshed by update() each frame.
   *
   * The array and the Light objects are reused, so this is safe to call every
   * frame — but do not cache the Vec3s, they are mutated in place.
   */
  lights(): Light[] {
    // Rebuilt rather than returned wholesale: a dead guard's torch is out, and
    // leaving a zero-intensity light in the array still costs every shading
    // point a candidate slot in the RIS pool.
    this.live.length = 0;
    for (const g of this.guards) if (!g.dead) this.live.push(g.light);
    return this.live;
  }

  /**
   * Nearest live guard within `range` of `from`, for a takedown.
   *
   * No facing requirement. A back-only rule reads as arbitrary when the camera
   * is overhead and the guard's facing is a few pixels of silhouette — the
   * range itself is the skill test.
   */
  nearestLive(from: Vec3, range: number): Guard | null {
    let best: Guard | null = null;
    let bestD2 = range * range;
    for (const g of this.guards) {
      if (g.dead) continue;
      const dx = g.pos.x - from.x, dz = g.pos.z - from.z;
      const d2 = dx * dx + dz * dz;
      if (d2 < bestD2) { bestD2 = d2; best = g; }
    }
    return best;
  }

  /** Nearest body within `range`, for picking up. */
  nearestBody(from: Vec3, range: number): Guard | null {
    let best: Guard | null = null;
    let bestD2 = range * range;
    for (const g of this.guards) {
      if (!g.dead || g.carried) continue;
      const dx = g.pos.x - from.x, dz = g.pos.z - from.z;
      const d2 = dx * dx + dz * dz;
      if (d2 < bestD2) { bestD2 = d2; best = g; }
    }
    return best;
  }

  /**
   * Nearest guard along the ray within `tmax`.
   *
   * Callers pass the distance to the world hit as `tmax`, so a guard behind a
   * wall is simply out of range rather than something to filter out afterwards.
   */
  hitScan(origin: Vec3, dir: Vec3, tmax: number): Guard | null {
    let best: Guard | null = null;
    let bestT = tmax;
    for (const g of this.guards) {
      const t = g.hitScan(origin, dir, bestT);
      if (t === null) continue;
      bestT = t;
      best = g;
    }
    return best;
  }
}

/**
 * Patrol routes for the office in scene/level.ts.
 *
 * Chosen to run past as much shadow-casting clutter as possible while staying
 * clear of it. Every leg was checked against the level's actual box footprints
 * (not just LevelInfo.colliders, which omits cubicle side panels and chairs);
 * the tightest route keeps ~0.3 m of clearance beyond a 0.32 m body radius.
 *
 * Cost note: each guard adds a full Character's worth of boxes (26 at the time
 * of writing — 24 before the pistol, which costs two boxes more than the torch
 * it replaced) to the dynamic list, which every ray tests linearly. Trimming
 * this array is the dial for trading guards against frame time.
 */
export const DEFAULT_PATROLS: PatrolRoute[] = [
  // The main corridor, end to end. Two lanes at z = +/-0.8 keep the guard on
  // the polished concrete strip (z in [-1.5, 1.5]) for the specular streak,
  // inside the scattered crates (nearest at |z| = 1.51) and well clear of the
  // support columns at z = +/-3.9. West end stops at x = -11.5: the conference
  // room's east wall closes the corridor at x = -13.92, and this also keeps the
  // guard off the player's spawn at (-13, 0.5).
  { waypoints: [[-11.5, 0.8], [24.5, 0.8], [24.5, -0.8], [-11.5, -0.8]], speed: 1.5 },

  // A lap around the north cubicle farm's second row. The long legs run down
  // the aisle between the two rows (clear from z = -12.25 to z = -8.95) and
  // along the strip between the row and the corridor columns, so the beam rakes
  // across partitions from both sides.
  { waypoints: [[-20.5, -5.35], [-2.4, -5.35], [-2.4, -10.0], [-20.5, -10.0]], speed: 1.35 },

  // The south-east server room: between the south wall and the rack line
  // (z 7.6-9.4), then back between the racks and the cubicles at z = 13.5.
  // Four free-standing racks make for good hard shadows.
  { waypoints: [[9.2, 6.1], [24.5, 6.1], [24.5, 11.4], [9.2, 11.4]], speed: 1.4 },

  // The west conference room, circling the big table. The north leg threads the
  // 1.26 m gap between a stray crate and the column at x = -18, which is the
  // tightest point on any of these routes.
  { waypoints: [[-23.5, -3.0], [-15.4, -3.0], [-15.4, 5.0], [-23.5, 5.0]], speed: 1.25 },
];
