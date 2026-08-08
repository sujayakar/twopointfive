// Character = animated mannequin instance: locomotion/upper-body animation graph, procedural aim twist,
// world placement, bone queries and RT proxy boxes — and, once dead, a ragdoll (ragdoll.ts) that poses the same skeleton.
import { Mat4, Quat, Vec3, m4, quat, v3, clamp, damp, wrapAngle, lerp, smoothstep } from '../math/vec';
import { GltfCharacter, Clip } from '../anim/gltf';
import { NodeOverride, Pose, PoseFK, SkeletonInstance, TwistSpec, blendPose, buildMask, restPose, rotateLocal, rotateModel, sampleClip, setModelRot, twoBoneIK } from '../anim/skeleton';
import { Ragdoll, RagdollWorld, RAG } from './ragdoll';

export type UpperMode = 'none' | 'relaxed' | 'aim' | 'torch';
/** Procedural ready postures layered over the locomotion (see CharacterAnimator.stance). */
export type Stance = 'none' | 'highReady' | 'lowReady' | 'stack' | 'wall';
/** Left-hand signals (CharacterAnimator.signal). */
export type SignalKind = 'hold' | 'go' | 'rally';
/** The grab-from-behind beat both figures share (player.ts Hold / Guard.held): the reach (grab), the hold itself, the choke (arm variant) or the pistol-whip (gun
 *  variant), the shove that lets him go. */
export type HoldPhase = 'grab' | 'held' | 'choke' | 'whip' | 'release';
/** Which hold it is, fixed at the grab by what was in Sam's hand: 'arm' — both hands on the man (the pistol, if any, rides the thigh); 'gun' — the left forearm
 *  across his throat as before, the Five-seveN in the right hand up beside his head (fired past him, or brought down on it). */
export type HoldVariant = 'arm' | 'gun';
/** What the controller tells each figure's layer every frame of a hold (CharacterAnimator.holdPose = Sam, .heldPose = the man): the phase and the seconds into
 *  it, and the variant (absent = 'arm'). Sam's gun hold also carries how far the pistol is presented off the man's head to fire past him (gunOut 0‥1, eased by
 *  the controller) and the pitch it is presented at (rad, + up: the aim). The man's carries how rattled he is (0‥1, the dialogue engine's agitation — the
 *  struggle's amplitude, so body and mouth agree) and, in the gun hold, the seconds since a round last went past his ear (shot; absent / < 0 = none lately). */
export interface HoldPose { phase: HoldPhase; t: number; variant?: HoldVariant; gunOut?: number; gunPitch?: number; }
export interface HeldPose { phase: HoldPhase; t: number; agitation: number; variant?: HoldVariant; shot?: number; }
/** The hold's contact constants (docs/internal/grab-interrogate-design.md §3). The two roots are locked HOLD.dist apart on ONE facing, so a point on the other man
 *  is a constant in each figure's own model space (x = the figure's left, y up, z ahead of its root) — no bone hand-off between the characters. Timings are the
 *  controller's too (player.ts reads secs). First-pass numbers off the mannequin's measured joints; the viewer will get a page to tune them like RIG. */
export const HOLD = {
  /** his root this far dead ahead of Sam's, both on Sam's facing: chest to his back — any further and no arm reaches round a throat with the forearm ACROSS it
   *  (the mannequin's upper arm is 0.27: the elbow has to sit on his left collarbone) */
  dist: 0.27,
  /** seconds: the reach before he is held · Space held this long on his row starts the choke (arm) / the whip (gun) · of tightening before he goes limp · from the
   *  pistol coming off his head to it landing behind his ear (the strike arrives AT this instant; Space up sooner and it never does) · the shove */
  secs: { grab: 0.45, chokeHold: 0.35, choke: 2.6, whip: 0.6, release: 0.35 },
  // ---- Sam, his model space (the man's root sits at [0, 0, dist]; his neck joint stands about [0, 1.43, dist − 0.02], the front of his throat ~7 cm ahead of that)
  throat: [-0.11, 1.44, 0.35] as Vec3,        // left wrist: past the middle of the throat to its far side, the forearm's line a finger off the throat — so it lies across it, elbow on his left collarbone
  viaL: [0.38, 1.42, 0.22] as Vec3, overL: [0.16, 1.52, 0.33] as Vec3,   // …reached round the OUTSIDE of his left shoulder and OVER his collarbone onto the throat (the grab's mid keys), never through him
  tight: [-0.08, 1.45, 0.33] as Vec3,         // the choke: the elbow closes, the wrist comes back a finger toward Sam's own chest (the throat gives; the spine behind it does not)
  farShoulder: [-0.21, 1.38, 0.24] as Vec3,   // right wrist: the hand clamped over the front of his right shoulder
  viaR: [-0.36, 1.47, 0.10] as Vec3,          // …up behind his right shoulder, then down over it
  shove: [[0.14, 1.33, 0.30], [-0.14, 1.33, 0.30]] as Vec3[],   // release: both palms into his shoulder blades (left, right)
  lean: 8, chokeLean: -4, shoveLean: 12,      // degrees over the three spine joints: chest onto his back · leaning back to finish it · into the shove
  side: 9,                                    // degrees of side-bend (upper spine + neck) carrying Sam's head out to the right of the man's — cheek by his right ear, not skull through skull
  headYaw: -22, headPitch: 6,                 // he looks past the man's right ear, chin down a touch
  // ---- the man, his own model space (Sam's forearm crosses in front of his throat: elbow about [0.14, 1.44, 0.06], wrist [-0.11, 1.44, 0.08])
  clawL: [0.09, 1.39, 0.13] as Vec3,          // left wrist: just below and ahead of the forearm, the hand hooked up and back over it
  torchUp: [-0.32, 1.55, 0.15] as Vec3,       // right (torch / pistol) wrist: up and out and useless — the beam wanders over the ceiling
  viaUp: [-0.38, 1.20, 0.12] as Vec3,
  clawR: [-0.15, 1.39, 0.14] as Vec3,         // …and onto the forearm as well once the choke bites
  hang: [[0.27, 0.98, 0.14], [-0.27, 0.98, 0.14]] as Vec3[],     // limp: the hands fall away (left, right)
  flail: [[0.30, 1.18, 0.42], [-0.30, 1.22, 0.44]] as Vec3[],    // shoved off: arms thrown forward for balance
  arch: -5, chinUp: -9, aside: 8, clavUp: 8,  // degrees: spine arched back over three joints, head pulled back and tipped away to his left (off Sam's face), shoulders hunched up
  loll: 26, lollRoll: 11,                     // degrees the head drops forward / tips as he goes
  sag: [0.02, 0.06] as [number, number],      // m the pelvis sinks onto the knees: held … out on his feet at the end of the choke (no further: the arm under his jaw carries him until it lets go)
  // ---- the gun variant (HoldVariant 'gun'): the left arm exactly as above; the right hand keeps the Five-seveN. Measured, not wished: wrist to the can's mouth is
  //      0.43 m and his temple stands 0.36 m ahead of Sam's shoulder, so a muzzle pressed INTO the temple folds the arm double behind Sam's own ear — instead the
  //      forearm lies over the man's right shoulder and the can runs along his right temple a few centimetres off the skin, mouth just ahead of his brow, toed in
  //      and tipped down at his head: 'gun to his head' from the game's camera, and already pointing past him when it fires. Sam's head goes out to the LEFT here
  //      (the gun has the right). Directions are model-space [left, up, ahead] and get normalized on use.
  gun: {
    hand: [-0.20, 1.53, 0.15] as Vec3, bore: [0.21, -0.10, 0.97] as Vec3,        // at rest: wrist over his right shoulder, bore toed in ~12° and down ~6° across his temple
    outHand: [-0.24, 1.53, 0.17] as Vec3,                                         // presented to fire past him: a hand further out and forward, bore dead ahead at the aim's pitch (HoldPose.gunPitch)
    kickPitch: 6, kickBack: 0.03,                                                 // recoil: muzzle flip (°) and the hand coming back (m), ~35 ms up, ~0.2 s down (the drawn pistol rides the aim while the shot plays, like every carry's: keep the flip to what the wrist shows)
    viaA: [-0.42, 1.24, 0.06] as Vec3, viaB: [-0.40, 1.50, 0.02] as Vec3,         // the grab: the gun hand swung out wide to the right and up round him — never through his back
    viaBore: [0.1, 0.6, 0.8] as Vec3,                                             // …muzzle high while it travels
    windHand: [-0.36, 1.68, -0.04] as Vec3, windBore: [-0.15, 0.8, 0.58] as Vec3, // the whip: cocked back, up and out (muzzle skyward)…
    hitHand: [-0.10, 1.62, 0.15] as Vec3, hitBore: [0.5, 0.3, 0.8] as Vec3,       // …and brought down onto the skull behind his right ear (the frame just ahead of and below the wrist is what lands: it meets the bone, the boxes say so)
    thruHand: [-0.06, 1.50, 0.20] as Vec3,                                        // follow-through past the contact as he drops out of the arm (played on the layer's way out)
    shoveBore: [0.15, 0.6, 0.78] as Vec3,                                         // the release: the fist into his shoulder blade with the muzzle up past his head, not into his back
    lean: 5, side: -8, headYaw: 19, headPitch: 4,                                 // Sam: less chest on him; head carried out to the left, looking past the man's LEFT ear
    windYaw: -8, hitYaw: 5,                                                       // chest loading back to the right in the wind-up, coming through with the strike (°)
    clav: 7,                                                                      // right clavicle up with the raised gun arm (°)
    // the man, his own model space
    handsUp: [[0.27, 1.43, 0.31], [-0.34, 1.41, 0.29]] as Vec3[],                 // hands half raised in front of his shoulders, palms forward (the torch / pistol in the right points at the ceiling)
    aside: 13, awayYaw: 6, chinUp: -6,                                            // head tipped and turned away to his left off the can at his right temple, chin not so high
    duck: { yaw: 5, roll: 9, pitch: 10, hunch: 6, secs: 0.45 },                   // the flinch when a round goes past his ear (°, and how long it takes to come back)
    knock: { yaw: 24, roll: 16, pitch: 18 },                                      // the head going with the blow over the strike's last 0.1 s (seeds the ragdoll's head that way)
  },
};
/** the whip's shape inside HOLD.secs.whip: wound up by WIND, the strike leaving at STRIKE and ARRIVING at secs.whip (fast, like the kick meeting the door); the
 *  follow-through runs THRU past it while the layer eases out */
const WHIP_WIND = 0.36, WHIP_STRIKE = 0.42, WHIP_THRU = 0.14;

export interface AnimParams { walkSpeed: number; jogSpeed: number; crouchSpeed: number; sprintSpeed: number; }
export const defaultAnimParams: AnimParams = { walkSpeed: 1.45, jogSpeed: 3.3, crouchSpeed: 1.05, sprintSpeed: 4.7 };

const N = {
  hips: 'DEF-hips', spine1: 'DEF-spine.001', spine2: 'DEF-spine.002', spine3: 'DEF-spine.003', neck: 'DEF-neck', head: 'DEF-head',
  upperArmL: 'DEF-upper_arm.L', forearmL: 'DEF-forearm.L', handL: 'DEF-hand.L', upperArmR: 'DEF-upper_arm.R', forearmR: 'DEF-forearm.R', handR: 'DEF-hand.R',
  thighL: 'DEF-thigh.L', shinL: 'DEF-shin.L', footL: 'DEF-foot.L', toeL: 'DEF-toe.L', thighR: 'DEF-thigh.R', shinR: 'DEF-shin.R', footR: 'DEF-foot.R', toeR: 'DEF-toe.R',
  shoulderL: 'DEF-shoulder.L', shoulderR: 'DEF-shoulder.R', middle1R: 'DEF-f_middle.01.R', middle1L: 'DEF-f_middle.01.L',
};
const DEGR = Math.PI / 180;
const AX: Vec3 = [1, 0, 0], AY: Vec3 = [0, 1, 0], AZ: Vec3 = [0, 0, 1];
/** model-axis rotations: pitch (+ = bow forward), yaw (+ = turn to the figure's left), roll (+ = left shoulder up) */
const qPitch = (a: number) => quat.axisAngle(AX, a), qYaw = (a: number) => quat.axisAngle(AY, a), qRoll = (a: number) => quat.axisAngle(AZ, a);

/** Critically damped follower (value + velocity): unlike damp() it also eases IN, so a posture change starts as softly as it ends. w ≈ 6 / settle time. */
class Spring {
  v = 0; constructor(public x = 0) {}
  step(target: number, w: number, dt: number) { const y = this.x - target, j = this.v + y * w, e = Math.exp(-w * dt); this.x = target + (y + j * dt) * e; this.v = (this.v - j * w * dt) * e; return this.x; }
}
/** One hand-animated channel of a one-shot: Catmull-Rom through (time s, value) keys — passes through every key with a continuous slope (no stop-start at the
 *  keys, a little natural overshoot between them), flat at the first and last key, clamped outside. */
type Keys = readonly (readonly [number, number])[];
function curve(t: number, K: Keys): number {
  const n = K.length; if (t <= K[0][0]) return K[0][1]; if (t >= K[n - 1][0]) return K[n - 1][1];
  let i = 0; while (t > K[i + 1][0]) i++;
  const t0 = K[i][0], p0 = K[i][1], t1 = K[i + 1][0], p1 = K[i + 1][1]; const h = t1 - t0, u = (t - t0) / h;
  const m0 = i > 0 ? (p1 - K[i - 1][1]) / (t1 - K[i - 1][0]) : 0, m1 = i + 2 < n ? (K[i + 2][1] - p0) / (K[i + 2][0] - t0) : 0;
  const u2 = u * u, u3 = u2 * u;
  return (2 * u3 - 3 * u2 + 1) * p0 + (u3 - 2 * u2 + u) * h * m0 + (3 * u2 - 2 * u3) * p1 + (u3 - u2) * h * m1;
}
/** the same spline for a keyed model-space vector (hand paths, palm / finger directions): one segment search, Catmull-Rom per component */
type Keys3 = readonly (readonly [number, readonly number[]])[];
function curve3(t: number, K: Keys3): Vec3 {
  const n = K.length; if (t <= K[0][0]) { const p = K[0][1]; return [p[0], p[1], p[2]]; } if (t >= K[n - 1][0]) { const p = K[n - 1][1]; return [p[0], p[1], p[2]]; }
  let i = 0; while (t > K[i + 1][0]) i++;
  const t0 = K[i][0], t1 = K[i + 1][0], P0 = K[i][1], P1 = K[i + 1][1], Km = i > 0 ? K[i - 1] : null, Kp = i + 2 < n ? K[i + 2] : null;
  const h = t1 - t0, u = (t - t0) / h, u2 = u * u, u3 = u2 * u;
  const a = 2 * u3 - 3 * u2 + 1, b = (u3 - 2 * u2 + u) * h, c = 3 * u2 - 2 * u3, d = (u3 - u2) * h;
  const out: Vec3 = [0, 0, 0];
  for (let k = 0; k < 3; k++) {
    const m0 = Km ? (P1[k] - Km[1][k]) / (t1 - Km[0]) : 0, m1 = Kp ? (Kp[1][k] - P0[k]) / (Kp[0] - t0) : 0;
    out[k] = a * P0[k] + b * m0 + c * P1[k] + d * m1;
  }
  return out;
}
/** IK pole for an arm from a swivel angle: the direction square to the shoulder→hand offset `h` at angle `psi` in the basis (b1 = `ref` made ⊥ h, b2 = ĥ × b1).
 *  `ref` is a fixed direction the hand never points along (or against), so the basis — and with it the elbow — turns continuously wherever the hand goes. */
function swivelPole(h: Vec3, ref: Vec3, psi: number): Vec3 {
  const hn = v3.normalize(h); const b1 = v3.normalize(v3.mad(ref, hn, -v3.dot(ref, hn))); const b2 = v3.cross(hn, b1);
  return v3.mad(v3.scale(b1, Math.cos(psi)), b2, Math.sin(psi));
}
/** those references for the right / left arm: across the body, a little up and back — ≥ 44° off every bearing either hand takes in the clips and layers here */
const SWIV_REF_R: Vec3 = [0.92, 0.276, -0.276], SWIV_REF_L: Vec3 = [-0.92, 0.276, -0.276];
/** cubic Hermite segment with explicit end slopes (a one-shot phase that must arrive at speed — the kick meeting the door — instead of easing in) */
function hermite(t: number, t0: number, p0: number, m0: number, t1: number, p1: number, m1: number): number {
  const h = t1 - t0, u = clamp((t - t0) / h, 0, 1), u2 = u * u, u3 = u2 * u;
  return (2 * u3 - 3 * u2 + 1) * p0 + (u3 - 2 * u2 + u) * h * m0 + (3 * u2 - 2 * u3) * p1 + (u3 - u2) * h * m1;
}
/** smooth 0→1→0 window: rises over [a,b], falls over [c,d] (smoothstep flanks) */
const bump = (t: number, a: number, b: number, c: number, d: number) => smoothstep(a, b, t) * (1 - smoothstep(c, d, t));

/** The continuous shape a stance drives toward (springs interpolate between these, so high ↔ low ↔ stack morph instead of cross-fading through the base pose).
 *  Hand offsets are from the chest joint (spine.003) along MODEL axes — the gun is carried relative to the body but kept level in the world, so a crouched or
 *  leaning torso does not point it at the floor. Angles in degrees here, converted on use. */
interface StanceShape {
  fwd: number; up: number; side: number;   // right-hand (gun hand) position: m ahead of / above / to the figure's left of the chest joint
  pitch: number;                           // muzzle elevation (+ up)
  lean: number;                            // forward lean spread over the three spine joints
  headPitch: number;                       // + = chin down (after the lean is countered: the head stays level on target by itself)
  tuck: number;                            // 0 elbows where the mocap leaves them … 1 pulled down and in against the ribs
  blade: number; headYaw: number;          // stack only, mirrored by stackSide: pelvis yaw toward the wall (upper spine counters ⅔ of it), head yaw toward the door
  squat: number;                           // m the pelvis settles onto bent knees (feet stay planted)
  // ---- the wall hold only ('wall': back to a wall, wallBody / wallArms) — left undefined by the carries above. Its frame is the WALL, not the legs: `n` = the
  //      wall's normal into the room (anim.wallYaw off the figure's front: 0 parked, ±55° while the legs are turned to sidle), 'toward side' = along the wall the
  //      way stackSide says he looks / leads. fwd / up above are then read off n / up from the chest joint, and the gun hand goes gunSide toward side.
  back?: [number, number, number];         // pelvis joint this far off the wall plane (m): parked standing, parked crouched, on the move (the root stands 0.45 off it)
  drop?: [number, number];                 // pelvis lowered onto the knees (m): standing, crouched (on top of the crouch clip's own)
  gaze?: number;                           // head yaw toward side, degrees off the wall normal — measured from the wall, so whatever the chest supplies the neck need not
  gunSide?: number;                        // gun hand toward side (m): with fwd / up / pitch = high and tight by the cheek on the look side, muzzle up
  peekLean?: number; peekShift?: number; peekTurn?: number; peekLook?: number;   // at wallPeek 1: side bend of the spine toward side (°), pelvis carried that way along the wall (m), chest let round toward the edge (°), head a little further round on top of the controller's crane (°)
  feet?: { lead: Vec3; trail: Vec3; leadCrouch: Vec3; trailCrouch: Vec3 };   // parked foot spots [along the wall toward side from the root (m), off the wall plane (m), toes turned toward side (°)]: lead = the side foot
}
const STANCES: Record<Exclude<Stance, 'none'>, StanceShape> = {
  highReady: { fwd: 0.25, up: 0.15, side: -0.05, pitch: 8, lean: 7, headPitch: 2, tuck: 0.85, blade: 0, headYaw: 0, squat: 0.015 },
  lowReady: { fwd: 0.30, up: -0.215, side: -0.07, pitch: -42, lean: 3, headPitch: 1, tuck: 0.3, blade: 0, headYaw: 0, squat: 0 },
  stack: { fwd: 0.19, up: 0.13, side: -0.03, pitch: 22, lean: 9, headPitch: 4, tuck: 1, blade: 22, headYaw: 14, squat: 0.07 },
  // back flat to the wall and pressed to it (lean −3: the shoulder blades further back than the pelvis), knees soft, feet apart along the wall with the side foot
  // stepped out and turned the way he is going, head turned 62° along the wall; the pistol up by the cheek on that side, muzzle up (fwd / up / gunSide / pitch),
  // the free hand flat on the wall at the hip. Crouched: the same back, low, the lead leg out along the wall and the trail heel under him.
  wall: { fwd: 0.11, up: 0.21, side: 0, pitch: 72, lean: -3, headPitch: 4, tuck: 1, blade: 0, headYaw: 0, squat: 0,
    back: [0.20, 0.17, 0.30], drop: [0.035, 0.05], gaze: 62, gunSide: 0.15, peekLean: 19, peekShift: 0.09, peekTurn: 26, peekLook: 8,
    feet: { lead: [0.30, 0.26, 40], trail: [-0.14, 0.21, 10], leadCrouch: [0.54, 0.25, 65], trailCrouch: [-0.10, 0.30, 15] } },
};
/** the wall hold's frame: how far off the wall plane the figure's root stands (player.ts: BODY_R + WALL_GAP = 0.42 + 0.03). Which way the wall faces is not
 *  assumed — the controller says so each frame (anim.wallYaw), since it turns the legs away from flat to sidle. */
const WALL_ROOT_OFF = 0.45;
/** Deliberate movement and the crouch's centre of gravity by pace (CharacterAnimator.deliberate, sneakLayer). */
const SNEAK = {
  stride: 0.282,                                 // deliberate 1, crouched: the stride this much longer and the cadence this much lower (1 / 0.78): the planted foot keeps its ground speed
  paceStride: 0.10,                              // full crouch pace: the stride a further 10 % longer, NOT paid for in cadence (the clip's stance runs slow of the ground as it is; this closes a little of that)
  linger: 0.16,                                  // deliberate 1: the sampled phase runs 1 ± this about the true rate — slowest just after each plant, quickest mid-swing
  surge: 0.7,                                    // the share of the nominal ground-per-cycle the clip's stance really covers: sizes the fore-aft ride that keeps the planted foot's speed through the linger
  drop: 0.04, paceDrop: 0.03, softDrop: 0.01,   // pelvis lower (m): deliberate crouch-walking · full crouch pace · deliberate upright
  lean: 5, paceLean: 1.5,                        // torso further over (°, spread hips-first up the spine; the neck takes it all back out)
  gunIn: [0, -0.07, -0.06] as Vec3, gunInStand: [0, -0.03, -0.03] as Vec3,   // two-hand carry, crouched / upright: the gun hand this far [left, up, fwd] of where the clip has it — down and back toward the lap — the support hand rigid with it
  handIn: 0.10,                                  // free hands: each this far in toward the thighs (m; never across the middle), 2 cm down, 3 back — half of it upright
};
/** kick timing (s): total (the last ~0.25 s is the weight settling forward off the re-planted foot), and the instant the sole meets the door */
const KICK_DUR = 1.0; export const KICK_IMPACT = 0.38;
/** the swing foot is back on the floor by this time */
const KICK_PLANT = 0.72;
const SIGNAL_DUR: Record<SignalKind, number> = { hold: 0.9, go: 0.95, rally: 1.3 };
/** Overhand throw timing (s): the whole one-shot, and the instant the canister leaves the hand — the hand passing in front of the head at its fastest, at the end
 *  of the delivery. player.ts spawns the item at exactly THROW_RELEASE into the wind-up (pendingThrow), so a beat that clicks at T has its can in the air at
 *  T + THROW_RELEASE. Phases: wind-up 0 → 0.165 (top of the backswing), delivery → 0.28 = release, follow-through → ~0.48, recovery → 0.75. (Any earlier and
 *  the delivery is under four frames at 60 Hz — the hand covers its 0.7 m between two pictures and reads as a cut, not a whip; the wind-up from a hanging arm
 *  is already as brisk as one can look.) */
export const THROW_DUR = 0.75; export const THROW_RELEASE = 0.28;
const THROW_TOP = 0.165;
/** The throw's body English, degrees / metres over time, every channel starting and ending at zero (it goes straight into the pose at full weight). Yaw + = to
 *  the figure's left = the way a right-hander unwinds; the chest curve is the TOTAL turn of the shoulders, the pelvis its own share (the spine makes up the
 *  difference) — the pelvis turns back ≈ 25 ms before the shoulders do, and both go through square and past it before settling. */
const THR = {
  chestYaw: [[0, 0], [0.07, -9], [THROW_TOP, -26], [0.19, -25], [0.225, -14], [0.26, -3], [0.30, 6], [0.36, 10], [0.48, 7], [0.62, 2], [THROW_DUR, 0]] as Keys,
  pelvisYaw: [[0, 0], [0.07, -3.5], [0.145, -9], [0.17, -8], [0.21, 1], [0.25, 6.5], [0.31, 8], [0.44, 6], [0.60, 1.5], [THROW_DUR, 0]] as Keys,   // leads: turning back from 0.145 s (the chest from 0.17), through square by the release
  pitch: [[0, 0], [THROW_TOP, -4], [THROW_RELEASE, 4], [0.38, 9], [0.48, 7], [0.62, 2], [THROW_DUR, 0]] as Keys,          // + = forward: a touch of arch loading up, then over the front leg
  roll: [[0, 0], [THROW_TOP, 3], [THROW_RELEASE, -7], [0.38, -8], [0.48, -4], [0.66, 0], [THROW_DUR, 0]] as Keys,          // + = left shoulder up: the trunk tips off to the glove side as the arm comes over
  pelX: [[0, 0], [THROW_TOP, -0.02], [0.30, 0.025], [0.48, 0.02], [0.70, 0], [THROW_DUR, 0]] as Keys,                      // weight onto the back (right) foot, then across onto the front one
  pelY: [[0, 0], [THROW_TOP, -0.015], [THROW_RELEASE, -0.01], [0.40, -0.02], [0.58, -0.008], [THROW_DUR, 0]] as Keys,
  pelZ: [[0, 0], [THROW_TOP, -0.03], [THROW_RELEASE, 0.02], [0.39, 0.045], [0.54, 0.03], [THROW_DUR, 0]] as Keys,
  clavUp: [[0, 0], [THROW_TOP, 9], [THROW_RELEASE, 7], [0.40, 3], [0.62, 0], [THROW_DUR, 0]] as Keys,                     // scapula up with the raised arm…
  clavFwd: [[0, 0], [THROW_TOP, -4], [THROW_RELEASE, 5], [0.39, 8], [0.54, 3], [0.70, 0], [THROW_DUR, 0]] as Keys,          // …back in the wind-up, thrown forward through the release
  // right (throwing) hand from its shoulder joint, model axes [left, up, forward], from the top of the backswing (up behind the head, elbow folded to ≈ 80°) on: the
  // elbow leads and the hand lays back, then comes over the top past the head, fastest through the release out in front (elbow opening to ≈ 120°), down and across
  // to the left hip. The way up (hanging under the lifting elbow, then out behind it) and the way back (round beside the thigh) are built per frame off the live
  // hand of the pose underneath (throwLayer)
  handR: [[THROW_TOP, [-0.19, 0.21, -0.20]], [0.205, [-0.21, 0.27, -0.10]], [0.245, [-0.16, 0.27, 0.14]],
    [THROW_RELEASE, [-0.06, 0.16, 0.42]], [0.31, [0.06, -0.08, 0.47]], [0.37, [0.13, -0.27, 0.37]], [0.44, [0.16, -0.35, 0.28]], [0.50, [0.14, -0.38, 0.29]]] as Keys3,
  // where the right ELBOW goes meanwhile, as a swivel angle about the shoulder→hand line (degrees, see swivelPole: measured in a basis built off a reference
  // direction the hand never points along, so the pole is square to the reach by construction and cannot flip however the hand sweeps past the elbow's
  // bearing). Each key was solved offline for the elbow spot wanted against the hand key of its time: swung out sideways and up to shoulder height while the
  // hand still hangs below it (−103°), straight out at the top with the forearm cocked up behind (−176°), forward and high leading the delivery (−149°), level
  // through the release (−185°), turning over above the hand's line through the follow-through (−235°), back out behind the body coming down (−105°)
  swivR: [[0, -62], [0.06, -103], [0.125, -166], [THROW_TOP, -176], [0.205, -149], [0.245, -184.6], [THROW_RELEASE, -184.6], [0.31, -232.9], [0.37, -240.9], [0.44, -234.7], [0.50, -214.4], [0.62, -105.4], [THROW_DUR, -62]] as Keys,
  fingR: [[0, [0, -1, 0.15]], [0.07, [-0.1, -0.75, -0.65]], [0.115, [0, 0.6, -0.8]], [THROW_TOP, [0.15, 1, -0.35]], [0.23, [0.1, 1, -0.5]], [THROW_RELEASE, [0.05, 0.75, 0.65]], [0.32, [0.1, -0.35, 0.95]], [0.38, [0.3, -0.75, 0.6]], [0.48, [0.25, -1, 0.25]], [0.62, [0.05, -1, 0]], [THROW_DUR, [0, -1, 0.15]]] as Keys3,   // fingers: cocked back under the can at the top, laid back as the elbow leads, over the top and flicked past straight at the release, hanging after
  backR: [[0, [-1, 0, 0]], [0.07, [-1, 0.1, -0.2]], [0.115, [-0.8, -0.1, -0.6]], [THROW_TOP, [-0.3, -0.3, -1]], [0.23, [-0.4, -0.4, -0.9]], [THROW_RELEASE, [-0.2, 0.5, -0.8]], [0.32, [-0.3, 0.9, 0.3]], [0.38, [-0.6, 0.5, 0.6]], [0.48, [-0.7, 0.1, 0.7]], [0.62, [-1, 0, 0.2]], [THROW_DUR, [-1, 0, 0]]] as Keys3,   // back of the hand: palm forward behind the head, turning over (pronating) through the follow-through
  // left (balance) hand from its shoulder joint: up loosely toward the target, then pulled in to the ribs as the right comes through (ends built off the live hand too)
  handL: [[0.17, [0.10, -0.06, 0.44]], [0.24, [0.10, -0.12, 0.38]], [0.31, [0.06, -0.28, 0.22]], [0.40, [0.05, -0.33, 0.18]], [0.52, [0.07, -0.40, 0.13]]] as Keys3,
  swivL: [[0, 83], [0.09, 136.5], [0.17, 149.2], [0.24, 156.6], [0.31, 142], [0.40, 134.9], [0.52, 123.3], [0.63, 106.8], [THROW_DUR, 83]] as Keys,   // left elbow, same scheme (mirrored reference): down and out under the reach, winging out and back for the tuck
  fingL: [[0, [0, -1, 0.15]], [0.09, [0.1, -0.4, 0.9]], [0.17, [0.05, 0.15, 1]], [0.25, [0.1, 0.35, 0.9]], [0.33, [0.15, 0.6, 0.8]], [0.52, [0.1, 0.3, 0.9]], [0.63, [0.1, -0.7, 0.6]], [THROW_DUR, [0, -1, 0.15]]] as Keys3,
  backL: [[0, [1, 0, 0]], [0.09, [0.5, 0.8, 0.3]], [0.17, [0.3, 1, -0.1]], [0.25, [0.7, 0.7, -0.2]], [0.33, [1, 0.1, -0.3]], [0.52, [1, 0.2, -0.2]], [0.63, [1, 0.2, 0]], [THROW_DUR, [1, 0, 0]]] as Keys3,
};

/** Animation graph producing a local pose + twist list each frame. */
export class CharacterAnimator {
  pose: Pose; private pA: Pose; private pB: Pose; private pC: Pose; private pUpper: Pose; private rest: Pose; private pAim: Pose;
  twists: TwistSpec[] = [];
  params: AnimParams = { ...defaultAnimParams };
  // inputs (set by controller each frame)
  speed = 0;                 // ground speed (m/s)
  reverse = false;           // play locomotion backwards (backpedal)
  crouchTarget = 0;          // 0 stand, 1 crouch
  upper: UpperMode = 'none';
  aimYawOffset = 0;          // chest yaw relative to body (rad)
  aimPitch = 0;              // rad, positive = up
  lookYawExtra = 0;          // additional head/neck yaw (looking around)
  /** Ready posture layered under the aim layer (aim / firing still win): 'highReady' pistol up under the eye line, elbows in, leaning into it — pieing a corner,
   *  covering a door; 'lowReady' a conventional low ready: arms out and down, hands at belt height ~30 cm ahead of the pelvis, muzzle depressed ~42°, elbows soft,
   *  shoulders easy — moving between doors; 'stack' bladed to the wall on `stackSide`, compressed high ready,
   *  knees soft, head on the door. Arms only above the waist (legs keep the locomotion), eased in/out ≈ 0.3 s and morphing between kinds; composes with
   *  aimYawOffset (the chest twist turns the whole carry) and with walking / crouching (the gun stays level).
   *  'wall' is the player's back-to-the-wall hold (Q) and a body of its own (wallBody / wallArms, STANCES.wall): back flat and pressed to the wall behind
   *  (wallYaw), feet apart along it, head turned along it toward stackSide, the pistol up by the cheek if it is in the hand (upper relaxed / aim) or the
   *  hands on the wall if not; crouched it sits low with the back still flat; sidling, the torso stays square to the wall over the walking legs; wallPeek
   *  leans it out past the wall's end. In and out ≈ 0.2 s. */
  stance: Stance = 'none';
  /** which side the wall is on for 'stack': +1 = the figure's right, −1 = its left (mirrors blade, head and gun offset). For 'wall' (the wall is BEHIND): the way
   *  along it he looks and leads — +1 = to his right — which side the head turns, the gun comes up, the lead foot steps out and a peek leans. Flips ease (sprung). */
  stackSide: -1 | 1 = 1;
  /** 'wall' only: yaw of the wall's outward normal (into the room) off the figure's front, rad, + = to its left. 0 = the wall square behind him (parked); the
   *  controller turns the legs toward the direction of travel to sidle and says so here — the torso above is kept flat to the wall whatever the legs do. */
  wallYaw = 0;
  /** 'wall' only, 0…1: peeking round the wall's end on stackSide — the upper body leans out along the wall past the edge, the pelvis shifts onto the lead foot,
   *  the chest comes round a little (the neck's extra crane is the controller's lookYawExtra); the gun goes with the chest. Ramped by the controller, eased here too. */
  wallPeek = 0;
  /** Player: working a lock — forces the crouch, both hands come up to `lockpickAt` (pick in the right, tension wrench in the left) with small irregular wrist
   *  work, shoulders hunched, head in close and cocked. The pistol is not posed (the game holsters it). Eased in/out ≈ 0.35 s. */
  lockpick = false;
  /** the keyway in MODEL space (x to the figure's left, y up, z ahead of its root): default = just under a lever handle (doors.ts: 1.0 m), 0.45 m ahead */
  lockpickAt: Vec3 = [0.02, 0.96, 0.45];
  /** 0…1: how deliberately he moves — the game raises it as a living guard who has not yet clocked him comes close (game.ts threatProximity). The LOOK only,
   *  ground speed untouched: crouch-walking, the cycle is retimed longer and slower (cadence × 0.78 at 1, the stride warped longer by as much at the feet so the
   *  planted foot keeps exactly the ground it had), a linger over each plant with the body hanging back then coming on, the pelvis ~4 cm lower, the torso a
   *  few degrees further over with the head kept level, the arms drawn in to the body; walking upright, only the arms and a centimetre of knee. Eased in here
   *  (~0.4 s either way), so a guard flipping to alert relaxes it without a pop. Independent of it, the crouch also sits lower and steps longer as its pace
   *  comes up (sneakLayer). */
  deliberate = 0;
  /** Player: he has a man from behind (player.ts updateHold) — the phase of it and the seconds into that phase; null = not. The layer (holdLayer) eases itself in
   *  and out on a spring, so the controller just sets and clears this: left arm round the man's throat, right hand clamped on his far shoulder, chest to his back,
   *  looking past his ear; the reach of the grab, the elbow closing for the choke and the shove of the release are shaped off phase / t. The legs stay the cycle's.
   *  variant 'gun': the right hand keeps the pistol up beside the man's head instead (HOLD.gun) — presented forward by gunOut to fire past him (a shot's recoil is
   *  taken here, not by the aim clip), swung down behind his ear through 'whip'. */
  holdPose: HoldPose | null = null;
  /** Guard: he is the man being held (guards.ts held branch, posed by player.ts updateHold): arched back onto the arm with his chin up, left hand clawing at the
   *  forearm across his throat, the torch hand up and out (its beam on the ceiling), a struggle that grows with his agitation; through the choke his knees go, both
   *  hands come to the arm, then he hangs; shoved off, the arms fly forward. Eased in / out on its own spring like holdPose; the legs walk at the pair's speed.
   *  variant 'gun': hands half raised instead, the head tipped away off the can at his temple, a duck when a round goes past it, and the head going with the blow
   *  at the end of 'whip'. */
  heldPose: HeldPose | null = null;
  /** Output: whatever the right hand holds (pistol / canister) should not be drawn this frame — the hands are more than half way onto the lock tools (lockpick
   *  layer weight > 0.5) or BOTH round a held man (the arm variant's hold layer > 0.5: the pistol rides the thigh; the gun variant keeps it in the hand). Whoever
   *  draws the hand props (game.ts handProps, the viewer) reads this and shows an empty hand + the holstered sidearm instead. */
  hideHeldItem = false;
  /** Viewer / debug: when set the graph is bypassed and this one clip plays raw over the rest pose (no layers, no twists) — for eyeballing
   *  the library's clips with the props attached. `time` advances by dt·rate inside update(). */
  rawClip: { clip: Clip; time: number; rate: number; loop: boolean } | null = null;
  // internal state
  phase = 0;                 // normalized locomotion phase
  crouch = 0;
  private upperW = { relaxed: 0, aim: 0, torch: 0 };
  private shootT = -1; private hitT = -1; private reloadT = -1; private throwT = -1; private strikeT = -1;
  private kickT = -1; private kickHit = false; private sigT = -1; private sigKind: SignalKind = 'hold';
  private idleT = 0;
  /** stance follow-through: master weight + one spring per shape channel (blade / headYaw / side carry the stackSide sign) */
  private st = { w: new Spring(), fwd: new Spring(STANCES.highReady.fwd), up: new Spring(STANCES.highReady.up), side: new Spring(STANCES.highReady.side), pitch: new Spring(STANCES.highReady.pitch), lean: new Spring(STANCES.highReady.lean), headPitch: new Spring(0), tuck: new Spring(STANCES.highReady.tuck), blade: new Spring(0), headYaw: new Spring(0), squat: new Spring(0) };
  /** the wall hold's own followers: wl = how much of the stance weight is the WALL's body / arms rather than the carries' (1 while 'wall' is held, so stack ↔ wall
   *  cross-fade and none ↔ wall rides the master weight alone); side = stackSide made continuous (a flip sweeps head, gun and feet across instead of popping them);
   *  peek = wallPeek eased; armed = pistol in the hand (upper relaxed / aim) vs a free hand — the two arm sets blend; park = standing on the hold's own foot spots
   *  (1) vs the cycle's feet (0), by ground speed — quick off them, slower back on */
  private wst = { wl: new Spring(), side: new Spring(1), peek: new Spring(), armed: new Spring(1), park: new Spring(1) };
  private stanceSeen: Stance = 'highReady'; private lpW = new Spring(); private lpT = 0;
  /** the hold's two layers: master weights (sprung on holdPose / heldPose being set), the last inputs seen (what the layer eases OUT from once the controller has
   *  cleared them), and the man's eased choke amount (an aborted choke lets his knees and hands come back instead of snapping) */
  private hdW = new Spring(); private hdLast: HoldPose = { phase: 'held', t: 0 };
  private heW = new Spring(); private heLast: HeldPose = { phase: 'held', t: 0, agitation: 0 }; private heCk = new Spring();
  /** `deliberate` eased; and the crouch walk's gait numbers its retime needs (measured off Crouch_Fwd_Loop at construction): the fore-aft middle of the ankles'
   *  travel — the stride is warped about it — and the phase the linger centres on (just after a foot is set down; the other plant is half a cycle on) */
  private dlb = new Spring(); private gait = { mid: -0.15, linger: 0.08 };
  /** throw: where each hand ended up last frame (offset from its shoulder joint, model axes) — a throw re-triggered mid-swing carries on from there instead of
   *  from the pose underneath — and that carried-in start while it applies */
  private thrLast: { r: Vec3; l: Vec3 } | null = null; private thrCarry: { r: Vec3; l: Vec3 } | null = null;
  /** the standing idle's foot placement (model space, constant over the loop): where a kick plants both feet, soles as the clip has them */
  private idleFeet: { L: Vec3; R: Vec3; rL: Quat; rR: Quat };
  /** per kick: where the feet were when it started (the support foot is held from the first frame, the swing foot's path starts from its own spot) */
  private kickFrom: { L: Vec3; rL: Quat; R: Vec3 } | null = null;
  private clips: Record<string, Clip>;
  private upperMask: Float32Array; private armRMask: Float32Array; private headMask: Float32Array; private armsMask: Float32Array; private dynMask: Float32Array;
  private fk: PoseFK;
  /** per hand: rotation from handRot's construction frame to the bone's rest axes; gun hand: its local axis that is 'up' when the aim clip holds the pistol level */
  private handFix: Quat[]; private gripUp: Vec3;
  /** head-local direction that is 'looking straight ahead' (taken from the standing idle, whose head is not at the bone's rest angle) */
  private gazeLocal: Vec3;
  /** inverse of the chest joint's (spine.003) rest model rotation: current · this takes rest-pose model axes to where the chest points them now (its facing / its left) */
  private chestRestInv: Quat;
  /** finger joints [side L=0/R=1][finger index,middle,ring,pinky][joint 0..2] and thumbs [side][joint] */
  private fingers: number[][][]; private thumbs: number[][];
  nodes: Record<keyof typeof N, number>;

  constructor(public ch: GltfCharacter) {
    const n = ch.nodes.length;
    this.pose = new Pose(n); this.pA = new Pose(n); this.pB = new Pose(n); this.pC = new Pose(n); this.pUpper = new Pose(n); this.pAim = new Pose(n); this.rest = restPose(ch, new Pose(n));
    for (const p of [this.pose, this.pA, this.pB, this.pC, this.pUpper, this.pAim]) p.copy(this.rest);
    const need = ['Idle_Loop', 'Walk_Loop', 'Jog_Fwd_Loop', 'Crouch_Idle_Loop', 'Crouch_Fwd_Loop', 'Pistol_Idle_Loop', 'Pistol_Aim_Neutral', 'Pistol_Aim_Up', 'Pistol_Aim_Down', 'Pistol_Shoot', 'Idle_Torch_Loop', 'Hit_Chest', 'Sprint_Loop', 'Pistol_Reload', 'Punch_Cross'];   // (no death clip: dying hands the skeleton to the ragdoll, Character.ragdollize; no throw clip: the library's only cast is left-handed, the throw is procedural — throwLayer)
    this.clips = {};
    for (const k of need) { const c = ch.clips.get(k); if (!c) throw new Error('missing clip ' + k); this.clips[k] = c; }
    this.nodes = {} as Record<keyof typeof N, number>;
    for (const [k, name] of Object.entries(N)) { const i = ch.nodeByName.get(name); if (i === undefined) throw new Error('missing node ' + name); (this.nodes as Record<string, number>)[k] = i; }
    const node = (name: string) => { const i = ch.nodeByName.get(name); if (i === undefined) throw new Error('missing node ' + name); return i; };
    this.fingers = ['L', 'R'].map(s => ['f_index', 'f_middle', 'f_ring', 'f_pinky'].map(f => [1, 2, 3].map(j => node(`DEF-${f}.0${j}.${s}`))));
    this.thumbs = ['L', 'R'].map(s => [1, 2, 3].map(j => node(`DEF-thumb.0${j}.${s}`)));
    this.upperMask = buildMask(ch, [{ name: N.spine1, w: 0.3, subtree: false }, { name: N.spine2, w: 0.65, subtree: false }, { name: N.spine3, w: 1, subtree: true }]);
    this.armRMask = buildMask(ch, [{ name: N.shoulderR, w: 1, subtree: true }, { name: N.spine3, w: 0.25, subtree: false }]);
    this.headMask = buildMask(ch, [{ name: N.neck, w: 1, subtree: true }]);
    this.armsMask = buildMask(ch, [{ name: N.shoulderL, w: 1, subtree: true }, { name: N.shoulderR, w: 1, subtree: true }]);   // both clavicles down to the fingertips: the procedural carries / lock work
    this.dynMask = new Float32Array(n);                                                                                            // the signals' left-arm mask, rebuilt per frame with staggered weights (shoulder → elbow → hand)
    this.fk = new PoseFK(ch);
    sampleClip(this.clips.Pistol_Aim_Neutral, 0, false, this.pAim);   // the two-hand grip every pistol carry below is built from (static clip)
    this.fk.run(this.rest); this.chestRestInv = quat.conj(this.fk.rot[this.nodes.spine3]);
    this.handFix = [this.nodes.handL, this.nodes.handR].map(h => { const r = this.fk.rot[h]; const F = v3.normalize(quat.rotate(r, AY)); const B = v3.normalize(v3.mad(AY, F, -v3.dot(AY, F))); return quat.mul(quat.conj(quat.fromBasis(v3.cross(F, B), F, B)), r); });   // rest = T-pose, palms down: fingers along local +Y, back of the hand facing model +Y
    this.fk.run(this.pAim); this.gripUp = quat.rotate(quat.conj(this.fk.rot[this.nodes.handR]), AY);
    { const p = new Pose(n).copy(this.rest); sampleClip(this.clips.Idle_Loop, 0, false, p); const fk = this.fk.run(p); const nd = this.nodes;
      this.gazeLocal = quat.rotate(quat.conj(fk.rot[nd.head]), AZ);
      this.idleFeet = { L: v3.copy(fk.pos[nd.footL]), R: v3.copy(fk.pos[nd.footR]), rL: [...fk.rot[nd.footL]] as Quat, rR: [...fk.rot[nd.footR]] as Quat }; }
    { // the crouch walk's gait for the deliberate retime (sneakLayer): where the ankles' fore-aft travel is centred, and where in the cycle each toe comes down
      // flat (its height back within a centimetre of the floor after the swing) — the linger sits a little after that, the two plants folded onto one half-cycle
      const p = new Pose(n).copy(this.rest), cf = this.clips.Crouch_Fwd_Loop, nd = this.nodes, M = 48;
      let zmin = 1e9, zmax = -1e9; const ty: number[][] = [[], []];
      for (let i = 0; i < M; i++) {
        sampleClip(cf, cf.duration * i / M, true, p); const fk = this.fk.run(p);
        for (const f of [nd.footL, nd.footR]) { zmin = Math.min(zmin, fk.pos[f][2]); zmax = Math.max(zmax, fk.pos[f][2]); }
        ty[0].push(fk.pos[nd.toeL][1]); ty[1].push(fk.pos[nd.toeR][1]);
      }
      const plants: number[] = [];
      ty.forEach((ys, side) => { const lo = Math.min(...ys) + 0.01; for (let i = 0; i < M; i++) if (ys[(i + M - 1) % M] > lo && ys[i] <= lo) { plants.push(i / M - 0.5 * side); break; } });
      if (zmax > zmin) this.gait.mid = (zmin + zmax) / 2;
      if (plants.length) { let sx = 0, cy = 0; for (const ph of plants) { sx += Math.sin(4 * Math.PI * ph); cy += Math.cos(4 * Math.PI * ph); } this.gait.linger = ((Math.atan2(sx, cy) / (4 * Math.PI)) + 0.07 + 1) % 0.5; }   // (circular mean on the half-cycle, + the linger's lag behind the plant)
    }
    this.phase = Math.random();
  }

  fire() { this.shootT = 0; }
  hit() { this.hitT = 0; }
  /** startle: same recoil as taking a round (Hit_Chest layer), no damage implied */
  flinch() { this.hit(); }
  /** takedown strike (full upper body, brief): Punch_Cross */
  strike() { this.strikeT = 0; }
  get striking() { return this.strikeT >= 0; }
  reload() { this.reloadT = 0; }
  cancelReload() { this.reloadT = -1; }
  /** Overhand throw off the right hand, THROW_DUR s, layered over whatever the legs are doing (standing, crouched, walking): torso and pelvis wind away and
   *  unwind hips-first, the hand comes up behind the head and whips past it — the item should leave it at THROW_RELEASE — then follows through down across
   *  the body and eases back to the pose beneath. Re-triggering mid-swing starts the new wind-up from where the hands are. */
  throwItem() { this.thrCarry = this.throwT >= 0 && this.thrLast ? { r: v3.copy(this.thrLast.r), l: v3.copy(this.thrLast.l) } : null; this.throwT = 0; }
  get reloading() { return this.reloadT >= 0; }
  /** the item is still in the hand (up to a hair past THROW_RELEASE): hand props keep drawing it, a beat waits with the re-draw */
  get throwing() { return this.throwT >= 0 && this.throwT < THROW_RELEASE + 0.05; }
  /** the throw one-shot is playing at all (wind-up through recovery) */
  get inThrow() { return this.throwT >= 0; }
  /** seconds into the throw, −1 when not throwing (THROW_RELEASE is the instant the hand lets go) */
  get throwTime() { return this.throwT; }
  get shooting() { return this.shootT >= 0 && this.shootT < 0.35; }
  /** Front kick at door-handle height off the rear (right) leg, 1.0 s, full body (locomotion is faded out under it and any stance stands up square — call it on
   *  a standing character): weight onto the left leg — held where it stood from the first frame — and a knee-up chamber (0–0.27 s), the drive (sole meets the
   *  door at KICK_IMPACT = 0.38 s, ~0.75 m ahead of the root, ~0.9 m up), recoil with the pelvis riding back so the foot re-plants under the hip (≈0.72 s) on the
   *  idle's own spot, then the weight settles forward. The pelvis counter-rotates and leans back, the pistol is pulled into a compressed high ready throughout. */
  kickDoor() { if (this.kickT < 0) { this.kickT = 0; this.kickHit = false; this.kickFrom = null; } }
  get kicking() { return this.kickT >= 0; }
  /** seconds into the kick, −1 when not kicking (KICK_IMPACT is the contact instant) */
  get kickTime() { return this.kickT; }
  /** true for exactly one update — the one in which the sole reached the door: swing the door / play the crack on this */
  get kickImpact() { return this.kickHit; }
  /** Left-hand signal, ~1 s, left arm only (the gun hand is untouched, so it layers over any carry or aim): 'hold' fist up beside the head, 'go' knife hand raised
   *  then chopped forward to point the way, 'rally' open hand circling above the head. Ignored while one is still playing. */
  signal(kind: SignalKind) { if (this.sigT < 0) { this.sigT = 0; this.sigKind = kind; } }
  get signalling() { return this.sigT >= 0; }

  update(dt: number) {
    const P = this.params; const c = this.clips; const nd = this.nodes;
    if (this.rawClip) {   // raw clip preview: rest pose underneath so nodes the clip does not key stay sane; one-shots cancelled (nothing here would ever finish them)
      const r = this.rawClip; r.time += dt * r.rate;
      this.pose.copy(this.rest); sampleClip(r.clip, r.time, r.loop, this.pose); this.twists.length = 0;
      this.shootT = this.hitT = this.reloadT = this.throwT = this.strikeT = this.kickT = this.sigT = -1; this.kickHit = false; this.hideHeldItem = false; this.thrCarry = this.thrLast = null;
      return;
    }
    // ---- one-shot clocks that shape the base graph (kick fades locomotion out under itself)
    const kt = this.kickT; const wK = kt >= 0 ? bump(kt, 0, 0.12, KICK_DUR - 0.2, KICK_DUR) : 0;   // locomotion fade under the kick
    const wKbody = kt >= 0 ? 1 - smoothstep(KICK_DUR - 0.2, KICK_DUR, kt) : 0;                      // the kick's body layer: full from the first frame (its curves and foot targets all start where the body is), eased off at the end
    const wKstand = kt >= 0 ? bump(kt, 0, 0.2, KICK_DUR - 0.3, KICK_DUR) : 0;                       // a stance's squat / blade / lean standing up square for the kick
    const wKslow = kt >= 0 ? bump(kt, 0, 0.3, KICK_DUR - 0.3, KICK_DUR) : 0;                        // what the arms and a crouch follow (hanging arms need longer than 0.1 s to find the gun)
    const wLp = clamp(this.lpW.step(this.lockpick ? 1 : 0, 17, dt), 0, 1);
    if (this.holdPose) this.hdLast = this.holdPose; else if (this.hdLast.phase === 'whip' && this.hdLast.t < HOLD.secs.whip + 1) this.hdLast = { ...this.hdLast, t: this.hdLast.t + dt };   // (what the layers ease out from once the controller lets go — a whip that has landed plays its follow-through on the way out)
    if (this.heldPose) this.heLast = this.heldPose;
    const thruWhip = !this.holdPose && this.hdLast.phase === 'whip' && this.hdLast.t >= HOLD.secs.whip && this.hdLast.t < HOLD.secs.whip + WHIP_THRU;   // (a whip that landed keeps the layer up through its follow-through, then lets go)
    const wHd = clamp(this.hdW.step(this.holdPose || thruWhip ? 1 : 0, 16, dt), 0, 1), wHe = clamp(this.heW.step(this.heldPose ? 1 : 0, 16, dt), 0, 1);
    const gunHold = wHd > 0.003 && (this.holdPose ?? this.hdLast).variant === 'gun';   // the pistol stays in the hand through this hold: drawn, lit if it was, and a shot's recoil is the hold layer's (not the two-handed aim clip's)
    this.hideHeldItem = wLp > 0.5 || (wHd > 0.5 && !gunHold);   // hands on the lock tools, or both on a held man: the pistol rides the thigh
    this.crouch = damp(this.crouch, Math.max(this.crouchTarget, this.lockpick ? 1 : 0), 10, dt);
    const s = Math.abs(this.speed) * (1 - wK);
    const crouchPose = this.crouch * (1 - wKslow);   // the crouch actually shown (the kick stands him up)
    // ---- deliberate movement (`deliberate`, eased here) and the crouch's centre of gravity by pace — the shares everything below reads (sneakLayer):
    //      Dl = deliberate AND crouch-walking (retime, stride, linger, the 4 cm, the lean), pace = creep → full crouch pace (the 3 cm and the longer step),
    //      soft = deliberate walking upright (a centimetre of knee), armsIn = deliberate at all (a little even planted, the rest with movement). The wall
    //      hold keeps its own body and feet: all but the arms let go by its share of the pose (last frame's springs — this frame's are stepped further down).
    const D = clamp(this.dlb.step(clamp(this.deliberate, 0, 1), 15, dt), 0, 1);
    const offWall = 1 - clamp(this.st.w.x, 0, 1) * clamp(this.wst.wl.x, 0, 1);
    const mv = smoothstep(0.15, 0.5, s);
    const Dl = D * mv * offWall * crouchPose, pace = smoothstep(0.38, 0.95, s / P.crouchSpeed) * mv * offWall * crouchPose, soft = D * mv * offWall * (1 - crouchPose);
    const armsIn = D * offWall * lerp(0.4, 1, mv);
    const strideD = 1 + SNEAK.stride * D * mv * offWall;   // the crouch cycle's cadence comes down by this and its stride goes up by it (× crouch: the rate lerp below, the pose in sneakLayer)
    // ---- locomotion (stand) ----
    // weights among idle / walk / jog by speed
    let wWalk = 0, wJog = 0, wSprint = 0;
    if (s <= P.walkSpeed) wWalk = clamp(s / P.walkSpeed, 0, 1);
    else if (s <= P.jogSpeed) { wJog = clamp((s - P.walkSpeed) / (P.jogSpeed - P.walkSpeed), 0, 1); wWalk = 1 - wJog; }
    else { wSprint = clamp((s - P.jogSpeed) / (P.sprintSpeed - P.jogSpeed), 0, 1); wJog = 1 - wSprint; }
    const wIdle = 1 - wWalk - wJog - wSprint;
    // phase advance: rate so that feet match ground speed (natural clip speed = P.walkSpeed / P.jogSpeed / P.sprintSpeed)
    const rateWalk = (s / P.walkSpeed) / c.Walk_Loop.duration, rateJog = (s / P.jogSpeed) / c.Jog_Fwd_Loop.duration, rateCrouch = (s / P.crouchSpeed) / c.Crouch_Fwd_Loop.duration / strideD, rateSprint = (s / P.sprintSpeed) / c.Sprint_Loop.duration;
    const wsum = Math.max(1e-3, wWalk + wJog + wSprint);
    let rate = (wWalk * rateWalk + wJog * rateJog + wSprint * rateSprint) / wsum;
    rate = lerp(rate, rateCrouch, this.crouch);
    if (s < 0.05) rate = 0;
    this.phase = (this.phase + dt * rate * (this.reverse ? -1 : 1) + 100) % 1;
    this.idleT += dt;
    // the linger (deliberate, crouched): the cycle is SAMPLED at a phase eased about the true one — slow just after each foot comes down, quick through the
    // swings, the mean rate untouched (so footfalls, and anything else keyed to `phase`, keep their count) — and the lower body rides a matching fore-aft
    // surge (sneakLayer) so the planted foot holds its ground speed through it: the body hangs back over the foot just set down, then comes on
    const aL = SNEAK.linger * Dl, th = 4 * Math.PI * (this.phase - this.gait.linger);
    const ph = aL > 1e-5 ? (this.phase - aL / (4 * Math.PI) * Math.sin(th) + 1) % 1 : this.phase;
    const surge = aL > 1e-5 ? -aL * SNEAK.surge * strideD * (1 + SNEAK.paceStride * pace) * P.crouchSpeed * c.Crouch_Fwd_Loop.duration / (4 * Math.PI) * Math.sin(th) * (this.reverse ? -1 : 1) : 0;   // (ground per cycle = stride factor × the clip's own; × the share of it the clip's stance really covers)
    // stand pose
    sampleClip(c.Idle_Loop, this.idleT, true, this.pA);
    if (wWalk + wJog + wSprint > 0.001) {
      if (wSprint > 0.001) {
        sampleClip(c.Jog_Fwd_Loop, ph * c.Jog_Fwd_Loop.duration, true, this.pB);
        sampleClip(c.Sprint_Loop, ph * c.Sprint_Loop.duration, true, this.pC); blendPose(this.pB, this.pC, wSprint, null, this.pB);
      } else {
        sampleClip(c.Walk_Loop, ph * c.Walk_Loop.duration, true, this.pB);
        if (wJog > 0.001) { sampleClip(c.Jog_Fwd_Loop, ph * c.Jog_Fwd_Loop.duration, true, this.pC); blendPose(this.pB, this.pC, wJog, null, this.pB); }
      }
      blendPose(this.pA, this.pB, 1 - wIdle, null, this.pA);
    }
    // ---- stance springs (run always: they hold the shape the arms morph through; the master weight is what comes and goes)
    const st = this.st, ws = this.wst; const SW = 13;   // 90 % there in 0.3 s, soft at both ends
    const held: Stance = kt >= 0 && kt < 0.55 ? 'highReady' : this.stance;   // the kick pulls whatever carry is up into the compressed high ready (an extended low ready would meet the knee), handing back for the plant
    const shape = held !== 'none' ? STANCES[held] : null; const side = this.stackSide;
    const wWas = Math.max(clamp(st.w.x, 0, 1), wKslow);   // the weight the last frame showed (read before stepping: one 60 Hz step from nothing already carries the master past the snap threshold below)
    const wSt = clamp(st.w.step(this.stance !== 'none' ? 1 : 0, this.stance === 'wall' || this.stanceSeen === 'wall' ? 22 : SW, dt), 0, 1);   // the master weight follows the stance itself (a kick from no stance shows the carry through wKslow); on and off the wall in ~0.2 s
    const armed = this.upper === 'relaxed' || this.upper === 'aim' || this.shootT >= 0;   // the pistol is in the hand (the wall hold poses a free hand otherwise)
    const parkNow = 1 - smoothstep(0.12, 0.7, s);                                          // the wall hold: standing on its own foot spots (1) or on the cycle's feet (0)
    if (shape) {
      const snap = wWas < 0.02 && this.stanceSeen !== held;   // taking a shape from (next to) nothing: start AT it rather than morphing from the last one under a rising weight
      if (snap) {
        for (const [k, sp] of Object.entries(st)) if (k !== 'w') { sp.v = 0; sp.x = k === 'blade' ? -side * shape.blade : k === 'headYaw' ? -side * shape.headYaw : k === 'side' ? shape.side - 0.02 * side * (shape.blade / 22) : (shape as unknown as Record<string, number>)[k]; }
        for (const sp of Object.values(ws)) sp.v = 0; ws.wl.x = held === 'wall' ? 1 : 0; ws.side.x = side; ws.peek.x = clamp(this.wallPeek, 0, 1); ws.armed.x = armed ? 1 : 0; ws.park.x = parkNow;
      }
      this.stanceSeen = held;
      st.fwd.step(shape.fwd, SW, dt); st.up.step(shape.up, SW, dt); st.side.step(shape.side - 0.02 * side * (shape.blade / 22), SW, dt); st.pitch.step(shape.pitch, SW, dt);
      st.lean.step(shape.lean, SW, dt); st.headPitch.step(shape.headPitch, SW, dt); st.tuck.step(shape.tuck, SW, dt);
      st.blade.step(-side * shape.blade, SW * 0.8, dt); st.headYaw.step(-side * shape.headYaw, SW, dt); st.squat.step(shape.squat, SW * 0.8, dt);   // (yaw + = to the figure's left; side + = wall on its right)
      ws.wl.step(held === 'wall' ? 1 : 0, 22, dt); ws.side.step(side, SW, dt); ws.peek.step(clamp(this.wallPeek, 0, 1), 20, dt); ws.armed.step(armed ? 1 : 0, SW, dt);   // (wl at the wall's own brisker rate; the peek is already ramped by the controller)
      ws.park.step(parkNow, parkNow < ws.park.x ? 30 : 12, dt);   // off the spots briskly when he sets off (the feet must be the cycle's within a stride), back onto them over ~0.3 s when he stops — a shuffle into place, not a skate
    }
    const wCarry = Math.max(wSt, wKslow);   // the kick pulls the gun into the compressed carry whatever the stance
    const wl = clamp(ws.wl.x, 0, 1);        // …of which the wall hold's share
    // crouch pose (the kick stands him up: crouchPose above)
    if (crouchPose > 0.001) {
      sampleClip(c.Crouch_Idle_Loop, this.idleT, true, this.pB);
      const wMove = clamp(s / (P.crouchSpeed * 0.6), 0, 1);
      if (wMove > 0.001) { sampleClip(c.Crouch_Fwd_Loop, ph * c.Crouch_Fwd_Loop.duration, true, this.pC); blendPose(this.pB, this.pC, wMove, null, this.pB); }
      blendPose(this.pA, this.pB, crouchPose, null, this.pA);
    }
    // ---- upper body layers ----
    const tgt = { relaxed: this.upper === 'relaxed' ? 1 : 0, aim: this.upper === 'aim' ? 1 : 0, torch: this.upper === 'torch' ? 1 : 0 };
    if (this.shootT >= 0 && !gunHold) { tgt.aim = 1; tgt.relaxed = 0; tgt.torch = 0; }   // (a round fired past a held man: the left arm stays on his throat — holdLayer kicks the gun hand itself)
    if (this.throwT >= 0 && this.throwT < THROW_DUR - 0.25) tgt.aim = 0;   // the throwing arm has the upper body: the aim clip fades out under the wind-up (≈ 0.15 s at rate 14) and comes back up over the recovery
    this.upperW.relaxed = damp(this.upperW.relaxed, tgt.relaxed, 9, dt);
    this.upperW.aim = damp(this.upperW.aim, tgt.aim, 14, dt);
    this.upperW.torch = damp(this.upperW.torch, tgt.torch, 9, dt);
    if (this.upperW.relaxed > 0.003) { sampleClip(c.Pistol_Idle_Loop, this.idleT, true, this.pUpper); blendPose(this.pA, this.pUpper, this.upperW.relaxed, this.upperMask, this.pA); }
    if (this.upperW.torch > 0.003) { sampleClip(c.Idle_Torch_Loop, this.idleT, true, this.pUpper); blendPose(this.pA, this.pUpper, this.upperW.torch, this.upperMask, this.pA); }
    // ---- deliberate movement / the crouch's centre of gravity (legs, pelvis, torso, arms) — over the finished locomotion and its clip arms, under every stance,
    //      one-shot and aim layer below (they read the feet where this leaves them, and pose the arms over these when they have them)
    if (Dl > 0.002 || pace > 0.002 || soft > 0.002 || armsIn > 0.003) this.sneakLayer(Dl, pace, soft, armsIn, surge);
    // ---- procedural: stance body (pelvis blade, lean, head), then the kick (pelvis + legs), then the pistol carry arms — in that order so the hands are solved
    //      against the chest where it finally is. The wall hold has its own body and arms (wallBody / wallArms), sharing the stance weight by wl.
    if (wSt * (1 - wl) > 0.003) this.stanceBody(wSt * (1 - wl), s, wKstand);
    if (wSt * wl > 0.003) this.wallBody(wSt * wl);
    if (kt >= 0) this.kickBody(kt, wKbody);
    if (wCarry * (1 - wl) > 0.003) this.carryArms(wCarry * (1 - wl), kt);
    if (wSt * wl > 0.003) this.wallArms(wSt * wl);
    if (wLp > 0.003) this.lockpickLayer(wLp);
    if (wHd > 0.003) this.holdLayer(wHd);
    if (wHe > 0.003) this.heldLayer(wHe, dt);
    if (this.upperW.aim > 0.003) {
      // pitch: blend aim down/neutral/up (assume ±45° extremes), remainder handled by twist
      const pn = clamp(this.aimPitch / (45 * Math.PI / 180), -1, 1);
      sampleClip(c.Pistol_Aim_Neutral, 0, false, this.pUpper);
      if (pn > 0.01) { sampleClip(c.Pistol_Aim_Up, 0, false, this.pB); blendPose(this.pUpper, this.pB, pn, null, this.pUpper); }
      else if (pn < -0.01) { sampleClip(c.Pistol_Aim_Down, 0, false, this.pB); blendPose(this.pUpper, this.pB, -pn, null, this.pUpper); }
      if (this.shootT >= 0) {
        sampleClip(c.Pistol_Shoot, this.shootT, false, this.pB);
        const w = this.shootT < 0.04 ? this.shootT / 0.04 : clamp(1 - (this.shootT - 0.3) / 0.3, 0, 1);
        blendPose(this.pUpper, this.pB, w, null, this.pUpper);
      }
      blendPose(this.pA, this.pUpper, this.upperW.aim, this.upperMask, this.pA);
    }
    if (this.shootT >= 0) { this.shootT += dt; if (this.shootT > c.Pistol_Shoot.duration) this.shootT = -1; }   // (the shot's clock runs whether or not the aim clip shows it — the gun hold takes its recoil off it instead)
    if (this.sigT >= 0) { this.signalLayer(this.sigT); this.sigT += dt; if (this.sigT > SIGNAL_DUR[this.sigKind]) this.sigT = -1; }
    if (this.reloadT >= 0) {
      sampleClip(c.Pistol_Reload, this.reloadT, false, this.pB);
      const d = c.Pistol_Reload.duration; const w = clamp(Math.min(this.reloadT / 0.12, (d - this.reloadT) / 0.15), 0, 1);
      blendPose(this.pA, this.pB, w, this.upperMask, this.pA);
      this.reloadT += dt * (d / 1.6); if (this.reloadT > d) this.reloadT = -1;   // retimed to the weapon's 1.6 s
    }
    if (this.throwT >= 0) { this.throwLayer(this.throwT); this.throwT += dt; if (this.throwT > THROW_DUR) { this.throwT = -1; this.thrCarry = this.thrLast = null; } }
    if (this.strikeT >= 0) {
      sampleClip(c.Punch_Cross, this.strikeT, false, this.pB);
      const d = c.Punch_Cross.duration; const w = clamp(Math.min(this.strikeT / 0.05, (d - this.strikeT) / 0.15), 0, 1);
      blendPose(this.pA, this.pB, w, this.upperMask, this.pA);
      this.strikeT += dt * 1.15; if (this.strikeT > d) this.strikeT = -1;
    }
    if (this.hitT >= 0) {
      sampleClip(c.Hit_Chest, this.hitT, false, this.pB);
      const w = clamp(1 - Math.abs(this.hitT / c.Hit_Chest.duration - 0.3) / 0.7, 0, 1) * 0.8;
      blendPose(this.pA, this.pB, w, this.upperMask, this.pA);
      this.hitT += dt; if (this.hitT > c.Hit_Chest.duration) this.hitT = -1;
    }
    // kick clock (impact flag is for this one update)
    this.kickHit = false;
    if (kt >= 0) { const nt = kt + dt; this.kickHit = kt < KICK_IMPACT && nt >= KICK_IMPACT; this.kickT = nt > KICK_DUR ? -1 : nt; }
    this.lpT += dt;
    this.pose.copy(this.pA);
    // ---- procedural twists ----
    this.twists.length = 0;
    const y = this.aimYawOffset * (1 - wSt * wl * (1 - this.upperW.aim)); const p = this.aimPitch * (this.upperW.aim);   // (back to the wall, the controller's idle chest twist is the hold's to place — flat, the head turned instead — until he actually aims)
    const look = this.lookYawExtra;
    this.twists.push({ node: nd.spine1, yaw: y * 0.25, pitch: 0 });
    this.twists.push({ node: nd.spine2, yaw: y * 0.35, pitch: 0 });
    this.twists.push({ node: nd.spine3, yaw: y * 0.40, pitch: -p * 0.35 });
    if (Math.abs(look) > 1e-3) { this.twists.push({ node: nd.neck, yaw: look * 0.4, pitch: 0 }); this.twists.push({ node: nd.head, yaw: look * 0.6, pitch: 0 }); }
  }

  // ================================================================ procedural layers
  // Conventions (see skeleton.ts): everything is posed in the un-twisted model space of the local pose (+Z the figure's front, +X its left, +Y up); the aim /
  // look twists are applied afterwards by the SkeletonInstance and turn these layers with the chest like they turn the clips. Body offsets are added straight
  // into pA scaled by the layer weight (a rotation × w is still a rotation); limbs that must reach a point are IK-solved on a copy and blended in through a mask.

  /** turn node i (and its subtree) about model axes, in place */
  private turn(pose: Pose, i: number, q: Quat) { rotateModel(pose, i, q, this.fk.parentRot(pose, i)); }
  /** shift node i's origin by a model-space offset (the hips: its parent 'root' is the Z-up armature frame) */
  private shift(pose: Pose, i: number, off: Vec3) { const l = quat.rotate(quat.conj(this.fk.parentRot(pose, i)), off); const p = pose.pos(i); pose.setPos(i, [p[0] + l[0], p[1] + l[1], p[2] + l[2]]); }
  /** curl the four fingers (0 straight … 1 fist) and set the thumb (0 alongside … 1 folded over the fist) of one hand, from the rest (straight) hand */
  private hand(pose: Pose, side: 0 | 1, curl: readonly number[], thumb: number, spread = 0) {
    const J = [72, 92, 62];   // full-fist flexion per knuckle (°)
    this.fingers[side].forEach((f, fi) => f.forEach((node, j) => {
      let q = quat.axisAngle(AX, curl[fi] * J[j] * DEGR);
      if (j === 0 && spread) q = quat.mul(quat.axisAngle(AZ, (fi - 1.2) * spread * (side ? 1 : -1) * 9 * DEGR), q);   // fan about the knuckle's palm-normal axis
      pose.setRot(node, quat.normalize(quat.mul(this.rest.rot(node), q)));
    }));
    const th = this.thumbs[side], TJ = [30, 25, 45];   // thumb: swings across under the fingers joint by joint
    th.forEach((node, j) => pose.setRot(node, quat.normalize(quat.mul(this.rest.rot(node), quat.axisAngle(AX, thumb * TJ[j] * DEGR)))));
  }
  /** Model rotation for a hand given where its fingers point (`f`) and which way the back of the hand faces (`b`, made ⊥ f) — calibrated per side from the rest
   *  pose (T-pose, palms down), so one description fits either hand. */
  private handRot(side: 0 | 1, f: Vec3, b: Vec3): Quat {
    const F = v3.normalize(f); const B = v3.normalize(v3.mad(b, F, -v3.dot(b, F)));
    return quat.mul(quat.fromBasis(v3.cross(F, B), F, B), this.handFix[side]);
  }

  /** Turn neck (40 %) and head (60 %) so the gaze — the head direction that looks dead ahead in the standing idle — points at `target` (model space) from
   *  where the eyes are; limited to what a neck does (±40° yaw, −35…+45° pitch), scaled by w, with an optional roll (head cocked) on top. */
  private lookAt(pose: Pose, target: Vec3, w: number, roll = 0) {
    const nd = this.nodes, fk = this.fk; fk.run(pose);
    const hr = fk.rot[nd.head]; const gaze = quat.rotate(hr, this.gazeLocal); const eyes = v3.mad(v3.mad(fk.pos[nd.head], quat.rotate(hr, AY), 0.1), gaze, 0.08);
    const want = v3.normalize(v3.sub(target, eyes));
    const yawE = clamp(wrapAngle(Math.atan2(want[0], want[2]) - Math.atan2(gaze[0], gaze[2])), -0.7, 0.7);
    const pitchE = clamp(Math.asin(clamp(gaze[1], -1, 1)) - Math.asin(clamp(want[1], -1, 1)), -0.6, 0.8);   // + = look further down = pitch forward
    const look = (k: number) => quat.mul(qYaw(yawE * k * w), qPitch(pitchE * k * w));
    this.turn(pose, nd.neck, look(0.4)); this.turn(pose, nd.head, roll ? quat.mul(qRoll(roll), look(0.6)) : look(0.6));
  }
  /** where a bending knee should head: forward, a quarter of the way toward the line of the (often turned-out) foot, and a little up */
  private kneePole(footRot: Quat): Vec3 { const f = quat.rotate(footRot, AY); const h = v3.normalize([f[0], 0, f[2] + 1e-6]); return v3.normalize([h[0] * 0.25, 0.5, h[2] * 0.25 + 0.75]); }
  /** Stance, body half: pelvis bladed toward the wall and settled a few cm onto soft knees — over feet that are IK-pinned back exactly where the locomotion has
   *  them (so nothing slides: standing, the legs twist and bend under a turning pelvis; walking, the cycle's foot placement is untouched) — the upper spine
   *  countering ⅔ of the blade, a forward lean spread hips-first up the spine with the head levelled back onto the target, and the stance's own head yaw / chin.
   *  `stand` (0…1, the kick) takes the blade and the squat out and halves the lean: a kick is thrown standing square to the door. */
  private stanceBody(w: number, speed: number, stand: number) {
    const pA = this.pA, nd = this.nodes, st = this.st, fk = this.fk;
    const blade = st.blade.x * DEGR * w * (1 - stand) * lerp(1, 0.5, smoothstep(0.3, 1.3, speed)); const drop = Math.max(0, st.squat.x) * w * (1 - stand);
    const replant = Math.abs(blade) > 1e-4 || drop > 1e-4;
    let fL: Vec3 | null = null, fR: Vec3 | null = null, rL: Quat | null = null, rR: Quat | null = null;
    if (replant) { fk.run(pA); fL = v3.copy(fk.pos[nd.footL]); fR = v3.copy(fk.pos[nd.footR]); rL = [...fk.rot[nd.footL]] as Quat; rR = [...fk.rot[nd.footR]] as Quat; }
    if (Math.abs(blade) > 1e-4) {
      this.turn(pA, nd.hips, qYaw(blade));                                                   // pelvis toward the wall…
      this.turn(pA, nd.spine1, qYaw(-blade * 0.25)); this.turn(pA, nd.spine2, qYaw(-blade * 0.25)); this.turn(pA, nd.spine3, qYaw(-blade * 0.15));   // …chest comes ⅔ of the way back
    }
    if (drop > 1e-4) { this.shift(pA, nd.hips, [0, -drop, 0]); this.turn(pA, nd.hips, qPitch(drop * 40 * DEGR)); }   // sit a little into the knees, pelvis tipping forward with it (≈3° at 7 cm)
    const lean = st.lean.x * DEGR * w * (1 - 0.5 * stand);
    this.turn(pA, nd.spine1, qPitch(lean * 0.45)); this.turn(pA, nd.spine2, qPitch(lean * 0.35)); this.turn(pA, nd.spine3, qPitch(lean * 0.2));   // hips lead, chest follows
    const hp = st.headPitch.x * DEGR * w, hy = st.headYaw.x * DEGR * w, unl = lean + drop * 40 * DEGR;
    this.turn(pA, nd.neck, quat.mul(qYaw(hy * 0.4), qPitch(-unl * 0.4 + hp * 0.4))); this.turn(pA, nd.head, quat.mul(qYaw(hy * 0.6), qPitch(-unl * 0.6 + hp * 0.6)));   // level the head back out of the lean, then the stance's own chin / door look
    if (replant) {   // feet back where they were, soles as they were, knees sent out over the toes
      fk.run(pA);
      twoBoneIK(fk, pA, nd.thighL, nd.shinL, nd.footL, fL!, this.kneePole(rL!), 0.6, rL!);
      twoBoneIK(fk, pA, nd.thighR, nd.shinR, nd.footR, fR!, this.kneePole(rR!), 0.6, rR!);
    }
  }

  /** Deliberate movement and the crouch's centre of gravity by pace (`deliberate`; the shares come from update(): Dl = deliberate × crouch-walking, pace = creep →
   *  full crouch pace, soft = deliberate × walking upright, armsIn = the arms' share, surge = this frame's fore-aft ride of the lower body).
   *  - Legs: the crouch cycle is time-sampled and in place (its pelvis keys no travel), so there is no root motion to counter-scale against the lower cadence —
   *    the stride is warped at the feet instead: each ankle is IK-held where the cycle has it, sole as it was, with its fore-aft travel scaled about the
   *    gait's middle (gait.mid) by the same factor the cadence came down by (SNEAK.stride: the planted foot moves back exactly as fast as it did, so nothing
   *    is added to whatever the clip itself slides), and by a little more with pace (paceStride, cadence untouched: that share only closes some of the
   *    clip's own shortfall). Both feet and the pelvis ride `surge` together (the sampled phase lingers after each plant — update() — and this is what
   *    keeps the planted foot's ground speed through it, and what reads as the body hanging back over the foot just set down, then coming on).
   *  - Pelvis: lower (4 cm deliberate + 3 cm at full pace crouched — the knees, bent past 90° down there, just fold further; 1 cm upright, over feet
   *    re-planted where they were, so the knees soften instead of the soles sinking), tipped forward a touch as it settles.
   *  - Torso: a few degrees further over, hips first up the spine, the neck and head taking all of it (and the pelvis tip) back out: the eyes stay where
   *    they were. Left to the aim clip while aiming.
   *  - Arms (IK on a copy, in through the arms mask, the two schemes blended by the relaxed weight so drawing / holstering carries over): the pistol idle's
   *    two-hand hold comes down and back toward the lap with the elbows to the ribs, the support hand moved rigidly with the gun hand so the grip holds;
   *    free hands come in toward the thighs, a shade down and back, elbows following. Whatever poses the arms after this (the carries, the wall, lock
   *    work, aim, a throw, a reload) does so over it. */
  private sneakLayer(Dl: number, pace: number, soft: number, armsIn: number, surge: number) {
    const pA = this.pA, pB = this.pB, nd = this.nodes, fk = this.fk, S = SNEAK;
    const K = (1 + S.stride * Dl) * (1 + S.paceStride * pace);
    const drop = S.drop * Dl + S.paceDrop * pace + S.softDrop * soft;
    const tip = drop * 25 * DEGR;                                                    // ≈ 1.75° at 7 cm
    const lean = (S.lean * Dl + S.paceLean * pace) * DEGR * (1 - this.upperW.aim);
    if (K > 1.0005 || drop > 2e-4 || Math.abs(surge) > 2e-4) {
      fk.run(pA);
      const fL = v3.copy(fk.pos[nd.footL]), fR = v3.copy(fk.pos[nd.footR]); const rL = [...fk.rot[nd.footL]] as Quat, rR = [...fk.rot[nd.footR]] as Quat;
      const m = this.gait.mid; const tgt = (f: Vec3): Vec3 => [f[0], f[1], m + (f[2] - m) * K + surge];
      this.shift(pA, nd.hips, [0, -drop, surge]); if (tip > 1e-5) this.turn(pA, nd.hips, qPitch(tip));
      fk.run(pA);   // feet: where the cycle had them but for the longer stride and the surge, soles as they were, in the clip's own knee plane (pole weight 0: at no stride / drop / surge this is the identity, so the layer fades to exactly nothing)
      twoBoneIK(fk, pA, nd.thighL, nd.shinL, nd.footL, tgt(fL), this.kneePole(rL), 0, rL);
      twoBoneIK(fk, pA, nd.thighR, nd.shinR, nd.footR, tgt(fR), this.kneePole(rR), 0, rR);
    }
    if (lean + tip > 1e-5) {
      this.turn(pA, nd.spine1, qPitch(lean * 0.45)); this.turn(pA, nd.spine2, qPitch(lean * 0.35)); this.turn(pA, nd.spine3, qPitch(lean * 0.2));   // hips lead, chest follows
      const unl = lean + tip; this.turn(pA, nd.neck, qPitch(-unl * 0.4)); this.turn(pA, nd.head, qPitch(-unl * 0.6));                          // …and the head comes level back out of all of it
    }
    if (armsIn > 0.003) {
      pB.copy(pA); fk.run(pB);
      const cr = clamp(this.crouch, 0, 1), r = clamp(this.upperW.relaxed, 0, 1), k = lerp(0.5, 1, cr);
      const pR0 = v3.copy(fk.pos[nd.handR]), pL0 = v3.copy(fk.pos[nd.handL]); const rR0 = [...fk.rot[nd.handR]] as Quat, rL0 = [...fk.rot[nd.handL]] as Quat;
      const gunR = v3.add(pR0, v3.lerp(S.gunInStand, S.gunIn, cr));
      const freeR: Vec3 = [pR0[0] + Math.min(S.handIn, Math.max(0, -pR0[0])) * k, pR0[1] - 0.02 * k, pR0[2] - 0.03 * k];   // (the right hand lives at −x: in = toward the middle, never past it)
      const freeL: Vec3 = [pL0[0] - Math.min(S.handIn, Math.max(0, pL0[0])) * k, pL0[1] - 0.02 * k, pL0[2] - 0.03 * k];
      twoBoneIK(fk, pB, nd.upperArmR, nd.forearmR, nd.handR, v3.lerp(freeR, gunR, r), v3.lerp([-0.05, 0.4, -1], [-0.12, -1, -0.5], r), lerp(0.35, 0.7, r), rR0);   // elbow: back and a shade out hanging; down and in on the gun
      const gunL = v3.add(fk.pos[nd.handR], v3.sub(pL0, pR0));                                                                                                // the support hand rigid with wherever the gun hand actually got to
      twoBoneIK(fk, pB, nd.upperArmL, nd.forearmL, nd.handL, v3.lerp(freeL, gunL, r), v3.lerp([0.05, 0.4, -1], [0.2, -1, -0.35], r), lerp(0.35, 0.7, r), rL0);
      blendPose(pA, pB, clamp(armsIn, 0, 1), this.armsMask, pA);
    }
  }

  /** Both arms onto the pistol: the right (gun) hand at chest + [side, up, fwd] (model axes), the muzzle at `pitch` and level in roll, the left hand keeping the
   *  exact hold it has in Pistol_Aim_Neutral (both hands are moved by one rigid transform of the clip's grip, so the support hand never leaves the gun); elbows
   *  swivelled down-and-in by `tuck`. Solved on a copy seeded with the aim clip's shoulders / hands / fingers, blended in through the arms mask. A little sway
   *  and, during a kick, a pump of the gun toward the chest keep it from reading as a still. */
  private carryArms(w: number, kickT: number) {
    const pA = this.pA, pB = this.pB, nd = this.nodes, st = this.st, fk = this.fk, t = this.idleT;
    pB.copy(pA); blendPose(pB, this.pAim, 1, this.armsMask, pB);   // current body, aim-clip arms
    fk.run(pB);
    const chest = fk.pos[nd.spine3];
    const pR0 = fk.pos[nd.handR], pL0 = fk.pos[nd.handL]; const rR0 = fk.rot[nd.handR], rL0 = fk.rot[nd.handL];
    const g0 = v3.normalize(v3.sub(fk.pos[nd.middle1R], pR0));   // the clip's bore axis as posed on this spine (Character.gunDir's definition)
    // where the gun goes: sway (slow figure-eight, mm) + kick pump
    const pump = kickT >= 0 ? 0.05 * bump(kickT, 0.15, 0.36, 0.5, 0.85) : 0;
    const fwd = st.fwd.x - pump, up = st.up.x + 0.004 * Math.sin(t * 2.1) + 0.02 * pump, sidew = st.side.x + 0.005 * Math.sin(t * 1.3 + 0.6);
    const pitch = (st.pitch.x + 0.8 * Math.sin(t * 1.7 + 1.1)) * DEGR + pump * 4, yaw = 0.6 * Math.sin(t * 1.1) * DEGR + st.blade.x * DEGR * 0.2;   // (stack: bore drifts a touch toward the door edge)
    const gd: Vec3 = [Math.sin(yaw) * Math.cos(pitch), Math.sin(pitch), Math.cos(yaw) * Math.cos(pitch)];
    // rigid motion of the grip: rotation D (shortest arc bore → wanted bore, then unroll so the sights stay up), translation to the wanted hand point
    const D = this.gripTurn(g0, rR0, gd, AY);
    const pR: Vec3 = [chest[0] + sidew, chest[1] + up, chest[2] + fwd];
    const grip = quat.rotate(D, v3.sub(pL0, pR0));   // support wrist relative to the gun wrist, turned with the gun
    const tuck = st.tuck.x;
    twoBoneIK(fk, pB, nd.upperArmR, nd.forearmR, nd.handR, pR, [-0.12 - 0.6 * (1 - tuck), -1, -0.5], 0.4 + 0.55 * tuck, quat.mul(D, rR0));
    const pL = v3.add(fk.pos[nd.handR], grip);        // from where the gun hand actually got to (an extended carry can sit at the edge of reach on some torsos: the hands must not part)
    twoBoneIK(fk, pB, nd.upperArmL, nd.forearmL, nd.handL, pL, [0.2 + 0.6 * (1 - tuck), -1, -0.35], 0.4 + 0.5 * tuck, quat.mul(D, rL0));   // (the support arm reaches across, so its elbow sits a little wider)
    blendPose(pA, pB, w, this.armsMask, pA);
  }
  /** The rotation that carries the aim clip's grip (bore `g0`, gun hand at model rotation `rR0`) onto the wanted bore `gd`: the shortest arc, then unrolled about
   *  the bore so the pistol's top strap faces `upRef` as nearly as the bore allows (AY: sights up, a level carry; toward the wall: muzzle-up by the cheek). */
  private gripTurn(g0: Vec3, rR0: Quat, gd: Vec3, upRef: Vec3): Quat {
    let D = quat.fromTo(g0, gd);
    const up0 = quat.rotate(quat.mul(D, rR0), this.gripUp);
    const want = v3.sub(upRef, v3.scale(gd, v3.dot(upRef, gd))), cur = v3.sub(up0, v3.scale(gd, v3.dot(up0, gd)));
    if (v3.len(want) > 1e-4 && v3.len(cur) > 1e-4) { const wn = v3.normalize(want), cn = v3.normalize(cur); D = quat.mul(quat.axisAngle(gd, Math.atan2(v3.dot(v3.cross(cn, wn), gd), v3.dot(cn, wn))), D); }
    return D;
  }

  /** The wall hold, body half ('wall', weight w — everything below scales with it, so the way in and out is the master spring's). In the WALL's frame (n = its
   *  normal into the room at anim.wallYaw off the figure's front, tS = along it toward stackSide):
   *  - the spine is first blended toward the standing idle's own (joint by joint), which takes the crouch clip's hunch and the walk's shoulder swing / lean out of
   *    whatever is underneath, then MEASURED and put: the chest square to the wall (let round toward the edge by the peek; update() meanwhile scales the
   *    controller's idle chest twist off, so only a real aim turns the chest off the wall), the hips→neck line `lean` off vertical toward the room (−3°: the
   *    shoulder blades on the wall; a hand more in a peek, rolling on the look-side blade) with a crouch's pelvis tilt taken back at the pelvis, and side-bent
   *    toward the edge by the peek;
   *  - the pelvis is carried back to `back` off the plane (less on the move: the legs must still reach the cycle's footfalls), lowered a little, turned a third
   *    of the way back to flat while the legs are turned to sidle, and shifted along the wall onto the lead foot by the peek;
   *  - the head is turned `gaze` off the normal along the wall (from the wall, not the chest: clamped to what a neck does), levelled, chin a touch down, and only
   *    part-way un-tilted out of a peek's lean; under the aim layer it hands back to the aim clip;
   *  - the feet are IK-planted: where the clip has them on the move, and parked on the stance's own spots — apart along the wall, the side foot stepped out and
   *    turned the way he looks (crouched: that leg out long, the other heel folded under) — blended by ground speed, swept across (with a small lift) when the
   *    side flips. */
  private wallBody(w: number) {
    const pA = this.pA, pC = this.pC, nd = this.nodes, fk = this.fk, st = this.st, ws = this.wst, S = STANCES.wall;
    const cr = clamp(this.crouch, 0, 1), s = clamp(ws.side.x, -1, 1), pk = clamp(ws.peek.x, 0, 1), e = pk * pk * (3 - 2 * pk);
    const park = clamp(ws.park.x, 0, 1);                                            // 1 parked … 0 sidling (by ground speed, eased: update())
    const aimW = this.upperW.aim;
    const ny = this.wallYaw; const n: Vec3 = [Math.sin(ny), 0, Math.cos(ny)], tr: Vec3 = [-n[2], 0, n[0]];   // wall normal (into the room) and his right along the wall, model space
    // ---- what is underneath: the clip's feet (targets on the move, and what the parked spots blend from)
    fk.run(pA);
    const fL = v3.copy(fk.pos[nd.footL]), fR = v3.copy(fk.pos[nd.footR]); const rL = [...fk.rot[nd.footL]] as Quat, rR = [...fk.rot[nd.footR]] as Quat;
    // ---- 1. spine toward the standing idle's, joint by joint (a rotation blend, so at w it is part way there — not a snap)
    pC.copy(this.rest); sampleClip(this.clips.Idle_Loop, this.idleT, true, pC);
    for (const [i, k] of [[nd.spine1, 0.9], [nd.spine2, 0.9], [nd.spine3, 0.9], [nd.neck, 0.85], [nd.head, 0.85], [nd.shoulderL, 0.7], [nd.shoulderR, 0.7]] as [number, number][])
      pA.setRot(i, quat.nlerp(pA.rot(i), pC.rot(i), k * w));
    // ---- 2. pelvis: a third of the way back to flat under turned legs, back toward the wall, down, and along the wall onto the lead foot in a peek
    fk.run(pA);
    const hips = fk.pos[nd.hips];
    const backOff = lerp(lerp(S.back![0], S.back![1], cr), S.back![2], 1 - park);   // pelvis joint off the plane
    const along = v3.dot(hips, n) - (-WALL_ROOT_OFF + backOff);                     // + = further out in the room than wanted
    const drop = lerp(S.drop![0], S.drop![1], cr) + 0.006 * (1 - park) * (1 - Math.cos(this.phase * 4 * Math.PI));   // (a touch of bob on the move, twice a cycle)
    this.shift(pA, nd.hips, v3.add(v3.add(v3.scale(n, -along * w), [0, -drop * w, 0]), v3.scale(tr, s * S.peekShift! * e * w)));
    this.turn(pA, nd.hips, quat.mul(quat.axisAngle(n, s * 3 * DEGR * e * w), qYaw(ny * 0.33 * w)));   // (about n: + tips the pelvis toward his right)
    // ---- 3. the spine measured and put, twice over (the three corrections nudge one another): the chest's facing square to n — let round toward the edge by the
    //         peek, less whatever of the controller's chest twist is still to be applied after this pose while the hold is part way in (an aim's share is left
    //         to turn the chest) — then the hips→neck line to `lean` toward the room and to the peek's side bend
    const chestYawOf = () => { const l = quat.rotate(quat.mul(fk.rot[nd.spine3], this.chestRestInv), AX); return Math.atan2(-l[2], l[0]); };   // yaw of the chest's own left axis: 0 = square to the figure's front, + = turned to its left
    const axN: Vec3 = [n[2], 0, -n[0]];   // up × n: + about it bows toward the room (about n itself: + tips toward his right)
    let hipShare = 0;
    const twist = this.aimYawOffset * (1 - w * (1 - aimW));   // the chest twist update() will still apply after this pose: the hold scales the controller's idle twist off (an aim's share stays)
    for (let pass = 0; pass < 2; pass++) {
      fk.run(pA);
      const dy = wrapAngle(ny - s * S.peekTurn! * DEGR * e - (twist - this.aimYawOffset * aimW) - chestYawOf()) * w;   // (the aim's share is the aim's to turn the chest with)
      this.turn(pA, nd.spine1, qYaw(dy * 0.3)); this.turn(pA, nd.spine2, qYaw(dy * 0.35)); this.turn(pA, nd.spine3, qYaw(dy * 0.35));
      fk.run(pA);
      const d = v3.sub(fk.pos[nd.neck], fk.pos[nd.spine1]);
      const tiltN = Math.atan2(v3.dot(d, n), d[1]), tiltT = Math.atan2(v3.dot(d, tr), d[1]);   // toward the room / toward his right, off vertical
      const dN = ((st.lean.x + 10 * e) * DEGR - tiltN) * w, dT = (s * S.peekLean! * DEGR * e - tiltT) * w;   // (peeking, the chest comes round pivoting on the look-side shoulder blade: it rolls ON the wall, so the spine comes off it a hand)
      const hs = clamp(dN, -0.45 * cr - hipShare, 0.45 * cr - hipShare); hipShare += hs;   // a crouch clip's forward pelvis tilt comes back AT the pelvis (up to ~25° in all): the back un-rounds from the bottom, not by arching the lumbar
      if (Math.abs(hs) > 1e-5) this.turn(pA, nd.hips, quat.axisAngle(axN, hs));
      const rest = dN - hs;
      this.turn(pA, nd.spine1, quat.mul(quat.axisAngle(n, dT * 0.5), quat.axisAngle(axN, rest * 0.6)));
      this.turn(pA, nd.spine2, quat.mul(quat.axisAngle(n, dT * 0.3), quat.axisAngle(axN, rest * 0.4)));
      this.turn(pA, nd.spine3, quat.axisAngle(n, dT * 0.2));
    }
    // ---- 4. head: gaze along the wall toward side, from the wall (whatever the chest gives, the neck makes up — within ±80° of the chest, the controller's crane
    //         counted in), level, chin down a touch; a little further round and half un-tilted in a peek; handed to the aim clip under the aim layer
    fk.run(pA);
    { const hw = w * (1 - aimW);
      const g = quat.rotate(fk.rot[nd.head], this.gazeLocal); const gy = Math.atan2(g[0], g[2]);
      const chestYaw = chestYawOf();
      let want = ny - s * (S.gaze! + S.peekLook! * e) * DEGR - twist;   // (whatever chest twist is still to come turns the head with it; lookYawExtra cranes the neck on top — the controller's peek)
      const lk = this.lookYawExtra; want = chestYaw - lk + clamp(wrapAngle(want + lk - chestYaw), -80 * DEGR, 80 * DEGR);   // a neck turns ~80° off the chest, crane included
      const dyaw = wrapAngle(want - gy) * hw;
      const el = Math.asin(clamp(g[1], -1, 1)); const dpitch = (el + st.headPitch.x * DEGR) * hw;   // + = look further down
      const gh = v3.normalize([g[0], 0, g[2] + 1e-6]); const axP: Vec3 = [gh[2], 0, -gh[0]];      // up × gaze: + about it nods down
      const unroll = -s * S.peekLean! * DEGR * e * 0.5 * hw;
      const q = (k: number) => quat.mul(quat.axisAngle(n, unroll * k), quat.mul(qYaw(dyaw * k), quat.axisAngle(axP, dpitch * k)));
      this.turn(pA, nd.neck, q(0.4)); this.turn(pA, nd.head, q(0.6)); }
    // ---- 5. feet: the clip's on the move; parked, the stance's own spots (lead = the side foot), blended by `park`; a flip of side sweeps them across with a lift
    const F = S.feet!, I = this.idleFeet; const u = (1 + s) / 2;   // u: 1 = the right foot leads (side + = his right)
    const spot = (lead: boolean, right: boolean): { p: Vec3; yaw: number } => {   // parked spot in the wall frame for one role of one foot: `a` is authored toward the side this role implies for it
      const a = v3.lerp(lead ? F.lead : F.trail, lead ? F.leadCrouch : F.trailCrouch, cr); const sg = right === lead ? 1 : -1;   // the right foot leads when side is +1 and trails when it is −1 (the left, the reverse)
      return { p: v3.add(v3.scale(tr, sg * a[0]), v3.scale(n, a[1] - WALL_ROOT_OFF)), yaw: -sg * a[2] * DEGR };             // (tr = his right; yaw + = toes to his left)
    };
    const footTarget = (right: boolean, fClip: Vec3, rClip: Quat, rIdle: Quat, yIdle: number): { p: Vec3; r: Quat } => {
      const lw = right ? u : 1 - u;                                     // how much this foot is the lead foot right now
      const L = spot(true, right), T = spot(false, right);
      const p = v3.lerp(T.p, L.p, lw); const yawRel = lerp(T.yaw, L.yaw, lw);
      const onToes = cr * (1 - lw);                                     // the crouched trail foot keeps the crouch clip's raised heel; a lead foot, and any standing foot, is flat (the idle's sole)
      p[1] = lerp(yIdle, fClip[1], onToes) + 0.04 * (1 - s * s) * ((ws.side.v > 0) === right ? 1 : 0.25) + 0.1 * park * (1 - park) * (0.4 + 0.6 * lw);   // (mid-flip: the foot about to lead steps, the other shuffles; settling onto the spots from the cycle: a low step, the lead foot's the clearer)
      const base = quat.nlerp(rIdle, rClip, onToes);
      const fwd = quat.rotate(base, AY); const yaw0 = Math.atan2(fwd[0], fwd[2]);   // the foot bone's +Y runs heel → toes
      const r = quat.normalize(quat.mul(qYaw(wrapAngle(ny + yawRel - yaw0)), base));
      const k = park * w;
      const q = v3.lerp(fClip, p, k); const into = -WALL_ROOT_OFF + 0.11 - v3.dot(q, n);   // the wall stops a heel: a footfall of the turned cycle that would swing through the face slides along it instead (ankle ≥ 11 cm proud)
      return { p: into > 0 ? v3.mad(q, n, into * w) : q, r: quat.nlerp(rClip, r, k) };
    };
    const tL = footTarget(false, fL, rL, I.rL, I.L[1]), tR = footTarget(true, fR, rR, I.rR, I.R[1]);
    fk.run(pA);
    twoBoneIK(fk, pA, nd.thighL, nd.shinL, nd.footL, tL.p, this.kneePole(tL.r), 0.6, tL.r);
    twoBoneIK(fk, pA, nd.thighR, nd.shinR, nd.footR, tR.p, this.kneePole(tR.r), 0.6, tR.r);
  }

  /** The wall hold, arms (weight w through the arms mask), against the body wallBody left. Armed (pistol in hand): the gun hand high and tight by the cheek on
   *  the look side — chest joint + up·up + n·fwd + tS·gunSide, going with a peek's lean — muzzle up (pitch) and canted a little along the wall, the top strap
   *  toward the wall and the palm toward the face, elbow down against the ribs; the free (left) hand flat on the wall beside the hip, fingers down and out.
   *  Unarmed: the look-side hand flat back against the wall at the hip, the far arm relaxed across the body. Both sets are built and blended by `armed` and by
   *  the continuous side, so a slot change or a flip carries the hands over instead of cutting. */
  private wallArms(w: number) {
    const pA = this.pA, pB = this.pB, nd = this.nodes, fk = this.fk, ws = this.wst, st = this.st, S = STANCES.wall, t = this.idleT;
    const s = clamp(ws.side.x, -1, 1), u = (1 + s) / 2, armed = clamp(ws.armed.x, 0, 1), pk = clamp(ws.peek.x, 0, 1), e = pk * pk * (3 - 2 * pk);
    const ny = this.wallYaw; const n: Vec3 = [Math.sin(ny), 0, Math.cos(ny)], tr: Vec3 = [-n[2], 0, n[0]];
    const inWall = (p: Vec3, off: number): Vec3 => v3.mad(p, n, (-WALL_ROOT_OFF + off) - v3.dot(p, n));   // p carried square onto the plane `off` proud of the wall
    pB.copy(pA); blendPose(pB, this.pAim, 1, this.armsMask, pB);   // the aim clip's arms: the gun hand's grip and trigger finger (free hands are re-posed below)…
    pB.setRot(nd.shoulderL, pA.rot(nd.shoulderL)); pB.setRot(nd.shoulderR, pA.rot(nd.shoulderR));   // …but not its shooting shoulders (one thrown forward, one back): the clavicles stay square with the chest on the wall
    fk.run(pB);
    const chest = v3.copy(fk.pos[nd.spine3]), hips = v3.copy(fk.pos[nd.hips]);
    const pR0 = fk.pos[nd.handR], rR0: Quat = [...fk.rot[nd.handR]] as Quat; const g0 = v3.normalize(v3.sub(fk.pos[nd.middle1R], pR0));
    // ---- the gun hand up by the cheek (look side), with sway; the offset leans with the peek about n like the chest does
    const lean = quat.axisAngle(n, s * S.peekLean! * DEGR * e * 0.8);
    const off = quat.rotate(lean, v3.add(v3.add(v3.scale(n, st.fwd.x), [0, st.up.x + 0.004 * Math.sin(t * 2.1), 0]), v3.scale(tr, s * S.gunSide! + 0.004 * Math.sin(t * 1.3 + 0.6))));
    const gunP = v3.add(chest, off);
    const pitch = (st.pitch.x + 0.8 * Math.sin(t * 1.7 + 1.1)) * DEGR, cant = 18 * DEGR;   // muzzle up, tipped a little along the wall the way he looks
    const h = v3.normalize(v3.add(v3.scale(n, Math.cos(cant)), v3.scale(tr, s * Math.sin(cant))));
    const gd = quat.rotate(lean, v3.normalize(v3.add(v3.scale(h, Math.cos(pitch)), [0, Math.sin(pitch), 0])));
    const palm = v3.normalize(v3.add(v3.scale(tr, -s), v3.scale(n, 0.35)));                 // palm toward the face (from the look side back in), a shade toward the room
    const D = this.gripTurn(g0, rR0, gd, v3.cross(gd, palm));                                 // top strap = bore × palm
    const gunR = quat.mul(D, rR0);
    const gunPole: Vec3 = v3.add(v3.add([0, -1, 0], v3.scale(tr, lerp(-0.25, 0.5, u))), v3.scale(n, lerp(0.6, 0.3, u)));   // elbow down; out to the right under a right-cheek hold, forward under a cross-body one
    // ---- hands on the wall at hip height (wrist 9 cm proud: the flat hand lies on the face), and hands relaxed across the belly
    const wallHand = (right: boolean): { p: Vec3; r: Quat; pole: Vec3 } => {
      const sg = right ? 1 : -1;
      const p = inWall(v3.add(v3.add(hips, [0, lerp(0.10, 0.06, clamp(this.crouch, 0, 1)), 0]), v3.scale(tr, sg * 0.34)), 0.09);
      const r = this.handRot(right ? 1 : 0, v3.add(v3.scale(tr, sg * 0.45), [0, -0.85, 0]), n);   // fingers down and out along the wall, back of the hand to the room
      return { p, r, pole: v3.add(v3.add(v3.scale(tr, sg * 0.8), [0, -0.4, 0]), v3.scale(n, 0.25)) };
    };
    const acrossHand = (right: boolean): { p: Vec3; r: Quat; pole: Vec3 } => {
      const sg = right ? 1 : -1;
      const p = v3.add(v3.add(v3.add(hips, [0, 0.15, 0]), v3.scale(n, 0.17)), v3.scale(tr, -sg * 0.07));   // just past the midline, a hand's breadth off the belly
      const r = this.handRot(right ? 1 : 0, v3.add(v3.scale(tr, -sg), [0, -0.35, 0]), v3.add(n, [0, 0.6, 0]));   // fingers on across and down, palm in toward the body
      return { p, r, pole: v3.add(v3.add(v3.scale(tr, sg * 0.7), [0, -1, 0]), v3.scale(n, 0.1)) };
    };
    // ---- right hand: gun (armed) | wall hand if it is the look side's, else across;  left hand: wall hand (armed, or the look side's) | across
    const wR = wallHand(true), aR = acrossHand(true), wL = wallHand(false), aL = acrossHand(false);
    const freeR = { p: v3.lerp(aR.p, wR.p, u), r: quat.nlerp(aR.r, wR.r, u), pole: v3.lerp(aR.pole, wR.pole, u) };
    const tgtR = { p: v3.lerp(freeR.p, gunP, armed), r: quat.nlerp(freeR.r, gunR, armed), pole: v3.lerp(freeR.pole, gunPole, armed) };
    const kL = lerp(1 - u, 1, armed);   // how much the left hand is on the wall
    const tgtL = { p: v3.lerp(aL.p, wL.p, kL), r: quat.nlerp(aL.r, wL.r, kL), pole: v3.lerp(aL.pole, wL.pole, kL) };
    twoBoneIK(fk, pB, nd.upperArmR, nd.forearmR, nd.handR, tgtR.p, tgtR.pole, 0.9, tgtR.r);
    twoBoneIK(fk, pB, nd.upperArmL, nd.forearmL, nd.handL, tgtL.p, tgtL.pole, 0.85, tgtL.r);
    if (armed < 0.5) this.hand(pB, 1, [0.12, 0.15, 0.2, 0.25], 0.2, 0.3);   // free right hand: open, loosely (the grip's fingers while the pistol is in it)
    this.hand(pB, 0, kL > 0.5 ? [0.05, 0.06, 0.08, 0.12] : [0.2, 0.25, 0.3, 0.35], kL > 0.5 ? 0.1 : 0.3, kL > 0.5 ? 0.5 : 0);   // left: flat and a little spread on the wall, softly curled across the body
    blendPose(pA, pB, w, this.armsMask, pA);
  }

  /** The kick, body half (arms are carryArms' business): pelvis shift / dip / drive / lean-back / counter-yaw / hike as overlapping curves, spine and head
   *  counter so the eyes stay on the door, right foot flown along a chamber → strike → recoil → plant path by IK (knee up and forward), left foot IK-held from
   *  the first frame with its sole flat. Every curve starts at zero and both foot targets start where the feet are, so the layer goes in at full weight with
   *  no fade (nothing under it can slide); it is eased off over the last 0.2 s, by when it has brought everything back onto the idle. Feet: the support foot
   *  is held where the kick found it and — if that is not the idle's spot (taken from a walk or a crouch) — set down on it in one short step while the other
   *  knee comes up; the swing foot's path starts from its own spot and lands on the idle's, so what the layer hands back is exactly the pose beneath it. The
   *  pelvis rides the door's reaction back so that the landing foot is under the hip, then the weight settles forward again with both soles planted. */
  private kickBody(t: number, w: number) {
    const pA = this.pA, pC = this.pC, nd = this.nodes, fk = this.fk, F = this.idleFeet;
    fk.run(pA);
    const K = this.kickFrom ??= { L: v3.copy(fk.pos[nd.footL]), rL: [...fk.rot[nd.footL]] as Quat, R: v3.copy(fk.pos[nd.footR]) };
    pC.copy(pA);
    // pelvis: onto the standing leg (left, +X), a small dip, drive through, lean back against the leg's mass, right hip forward, swing side hiked; after the
    // hit it is carried back (~12 cm behind its idle spot at the plant: the landing heel is under the hip) and comes forward again as the weight is shared out
    const sx = curve(t, [[0, 0], [0.22, 0.05], [0.5, 0.045], [KICK_PLANT, 0.03], [1.0, 0]]);
    const sy = curve(t, [[0, 0], [0.2, -0.02], [0.38, -0.01], [0.55, -0.02], [KICK_PLANT, -0.03], [1.0, 0]]);
    const sz = curve(t, [[0, 0], [0.16, -0.015], [0.38, 0.09], [0.46, 0.04], [0.6, -0.06], [KICK_PLANT, -0.12], [0.86, -0.07], [1.0, 0]]);
    const tilt = curve(t, [[0, 0], [0.16, 4], [0.4, -19], [0.56, -9], [KICK_PLANT, -2], [0.86, 1.5], [1.0, 0]]) * DEGR;   // + = forward: crunch into the chamber, lay back against the drive, sit back onto the plant, settle
    const hyaw = curve(t, [[0, 0], [0.2, -3], [0.4, 9], [0.6, 4], [0.9, 0]]) * DEGR;                                     // + = right hip forward
    const roll = curve(t, [[0, 0], [0.25, -5], [0.5, -4], [KICK_PLANT, -1], [0.9, 0]]) * DEGR;                            // − = right side of the pelvis up
    this.shift(pC, nd.hips, [sx, sy, sz]);
    this.turn(pC, nd.hips, quat.mul(qYaw(hyaw), quat.mul(qRoll(roll), qPitch(tilt))));
    // spine: a little more lay-back at the strike low down, shoulders squared against the hip yaw, head back onto the door
    const lay = curve(t, [[0, 0], [0.2, 3], [0.39, -8], [0.47, -3.5], [0.6, -2.5], [0.85, 0]]) * DEGR;   // the jolt of the hit comes back up the spine
    this.turn(pC, nd.spine1, quat.mul(qYaw(-hyaw * 0.3), quat.mul(qRoll(-roll * 0.4), qPitch(lay)))); this.turn(pC, nd.spine2, quat.mul(qYaw(-hyaw * 0.3), quat.mul(qRoll(-roll * 0.4), qPitch(lay * 0.7))));
    this.turn(pC, nd.spine3, qYaw(-hyaw * 0.2));
    this.turn(pC, nd.neck, qPitch(-(tilt + lay * 1.7) * 0.45)); this.turn(pC, nd.head, qPitch(-(tilt + lay * 1.7) * 0.55));
    fk.run(pC);
    // right foot: model-space path = a moving origin (its own spot → the idle's over the chamber; the same point when kicked from the idle) + offsets: forward
    // and up under the rising knee from the first frame (no hanging back), strike ~0.86 m up / 0.75 m ahead of the root, recoil, plant on the idle's spot
    const o = v3.lerp(K.R, F.R, smoothstep(0, 0.3, t)); const down = t < KICK_PLANT ? 1 : 0;   // (past the plant the offsets are exactly zero: the trailing phantom keys only shape the arrival, which is firm, not eased)
    const cx = -0.07 - F.R[0];   // the drive comes in toward the centre line from the (wide, turned-out) idle spot
    const ox = down * curve(t, [[0, 0], [0.12, cx * 0.15], [0.26, cx * 0.6], [0.38, cx], [0.5, cx * 0.8], [0.64, cx * 0.3], [KICK_PLANT, 0], [1.0, 0]]);
    // up + out in three pieces: chamber (spline; the phantom key past 0.26 only sets the hand-off slope), drive (Hermite that ARRIVES fast: ~9 m/s into the
    // door, levelling off into a horizontal thrust), and from the impact on (spline starting at rest: ~50 ms on the door as it gives, recoil, re-plant)
    const TC = 0.26, TI = KICK_IMPACT;
    const oy = down * (t < TC ? curve(t, [[0, 0], [0.08, 0.03], [0.17, 0.14], [TC, 0.4], [0.32, 0.635]]) : t < TI ? hermite(t, TC, 0.4, 3.3, TI, 0.76, 0.5)
      : curve(t, [[TI, 0.76], [0.43, 0.75], [0.5, 0.58], [0.62, 0.24], [KICK_PLANT, 0], [1.0, 0]]));
    const oz = down * (t < TC ? curve(t, [[0, 0], [0.1, 0.04], [0.17, 0.13], [TC, 0.3], [0.32, 0.745]]) : t < TI ? hermite(t, TC, 0.3, 4.1, TI, 1.02, 9)
      : curve(t, [[TI, 1.02], [0.43, 1.0], [0.5, 0.68], [0.62, 0.26], [KICK_PLANT, 0], [1.0, 0]]));
    const target: Vec3 = [o[0] + ox, Math.max(o[1] + oy, F.R[1]), o[2] + oz];
    const kneeUp: Vec3 = [-0.04, 0.5, 1];   // knee points up-and-ahead through the swing
    twoBoneIK(fk, pC, nd.thighR, nd.shinR, nd.footR, target, kneeUp, bump(t, 0.04, 0.2, 0.62, 0.8), null);
    // ankle: toes drop as the knee comes up, heel drives through the door, relax (about the foot's own hinge); then the sole is levelled onto the idle's for the plant
    rotateLocal(pC, nd.footR, quat.axisAngle(AX, curve(t, [[0, 0], [0.2, 14], [0.34, -16], [0.42, -18], [0.58, 4], [0.68, 0]]) * DEGR));
    const flat = smoothstep(0.6, KICK_PLANT, t);
    if (flat > 0) setModelRot(pC, nd.footR, quat.nlerp(quat.mul(fk.rot[nd.shinR], pC.rot(nd.footR)), F.rR, flat), fk.rot[nd.shinR]);
    // left foot: held from the first frame; if the kick found it off the idle's spot it is set down there in one short low step (0–0.24 s) and stays
    const e = smoothstep(0, 0.24, t); const far = clamp(Math.hypot(K.L[0] - F.L[0], K.L[2] - F.L[2]) / 0.2, 0, 1);
    const pin = v3.lerp(K.L, F.L, e); pin[1] += 0.05 * Math.sin(Math.PI * e) * far;
    fk.run(pC);
    twoBoneIK(fk, pC, nd.thighL, nd.shinL, nd.footL, pin, this.kneePole(F.rL), 0.6 * smoothstep(0, 0.15, t), quat.nlerp(K.rL, F.rL, e));
    blendPose(pA, pC, w, null, pA);
  }

  /** Left-hand signals: the gesture arm is IK-posed against the current body each frame (so the hand holds its place by the head while the torso moves) and blended
   *  in with staggered per-segment envelopes — shoulder first, then elbow, then hand — and out the same way, so nothing starts or stops together. */
  private signalLayer(t: number) {
    const kind = this.sigKind, D = SIGNAL_DUR[kind]; const pA = this.pA, pB = this.pB, nd = this.nodes, fk = this.fk;
    if (kind === 'go') {   // body English on the chop: the upper spine goes a few degrees with the hand — left shoulder forward and a touch down, chest tipping into it —
      const ft = curve(t, [[0.22, 0], [0.46, 1], [0.62, 0.5], [0.9, 0]]);   // arriving with the chop (0.46), giving back half while the hand holds, gone before the arm comes down
      if (ft > 1e-3) {
        const q = (k: number) => quat.mul(qYaw(-3.5 * DEGR * ft * k), quat.mul(qRoll(-2 * DEGR * ft * k), qPitch(2.5 * DEGR * ft * k)));
        this.turn(pA, nd.spine2, q(0.4)); this.turn(pA, nd.spine3, q(0.6));
        this.turn(pA, nd.neck, q(-0.3)); this.turn(pA, nd.head, q(-0.4));   // the eyes stay on where the hand is sending people (head keeps ~30 % of the turn)
      }
    }
    pB.copy(pA); fk.run(pB);
    const sh = fk.pos[nd.upperArmL];   // shoulder joint (model)
    let hand: Vec3, handRot: Quat, pole: Vec3, clav = 0, curl: number[], thumb: number, spread = 0;
    // hands are described by where the fingers point and which way the back of the hand faces (handRot)
    if (kind === 'hold') {
      // fist snaps up beside the head, palm forward; tiny settle drift while held
      const settle = 0.006 * Math.sin(t * 5.5) * bump(t, 0.3, 0.4, 0.6, 0.7);
      hand = [sh[0] + 0.14, sh[1] + 0.19 + settle, sh[2] + 0.16]; handRot = this.handRot(0, [0.12, 1, 0.12], [0, 0.05, -1]); pole = [1, -0.4, 0.35]; clav = 8;
      curl = [1, 1, 1, 1]; thumb = 1;
    } else if (kind === 'go') {
      // knife hand comes up by the ear (0 → 0.28), chops over and down to point the way (→ 0.46, overshoot, settle), holds, returns
      const u = t;
      const hx = curve(u, [[0, 0.1], [0.28, 0.1], [0.46, 0.04], [0.95, 0.04]]), hy = curve(u, [[0, 0.2], [0.28, 0.22], [0.36, 0.2], [0.46, -0.02], [0.54, 0.015], [0.95, 0.01]]), hz = curve(u, [[0, 0.1], [0.28, 0.08], [0.38, 0.3], [0.46, 0.43], [0.95, 0.42]]);
      hand = [sh[0] + hx, sh[1] + hy, sh[2] + hz];
      const chop = smoothstep(0.28, 0.46, u);   // fingers: up by the ear → forward and a little down at full chop; palm: facing the head → facing right/down
      handRot = this.handRot(0, v3.lerp([0.1, 1, 0.25], [0.05, -0.18, 1], chop), v3.lerp([1, 0, -0.2], [0.6, 0.8, 0.1], chop)); pole = [1, -0.35, 0.1]; clav = 7 * (1 - 0.6 * chop);
      curl = [0.05, 0.08, 0.12, 0.18]; thumb = 0.3; spread = 0;
    } else {
      // open hand goes up over the head and circles (≈2.2 Hz, the forearm coning from the elbow), then comes down
      const circ = bump(t, 0.16, 0.32, D - 0.42, D - 0.26); const ph = (t - 0.16) * 2.2 * Math.PI * 2;
      hand = [sh[0] + 0.08 + 0.12 * Math.cos(ph) * circ, sh[1] + 0.36 - 0.02 * circ * (1 - Math.cos(ph * 2)), sh[2] + 0.08 + 0.12 * Math.sin(ph) * circ];
      handRot = this.handRot(0, [0.35 * Math.cos(ph) * circ, 1, 0.1 + 0.35 * Math.sin(ph) * circ], [-0.2, 0, -1]); pole = [1, 0.05, 0.3]; clav = 14;
      curl = [0.1, 0.15, 0.2, 0.3]; thumb = 0.2; spread = 0.6;
    }
    if (clav) this.turn(pB, nd.shoulderL, qRoll(clav * DEGR));   // scapula comes up with a raised arm
    fk.run(pB);
    twoBoneIK(fk, pB, nd.upperArmL, nd.forearmL, nd.handL, hand, pole, 0.85, handRot);
    this.hand(pB, 0, curl, thumb, spread);
    // staggered envelopes: in — clavicle/upper arm lead, forearm +50 ms, hand +90 ms; out — same order, the hand trailing last
    const tin = 0.2, tout = 0.26, e = D;
    const seg = (lag: number) => bump(t, lag, lag + tin, e - tout + lag * 0.6 - 0.06, e - 0.06 + lag * 0.6);
    const m = this.dynMask; m.fill(0);
    const wUp = seg(0), wFore = seg(0.05), wHand = seg(0.09);
    m[nd.shoulderL] = wUp; m[nd.upperArmL] = wUp; m[nd.forearmL] = wFore;
    const setSub = (i: number, wv: number) => { m[i] = wv; for (const ch of this.ch.nodes[i].children) setSub(ch, wv); }; setSub(nd.handL, wHand);
    blendPose(pA, pB, 1, m, pA);
  }
  /** Lock work: over the forced crouch the back un-rounds enough to bring the eyes up level with the keyway (the crouch clip alone would leave the head below
   *  the hands), shoulders come up round the ears, and the head is aimed at the lock (look-at on neck + head) and cocked; both hands are IK'd to the keyway — a
   *  model-space point, so the body's breathing moves around still hands: right = the pick, raking / jiggling in an irregular 2–3 Hz rhythm, left = the tension
   *  wrench at the bottom of the keyway with a slow pressure wobble; now and then a pin sets (the pick pauses, the wrench gives a little). */
  private lockpickLayer(w: number) {
    const pA = this.pA, pB = this.pB, nd = this.nodes, fk = this.fk, t = this.lpT; const at = this.lockpickAt;
    // body, additive × w: extend the crouch clip's rounded spine so the torso sits up toward the lock, shoulders up, then aim the head
    const ext = 28 * DEGR * w;
    this.turn(pA, nd.spine1, qPitch(-ext * 0.36)); this.turn(pA, nd.spine2, qPitch(-ext * 0.39)); this.turn(pA, nd.spine3, qPitch(-ext * 0.25));
    this.turn(pA, nd.neck, qPitch(ext * 0.45)); this.turn(pA, nd.head, qPitch(ext * 0.55));   // (the clip cranes the neck to look ahead from the crouch; sitting up, that crane comes off first)
    this.turn(pA, nd.shoulderL, qRoll(10 * DEGR * w)); this.turn(pA, nd.shoulderR, qRoll(-10 * DEGR * w));
    this.lookAt(pA, at, w, 9 * DEGR * w);
    pB.copy(pA); fk.run(pB);
    // irregular rhythm: a frequency-wobbled rake plus a faster jiggle that comes and goes; every few seconds a pin 'sets' (brief pause + twist of the wrench)
    const rake = Math.sin(t * 2 * Math.PI * 2.3 + 1.4 * Math.sin(t * 2.1)), jig = Math.sin(t * 2 * Math.PI * 3.6) * (0.5 + 0.5 * Math.sin(t * 0.9 + 2));
    const setEv = smoothstep(0.7, 1, Math.sin(t * 1.7) * Math.sin(t * 0.61 + 1));   // sparse 0…1 blips
    const work = 1 - 0.8 * setEv;
    // wrists sit a hand's length back from the door face: the pick reaches the keyway from below-right, the wrench hand lower and to the left
    const pick: Vec3 = [at[0] - 0.04, at[1] - 0.035 + 0.004 * jig * work, at[2] - 0.135 + 0.01 * rake * work];
    const wrench: Vec3 = [at[0] + 0.05, at[1] - 0.075, at[2] - 0.125];
    // right hand: fingers forward into the lock, back of the hand up-and-right (pinch grip on the pick), rolling with the rake
    const rollR = (11 * rake * work + 4 * jig * work - 8 * setEv) * DEGR;
    const bR = quat.rotate(quat.axisAngle(AZ, rollR), v3.normalize([-0.6, 1, 0]));
    const rotR = this.handRot(1, [-0.1, 0.15, 1], bR);
    // left hand: below the keyway, fingers forward-up-right holding the wrench, palm up-ish; slow pressure wobble + the give when a pin sets
    const rollL = (3 * Math.sin(t * 2 * Math.PI * 0.55) + 7 * setEv) * DEGR;
    const fL = v3.normalize([-0.4, 0.5, 1]); const bL = quat.rotate(quat.axisAngle(fL, rollL), v3.normalize([0.35, -1, 0.55]));
    const rotL = this.handRot(0, fL, bL);
    twoBoneIK(fk, pB, nd.upperArmR, nd.forearmR, nd.handR, pick, [-0.45, -1, -0.45], 0.75, rotR);
    twoBoneIK(fk, pB, nd.upperArmL, nd.forearmL, nd.handL, wrench, [0.3, -1, -0.45], 0.75, rotL);
    this.hand(pB, 1, [0.55, 0.7, 0.9, 0.95], 0.75);   // pick pinched between thumb and index, the rest folded
    this.hand(pB, 0, [0.75, 0.8, 0.85, 0.85], 0.6);   // wrapped round the wrench
    blendPose(pA, pB, w, this.armsMask, pA);
  }

  /** Sam holding a man from behind (holdPose, weight w — HOLD has the numbers): over whatever the legs are doing, the torso comes forward to the man's back (leaning
   *  back instead to finish a choke, into him for the shove), the head turns to look past his right ear; then both arms by IK against that body — the left wrist
   *  past the far side of his throat so the forearm lies across it (drawn back toward Sam's own chest as the choke closes), the right hand clamped over the front
   *  of his far shoulder. The grab REACHES: each wrist travels from wherever the pose beneath has it, round the outside of the man (a bowed mid key), onto its
   *  point, the left leading; the release throws both palms into his shoulder blades. Targets move, never the weight, so nothing pops between phases; in and
   *  out of the whole thing rides the master spring. */
  private holdLayer(w: number) {
    const H = this.holdPose ?? this.hdLast, K = HOLD; const pA = this.pA, pB = this.pB, nd = this.nodes, fk = this.fk;
    const ph = H.phase, t = H.t, gun = H.variant === 'gun', G = K.gun;
    const ug = ph === 'grab' ? clamp(t / K.secs.grab, 0, 1) : 1;                              // the reach, 0 → 1
    const ck = ph === 'choke' && !gun ? smoothstep(0, K.secs.choke * 0.75, t) : 0;            // the elbow closing
    const sv = ph === 'release' ? smoothstep(0, K.secs.release * 0.6, t) : 0;                 // the shove
    const wnd = ph === 'whip' ? smoothstep(0, WHIP_WIND, t) : 0;                                                     // the whip: cocked back (and it stays cocked under the strike's blend: wind → hit is one straight whip of the hand)…
    const stk = ph === 'whip' ? (t < WHIP_STRIKE ? 0 : hermite(t, WHIP_STRIKE, 0, 0, K.secs.whip, 1, 7)) : 0;      // …brought down, ARRIVING fast (≈ 2 m/s at the bone)…
    const thru = ph === 'whip' ? smoothstep(K.secs.whip, K.secs.whip + WHIP_THRU, t) : 0;                            // …and on through as he drops (the layer lets go after it)
    // ---- body English straight into pA: lean over the three spine joints (hips lead), head yaw / pitch on neck + head, levelled back out of the lean; the gun hold
    //      keeps less chest on him and carries the head out to the other side, and its whip loads the chest back then brings it through
    const eBody = smoothstep(0.15, 0.9, ug);
    const lean = lerp(lerp(gun ? G.lean : K.lean, K.chokeLean, ck), K.shoveLean, sv) * DEGR * w * eBody;
    const side = (gun ? G.side : K.side) * DEGR * w * eBody * (1 - sv);   // (+ roll = left shoulder up = the head carried out to the right)
    const yawB = gun ? (G.windYaw * wnd * (1 - stk) + G.hitYaw * (stk - thru)) * DEGR * w : 0;
    this.turn(pA, nd.spine1, quat.mul(qYaw(yawB * 0.3), qPitch(lean * 0.45))); this.turn(pA, nd.spine2, quat.mul(qYaw(yawB * 0.35), quat.mul(qRoll(side * 0.4), qPitch(lean * 0.35)))); this.turn(pA, nd.spine3, quat.mul(qYaw(yawB * 0.35), quat.mul(qRoll(side * 0.6), qPitch(lean * 0.2))));
    const hy = (gun ? G.headYaw : K.headYaw) * DEGR * w * eBody * (1 - sv) - yawB, hp = ((gun ? G.headPitch : K.headPitch) * DEGR - lean) * w * eBody;   // (the head stays where it looks while the chest loads and comes through under it)
    this.turn(pA, nd.neck, quat.mul(qRoll(side * 0.8), quat.mul(qYaw(hy * 0.4), qPitch(hp * 0.4)))); this.turn(pA, nd.head, quat.mul(qRoll(-side * 0.9), quat.mul(qYaw(hy * 0.6), qPitch(hp * 0.6))));   // (the head un-tips itself: eyes level, out beside his)
    this.turn(pA, nd.shoulderL, qRoll(5 * DEGR * w * eBody)); this.turn(pA, nd.shoulderR, qRoll(-(gun ? G.clav + 4 * wnd * (1 - stk) : 3) * DEGR * w * eBody));   // the choking shoulder comes up and round; the gun arm's clavicle up with it
    // ---- arms on a copy, against the body as it now is
    pB.copy(pA); fk.run(pB);
    const liveL = v3.copy(fk.pos[nd.handL]), liveR = v3.copy(fk.pos[nd.handR]); const rL0 = [...fk.rot[nd.handL]] as Quat, rR0 = [...fk.rot[nd.handR]] as Quat;
    const limp = ph === 'choke' && !gun ? smoothstep(0.62 * K.secs.choke, 0.95 * K.secs.choke, t) : 0;   // (the man's own going-limp clock, heldLayer's: as his knees give the arm goes down with him — it is what holds him up)
    const give: Vec3 = [0, -(K.sag[1] - K.sag[0]) * limp, 0.025 * limp];
    const holdL = v3.add(v3.lerp(K.throat, K.tight, ck), give), holdR = v3.add(K.farShoulder, give);
    let tgtL: Vec3, tgtR: Vec3;
    if (ph === 'grab') {   // round the outside of him and on: the left leads (0 → 0.75 of the reach), the right follows (0.2 → 1)
      const uL = smoothstep(0, 0.75, ug), uR = smoothstep(0.2, 1, ug);
      tgtL = curve3(uL, [[0, liveL], [0.4, K.viaL], [0.72, K.overL], [1, holdL]]); tgtR = curve3(uR, [[0, liveR], [0.55, K.viaR], [1, holdR]]);
    } else if (ph === 'release') { tgtL = v3.lerp(holdL, K.shove[0], sv); tgtR = v3.lerp(holdR, K.shove[1], sv); }
    else { tgtL = holdL; tgtR = holdR; }
    // hands: the left's fingers run on across the throat to his right and hook back round it, palm to the neck (back of the hand forward); the right's over the top
    // of the shoulder pointing down its front, back of the hand up — both opening flat, fingers up, for the shove
    const rotHoldL = this.handRot(0, [-1, 0.12, -0.4], [0.1, 0.35, 1]), rotHoldR = this.handRot(1, [0.15, -0.55, 0.8], [-0.15, 0.85, 0.35]);
    const rotShoveL = this.handRot(0, [0.15, 1, 0.1], [0, 0.1, -1]), rotShoveR = this.handRot(1, [-0.15, 1, 0.1], [0, 0.1, -1]);
    const eL = ph === 'grab' ? smoothstep(0.1, 0.7, ug) : 1, eR = ph === 'grab' ? smoothstep(0.3, 0.95, ug) : 1;
    const rotL = quat.nlerp(quat.nlerp(rL0, rotHoldL, eL), rotShoveL, sv);
    twoBoneIK(fk, pB, nd.upperArmL, nd.forearmL, nd.handL, tgtL, [0.55, 0.25, 0.85], 0.95 * eL, rotL);   // elbow forward and a little left, up on his left collarbone: the upper arm over his shoulder, the forearm level across the throat
    const open = Math.max(sv, 1 - (ph === 'grab' ? smoothstep(0.5, 1, ug) : 1));   // fingers close as the hands arrive, open again to shove
    const cl = (a: number) => lerp(a, 0.08, open);
    this.hand(pB, 0, [cl(0.5), cl(0.58), cl(0.64), cl(0.68)], lerp(0.55, 0.1, open), open * 0.4);
    if (!gun) {
      const rotR = quat.nlerp(quat.nlerp(rR0, rotHoldR, eR), rotShoveR, sv);
      twoBoneIK(fk, pB, nd.upperArmR, nd.forearmR, nd.handR, tgtR, [-0.8, -0.55, -0.3], 0.85 * eR, rotR); // elbow down and back on the right
      this.hand(pB, 1, [cl(0.42), cl(0.5), cl(0.55), cl(0.6)], lerp(0.5, 0.1, open), open * 0.4);
    } else {
      // ---- the gun hand: the right arm re-seeded from the aim clip (its grip and trigger finger — the pistol prop was tuned against that hand), then one rigid turn of
      //      that grip onto the wanted bore (gripTurn: sights up) and the wrist IK'd to its station; through the grab it comes from wherever the carry had it, wide
      //      round his right side with the muzzle high, so nothing sweeps his back
      const m = this.dynMask; m.fill(0); const setSub = (i: number, wv: number) => { m[i] = wv; for (const ch of this.ch.nodes[i].children) setSub(ch, wv); }; setSub(nd.shoulderR, 1); m[nd.shoulderR] = 0;   // (the clavicle stays the body's: posed above)
      blendPose(pB, this.pAim, 1, m, pB); fk.run(pB);
      const pR0 = fk.pos[nd.handR], rAim: Quat = [...fk.rot[nd.handR]] as Quat; const g0 = v3.normalize(v3.sub(fk.pos[nd.middle1R], pR0));   // the clip's bore as posed on this body
      const gLive = v3.normalize(quat.rotate(quat.mul(rR0, quat.conj(rAim)), g0));                                                             // …and where the carry underneath actually has it pointing
      const out = clamp(H.gunOut ?? 0, 0, 1) * (1 - wnd), pitch = H.gunPitch ?? 0;
      const kick = this.shootT >= 0 ? bump(this.shootT, 0, 0.035, 0.06, 0.24) : 0;   // recoil: up in ~35 ms, back down over ~0.2 s
      // wrist station and bore by phase
      const rest = v3.lerp(G.hand, G.outHand, out);
      let hand: Vec3, bore: Vec3;
      const boreRest = v3.normalize(v3.lerp(G.bore, [0, Math.sin(pitch), Math.cos(pitch)], out));
      if (ph === 'grab') {
        const uR = smoothstep(0.1, 1, ug);
        hand = curve3(uR, [[0, liveR], [0.38, G.viaA], [0.7, G.viaB], [1, rest]]);
        bore = v3.normalize(curve3(uR, [[0, gLive], [0.5, G.viaBore], [1, boreRest]]));
      } else if (ph === 'whip') {
        hand = v3.lerp(v3.lerp(v3.lerp(rest, G.windHand, wnd), G.hitHand, stk), G.thruHand, thru);
        bore = v3.normalize(v3.lerp(v3.lerp(v3.lerp(boreRest, G.windBore, wnd), G.hitBore, stk), G.bore, thru));
      } else if (ph === 'release') { hand = v3.lerp(rest, K.shove[1], sv); bore = v3.normalize(v3.lerp(boreRest, G.shoveBore, sv)); }
      else { hand = rest; bore = boreRest; }
      if (kick > 0) {   // the muzzle flips up about the wrist and the hand comes back along the bore
        const upK = Math.sin(G.kickPitch * DEGR * kick); bore = v3.normalize([bore[0], bore[1] + upK, bore[2]]);
        hand = v3.mad(v3.add(hand, [0, 0.01 * kick, 0]), bore, -G.kickBack * kick);
      }
      const D = this.gripTurn(g0, rAim, bore, AY);
      const eG = ph === 'grab' ? smoothstep(0.05, 0.6, ug) : 1;   // (the first frames of the reach: still mostly the carry's own hand)
      const rotR = quat.nlerp(rR0, quat.mul(D, rAim), eG);
      const pole = v3.lerp([-1, -0.35, -0.45], [-0.7, 0.2, -0.7], wnd * (1 - stk));   // elbow out to the right and a little down; up and back as it cocks
      twoBoneIK(fk, pB, nd.upperArmR, nd.forearmR, nd.handR, hand, pole, 0.9 * Math.max(eG, 0.3), rotR);
    }
    blendPose(pA, pB, w, this.armsMask, pA);
  }

  /** The held man (heldPose, weight w). Body into pA: the spine arched back onto the arm and the head pulled back chin-up (both giving way to a forward loll as a
   *  choke takes him), shoulders hunched, a struggle — shoulder roll, a twist, now and then a jerk — whose size is his agitation, and the pelvis sinking onto soft
   *  knees (a couple of centimetres held, a hand's breadth by the end of a choke; heCk eases it either way so easing off a choke stands him back up) over feet
   *  IK-pinned where the cycle has them. Arms on a copy through the arms mask: the left hooked over the forearm at his throat, tugging; the right — torch or pistol
   *  still in it — up and out and wandering (the beam on the ceiling), coming to the arm too as the choke bites, both falling away as he goes; shoved off, both
   *  thrown forward. During the grab they come up from wherever they were. */
  private heldLayer(w: number, dt: number) {
    const H = this.heldPose ?? this.heLast, K = HOLD; const pA = this.pA, pB = this.pB, nd = this.nodes, fk = this.fk, tt = this.idleT;
    const ph = H.phase, t = H.t, ag = clamp(H.agitation, 0, 1), gun = H.variant === 'gun', G = K.gun;
    const ug = ph === 'grab' ? clamp(t / K.secs.grab, 0, 1) : 1;
    const ck = clamp(this.heCk.step(ph === 'choke' ? clamp(t / K.secs.choke, 0, 1) : 0, ph === 'choke' ? 30 : 9, dt), 0, 1);   // follows the choke closely, comes back off an aborted one over ~half a second
    const limp = smoothstep(0.62, 0.95, ck);                                                    // going: the arch and the struggle give way, the head lolls, the hands fall
    const sv = ph === 'release' ? smoothstep(0, K.secs.release * 0.5, t) : 0;
    const eIn = smoothstep(0.05, 0.6, ug);                                                     // the startle coming on through the grab
    const shot = gun && H.shot !== undefined && H.shot >= 0 ? bump(H.shot, 0, 0.05, 0.12, G.duck.secs) * w : 0;   // a round just went past his ear: he ducks off it
    const knock = ph === 'whip' ? smoothstep(K.secs.whip - 0.1, K.secs.whip, t) * w : 0;                             // the pistol landing behind his right ear: the head goes with it (and the ragdoll is seeded going that way)
    // ---- struggle: amplitude by agitation (bigger in the first second of it, gone as he goes limp / is let go; smaller with a gun at his head — he holds still for it)
    const A = (0.35 + 0.65 * ag) * (1 - limp) * (1 - sv) * (ph === 'grab' ? 1.4 : ph === 'choke' ? 1.2 : 1) * (gun ? 0.55 : 1) * w;
    const period = 3 + 3 * (0.5 + 0.5 * Math.sin(Math.floor(tt / 4.5) * 12.9898));           // a jerk every 3–6 s (re-dealt each spell), 0.2 s long
    const jerk = bump(tt % period, 0, 0.06, 0.14, 0.24) * (1 - limp) * (1 - sv) * w * (0.5 + 0.5 * ag);
    const roll = (4 * Math.sin(tt * 2 * Math.PI * 0.7) * A + 5 * jerk) * DEGR, yawS = (3 * Math.sin(tt * 2 * Math.PI * 0.45 + 1) * A - 7 * jerk) * DEGR;
    // ---- body: feet noted first (re-planted after the pelvis sinks), then arch / hunch / head
    fk.run(pA);
    const fL = v3.copy(fk.pos[nd.footL]), fR = v3.copy(fk.pos[nd.footR]); const rL = [...fk.rot[nd.footL]] as Quat, rR = [...fk.rot[nd.footR]] as Quat;
    const drop = (lerp(K.sag[0], K.sag[1], ck) + 0.03 * sv + 0.02 * shot) * w * eIn;
    if (drop > 1e-4) { this.shift(pA, nd.hips, [0, -drop, 0]); this.turn(pA, nd.hips, qPitch(drop * 40 * DEGR)); }
    const arch = lerp(K.arch, 2, limp) * DEGR * w * eIn * (1 - sv) + 10 * DEGR * sv * w + G.duck.pitch * DEGR * 0.5 * shot;     // arched back → the arch just gone as he goes (his neck stays in the crook of the arm: it is what holds him up — the head lolls over it, the knees give under it); pitched forward by the shove
    this.turn(pA, nd.spine1, quat.mul(qYaw(yawS * 0.3), qPitch(arch * 0.3))); this.turn(pA, nd.spine2, quat.mul(qYaw(yawS * 0.4), quat.mul(qRoll(roll * 0.4), qPitch(arch * 0.35)))); this.turn(pA, nd.spine3, quat.mul(qYaw(yawS * 0.3), quat.mul(qRoll(roll * 0.6), qPitch(arch * 0.35))));
    const clav = (K.clavUp * eIn * (1 - limp) + G.duck.hunch * shot) * DEGR * w; this.turn(pA, nd.shoulderL, qRoll(clav)); this.turn(pA, nd.shoulderR, qRoll(-clav));
    const aside = gun ? G.aside : K.aside, chinUp = gun ? G.chinUp : K.chinUp;
    const chin = lerp(chinUp, K.loll, limp) * DEGR * w * eIn * (1 - sv) - drop * 40 * DEGR + (G.duck.pitch * shot + G.knock.pitch * knock) * DEGR;
    const hr = (K.lollRoll * limp - aside * (1 - limp) * eIn * (1 - sv) + 3 * Math.sin(tt * 3.1) * A - G.duck.roll * shot - G.knock.roll * knock) * DEGR * w;   // (− roll: his head tips out to his left — off the face pressed by his right ear, or off the can at his right temple; ducked further off a shot, knocked that way by the whip)
    const hyw = (6 * Math.sin(tt * 1.7 + 2) * A + 9 * jerk + (gun ? G.awayYaw * eIn * (1 - sv) : 0) + G.duck.yaw * shot + G.knock.yaw * knock) * DEGR;
    this.turn(pA, nd.neck, quat.mul(qYaw(hyw * 0.4), quat.mul(qRoll(hr * 0.4), qPitch(chin * 0.45)))); this.turn(pA, nd.head, quat.mul(qYaw(hyw * 0.6), quat.mul(qRoll(hr * 0.6), qPitch(chin * 0.55))));
    if (drop > 1e-4) { fk.run(pA); twoBoneIK(fk, pA, nd.thighL, nd.shinL, nd.footL, fL, this.kneePole(rL), 0.6, rL); twoBoneIK(fk, pA, nd.thighR, nd.shinR, nd.footR, fR, this.kneePole(rR), 0.6, rR); }
    // ---- arms
    pB.copy(pA); fk.run(pB);
    const liveL = v3.copy(fk.pos[nd.handL]), liveR = v3.copy(fk.pos[nd.handR]); const rL0 = [...fk.rot[nd.handL]] as Quat, rR0 = [...fk.rot[nd.handR]] as Quat;
    const tug = 0.02 * Math.sin(tt * 2 * Math.PI * 1.3) * A * (ph === 'choke' ? 1.6 : 1);
    let tgtL: Vec3, tgtR: Vec3, rotBaseL: Quat, rotBaseR: Quat, poleL: Vec3, poleR: Vec3;
    if (!gun) {
      const claw: Vec3 = [K.clawL[0] + tug * 0.5, K.clawL[1] - Math.abs(tug), K.clawL[2] + tug];
      const wander: Vec3 = [K.torchUp[0] + 0.06 * Math.sin(tt * 0.8) * A - 0.05 * jerk, K.torchUp[1] + 0.04 * Math.sin(tt * 1.1 + 1) * A + 0.06 * jerk, K.torchUp[2] + 0.06 * Math.cos(tt * 0.6) * A];
      tgtL = v3.lerp(claw, K.hang[0], limp); tgtR = v3.lerp(v3.lerp(wander, [K.clawR[0], K.clawR[1] - Math.abs(tug), K.clawR[2] + tug], smoothstep(0.08, 0.4, ck)), K.hang[1], limp);
      // hands: the left hooked over the forearm (fingers up-right-back over it, back of the hand forward); the right a fist round the raised torch, wrist cocked back
      rotBaseL = this.handRot(0, [-0.5, 0.6, -0.6], [0.2, 0.5, 0.85]);
      rotBaseR = quat.nlerp(this.handRot(1, [-0.35, 0.85, 0.35], [-0.5, 0.1, -0.85]), this.handRot(1, [0.5, 0.55, -0.6], [-0.2, 0.5, 0.85]), smoothstep(0.08, 0.4, ck));
      poleL = [1, -0.5, 0.55]; poleR = v3.lerp([-1, -0.45, 0.25], [-1, -0.6, 0.5], smoothstep(0.08, 0.4, ck));   // elbows: out to his left and forward-down; out right, coming in for the arm
    } else {
      // the gun at his head: both hands half up in front of the shoulders, palms forward, trembling with the struggle, jerked in when a round goes past; whatever
      // the right holds (torch, pistol) points at the ceiling out of the raised fist
      const trem = 0.012 * Math.sin(tt * 2 * Math.PI * 1.1) * A, U = G.handsUp;
      tgtL = [U[0][0] - 0.03 * shot, U[0][1] + trem + 0.05 * shot + 0.03 * jerk, U[0][2] - 0.04 * shot];
      tgtR = [U[1][0] + 0.03 * shot + 0.03 * Math.sin(tt * 0.8) * A, U[1][1] - trem + 0.05 * shot + 0.04 * jerk, U[1][2] - 0.04 * shot + 0.02 * Math.cos(tt * 0.6) * A];
      tgtL = v3.lerp(tgtL, K.hang[0], limp); tgtR = v3.lerp(tgtR, K.hang[1], limp);
      rotBaseL = this.handRot(0, [0.15, 1, 0.2], [0.1, 0.15, -1]); rotBaseR = this.handRot(1, [-0.15, 1, 0.2], [-0.1, 0.15, -1]);   // fingers up, palms forward
      poleL = [0.8, -1, 0.2]; poleR = [-0.8, -1, 0.2];   // elbows down and a little out
    }
    if (ph === 'grab') { tgtL = v3.lerp(liveL, tgtL, smoothstep(0.15, 0.7, ug)); tgtR = curve3(smoothstep(0.05, 0.85, ug), [[0, liveR], [0.5, K.viaUp], [1, tgtR]]); }
    if (sv > 0) { tgtL = v3.lerp(tgtL, K.flail[0], sv); tgtR = v3.lerp(tgtR, K.flail[1], sv); }
    // limp, both hang palm-in; flung, open and forward
    const rotHangL = this.handRot(0, [0.1, -1, 0.15], [1, 0, 0.1]), rotFlailL = this.handRot(0, [0.2, 0.3, 1], [0.1, 1, -0.3]);
    const rotHangR = this.handRot(1, [-0.1, -1, 0.15], [-1, 0, 0.1]), rotFlailR = this.handRot(1, [-0.2, 0.3, 1], [-0.1, 1, -0.3]);
    const eA = ph === 'grab' ? smoothstep(0.1, 0.75, ug) : 1;
    const rotL = quat.nlerp(quat.nlerp(quat.nlerp(rL0, rotBaseL, eA), rotHangL, limp), rotFlailL, sv);
    const rotR = quat.nlerp(quat.nlerp(quat.nlerp(rR0, rotBaseR, eA), rotHangR, limp), rotFlailR, sv);
    twoBoneIK(fk, pB, nd.upperArmL, nd.forearmL, nd.handL, tgtL, poleL, 0.85 * eA, rotL);
    twoBoneIK(fk, pB, nd.upperArmR, nd.forearmR, nd.handR, tgtR, poleR, 0.85 * eA, rotR);
    const openL = Math.max(limp, sv, gun ? 0.85 : 0), fist = 1 - Math.max(limp * 0.7, sv);
    this.hand(pB, 0, [lerp(0.6, 0.15, openL), lerp(0.7, 0.2, openL), lerp(0.75, 0.25, openL), lerp(0.8, 0.3, openL)], lerp(0.3, 0.15, openL), Math.max(sv, gun ? 0.7 : 0) * 0.5);
    this.hand(pB, 1, [0.2 + 0.55 * fist, 0.25 + 0.55 * fist, 0.3 + 0.55 * fist, 0.3 + 0.55 * fist], 0.2 + 0.4 * fist, sv * 0.5);   // (never quite open: the torch stays in it until killGuard takes it)
    blendPose(pA, pB, w * eA + w * (1 - eA) * 0.35, this.armsMask, pA);   // (the first frames of the grab: mostly still his own arms)
  }

  /** The overhand throw (THR / THROW_*): body English straight into pA — pelvis carried back onto the right foot then through onto the left, turning away and
   *  unwinding ≈ 35 ms ahead of the shoulders (the spine makes up the rest of the chest's turn: −26° at the top of the backswing, +10° past square in the
   *  follow-through), a little arch then a lean over the front leg, the trunk tipping off to the glove side as the arm comes over, the right scapula up and
   *  thrown forward, the head countering most of it so the eyes stay on the target; the feet IK-pinned back where the locomotion has them (standing, the legs
   *  twist under the turning pelvis; walking, the cycle is untouched). Crouched, the torso comes up out of the crouch clip's hunch to throw and the legs' share
   *  is halved; moving, it shrinks too. Then both arms by IK against that body: the right hand along a shoulder-relative path — the elbow swinging out and up
   *  with the hand hanging under it, the forearm cocking up behind the head (the top), the elbow leading forward while the hand lays back, over the top past the
   *  head to the release point out in front (its fastest, ≈ 10 m/s, through THROW_RELEASE), on down across to the left hip and back beside the right thigh —
   *  with the elbow's swivel, the palm and a finger release keyed along it; the left comes up loosely at the target and tucks in to the ribs. Both paths begin
   *  and end on the live hand positions of the pose underneath, so the layer needs no cross-fade to get in or out (the arm weights only soften the elbow
   *  hand-over at each end). */
  private throwLayer(t: number) {
    const pA = this.pA, pB = this.pB, nd = this.nodes, fk = this.fk, T = THR;
    const cr = clamp(this.crouch, 0, 1), legK = (1 - 0.5 * cr) * (1 - 0.4 * smoothstep(0.3, 1.5, Math.abs(this.speed)));   // the legs' share: halved on bent knees, less again on the move
    // ---- what is underneath: feet to re-plant, and each hand from its shoulder (the paths' live end keys)
    fk.run(pA);
    const fL = v3.copy(fk.pos[nd.footL]), fR = v3.copy(fk.pos[nd.footR]); const rL = [...fk.rot[nd.footL]] as Quat, rR = [...fk.rot[nd.footR]] as Quat;
    const uR = v3.sub(fk.pos[nd.handR], fk.pos[nd.upperArmR]), uL = v3.sub(fk.pos[nd.handL], fk.pos[nd.upperArmL]);
    // ---- body English (every curve is zero at both ends: full weight throughout)
    const chestYaw = curve(t, T.chestYaw) * DEGR, pelYaw = curve(t, T.pelvisYaw) * DEGR * legK, spYaw = chestYaw - pelYaw;   // the pelvis turn carries the whole spine; the spine joints add the rest
    const pitch = curve(t, T.pitch) * DEGR, roll = curve(t, T.roll) * DEGR;
    const ext = 18 * DEGR * cr * bump(t, 0, 0.12, 0.5, THROW_DUR);   // crouched: sit up out of the clip's hunch to get the arm over (as lockpickLayer un-rounds the back), chest first
    this.shift(pA, nd.hips, [curve(t, T.pelX) * legK, curve(t, T.pelY) * legK, curve(t, T.pelZ) * legK]);
    this.turn(pA, nd.hips, qYaw(pelYaw));
    const seg = (k: number, kp: number, ke: number) => quat.mul(qYaw(spYaw * k), quat.mul(qRoll(roll * k), qPitch(pitch * kp - ext * ke)));
    this.turn(pA, nd.spine1, seg(0.3 + 0.12 * cr, 0.4, 0.25)); this.turn(pA, nd.spine2, seg(0.35 + 0.13 * cr, 0.35, 0.3)); this.turn(pA, nd.spine3, seg(0.35 - 0.25 * cr, 0.25, 0.45));   // (the crouch clip lays the chest joint almost flat: there the turn is taken lower down, where the spine is still upright enough to twist about the vertical)
    const hy = -chestYaw * 0.85, hr = -roll * 0.7, hp = -pitch * 0.8 + ext;   // head: stays on the target (keeps 15 % of the turn), levelled out of the lean and the tilt, craned back down out of the sit-up
    this.turn(pA, nd.neck, quat.mul(qYaw(hy * 0.4), quat.mul(qRoll(hr * 0.4), qPitch(hp * 0.45)))); this.turn(pA, nd.head, quat.mul(qYaw(hy * 0.6), quat.mul(qRoll(hr * 0.6), qPitch(hp * 0.55))));
    this.turn(pA, nd.shoulderR, quat.mul(qYaw(curve(t, T.clavFwd) * DEGR), qRoll(-curve(t, T.clavUp) * DEGR)));
    fk.run(pA);   // feet back where the locomotion has them, soles as they were
    twoBoneIK(fk, pA, nd.thighL, nd.shinL, nd.footL, fL, this.kneePole(rL), 0.6, rL);
    twoBoneIK(fk, pA, nd.thighR, nd.shinR, nd.footR, fR, this.kneePole(rR), 0.6, rR);
    // ---- arms, IK'd on a copy against the body as it now is
    pB.copy(pA); fk.run(pB);
    const shR = v3.copy(fk.pos[nd.upperArmR]), shL = v3.copy(fk.pos[nd.upperArmL]); const cin = this.thrCarry;
    const lowK = 1 - 0.45 * cr;   // crouched: the follow-through stays up off the front knee
    const lift = (K: Keys3) => K.map(([kt, p]) => [kt, [p[0], p[1] < -0.25 ? -0.25 + (p[1] + 0.25) * lowK : p[1], p[2]]] as const);
    const kR = lift(T.handR), kL = lift(T.handL); const r0 = cin?.r ?? uR, l0 = cin?.l ?? uL;
    // the ends of each path hang off the live hand: part way toward the first / from the last authored point, bowed out round the body — so an arm that starts
    // forward (walking), low and wide (crouched) or mid-swing (re-triggered) takes no detour through a waypoint that only suits the standing idle
    const via = (a: readonly number[], b: readonly number[], s: number, bow: Vec3): Vec3 => [lerp(a[0], b[0], s) + bow[0], lerp(a[1], b[1], s) + bow[1], lerp(a[2], b[2], s) + bow[2]];
    const topR = kR[0][1], endR = kR[kR.length - 1][1], reachL = kL[0][1], endL = kL[kL.length - 1][1];
    const offR = curve3(t, [[0, r0], [0.06, via(r0, topR, 0.35, [-0.075, -0.015, 0.02])], [0.125, via(r0, topR, 0.75, [-0.11, 0.03, -0.09])], ...kR, [0.62, via(endR, uR, 0.55, [-0.10, 0.02, -0.02])], [THROW_DUR, uR]]);   // (up: hanging below the lifting elbow, then out behind it at shoulder height before it cocks up over it)
    const offL = curve3(t, [[0, l0], [0.09, via(l0, reachL, 0.5, [0.04, 0, 0.02])], ...kL, [0.63, via(endL, uL, 0.5, [0.03, 0, 0.02])], [THROW_DUR, uL]]);
    twoBoneIK(fk, pB, nd.upperArmR, nd.forearmR, nd.handR, v3.add(shR, offR), swivelPole(offR, SWIV_REF_R, curve(t, T.swivR) * DEGR), 1, this.handRot(1, curve3(t, T.fingR), curve3(t, T.backR)));
    twoBoneIK(fk, pB, nd.upperArmL, nd.forearmL, nd.handL, v3.add(shL, offL), swivelPole(offL, SWIV_REF_L, curve(t, T.swivL) * DEGR), 1, this.handRot(0, curve3(t, T.fingL), curve3(t, T.backL)));
    // fingers: closed round the can, sprung open at the release, hanging loose after; the left hand firms up as it tucks in
    const open = smoothstep(THROW_RELEASE - 0.025, THROW_RELEASE + 0.03, t), loose = smoothstep(0.34, 0.49, t);
    const cR = (grip: number, flat: number, hang: number) => lerp(lerp(grip, flat, open), hang, loose);
    this.hand(pB, 1, [cR(0.5, 0.05, 0.25), cR(0.55, 0.08, 0.3), cR(0.62, 0.12, 0.35), cR(0.7, 0.15, 0.42)], cR(0.7, 0.1, 0.3));
    const firm = bump(t, 0.22, 0.32, 0.5, 0.64); const cL = (a: number) => lerp(a, a + 0.25, firm);
    this.hand(pB, 0, [cL(0.3), cL(0.35), cL(0.4), cL(0.45)], lerp(0.3, 0.6, firm));
    // ---- in through per-arm masks (clavicle subtrees): full weight but for a short hand-over of the elbow at each end; a carried-in restart is already at weight
    const wR = cin ? 1 - smoothstep(0.62, THROW_DUR, t) : bump(t, 0, 0.06, 0.62, THROW_DUR), wL = cin ? 1 - smoothstep(0.57, 0.72, t) : bump(t, 0, 0.09, 0.57, 0.72);
    const m = this.dynMask; m.fill(0);
    const setSub = (i: number, wv: number) => { m[i] = wv; for (const ch of this.ch.nodes[i].children) setSub(ch, wv); };
    setSub(nd.shoulderR, wR); setSub(nd.shoulderL, wL); m[nd.shoulderR] = 0; m[nd.shoulderL] = 0;   // (the clavicles themselves are already posed in pA; pB carries the same turn)
    blendPose(pA, pB, 1, m, pA);
    fk.run(pA); this.thrLast = { r: v3.sub(fk.pos[nd.handR], fk.pos[nd.upperArmR]), l: v3.sub(fk.pos[nd.handL], fk.pos[nd.upperArmL]) };
  }
}

/** Skeleton joints the ragdoll steers, parent-first: [node, parent row (−1 = the rigid trunk), from-particle, to-particle]. Each gets the smallest rotation (relative
 *  to its parent's) that turns its bind direction onto the particle direction; joints not listed (spine in-betweens, clavicles, hands, feet, fingers, face) keep their
 *  death-frame local transform and ride along. */
const RAG_LIMBS: [keyof typeof N, number, number, number][] = [
  ['thighL', -1, RAG.hipL, RAG.kneeL], ['shinL', 0, RAG.kneeL, RAG.ankL], ['thighR', -1, RAG.hipR, RAG.kneeR], ['shinR', 2, RAG.kneeR, RAG.ankR],
  ['upperArmL', -1, RAG.shoL, RAG.elbL], ['forearmL', 4, RAG.elbL, RAG.handL], ['upperArmR', -1, RAG.shoR, RAG.elbR], ['forearmR', 6, RAG.elbR, RAG.handR],
  ['neck', -1, RAG.neck, RAG.head],
];
/** Character.bones key each particle is seeded from (position + velocity estimate); head and hands are then offset from theirs (skull centre, mid-palm). */
const RAG_BONE: Record<keyof typeof RAG, string> = { pelvis: 'hips', spine: 'spine2', neck: 'neck', head: 'head', shoL: 'upperArmL', shoR: 'upperArmR', elbL: 'forearmL', elbR: 'forearmR', handL: 'handL', handR: 'handR', hipL: 'thighL', hipR: 'thighR', kneeL: 'shinL', kneeR: 'shinR', ankL: 'footL', ankR: 'footR' };
const RAG_BONES: string[] = []; for (const k of Object.keys(RAG) as (keyof typeof RAG)[]) RAG_BONES[RAG[k]] = RAG_BONE[k];   // in particle order
interface RagDrive { pose: Pose; ov: (NodeOverride | undefined)[]; hips: NodeOverride; hipsRot0: Quat; rot0: Quat[]; dir0: Vec3[]; delta: Quat[]; limbOv: NodeOverride[]; synced: boolean; }

export interface CharacterDesc { id: number; tint: Vec3; tint2: Vec3; pos: Vec3; yaw: number; }

export class Character {
  id: number; tint: Vec3; tint2: Vec3;
  pos: Vec3; vel: Vec3 = [0, 0, 0];
  bodyYaw: number; aimYaw: number; aimPitch = 0;
  anim: CharacterAnimator; skel: SkeletonInstance;
  world: Mat4 = m4.create();
  radius = 0.3;
  alive = true;
  /** cached bone world positions (updated in update()) */
  bones: Record<string, Vec3> = {};
  gunDir: Vec3 = [0, 0, 1]; muzzle: Vec3 = [0, 0, 0]; chestDir: Vec3 = [0, 0, 1];
  private restChestInv: Quat; private restHeadInv: Quat;
  /** last frame's bones (and the dt between): the ragdoll is seeded with the body's own momentum — a guard shot mid-stride carries on forward as he folds */
  private prevBones: Record<string, Vec3> = {}; private prevDt = 0;
  /** once dead: the particle body that owns the pose (see ragdollize / updateRagdoll); `pos` then tracks the pelvis, `bodyYaw` stays what it was */
  ragdoll: Ragdoll | null = null; private rd: RagDrive | null = null;

  constructor(public ch: GltfCharacter, d: CharacterDesc) {
    this.id = d.id; this.tint = d.tint; this.tint2 = d.tint2; this.pos = v3.copy(d.pos); this.bodyYaw = d.yaw; this.aimYaw = d.yaw;
    this.anim = new CharacterAnimator(ch); this.skel = new SkeletonInstance(ch);
    // rest-pose chest rotation for "facing" extraction
    const rp = restPose(ch, new Pose(ch.nodes.length)); this.skel.update(rp, m4.create(), null);
    this.restChestInv = quat.conj(this.skel.globalRot[this.anim.nodes.spine3]);
    this.restHeadInv = quat.conj(this.skel.globalRot[this.anim.nodes.head]);
  }

  forward(): Vec3 { return [Math.sin(this.bodyYaw), 0, Math.cos(this.bodyYaw)]; }
  aimDir(): Vec3 { const cp = Math.cos(this.aimPitch); return [Math.sin(this.aimYaw) * cp, Math.sin(this.aimPitch), Math.cos(this.aimYaw) * cp]; }

  update(dt: number) {
    if (this.ragdoll) { this.updateRagdoll(dt); return; }
    this.anim.aimYawOffset = clamp(wrapAngle(this.aimYaw - this.bodyYaw), -1.9, 1.9);
    this.anim.aimPitch = this.aimPitch;
    this.anim.update(dt);
    m4.fromTRS(this.world, this.pos, quat.yaw(this.bodyYaw), [1, 1, 1]);
    this.skel.update(this.anim.pose, this.world, this.anim.twists);
    this.readBones(dt);
  }

  /** World-space joint positions and the derived directions (chest facing, gun axis, muzzle) from the skeleton's current globals. */
  private readBones(dt: number) {
    const n = this.anim.nodes; const wp = (node: number): Vec3 => m4.transformPoint(this.world, this.skel.nodePos(node));
    const b = this.bones;
    if (dt > 1e-5 && b.hips) { const pb = this.prevBones; for (const k in b) pb[k] = b[k]; this.prevDt = dt; }   // (a dt-0 re-bake keeps the older sample: it is still one real frame back)
    b.hips = wp(n.hips); b.spine2 = wp(n.spine2); b.chest = wp(n.spine3); b.neck = wp(n.neck); b.head = wp(n.head);
    b.upperArmL = wp(n.upperArmL); b.forearmL = wp(n.forearmL); b.handL = wp(n.handL); b.upperArmR = wp(n.upperArmR); b.forearmR = wp(n.forearmR); b.handR = wp(n.handR);
    b.thighL = wp(n.thighL); b.shinL = wp(n.shinL); b.footL = wp(n.footL); b.toeL = wp(n.toeL); b.thighR = wp(n.thighR); b.shinR = wp(n.shinR); b.footR = wp(n.footR); b.toeR = wp(n.toeR);
    b.fingerR = wp(n.middle1R); b.fingerL = wp(n.middle1L);
    b.headTop = v3.add(b.head, [0, 0.2, 0]);
    // chest facing (rest → current delta applied to model +Z), in world
    const qYaw = quat.yaw(this.bodyYaw);
    const chestDelta = quat.mul(this.skel.globalRot[n.spine3], this.restChestInv);
    this.chestDir = v3.normalize(quat.rotate(quat.mul(qYaw, chestDelta), [0, 0, 1]));
    // gun direction: wrist → middle finger base, muzzle ahead of the hand
    this.gunDir = v3.normalize(v3.sub(b.fingerR, b.handR));
    if (!this.ragdoll && (this.anim.upper === 'aim' || this.anim.shooting) && !this.anim.inThrow) {
      // when aiming, trust the analytic aim direction more (animation is approximate) — except mid-throw, when the hand (and whatever it holds) is off round the head
      this.gunDir = this.aimDir();          // the prop, the rail light and the shot all share this exact direction
    }
    this.muzzle = v3.mad(b.handR, this.gunDir, 0.26);
  }

  /** Die: stop animating and hand the skeleton to a ragdoll seeded from the current pose (twists baked in) and the bones' velocities, then shove it — a round
   *  along `dir` carries the body that way and pitches it head-first the same way (shot from the front: over backwards, away from the shooter; in the back:
   *  onto its face), knees kicked out so it drops rather than topples like a plank; `quiet` (takedown) is the same fold, gentler, along `dir` = his own
   *  facing. `world` = what it collides with (null: just the floor). */
  ragdollize(world: RagdollWorld | null, dir: Vec3, quiet: boolean) {
    if (this.ragdoll) return;
    if (!this.bones.hips) this.update(0);
    this.alive = false; this.vel = [0, 0, 0];
    const n = this.anim.nodes, b = this.bones, pb = this.prevBones, pdt = this.prevDt; const Y = quat.yaw(this.bodyYaw);
    const headUp = quat.rotate(quat.mul(Y, this.skel.globalRot[n.head]), [0, 1, 0]);
    const pts: Vec3[] = RAG_BONES.map(k => v3.copy(b[k]));
    pts[RAG.head] = v3.mad(b.head, headUp, 0.12); pts[RAG.handL] = v3.lerp(b.handL, b.fingerL, 0.5); pts[RAG.handR] = v3.lerp(b.handR, b.fingerR, 0.5);
    const vel: Vec3[] | null = pdt > 1e-5 && pb.hips ? RAG_BONES.map(k => { const v = v3.scale(v3.sub(b[k], pb[k]), 1 / pdt); const l = v3.len(v); return l > 3 ? v3.scale(v, 3 / l) : v; }) : null;   // (capped: an animation pop is not momentum)
    const rag = new Ragdoll(pts, vel, world);
    // the shove, as one rigid motion of the whole body: carried along `dir` and pitched top-first the same way (ω = up × dir turns the head end toward dir),
    // a little random yaw / strength so no two land alike; then the knees are kicked out forward so it drops instead of toppling like a plank
    const f = this.forward(); const j = 0.85 + Math.random() * 0.3, twist = (Math.random() - 0.5) * (quiet ? 0.8 : 1.6);
    const h = v3.normalize([dir[0], 0, dir[2]]); const pitchAxis: Vec3 = [h[2], 0, -h[0]];   // up × h: (up × h) × up = h
    if (!quiet) { rag.addRigidVelocity(v3.scale(h, 1.9 * j), [pitchAxis[0] * 2.2 * j, twist, pitchAxis[2] * 2.2 * j]); rag.addVelocity(RAG.kneeL, f, 1.2); rag.addVelocity(RAG.kneeR, f, 1.2); }
    else { rag.addRigidVelocity(v3.scale(h, 0.5 * j), [pitchAxis[0] * 3.2 * j, twist, pitchAxis[2] * 3.2 * j]); rag.addVelocity(RAG.kneeL, h, 1.0); rag.addVelocity(RAG.kneeR, h, 1.0); }
    // what the mesh drive needs from the moment of death: the frozen local pose, world rotations of the driven joints, particle directions along them
    const pose = this.skel.bakeLocalRotations(new Pose(this.ch.nodes.length).copy(this.anim.pose));
    const rot0 = RAG_LIMBS.map(([k]) => quat.mul(Y, this.skel.globalRot[n[k]]));
    const dir0 = RAG_LIMBS.map(([, , a, c]) => v3.normalize(v3.sub(pts[c], pts[a])));
    const ov: (NodeOverride | undefined)[] = new Array(this.ch.nodes.length);
    const hips: NodeOverride = { rot: [0, 0, 0, 1], pos: [0, 0, 0] }; ov[n.hips] = hips;
    const limbOv = RAG_LIMBS.map(([k]) => { const o: NodeOverride = { rot: [0, 0, 0, 1] }; ov[n[k]] = o; return o; });
    this.ragdoll = rag; this.rd = { pose, ov, hips, hipsRot0: quat.mul(Y, this.skel.globalRot[n.hips]), rot0, dir0, delta: RAG_LIMBS.map(() => quat.ident()), limbOv, synced: false };
  }

  /** Dead: step the particles, then pose the skeleton off them — hips = the trunk cluster's rotation applied to the death-frame hips (origin at the pelvis
   *  particle), each limb joint = its parent's rotation · the shortest arc taking the parent-relative bind direction onto the particle direction · its
   *  death-frame rotation; everything else follows by FK from the frozen pose. World placement: pelvis ground point + the old yaw, so overrides go in in
   *  model space. A sleeping body costs nothing (bones / skin / capsules stand). */
  private updateRagdoll(dt: number) {
    const rag = this.ragdoll!, rd = this.rd!;
    rag.step(dt);
    if (rag.sleeping && rd.synced) return;
    rd.synced = rag.sleeping;
    const x = rag.x; this.pos = [x[0], 0, x[2]];
    const Y = quat.yaw(this.bodyYaw), Yi = quat.conj(Y);
    m4.fromTRS(this.world, this.pos, Y, [1, 1, 1]);
    const D = rag.rot;
    rd.hips.rot = quat.mul(Yi, quat.mul(D, rd.hipsRot0)); rd.hips.pos = [0, x[1], 0];   // (pelvis − pos is straight up: yaw leaves it alone)
    for (let r = 0; r < RAG_LIMBS.length; r++) {
      const [, parent, a, c] = RAG_LIMBS[r]; const Dp = parent < 0 ? D : rd.delta[parent];
      const d1 = v3.normalize([x[c * 3] - x[a * 3], x[c * 3 + 1] - x[a * 3 + 1], x[c * 3 + 2] - x[a * 3 + 2]]);
      const m = quat.fromTo(rd.dir0[r], quat.rotate(quat.conj(Dp), d1));   // swing only, measured in the parent's frame: no twist accumulates, and turning the whole body turns the limbs with it
      const delta = quat.normalize(quat.mul(Dp, m)); rd.delta[r] = delta;
      rd.limbOv[r].rot = quat.mul(Yi, quat.mul(delta, rd.rot0[r]));
    }
    this.skel.update(rd.pose, this.world, null, rd.ov);
    this.readBones(dt);
  }

  /** Unit vector pointing 'down' out of the underside of the gun (perpendicular to gunDir, toward the floor for a level gun). */
  gunUnder(): Vec3 {
    const g = this.gunDir; const gu = g[1];                       // g × (g × up) = (g·up) g − up
    const d: Vec3 = [g[0] * gu, g[1] * gu - 1, g[2] * gu];
    return v3.normalize(d);
  }

  /** Head rotation (world): takes rest-pose model-frame vectors (+Z forward, +Y up, +X the figure's left) to where the skull currently points them. */
  headRot(): Quat { return quat.mul(quat.yaw(this.bodyYaw), quat.mul(this.skel.globalRot[this.anim.nodes.head], this.restHeadInv)); }
  /** Head look direction (world). */
  headDir(): Vec3 { return v3.normalize(quat.rotate(this.headRot(), [0, 0, 1])); }

  /** Capsule shadow proxies (segment a→b, radius) in world space + a bounding sphere; consumed by BoxWorld.addCharacterCapsules. */
  capsules(): { caps: number[][]; bound: [number, number, number, number] } {
    const b = this.bones;
    const cap = (a: Vec3, c: Vec3, r: number, extendB = 0): number[] => {
      let bx = c[0], by = c[1], bz = c[2];
      if (extendB > 0) { const d = v3.normalize(v3.sub(c, a)); bx += d[0] * extendB; by += d[1] * extendB; bz += d[2] * extendB; }
      return [a[0], a[1], a[2], r, bx, by, bz];
    };
    // shoulders: widen the upper torso capsule sideways by using the shoulder line
    const shoulderMid: Vec3 = v3.lerp(b.upperArmL, b.upperArmR, 0.5);
    const caps: number[][] = [
      cap(b.hips, b.spine2, 0.15),                                   // lower torso
      cap(b.spine2, shoulderMid, 0.16),                              // chest
      cap(b.upperArmL, b.upperArmR, 0.07),                           // shoulder line
      cap(b.thighL, b.thighR, 0.11),                                 // pelvis
      cap(b.neck, v3.mad(b.head, quat.rotate(this.headRot(), [0, 1, 0]), 0.13), 0.105),   // neck + head (along the skull's up, so a lying body's head proxy lies with it)
      cap(b.upperArmL, b.forearmL, 0.052), cap(b.forearmL, b.handL, 0.042, 0.09),
      cap(b.upperArmR, b.forearmR, 0.052), cap(b.forearmR, b.handR, 0.042, 0.09),
      cap(b.thighL, b.shinL, 0.075), cap(b.shinL, b.footL, 0.058), cap(b.footL, b.toeL, 0.045, 0.03),
      cap(b.thighR, b.shinR, 0.075), cap(b.shinR, b.footR, 0.058),
    ];
    // (14 = BoxWorld.CAPS_PER_CHAR; right toe folded into the shin→foot capsule budget)
    const c = v3.lerp(b.hips, b.spine2, 0.5);
    return { caps, bound: [c[0], c[1], c[2], 1.5] };   // generous: sprint strides / raised arms + capsule radii must stay inside the cull sphere
  }

  /** Approximate ray test against the character (capsule-ish: vertical cylinder + head sphere). Returns t or -1. */
  rayHit(ro: Vec3, rd: Vec3, tmax: number): number {
    if (!this.alive) return -1;
    const top = (this.bones.headTop?.[1] ?? this.pos[1] + 1.75), bot = this.pos[1] + 0.05;
    const cx = (this.bones.hips?.[0] ?? this.pos[0]), cz = (this.bones.hips?.[2] ?? this.pos[2]);
    const r = 0.28;
    // infinite cylinder in xz
    const ox = ro[0] - cx, oz = ro[2] - cz; const a = rd[0] * rd[0] + rd[2] * rd[2]; const bq = ox * rd[0] + oz * rd[2]; const c = ox * ox + oz * oz - r * r;
    if (a < 1e-8) return -1;
    const disc = bq * bq - a * c; if (disc < 0) return -1;
    const t = (-bq - Math.sqrt(disc)) / a; if (t < 0 || t > tmax) return -1;
    const y = ro[1] + rd[1] * t; if (y < bot || y > top) return -1;
    return t;
  }
}
