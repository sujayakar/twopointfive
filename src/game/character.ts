import { Vec3, clamp, v3 } from "../core/math";
import { Pose, Rig, makePose, quatMul, quatRotate } from "../anim/rig";
import { BOX_STRIDE_F32, FLAG_EMISSIVE } from "../scene/scene";

// ---------------------------------------------------------------------------
// Blocky character bound to the Quaternius rig.
//
// The world is made of boxes and looks deliberately blocky, so rendering a
// smooth 13.7k-triangle mannequin against it would read as foreign — and would
// mean adding a triangle path to a tracer that is fast precisely because it only
// does oriented boxes. Instead each limb is a box spanning a *pair* of joints,
// which gets its length and orientation straight from the animation. The motion
// is the professionally authored motion; only the shading proxy is blocky.
// ---------------------------------------------------------------------------

export interface CharacterMaterials {
  skin: number;
  cloth: number;
  clothDark: number;
  metal: number;
  lens: number;
  /** Night-vision tubes. Emissive, but deliberately not a registered light. */
  nvgLens: number;
}

/** A material the character needs, for the caller to register with the scene. */
export interface CharacterMaterialSpec {
  name: keyof CharacterMaterials;
  albedo: [number, number, number];
  roughness: number;
  metallic: number;
  emissive?: [number, number, number];
}

/**
 * The materials the box rig asks for, in no particular order.
 *
 * Declared next to the geometry that uses them rather than at the call site:
 * the segment table is what decides which slots exist, so adding a limb
 * material should not mean editing the scene setup to keep it compiling.
 */
export function characterMaterialSpec(): CharacterMaterialSpec[] {
  return [
    // Face and hands stay bright. With the outfit this dark they are most of
    // what locates the character when he is standing behind his own beam.
    { name: "skin", albedo: [0.74, 0.60, 0.49], roughness: 0.55, metallic: 0.0 },
    // Charcoal with a cold cast rather than the old light grey. It costs some
    // readability in unlit rooms, which is the point: a stealth silhouette
    // should be legible from its shape and its goggles, not from its albedo.
    { name: "cloth", albedo: [0.16, 0.17, 0.20], roughness: 0.80, metallic: 0.0 },
    { name: "clothDark", albedo: [0.095, 0.10, 0.12], roughness: 0.86, metallic: 0.0 },
    { name: "metal", albedo: [0.42, 0.44, 0.47], roughness: 0.28, metallic: 1.0 },
    { name: "lens", albedo: [0, 0, 0], roughness: 1.0, metallic: 0.0, emissive: [9.0, 8.4, 7.2] },
    // Bright enough to read as "on" through the tonemap, far too dim to be
    // worth a light slot — nothing is meant to be lit by these.
    {
      name: "nvgLens",
      albedo: [0.02, 0.03, 0.02],
      roughness: 1.0,
      metallic: 0.0,
      emissive: [0.20, 2.60, 0.70],
    },
  ];
}

/** A limb: a box spanning from one joint to another. */
interface Segment {
  from: string;
  to: string;
  /** Half-thickness across the bone axis, in metres. */
  thick: number;
  /** Half-thickness along the second cross axis; defaults to `thick`. */
  thick2?: number;
  mat: keyof CharacterMaterials;
  /** Extends the box past the `to` joint, for feet and the head. */
  extend?: number;
}

const SEGMENTS: Segment[] = [
  // Torso. Two segments so the spine can bend visibly.
  { from: "pelvis", to: "spine_02", thick: 0.115, thick2: 0.088, mat: "cloth" },
  { from: "spine_02", to: "neck_01", thick: 0.125, thick2: 0.093, mat: "cloth" },
  { from: "neck_01", to: "Head", thick: 0.045, mat: "skin" },
  // Head sits above the Head joint, so it needs to extend past its endpoint.
  { from: "Head", to: "Head", thick: 0.098, mat: "skin", extend: 0.20 },

  { from: "clavicle_l", to: "upperarm_l", thick: 0.055, mat: "cloth" },
  { from: "upperarm_l", to: "lowerarm_l", thick: 0.050, mat: "cloth" },
  { from: "lowerarm_l", to: "hand_l", thick: 0.043, mat: "cloth" },
  { from: "hand_l", to: "hand_l", thick: 0.040, mat: "skin", extend: 0.09 },

  { from: "clavicle_r", to: "upperarm_r", thick: 0.055, mat: "cloth" },
  { from: "upperarm_r", to: "lowerarm_r", thick: 0.050, mat: "cloth" },
  { from: "lowerarm_r", to: "hand_r", thick: 0.043, mat: "cloth" },
  { from: "hand_r", to: "hand_r", thick: 0.040, mat: "skin", extend: 0.09 },

  { from: "thigh_l", to: "calf_l", thick: 0.070, mat: "clothDark" },
  { from: "calf_l", to: "foot_l", thick: 0.058, mat: "clothDark" },
  { from: "foot_l", to: "ball_l", thick: 0.048, thick2: 0.055, mat: "clothDark", extend: 0.05 },

  { from: "thigh_r", to: "calf_r", thick: 0.070, mat: "clothDark" },
  { from: "calf_r", to: "foot_r", thick: 0.058, mat: "clothDark" },
  { from: "foot_r", to: "ball_r", thick: 0.048, thick2: 0.055, mat: "clothDark", extend: 0.05 },
];

/**
 * Night-vision goggles, in the Head bone's own frame: +X across, +Y up the
 * skull, +Z out through the face (verified against the rest pose, not assumed).
 *
 * Riding the bone rather than the body yaw is what makes them track head motion
 * — the neck leads every turn by a few frames and static goggles would slide.
 */
const NVG_MOUNT = { x: 0, y: 0.105, z: 0.100, hx: 0.078, hy: 0.045, hz: 0.020 };
/** Classic tri-lens arrangement: two low and outboard, one raised on centre. */
const NVG_LENSES = [
  { x: 0.048, y: 0.086 },
  { x: -0.048, y: 0.086 },
  { x: 0, y: 0.130 },
];
const NVG_LENS_R = 0.022;
const NVG_LENS_Z = NVG_MOUNT.z + NVG_MOUNT.hz + 0.012;

/**
 * Guards wear a cap instead.
 *
 * Same four boxes as the goggles, re-proportioned: night vision is the player's
 * signature and a guard with it reads as another player. The count has to match
 * exactly — the renderer groups dynamic boxes by a fixed stride of one
 * character, and a 22-box guard would put every character after it out of
 * alignment with its own group bounds.
 */
const CAP_CROWN = { x: 0, y: 0.115, z: 0.012, hx: 0.086, hy: 0.036, hz: 0.088 };
/** Brim, in three segments so it can curve slightly rather than reading as a plank. */
const CAP_BRIM = [
  { x: 0.052, y: 0.088, z: 0.128, hx: 0.030, hy: 0.010, hz: 0.040 },
  { x: -0.052, y: 0.088, z: 0.128, hx: 0.030, hy: 0.010, hz: 0.040 },
  { x: 0, y: 0.090, z: 0.140, hx: 0.034, hy: 0.010, hz: 0.048 },
];

/** Bones the aim pose overrides while the legs keep running locomotion. */
const UPPER_BODY = ["spine_02", "clavicle_l", "clavicle_r", "neck_01"];
/** Both arms, for the low-ready tuck near walls. */
const BOTH_ARMS = ["clavicle_l", "clavicle_r"];

/**
 * The two-handed pistol grip. Base pose for everything the upper body does.
 *
 * Layered over locomotion through UPPER_BODY, and played on its own as the
 * standing idle, so the hands are in the same place either way and the weapon
 * geometry below never has to know which.
 *
 * (Historical note: this used to be `Idle_Torch_Loop` plus a corrective that
 * blended the free arm back toward `Idle_Loop`, because the torch idle braced
 * the empty hand out to the side as if leaning on something. A two-handed grip
 * has no free arm, so that corrective and its mask are gone with the clip.)
 */
const PISTOL_IDLE = "Pistol_Idle_Loop";
/** One-shot recoil, layered over whatever the legs are doing. */
const SHOOT_CLIP = "Pistol_Shoot";
const RELOAD_CLIP = "Pistol_Reload";
/**
 * Takedown swing. The library has no grab or takedown clip, so the move is
 * staged: the attacker throws this hook while the victim plays its knockback.
 * At this camera distance the hands never visibly need to connect.
 */
const MELEE_CLIP = "Melee_Hook";
const MELEE_LEN = 0.47;
/** Carrying a body. Full-body, so it replaces locomotion rather than layering. */
export const CARRY_CLIP = "Walk_Carry_Loop";
/** Pistol_Reload runs 1.67s. */
const RELOAD_LEN = 1.67;

/**
 * Source pose for the near-wall tuck.
 *
 * Not the relaxed idle, which is what this used to be. Measured on the open
 * floor across the locomotion set, the furthest any right-arm or torch box gets
 * from the character's centre is 0.484 m with `Idle_Loop` arms but only 0.368 m
 * with these — and the difference is concentrated exactly where it matters. A
 * hanging arm reaches 0.41-0.48 m out to the character's right and rear, which
 * is where practically every measured wall intersection was, so blending toward
 * it moved the torch *into* a wall alongside the body as often as out of one.
 * Folded arms pull the torch in against the chest instead.
 *
 * The trade is that the folded arms cross the chest, so reach forward and to
 * the *left* of the aim goes the other way, 0.13 m -> 0.37 m. That is what the
 * whole remaining residual is, and it is the cheap side of the deal: the
 * forward probe is the long one, so the character never gets that close to
 * something it is facing. Level-wide the worst torch intersection went from
 * 0.261 m to 0.077 m, and nothing anywhere reaches 0.10 m.
 *
 * Those numbers are from the torch era and from a gentler sweep than the one
 * quoted in player.ts, which samples twist as well and reports 0.142 m for the
 * pistol. The conclusion is unchanged and re-measured: folded arms still beat
 * hanging ones, and the residual is a sprinting arm, not the weapon.
 */
const TUCK_CLIP = "Idle_FoldArms_Loop";

/**
 * Pistol geometry, in the right hand's own frame.
 *
 * Measured off `Pistol_Idle_Loop` rather than assumed: in that pose the hand's
 * local +Y is (0.02, 0.19, 0.98) in rig space and its local +Z is
 * (-0.05, 0.98, -0.18), so +Y is the barrel direction and +Z is up out of the
 * slide. Every offset below is `f` metres along that forward axis and `u`
 * metres along that up axis, from the hand joint (which sits at the wrist, with
 * the fist box already spanning f = 0 .. 0.09).
 *
 * Nothing is offset sideways: at the game's 23 m camera distance the whole
 * weapon is about a dozen pixels long, so the budget goes on silhouette — a
 * long horizontal slide, a grip hanging below it, a light slung underneath —
 * and lateral detail would not survive the trip to the screen.
 *
 * The packed boxes, `weaponLight()` and `muzzle()` all derive from these, so
 * the light source and the model it comes out of cannot drift apart.
 */
const PISTOL = {
  /** Slide and frame: the one shape that has to read at distance. */
  slide: { f: 0.080, u: 0.028, hs: 0.020, hf: 0.080, hu: 0.024 },
  /** Grip, hanging below the fist and raked back like a real one. */
  grip: { f: 0.035, u: -0.055, hs: 0.019, hf: 0.026, hu: 0.048, rake: 0.30 },
  /** Weapon light, slung under the barrel ahead of the trigger guard. */
  light: { f: 0.075, u: -0.014, hs: 0.016, hf: 0.040, hu: 0.015 },
};
/** Bezel between the light body and its lens. */
const LIGHT_LENS_GAP = 0.005;
const LIGHT_LENS_HALF = 0.006;
/**
 * Hand joint to lens centre. This is where the light comes from.
 *
 * 0.120 m forward and 0.014 m below the wrist, against 0.196 m dead ahead of it
 * for the torch this replaces: a rail light sits over the trigger guard, not
 * out at the end of a fist-held tube.
 */
const LIGHT_LENS_F = PISTOL.light.f + PISTOL.light.hf + LIGHT_LENS_GAP;
const LIGHT_LENS_U = PISTOL.light.u;
/** Muzzle, at the front face of the slide. */
const MUZZLE_F = PISTOL.slide.f + PISTOL.slide.hf;
const MUZZLE_U = PISTOL.slide.u;

/**
 * Seconds of Pistol_Shoot ramp-in and ramp-out over the locomotion arms.
 *
 * The attack is set by measurement, not feel: on its own the clip moves the
 * right hand at most 0.0245 m per frame at 60 Hz, and the blend should not add
 * to that. Ramping in over 0.04 s peaks at 0.0362, over 0.05 s at 0.0292 —
 * both faster than the animation they are blending toward, which is a blend
 * artefact rather than recoil. 0.08 s peaks at 0.0147 and still reaches 0.048 m
 * of hand travel against the 0.050 m the clip asks for, so it keeps 96% of the
 * authored kick and contributes no snap of its own. Past that (0.12 s) the
 * recoil starts visibly softening.
 */
const SHOOT_ATTACK = 0.08;
const SHOOT_RELEASE = 0.18;
/**
 * Minimum seconds between shots.
 *
 * Shorter than the 0.633 s clip on purpose — a held trigger should retrigger
 * the recoil rather than wait for it to play out — but long enough that the
 * attack ramp is never cut off mid-blend, which is what makes a restart snap.
 */
const FIRE_COOLDOWN = 0.22;

/** Seconds for the upper-body aim layer to ramp in or out. */
const AIM_FADE = 0.22;

/** Hermite ease. Blend weights that start and stop linearly still read as a pop. */
function smoothstep(t: number): number {
  return t * t * (3 - 2 * t);
}

const MAX_BOXES = 32;

export class Character {
  private poseA: Pose;
  private poseB: Pose;
  private poseLoco: Pose;
  /** The folded-arms idle the wall tuck blends toward, on its own clock. */
  private poseTuck: Pose;
  /** The recoil one-shot, on its own clock. */
  private poseShoot: Pose;
  private poseReload: Pose;
  private poseMelee: Pose;
  /** Ping-pong targets for the layer stack, so no layer writes over its input. */
  private poseS0: Pose;
  private poseS1: Pose;
  private upperMask: Float32Array;
  private armsMask: Float32Array;

  private boxData = new Float32Array(MAX_BOXES * BOX_STRIDE_F32);
  private boxU32 = new Uint32Array(this.boxData.buffer);

  /** Cross-fade state between the outgoing and incoming locomotion clip. */
  private currentClip = PISTOL_IDLE;
  private prevClip = PISTOL_IDLE;
  private fade = 1;
  private clipTime = 0;
  private prevClipTime = 0;
  /** Runs at wall-clock rate: layered idles should not speed up with the legs. */
  private relaxTime = 0;
  private aimWeight = 0;
  /**
   * Seconds into the recoil one-shot, or null when not firing. Its own clock:
   * recoil is a property of the weapon, not of how fast the legs are moving.
   */
  private shootTime: number | null = null;
  private reloadTime: number | null = null;
  private meleeTime: number | null = null;
  /** Time since the last shot started, for the cooldown. */
  private sinceFire = FIRE_COOLDOWN;

  private segIndex: Array<{ from: number; to: number; seg: Segment }>;
  private handR: number;
  private head: number;
  private pelvis: number;
  /** Spine joints the aim twist is distributed across. */
  private spine: number[];
  private twist = 0;
  private twistQuat = new Float32Array(4);
  private tmpQuat = new Float32Array(4);

  /**
   * "nvg" is the player's lit tri-lens rig; "cap" is what guards wear. Both
   * cost exactly four head boxes — see CAP_CROWN.
   */
  headgear: "nvg" | "cap" = "nvg";

  /**
   * Holsters the weapon on the hip instead of holding it.
   *
   * Both hands are on a body while carrying one, so a pistol floating in a fist
   * reads as a bug. Same four boxes either way — the character's box count is
   * load-bearing for the renderer's dynamic-box grouping.
   */
  stowed = false;

  constructor(private rig: Rig) {
    const n = rig.boneCount;
    this.poseA = makePose(n);
    this.poseB = makePose(n);
    this.poseLoco = makePose(n);
    this.poseTuck = makePose(n);
    this.poseShoot = makePose(n);
    this.poseReload = makePose(n);
    this.poseMelee = makePose(n);
    this.poseS0 = makePose(n);
    this.poseS1 = makePose(n);
    this.upperMask = rig.maskFrom(UPPER_BODY);
    this.armsMask = rig.maskFrom(BOTH_ARMS);

    this.segIndex = SEGMENTS.map((seg) => {
      const from = rig.boneIndex.get(seg.from);
      const to = rig.boneIndex.get(seg.to);
      if (from === undefined || to === undefined) {
        throw new Error(`character rig missing bone: ${seg.from} or ${seg.to}`);
      }
      return { from, to, seg };
    });
    const hr = rig.boneIndex.get("hand_r");
    if (hr === undefined) throw new Error("character rig missing hand_r");
    this.handR = hr;
    const hd = rig.boneIndex.get("Head");
    if (hd === undefined) throw new Error("character rig missing Head");
    this.head = hd;
    const pv = rig.boneIndex.get("pelvis");
    if (pv === undefined) throw new Error("character rig missing pelvis");
    this.pelvis = pv;

    this.spine = ["spine_01", "spine_02", "spine_03"]
      .map((n) => rig.boneIndex.get(n))
      .filter((i): i is number => i !== undefined);
  }

  /**
   * Requests a locomotion clip, cross-fading from whatever is playing.
   *
   * `atTime` enters the clip already advanced. Needed where the *end* of a clip
   * is the pose being asked for rather than the clip as a performance — see the
   * takedown in guards.ts, which has to reach the death clip's resting pose
   * without replaying the fall that leads to it.
   */
  play(name: string, fadeSeconds = 0.18, atTime = 0): void {
    if (name === this.currentClip) return;
    const rate = fadeSeconds > 0 ? 1 / fadeSeconds : 1;

    // Asking again for the clip we are still fading *out* of: reverse the fade
    // rather than start a fresh one. Otherwise a tapped movement key blends
    // toward a walk pose, then blends back from a pure idle it was never in,
    // and the discontinuity between the two is a visible snap.
    if (name === this.prevClip && this.fade < 1) {
      const t = this.clipTime;
      this.clipTime = this.prevClipTime;
      this.prevClipTime = t;
      this.prevClip = this.currentClip;
      this.currentClip = name;
      this.fade = 1 - this.fade;
      this.fadeRate = rate;
      return;
    }

    this.prevClip = this.currentClip;
    this.prevClipTime = this.clipTime;
    this.currentClip = name;
    // Measured, against carrying the normalised gait phase across and against
    // resuming where this clip was last left. Both are the textbook answer and
    // both are slightly *worse* here (peak jerk over the transition 0.0126 ->
    // 0.0157 m/frame^2 for resume, 0.115 -> 0.171 for phase matching into
    // Sprint), because these clips are separately authored and their frame 0 is
    // a near-neutral contact pose rather than a common phase origin. The case
    // where entry time genuinely matters is re-entering a clip mid-fade, and
    // that is the reversal above.
    this.clipTime = atTime;
    this.fade = fadeSeconds > 0 ? 0 : 1;
    this.fadeRate = rate;
  }

  private fadeRate = 1;

  /**
   * Angle between where the legs point and where the character is aiming.
   *
   * The animation library has no strafe or turn clips, so sideways and backward
   * movement is produced the way engines normally do it: the lower body faces
   * the direction of travel and plays a forward locomotion cycle, while the
   * torso counter-rotates to keep the aim on the cursor.
   */
  setTwist(radians: number): void {
    this.twist = radians;
  }

  /**
   * Starts the recoil one-shot, unless one started too recently.
   *
   * @returns whether a shot actually went off, so the caller can hang a muzzle
   *          flash on the frame it happened without duplicating the cooldown.
   */
  /** True while the reload one-shot is playing; blocks firing. */
  get reloading(): boolean {
    return this.reloadTime !== null;
  }

  /** Starts the reload animation. Returns false if one is already running. */
  reload(): boolean {
    if (this.reloading) return false;
    this.reloadTime = 0;
    return true;
  }

  /** True while the takedown swing is playing. */
  get swinging(): boolean {
    return this.meleeTime !== null;
  }

  /** Starts the takedown swing. Returns false if one is already running. */
  melee(): boolean {
    if (this.meleeTime !== null) return false;
    this.meleeTime = 0;
    return true;
  }

  fire(): boolean {
    if (this.reloading || this.swinging) return false;
    if (this.sinceFire < FIRE_COOLDOWN) return false;
    this.sinceFire = 0;
    this.shootTime = 0;
    return true;
  }

  /** True while the recoil one-shot is playing out. */
  get firing(): boolean {
    return this.shootTime !== null;
  }

  /**
   * @param rate  playback rate, set by the caller from movement speed.
   * @param aiming layer the weapon-ready pose over the upper body.
   * @param tuck  0..1 pull of both arms into a low-ready pose, for wall proximity.
   */
  update(dt: number, rate: number, aiming: boolean, tuck = 0): void {
    const clip = this.rig.clip(this.currentClip);
    this.clipTime += dt * rate;
    this.prevClipTime += dt * rate;
    this.relaxTime += dt;
    this.sinceFire += dt;
    if (this.fade < 1) this.fade = Math.min(1, this.fade + dt * this.fadeRate);
    const fade = smoothstep(this.fade);

    this.rig.sample(clip, this.clipTime, this.poseA);
    let pose = this.poseA;
    if (this.fade < 1) {
      this.rig.sample(this.rig.clip(this.prevClip), this.prevClipTime, this.poseB);
      // Blend from the outgoing pose toward the incoming one.
      this.rig.blendInto(this.poseB, this.poseA, fade, this.poseLoco);
      pose = this.poseLoco;
    }

    // Layers write into an unused scratch pose so a layer never reads a buffer
    // it is halfway through overwriting.
    let scratch = 0;
    const layer = (base: Pose, over: Pose, weight: number, mask: Float32Array): Pose => {
      const out = scratch++ % 2 === 0 ? this.poseS0 : this.poseS1;
      this.rig.blendInto(base, over, weight, out, mask);
      return out;
    };

    // Ramped rather than switched: the caller flips `aiming` the instant the
    // player crosses the moving threshold, and a step change on a full-weight
    // upper-body layer is the pop you see leaving a standstill.
    const step = dt / AIM_FADE;
    this.aimWeight += clamp((aiming ? 1 : 0) - this.aimWeight, -step, step);
    const aim = smoothstep(this.aimWeight);
    if (aim > 1e-3) {
      this.rig.sample(this.rig.clip(PISTOL_IDLE), this.clipTime, this.poseB);
      pose = layer(pose, this.poseB, aim, this.upperMask);
    }

    // Recoil, over the same upper-body mask as the aim pose rather than a
    // parallel one, so firing while running cannot touch the legs: the mask is
    // rooted at spine_02 and the pelvis, thighs and spine_01 are outside it.
    if (this.shootTime !== null) {
      const t = this.shootTime;
      const dur = this.rig.clip(SHOOT_CLIP).duration;
      this.shootTime = t + dt;
      if (this.shootTime > dur) this.shootTime = null;
      // Both ends of the clip are the neutral grip, so this envelope is only
      // there to cover the blend against locomotion arms mid-stride.
      const w = smoothstep(clamp(t / SHOOT_ATTACK, 0, 1)) *
        smoothstep(clamp((dur - t) / SHOOT_RELEASE, 0, 1));
      if (w > 1e-3) {
        this.rig.sample(this.rig.clip(SHOOT_CLIP), t, this.poseShoot);
        pose = layer(pose, this.poseShoot, w, this.upperMask);
      }
    }

    // Reload, same upper-body mask. Full weight rather than an envelope: unlike
    // the recoil one-shot this clip does not start and end on the neutral grip,
    // so ramping it would blend a half-inserted magazine against the ready pose.
    if (this.reloadTime !== null) {
      const t = this.reloadTime;
      this.reloadTime = t + dt;
      if (this.reloadTime > RELOAD_LEN) this.reloadTime = null;
      const w = smoothstep(clamp(t / SHOOT_ATTACK, 0, 1)) *
        smoothstep(clamp((RELOAD_LEN - t) / SHOOT_RELEASE, 0, 1));
      if (w > 1e-3) {
        this.rig.sample(this.rig.clip(RELOAD_CLIP), t, this.poseReload);
        pose = layer(pose, this.poseReload, w, this.upperMask);
      }
    }

    // Takedown swing, over the same upper-body mask. Full weight at the peak:
    // this is a whole-arm motion and blending it at half strength against the
    // pistol grip reads as a shrug rather than a strike.
    if (this.meleeTime !== null) {
      const t = this.meleeTime;
      this.meleeTime = t + dt;
      if (this.meleeTime > MELEE_LEN) this.meleeTime = null;
      const w = smoothstep(clamp(t / 0.06, 0, 1)) *
        smoothstep(clamp((MELEE_LEN - t) / 0.14, 0, 1));
      if (w > 1e-3) {
        this.rig.sample(this.rig.clip(MELEE_CLIP), t, this.poseMelee);
        pose = layer(pose, this.poseMelee, w, this.upperMask);
      }
    }

    // Last, so it wins over the ready pose: near a wall the arms come in and
    // the weapon — whose position is taken from the hand — comes back out of it.
    if (tuck > 1e-3) {
      this.rig.sample(this.rig.clip(TUCK_CLIP), this.relaxTime, this.poseTuck);
      pose = layer(pose, this.poseTuck, tuck, this.armsMask);
    }

    // Distribute the twist across the spine joints so the bend is gradual
    // rather than a hinge at one vertebra. Post-multiplying rotates each joint
    // about its own axis, which for this rig is local +Y.
    if (Math.abs(this.twist) > 1e-4 && this.spine.length > 0) {
      const half = this.twist / this.spine.length / 2;
      this.twistQuat[0] = 0;
      this.twistQuat[1] = Math.sin(half);
      this.twistQuat[2] = 0;
      this.twistQuat[3] = Math.cos(half);
      for (const b of this.spine) {
        quatMul(pose.rot, b * 4, this.twistQuat, 0, this.tmpQuat, 0);
        pose.rot.set(this.tmpQuat, b * 4);
      }
    }

    this.rig.computeWorld(pose);
  }

  /** Rig-space position of the right hand, which is the pistol's grip. */
  handPosition(out: Vec3): void {
    const i = this.handR * 3;
    out.x = this.rig.worldPos[i];
    out.y = this.rig.worldPos[i + 1];
    out.z = this.rig.worldPos[i + 2];
  }

  /**
   * Packs the limb boxes.
   *
   * @param origin  world position of the character's feet
   * @param yaw     facing; the rig authors +Z forward, matching our convention
   * @param lightOn packs the weapon light's emissive lens
   */
  buildBoxes(
    origin: Vec3,
    yaw: number,
    m: CharacterMaterials,
    lightOn: boolean,
  ): { data: Float32Array<ArrayBuffer>; count: number } {
    const d = this.boxData;
    const u = this.boxU32;
    const cy = Math.cos(yaw);
    const sy = Math.sin(yaw);
    const wp = this.rig.worldPos;

    // Rig space -> world: yaw about Y, then translate to the character's feet.
    const toWorld = (x: number, y: number, z: number): [number, number, number] => [
      origin.x + cy * x + sy * z,
      origin.y + y,
      origin.z - sy * x + cy * z,
    ];
    /** Rotates a rig-space direction into world by the character's yaw. */
    const rot = (x: number, y: number, z: number): [number, number, number] =>
      [cy * x + sy * z, y, -sy * x + cy * z];

    let i = 0;
    const tmp = new Float32Array(3);

    for (const { from, to, seg } of this.segIndex) {
      let ax = wp[from * 3], ay = wp[from * 3 + 1], az = wp[from * 3 + 2];
      let bx = wp[to * 3], by = wp[to * 3 + 1], bz = wp[to * 3 + 2];

      // Degenerate segments (head, hands) have no second joint, so extend along
      // the bone's own +Y, which is the rig's bone axis.
      if (from === to) {
        quatRotate(this.rig.worldRot, from * 4, 0, 1, 0, tmp, 0);
        bx = ax + tmp[0] * (seg.extend ?? 0.1);
        by = ay + tmp[1] * (seg.extend ?? 0.1);
        bz = az + tmp[2] * (seg.extend ?? 0.1);
      } else if (seg.extend) {
        const ex = bx - ax, ey = by - ay, ez = bz - az;
        const el = Math.hypot(ex, ey, ez) || 1;
        bx += (ex / el) * seg.extend;
        by += (ey / el) * seg.extend;
        bz += (ez / el) * seg.extend;
      }

      let dx = bx - ax, dy = by - ay, dz = bz - az;
      let len = Math.hypot(dx, dy, dz);
      if (len < 1e-5) { dx = 0; dy = 1; dz = 0; len = 1e-5; }
      dx /= len; dy /= len; dz /= len;

      // Build a basis with local +Y along the bone. Use the joint's own
      // rotation for the twist reference so limbs roll with the animation
      // rather than snapping when the segment passes vertical.
      quatRotate(this.rig.worldRot, from * 4, 0, 0, 1, tmp, 0);
      let rx = tmp[1] * dz - tmp[2] * dy;
      let ry = tmp[2] * dx - tmp[0] * dz;
      let rz = tmp[0] * dy - tmp[1] * dx;
      let rl = Math.hypot(rx, ry, rz);
      if (rl < 1e-4) {
        // Reference parallel to the bone — pick any perpendicular.
        rx = 1; ry = 0; rz = 0;
        const dot = dx;
        rx -= dot * dx; ry -= dot * dy; rz -= dot * dz;
        rl = Math.hypot(rx, ry, rz) || 1;
      }
      rx /= rl; ry /= rl; rz /= rl;
      const fx = dy * rz - dz * ry;
      const fy = dz * rx - dx * rz;
      const fz = dx * ry - dy * rx;

      const [wcx, wcy, wcz] = toWorld((ax + bx) / 2, (ay + by) / 2, (az + bz) / 2);

      const R0 = rot(rx, ry, rz);
      const R1 = rot(dx, dy, dz);
      const R2 = rot(fx, fy, fz);

      const o = i * BOX_STRIDE_F32;
      d[o] = wcx; d[o + 1] = wcy; d[o + 2] = wcz;
      u[o + 3] = m[seg.mat];
      d[o + 4] = seg.thick; d[o + 5] = len / 2; d[o + 6] = seg.thick2 ?? seg.thick;
      u[o + 7] = 0;
      d[o + 8] = R0[0]; d[o + 9] = R0[1]; d[o + 10] = R0[2]; d[o + 11] = 0;
      d[o + 12] = R1[0]; d[o + 13] = R1[1]; d[o + 14] = R1[2]; d[o + 15] = 0;
      d[o + 16] = R2[0]; d[o + 17] = R2[1]; d[o + 18] = R2[2]; d[o + 19] = 0;
      i++;
    }

    // Night-vision goggles, in the Head bone's frame so they track head motion.
    const headSide = new Float32Array(3);
    const headUp = new Float32Array(3);
    const headFwd = new Float32Array(3);
    quatRotate(this.rig.worldRot, this.head * 4, 1, 0, 0, headSide, 0);
    quatRotate(this.rig.worldRot, this.head * 4, 0, 1, 0, headUp, 0);
    quatRotate(this.rig.worldRot, this.head * 4, 0, 0, 1, headFwd, 0);
    const hx = wp[this.head * 3];
    const hy = wp[this.head * 3 + 1];
    const hz = wp[this.head * 3 + 2];

    const writeHeadBox = (
      lx: number, lyy: number, lz: number,
      ex: number, ey: number, ez: number,
      mat: number, flags: number,
    ) => {
      const [wx, wy, wz] = toWorld(
        hx + headSide[0] * lx + headUp[0] * lyy + headFwd[0] * lz,
        hy + headSide[1] * lx + headUp[1] * lyy + headFwd[1] * lz,
        hz + headSide[2] * lx + headUp[2] * lyy + headFwd[2] * lz,
      );
      const A = rot(headSide[0], headSide[1], headSide[2]);
      const B = rot(headUp[0], headUp[1], headUp[2]);
      const C = rot(headFwd[0], headFwd[1], headFwd[2]);
      const o = i * BOX_STRIDE_F32;
      d[o] = wx; d[o + 1] = wy; d[o + 2] = wz;
      u[o + 3] = mat;
      d[o + 4] = ex; d[o + 5] = ey; d[o + 6] = ez;
      u[o + 7] = flags;
      d[o + 8] = A[0]; d[o + 9] = A[1]; d[o + 10] = A[2]; d[o + 11] = 0;
      d[o + 12] = B[0]; d[o + 13] = B[1]; d[o + 14] = B[2]; d[o + 15] = 0;
      d[o + 16] = C[0]; d[o + 17] = C[1]; d[o + 18] = C[2]; d[o + 19] = 0;
      i++;
    };

    // Exactly four boxes either way — see CAP_CROWN.
    if (this.headgear === "cap") {
      writeHeadBox(
        CAP_CROWN.x, CAP_CROWN.y, CAP_CROWN.z,
        CAP_CROWN.hx, CAP_CROWN.hy, CAP_CROWN.hz,
        m.clothDark, 0,
      );
      for (const b of CAP_BRIM) {
        writeHeadBox(b.x, b.y, b.z, b.hx, b.hy, b.hz, m.clothDark, 0);
      }
    } else {
      writeHeadBox(
        NVG_MOUNT.x, NVG_MOUNT.y, NVG_MOUNT.z,
        NVG_MOUNT.hx, NVG_MOUNT.hy, NVG_MOUNT.hz,
        m.metal, 0,
      );
      for (const lens of NVG_LENSES) {
        writeHeadBox(
          lens.x, lens.y, NVG_LENS_Z,
          NVG_LENS_R, NVG_LENS_R, 0.012,
          m.nvgLens, FLAG_EMISSIVE,
        );
      }
    }

    // Pistol, in the right hand's frame. Local +X across the slide, +Y along the
    // barrel, +Z up out of it — the hand's own axes, so nothing here has to
    // re-derive an orientation the animation already carries.
    const gunSide = new Float32Array(3);
    const gunFwd = new Float32Array(3);
    const gunUp = new Float32Array(3);
    // Holstered, the weapon rides the pelvis in world-aligned axes rather than
    // the hand's: the hand is busy, and a holster does not swing with the arm.
    const frameBone = this.stowed ? this.pelvis : this.handR;
    if (this.stowed) {
      gunSide.set([1, 0, 0]);
      gunFwd.set([0, -1, 0]);
      gunUp.set([0, 0, 1]);
    } else {
      quatRotate(this.rig.worldRot, this.handR * 4, 1, 0, 0, gunSide, 0);
      quatRotate(this.rig.worldRot, this.handR * 4, 0, 1, 0, gunFwd, 0);
      quatRotate(this.rig.worldRot, this.handR * 4, 0, 0, 1, gunUp, 0);
    }
    const hp = frameBone * 3;
    // Offset onto the right hip when holstered.
    const stowX = this.stowed ? 0.17 : 0;
    const stowY = this.stowed ? -0.06 : 0;

    /**
     * @param rake tips the box back about the lateral axis, for the grip. The
     *             offset is taken along the untilted axes, so the box pivots
     *             about its own centre and stays attached to the frame.
     */
    const writeGunBox = (
      f: number, uOff: number,
      hs: number, hf: number, hu: number,
      mat: number, flags: number, rake = 0,
    ) => {
      const cr = Math.cos(rake);
      const sr = Math.sin(rake);
      const [wx, wy, wz] = toWorld(
        wp[hp] + gunSide[0] * stowX + gunFwd[0] * (f + stowY) + gunUp[0] * uOff,
        wp[hp + 1] + gunSide[1] * stowX + gunFwd[1] * (f + stowY) + gunUp[1] * uOff,
        wp[hp + 2] + gunSide[2] * stowX + gunFwd[2] * (f + stowY) + gunUp[2] * uOff,
      );
      const A = rot(gunSide[0], gunSide[1], gunSide[2]);
      const B = rot(
        gunFwd[0] * cr - gunUp[0] * sr,
        gunFwd[1] * cr - gunUp[1] * sr,
        gunFwd[2] * cr - gunUp[2] * sr,
      );
      const C = rot(
        gunUp[0] * cr + gunFwd[0] * sr,
        gunUp[1] * cr + gunFwd[1] * sr,
        gunUp[2] * cr + gunFwd[2] * sr,
      );
      const o = i * BOX_STRIDE_F32;
      d[o] = wx; d[o + 1] = wy; d[o + 2] = wz;
      u[o + 3] = mat;
      d[o + 4] = hs; d[o + 5] = hf; d[o + 6] = hu;
      u[o + 7] = flags;
      d[o + 8] = A[0]; d[o + 9] = A[1]; d[o + 10] = A[2]; d[o + 11] = 0;
      d[o + 12] = B[0]; d[o + 13] = B[1]; d[o + 14] = B[2]; d[o + 15] = 0;
      d[o + 16] = C[0]; d[o + 17] = C[1]; d[o + 18] = C[2]; d[o + 19] = 0;
      i++;
    };

    const s = PISTOL.slide;
    writeGunBox(s.f, s.u, s.hs, s.hf, s.hu, m.metal, 0);
    const g = PISTOL.grip;
    writeGunBox(g.f, g.u, g.hs, g.hf, g.hu, m.clothDark, 0, g.rake);
    const wl = PISTOL.light;
    writeGunBox(wl.f, wl.u, wl.hs, wl.hf, wl.hu, m.metal, 0);
    // The lens is always emitted, lit or not.
    //
    // Emitting it conditionally made a character's box count vary with the
    // torch switch, which quietly breaks the renderer: dynamic boxes are grouped
    // by a fixed stride, so a 25-box character puts every later character out of
    // alignment with its group AABB, and the gameplay light probe — which skips
    // its own group to avoid being shadowed by its own torso — would then skip
    // only part of the body and read as permanently in darkness.
    //
    // A switched-off lens is still a lens, so this is also just more honest.
    // Emissive-flagged either way, so it never occludes its own beam.
    writeGunBox(
      LIGHT_LENS_F, LIGHT_LENS_U,
      0.012, LIGHT_LENS_HALF, 0.012,
      lightOn ? m.lens : m.metal, FLAG_EMISSIVE,
    );

    return { data: d, count: i };
  }

  /**
   * Rig-space point `f` along and `uOff` above the right hand's own axes,
   * mapped into world. Shared by the light and the muzzle so both come out of
   * the same hand transform the packed boxes used.
   */
  private handPoint(origin: Vec3, yaw: number, f: number, uOff: number): {
    pos: Vec3; dir: Vec3;
  } {
    const cy = Math.cos(yaw);
    const sy = Math.sin(yaw);
    const fwd = new Float32Array(3);
    const up = new Float32Array(3);
    quatRotate(this.rig.worldRot, this.handR * 4, 0, 1, 0, fwd, 0);
    quatRotate(this.rig.worldRot, this.handR * 4, 0, 0, 1, up, 0);
    const wp = this.rig.worldPos;
    const hp = this.handR * 3;
    const lx = wp[hp] + fwd[0] * f + up[0] * uOff;
    const ly = wp[hp + 1] + fwd[1] * f + up[1] * uOff;
    const lz = wp[hp + 2] + fwd[2] * f + up[2] * uOff;
    return {
      pos: v3(origin.x + cy * lx + sy * lz, origin.y + ly, origin.z - sy * lx + cy * lz),
      dir: v3(cy * fwd[0] + sy * fwd[2], fwd[1], -sy * fwd[0] + cy * fwd[2]),
    };
  }

  /**
   * World position and barrel direction of the weapon light, from the animated
   * hand. Replaces the hand-held torch.
   *
   * `dir` is what the light is physically pointing along. Whether to use it is
   * the caller's business — see Player.flashlightDir, which deliberately does
   * not, because a beam that bobbed with the walk cycle is unusable to play
   * with. It is the right direction for a muzzle flash, which is one frame long
   * and should come out of the barrel.
   *
   * How far apart the two are, measured: standing in `Pistol_Idle_Loop` the
   * barrel is 1.3 degrees off the body yaw, but layered over locomotion it sits
   * 14-15 degrees off it (walk 14.5, jog 14.3, sprint 15.0 mean). That is not
   * the arms wandering — it is the pelvis. `Pistol_Idle_Loop` is authored in a
   * bladed stance with the hips turned 13.3 degrees, and the pelvis is below
   * the aim mask because thigh_l/thigh_r hang off it, so the legs would snap if
   * it were in. Anything that wants the barrel to agree with the cursor should
   * use the aim, not this. Pitch varies the same way: +10.7 degrees standing,
   * -16.7 sprinting.
   *
   * Only valid for whichever Character posed the rig last: `Rig.computeWorld`
   * writes into shared scratch, so read this immediately after `update()` and
   * before anybody else animates. See the note at the top of guards.ts.
   */
  weaponLight(origin: Vec3, yaw: number): { pos: Vec3; dir: Vec3 } {
    return this.handPoint(origin, yaw, LIGHT_LENS_F, LIGHT_LENS_U);
  }

  /**
   * World position and direction of the muzzle, for hanging a flash on.
   * Same latching rule as `weaponLight`.
   */
  muzzle(origin: Vec3, yaw: number): { pos: Vec3; dir: Vec3 } {
    return this.handPoint(origin, yaw, MUZZLE_F, MUZZLE_U);
  }
}
