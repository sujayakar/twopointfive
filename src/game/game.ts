// Game layer core: the Game state holder and its per-frame orchestration, the Player / Guard / LightTarget types, door + prop contact plumbing.
// The systems live beside it as plain-function modules taking the Game first: player.ts (controller, takedown / drag, throws, the cursor's
// interactables), guards.ts (perception, state machine, barks, escalation), combat.ts (shots, kills, hand props, live throwables), lighting.ts
// (fixtures, OCP, blackout), mission.ts (the mission thread), squad.ts (scripted guards).
import { lookConfig } from '../render/frame';
import { HostOwned } from '../math/side';
import { Vec3, v3, clamp, lerp } from '../math/vec';
import type { HostApi } from '../host/api';
import type { View } from '../host/hud';   // the view as the game reads it: clientWidth / clientHeight / width / height — the page's canvas, a plain object headless and natively   // the engine as the game sees it: a type, so this layer no longer imports src/engine.ts
import type { RingState } from './cmdring';
import { Level, PatrolRoute, Chokepoint } from '../scene/level';
import { StaticCollision } from '../scene/collision';
import { GltfCharacter } from '../anim/gltf';
import { CharacterRenderer } from '../anim/characters';
import { Character } from './character';
import type { Stance, HoldPhase, HoldVariant, KitItem } from './character';
import { Input } from './input';
import { FollowCamera } from './camera';
import { Seat, sampleCmd } from './cmd';   // the player's input as one plain record a tick (cmd.ts)
import type { PlayerCmd } from './cmd';
import { RtLight } from '../render/lights';
import { IrradianceProbe } from '../render/probe';
import { MeshShadows } from '../render/meshShadows';
import { fitRig, fitRigCast } from '../render/sdfrig';
import { castParts } from './cast';
import { statureOf, drawnJoints, drawnCapsules, drawnFit, drawnBoxes } from './stature';
import { Decals } from '../render/decals';
import { buildWearMap } from '../render/wear';
import { buildSnowField, snowCfgRows, type SnowField } from '../render/snowField';
import { makeSnowTracks, updateSnowTracks, clearSnowTracks, formSnowTracks, TRACK_RES, type SnowTracks } from './snowTracks';
import { updateSnowMarks, clearSnowMarks } from './snowMarks';
import { snowBogAt, snowBogMay, snowBogKinds } from './snowBog';
import { attachSnowBlast, clearSnowBlast, stepSnowDigs } from './snowBlast';
import { SNOW_LOOK, snowLevel, WEATHER_PRESETS } from './weatherKinds';
import { levelNameOf } from '../scene/levelLoad';
import { SmokeSystemLike } from '../smoke/types';
import { Box, BoxFlag } from '../scene/boxes';
import { markStats, markWorldRect, AREA_LIT_MARKS, MARKING_TEXEL } from '../scene/markings';
/** an emissive marking registers a light only from this on-axis intensity up (W/sr): the vending fronts are ≈ 2.2–2.8, a monitor's UI bars 0.02, an LED row 0.01 */
export const MARK_LIGHT_MIN_I0 = 0.05;
import { Pistol, Rifle, Ocp, Firearm, WEAPONS } from './weapons';
import { AimWeight } from './aimweight';
import type { Ring } from './launcher';
import type { Shell } from './he40';
import './he40';
import { glassSummary, crackTest, resetGlass } from './glass';
import { glassPenLive, shotPasses, resetGlassPen, GLASSPEN_HOOKS } from './glasspen';
import { bootBars, resetBars } from './winbars';
import type { WeaponId, RifleAttachment } from './weapons';
import { Doors, Door, DoorEvent, DoorContact, FLOOR_KEY } from './doors';
import { dealSwings, swingsNow, type SwingDeal } from './doorswing';
import { nameDoors } from './doornames';
import { doorBlast } from './doorblast';
import { RAG } from './ragdoll';
import { Throwables, ThrowSolution, Item } from './throwables';
import { PropSystem, PropEvent, Prop } from './props';
import { GameAudioLike, nullAudio } from './audio_iface';
import * as fx from './effects';
import { Sparks } from './sparks';
import { Tracers } from './tracers';
import { Particles } from '../render/particles';
import { sparkSink, kickDustFx, lockSplinterFx, clearParticleFx, applyDustLook } from './particleFx';
import { Weather } from './weather';
import { stoodOfPick, type StoodWith } from './stood';
import { wetInstance, wetKnobs, wetLineRow, wetSummary, clearWetPrints } from './wet';
import './takedown';   // takedowns as moves (takedown.ts): who breathes, once a frame
import { hourAfterReset } from './hours';
import { charLookFrame, charSides } from './charLook';
import { placeholderTints, TINTS_AS_IS } from '../render/charMaterials';
import { rainSummary } from './rainFx';
import './smokebreak';
import { resetShowWhy } from './showwhy';
import { updateFire } from './fire';
import { railLightDef, torchLightDef } from './rigProps';
import { hostileTo, type Team } from './teams';
import { PLAYER_ID } from './consts';
import { seatsStand, seatManId, standMen, spotBeside, manById, isManId, eachSeat } from './seats';
import { doorMan } from './doorthrow';
import './mission';
import { NO_VIEW, type TickView } from '../view/seatview';
import { NO_DEBRIEF, type DebriefView } from '../view/debriefview';
import type { WorldAsk } from '../view/worldask';
import type { CursorAt, CursorLens, CursorView, Under } from '../view/cursor';
import { Chronicle, chronInterrogated } from './chronicle';
import { hiddenProps, irBeams } from './combat';
import { irViewer, irMade } from './irlaser';
import { thinFit } from './thinline';
import { dropThrowable } from './combat';
import { FRAG, fragReset } from './frag';
import { moraleReset } from './morale';
import './runner';
import './flinch';
import './darkdoor';
import './netquiet';
import './samstun';
import './sprintdoor';
import './deathrecap';
import { bodyLightSummary, groundFresh } from './guards';
import { CpuSmoke } from './sense';
import { DRESS } from './dress';
import { say, resetGuards, tension, escalationSummary, raiseEscalation, standDownEscalation, clearingSummary, clearQueueSummary, forceClear } from './guards';
import type { SearchPlan, Pieing, Standoff } from './guards';
import type { Clearing } from './squad';
import { launchCancel } from './player';
import { closeReset } from './close';
import { STALK, stalkReset } from './stalk';
import { tapReset } from './tapstep';
import './samlook';
import type { CloseKind } from './close';
import type { Haul, DoorMove } from './hands';
import type { Wanding } from './wand';
import './bigkit';
import './gunslots';   // (called inside the update only)
import './workstow';   // (the same)
import type { StickyCam } from './stickycam';
import './stickycam';
import { lightSwitchReset } from './lightswitch';
import './handwork';
import { throwCount, toggleDrag, teleportPlayer, settleSpot, freeHeld, takeKey, pickHover } from './player';
import type { WallHold, Hold } from './player';
import { escapeInteract, backableInteract } from './interact';
import { tacticsSummary, tacticsReset } from './tactics';
import { lanesReset } from './lanes';
import { pictureReset } from './picture';
import './darkfoe';
import { powerOutReset } from './powerout';
import { rescueCheckReset } from './rescuecheck';
import { dozeReset } from './doze';
import { wallReactReset } from './wallreact';
import './nvg';
import { irFlags } from './irlight';
import './silent';
import { holdFireReset } from './holdfire';
import './spotted';
import './handleside';
import './doorway';
import { needsSummary } from './needs';
import { Director } from './director';
import { NightBreach } from './nightbreach';
import { QuietMission } from './quiet';
import { Lesson } from './lesson';
import { Rails } from './rails';
import { Par } from './par';
import type { LostMan } from './deathrecap';
import { Verdict } from './verdict';
import { Defuse } from './defuse';
import { RoomClearScenario } from './roomclear';
import { SquadOrders } from './orders';
import { leversFog } from './difficulty';   // (called in the constructor only: the fog lever's rung over the kept feel rows)
import { FogOfWar, fogFlags } from './fogofwar';
import { RoomFight } from './roomfight';
import { Cues } from './cues';
import { SquadNet } from './squadnet';
import { Cover } from './cover';
import './rearwatch';
import { EntryPoints } from './entrypoints';
import { ClearedRooms } from './cleared';
import { Rescue } from './rescue';
import { DoorTools } from './doortools';
import './squadsmoke';
import './wandorder';
import './squadthrow';
import './squad40';
import { PairOrders } from './pairorders';
import { pairUpApply } from './pairup';
import { qrfAddressWord } from './qrf';
import { CHAOS } from './destruction';
import { WallCharge } from './wallcharge';
import { WorkThings } from './workthings';
import { DoorPath } from './doorpath';
import { registerShutDoor } from './shutdoor';
import { spawnCivilians, resetCivilians, civilianAnswer, civiliansSummary, loadCivTune } from './civilians';
import type { Civ } from './civilians';
import { updateTargets } from './lighting';
import { LightHome, LIGHT_IN, type Fixture } from '../sim/lightshome';
import { carLampTargets } from './carlamps';   // a parked vehicle's lamps are light targets a round breaks (nothing with CARLAMPS down)
import { setBlackout, endBlackout, lightEvent, ocp, setGroup, repairLights, freshLights } from './lighting';
import { gunWallFresh } from './gunwall';   // (called inside cleanSlate only)
import { CLEANRESTART, CLEANRESTART_STATS } from './cleanrestart';   // a restart gives a clean level (a leaf: the switch and its tally)
import { stepMat } from './floor';
import type { FloorMat } from './floor';
import * as dialogue from './dialogue';
import type { DialoguePick } from './dialogue';
import { Sup, suppressionSummary } from './suppression';
import { conditionSummary } from './condition';
import { AimState, aimSummary } from './aim';
import { Qrf, resetQrf } from './qrf';
import { openingStart } from './opening';
import { Checkpoints } from './savestate';
import { Sensors } from './sensors';
import type { ProbeLike } from './sensors';
import { Devices, learnCode, DEVICE_TUNE_DEFAULTS } from './devices';
import { Destruction } from './destruction';
import './fixtureBreak';   // the look of a fixture breaking and hanging broken (picture only)
import { coverfireReset } from './coverfire';
import { doorForceReset } from './doorforce';
import { Wind, loadWind } from './wind';
import { attachAir, airResend } from './airSim';
import { AirDust } from './airDust';
import { Moths } from './moths';
import { WetGround } from './wetGround';
import { GlassBreak } from './glassbreak';
import { flatPanel, levelOver, stoodOver } from './levelfit';
import { GlassShards } from './shards';
import { kitFresh } from './kit';
import type { BigKit } from './kit';
import type { KitWorn } from './kitload';
import './kitload';   // (asked inside the frame only)
import type { Wound } from './wounds';
import { buildInteriorCells } from '../render/reach';
import { simRandom, useDice } from '../math/dice';   // the simulation's die (math/dice.ts): the Game's own when it has one, Math.random otherwise
import type { SimDice } from '../math/dice';
import type { PoseSet } from '../anim/skeleton';
import { SimState } from '../sim/state';   // (below the game: no part of src/game's import cycle)
import { MenStore, sameMan, manLeaves } from '../sim/men/store';   // (below the game too; the men's tenant registers itself)
import { GuardCols, PlayerCols } from '../sim/men/cols';
import { FallStore } from '../sim/fallshome';   // (below the game too)
import '../sim/dicehome';   // (the die's home in the Rust module registers itself: sim/state.ts simTenant)
import './posehome';   // (…and the poses': registered as it loads, by a statement that uses src/sim's export alone)
import './rustknobs';
import { RUST_PASS, RUST_SURFACE, crossToRust, goneWithTick } from './crossing';

// ---------------------------------------------------------------- light targets
/** a light fixture of the level: what the level says of it, and its state read from the Rust module (sim/lightshome.ts: written only through `Game.lightHome.in`) */
export type LightTarget = Fixture;

/** kick: a door kicked in — carries through the floor and is answered like a shot (guards.ts hearingCheck); every other door noise is 'door' with a level. A round
 *  ('shot' / 'guardShot' out of combat.ts fireWeapon) also says which weapons.ts row fired it and the segment it actually flew, muzzle → where it stopped (from / to):
 *  nothing in the hearing model reads those — a report is a report — they are for whoever reasons about ROUNDS (the QA soak; suppression). whistle: Sam's low whistle
 *  (V, whistle.ts) — the nearest calm man who hears it comes for a look, twice and he searches; level WHISTLE.level, at his feet, who = the player. */
export interface GameEvent { announced?: boolean; kind: 'shot' | 'lightOut' | 'guardShot' | 'step' | 'door' | 'kick' | 'bang' | 'prop' | 'whistle' | 'glass'; pos: Vec3; time: number; loud: boolean; level?: number; id?: number; who?: number; weapon?: WeaponId; from?: Vec3; to?: Vec3; cause?: 'ocp' | 'shot'; /** a 'lightOut' a hand made at a wall switch: the floor before the plate — where a man who saw it comes to (lightswitch.ts) */ swAt?: Vec3; blind?: boolean; /** a 'prop' knock made to be looked at (a ring's clack): attention.ts — it reaches a calm man and he goes to look at IT */ lure?: boolean; /** a 'bang' that was a fragmentation grenade (frag.ts blast), not a stun charge */ frag?: boolean; /** a 'bang' that was a door or wall CHARGE borrowing the canister's report (combat.ts detonateStun `borrowed`): read only for what a man who hears it SAYS */ charge?: boolean; /** a 'glass' event under GLASS.bothFaces (glass.ts glassEvent): a point just off each face of the pane, which the crash is heard from as well as from `pos` */ ears?: [Vec3, Vec3]; }   // (cause, lightOut only: the OCP's spell — the light dims out and comes back — or a fixture SHOT dark; guards.ts hearingCheck weighs them differently)
export type ThrowKind = 'smoke' | 'flash' | 'frag';
/** What a slot of the kit holds — the item Player.slot has SELECTED (in the hand, or holstered / slung with E): the Five-seveN, the FN2000, a smoke canister, a stun
 *  canister. Every rule about "what is in his hand" reads the kind, never a digit; the digit keys and the kit column's order and numbering are KIT_ORDER's alone
 *  (1 pistol · 2 rifle · 3 smoke · 4 stun — the numbering is stable per item: a kit without the rifle still throws smoke on 3). */
export type SlotKind = 'pistol' | 'rifle' | ThrowKind;
/** the kit top to bottom = the digit that selects each item (index + 1) */
export const KIT_ORDER: readonly SlotKind[] = ['pistol', 'rifle', 'smoke', 'flash', 'frag'];   // (5: the fragmentation grenade — frag.ts; the last digit there is: nothing in the game is on a digit above 5)
/** Anything Space acts on. Two shapes share the type: the MARKER of a thing (Game.interactables — one per door / body / guard / rack / breaker / light: its dot in the
 *  world, the auto-aim's lights; `verb` absent) and the ROW of one thing Space can do to it right now (Game.nearby — player.ts buildInteractables: 'push it open'
 *  and 'ease it open' are two rows of one shut door; `verb` set, `line2` = 'Space␠␠<the verb>' plus hint lines for the thing's other inputs). progress (0‥1): the
 *  bar in the row / the ring round the reticle while a sustained verb runs (`running`: that row IS the running verb — a lock being picked, a leaf being eased,
 *  the choke, the drive coming out). 'held' = the man in Sam's arm (player.ts Hold): his rows are the only rows while he is held. 'cancel' = the selector's
 *  bottom row while it is open (interact.ts): letting go of Space on it does nothing. */
export interface Interactable { kind: 'light' | 'door' | 'body' | 'guard' | 'objective' | 'held' | 'breaker' | 'wall' | 'cam' | 'switch' | 'charge' | 'cancel'; pos: Vec3; line1: string; line2: string; inReach: boolean; off: boolean; progress?: number; target?: LightTarget; door?: Door; guard?: Guard; verb?: Verb; running?: boolean;
  /** a man's row with the close-in windows on (close.ts): the way in Sam's spot gives — the click and the grab that follow make that move */
  close?: CloseKind;
  /** a door's row that is listed PAST the list's cap, under the rows Space had before it (the big slot's tool on an unlocked door — bigkit.ts; a charge by hand
   *  is the other such row, known by its verb) */
  past?: boolean;
  /** a MARKER whose every row the mission's frame kept back this frame (rails.ts `rows`; player.ts buildInteractables): the key does nothing on the
   *  thing though it has rows — an empty tap in reach of it says so (interact.ts emptyWord) */
  framed?: boolean; }
/** Every verb a row can be (player.ts buildInteractables words them; game.ts updateDoors / player.ts updateHold / mission.ts act on Game.act): a shut door's
 *  push / ease (silent, sustained), an open one's pull, a lock's pick (sustained) / kick / rattle (the handle tried, crouched), a keyed lock's unlock; the grab
 *  of a man from behind; the held man's question / choke (sustained) / pistol-whip (sustained); a body's drag / drop; the rack's drive (sustained); the mains
 *  thrown / re-armed by hand; and the selector's own cancel. */
export type Verb = 'push' | 'pull' | 'ease' | 'unlock' | 'pick' | 'kick' | 'rattle' | 'code' | 'nocode' | 'wand' | 'breach' | 'hammer' | 'grab' | 'question' | 'choke' | 'whip' | 'comply' | 'drag' | 'drop' | 'drive' | 'breakerOff' | 'breakerOn' | 'charge' | 'blow' | 'defuse' | 'takeCam' | 'lightsOff' | 'lightsOn' | 'lead' | 'stay' | 'cancel';   // lightsOff / lightsOn: a room's wall switch (lightswitch.ts);   // lead / stay: a rescue's hostage told to get up and follow, or to stay where she stands (rescueend.ts);   // takeCam: a stuck sticky camera of his own taken back onto the launcher (stickycam.ts);   // comply: a held CIVILIAN told to get down and stay quiet — lets her go, cowering (civilians.ts);   // code / nocode: a keypad door's pad — the code entered (sustained, devices.ts) / the pad tried without one (a deny beep)

// ---------------------------------------------------------------- mission
/** The one mission thread: insert (the spawn) → reach the server room → pull the drive from one rack (F, a few seconds within reach) → out through the
 *  fire exit. Everything it touches already exists (the rooms, that rack's LED switchables, the hearing model, the down / restart flow): it only watches,
 *  counts and says what is next. Geometry lives in level.mission. */
export type MissionStage = 'infiltrate' | 'drive' | 'exfil' | 'done' | 'failed';
export interface MissionStats { /** RESPONSE: the level's name for the team that came (qrf.ts deploy; absent: the office's) */ qrfWho?: string; elapsed: number; alerts: number; spotted: number; bodies: number; shots: number; kills: number; knockouts: number; keys: number; blackout: boolean; sandbox: boolean; qrf: 0 | 1 | 2; qrfAt: number; }   // qrf = how far the response went this run (game/qrf.ts: 0 nobody called it, 1 the response team came onto the floor, 2 the compound was locked down round him — the debrief's 'exfil under perimeter lockdown'), qrfAt = mission seconds when the team deployed (−1); alerts = alarm EPISODES (someone went to alert while nobody was), bodies = distinct bodies discovered (dead or out cold); kills = men the PLAYER shot dead, knockouts = men he put down breathing (the takedown; a choke-out later) — combat.ts killGuard's how / by; keys = keys taken off questioned men (player.ts takeKey); sandbox = a cheat flag was on at some point
export interface Mission {
  stage: MissionStage; t0: number;   // t0: game time the run started (elapsed is accumulated separately: pauses and the tour do not count)
  driveT: number;                    // seconds into pulling the drive, -1 = not pulling
  pulled: boolean;                   // the drive is out: its rack stays dark until the encounter restarts
  alarmOn: boolean;                  // somebody is at 'alert' right now (edge → an alarm episode)
  sweepAt: number;                   // game time at which somebody gets sent to look at the dead rack (-1 = nobody pending)
  spottedT: number;                  // last time an alert guard had eyes on you (episode debounce for stats.spotted)
  downCardAt: number;                // game time the down card comes up (mission.ts DOWN_CARD_SECS after he dropped; -1 = not pending)
  handsOn: boolean;                  // the drive has been touched at least once (the chronicle notes the first time only)
  stats: MissionStats;
}
export type MissionRating = 'ghost' | 'panther' | 'assault';
/** Chaos Theory's own words: ghost = never spotted, no kills, no alarms; panther = lethal or loud but never actually seen; assault = they had eyes on you. */
export function missionRating(s: MissionStats): MissionRating { return s.spotted > 0 ? 'assault' : s.kills === 0 && s.alerts === 0 && s.bodies === 0 ? 'ghost' : 'panther'; }   // ghost: never seen, no alarm, nobody shot, nothing found (a silent takedown nobody finds keeps it — a man out cold is not a kill); panther: they know someone was there but never saw who
export function fmtClock(sec: number): string { const s = Math.max(0, Math.floor(sec)); return `${Math.floor(s / 60)}:${String(s % 60).padStart(2, '0')}`; }
const newMission = (t0: number): Mission => ({ stage: 'infiltrate', t0, driveT: -1, pulled: false, alarmOn: false, sweepAt: -1, spottedT: -100, downCardAt: -1, handsOn: false, stats: { elapsed: 0, alerts: 0, spotted: 0, bodies: 0, shots: 0, kills: 0, knockouts: 0, keys: 0, blackout: false, sandbox: false, qrf: 0, qrfAt: -1 } });

/** the ragdoll particles a door leaf feels (Game.updateDoors): enough of the body that a leaf cannot close through it */
export const BODY_DOOR_PARTICLES = [RAG.pelvis, RAG.spine, RAG.head, RAG.kneeL, RAG.kneeR, RAG.ankL, RAG.ankR];   // (how many of them there are is in the arithmetic of what a doors' step is handed: levelfit.ts handOverMost)

// ---------------------------------------------------------------- player
/** the query tables the world is asked aside with (Game.aside): scratch of the host's, in no snapshot */
export class AsideOwn extends HostOwned { s?: object; q?: object; w?: object; lens?: FollowCamera }
export class Player extends PlayerCols {
  char: Character;
  crouch = false; aimHeld = false; lastFireT = -10; fireCd = 0; backpedal = false; sprinting = false; /** Shift held in a crouch this frame (player.ts CROUCHRUN): the fast crouch walk — set where `sprinting` is, false wherever that is not reached */ dash = false;
  friendLow = false; friendT = -10;   // friends.ts: his raised gun is down for a teammate in its line (FRIENDS.lowReady), and how long that line has been clear
  shoved: [number, number] = [0, 0]; shovedT = -1; shovedFrom: [number, number] = [0, 0];   // noshove.ts: what the men he met moved him by in this update (x, z), the clock of that update, and where the update found him — written and read inside one frame
  haul: Haul | null = null; doorMove: DoorMove | null = null;   // hands.ts: the pick-up of the body in `dragging` (which end, which beat) · the door he is working by hand (HANDS, its switch; both null with it off)
  dragging: Guard | null = null; dragYaw = 0; dragNoiseT = 0; dragStuckT = 0;   // the body being dragged (F): crouched, facing it, its ankles in your hands (ragdoll pins), the rest trailing along the floor
  noise = 0;                                                    // HUD sound meter: the loudest thing you did lately (hearing-model level, decays)
  takedown: { g: Guard; t: number; kind?: CloseKind; from?: number; alert?: boolean; yaw?: number; at?: Vec3; lethal?: boolean; /** CLOSE.rmbHold: the quick knockout — heard further */ loud?: boolean } | null = null;             // silent takedown in progress (a LEFT CLICK with the hands empty on the man in reach behind): locked behind the guard until the strike lands
  /** the man in his arm (player.ts startGrab / updateHold — the 'grab him' row on a living guard from behind): the pair moves as one at a slow walk, his rows put
   *  questions to him (Game.interrogate) and choke him out quiet (holstered: the arm variant) or pistol-whip him (pistol drawn: the gun variant, which also
   *  fires past him on a click) — there is no letting him go; null when his hands are his own. */
  holding: Hold | null = null;
  /** disarm.ts HOLD.stripFirst: the game time the grab's catch voice is due (the arm landing, behind the strip's) — undefined = none owed */
  catchDue: number | undefined = undefined;
  /** what Sam is saying right now — his side of an exchange with the man in his arm (player.ts askHeld: his line first, the man's answer when it ends) — drawn
   *  over HIS head like a guard's line, in his own tint (ui/bubbles.ts); the same shape as Guard.bubble */
  bubble: { text: string; t: number; dur: number; radio: boolean } | null = null;
  speedSm = 0;
  visibility = 0;          // 0..1 from light meter
  irradiance = 0;
  hitFlash = 0;
  /** a blast that left him up (samstun.ts): folded until / blind until, on the game clock, and where the flash was — the same two stamps a Guard carries */
  staggerUntil = -1; dazzledUntil = -1; dazzledAt: Vec3 | null = null; stunPosed = false;
  /** game time a blast left him hurt (condition.ts: -1 = never; read through isHurt) */
  hurtAt = -1;
  /** a sprint into a door (sprintdoor.ts): when he last put his shoulder through one and which way it turned him, and until when his hand is ripping one round (the run held to a walk) — on the game clock */
  doorBashAt = -10; doorBashLean = 0; doorRipUntil = -1; doorKickTail = false;
  /** the look his head was last given against that turn, and the game time it was given on (player.ts's sprint facing; game.ts takes it back off the frame nobody renews it) */
  doorHead = 0; doorHeadT = -1;
  /** suppression (suppression.ts, docs/internal/sc20k-design.md §4): the rounds passing / striking close to him — the same scalar and rule the guards duck under.
   *  This lane reads it for one thing only, the cone his own rounds open up by (player.ts fire sites, samSpread); the picture, the mix and the reticle are R4's. */
  sup = new Sup();
  /** the scalar itself, 0‥1 (what R4's presentation reads) */
  get suppression() { return this.sup.v; }
  /** the aim-settle model (aim.ts): his track on the man under the reticle (or on nothing in particular), that man's bearing rate across his view, the last round's
   *  numbers — fed once a frame by game.ts (trackPlayer), read by his fire sites (samAim) and the overlay's ring. Rests at 0 with tune.aimModel off. */
  aim = new AimState();
  /** aimweight.ts: his gun's heading trails the cursor — the swing's rate and the circle's range, live only on frames the weighted turn drove his aim */
  aimw = new AimWeight();
  static readonly MAX_HITS = 1; hitsLeft = Player.MAX_HITS; down = false;   // one 9 mm hit and you are down, same as the guards; Enter restarts the encounter
  // equipment, by KIND (SlotKind — the digits and the kit column's numbering are KIT_ORDER's: 1 the pistol, 2 the FN2000 when the kit has it — hasRifle, the start
  // card's loadout toggle, remembered per machine — 3 smoke canisters, 4 stun canisters); G quick-throws whichever throwable was selected last (`throwKind`) where the quick throw is on (quickthrow.ts: not in the page). "The
  // weapon in his hand" is gunInHand(): the pistol with 'pistol' selected and drawn, the rifle with 'rifle' selected and off the sling, nothing holstered / slung or
  // with a canister up — every site that used to spell out `slot === 1 && !holstered` for the pistol reads pistolUp, which means exactly that.
  pistol = new Pistol(); ocp = new Ocp(); canisters = 4; flashbangs = 2; /** fragmentation grenades (frag.ts FRAG.issue; digit 5 then a click — G never throws one) */ frags = FRAG.issue; slot: SlotKind = 'pistol'; throwKind: ThrowKind = 'smoke';
  /** Sam's FN2000 (docs/internal/sc20k-design.md §5, weapons.ts Rifle: the fn2000 row + its light switch, its selector, its attachments and the rings left) and
   *  whether this kit has it at all. Selected with 2 (2 again with it in the hands flips the load its launcher has on the right button: ring ⇄ 40 mm); `holstered` with it selected means
   *  SLUNG across his back (rigProps RIG.sc20k.slung) — the same one flag E, the wall hold, the grab and every hands-free rule already read; in the hands it is
   *  the animator's long gun (player.ts arms it: LONG carriages by state). */
  rifle = new Rifle(); hasRifle: boolean;
  /** seconds left of the rifle coming off the sling into the hands (or of any change of what is in them that involves it): no round or ring meanwhile */
  equipT = 0;
  /** slotsetup.ts: the setup of the gun in his hands is up (its digit pressed again) — which gun, the game time it opened or was last used, and the last
   *  update that saw it; null = not up. Never saved: a load is a fresh Player with nothing up */
  setup: { gun: 'pistol' | 'rifle'; t: number; seen: number } | null = null;
  /** slotsetup.ts: the gun its hook last found in his hands (null: none) and the game time it first did — a gun just drawn does not open its setup,
   *  unless his own press of its digit is what drew it (`own`: the game time that draw is over; SLOTSETUP.draw) */
  setupHand: { gun: 'pistol' | 'rifle' | null; t: number; own?: number } | null = null;
  /** slotsetup.ts: the last press of a gun's digit that the setup did not take (the press that draws that gun), and when */
  setupKey: { gun: 'pistol' | 'rifle'; t: number } | null = null;
  /** the rifle left his hands when he went down (combat.ts hitPlayer dropped it as an item): nothing to draw slung on the body */
  rifleGone = false;
  /** KIT ANIMATIONS (kitanim.ts): what was in his right hand last frame by the game's rules (undefined = not watched yet: a fresh Player — a restart, a loaded
   *  checkpoint — starts with no errand pending) and the kind of can it last was (the one a put-away shows after the slot has moved on) */
  kitWas: KitItem | null | undefined = undefined; kitCan: ThrowKind | null = null;
  /** the launcher's button (the right one) is DOWN with the rifle in the hands (player.ts), and the load it will send: its arc is on screen and letting go sends
   *  the round; null = not down. Lasts only from frame to frame of the rifle up and free — not saved (a load is a fresh Player: nothing held). */
  launchHeld: RifleAttachment | null = null;
  /** kitanim.ts kitClick: a left click made while a gun was still coming up, owed to the frame it is up: the game time it came, -1 = none (never saved: no checkpoint inside a kit change) */
  kitOwed = -1; kitOwedFor: KitItem | ThrowKind | null = null;   // (…and the thing he was fetching when it came — a can by its kind: it is that thing's click only)
  /** kitanim.ts kitSlingRest: how far the rifle hanging in front is swung onto his thighs (rad) and the game time it was last eased at; picture only, not saved */
  slingRest: { a: number; t: number } | null = null;
  /** the one-time word that the OCP lives on the pistol: said the first time the right button is pressed over a light with a long gun in the hands that has
   *  nothing under its barrel (the shotgun in the rifle's slot — player.ts) — after that the button is simply dead there */
  ocpHinted = false;
  holstered = true;    // he starts with the pistol on the thigh (playtest call: hands free by default — Space grabs, a click behind a man is the takedown); E puts the selected item away / draws it again (Chaos Theory: equip / unequip); holstered + LMB behind a guard in reach = the silent takedown (its one and only input), the pistol's paths all check it
  /** the keys in his pocket, by lock name (doors.ts Door.lockId / FLOOR_KEY): taken off a man he questions the moment the man gives the keyed door up (player.ts
   *  takeKey — Guard.keys empties into this). His door contact carries them (updateDoors) and every 'is this lock against you' test reads them (Doors.lockedOut),
   *  so a keyed door on its corridor face opens to him as it does to the staff — no pick, no kick — and springs locked again behind him. The kit column shows a
   *  KEY tag while it holds anything. Empty on a fresh Player: a restart takes them with everything else. */
  keys = new Set<string>();
  /** the door codes he KNOWS, by lock name (a keypad door's Door.lockId is its pad's name — devices.ts): learned off a man he questions (dialogue intel 'code') or
   *  overheard from two of them talking within earshot (devices.ts overhear). A keypad door whose code is in here offers 'enter the code'; his door contact never
   *  carries them (he types it in — the leaf stays a wall till the pad takes it). Empty on a fresh Player. */
  codes = new Set<string>();
  /** the keypad door he is entering the code at (the 'code' row — a sustained verb like the pick: devices.ts updateCoding beeps four keys over CODE_SECS and the pad
   *  takes it; a step, Space or Esc breaks off) and how far in he is; null when none */
  coding: Door | null = null; codeT = 0;
  /** handwork.ts: the mains breaker's lever under his hand (the 'breaker' rows with HANDWORK on — `on`: re-arming it; t: s since his hand got there; thrown: the
   *  mains have gone over), null when none · s the verb in hand has waited for his hand to reach the thing (a code, the lever) */
  breakerMove: { on: boolean; t: number; thrown: boolean } | null = null; handWait = 0;
  nearSel = 0; nearKey = '';    // index of the highlighted row of the interaction list (↑ / ↓, the wheel while Space is down) and the identity of the current list
  /** The Space selector (interact.ts): downT = seconds Space has been down this press (−1 = up, or a press not armed); open = held past TAP_SECS (or scrolled while
   *  down) with rows on offer — the list shows its open state, a cancel row sits at its foot, his feet are frozen, and letting go performs the highlighted row;
   *  moved = the highlight was scrolled during this press (a press that scrolls is a choice, never a tap); wheelAcc = wheel travel toward the next row; runKey / moveLatch: which sustained verb was running
   *  last frame and the movement keys already down when it began (a step cancels a running verb — a key held into it does not until it has come up once). */
  sel = { downT: -1, open: false, moved: false, wheelAcc: 0, runKey: '', moveLatch: 0,
    /** handring.ts `empty`: what the last tap that found no row said and when (the corner shows it for a moment), and when a line of it last went to the log */
    word: '', wordT: -1e9, fileT: -1e9,
    /** the list up as a ring at the cursor (cmdring.ts CMDRING.hands — null: the list as it was): where, and whether the cursor may light a wedge yet; `key`: the rows it was last read over (Player.nearKey) */
    ring: null as (RingState & { key: string; /** handring.ts: the press that opened it has come up and it stands by itself */ free?: boolean; /** …the lit wedge is the whistle's, which is no row (interact.ts) */ whistle?: boolean }) | null };
  /** the selector open AS A LIST (feet still, a cancel row at its foot, the panel in its open look) — not up as a ring */
  get listOpen(): boolean { return this.sel.open && !this.sel.ring; }
  pace = 1; paceShownT = -10;   // walking-pace multiplier set with the mouse wheel, PACE_MIN‥1 of the walk / crouch-walk speed (sprint is its own held modifier); shown as ticks in the kit column
  nv = false; nvAmount = 0;
  light!: RtLight;   // the rail light: posed from the gun every frame (rigProps.rigLightPose)
  throwEaten = false;   // (the quick-throw key's press went to the order menu's frag row: the rest of that press — the key still down, on the page — is not a throw; cleared when the key is up. player.ts)
  throwHeld = false; heldKind: ThrowKind = 'smoke'; throwPreview: ThrowSolution | null = null; pendingThrow: { t: number; sol: ThrowSolution; kind: ThrowKind } | null = null;
  /** the leaf he is easing open by hand (the 'ease' row: started by a tap, it goes on round at the crack rate on its own until it stands open, or a step / Space /
   *  Esc stops it where it is — game.ts updateDoors runs it; stallT = seconds it has not moved, a body or a chair against it); null when none */
  easing: Door | null = null; easeStallT = 0; easeBest = 0;
  /** locked doors (player.ts updatePicking / updateKick): the lock being worked — planted in a crouch at it, hands on the tools, on its own clock once begun (a step,
   *  Space or Esc breaks off, the progress kept on the door a moment); the kick in progress — rooted square to the leaf until the foot is back down (spot / yaw:
   *  where, fixed when the kick began); fHeld: the Space press now down has been SPENT (it stopped a running verb, a teleport / a kick / a finished pick swallowed
   *  it) and must come up before Space arms again */
  picking: Door | null = null; kick: { door: Door; spot: Vec3; yaw: number; run?: boolean; /** the beat is the big slot's tool on the lock, not the boot (bigkit.ts samBigTick): which, its clock, the blows or shells so far, the next one's time */ tool?: { kind: 'shotgun' | 'hammer'; side: 1 | -1; /** the beat's clock: 0 = his hands leave for the tool; it starts `lead` below that while his own gun is carried away by hand (bigkit.ts BIGSLOT.weight) */ t: number; lead?: number; n: number; /** blows the hammer needs here (one where a boot would do, BIGSLOT.swings where the frame holds one) */ need: number; next: number; done: number; /** the door gave to it: a leaf left standing home (broken from the face it opens toward) is pulled open as the beat ends — BIGSLOT.shut */ gave?: boolean;
    /** BIGSLOT.moves (bigkit.ts): he WALKS onto his place with the tool coming into his hands — `arrived` once he stands on it; the point struck or shot; when each blow or shell lands, on `t` (the last one may still be to come); when his hands began putting it back (−1: not yet) */ moves?: { arrived: boolean; at: Vec3; blows: number[]; end: number; /** the corner of his walk, round a man of the file (gone once he has turned it) */ via?: Vec3 } } } | null = null; fHeld = false;
  /** the optical wand (wand.ts): `wand` = it is in his kit (his card's row — kit.ts kitPlayer); `wanding` = he is on his knee at a shut door with it under the
   *  leaf (player.ts startWand / updateWanding; a step, Space or Esc pulls it out) */
  wand = false; wanding: Wanding | null = null;
  /** the breaching shotgun or the hammer in his big slot (kit.ts BIGSLOT, his card's; bigkit.ts: a row of a door locked against him). 'none' as built. */
  bigKit: BigKit = 'none';
  /** what his card weighs (kitload.ts: its mobility and concealment beside his reference's, written where his card goes on him — kit.ts kitPlayer; null: nothing weighed on him, nothing of his is touched) */
  kitLoad: KitWorn | null = null;
  /** the sticky cameras he has sent, in flight and stuck (stickycam.ts; how many are left to send rides his rifle: Rifle.cams) */
  cams: StickyCam[] = [];
  /** the last id a camera of his was given (stickycam.ts fireCam: one up each, never reused in a run) */
  camSeq = 0;
  /** a door charge going on by his own hands (doortools.ts samTick; owncharge.ts): the door, the face he is on, when it began, where he stood. Not saved:
   *  savestate.ts playerBusy refuses a plain save while it runs */
  charging: { door: Door; side: 1 | -1; t0: number; at: Vec3 } | null = null;
  /** what was in his hand is put away for a charge going on by his own hands (workstow.ts samStowTick): `drew` = something was, and comes back when the job is
   *  over; `t` when the job began; `down` = his hands are free of it (the job's clock runs). null = no such job. Not saved: no save is written inside one. */
  workStow: { drew: boolean; t: number; down: boolean } | null = null;
  /** back to the wall (player.ts, Q): the static face he is pressed against and slides along — null when free. For the HUD / camera: n (unit normal out of the
   *  wall into the room), along (unit tangent: his right when his back is flat to it), atEdge (parked at an OPEN end of the face — an outside corner, a door jamb —
   *  −1 / +1 = which end along `along`, 0 = not at one or the way on is walled), peek (0‥1, eased in while he keeps pushing into that open end), crouched. The
   *  rest is the controller's bookkeeping (see WallHold). Anything that plants him elsewhere (takedown, kick, pick, drag, teleport, going down) lets go first. */
  wall: WallHold | null = null;
  /** the last spot the collision sweep settled him at (player.ts collidePlayer walks him from here to wherever he has been put since, in short strides; null until the first settle, reset by a teleport) */
  safe: Vec3 | null = null;
  stepPhase = 0;
  /** how far the walk cycle has come since the last footfall filed, in turns, either way added up, 1 at the most (player.ts FOOTFALL.once; 1 = far enough,
   *  which is where it starts, and where it stays while the key has never been on) */
  stepTravel = 1;
  /** how far the walk cycle has come at a footfall's speed since the last edge of it passed or footfall filed, in turns, either way added up (player.ts
   *  FOOTFALL.owed: at half a turn a footfall is owed; 0 where it starts, when he slows, after frames the footfall test was not asked, and while the key
   *  is down), and the game clock the test was last asked at with the key up (-1: never) */
  stepOwed = 0; stepAt = -1;
  constructor(men: MenStore, ch: GltfCharacter, pos: Vec3, hasRifle = true, seat = 0) {   // (`men`: his game's rows · `seat`: whose man — seats.ts; the first is PLAYER_ID in the suit he always wore, a second player's wears another trim so the two tell apart)
    super();
    this.char = new Character(ch, { id: seatManId(seat), tint: [0.05, 0.06, 0.07], tint2: seat ? [0.1, 0.2, 0.36] : [0.1, 0.32, 0.18], pos, yaw: Math.PI, men });
    sameMan(this, this.char);
    this.char.muzzleAhead = this.pistol.def.muzzleAhead;   // (the row's: where the round, the flash and the rail light's beam start along the gun)
    this.hasRifle = hasRifle;
  }
  /** the firearm the kit has SELECTED (the rifle with 'rifle' selected and in the kit, else the pistol — a canister up leaves the pistol as the gun 1 comes back to) */
  get firearm(): Firearm { return this.rifleSel ? this.rifle : this.pistol; }
  /** what is physically in the gun hand right now: the Five-seveN drawn, the FN2000 off the sling (not while he hauls a body by the ankles: it goes to the sling for
   *  that), or nothing (holstered / slung / a canister up / no rifle in the kit) */
  gunInHand(): Firearm | null { return this.holstered ? null : this.slot === 'pistol' ? this.pistol : this.rifleSel && !this.dragging ? this.rifle : null; }
  /** the Five-seveN is the selected item and drawn — what `slot === 1 && !holstered` always meant */
  get pistolUp(): boolean { return this.slot === 'pistol' && !this.holstered; }
  /** the FN2000 is the selected item (in the hands or slung) and the kit has it: L / Z / R / the kit column's lit row are about the rifle */
  get rifleSel(): boolean { return this.slot === 'rifle' && this.hasRifle; }
  /** the FN2000 is in both hands (selected, drawn, and not hauling a body by the ankles — that slings it) */
  get rifleUp(): boolean { return this.rifleSel && !this.holstered && !this.dragging && !this.down; }
  /** Braced on the foregrip, the stock in the shoulder: ALWAYS, whenever the rifle is in the hands and he is neither sprinting (gun down at the run) nor kicking a
   *  door (across the chest for that) — there is no brace input any more (playtest 08-08: right + left button together is misery on a trackpad, and a rifle in the
   *  hands that is not being aimed had no reason to be in them). What reads it: player.ts rifleSpread (RIFLE_SPREAD.braceK is simply the rifle's in-hand steadiness),
   *  the aim model's cone (aim.ts AIM_HOOKS.playerExternalK), and `aiming` — the rifle is shouldered for as long as it is up. */
  get braced(): boolean { return this.bracedAt(this.sprinting); }
  /** …asked of legs that are or are not at a run (player.ts gunRaised: his own this tick; a look ahead's copy of him, walkahead.ts) */
  bracedAt(sprinting: boolean): boolean { return this.rifleUp && !sprinting && !this.kick; }
  /** the canister the selected slot puts in the fist ('smoke' / 'flash'), null with a gun selected — drawn or not is the caller's `holstered` test. The ONE rule for
   *  "is a can in his hand": player.ts (what a click throws) and combat.ts handProps (what is drawn in the fist) both read it, so the rifle in the hands can never
   *  again come with a smoke can in the other (the old test was 'any slot but the pistol's'). */
  get canisterSel(): ThrowKind | null { return this.slot === 'smoke' || this.slot === 'flash' || this.slot === 'frag' ? this.slot : null; }
}

// ---------------------------------------------------------------- guard
export type GuardState = 'patrol' | 'suspicious' | 'alert' | 'search' | 'dead';
/** One frame's worth of orders for a scripted guard (see squad.ts). goal: walk there (nav path) at `speed` (m/s, default guard walk); face: body
 *  yaw to hold once planted (or while `strafe`); aimAt: where the gun / torch / eyes point; stance + stackSide + crouch feed the animator's tactical
 *  layers; upper: 'aim' draws the pistol, 'torch' carries the torch; arrived is written back by the runner each frame. */
export interface GuardScript { goal: Vec3 | null; speed?: number; face?: number | null; strafe?: boolean; aimAt?: Vec3 | null; stance?: Stance; stackSide?: -1 | 1; crouch?: boolean; lean?: number; upper?: 'aim' | 'torch' | 'relaxed' | 'none'; light?: boolean; arrived?: boolean;
  /** weapons free on the men in front of him: the runner's muzzle discipline (squad.ts — a colleague in the line puts the gun down) is off for this man. The breach
   *  showcase's operatives carry it while they engage the room (breach.ts): until the guards have SIDES every man in Game.guards is a 'colleague', the opposition
   *  included, and an operative could not so much as cover the man he is about to shoot. The teams lane replaces this with sides in colleagueInLine. */
  hot?: boolean;
  /** rounds past this scripted man make him START (suppression.ts: the anim flinch on the rising edge over the flinch line — never the bark, the late round, the head going down or the
   *  shift to cover, which are the AI's; his feet stay the choreographer's). Off by default: the runner's men are otherwise deaf to fire, as they always were. The day showcase's
   *  firefight orders it on both sides (director.ts OrderPatch; the removed day showcase ordered it). */
  flinch?: boolean;
  /** his walk to `goal` is the script's own way: the short one, not priced by the lanes his side knows are covered (lanes.ts lanePenalty). A staged advance
   *  down a road the other side has guns on: without it he goes round, as a man left to himself would (director.ts OrderPatch). */
  direct?: boolean;
  /** he is having a smoke where he stands (smokebreak.ts: the left hand to his mouth and back, the lit end, real smoke — a level script's order, director.ts
   *  OrderPatch.smoke; nothing with that file's switch off) */
  smoke?: boolean;
  /** his muzzle taken off the order by his own read of the fight (roomfight.ts): while set the runner aims at `aimAt` with the gun up — whatever aimAt / upper /
   *  stance the order holds, which are left exactly as ordered and take over again the frame this is cleared — and, `face` given, squares him up to it where he stands */
  over?: { aimAt: Vec3; face: number | null; walk?: boolean } | null; }
/** What the men call each other on the net (by roster order; the radio check asks for a silent man by it — guards.ts updateRollcall). */
export const CALLSIGNS = ['Kowalski', 'Novak', 'Reyes', 'Okafor', 'Lindqvist', 'Brandt', 'Marsh'];   // (seven: a level with a seventh route — the bazaar — must not deal one name twice)
/** who is dealt the carbine over his sidearm row (Game.tune.carbineGuards → Guard.carbine each frame; guards.ts rigOf) */
export type CarbineDeal = 'none' | 'reyes' | 'all';
export const CARBINE_DEALS: CarbineDeal[] = ['none', 'reyes', 'all'];
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
  Marsh: 'steady',       // (seventh) the night watchman: walks his own round, keeps his own counsel
};
/** How a downed man went down and by whose hand (combat.ts killGuard): shot = dead; struck (the takedown's cross, a pistol-whip later) / choked (the choke-out
 *  from a hold, later) = out cold and breathing. His state is 'dead' either way — every one of the floor's 'is this man out of the picture' tests keeps working —
 *  this is the side record the found-body lines, the room clear's grim lines, the debrief and (one day) a wake-up read. by: only the player's own count on the card. */
export type DownKind = 'shot' | 'struck' | 'choked' | 'wounded';   // (wounded: a round that did not land where it kills — wounds.ts, behind WOUND.on: down, breathing, out of the fight, and POSED rather than a ragdoll — Guard.wound)
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
  heard: { shots: number; lastShotT: number; kick: number; bang: number; small: number; smallWhere: Vec3 | null; loudAt: Vec3 | null; lastT: number };
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
export const newWitness = (): Witness => ({ heard: { shots: 0, lastShotT: -1, kick: 0, bang: 0, small: 0, smallWhere: null, loudAt: null, lastT: -1 }, peakAwareness: 0, alertedBy: null, alertT: -1, sawAct: null, sawBody: null, calledToBody: null, sawHeld: null, wasHeld: 0, dazzled: 0 });
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
export type GuardTask = { kind: 'breaker' | 'rack' | 'device'; pos: Vec3 } | { kind: 'checkOn'; pos: Vec3; who: Guard; wp: number; left: number; t0: number };   // device: sent by control to where a camera had movement / a beam broke / a camera went dead (devices.ts) — walked and looked round exactly like the rack
/** A spawned unit's papers (guards.ts spawnUnit — the showcase's operatives and hostiles): not a man of the level's routes, so resetGuards drops him instead of
 *  standing him up again, and despawnUnit knows what to take back out of the world (his weapon light). */
export interface UnitInfo { name: string; spawnedAt: number; light: boolean;
  /** the pair he was dealt into, for a teammate of a squad ordered in pairs (nightbreach.ts — 'red' / 'blue'; orders.ts men() is the one place that reads it) */
  pair?: PairId;
  /** the seat whose player orders him, in a game of more than one seat (seats.ts dealSeats; never written in a game of one) */
  seat?: number; }
/** The two pairs of a four-man squad (Ready or Not's red and blue teams). */
export type PairId = 'red' | 'blue';
/** Who a man's senses, fix and gun are ON (guards.ts, teams): Sam — every staff guard's for life, and today's whole game — or a hostile AI man, or nobody yet. */
export type Foe = 'player' | Guard | null;
export class Guard extends GuardCols {
  char: Character; state: GuardState = 'patrol';
  /** his name on the net (CALLSIGNS by roster index; a spawned unit's given name) */
  callsign: string;
  /** SIDES (teams.ts): which side he is on — 'guard' for every man the level's routes stand up (hostile to Sam and nobody else: the game as it was); the
   *  showcase spawns 'operative' / 'hostile' units (guards.ts spawnUnit). Read wherever "the player" used to be the only possible enemy and "every other guard"
   *  the only possible colleague. */
  team: Team = 'guard';
  /** his foe right now (see Foe): 'player' from birth for the guard side (never anything else — no unit is hostile to staff), chosen each frame by salience among
   *  the hostile characters in his sight for the other sides (guards.ts acquireFoe), null = nobody acquired yet / his man is dead and no other shows */
  foe: Foe = 'player';
  /** the player an enemy has in mind, by seat, in a game of more than one seat (seats.ts facedSeat; never written in a game of one) */
  declare faces?: number;
  /** the route he walks: the level's (level.routes[routeI]) for staff, a one-point synthetic one for a spawned unit (routeI −1) — read as gd.route everywhere the
   *  index used to be looked up, so a man with no level route still has points to stand on and a weapon row to carry */
  route: PatrolRoute;
  /** set on a spawned unit (see UnitInfo), null for the level's own men */
  unit: UnitInfo | null = null;
  /** set on a CIVILIAN (civilians.ts — the cleaner, the late worker: a Guard record for the plumbing that takes one — the hold, the ragdoll, the bubble, the path
   *  follower — kept in Game.civilians, never in Game.guards); null for every man of every AI side */
  civ: Civ | null = null;
  /** a trained man (the response team, game/qrf.ts): the aim-settle model lines him up quicker and his hands are steadier (aim.ts TRAINED — × the settle time,
   *  × his row's cone), and qrf.ts keeps his reaction beats short. False for the night staff and the showcase's men: their numbers are untouched. */
  trained = false;
  /** in a room fight (roomfight.ts sets it every frame he is in one): his side's multipliers on the aim model's settle time and cone — takes TRAINED's place, and a
   *  pinned man with it earns his track like anybody instead of standing on 1 */
  skill: { settleK: number; spreadK: number } | null = null;
  /** an AI target's light meter (guards.ts unitLightMeter — only ever fed for a man some other AI side can be hostile to, i.e. never for staff): the renderer's
   *  irradiance at his chest / head when the probe answers (the browser), the CPU estimate of the same direct light otherwise (headless, or the first frames),
   *  mapped and eased exactly as Sam's meter is (Game.visibilityOf, damp 6). What a hostile AI eye reads where it reads player.visibility for Sam. */
  visibility = 0; irradiance = 0;
  routeI: number; wp = 0; wait = 0; awareness = 0; lastKnown: Vec3 | null = null; lastSeenT = -100;
  /** belief.ts: the point his duty has his gun on while no man is in his eyes (the room fight's, the same object as its watched point); lastKnown is only ever where he believes the enemy to be. Always null with BELIEF off */
  covers: Vec3 | null = null;
  /** belief.ts word(): when a blast or a mate's word last placed his belief (what he hears of that moment or before does not write over it) */
  wordT = -100;
  /** where he worked out a man he lost had gone (guards.ts leadFix): still what his eyes gave him. belief.ts eyesFix */
  ledAt: Vec3 | null = null;
  hold = false;                  // scripted (showcase tour): stand your ground while on patrol / merely suspicious (look, don't walk over)
  pinned = false;                // scripted: in a fight, keep the position you were given (shoot from it, cover the last fix from it) instead of closing in
  /** scripted, SHOWCASE CAST ONLY (director.ts LoadoutSpec.god — the day showcase's two operatives): a round that would have found him passes through him as it passes a colleague (combat.ts
   *  fireWeapon). The show's doctrine keeps the opposition's rounds off them by construction (laid on their cover, traced: tools/qa/day-probes.ts); this is the net under a stray, never
   *  the mechanism, and nothing the player or the level's men can ever carry. */
  god = false;
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
  /** WOUNDS (wounds.ts): down wounded — out of the fight, breathing, posed where he went down (state 'dead', downKind 'wounded'). null on everybody else, always with the switch off. */
  wound: Wound | null = null;
  /** what he has personally seen and heard this encounter (see Witness) — event memory for the dialogue to come; the AI itself never reads it */
  witness: Witness = newWitness();
  /** what Sam has got out of him so far (see Talk) — read and advanced only by the interrogation (dialogue.ts via Game.interrogate) */
  talk: Talk = newTalk();
  /** the keys on his belt, by lock name (level.ts PatrolRoute.keys; the floor key unless the route says otherwise): what Sam takes off him when he gives the
   *  keyed door up (player.ts takeKey empties it into Player.keys — the same man has nothing more to hand over, held again or not), and what makes that
   *  admission true of him at all (dialogue.ts gather: no key on him, no 'mine's on my belt'). NOT what the leaves ask of him: to a door every man on the
   *  staff is keyed (updateDoors `key: true`) — the man in Sam's arm still walks the pair through one whatever is left on his belt. Dies with the Guard. */
  keys: Set<string>;
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
  /** the side-step round a colleague (guards.ts lineStep): lineSide = which way the step under way goes (−1 / +1 along the square-off-the-line basis, 0 = none);
   *  lineGain / lineStepM = how far last frame's step should have left him from its goal, and how long that step was (a step handed straight back by a chair
   *  wedged against the table, a leaf, a man standing on the spot reads as no gain: lineStuckT counts those seconds); lineFail = the sides that went nowhere from
   *  about `pos` lately (bit 0 = −, bit 1 = +, stamped `t`) — not offered again from there until he has moved off or it has lapsed, so a blocked side and a
   *  colleague-fouled other side make 'none' (push past / wait) instead of the same quarter-metre into the same chair every second for as long as the scrum holds */
  lineSide = 0; lineGain = -1; lineStepM = 0; lineStuckT = 0; lineFail: { pos: Vec3; sides: number; t: number } | null = null;
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
  handleGateT = -Infinity;   // when the handle's gate (handleside.ts) last had his feet, by the game's clock
  staysInRooms = false;   // HIS SQUAD'S ORDER IS TO STAY IN ITS ROOMS (enemysquads.ts, a squad's `mayLeave: false`): a fact of his order, not an owner
  fled = false;   // THE WEAPON HAS LEFT HIS HAND FOR GOOD (runner.ts): unarmed, out of the fight and nobody's living hostile from that frame on — running, low in a corner, gone or since shot. A fact of the man, not an owner: it outlasts his death
  gunLostAt = -1;   // a LIVING man whose gun was knocked from his hand at the grab (HOLD.dropGun): dropped = that gun, this = when — his hands are empty (guards.ts inHand) and its light dies on the floor; −1 again once he has it back (player.ts handBack)
  torchLostAt = -1;   // a LIVING man whose torch was knocked from his hand (the grab): dropped = that torch, this = when — the beam follows it down and sputters out (guards.ts updateFlashlight); on patrol afterwards his hand is empty
  path: Vec3[] = []; pathI = 0; repathT = 0; pathGoal: Vec3 | null = null;
  /** What he carries and its state (weapons.ts: the row from his route — the 9 mm service pistol unless the level says otherwise — with its chamber and magazine;
   *  guards.ts fires it and reads its cadence, fireWeapon its report / flash / spread, killGuard what falls out of the hand). fireCd: seconds until the trigger
   *  will go again (the AI's beat: cadence, the reaction on going alert, the shield's floor). reloadT: his reload BEAT — seconds left standing there changing
   *  magazines (the row's reload.dur), −1 = not; when it runs out the weapon is simply full again (weapon.rearm: a guard's spares are bottomless). */
  weapon: Firearm; fireCd = 1; reloadT = -1;
  /** rifles (docs/internal/sc20k-design.md §3): dealt the carbine by the sandbox knob (tune.carbineGuards, written by Game.update each frame) — and the deal IS the
   *  weapon: while it stands, `weapon` is the AKS-74U row (bursts, the 30-round box, the harder report, the rifle that drops — guards.ts burstTrigger, combat.ts),
   *  the RIG in his hands the carbine and his poses the long gun's; dealt away again, his route's row comes back. guards.ts rigOf reads it. */
  carbine = false;
  /** suppression (suppression.ts): rounds passing / striking close to him — the scalar every reaction branches on (guards.ts: flinch, worse and later rounds, head
   *  down, the shift to cover) and its clocks. Nothing else writes it; the panel line and the QA probes read it. */
  sup = new Sup();
  /** the aim-settle model (aim.ts): his track on Sam (0‥1 — no round under AIM.guard.acquireMin of it, his cone × up to unsettledK short of 1), Sam's bearing
   *  rate across him, whether he is braking for a shot, the last round's numbers. guards.ts trackGuard feeds it once a frame; the three fire sites read it. */
  aim = new AimState();
  reactT = 0; searchT = 0; lookPhase = simRandom() * 10;
  speed = 0; flashlight: RtLight; sweep = 0; beamDir: Vec3 = [0, -0.3, 1];
  stepPhase = 0; heardUpTo = -1; smokeTrans = 1; lightDeadUntil = -1; diedAt = -1;
  /** a teammate's breaching shotgun out of his big slot and in his hands for a lock (bigkit.ts gunUp / gunDown): the gun he put down for it, to take up again */
  slung: { own: Firearm; ownId: WeaponId | undefined; ahead: number; gun: Firearm; out: boolean; t: number } | null = null;
  /** a teammate's second gun (gunslots.ts GUNSLOTS: his card's other slot) — `gun` / `id` / `ahead` are the one that is NOT in his hands; `out`: that is his
   *  primary, let go onto its sling for the sidearm; `t` the game time a man was last in front of him with it out (in his eyes, or a round from it); `lock`: no round before this (his hands are between guns) */
  second: { gun: Firearm; id: WeaponId; ahead: number; out: boolean; t: number; lock: number; /** the sidearm's round count when he was last seen firing it */ shots: number } | null = null;
  /** BIGSLOT.moves (bigkit.ts hammerOut): the hammer off a teammate's back and in his hands for a door — the game time his hands left his gun for it, when each
   *  blow lands and when he began putting it back, both on that clock (end −1: not yet), and the point struck. null = on his back. Not saved: a load finds it there. */
  bigOut: { t0: number; blows: number[]; end: number; at: Vec3 } | null = null;
  /** a teammate's gun let go of for a two-handed job (workstow.ts mateStow / mateStowTick): the game time his hands left it, the time it is back up (−1: the job
   *  still runs) and whether the job fed it this frame. null = it is in his hands. Not saved: a load finds it there. */
  workStow: { upT: number; fed: boolean } | null = null;
  /** was his light burning the moment it left his hand (killed / grabbed)? a dark weapon light must not sputter ON as it tumbles (review 08-10 #7: the breach hostiles' unlit lights swept the sunlit room the frame each died) */
  lightLitAtLoss = true;
  dazzledAt: Vec3 | null = null;   // where the charge that blinded him went off (the motion pass turns him away from it — character.ts MOVES; not saved)
  dazzledUntil = -1;   // stun canister with line of sight: no vision, no shooting, rooted to the spot until this time (see Game.detonateStun)
  /** a ring airfoil took him in the body (launcher.ts staggerGuard): doubled over till this time — rooted like the dazzle (reactT), no round from him, but he can SEE,
   *  and he comes out of it alert toward the side it came from */
  staggerUntil = -1;
  /** a booted door met him where he stood right at it (doorthrow.ts): on a knee till this time, inside the fold the same blow gave him */
  doorDownUntil = -1;
  /** game time a blast left him hurt (condition.ts: -1 = never; read through isHurt) */
  hurtAt = -1;
  sawPlayerThisFrame = false; lastPos: Vec3; stuckT = 0; stuckN = 0;
  /** a real sighting of his FOE this frame, whoever that is (teams) — sawPlayerThisFrame stays what its name says: eyes on Sam (what the mission's 'spotted' count
   *  and the witnesses read), which for the staff is the same thing */
  sawFoeThisFrame = false;
  /** (dead) the id of the character whose round put him down, −1 = not by a round / unknown (combat.ts fireWeapon stamps it; the engagement record names him) */
  killerId = -1;
  sawBodyT = -100;                     // game time he last discovered a body: a short refractory before he can call in another (two lying together are one 'man down', not two in two frames)
  leaderStillT = 0;                    // lockdown follower: seconds the man he trails has not moved (planted beside a leader who never moves again, he turns to look at him — guards.ts followLeader)
  alertBest = 1e9; alertStallT = 0; alertRef: Vec3 | null = null; chaseT = 0;   // alert with a fix: nearest he has got to it, how long without getting nearer, the fix that was measured against (a no-progress exit into 'search', guards.ts — re-based when the fix moves), and the seconds this chase has run without eyes on Sam (its hard backstop)
  shotBlockedT = 0;                    // alert with eyes on the player: seconds his line of FIRE (not sight) has been stopped short by something solid — a jamb on his gun-hand side, a leaf, the desk Sam is behind (guards.ts: he closes along the path instead of emptying magazines into the frame)
  sightPos: Vec3 = [0, 0, 0]; sightVel: Vec3 = [0, 0, 0]; sightT = -100; ledT = -100;   // where and how fast Sam was going the last frame this man had him in view (even faintly), when that was, and the sighting whose loss has already been led (guards.ts leadFix: the fix he is left with runs on the way Sam was going, once, the frame he loses him)
  /** when the fix he is working from was PLACED (guards.ts updateGuard stamps it whenever lastKnown jumps): the search reasons about how far Sam can have got since then */
  fixT = -100; fixRef: Vec3 | null = null;
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
  /** `look` (a spawned unit's, guards.ts spawnUnit — absent for the level's men, whose call is unchanged): the trouser tint, the facing (a one-point route has no
   *  first leg to face down), the name he answers to, his side. */
  constructor(men: MenStore, ch: GltfCharacter, id: number, route: PatrolRoute, routeI: number, flashlight: RtLight, tint: Vec3, look?: { tint2?: Vec3; yaw?: number; name?: string; team?: Team }) {
    super();
    const pos = v3.copy(route.points[0]); const p1 = route.points[1 % route.points.length];
    this.char = new Character(ch, { id, tint, tint2: look?.tint2 ?? [0.02, 0.02, 0.025], pos, yaw: look?.yaw ?? v3.yawTo(pos, p1), men });   // facing down the first leg
    sameMan(this, this.char);
    this.lastPos = v3.copy(pos); this.routeI = routeI; this.route = route; this.flashlight = flashlight; this.callsign = look?.name ?? CALLSIGNS[Math.max(0, id - 10) % CALLSIGNS.length];
    if (look?.team) this.team = look.team;
    if (this.team !== 'guard' && this.team !== 'hostile') this.foe = null;   // a side not hostile to Sam has nobody until a hostile man shows himself (a hostile unit starts on Sam like the staff, and re-targets by salience)
    this.keys = new Set(route.keys ?? [FLOOR_KEY]);                    // every man on nights carries the floor key unless his route says otherwise
    this.weapon = new Firearm(WEAPONS[route.weapon ?? 'guard9mm']);   // (a row still switched off throws here, naming itself: nobody carries a long gun before its lane lands)
    this.char.muzzleAhead = this.weapon.def.muzzleAhead;
  }
}

/** STIRRED SMOKE CURLS (smokeWake below; the look page's Stirred smoke row): a moving man as the smoke solver is told of him. body = his capsule's radius (m), from
 *  `foot` above the floor to `above` over his hips; a kick behind him at every step of his walk cycle, `back` behind and `low` under his hips, of
 *  radius kickR, at `kick` × his speed and never over `most` m/s, left out while the sine that picks its side is under `gap`. */
const SMOKE_STIR = { body: 0.24, foot: 0.12, above: 0.5, back: 0.3, low: 0.2, kickR: 0.17, kick: 0.9, most: 2.2, gap: 0.35 };
/** A castor chair that has started to roll (rollTick): where it was last step, how hard it goes (0‥1), when it last really moved, and its voice's serial (0 = none). */
interface Roll { p: Prop; x: number; z: number; k: number; moveT: number; voice: number }
export class Game {
  col: StaticCollision;
  /** a snow level's lying snow as one surface (render/snowField.ts; SNOW_LOOK.on) — PICTURE ONLY: the g-buffer's, and a check's. null on every other level and with the switch off */
  snowField: SnowField | null = null;
  /** … and the prints boots have pressed into it (snowTracks.ts; SNOW_LOOK.tracks) — picture only too; null without the field or with its switch off */
  snowTracks: SnowTracks | null = null;
  private snowLookWas = -1;
  /** what the simulation hands the GPU only when it changed is handed over whole again (host/putback.ts: the GPU's copy is another moment's): the snow's field
   *  and tracks on the next frame, the wind's map of what stands, the dust under the roofs. Here and not in the host file: a host module that imported a game
   *  module by VALUE would be one more way into the game's import cycle, and that cycle does not load from every door. */
  rehand() { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.rehand', this, []); if (__r !== RUST_PASS) return __r as any; } } this.snowLookWas = -1; airResend(this); this.airDust.resend(); }
  /** the field and the prints follow their switches (at the level's start, and again on any frame a switch has moved: a recording's knobs are applied to a game
   *  already built): baked / made the first time they are wanted, handed to the g-buffer, taken back when the switch goes off */
  private syncSnowLook() {
    const on = SNOW_LOOK.on && snowLevel(levelNameOf(this.level)), tracks = on && SNOW_LOOK.tracks;
    if (on) attachSnowBlast(this);   // (a blast over lying snow marks it: snowBlast.ts hears the destruction engine's feed — once an engine, asked each frame)
    const key = (on ? 1 : 0) | (tracks ? 2 : 0); if (key === this.snowLookWas) return;
    this.snowLookWas = key;
    this.snowField = on ? (this.snowField ?? buildSnowField(this.level, { wind: WEATHER_PRESETS.snow.wind })) : null;
    this.snowTracks = tracks && this.snowField ? (this.snowTracks ?? makeSnowTracks()) : null;
    this.engine.gbuffer?.setSnowField?.(this.snowField, snowCfgRows(this.snowField, this.snowTracks ? TRACK_RES : 0));
    this.engine.gbuffer?.setSnowTracks?.(this.snowTracks);
  }
  /** once a frame, after everybody is posed: this frame's footfalls pressed into the snow, and what changed handed to the g-buffer */
  private stepSnowTracks(dt: number) {
    const dug = this.snowField ? stepSnowDigs(this, dt) : null; if (dug) { this.engine.gbuffer?.setSnowFieldRect?.(this.snowField!, dug); }   // (a blast dug a hill out: snowBlast.ts)
    const T = this.snowTracks; if (!T || !this.snowField) return;
    updateSnowTracks(this, T, this.snowField, dt);
    if (SNOW_LOOK.marks) updateSnowMarks(this, T, this.snowField, dt);   // … and every thing that moved over the snow marks it (snowMarks.ts)
    formSnowTracks(T, dt);   // the marks still forming come on, this frame's among them (SNOW_LOOK.marks; nothing forms with it off)
    if (T.dirty) {   // (handed over piece by piece: marks forming a street apart are not one rect round them all)
      const set = this.engine.gbuffer?.setSnowTracks; if (set) { if (T.rects.length > 0 && T.rects.length <= 24) for (const r of T.rects) set.call(this.engine.gbuffer, T, r); else set.call(this.engine.gbuffer, T, T.dirty); }
      T.dirty = null; T.rects.length = 0;
    }
  }
  chars: CharacterRenderer;
  /** THE CAST: each drawn man's own round-cone fit by character id (render/sdfrig.ts RigFit.byOwner — the engine's proxy pack reads it); null with the mannequin loaded */
  rigFits: Map<number, { r0: Float32Array; r1: Float32Array }> | null = null;
  private rigFitOf = new Map<number, { r0: Float32Array; r1: Float32Array } | null>();   // (by part mask)
  /** the mannequins' shadow trees (render/meshShadows.ts — the ultra experiment behind settings.meshShadows): built here next to the skin instances they follow */
  meshShadows: MeshShadows;
  player!: Player; guards: Guard[] = [];
  /** the building's own night people — the cleaner, the late worker (civilians.ts): Guard records with `civ` set, NOT in `guards` (no AI side perceives, targets or
   *  answers for them; the module runs them after the guards each frame). Empty with tune.civilians off or on a level that authors none. */
  civilians: Guard[] = [];
  targets: LightTarget[] = [];
  events: GameEvent[] = [];
  probe: ProbeLike;
  /** THE seam between the GPU and the simulation (sensors.ts): every value the game takes off the renderer — the light meters, the smoke sight lines — is asked
   *  and read through this, by key; live it is the probe verbatim, a recording journals what it delivered, a replay serves that back without a GPU (journal.ts) */
  sensors: Sensors = new Sensors(() => this.probe);
  /** THE SENSES ON THE CPU (sense.ts SENSE.cpu; off = nothing below runs): the game's own smoke, fed by every emitter handed to the solver while the switch is on,
   *  and the solver as it was before the game's tee was put in front of it (null = no tee) */
  cpuSmoke = new CpuSmoke(); private smokeOwned: { solver: SmokeSystemLike; undo: () => void } | null = null;
  /** THE REPLAY BUFFER's tap (replaybuf.ts): called as a step's first statement with the record it is about to run on, and as its last with none — it only reads. null (a game nobody gave a buffer) = nothing */
  replayTap: ((dt: number, cmd?: PlayerCmd) => void) | null = null;
  /** The AI's window on the renderer's light, by key — the same GPU irradiance queries the light meter is made of (1–2 frames' latency, read back by key):
   *  lightQuery asks for the irradiance at a world point under `key`, lightAt returns the latest answer for it (max-axis irradiance, W/m² — what the meter maps
   *  to visibility) or undefined while there is none. Through the sensors by default; the QA harness (whose fake GPU never answers) swaps in a table to test the logic. */
  lightQuery: (key: string, pos: Vec3, skipOwner?: number) => void = (key, pos, skipOwner = 0) => this.sensors.ask(key, pos, skipOwner);   // (skipOwner: a reading INSIDE a man's chest must not be shadowed by his own capsules — the AI targets' meter, guards.ts unitLightMeter, as Sam's passes his id)
  lightAt: (key: string) => number | undefined = key => this.sensors.read(key);
  /** drop a stale answer for `key` (a search plan re-using its keys for new spots): the probe keeps the last value per key for ever otherwise */
  lightForget: (key: string) => void = key => this.sensors.forget(key);
  /** irradiance queries the searching guards queued THIS frame (guards.ts searchProbes keeps them inside a budget well under the probe's 32; the QA soak asserts it) */
  searchProbeN = 0;
  time = 0;
  /** THE GAME'S OWN DIE (math/dice.ts): set by the stand-up when the game was given a seed (the page always is) — every simRandom() draw then comes off it, a checkpoint
   *  carries where it stands, and nothing outside the game can take a number from it. null = the game draws Math.random, as the QA harness's tools expect */
  dice: SimDice | null = null;
  /** THIS GAME'S BLOCK (sim/state.ts): the one run of bytes the families whose state is columns are parts of from the moment the game is made (the falls
   *  below, the furniture, the thrown things, the doors and their slabs: each store is made over it), and where the systems that still move keep theirs
   *  while their row of RUST is on. The rollback snapshot carries it; no other object of the game may keep a typed array over that memory. */
  readonly sim = new SimState();
  /** A MAN'S STATE, AS COLUMNS (sim/men/): his row is his for his life, in this game's block once its first tick has opened. Every Character of this game is made with it */
  readonly men = new MenStore();
  /** THE LEVEL'S LIGHTS (fixtures, the mains, the lamps they feed, the cameras' and tripwires' machines): the `lights` part of the block (rust/sim/src/lights.rs). The one way to write them: `lightHome.in`. */
  readonly lightHome = new LightHome(this.sim);
  /** the pose sets of this game's men that are at home in the block (posehome.ts): what goes back into arrays of its own when the row RUST.poses drops */
  poseSets: PoseSet[] = [];
  /** WHAT FALLS AND WHAT IT LEAVES (sim/fallshome.ts): the landings the simulation has scheduled for things that let go and the litter map of broken glass —
   *  the gameplay half of shards.ts, a part of the game's block */
  readonly falls = new FallStore(this.sim);
  aiEnabled = true; godMode = false; infiniteAmmo = false; showDebug = false; spectate = -1;   // (aiEnabled / godMode / infiniteAmmo are also the START CARD's sandbox row — ui/menu.ts `SANDBOX_LS`, kept per machine and applied by main.ts at boot) // guard index the camera follows (-1 = player); infiniteAmmo: sandbox bottomless magazine, applied to whichever pistol the current Player carries (survives restarts)
  playerInvisible = false;   // showcase tour: guards neither see nor hear the idle player
  /** 0‥1, how close Sam is to being walked in on — for the LOOK of his movement only (fed to player.char.anim.deliberate each frame, see update(); the HUD may
   *  read it too): 1 with a living guard who has not clocked him inside ~3 m, gone by ~7 m; nothing once that man is alert; ≥ 0.6 while he is suspicious or
   *  searching; ≥ 0.5 while a lit torch beam lies across Sam. Nothing the AI, the speeds or the meters read. */
  threatProximity = 0; private torchOnT = -100;   // (game time a beam last lay on him: the torch's share is held 1.5 s past it, so a sweeping beam does not pump the gait)
  aimPoint: Vec3 = [0, 0, 0]; aimTarget: LightTarget | null = null; aimGuard: Guard | null = null; aimFriend: Guard | null = null;   /* (friends.ts: the man of his own side the cursor's ray lands on — never the reticle's with FRIENDS.noSnap; his gun comes off him by FRIENDS.lowReady) */
  dynBoxes: Box[] = [];
  /** How many boxes the game itself made this frame, before the fog of war's rule for loose things rewrote the list (edgeLook.ts budgets by it). */
  dynRaw = 0;
  /** ring airfoils in flight (launcher.ts: fired off the FN2000's launcher, stepped after the player, drawn with the items; a spent one becomes a 'ring' item) */
  rings: Ring[] = [];
  /** 40 mm rounds in flight (he40.ts: the FN2000 launcher's other attachment — stepped and drawn beside the rings; a burst is frag.ts's blast, a dud a 'shell' item) */
  shells: Shell[] = [];
  /** the loadout: does Sam's kit have the FN2000 (docs/internal/sc20k-design.md §11.2 — default in the kit; the start card / pause menu toggle writes it and it is
   *  remembered per machine, localStorage twopointsix.fn2000 — an older machine's twopointsix.sc20k is still honoured). Read whenever a fresh Player is made (boot, restartEncounter); flipping it   (08-12: no longer written or read — the FN2000 is always in the kit; the key survives only in older profiles)
   *  live re-kits the current one. */
  fn2000 = true;
  doors!: Doors;
  /** which side each door opens into and why it was dealt that side (doorswing.ts dealSwings; doors.ts DOORWAY) */
  get doorSwings(): SwingDeal[] { return swingsNow(this.doorDeals); }
  /** …as dealt: each door's row by the room rules, an outside door's open-air deal on its `out` (DOORWAY.exitsOut picks, live) */
  doorDeals: SwingDeal[] = [];
  /** loose furniture (chairs, cartons, bins, plants) shoved around by characters, bullets, canisters and door leaves */
  props: PropSystem;
  /** door the player would operate with F right now (also drives the HUD prompt) */
  useDoor: Door | null = null;   // the hovered door when it is within reach (F acts on it)
  /** the cells under the ceiling slab (render/reach.ts buildInteriorCells): is there a roof over a point (wallpen.ts, wallbreak.ts, furnblast.ts). The lights'
   *  REACH MASKS, which stood here with their queue until 11-05, are the picture's and are kept by it (render/reachKeep.ts: the host's `reach`, none on a stand
   *  with no picture). The game tells it that the level stands (the constructor), where walls changed (wallbreak.ts) and that a sun wants a mask (hours.ts,
   *  lighting.ts), and reads nothing back. */
  reachInterior: Uint8Array | null = null;
  touring = false;   // a show has the floor (written by the Director and Attract) — a plain signal for the HUD / mission getters; quietUtility is NOT it (a stop can run 'live' with the flag down)
  hover: Interactable | null = null; interactables: Interactable[] = []; nearby: Interactable[] = []; aimSnap: Vec3 | null = null;   // aimSnap: where the auto-aim put the reticle this frame (a light or a man's chest), null = raw cursor aim   // hover = the highlighted ROW (what a Space tap / release performs), nearby = the panel's rows (player.ts buildInteractables), interactables = the per-thing markers
  /** The row Space performed THIS frame (interact.ts updateInteract sets it right after the rows are built — a tap released, or Space let go with the selector open —
   *  and update() clears it at the end): each system reads it where it used to read the key itself, so the frame's order of business is what it always was
   *  (updateHold the held man's verbs, updateDoors the doors' / the grab / the body / the breaker, updateMission the drive). null the rest of the time. */
  act: Interactable | null = null;
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
  /** the mains, read */
  get blackout() { return this.lightHome.mains; }
  /** how long a man of theirs has been at the panel, read */
  get breakerFixT() { return this.lightHome.mains.fixT; }
  beacons: { light: RtLight; phase: number; target: LightTarget }[] = [];
  items!: Throwables;
  fx: { box: Box; ttl: number }[] = [];
  sparks = new Sparks(fx.sparks);   // powder streaks + stun-canister fragments (CPU particles → emissive dynamic boxes)
  /** tracer rounds in flight (tracers.ts, lane R6): the carbine's every-third round and its last three drawn as a travelling streak with a small light on the head —
   *  fed by combat.ts fireWeapon (behind tune.tracers), advanced and drawn with the sparks in update(), emptied with them in clearAftermath */
  tracers: Tracers;
  /** decals (experiment E1, render/decals.ts): the marks the encounter leaves — bullet holes, the stun charge's scorch, boot scuffs on kicked leaves, the ring's chips —
   *  fed by marks.ts from the four places things hit things, handed to the engine (whose g-buffer draws them), emptied with the level's own state (resetLevelState) */
  decals: Decals;
  /** the night storm (weather.ts, E10): rain outdoors, lightning through the windows, thunder, the wet lot — on by default (?noweather boots it off; Tab ▸ Weather;
   *  window.__weather) */
  weather: Weather;
  /** what this game was stood with (stood.ts), set once at the stand: the rules read this, never the module tables it was picked from */
  stood: StoodWith;
  /** THE SOLO NIGHT'S CARD (kit.ts kitSolo): your own kit on the plain night's player and on every fresh one */
  soloCard = false;
  fxSeed = 1;               // counting seed for the seeded effects: every shot / can differs, a replayed session repeats
  audio: GameAudioLike = nullAudio;
  ocpCooldown = 0; lastStingT = -100; eventSeq = 0;
  messages: { text: string; t: number }[] = [];
  /** the mission thread (see Mission): reset by restartEncounter(); frozen — neither progressing nor counting — while the showcase tour runs */
  mission: Mission = newMission(0);
  /** the run's chronicle (chronicle.ts): the debrief's timeline, Sam's path, the score's counters — written through the mission's own live gate, replaced with the
   *  mission by restartEncounter(), read by the debrief card (ui/debrief.ts) as the run stood when it ended */
  chronicle: Chronicle = new Chronicle();
  /** the response to going loud (game/qrf.ts): idle until a locked-down floor has confirmed gunfire (or found two men down), then the response team is requested,
   *  called on the radio all the way in, comes through the west stair door as one four-man element and clears toward the last fix — and past that, the compound's
   *  perimeter is locked down round him. One wave a night; reset with the guards (resetGuards / restartEncounter). */
  qrf = new Qrf();
  /** the level-design devices (game/devices.ts): the level's cameras, keypads and tripwires — built after the doors (a keypad hangs on one) and before the
   *  authored switch states are captured (a camera / an emitter is a light target like any fixture: OCP, a round, the mains) */
  devices!: Devices;
  /** DESTRUCTION (game/destruction.ts, STATUS stage 3b): what sustained fire does to the office behind `tune.chaos` — paper stacks turned to fluttering sheets, dust puffs into the
   *  smoke solver, damage decals, dying screens, desk clutter and ceiling tiles as capped rigid debris (game/debris.ts); built after the targets and devices (its index reads them),
   *  ticked after the props, its chunks in the frame's dynamic boxes, reset with the level */
  destr!: Destruction;
  /** WIND (game/wind.ts): the building's air as one analytic field — the level's HVAC vents, the storm outdoors, the gust through an open exterior door — handed each frame
   *  to the GPU particles (paper trembles and slides under a diffuser, dust rides it) AND the smoke solver (a plume under a vent bends; the guards' sight lines march that
   *  same density); built after the doors, ticked after the weather; `tune.wind` 0 = still air, the game as it was */
  wind!: Wind;
  /** THE WEATHER'S DUST IN THE ROOMS (game/airDust.ts): a picked weather's dust / fog carried in under the roofs at open doors and blown walls — one field over the plan
   *  for the picture (the volumetric pass) and the men's eyes (weather.ts airTrans); built after the doors, ticked after whatever breaks; no weather picked = inert */
  airDust!: AirDust;
  /** MOTHS (game/moths.ts): a dozen fluttering specks round the lit sodium lamps at night; ticked after the wind, their boxes into the frame's dynamics */
  moths!: Moths;
  /** WET GROUND (game/wetGround.ts): the open ground a picked rain wets, as the picture draws it — dressing, ticked in the picture's block */
  wetGround = new WetGround();
  /** GLASS BREAKING (game/glassbreak.ts): the panes' impact lists / Voronoi cells / dropped masks and the shards they become — always constructed (its hooks answer 'whole' for
   *  a pane never hit), inert unless tune.glassBreak (default off) AND chaos > 0 outside the tour; the renderer's half is the boot-time prelude const GLASS_BREAK */
  glassBreak!: GlassBreak;
  /** BROKEN GLASS AFTER IT LETS GO (game/shards.ts): each released fragment as rigid plates that break again on landing + 'shard' GPU particles (mids, fines, back spray,
   *  tempered dice), its landings heard, its litter crunching underfoot — inert unless tune.glassShards (default on) AND glass breaking is live */
  shards!: GlassShards;
  /** THE DIRECTOR (game/director.ts, docs/design/level-scripting.md M1): the level's script — its programs, triggers and generator beats on the game clock, the showcase
   *  plumbing — built last in the constructor from level.script (none = no programs, as on every shipped level: the game exactly), ticked in update() right
   *  before tacticsTick, read by main.ts through `frame` (the pinned look-at, caption, veil) while a showcase program has the frame */
  director!: Director;
  /** THE HOST'S OWN CAMERA AND KEYS, handed to the game ONCE, at its stand, by every host alike (host/standup.ts standGame). They are the host's: what a player's machine draws through and types on, never part of
   *  what two machines of one game compare (the wire's hosts-own list). The game names them for the three things that reach for them: a level
   *  script's show (director.ts: its pin, its lock of the keys), a rails level's first view, and the view's matrix between two ticks. Before this the
   *  Director kept them and `update` handed them over again every frame, so a host that stepped by `update` held two rows more than one that stepped by
   *  `stepSeats`, and two machines of one game numbered their cameras differently. */
  hostCam: FollowCamera | null = null; hostInput: Input | null = null;
  /** the stand's one call (standGame). A tick is still SAMPLED from whatever camera and Input `update` is handed (a replay's player steps with the tape's
   *  own): these two are what the HOST shows through and types on, which is what a show pins and locks */
  hostHands(cam: FollowCamera, input: Input): void { this.hostCam = cam; this.hostInput = input; }
  /** the night breach playtest (nightbreach.ts; ?nightbreach / native --nightbreach / __nightbreach): Sam leading two AI operatives against three men barricaded in the
   *  break room. Off — the game exactly — until its start(); ticked right after the Director */
  nightBreach!: NightBreach;
  /** the quiet mission (quiet.ts): the level's own night with a thing to take and ways out, off the level's data — inert until started */
  quiet!: QuietMission;
  /** a training lesson's steps (lesson.ts): reads the game, writes nothing of it; inert on a level without the block */
  lesson!: Lesson;
  /** a tutorial mission's frame over the lesson (rails.ts): locked actions, cues to the level's script, authored checkpoints, the sky seat; inert on a level without a lesson.rails record */
  rails!: Rails;
  /** a room clear against a clock: the Rust game's record, written by the bridge (par.ts is how the page reads it); blank on a level without a scenarios.par block */
  par!: Par;
  /** the squad's men lost, as the squad knows them: the game's record (rust/game_rules/src/deathrecap), read by the HUD */
  recap: { lostLive: LostMan[]; allDown: boolean } = { lostLive: [], allDown: false };
  /** how the run went, said once it is over (verdict.ts): reads only — the end card's record */
  verdict!: Verdict;
  /** a defuse mission's charges and their one clock (defuse.ts): an overlay on a room clear; inert on a level without a scenarios.defuse block */
  defuse!: Defuse;
  /** the room clear (roomclear.ts; ?roomclear / native --roomclear / __roomclear): the night breach on a structure of four held rooms, with per-room cleared state
   *  and two ends. Off until its start(); ticked right after the night breach */
  roomClear!: RoomClearScenario;
  /** Sam's live orders to the operatives on his side (orders.ts; __orders): the command menu (X / the middle button: up until a row is picked), B the quick order, T who takes them — inert with no operative on the floor */
  orders!: SquadOrders;
  /** FOG OF WAR (fogofwar.ts): what nobody on Sam's team sees is left out of the picture — inert until its switch is on */
  fogOfWar!: FogOfWar;
  /** the room fight (roomfight.ts): hostile and operative units perceiving and fighting on their own read — inert while none stands */
  fight!: RoomFight;
  /** what the defenders decide, told to the player: the radio strip's rows and the cue lines (cues.ts — it only reads the game) */
  cues!: Cues;
  /** the squad's own lines where the player cannot see the man who says them, and a teammate down (squadnet.ts — it only reads the game) */
  squadNet!: SquadNet;
  /** unasked cover (cover.ts): the openings round the squad, dealt to the men who follow or hold with nothing ordered to watch */
  cover!: Cover;
  /** entry points off uncleared openings (entrypoints.ts): which side of which opening the squad has been on, and the door tools' points moved off the rest */
  entry!: EntryPoints;
  /** which rooms the team has been in (cleared.ts): the way an ordered walk takes */
  cleared!: ClearedRooms;
  /** the hostage rescue playtest (rescue.ts; ?rescue / native --rescue / __rescue): two hostages in the manager's office, a captor on a clock, Sam and the pair
   *  flowing down the corridor past rooms they have not cleared — inert (tick returns on its first line) until started */
  rescue!: Rescue;
  /** the squad's door tools (doortools.ts; __doortools): open / bang / charge / clear-from-outside rows on the command menu, the per-run counts — inert with no operative on the floor */
  doorTools!: DoorTools;
  /** quiet orders for one partner (pairorders.ts): a person as a target, 'take him on your mark', 'hold in the dark' — inert with its switch off */
  pair!: PairOrders;
  work!: WorkThings;   // things a teammate can be sent to work (workthings.ts)
  wallCharge!: WallCharge;   // the door charge's twin on a partition wall (wallcharge.ts)
  /** each man of a room entry on a real path through the door (doorpath.ts; DOORPATH.on — off: nothing of it runs) */
  doorPath = new DoorPath(this);
  /** checkpoints (game/savestate.ts, STATUS G5): the autosave triggers + gates ticked at the end of update(), the slots in localStorage (the harness hands it a
   *  Map), the cards' 'restart from checkpoint', export / import — saveState / loadState themselves are plain functions over the Game */
  saves = new Checkpoints();
  rackTargets: LightTarget[] = [];   // the mission rack's LED / glow switchables: forced dark once the drive is out
  /** the analytic lights registered for emissive MARKINGS at construction (the vending windows): for the panel / __mark.lights / the probes */
  markLights: { target: string; box: number; face: string; pos: Vec3; I0: number; area: number; meanL: Vec3 }[] = [];
  tune = { guardWalk: 1.15, guardInvestigate: 1.75, guardChase: 2.7, playerWalk: 1.55, playerSprint: 4.7, playerCrouch: 1.3, playerLight: 60, flashIntensity: 70, flashInner: 7, flashOuter: 19, detectRate: 1.6, ocpDuration: 9, ocpNotice: 'glance' as 'glance' | 'investigate', visSlope: 0.2, visFloor: 0.03,
    floorMaterials: true,       // floorMaterials: footsteps take the floor they land on into account (floor.ts: carpet quieter, stone louder — for the mixer, the guards' ears and the meter alike); off = every floor is lino, the A/B seam
    muzzleDiscipline: true,     // muzzleDiscipline: while Sam holds a living man nobody who can see it fires — weapons up, talking, circling, never a round until the shield is gone (guards.ts standoffAlert); off = the flank rule fires as before, the A/B seam
    suppression: true,          // suppression (suppression.ts, docs/internal/sc20k-design.md §4): rounds close to a man put his head down — the guards flinch, shoot worse and later, crouch behind what they stand at and shift to cover under sustained fire; Sam's own rounds open up under theirs. Off = the scalar is still integrated and printed, nobody reacts to it and Sam's rounds go where they are sent (the A/B seam; with no rounds in the air nothing differs either way)
    carbineGuards: 'none' as CarbineDeal,   // rifles (docs/internal/sc20k-design.md §3): who carries the AKS-74U-style carbine instead of torch + sidearm — 'none' = today's floor exactly, 'reyes' = the lobby / break-room man, 'all'. The deal is the weapon (update(): Guard.carbine + Guard.weapon swapped to the aks74u row and back): short bursts, thirty to a box, the fireball, the rifle tumbling out of a dying hand; read through guards.ts rigOf / burstTrigger
    suppressFirst: true,        // rifles: a rifleman's OPENING burst at a Sam ten metres or more off is laid the better part of a metre beside him — into the partition, the jamb, the wall — not on him: the fairness valve of a one-shot game meeting automatic fire, and what trained men do (fix, then work); every burst after it is aimed. Off = aimed from the first (guards.ts burstTrigger)
    tracers: true,              // rifles (lane R6, tracers.ts): the carbine's every-third round and the last three of its box are tracers — a streak travelling muzzle → impact over a few frames with a small warm light on its head (the wall it passes, the haze it crosses), an ember where it stops in geometry. Off = combat.ts fireWeapon never calls the tracer code: the round is exactly what it was (the A/B seam; pistols have none either way)
    aimModel: true,             // the aim-settle model (aim.ts, playtest 08-08 'guards have really good aim'): both sides take time to line up a shot (a guard's first round waits ~0.7 s of track after his reaction beat and his early rounds go wide), pay for a target crossing their view and pay more for their own feet moving — a closing guard brakes to a standstill for each shot, Sam's ring blooms while he moves and tightens as he holds still. Its numbers: AIM in aim.ts / Tab ▸ Sandbox ▸ aim rows (kept per machine). Off = every round exactly what it was: the guards' rows in fireWeapon, Sam dead-on (the A/B seam; tools/qa weapons-identity is frozen on it)
    teams: true,                // sides for the AI (teams.ts / guards.ts spawnUnit · engage — the 'Breach' showcase's operatives clearing a room of hostiles): on by default because with no unit spawned nothing of it runs — the staff's foe is Sam by construction and every seeded QA run is byte for byte the game it was (tools/qa teams-identity); off = spawnUnit refuses, so no unit can ever exist: the kill switch, not an A/B
    needs: true,                // needs-driven patrol variation (needs.ts, STATUS E12 c): each man on nights carries two or three slow needs — coffee, a smoke, water, a stretch — that take him off his route for a fixed, telegraphed spell (watch check + glance 20 s out, said to his partner or the net at 12, neck rub + turn at 5, then the walk, the dwell, the walk back, 'right, where was I'); one man away at a time, calm floor only, broken off by anything that reaches him. Its numbers: NEEDS in needs.ts / Tab ▸ Sandbox ▸ needs rows (kept per machine with this switch). Off = the patrol exactly as it was (tools/qa needs-identity)
    tactics: true, flankWait: 6, loneWait: 8, doorDanger: 25, doorWait: 6,
    chaos: 0.5,                 // DESTRUCTION (destruction.ts): 0 = the game byte for byte as it was; up to 1 = how readily sustained fire scatters paper, raises dust, knocks things off desks and drops ceiling tiles — every path capped whatever it says
    wind: 1,                    // WIND (wind.ts): the master 0‥1 on every source of moving air — the vents, the storm outdoors, the gust through an open exterior door — that the particles and the smoke solver both take; 0 = still air, the game byte for byte as it was (kept per machine with the wind rows: loadWind; tools/qa wind-identity)
    moths: true,                // MOTHS (moths.ts): moths round the lit sodium lamps at night (kept with the wind rows)
    glassBreak: true,           // GLASS BREAKING (glassbreak.ts): panes crack and come apart where the rounds went — on (under glass.ts GLASSFX, the one switch over all of it; the QA harness stands it off: src/host/standup.ts); the GPU side follows at boot (main.ts → the prelude const), so a change wants a page reload to show
    glassShards: true,          // BROKEN GLASS AS SHARDS (shards.ts): what a released fragment becomes — plates + shard particles + landing sounds + litter underfoot; only ever acts while glass breaking is live, so with glassBreak off it changes nothing; off = the base's rigid shard boxes
    wallBreak: true,            // WALL BREAKING (wallbreak.ts): partitions holed / ported and cover chewed away under rounds and blasts — live only with THE ONE SWITCH on (destructionSwitch.ts) and chaos > 0
    debrisImpulses: true,       // DESTRUCTION: what already lies about is kicked again by what happens next — a stun charge throws the settled leaves back up, a round beside them scoots them, a body / boot / shoved desk twitches them (destruction.ts impulses → the GPU particles + the debris pool); off = the lane before it, byte for byte (kept with chaos)   // pursuit tactics (tactics.ts, STATUS E12 a+b): two men on one fix flank it — a presser stops short at his last corner, a flanker comes round from ≥ 90° the other way, 'set' / 'go' on the net — a lone man on an armed intruder holds an angle and calls it in instead of charging, and a doorway a colleague went down in (or rounds came through) is a kill zone nobody walks into: paths go round it, else stack, call, wait for a second man, bang or slice the pie, cross fast. flankWait: the presser's patience at his corner; loneWait: the lone man's; doorDanger: how long a doorway stays a kill zone; doorWait: how long the man on the jamb waits for his second. off = the chase exactly as it was (tools/qa tactics-identity)
    // the response to going loud (game/qrf.ts, Tab ▸ Sandbox ▸ QRF rows — remembered per machine): qrf = the response team comes at all; qrfEta = seconds from the request to
    // the stair door going (called on the radio at the request, at thirty and at ten); qrfSize = men in the element (2‥4, capped by the renderer's character budget);
    // perimeter = the second tier — the compound locked down round him — at all; perimeterEta = its telegraph; perimeterSize = men on the outside posts (2‥4, spawned
    // if the budget has room, else peeled off the element leaving two inside)
    qrf: true, qrfEta: 75, qrfSize: 4, perimeter: true, perimeterEta: 45, perimeterSize: 2,
    // qrfSilent = the silent net (qrf.ts silentNet): the whole shift off the net after your gunfire was heard on it also brings the team — control's radio check first
    qrfSilent: true,
    // qrfSquad = the response team takes a room by the squad's own door and room code (qrf.ts startDrill, squadcast.ts): off = its own drill on the level's marks, as it was
    qrfSquad: true,
    // civilians (civilians.ts, Tab ▸ Sandbox ▸ civilians rows — remembered per machine): civilians = the level's cleaner / late worker are on the floor at all (off = nobody but
    // the shift: today's game byte for byte); civilianKillFails = a civilian killed by Sam's hand fails the mission there and then (off: it costs him forty points, a
    // 'civilian casualty' on the card and control's fury on the net — Chaos Theory fails some missions for it, this one prices it)
    civilians: true, civilianKillFails: false,
    // bodies found by the LIGHT on them (guards.ts bodyInView / bodyLit — review A6.2, Chaos Theory's rule; : a body in view
    // is found only when the light where it lies reads high enough on the meter's own scale (or he is inside two metres of it), and not through a plume; off = range, cone and line alone, as before
    bodyLight: true,
    // the level-design devices (game/devices.ts, Tab ▸ Sandbox ▸ devices rows — remembered per machine): devices = cameras and tripwires sense at all (off = they are
    // drawn, pan and glow but report nothing; keypads stay keypads); lasers = the tripwires alone; cameraSuspicionSecs = lit-seconds in a camera's picture before
    // control radios it and a man is sent (a fully lit man close to fills a second a second; half-lit at range, a fifth of that); cameraAlarmSecs = lit-seconds held
    // in view before the floor goes up a step; deviceCones = each live camera throws a faint red cone into the haze (a light slot apiece)
    ...DEVICE_TUNE_DEFAULTS };

  constructor(public engine: HostApi, public level: Level, public ch: GltfCharacter, public smoke: SmokeSystemLike, opts: { seats?: number; stood?: StoodWith } = {}) { RUST_SURFACE?.newGame();
    this.stood = { ...(opts.stood ?? stoodOfPick()) };   // (its own copy. First: the Weather made below is born in it)
    { const over = levelOver(level); if (over) throw new Error(over); }   // (levelfit.ts: a level the pools cannot hold is not loaded, said before anything of it is built)
    this.seats = seatsStand(this, opts.seats ?? 1); this.localSeat = this.seats[0];   // (seats.ts: before anything is built over a seat's view; one seat = the Game is its own seat's view and nothing below differs)
    const first = this.seats[0].view;   // (the first seat's view: this Game, in a game of one)
    qrfAddressWord(this);   // (?qrfsquad=0: qrf.ts)
    leversFog(); fogFlags(); irFlags();           // (before the squad's systems read them)
    this.tune.chaos = CHAOS.def;
    loadWind(this.tune);      // this machine's wind (wind.ts, the Weather page's rows; the moths' switch rides with it)
    loadCivTune(this);      // (?nocivilians: civilians.ts)
    // (the FN2000 is always in the kit now — no per-machine loadout memory; Game.fn2000 / setLoadout stay for the harness)
    this.col = new StaticCollision(level.boxes, levelNameOf(level));
    if (engine.gbuffer?.setWearMap) { const wm = buildWearMap(level, this.col.nav, lookConfig.longFoot > 0, lookConfig.walked > 0); engine.gbuffer.setWearMap(wm.data, wm.w, wm.h); }   // the detail layer's traffic map (render/wear.ts: the routes over the nav grid + the thresholds, the walls' feet) — the harness's engine stand-in has no g-buffer to hand it to
    this.syncSnowLook();   // a snow level's lying snow as one surface (render/snowField.ts; picture only — nothing below reads it)
    this.chars = new CharacterRenderer(engine.device, ch);
    engine.skin = this.chars;
    this.meshShadows = new MeshShadows(engine.device, ch, this.chars.jointMats, this.chars.charInsts);   // (the QA harness's fake device takes this too: the tree build is the real code, the buffers go nowhere)
    engine.setMeshShadows?.(this.meshShadows);                                     // (?.: the harness's engine stand-in has no renderer to attach to)
    try { const fit = fitRig(ch); if (ch.cast) { this.rigFits = new Map(); engine.setRigFit?.({ ...fit, byOwner: this.rigFits }); fitRigCast(ch); } else engine.setRigFit?.(fit); } catch (e) { console.warn('SDF rig: the round-cone fit failed, characters trace as untapered cones with hand radii', e); engine.setRigFit?.(null); }                                                 // the SDF character rig's radius fit (render/sdfrig.ts; the round cones are the proxy at every tier; a failed fit only loses the taper)
    if (engine.setParticles) { engine.setParticles(new Particles(engine.device, engine.sceneLayout)); applyDustLook((engine as { particles?: Particles | null }).particles ?? null); }   // GPU particles (experiment E4): only on a real Engine — the harness's stand-in has no setParticles, so every effect site there keeps to the CPU sparks (particleFx.ts gpuFx); the dust LOOK table onto the kind whether or not the switch is on yet
    attachAir(this);   // THE AIR AS A SIMULATED FIELD (airSim.ts, render/airField.ts): the page's engine gets its field once (a real device with the switch up; the harness none)
    this.probe = engine.probe?.() ?? new IrradianceProbe(engine.device, engine.sceneLayout);   // (a host that owns its probe — the native one over the shared query / answer tables — hands it here; else the WebGPU kernel)
    first.player = new Player(this.men, ch, level.playerSpawn, this.fn2000);
    // loose furniture: simulated on the CPU, emitted as dynamic boxes every frame (rendered + traced like everything else), deliberately never registered as
    // collision occluders — a chair must not hide the player, muffle a shot or dent the nav grid; guards path through props and shove them aside
    this.props = new PropSystem(level.props, this.col, (e) => this.onPropEvent(e), this.sim);
    this.items = new Throwables(this.col, this.props, this.sim); this.items.bog = { may: () => snowBogMay(this), at: (x, y, z) => snowBogAt(this, x, y, z), kinds: () => snowBogKinds(this), roofs: () => this.weather.roofsNow() };   // (… and bog down where they land in lying snow: snowBog.ts, 0 everywhere with its switch off)
      // canisters and dropped magazines bounce off (and nudge) the furniture
    this.tracers = new Tracers(engine.lights, sparkSink(this)); this.tracers.fog = { begin: who => this.fogOfWar.begin(who), done: was => this.fogOfWar.done(was), lightBy: (l, who) => this.fogOfWar.lightBy(l, who) };   // tracer streaks: their head lights live in the engine's LightSet (transient), their embers are sparks — GPU sparks while that path is on, the CPU pool's otherwise (particleFx.ts sparkSink)
    this.decals = new Decals(engine.device); engine.setDecals?.(this.decals);   // (the QA harness's engine stand-in has no renderer to hand it to: the list still fills, nothing draws)
    if (engine.settings && engine.settings.decals === false) this.decals.enabled = false;
    this.weather = new Weather(this);
    first.player.light = engine.lights.add(railLightDef(PLAYER_ID, this.tune.playerLight));
    // light targets from analytic lights + switchables
    // contactor groups restrike one after another when the mains return: stagger by order of first appearance
    const groupStagger = new Map<string, number>(); const staggerOf = (grp: string) => { if (!groupStagger.has(grp)) groupStagger.set(grp, groupStagger.size * 0.45 + simRandom() * 0.5); return groupStagger.get(grp)!; };
    const lightDefs = new Map(level.lights.map(d => [d.name, d]));
    for (const l of engine.lights.lights) {
      if (l.ttl >= 0 || l.kind === 2 || l.group === 'player' || l.group === 'flashlights') continue;
      const mains = lightDefs.get(l.name)?.mains !== false;
      this.targets.push(this.lightHome.add({ name: l.name, group: l.group, kind: 'analytic', pos: v3.copy(l.pos), light: l, boxes: l.fixtureBox >= 0 ? [l.fixtureBox] : [], baseEmissive: l.fixtureBox >= 0 ? v3.copy(level.boxes[l.fixtureBox].emissive) : [0, 0, 0], baseIntensity: l.intensity, on: l.enabled, broken: false, disabledUntil: -1, fluorescentFlicker: false, factor: 1, mains, stagger: staggerOf(l.group), interactive: true }));
    }
    for (const sw of level.switchables) {
      const b = level.boxes[sw.boxes[0]];
      this.targets.push(this.lightHome.add({ name: sw.name, group: sw.group, kind: 'emissive', pos: v3.copy(b.c), boxes: sw.boxes, baseEmissive: v3.copy(sw.baseEmissive), baseIntensity: 0, on: sw.on, broken: false, disabledUntil: -1, fluorescentFlicker: !!sw.flicker, factor: sw.on ? 1 : 0, mains: sw.mains !== false, stagger: staggerOf(sw.group), interactive: sw.group !== 'server_leds' && sw.group !== 'server_glow' }));
    }
    for (const t of carLampTargets(level.boxes)) this.targets.push(this.lightHome.add(t));   // (carlamps.ts: the lamps of the level's cars and pickups, one target each — no die drawn, none on the mains)
    // flat emissive fixtures (ceiling panels, monitors, logo, TV, vending fronts, exit signs) light the scene through analytic one-sided area
    // lights; their boxes keep glowing on screen but rays ignore the emissive term (BoxFlag.AreaLit) — kills small-emitter aliasing in the cascades
    for (const t of this.targets) {
      if (t.kind !== 'emissive') continue;
      for (const bi of t.boxes) {
        const b = level.boxes[bi]; const h = b.h, panel = flatPanel(t.group, h); if (!panel) continue;
        const { thin, a0, a1, area } = panel;
        // emitting side: ceiling fixtures face down; wall/desk fixtures face whichever side is open (props are authored with the glowing face on
        // local +X/+Z of their backing, which is also the fallback when both sides look open)
        let n: Vec3 = [0, 0, 0];
        if (thin === 1) n = [0, b.c[1] > 1.5 ? -1 : 1, 0];
        else { const cs = Math.cos(b.yaw), sn = Math.sin(b.yaw); const ax: Vec3 = thin === 0 ? [cs, 0, -sn] : [sn, 0, cs];   // box local X / Z in world
          const front = !this.col.segmentBlocked(b.c, v3.mad(b.c, ax, h[thin] + 0.35)), back = !this.col.segmentBlocked(b.c, v3.mad(b.c, ax, -(h[thin] + 0.35)));
          n = front && !back ? ax : !front && back ? v3.scale(ax, -1) : ax; }
        const e = t.baseEmissive; const maxc = Math.max(e[0], e[1], e[2], 1e-3); const I0 = maxc * area;   // radiance × area = on-axis intensity
        const light = engine.lights.add({ name: `${t.name}_area${bi}`, group: t.group, kind: 3, pos: v3.mad(b.c, n, h[thin] + 0.07), dir: n,   /* 7 cm proud of the glowing face: shadow rays stop 3 cm short of their sample (direct.wgsl shadowSample tmax), and at the old 3 cm the ray's end sat exactly ON the fixture's own box — a floating-point coin-flip that occluded whole regions of samples and drew hard arcs across a dark floor (the break-room vending fronts, 08-08) */ color: [e[0] / maxc, e[1] / maxc, e[2] / maxc], intensity: I0,
          range: clamp(Math.sqrt(I0 / 0.0015), 3, 26), innerDeg: 0, outerDeg: 90,   /* range = where the panel has fallen to ~nothing (0.0015 W/m²): the old 8 m cap cut a visible circle into dark neighbouring rooms; the scoring loop is analytic so the extra reach costs no rays */ radius: Math.max(0.1, Math.min(Math.sqrt(area / Math.PI), Math.min(h[a0], h[a1]) + 0.02))   /* the emitter disc never wider than the fixture's own short side: an equal-AREA disc overhangs a rectangle's short edges, and an overhang that pokes into the wall / cabinet beside the fixture is a slice of permanently 'occluded' samples (dimming + grain everywhere the light reaches) */, volumetric: 0.0, /* diffuse panels: no haze beams (cost) */ owner: 0, enabled: t.on, fixtureBox: -1, flicker: 0, ttl: -1, age: 0, peakIntensity: I0, broad: true });
        (t.areaLights ??= []).push({ light, base: I0 });
        b.flags |= BoxFlag.AreaLit;
      }
    }
    // EMISSIVE MARKINGS ARE LIGHTS (scene/markings.ts; the one-model rule): a switchable box's marking with `emit` — the vending machines' product windows — registers a one-sided
    // rectangular emitter over its rect exactly as an emissive fixture box does above (radiance × area = on-axis intensity, the disc never wider than the rect's short side, 7 cm proud
    // of the face), rides the same target's supply (t.areaLights → lighting.ts; the drawn emission follows Box.markGain), and is marked area-lit so a reflection ray takes only
    // emitTraceK of its traced radiance (the analytic image carries the rest — one window either way). LED rows and UI bars are too small / dim to light anything (< 0.01 m² or an
    // intensity under MARK_LIGHT_MIN_I0): they glow on screen and in mirrors only, as the raster-only LED nubs they replace did. Skipped with markings off at boot (?nomarkings).
    if (engine.settings?.markings !== false) for (const t of this.targets) {
      if (t.kind !== 'emissive' || t.group === 'emergency') continue;
      for (const bi of t.boxes) {
        const b = level.boxes[bi]; if (!b.marks?.length || b.rot) continue;
        const mkTexel = engine.settings?.markingTexel ?? MARKING_TEXEL;   // (the level's base texel: main.ts hands Level.markTexel to the settings before the Game is built)
        b.marks.forEach((m, k) => {
          const st = markStats(m, b.albedo, mkTexel);
          if (!st.emask) return;
          const maxc = Math.max(st.meanL[0], st.meanL[1], st.meanL[2]);
          const I0 = maxc * st.area;                                        // radiance × area = on-axis intensity, as for the fixture boxes above
          if (st.area < 0.01 || I0 < MARK_LIGHT_MIN_I0) return;             // LED rows, UI bars, buttons: they glow, they light nothing (the dimmest fixture light on the floor is an exit sign at ≈ 0.46)
          const R = markWorldRect(m, b.c, b.h, b.yaw, mkTexel); const n = R.n;
          const light = engine.lights.add({ name: `${t.name}_mark${bi}_${k}`, group: t.group, kind: 3, pos: v3.mad(R.centre, n, 0.07), dir: n, color: [st.meanL[0] / maxc, st.meanL[1] / maxc, st.meanL[2] / maxc], intensity: I0,
            range: clamp(Math.sqrt(I0 / 0.0015), 3, 26), innerDeg: 0, outerDeg: 90, radius: Math.max(0.1, Math.min(Math.sqrt(st.area / Math.PI), Math.min(R.hu, R.hv) + 0.02)), volumetric: 0.0, owner: 0, enabled: t.on, fixtureBox: -1, flicker: 0, ttl: -1, age: 0, peakIntensity: I0, broad: true, markRef: bi < 4096 && k < 4 ? (bi << 2) | k : undefined });   // (markRef: the reflection pass textures this light's analytic highlight with the window's own pattern)
          (t.areaLights ??= []).push({ light, base: I0 });
          AREA_LIT_MARKS.add(m); this.markLights.push({ target: t.name, box: bi, face: m.face, pos: v3.mad(R.centre, n, 0.07), I0, area: st.area, meanL: st.meanL });
        });
      }
    }
    if (this.markLights.length) engine.world.repackMarkings?.();   // (the area-lit bit reaches records packed before the lights existed — the harness's stand-in world has no table)
    for (const b of level.beacons) level.boxes[b.box].flags |= BoxFlag.AreaLit;   // dome glow is screen-only; the beam is the spot light
    // mains breaker (OCP → temporary blackout, bullet → permanent) and the emergency beacons it brings up (each beacon light is slaved to its dome fixture)
    this.targets.push(this.lightHome.add({ name: 'breaker', group: 'mains', kind: 'breaker', pos: v3.copy(level.breaker.pos), boxes: [level.breaker.boxes[1]], baseEmissive: [0.3, 4, 0.6], baseIntensity: 0, on: true, broken: false, disabledUntil: -1, fluorescentFlicker: false, factor: 1, mains: false, stagger: 0, interactive: true }));
    level.beacons.forEach((b, i) => {
      const target = this.targets.find(t => t.kind === 'emissive' && t.boxes.includes(b.box));
      if (!target) throw new Error(`beacon ${i}: no switchable owns box ${b.box}`);
      const light = engine.lights.add({ name: target.name, group: target.group, kind: 1, pos: v3.copy(b.pos), dir: [1, 0, 0], color: [1.0, 0.22, 0.06], intensity: 55, range: 14, innerDeg: 9, outerDeg: 24, radius: 0.06, volumetric: 1.0, owner: 0, enabled: false, fixtureBox: -1, flicker: 0, ttl: -1, age: 0, peakIntensity: 55 });
      this.beacons.push({ light, phase: i * 1.7, target });
    });
    // the cells under a roof (the game's own rules read them), and the lights' reach masks, made where there is a picture to read them (render/reachKeep.ts boot)
    this.reachInterior = buildInteriorCells(engine.world.statics);
    engine.reach?.boot(level.doors.map(d => ({ hinge: d.hinge, closedDir: d.closedDir, width: d.width })), this.reachInterior);
    // doors: leaves are dynamic occluders for sight / sound / projectiles (collision layer)
    this.doors = new Doors(level.doors.map(d => ({ ...d, albedo: d.exterior ? [0.2, 0.22, 0.25] as Vec3 : undefined })), (e) => this.onDoorEvent(e), this.sim);
    this.props.doorways = this.doors.list.map(d => ({ cx: d.frameCentre[0], cz: d.frameCentre[2], ux: Math.cos(d.def.closedDir), uz: Math.sin(d.def.closedDir), half: d.def.width / 2 - 0.02 }));   // (a door frame guides shoved furniture through: props.ts doorways — half the air between the frame's stops)
    nameDoors(level, dialogue.isOffice(level), this.doors.list);   // what a door with a shorthand id is CALLED, by the rooms it joins (doornames.ts; read only while the dialogue pass is on)
    this.doorDeals = dealSwings(level, this.doors, this.col.nav, this.col);   // the side each leaf opens into (doors.ts DOORWAY; doorswing.ts: into the room it serves) — geometry only, no dice
    this.doors.staged = () => this.touring;   // (DOORSTAY: a showcase's choreography keeps the closers it was written against)
    this.col.dynamicBoxes = this.doors.leafBoxes();
    this.doors.man = who => doorMan(this, who);   // (doorthrow.ts: whose man, folded, braced, calm)
    this.doors.standable = (x, z, r) => { const q: Vec3 = [x, 0, z]; this.col.collideCircle(q, Math.min(r, 0.3), 0.2, 1.5, 1); return Math.hypot(q[0] - x, q[2] - z) < 0.02 && !this.col.clampToWorld(q, 0.3); };   // (doors.ts DOORTHROW: floor for a body a leaf would carry there — the walkers' own radius against the statics)
    this.doors.settle = p => settleSpot(this, p);   // (doors.ts DOORWAY.solid: where the statics let a man a pulled leaf moved stand)
    this.doors.host = { col: this.col, standable: this.doors.standable, settle: this.doors.settle, settleY1: () => this.seats[0].view.player.crouch ? 1.0 : 1.7 };   // (the doors' step in the Rust module — RUST.doors — asks the statics as these two do: doors.ts DoorsHost; the span's head is settleSpot's own, of the first seat's man)
    this.doors.blaster = (at, tearM, who) => doorBlast(this, at, tearM, who);   // the doors' side of a blast (doorblast.ts): a leaf inside its reach comes off its hinges
    // …and where the build carries the destruction goal's change feed, a torn leaf is a row in it (kind 'door': ONE feed of what the level no longer is)
    this.doors.onTorn((d, who, cause) => {
      const c = d.frameCentre, r = d.def.width / 2 + 0.1;
      (this.destr as unknown as { walls?: { note?: (row: object) => unknown } }).walls?.note?.({ kind: 'door', cause, id: this.doors.list.indexOf(d), name: d.def.name,
        pos: [c[0], c[1], c[2]], rect: [c[0] - r, c[2] - r, c[0] + r, c[2] + r], opened: [], walkable: true, sight: true, dead: true, who });
    });
    this.devices = new Devices(this);   // cameras + emitters join `targets` (before the authored switch states are said, below); a keypad finds its door
    // guards
    const tints: Vec3[] = [[0.42, 0.36, 0.25], [0.3, 0.34, 0.4], [0.38, 0.28, 0.28]];
    level.routes.forEach((r, i) => {
      const fl = engine.lights.add(torchLightDef(`flashlight${i}`, 10 + i, this.tune.flashIntensity, this.tune.flashInner, this.tune.flashOuter));
      this.guards.push(new Guard(this.men, ch, 10 + i, r, i, fl, tints[i % tints.length]));
    });
    openingStart(this);   // the night's opening (opening.ts): a man whose route starts on top of the way in is found down his beat instead — nothing with OPENING.on off
    { const n = level.mission.rack.index; this.rackTargets = this.targets.filter(t => t.name === `rack${n}_leds` || t.name === `rack${n}_glow`); }   // what goes dark when the drive comes out (level.ts "server room")
    this.destr = new Destruction(this);   // the destructible statics indexed by shape (paper stacks, screens + housings, desk clutter) — after the targets and devices exist: a lit screen dies through its target
    this.glassBreak = new GlassBreak(this);   // glass breaking (glassbreak.ts): attaches its cell-table / open-cell / walkable hooks to the world and the collision; its shards go into destr's debris pool — inert until tune.glassBreak
    this.col.glassPen = { live: () => glassPenLive(this), passes: (a, rd, panes, energy) => shotPasses(this, a, rd, panes, energy) };   // rounds through glass (glasspen.ts): the line of FIRE asks the round's own toll at each pane — behind GLASSPEN.on (off: shotBlocked is the solid walk it was)
    bootBars(this);   // a window's bar breaks from one round (winbars.ts): the line of fire's own question and the blasts' call list — nothing while WINBARS.on is off
    this.shards = new GlassShards(this);      // broken glass after it lets go (shards.ts): takes the debris pool's glass door (and the fracture model's release hook where there is one) — inert until glass breaking is live
    GLASSPEN_HOOKS.crossed = (g, x, who) => { g.shards.crossed(x, who); };   // a round THROUGH a sheet: the exit jet off its far face (shards.ts crossed; nothing unless the falling-glass model is live)
    this.wind = new Wind(this);          // the air (wind.ts): the level's vents + the storm + a source per exterior door — after the doors exist
    this.airDust = new AirDust(this);    // the weather's dust in the rooms (airDust.ts): seeded with the doors as they stand — after the weather, the doors and the collision exist
    this.moths = new Moths(this);        // moths at the lamps (moths.ts) — after the targets (the lamps are light targets)
    this.standLights();   // (the lamps the fixtures feed, as rows in the list's order: rust/sim/src/lights.rs)
    spawnCivilians(this);   // the cleaner and the late worker on their marks (civilians.ts — after the shift, whose construction and dice they must not disturb; nothing with tune.civilians off or none authored)
    this.fogOfWar = new FogOfWar(this);         // fog of war (fogofwar.ts): inert until switched on
    this.quiet = new QuietMission(this);        // the quiet mission (quiet.ts): inert until started
    this.par = new Par(this);                   // a par clock over a room clear (par.ts): inert on a level without a scenarios.par block
    this.verdict = new Verdict(this);           // the run's verdict (verdict.ts): reads only, said once at the run's end
    this.rails = new Rails(this);               // a tutorial mission's frame (rails.ts): inert unless the level's lesson carries a rails record
    this.lesson = new Lesson(this);             // a training lesson's steps (lesson.ts): inert on a level without a scenarios.lesson block
    this.defuse = new Defuse(this);             // a defuse mission's charges (defuse.ts): inert on a level without a scenarios.defuse block
    this.nightBreach = new NightBreach(this);   // the night breach playtest (nightbreach.ts): inert until started
    this.roomClear = new RoomClearScenario(this);   // the room clear (roomclear.ts): the same playtest on a four-room structure — inert until started
    for (const s of this.seats) s.view.orders = new SquadOrders(s.view);   // (a seat's own: its menu, its men's orders — seats.ts)
    // squad orders (orders.ts): inert with no operative on the floor
    this.fight = new RoomFight(this);
    this.cues = new Cues(this); this.squadNet = new SquadNet(this);
    this.cover = new Cover(this);               // unasked cover (cover.ts): what the squad's free men watch — inert with nobody following or holding
    this.entry = new EntryPoints(this);         // entry points off uncleared openings (entrypoints.ts): inert with no squad on the floor
    this.cleared = new ClearedRooms(this);      // cleared rooms (cleared.ts): where the team has stood — the way an order walks a man keeps to it; inert with no squad on the floor
    this.rescue = new Rescue(this);             // the hostage rescue playtest (rescue.ts): inert until started
    for (const s of this.seats) s.view.doorTools = new DoorTools(s.view);       // (after the orders: its rows are registered on their menu)
    pairUpApply();                              // the pair's ONE switch (pairup.ts): on, its three parts are on — nothing touched with it off
    this.pair = new PairOrders(first);           // quiet orders for one partner (pairorders.ts): inert with its switch off
    for (const s of this.seats) s.view.work = new WorkThings(s.view);   // a teammate's hand on the breaker / the mission's thing (workthings.ts: its row on the orders' menu)
    for (const s of this.seats) s.view.wallCharge = new WallCharge(s.view);     // (after the door tools: it spends their charges)
    for (const s of this.seats) registerShutDoor(s.view);                     // (after the door tools: 'shut that door' stands in for their charge row on a door that stands open)
    standMen(this);   // (seats.ts: a second seat's man and his light, made last — a game of one makes nothing here)
    this.director = new Director(this, level.script ?? null);   // the level's script: its define pass run and linted against this level here (nothing of the world touched); programs start on demand
    this.glassBreak.stood();   // (which glass each pane is, settled as the level stands)
    { const over = stoodOver(this, this.col.mirror.tables); if (over) throw new Error(over); }   // (levelfit.ts: the game as it stands, against every table and pool that holds it — one over is not a game)
  }

  /** the showcase tour flips this so its own plumbing (teleports, resets) stays out of the log while guard barks still show */
  quietUtility = false;
  /** tour seam: a beat that stages the mission itself (the drive pull) sets this so the stage machine, the rack marker and the objective line run under
   *  the puppet — the debrief tallies still stay off (missionLive), so nothing the tour does counts as the player's run */
  missionInTour = false;
  msg(text: string) { if (RUST_SURFACE?.tapFiring('Game.msg')) return RUST_SURFACE.tapAnswer() as any; this.messages.push({ text, t: this.time }); if (this.messages.length > 5) this.messages.shift(); this.cues?.raw(text); }
  say(g: Guard, text: string, radio = false, force = false, row = true) { if (RUST_SURFACE?.tapFiring('Game.say')) return RUST_SURFACE.tapAnswer() as any; { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.say', this, [g, text, radio, force, row]); if (__r !== RUST_PASS) return __r as any; } } return say(this, g, text, radio, force, row); }
  viewProj: Float32Array | number[] | null = null;
  note(text: string) { if (RUST_SURFACE?.tapFiring('Game.note')) return RUST_SURFACE.tapAnswer() as any; { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.note', this, [text]); if (__r !== RUST_PASS) return __r as any; } } if (!this.quietUtility) this.msg(text); }   // utility / sandbox confirmations

  private onDoorEvent(e: DoorEvent) { if (RUST_SURFACE?.tapFiring('Game.onDoorEvent')) return RUST_SURFACE.tapAnswer() as any;
    { const pl = e.who === undefined ? null : manById(this, e.who); if (pl) pl.noise = Math.max(pl.noise, e.level); }
    switch (e.sound) {
      case 'creak': this.audio.play('doorCreak', e.pos, 0.5 + 0.5 * e.level / 0.3); break;
      case 'bang': this.audio.play('doorBang', e.pos, 0.9); break;
      case 'latch': this.audio.play('doorLatch', e.pos, 0.6); break;
      case 'bash': if (e.level < 1) this.audio.play('doorBash', e.pos, 0.6, { rate: 1.15 }); else this.audio.play('doorBash', e.pos, 0.9); break;   // (under 1: a leaf ripped round by its handle at a run — sprintdoor.ts — the same slam, lighter and shorter; nothing else emits a bash under 1)
      case 'rattle': this.audio.play('doorRattle', e.pos, 0.35 + 2 * e.level); break;
      case 'pick': this.audio.play('lockPick', e.pos, 0.5, { rate: 0.9 + simRandom() * 0.25 }); break;
      case 'unlock': this.audio.play('lockOpen', e.pos, 0.4 + 1.3 * e.level); break;   // the picked cylinder snapping over (0.3 → 0.8), or a key turning it (0.12 → 0.56: the same bolt, drawn gently)
      case 'kick': { this.audio.play('doorBash', e.pos, 1.0, { rate: 0.9 }); this.audio.play('lockBreak', e.pos, 0.95);   // the slam of the leaf, lower and harder than a shoulder, under the keep splintering out of the jamb
        const kicker = manById(this, e.who ?? -1)?.char ?? this.guards.find(x => x.char.id === e.who)?.char; kickDustFx(this, e.door, kicker?.pos ?? e.pos); if (e.door.lockBroken) lockSplinterFx(this, e.door, kicker?.pos ?? e.pos); this.destr?.thud(e.pos, 1.2, e.who); break; }   // …and the plaster and splinters that come out with it (GPU particles; nothing with them off), and the litter at the threshold jumping with the slam (destruction.ts impulses, behind chaos)
      case 'thud': this.audio.play('doorBash', e.pos, 1.0, { rate: 0.8 }); if (manById(this, e.who ?? -1) && !manById(this, e.who ?? -1)!.kick?.tool) this.msg('it opens toward you, so the frame took the kick. that was loud'); break;   // a boot at the face a shut door opens toward (doors.ts DOORWAY.frameHolds): the slam of it, nothing splintered
      case 'clang': this.audio.play('doorBash', e.pos, 1.0, { rate: 0.7 }); this.audio.play('stairDoor', e.pos, 0.55, { rate: 1.3 }); if (isManId(e.who ?? -1)) this.msg("it's a steel fire door, so it won't give. that was loud"); break;   // a boot on the steel stair door (doors.ts DoorDef.steel): the ring of it, no keep to splinter
    }
    // heard (guards.ts hearingCheck): a kick is its own kind — floor-wide, answered like a shot; everything else is a 'door' noise with a level, placed at the leaf (the
    // clang of a boot on steel that held is a slam's worth of door noise: the men within earshot come and look, the floor is not told 'someone's inside')
    this.events.push({ kind: e.sound === 'kick' ? 'kick' : 'door', pos: [e.pos[0], 0, e.pos[2]], time: this.time, loud: (e.sound === 'bash' && e.level >= 1) || e.sound === 'kick' || e.sound === 'clang' || e.sound === 'thud' || (e.sound === 'bang' && e.level > 0.6), level: e.level, who: e.who });   // (a bash under 1 is a leaf ripped round by its handle at a run — sprintdoor.ts — a door noise at its level, not the slam; every other bash is emitted at 1)
  }
  private rolls: Roll[] = [];
  private rollSeq = 0;
  /** Furniture noise: a scrape / caster rattle when something is set going, a hollow knock when it hits a wall, another prop or takes a round.
   *  Heard like door sounds (level 0..1, blamed on whoever moved it — guards ignore each other's clumsiness). */
  private onPropEvent(e: PropEvent) {
    // a castor chair coasting (props.ts CASTOR) is voiced by rollTick, not from here: nobody hears it and no die is drawn — the game is the same game with or without
    if (e.sound === 'roll') {
      if (e.roll === 'start' && !this.rolls.some(r => r.p === e.prop)) this.rolls.push({ p: e.prop, x: e.pos[0], z: e.pos[2], k: 0, moveT: -1e9, voice: 0 });   // (where it was AS IT STARTED, which the event says: the step that made the noise may have moved it on before this is heard)
      return;
    }
    { const pl = e.who === undefined ? null : manById(this, e.who); if (pl) pl.noise = Math.max(pl.noise, e.level); }
    const t = e.prop.tune; const jitter = 0.95 + simRandom() * 0.1;
    if (e.sound === 'thud') { this.audio.play('propThud', e.pos, clamp(0.35 + 0.15 * e.speed, 0.3, 1), { rate: t.thudRate * jitter }); this.destr?.thud(e.pos, 0.4 + 0.3 * e.speed, e.who); }   // (…and the litter round its feet twitching with the knock: destruction.ts impulses, behind chaos)
    else if (!e.rolling) this.audio.play('propScrape', e.pos, clamp(0.25 + 0.18 * e.speed, 0.25, 1), { rate: t.scrapeRate * jitter });   // (a chair already rolling on its castors is heard rolling: no scrape voiced over it — its event, below, is filed all the same)
    this.events.push({ kind: 'prop', pos: v3.copy(e.pos), time: this.time, loud: e.level > 0.7, level: e.level, who: e.who });
  }


  resetGuards() { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.resetGuards', this, []); if (__r !== RUST_PASS) return __r as any; } } powerOutReset(this); rescueCheckReset(this); dozeReset(this); eachSeat(this, v => { closeReset(v); stalkReset(v); tapReset(v); }); this.fight?.reset(); this.cover?.reset(); this.entry?.reset(); this.cleared?.reset(); this.doorPath?.reset(); wallReactReset(this); coverfireReset(this); doorForceReset(this); resetQrf(this); resetGuards(this); resetCivilians(this); tacticsReset(this); lanesReset(this); pictureReset(this); }   // (…and the civilians: fresh ones on their marks with the fresh shift — a held / dragged one let go of first; and the tactics' memory: plans, gates and kill-zone doorways were the OLD shift's — tactics.ts, review G3.3)   // (the response team first: its men are units resetGuards despawns anyway, its radio clocks and marks are its own to drop)
  tension(theirs = false) : number { return tension(this, theirs); }
  /** one line on the alarm escalation for the panel / debugging (level, clock, episode, lockdown pair + post) */
  escalationSummary(): string { return escalationSummary(this); }
  /** panel / debug: one level up by hand (as if an alarm had just found nobody), or straight back to calm */
  escalate() { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.escalate', this, []); if (__r !== RUST_PASS) return __r as any; } } raiseEscalation(this, this.escalation >= 1 ? 2 : 1); }
  standDown() { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.standDown', this, []); if (__r !== RUST_PASS) return __r as any; } } standDownEscalation(this); }
  /** panel: 'clearing: <room> · <phase> …' while the lockdown pair clears, else '' ; and the test button — lock down and clear the room nearest the player now */
  clearingSummary(): string { return clearingSummary(this); }
  /** panel / QA: what each unfound body reads on the light meter right now (guards.ts bodyLightSummary) */
  bodyLightSummary(): string { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.bodyLightSummary', this, []); if (__r !== RUST_PASS) return __r as any; } } return bodyLightSummary(this); }
  /** panel / console: everybody's suppression — Sam's scalar, then every living man with anything on him: his scalar and what it has him doing (flinching, head
   *  down and for how long, shifting to cover, holding it) — or the nearest man at rest (suppression.ts suppressionSummary) */
  suppressionSummary(): string { return suppressionSummary(this); }
  conditionSummary(): string { return conditionSummary(this); }
  /** panel / console: the aim-settle model — Sam's track and cone with its terms, then every alert man's (or the nearest at rest): track, Sam's bearing rate
   *  across him, his cone and its terms, whether he is braking for a shot, his last round (aim.ts aimSummary) */
  aimSummary(): string { return aimSummary(this); }
  /** panel / console: the pursuit tactics — flanking plans (who presses, who flanks, phase), lone men holding an angle, kill-zone doorways with their seconds
   *  left, men on a doorway drill (tactics.ts tacticsSummary); '' when nothing is up */
  tacticsSummary(): string { return tacticsSummary(this); }
  /** panel / console: the needs — per man his needs and their levels or the trip he is on (about to go / walking / there / coming back), the floor's slot (needs.ts needsSummary) */
  needsSummary(): string { return needsSummary(this); }
  /** panel / console: the civilians — each one's state, what frightened her, whom she is running to / has told (civilians.ts civiliansSummary) */
  civiliansSummary(): string { return civiliansSummary(this); }
  /** One exchange of the interrogation with `gd` (dialogue.ts — docs/internal/grab-interrogate-design.md §4, slice 5: the engine without the grab). The man answers
   *  from his own state: what he personally saw and heard (Witness), how rattled he is, his temperament, and how far Sam has already leaned on him (Talk) — the line
   *  goes up as his bubble (or the log) with the interrogation's longer hold, and the pick comes back (text, stage, cell, and the useful thing's kind if he gave one)
   *  for whoever asked: the held man's row (player.ts askHeld — THE ANSWER half of an exchange there: Sam's own line goes up over Sam first, this runs when it
   *  ends), the sandbox button (the bare answer, at once), a mission hook later. The gate: the man in Sam's arm (player.holding) always; anybody else only for
   *  the sandbox — 'a living man within reach of a living Sam', INTERROGATE_REACH, 2 m, nothing solid between — and null otherwise. `rnd` picks within the
   *  pool (seeded by the QA harness for replayable runs; the game's own dice by default). The one thing an answer DOES, beyond being said: the man who gives
   *  the keyed door up gives his key up with it (player.ts takeKey — 'mine's on my belt', and Sam takes it). */
  interrogate(gd: Guard, rnd: () => number = simRandom): DialoguePick | null { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.interrogate', this, [gd, rnd]); if (__r !== RUST_PASS) return __r as any; } }
    if (this.player.down || gd.state === 'dead' || !(this.guards.includes(gd) || this.civilians.includes(gd))) return null;
    if (this.player.holding?.g !== gd) {   // not the man in his arm: the sandbox's reach gate
      const a = this.player.char.pos, b = gd.char.pos;
      if (v3.distXZ(a, b) > dialogue.INTERROGATE_REACH || this.col.solidBlocked([a[0], 1.0, a[2]], [b[0], 1.0, b[2]])) return null;   // in reach and nothing solid between, statics or a leaf (the takedown gate's own test): no leaning on a man through a partition
    }
    const pick = gd.civ ? civilianAnswer(this, gd, rnd) : dialogue.interrogate(this, gd, rnd); if (!pick) return null;   // (a civilian answers from her own small pool — fear, the thing or two she knows, then nothing: civilians.ts)
    say(this, gd, pick.text, false, true);   // (forced: a held man's only voice is his answers — guards.ts say drops everything else while he is held)
    if (gd.bubble) gd.bubble.dur = clamp(1.2 + 0.07 * pick.text.length, 2.5, 6.5);   // held speech runs longer than a bark's 4 s cap
    chronInterrogated(this, gd, pick.kind ?? null, pick.stageN);   // one for the debrief's timeline (who, and what he gave if anything) — ahead of the key coming off his belt, which is its own row
    if (pick.kind === 'door') takeKey(this, gd, rnd);   // the keyed door given up: the key on his belt comes off it (Player.keys), and Sam says so once the man has finished
    else if (pick.kind === 'code') { const K = this.devices.padNear(gd.char.pos); if (K) learnCode(this, K.def.name, 'told', gd); }   // the pad's digits given up: said is learned (Player.codes — devices.ts; the pad's door then offers 'enter the code')
    return pick;
  }
  /** Let go of the man in Sam's arm at once, no shove (player.ts freeHeld): Sam going down with him, a teleport, the tour taking over. He comes off it knowing everything. */
  freeHeld(why: 'shot' | 'cancel') { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.freeHeld', this, [why]); if (__r !== RUST_PASS) return __r as any; } } return freeHeld(this, why); }
  /** the living man nearest Sam, within `reach` metres (Infinity = anywhere) — the sandbox button's target, and the panel line's */
  nearestLivingGuard(reach = Infinity, foe = false): Guard | null {   // (foe: only men hostile to the player — a teammate walking behind him is nobody to lean on)
    let best: Guard | null = null, bd = reach;
    for (const gd of this.guards) { if (gd.state === 'dead' || (foe && !hostileTo(gd, 'player'))) continue; const d = v3.distXZ(gd.char.pos, this.player.char.pos); if (d <= bd) { bd = d; best = gd; } }
    return best;
  }
  /** sandbox: lean on whoever is within reach (a note if nobody is) */
  interrogateNearest(rnd: () => number = simRandom): DialoguePick | null { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.interrogateNearest', this, [rnd]); if (__r !== RUST_PASS) return __r as any; } }
    if (this.player.down) { this.note("you're down — nobody answers to a man on the floor"); return null; }
    const gd = this.nearestLivingGuard(dialogue.INTERROGATE_REACH, true);
    if (!gd) { this.note(`nobody hostile within ${dialogue.INTERROGATE_REACH} m to lean on — walk up to a living guard`); return null; }
    const pick = this.interrogate(gd, rnd);
    if (!pick) this.note(`${gd.callsign} is in reach but there is something solid between you`);
    return pick;
  }
  /** panel / console: the interrogation engine's read of the man nearest Sam (any distance) — his knowledge / agitation / temperament cell with the agitation terms,
   *  the stage the next press would be, what he has said and given, the useful thing he would give next (dialogue.ts dialogueSummary) */
  dialogueSummary(): string { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.dialogueSummary', this, []); if (__r !== RUST_PASS) return __r as any; } }
    const gd = this.nearestLivingGuard(Infinity, true); if (!gd) return 'dialogue: nobody left standing';
    const d = v3.distXZ(gd.char.pos, this.player.char.pos);
    return `dialogue ▸ ${dialogue.dialogueSummary(this, gd)} · ${d <= dialogue.INTERROGATE_REACH ? 'in reach' : `${d.toFixed(1)} m off`}`;
  }
  /** panel: the rooms still queued for the pair and the ones called clear in this lockdown lately (with how long ago) */
  clearQueueSummary(): string { return clearQueueSummary(this); }
  clearNearestRoom() { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.clearNearestRoom', this, []); if (__r !== RUST_PASS) return __r as any; } } forceClear(this); }
  /** Camera follow position (player or spectated guard). */
  followPos(at?: Vec3): Vec3 { const g = this.spectate >= 0 ? this.guards[this.spectate] : null; return g ? g.char.pos : this.view.lesson?.rails?.sky ?? at ?? this.player.char.pos; }   // (`at`: where his man is — a look ahead asks with where he will be, walkahead.ts)
  teleportPlayer(p: Vec3) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.teleportPlayer', this, [p]); if (__r !== RUST_PASS) return __r as any; } } return teleportPlayer(this, p); }
  /** HUD / panel: what the player's feet are on as the hearing model has it (floor.ts — lino everywhere with tune.floorMaterials off) */
  get floorUnderPlayer(): FloorMat { return stepMat(this, this.player.char.pos); }

  repairLights() { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.repairLights', this, []); if (__r !== RUST_PASS) return __r as any; } } return repairLights(this); }
  /** Everything still in the air / in the sound queue from what just happened: fused canisters, sparks, the last half second of noises, a wind-up.
   *  Fresh guards must not inherit it (they would hear shots nobody alive fired). */
  clearAftermath() { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.clearAftermath', this, []); if (__r !== RUST_PASS) return __r as any; } } fragReset(this); moraleReset(this); this.items.items.length = 0; this.rings.length = 0; this.shells.length = 0; this.sparks.clear(); this.tracers.clear(); clearParticleFx(this); this.moths?.clear(); this.destr?.clearMoving(); this.shards?.clear(); this.events.length = 0; for (const s of this.seats) { const pl = s.view.player; if (pl) { pl.pendingThrow = null; pl.throwHeld = false; pl.throwPreview = null; } } }   // (clearParticleFx: the GPU particles' windows and the cases waiting to be heard — brass from the last take does not lie in the next)
  /** The loadout toggle (start card / pause menu): the FN2000 in or out of the kit — remembered per machine and applied to the man standing there now (taken out with
   *  it in his hands or slung, he is left with the pistol selected, hands as they were). */
  setLoadout(withRifle: boolean) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.setLoadout', this, [withRifle]); if (__r !== RUST_PASS) return __r as any; } }
    this.fn2000 = withRifle;   // (nothing persisted: the FN2000 is always in the kit at boot — twopointsix.fn2000 was written here and read nowhere; the harness flips it live)
    const pl = this.player; if (!pl) return;
    if (!withRifle && pl.slot === 'rifle') { pl.slot = 'pistol'; if (pl.rifle.reloading) pl.char.anim.cancelReload(); pl.rifle.cancelReload(); }
    pl.hasRifle = withRifle;
  }

  /** THE ROWS OF THE LIGHTS STOOD ON THE LIST AS IT IS: as the level is built, and again when a fixture is hung after that (the kit picture's, host/kitView.ts). So every lamp a fixture feeds has a row and one way of burning. */
  standLights(): void { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.standLights', this, []); if (__r !== RUST_PASS) return __r as any; } } this.lightHome.stand(this.engine.lights.lights, this.targets, this.beacons, this.devices.cameras.map(c => ({ light: c.cone, target: c.target, slot: c.slot }))); }
  resetLevelState() { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.resetLevelState', this, []); if (__r !== RUST_PASS) return __r as any; } }
    for (const t of this.targets) this.lightHome.in(LIGHT_IN.on, t, t.authored ? 1 : 0);
    for (const d of this.doors.list) d.reset();
    this.decals.clear(); resetGlass(this); resetGlassPen(this);   // the marks go with the encounter that made them (a restart, the tour's hand-over) — and the glass's count / log of the crack stars among them (glass.ts, review U9.10)
    resetBars(this);       // …and every window's bar a round or a blast took stands again (winbars.ts) — before the destruction's own reset, like the panes
    this.glassBreak.reset();   // …and every pane is whole again (glassbreak.ts: impact lists, dropped cells, gust sources — the shards went with the debris pool above)   // (BEFORE the destruction's: the walls' nav re-bake in there must see whole panes)
    this.destr.reset();    // …and so does the destruction: scattered paper stacks / knocked-off clutter / dead screens back as authored, debris and heat gone (destruction.ts)
    this.shards.reset();       // …and no glass is pending, lying about or underfoot (shards.ts; its plates went with the debris pool, its particles with the particle system's clear)
    clearWetPrints(this);  // …and the wet-print records that pointed at some of them (wet.ts)
    clearSnowBlast(this);
    clearSnowMarks(); if (this.snowTracks) clearSnowTracks(this.snowTracks);   // …and the snow is unmarked (snowTracks.ts; the next frame hands the cleared bytes over)
    hourAfterReset(this);  // …and, the level started at another hour (hours.ts), that hour's switches over the authored ones: the outside lamps out by day
    this.airDust?.reset(); // …and the weather's dust in the rooms stands as on a fresh start: the doors and the walls are back (airDust.ts)
  }
  /** glass experiment: the panel's / console's handles */
  /** glass: star the pane under / nearest the cursor as a round would (the panel's 'crack test'; returns the pane's index or −1), and the panel's line */
  crackTest(): number { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.crackTest', this, []); if (__r !== RUST_PASS) return __r as any; } } return crackTest(this); }
  glassSummary(): string { return glassSummary(this); }
  /** wet characters (wet.ts): everybody's coat / beads and whether it is raining on him, the prints and drips so far — the panel's line and __wet.summary() */
  wetSummary(): string { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.wetSummary', this, []); if (__r !== RUST_PASS) return __r as any; } } return wetSummary(this); }
  /** the particle-rain experiment's one line (rainFx.ts; Tab ▸ Weather) */
  rainSummary(): string { return rainSummary(this); }

  /** CLEANRESTART (cleanrestart.ts), restartEncounter's part: the clocks the last try left running, as a new Game has them, and the vents as the hour has them. */
  private cleanSlate() {
    this.alarm.raisedAt = -100; this.alarm.pos = [0, 0, 0]; for (const s of this.seats) s.view.torchOnT = -100; this.lastStingT = -100;
    gunWallFresh(this); eachSeat(this, groundFresh); this.wind?.fresh();
  }
  /** set by restart(), taken by the next update(): the camera's cut is put on the host's camera BEFORE that tick's record reads the view */
  private restartCut = false;
  /** THE PLAYER'S RESTART — the pause card's and the down card's buttons, the panel's, Shift+Enter, Enter when down. (Not a checkpoint's load, a recording's
   *  first frame, a show's own restore or a mission's staging: those call restartEncounter and keep their own camera and picture.) With CLEANRESTART up it is
   *  restartEncounter and, for whoever watches: the camera cut to the fresh man (the seat's cut, cmd.ts takeCut; a show's or
   *  a spectator's camera is its own) and the host told to start the picture's histories from nothing. `said`: a line about this restart itself (why a
   *  checkpoint would not load), which outlives it. Off: restartEncounter, the line said first, as it was. */
  restart(said?: string) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.restart', this, [said]); if (__r !== RUST_PASS) return __r as any; } }
    if (!CLEANRESTART.on) { if (said) this.msg(said); this.restartEncounter(); return; }
    this.restartEncounter();
    if (said) this.msg(said);   // (after it: restartEncounter takes the old encounter's lines off the HUD)
    if (!this.touring && !this.director?.showing && this.spectate < 0) { for (const s of this.seats) { const lens = s.cam, p = s.view.followPos(); lens.target = v3.copy(p); lens.desired = v3.copy(p); } this.restartCut = true; CLEANRESTART_STATS.cuts++; }
    if (CLEANRESTART.picture && this.engine.restartPicture) { this.engine.restartPicture('level restarted'); CLEANRESTART_STATS.pictures++; }
  }
  /** Restart the encounter: a fresh Player at the insertion point with full kit, fresh Guards on their routes, lights repaired, the level as authored. */
  restartEncounter() { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.restartEncounter', this, []); if (__r !== RUST_PASS) return __r as any; } }
    useDice(this.dice); this.sensors.ownReset(); this.quiet?.current();
    const clean = CLEANRESTART.on;   // (cleanrestart.ts: nothing of the try that ended stands, sounds or plays on in this one — off, every line below as it was)
    if (clean) { CLEANRESTART_STATS.restarts++; CLEANRESTART_STATS.lines += this.messages.length; this.messages.length = 0; }   // (the lines of the encounter that ended go before anything below speaks: a staging's own line is this one's)
    this.quiet?.beforeRestart();   // the quiet mission, while it is up, deals this encounter's start / spot / patrol and writes them into the level BEFORE the fresh Player and Guards read it (quiet.ts) — a return on its first line otherwise
    for (const s of this.seats) { const v = s.view, light = v.player.light;   // (every seat's man afresh — seats.ts: the first on the spawn, the next beside him)
      manLeaves(v.player.char, v.player);   // (his row is free for the fresh man)
      v.player = new Player(this.men, this.ch, spotBeside(this, v3.copy(this.level.playerSpawn), s.index), this.fn2000, s.index); v.player.light = light; kitFresh(v); v.player.char.update(0); }   // pose + bones exist before this frame's capsule upload
    this.cues?.reset(); this.squadNet?.reset(); eachSeat(this, v => { holdFireReset(v); resetShowWhy(v); });   // (holdfire.ts: a fresh encounter — nothing has started the fight)
    dialogue.ambientRestart(this);   // (the staff's ambient chat: its first-chat clock runs from this restart, not from the page's first frame)
    this.fogOfWar?.reset();   // (fog of war: its memory of who was seen and its turn at the probe belong to the encounter that just ended)
    this.resetGuards(); if (clean) { if (this.messages[this.messages.length - 1]?.text === 'guards reset') this.messages.pop(); if (freshLights(this)) CLEANRESTART_STATS.relit++; } else this.repairLights(); this.resetLevelState(); lightSwitchReset(this); this.props.reset(); for (const r of this.rolls) r.voice = 0; this.rolls.length = 0; this.events.length = 0; for (const s of this.seats) s.view.useDoor = null; this.calledIn = false; this.stoodDown = false;   // (resetGuards also stands the floor down: escalation 0, no episode, no lockdown duties)
    // anything still in the air or still venting belongs to the encounter that just ended: an armed stun canister landing into the new one would
    // dazzle the fresh guards and alert the floor from a throw the restarted player never made, and a deployed can's 30 s screen would go on
    // degrading their line of sight (its emitter tracks the item and is priority-pinned, so dropping the item alone does not stop it)
    this.clearAftermath(); this.smoke.clearAll?.(); this.cpuSmoke.grid?.renew(); this.engine.hostBooks = undefined;   // (… and with that solver's smoke, every host book of what was filed with it and asked of the mixer, host/api.ts HostBooks: the open fires', fire.ts FireShown, and a smoke break's, whose rows are by a man's id and would be the fresh man's — asked for afresh on the next frame)
    for (const s of this.seats) { const v = s.view; v.hover = null; v.interactables = []; v.nearby = []; v.act = null; }   // (like useDoor above) this frame's cursor state still names the guards / bodies of the encounter that just ended: when the restart
                                                  // came from updatePlayer (Shift+Enter), the Space handlers later in the same update() must not grab, take down or drag any of them
    if (clean) this.cleanSlate();
    this.devices.reset();                   // cameras forget what they saw, beams their trips (repairLights above has already mended any shot dead)
    this.mission = newMission(this.time);   // a fresh run: stage, clock and stats from zero (resetLevelState above has already relit the rack)
    this.chronicle = new Chronicle();       // …and an empty chronicle to go with it (the debrief's timeline, path and counters)
    this.director?.onEncounterRestart();    // a PLAY program of the level script ends with the encounter it was scripting (a showcase's own restore restarts the encounter itself and is left alone)
    this.nightBreach?.onEncounterRestart(); // the night breach playtest, while it is up, stages itself again on the fresh encounter (nightbreach.ts)
    this.rescue?.onEncounterRestart();      // …and so does the hostage rescue (rescue.ts)
    this.quiet?.onEncounterRestart();       // …and the quiet mission faces Sam as its start has him and stages the partner (quiet.ts)
    if (clean) { updateTargets(this, 0); updateFire(this, 0); }   // the lights painted as they now stand: what the next tick reads before its own paint — the reticle's rows, the men's eyes — is the fresh level, not the last try's last frame
    if (!clean) this.note('encounter restarted');   // (clean: a level just opened says nothing of how it came to be — nor of its guards, above)
  }

  setGroup(group: string, on: boolean) { return setGroup(this, group, on); }
  setBlackout(seconds: number, cause: 'ocp' | 'shot' | 'debug' | 'hand') { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.setBlackout', this, [seconds, cause]); if (__r !== RUST_PASS) return __r as any; } } return setBlackout(this, seconds, cause); }
  endBlackout() { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.endBlackout', this, []); if (__r !== RUST_PASS) return __r as any; } } return endBlackout(this); }
  lightEvent(t: LightTarget, cause: 'ocp' | 'shot' = 'shot', swAt?: Vec3) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.lightEvent', this, [t, cause, swAt]); if (__r !== RUST_PASS) return __r as any; } } return lightEvent(this, t, cause, swAt); }
  ocp(t: LightTarget | null): void { return crossToRust('Game.ocp', this, [t]) as any; }
  toggleDrag(which?: Guard) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.toggleDrag', this, [which]); if (__r !== RUST_PASS) return __r as any; } } return toggleDrag(this, which); }

  // ------------------------------------------------------------ update
  /** ONE TICK, as the page, the harness and a replay call it: the host's live input, camera and view are SAMPLED into the tick's plain record (cmd.ts sampleCmd) and the
   *  simulation runs off that record alone (step). What the tick hands back for the seat's camera — the wall peek's nudge, a blast's kick, a scenario's opening cut — is
   *  copied to the host's. (The record is ONE object refilled each tick: a caller that keeps records keeps a copy or its text, cmd.ts cmdText.) */
  update(dt: number, input: Input, cam: FollowCamera, canvas: View) {
    useDice(this.dice);   // this Game's die is the one simRandom() draws for the length of its frame (null: Math.random) — with two Games in one process each rolls its own
    this.quiet.current();   // … and the module switches a quiet mission's partner run holds are THIS Game's for the frame (quiet.ts: a compare with one Game, as on the page)
    const me = this.hostSeat();   // (the seat this host plays: its record is sampled here; any other seat's is whatever was last handed to it — Seat.hand)
    if (this.restartCut) { this.restartCut = false; const cut = me.takeCut();   // a restart from outside a tick (a card's button) cut the seat's camera: taken HERE, before this tick's record reads the view, so the fresh man's first tick is aimed through the camera that is on him
      if (cut && !cam.pin) { cam.target = v3.copy(cut); cam.desired = v3.copy(cam.target); cam.rebuild(); } }
    this.director?.injectInput(input);   // a showcase beat's synthetic key / click from LAST frame's tick (director.ts c.press / c.click) becomes this frame's input, ahead of everybody who reads it — a return on its first line otherwise
    this.cmdBuf = sampleCmd(me.view, input, cam, canvas, this.cmdBuf);
    const cmds: readonly PlayerCmd[] = this.seats.length === 1 ? (this.stepBuf[0] = this.cmdBuf, this.stepBuf) : this.seats.map(s => s === this.localSeat ? this.cmdBuf! : s.handed(this.cmdBuf!));
    this.replayTap?.(dt, cmds[0]);   // the replay buffer's run tape (runtape.ts): this step's dt and the first seat's record, before anything of it has run — reads only
    this.stepSeats(dt, cmds);
    this.hostAfter(input, cam, canvas);
    this.replayTap?.(dt);        // the replay buffer (replaybuf.ts): the settled frame read into its ring — reads only
  }
  /** THE HOST'S HALF OF A TICK, round the step — `update`'s own lines, named for a host that steps the game by records it did not sample itself (a networked
   *  page: net/page.ts). Before: what the host's INPUT is told. After: what the tick hands back for the camera of the seat this host shows (`hostSeat`), and
   *  that seat's live view put back. */
  hostAfter(input: Input, cam: FollowCamera, canvas: View) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.hostAfter', this, [input, cam, canvas]); if (__r !== RUST_PASS) return __r as any; } }
    const me = this.hostSeat();
    cam.peekOffset = me.cam.peekOffset; cam.kick = me.cam.kick;
    { const cut = me.takeCut(); if (cut) { cam.target = v3.copy(cut); cam.desired = v3.copy(cam.target); } }   // (a scenario's opening cut: the look-at snapped to its man, not flown)
    me.view.viewProj = cam.viewProj;   // (between ticks `viewProj` is the host's live matrix again, as it always was for whoever asks on-screen-or-not from outside a tick)
    this.rails.hostView(cam);   // (a tutorial mission staged again opens on its first view: a return on its first line off a rails level)
    me.view.orders.hostView(input, cam, canvas);   // (between ticks the HUD and the tools ask the menu what the LIVE cursor is on, as they always did)
  }
  /** the tick for a game of one seat: `stepSeats(dt, [cmd])` */
  step(dt: number, cmd: PlayerCmd) { this.stepBuf[0] = cmd; this.stepSeats(dt, this.stepBuf); }
  private stepBuf: PlayerCmd[] = [];
  /** the player's seat (cmd.ts): the tick's record loaded into an Input and a camera of the game's own. In a game of more than one seat this is the FIRST seat's,
   *  and a seat's own code reads it off its view (seats.ts). */
  seat!: Seat;
  /** every seat, in order (seats.ts): one or two, fixed when the game is stood up */
  readonly seats: readonly Seat[];
  /** THE SEAT THIS HOST SHOWS (seats.ts): whose camera the picture follows, whose HUD is drawn, whose ears the mix is for. The first seat unless the host says
   *  otherwise; null = a host that shows nobody (a server). The simulation never reads it: two hosts of one game with different local seats step the same game.
   *  OUTSIDE the tick the Game's own fields of a seat's name (`game.player`, `game.aimPoint`, `game.orders`: what the HUD, the camera and a panel ask) answer
   *  this seat's, so the host's own code shows the man it plays with nothing rewritten (seats.ts). */
  localSeat: Seat | null;
  /** true for the length of a tick (stepSeats): inside it, a game of more than one seat counts an ask of the Game's own `player` as a miss (seats.ts) */
  ticking = false;
  /** the local seat, or the first for a host that shows nobody — for the picture-only parts of a tick, which need SOME lens */
  hostSeat(): Seat { return this.localSeat ?? this.seats[0]; }
  private readonly aloneOwn = new AsideOwn();
  /** THE WORLD ASKED ASIDE: `fn` runs with the rays and the collision queries it asks kept off the game's own scratch (scene/boxes.ts rayAside, collision.ts
   *  aside) — for code that reads the world between ticks and must leave nothing on it (a look ahead: walkahead.ts) */
  aside<T>(fn: () => T): T { return this.engine.world.rayAside(this.aloneOwn, () => this.col.aside(this.aloneOwn, fn)); }
  /** …and a lens of the host's to put a record's view on while it does (cmd.ts loadLens): scratch, in no snapshot */
  asideLens(): FollowCamera { return this.aloneOwn.lens ??= new FollowCamera(); }
  private cmdBuf: PlayerCmd | null = null;
  /** the tick itself, off the player's plain record: no Input, no camera, no canvas of the host's. What it leaves for the seat's camera is on the seat (cmd.ts Seat: `cam.peekOffset`,
   *  `cam.kick`, `takeCut()`) for whoever hosts the picture. Reads of the seat's view inside the tick go to the record: the reticle's snaps and the order menu's cursor,
   *  the walk keys' basis, on-screen-or-not for a line (viewProj), the thunder's pan, the rain's near field, metres per pixel, a blast's kick. */
  stepSeats(dt: number, cmds: readonly PlayerCmd[]): void { return goneWithTick('Game.stepSeats') as any; }
  /** men moving through smoke push it */
  private smokeWake(all: Character[]) {
    // characters wading through smoke push it around
    const stir = Math.max(0, lookConfig.stir);   // (THE LOOK PROTOTYPE's Stirred smoke row; 0 = the two soft shoves below, as the build has them)
    this.smoke.setStir?.(stir);
    for (const c of all) {
      const sp = Math.hypot(c.vel[0], c.vel[2]); if (sp < 0.25 || !c.bones.hips) continue;
      const dirv: Vec3 = [c.vel[0] / sp, 0.05, c.vel[2] / sp];
      if (stir > 0) {
        // STIRRED SMOKE CURLS (the maintainer, 10-31, of smoke in the yard: "more turbulence or vorticity near the edges, especially when it's driven by player
        // movement", with the smooth flow the wind makes along a wall kept). HIS BODY is a standing capsule from his shins to his shoulders: the air inside it
        // goes with him and the solver sends what he displaces round his sides (smoke.wgsl emBody). HIS STRIDE throws the air just behind him out to one side
        // and then the other, a side a step: the shedding a walking man's legs and arms do, which a lattice this coarse would smooth into two still eddies.
        // The side is his own walk cycle's (one cycle is two steps), so it changes when his foot does, whichever way he turns.
        const S = SMOKE_STIR, h = c.bones.hips, hx = dirv[0], hz = dirv[2], top = h[1] + S.above, mid = (top + S.foot) / 2;
        this.smoke.push({ pos: [h[0], mid, h[2]], dir: [hx, 0, hz], speed: sp, radius: S.body, body: Math.max(0, (top - S.foot) / 2 - S.body) });
        const side = Math.sin(2 * Math.PI * c.anim.phase);
        if (Math.abs(side) > S.gap) { const k = Math.sign(side);
          this.smoke.push({ pos: [h[0] - hx * S.back, h[1] - S.low, h[2] - hz * S.back], dir: [-hz * k, 0, hx * k], speed: Math.min(S.most, sp * S.kick * Math.min(2, stir)), radius: S.kickR }); }
        continue;
      }
      this.smoke.push({ pos: c.bones.hips, dir: dirv, speed: sp * 1.15, radius: 0.3 });
      if (c.bones.chest) this.smoke.push({ pos: c.bones.chest, dir: dirv, speed: sp * 1.1, radius: 0.26 });
    }
  }
  /** the world size of one drawn pixel at a camera's look-at depth, for a view of `canvas`'s height (what `mPerPx` is of the tick's own view; the engine replay asks it of the
   *  picture's camera and window) */
  viewPx(cam: { distance: number; fov: number }, canvas: { height: number }): number {
    const sharpStill = this.engine.sharpOn && this.engine.settings.sharpDecouple;
    return 2 * cam.distance * (sharpStill ? 1.6 : 1) * Math.tan(cam.fov / 2) / Math.max(200, canvas.height * (sharpStill ? 1 : (this.engine.effectiveScale || 1)));
  }
  private menAll: Character[] = []; private menStatured = false; private menBare = false; private fogHeld: { dyn: Box[]; on: boolean } | null = null;
  /** A VIEWER'S PICTURE WITHOUT THE FOG OF WAR (replay/engineplay.ts's 'Fog' switch; only where `fogOfWar.viewer` was raised, so never in play): call right before
   *  a draw, BETWEEN two steps, and `fogBack()` right after it. With `want` the renderer is handed everybody — the men the fog leaves out after the men it draws —,
   *  the frame's boxes as they stood before the fog cut them, what the left-out men carry, and no fog pass; the HUD's `hidden()` and its marks answer for an open
   *  floor meanwhile. Nothing a step reads is written: `fogBack()` hands the CPU's rays the step's own list again (the cursor's ray walks the packed list); the
   *  skin instances and shadow proxies stay the lifted draw's until the next step or draw deals them again (the GPU's alone: no step reads them).
   *  `tube` (the replay's 'NV' switch; null = the run's own): the picture's night vision is the viewer's for this draw, so the INFRARED laser lines — boxes a step
   *  makes only for a picture through the tube (irlaser.ts irViewer) — are added (combat.ts irBeams) or left out (irMade) to match; the run's goggles, and
   *  everything that reads them, stay the tape's.
   *  `px` (over 0: the picture's own pixel in metres, `viewPx` of the camera and window it is drawn with): the thin lines a step sized for ITS view's pixel — laser beams,
   *  tracer streaks (thinline.ts) — are handed over re-sized for the picture's. A tape recorded at another zoom or window size drew a beam as a fat pale bar. */
  fogLift(want: boolean, tube: boolean | null = null, px = 0) {
    if (this.fogHeld) return;   // (already lifted for this draw)
    const fog = this.fogOfWar, bare = want && fog.viewer && fog.active;
    const ir = fog.viewer && tube !== null && !(this.engine.settings.debugView > 0) && tube !== irViewer(this);   // (the viewer's tube is not the run's: the infrared lines follow the picture — a debug view shows no tube, so none are added to one)
    if (bare !== this.menBare && this.menAll.length) { this.drawMen(this.menAll, this.menStatured, bare, DRESS.men); this.menBare = bare; }
    const world = this.engine.world, thin = fog.viewer && px > 0;
    if (!bare && !ir) {
      const T = thin ? thinFit(world.dynamics, px) : null; if (!T) return;   // (every line already fits this picture: nothing lifted)
      this.fogHeld = { dyn: world.dynamics, on: fog.frame.on }; world.dynamics = T; return;
    }
    let B = (bare ? fog.raw() : null) ?? world.dynamics.slice(); const n0 = B.length;
    this.fogHeld = { dyn: world.dynamics, on: fog.frame.on };
    if (bare) { hiddenProps(this, B);
      if (this.menStatured) drawnBoxes(this, this.menAll, B, n0);   // (their kit where their drawn hands are, as the step's own rigs were placed)
      fog.frame.on = false; fog.bared = true; fog.facesLift(); }   // (and no lamp's face as the team last saw it: the picture without the fog is the lamps as they are)
    if (ir) { if (tube) irBeams(this, B); else B = B.filter(b => !irMade(b)); }   // (tube on: the lines the run's men have on and its picture did not show; tube off: the lines the step drew for its own tube left out)
    if (thin) B = thinFit(B, px) ?? B;
    world.dynamics = B;
  }
  fogBack() {
    const H = this.fogHeld; if (!H) return; this.fogHeld = null;
    const fog = this.fogOfWar, world = this.engine.world;
    if (fog.bared) fog.facesBack();
    fog.bared = false; fog.frame.on = H.on; world.dynamics = H.dyn;
    world.relist();   // (listed again as the step left it: the game reads the list, and the next draw packs what it is handed — scene/boxes.ts relist)
  }
  /** `draw` false (dress.ts, the `men` row down): nobody is handed to the renderer, and the one thing here the simulation reads is still done — what each man
   *  wears (`parts`: his kit's boxes are placed off it, rigProps.ts packOf), dealt to the same men on the same frame as with the row up */
  private drawMen(all: Character[], statured: boolean, bare: boolean, draw = true) {
    const world = this.engine.world; world.resetCapsules();   // (nobody handed over with the row down: the lists are emptied, not left as they last stood)
    const wetOn = (this.engine.settings as { wetChars?: boolean }).wetChars !== false;   // wet characters (wet.ts): the renderer's switch — off (or a dry man) hands the g-buffer zeros, and it shades him exactly as before
    let drawn = bare ? this.fogOfWar.everybody(all) : this.fogOfWar.filter(all);
    if (this.view.lesson?.rails?.sky) { const me = this.player.char; drawn = drawn.filter(c => c !== me); }   // (rails.ts, the sky seat: nobody on the floor is the player — he is parked, and not in the picture)   // (fog of war: the men nobody on the team sees are not drawn and cast nothing — the list itself while it is off)
    if (drawn.length > 8 && (this.nightBreach.active || this.rescue.active)) {   // more men on screen than have a mesh tree (the four-man squad, nightbreach.ts: Sam + four + up to five defenders; render/meshbvh.ts MESH_MAX_CHARS is eight. Every drawn man has his round-cone proxies: BoxWorld.MAX_CAST): the men down give up the trees' places first, so it is a body that is shadowed by his cones before any man standing is. Eight or fewer = the list as it was
      const down = new Set<Character>(); for (const gd of this.guards) if (gd.state === 'dead') down.add(gd.char);
      if (down.size) drawn = [...drawn.filter(c => !down.has(c)), ...drawn.filter(c => down.has(c))];
    }
    const looks = draw ? charLookFrame(this) : null;   // CHARACTER MATERIALS (charLook.ts): null unless settings.charMats — then every man is his two tints, as ever
    const sideOf = this.ch.cast ? charSides(this) : null;   // (the cast's placeholder colours go over by side — below; nothing with the mannequin loaded)
    // ONE MAN'S FAULT IS ONE MAN'S: a man who cannot be posed or handed over (a row with no rig behind it) is left out of this frame and SAID once by name
    // in the console; the men after him are drawn. `n` counts the men handed over, so nobody's instance is left a hole.
    let n = 0;
    for (const c of drawn) { const i = n; try {
      if (this.ch.cast) {   // THE CAST (game/cast.ts): the parts this man wears + his own proxy fit (render/sdfrig.ts, cached per mask) — nothing here with the mannequin loaded
        c.parts = castParts(this, c);
        if (draw && this.rigFits) {
          let f = this.rigFitOf.get(c.parts);
          if (f === undefined) { try { f = fitRig(this.ch, c.parts); } catch { f = null; } this.rigFitOf.set(c.parts, f); }   // (solved once per mask — ahead of play for every mask castParts can hand out: sdfrig.ts fitRigCast, in the constructor —; a failed fit keeps the shared one)
          if (f) this.rigFits.set(c.id & 0xff, drawnFit(c, f, statureOf(c))); else this.rigFits.delete(c.id & 0xff);
        }
      }
      if (!draw) continue;
      const po = sideOf && c.parts ? placeholderTints(sideOf(c)) : TINTS_AS_IS;   // the instance's two colours: his tint then his tint2 on the mannequin and on an operator; for a cast civilian / irregular the tint his side's TABLE reads for trousers, then for the top (charMaterials.ts placeholderTints)
      const tall = statured ? statureOf(c) : 1;   // (1 = the skeleton's own palette and proxies handed over, as ever)
      this.chars.setInstance(i, drawnJoints(c, tall), po & 1 ? c.tint2 : c.tint, po & 2 ? c.tint2 : c.tint, c.id, wetOn && (c.wet > 0 || c.beads > 0) ? (f, o) => { wetInstance(this, c, true, f, o); wetKnobs(f, o + 4); wetLineRow(c, f, o + 12); } : undefined, c.parts);
      this.chars.setMaterials(i, looks ? (f, o) => f.set(looks.palette(c), o) : undefined);
      let capAlb = v3.lerp(c.tint, [0.5, 0.5, 0.5], 0.3);
      if (wetOn && c.wet > 0) { const k = 1 - 0.35 * c.wet; capAlb = [capAlb[0] * k, capAlb[1] * k, capAlb[2] * k]; }   // his shadow proxies (what bounce rays and reflections see of him) darken with the coat like the fabric on screen
      const { caps, bound } = drawnCapsules(c, c.capsules(), tall); world.addCharacterCapsules(c.id, looks ? looks.proxies(c, wetOn && c.wet > 0 ? 1 - 0.35 * c.wet : 1, false) : capAlb, caps, bound);   // (per slot: one colour per proxy — his head bounces as helmet and skin, his legs as trousers)
      n++;
    } catch (e) { manNotDrawn(this, c, e); } }
    this.chars.finish(draw ? n : 0);
  }
  /** Scripts / probes / the tour: put the highlight on the row of `kind` (the one whose verb is `verb`, if given) in this frame's list, so the next Space tap does
   *  THAT — the by-hand equivalent of holding Space and scrolling to it. False if no such row is on offer right now. */
  selectVerb(kind: Interactable['kind'], verb?: Verb): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.selectVerb', this, [kind, verb]); if (__r !== RUST_PASS) return __r as any; } }
    const i = this.nearby.findIndex(r => r.kind === kind && (!verb || r.verb === verb));
    if (i < 0) return false;
    this.player.nearSel = i; pickHover(this); return true;
  }
  /** Esc, before the pause menu gets it (main.ts): the open selector backs out / a running sustained verb stops. True = used here, no menu this press. */
  escape(): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.escape', this, []); if (__r !== RUST_PASS) return __r as any; } } return this.orders.escape() || launchCancel(this) || escapeInteract(this); }   // (the squad's command menu backs out first: orders.ts)
  /** is there anything the back key would back out of right now (backkey.ts; asked by the page before Tab's trial choice gets the key) */
  backable(): boolean { return !!this.orders.menu || !!this.player.launchHeld || backableInteract(this) || !!this.view.kit.setup; }   // (…and the open setup of the gun in his hands)
  /** The light meter's mapping, irradiance (W/m², max over the axes as the probe reports it) → 0‥1 visibility: 0 at or below visFloor, 1 by visFloor·e^(1/visSlope).
   *  One mapping for the meter on Sam and for a searcher judging how dark a hiding place is (guards.ts): dark for the one is dark for the other. */
  visibilityOf(E: number): number { return clamp(this.tune.visSlope * Math.log(Math.max(E, 1e-5) / this.tune.visFloor), 0, 1); }


  private doorOwns: (DoorContact | null)[] = []; private doorWas: number[] = [];

  /** per living character: the leaf his hand is on (or was, a moment ago), when he last actually pushed it, whether that was the player easing it by hand, and
   *  which hand this push settled on (−1: not yet; t0 = when the record began) */
  private doorHandRecs = new Map<Character, { door: Door; t: number; t0: number; crack: boolean; hand: 0 | 1 | -1; holdOnly?: boolean; move?: boolean }>();


  dropThrowable(kind: ThrowKind, p: Vec3) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Game.dropThrowable', this, [kind, p]); if (__r !== RUST_PASS) return __r as any; } } return dropThrowable(this, kind, p); }
  throwCount(kind: ThrowKind) { return throwCount(this, kind); }











  /** WHAT THE HUD IS TOLD (src/view/seatview.ts): the Rust game's own answer, handed over by the host that steps this game. A read */
  get view(): TickView { return this.viewAsk?.() ?? NO_VIEW; }
  /** the host's: asks the Rust game for its view (src/host/rustgame.ts in a page, tools/blitz/rust-host.ts in bun) */
  viewAsk: (() => TickView) | null = null;
  /** WHAT THE END CARD SHOWS OF THE RUN (src/view/debriefview.ts): asked of the Rust game by the host that steps it, as the card is built */
  debriefView(): DebriefView { return this.debriefAsk?.() ?? NO_DEBRIEF; }
  debriefAsk: (() => DebriefView) | null = null;
  /** THE BOX WORLD, ASKED (src/view/worldask.ts) by whoever draws, hears or points: the Rust game's answer where one steps this game (the host sets `askHost`), and
   *  this page's own copy of the level where none does (a workbench's floor, the moment before the first start) */
  readonly ask: WorldAsk = {
    blocked: (a, b) => this.askHost ? this.askHost.blocked(a, b) : this.col.segmentBlocked(a, b),
    solid: (a, b) => this.askHost ? this.askHost.solid(a, b) : this.col.solidBlocked(a, b),
    ray: (o, d, max) => this.askHost ? this.askHost.ray(o, d, max) : this.col.raycast(o, d, max),
    hear: (from, to) => this.askHost ? this.askHost.hear(from, to) : this.col.sound.propagate(from, to),
    together: f => this.askHost ? f() : this.col.window(f),
  };
  askHost: WorldAsk | null = null;
  /** what the cursor is on, through a camera as it stands, and what a point is, as an order would be about them (src/view/read.ts): the host's, asked of the Rust game */
  cursorAsk: ((input: CursorAt, cam: CursorLens, canvas: { clientWidth: number; clientHeight: number }, seat?: number) => CursorView) | null = null;
  underAsk: ((p: Vec3, seat?: number) => Under | null) | null = null;
  objectivePos(): Vec3 | null { return this.view.mission?.pos ?? null; }
  objectiveText(): string { return this.view.mission?.text ?? ''; }

}


/** the men `drawMen` left out, each said once (game.ts drawMen): who, and the fault that kept him out of the picture */
let NOT_DRAWN: WeakSet<object> | null = null;
function manNotDrawn(g: Game, c: Character, e: unknown): void {
  const said = (NOT_DRAWN ??= new WeakSet());
  if (said.has(c)) return;
  said.add(c);
  const gd = g.guards.find(x => x.char === c), who = gd ? gd.callsign : c === g.player.char ? 'the player' : `man ${c.id}`;
  const book = ((g as unknown as { menNotDrawn?: { who: string; id: number; why: string }[] }).menNotDrawn ??= []);
  const why = String((e as { message?: string })?.message ?? e).slice(0, 300);
  book.push({ who, id: c.id, why });
  console.error(`${who} (id ${c.id}) cannot be posed and is NOT DRAWN: ${why}`);
}
