// Character = animated mannequin instance: locomotion/upper-body animation graph, procedural aim twist,
// world placement, bone queries and RT proxy boxes — and, once dead, a ragdoll (ragdoll.ts) that poses the same skeleton.
import { Mat4, Quat, Vec3, m4, quat, v3, clamp, damp, wrapAngle, lerp, smoothstep } from '../math/vec';
import { GltfCharacter, Clip } from '../anim/gltf';
import { NodeOverride, Pose, PoseFK, SkeletonInstance, TwistSpec, blendPose, buildMask, restPose, rotateLocal, rotateModel, sampleClip, setModelRot, twoBoneIK } from '../anim/skeleton';
import { Ragdoll, RagdollWorld, RAG } from './ragdoll';

export type UpperMode = 'none' | 'relaxed' | 'aim' | 'torch';
/** Procedural ready postures layered over the locomotion (see CharacterAnimator.stance). */
export type Stance = 'none' | 'highReady' | 'lowReady' | 'stack';
/** Left-hand signals (CharacterAnimator.signal). */
export type SignalKind = 'hold' | 'go' | 'rally';

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
}
const STANCES: Record<Exclude<Stance, 'none'>, StanceShape> = {
  highReady: { fwd: 0.25, up: 0.15, side: -0.05, pitch: 8, lean: 7, headPitch: 2, tuck: 0.85, blade: 0, headYaw: 0, squat: 0.015 },
  lowReady: { fwd: 0.30, up: -0.215, side: -0.07, pitch: -42, lean: 3, headPitch: 1, tuck: 0.3, blade: 0, headYaw: 0, squat: 0 },
  stack: { fwd: 0.19, up: 0.13, side: -0.03, pitch: 22, lean: 9, headPitch: 4, tuck: 1, blade: 22, headYaw: 14, squat: 0.07 },
};
/** kick timing (s): total (the last ~0.25 s is the weight settling forward off the re-planted foot), and the instant the sole meets the door */
const KICK_DUR = 1.0; export const KICK_IMPACT = 0.38;
/** the swing foot is back on the floor by this time */
const KICK_PLANT = 0.72;
const SIGNAL_DUR: Record<SignalKind, number> = { hold: 0.9, go: 0.95, rally: 1.3 };

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
   *  aimYawOffset (the chest twist turns the whole carry) and with walking / crouching (the gun stays level). */
  stance: Stance = 'none';
  /** which side the wall is on for 'stack': +1 = the figure's right, −1 = its left (mirrors blade, head and gun offset) */
  stackSide: -1 | 1 = 1;
  /** Player: working a lock — forces the crouch, both hands come up to `lockpickAt` (pick in the right, tension wrench in the left) with small irregular wrist
   *  work, shoulders hunched, head in close and cocked. The pistol is not posed (the game holsters it). Eased in/out ≈ 0.35 s. */
  lockpick = false;
  /** the keyway in MODEL space (x to the figure's left, y up, z ahead of its root): default = just under a lever handle (doors.ts: 1.0 m), 0.45 m ahead */
  lockpickAt: Vec3 = [0.02, 0.96, 0.45];
  /** Output: whatever the right hand holds (pistol / canister) should not be drawn this frame — the hands are more than half way onto the lock tools (lockpick
   *  layer weight > 0.5). Whoever draws the hand props (game.ts handProps, the viewer) reads this and shows an empty hand + the holstered sidearm instead. */
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
  private stanceSeen: Stance = 'highReady'; private lpW = new Spring(); private lpT = 0;
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
  /** finger joints [side L=0/R=1][finger index,middle,ring,pinky][joint 0..2] and thumbs [side][joint] */
  private fingers: number[][][]; private thumbs: number[][];
  nodes: Record<keyof typeof N, number>;

  constructor(public ch: GltfCharacter) {
    const n = ch.nodes.length;
    this.pose = new Pose(n); this.pA = new Pose(n); this.pB = new Pose(n); this.pC = new Pose(n); this.pUpper = new Pose(n); this.pAim = new Pose(n); this.rest = restPose(ch, new Pose(n));
    for (const p of [this.pose, this.pA, this.pB, this.pC, this.pUpper, this.pAim]) p.copy(this.rest);
    const need = ['Idle_Loop', 'Walk_Loop', 'Jog_Fwd_Loop', 'Crouch_Idle_Loop', 'Crouch_Fwd_Loop', 'Pistol_Idle_Loop', 'Pistol_Aim_Neutral', 'Pistol_Aim_Up', 'Pistol_Aim_Down', 'Pistol_Shoot', 'Idle_Torch_Loop', 'Hit_Chest', 'Sprint_Loop', 'Pistol_Reload', 'Spell_Simple_Shoot', 'Punch_Cross'];   // (no death clip: dying hands the skeleton to the ragdoll, Character.ragdollize)
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
    this.fk.run(this.rest);
    this.handFix = [this.nodes.handL, this.nodes.handR].map(h => { const r = this.fk.rot[h]; const F = v3.normalize(quat.rotate(r, AY)); const B = v3.normalize(v3.mad(AY, F, -v3.dot(AY, F))); return quat.mul(quat.conj(quat.fromBasis(v3.cross(F, B), F, B)), r); });   // rest = T-pose, palms down: fingers along local +Y, back of the hand facing model +Y
    this.fk.run(this.pAim); this.gripUp = quat.rotate(quat.conj(this.fk.rot[this.nodes.handR]), AY);
    { const p = new Pose(n).copy(this.rest); sampleClip(this.clips.Idle_Loop, 0, false, p); const fk = this.fk.run(p); const nd = this.nodes;
      this.gazeLocal = quat.rotate(quat.conj(fk.rot[nd.head]), AZ);
      this.idleFeet = { L: v3.copy(fk.pos[nd.footL]), R: v3.copy(fk.pos[nd.footR]), rL: [...fk.rot[nd.footL]] as Quat, rR: [...fk.rot[nd.footR]] as Quat }; }
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
  throwItem() { this.throwT = 0; }
  get reloading() { return this.reloadT >= 0; }
  get throwing() { return this.throwT >= 0 && this.throwT < 0.3; }
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
      this.shootT = this.hitT = this.reloadT = this.throwT = this.strikeT = this.kickT = this.sigT = -1; this.kickHit = false; this.hideHeldItem = false;
      return;
    }
    // ---- one-shot clocks that shape the base graph (kick fades locomotion out under itself)
    const kt = this.kickT; const wK = kt >= 0 ? bump(kt, 0, 0.12, KICK_DUR - 0.2, KICK_DUR) : 0;   // locomotion fade under the kick
    const wKbody = kt >= 0 ? 1 - smoothstep(KICK_DUR - 0.2, KICK_DUR, kt) : 0;                      // the kick's body layer: full from the first frame (its curves and foot targets all start where the body is), eased off at the end
    const wKstand = kt >= 0 ? bump(kt, 0, 0.2, KICK_DUR - 0.3, KICK_DUR) : 0;                       // a stance's squat / blade / lean standing up square for the kick
    const wKslow = kt >= 0 ? bump(kt, 0, 0.3, KICK_DUR - 0.3, KICK_DUR) : 0;                        // what the arms and a crouch follow (hanging arms need longer than 0.1 s to find the gun)
    const wLp = clamp(this.lpW.step(this.lockpick ? 1 : 0, 17, dt), 0, 1); this.hideHeldItem = wLp > 0.5;
    this.crouch = damp(this.crouch, Math.max(this.crouchTarget, this.lockpick ? 1 : 0), 10, dt);
    const s = Math.abs(this.speed) * (1 - wK);
    // ---- locomotion (stand) ----
    // weights among idle / walk / jog by speed
    let wWalk = 0, wJog = 0, wSprint = 0;
    if (s <= P.walkSpeed) wWalk = clamp(s / P.walkSpeed, 0, 1);
    else if (s <= P.jogSpeed) { wJog = clamp((s - P.walkSpeed) / (P.jogSpeed - P.walkSpeed), 0, 1); wWalk = 1 - wJog; }
    else { wSprint = clamp((s - P.jogSpeed) / (P.sprintSpeed - P.jogSpeed), 0, 1); wJog = 1 - wSprint; }
    const wIdle = 1 - wWalk - wJog - wSprint;
    // phase advance: rate so that feet match ground speed (natural clip speed = P.walkSpeed / P.jogSpeed / P.sprintSpeed)
    const rateWalk = (s / P.walkSpeed) / c.Walk_Loop.duration, rateJog = (s / P.jogSpeed) / c.Jog_Fwd_Loop.duration, rateCrouch = (s / P.crouchSpeed) / c.Crouch_Fwd_Loop.duration, rateSprint = (s / P.sprintSpeed) / c.Sprint_Loop.duration;
    const wsum = Math.max(1e-3, wWalk + wJog + wSprint);
    let rate = (wWalk * rateWalk + wJog * rateJog + wSprint * rateSprint) / wsum;
    rate = lerp(rate, rateCrouch, this.crouch);
    if (s < 0.05) rate = 0;
    this.phase = (this.phase + dt * rate * (this.reverse ? -1 : 1) + 100) % 1;
    this.idleT += dt;
    // stand pose
    sampleClip(c.Idle_Loop, this.idleT, true, this.pA);
    if (wWalk + wJog + wSprint > 0.001) {
      if (wSprint > 0.001) {
        sampleClip(c.Jog_Fwd_Loop, this.phase * c.Jog_Fwd_Loop.duration, true, this.pB);
        sampleClip(c.Sprint_Loop, this.phase * c.Sprint_Loop.duration, true, this.pC); blendPose(this.pB, this.pC, wSprint, null, this.pB);
      } else {
        sampleClip(c.Walk_Loop, this.phase * c.Walk_Loop.duration, true, this.pB);
        if (wJog > 0.001) { sampleClip(c.Jog_Fwd_Loop, this.phase * c.Jog_Fwd_Loop.duration, true, this.pC); blendPose(this.pB, this.pC, wJog, null, this.pB); }
      }
      blendPose(this.pA, this.pB, 1 - wIdle, null, this.pA);
    }
    // ---- stance springs (run always: they hold the shape the arms morph through; the master weight is what comes and goes)
    const st = this.st; const SW = 13;   // 90 % there in 0.3 s, soft at both ends
    const held: Stance = kt >= 0 && kt < 0.55 ? 'highReady' : this.stance;   // the kick pulls whatever carry is up into the compressed high ready (an extended low ready would meet the knee), handing back for the plant
    const shape = held !== 'none' ? STANCES[held] : null; const side = this.stackSide;
    const wSt = clamp(st.w.step(this.stance !== 'none' ? 1 : 0, SW, dt), 0, 1);   // the master weight follows the stance itself (a kick from no stance shows the carry through wKslow)
    if (shape) {
      const snap = Math.max(wSt, wKslow) < 0.02 && this.stanceSeen !== held;   // taking a shape from (next to) nothing: start AT it rather than morphing from the last one under a rising weight
      if (snap) { for (const [k, sp] of Object.entries(st)) if (k !== 'w') { sp.v = 0; sp.x = k === 'blade' ? -side * shape.blade : k === 'headYaw' ? -side * shape.headYaw : k === 'side' ? shape.side - 0.02 * side * (shape.blade / 22) : (shape as unknown as Record<string, number>)[k]; } }
      this.stanceSeen = held;
      st.fwd.step(shape.fwd, SW, dt); st.up.step(shape.up, SW, dt); st.side.step(shape.side - 0.02 * side * (shape.blade / 22), SW, dt); st.pitch.step(shape.pitch, SW, dt);
      st.lean.step(shape.lean, SW, dt); st.headPitch.step(shape.headPitch, SW, dt); st.tuck.step(shape.tuck, SW, dt);
      st.blade.step(-side * shape.blade, SW * 0.8, dt); st.headYaw.step(-side * shape.headYaw, SW, dt); st.squat.step(shape.squat, SW * 0.8, dt);   // (yaw + = to the figure's left; side + = wall on its right)
    }
    const wCarry = Math.max(wSt, wKslow);   // the kick pulls the gun into the compressed carry whatever the stance
    // crouch pose (the kick stands him up)
    const crouchPose = this.crouch * (1 - wKslow);
    if (crouchPose > 0.001) {
      sampleClip(c.Crouch_Idle_Loop, this.idleT, true, this.pB);
      const wMove = clamp(s / (P.crouchSpeed * 0.6), 0, 1);
      if (wMove > 0.001) { sampleClip(c.Crouch_Fwd_Loop, this.phase * c.Crouch_Fwd_Loop.duration, true, this.pC); blendPose(this.pB, this.pC, wMove, null, this.pB); }
      blendPose(this.pA, this.pB, crouchPose, null, this.pA);
    }
    // ---- upper body layers ----
    const tgt = { relaxed: this.upper === 'relaxed' ? 1 : 0, aim: this.upper === 'aim' ? 1 : 0, torch: this.upper === 'torch' ? 1 : 0 };
    if (this.shootT >= 0) { tgt.aim = 1; tgt.relaxed = 0; tgt.torch = 0; }
    this.upperW.relaxed = damp(this.upperW.relaxed, tgt.relaxed, 9, dt);
    this.upperW.aim = damp(this.upperW.aim, tgt.aim, 14, dt);
    this.upperW.torch = damp(this.upperW.torch, tgt.torch, 9, dt);
    if (this.upperW.relaxed > 0.003) { sampleClip(c.Pistol_Idle_Loop, this.idleT, true, this.pUpper); blendPose(this.pA, this.pUpper, this.upperW.relaxed, this.upperMask, this.pA); }
    if (this.upperW.torch > 0.003) { sampleClip(c.Idle_Torch_Loop, this.idleT, true, this.pUpper); blendPose(this.pA, this.pUpper, this.upperW.torch, this.upperMask, this.pA); }
    // ---- procedural: stance body (pelvis blade, lean, head), then the kick (pelvis + legs), then the pistol carry arms — in that order so the hands are solved
    //      against the chest where it finally is
    if (wSt > 0.003) this.stanceBody(wSt, s, wKstand);
    if (kt >= 0) this.kickBody(kt, wKbody);
    if (wCarry > 0.003) this.carryArms(wCarry, kt);
    if (wLp > 0.003) this.lockpickLayer(wLp);
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
        this.shootT += dt; if (this.shootT > c.Pistol_Shoot.duration) this.shootT = -1;
      }
      blendPose(this.pA, this.pUpper, this.upperW.aim, this.upperMask, this.pA);
    }
    if (this.sigT >= 0) { this.signalLayer(this.sigT); this.sigT += dt; if (this.sigT > SIGNAL_DUR[this.sigKind]) this.sigT = -1; }
    if (this.reloadT >= 0) {
      sampleClip(c.Pistol_Reload, this.reloadT, false, this.pB);
      const d = c.Pistol_Reload.duration; const w = clamp(Math.min(this.reloadT / 0.12, (d - this.reloadT) / 0.15), 0, 1);
      blendPose(this.pA, this.pB, w, this.upperMask, this.pA);
      this.reloadT += dt * (d / 1.6); if (this.reloadT > d) this.reloadT = -1;   // retimed to the weapon's 1.6 s
    }
    if (this.throwT >= 0) {
      sampleClip(c.Spell_Simple_Shoot, this.throwT, false, this.pB);
      const d = c.Spell_Simple_Shoot.duration; const w = clamp(Math.min(this.throwT / 0.06, (d - this.throwT) / 0.12), 0, 1);
      blendPose(this.pA, this.pB, w, this.upperMask, this.pA);
      this.throwT += dt; if (this.throwT > d) this.throwT = -1;
    }
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
    const y = this.aimYawOffset; const p = this.aimPitch * (this.upperW.aim);
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
    let D = quat.fromTo(g0, gd);
    { const up0 = quat.rotate(quat.mul(D, rR0), this.gripUp); const want = v3.normalize(v3.sub(AY, v3.scale(gd, v3.dot(AY, gd)))); const cur = v3.normalize(v3.sub(up0, v3.scale(gd, v3.dot(up0, gd))));
      D = quat.mul(quat.axisAngle(gd, Math.atan2(v3.dot(v3.cross(cur, want), gd), v3.dot(cur, want))), D); }
    const pR: Vec3 = [chest[0] + sidew, chest[1] + up, chest[2] + fwd];
    const grip = quat.rotate(D, v3.sub(pL0, pR0));   // support wrist relative to the gun wrist, turned with the gun
    const tuck = st.tuck.x;
    twoBoneIK(fk, pB, nd.upperArmR, nd.forearmR, nd.handR, pR, [-0.12 - 0.6 * (1 - tuck), -1, -0.5], 0.4 + 0.55 * tuck, quat.mul(D, rR0));
    const pL = v3.add(fk.pos[nd.handR], grip);        // from where the gun hand actually got to (an extended carry can sit at the edge of reach on some torsos: the hands must not part)
    twoBoneIK(fk, pB, nd.upperArmL, nd.forearmL, nd.handL, pL, [0.2 + 0.6 * (1 - tuck), -1, -0.35], 0.4 + 0.5 * tuck, quat.mul(D, rL0));   // (the support arm reaches across, so its elbow sits a little wider)
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
    if (!this.ragdoll && (this.anim.upper === 'aim' || this.anim.shooting)) {
      // when aiming, trust the analytic aim direction more (animation is approximate)
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
