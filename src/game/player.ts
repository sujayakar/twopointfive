// Player controller: movement / crouch / sprint under real or puppet input, back-to-the-wall (Q), the pistol and rail light + its OCP, the FN2000 (kit slot 2: SEMI / AUTO on the left button, its attachment — the ring launcher — on the right, its light), throws, silent takedown and body drag, plus the cursor: what is under it and what Space would do (interactables).
// (Split out of game.ts mechanically: plain functions taking the Game as their first argument; Game keeps one-line delegators for
//  anything other files call.)
import { SideMap } from '../math/side';
import type { View } from '../host/hud';
import { Vec3, v3, clamp, wrapAngle, approachAngle, damp } from '../math/vec';
import { Input } from './input';
import { FollowCamera } from './camera';
import { Door } from './doors';
import './ragdoll';
import { ThrowSolution } from './throwables';
import './rigProps';
import { samWork } from './handwork';
import './workstow';
import { disarmBegin, disarmCatch, type Strip } from './disarm';
import './handwork';
import { samLoadPace } from './kitload';   // what his card weighs: his legs' pace and his footfall (as asked with its switch off)
import { tally } from './mission';
import './quiet';   // (called inside buildInteractables only)
import './workthings';   // (called inside buildInteractables only)
import { chronKey, chronGrab } from './chronicle';
import { killGuard, dropFromHand } from './combat';
import { restOfThrow } from './frag';
import './bigkit';
import './kit';   // (the two asked at the two keys and each frame beside them: a card that left your light or your goggles at home — kittree.ts)
import './shotgun';
import { samBigDrop } from './bigkit';
import './suppression';
import './condition';   // (a hurt man does not sprint: Shift is not a run for him, so it lets go of nothing either)
import './he40';
import './weapons';
import './stickycam';
import './lightswitch';
import './weapons';
import { ringTag } from './gear';   // (a leaf: the kit's names as issued, applied to the lines that name it)
import { AIM_HOOKS } from './aim';
import './aimcost';
import './lighting';
import './carlamps';
import { GUNWALL, gunWallIs, woodBetween } from './gunwall';
import './friends';
import { fallingInYields, noShoveBack, noShoveNote, type Shoved } from './noshove';
import './samstun';
import { sprintDoorPace, sprintDoorLean, type DoorStamps } from './sprintdoor';
import './doorblast';
import './aimweight';
import { HOLD } from './character';
import { WALLSNAP } from './wallsnap';
import { THROWBACK } from './throwback';
import type { HoldPhase, HoldVariant } from './character';
import type { Game, Guard, Player, ThrowKind, Interactable } from './game';
import { endGuardScript } from './squad';
import { resetChase, inHand, gunBack } from './guards';
import './whistle';
import { wandDrop } from './wand';
import './wallcharge';
import './defuse';
import { STALK, stalkOn } from './stalk';
import { TAPSTEP } from './tapstep';
import { closeLive, closeAt, closeKindOf, closeTook, closeSeated, closeUnseat } from './close';
import './kitanim';
import type { CloseKind } from './close';
import './wet';
import { bodyGripDist, haulBegin, haulSpeed, haulCrouch, handsCancel } from './hands';
import { stopCoding } from './devices';
import { civilianGrabbed, civiliansShown } from './civilians';
import { samKeyLineFor } from './dialogue';
import { simRandom } from '../math/dice';   // the simulation's die (math/dice.ts): the Game's own when it has one, Math.random otherwise
import { WORDS } from './words';   // (a leaf: the dialogue pass's one switch)
import { takeFall } from './takedown';   // a takedown's own move and the fall it leaves (takedown.ts): poses only
import { word } from './belief';
import { RUST_PASS, RUST_SURFACE, crossToRust, goneWithTick } from './crossing';

/** the player's movement circle: wider than the body so arms / gun don't clip into walls */
const BODY_R = 0.42;
/** living guards are solid to you out to this (centre to centre) */
export const KEEP_OFF = 0.55;

/** how long Sam's line hangs over him before the man answers (s): a beat, stretched a little for a longer line — SAM_MAX_CHARS keeps them to a clause or two */
export function samBeat(chars: number): number { return goneWithTick('samBeat') as any; }
/** hearing levels ('prop' events at the pair, the room's business not the floor's — the takedown's thud is 0.3): the scuffle of the grab, the choke going on, him
 *  hitting the floor at the end of it, the crack of the pistol on his skull and him going down under it (the loudest of them: a whip is quick, not quiet) */
const GRAB_NOISE = 0.12, FOLD_NOISE = 0.3;
/** firing past the man in the arm (the gun variant, LMB): one-handed from beside his jaw — seconds between rounds (single, deliberate shots) and the spread they go
 *  with (combat.ts fireWeapon: ± half of it per axis on the unit direction ≈ ±2.6° — a torso at 6 m mostly, not always) */
export const HOLD_FIRE_CD = 0.7;
/** Player.holding while Sam has a man from behind. phase / t: the beat (HOLD.secs) — 'grab' easing in behind him and the arm coming round (he is frozen, not yet
 *  turned), 'held' the pair locked HOLD.dist apart on Sam's facing, 'choke' the same with the arm closing (chosen off its row, it runs to HOLD.secs.choke on its
 *  own; a Space tap, a step or Esc sooner = he breathes), 'whip' the pistol coming off his head and down behind his ear (likewise: stopped before HOLD.secs.whip
 *  it never lands). since: game time the hold proper began. windT: the commit beat — seconds since the choke / the whip was chosen (−1 = not chosen): at
 *  HOLD.secs.chokeHold the arm closes / the pistol cocks back (the timing the held key used to have; it does not run while the hand is between tools). variant:
 *  set at the grab by what was in the hand — 'arm' (holstered, or a canister up: both hands on him, the knife at his throat; his rows are the question and the
 *  choke) or 'gun' (the Five-seveN drawn: it goes to his head; a click fires past him, his one row is the whip, and no question is put over a gun) — and
 *  SWAPPED by E once he is held (swapHold: he draws / holsters with the man in the arm). swapT: seconds since that last happened (Infinity = never this hold):
 *  inside HOLD.secs.swap the hand is still changing tools and the row starts nothing. remarked: Sam has already said his piece on drawing this hold (one remark
 *  per hold). gunOut: 0‥1, the pistol presented off his head to fire (eased here, posed by the layer); fireQueued: a click waiting the few frames that takes;
 *  shotT: game time of the last round past him (−1 none: his flinch, and how long the gun stays presented). ask: the exchange in flight — Sam's line went up
 *  at `t` for `dur` (his bubble); the man's answer follows when it ends (updateHold → answerHeld); null between. after: a remark of Sam's waiting for the
 *  man to finish — his word on the key he has just pocketed (takeKey) — put up the first frame the answer is down and no question is going up, dropped by
 *  whatever kills a pending question (cancelAsk) or by the next question beating it to the air (askHeld); null when none waits. */
export interface Hold { g: Guard; variant: HoldVariant; phase: HoldPhase; t: number; since: number; windT: number; swapT: number; remarked: boolean; gunOut: number; fireQueued: boolean; shotT: number; ask: { t: number; dur: number } | null; after: { text: string; id: string } | null;
  /** the grab's disarm while it runs (disarm.ts: begun by startGrab, moved on once a frame by disarmTick, gone when the hand is back on him) */
  strip?: Strip;
  /** the stalk's grab (stalk.ts, game.ts): where Sam stood as it fired. The way in is then ONE step from there over STALK.catchSecs, where the grab's own
   *  is a snap (a fifth of the gap in its first frame). Unset = the grab as it was. `hitAt` = the game's time as that step landed (the yank's clock, for
   *  both poses); `yankM` / `yanked` = how far the man is hauled back after it (STALK.yank, less where there is no room) and how much of that is done */
  from?: Vec3; hitAt?: number; yankM?: number; yanked?: number; stepE?: number; stepM?: number; yaw0?: number; coil0?: number;
  /** close.ts CLOSE.rmbHold: the right button is on him (he was taken by it held down, or it was pressed in the arm) — 1 = the arm closes as soon as it has
   *  him, 2 = it is closed and stays so while the button is down (let go, or the choke stopped by anything else: unset, and the hold is a hold) */
  rmbChoke?: number; /** …and for how long the button has been down on him, s (a press made in the arm starts at 0: let go inside a tap, the choke is the whole one) */ rmbT?: number;
}

/** IN A DOOR FRAME HE IS AS WIDE AS HIS SHOULDERS. BODY_R is a margin for his arms and his gun, 0.84 m across; a real door (scene/doorSize.ts) leaves 0.83 m
 *  between its stops. So the circle the statics push on narrows to a man's own (DOOR_R — the men's, guards.ts) as his centre comes onto a doorway's middle line,
 *  and widens again as he leaves it: whole within `DOOR_FIT.lat[0]` of the middle line and `DOOR_FIT.out[0]` of the frame line, gone by `lat[1]` / `out[1]`,
 *  eased between (no step in it: nothing shoves him as it changes). Beside the frame, flat on the wall, he is as wide as ever. Every door of the level, standing
 *  or off its hinges: it is the hole in the wall that is narrow. */
const DOOR_R = 0.3, DOOR_FIT = { lat: [0.12, 0.5] as const, out: [0.55, 1.0] as const };
/** his collision circle's radius against the statics with his centre at `p` (the checks' own: tools/qa — a watcher that holds him to BODY_R reads the air a
 *  door frame gives him as a wall he is inside) */
export const playerBodyR = (g: Game, p: Vec3): number => bodyR(g, p);
/** the player's own push-out of the statics for a spot he is not standing on yet (doors.ts DOORWAY.solid: where a leaf may put him — Doors.settle) */
export function settleSpot(g: Game, p: Vec3) { const y1 = g.player.crouch ? 1.0 : 1.7; for (let k = 0; k < 4; k++) { g.col.collideCircle(p, bodyR(g, p), 0.2, y1, 6); if (!g.col.clampToWorld(p, BODY_R)) break; } }
function bodyR(g: Game, p: Vec3): number {
  let k = 0;
  for (const d of g.doors.list) {
    const dx = p[0] - d.frameCentre[0], dz = p[2] - d.frameCentre[2]; if (dx * dx + dz * dz > 1.3) continue;
    const c = d.def.closedDir, ux = Math.cos(c), uz = Math.sin(c), lat = Math.abs(dx * ux + dz * uz), out = Math.abs(dz * ux - dx * uz);
    const kl = 1 - clamp((lat - DOOR_FIT.lat[0]) / (DOOR_FIT.lat[1] - DOOR_FIT.lat[0]), 0, 1), ko = 1 - clamp((out - DOOR_FIT.out[0]) / (DOOR_FIT.out[1] - DOOR_FIT.out[0]), 0, 1);
    k = Math.max(k, kl * kl * (3 - 2 * kl) * ko * ko * (3 - 2 * ko));
  }
  return BODY_R - (BODY_R - DOOR_R) * k;
}
/** Longest stride the collision sweep takes in one go: from a settled spot (the circle clear of every face) a stride this short cannot carry the centre past the
 *  middle of the thinnest thing in the level (8 cm cubicle panels: 0.42 + 0.04), so every push-out sends him back the way he came — never out of the far side. */
const SWEEP_STEP = 0.2;
/** …the same settle for any body with a place and the spot it last stood clear at: the player's own (collidePlayer) and the scratch body of a look ahead
 *  (walkahead.ts), which is why it takes the body and not the game's man */
export function collideMan(g: Game, pl: { char: { pos: Vec3 }; safe: Vec3 | null }, y1: number) {
  const p = pl.char.pos, col = g.col, s = pl.safe;
  if (s) {
    const dx = p[0] - s[0], dz = p[2] - s[2]; const n = Math.min(32, Math.ceil(Math.hypot(dx, dz) / SWEEP_STEP));
    if (n > 1) { p[0] = s[0]; p[2] = s[2]; for (let i = 1; i < n; i++) { p[0] += dx / n; p[2] += dz / n; col.collideCircle(p, bodyR(g, p), 0.2, y1, 4); col.clampToWorld(p, BODY_R); } p[0] += dx / n; p[2] += dz / n; }
  }
  for (let k = 0; k < 4; k++) { col.collideCircle(p, bodyR(g, p), 0.2, y1, 6); if (!col.clampToWorld(p, BODY_R)) break; }   // (6 passes: three walls and a shelf end meeting in a pocket left him centimetres inside one of them at 3; and where the world edge and a static leave less than his width — the van's corner on the lot's rim — the two are alternated until they agree on the mouth of the wedge instead of each undoing the other)
  if (s) { s[0] = p[0]; s[1] = 0; s[2] = p[2]; } else pl.safe = [p[0], 0, p[2]];
}
/** May he stand up here? Anything static in the column between crouched head height and standing head height (a counter top's overhang, a shelf, a desk he
 *  crept under) says no — otherwise standing switches the collision headroom 1.0 → 1.7 and the sweep would shove him a foot out through the slab in one frame
 *  (the QA soak's 'popped 0.30 m' under the reception counter's north overhang). Non-destructive: settles a COPY of his spot against that band only. */
export function canStand(g: Game): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('canStand', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } } return canStandAt(g, g.player.char.pos); }
/** …asked of a spot (the look ahead's scratch body: walkahead.ts) */
export function canStandAt(g: Game, p: Vec3): boolean {
  const q: Vec3 = [p[0], p[1], p[2]];
  g.col.collideCircle(q, bodyR(g, p), 1.0, 1.7, 2);   // (as wide as he is HERE: in a door frame his shoulders are turned — bodyR)
  return Math.hypot(q[0] - p[0], q[2] - p[2]) < 0.01;
}

/** TWO GEARS, NOT A DIAL (`on`; off = the wheel sets his pace and the crouch-walk is tune.playerCrouch, as it always was). The wheel's notches bought nothing a
 *  man sneaking could use: a footfall's level is the stance's × the floor's whatever the notch, a crouched step is under a man's ear at any pace, and one notch
 *  down a crouched man no longer gains on a patrol (1.13 against tune.guardWalk 1.15). On: the wheel is nobody's pace (his pace is whole, a saved notch
 *  included), the walk and the sprint are as they were, and the crouch-walk IS the sneak, at `crouch` m/s — under the walk, so the walk keeps its job, and
 *  0.3 m/s over a patrol's (the two metres from the close-in mode's edge to reach in about six seconds; it was thirteen). The sidle along a wall and a
 *  hauled body keep tune.playerCrouch. ON in the page unless ?twogears=0 (src/host/profile.ts pageSwitches); off here and on the harness's frozen stand
 *  (src/host/standup.ts; TPS_TWOGEARS=1 turns it on there). */
export const TWOGEARS = { on: false, crouch: 1.45 };
/** SHIFT IN A CROUCH IS A FAST CROUCH WALK (the maintainer, 10-23: "we can make sprint when crouching a really fast crouch walk, but you have to stand up to sprint
 *  properly"). With `on`, Shift held while he is crouched no longer stands him up into the sprint: he stays down and covers ground at `pace` (m/s; between
 *  the walk's 1.55 and the run), each footfall at `loud` (the crouch walk's is 0.125, the walk's 0.26, a run's 0.6: quicker than walking and heard further
 *  than walking, a good deal less than running), and it is no sneak (unseen.ts samSneaks). The sprint is the standing man's: [C] first. He is not
 *  `sprinting` — his hands are still his own (a door, a switch, a grab) — and a hold on left click (stalk.ts) is the creep whatever Shift says, as before.
 *  A body in tow is let go of, as a sprint's is. Not under a puppet or a tour (their Shift is the old one). Off = Shift stands him up, as it did.
 *  The page turns it on (?crouchrun=0 puts it back); off here and on the frozen stand (TPS_CROUCHRUN=1). Its look: character.ts DASH. */
export const CROUCHRUN = { on: false, pace: 2.5, loud: 0.34 };
/** the crouch-walk's top speed: the sneak's own under TWOGEARS, else the tune's */
export const crouchPace = (g: Game): number => TWOGEARS.on ? TWOGEARS.crouch : g.tune.playerCrouch;
/** Walking pace on the wheel: PACE_MIN of the walk / crouch-walk speed at the slowest notch, 1 at the fastest, PACE_STEP a notch. */
export const PACE_MIN = 0.35, PACE_STEP = 0.13;
/** Sam's hands on the FN2000 (docs/internal/sc20k-design.md §5; combat.ts fireWeapon's `spread`: ± half of it per axis on the unit direction): a shouldered long gun
 *  standing still, on the move (any ground speed over `moveAt`), × crouched, × braced on the foregrip — which he now ALWAYS is with it in the hands (Player.braced:
 *  no brace input any more, so braceK is simply the rifle's in-hand steadiness and `still` × braceK the cone a standing round is drawn with) — then + the AUTO string's
 *  own growth off the row (weapons.ts AutoSpec) and + his suppression's (suppression.ts samSpread), as the pistol's rounds get. The pistol stays dead-on. */
export const RIFLE_SPREAD = { still: 0.010, moving: 0.028, moveAt: 0.3, crouchK: 0.7, braceK: 0.45 };
/** ONE FOOTFALL A STEP (`once`; off = every crossing of the cycle's half or its turn files one, as it always did). A footfall is filed when the walk cycle
 *  crosses its half or turns over. The cycle runs BACKWARDS while he backpedals, and backpedalling is decided from where he aims against where he goes: when
 *  that flips frame by frame the cycle rocks over the one edge, and every frame of it filed a footfall — seven running in a scripted creep up to a man's back,
 *  which is seven crouched steps in a tenth of a second to the man's ears. On: a crossing files a footfall only when the cycle has TRAVELLED `travel` of a turn
 *  (either way, added up) since the last footfall filed. A plain walk travels half a turn between its edges and files what it always did; a cycle rocking on
 *  an edge files at twice a walk's rate at the most, not sixty a second; a turn about within an eighth of a cycle of an edge loses that one footfall. (A FIRST
 *  CUT asked only 'is this the edge last crossed?': a man pacing, or waving the cursor across himself twice a second, then walked the whole floor in silence.)
 *  A FOOTFALL THE LEGS OWE (`owed`, a key of its own UNDER `once`: with the first key down it does nothing). A cycle that is turned about again and again
 *  (the cursor shaken across him three times a second: forwards, backwards, forwards) either rocks between the two edges and reaches neither, or crosses one
 *  with too little travel behind it to count: with the first key alone that walk files a third to two thirds of a plain walk's footfalls over the same
 *  floor. On: the cycle's travel is also added up since the last EDGE PASSED (filed or not) or footfall filed, on the frames he moves at a footfall's speed,
 *  and when it comes to `owe` of a turn (a half: one step's worth of leg) that step is filed where the legs are. A steady walk comes to its edge first,
 *  every step, and files what it always did; after a footfall owed mid-stride the next edge passed puts the count back on the feet. The count starts over
 *  when he slows under a footfall's speed (the cycle easing to a stop is not a step, and a debt is not kept while he stands) and on the first frame the
 *  test is asked again after frames it was not (a takedown, a keypad, a kick: those beats walk or root him without it). */
export const FOOTFALL = { once: false, travel: 0.25, owed: false, owe: 0.5 };
AIM_HOOKS.playerExternalK = (g: Game) => (g.player.braced ? RIFLE_SPREAD.braceK : 1);   // the foregrip brace steadies the aim model's cone too (aim.ts hook), not only the rifle's own hands term
/** seconds the rifle takes coming off the sling into the hands (or going back, or the pistol coming out in its place): no round and no ring inside it */
export const RIFLE_EQUIP_SECS = 0.45;
/** what the selected slot puts in his hand, for the HUD line */
const itemName = (pl: Player) => pl.slot === 'pistol' ? pl.pistol.def.short : pl.slot === 'smoke' ? 'smoke canister' : pl.slot === 'flash' ? 'stun canister' : pl.slot === 'frag' ? 'frag grenade' : rifleName(pl);
const rifleName = (pl: Player) => `${pl.rifle.def.short} · ${pl.rifle.mode.toUpperCase()}${pl.rifle.attachment === 'ring' ? ` · ${ringTag()} ×${pl.rifle.rings}` : pl.rifle.attachment === 'he40' ? ` · 40 MM ×${pl.rifle.shells}` : pl.rifle.attachment === 'cam' ? ` · CAM ×${pl.rifle.cams}` : ''}`;
/** Put the selected item away (E; taking hold of a wall) or draw it again (E; stepping off one): a reload in hand is abandoned on holstering, and unless `quiet`
 *  the HUD line says which and the holster sounds. The rig, the rail light, the takedown click and every path of the pistol read Player.holstered itself; with the
 *  rifle selected 'holstered' is SLUNG — across his back — and drawing it is RIFLE_EQUIP_SECS of no fire (equipT). */
function setHolstered(g: Game, on: boolean, quiet = false) {
  const pl = g.player; if (pl.holstered === on) return;
  pl.holstered = on;
  if (on && pl.pistol.reloading) { pl.pistol.cancelReload(); pl.char.anim.cancelReload(); }
  if (on && pl.rifle.reloading) { pl.rifle.cancelReload(); pl.char.anim.cancelReload(); }
  if (pl.slot === 'rifle') pl.equipT = RIFLE_EQUIP_SECS;
  if (!quiet) { g.msg(on ? (pl.rifleSel ? 'slung' : 'holstered') : itemName(pl)); g.audio.play('equip', null, 0.45); }
}
/** every gun's reload in hand abandoned (the takedown, the grab, the lock, the kick: the hands are wanted) */
function cancelReloads(pl: Player) { pl.pistol.cancelReload(); pl.rifle.cancelReload(); pl.char.anim.cancelReload(); }

/** ARE HIS LEGS HIS OWN: is what moves him this tick his walk keys and nothing else — not down, not in one of the beats updatePlayer returns out of before it
 *  reads them (a takedown, a man in his arm, a kick, a lock, the wand, his hands at work, a keypad), not on a wall, not with a body or a door in his hands,
 *  not creeping in on a man, not walked by a script, not stepping by taps, not in a lesson that gates his keys. A state added to updatePlayer that moves him some other way is added here: a look ahead (walkahead.ts)
 *  walks him only while this holds, and answers him where he stands while it does not. Asked, nothing written. */
export function legsOwn(g: Game): boolean {
  const pl = g.player;
  return !(pl.down || pl.takedown || pl.holding || pl.kick || pl.picking || pl.wanding || pl.coding || pl.wall || pl.dragging || pl.haul || pl.doorMove || g.puppet || g.touring
    || samWork(g) || (STALK.on && stalkOn(g)) || (TAPSTEP.on && pl.crouch) || g.rails?.allowed());   // (tapstep.ts: crouched, his keys are steps with a rhythm of their own · rails.ts: a lesson's gate takes keys out of the tick)
}
/** THE TOP PACE OF A WALK OF HIS OWN this tick, before a tap step, a creep or a door in his hand have their say: by what his legs are doing (`m`: the
 *  player's own flags in the tick; a look ahead's scratch flags — walkahead.ts) and what his card weighs. One working for both. */
export function freePace(g: Game, m: Legs, ml: number): number {
  const T = g.tune, pl = m;
  let maxSpeed = ml === 0 ? 0 : pl.sprinting ? T.playerSprint : pl.dragging ? (pl.haul ? haulSpeed(g) : T.playerCrouch * 0.6) : pl.dash ? CROUCHRUN.pace : (pl.crouch ? crouchPace(g) : T.playerWalk) * pl.pace;   // hauling 80 kg along the floor is slow; the wheel-set pace scales the walk and the crouch-walk alike (the rifle in the shoulder costs no pace: the aim model's moving / settle terms are what the move costs him)
  if (g.puppet?.walk !== undefined && ml > 0) maxSpeed *= g.puppet.walk;
  return samLoadPace(g, maxSpeed, pl.sprinting);   // (kitload.ts: what his card weighs — a light one's legs are quicker, a heavy one's slower, a sprint by part of it; as asked with its switch off or no card on him)
}
/** A MAN'S LEGS, as the lines below walk them: the Player's own fields by their own names, so the tick hands them `g.player` and a look ahead (walkahead.ts)
 *  a copy of him — one working for both, and nothing here reaches for `g.player` behind the body it was handed. */
export interface Legs extends Shoved, DoorStamps {
  char: { pos: Vec3; vel: Vec3; bodyYaw: number; aimYaw: number }; safe: Vec3 | null;
  crouch: boolean; sprinting: boolean; dash: boolean; backpedal: boolean; speedSm: number; pace: number; dragging: unknown; haul: unknown;
}
/** his move keys as a way across the floor, in the view's basis (not yet of unit length) */
export function keysWay(fwd: Vec3, right: Vec3, w: boolean, s: boolean, d: boolean, a: boolean): [number, number] {
  let mx = 0, mz = 0;
  if (w) { mx += fwd[0]; mz += fwd[2]; }
  if (s) { mx -= fwd[0]; mz -= fwd[2]; }
  if (d) { mx += right[0]; mz += right[2]; }
  if (a) { mx -= right[0]; mz -= right[2]; }
  return [mx, mz];
}
/** C: down, or up where there is the headroom — false: no room to stand, he stays down */
export function crouchKey(g: Game, m: Legs): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('crouchKey', undefined, [g, m]); if (__r !== RUST_PASS) return __r as any; } } if (!m.crouch) m.crouch = true; else if (canStandAt(g, m.char.pos)) m.crouch = false; else return false; return true; }
/** his stance for this stride: Shift and a direction stands him up to run (or, CROUCHRUN, keeps him down and quick); a body in his hands sets it for him */
export function sprintStance(g: Game, m: Legs, wantSprint: boolean, stalk: boolean): void {
  const dashes = CROUCHRUN.on && !g.puppet && !g.touring;
  if (wantSprint && m.crouch && canStandAt(g, m.char.pos) && !dashes) m.crouch = false;   // (under a counter top Shift just crouch-walks on: no standing into the slab)
  if (m.dragging) m.crouch = haulCrouch(g);   // (hauled along the floor: on his heels; carried on his shoulder: upright once it is half way up — hands.ts)
  m.sprinting = wantSprint && !m.crouch;
  m.dash = dashes && wantSprint && m.crouch && !m.dragging && !stalk;   // (CROUCHRUN: Shift in a crouch stays down and goes quickly)
}
/** ONE STRIDE: his pace taken up or let go, his place moved by it, the walls and then the men he meets given their say. `tap` / `stalk`: the step or the creep
 *  that has his legs this tick (tapstep.ts, stalk.ts) — null for a plain walk. `now`: the clock the tick runs at. */
export function strideStep(g: Game, m: Legs, mx: number, mz: number, ml: number, tap: { speed: number } | null, stalk: { pace: number; keep?: boolean } | null, dt: number, now = g.time): void {
  const c = m.char;
  let maxSpeed = freePace(g, m, ml);
  if (tap) maxSpeed = tap.speed;
  if (stalk) maxSpeed = ml === 0 ? 0 : stalk.pace;   // (stalk.ts: the man's own speed and a closing speed that eases off as he nears — the game's, whatever his card weighs)
  if (m.sprinting) maxSpeed *= sprintDoorPace(g, m, now);   // (sprintdoor.ts: his hand is ripping a door round toward him — the run waits for it; 1 otherwise)
  m.speedSm = damp(m.speedSm, maxSpeed, stalk ? (stalk.keep ? 12 : maxSpeed > m.speedSm ? STALK.easeUp : STALK.easeDown) : maxSpeed > m.speedSm ? 6 : 12, dt);   // (stalk.ts: the creep takes up its speed more gently than a walk)
  if (tap) m.speedSm = tap.speed;
  if (tap ? tap.speed > 0 : ml > 0) { c.vel = [mx * m.speedSm, 0, mz * m.speedSm]; } else { c.vel = v3.scale(c.vel, Math.exp(-14 * dt)); }
  c.pos = v3.mad(c.pos, c.vel, dt);
  const headroom = m.crouch ? 1.0 : 1.7;
  collideMan(g, m, headroom);
  keepOffGuards(g, dt, headroom, m);
}
/** is his gun up to aim: the pistol in his hand just fired or with its light on, the rifle braced (`now`: the clock asked of) */
export function gunRaised(g: Game, pl: Player, now = g.time, m: { sprinting: boolean } = pl): boolean { return (pl.pistolUp && (now - pl.lastFireT < 1.6 || pl.pistol.lightOn || pl.aimHeld)) || pl.bracedAt(m.sprinting); }   // (`m`: whose legs say whether he is at a run — his own, or a look ahead's copy of him)
/** the way a leaf he is pushing open turns him: square into its face, unless the gun is up or it merely rests on him — null: no leaf has a say */
export function leafFacing(dp: { w: number; n: ArrayLike<number>; holdOnly?: boolean } | null | undefined, aiming: boolean): number | null { return dp && dp.w >= 1 && !aiming && !dp.holdOnly ? Math.atan2(-dp.n[0], -dp.n[2]) : null; }
/** A SPRINT'S FACING: gun down, the body follows the run and the aim is the body's. Returns the shoulder's turn into a door just run open (sprintdoor.ts; null: none). */
export function sprintFacing(g: Game, m: Legs, dt: number, now = g.time): number | null {
  const c = m.char, lean = sprintDoorLean(g, m, now);
  c.bodyYaw = approachAngle(c.bodyYaw, v3.yawOf(c.vel) + (lean ?? 0), 14 * dt); c.aimYaw = c.bodyYaw; m.backpedal = false;
  return lean;
}
/** THE HIPS OF A WALK OF HIS OWN, his aim already turned: they follow the legs' travel (or its reverse, backing up) and yield toward the aim; standing, they
 *  come round only when the aim has gone far enough off them. `intoDoor` / `byHand`: a leaf he is walking open or working by hand turns them instead. */
export function hipsTurn(m: Legs, speed: number, aiming: boolean, orbit: boolean, intoDoor: number | null, byHand: boolean, dt: number): void {
  const c = m.char;
  if (speed > 0.2) {
    const moveYaw = v3.yawOf(c.vel);
    const d = Math.cos(wrapAngle(c.aimYaw - moveYaw));
    if (byHand) m.backpedal = Math.cos(wrapAngle(intoDoor! - moveYaw)) < 0;
    else if (intoDoor !== null) m.backpedal = false;
    else if (m.backpedal ? d > -0.05 : d < -0.35) m.backpedal = !m.backpedal;
    // hips follow the legs' travel direction (or its reverse) but yield up to ~45° toward the aim so the spine twist stays natural
    const base = m.backpedal ? wrapAngle(moveYaw + Math.PI) : moveYaw;
    const bodyTarget = byHand ? intoDoor! : intoDoor !== null ? base + clamp(wrapAngle(intoDoor - base), -0.35, 0.35) : base + clamp(wrapAngle(c.aimYaw - base), -0.8, 0.8) * (aiming || orbit ? 1 : 0.6);   // (square to the leaf, within 20° of his line: it swings as it opens and he is walking through, not round it)
    c.bodyYaw = approachAngle(c.bodyYaw, bodyTarget, (intoDoor !== null ? 12 : 9) * dt);
  } else {
    m.backpedal = false;
    const off = wrapAngle((intoDoor ?? c.aimYaw) - c.bodyYaw);
    const limit = byHand ? 0.1 : intoDoor !== null ? 0.25 : aiming ? 0.6 : 1.2;
    if (Math.abs(off) > limit) c.bodyYaw = approachAngle(c.bodyYaw, (intoDoor ?? c.aimYaw) - Math.sign(off) * limit * 0.7, (byHand ? 12 : 7) * dt);
  }
}

/** The cone Sam's next rifle round is drawn with for how he stands (RIFLE_SPREAD): still or on the move at `speed`, × crouched, × braced. The AUTO string's growth
 *  and his suppression's share are added at the fire site (they are the row's and suppression.ts's, not his stance's). */
export function rifleSpread(g: Game, speed: number): number { return goneWithTick('rifleSpread') as any; }

/** Living guards are solid to you and never step aside (updateGuard / runGuardScript separate guards from each other only — a man must not be blocked or steered by a
 *  player he may not even know is there), so you are the one who yields, and every shove is collided with the statics and the world edge AGAIN, so none ever
 *  leaves you standing in a wall for the frame (or, pinned to a 12 cm partition, popped out of its far side):
 *  - a man who STANDS (or whom you run into): pushed straight back out to KEEP_OFF — capped above sprint speed, so solid to anything you do, while a man a script
 *    drops on top of you eases you off over a few frames instead of popping you half a metre; squeezing between him and a wall you slide along the wall past him;
 *  - a man WALKING into you (you within ~50° of dead ahead of the leg he is on): shouldered SIDEWAYS off his line of travel — to the side of it you are on if the
 *    statics leave your body the room to clear him there, else across it if the other side does, else to whichever side has the more room (he brushes through
 *    what is left of the overlap), and only where neither has any to speak of (a narrow doorway) does he simply walk through you — never straight ahead of him,
 *    which would carry you the length of the corridor; less squarely in front, straight away from him minus the share that is only his stride closing; and
 *    wherever the statics eat the shove (pinned to the wall beside his track) you stay where you stand rather than the leftover feeding you along the wall ahead
 *    of him frame after frame. In live play a man coming on has seen you long before he touches you; this is what the tour's unseen Sam and the sandbox get.
 *  `y1` = the headroom updatePlayer collided you with this frame. */
function keepOffGuards(g: Game, dt: number, y1: number, m: Legs = g.player) {   // (`m`: the body kept off them — the player's own, or a look ahead's copy of him, walkahead.ts)
  const p = m.char.pos;
  const room = (sx: number, sz: number) => { const hit = g.col.raycast([p[0], 0.45, p[2]], [sx, 0, sz], 1.5); return (hit ? hit.t : 1.5) - BODY_R; };   // how far your centre can travel that way before your circle meets a static (or a leaf) at shin height
  for (const gd of g.guards) {
    if (gd.state === 'dead' || gd.held) continue;   // (the man in his arm is placed off him, not kept off him: updateHold)
    const dx = p[0] - gd.char.pos[0], dz = p[2] - gd.char.pos[2]; const l = Math.hypot(dx, dz);
    if (l >= KEEP_OFF) continue;
    if (fallingInYields(g, gd)) {   // (noshove.ts: a man of his own falling in is the one who yields — only what his own legs closed on the man is taken back)
      const back = l > 1e-4 ? noShoveBack(g, gd.char.pos[0], gd.char.pos[2], l, m) : 0;
      if (back > 0) { const bx = p[0], bz = p[2]; p[0] += dx / l * back; p[2] += dz / l * back; collideMan(g, m, y1); noShoveNote(g, p[0] - bx, p[2] - bz, m); }
      continue;
    }
    const gv = gd.char.vel; const gs = Math.hypot(gv[0], gv[2]); const walking = gs > 0.2;
    let nx: number, nz: number;   // straight away from him (dead on his centre: to the right of wherever he is heading)
    if (l > 1e-4) { nx = dx / l; nz = dz / l; } else if (gs > 1e-3) { nx = gv[2] / gs; nz = -gv[0] / gs; } else { nx = 1; nz = 0; }
    let cap = 6, fx = 0, fz = 0, ahead = 0;   // his line of travel — the leg to his next waypoint (steady while he pivots onto it through you), else his velocity — and how squarely you are in front on it (cos)
    if (walking) {
      const wp = gd.pathI < gd.path.length ? gd.path[gd.pathI] : null; const wx = wp ? wp[0] - gd.char.pos[0] : 0, wz = wp ? wp[2] - gd.char.pos[2] : 0, wl = Math.hypot(wx, wz);
      if (wl > 0.3) { fx = wx / wl; fz = wz / wl; } else { fx = gv[0] / gs; fz = gv[2] / gs; }
      ahead = nx * fx + nz * fz;
      if (ahead > 0.64) {
        let sx = nx - ahead * fx, sz = nz - ahead * fz; const sl = Math.hypot(sx, sz); let b = l * sl;   // your side of his line, and how far off it you already are
        if (b >= 0.04) { sx /= sl; sz /= sl; } else { sx = fz; sz = -fx; b = 0; }                      // (square in his way: his right)
        const near = b + room(sx, sz), far = room(-sx, -sz) - b;                                        // how far off his line you can get on your side / by crossing to the other
        if (near < KEEP_OFF && (far >= KEEP_OFF || far > near + 0.25)) { if (far < 0.25) continue; sx = -sx; sz = -sz; } else if (near < 0.25) continue;   // (crossing his line only for clearly more room; none worth having either way — a narrow doorway — he walks through you)
        nx = sx; nz = sz; cap = 4; ahead = 0;
      }
    }
    const push = Math.min(KEEP_OFF - l, cap * dt); const bx = p[0], bz = p[2];
    p[0] += nx * push; p[2] += nz * push;
    if (ahead > 0) { const his = ahead * Math.min(push, gs * dt * ahead); p[0] -= fx * his; p[2] -= fz * his; }   // (the forward share of a straight shove that is only his stride closing on you this frame is not yours to travel: half beside him you would still ride along)
    collideMan(g, m, y1);
    if (walking && (p[0] - bx) * nx + (p[2] - bz) * nz < 0.5 * push) { p[0] = bx; p[2] = bz; const s = m.safe; if (s) { s[0] = bx; s[2] = bz; } }   // the statics ate it: pinned — stay put (bx, bz is already a settled spot) and let him brush through
    noShoveNote(g, p[0] - bx, p[2] - bz, m);   // (noshove.ts: what a man moved him by is not his walking — nothing with its switch off)
  }
}

// ------------------------------------------------------------ back to the wall (Q) — Chaos Theory's wall hug: your back to a wall face, sidling along it, peeking round its end
/** how far beyond the body circle a face may stand for Q to reach it (m) */
const WALL_REACH = 0.7;
/** the shortest run of face worth putting your back to: both shoulders on it */
const WALL_MIN_FACE = 2 * BODY_R + 0.06;

/** Player.wall while his back is to a wall. The face is the plane {q · n = d}, n out of the wall into the room, along = [−n.z, 0, n.x] (his right when flat to it);
 *  [lo, hi] is the run of coplanar, tall-enough STATIC face along `along`, traced once when he took hold (statics never move; leaves and furniture are not wall),
 *  openLo / openHi whether the way on past each end is open (an outside corner, a jamb: somewhere to peek round) or walled (an inside corner: he simply stops).
 *  u = his station along it (clamped so the whole body stays on the face), v = signed sidle speed, side = the way he last went (which shoulder leads, which way
 *  the head turns), off = how far something is holding him off the plane right now (eases back to 0 as it lets him), awayT = stick-held-away clock, settle = 0→1
 *  ease onto the face from `from` (where Q found him), peekDir = the end the current (or fading) peek is round. atEdge / peek / crouched are the read-outs game.ts documents. */
export interface WallHold {
  n: Vec3; along: Vec3; d: number; lo: number; hi: number; openLo: boolean; openHi: boolean;
  u: number; v: number; side: -1 | 1; off: number; awayT: number; settle: number; from: Vec3;
  /** the way (±1) something the trace could not see stopped him, and for how long it has: no re-trying it every frame (cleared when the stick lets off that way, or after half a second for another look) */
  blockDir: -1 | 0 | 1; blockT: number;
  /** has his back actually reached the face yet (a read-back found him within the shove allowance of the plane)? Only for the words when he comes off: shoved
   *  off a wall he was on, or never got to it — a carton stack, a chair, a man between him and it that the face trace could not see */
  seated: boolean;
  /** was the selected item in his hand when he took the wall? On the BACK hold the hands are always empty (wallSnap holsters it: Chaos Theory's wall hug is a hands-on-
   *  the-plaster pose, and nothing is fired from it); the frame he steps off, releaseWall puts it back in his hand if this says so. E on the wall flips it,
   *  1 / 2 / 3 set it (updatePlayer) — choosing what he comes off with. (The rifle is the exception under WALLSNAP.stackDraw: it comes into his hands on the wall and the hold becomes the stack.) */
  drawn: boolean;
  atEdge: -1 | 0 | 1; peek: number; peekDir: -1 | 1; crouched: boolean;
  /** WALLSNAP (wallsnap.ts) — absent, all of them, on a hold taken with the switch off. takeT = the seconds this take runs (the glide and the turn share it: `settle`
   *  counts through it), yaw0 / yawD = the body's yaw when Q found him and the turn he makes over the take, hitT = seconds since his back landed (the settle). */
  takeT?: number; yaw0?: number; yawD?: number; hitT?: number;
  /** …and the STACK: the hold taken with the rifle in his hands keeps it there — shoulder to the wall, facing along it toward `side`, low ready. up = seconds the
   *  rifle stays in the shoulder (a click, or stepped out), lean = 0‥1 stepped out past the open end `leanDir` (leanWant = where it is going, leanDwell / leanOutT
   *  its clocks, leanOff = the metres along the wall he actually stands past his station for it this frame — the read-back takes them off), flipT = the
   *  cursor-behind-him clock, sayT = when the 'wall in the way' line was last said. */
  kind?: 'stack'; up?: number; lean?: number; leanWant?: boolean; leanDir?: -1 | 1; leanDwell?: number; leanOutT?: number; leanOff?: number; leanMore?: number; flipT?: number; sayT?: number; rise?: number; owed?: number; camAlong?: number; cam?: FollowCamera;
  /** WALLSNAP.pressRun: the hold was taken AT A RUN and that run is still held — the move keys that were down as he landed (moveKeysDown's bits). While
   *  Shift stays down and those keys stand as they were the run is not a way off this wall (wallRanOn); gone, for the rest of the hold, the first frame
   *  either changes. Absent on every other hold. */
  ran?: number;
}






/** For the HUD / the input pass: is he free to put his back to a wall, and would Q find one from where he stands? (Cheap enough to ask every frame: a ring of
 *  short rays and a metre of face traced either side of him.) */
export function canWallSnap(g: Game): boolean { return crossToRust('canWallSnap', undefined, [g]) as any; }


/** Step off the wall — Q again, a sprint, walking away from it, being shoved off it, or anything that plants him elsewhere (takedown, kick, pick, drag, teleport,
 *  going down): the ready stance and the craned neck come off, nothing carries him on along or into it, and what he had in his hand when he took it is back in
 *  it (WallHold.drawn — not for a man going down: hitPlayer has already settled what leaves a falling hand). */
export function releaseWall(g: Game) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('releaseWall', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } }
  const pl = g.player, w = pl.wall; if (!w) return;
  pl.wall = null; pl.char.anim.stance = 'none'; pl.char.anim.lookYawExtra = 0; pl.char.vel = [0, 0, 0];
  pl.char.anim.wallTake = -1; pl.char.anim.wallHit = 0; pl.char.anim.stackPress = 0; pl.char.anim.stanceIn = 0;   // (WALLSNAP's channels at rest: where they stand anyway with the switch off)
  if (w.cam) w.cam.peekOffset = null;   // (…and the look-at it eased into the room: a takedown, a kick, a teleport lets go after this frame's camera line has run)
  if (w.drawn && !pl.down) setHolstered(g, false);
}

// ---- WALLSNAP (wallsnap.ts): the hold's feel pass. Everything below is reached only with WALLSNAP.on — except wallStepOff, which with it off IS releaseWall.



/** Q pressed with no face in reach while he is on the move: remembered for WALLSNAP.buffer seconds (the words wait with it). `at` = the game clock the last
 *  time it was looked at: a frame that went to something else (a takedown, a man in his arm, a kick, a lock, a keypad return before the Q lines) drops it. */
const wsPend = new SideMap<Player, { t: number; at: number; blocked: boolean }>('player.wsPend');

/** WALLSNAP.press: the wall taken WITHOUT the key. A move key held INTO a face (pressFace: the stick within pressDeg of square to it, the face within pressNear
 *  of his body) for pressSecs takes it exactly as Q would — wallSnap, kept to that face: the same faces, the same refusals, and nothing said where there is
 *  none. The clock runs on the walk up, so a man who walks squarely at a wall has it from pressFlow short of touching and his back comes round as he arrives;
 *  one who turns into a wall he stands against waits the whole of it there. At a run only with WALLSNAP.pressRun (below). NEVER with his hands on anything, a list or an order menu up, a mouse
 *  button held (he is shooting, or about to), the gun in his hand reloading or still coming out, or inside pressAfterShot of a round of his own. With a
 *  sidearm or a can in the hand, which the hold puts away, only while no enemy is alert (pressCalm: in a fight the pistol stays up; the rifle stacks in his
 *  hands either way). Where the face he walks into has a thing to use on it or beside it (pressThingAt: a door, a switch, the breaker, the mission's thing,
 *  a body) the wait is pressThing and all of it against the face: he walked up to use it.
 *  THE LATCH (`n`, `dx dz`, `until`): when a hold ends, or the wall refuses a take, the stick as it then stands is remembered with that wall. Until the stick
 *  rests or turns (pressTurn), a wall facing as that one does is not taken at all — Q, Shift or a step away is not undone by the key still held, and a
 *  refusal is not asked again every frame — and any other wall not for pressAgain: stepping off does not throw him onto the far side of a narrow passage,
 *  while a walk straight across a corridor has the far wall as he arrives.
 *  AT A RUN (WALLSNAP.pressRun): Shift held changes nothing above — the same stick, clock, faces, refusals and latch — but where the clock starts: as far out
 *  as his pace into the face needs for the wait to be done pressFlow short of it (wsPressNear), so a sprint has the wall as it arrives, as a walk does. (A
 *  thing on the face is read as at a walk, off the rows in reach: a door's, the breaker's and the mission's thing are out of a sprinting man's reach, so
 *  there the wall is his as he arrives and the row is there from the hold; a body's row, and any row in the crouched dash, is in reach and has the long
 *  wait.) The hold it takes carries the keys of that run (WallHold.ran): while they are held as they were, Shift is not a way off it (wallRanOn). */
const wsPress = new SideMap<Player, { t: number; at: number; n: Vec3 | null; dx: number; dz: number; held: boolean; until: number }>('player.wsPress');
/** the things that stand where they are and are worked by hand (the wall press's touch; interact.ts emptyThing: what an empty tap can be beside) */
export const PRESS_THINGS: ReadonlySet<string> = new Set(['door', 'switch', 'breaker', 'objective', 'cam', 'body']);

// ---- the hold in a checkpoint (savestate.ts SaveState.wallHold, under SAVETIMELINE.holds). No file held a hold before the wall could be taken by walking: a
// load stood him free of it, the pistol the hold had put away still away. Now a save finds him on a wall as often as not, and the seconds after a load must be
// the seconds after the save: the hold as it stands (all but the camera it eases, which the next frame hands it again) and the key-less take's clock and latch.
// WH names every field of WallHold and how a load reads it: a field added to the hold and not named here does not compile.
type WhKind = 'vec' | 'num' | 'unit' | 'opt' | 'bool' | 'optbool' | 'sign' | 'sign0' | 'optsign' | 'stack' | 'derived';
const WH = {
  n: 'vec', along: 'derived', from: 'vec', d: 'num', lo: 'num', hi: 'num', u: 'num', v: 'num', off: 'num', awayT: 'num', blockT: 'num', settle: 'unit', peek: 'unit',
  openLo: 'bool', openHi: 'bool', seated: 'bool', drawn: 'bool', crouched: 'bool', side: 'sign', peekDir: 'sign', blockDir: 'sign0', atEdge: 'sign0',
  takeT: 'opt', yaw0: 'opt', yawD: 'opt', hitT: 'opt', ran: 'opt', up: 'opt', lean: 'opt', leanDwell: 'opt', leanOutT: 'opt', leanOff: 'opt', leanMore: 'opt', flipT: 'opt', sayT: 'opt', rise: 'opt', owed: 'opt', camAlong: 'opt',
  leanWant: 'optbool', leanDir: 'optsign', kind: 'stack',
} as const satisfies Record<keyof Omit<WallHold, 'cam'>, WhKind>;
export type WallSnap = { hold?: Record<string, unknown>; press?: { t: number; at: number; n: number[] | null; dx: number; dz: number; held: boolean; until: number } };
export function wallHoldSnapshot(g: Game): WallSnap | undefined {
  const pl = g.player, w = pl.wall, P = wsPress.get(pl), out: WallSnap = {};
  if (w) { const { cam: _cam, ...rest } = w; out.hold = { ...rest, n: [...w.n], along: [...w.along], from: [...w.from] }; }
  if (P) out.press = { t: P.t, at: P.at, n: P.n ? [...P.n] : null, dx: P.dx, dz: P.dz, held: P.held, until: P.until };
  return out.hold || out.press ? out : undefined;
}
/** …and back, onto the fresh Player a load has placed. A file that does not say, or a slice that is not a hold he could be in where he stands (a face that is
 *  not level, a run with no length, a station off it, his body not at that plane): free of the wall, as a load always left him. */
export function wallHoldRestore(g: Game, raw: unknown): void {
  if (!raw || typeof raw !== 'object') return;
  const pl = g.player, p = pl.char.pos, S = raw as WallSnap, num = (x: unknown): x is number => typeof x === 'number' && Number.isFinite(x), vec = (x: unknown): x is number[] => Array.isArray(x) && x.length === 3 && x.every(num);
  const H = S.hold;
  hold: if (H && typeof H === 'object') {
    const w: Record<string, unknown> = {};
    for (const k of Object.keys(WH) as (keyof typeof WH)[]) {
      const kind: WhKind = WH[k], v = H[k];
      if (kind === 'vec') { if (!vec(v)) break hold; w[k] = [...v]; }
      else if (kind === 'num') { if (!num(v)) break hold; w[k] = v; }
      else if (kind === 'unit') { if (!num(v)) break hold; w[k] = clamp(v, 0, 1); }
      else if (kind === 'bool') w[k] = !!v;
      else if (kind === 'sign') { if (v !== 1 && v !== -1) break hold; w[k] = v; }
      else if (kind === 'sign0') { if (v !== 1 && v !== -1 && v !== 0) break hold; w[k] = v; }
      else if (kind === 'opt') { if (num(v)) w[k] = v; }
      else if (kind === 'optbool') { if (typeof v === 'boolean') w[k] = v; }
      else if (kind === 'optsign') { if (v === 1 || v === -1) w[k] = v; }
      else if (kind === 'stack') { if (v === 'stack') w[k] = v; }
    }
    const n = w.n as Vec3, len = Math.hypot(n[0], n[2]); if (Math.abs(n[1]) > 1e-6 || Math.abs(len - 1) > 1e-3) break hold;
    w.n = [n[0] / len, 0, n[2] / len]; const N = w.n as Vec3; w.along = [-N[2], 0, N[0]];
    const d = w.d as number, lo = w.lo as number, hi = w.hi as number, u = w.u as number, off = p[0] * N[0] + p[2] * N[2] - d;
    if (hi - lo < WALL_MIN_FACE || u < lo - 1e-6 || u > hi + 1e-6 || off < BODY_R - 0.2 || off > BODY_R + WALL_REACH + 0.5) break hold;
    if (w.takeT !== undefined && !((w.takeT as number) > 0)) break hold;
    pl.wall = w as unknown as WallHold;
  }
  const P = S.press;
  if (P && typeof P === 'object' && num(P.t) && num(P.at) && num(P.dx) && num(P.dz) && num(P.until) && (P.n === null || vec(P.n)))
    wsPress.set(pl, { t: Math.max(0, P.t), at: P.at, n: P.n ? [...P.n] as Vec3 : null, dx: P.dx, dz: P.dz, held: !!P.held, until: P.until });
}



/** Esc with the launcher's button down: the gesture is let go of, nothing sent (Game.escape; the button still down afterwards begins nothing — only a fresh press does) */
/** WHERE THE CAN HE IS AIMING WILL COME TO REST (THROWBACK.restMark; overlay.ts draws the preview's ring there): the arc's own first touch is not where a can
 *  stays — it bounces and rolls on a metre or two — and the maintainer could not put a smoke on a gap by it. frag.ts restOfThrow on the arc being shown.
 *  `path`: the floor it crosses from that touch to there. Null with the mark off, with no arc up, and for the launcher's arc (its round bursts or sticks
 *  where it lands). Read only. */
export function throwRest(g: Game): { at: Vec3; path: Vec3[] } | null { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('throwRest', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } }
  const pl = g.player, sol = pl.throwPreview; if (!THROWBACK.restMark || !sol || !sol.points.length || !(pl.throwHeld || (!pl.holstered && pl.canisterSel))) return null;
  return restOfThrow(g, pl.heldKind, sol.points[0], sol.v0);
}
export function launchCancel(g: Game): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('launchCancel', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } } const pl = g.player; if (!pl.launchHeld) return false; pl.launchHeld = null; pl.throwPreview = null; return true; }
/** Stacked on the wall: is the rifle UP this frame? A click (or the trigger held; the right button too with an attachment on it) brings it into the shoulder and
 *  keeps it there WALLSNAP.upSecs; stepped out round the end it is up for as long as he is out. Never while the first WALLSNAP.bore of its line is in a wall
 *  (pointing through the wall he is stacked on): then it stays low and no round leaves — held off through equipT, the 'still coming into the hands' beat every
 *  fire site already honours. Nothing leaves the low ready either: for WALLSNAP.upRise after it starts up the click is held (equipT again) and given to the
 *  fire sites on the frame the rise is over — the first round leaves with the rifle most of the way to the shoulder, a sixth of a second after the click, not level from the thigh. */
function stackInWay(g: Game): boolean {
  const pl = g.player, c = pl.char, ap = g.aimPoint, dx = ap[0] - c.pos[0], dz = ap[2] - c.pos[2], d = Math.hypot(dx, dz);
  const hit = d > 0.25 ? g.col.raycast([c.pos[0], pl.crouch ? 0.85 : 1.3, c.pos[2]], [dx / d, 0, dz / d], WALLSNAP.bore) : null;
  return !!hit && hit.index < 1e6;
}
/** HIS GUN CANNOT GET A ROUND OUT THIS FRAME (GUNWALL.mark; overlay.ts bars the aim ring with it — the words a dead click files are in no list the page
 *  draws): held down by a wall (gunwall.ts), kept low on the stack with the wall he is on in its line (stackUp), or up with its muzzle the far side of a
 *  jamb from its shoulder (combat.ts armThrough: that round stops in the wood). Read only: nothing of the game's is written, nothing counted. */
export function gunShut(g: Game): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('gunShut', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } }
  const pl = g.player, c = pl.char; if (!GUNWALL.mark || pl.holstered) return false;
  if (gunWallIs(g, c)) return true;
  const w = pl.wall;   // (not through a step-out: from his station the point he steps out FOR is behind his wall, and the ring would flash barred each time)
  if (w?.kind === 'stack') { if (w.leanWant || w.lean! > 0 || w.leanDwell! > 0) { if (w.lean! < 1) return false; } else if (w.settle >= 1 && stackInWay(g)) return true; }
  return (c.anim.upper === 'aim' || (pl.rifleUp && c.anim.longStance === 'shoulder')) && woodBetween(g, c.bones.upperArmR ?? v3.add(c.pos, [0, 1.4, 0]), c.muzzle);   // (combat.ts armThrough's own two points; a sheet his rounds go through is no wood: gunwall.ts)
}





/** Silent takedown (a left click with the hands empty on the man in reach behind — game.ts updateDoors; the tour and the harness call it too): lock in behind the
 *  guard, throw the cross, he drops with a soft thud only the room hears. */
export function startTakedown(g: Game, gd: Guard, way?: CloseKind, lethal = false, loud = false): void { return crossToRust('startTakedown', undefined, [g, gd, way, lethal, loud]) as any; }


// ------------------------------------------------------------ grab → hold → interrogate → choke out / pistol-whip
/** Which hold a grab made now would be to begin with: the Five-seveN drawn in the hand → it goes to his head ('gun'); holstered, or a canister up → both hands on
 *  him ('arm'). The guard's row says which before you commit (buildInteractables); E changes it once he is held (swapHold) — and keeps this true of the hands
 *  throughout, so it also names the hold Sam is in. */
export function grabVariant(g: Game): HoldVariant { return g.player.pistolUp ? 'gun' : 'arm'; }
/** The 'grab him' row on a living guard from behind (game.ts updateDoors — the takedown's own gate: in reach, in his rear half-space, not alert, nothing solid
 *  between): drop whatever the hands had, and he is frozen where he stands for the beat it takes to get the arm round him (updateHold's 'grab' phase does the
 *  closing in). From here he is `held`: off the AI, off any choreography or body script (a pair clearing rooms loses him — his partner comes off it toward
 *  the spot), off the net, silent but for his answers; a patrolling man's torch is knocked out of his hand and dies on the floor. Which hold it is to begin with
 *  (grabVariant) is decided here; E swaps it later (swapHold). Refused for a man onto you, with your hands already full, or crouched where he cannot stand up
 *  into it (a counter top over his head: the hold stands the pair up and collides them at full height). */
export function startGrab(g: Game, gd: Guard, way?: CloseKind) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('startGrab', undefined, [g, gd, way]); if (__r !== RUST_PASS) return __r as any; } }
  const pl = g.player, c = pl.char;
  // closer than ever (close.ts): the way in the row offered (behind him, or behind his chair), for the tallies
  const kind: CloseKind | undefined = closeLive(g) ? way ?? closeKindOf(closeAt(g, gd, c.pos)) : undefined;
  const b0 = kind ? wrapAngle(Math.atan2(c.pos[0] - gd.char.pos[0], c.pos[2] - gd.char.pos[2]) - gd.char.bodyYaw) : 0, r0 = Math.hypot(c.pos[0] - gd.char.pos[0], c.pos[2] - gd.char.pos[2]);
  if (gd.state === 'dead' || gd.state === 'alert' || gd.held || pl.down || pl.holding || pl.takedown || pl.kick || pl.picking || pl.wanding) return;
  if (pl.crouch && !canStand(g)) { g.msg('no room to stand up behind him here'); return; }
  if (pl.rifleUp) setHolstered(g, true, true);   // the rifle in his hands goes to the sling as the arm goes round the man (Chaos Theory slings it: both hands are for him — the arm variant, the knife in the fist); it stays there when the hold is over
  const variant = grabVariant(g);
  if (pl.wall) pl.wall.drawn = false;   // (grabbed from the wall: with the empty hands he had on it — the arm variant, above — and they stay empty when it is over)
  releaseWall(g); if (pl.dragging) g.toggleDrag();
  pl.sprinting = false; pl.crouch = false; pl.speedSm = 0; c.vel = [0, 0, 0]; pl.backpedal = false; c.anim.reverse = false;   // you rise into it
  cancelReloads(pl);
  throwOff(pl);   // a wind-up does not survive it (the can goes back on the belt)
  pl.throwHeld = false; pl.throwPreview = null; pl.easing = null;
  // him
  if (gd.bubble && g.time - gd.bubble.t < gd.bubble.dur) { if (g.time - gd.bubble.t < 0.8) g.msg((WORDS.on && gd.unit ? `${gd.callsign}: ` : 'guard: ') + gd.bubble.text); gd.bubble = null; }   // whatever he was saying is cut off (logged if he had barely begun it, as killGuard does)
  const C = g.clearing;
  if (C && C.stage !== 'done' && (C.a === gd || C.b === gd)) {   // one of the pair clearing rooms: the clear is over — his partner comes off the drill toward where he was, on edge (what he then sees is his own senses' business)
    const other = C.a === gd ? C.b : C.a;
    C.cancel(); g.clearing = null;
    if (other && other.state !== 'dead' && !other.held) { other.state = 'suspicious'; other.awareness = Math.max(other.awareness, 0.6); word(g, other, gd.char.pos); other.reactT = 0.7; other.searchT = 0; other.path = []; other.pathGoal = null; other.searchPlan = null; other.pieing = null; }   // (a search plan and a half-pied doorway live inside search / alert — guards.ts drops them itself next frame, but he is 'suspicious' from THIS one: the grab lands after the guards' update)
  }
  if (gd.script) endGuardScript(gd, gd.state === 'patrol' ? 'patrol' : gd.state === 'search' ? 'search' : 'suspicious');   // (any other choreography: unhooked where he stands, keeping the state he had)
  gd.speed = 0; gd.char.vel = [0, 0, 0]; gd.path = []; gd.pathGoal = null; gd.bodyDuty = null; gd.pieing = null; gd.searchPlan = null; gd.muzzleDown = false; gd.stallRef = null;
  gd.char.anim.lookYawExtra = 0; gd.sweep = 0;
  gd.lightLitAtLoss = gd.flashlight.enabled;   // (the grab: was the torch burning as it left his hand — attachFlashlight's floor sputter shows only what was lit)
  if (inHand(gd) === 'torch') { gd.dropped = dropFromHand(g, gd.char, 'torch', gd.beamDir, [0, 0, 0], false, true); gd.torchLostAt = g.time; }   // the torch is knocked out of his hand as the arm goes round: it clatters down quietly and dies on the floor like a dropped torch does (guards.ts) — his hands come up empty
  else if (HOLD.dropGun && !HOLD.disarm && inHand(gd) === 'weapon') { gd.dropped = dropFromHand(g, gd.char, gd.weapon.def.drop, gd.char.gunDir, [0, 0, 0], gd.weapon.def.suppressed, true); gd.gunLostAt = g.time; }   // (HOLD.dropGun) …and so is the gun, if that is what the hand had: it lies where it fell like any dropped gun, and he is held with empty hands
  if (kind && !gd.civ && closeSeated(g, gd)) closeUnseat(g, gd);   // (hauled out of the chair: the sitting clip comes off as the arm goes round)
  gd.held = { by: 'player', since: g.time, variant, phase: 'grab', reverse: false };
  if (gd.civ) civilianGrabbed(g, gd);   // (a civilian: whatever she was doing stops, and she cannot scream with a forearm across her throat — civilians.ts)
  if (g.aiEnabled) gd.witness.wasHeld++; gd.talk.heldSince = g.time;   // his own record of it (the dialogue reads both: how long he has been held, whether this is the second time) — nothing is remembered with the AI off, like every other witness write
  pl.holding = { g: gd, variant, phase: 'grab', t: 0, since: g.time, windT: -1, swapT: Infinity, remarked: false, gunOut: 0, fireQueued: false, shotT: -1, ask: null, after: null };
  disarmBegin(g);   // (disarm.ts, HOLD.disarm: the gun he still has in his hand is Sam's right hand's to take, a beat from now — nothing with the switch off)
  if (kind) closeTook(g, gd, 'grab', kind, b0, r0);
  pl.fireCd = Math.max(pl.fireCd, HOLD.secs.grab);   // (no round leaves the gun on its way round him)
  disarmCatch(g, v3.add(gd.char.pos, [0, 1.4, 0]));   // the arm going round him: sleeve over his shoulder, chest into his back, the breath he snatches choked off
  g.events.push({ kind: 'prop', pos: v3.copy(gd.char.pos), time: g.time, loud: false, level: GRAB_NOISE, who: g.player.char.id }); pl.noise = Math.max(pl.noise, GRAB_NOISE); chronGrab(g, gd);   // a scuffle the room hears inside a few metres, not the floor
}


function setPhase(g: Game, phase: HoldPhase) {
  const pl = g.player, H = pl.holding; if (!H) return;
  H.phase = phase; H.t = 0; H.windT = -1; H.fireQueued = false; if (H.g.held) H.g.held.phase = phase;
  cancelAsk(g);   // the choke / the whip going in over Sam's own question
}
/** Stop the choke / the whip that is under way (interact.ts cancelSustained: a Space tap, a step or Esc while it runs — what letting go of the held key used to
 *  be): mid-choke he drags a breath and the hold goes on; mid-whip the pistol goes back to his head — either way he felt what nearly happened (talk.abortT:
 *  rattled for it, dialogue.ts agitationOf). A choke merely CHOSEN and not yet on (windT inside its commit beat) is simply un-chosen, nothing felt. */
export function easeOffHold(g: Game) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('easeOffHold', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } }
  const pl = g.player, H = pl.holding; if (!H) return; const gd = H.g;
  if (H.phase !== 'choke' && H.phase !== 'whip') { H.windT = -1; return; }
  const was = H.phase; setPhase(g, 'held'); gd.talk.abortT = g.time;
  g.msg(was === 'whip' ? `checked it — the pistol goes back to ${gd.callsign}'s head` : `eased off — ${gd.callsign} is still with you`); g.audio.play('propScrape', v3.add(gd.char.pos, [0, 1.4, 0]), 0.1, { rate: 2.2 });
}
/** Sam's unfinished question dies in his mouth: no answer is coming (nothing was asked of the engine — the man's stage stands, and the next tap puts it afresh)
 *  and the line is cut to a breath. The choke or the whip going in over it (setPhase), or the gun coming out over it (swapHold) — one rule. An answer the man has
 *  already begun is his: it runs on. A remark still waiting its turn (H.after) goes the same way: the moment for it has passed. */
function cancelAsk(g: Game) {
  const pl = g.player, H = pl.holding; if (!H) return;
  H.after = null;
  if (!H.ask) return;
  H.ask = null; if (pl.bubble) pl.bubble.dur = Math.min(pl.bubble.dur, g.time - pl.bubble.t + 0.2);
}
/** the knife's clock in the hold (character.ts HOLD.secs.kill: the move is the takedowns' — holdLayer / heldLayer pose phase 'kill' off it) */
const killSecs = (): number => HOLD.secs.kill;
/** s into the knife's clock at which the blade is in: the hold broken from there on (Sam shot down, folded) leaves the man dead all the same — freeHeld */
const killCut = (): number => HOLD.kill.hit * killSecs();
/** the knife is going in on the man in his arm (interact.ts: a commit — it is the running verb and nothing in cancelSustained stops it) */
export function holdKilling(H: Hold): boolean { return H.phase === 'kill'; }
/** Where the exchange with the man in his arm stands: `asking` — Sam's own line is up and the answer is on its way (H.ask: a tap does nothing); `samUp` — a line
 *  of Sam's is in the air, question or remark (his word on the key he pocketed, sayAfter; the one as he draws): he is never cut, so a tap does nothing either;
 *  `speaking` — the man's current bubble is his answer this hold (else null); `ready` — a tap now opens the NEXT exchange: not while asking or while Sam speaks,
 *  and only once INTERROGATE_GAP of that answer is out (a tap sooner just cuts the line short) or the answer is gone (cut short already: nothing of it left to
 *  keep — the row must not offer to cut a line nobody can see) — at once if he has said nothing this hold; `settled` — ready AND the answer down to its last
 *  ANSWER_TAIL or gone: nothing would be cut (the row's wording; the tour's beat waits for it). askHeld acts on it, the panel row words itself by it. */
export function heldTalk(g: Game, H: Hold): { asking: boolean; samUp: boolean; ready: boolean; settled: boolean; speaking: Guard['bubble'] } { return goneWithTick('heldTalk') as any; }
/** The man has just given the keyed door up — dialogue kind 'door', '…we all carry one, mine's on my belt' (game.ts interrogate calls this on that answer, held
 *  or the sandbox's bare exchange alike): Sam takes what is on his belt. Player.keys gains every key the man carried that Sam lacked and Guard.keys empties
 *  (the same man has nothing more to hand over; a second holder of a key already pocketed changes nothing — no sound, no word); for anything new: the small
 *  bright clack of the clip coming off his belt, one for the debrief (mission.ts tally, live runs only), a line in the log, the kit column's KEY tag from this
 *  frame (overlay.ts reads Player.keys) — and, with the man in his arm, Sam's word on it (dialogue.ts samKeyLineFor) queued to follow the man's line down
 *  (Hold.after → sayAfter: the two are never up together). From here his door contact and every lock test carry the key (game.ts updateDoors, Doors.lockedOut). */
export function takeKey(g: Game, gd: Guard, rnd: () => number = simRandom) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('takeKey', undefined, [g, gd, rnd]); if (__r !== RUST_PASS) return __r as any; } }
  const pl = g.player, H = pl.holding;
  const fresh = [...gd.keys].filter(k => !pl.keys.has(k)); gd.keys.clear();
  if (!fresh.length) return;
  for (const k of fresh) pl.keys.add(k);
  g.audio.play('lockPick', v3.add(gd.char.pos, [0, 1.0, 0]), 0.45, { rate: 1.6 });   // the key ring unclipped at his hip: the pick's tick, pitched up — small, bright, close
  tally(g, 'keys'); chronKey(g, gd);   // one for the card's count, one for the timeline
  g.msg(`you took ${gd.callsign}'s key. keyed doors will open for you now`);
  if (H && H.g === gd) { const line = samKeyLineFor(g, gd, H.variant, rnd); if (line) H.after = line; }
}

/** The knife run through its clock (killSecs): he is dead in the arm and let fall at Sam's feet — killGuard as the standing knife calls it ('shot' is the model's
 *  word for dead: a kill on the card, 'knife — dead'; a man who watched saw him DROPPED, not shot), then the hold's end as the choke's is: the last of him over
 *  the arm, the fold the room hears. */
function killOut(g: Game) {
  const pl = g.player, H = pl.holding; if (!H) return; const gd = H.g;
  const f = pl.char.forward();
  gd.held = null; gd.char.anim.heldPose = null; gd.talk.heldSince = -1;
  g.audio.play('chokeLimp', v3.add(gd.char.pos, [0, 1.4, 0]), 0.35);
  takeFall(g, gd, undefined, true, 'kill');   // (takedown.ts: the kill's own fall, or the old quiet one with TAKEDOWN.kill off)
  killGuard(g, gd, f, true, 'shot', 'player', 'knife — dead');
  for (const o of g.guards) { const W = o.witness.sawAct; if (W && W.kind === 'shot' && W.t === g.time && W.victim === gd.callsign) W.kind = 'struck'; }   // (startTakedown's own rewrite for the knife)
  g.events.push({ kind: 'prop', pos: v3.copy(gd.char.pos), time: g.time, loud: false, level: FOLD_NOISE, who: g.player.char.id }); pl.noise = Math.max(pl.noise, FOLD_NOISE);
  pl.holding = null;
}
/** He comes out of the arm his own man — and an instant witness: he knows exactly who had him and where. Off the hold (held, the animator's pose, the dialogue's
 *  clock), everything the AI reads for 'eyes on Sam right now' seeded (awareness full, the fix on Sam, a fresh real sighting), a stagger of `stagger` seconds
 *  (reactT) before he can act — updateGuard's own transition takes him to alert next frame with its bark and books what did it — and the jolt of it through his
 *  trunk. Back on the net too: nothing about him is missing any more. A man already down is only unhooked. */
function handBack(g: Game, gd: Guard, stagger: number) {
  gd.held = null; gd.char.anim.heldPose = null; gd.talk.heldSince = -1;
  gd.speed = 0; gd.char.vel = [0, 0, 0]; gd.path = []; gd.pathGoal = null; gd.bodyDuty = null; gd.stallRef = null; gd.char.aimPitch = 0;
  if (gd.state === 'dead') return;
  if (gd.gunLostAt >= 0) {   // (HOLD.dropGun) loose and on his feet: the gun that was knocked out of his hand at the grab is the first thing he goes for — it is off the floor and in his fist again, at the price of a longer stagger
    gunBack(g, gd); stagger += HOLD.regrab;
  }
  const pp = g.player.char.pos;
  gd.awareness = 1; gd.lastKnown = v3.copy(pp); gd.lastSeenT = g.time; gd.sightPos = v3.copy(pp); gd.sightVel = [0, 0, 0]; gd.sightT = g.time; gd.ledT = g.time;   // (ledT: nothing to lead — he was not watching Sam walk out of sight, Sam had him by the throat)
  gd.reactT = Math.max(gd.reactT, stagger); gd.heardUpTo = g.eventSeq;   // (nothing stale in the queue is news to him)
  gd.missedAt = -1; gd.missingRaised = false;
  resetChase(gd); gd.searchT = 0;
  gd.char.anim.hit();
}
/** Let go at once — Sam shot down with him in the arm, a teleport / the tour taking the hands (cancelActions): the man is handed straight back (handBack), Sam's side
 *  cleared. The one way a held man comes free on his feet: there is no letting him go. (Shot down with the knife already in him — phase 'kill' past killCut — he
 *  does not come free: killOut.) */
export function freeHeld(g: Game, why: 'shot' | 'cancel') { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('freeHeld', undefined, [g, why]); if (__r !== RUST_PASS) return __r as any; } }
  const pl = g.player, H = pl.holding; if (!H) return;
  if (why === 'shot' && holdKilling(H) && H.t >= killCut()) { killOut(g); pl.char.anim.holdPose = null; return; }   // (CLOSE.heldClicks: Sam put down or folded with the knife already in — past killCut the man dies by it all the same, the kill's own end; before it he is loose as ever. A cancel — the tour, a teleport, a unit taken off the floor — hands him back whenever it comes)
  handBack(g, H.g, why === 'shot' ? 0.6 : 0.3);
  pl.holding = null; pl.char.anim.holdPose = null;
  if (why === 'shot') { g.audio.play('shove', v3.add(H.g.char.pos, [0, 1.3, 0]), 0.3); g.msg(`${H.g.callsign} is loose`); }   // he wrenches free of the slackening arm (quieter than a shove: nobody pushed); a cancel — the tour, a teleport — makes no sound
}

// ------------------------------------------------------------ locked doors: pick it (quiet, slow) or kick it in (instant, floor-wide loud)
/** The 'pick the lock' row on a door locked against you (Game.updateDoors): drop whatever the hands were doing and start working the lock — updatePicking runs
 *  the beat on its own from here; a step, a Space tap or Esc breaks off (interact.ts cancelSustained → stopPicking). */
export function startPicking(g: Game, d: Door): void { return crossToRust('startPicking', undefined, [g, d]) as any; }
/** Hands off the tools (the pick broken off — a step, Space, Esc, the tour, a man coming through with his key — or done): the progress stays on the door a couple
 *  of seconds (doors.ts), he stays on his knee. */
export function stopPicking(g: Game, text?: string) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('stopPicking', undefined, [g, text]); if (__r !== RUST_PASS) return __r as any; } }
  const pl = g.player; if (!pl.picking) return;
  pl.picking = null; pl.char.anim.lockpick = false; pl.speedSm = 0; if (text) g.msg(text);
}


// ------------------------------------------------------------ the optical wand: a look under a shut door (wand.ts)
/** The 'look under it' row of a shut door (Game.updateDoors): whatever the hands were doing is dropped and he goes onto his knee at the middle of the leaf —
 *  updateWanding runs it from here; a step, Space or Esc pulls it out (interact.ts cancelSustained → stopWand). */
export function startWand(g: Game, d: Door): void { return crossToRust('startWand', undefined, [g, d]) as any; }
/** The wand pulled out (a step, Space, Esc; the door moving; a show): no round for WAND.stowSecs, he stays on his knee. */
export function stopWand(g: Game, text?: string) {
  const pl = g.player; if (!pl.wanding) return;
  wandDrop(pl); pl.speedSm = 0; if (text) g.msg(text);
}



/** Kick a door in: the 'kick it in' row on a door locked against you, or a sprint straight into one (Game.updateDoors). One committed second: updateKick.
 *  Offered crouched too — it stands him up into it — so not where he cannot stand (canStand: the row is withheld there, and this refuses). */
export function startKick(g: Game, d: Door, run = false, tool?: 'shotgun' | 'hammer'): void { return crossToRust('startKick', undefined, [g, d, run, tool]) as any; }



/** Planar distance from the player to a corpse, wherever the ragdoll left it: the nearer of its ankles (what you take hold of) and its hips.
 *  `within`: the caller only asks 'nearer than this?' (both in-file callers compare against 1.3) — a point already that far off is answered by its plain distance,
 *  its reach line never walked: blocked it would count 99, clear its distance, and either way not under the bar, so every such comparison comes out as it did. */
export function bodyDist(g: Game, gd: Guard, within = Infinity): number { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('bodyDist', undefined, [g, gd, within]); if (__r !== RUST_PASS) return __r as any; } }
  const p = g.player.char.pos, b = gd.char.bones;
  const a: Vec3 = [p[0], 0.5, p[2]];
  const reach = (q: Vec3 | undefined, add: number) => { if (!q) return 99; const d = v3.distXZ(q, p); if (d + add >= within) return d; const t: Vec3 = [q[0], 0.5, q[2]]; return g.col.solidBlocked(a, t) ? 99 : d; };   // not through a partition or a shut door (solidBlocked asks the leaves itself, first: the segmentBlockedDynamic that used to follow it here could only ever repeat its 'clear')
  return Math.min(reach(b.footL, 0), reach(b.footR, 0), reach(b.hips, 0.3) + 0.3, bodyGripDist(g, gd, within));   // (hands.ts: the head end too — he takes a body by the end he is at)   // by the hips still means gathering the legs in first: count it a little further
}

/** Throw origin (right hand-ish) */
export function throwOrigin(g: Game): Vec3 { return goneWithTick('throwOrigin') as any; }

export function doThrow(g: Game, sol: ThrowSolution, kind: ThrowKind): void { return crossToRust('doThrow', undefined, [g, sol, kind]) as any; }

/** the gun's heading eased to the cursor's bearing (a free walk with no aim weight on it): the tick's line, and a look ahead's (walkahead.ts) */
export function aimEase(yaw: number, toAim: Vec3, dt: number): number { return yaw + wrapAngle(v3.yawOf(toAim) - yaw) * (1 - Math.exp(-16 * dt)); }
/** the aim of a cursor whose ray met nothing: level, a metre ahead of the way his body faces */
export function aimLevel(c: { pos: Vec3; bodyYaw: number }): Vec3 { return v3.add(v3.add(c.pos, [Math.sin(c.bodyYaw), 0, Math.cos(c.bodyYaw)]), [0, 1.15, 0]); }
/** WHAT THE CURSOR'S RAY MEETS, and the point it gives: the first thing of the world on it (never the man's own body), else the floor under it — null off
 *  both. The tick's aim starts here (updateAim) and so does a look ahead's (walkahead.ts). */
export function cursorHit(g: Game, ray: { ro: Vec3; rd: Vec3 }, own: number) { return g.engine.world.raycast(ray.ro, ray.rd, 200, undefined, own); }
export function cursorPoint(cam: FollowCamera, u: number, v: number, ray: { ro: Vec3; rd: Vec3 }, hit: { t: number } | null | undefined): Vec3 | null { return hit ? v3.mad(ray.ro, ray.rd, hit.t) : cam.groundPoint(u, v, 0); }
export function updateAim(g: Game, input: Input, cam: FollowCamera, canvas: View): void { return crossToRust('updateAim', undefined, [g, input, cam, canvas]) as any; }
/** where the cursor last was ON the light the reticle has (CSS px) — updateAim keeps that light while the cursor stays there */
const lightAt = new SideMap<Game, [number, number]>('player.lightAt');

/** Every marker Space concerns itself with this frame (Game.interactables: one per thing — the dots in the world, the auto-aim's lights) and the ROWS: what Space
 *  can do right now, one verb per row (Game.nearby — the top MAX_NEAR by what matters, the cancel row at the foot while the selector is open), and the highlighted
 *  one (Game.hover: a Space tap does it, Space held opens the list to choose another — interact.ts). Every verb is its own row: a shut door is 'push it open' AND
 *  'ease it open — silent'; a lock is 'pick' and 'kick'; the man in the arm is 'question him' and 'choke him out'. While a sustained verb runs (a leaf easing, a
 *  lock being picked, the choke, the drive) its row is the ONLY row — worded as running, with its progress — and nothing else is on offer until it ends or is stopped. */
export function buildInteractables(g: Game, cam: FollowCamera, mx: number, my: number, w: number, h: number): void { return crossToRust('buildInteractables', undefined, [g, cam, mx, my, w, h]) as any; }
/** Game.hover ← the highlighted row (nearSel wrapped onto the list), Game.useDoor ← its door if it is a door row in reach. interact.ts calls it again after moving the highlight. */
export function pickHover(g: Game) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('pickHover', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } }
  const pl = g.player, rows = g.nearby;
  pl.nearSel = rows.length ? ((pl.nearSel % rows.length) + rows.length) % rows.length : 0;
  const best = rows[pl.nearSel] ?? null;
  g.hover = best; g.useDoor = best?.kind === 'door' && best.inReach && best.door ? best.door : null;
}
/** The one ordering for 'what is offered near Sam' — the top-left panel (overlay.ts) and the default Space target read the same list in the same order:
 *  in reach before out of reach, then by what MATTERS (a man you could grab is perishable and decisive; a door will still be there), then nearest.
 *  Lower = higher up the list. Within one thing its verbs keep the order buildInteractables gave them; MAX_NEAR rows in all, so a man to grab beside a shut
 *  door reads [grab · push · ease] and the door's third verb (if it had one) would be the row that falls off. */
export const INTERACT_PRIORITY: Record<Interactable['kind'], number> = { guard: 0, held: 0, body: 1, objective: 2, door: 3, light: 9, breaker: 2.5, switch: 3.2, charge: 9.4, wall: 9.5, cam: 9.6, cancel: 10 };   // (wall: a partition's 'breach with a charge' row ranks last — it never takes one of the MAX_NEAR rows from a door, a body or a switch · charge: ownblow.ts's row is put after the capped rows by hand and is never ordered by this — its rank is only the table's)
/** how far a thing is from him, as its reach is measured: a door by its leaf or its doorway's centre, whichever is nearer */
export function thingDist(it: Interactable, p: Vec3): number { const d = v3.distXZ(it.pos, p); return it.door ? Math.min(d, v3.distXZ(it.door.frameCentre, p)) : d; }
export function interactOrder(p: Vec3): (a: Interactable, b: Interactable) => number { return goneWithTick('interactOrder') as any; }
/** the most verb rows the interaction list ever holds (the highest-priority available ones; the cancel row while the selector is open is extra) */
export const MAX_NEAR = 3;
/** Easing a leaf by hand (the 'ease' row — game.ts updateDoors runs it through Doors.crack): the rate it comes round at (rad/s — the crack rate it always had),
 *  how far it goes on its own before it counts as open (where a push would have left it: 3.2 rad/s coasting against the hinge friction ≈ 1.45), and how wide a
 *  leaf may already stand and still offer the row (shut, or a hand ajar). */
export const EASE_RATE = 0.55, EASE_MAX = 1.45;

/** a thrown thing not yet out of the hand (or, under unlimited ammo, just out of it) goes back on the belt */
function canBack(pl: Player, kind: ThrowKind) { if (kind === 'frag') pl.frags++; else if (kind === 'flash') pl.flashbangs++; else pl.canisters++; }
/** a wind-up called off with the can still in his fist: the can is back on the belt and nothing leaves his hand (a grab, cancelActions; under throwback.ts
 *  every beat that stops the wind-up's clock, and a takedown). The swing already begun on the animator is left to play out or be overridden */
function throwOff(pl: Player) { if (pl.pendingThrow) { canBack(pl, pl.pendingThrow.kind); pl.pendingThrow = null; } }

/** F: let go of the body you are dragging (its ankles fall where they are), or take hold of the nearest one within reach. */
export function toggleDrag(g: Game, which?: Guard) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('toggleDrag', undefined, [g, which]); if (__r !== RUST_PASS) return __r as any; } }
  const pl = g.player;
  if (pl.dragging) { releaseBody(g); g.msg('body dropped'); return; }
  if (pl.down || pl.sprinting) return;
  let best: Guard | null = which && which.state === 'dead' ? which : null;
  if (!best) { let bd = 1.3; for (const gd of [...g.guards, ...civiliansShown(g)]) { if (gd.state !== 'dead' || gd.wound) continue; const d = bodyDist(g, gd, 1.3); if (d < bd) { bd = d; best = gd; } } }   // (a civilian on the floor is a body to haul like any)
  if (!best || !best.char.ragdoll) { g.msg('no body within reach'); return; }
  releaseWall(g);   // (you take hold of a body off the wall, crouched to it)
  pl.dragging = best; pl.crouch = true; pl.dragNoiseT = 0.3; pl.dragStuckT = 0; best.char.ragdoll.wake(); haulBegin(g, best); g.msg(pl.haul?.carry ? 'picking him up — Space puts him down, sprint drops him' : pl.haul?.end === 'head' ? 'got him under the arms — Space lets go, sprint drops him' : 'dragging the body — Space lets go, sprint drops it');
}

/** Let go of the body in tow: the ankle pins come off and its feet fall where they are (the soft thud of that; the HUD line is the caller's). */
function releaseBody(g: Game, laid = false) {
  const pl = g.player; if (!pl.dragging) return;
  const b = pl.dragging.char; b.ragdoll?.clearPins(); pl.dragging = null; pl.haul = null; pl.char.anim.haul = null;
  g.audio.play('bodyFall', b.bones.footL ?? b.pos, laid ? 0.12 : 0.3);   // (laid: hands.ts lowered it to the floor first — only the last inch is a fall)
}

/** Break off whatever the hands and feet are in the middle of that belongs to the SPOT, and stand ready: the kick (its door stays as it is), the pick (its progress
 *  stays on the door), a takedown that has not landed (he was only frozen for the beat: he carries on), a body in tow (dropped where it lies), a wind-up (the
 *  canister goes back on the belt), a door held. What a teleport needs — updateKick / updatePicking would otherwise reel you back to their door at 12 m a frame
 *  through every wall between, the cross would land on a man now rooms away, the body's ankles would come with you. A reload travels with you. The animator's
 *  one-shots (the kicking leg, the cross) play out their last fraction of a second where you arrive; nothing reads them any more (startKick waits for the leg). */
export function cancelActions(g: Game) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('cancelActions', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } }
  const pl = g.player, c = pl.char;
  releaseWall(g);                                             // the wall he had his back to is not where he is going
  if (pl.dragging) releaseBody(g);
  if (pl.holding) freeHeld(g, 'cancel');                      // the man in his arm stays where he is — free, and knowing everything
  pl.takedown = null;                                         // (a takedown that has not landed: he was only frozen for the beat — he carries on)
  if (pl.kick || pl.picking || pl.wanding || pl.easing || pl.coding || pl.sel.downT >= 0) pl.fHeld = true;   // a Space still down from what he was doing (or a press half made) is spent: it must come up before it arms again where he arrives (interact.ts clears this the first frame it is up)
  pl.kick = null; pl.picking = null; wandDrop(pl); c.anim.lockpick = false; if (pl.coding) stopCoding(g); samBigDrop(g);   // (bigkit.ts: the big slot's tool, in its beat or on its way back, is on him again)
  pl.easing = null; handsCancel(g); pl.sel.downT = -1; pl.sel.open = false; pl.sel.moved = false; pl.sel.ring = null; c.anim.doorPush = null;   // (the door hand: game.ts drops the record itself once he is carried out of the leaf's reach)
  throwOff(pl);   // not yet out of the hand: keep it
  pl.throwHeld = false; pl.throwPreview = null;
  pl.sprinting = false; pl.speedSm = 0; c.vel = [0, 0, 0];
}

/** Sandbox / tour / harness: stand the player at p (pushed out of whatever is there, snapped to the nearest walkable cell if that is inside something big or off the
 *  map, always inside the world), free of anything he was in the middle of (cancelActions). A body on the floor stays where it fell. */
export function teleportPlayer(g: Game, p: Vec3) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('teleportPlayer', undefined, [g, p]); if (__r !== RUST_PASS) return __r as any; } }
  const pl = g.player; if (pl.down) return;
  if (!Number.isFinite(p[0]) || !Number.isFinite(p[2])) return;   // a bad target from a debug hook must not poison the position (NaN reaches the audio listener and throws)
  cancelActions(g);
  let q: Vec3 = [p[0], 0, p[2]];
  const settle = () => { g.col.clampToWorld(q, BODY_R); g.col.collideCircle(q, bodyR(g, q), 0.05, 2.2, 6); const t = v3.copy(q); return !g.col.collideCircle(t, bodyR(g, t), 0.05, 2.2, 1) || v3.distXZ(t, q) < 0.005; };   // pushed out of whatever is there — and did that converge (false: a gap the body cannot fit, the pushes just trade him back and forth)
  if (!settle() || g.col.nav.isBlocked(q[0], q[2])) {   // inside something big, in a gap too narrow for him, off the walkable floor: stand him on the nearest walkable cell instead
    const c = g.col.nav.nearestFreePoint(q[0], q[2]); if (c) { q = c; settle(); }
  }
  g.col.clampToWorld(q, BODY_R);
  pl.char.pos = q; pl.safe = [q[0], 0, q[2]];      // (a teleport is the one move the collision sweep must not walk him through: it starts afresh from here)
  pl.char.update(0);                               // bones at the new spot now: the capsules, the light meter query and the cursor's markers this frame must not read the old room
  g.note('teleported');
}
