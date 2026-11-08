// THE CAMERA RUNS ITSELF (AUTOCAM). Asked for by the maintainer, 2026-10-23: a double tap of the camera's key turns an auto camera on, that key with movement
// gives the view back to the hand, and the camera should read what the player means the way one tap of the squad's key does.
// The follow camera frames one thing: Sam, leaned toward the cursor. This module works out a frame from what the game already knows and the camera never
// used — the door under his hand and the floor past it, the squad's orders and the beat each door order is on, the enemies IN THE PICTURE, the space he
// stands in — and the loop (loop.ts camStep) gives the follow camera that frame in place of his feet while the mode is on (findme.ts AUTOVIEW:
// the camera's key tapped twice puts it on; the hand on the camera — the key held with the mouse moved, or with a walk key — puts it off).
//   WHAT IT FRAMES, the higher taking over from the lower as it comes up (frame()):
//     a fight    an enemy he could shoot (in the picture and in his line): the look-at goes part of the way to them, and nothing lower is asked
//     the squad  an order just given, and each beat of a door order (set, the word, the entry): the door, then the floor past it, for `beatSecs`
//     the door   the one under his hand (Game.useDoor), or the way out he is walking at: the floor past it
//     the room   a little of the way to the middle of the room he stands in, so the frame moves less than he does
//   …always inside a box round him in the camera's own basis (`sideM`, `awayM`, `backM` at the usual distance: he never leaves the middle of the screen),
//   eased (`lean`), still while a button is down (a burst under way aims through a frame that only follows him), and NEVER BY THE CURSOR: the aim lean is
//   off while the mode is on. THE ZOOM is the hand's own (`AutoCam.base`: where it stood when the mode came on, and the wheel under the camera's key moves
//   it), a little tighter in a room and in close, wider under the sky and under fire, and out as far as `fitMost` to hold what it frames; it never comes in
//   on a fight. THE TURN (turnGoal) only with his feet still and nothing hostile in the picture, nothing in his hands' way, and for two reasons alone: a
//   wall between the lens and him, or the thing it frames lying on the lens's side of him, where the picture is shortest. The walk keys and the aim are
//   read through the view (cmd.ts), so a turn under a held key would walk him somewhere else: a walk key or a button ends a turn where it is, and his
//   aim is carried round with it (host/findme.ts AimCarry, as the double tap's is).
//   THE SWAY (`sway`, autoSway): the turn above is a turn at rest. With `sway` on the view may also turn while he fights and while he moves, by another
//   rule that keeps the hand's own yaw as its anchor: the yaw leans, slowly and by a spring (no step, no jump), toward the men he could shoot — so the
//   fight comes up the screen, where the picture is longest — and toward the side he is moving to, never more than `swayMax` from where the hand left the
//   view, never to a yaw that would put a wall between the lens and him or a man he could shoot, and still while a button is down. It is a function of the
//   moment, not a sum of turns: the fight over or his feet still, it comes back to the hand's yaw. Nothing carries his aim through it: the view turns
//   slowly under a live cursor, as the floor does when he walks. Off = the view turns by itself only at rest.
//   WHAT IT MAY KNOW: what the picture shows. An enemy the fog hides is never asked about (FogOfWar.hidden), and no sound is: Game.events is every sound
//   on the floor, the HUD marks none of them, and a lean toward one would point at a man the fog hides.
//   IT WRITES NOTHING OF THE GAME'S: read() asks (roomRead, the fog, the orders, the door tools) and the frame goes to the host's camera. The view is the
//   seat's input (cmd.ts), so a recording replays the view it was made with whatever this table says; with the mode never put on the build is the build
//   as it was. It stands down wherever the view is not Sam's own: the sky seat, a spectated man, a show, a pin — the loop asks.
//   THE HAND'S OWN TUNING (`tune`, the foot of this file). The maintainer, of his first play of the mode: "I was getting a little bit motion sick ... I
//   think it's from like having too much rotation", and asked for "a ton of different options of like modes or like sliders to configure in the settings
//   panel". With `tune` on the numbers that decide how much the view turns, leans and zooms are the player's: a MODE (never · only at rest · gentle ·
//   free; `gentle` is the start, `free` is every number as it was) and rows over it, kept per machine and shown on Options (ui/findme.ts). These
//   exist for it and do nothing at their module values: a turn of the mode's own may be EASED (`turnCap`: a spring with a top speed and a ramp up to
//   it, where the turn as built took a share of what was left each second and so began at its fastest) and so may the double tap's swing
//   (`swingCap`); a turn at rest has a row for each of its two reasons (`rest`, the wall that hides him; `restAhead`, the thing framed on the lens's
//   side); and the frame's lean and the zoom's travel each have a share of their own (`leanK`, `zoomAmt`). An eased turn is seconds long, so the
//   loop treats it as it treats the sway and not as the quick turn: no aim carried, the mouse does not end it, `gap` from where it is seen to stop
//   (host/loop.ts). Off = the numbers as they were, to the bit.
//   THE SOFT LOCK (`lock`, read() and frame()). The maintainer, of a review of Chaos Theory's "closer than ever": "there's definitely stuff for the auto
//   camera mode to learn". What that review says of the camera on an approach (a machine's rough transcript of it): whenever an enemy is on screen the
//   camera shifts its focus to him, and as the player moves closer its focus becomes stronger and stronger, almost as if he were soft locked on — so
//   subtly that most players never notice. Here: the man he is stalking (game/stalk.ts), or else the nearest hostile man the picture shows in his
//   line who is NOT onto him (the one it has stays until another is `lockStick` nearer, and through `lockLinger` behind a pillar), is the man of the
//   lock from `lockM` in; its strength rises to full at `lockNearM`, eased (`lockEase`). Nobody is while it is a FIGHT (somebody the picture shows is
//   onto him, or was within `lockCalm`), and but for a stalk nobody is while he is not sneaking (the rifle up, a sprint, a body in tow, a kick, a
//   lock, the wand, a throw being aimed). The frame goes from the middle of the men he could shoot to the middle of the two of them, and the camera
//   goes toward the lock's own distance — the place's zoom less `lockIn` of it, never so near that a man he could shoot leaves the picture (a door
//   ahead and the squad's beat do not hold the lock out). A man held keeps it in close (the frame as it was). The zoom as it was is worked out
//   untouched under it (`A.goal`: the place's own zoom, the band); `lockIn` goes by the hand's share of the zoom (`zoomAmt`), and at none of either
//   the distance is as it was to the bit. One rule of the zoom gives way to it, by its strength: "it never comes in on a fight" holds the zoom out
//   while any man is in his line, and the lock's man lets it back in as the lock takes hold. A held button stills it as it stills the frame — the
//   strength and the share it takes, though never so that the men he could shoot leave the picture — but for the held click of a stalk or of the
//   wait at a corner (it aims at nothing), a man in his arm, and letting go once a fight has begun. The view gone to somebody else (plan mode, a
//   show): the lock lets go at once and the distance is the zoom's own. No turn is added: it moves and zooms the frame.
// Off here (the frozen stand); the page raises `on`, `sway`, `tune` and `lock` (host/profile.ts: `?autocam=0` off, `?autoturn=0`, `?autozoom=0`,
// `?autosway=0`, `?autotune=0` and `?autolock=0` for those alone).
// tools/qa/autocam-check.ts holds it (AUTOCAM_BREAK=1 = the switch left off, AUTOSWAY_BREAK=1 = the sway left off, AUTOTUNE_BREAK=1 = the tuning left
// off, AUTOLOCK_BREAK=1 = the soft lock left off: their lines must fail).
// THIS FILE IS THE MODE'S OWN HALF, which both pages use and which imports nothing of the game: the table, the frame, the turn, the sway, the player's tuning. What there is to frame comes
// as an AutoScene: from the copy of the game on the page before this one (host/autocam.ts autoRead), from a row of Rust's on the page that draws (rust/game_draw shot.rs, draw/shot.ts).
import type { Vec3 } from '../math/vec';

export const AUTOCAM = {
  /** the switch: the camera's key tapped twice puts the auto camera on. Off = the double tap is the Options row's, as it was */
  on: false,
  /** it may turn the view (off: it frames and zooms only) · it may zoom */
  turn: true, zoom: true,
  /** THE BOX he stays in, round the look-at in the camera's basis at the usual distance (21 m), scaled with the distance: to either side · with the view
   *  pushed up the screen (he sits low) · with it pulled down (he sits high) */
  sideM: 3.5, awayM: 2.8, backM: 3,
  /** how fast the frame goes to where it is wanted (1/s) */
  lean: 2.5,
  /** a fight: how far toward the middle of the men he could shoot the look-at goes (a share) · how far off a man counts (m) · how near a man the picture
   *  shows stops a turn of the mode's own though something stands between them (m) */
  fight: 0.5, fightM: 22, nearM: 9,
  /** the squad: the share · how long a beat holds the frame (s) and how long it then lets go over · how far off a beat is still framed (m) */
  beat: 0.5, beatSecs: 3.5, beatFade: 1.5, beatM: 16,
  /** the door: the share · how far off a way out he walks at begins to count (m) · how straight at it he has to be walking (cos) and how fast (m/s) ·
   *  how near he has to stay for it to keep the frame once his feet stop (m) · how far past a frame the floor beyond is taken (m) */
  door: 0.6, doorM: 5, doorAim: 0.6, doorMps: 0.4, doorKeepM: 2.5, beyondM: 2.5,
  /** the room: the share · the longest room that has a middle worth leaning on (m) */
  room: 0.25, roomMaxM: 12,
  /** THE ZOOM, as shares of the hand's own: in a room · under the sky · in close · under fire (over `fireAt` of suppression) */
  roomK: 0.92, skyK: 1.15, closeK: 0.8, fireK: 1.1, fireAt: 0.3,
  /** …out to this many times the hand's own to hold what is framed, each thing `fitIn` of the way from the look-at to the picture's edge at the most ·
   *  a new distance is taken only when it differs by `band` · and gone to at `zoomRate` (1/s) */
  fitMost: 1.5, fitIn: 0.8, band: 0.06, zoomRate: 1.2,
  /** THE TURN: his feet still this long (s) · nothing fired for this long (s) · this long since the last turn of its own (s) */
  still: 0.5, calm: 3, gap: 2.5,
  /** a thing framed counts as on the lens's side when the line to it is this far round from up the screen (cos) and it is this far off (m) · the view comes
   *  to rest this far off square to it (rad: the level's walls are square, and a view square to them shows one wall of a corridor and hides the other) */
  behind: -0.34, behindM: 1.5, skew: 22 * Math.PI / 180,
  /** a wall hides him when the line from his chest toward the lens meets one inside `hidM`; a view is clear of that with `clearM` free · the line's step (m) */
  hidM: 1.5, clearM: 2.5, probeM: 0.5,
  /** how fast a turn of its own comes round (1/s: the share of what is left taken each second) */
  turnRate: 5,
  /** THE HAND'S OWN TUNING (off = every number here is the module's; on = the mode and rows at the foot of this file write the ones they name) */
  tune: false,
  /** …a turn at rest is made for a wall that hides him · and for the thing framed lying on the lens's side (each on its own) */
  rest: true, restAhead: true,
  /** …a turn of its own goes no faster than this (rad/s; 0 = by `turnRate`, as built): it takes `turnRamp` (s) to reach that speed from rest and
   *  begins to slow about `turnLand` (rad) short of where it is going, whatever the speed (so the top speed is what a turn of 45° or more goes at) ·
   *  the double tap that puts the mode on brings the view round to where he looks · and that swing goes no faster than this (rad/s; 0 = as built) */
  turnCap: 0, turnLand: 0.24, turnRamp: 0.6, swing: true, swingCap: 0,
  /** …an eased turn slower than this (rad/s) is not seen: `gap` counts from there, not from the last of its settling */
  turnSeen: 0.0175,
  /** …a share of every lean of the frame (0 = it stays on him) · and of how far the zoom leaves the hand's own (0 = it stays there) */
  leanK: 1, zoomAmt: 1,
  /** THE SOFT LOCK (off = the frame and the zoom as they were): a man who is not onto him begins to hold the frame this far off (m) and holds it in
   *  full this near (m) · the frame's share of the way to him at full (the middle of the two) · the camera comes in by this share of where it
   *  would be at full (0 = it does not come in) · how fast the strength goes to where it is wanted (1/s) · the man it has is kept until another
   *  is this much nearer (m), and through this long out of his line (s: a pillar he passes) · after somebody in the picture was onto him it is
   *  nobody's for this long (s: a fight does not end the moment the last man loses him) */
  lock: false, lockM: 8, lockNearM: 2, lockK: 0.5, lockIn: 0.4, lockEase: 2, lockStick: 0.75, lockLinger: 1, lockCalm: 4,
  /** THE SWAY (off = the view turns by itself only at rest): the share of the angle to the middle of the men of the fight that the yaw leans, and the
   *  most of it (rad) · men gone further round than this to the OTHER side (rad) do not flip the side it took · that middle nearer him than `swayNearM`
   *  leans nothing, and counts in full `swayFadeM` further out (m) · a man of the fight is one the picture shows who was in his line within `swayLinger`
   *  (s: a man stepping in and out of cover does not rock the view) */
  sway: false, swayFight: 0.6, swayFightMax: 38 * Math.PI / 180, swayBehind: 110 * Math.PI / 180, swayNearM: 1.5, swayFadeM: 4, swayLinger: 2,
  /** …the lean toward the side he moves to (rad) at this speed across the view (m/s): only while no fight asks for one · the most of both (rad) */
  swayMove: 14 * Math.PI / 180, swayMps: 2.5, swayMax: 45 * Math.PI / 180,
  /** …how it gets there: what is asked for is eased (s) before the spring sees it · the spring's rate (1/s) · never faster than this (rad/s) · under a
   *  button its speed dies away at this rate (1/s), and it stays stopped this long after the button comes up (s: shots tapped one after another are one
   *  stop) · a view a wall forbade is tried again only after this long clear (s) */
  swayEase: 0.6, swayHz: 2.2, swayRate: 0.2, swayStop: 12, swayAfter: 0.6, swayClear: 1,
  /** THE HAND TAKES IT BACK: the camera's key held this long with a walk key down (s: longer than a tap) · or the mouse moved this far under it (px: less is
   *  a hand's tremor on the key, host/findme.ts TAPTWICE.driftPx) inside `handSecs` */
  holdSecs: 0.25, handPx: 12, handSecs: 0.4,
};
export const AUTOCAM_STATS = { on: 0, off: 0, turns: 0, reads: 0, swayed: 0 };

/** the numbers a read of the scene is made by, in the order rust/game_draw shot.rs takes them: the table above is the one place they are written (host/drawgame.ts says them when they change) */
export const SHOT_NUMBERS = ['fightM', 'nearM', 'swayLinger', 'lockM', 'lockNearM', 'lockStick', 'lockLinger', 'lockCalm', 'beatSecs', 'beatFade', 'beatM', 'beyondM', 'doorM', 'doorAim', 'doorMps', 'doorKeepM', 'roomMaxM', 'roomK', 'skyK', 'closeK', 'fireK', 'fireAt', 'lockIn', 'zoomAmt'] as const;

/** what a frame is worked out from: plain numbers, so a check can hand one over. Each thing framed is a floor point and how much of the frame it has (0‥1) */
export interface AutoScene {
  sam: Vec3;
  /** hostile men in the picture and in his line */
  threats: Vec3[];
  beat: { at: Vec3; w: number } | null; door: { at: Vec3; w: number } | null; room: { at: Vec3; w: number } | null;
  /** the hand's zoom is scaled by this where he stands */
  zoomK: number;
  /** a button is down: the frame only follows him */
  hold: boolean;
  /** his feet are still this tick · nothing stands in a turn's way but the clocks (no man held, no arc up, no wall taken, the camera not locked) */
  still: boolean; free: boolean;
  /** how fast he moves over the floor (X, Z: m/s) · the men of the fight (the sway's: shown, and in his line within `swayLinger`) */
  vel: [number, number]; fight: Vec3[];
  /** hostile men the picture shows in his line within `fightM`, or within `nearM` in his line or not (no turn of its own while there is one) · the zoom is the mode's to move (no lesson holds it) */
  shown: number; zoomFree: boolean;
  /** seconds since he fired */
  firedAgo: number;
  /** THE SOFT LOCK's man (AUTOCAM.lock): where he stands, how strongly he holds the frame by how near he is (0..1), and whether the button that is
   *  down is the stalk's own held click (the frame is not stilled by it). A man Sam holds: no place (the frame as it is), in full ·
   *  somebody the picture shows is onto him (a fight: nobody is the lock's man, and the zoom does not come in) */
  lock?: { at: Vec3 | null; s: number; stalk: boolean } | null; onto?: boolean;
}

/** the camera, as the mode asks and moves it */
export interface AutoLens { fov: number; aspect: number; pitch: number; yaw: number; distance: number; desiredDistance: number; planarBasis(): { right: ArrayLike<number>; fwd: ArrayLike<number> } }
/** the keys, as it asks them */
export interface AutoKeys { down(code: string): boolean }

/** the mode's own working state: one per seat, kept by the page */
export interface AutoFrame {
  /** the look-at's shift from his feet (x, z), as eased */
  off: [number, number];
  /** the hand's own distance · the distance wanted (NaN: none taken yet) */
  base: number; goal: number;
  /** seconds his feet have been still · since its last turn began · how fast an eased turn of its own is going (rad/s) */
  stillT: number; turnT: number; turnV: number;
  /** the camera's key held with a walk key (s) · the mouse's travel under the key (px) and how long since it last moved (s) */
  heldT: number; handPx: number; handT: number;
  /** the sway: how far the yaw stands from the hand's own (rad), how fast it is going (rad/s), where it is going, and how long since that was asked (s) */
  sway: number; swayV: number; swayGoal: number; swayT: number;
  /** …the hand's own yaw as the sway left it (it comes back to this, to the bit) · what the fight last asked for, and that and the walk's share as eased */
  swayHome: number; swayRaw: number; swayFight: number; swayMove: number;
  /** …how long since a button was down (s) · the share of the lean the walls allow, and how long a larger share has been clear (s) */
  swayCalm: number; swayK: number; swayKT: number;
  /** the soft lock: its strength as eased, and the distance it last asked for (NaN: none of it) */
  lockS: number; lockTo: number;
}
export function frameState(): AutoFrame {
  return { off: [0, 0], base: NaN, goal: NaN, stillT: 0, turnT: 1e9, turnV: 0, heldT: 0, handPx: 0, handT: 1e9, sway: 0, swayV: 0, swayGoal: 0, swayT: 1e9, swayHome: 0, swayRaw: 0, swayFight: 0, swayMove: 0, swayCalm: 1e9, swayK: 1, swayKT: 0, lockS: 0, lockTo: NaN };
}
/** the mode came on: the zoom as it stands is the hand's own, and nothing of an earlier run is kept */
export function frameBegin(A: AutoFrame, cam: AutoLens): void { A.off[0] = A.off[1] = 0; A.base = cam.desiredDistance; A.goal = NaN; A.stillT = 0; A.turnT = 1e9; A.turnV = 0; A.heldT = 0; A.handPx = 0; A.lockS = 0; A.lockTo = NaN; swayDrop(A); AUTOCAM_STATS.on++; }
/** the view is somebody else's for now (a show, a pin): nothing is leaned, and it starts from his feet when it is his again */
export function frameRest(A: AutoFrame, cam?: AutoLens): void {
  A.off[0] = A.off[1] = 0; A.stillT = 0;
  if (A.lockS > 0 && cam && A.goal > 0) cam.desiredDistance = A.goal;   // (the soft lock's share of the distance is not the other view's: back to where the zoom stood without it)
  A.lockS = 0; A.lockTo = NaN;
}
/** the yaw as it stands is the hand's own from here: the mode put off, or a come-round begun */
export function swayDrop(A: AutoFrame): void { A.sway = 0; A.swayV = 0; A.swayGoal = 0; A.swayT = 1e9; A.swayRaw = 0; A.swayFight = 0; A.swayMove = 0; A.swayK = 1; A.swayKT = 0; }
/** the sway stopping where it is — a button down, or the view gone to somebody else for now: its speed dies away, with no step. The turn to give the camera */
export function swayCoast(A: AutoFrame, dt: number): number {
  if (!A.swayV) return 0;
  A.swayV *= Math.exp(-AUTOCAM.swayStop * dt); if (Math.abs(A.swayV) < 1e-4) A.swayV = 0;
  const d = A.swayV * dt; A.sway += d; A.swayGoal = A.sway; A.swayT = 0.1; return d;
}
/** the hand zoomed under the camera's key with the mode on: that is the hand's own distance now */
export function autoZoom(A: AutoFrame, cam: AutoLens, delta: number): void { if (!(A.base > 0)) A.base = cam.desiredDistance; A.base = Math.max(6, Math.min(40, A.base * Math.exp(delta * 0.0012))); A.goal = NaN; }

const WALK = ['KeyW', 'KeyS', 'KeyD', 'KeyA'];
export function walkKey(input: AutoKeys): boolean { return WALK.some(k => input.down(k)); }
/** THE HAND ON THE CAMERA, once a frame with the frame's own time: `orbitPx` = the mouse's travel under the camera's key this frame, `held` = that key is
 *  down. True = the hand has taken the view (the mode goes off): the mouse moved past a tremor, or the key held longer than a tap with a walk key down */
export function autoHand(A: AutoFrame, input: AutoKeys, orbitPx: number, held: boolean, dt: number): boolean {
  const K = AUTOCAM;
  if (orbitPx) { A.handPx = (A.handT > K.handSecs ? 0 : A.handPx) + Math.abs(orbitPx); A.handT = 0; } else A.handT += dt;
  A.heldT = held && walkKey(input) ? A.heldT + dt : 0;
  const took = A.handPx > K.handPx || A.heldT > K.holdSecs;
  if (took) { A.handPx = 0; A.heldT = 0; }
  if (!held) A.handPx = 0;   // (the travel is one hold's: two taps with a tremor each are not a drag)
  return took;
}

/** how far the floor shows from the look-at at one metre of distance: to a side, up the screen and down it */
function reach(cam: AutoLens): { side: number; up: number; down: number } {
  const h = cam.fov / 2, P = cam.pitch, s = Math.sin(P), c = Math.cos(P);
  return { side: Math.tan(h) * cam.aspect, up: P - h > 0.05 ? s / Math.tan(P - h) - c : 4, down: c - s / Math.tan(Math.min(Math.PI / 2 - 1e-3, P + h)) };
}

/** ONE STEP OF THE FRAME: `A.off` eased to where the scene wants the look-at, the camera's distance asked for, and the look-at returned (his feet, shifted).
 *  The loop hands that to FollowCamera.step in place of his feet, with no aim to lean on. */
export function autoFrame(A: AutoFrame, S: AutoScene, cam: AutoLens, dt: number): Vec3 {
  const K = AUTOCAM, p = S.sam; let ox = 0, oz = 0;
  // the soft lock: its strength eased to what the scene asks. A held button stills the frame, the zoom and the strength — but not the held click of a
  // stalk, not a man in his arm, and not the letting go once a fight has begun (a stalk the man broke, the button still down: the fight is not left
  // in close; a man shot under a held aim with nobody onto him is, until the button is up)
  const L = S.lock ?? null, stilled = S.hold && !L?.stalk, moves = !stilled || (L ? !L.at : !!S.onto);
  if (moves) { A.lockS += ((L ? L.s : 0) - A.lockS) * Math.min(1, dt * K.lockEase); if (A.lockS < 1e-4 && !L) A.lockS = 0; }
  const ls = A.lockS;
  const take = (at: Vec3, share: number, w: number) => { share *= K.leanK; ox += (share * (at[0] - p[0]) - ox) * w; oz += (share * (at[2] - p[2]) - oz) * w; };
  if (S.room) take(S.room.at, K.room, S.room.w);
  if (S.door) take(S.door.at, K.door, S.door.w);
  if (S.beat) take(S.beat.at, K.beat, S.beat.w);
  if (S.threats.length) { let x = 0, z = 0; for (const t of S.threats) { x += t[0]; z += t[2]; } x /= S.threats.length; z /= S.threats.length;
    if (L?.at && ls > 0) take([x + (L.at[0] - x) * ls, 0, z + (L.at[2] - z) * ls], K.fight + (K.lockK - K.fight) * ls, 1);   // (from the middle of the men he could shoot to the middle of the two of them)
    else take([x, 0, z], K.fight, 1); }
  else if (L?.at && ls > 0) take(L.at, K.fight + (K.lockK - K.fight) * ls, 1);   // (its man out of his line for a moment, or stalked from behind something: framed as he was in it)
  // …inside the box he stays in: the camera's own across and up, at the camera's own distance
  const B = cam.planarBasis(), k = cam.distance / 21, box = (x: number, z: number): [number, number] => {
    const r = x * B.right[0] + z * B.right[2], f = x * B.fwd[0] + z * B.fwd[2], r1 = Math.max(-K.sideM * k, Math.min(K.sideM * k, r)), f1 = Math.max(-K.backM * k, Math.min(K.awayM * k, f));
    return r1 === r && f1 === f ? [x, z] : [B.right[0] * r1 + B.fwd[0] * f1, B.right[2] * r1 + B.fwd[2] * f1]; };   // (inside it: as it is, to the bit)
  [ox, oz] = box(ox, oz);
  if (!stilled) { const e = Math.min(1, dt * K.lean); A.off[0] += (ox - A.off[0]) * e; A.off[1] += (oz - A.off[1]) * e; }
  [A.off[0], A.off[1]] = box(A.off[0], A.off[1]);
  // the zoom: the hand's own by where he stands, and out to hold what is framed from where the look-at is going
  if (S.zoomFree && A.base > 0) {
    const lockIn = K.lockIn * Math.max(0, Math.min(1, K.zoomAmt));   // (the lock's zoom is a zoom of the mode's own: the hand's share of those is its share of this)
    const V = reach(cam), fit = (at: Vec3) => { const x = at[0] - p[0] - ox, z = at[2] - p[2] - oz, r = Math.abs(x * B.right[0] + z * B.right[2]), f = x * B.fwd[0] + z * B.fwd[2]; return Math.max(r / V.side, f > 0 ? f / V.up : -f / V.down) / K.fitIn; };
    let need = fit(p); for (const t of S.threats) need = Math.max(need, fit(t)); const men = need; if (S.beat && S.beat.w > 0.5) need = Math.max(need, fit(S.beat.at)); if (S.door && S.door.w > 0.5) need = Math.max(need, fit(S.door.at));
    let want = Math.max(6, Math.min(40, Math.max(A.base * S.zoomK, Math.min(need, A.base * K.fitMost))));
    if (K.zoomAmt !== 1) want = Math.max(6, Math.min(40, A.base + (want - A.base) * K.zoomAmt));   // (the hand's own share of that travel: AUTOCAM.tune)
    if (S.threats.length && A.goal > 0 && want < A.goal) want = A.goal + (want - A.goal) * (lockIn > 0 ? ls : 0);   // (it never comes in on a fight — but for the soft lock's man, by the lock's strength)
    if (!stilled && (!(A.goal > 0) || Math.abs(want / A.goal - 1) > K.band)) A.goal = want;
    // …and the soft lock goes from there toward its own distance by its strength: the place's zoom (by the hand's share, as the goal's is) less its
    // share, never past what holds the MEN he could shoot in the picture (a door ahead or the squad's beat holds the zoom out under it, and not the
    // lock in full). A held button stills where that goes, as it stills the goal under it — but the men stay in the picture.
    let to = A.goal;
    if (ls > 0 && A.goal > 0 && lockIn > 0) {
      const floor = Math.min(men, A.base * K.fitMost);
      if (moves || !(A.lockTo > 0)) { const place = Math.max(6, Math.min(40, A.base + (A.base * S.zoomK - A.base) * K.zoomAmt)); A.lockTo = A.goal + (Math.min(A.goal, Math.max(6, place * (1 - lockIn))) - A.goal) * ls; }
      to = Math.min(A.goal, Math.max(A.lockTo, floor)); } else A.lockTo = NaN;
    if (to > 0) cam.desiredDistance += (to - cam.desiredDistance) * Math.min(1, dt * K.zoomRate);
  }
  return [p[0] + A.off[0], p[1], p[2] + A.off[1]];
}

/** how far the line from his chest toward the lens of a view at `yaw` runs before it meets something (m, `clearM` at the most) */
function clearOf(p: Vec3, yaw: number, blocked: (a: Vec3, b: Vec3) => boolean): number {
  const K = AUTOCAM, chest: Vec3 = [p[0], 1.3, p[2]], sx = Math.sin(yaw), cz = Math.cos(yaw); let clear = 0;
  for (let s = K.probeM; s <= K.clearM + 1e-6; s += K.probeM) { if (blocked(chest, [p[0] + sx * s, 1.3, p[2] + cz * s])) break; clear = s; }
  return clear;
}
const wrap = (a: number) => Math.atan2(Math.sin(a), Math.cos(a));

/** A TURN OF ITS OWN, asked once a camera step: the yaw to come round to, or null (the usual answer). Only with his feet still for `still`, nothing fired
 *  for `calm`, no man he could shoot, no button down, nothing in his hands' way, and `gap` since its last — and then only for a wall between the lens and
 *  him, or the thing framed lying on the lens's side of him. `blocked` = the level's sight line. */
export function turnGoal(A: AutoFrame, S: AutoScene, cam: AutoLens, dt: number, blocked: (a: Vec3, b: Vec3) => boolean): number | null {
  const K = AUTOCAM; A.stillT = S.still ? A.stillT + dt : 0; A.turnT += dt;
  if (!(K.rest || K.restAhead) || A.stillT < K.still || A.turnT < K.gap || S.hold || !S.free || S.shown || S.threats.length || S.firedAgo < K.calm) return null;
  const p = S.sam, sub = S.beat && S.beat.w > 0.5 ? S.beat.at : S.door && S.door.w > 0.5 ? S.door.at : null;
  if (sub && K.restAhead) {
    const dx = sub[0] - p[0], dz = sub[2] - p[2], d = Math.hypot(dx, dz), B = cam.planarBasis();
    if (d >= K.behindM && (dx * B.fwd[0] + dz * B.fwd[2]) / d < K.behind) {
      const square = Math.atan2(dx, dz) + Math.PI, a = square + K.skew, b = square - K.skew;   // (a view that looks along the line to it: the camera's yaw 0 looks toward −Z)
      const first = Math.abs(wrap(a - cam.yaw)) <= Math.abs(wrap(b - cam.yaw)) ? a : b, other = first === a ? b : a;
      for (const y of [first, other]) if (clearOf(p, y, blocked) >= K.hidM) return cam.yaw + wrap(y - cam.yaw);
    }
  }
  if (K.rest && clearOf(p, cam.yaw, blocked) < K.hidM) {
    for (let i = 1; i < 8; i++) { const y = cam.yaw + (i % 2 ? 1 : -1) * Math.ceil(i / 2) * (Math.PI / 4); if (clearOf(p, y, blocked) >= K.clearM) return y; }   // (±45°, ±90°, ±135°, 180°: the least turn that clears him)
    A.turnT = K.gap - 0.5;   // (nowhere clears him: asked again in half a second, not every step)
  }
  return null;
}

/** ONE STEP OF THE SWAY (AUTOCAM.sway), asked once a camera step when no come-round is under way: the turn to give the camera now (rad). The hand's own
 *  yaw is `cam.yaw − A.sway`; what the scene asks for is worked out ten times a second and eased, and the yaw goes to it by a critically damped spring
 *  whose speed is capped. Nothing in it steps: every way it stops or changes its mind goes through the spring or a decay. */
export function autoSway(A: AutoFrame, S: AutoScene, cam: AutoLens, dt: number, blocked: (a: Vec3, b: Vec3) => boolean): number {
  const K = AUTOCAM; if (!(dt > 0)) return 0;
  if (!A.sway && !A.swayV) A.swayHome = cam.yaw;
  A.swayCalm = S.hold || !S.free ? 0 : A.swayCalm + dt;
  if (A.swayCalm < K.swayAfter) return swayCoast(A, dt);   // a button down (or just up), or his hands at something the view serves
  const home = cam.yaw - A.sway, p = S.sam; A.swayT += dt;
  if (A.swayT >= 0.1) {
    const asked = Math.min(A.swayT, 0.5), ease = 1 - Math.exp(-asked / K.swayEase); A.swayT = 0;
    // the fight: the middle of its men, up the screen
    let raw = 0;
    if (S.fight.length) {
      let x = 0, z = 0; for (const t of S.fight) { x += t[0]; z += t[2]; } x = x / S.fight.length - p[0]; z = z / S.fight.length - p[2]; const d = Math.hypot(x, z);
      if (d > K.swayNearM) {
        const off = wrap(Math.atan2(x, z) + Math.PI - home), kept = A.swayRaw !== 0 && Math.sign(off) !== Math.sign(A.swayRaw) && Math.abs(off) > K.swayBehind;   // (gone round behind the lens to the other side: the side it took is kept)
        raw = (kept ? Math.sign(A.swayRaw) * K.swayFightMax : Math.sign(off) * Math.min(K.swayFightMax, K.swayFight * Math.abs(off))) * Math.min(1, (d - K.swayNearM) / K.swayFadeM);
      }
    }
    A.swayRaw = raw; A.swayFight += (raw - A.swayFight) * ease; if (!raw && Math.abs(A.swayFight) < 1e-4) A.swayFight = 0;
    // the walk: the side he moves to, while no fight asks for a lean of its own (a fight whose lean the hand has put at none asks for none of this either)
    const speed = Math.hypot(S.vel[0], S.vel[1]), lat = speed > 0.3 && !raw && !(K.swayFightMax <= 0 && S.fight.length) ? Math.max(-1, Math.min(1, Math.sin(wrap(Math.atan2(S.vel[0], S.vel[1]) + Math.PI - home)) * speed / K.swayMps)) : 0;
    A.swayMove += (K.swayMove * lat - A.swayMove) * ease; if (!lat && Math.abs(A.swayMove) < 1e-4) A.swayMove = 0;
    let goal = Math.max(-K.swayMax, Math.min(K.swayMax, A.swayFight + A.swayMove)); if (Math.abs(goal) < 1e-3) goal = 0;
    // …and never to a view that hides him, or a man in his line, behind a wall the hand's own view does not: the share the walls allow falls at once and
    // comes back only after `swayClear` clear (a pillar passed on a walk does not rock the view)
    let k = 1;
    if (Math.abs(goal) > 0.05) {
      const pts: Vec3[] = [p]; for (let i = 0; i < S.threats.length && i < 4; i++) pts.push(S.threats[i]);
      const open = pts.filter(q => clearOf(q, home, blocked) >= K.hidM), ok = (y: number) => open.every(q => clearOf(q, y, blocked) >= K.hidM);
      while (k > 1e-6 && !ok(home + goal * k)) k -= 1 / 3; k = Math.max(0, k);
    }
    if (k < A.swayK) { A.swayK = k; A.swayKT = 0; } else if (k > A.swayK) { if ((A.swayKT += asked) >= K.swayClear) { A.swayK = k; A.swayKT = 0; } } else A.swayKT = 0;
    A.swayGoal = goal * A.swayK;
  }
  let turn = 0; const hz = K.swayHz * Math.max(1, K.swayRate / FREE.swayRate);   // (a top speed over the one as built: the spring is that much stiffer, or a small lean would never reach it)
  for (let left = dt; left > 1e-9;) {   // (a long frame in short steps: the spring is stepped as it would be at the fixed step)
    const h = Math.min(left, 1 / 60); left -= h;
    A.swayV += (-2 * hz * A.swayV - hz * hz * (A.sway - A.swayGoal)) * h; A.swayV = Math.max(-K.swayRate, Math.min(K.swayRate, A.swayV));
    A.sway += A.swayV * h; turn += A.swayV * h;
  }
  if (!A.swayGoal && Math.abs(A.sway) < 1e-4 && Math.abs(A.swayV) < 1e-4) {   // home — to the bit where nothing else has turned the view since it left
    const was = A.sway - turn, back = A.swayHome - cam.yaw; turn = Math.abs(back + was) < 1e-9 ? back : -was; A.sway = 0; A.swayV = 0;   // (`was`: the lean before this step's share of it, which the camera has not been given yet)
  }
  return turn;
}

/** ONE STEP OF AN EASED TURN (`turnCap` over none; the double tap's swing under `swingCap`): the view at `yaw` coming round to `goal` the short way
 *  by a critically damped spring whose speed is capped at `cap` (rad/s) and which is let speed up no faster than reaches the cap in `turnRamp`. The
 *  spring's rate is the cap over `turnLand`, so it brakes over the same angle at any speed and the cap is what a turn of any size goes at (braking is
 *  the spring's own, so it lands without going past). The turn to give the camera, and whether that lands it. */
export function turnEase(A: AutoFrame, yaw: number, goal: number, dt: number, cap = AUTOCAM.turnCap): { turn: number; done: boolean } {
  const K = AUTOCAM, w = cap / Math.max(0.02, K.turnLand), up = cap / Math.max(0.05, K.turnRamp), brake = 2 * w * cap; let d = wrap(goal - yaw), turn = 0;
  for (let left = dt; left > 1e-9;) {
    const h = Math.min(left, 1 / 60); left -= h;
    let a = w * w * d - 2 * w * A.turnV; const lim = a * A.turnV >= 0 ? up : brake; a = Math.max(-lim, Math.min(lim, a));   // (speeding up: by the ramp; slowing down: the spring's own, which a landing never asks more of than `brake` — a goal that has gone round to the other side, or a speed from a faster turn handed on, is braked at that and no harder)
    const top = Math.max(cap, Math.abs(A.turnV) - brake * h); A.turnV = Math.max(-top, Math.min(top, A.turnV + a * h)); const st = A.turnV * h; d -= st; turn += st;
  }
  if (Math.abs(d) < 2e-4 && Math.abs(A.turnV) < 2e-3) { A.turnV = 0; return { turn: turn + d, done: true }; }   // (there: what is left is a hundredth of a degree, no step to see)
  return { turn, done: false };
}
/** …and one ended where it is (a walk key, a button, the hand's own turn): its speed dies away, with no step. The turn to give the camera */
export function turnCoast(A: AutoFrame, dt: number): number {
  if (!A.turnV) return 0;
  const v0 = A.turnV; A.turnV *= Math.exp(-AUTOCAM.swayStop * dt); if (Math.abs(A.turnV) < 1e-4) A.turnV = 0;
  return (v0 + A.turnV) / 2 * dt;
}

// ---- THE HAND'S OWN TUNING (AUTOCAM.tune): a mode, and rows over it. Kept per machine (localStorage `twopointsix.autotune`), read once as this module
// loads; a harness has no store and gets the start. It writes this file's own table and nothing else: the view is the seat's input, so no recording
// and no digest can tell one tuning from another, and with `tune` off every number is the module's (autoTuneApply).
const DEG = Math.PI / 180;
export const AUTO_KEYS = ['rest', 'restAhead', 'swing', 'turnCap', 'swingCap', 'still', 'gap', 'swayFightMax', 'swayMove', 'swayRate', 'leanK', 'lean', 'zoomAmt', 'zoomRate', 'beatSecs', 'lockIn', 'lockM'] as const;
export type AutoKey = typeof AUTO_KEYS[number];
export type AutoTuned = Pick<typeof AUTOCAM, AutoKey>;
export type AutoMode = 'never' | 'rest' | 'gentle' | 'free';
const tunedOf = (K: typeof AUTOCAM): AutoTuned => { const o: Record<string, unknown> = {}; for (const k of AUTO_KEYS) o[k] = K[k]; return o as AutoTuned; };
/** the numbers as built … */
const FREE: AutoTuned = tunedOf(AUTOCAM);
/** …and the modes. `gentle`: a turn at rest only for a wall that hides him, eased and slow; the double tap's swing eased too; a small slow lean of
 *  the view in a fight and none for the way he walks. `rest`: those turns, and nothing while he moves or fights. `never`: the view turns only by the
 *  hand and by the double tap he asks it with (the frame and the zoom go on). */
const GENTLE: AutoTuned = { ...FREE, restAhead: false, turnCap: 30 * DEG, swingCap: 180 * DEG, swayFightMax: 15 * DEG, swayMove: 0, swayRate: 6 * DEG };
export const AUTO_MODES: Record<AutoMode, AutoTuned> = { never: { ...GENTLE, rest: false, swayFightMax: 0 }, rest: { ...GENTLE, swayFightMax: 0 }, gentle: GENTLE, free: FREE };
/** what a row may be put to, in the table's own units (ui/findme.ts draws its sliders from these; a kept line is held to them). A speed's 0 is 'as
 *  built' and its least is the first real notch */
export const AUTO_RANGE: { [K in AutoKey as AutoTuned[K] extends number ? K : never]: [min: number, max: number] } = {
  turnCap: [10 * DEG, 120 * DEG], swingCap: [30 * DEG, 210 * DEG], still: [0.5, 5], gap: [1, 15], swayFightMax: [0, AUTOCAM.swayMax], swayMove: [0, 30 * DEG], swayRate: [2 * DEG, 20 * DEG],
  leanK: [0, 1.5], lean: [0.5, 5], zoomAmt: [0, 1.5], zoomRate: [0.4, 2.4], beatSecs: [0.5, 8], lockIn: [0, 0.6], lockM: [4, 12] };
const held = (k: AutoKey, v: number): number => { const r = (AUTO_RANGE as Record<string, [number, number]>)[k]; return (k === 'turnCap' || k === 'swingCap') && v <= 0 ? 0 : Math.max(r[0], Math.min(r[1], v)); };
export const AUTOTUNE_START: AutoMode = 'gentle';
export const AUTOTUNE: { mode: AutoMode; over: Partial<AutoTuned> } = { mode: AUTOTUNE_START, over: {} };
const LS_TUNE = 'twopointsix.autotune';
const keepTune = () => { try { const plain = AUTOTUNE.mode === AUTOTUNE_START && !Object.keys(AUTOTUNE.over).length; if (plain) globalThis.localStorage?.removeItem(LS_TUNE); else globalThis.localStorage?.setItem(LS_TUNE, JSON.stringify(AUTOTUNE)); } catch { /* no store */ } };
/** the table as the switch, the mode and the rows over it have it: called by whoever sets `tune` (host/profile.ts, host/standup.ts) and by every
 *  change below */
export function autoTuneApply(): void { Object.assign(AUTOCAM, FREE, AUTOCAM.tune ? AUTO_MODES[AUTOTUNE.mode] : null, AUTOCAM.tune ? AUTOTUNE.over : null); }
/** a recording from before the key (game/journal.ts applyKnobs, with the table it carries): the switch off, and of the numbers only the ones that
 *  recording has no word for go to the module's — what it does carry stands */
export function autoTuneOld(carried: Partial<Record<AutoKey, unknown>>): void { AUTOCAM.tune = false; for (const k of AUTO_KEYS) if (carried[k] === undefined) (AUTOCAM as Record<string, unknown>)[k] = FREE[k]; }
/** a mode chosen: its numbers, and no row over them */
export function setAutoMode(m: AutoMode): void { if (!Object.hasOwn(AUTO_MODES, m)) return; AUTOTUNE.mode = m; AUTOTUNE.over = {}; autoTuneApply(); keepTune(); }
/** a row moved: kept over the mode only while it differs from the mode's own */
export function setAutoTune<K extends AutoKey>(k: K, v: AutoTuned[K]): void {
  const own = AUTO_MODES[AUTOTUNE.mode][k], same = typeof v === 'number' && typeof own === 'number' ? Math.abs(own - v) < 1e-9 : own === v;   // (a row put back on the mode's own notch through its unit is the mode's again)
  if (same) delete AUTOTUNE.over[k]; else AUTOTUNE.over[k] = v;
  autoTuneApply(); keepTune();
}
/** the numbers are not the mode's own: a row stands over it, or something else wrote the table (a recording being played carries its own) */
export function autoTuneOwn(): boolean { const M = AUTO_MODES[AUTOTUNE.mode]; return Object.keys(AUTOTUNE.over).length > 0 || AUTO_KEYS.some(k => { const a = AUTOCAM[k], b = M[k]; return typeof a === 'number' && typeof b === 'number' ? Math.abs(a - b) > 1e-9 : a !== b; }); }
export function resetAutoTune(): void { setAutoMode(AUTOTUNE_START); }
/** what was kept, read back: a mode this build has, and of the rows only keys it has, of the kind it keeps, held to what a row can be put to. A
 *  line whose mode this build does not have is not read at all (its rows were over that mode, not over this build's start) */
export function autoTuneLoad(kept: unknown): void {
  const k = kept as { mode?: unknown; over?: Record<string, unknown> } | null; if (!k || typeof k !== 'object') return;
  if (typeof k.mode !== 'string' || !Object.hasOwn(AUTO_MODES, k.mode)) return;
  const over: Record<string, unknown> = {}; if (k.over && typeof k.over === 'object') for (const key of AUTO_KEYS) { const v = k.over[key]; if (typeof v !== typeof FREE[key]) continue; if (typeof v !== 'number') over[key] = v; else if (Number.isFinite(v)) over[key] = held(key, v); }
  AUTOTUNE.mode = k.mode as AutoMode; AUTOTUNE.over = over as Partial<AutoTuned>;
}
try { const raw = globalThis.localStorage?.getItem(LS_TUNE); if (raw) autoTuneLoad(JSON.parse(raw)); } catch { /* no store, or a line that is not one */ }
