import { Vec3, angleDelta, clamp, lerpAngle, v3 } from "../core/math";
import { Character, CharacterMaterials } from "./character";
import { Rig } from "../anim/rig";
import { Camera } from "./camera";
import { Input } from "./input";

const RADIUS = 0.28;
// Walking is the default gait. This is a stealth game: you should have to ask
// to move fast, not ask to move quietly.
const WALK_SPEED = 1.6;
const JOG_SPEED = 3.3;
const SPRINT_SPEED = 5.4;
const CROUCH_SPEED = 1.2;
/** You cannot run backwards. Backpedalling is capped to a walk. */
const BACKPEDAL_SPEED = 1.5;
const ACCEL = 22;

/**
 * Ground speed each locomotion clip was authored for.
 *
 * Playback rate is scaled by actual speed over these so the feet advance in
 * step with real movement. Playing a cycle at a fixed rate is exactly what
 * produces the classic ice-skating foot slide.
 */
const CLIP_SPEED: Record<string, number> = {
  Walk_Loop: 1.55,
  Walk_Formal_Loop: 1.55,
  Jog_Fwd_Loop: 3.1,
  Sprint_Loop: 5.2,
  Crouch_Fwd_Loop: 1.2,
};

export type PlayerMaterials = CharacterMaterials;

const DEG = Math.PI / 180;

/**
 * Directional locomotion tuning. Live-editable from the debug panel, because
 * how these feel is not something you can reason your way to.
 */
export const movementTuning = {
  /** Max torso twist before the feet have to step round. */
  maxTwistDeg: 60,
  /** Tighter crouched — you cannot wind up as far in a crouch. */
  maxTwistCrouchDeg: 40,
  /**
   * Angle between aim and travel beyond which the character backpedals instead
   * of turning to face where it is going. Walking away from the cursor while
   * facing it is what you want when the cursor is a threat.
   */
  backwardEnterDeg: 100,
  /** Hysteresis: drop back out of backpedalling below this, so it cannot flicker. */
  backwardExitDeg: 75,
  /** Fraction of the turn remaining per second. Lower is snappier. */
  turnRateMoving: 0.0005,
  turnRateCrouch: 0.002,
  turnRateStanding: 0.006,
  /** Use the more composed 'formal' walk cycle instead of the default one. */
  formalWalk: true,
  /**
   * Sprint is a toggle, so it needs a rule for when it ends. Turning the
   * movement direction further than this from the one being sprinted breaks it:
   * you cannot cut a hard corner at a dead run, and having to tap Shift again
   * makes the commitment explicit.
   */
  sprintCancelDeg: 50,
  /**
   * Cross-fade lengths for locomotion clip changes, interpolated by speed.
   * Leaving a standstill moves every limb at once and needs a long blend; a
   * change of gear at speed has to keep up with the legs or it lags visibly.
   */
  clipFadeSlowSec: 0.32,
  clipFadeFastSec: 0.14,
  /**
   * How far the weapon reaches along the aim, measured rather than guessed: the
   * furthest any right-arm or weapon box gets from the character's centre, over
   * every locomotion clip on open floor, is 0.73 m and it happens within 15
   * degrees of the aim. Rounded up for the box's own thickness.
   *
   * Re-measured for the pistol over the same clips but sampling twist at -60,
   * 0 and +60 degrees as well, which the original sweep did not: 0.831 m with
   * the old torch against 0.801 m with the pistol. Left at 0.80 rather than
   * trimmed to match — the envelope should err toward tucking, and the pistol
   * is only 30 mm shorter in the reach that matters.
   */
  armReachFwd: 0.80,
  /**
   * The same reach out to the character's right and rear, where the arm swings
   * rather than extends. Measured max there is 0.41 m; 0.52 leaves room for the
   * tuck to have finished ramping before contact.
   */
  armReachSide: 0.52,
  /**
   * Metres of approach over which the tuck ramps from none to full. Distance-
   * driven rather than a fixed rate: sprinting covers 0.09 m per frame, so a
   * rate alone always arrives late at speed however high it is set.
   */
  wallTuckFade: 0.22,
  /**
   * Rate the arms tuck in near a wall and come back out again, per second.
   * Raised from 9: with the distance ramp doing the shaping this is only
   * catching the residual lag, and 9 still let the torch reach 0.120 m into a
   * wall against 0.077 m here. 20 buys another 9 mm and is fast enough to read
   * as a snap, so it is not worth it.
   */
  wallTuckRate: 14,
};

/**
 * Where the arm and weapon penetration now stands, after the torch became a
 * body-mounted weapon light.
 *
 * Swept level-wide on a 0.4 m grid over every cell the collision circle can
 * occupy within arm's reach of a collider, at 12 aim yaws, 5 locomotion clips,
 * 3 twists and 4 clip phases (1.78 M poses), taking the deepest any box corner
 * gets past a collider face with the tuck converged:
 *
 *   worst weapon box   0.177 m (torch)  ->  0.142 m (pistol + weapon light)
 *   worst arm box      0.167 m          ->  0.167 m, the same pose, unchanged
 *
 * Firing does not make it worse: parking Pistol_Shoot at its deepest frame
 * gives 0.136 m, because the recoil pulls the weapon back toward the body.
 *
 * Two things measured and left out. Adding +60 and +90 degree probe bearings
 * changes the worst case by nothing at all and costs 1.6-3.3 points of tuck
 * duty cycle. Halving the probe spacing to 15 degrees (13 rays instead of 7)
 * buys 6 mm on the weapon, nothing on the arm, for nearly double the ray casts
 * — the residual is not a coverage problem. Every remaining case is a
 * sprinting arm at full twist, where the arm is simply longer than the tuck
 * can retract it, and it is the same case before and after this change.
 */

/**
 * Bearings, relative to the aim, that the arm probe is cast along.
 *
 * The arm is not a point in front of the character: measured across the
 * locomotion set it occupies everything from 30 degrees left of the aim round
 * to 150 degrees right of it, because the weapon leads the aim while the
 * shoulder and elbow trail the body. Every wall intersection measured in the
 * office fell in that arc, and 245 of 347 of them were at bearings the old
 * single forward probe could not see at all.
 */
const ARM_PROBE_DEG = [30, 0, -30, -60, -90, -120, -150];

export class Player {
  pos: Vec3 = v3(0, 0, 0);
  /** Where the character is aiming. Drives the weapon light and the upper body. */
  yaw = 0;
  /** Where the legs point. Follows travel direction, lags the aim when still. */
  bodyYaw = 0;
  velX = 0;
  velZ = 0;
  /**
   * The weapon light under the pistol's barrel. Kept the old name for the HUD.
   *
   * Starts on. Opening dark and empty-handed was the purer reading of a stealth
   * game, but it opens on a black screen with no visible way forward, which
   * reads as broken rather than as restraint. Turning it off is one key.
   */
  flashlightOn = true;
  crouching = false;
  /** True while the recoil one-shot is playing. */
  firing = false;
  /** True only on the frame a shot went off — for hanging a muzzle flash on. */
  justFired = false;
  aimPoint: Vec3 = v3(0, 1, 1);
  /** True while travelling away from the aim, i.e. playing the cycle in reverse. */
  backpedalling = false;
  /** Toggled by Shift; cancelled by stopping, crouching or turning hard. */
  sprinting = false;

  readonly character: Character;

  /** Direction being sprinted in, or null while the toggle is armed but idle. */
  private sprintDir: number | null = null;
  /** Smoothed 0..1 arm tuck, driven by a wall the weapon would enter. */
  private armTuck = 0;
  /** Weapon light position, latched each frame while this character's pose is live. */
  private flashPos: Vec3;
  /** Muzzle position and direction, latched on the same frame and for the same reason. */
  private muzzlePos: Vec3;
  private muzzleDir: Vec3;

  constructor(spawn: Vec3, private colliders: Float32Array, rig: Rig) {
    this.pos = v3(spawn.x, spawn.y, spawn.z);
    this.character = new Character(rig);
    // Roughly hand height, for the one frame before the first update().
    this.flashPos = v3(spawn.x, spawn.y + 1.2, spawn.z);
    this.muzzlePos = v3(spawn.x, spawn.y + 1.2, spawn.z);
    this.muzzleDir = v3(0, 0, 1);
  }

  /**
   * Fires, if the weapon is off cooldown. Sets `justFired` for this frame.
   *
   * Safe to call every frame from a held button: the cooldown lives in
   * Character, so the recoil animation and the flash cannot disagree about
   * whether a shot happened.
   */
  /**
   * Ammunition.
   *
   * Finite on purpose. One shot kills, so without a magazine to run down there
   * is never a reason to do anything other than shoot — and a silent takedown
   * exists precisely because shooting should cost you something.
   */
  /**
   * FN Five-seveN, modelled as 10+1.
   *
   * MAG_SIZE is what a magazine holds. The +1 is a round already chambered, so
   * a reload performed *before* running dry gives you 11 — you keep the
   * chambered round and seat a full magazine — while reloading from empty gives
   * you 10, because there was nothing left to chamber. Small detail, but it is
   * the difference between reloading in cover and reloading because you had to.
   */
  static readonly MAG_SIZE = 10;
  static readonly SPARE_MAGS = 2;
  rounds = Player.MAG_SIZE + 1;
  spare = Player.MAG_SIZE * Player.SPARE_MAGS;

  get reloading(): boolean {
    return this.character.reloading;
  }

  fire(): boolean {
    if (this.rounds <= 0) {
      // Dry fire: start a reload rather than making the player press R to learn
      // the magazine is empty. The click is the tell; the reload is the answer.
      this.reload();
      return false;
    }
    const went = this.character.fire();
    if (went) {
      this.rounds--;
      this.justFired = true;
    }
    return went;
  }

  /** Returns false when there is nothing to do — full magazine or no spare. */
  reload(): boolean {
    if (this.rounds >= Player.MAG_SIZE + 1 || this.spare <= 0) return false;
    if (!this.character.reload()) return false;
    // Rounds arrive when the animation completes, not when it starts, so the
    // magazine is genuinely empty while the hands are busy.
    this.reloadPending = true;
    return true;
  }

  private reloadPending = false;
  /**
   * False while a non-firearm slot is selected, so the OCP does not also empty
   * the magazine. Set by main.ts, which owns equipment selection.
   */
  weaponLive = true;
  /**
   * Whether anything is in the hands at all.
   *
   * Distinct from `weaponLive`, which only asks whether the trigger fires a
   * bullet. The OCP is a pistol attachment, so it is still "drawn" — what this
   * gates is the whole two-handed presentation: the grip idle, the aim layer
   * that points the arms at the cursor, and the weapon geometry itself.
   */
  weaponDrawn = true;
  /** Speed multiplier while carrying a body. */
  static readonly CARRY_SPEED = 0.55;

  /**
   * Reach for a takedown or for picking a body up.
   *
   * Generous relative to the collision radius (0.28) because the camera is
   * overhead and judging a 30cm gap from up there is not a skill, it is a
   * guess.
   */
  static readonly REACH = 1.35;
  /**
   * Set by main.ts. Dragging a body: crouched, facing what you are pulling, and
   * moving backwards.
   */
  carrying = false;
  /**
   * World yaw the body being dragged sits at, relative to the player.
   *
   * Set by main.ts each frame. Dragging locks the feet to face it: you hold a
   * pair of ankles and walk backwards, so the facing is dictated by the load,
   * not by the cursor.
   */
  dragYaw = 0;

  get swinging(): boolean {
    return this.character.swinging;
  }

  /** Starts the takedown swing. Returns false if one is already playing. */
  swing(): boolean {
    if (this.character.swinging) return false;
    return this.character.melee();
  }

  private blocked(x: number, z: number): boolean {
    const c = this.colliders;
    for (let i = 0; i < c.length; i += 4) {
      if (x + RADIUS > c[i] && x - RADIUS < c[i + 2] &&
          z + RADIUS > c[i + 1] && z - RADIUS < c[i + 3]) {
        return true;
      }
    }
    return false;
  }

  /**
   * Distance from the body's centre to the first collider along a ray, or
   * `maxDist` if nothing is in the way.
   *
   * Deliberately *not* built on `blocked`: that inflates every collider by
   * RADIUS, which ties the arm's reach to the collision circle. The circle is
   * 0.28 m and the arm is 0.73 m, so anything derived from it is short by a
   * factor of two and a half — and the circle has to stay small for corners to
   * be reachable. A plain 2D slab test gives the arm its own reach.
   */
  private rayWall(dirX: number, dirZ: number, maxDist: number): number {
    const c = this.colliders;
    const ox = this.pos.x;
    const oz = this.pos.z;
    const invX = 1 / (dirX || 1e-9);
    const invZ = 1 / (dirZ || 1e-9);
    let best = maxDist;
    for (let i = 0; i < c.length; i += 4) {
      let t0 = (c[i] - ox) * invX;
      let t1 = (c[i + 2] - ox) * invX;
      if (t0 > t1) { const t = t0; t0 = t1; t1 = t; }
      let u0 = (c[i + 1] - oz) * invZ;
      let u1 = (c[i + 3] - oz) * invZ;
      if (u0 > u1) { const u = u0; u0 = u1; u1 = u; }
      const enter = Math.max(t0, u0);
      const exit = Math.min(t1, u1);
      // enter < 0 with exit > 0 means the origin is already inside; treat that
      // as a hit at zero so a player wedged into geometry still tucks.
      if (exit < 0 || enter > exit) continue;
      const hit = Math.max(enter, 0);
      if (hit < best) best = hit;
    }
    return best;
  }

  update(dt: number, input: Input, camera: Camera, canvasW: number, canvasH: number): void {
    // Cleared at the top of the frame that follows the shot, so a caller that
    // reads it after update() sees exactly one true per shot.
    this.justFired = false;

    // ---- aim -------------------------------------------------------------
    this.aimPoint = camera.screenToGround(
      input.mouseX, input.mouseY, canvasW, canvasH, this.pos.y + 1.0,
    );
    const ax = this.aimPoint.x - this.pos.x;
    const az = this.aimPoint.z - this.pos.z;
    if (ax * ax + az * az > 1e-4) {
      const targetYaw = Math.atan2(ax, az);
      this.yaw = lerpAngle(this.yaw, targetYaw, 1 - Math.pow(0.0001, dt));
    }

    if (input.pressed("KeyF")) this.flashlightOn = !this.flashlightOn;
    if (input.pressed("KeyC") || input.pressed("ControlLeft")) this.crouching = !this.crouching;
    // `held` gives auto-fire off a held button, rate-limited by the weapon in
    // Character. `pressed` as well, and not redundantly: a click whose down and
    // up both land between two frames is never `held` on any frame, and the
    // edge set is only cleared by endFrame(), so it is the half that cannot
    // drop a fast tap. Verified in the browser that a left click fires.
    if (this.weaponLive && (input.pressed("Mouse0") || input.held("Mouse0"))) this.fire();
    // Only the firearm reloads. Racking a magazine while holding the OCP was
    // both nonsense and a way to lock yourself out of firing it for 1.67s.
    if (this.weaponLive && input.pressed("KeyR")) this.reload();
    if (this.reloadPending && !this.character.reloading) {
      this.reloadPending = false;
      // A round still chambered survives the magazine swap; an empty gun has
      // to chamber one out of the new magazine, so it comes back one short.
      const chambered = this.rounds > 0 ? 1 : 0;
      const got = Math.min(Player.MAG_SIZE, this.spare);
      this.spare -= got;
      this.rounds = got + chambered;
    }

    // ---- movement --------------------------------------------------------
    const { forward, right } = camera.groundBasis();
    let mx = 0;
    let mz = 0;
    if (input.held("KeyW")) { mx += forward.x; mz += forward.z; }
    if (input.held("KeyS")) { mx -= forward.x; mz -= forward.z; }
    if (input.held("KeyD")) { mx += right.x; mz += right.z; }
    if (input.held("KeyA")) { mx -= right.x; mz -= right.z; }

    const tune = movementTuning;
    const mag = Math.hypot(mx, mz);
    const inputYaw = mag > 1e-4 ? Math.atan2(mx / mag, mz / mag) : null;

    // Decide the backpedal state from the *intended* direction, before speed is
    // chosen — the speed cap depends on it, so it cannot be worked out later
    // from the resulting velocity without lagging a frame.
    if (inputYaw !== null) {
      const rel = Math.abs(angleDelta(this.yaw, inputYaw));
      const enter = tune.backwardEnterDeg * DEG;
      const exit = tune.backwardExitDeg * DEG;
      this.backpedalling = this.backpedalling ? rel > exit : rel > enter;
    } else {
      this.backpedalling = false;
    }

    // A toggle rather than a hold: sprinting is a commitment in a stealth game,
    // and holding a key down for the length of a corridor is not a decision.
    // It ends on its own the moment the commitment stops being true.
    if (input.pressed("ShiftLeft") || input.pressed("ShiftRight")) {
      this.sprinting = !this.sprinting && !this.crouching;
      this.sprintDir = inputYaw;
    }
    if (this.sprinting) {
      if (this.crouching) {
        this.sprinting = false;
      } else if (inputYaw === null) {
        // Only counts as stopping once they have actually been moving —
        // arming the toggle first and then pressing a key has to work.
        if (this.sprintDir !== null) this.sprinting = false;
      } else if (this.sprintDir === null) {
        this.sprintDir = inputYaw;
      } else if (Math.abs(angleDelta(this.sprintDir, inputYaw)) > tune.sprintCancelDeg * DEG) {
        this.sprinting = false;
      }
      // Note the direction is latched, not tracked. Following it frame by frame
      // ratchets: W then W+D then D is three 45 degree steps, none of which
      // trips a 50 degree limit, and you can turn any angle at a dead run.
    }
    if (this.carrying) this.sprinting = false;
    if (!this.sprinting) this.sprintDir = null;

    const base = this.crouching ? CROUCH_SPEED
      : this.backpedalling ? BACKPEDAL_SPEED
      : this.sprinting ? SPRINT_SPEED
      : WALK_SPEED;
    // Carrying a body has to cost something, or hiding one is free and there is
    // no reason ever to leave it where it fell.
    const speed = this.carrying ? base * Player.CARRY_SPEED : base;

    if (mag > 1e-4) {
      mx = (mx / mag) * speed;
      mz = (mz / mag) * speed;
    } else {
      mx = 0;
      mz = 0;
    }

    const k = 1 - Math.exp(-ACCEL * dt);
    this.velX += (mx - this.velX) * k;
    this.velZ += (mz - this.velZ) * k;

    // Resolve each axis independently so we slide along walls.
    const nx = this.pos.x + this.velX * dt;
    if (!this.blocked(nx, this.pos.z)) this.pos.x = nx;
    else this.velX = 0;
    const nz = this.pos.z + this.velZ * dt;
    if (!this.blocked(this.pos.x, nz)) this.pos.z = nz;
    else this.velZ = 0;

    // ---- wall proximity --------------------------------------------------
    // The collision circle is deliberately tight so corners stay reachable, but
    // the weapon arm reaches well past it: standing square to a wall would push
    // the hands, and with them the light source, through to the far side.
    // Widening the circle would fix the arm and ruin the movement, so instead
    // probe the volume the arm actually sweeps and bring the arms in — which is
    // what real games do with a weapon near a wall, and it moves the weapon
    // back out of the geometry for free.
    //
    // A fan of rays, not one point ahead. The reach along each falls off from
    // `armReachFwd` to `armReachSide` as cos^2 of the bearing, which is the
    // measured envelope to within a few centimetres and errs toward tucking.
    //
    // Known limitation: `colliders` is flat rects with no height, so a desk or
    // a crate reads the same as a wall even though the arm rides above it at
    // ~1.2 m. Over the whole level 61% of tucked frames are that false positive
    // (69% before this change — the forward probe had the same blind spot), and
    // fixing it properly means giving LevelInfo.colliders a height.
    let intrusion = 0;
    for (const deg of ARM_PROBE_DEG) {
      const b = this.yaw + deg * DEG;
      // Clamped before squaring: cos^2 alone is symmetric about 90 degrees, so
      // the rear bearings would probe out to 0.73 m instead of 0.52 m — reach
      // the arm does not have back there. Worth 1.3 points of tuck duty cycle
      // for no change in penetration.
      const c = Math.max(Math.cos(deg * DEG), 0);
      const reach = tune.armReachSide + (tune.armReachFwd - tune.armReachSide) * c * c;
      const hit = this.rayWall(Math.sin(b), Math.cos(b), reach);
      intrusion = Math.max(intrusion, (reach - hit) / tune.wallTuckFade);
    }
    const tuckTarget = clamp(intrusion, 0, 1);
    this.armTuck += (tuckTarget - this.armTuck) * (1 - Math.exp(-tune.wallTuckRate * dt));

    // ---- body facing vs aim ---------------------------------------------
    const moveSpeed = Math.hypot(this.velX, this.velZ);
    const moving = moveSpeed > 0.15;
    const maxTwist =
      (this.crouching ? tune.maxTwistCrouchDeg : tune.maxTwistDeg) * DEG;

    if (moving) {
      const moveYaw = Math.atan2(this.velX, this.velZ);

      // Feet want to point along travel — or, when backpedalling, along the
      // direction we are backing away from, which keeps the aim in front.
      const desiredFacing = this.backpedalling ? moveYaw + Math.PI : moveYaw;

      // Clamp the feet to within the twist limit of the aim. Without this a 90
      // degree strafe demands a twist the spine cannot deliver, and the torso
      // visibly detaches from the cursor.
      const d = clamp(angleDelta(this.yaw, desiredFacing), -maxTwist, maxTwist);
      const target = this.yaw + d;

      const turnRate = this.crouching ? tune.turnRateCrouch : tune.turnRateMoving;
      // Dragging overrides the aim entirely: the feet face the body, whatever
      // the cursor is doing.
      const facing = this.carrying ? this.dragYaw : target;
      this.bodyYaw = lerpAngle(this.bodyYaw, facing, 1 - Math.pow(turnRate, dt));
    } else {
      this.backpedalling = false;
      if (this.carrying) {
        // Standing still while dragging still holds the facing on the body.
        this.bodyYaw = lerpAngle(
          this.bodyYaw, this.dragYaw, 1 - Math.pow(tune.turnRateCrouch, dt),
        );
      }
      // Standing: hold the feet still while the torso winds up, then step round
      // once the twist gets uncomfortable. This is the turn-in-place.
      const t = angleDelta(this.bodyYaw, this.yaw);
      if (Math.abs(t) > maxTwist) {
        const target = this.yaw - Math.sign(t) * maxTwist * 0.35;
        const turnRate = this.crouching ? tune.turnRateStanding * 3 : tune.turnRateStanding;
        this.bodyYaw = lerpAngle(this.bodyYaw, target, 1 - Math.pow(turnRate, dt));
      }
    }
    this.character.setTwist(
      clamp(angleDelta(this.bodyYaw, this.yaw), -maxTwist, maxTwist),
    );

    let clip: string;
    if (this.carrying) {
      // Crouched, always. Walk_Carry_Loop is a two-handed *carry* — upright with
      // the arms raised around a crate — which read as holding an invisible box
      // when standing still. Dragging happens down at the body's level, and the
      // crouch set already has a moving and a standing pose.
      clip = moving ? "Crouch_Fwd_Loop" : "Crouch_Idle_Loop";
    } else if (this.crouching) {
      clip = moving ? "Crouch_Fwd_Loop" : "Crouch_Idle_Loop";
    } else if (moving) {
      clip = moveSpeed > JOG_SPEED * 1.15 ? "Sprint_Loop"
        : moveSpeed > WALK_SPEED * 1.45 ? "Jog_Fwd_Loop"
        : tune.formalWalk ? "Walk_Formal_Loop" : "Walk_Loop";
    } else {
      // Empty hands get a plain idle. The grip pose with nothing in it reads as
      // aiming an invisible pistol, which is exactly what the "hands" slot is
      // supposed to stop looking like.
      clip = this.weaponDrawn ? "Pistol_Idle_Loop" : "Idle_Loop";
    }
    // Fade length scales with how fast the character is actually travelling.
    const fade = tune.clipFadeSlowSec +
      (tune.clipFadeFastSec - tune.clipFadeSlowSec) * clamp(moveSpeed / JOG_SPEED, 0, 1);
    this.character.play(clip, fade);

    const nominal = CLIP_SPEED[clip];
    let rate = nominal ? clamp(moveSpeed / nominal, 0.4, 2.2) : 1;
    // Dragging means walking backwards away from the body you are facing, so
    // the crouch cycle runs in reverse — the same trick the backpedal uses,
    // because the library has no backward clip.
    if (this.carrying && moving) rate = -rate;
    // There is no backward clip in the library, so run the forward cycle in
    // reverse. The feet still travel the right way because the body is facing
    // the direction being backed away from.
    if (this.backpedalling) rate = -rate;
    // Layer the two-handed grip over locomotion, so the arms hold the weapon
    // where you are aiming instead of swinging with the stride. Unconditional
    // while standing — the pistol is never stowed, and the standing idle is
    // already the grip pose, so this is a no-op there rather than a ramp that
    // has to switch at the moving threshold.
    // The aim layer is what swings the arms onto the cursor; with nothing in
    // hand there is nothing to point.
    this.character.update(
      dt, rate, !this.crouching && this.weaponDrawn && !this.carrying, this.armTuck,
    );
    this.firing = this.character.firing;

    // Latch the light and muzzle transforms *here*, while this character owns
    // the rig. Rig.computeWorld writes into shared scratch, and main.ts poses
    // every guard between packing the player and reading the light, so reading
    // it lazily returned a guard's hand transform placed at the player's feet.
    // Measured against the packed lens box over an idle -> walk -> idle cycle,
    // that put the light source 1.05 m from the model it comes out of while
    // standing (0.10 m while walking, because a walking player's pose happens
    // to resemble the guards'), which is the beam detaching from the hand.
    // Rig space is rooted at the body yaw; the twist has already carried the
    // hand toward the aim.
    const p = this.character.weaponLight(this.pos, this.bodyYaw).pos;
    this.flashPos.x = p.x;
    this.flashPos.y = p.y;
    this.flashPos.z = p.z;
    const gun = this.character.muzzle(this.pos, this.bodyYaw);
    this.muzzlePos.x = gun.pos.x;
    this.muzzlePos.y = gun.pos.y;
    this.muzzlePos.z = gun.pos.z;
    this.muzzleDir.x = gun.dir.x;
    this.muzzleDir.y = gun.dir.y;
    this.muzzleDir.z = gun.dir.z;
  }

  /** World position of the weapon light's lens, taken from the animated hand. */
  flashlightOrigin(): Vec3 {
    return v3(this.flashPos.x, this.flashPos.y, this.flashPos.z);
  }

  /**
   * World position and direction of the muzzle, latched during update().
   *
   * Unlike the beam this *is* the barrel's own direction: a muzzle flash lasts
   * one frame and belongs where the gun is actually pointing, and there is no
   * time for the walk-cycle bob to read as wobble. Be aware it is not the aim —
   * measured 14-15 degrees off it while moving, for the reason written up on
   * Character.weaponLight. Use `yaw` if you want the cursor.
   */
  muzzle(): { pos: Vec3; dir: Vec3 } {
    return {
      pos: v3(this.muzzlePos.x, this.muzzlePos.y, this.muzzlePos.z),
      dir: v3(this.muzzleDir.x, this.muzzleDir.y, this.muzzleDir.z),
    };
  }

  /**
   * Beam direction.
   *
   * Deliberately taken from the aim rather than the hand bone: the animation
   * has its own arm motion, and a beam that bobbed with the walk cycle would be
   * unusable to play with. The lens *position* still comes from the rig, so the
   * light source moves naturally.
   *
   * Mounting the light on the weapon did not change that. Measured, the barrel
   * sits 14-15 degrees off the body yaw while moving and swings 27 degrees in
   * pitch between standing and sprinting, so `Character.weaponLight().dir` is
   * still the wrong thing to point a playable beam along.
   */
  flashlightDir(): Vec3 {
    const pitch = -0.13;
    const cp = Math.cos(pitch);
    return v3(Math.sin(this.yaw) * cp, Math.sin(pitch), Math.cos(this.yaw) * cp);
  }

  buildBoxes(m: PlayerMaterials): { data: Float32Array<ArrayBuffer>; count: number } {
    // Both hands are on the body being carried, so the weapon goes to the hip
    // and its light goes out with it.
    // Holstered whenever nothing is drawn — while dragging, or on empty hands.
    this.character.stowed = this.carrying || !this.weaponDrawn;
    return this.character.buildBoxes(
      this.pos, this.bodyYaw, m,
      this.flashlightOn && !this.carrying && this.weaponDrawn,
    );
  }
}
