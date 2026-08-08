// Game layer core: the Game state holder and its per-frame orchestration, the Player / Guard / LightTarget types, door + prop contact plumbing.
// The systems live beside it as plain-function modules taking the Game first: player.ts (controller, takedown / drag, throws, the cursor's
// interactables), guards.ts (perception, state machine, barks, escalation), combat.ts (shots, kills, hand props, live throwables), lighting.ts
// (fixtures, OCP, blackout), mission.ts (the mission thread), squad.ts (scripted guards).
import { Vec3, v3, clamp, wrapAngle, approachAngle, damp, lerp, DEG, quat, smoothstep } from '../math/vec';
import { Engine } from '../engine';
import { Level, PatrolRoute, Chokepoint } from '../scene/level';
import { StaticCollision } from '../scene/collision';
import { GltfCharacter } from '../anim/gltf';
import { CharacterRenderer } from '../anim/characters';
import { Character } from './character';
import type { Stance, HoldPhase, HoldVariant } from './character';
import { Input } from './input';
import { FollowCamera } from './camera';
import { RtLight } from '../render/lights';
import { IrradianceProbe } from '../render/probe';
import { SmokeEmitter, SmokeSystemLike } from '../smoke/types';
import { Box, BoxFlag, makeBox } from '../scene/boxes';
import { Pistol, Ocp } from './weapons';
import { Doors, Door, DoorEvent, DoorContact } from './doors';
import { RAG } from './ragdoll';
import { Throwables, ThrowSolution, Item } from './throwables';
import { PropSystem, PropEvent, PropContact } from './props';
import { GameAudioLike, nullAudio } from './audio_iface';
import * as fx from './effects';
import { Sparks } from './sparks';
import { rigBoxes, rigLightPose, rigTorchPos, torchCarryDir, railLightDef, torchLightDef, holsterBoxes, RIG } from './rigProps';
import { PLAYER_ID, DRIVE_REACH, DRIVE_SECS } from './consts';
import { missionLive, tally, rackDist, updateMission, drivePulled, sendRackCheck } from './mission';
import { objectivePos, objectiveText } from './mission';
import { fireWeapon, killGuard, dropFromHand, hitPlayer, handProps, laserBoxes, updateItems, smokeParams, startCanister, detonateStun } from './combat';
import { dropThrowable } from './combat';
import { hearingCheck, canSee, goTo, followPath, updateGuard, attachFlashlight, onScreen } from './guards';
import { say, resetGuards, tension, updateEscalation, escalationSummary, raiseEscalation, standDownEscalation, clearingSummary, clearQueueSummary, forceClear, searchSummary, witnessSummary, standoffSummary } from './guards';
import type { SearchPlan, Pieing, Standoff } from './guards';
import type { Clearing } from './squad';
import { updatePlayer, startTakedown, updateTakedown, bodyDist, throwOrigin, doThrow, updateAim, buildInteractables } from './player';
import { throwCount, toggleDrag, teleportPlayer, startPicking, startKick, settlePlayer, startGrab, freeHeld, settleHeld, GRAB_HOLD_SECS } from './player';
import type { WallHold, Hold } from './player';
import { updateTargets } from './lighting';
import { setBlackout, endBlackout, lightEvent, ocp, setGroup, toggleTarget, repairLights } from './lighting';
import { INTERACT_KEY } from './consts';
import * as dialogue from './dialogue';
import type { DialoguePick } from './dialogue';

// ---------------------------------------------------------------- light targets
export interface LightTarget {
  name: string; group: string; kind: 'analytic' | 'emissive' | 'breaker';
  pos: Vec3;                       // fixture position (for targeting / AI)
  light?: RtLight; boxes: number[]; baseEmissive: Vec3; baseIntensity: number;
  on: boolean; broken: boolean; disabledUntil: number; fluorescentFlicker: boolean;
  factor: number;
  mains: boolean;                  // fed from the breaker (drops in a blackout); false for street lighting, exit signs, UPS-fed LEDs, beacons
  interactive: boolean;            // can be aimed at / OCP'd / shot as a fixture (server LEDs are scenery: too many, too small, pure UI noise)
  stagger: number;                 // restrike delay for this fixture's contactor group when the mains come back
  areaLights?: { light: RtLight; base: number }[];   // analytic area lights standing in for a flat emissive fixture's emission (smooth direct light + soft shadows instead of cascade rays finding a small bright box)
}

/** kick: a door kicked in — carries through the floor and is answered like a shot (guards.ts hearingCheck); every other door noise is 'door' with a level */
export interface GameEvent { announced?: boolean; kind: 'shot' | 'lightOut' | 'guardShot' | 'step' | 'door' | 'kick' | 'bang' | 'prop'; pos: Vec3; time: number; loud: boolean; level?: number; id?: number; who?: number; }
export type ThrowKind = 'smoke' | 'flash';
/** Anything the F key acts on: a marker in the world, hovered with the cursor. Lights are ranged (OCP / shoot); doors and bodies need reach. progress (0‥1): a ring round the reticle (a lock being picked, the drive coming out).
 *  'held' = the man in Sam's arm (player.ts Hold): the one row while he holds him — the question Space puts, the choke, letting him go. */
export interface Interactable { kind: 'light' | 'door' | 'body' | 'guard' | 'pistol' | 'objective' | 'held'; pos: Vec3; line1: string; line2: string; inReach: boolean; off: boolean; progress?: number; target?: LightTarget; door?: Door; guard?: Guard; item?: Item; }

// ---------------------------------------------------------------- mission
/** The one mission thread: insert (the spawn) → reach the server room → pull the drive from one rack (F, a few seconds within reach) → out through the
 *  fire exit. Everything it touches already exists (the rooms, that rack's LED switchables, the hearing model, the down / restart flow): it only watches,
 *  counts and says what is next. Geometry lives in level.mission. */
export type MissionStage = 'infiltrate' | 'drive' | 'exfil' | 'done' | 'failed';
export interface MissionStats { elapsed: number; alerts: number; spotted: number; bodies: number; shots: number; kills: number; knockouts: number; blackout: boolean; sandbox: boolean; }   // alerts = alarm EPISODES (someone went to alert while nobody was), bodies = distinct bodies discovered (dead or out cold); kills = men the PLAYER shot dead, knockouts = men he put down breathing (the takedown; a choke-out later) — combat.ts killGuard's how / by; sandbox = a cheat flag was on at some point
export interface Mission {
  stage: MissionStage; t0: number;   // t0: game time the run started (elapsed is accumulated separately: pauses and the tour do not count)
  driveT: number;                    // seconds into pulling the drive, -1 = not pulling
  pulled: boolean;                   // the drive is out: its rack stays dark until the encounter restarts
  alarmOn: boolean;                  // somebody is at 'alert' right now (edge → an alarm episode)
  sweepAt: number;                   // game time at which somebody gets sent to look at the dead rack (-1 = nobody pending)
  spottedT: number;                  // last time an alert guard had eyes on you (episode debounce for stats.spotted)
  debrief: boolean;                  // one-shot for the page: open the end card
  stats: MissionStats;
}
export type MissionRating = 'ghost' | 'panther' | 'assault';
/** Chaos Theory's own words: ghost = never spotted, no kills, no alarms; panther = lethal or loud but never actually seen; assault = they had eyes on you. */
export function missionRating(s: MissionStats): MissionRating { return s.spotted > 0 ? 'assault' : s.kills === 0 && s.alerts === 0 && s.bodies === 0 ? 'ghost' : 'panther'; }   // ghost: never seen, no alarm, nobody shot, nothing found (a silent takedown nobody finds keeps it — a man out cold is not a kill); panther: they know someone was there but never saw who
export function fmtClock(sec: number): string { const s = Math.max(0, Math.floor(sec)); return `${Math.floor(s / 60)}:${String(s % 60).padStart(2, '0')}`; }
const newMission = (t0: number): Mission => ({ stage: 'infiltrate', t0, driveT: -1, pulled: false, alarmOn: false, sweepAt: -1, spottedT: -100, debrief: false, stats: { elapsed: 0, alerts: 0, spotted: 0, bodies: 0, shots: 0, kills: 0, knockouts: 0, blackout: false, sandbox: false } });

/** the ragdoll particles a door leaf feels (Game.updateDoors): enough of the body that a leaf cannot close through it */
const BODY_DOOR_PARTICLES = [RAG.pelvis, RAG.spine, RAG.head, RAG.kneeL, RAG.kneeR, RAG.ankL, RAG.ankR];

// ---------------------------------------------------------------- player
export class Player {
  char: Character;
  crouch = false; aimHeld = false; lastFireT = -10; fireCd = 0; backpedal = false; sprinting = false;
  dragging: Guard | null = null; dragYaw = 0; dragNoiseT = 0; dragStuckT = 0;   // the body being dragged (F): crouched, facing it, its ankles in your hands (ragdoll pins), the rest trailing along the floor
  noise = 0;                                                    // HUD sound meter: the loudest thing you did lately (hearing-model level, decays)
  takedown: { g: Guard; t: number } | null = null;             // silent takedown in progress: locked behind the guard until the strike lands
  /** the man in his arm (player.ts startGrab / updateHold — Space HELD on a living guard's row from behind): the pair moves as one at a slow walk, Space taps put
   *  questions to him (Game.interrogate), Space held chokes him out quiet (holstered: the arm variant) or pistol-whips him (pistol drawn: the gun variant, which
   *  also fires past him on a click), E shoves him off knowing everything; null when his hands are his own. guardHold: seconds Space has been down on a living
   *  guard's marker this press (game.ts updateDoors: let go sooner = the takedown, GRAB_HOLD_SECS = the grab). */
  holding: Hold | null = null; guardHold = 0;
  speedSm = 0;
  visibility = 0;          // 0..1 from light meter
  irradiance = 0;
  hitFlash = 0;
  static readonly MAX_HITS = 1; hitsLeft = Player.MAX_HITS; down = false;   // one 9 mm hit and you are down, same as the guards; Enter restarts the encounter
  // equipment: 1 pistol, 2 smoke canisters, 3 stun canisters; G quick-throws whichever throwable was selected last (`throwKind`)
  pistol = new Pistol(); ocp = new Ocp(); canisters = 4; flashbangs = 2; slot: 1 | 2 | 3 = 1; throwKind: ThrowKind = 'smoke';
  holstered = false;   // E puts the selected item away / draws it again (Chaos Theory: equip / unequip); holstered + LMB near a guard = melee, the pistol's paths all check it
  nearSel = 0; nearKey = '';    // index of the highlighted nearby interaction (↑ / ↓) and the identity of the current list
  pace = 1; paceShownT = -10;   // walking-pace multiplier set with the mouse wheel, PACE_MIN‥1 of the walk / crouch-walk speed (sprint is its own held modifier); shown as ticks in the kit column
  nv = false; nvAmount = 0;
  light!: RtLight;   // the rail light: posed from the gun every frame (rigProps.rigLightPose)
  throwHeld = false; heldKind: ThrowKind = 'smoke'; throwPreview: ThrowSolution | null = null; pendingThrow: { t: number; sol: ThrowSolution; kind: ThrowKind } | null = null;
  doorHold = 0; doorCracking = false;   // F held on a door: crack it open slowly
  /** locked doors (player.ts updatePicking / updateKick): the lock being worked while F stays down — planted in a crouch at it, hands on the tools; the kick in
   *  progress — rooted square to the leaf until the foot is back down (spot / yaw: where, fixed when the kick began); fHeld: that F press has been spent (a pick that
   *  finished, a kick) and must come up before a door listens to it again */
  picking: Door | null = null; kick: { door: Door; spot: Vec3; yaw: number } | null = null; fHeld = false;
  /** back to the wall (player.ts, Q): the static face he is pressed against and slides along — null when free. For the HUD / camera: n (unit normal out of the
   *  wall into the room), along (unit tangent: his right when his back is flat to it), atEdge (parked at an OPEN end of the face — an outside corner, a door jamb —
   *  −1 / +1 = which end along `along`, 0 = not at one or the way on is walled), peek (0‥1, eased in while he keeps pushing into that open end), crouched. The
   *  rest is the controller's bookkeeping (see WallHold). Anything that plants him elsewhere (takedown, kick, pick, drag, teleport, going down) lets go first. */
  wall: WallHold | null = null;
  /** the last spot the collision sweep settled him at (player.ts collidePlayer walks him from here to wherever he has been put since, in short strides; null until the first settle, reset by a teleport) */
  safe: Vec3 | null = null;
  stepPhase = 0;
  constructor(ch: GltfCharacter, pos: Vec3) {
    this.char = new Character(ch, { id: PLAYER_ID, tint: [0.05, 0.06, 0.07], tint2: [0.1, 0.32, 0.18], pos, yaw: Math.PI });
  }
}

// ---------------------------------------------------------------- guard
export type GuardState = 'patrol' | 'suspicious' | 'alert' | 'search' | 'dead';
/** One frame's worth of orders for a scripted guard (see squad.ts). goal: walk there (nav path) at `speed` (m/s, default guard walk); face: body
 *  yaw to hold once planted (or while `strafe`); aimAt: where the gun / torch / eyes point; stance + stackSide + crouch feed the animator's tactical
 *  layers; upper: 'aim' draws the pistol, 'torch' carries the torch; arrived is written back by the runner each frame. */
export interface GuardScript { goal: Vec3 | null; speed?: number; face?: number | null; strafe?: boolean; aimAt?: Vec3 | null; stance?: Stance; stackSide?: -1 | 1; crouch?: boolean; upper?: 'aim' | 'torch' | 'relaxed' | 'none'; light?: boolean; arrived?: boolean; }
/** What the men call each other on the net (by roster order; the radio check asks for a silent man by it — guards.ts updateRollcall). */
export const CALLSIGNS = ['Kowalski', 'Novak', 'Reyes', 'Okafor', 'Lindqvist', 'Brandt'];
/** How each man takes pressure (docs/internal/grab-interrogate-design.md §4.3): steady = professional, settles, bargains, gives a fact plainly; nervy = over-talks,
 *  gives it early, unreliable extras; hard = defiant, gives nothing until properly rattled. NOTHING reads it yet — the interrogation picker, the standoff barks
 *  and the found-body lines will (one voice per man across the whole floor's speech). Keyed by callsign so a level's roster order can change under it. */
export type Temperament = 'steady' | 'nervy' | 'hard';
export const TEMPERAMENT: Record<string, Temperament> = {
  Kowalski: 'steady',    // the corridor man: runs the radio checks and walks the long loop past every door — the floor's NCO, nothing hurries him
  Novak: 'nervy',        // the cubicle farm: boxed in among the partitions with the vending machine for company, first with a 'huh?' at every creak
  Reyes: 'hard',         // lobby and break room: the front-of-house man who deals with whoever walks in — used to saying no
  Okafor: 'steady',      // (fourth route when a level has one) pairs with anyone, says little
  Lindqvist: 'nervy',    // (fifth) the new man on nights
  Brandt: 'hard',        // (sixth) ex-forces, the one the others defer to in a lockdown
};
/** How a downed man went down and by whose hand (combat.ts killGuard): shot = dead; struck (the takedown's cross, a pistol-whip later) / choked (the choke-out
 *  from a hold, later) = out cold and breathing. His state is 'dead' either way — every one of the floor's 'is this man out of the picture' tests keeps working —
 *  this is the side record the found-body lines, the room clear's grim lines, the debrief and (one day) a wake-up read. by: only the player's own count on the card. */
export type DownKind = 'shot' | 'struck' | 'choked';
export type DownBy = 'player' | 'guard' | 'world';
/** Down but breathing: put out cold (struck / choked), not shot. False for a living man and for a corpse. */
export function isBreathing(gd: Guard): boolean { return gd.state === 'dead' && gd.downKind !== null && gd.downKind !== 'shot'; }
/** What this man has PERSONALLY perceived this encounter (docs/internal/grab-interrogate-design.md §4.2) — written where the event already happens (guards.ts
 *  hearingCheck / the alert transition / body discovery and its radio call, combat.ts killGuard / detonateStun), through the same gates as the AI itself (his
 *  hearing model for sounds; line of sight + cone for what he saw; nothing global, nothing with the AI off), read by nobody yet but the panel line and the QA
 *  probes: the interrogation dialogue and the debrief will. Dies with the Guard (resetGuards). Victims are named by callsign. */
export interface Witness {
  /** sounds that actually reached him and raised him (hearingCheck): gunfire (anyone's), doors kicked in, stun charges, and the small stuff — a creak, a scrape, a
   *  footfall, a canister's pop, a light dying near him — with where the last small one seemed to come from (the doorway it spilled through if not direct) */
  heard: { shots: number; lastShotT: number; kick: number; bang: number; small: number; smallWhere: Vec3 | null; lastT: number };
  /** the highest his awareness has ever been */
  peakAwareness: number;
  /** what took him to 'alert' the last time he went (the guards.ts transition): eyes on Sam, a sound, word over the net (a colleague's contact, an order), a body —
   *  'held' is reserved for the human-shield standoff; null = never been alert. alertT: when (-1 never). */
  alertedBy: 'sight' | 'sound' | 'radio' | 'body' | 'held' | null; alertT: number;
  /** he SAW Sam put a man down (killGuard: the victim in his view as he dropped, or his eyes on Sam as the shot went) — kind as DownKind, 'grab' reserved */
  sawAct: { kind: DownKind | 'grab'; victim: string; t: number } | null;
  /** he found (walked onto and called in) this body / was told of it over the net and called over to it; breathing = out cold rather than dead, as found */
  sawBody: { victim: string; t: number; breathing: boolean } | null;
  calledToBody: { victim: string; t: number; breathing: boolean } | null;
  /** sawHeld: a colleague he saw held in Sam's arm — who, and when he first saw it (guards.ts standoffPerceive: the human shield's standoff, through his own eyes,
   *  cone and line of sight, no light asked); wasHeld: how many times Sam has had HIM in an arm lock (player.ts startGrab counts the grab itself: 1 while he is held
   *  the first time, 2 = grabbed again after being let go) */
  sawHeld: { who: string; t: number } | null; wasHeld: number;
  /** stun charges that went off in his open view (combat.ts detonateStun) */
  dazzled: number;
}
export const newWitness = (): Witness => ({ heard: { shots: 0, lastShotT: -1, kick: 0, bang: 0, small: 0, smallWhere: null, lastT: -1 }, peakAwareness: 0, alertedBy: null, alertT: -1, sawAct: null, sawBody: null, calledToBody: null, sawHeld: null, wasHeld: 0, dazzled: 0 });
/** Interrogation progress for this man (dialogue.ts, docs/internal/grab-interrogate-design.md §4): how many exchanges Sam has had out of him (stage: 0 = none yet,
 *  1 the reaction, 2 the pressure, 3 the useful thing or the refusal, 4+ nothing left — it never goes backwards this encounter), the line ids he has said (no
 *  repeats until his pool is dry; saidN counts the repeats after that), the useful things he has given (by kind), when he last spoke, and since when he has been
 *  held (heldSince: stamped by the grab and cleared when he is let go — player.ts startGrab / handBack; the sandbox button's bare exchange still stamps it itself for
 *  a man nobody holds; -1 = not held), and when Sam last started to put him out and eased off instead (abortT: a choke or a pistol-whip let go of — player.ts
 *  updateHold; -1 never: he is that much more rattled for it, dialogue.ts agitationOf). last: the panel's peek at the previous pick. Dies with the Guard
 *  (resetGuards), like the witness record it reads. */
export interface Talk { stage: number; presses: number; said: Set<string>; saidN: Map<string, number>; given: Set<string>; lastT: number; heldSince: number; abortT: number; last: { id: string; cell: string; stage: string; kind: string | null } | null; }
export const newTalk = (): Talk => ({ stage: 0, presses: 0, said: new Set(), saidN: new Map(), given: new Set(), lastT: -1, heldSince: -1, abortT: -1, last: null });
/** An errand that survives the ordinary suspicion decay (guards.ts, the 'suspicious' branch): reset the breaker; look at the rack that dropped off the network; walk a
 *  silent colleague's route looking for him ('checkOn': `who` he is, `wp` the waypoint of his being walked to now, `left` how many remain, `t0` when it began). */
export type GuardTask = { kind: 'breaker' | 'rack'; pos: Vec3 } | { kind: 'checkOn'; pos: Vec3; who: Guard; wp: number; left: number; t0: number };
export class Guard {
  char: Character; state: GuardState = 'patrol';
  /** his name on the net (CALLSIGNS by roster index) */
  callsign: string;
  routeI: number; wp = 0; wait = 0; awareness = 0; lastKnown: Vec3 | null = null; lastSeenT = -100;
  hold = false;                  // scripted (showcase tour): stand your ground while on patrol / merely suspicious (look, don't walk over)
  pinned = false;                // scripted: in a fight, keep the position you were given (shoot from it, cover the last fix from it) instead of closing in
  /** Sam has him from behind (player.ts Hold — docs/internal/grab-interrogate-design.md §2.1): NOT a state — his `state` stays what it was so what he knew survives
   *  it — but a side record updateGuard branches on before anything else, the way it does for a scripted man: no senses, no feet of his own (updateHold places and
   *  turns him), no voice but his answers to Sam (guards.ts say drops the rest), off the net (guards.ts offNet: the radio check asks after him and sends somebody
   *  round his route). since = game time of the grab; variant = an arm round the throat / the pistol at his head (character.ts HoldVariant, fixed at the grab);
   *  phase mirrors the hold's beat; reverse = Sam is backing the pair up (his legs play the cycle backwards too). */
  held: { by: 'player'; since: number; variant: HoldVariant; phase: HoldPhase; reverse: boolean } | null = null;
  bubble: { text: string; t: number; dur: number; radio: boolean } | null = null;   // what he is saying right now (drawn over his head when on screen; off-screen lines go to the log)
  found = false;                 // (dead) somebody has discovered this body — counted once for the debrief
  /** (dead) how he went down and by whose hand — set once by combat.ts killGuard; null while he lives. isBreathing() reads it: struck / choked = out cold. */
  downKind: DownKind | null = null; downBy: DownBy | null = null;
  /** what he has personally seen and heard this encounter (see Witness) — event memory for the dialogue to come; the AI itself never reads it */
  witness: Witness = newWitness();
  /** what Sam has got out of him so far (see Talk) — read and advanced only by the interrogation (dialogue.ts via Game.interrogate) */
  talk: Talk = newTalk();
  /** (dead, not found) the net has called this man and got nothing back — when (guards.ts updateRollcall; -1 = never): from then on the others count him as
   *  missing (superviseLockdown stops dealing his corpse a role) — and whether the floor has already been raised once on his account (it is, once, when the man
   *  sent round his route comes back with nothing) */
  missedAt = -1; missingRaised = false;
  /** muzzle discipline (guards.ts muzzleCheck): a living colleague stands in his line this frame, so the pistol is at the low ready and no round leaves it;
   *  clearT = seconds the line has been clear again (it comes back up after a beat, not on the frame); evalT = the game time it was last worked out (the alert
   *  branch does it early, before the shot); lineBarkT = when he (or the man in his line) last said so */
  muzzleDown = false; muzzleClearT = 0; muzzleEvalT = -1; lineBarkT = -100;
  /** alert, licensed to shoot: seconds a colleague has stood in the line of the shot itself (a man crossing is waited out for a beat; one standing there is stepped round) */
  lineInT = 0;
  /** lockdown follower: when he last had his leader in plain sight (no wall between, inside 16 m) and where the man was then — twenty seconds without, and the
   *  man nowhere near that spot, and he reports him missing (guards.ts partnerWatch) */
  leaderSeenT = -1; leaderSeenPos: Vec3 = [0, 0, 0];
  /** (dead, found) the aftermath script's bookkeeping ON the body (guards.ts bodyAftermath): how many living men have reached it (1 = the finder covering it, 2 = the
   *  one who says '…christ') and when the last did, whether that second-man line and the one closing line have been said, and a counter dealing the directions they widen out along */
  bodyVisits = 0; bodyVisitT = -1; bodyRemarked = false; bodyClosed = false; bodyFan = 0;
  /** (living) the found body this man's current suspicion is about, and where he is in its short script: walking up ('to' his own spot short of it), covering it,
   *  widening out to a search point away from it ('fan'); null = ordinary suspicion. Cleared whenever he leaves 'suspicious'. */
  bodyDuty: { body: Guard; role: 'finder' | 'called'; stage: 'to' | 'cover' | 'fan'; t: number; t0: number; spot: Vec3 | null } | null = null;
  dropped: Item | null = null;   // the torch / pistol that left his hand when he died (the sputtering light rides it)
  path: Vec3[] = []; pathI = 0; repathT = 0; pathGoal: Vec3 | null = null;
  fireCd = 1; reactT = 0; searchT = 0; lookPhase = Math.random() * 10;
  speed = 0; flashlight: RtLight; sweep = 0; beamDir: Vec3 = [0, -0.3, 1];
  shots = 0; reloadT = -1; stepPhase = 0; heardUpTo = -1; smokeTrans = 1; lightDeadUntil = -1; diedAt = -1;
  dazzledUntil = -1;   // stun canister with line of sight: no vision, no shooting, rooted to the spot until this time (see Game.detonateStun)
  sawPlayerThisFrame = false; lastPos: Vec3; stuckT = 0; stuckN = 0;
  sawBodyT = -100;                     // game time he last discovered a body: a short refractory before he can call in another (two lying together are one 'man down', not two in two frames)
  leaderStillT = 0;                    // lockdown follower: seconds the man he trails has not moved (planted beside a leader who never moves again, he turns to look at him — guards.ts followLeader)
  alertBest = 1e9; alertStallT = 0; alertRef: Vec3 | null = null; chaseT = 0;   // alert with a fix: nearest he has got to it, how long without getting nearer, the fix that was measured against (a no-progress exit into 'search', guards.ts — re-based when the fix moves), and the seconds this chase has run without eyes on Sam (its hard backstop)
  shotBlockedT = 0;                    // alert with eyes on the player: seconds his line of FIRE (not sight) has been stopped short by something solid — a jamb on his gun-hand side, a leaf, the desk Sam is behind (guards.ts: he closes along the path instead of emptying magazines into the frame)
  sightPos: Vec3 = [0, 0, 0]; sightVel: Vec3 = [0, 0, 0]; sightT = -100; ledT = -100;   // where and how fast Sam was going the last frame this man had him in view (even faintly), when that was, and the sighting whose loss has already been led (guards.ts leadFix: the fix he is left with runs on the way Sam was going, once, the frame he loses him)
  /** when the fix he is working from was PLACED (guards.ts updateGuard stamps it whenever lastKnown jumps): the search reasons about how far Sam can have got since then */
  fixT = -100; fixRef: Vec3 | null = null;
  /** stamp fixT if the fix has moved since last looked at (called once a frame by updateGuard; a fix that creeps with a man in view moves it along too) */
  trackFix(now: number) { const lk = this.lastKnown; if (!lk) { this.fixRef = null; return; } if (!this.fixRef || Math.hypot(lk[0] - this.fixRef[0], lk[2] - this.fixRef[2]) > 0.5) { this.fixRef = v3.copy(lk); this.fixT = now; } }
  /** the search that reasons about darkness (guards.ts searchPlanFor): the hide spots dealt round the fix, scored dark × cover × plausible off the renderer's own
   *  irradiance readings, visited best-first — null outside 'search' (built on its first frame, dropped on leaving it). The panel / HUD / QA read it. */
  searchPlan: SearchPlan | null = null;
  /** threshold discipline (guards.ts pieDoorway): the doorway his alert / search path is taking him through right now and where he is in slicing it — null when
   *  none; pieLast / pieLastT: the door he last went through that way (or any way, while hunting) and when, so the one he has just come through is not pied again;
   *  piePrev / piePrevT: where he stood the last frame that was looked at (the crossing itself is his stride cutting the frame line) */
  pieing: Pieing | null = null; pieLast: Door | null = null; pieLastT = -100; piePrev: Vec3 | null = null; piePrevT = -100;
  /** the human shield's standoff (guards.ts standoffPerceive / standoffAlert, docs/internal/grab-interrogate-design.md §2.2): a colleague held in Sam's arm in his
   *  view — he is 'alert' throughout, but instead of the chase and the shot he keeps his distance, works round toward Sam's back for a clean line, and talks; null
   *  when nobody he can see is being held (dropped the moment the hold ends, or four seconds after he last had the pair in view) */
  standoff: Standoff | null = null;
  stallRef: Vec3 | null = null; stallT = 0;   // walking watchdog (guards.ts notGettingAnywhere): where he last made half a metre of ground from, and the seconds since — a suspicious / searching / body-script leg that goes nowhere is dropped where he stands
  script: GuardScript | null = null;   // scripted choreography (tour beats / squad drills): while set, squad.ts drives him and the AI state machine is bypassed (the live clearing dispatcher runs a light perception pass for him; tour scripts run blind)
  task: GuardTask | null = null;   // errands that should survive the normal suspicion decay (reset the breaker; look at the rack that dropped off the network; walk a silent colleague's route)
  /** alarm escalation (guards.ts updateEscalation): drawn = the sidearm is out on patrol (level ≥ 1) — handProps / attachFlashlight / killGuard read it where they
   *  used to read 'patrol ⇒ torch'; leader = lockdown pairing: trail this man while calm (followT: re-plan clock, ≤ 2 path plans a second; followGoal: his
   *  station behind him); post = lockdown: hold this corridor junction instead of walking the route */
  drawn = false; leader: Guard | null = null; followT = 0; followGoal: Vec3 = [0, 0, 0]; post: Chokepoint | null = null;
  constructor(ch: GltfCharacter, id: number, route: PatrolRoute, routeI: number, flashlight: RtLight, tint: Vec3) {
    const pos = v3.copy(route.points[0]); const p1 = route.points[1 % route.points.length];
    this.char = new Character(ch, { id, tint, tint2: [0.02, 0.02, 0.025], pos, yaw: v3.yawTo(pos, p1) });   // facing down the first leg
    this.lastPos = v3.copy(pos); this.routeI = routeI; this.flashlight = flashlight; this.callsign = CALLSIGNS[Math.max(0, id - 10) % CALLSIGNS.length];
  }
}

// ---------------------------------------------------------------- game
export class Game {
  col: StaticCollision;
  chars: CharacterRenderer;
  player: Player; guards: Guard[] = [];
  targets: LightTarget[] = [];
  events: GameEvent[] = [];
  probe: IrradianceProbe;
  /** The AI's window on the renderer's light, by key — the same GPU irradiance queries the light meter is made of (1–2 frames' latency, read back by key):
   *  lightQuery asks for the irradiance at a world point under `key`, lightAt returns the latest answer for it (max-axis irradiance, W/m² — what the meter maps
   *  to visibility) or undefined while there is none. The probe by default; the QA harness (whose fake GPU never answers) swaps in a table to test the logic. */
  lightQuery: (key: string, pos: Vec3) => void = (key, pos) => this.probe.query(key, pos, 0);
  lightAt: (key: string) => number | undefined = key => this.probe.get(key)?.[0];
  /** drop a stale answer for `key` (a search plan re-using its keys for new spots): the probe keeps the last value per key for ever otherwise */
  lightForget: (key: string) => void = key => { this.probe.results.delete(key); };
  /** irradiance queries the searching guards queued THIS frame (guards.ts searchProbes keeps them inside a budget well under the probe's 32; the QA soak asserts it) */
  searchProbeN = 0;
  time = 0;
  aiEnabled = true; godMode = false; infiniteAmmo = false; showDebug = false; spectate = -1; // guard index the camera follows (-1 = player); infiniteAmmo: sandbox bottomless magazine, applied to whichever pistol the current Player carries (survives restarts)
  playerInvisible = false;   // showcase tour: guards neither see nor hear the idle player
  /** 0‥1, how close Sam is to being walked in on — for the LOOK of his movement only (fed to player.char.anim.deliberate each frame, see update(); the HUD may
   *  read it too): 1 with a living guard who has not clocked him inside ~3 m, gone by ~7 m; nothing once that man is alert; ≥ 0.6 while he is suspicious or
   *  searching; ≥ 0.5 while a lit torch beam lies across Sam. Nothing the AI, the speeds or the meters read. */
  threatProximity = 0; private torchOnT = -100;   // (game time a beam last lay on him: the torch's share is held 1.5 s past it, so a sweeping beam does not pump the gait)
  aimPoint: Vec3 = [0, 0, 0]; aimTarget: LightTarget | null = null; aimGuard: Guard | null = null;
  dynBoxes: Box[] = [];
  doors!: Doors;
  /** loose furniture (chairs, cartons, bins, plants) shoved around by characters, bullets, canisters and door leaves */
  props: PropSystem;
  /** door the player would operate with F right now (also drives the HUD prompt) */
  useDoor: Door | null = null;   // the hovered door when it is within reach (F acts on it)
  touring = false;   // the showcase tour is running (demo.ts start/stop) — a plain signal for the HUD / mission getters; quietUtility is NOT it (a stop can run 'live' with the flag down)
  hover: Interactable | null = null; interactables: Interactable[] = []; nearby: Interactable[] = []; aimSnap: Vec3 | null = null;   // aimSnap: where the auto-aim put the reticle this frame (a light or a man's chest), null = raw cursor aim   // hover = the highlighted nearby interaction (what Space acts on), nearby = the panel's list
  /** Scripted control (showcase tour): head for `goal` instead of WASD, aim at `aim` (world point — the cursor is moved onto its projection so hover /
   *  markers / reticle all behave as if a hand were on the mouse), optional crouch. One-shot actions go in as synthetic key hits / clicks on Input.
   *  target: the man a scripted shot is FOR (cursor picking from a top-down camera is unreliable against someone hugging a partition); throwAt: world
   *  point a scripted throw is solved to (the cursor cannot name a floor point the camera sees a wall in front of). */
  puppet: { goal: Vec3 | null; aim: Vec3 | null; crouch?: boolean; walk?: number; fixtures?: boolean; target?: Guard | null; throwAt?: Vec3 | null } | null = null;
  mPerPx = 0.01;   // world size of one internal pixel at the look-at depth (update(): thin props such as the lasers widen to stay >= ~1 px)
  private tmpV: Vec3 = [0, 0, 0];
  calledIn = false; stoodDown = false;   // 'target down' radioed / 'back to your posts' said this encounter
  /** Alarm escalation (guards.ts updateEscalation): 0 calm · 1 heightened · 2 lockdown — the floor's memory of having been alarmed. Raised when an alarm episode
   *  ends with nobody found, and one step by a body being found; steps down one level at a time once escalationT has passed AND nobody is alert or
   *  searching. alarm: the running episode (somebody went to 'alert' and not everybody has given up the search yet), the latest fix it produced (picks the
   *  chokepoint and who radios), the lockdown pair's time apart. Live play only — the tour never moves it (quietUtility) — and resetGuards() zeroes it. */
  escalation: 0 | 1 | 2 = 0; escalationT = -1;
  alarm = { episode: false, pos: [0, 0, 0] as Vec3, placed: false, apartT: 0, raisedAt: -100 };   // raisedAt: game time of the last raiseEscalation (two raises in a beat say one thing, guards.ts)
  /** Room clearing in a lockdown (guards.ts planClears deals it, squad.ts Clearing runs it): the pair working through the `clearRooms` rooms nearest the last fix —
   *  approach, the RoomClear drill live, the next — with their own senses run for them so anything real breaks it off; null when nobody is clearing. Dies with
   *  the guards (resetGuards) and with the lockdown (stand-down). */
  clearing: Clearing | null = null;
  /** rooms per sweep: 0 = by what raised the lockdown (a body found → the 3 nearest, an alarm that came to nothing → 2; guards.ts planClears), 1‥5 = that many (panel) */
  clearRooms = 0;
  /** room name → game time the pair last called it clear in THIS lockdown (planClears will not send them round the same room again within a minute unless the new
   *  fix is inside it); emptied whenever the lockdown ends (step-down, stand-down, reset) */
  clearedAt = new Map<string, number>();
  blackout = { active: false, until: -1, since: -1, permanent: false, restoredAt: -100 }; breakerFixT = 0;
  beacons: { light: RtLight; phase: number; target: LightTarget }[] = [];
  items!: Throwables;
  fx: { box: Box; ttl: number }[] = [];
  sparks = new Sparks(fx.sparks);   // powder streaks + stun-canister fragments (CPU particles → emissive dynamic boxes)
  fxSeed = 1;               // counting seed for the seeded effects: every shot / can differs, a replayed session repeats
  audio: GameAudioLike = nullAudio;
  ocpCooldown = 0; lastStingT = -100; eventSeq = 0;
  messages: { text: string; t: number }[] = [];
  /** the mission thread (see Mission): reset by restartEncounter(); frozen — neither progressing nor counting — while the showcase tour runs */
  mission: Mission = newMission(0);
  rackTargets: LightTarget[] = [];   // the mission rack's LED / glow switchables: forced dark once the drive is out
  tune = { guardWalk: 1.15, guardInvestigate: 1.75, guardChase: 2.7, playerWalk: 1.55, playerSprint: 4.7, playerCrouch: 1.0, playerLight: 60, flashIntensity: 70, flashInner: 7, flashOuter: 19, detectRate: 1.6, ocpDuration: 9, visSlope: 0.2, visFloor: 0.03 };

  constructor(public engine: Engine, public level: Level, public ch: GltfCharacter, public smoke: SmokeSystemLike) {
    this.col = new StaticCollision(level.boxes);
    this.chars = new CharacterRenderer(engine.device, ch);
    engine.skin = this.chars;
    this.probe = new IrradianceProbe(engine.device, engine.sceneLayout);
    this.player = new Player(ch, level.playerSpawn);
    // loose furniture: simulated on the CPU, emitted as dynamic boxes every frame (rendered + traced like everything else), deliberately never registered as
    // collision occluders — a chair must not hide the player, muffle a shot or dent the nav grid; guards path through props and shove them aside
    this.props = new PropSystem(level.props, this.col, (e) => this.onPropEvent(e));
    this.items = new Throwables(this.col, this.props);   // canisters and dropped magazines bounce off (and nudge) the furniture
    this.player.light = engine.lights.add(railLightDef(PLAYER_ID, this.tune.playerLight));
    // light targets from analytic lights + switchables
    // contactor groups restrike one after another when the mains return: stagger by order of first appearance
    const groupStagger = new Map<string, number>(); const staggerOf = (grp: string) => { if (!groupStagger.has(grp)) groupStagger.set(grp, groupStagger.size * 0.45 + Math.random() * 0.5); return groupStagger.get(grp)!; };
    const lightDefs = new Map(level.lights.map(d => [d.name, d]));
    for (const l of engine.lights.lights) {
      if (l.ttl >= 0 || l.kind === 2 || l.group === 'player' || l.group === 'flashlights') continue;
      const mains = lightDefs.get(l.name)?.mains !== false;
      this.targets.push({ name: l.name, group: l.group, kind: 'analytic', pos: v3.copy(l.pos), light: l, boxes: l.fixtureBox >= 0 ? [l.fixtureBox] : [], baseEmissive: l.fixtureBox >= 0 ? v3.copy(level.boxes[l.fixtureBox].emissive) : [0, 0, 0], baseIntensity: l.intensity, on: l.enabled, broken: false, disabledUntil: -1, fluorescentFlicker: false, factor: 1, mains, stagger: staggerOf(l.group), interactive: true });
    }
    for (const sw of level.switchables) {
      const b = level.boxes[sw.boxes[0]];
      this.targets.push({ name: sw.name, group: sw.group, kind: 'emissive', pos: v3.copy(b.c), boxes: sw.boxes, baseEmissive: v3.copy(sw.baseEmissive), baseIntensity: 0, on: sw.on, broken: false, disabledUntil: -1, fluorescentFlicker: !!sw.flicker, factor: sw.on ? 1 : 0, mains: sw.mains !== false, stagger: staggerOf(sw.group), interactive: sw.group !== 'server_leds' && sw.group !== 'server_glow' });
    }
    // flat emissive fixtures (ceiling panels, monitors, logo, TV, vending fronts, exit signs) light the scene through analytic one-sided area
    // lights; their boxes keep glowing on screen but rays ignore the emissive term (BoxFlag.AreaLit) — kills small-emitter aliasing in the cascades
    for (const t of this.targets) {
      if (t.kind !== 'emissive' || t.group === 'server_leds' || t.group === 'emergency') continue;
      for (const bi of t.boxes) {
        const b = level.boxes[bi]; const h = b.h;
        const thin = h[0] <= h[1] && h[0] <= h[2] ? 0 : h[1] <= h[2] ? 1 : 2; if (h[thin] > 0.06) continue;
        const a0 = (thin + 1) % 3, a1 = (thin + 2) % 3; const area = 4 * h[a0] * h[a1]; if (area < 0.01) continue;
        // emitting side: ceiling fixtures face down; wall/desk fixtures face whichever side is open (props are authored with the glowing face on
        // local +X/+Z of their backing, which is also the fallback when both sides look open)
        let n: Vec3 = [0, 0, 0];
        if (thin === 1) n = [0, b.c[1] > 1.5 ? -1 : 1, 0];
        else { const cs = Math.cos(b.yaw), sn = Math.sin(b.yaw); const ax: Vec3 = thin === 0 ? [cs, 0, -sn] : [sn, 0, cs];   // box local X / Z in world
          const front = !this.col.segmentBlocked(b.c, v3.mad(b.c, ax, h[thin] + 0.35)), back = !this.col.segmentBlocked(b.c, v3.mad(b.c, ax, -(h[thin] + 0.35)));
          n = front && !back ? ax : !front && back ? v3.scale(ax, -1) : ax; }
        const e = t.baseEmissive; const maxc = Math.max(e[0], e[1], e[2], 1e-3); const I0 = maxc * area;   // radiance × area = on-axis intensity
        const light = engine.lights.add({ name: `${t.name}_area${bi}`, group: t.group, kind: 3, pos: v3.mad(b.c, n, h[thin] + 0.03), dir: n, color: [e[0] / maxc, e[1] / maxc, e[2] / maxc], intensity: I0,
          range: clamp(Math.sqrt(I0 / 0.0015), 3, 26), innerDeg: 0, outerDeg: 90,   /* range = where the panel has fallen to ~nothing (0.0015 W/m²): the old 8 m cap cut a visible circle into dark neighbouring rooms; the scoring loop is analytic so the extra reach costs no rays */ radius: Math.max(0.1, Math.sqrt(area / Math.PI)), volumetric: 0.0, /* diffuse panels: no haze beams (cost) */ owner: 0, enabled: t.on, fixtureBox: -1, flicker: 0, ttl: -1, age: 0, peakIntensity: I0, broad: true });
        (t.areaLights ??= []).push({ light, base: I0 });
        b.flags |= BoxFlag.AreaLit;
      }
    }
    for (const b of level.beacons) level.boxes[b.box].flags |= BoxFlag.AreaLit;   // dome glow is screen-only; the beam is the spot light
    // mains breaker (OCP → temporary blackout, bullet → permanent) and the emergency beacons it brings up (each beacon light is slaved to its dome fixture)
    this.targets.push({ name: 'breaker', group: 'mains', kind: 'breaker', pos: v3.copy(level.breaker.pos), boxes: [level.breaker.boxes[1]], baseEmissive: [0.3, 4, 0.6], baseIntensity: 0, on: true, broken: false, disabledUntil: -1, fluorescentFlicker: false, factor: 1, mains: false, stagger: 0, interactive: true });
    level.beacons.forEach((b, i) => {
      const target = this.targets.find(t => t.kind === 'emissive' && t.boxes.includes(b.box));
      if (!target) throw new Error(`beacon ${i}: no switchable owns box ${b.box}`);
      const light = engine.lights.add({ name: target.name, group: target.group, kind: 1, pos: v3.copy(b.pos), dir: [1, 0, 0], color: [1.0, 0.22, 0.06], intensity: 55, range: 14, innerDeg: 9, outerDeg: 24, radius: 0.06, volumetric: 1.0, owner: 0, enabled: false, fixtureBox: -1, flicker: 0, ttl: -1, age: 0, peakIntensity: 55 });
      this.beacons.push({ light, phase: i * 1.7, target });
    });
    // doors: leaves are dynamic occluders for sight / sound / projectiles (collision layer)
    this.doors = new Doors(level.doors.map(d => ({ ...d, albedo: d.exterior ? [0.2, 0.22, 0.25] as Vec3 : undefined })), (e) => this.onDoorEvent(e));
    this.col.dynamicBoxes = this.doors.leafBoxes();
    // guards
    const tints: Vec3[] = [[0.42, 0.36, 0.25], [0.3, 0.34, 0.4], [0.38, 0.28, 0.28]];
    level.routes.forEach((r, i) => {
      const fl = engine.lights.add(torchLightDef(`flashlight${i}`, 10 + i, this.tune.flashIntensity, this.tune.flashInner, this.tune.flashOuter));
      this.guards.push(new Guard(ch, 10 + i, r, i, fl, tints[i % tints.length]));
    });
    { const n = level.mission.rack.index; this.rackTargets = this.targets.filter(t => t.name === `rack${n}_leds` || t.name === `rack${n}_glow`); }   // what goes dark when the drive comes out (level.ts "server room")
    this.initialOn = new Map(); for (const t of this.targets) this.initialOn.set(t, t.on);   // the level as authored, for resetLevelState() (captured here, before a tour or the panel touches a switch)
  }

  /** the showcase tour flips this so its own plumbing (teleports, resets) stays out of the log while guard barks still show */
  quietUtility = false;
  /** tour seam: a beat that stages the mission itself (the drive pull) sets this so the stage machine, the rack marker and the objective line run under
   *  the puppet — the debrief tallies still stay off (missionLive), so nothing the tour does counts as the player's run */
  missionInTour = false;
  msg(text: string) { this.messages.push({ text, t: this.time }); if (this.messages.length > 5) this.messages.shift(); }
  say(g: Guard, text: string, radio = false) { return say(this, g, text, radio); }
  viewProj: Float32Array | number[] | null = null;
  note(text: string) { if (!this.quietUtility) this.msg(text); }   // utility / sandbox confirmations

  private onDoorEvent(e: DoorEvent) {
    if (e.who === PLAYER_ID) this.player.noise = Math.max(this.player.noise, e.level);
    switch (e.sound) {
      case 'creak': this.audio.play('doorCreak', e.pos, 0.5 + 0.5 * e.level / 0.3); break;
      case 'bang': this.audio.play('doorBang', e.pos, 0.9); break;
      case 'latch': this.audio.play('doorLatch', e.pos, 0.6); break;
      case 'bash': this.audio.play('doorBash', e.pos, 0.9); break;
      case 'rattle': this.audio.play('doorRattle', e.pos, 0.35 + 2 * e.level); break;
      case 'pick': this.audio.play('lockPick', e.pos, 0.5, { rate: 0.9 + Math.random() * 0.25 }); break;
      case 'unlock': this.audio.play('lockOpen', e.pos, 0.8); break;
      case 'kick': this.audio.play('doorBash', e.pos, 1.0, { rate: 0.9 }); this.audio.play('lockBreak', e.pos, 0.95); break;   // the slam of the leaf, lower and harder than a shoulder, under the keep splintering out of the jamb
    }
    // heard (guards.ts hearingCheck): a kick is its own kind — floor-wide, answered like a shot; everything else is a 'door' noise with a level, placed at the leaf
    this.events.push({ kind: e.sound === 'kick' ? 'kick' : 'door', pos: [e.pos[0], 0, e.pos[2]], time: this.time, loud: e.sound === 'bash' || e.sound === 'kick' || (e.sound === 'bang' && e.level > 0.6), level: e.level, who: e.who });
  }
  /** Furniture noise: a scrape / caster rattle when something is set going, a hollow knock when it hits a wall, another prop or takes a round.
   *  Heard like door sounds (level 0..1, blamed on whoever moved it — guards ignore each other's clumsiness). */
  private onPropEvent(e: PropEvent) {
    if (e.who === PLAYER_ID) this.player.noise = Math.max(this.player.noise, e.level);
    const t = e.prop.tune; const jitter = 0.95 + Math.random() * 0.1;
    if (e.sound === 'thud') this.audio.play('propThud', e.pos, clamp(0.35 + 0.15 * e.speed, 0.3, 1), { rate: t.thudRate * jitter });
    else this.audio.play('propScrape', e.pos, clamp(0.25 + 0.18 * e.speed, 0.25, 1), { rate: t.scrapeRate * jitter });
    this.events.push({ kind: 'prop', pos: v3.copy(e.pos), time: this.time, loud: e.level > 0.7, level: e.level, who: e.who });
  }


  resetGuards() { return resetGuards(this); }
  tension() : number { return tension(this); }
  /** one line on the alarm escalation for the panel / debugging (level, clock, episode, lockdown pair + post) */
  escalationSummary(): string { return escalationSummary(this); }
  /** panel / debug: one level up by hand (as if an alarm had just found nobody), or straight back to calm */
  escalate() { raiseEscalation(this, this.escalation >= 1 ? 2 : 1); }
  standDown() { standDownEscalation(this); }
  /** panel: 'clearing: <room> · <phase> …' while the lockdown pair clears, else '' ; and the test button — lock down and clear the room nearest the player now */
  clearingSummary(): string { return clearingSummary(this); }
  /** panel / console: one line per searching man — his plan's phase, the spot he is on, how many looked into, the best few left with their light readings
   *  (guards.ts searchSummary); and who is pieing which doorway. '' when nobody is. */
  searchSummary(): string {
    const parts: string[] = [];
    for (const gd of this.guards) { if (gd.state === 'dead') continue; const n = this.level.routes[gd.routeI]?.name ?? gd.callsign; const s = searchSummary(this, gd); if (s) parts.push(`${n}: ${s}`); if (gd.pieing) parts.push(`${n} pies the ${gd.pieing.door.def.name} door (${gd.pieing.phase})`); }
    return parts.join(' | ');
  }
  /** panel / console: one line per man in the human shield's standoff round the man Sam holds — distance, degrees off the pair's rear, the held man's clearance off
   *  his line, which way round he works, what his feet are doing, rounds fired (guards.ts standoffSummary); '' when nobody is */
  standoffSummary(): string { return standoffSummary(this); }
  /** panel / console: one line per man — callsign, temperament, dead / out cold if he is, and his witness record (guards.ts witnessSummary: what he has personally
   *  heard and seen this encounter). Nothing in the game reads any of it yet; it is there to be looked at while the dialogue that will is written. */
  witnessSummary(): string {
    return this.guards.map(gd => `${gd.callsign} (${TEMPERAMENT[gd.callsign] ?? '?'}${gd.state === 'dead' ? (isBreathing(gd) ? ', out cold' : ', dead') : ''}): ${witnessSummary(this, gd) || '—'}`).join(' | ');
  }
  /** One exchange of the interrogation with `gd` (dialogue.ts — docs/internal/grab-interrogate-design.md §4, slice 5: the engine without the grab). The man answers
   *  from his own state: what he personally saw and heard (Witness), how rattled he is, his temperament, and how far Sam has already leaned on him (Talk) — the line
   *  goes up as his bubble (or the log) with the interrogation's longer hold, and the pick comes back (text, stage, cell, and the useful thing's kind if he gave one)
   *  for whoever asked: the held man's row on a Space tap (player.ts updateHold), the sandbox button, a mission hook later. The gate: the man in Sam's arm
   *  (player.holding) always; anybody else only for the sandbox — 'a living man within reach of a living Sam', INTERROGATE_REACH, 2 m, nothing solid between —
   *  and null otherwise. `rnd` picks within the pool (seeded by the QA harness for replayable runs; the game's own dice by default). */
  interrogate(gd: Guard, rnd: () => number = Math.random): DialoguePick | null {
    if (this.player.down || gd.state === 'dead' || !this.guards.includes(gd)) return null;
    if (this.player.holding?.g !== gd) {   // not the man in his arm: the sandbox's reach gate
      const a = this.player.char.pos, b = gd.char.pos;
      if (v3.distXZ(a, b) > dialogue.INTERROGATE_REACH || this.col.segmentBlocked([a[0], 1.0, a[2]], [b[0], 1.0, b[2]])) return null;   // in reach and nothing solid between, statics or a leaf (the takedown gate's own test): no leaning on a man through a partition
    }
    const pick = dialogue.interrogate(this, gd, rnd); if (!pick) return null;
    say(this, gd, pick.text, false, true);   // (forced: a held man's only voice is his answers — guards.ts say drops everything else while he is held)
    if (gd.bubble) gd.bubble.dur = clamp(1.2 + 0.07 * pick.text.length, 2.5, 6.5);   // held speech runs longer than a bark's 4 s cap
    return pick;
  }
  /** Let go of the man in Sam's arm at once, no shove (player.ts freeHeld): Sam going down with him, a teleport, the tour taking over. He comes off it knowing everything. */
  freeHeld(why: 'shot' | 'cancel') { return freeHeld(this, why); }
  /** the living man nearest Sam, within `reach` metres (Infinity = anywhere) — the sandbox button's target, and the panel line's */
  nearestLivingGuard(reach = Infinity): Guard | null {
    let best: Guard | null = null, bd = reach;
    for (const gd of this.guards) { if (gd.state === 'dead') continue; const d = v3.distXZ(gd.char.pos, this.player.char.pos); if (d <= bd) { bd = d; best = gd; } }
    return best;
  }
  /** sandbox: lean on whoever is within reach (a note if nobody is) */
  interrogateNearest(rnd: () => number = Math.random): DialoguePick | null {
    if (this.player.down) { this.note("you're down — nobody answers to a man on the floor"); return null; }
    const gd = this.nearestLivingGuard(dialogue.INTERROGATE_REACH);
    if (!gd) { this.note(`nobody within ${dialogue.INTERROGATE_REACH} m to lean on — walk up to a living guard`); return null; }
    const pick = this.interrogate(gd, rnd);
    if (!pick) this.note(`${gd.callsign} is in reach but there is something solid between you`);
    return pick;
  }
  /** panel / console: the interrogation engine's read of the man nearest Sam (any distance) — his knowledge / agitation / temperament cell with the agitation terms,
   *  the stage the next press would be, what he has said and given, the useful thing he would give next (dialogue.ts dialogueSummary) */
  dialogueSummary(): string {
    const gd = this.nearestLivingGuard(); if (!gd) return 'dialogue: nobody left standing';
    const d = v3.distXZ(gd.char.pos, this.player.char.pos);
    return `dialogue ▸ ${dialogue.dialogueSummary(this, gd)} · ${d <= dialogue.INTERROGATE_REACH ? 'in reach' : `${d.toFixed(1)} m off`}`;
  }
  /** panel: the rooms still queued for the pair and the ones called clear in this lockdown lately (with how long ago) */
  clearQueueSummary(): string { return clearQueueSummary(this); }
  clearNearestRoom() { forceClear(this); }
  /** Camera follow position (player or spectated guard). */
  followPos(): Vec3 { const g = this.spectate >= 0 ? this.guards[this.spectate] : null; return g ? g.char.pos : this.player.char.pos; }
  teleportPlayer(p: Vec3) { return teleportPlayer(this, p); }

  /** The thud of a ragdoll's trunk / head meeting the floor (or a wall, a desk): polled after the body's update, rate-limited inside takeImpact. */
  bodyThud(c: Character) {
    const hit = c.ragdoll ? c.ragdoll.takeImpact() : 0;
    if (hit > 1.4) this.audio.play('bodyFall', c.bones.spine2 ?? c.pos, clamp(0.35 + hit * 0.13, 0.4, 1));
  }
  repairLights() { return repairLights(this); }
  /** Everything still in the air / in the sound queue from what just happened: fused canisters, sparks, the last half second of noises, a wind-up.
   *  Fresh guards must not inherit it (they would hear shots nobody alive fired). */
  clearAftermath() { this.items.items.length = 0; this.sparks.clear(); this.events.length = 0; if (this.player) { this.player.pendingThrow = null; this.player.throwHeld = false; this.player.throwPreview = null; } }

  /** Level as authored: every switch / group back to its initial on-off, every door back to its authored angle and lock (picked / kicked locks re-lock). */
  private initialOn: Map<LightTarget, boolean> | null = null;
  resetLevelState() {
    if (this.initialOn) for (const [t, on] of this.initialOn) t.on = on;
    for (const d of this.doors.list) d.reset();
  }

  /** Restart the encounter: a fresh Player at the insertion point with full kit, fresh Guards on their routes, lights repaired, the level as authored. */
  restartEncounter() {
    const light = this.player.light;
    this.player = new Player(this.ch, v3.copy(this.level.playerSpawn)); this.player.light = light; this.player.char.update(0);   // pose + bones exist before this frame's capsule upload
    this.resetGuards(); this.repairLights(); this.resetLevelState(); this.props.reset(); this.events.length = 0; this.useDoor = null; this.calledIn = false; this.stoodDown = false;   // (resetGuards also stands the floor down: escalation 0, no episode, no lockdown duties)
    // anything still in the air or still venting belongs to the encounter that just ended: an armed stun canister landing into the new one would
    // dazzle the fresh guards and alert the floor from a throw the restarted player never made, and a deployed can's 30 s screen would go on
    // degrading their line of sight (its emitter tracks the item and is priority-pinned, so dropping the item alone does not stop it)
    this.clearAftermath(); this.smoke.clearAll?.();
    this.hover = null; this.interactables = [];   // (like useDoor above) this frame's cursor state still names the guards / bodies / dropped pistols of the encounter that just ended: when the
                                                  // restart came from updatePlayer (Shift+Enter), the F handlers later in the same update() must not take down, drag or loot any of them
    this.mission = newMission(this.time);   // a fresh run: stage, clock and stats from zero (resetLevelState above has already relit the rack)
    this.note('encounter restarted');
  }

  setGroup(group: string, on: boolean) { return setGroup(this, group, on); }
  toggleTarget(t: LightTarget) { return toggleTarget(this, t); }
  setBlackout(seconds: number, cause: 'ocp' | 'shot' | 'debug') { return setBlackout(this, seconds, cause); }
  endBlackout() { return endBlackout(this); }
  lightEvent(t: LightTarget) { return lightEvent(this, t); }
  ocp(t: LightTarget | null) { return ocp(this, t); }
  toggleDrag(which?: Guard) { return toggleDrag(this, which); }

  // ------------------------------------------------------------ update
  update(dt: number, input: Input, cam: FollowCamera, canvas: HTMLCanvasElement) {
    this.viewProj = cam.viewProj;
    this.time += dt; this.ocpCooldown = Math.max(0, this.ocpCooldown - dt); this.col.sound.tick(this.time);
    updateAim(this, input, cam, canvas);
    { // threat proximity → how deliberately Sam moves (character.ts `deliberate`, eased there; presentation only — computed under the tour too, it only changes the look)
      const pl = this.player, pc = pl.char; let near: Guard | null = null, nd = Infinity, t = 0;
      for (const gd of this.guards) { if (gd.state === 'dead' || gd.held) continue; const d = v3.distXZ(gd.char.pos, pc.pos); if (d < nd) { nd = d; near = gd; } }   // (the man in his arm is not one to creep about)
      if (near && !pl.down && near.state !== 'alert') {   // the nearest living man: nothing to creep about once he is alert (he has him)
        t = 1 - smoothstep(3, 7, nd);
        if (near.state === 'suspicious' || near.state === 'search') t = Math.max(t, 0.6 * (1 - smoothstep(7, 10, nd)));   // extra care while he is looking into something
        const spots: Vec3[] = [[pc.pos[0], 0.15, pc.pos[2]], pc.bones.chest ?? v3.add(pc.pos, [0, 1.1, 0])];   // the floor where he is (a patrol torch hangs pitched down: its pool is on the floor) and his chest (a raised one)
        torch: for (const gd of this.guards) {   // a lit torch / weapon-light cone on him (inside its cone and 10 m, nothing solid between): half or more, held a beat past the beam moving on
          if (this.time - this.torchOnT < 1.5) break; const fl = gd.flashlight; if (gd.state === 'dead' || !fl.enabled) continue; const cosO = Math.cos(fl.outerDeg * DEG);
          for (const s of spots) { const to = v3.sub(s, fl.pos); const d = v3.len(to); if (d < 10 && d > 1e-3 && v3.dot(to, fl.dir) / d > cosO && !this.col.segmentBlocked(fl.pos, s)) { this.torchOnT = this.time; break torch; } }
        }
        if (this.time - this.torchOnT < 1.5) t = Math.max(t, 0.5);
      }
      this.threatProximity = t; pc.anim.deliberate = t;
    }
    updatePlayer(this, dt, input, cam);
    this.searchProbeN = 0;   // (this frame's budget of searchers' light queries starts again: guards.ts searchProbes)
    for (const g of this.guards) updateGuard(this, g, dt);
    updateEscalation(this, dt);   // after the guards: this frame's alert states settle the episode, the level and the lockdown duties they read next frame
    this.updateDoors(dt, input);
    updateMission(this, dt, input);   // after the guards (this frame's sightings) and the other F handlers; before updateTargets bakes the rack's switch state
    this.updateProps(dt);
    this.events = this.events.filter(e => this.time - e.time < 0.5);
    updateTargets(this, dt);
    updateItems(this, dt);
    // characters → GPU skin instances + RT proxies
    const all: Character[] = [this.player.char, ...this.guards.map(g => g.char)];
    this.dynBoxes.length = 0;
    this.doors.boxes(this.dynBoxes);
    this.props.boxes(this.dynBoxes);
    this.items.boxes(this.dynBoxes);
    this.mPerPx = 2 * cam.distance * Math.tan(cam.fov / 2) / Math.max(200, canvas.height * (this.engine.effectiveScale || 1));   // world size of one internal pixel at the look-at depth (thin props widen to stay ≥ ~1 px)
    handProps(this, this.dynBoxes);
    for (let i = this.fx.length - 1; i >= 0; i--) { const f = this.fx[i]; this.dynBoxes.push(f.box); f.ttl -= dt; if (f.ttl <= 0) this.fx.splice(i, 1); }
    this.sparks.update(dt, this.col); this.sparks.boxes(this.dynBoxes);   // before smoke.update() this frame reads the trail emitters' tracked positions
    // characters wading through smoke push it around
    for (const c of all) {
      const sp = Math.hypot(c.vel[0], c.vel[2]); if (sp < 0.25 || !c.bones.hips) continue;
      const dirv: Vec3 = [c.vel[0] / sp, 0.05, c.vel[2] / sp];
      this.smoke.push({ pos: c.bones.hips, dir: dirv, speed: sp * 1.15, radius: 0.3 });
      if (c.bones.chest) this.smoke.push({ pos: c.bones.chest, dir: dirv, speed: sp * 1.1, radius: 0.26 });
    }
    const world = this.engine.world; world.resetCapsules();
    all.forEach((c, i) => {
      this.chars.setInstance(i, c.skel.jointData, c.tint, c.tint2, c.id);
      const { caps, bound } = c.capsules(); world.addCharacterCapsules(c.id, v3.lerp(c.tint, [0.5, 0.5, 0.5], 0.3), caps, bound);
    });
    this.chars.finish(all.length);
    world.dynamics = this.dynBoxes;
    // light meter queries
    const pc = this.player.char;
    this.probe.query('playerChest', pc.bones.chest ?? v3.add(pc.pos, [0, 1.2, 0]), PLAYER_ID);
    this.probe.query('playerHead', pc.bones.head ?? v3.add(pc.pos, [0, 1.6, 0]), PLAYER_ID);
    this.guards.forEach((g, i) => {
      if (g.state === 'dead') return;
      const eye = g.char.bones.head ?? v3.add(g.char.pos, [0, 1.65, 0]);
      this.probe.querySegment(`smoke${i}`, eye, pc.bones.chest ?? v3.add(pc.pos, [0, 1.1, 0]));
      const tr = this.probe.get(`smoke${i}`); g.smokeTrans = tr ? tr[0] : 1;
    });
    const r1 = this.probe.get('playerChest'), r2 = this.probe.get('playerHead');
    if (r1 && r2) {
      const E = Math.max(r1[0], r2[0]); this.player.irradiance = E;
      this.player.visibility = damp(this.player.visibility, this.visibilityOf(E), 6, dt);
    }
    this.player.hitFlash = Math.max(0, this.player.hitFlash - dt * 3);
  }
  /** The light meter's mapping, irradiance (W/m², max over the axes as the probe reports it) → 0‥1 visibility: 0 at or below visFloor, 1 by visFloor·e^(1/visSlope).
   *  One mapping for the meter on Sam and for a searcher judging how dark a hiding place is (guards.ts): dark for the one is dark for the other. */
  visibilityOf(E: number): number { return clamp(this.tune.visSlope * Math.log(Math.max(E, 1e-5) / this.tune.visFloor), 0, 1); }


  private updateDoors(dt: number, input: Input) {
    const pl = this.player; const pc = pl.char;
    const own: DoorContact | null = pl.down ? null : { pos: pc.pos, radius: 0.34, bash: pl.sprinting, who: PLAYER_ID, quiet: pl.crouch };   // no key: a locked leaf is a wall to you (Doors.update reports it in blockedBy)
    const contacts: DoorContact[] = own ? [own] : [];
    for (const g of this.guards) if (g.state !== 'dead') contacts.push({ pos: g.char.pos, radius: 0.34, bash: g.state === 'alert' && g.speed > 2.2, who: g.char.id, quiet: false, key: true });   // staff carry keys: locked doors open to them and lock again behind them (closer + latch)
    // bodies: a few of each ragdoll's particles as 'weak' contacts — a corpse in the doorway keeps the closer from latching the leaf, a leaf somebody pushes (or
    // a latched / kicked one) shoves the corpse instead (written back into the particles below, which wakes it)
    const bodyContacts: { c: DoorContact; ch: Character; i: number }[] = []; const bodies: Character[] = [];
    for (let gi = -1; gi < this.guards.length; gi++) {
      const ch = gi < 0 ? pc : this.guards[gi].char; const rag = ch.ragdoll; if (!rag) continue;
      const hp = rag.point(RAG.pelvis, this.tmpV); if (!this.doors.list.some(d => Math.abs(d.pos[0] - hp[0]) < 2.2 && Math.abs(d.pos[2] - hp[2]) < 2.2)) continue;   // only a body actually near a doorway takes part (no per-frame contact churn for the rest)
      bodies.push(ch);
      for (const i of BODY_DOOR_PARTICLES) { const p = rag.point(i); const c: DoorContact = { pos: p, radius: rag.radius(i), bash: false, who: ch.id, quiet: true, weak: true }; contacts.push(c); bodyContacts.push({ c, ch, i }); }
    }
    const angles = bodies.length ? this.doors.list.map(d => d.angle) : null;
    if (!this.puppet && this.nearby.length > 1) { if (input.hit('ArrowDown')) pl.nearSel++; if (input.hit('ArrowUp')) pl.nearSel--; }   // ↑ / ↓: cycle the highlighted interaction (buildInteractables wraps it and re-picks hover next frame; the tour's arrows step its stops instead)
    // Space on a body marker: take hold / let go (the marker of the body you are hauling stays live, see buildInteractables)
    if (input.hit(INTERACT_KEY) && this.hover?.kind === 'body' && this.hover.inReach) { if (pl.dragging) this.toggleDrag(); else this.toggleDrag(this.hover.guard!); }
    if (input.hit(INTERACT_KEY) && this.hover?.kind === 'pistol' && this.hover.inReach && this.hover.item) {   // strip the magazine out of a dropped pistol
      const it = this.hover.item; pl.pistol.spare.push(it.rounds); this.msg(`took a magazine · ${it.rounds} rounds`); it.rounds = 0;
      this.audio.play('magOut', it.pos, 0.7); this.audio.play('magIn', pc.pos, 0.5);
    }
    // A living guard's marker from behind: TAP Space = the silent takedown (as ever — and, Chaos Theory, a LEFT CLICK with the hands empty (holstered) is the same
    // melee, whatever the panel highlights), HOLD Space for GRAB_HOLD_SECS = grab him: an arm round his throat and he is yours (player.ts startGrab / updateHold —
    // questions, the choke, letting him go all live on HIS row from then on). Read like a door's tap / hold: the takedown goes on the release, the grab the moment
    // the hold is long enough (that press is then spent: fHeld); a press that began and ended inside one frame — the tour's synthetic Space — is a tap.
    if (!input.down(INTERACT_KEY)) pl.fHeld = false;
    { const onGuard = this.hover?.kind === 'guard' && this.hover.inReach ? this.hover.guard! : null;                                          // Space: the highlighted man
      const viaClick = pl.holstered && input.lmbHit() ? (this.nearby.find(it => it.kind === 'guard' && it.inReach)?.guard ?? null) : null;   // click, hands empty: the man in reach
      const free = !pl.takedown && !pl.down && !pl.holding;
      if (viaClick && free) { startTakedown(this, viaClick); pl.guardHold = 0; }
      else if (onGuard && free && !pl.fHeld) {
        if (input.down(INTERACT_KEY)) { pl.guardHold += Math.min(dt, 0.05); if (pl.guardHold >= GRAB_HOLD_SECS) { pl.guardHold = 0; startGrab(this, onGuard); } }   // (a frame hitch must not turn a tap into a grab)
        else { if (pl.guardHold > 0 || input.hit(INTERACT_KEY)) startTakedown(this, onGuard); pl.guardHold = 0; }
      } else pl.guardHold = 0;   // (walked out of reach or he turned mid-press: nothing, and the count starts again)
    }
    // Space on a door marker within reach: tap = push open / pull shut; hold on a shut or barely-open door = crack it silently, a few degrees a second.
    // Locked against you (Doors.lockedOut): hold = pick the lock (player.ts updatePicking takes over while F stays down), tap standing = kick it in, tap
    // crouched = try the handle (a clack and 'won't budge'). A press spent on a finished pick or a kick (fHeld) has to come up before the door listens again.
    // With a man in his arm no door listens at all: the pair pushes leaves open by walking into them, and HIS key turns the locked ones (the contacts below).
    const door = this.useDoor;
    const busy = pl.picking || pl.kick || pl.takedown || pl.fHeld || pl.holding;
    if (input.down(INTERACT_KEY) && door && !busy) {
      pl.doorHold += Math.min(dt, 0.05);                       // a frame hitch must not turn a tap into a hold
      if (pl.doorHold > 0.22) {
        if (this.doors.lockedOut(door, pc.pos)) { if (!(input.down('KeyW') || input.down('KeyA') || input.down('KeyS') || input.down('KeyD'))) { startPicking(this, door); pl.doorHold = 0; } }   // (arriving with W held and F down: wait for the feet to stop rather than start-and-cancel)
        else if (Math.abs(door.angle) < 0.62) { this.doors.crack(door, pc.pos, PLAYER_ID, Math.min(dt, 0.05)); pl.doorCracking = true; }
      }
    } else {
      // released (or tapped and released within this frame): anything that was not a silent crack is a normal push / pull —
      // including a long press on an open door, which the marker promises will pull it shut
      if ((pl.doorHold > 0 || input.hit(INTERACT_KEY)) && door && !pl.doorCracking && !busy) {
        if (!this.doors.lockedOut(door, pc.pos)) this.doors.use(door, pc.pos, PLAYER_ID);
        else if (pl.crouch || pl.dragging) { this.doors.rattle(door, PLAYER_ID); this.msg('locked — hold Space to pick it, or stand up and kick it in (loud)'); }
        else startKick(this, door);
      }
      pl.doorHold = 0; pl.doorCracking = false;
    }
    const bx = pc.pos[0], bz = pc.pos[2];
    this.doors.update(dt, contacts);
    if (own) { settlePlayer(this); settleHeld(this); if (Math.abs(pc.pos[0] - bx) + Math.abs(pc.pos[2] - bz) > 0.005) pc.update(0); }   // a leaf shoved him (or the man in his arm: the pair gives as one, settleHeld): settle that against the statics now (never drawn inside the wall a held leaf pressed him onto), and re-bake the pose where he ended up
    // ran full tilt into a leaf that is locked against you: that is a kick (square on — brushing past the latch edge at a run is not)
    if (own?.blockedBy && pl.sprinting && !pl.kick && !pl.picking && !pl.takedown && !pl.holding && !this.puppet) {
      const d = own.blockedBy; const u = d.dir(); const into = -(-u[1] * pc.vel[0] + u[0] * pc.vel[2]) * this.doors.side(d, pc.pos);   // speed INTO the leaf from your side (its normal on your side is (−u.z, u.x)·side)
      if (into > 1.5) startKick(this, d);
    }
    for (const { c, ch, i } of bodyContacts) { const p = ch.ragdoll!.x; ch.ragdoll!.nudge(i, c.pos[0] - p[i * 3], c.pos[2] - p[i * 3 + 2]); }   // whatever a leaf pushed
    // a leaf that moved wakes the bodies asleep within its reach: one slumped against it must slide down when it swings away (a sleeping ragdoll never looks)
    if (angles) this.doors.list.forEach((d, k) => {
      if (Math.abs(d.angle - angles[k]) < 1e-3) return;
      for (const ch of bodies) { const h = ch.bones.hips ?? ch.pos; if (ch.ragdoll!.sleeping && Math.hypot(h[0] - d.def.hinge[0], h[2] - d.def.hinge[1]) < d.def.width + 1.2) ch.ragdoll!.wake(); }
    });
  }

  /** Loose furniture: same contact pattern as the doors (after everyone has moved and been collided with the statics this frame): characters are
   *  circles that shove props and are held back in place by whatever a prop cannot yield; door leaves sweep props aside. */
  private updateProps(dt: number) {
    const pl = this.player;
    const chars: Character[] = pl.down ? [] : [pl.char];   // a body on the floor neither pushes furniture nor gets pushed
    for (const g of this.guards) if (g.state !== 'dead') chars.push(g.char);
    const contacts: PropContact[] = chars.map(c => ({ pos: c.pos, vel: c.vel, radius: 0.3, who: c.id, mass: 75, quiet: c.id === PLAYER_ID && pl.crouch }));
    const before = chars.map(c => [c.pos[0], c.pos[2]]);
    this.props.update(dt, contacts, this.col.dynamicBoxes);
    if (!pl.down) { settlePlayer(this); settleHeld(this); }   // a chair driven into him by a man or a leaf held him back INTO the wall behind him: the statics win, this frame — and one that would not yield to the man in his arm holds the pair back as one (settleHeld), not him off his station
    // someone held back by a pinned prop was posed (bones, skin, capsules) at the uncorrected spot inside it earlier this frame: re-bake in place (dt 0)
    chars.forEach((c, i) => { if (Math.abs(c.pos[0] - before[i][0]) + Math.abs(c.pos[2] - before[i][1]) > 0.005) c.update(0); });
  }

  dropThrowable(kind: ThrowKind, p: Vec3) { return dropThrowable(this, kind, p); }
  throwCount(kind: ThrowKind) { return throwCount(this, kind); }











  objectivePos() : Vec3 | null { return objectivePos(this); }
  objectiveText() : string { return objectiveText(this); }

}

