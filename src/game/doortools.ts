// DOOR TOOLS (Game.doorTools, __doortools): a page of door rows on the squad's command menu (orders.ts — X or the middle button) when it is opened on a
// door: row 5 of the door's menu, 'door tools ▸', turns the menu to them, on 1 to 4 (no row of any menu is on a digit above 5 — orders.ts ORDER_DIGITS).
// Rows 1 to 3 are the ONE-PRESS ENTRIES; row 4 turns to a page of its own, the room WORKED FROM OUTSIDE first (X, 5, 4, then 1 to 3).
//
//   1  open and clear          the men stack on the door, the lead opens it (the boot when it is locked against the team) and they go in
//   2  bang and clear          the same, with a stun canister in first: only the man who opens is at the frame, the rest file up behind him along
//                              his wall and the man behind him throws it (`secondThrows`); the can is cooked — it goes off in the air, chest high, `airIn` inside the frame line, over the furniture that would
//                              shadow one lying on the floor; nobody crosses the threshold until it has gone off AND the defenders' blind burst through the
//                              doorway has run dry (or `entryCap` seconds are up). The men it blinds stay dazed a beat after their sight is back
//                              (roomfight.ts FIGHT_SKILL.hostile.daze): that beat is what the entry is timed into
//   3  breach with a charge    one man sets a charge by the lock, the squad stands off along the wall, and NOTHING happens until Sam's own trigger (F, or the
//                              same row — it reads 'blow the charge' while one is set on that door): the leaf goes in off a broken keep, the men near its far
//                              side are knocked down or flash-blind, and the squad goes in behind it
//   4  work it from the threshold ▸   a page (THR_PAGE) of three — the room worked from Sam's side of the frame line before anybody is in it:
//        1  hold the door until I say     WORK THE ROOM FROM THE THRESHOLD and HOLD the jambs until told what next (order id 'thresholdHold' — below)
//        2  step in once it goes quiet    the same, and they step in by themselves once the door has been quiet (order id 'threshold')
//        3  clear from outside            (order id 'outside'; it was row 4 of the tools) the door is opened and the men work the opening from Sam's side of
//                              the frame line — ONE arc from the opener's jamb out, round the front of the door and in to the far wall, a station at a time,
//                              the muzzle (and the light on it) across what each station shows of the room; the lead walks all of it, the man behind him
//                              stops two stations short — and they STAY outside until an entry order sends them in
//   5  wait for my word        the squad of four only, on the tools and on the threshold's page: a row that flips (F flips it too, with a door menu up) — a
//                              tool given with it ON stacks and waits
// WHILE A THRESHOLD IS HELD ON THE DOOR by men the menu's orders go to (thresholdHeld: the jambs manned, nobody in — either order of it), row 5 of the door's
// own menu is 'threshold held ▸' in place of 'door tools ▸' (orders.ts OrderDef.standsIn) and turns to WHAT NEXT (HELD_PAGE): 1 step in · 2 bang, then step in ·
// 3 leave it — digits 1 and 2 are the plain way in and the way in behind a bang, as on the tools — and the word's own row, on its key (F: the blind pockets
// banged, then the step in). F is that word with no menu up too, and — the pair — with any menu up.
//
// WORK THE ROOM FROM THE THRESHOLD (kind 'threshold'; `threshold(men, target, onWord)` / `thresholdHold(men, target, onWord)` + `canThreshold(target, men)`
// are the two rows' issue / valid, `thresholdOn(doorName, who?, side?, hold?)` gives it from the console or a probe, `order('threshold', …)` from a
// scenario; a room being held takes `thresholdThen('step' | 'bang' | 'leave', men, target?)` — `canThresholdThen` its valid, `thresholdTell(what, doorName?,
// who?)` the console's):
// the men stack a man on each jamb, the door is opened, and NOBODY goes in — the two jamb men hold their diagonals across the room, still, and shoot who comes
// to the noise, everybody else keeps the squad's back; when it has been quiet the floor no lit muzzle has been across — the blind pockets — is banged (cans off
// the same count, not thrown by the first man in); behind the last burst they STEP IN SHALLOW, and from there it is the entry every other tool ends in.
// TWO WAYS OF IT, both orders: the plain one goes on by itself once the door has been quiet; given to HOLD, the jambs are held — whoever comes to the door
// still shot — until Sam says what next: step in, bang the pockets and step in (the word — F — is that one), or leave the door.
// Never automatic; with it not ordered nothing of it runs (the DOORTOOLS.thr* knobs say what was measured and what is a guess).
//
// CHARGES AND BANGS ARE COUNTED per run (a staging of the squad): DOORTOOLS.charges / .bangs, shown on the rows and on the squad's HUD block; a row with none
// left is greyed. The night breach's F drill takes its stun canister from the same count. Sam's own canisters (slot 4) are his own, as ever.
//
// What this module does and does not own. It moves men ONLY through orders.ts (SquadOrders.assign: one assignment per stage — a man re-ordered by anything else
// is no longer this module's, found by his assignment no longer being the one it handed him), and it never fires a round: who a man shoots at, from the stack,
// from a slice station or inside the room, is the room fight's (roomfight.ts, through GuardScript.over). It never touches a defender's hold / pinned / facing.
// The door is opened with the game's own verbs (doors.ts kickIn; the keyed shove squad.ts RoomClear uses), the stun canister is a real item detonated by
// combat.ts detonateStun (the men of the entry kept off its list, heads turned, as the breach drill does), the charge's effect on men is the same dazzle that
// rule applies plus a knock-down ('struck': out cold) inside `knockM`. Stack marks are orders.ts stackPosts (geometry: every door, either face) — the F drill
// keeps its authored marks on the one door it knows. Every stage is capped: a man who never arrives does not hang the order.
//
// ORDERS FROM A SCENARIO (the hostage rescue's F — rescue.ts). `order(kind, men, door, side, gate)` gives one of the entry orders for Sam without the menu
// (nothing in the order log, nothing said for Sam); `together()` makes a GATE that two or more of them share: nobody opens his door until every order on the
// gate is stacked (and, made with `go` false, until somebody raises it); a door that only needs a shove waits for a boot elsewhere on the gate and opens on the
// frame it lands (two boots on one gate each start their walk-up when the gate opens and land on their own count); an open and clear beside a bang and clear (no can for that door) goes in when the men behind
// the can do, not ahead of the flash; the men of one order are not blinded by another's can; while it stands stacked a gated order waits on a man the fight
// has on his way, and a lead lost there is replaced on the frame before anything opens. `order()` asks what the menu asks (floor at a jamb, a way there for
// each man, not steel and locked). `linked`: doors whose set charges blow on ONE trigger. `lights`: a scenario's own weapon-light switch for men going in. Where a door's jambs have no floor, the stack and the stand-off take the marks a scenario registered for that face (orders.ts
// doorMarks — a fallback: never read where the geometry has an answer). With no gate, no link and no marks this module runs exactly as it did.
//
// With no operative on the floor nothing here runs (tick: a pass over the guard list and a return; the rows are orders.ts's, inert with it).
// (src/game is one import cycle: nothing below is built from another game module's exports at module load.)
import { eachSeat, seatShare } from './seats';
import { SideMap, SideSet } from '../math/side';
import { holdFireOrder } from './holdfire';   // (called / read inside blow / goMen / testStacks only)
import { FRAG, exposedTo } from './frag';
import { moraleBlast } from './morale';
import { Vec3, v3, clamp, lerp, wrapAngle } from '../math/vec';
import type { Game, Guard } from './game';
import type { Door } from './doors';
import { DOORWAY } from './doors';
import type { Item } from './throwables';
import type { Assignment, OrderDef, OrderTarget, StackPost, Who } from './orders';
import { sideOf, doorWords, WORD_HEAD, OWN_EPS } from './orders';
import { CMDRING } from './cmdring';
import './character';
import { killGuard, detonateStun } from './combat';
import { tearOff, DOORBLAST } from './doorblast';
import { toolSpot } from './handleside';
import { SHOTGUN, breachSpot, carriesBreacher } from './shotgun';
import './weapons';
import { BIGSLOT, kitBig } from './kit';   // (the big slot's switch and what a staged teammate carries in it: asked inside the lock stage only)
import { carriesHammer } from './bigkit';
import { SILENT, squadSilent, silentUrgent, silentNoBoot, silentPick, silentBoot, silentEase, silentNoCans, silentTool } from './silent';
import { doorPlan } from './roomclear';
import { NIGHTBREACH } from './nightbreach';
import './handwork';
import './workstow';   // (called inside samTick and the 'place' stage only)
import { nvgWorn } from './nvg';
import './irlight';
import { ENTRY } from './entrypoints';
import { lightEstimateAt, LIGHTS } from './guards';
import type { RoomClearPlan } from '../scene/level';
import { FloorSeen } from './floorseen';
import { DOORMANNERS, DOORMANNERS_STATS, doorwayInWay, doorwayClearNear, doorwayLane } from './doorway';
import { GLASSPEN } from './glasspen';   // (DOORWAY.kickGlass: read in bootSpot, at order time only)
import { wg } from './words';   // (a leaf: the dialogue pass's one switch)
import './autopilot';
import { STACKROOM, stackRoomFix, stackRoomHas, stackRoomKeeps, stackViewReach, stackViewOpen } from './stackroom';
import { SAMSTUN, dazzlePlayer } from './samstun';
import { rearLast, rearWatcher } from './rearwatch';
import { gunFirst, gunTurn, gunBefore } from './gunroles';   // (called inside firstIn / enter only)
import { OWNCHARGE } from './owncharge';   // (a leaf: the player's own charges' one switch)
import { OWNBLOW, SAM_BY } from './ownblow';   // (called inside rows, notes and the key's press only: his own charge is his own actions' to set off — ownblow.ts)
import { PICTURE, PICTURE_STATS, anyIdea, safestSets } from './picture';
import './runner';   // (asked inside a tick only: false with its switch off)
import { word } from './belief';
import { RUST_PASS, RUST_SURFACE, goneWithTick } from './crossing';

/** `WORDCUTS_BREAK=1` (a check's own word): the rule's body left out, everything else as it is */
const WORDCUTS_BREAK = (globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.WORDCUTS_BREAK === '1';

export const DOORTOOLS = {
  /** per run: door charges, and the squad's stun canisters (the F drill's included) */
  charges: 2, bangs: 3,
  /** the trigger: Sam's key for a set charge (the night breach's F — a set charge has it first) */
  blowKey: 'KeyF',
  /** the squad of four: that same key is THE WORD while anything of the men the orders go to is waiting on it (a pair stacked, a tool given 'on the word', a
   *  charge set) — one press, no menu. False: the word is the menu's 'go' row alone and the key is the trigger / the drill's as before */
  wordOnKey: true,
  /** … and for this long after a word given on the key, the key does nothing else: a second tap is not the breach drill's 'stack up' */
  wordTap: 0.6,
  /** THE THRESHOLD'S MENU — the calls taken on it while the maintainer was away (rebuild/away-build/calls.md), each with the other side one switch off:
   *  `thrHoldFirst`: on the threshold's page 'hold the door until I say' is key 1 and 'step in once it goes quiet' key 2 (false = the two trade keys).
   *  `thrBangFirst`: on a HELD threshold's page 'bang, then step in' is key 1 and the plain step in — worded 'step in · no bang' — key 2 (false = as it was:
   *  1 'step in', 2 'bang, then step in'. Told to step in with no can at every held door the team kept 157 of 432 men where behind a bang it kept 208).
   *  `thrOutsideRow`: 'clear from outside' keeps its row, the last of the threshold's page (false = not on the menu; doorTools.order('outside') still gives it).
   *  `thrLeaveFallIn`: 'leave it' brings the men back to Sam, as 'fall in' does (false = as built: they stay stacked on that door).
   *  `thrHandsOff`: 'step in once it goes quiet' is hands off once given — the door's menu keeps its tools row and offers no what-next for it (false = as built:
   *  the same step in / bang / leave rows while it is on the jambs). The word's key sends it either way.
   *  `thrHeldAll`: row 5 turns to 'threshold held' only when EVERY man the menu's orders go to holds that door (false = as built: any of them does).
   *  `thrMenuWord`: the pair's key is a held threshold's word with ANY menu up (as built; false = with a menu up that is not the held door's own the key is
   *  not the word: it goes on to whatever else it is there — a set charge, the scenario's own drill, which may well take the men off the door).
   *  (thrHandsOff / thrHeldAll take the 'F: go' wording off that door's menu with the page: the word's key then sends it with NO menu up.) */
  thrHoldFirst: true, thrBangFirst: true, thrOutsideRow: true, thrLeaveFallIn: false, thrHandsOff: false, thrHeldAll: false, thrMenuWord: true,
  /** caps (s): walking to the marks (given up sooner once nobody short of his has moved for `stillSecs`) · onto the kick spot · the whole entry */
  stackCap: 60, stillSecs: 2.5, kickCap: 5, inCap: 9,
  /** the keyed shove: leaf velocity (rad/s) — and how long a door this module opened is kept from its closer */
  shove: 5.5, holdOpen: 10,
  /** the stun canister: released this long after the leaf starts to move, thrown for a spot this far inside the frame line, and going off IN THE AIR once it
   *  is `airIn` inside and has dropped to `airH` — else this long after it lands, or `tossCap` after it left the hand */
  tossAfterOpen: 0.15, bangIn: 2.4, fuseAfterLand: 0.35, airIn: 1.6, airH: 1.2, tossCap: 1.6,
  /** a door that had to be BOOTED: the can is the second man's, cooked in his fist while the lead kicks, and leaves it this long after the leaf goes in (the
   *  breach drill's own toss) — the kicker is square in the opening until it goes off, so every tenth counts (it was the kicker's own throw, 0.3 s after) */
  tossAfterKick: 0.05,
  /** the can is the SECOND man's on every door, not only a booted one: the man nearest the frame behind the first man (within `throwReach` of it) throws it
   *  past him, the first man's gun stays on the opening and he is first through with it ready (false = the man who opened the door threw it, then went first). `tossTurn`: seconds a thrower who was rear
   *  security until the shove may take to come round to the door before he throws — 0, he throws as he turns: at 0.35 the door stood open with no can in
   *  the air that much longer, and the pair holding the next room lost four of twelve where it loses none (pairs-hold) */
  throwReach: 4, tossTurn: 0,
  /** FRAG AND CLEAR — THE LEAF IS LOOKED AT BEFORE THE GRENADE LEAVES THE HAND (`fragWay`, on; off = thrown `tossAfterOpen` after the shove whatever the leaf did,
   *  as it was). The shove does not always open a door: a defender walking up behind the leaf, or furniture against it, stops it in the first tenth of a second
   *  (and the man then pushes it back out at the stack). A grenade thrown at a leaf comes back off it and lies a metre and a half outside the door, among the
   *  stack, on a 2.4 s fuse (the warehouse's `off` door: two or three of the squad killed at one instant, seeds 6 24 53 73 frag-first). So the thrower throws
   *  only with the leaf WELL ROUND AND STILL GOING, away from the team (≥ `fragWayAjar` rad and really turning that way, by its angle from one look to the next —
   *  not `Door.vel`, which a booted leaf pinned on a man keeps for 0.6 s: past ~0.5 rad a grenade that meets the leaf is turned on INTO the room, with the 0.3
   *  restitution it has, so a leaf that stops the frame after the throw no longer sends it back), or STANDING OPEN (≥ `fragWayOpen` rad either way: clear of
   *  his hand's line and of the landing's own floor scan). An unstopped shove is at ~0.72 rad turning ~3.9 rad/s at the throw: the throw as it always was,
   *  frame for frame; a boot's throw waits a frame or two for the angle. A leaf AT REST part open is refused on purpose (the grenade would drop in the doorway).
   *  Speed alone is no reason to throw, at any small angle: a booted leaf stopped at 0.37 rad on the throw's own frame, or one resting on a man that lurches
   *  when the boot's 0.6 s run out, read as 'flying open' and the grenade came back (the review's sweep of where the man stands, 0.5 to 0.62 m behind the frame).
   *  Else he waits up to `fragWayWait` s for it, then the grenade goes back on his belt and they go in without it (the rule a mate behind the door already had).
   *  Nothing of the blast, the aim or the fuse changes: a grenade that does come back still kills whoever it reaches. */
  fragWay: true, fragWayWait: 1.0, fragWayOpen: 1.15, fragWayAjar: 0.6,
  /** the same look at the leaf for a STUN canister on a bang and clear (never a threshold's cans): off = thrown on the shove whatever the leaf did, as it was.
   *  The can off a stopped leaf hurts nobody of the entry (they are off its list) but it goes off on their side of the door and the room behind is not flashed.
   *  ON since 10-11: a door the squad PULLS open swings toward the stack, and a can thrown `tossAfterOpen` after it starts to move met the leaf every time and
   *  burst outside (the maintainer: "the squad does waste a lot of bangs"). Toward the team the leaf is never 'well round and still going', so the can waits
   *  for it to STAND OPEN (`fragWayOpen`): a pulled leaf is there about 0.15 s later. A SHOVED leaf's throw is the throw as it was, frame for frame; behind a BOOT
   *  (`tossAfterKick`) the can leaves a tick or two later, once the flung leaf is past `fragWayAjar`. Beats: 'can waits: …' / 'can held' ('bang · …' is the burst) */
  bangWay: true,
  /** a bang / frag order on a door that STANDS OPEN walks nobody across the front of the opening (the maintainer's replay, 10-12: "after being ordered to
   *  frag after externally clearing, blue one crosses across a dangerous threshold" — both men held the open door from either jamb, the new order filed the
   *  second man behind the lead on the lead's wall, and he walked 3.4 m through the doorway's own view to get there). With the leaf standing open
   *  (stackroom.ts `stackViewOpen`) nobody has a handle to work, so: the lead is a man whose jamb is on the hand of the opening he already stands on, where
   *  there is one (else the latch jamb's, as ever), and a man whose place is on the OTHER hand from where he stands takes his own hand's wall instead — a
   *  place back from that jamb, flat to the wall (the next places back, then the jamb itself, where that has no floor); no place there = the place as dealt,
   *  never a refusal. A man in front of the opening, far from the door or on its other face is on no hand and is dealt as ever. The can is thrown by the
   *  second man from his own end of the opening, and they go in from both jambs. Shut doors, 'open and clear' and the threshold are untouched.
   *  false = the deal as it was */
  openHand: true,
  /** SEVERAL DOORS ON ONE SIGNAL, one of them booted, another only wanting a hand: the hand door's lead used to wait on his jamb for the boot beside him to
   *  land and only THEN step to its handle (handleside.ts toolSpot — the page, where a leaf is worked from its handle), so its leaf, can and men were that walk
   *  behind the boot's (1.8 s on the office's hostage room). On, he steps to the handle as the signal is given and has his hand on it while the boot beside
   *  him goes in, and the boot is not swung before he has (`kickCap` at most): the shove is on the boot's frame. Nothing to it where no lead has a handle to
   *  step to (the frozen stand).
   *  false = the step after the boot, as it was */
  gateHand: true,
  /** three or more left holding a room: the LAST man through keeps his gun on the door they came in by (the footage's fourth man takes the corner by the door
   *  and turns to face back out) — the rest keep the sectors their points gave them (false = every sector into the room, as it was) */
  lastOut: true, lastOutM: 3,
  /** entry behind a bang / a charge: not before `afterBang`, then once no defender has fired for `quiet` seconds (the blind burst run dry) — at `entryCap` whatever */
  afterBang: 0.6, quiet: 0.15, entryCap: 2.8,
  /** the second man starts this long after the first · on his point of domination each man's muzzle goes `sweepRad` to one hand, to the other, then onto his
   *  sector, `sweepSecs` each */
  second: 0.25, sweepRad: 0.9, sweepSecs: 0.6,
  /** CHECK THE CORNERS (the maintainer, 10-05: 'teammates clearing rooms seem to miss enemies. they need to check their corners'): from the moment a man is
   *  through the frame his muzzle is on the NEAR corner of his own side of the door, then on that side's DEEP corner — each until it has been within
   *  `cornerRad` of it for `cornerSecs` (given up `cornerCap` after he crossed) — and only then his point's sweep. False: muzzle wherever he walks, as it was */
  cornerSecs: 0.3, cornerRad: 0.3, cornerCap: 1.6,
  /** … AND A SHOT DOES NOT COST HIM HIS CORNER (the footage, Ready or Not 7:45: the first two men 'are allowed to take a shot at that threat and then immediately
   *  collapse their corner'). `cornerOwed`: the seconds the room fight has his muzzle on a man are not seconds his corner had — `cornerCap`'s clock stands still
   *  for them (false = as it was: the cap ran from the moment he crossed, whatever his gun was doing, and a man who met somebody in the middle of the room
   *  walked on to his point with both corners of his side given up unlooked-at; the search got to them later, in whatever order needed the least turn).
   *  `collapse`: the man he was on DOWN and a corner still owed, his muzzle leaves the spot at once for the corner (false = as it was: the half second any
   *  teammate's gun rests where his man was). Only with a MATE THROUGH THE FRAME behind him — the middle of the room is that man's; alone inside he keeps the
   *  half second (where one stood the next often is: measured, a pair's first man who looked away at once lost both of them a room). A man he merely lost sight
   *  of keeps that half second either way: he is not dealt with.
   *  `cornerTurn`: stood on his point with the owed corner more than 1.3 rad round from his chest (he fought his way past it; his muzzle goes 1.5 rad round his
   *  chest and no further), he turns his body to it (false = as it was: squared to his point's facing, the muzzle against its stop until the cap ran out) */
  cornerOwed: true, 
  /** WHO GOES IN FIRST: the man who has already seen most of the room (the footage, Door Kickers: 'i'm going to lead with this man because he's already seen
   *  most of the room he knows what it looks like and where the threats are at'). Of the men AT the frame — the front man of the file on each jamb — the one
   *  whose own eyes have been over the most of the room's floor in the last `leadSeenSecs` (floorseen.ts: within `leadSeenM` of him, in his eye's wedge, with a
   *  line) is first through, when that is `leadSeenMin` cells (quarter square metres) more than the lead has seen; the lead follows off his own jamb. Behind
   *  a bang the can is then thrown by somebody who is not him — the lead himself, when they are two — and never is the man who threw it first through while
   *  there is another at the frame. A shut door nobody has looked through is the lead's, as ever: the man on the latch jamb, who opened it.
   *  false = as it was: the lead first whatever anybody had seen (who the lead is: the man the nearest mark put on the latch jamb) */
  leadSeen: true, leadSeenMin: 12, leadSeenSecs: 30, leadSeenM: 11,
  /** a man left holding a room swings his sector off a teammate who stands in it (off = as it was: every man on the sector his point gave him, and the gun of
   *  any man whose sector ran through a mate at the low ready for as long as they held — in one room three of the four) */
  sectorSecs: 0.6,
  /** … then SEARCH THE ROOM: the room's floor (the room fight's own model of it) is LOOKED OVER — a cell counts once it has been inside `lookRad` of a
   *  man's muzzle, within `lookM` of him, with a line to it — each man's muzzle travelling at `scanRate` rad/s to the floor nobody has looked at yet that
   *  needs the least turn. Floor no man can see from where he stands (`deadCells` of it or more) sends the nearest man on to where it can be seen, up to
   *  `searchMoves` times. The entry is over when the floor is looked over (or `searchCap` s after 'going in'); rooms past `searchCells` cells are open
   *  floor and get the fixed sweep alone. False: the two looks of the sweep, then every man frozen on his sector — as it was */
  search: true, scanRate: 1.4, lookRad: 0.3, lookM: 11, deadCells: 8, searchMoves: 3, searchCap: 24, searchCells: 900,
  /** this module's stacks stand this much further past the jamb than orders.ts's (a man 0.45 m past the latch is in the line of a round fired from deep in
   *  the room at an angle through the opening) · behind the bang the first man closes up to `edgeOut` past the opening's edge · behind a bang, or working a
   *  door from outside, the men who do not open it file up `bangFile` apart behind the man who does */
  tuck: 0.4, edgeOut: 0.4, bangFile: 1.1,
  /** the charge: seconds at the door to set it · how far along the wall from the jamb the squad stands off (first that has floor) */
  setSecs: 2.0, standoff: [2.6, 2.0, 1.5],
  /** … and what it does: far-side men within `knockM` of the doorway's middle go down (out cold); within `stunM` with no wall between they are flash-blind for
   *  stunSecs[0] (at the door) down to stunSecs[1] (at stunM) AND doubled over for `staggerSecs` (Guard.staggerUntil: rooted, gun down, no round — so no blind
   *  burst comes back through the doorway, which is the charge's edge over a canister); a teammate (or anybody) on the NEAR side within `backblastM` of it is
   *  dazzled `backblastSecs`; the leaf leaves at `blowVel` rad/s */
  knockM: 2.0, stunM: 8, stunSecs: [6.0, 2.5], staggerSecs: 2.0, backblastM: 1.6, backblastSecs: 1.5, blowVel: 12,
  /** clear from outside: × the arc's distances off the frame line · how long the muzzle rests on each of a station's angles with nobody in his sight · how far
   *  inside the jamb's edge the sight lines are taken · how deep into the room the points he aims at lie */
  sliceBack: 1, sliceLook: 0.35, sliceEdge: 0.12, sliceDeep: 3.5,
  /** a slicing man's step from one station to the next, m/s — the footage's men take an angle at under a metre a second (0 = the pace of any other short leg, as it was) */
  slicePace: 0.9,
  /** THE WORD (go()): the orders it releases wait this long for each other at the two moments they share — before anybody makes a sound (a pair still walking
   *  onto its door), and before anybody crosses a threshold (a bang still in the air at the other door) — then go without the one that is late */
  wordStack: 20, wordIn: 6,
  /** the squad of four: a pair stacked on a door the OTHER pair is sent to work steps back to the first of these (metres along the wall past a jamb) that has
   *  floor · two points of domination dealt to men of different orders are this far apart, or the later one takes another */
  roomBack: [2.8, 3.7, 4.6, 5.5], podApart: 1.2,
  /** WORK THE ROOM FROM THE THRESHOLD (kind 'threshold' — threshold() / thresholdHold() / thresholdOn(); the menu's THR_PAGE). A man on each jamb, the door opened, and
   *  nobody goes in. Each jamb man steps onto his own jamb's diagonal — `thrStill`: [metres past his jamb, metres off the frame line] — his muzzle down the middle
   *  of what it shows of the room, and holds it, STILL: between them the two lines cross inside, and whoever shows himself in them is the room fight's.
   *  `thrLight`: how many of the two burn a weapon light while they hold (1: the far-jamb man lights the door and the opener beside him stays dark — a lit lens
   *  is seen from anywhere in its cone, roomfight.ts lampGlare; 2: both; 0: neither, a door's own discipline — a light still comes on for a man found). The hold
   *  is over once nobody at the door has had a man in his sight, nor a defender fired within `thrShotM` of it, for `thrQuiet` s (not under `thrMin`, not past
   *  `thrCap`) — or on Sam's word (go(); its key), any time. Given to HOLD (thresholdHold) it is not over then: the lead says the door is quiet and the jambs
   *  are kept, whoever comes to the door still shot, until Sam says what next (thresholdThen: step in · bang and step in · leave it; the word = bang and step in).
   *  Then THE BLIND POCKETS — the room's floor no lit muzzle has been across from out there — are banged: `thrBangs` cans at most, while the kit has
   *  them, each thrown (not by the first man in while there is another) to go off in the air where it has a line to the most unseen floor — `thrPocket` cells
   *  of it or more, within `thrFlashM` of the burst, the burst no more than `thrThrowM` from the hand — the first `thrTurn` after the hold ends (the thrower
   *  comes round from the rear), the next not within `thrNext` of the last. `thrCover`: both jamb men stand in behind their jambs' edges for the bangs (the
   *  blind burst comes back through the opening; false = the far-jamb man keeps his diagonal through them). Behind the last burst, on a bang's own count, they
   *  STEP IN SHALLOW — the jamb men to the first free floor to either hand just inside the entry wall, the rest behind them and none past 2.5 m (`thrIn`: how
   *  many of the order step in at most; the others keep the squad's back, outside) — and from there it is an entry like any other: corners, the room looked
   *  over (what was seen from outside is not looked at twice), the sectors held.
   *  `thrSlice` (SHIPS OFF): the opener, instead of holding his diagonal, SLICES his side of the opening at `slicePace`, a station of `thrArc` at a time ([metres
   *  from his jamb's edge of the opening — negative: back along his wall —, metres off the frame line]), the muzzle across what each shows. Over the scripted
   *  play-throughs (five room-clearing scenarios × eight seeds, every held room worked this way, the plain order) it left 38 to 47 of 112 teammates standing
   *  where both men holding still left 49 to 65: a man walking a lit muzzle round a doorway is seen by every man in its cone before he has lit one of them.
   *  (The same forty runs with every room banged and cleared instead: 81. Run to run the same settings move by ten or so; nothing here is tuned past that.)
   *  The hold's length, one light or two and one can or two are guesses those runs could not sharpen (thrMin 2 s: 39; 4 s: 64; 6 s: 44; 9 s: 52 · as shipped
   *  49, both jamb men lit 50 to 54, both lit and one can a room 59 to 62 — the scatter is as wide as the trends) */
  /** A DOOR WORKED FROM THE FACE IT OPENS TOWARD (`thrPull`, on; false = as it was; nothing on the old two-way doors): its leaf comes round onto the
   *  team's own side and stands out from the hinge jamb — right across that jamb's diagonal, so the man holding it had his gun on the back of the door and was
   *  no part of the hold. He takes `thrPullAt` instead — [metres in from the hinge jamb, metres off the frame line]: past the leaf's free end, in front of
   *  the hinge half of the opening, his line still crossing the latch-jamb man's inside — and 'clear from outside' skips its stations beyond that jamb. */
  thrPull: true, thrPullAt: [0.45, 1.8],
  thrStill: [0.45, 1.2], thrArc: [[-0.45, 1.2], [0.25, 1.8]], thrMin: 4, thrQuiet: 2, thrCap: 25, thrBangs: 2, thrPocket: 12, thrShotM: 8, thrThrowM: 5, thrFlashM: 7, thrTurn: 0.5, thrNext: 0.3, thrCover: true, thrSlice: false, thrLight: 1, thrIn: 4,
  /** THE JAMB MEN SLICE THE DOORWAY BEFORE THEY CALL IT HELD (`thrPie`, on in the page; false = as it was: a diagonal each, held still — the maintainer, 10-18, of
   *  a pair that said 'holding on the door' with a defender stood square in front of it, in the wedge neither diagonal shows: "when externally clearing, men
   *  should try to pie from the outside as much as they can before considering their job done"). Each jamb man — both at once, each on his own side of the
   *  door's middle — takes his own mark's thin slice, his diagonal, and then the cuts of `thrPieArc` ([metres in from his own jamb's edge of the opening,
   *  metres off the frame line]: every one a few degrees more of the room past his own jamb), the muzzle on the ground each opens and then the middle of what
   *  it shows, at `slicePace`, resting `sliceLook` on each; a man in his sight keeps him on the angle that shows him. A cut that would show no floor the
   *  search has not had a muzzle across is not walked — he is done when none of his is left — and the hold is not quiet, nor 'holding on the door' said,
   *  until both are. No cut is nearer than `thrPieApart` to where the other man ends, and none is over the frame line. Then each holds the middle of what his
   *  last station shows.
   *  `thrLean` (on in the page; false = square on every station): THE MAN CUTTING LEANS — his hips stop `thrLeanM` short of the station, back along the
   *  wall, and his head and gun go the rest of the way (character.ts anim.lean: the eye IS the head bone, so what he sees and what sees him is what the
   *  picture shows; a right-handed man leaned to his left has his head out before his muzzle) — and THE CUTS THAT LOOK DOWN THE DOOR'S OWN LINE, where a
   *  man covering the door from inside is looking (a station further in than `thrKneeIn` from his jamb's edge), are taken ON A KNEE. The teammates' only:
   *  nothing of the player's own movement. */
  thrPie: false, thrPieArc: [[-0.2, 1.4], [-0.05, 1.6], [0.08, 1.8]], thrPieApart: 0.65, thrLean: false, thrLeanM: 0.15, thrKneeIn: -0.1,
  /** ONE MAN SLICES, THE OTHER HOLDS HIS ANGLE (`thrPieOne`, on in the page, nothing without `thrPie`; false = both jamb men cut, each leaned on every cut,
   *  the cuts down the door's own line on a knee — the maintainer, 10-19, of that: "really only one guy should be doing it. one guy holds one angle while
   *  the other pies and then does the lean? looks a bit silly now where the two of them synchronize their crouch and lean together … just do the lean, not
   *  the crouch"). The lead walks the cuts, square on every one; the far jamb's man steps onto his diagonal and stays there, his gun down the middle of
   *  what it shows, as with `thrPie` off. When the lead has no cut left that shows him new floor he LEANS, once (with `thrLean`), from where his last cut
   *  left his hips — head and gun round the edge at the ground the lean opens — for one look, then stands square and the door is called. Nobody kneels. */
  thrPieOne: false,
  /** … the seconds that one lean is held (a cut's own look is `sliceLook`: too short to be seen, or to be sure his lines are asked with his head out) */
  thrLeanLook: 1.1,
  /** THE ROOM'S OTHER DOORS ARE HELD (`thrDoors`; false = every man on the short crossing sector his shallow point gave him, as it was). The stage ends hard
   *  on the entry wall: in a long room that leaves its other ways in a room's length off, in nobody's sector — and the depot's crew room showed what that
   *  costs (thirty-two seeds: its far door, twelve metres down a dark lane from four men standing in the room's own light, let a man in unseen to shoot the
   *  first of them in twelve runs; bang and clear, three). MEASURED over the five room-clearing scenarios × 32 seeds, every held room worked this way (448
   *  teammates): 258 up before, 284 with this alone (bang and clear: 280) — the depot 63 → 95 of 128 (bang 97); watching the far door from the entry wall
   *  instead of walking up beside it (`thrDoorsM` 11) gave back most of it, 72. So once the floor is searched each other door of the room gets ONE man of the entry (never the man
   *  left on the door they came in by): the one who has a line to it from within `thrDoorsM` turns his gun onto its frame where he stands; with nobody that
   *  close a man walks up to the first free floor two to four metres from it and off its line — beside the opening, not in front of it — with a line to the
   *  frame (no walk past `thrDoorsWalk` metres by the floor; a room too big for the entry's search — open floor — hands out none; a door another order
   *  works, one already held, or one a teammate of another order stands on is left to them). `thrDoorsLit`: a man with NO GOGGLES — or one whose goggles show him nothing there (nvg.ts: black ahead) — keeps his weapon light on that door for as long as he holds it where the place does not light it; with goggles that show it he holds it dark like any sector: over thirty-two depot seeds lit and dark came out the same, 95 and 96 of 128 (the light rule's dark hold is
   *  for a sector across a searched floor; a doorway is where the next man comes from — and a lit room's men see nothing of a man in a dark frame without it) */
  /** `thrDoorsStay`: with nobody left on the door they came in by (that is the last of three or more — a pair, or a lone man, has no such man) that many of
   *  the entry's men are NOT sent: a lone survivor walked off to a side door with his back to the way in, and the registry's roamer came in behind him — 8 of
   *  32 rescues lost that had been won (hostages alive 70 → 54 of 128; with it 70 again. 0 = every man may be sent) */
  thrDoors: true, thrDoorsM: 5.5, thrDoorsWalk: 16, thrDoorsStay: 1,
  /** THE ROOM'S OTHER DOOR HAS A GUN ON IT FROM THE ENTRY ITSELF (`otherDoorM`, metres; 0 = as it was: every man of a bang / open and clear on his corners,
   *  the floor and then the sector his point gave him, the room's other doors in nobody's — the warehouse's front office and the compound's garage: two men
   *  shot in the back through the connecting door within seconds of taking the room, 72 of 167 teammates lost in 48 warehouse runs). Every other door of the
   *  room being taken that NOBODY OF THE SQUAD HAS BEEN BEYOND (entrypoints.ts farKnown: one model — what the team has seen for itself) is one man's from the
   *  moment a man of the entry is on his point, his corners checked, with a line to its frame from within this many metres: the nearest such man turns his gun
   *  onto it where he stands and keeps it there — through the rest of the search (the floor is the others': while the room is still being looked over no
   *  more than half the entry's men are turned off it — one of two or three, two of four) and on the hold after, until an order takes him; if he drops, the
   *  next man with a line has it. With more doors than men to spare: a door into the building before one to the outside, the nearer first. Nobody walks
   *  anywhere for it. A door another order works, one a man already holds, or one a teammate of another order stands by or was told to watch (a man Sam
   *  posted there himself) is left to them — and given back to them if they come to it after: its man is on his sector again; so is the man of a door the
   *  squad has since gone through. The man on a door holds it as a held door is held: `thrDoorsLit` — with no goggles, his weapon light stays on it. (The threshold order's own far doors — `thrDoors`, with its walk — come after, for what is still nobody's.) */
  otherDoorM: 9,
  /** A BANG IS ITS OWN SIDE'S CAN TO EVERY TEAMMATE (`ownCan`; false = as it was). bang() takes the entry's men — the thrower among them — off the guard list
   *  for the call, so combat.ts detonateStun, looking the thrower up on that list, found nobody and took the can for a guard's: a teammate still on the list
   *  (the other pair back down the passage with a line through the doorway, a man beyond the door) got a defender's full count and fold from his own side's
   *  can, where the own-side rule is 'only a man on top of it' (FLASH.ownM). On, the can says whose it is (Item.side). THE SAME WRONG ANSWER HAD A SECOND
   *  HALF, and the switch turns both: a man of team 'guard' (the staff a quiet mission's partner is up against, a response team) was taken for the can's
   *  OWN side and spared outside ownM; on, he is a defender to it like any other. `stats.ownFlashed` counts teammates outside the entry's list that a bang
   *  of the team's FOLDED (the defender's treatment: the own-side touch never folds a man), whatever the switch says. Not touched: breach.ts's drill and
   *  qrf.ts swap the list the same way — the drill's list holds no operative, the QRF's can is a guard's by the lookup's fallback, which is right. */
  ownCan: true,
  /** NOBODY IS SENT OFF ALONE (`buddy`; false = as it was). the maintainer, 10-07, playing the four-man room clear: "squad mates will sometimes leave the group and
   *  go off on their own (which seems very risky)". Two rules here dealt ONE man a walk with no thought for where his mates were: the room search's move-on
   *  (the man nearest the unseen floor walks to where it is best seen from — on the office's open floor that was ten metres across the cubicles, or twenty to
   *  the lobby, three men one after another, each by himself: room-clear-four seed 7, a man shot in the back six metres from where he was sent) and the
   *  threshold's far doors (one man walks up to sixteen metres to hold each). With this (away()): a point within `buddyM` BY THE WAY A MAN WALKS of where a
   *  friend stands or is going (a teammate, or Sam) is still one man's — he has not left anybody. Past that it is taken BY TWO or not at all: his pair-mate
   *  goes with him to a point beside his, on a sector his partner's is not (a man who has lost his partner goes with another who has; a pair is never split
   *  to make one, and a man whose partner is in a fight does not walk off: the other pair goes; two men with nobody else of the team up do not both
   *  walk on — somebody holds what was taken). Nobody, alone or paired, is sent past `buddyPairM` from
   *  everybody outside his pair (the squad is two pairs, not two patrols), and no walk is dealt that would leave a man who has company now with nobody
   *  inside `buddyLeftM`. (The far doors: `buddyDoors`, below.) A door nobody may walk to is the room's own cover's (cover.ts COVER.inRoom); floor nobody may walk to stays unseen, and the
   *  search says so ('N cells unseen'): it is the next order's. */
  buddy: true, buddyM: 5, buddyPairM: 12, buddyLeftM: 8,
  /** THE REST OF THE ROOM IS GONE INTO, NOT LOOKED AT FROM ITS MOUTH (`flowOn`; false = as it was). the maintainer, 10-10, the consul's office taken by its side
   *  door — the team bunched on its points by the door, the other half of the room (through the gap at the end of the wall across it) dark and nobody in
   *  it: "in corner fed rooms like this, the squad should really clear the whole thing without being told". The points of domination are a couple of metres
   *  from the door whatever the room's shape, and the search's move-on sent ONE man to wherever the unseen floor was best SEEN from — the mouth of the gap —
   *  and called the room done from there. With this: unseen floor in one piece of `flowCells` cells or more (a nook behind a cabinet is still looked into
   *  from where it can be seen) is a part of the room nobody has been in, and it is TAKEN like a room: the men nearest go THROUGH its mouth — two of them
   *  when a partner is free and somebody of the team stays behind them (away(), every rule of `buddy` as it stands) — to two points past the mouth, each
   *  by a wall where there is one, to either hand of the way in, with a line to each other and to as much of that floor as there is to see (within
   *  `flowReach` of the mouth, about `flowDeep` in); they look it over from there, and what is still unseen after that is gone into the same way, up to
   *  `flowMoves` times. The men who stay hold what was taken and the door they came in by. A LONG ROOM IS GONE THROUGH IN BOUNDS: floor further off than
   *  `flowBound` by the way a man walks is walked TOWARD, a bound at a time, and when `buddy` refuses the walk for its length from the men who stay, those
   *  men CLOSE UP on the man at the front first and the walk is asked again — the squad moves down the room together, the rear behind the front, never
   *  strung along it and never past `buddyPairM` apart. Each walk gives the entry `flowSecs` more before it is called,
   *  and the time its men spent in a fight does not count against the search (up to `flowFight` s). */
  flowOn: true, flowCells: 28, flowMoves: 5, flowReach: 5.5, flowDeep: 3, flowBound: 8, flowSecs: 10, flowFight: 20,
  /** WHERE CLEARING THE WHOLE ROOM STOPS. the maintainer, 10-11: "teammates are a bit too proactive now. i like that they try to fully clear rooms now, but it doesn't
   *  make sense for outdoor spaces or very large rooms." `openAir` (false = as it was): floor with no roof over most of it — a walled yard between buildings,
   *  145 m² of the compound — is OPEN FLOOR whatever its size (openFloor(), below): the entry's fixed sweep alone, no search of it, none of its far doors
   *  dealt, like floor past `searchCells`. `flowMax` (cells of free floor, a quarter of a square metre each; 0 = no limit, as it was): a room bigger than
   *  this — a hundred square metres to walk on: the office's open floor, the warehouse's hall, the consulate's — is searched as rooms were before `flowOn`
   *  (flows(), below): looked over, a man stepping on to where unseen floor can be SEEN from up to `searchMoves` times, called on `searchCap` — and nobody
   *  is walked INTO its unseen parts by pairs or down it in bounds: the far end of such a room is the next order's. */
  openAir: true, flowMax: 400,
  /** A DOOR THAT TURNS OUT SHUT, AND 'CLEAR FROM OUTSIDE' AS ONE ORDER. The maintainer, 10-11, on the defenders' wedged doors: "the default open and clear is now just
   *  suicide. if we don't know a door is locked or barricaded then it should just halt and they'll tell you when they test the door. and then maybe
   *  x -> 5 -> 1 then kicks it?" · "oh i like in ready or not then when you tell your squad to stack on a door they check it." `testDoor` (false = as it
   *  was: the lead says 'locked. kicking it.' and boots it in the same breath): the man on the jamb TRIES THE HANDLE when the stack is made — a plain 'stack
   *  up' and every entry order alike — and says what he found; an entry order at a door the team did not KNOW was locked or wedged, which would have gone to
   *  the boot (or the shotgun), HALTS there: its men stay stacked on their marks and the door is known from then on, so the same row — worded 'kick and
   *  clear' now — is the order that boots it. What the team knows: a door one of its men has tried, and a door whose lock rows the player has stood at.
   *  Quiet ways through (a silent squad's pick, a lock picked because the frame holds a boot) go on as they did: nothing loud was about to happen.
   *  Not asked of an order given by code (a scenario's own entry) nor at a captor's last warning.
   *  `outsidePlain` (false = the threshold's page of three orders and the held page of what next, as they were). His words: "i think the externally clear
   *  menu is a bit odd. it should just clear externally w/o the additional orders of what to do after. and then the squad does waste a lot of bangs in
   *  that path. so maybe it's just one level of external clearance and then you have to give a second order for them to go in". Row 4 of the door tools is
   *  ONE order, 'clear from outside' = the jambs held until told (thresholdHold: no can of its own); going in is the page's own rows given again — 'open
   *  and clear' steps the holding men in with no can, 'bang and clear' puts ONE can in first — and a can asked for at a held door is one can, not `thrBangs`. */
  testDoor: false, outsidePlain: false,
  /** THE WORD AT A DOOR HELD FROM OUTSIDE SPENDS NO CAN (acts with `outsidePlain`; false = the word bangs the blind pockets, then they step in, as it
   *  was). The men holding a door from outside go in on the word with no can; a can goes in first only when it is asked for — 'bang and clear' given
   *  to the holding men (row 2 of the door tools: one can, then in), or the word told 'bang, then step in' where that row exists. The maintainer, 10-16,
   *  off his own tape (the word given at a held door, a can nobody ordered into an empty room): "when externally clearing a room, the men shouldn't
   *  throw a bang unless i tell them to. i can do that by doing x 5 2 again". */
  wordNoCan: false,
  /** THE WORD WAITS FOR THE CUTS (`wordCuts`, acting with `wordNoCan` and `thrPie`; the page: on — `?wordcuts=0` — false here and on the frozen harness
   *  stand, `TPS_WORDCUTS=1`; a FROZEN_HOLDS row; false = the word steps them in there and then, as it was). The word that spends no can, given while the
   *  jamb men are still slicing the doorway, does not walk them off their cuts: the step in is OWED (the owed step a grenade's wait already uses), they
   *  finish the cuts — fighting whoever a cut shows from the frame, as the hold does — and step in on their own the moment the cuts are done, or after
   *  `wordCutsCap` seconds whatever is left. The word AGAIN (outside `wordTap`) pays it at once, as an owed step always could: the player's say stands.
   *  Not under a captor's last warning, nor at a door that shares its word with another, nor with a set charge on the same word. From a lesson's sixteen runs on the page's stand (10-23): the
   *  word a second and a half into a hold, a defender awake in the band neither jamb's diagonal shows, both men walked to the door's middle at once and
   *  were shot in its width before either had seen him, on ten; the same door left to finish its cuts found him and dropped him from the frame, on all. */
  wordCuts: false, wordCutsCap: 8,
  /** THE WORD SENDS A SMALL SQUAD IN FROM ITS STACK (false = as it was: with one or two teammates the word's key does nothing for men standing on a
   *  plain 'stack up', and no line of the panel offers it — only the squad of four had it). One or two teammates stacked on a door by a plain 'stack
   *  up' (the quick key on a door, or the menu's row) wait on the word as the four do: the panel reads 'F go · … waiting', the door carries the key's
   *  mark, and the key opens the door and they go in from the marks they stand on — the same open-and-clear the four's word makes of a stack (fromStack).
   *  A stack on a door that has a charge set on it is left out: there the key is the trigger's first, as it was, and the word's after. A free set
   *  charge elsewhere goes with the word, as with the pair's word at a held threshold. The quick key on the stacked door is untouched (the squad's own
   *  pick of entry), and so are the pair's menus and the word from code (`go()`): the rule is the key's, with no menu up. The maintainer, 10-21, off his own play: "Q on a door stacks but doesn't have the F command for entering?" */
  wordSmall: false,
  /** A BANG AND CLEAR'S CAN IS THROWN INTO THE ROOM (false = as it was: the can appears in the door frame at the thrower's end of the opening, up to two
   *  metres from him, and goes off in the air `airIn` 1.6 m past the frame a third of a second later, on every door). Two things, the maintainer's
   *  ("with bang and clear for a door, they often don't really throw the flash bang into the room", 10-16, with a tape of it):
   *  WHERE IT GOES: the point of the floor beyond the door — `bangIntoMin` or more inside, no more than `thrThrowM` from the hand, a clear throw from
   *  the opening — whose burst, chest high, has a line to the most of the floor the doorway opens onto; of the points that light `bangIntoLit` of what the
   *  best one lights, the DEEPEST (a can thrown into a room goes past its middle); it goes off as it gets there, as a threshold's pocket can does. Never a
   *  point with one of ours in its flash. No such point (a doorway onto a wall, a cupboard): the landing as it was.
   *  WHERE IT LEAVES: the thrower's own hand. A thrower more than `bangHandM` from the opening's edge with a clear line to it passes the can to that edge
   *  first — a quick flat lob past the first man, `bangPassV` m/s — and it turns in off the jamb. A leaf or a wall between them: from the edge, as it was.
   *  Who throws (the second man), when (the leaf well round), and whose list the burst keeps off are as they were. */
  bangInto: false, bangIntoMin: 1.5, bangIntoLit: 0.9, bangIntoDeep: 6.0, bangIntoWide: 4.2, bangHandM: 0.9, bangPassV: 6.5,
  /** HALTED ON A DOOR THE OTHER PAIR IS CHARGING (false = as it was). A pair stacked on a door steps back to rear security when the other pair is given a
   *  charge on it — asked once, as that order is given (startCharge). A pair that came to the door AFTER, on an entry order, and halted there 'found shut'
   *  (`testDoor`) was never asked: it stayed plain-stacked a stride and a half from the frame, inside `backblastM`, and the charge Sam blew dazzled his own
   *  man. `haltBack`: the halt asks it too — the same step back by the same hands (stepBacks, makeRoom: 'rear security · <door>', the order still
   *  'stack') when a teammate's charge order is live on that face of that door, or a teammate's charge stands set on it; a man with no floor to step back
   *  to keeps his mark and nothing is said of it. Only the halted men are moved, and nobody at a door with no charge. Not asked for a charge Sam
   *  set by his own hand that no pair has been stood off from: a stacked pair is not stepped back for that one either. */
  haltBack: false,
  /** SAM IS ONE OF OURS BEYOND THE DOOR. Off the maintainer's own play (the floodlit yard, 10-13, 6:47.9): the squad held the guard room's door from
   *  outside, he walked into that room, gave the word — and their stun can came in and blinded him. `behind()` counted the squad of four's other pair
   *  beyond a door and never him, though the can that goes in first (canFirst) and a frag (fragBehind) both did. `behindSam` (false = as it was): wherever
   *  the squad's own can is decided — the pockets of a held door on the word, 'bang, then go in', a bang and clear's row — Sam beyond that door, inside
   *  the can's reach (`stunM`) with a line from the opening, is what a teammate there is: the row is refused in words ('you are behind that door.'), and
   *  on the word the can stays in the kit ('no bang · you are in there') and they step in; a bang and clear asks again as the can is about to leave the
   *  hand (he may have gone in since the row was given) and goes in without it, the same beat. Any squad size. A can HE throws, or one he watches go in
   *  from outside through the open door, blinds him as it did (samstun.ts SAMSTUN.ownSeen: not this rule's). */
  behindSam: false,
  /** NO CAN INTO A ROOM OUR OWN MEN HOLD (the maintainer, 10-22: "the second team threw a flashbang when our guys were already on the inside"): a
   *  bang and clear asks about TEAMMATES again as the can is about to leave the hand, as behindSam asks about Sam — a teammate outside this order
   *  (and outside the orders that go on its word or its gate) beyond the door, inside the can's reach with a line from the opening: the can stays
   *  in the kit ('no bang · Blue One is in there'), a man says so, and they step in. Any squad size (the row's own test at the time it is given —
   *  `behind` — asks only a squad of four, and only then). false: the row is asked when it is given and never again, as it was. Off here; the
   *  page raises it (`?tossmate=0` off) */
  tossMate: false,
  /** A DOOR CHARGE ORDER IS CARRIED OUT OR ANSWERED (ON in the page — `?chargeanswer=0` — off here and on the frozen harness stand, `TPS_CHARGEANSWER=1`;
   *  a FROZEN_HOLDS row). Off the mission cards' dry run of the two hostage rescues (11-04): a charge ordered from the mission's start, a defender came
   *  through that door and locked it behind him with the pair twenty metres off, the order ended there on "door's open.", and the pair walked up to
   *  the shut door and stood at it — one reading 'setting the charge' — until the captor's clock ran out. With it: (1) the order asks about the leaf
   *  what the player is shown of it (`knownOpen`: the fog's own record of the door — where an eye of the team's last had it): a leaf that swings and
   *  shuts again out of the team's sight is nobody's news, the charge goes on the door they find; one the picture has open ends the order with the
   *  setter saying so, and the row is refused on the same question. (2) a charge order over short of its charge (`short`: the door open, never made
   *  the door, the bag empty at the lock, the charge off a door that came open) leaves every man still on it HOLDING on that door — the order book's
   *  own 'hold', gun on the door, on the stand-off mark he was dealt, or where he stands when the mark is across the open doorway from him (openHand):
   *  nobody keeps a charge order's assignment with no charge order behind it, and the setter of any squad is off the lock (the four's own case for
   *  that is this one). (3) a charge that comes off the door under the men stood off from it is SAID by the setter; the panel's line alone told it.
   *  false = as it was. */
  chargeAnswer: false,
  /** NO POINT OF AN ENTRY IN ANOTHER DOOR'S FRAME. The barracks (rebuild/missions/faults.md B2): the gallery entry by n_end dealt one of its points on the
   *  threshold of n4b, a door in the gallery's side wall 1.7 m on — its floor is floor to the nav grid and nothing solid stands between, so the hand of the
   *  door with no room took it; a man was left standing in an uncleared doorway, and entrypoints.ts had no floor on that hand to move him to. `podFrames`
   *  (false = as it was): a candidate point is refused when it stands in another door's opening (between its jambs, within `podFrameM` of its line) or when
   *  the line to it from this door's mouth passes through one (a point of the room BEYOND that door is no point of this room). The hand that then has no
   *  floor is dealt down the middle, as a room with a wall to that hand always was; entrypoints.ts does the rest. Authored door plans are not asked. */
  podFrames: false, podFrameM: 0.55,
  /** NO CAN FROM A HAND THAT IS NOT AT THE DOOR (`thrHand`; false = the thrower as he was picked, wherever he stands). A threshold's can leaves the hand in
   *  the doorway (handAt) whoever throws it, and the stage's pick asks no reach of the far-jamb man or of the lead: with the order's men run off the door
   *  (a frag landed by the stack: frag.ts fragFlee, which no staged order reads) the stage's clocks ran on, the lead was made the thrower from 6.6 m down
   *  the wall, the can left a doorway nobody stood at — and he was then the first man in, which the stage's own rule forbids while there is another (the
   *  depot's a_s, seed 2, as the page ships). On: the picked man throws while he is not the lead and is AT THE DOOR (thrAt: within `throwReach` of the
   *  frame, on the squad's face); otherwise ONE test (thrPick's `hand` form) says whose hand may throw now — the pick's own order of men, each at the
   *  door, and the lead only with no other man of the order still on his way back. A different man: the can is his, after his own `thrTurn`. Nobody: the
   *  can WAITS — the lead says so, the pocket step stays where it is, asked again every `thrHandEvery`, nothing else of the order moves.
   *  `thrHandWait` s is the ORDER's whole wait (the time between asks and every hand-over turn counted, over all its cans): once it is spent the stage is AS IT WAS
   *  for the rest of the order (the picked man throws from where he is — the old fault and all), so the rule can neither hang an order nor send it in
   *  without the can it would have had. A captor's last warning — before or during the wait — waits for nobody (lastStack / lastHold / rescue.lastWarning: as it
   *  was). With every man at the door or on his mark it changes nothing. THE COST: a lead who is the picked thrower waits out what is left of the order's
   *  wait for a man who is off the door and off his mark and never comes (rooted in a fight, stunned). NOT ASKED of a door that shares a word with another
   *  (the other doors of the word wait on this one's burst, and a can held here would stand their men on their jambs past their own flash — as it was
   *  there, fault and all; one door let go by the word IS asked). A hand whose own end of the doorway has no pocket for the can is no hand for it. The word's key during the wait is what it
   *  is during the pockets (not the word again). What an order should DO when a frag lands on its men — and so who is first over the line when they step
   *  in still walking back — is not decided here. */
  thrHand: false, thrHandWait: 6, thrHandEvery: 0.25,
  /** TWO ANSWERS TO A GRENADE AT THE STACK — the maintainer said yes to both, 10-16: ON in the page (?thrnocan=0, ?thrfraghold=0), false here and on the frozen
   *  stand (TPS_THRNOCAN=1, TPS_THRFRAGHOLD=1); nothing changes while both are false.
   *  `thrNoCan` (acts only under `thrHand`): the order's wait for a hand at the door is spent and still nobody is there — the can STAYS IN THE KIT, said why
   *  ('no bang · nobody at the door'), and the stage goes on as it does with no pocket worth a can (false = thrown as it was, from wherever the picked man
   *  stands). Once the wait is spent a later can of the order goes only from the picked man's own hand at the door, with no second wait.
   *  `thrFragHold`: THE ORDER WAITS FOR MEN WHO RAN FROM A GRENADE. While a man of the order who ran from one (frag.ts fragRunning) is not back at the
   *  door or on his mark (thrBack: a man down or given another order is no longer waited for), the hold does not count the door quiet (its own cap ends
   *  it as it always did), and the pockets give neither a can nor the step in ('the door waits', said by a man who did not run) — whoever sent them
   *  there: the hold's own clock, its cap, the word, Sam's 'bang, then step in'. The wait at the pockets is the ORDER's one wait (`thrHandWait` s in
   *  all, the can's own wait for a hand counted in the same sum; time in the hold is not counted): once it is spent the stage goes on as it was ('on
   *  without …'); back in time, the can's thrower is picked afresh and has his own turn. NOT held: Sam's plain 'step in', a captor's last warning, a
   *  door that shares a word with another (as under thrHand), and the step in behind a can of the order's own that has just gone off (a grenade landing
   *  while the can is in the air or its burst running is not read: they go behind their flash). The hold already waited for its LEAD to be back on his
   *  slice (he is `done` only there): this asks the same of every man. */
  thrNoCan: false, thrFragHold: false,
  /** `thrDoors` and `otherDoorM`, THE DOOR'S OWN LEAF: a man left on another door of the room is not put where that door's leaf, stood open, lies across
   *  his line to it while there is a post where it does not, and a man is not taken as holding it from where he stands if it would. The posts were
   *  tried with the leaf as it stood, which is shut: on the hinge side the door a man comes in by swings round between him and the opening and he is
   *  holding a door he cannot see (the depot's store, the man left on `c_n` 3.5 to 3.9 m down its own wall). Asked of the leaf from where a pushed
   *  leaf comes to rest, near square, to its stop, and only of one-way doors (a double-acting leaf can stand either way). It is a preference, never a
   *  refusal: a post the leaf would hide the door from costs metres of walk beside one it would not (LEAF_COST, more the more of that sweep hides
   *  it — where a door stands in a corner and every post is on its hinge side, the deep one that sees past a leaf stood square wins), and of the
   *  men who could cover a door from where they stand (`otherDoorM`) the one it would hide it from least is taken first. A door is never left
   *  unheld for it. Off = the posts as they were */
  thrDoorsLeaf: false,
  /** A GRENADE SCATTERS THE MEN OF A DOOR ORDER: THE ORDER WAITS FOR THEM (the maintainer's answer, 10-16: 'pause until they're back at the door, then go').
   *  `fragHold` is what `thrFragHold` is to a held doorway, for EVERY door order while its door is still to be opened: with a man of the order who ran from a
   *  grenade (frag.ts fragRunning) not back on his mark (fragBack; a man down or given another order is not waited for) the stack is not called, the leaf is
   *  not opened (the shove, the boot, the pick, the tool on the lock), a charge's man who ran is not given up on, 'stood off' is not said — and a word, or one
   *  signal for several doors, holds its first sound for all its doors ('the door waits', said by a man who did not run). The stage's own caps do not run
   *  meanwhile. `fragHoldCap` s of that, in all, is the order's whole wait: after it the order goes on as it was ('on without …'), so a man who cannot get
   *  back does not freeze the squad — and Sam's word given AGAIN while it is held sends them at once ('the word again'). NOT held: anything once the leaf has moved or the signal has gone (the can, the wait behind a burst, each man's turn
   *  through the frame), Sam's own trigger on a set charge, a stack still waiting for Sam's word (who ran is kept: the wait begins when the word comes), a captor's last warning, the response team's own door orders (Sam's squad only). A threshold's hold and its pockets are `thrFragHold`'s. false = the
   *  order as it was (the module's and the frozen stand's; ON in the page, ?fraghold=0 puts both off; TPS_FRAGHOLD=1). */
  fragHold: false, fragHoldCap: 8,
  /** A KNOWN THREAT FIRST. The maintainer, 10-11, on a replay (the compound's shed: a man inside shot the jamb man, the word came, and the first man through
   *  walked to his point looking 95° off where the fire had come from): "the point man here saw his buddy go down, knows roughly where the enemy is in the
   *  room and looks the wrong way going in". `threatFirst` (false = the entry as it was, run for run): while a door is worked the order REMEMBERS, for
   *  `threatSecs`, where its own men's senses put an enemy in the room beyond it — a man one of them had in sight (where he was last seen), or the muzzle a
   *  round came from that hit or passed within `threatNearM` of one of them, the man it dropped included where a man of the order still up saw him go down; never a place nobody living saw or was shot at from, and a man there counts as down only once one of them sees him down. As
   *  the men go in, the FIRST man's gun is on the known place nearest the doorway from the stack on, the second man's on the next (with one place known: on
   *  the same one, until the first man is through with a line to it); the others take their corners as ever. The first man goes on to his own corners — their
   *  clock starting then — once the man there is down, he has had his muzzle on the place with a line to it for `cornerSecs` and nobody the room fight took
   *  it for, or `threatCap` seconds after he crossed (the seconds the room fight had his gun do not count). Where he looks, never when he shoots. A man HELD UP
   *  short of the frame (`threatStuck` seconds without a stride, the room fight's seconds not counted: a leaf a defender stands behind) lets the place go and
   *  is the entry's as it was — with his gun kept on a place deep in the room he never looked round the leaf at the man stopping it (0 = kept until old news). */
  threatFirst: true, threatSecs: 12, threatCap: 1.5, threatNearM: 1.5, threatStuck: 1,
  /** THE FAR GUN IS SET BEFORE THE DOOR MOVES (`thrSet`; false = both jamb men step out as the leaf swings, as it was). The far-jamb man — the still gun —
   *  steps onto his diagonal with the door still shut, his gun up on where the opening will be, as the stack's last step: 'stacked' is called, and the
   *  lead opens it, only once he stands there (or `thrSetSecs` after he was sent, whichever is first) — the gun that covers the opening is planted and
   *  pointed when the room first sees it, not walking into its own light. He is no longer the pair's rear security from that step on. A door standing
   *  open already, one that wants the boot, and an order given ON A WORD (its men wait on the wall with their rear watched) are worked as before.
   *  SHIPS OFF: over the same 448 it moved nothing (283 with it, 284 without; men lost at the frame 33 either way — the door's duel is lost to the
   *  light, not to the step) and half a second later at the door cost the night breach's scripted last hunt four men */
  thrSetSecs: 1.5,
  /** THE HOLD AT THE DOOR IS DARK UNDER GOGGLES (`thrDark`; false = `thrLight` decides for every man, as it was): a jamb man who wears goggles burns no
   *  light for the hold — a lit lens in a doorway is seen at once by every man it points at (roomfight.ts lampGlare), where the tube shows him the room by
   *  its own light — unless what he looks at is black even to the tube (nvg.ts), when the still gun's light comes on after all. A man without goggles,
   *  and any man with somebody found (the light rule), is lit as `thrLight` and the rule have it. MEASURED neutral (283 of 448 with it and `thrTube`,
   *  284 without; men lost at the frame 30 against 33): on because a quiet door worked on goggles is the team's own light rule, not for the count */
  thrDark: true,
  /** A CAN GOES IN BEFORE ANYBODY SHOWS IN THE OPENING (`thrFirst`: 0 = never — both jamb men step onto their diagonals as the leaf swings, as it was ·
   *  1 = on a HOT floor: a shot, a charge or a boot in the ears of a man of the order inside ROOMFIGHT.teamHot · 2 = on every door). A room that has heard
   *  the floor go loud has its guns on its doors: the man who steps into the opening is seen by a man already settled on it, and that duel is a coin's — it
   *  was where the stage lost its men. So on such a door the leaf is opened from the jamb and the two guns stay on their marks behind the wall; the
   *  first of the stage's cans (one of `thrBangs`, not one more) goes where its burst has a line to the most floor; the blind burst it draws is let run
   *  dry on a bang's own count (`afterBang` · `quiet` · `entryCap`); and only then do they step out onto the diagonals and work the room from the door —
   *  for `thrFirstMin` s at least (the hold's `thrMin` is for a room that can still see; this one cannot for FLASH.secs — 1 s measured worse than 2: more lost
   *  stepping in than saved at the door). No can in the kit, one of ours (a teammate, or Sam) beyond the door inside its reach, an order on a word or held
   *  until told (the cans are the player's to call there), or a gate: the stage as it was. A can that went off on the team's side earns a plain hold. `thrFirstStale`: a hold
   *  that runs on past this many seconds from that can ends with its flash worn off the far men — the floor it lit and no muzzle has been across since is
   *  a blind pocket again, and gets the stage's next can before anybody steps in (0 = floor banged once stays banged: they walked in on men who could see).
   *  `thrFirstIn`: that first can is cooked short — it goes off this far inside the frame line on its way to the pocket it was thrown at, not at the pocket
   *  (0): thrown on the shove (`tossAfterOpen`) it bursts under half a second after the leaf moves, where the flight to a pocket three metres in took most of one.
   *  `thrFirstOnly`: a can that went in first is the door's ONLY can — behind it they hold `thrFirstMin`, turn in to the jambs and step in (off = the stage's
   *  own count, as the rule first went in: floor the first can left dark got the second of `thrBangs` before anybody crossed. Measured over 96 seeds the two
   *  lose and win the same within the scatter; the one can saves about 25 of a block's 445, cans the next door has a use for — and with it on `thrFirstStale`
   *  has nothing to do. Stepping straight in off the diagonals, with no turn in to the jambs since no throw is coming, was measured too: no better on 48 seeds,
   *  19 teammates of 576 worse on the next 48 — the turn in stays). Sam's word and a told 'bang' are his to ask a can with: they get the stage's count
   *  whatever this says */
  thrFirst: 1, thrFirstMin: 2, thrFirstStale: 0, thrFirstIn: 0, thrFirstOnly: true,
  /** THE CAPTOR'S LAST WARNING IS HEARD AT THE DOOR (`thrLast`; false = the stage as it was: every man waited for, the hold held). A threshold order on a
   *  hostage room whose captor is at his last step (rescue.ts lastWarning: 'about to' — the moment the HUD says so; his "last chance! i mean it!" —, his gun
   *  on one of them, or one already shot) stops being a deliberate entry and becomes the emergency one: (1) THE STACK DOES NOT WAIT for a man still more
   *  than `thrLastM` from his mark once a jamb man is on his — the door goes with who is there (handed to that man if it was the lead still walking), and a
   *  late man whose mark is the OTHER jamb is given the lead's wall, a place behind him, where there is floor (so he does not walk across a door that is
   *  about to open); (2) THE HOLD IS NOT HELD — ONE can goes in at once, whatever the pockets say (as on Sam's 'bang and step in'), and they step in behind
   *  it; with a can already burst in there inside `thrLastFresh` s (`thrFirst`'s, on a hot floor) they step in behind THAT one and throw nothing. A first can that went off on the team's side (`Threshold.wide`) is not that one: the last-warning can still goes in, `thrFirstOnly` or not. Not for an order on a word, on a gate with other doors, or given to hold: those are Sam's to time. The cue is the scenario's step, the same one the
   *  HUD words for Sam — no sound is propagated for it (a shout through the door they stand at). TRACED (registry rescue, 48 seeds, the scripted commander):
   *  every hostage lost from the threshold was shot by the captor's own clock while the team stood at the door — the walk-up of a man 12 m back on his
   *  cover (5.2 s of stack) and the 4 s hold put the can at 35.8 s where the shortest clocks run out at 30.1 to 33.0 s */
  thrLast: true, thrLastM: 2, thrLastFresh: 3,
};
const flat = v3.distXZ, yawTo = v3.yawTo;

export type DoorToolKind = 'openClear' | 'bangClear' | 'charge' | 'outside' | 'threshold';
/** the page of the command menu the tools are on (orders.ts OrderDef.page), and the id of the row that opens it */
export const TOOLS_PAGE = 'tools';
/** the big slot's own row on a door's tools (kit.ts BIGSLOT.shut): its id, and the key it is on — the page's digits are taken */
export const BIG_ORDER = 'bigClear', BIG_KEY = 'KeyH';
/** … the page behind the tools' row 4 — the room worked from outside: the threshold's two orders, and 'clear from outside' — and the page that stands in for
 *  the tools on a door whose threshold is held: what to tell the men on its jambs (each is its opening row's id too) */
export const THR_PAGE = 'thr', HELD_PAGE = 'held';
type Phase = 'stack' | 'ready' | 'kick' | 'toss' | 'fuse' | 'wait' | 'enter' | 'place' | 'standoff' | 'armed' | 'slice' | 'held' | 'thr' | 'pocket';
/** One word of Sam's (go()) and the orders it let go together. `go1`: when they were all on their doors and the loud ones began (NaN: not yet);
 *  it is dropped from `words` once they have all crossed. */
export interface Word { t0: number; go1: number; /** DOORTOOLS.fragHold: the game time this word was last held for a door whose men ran from a grenade (unset: not held) */ fledT?: number }
type Post = StackPost;
/** One order of this module's on one door, from the menu's release to the men standing in the room (or holding outside it). */
export interface DoorOp {
  kind: DoorToolKind; door: Door; side: 1 | -1;
  /** given by code (order(): a scenario's own entry), not off the menu */
  coded?: boolean;
  men: Guard[]; lead: Guard;
  /** DOORTOOLS.fragHold: the men of the order who ran from a grenade and are not back on their marks · the seconds the order has waited for such men, in all
   *  (`fragHoldCap`) · the game time that was last counted (unset: not waiting) · 'the door waits' said and not answered yet · the order's signal has gone (a
   *  word's first sound, a gate's): nothing of it waits from here */
  fled?: Set<Guard>; fledW?: number; fledT?: number; fledSaid?: boolean; fledGo?: boolean;
  /** clear from outside by three or more: the men who do not work the opening — they watch the squad's back from the end of the file */
  rear: Set<Guard>;
  /** doorway.ts DOORMANNERS.stackCover: a man of the stack is covering a door standing open onto it (`door`) before he comes up to his mark — `cover`: the
   *  assignment that put him there (somebody else's since: he is theirs) · `back`: the stack's own for him, given him again when he comes up · `at`: when he got there */
  lane?: { gd: Guard; door: Door; cover: Assignment; back: Pick<Assignment, 'post' | 'face' | 'going' | 'there'> & Partial<Assignment>; at?: number } | null;
  /** where each man was when the order came (the stack's last man watches the way he came: orders.ts spareRear) */
  from: Map<Guard, Vec3>;
  phase: Phase; t0: number;
  /** bang and clear: the canister this order took from the count at the word and has not thrown yet (back in the bag if it ends first) */
  bangHeld: boolean;
  /** a bang and clear whose can is a FRAGMENTATION GRENADE (the 'frag and clear' row): every rule of the bang and clear, but the thing thrown is frag.ts's item on
   *  its own fuse, thrown deeper — and it spares nobody: the stack stays off the opening until it has gone */
  frag?: boolean;
  /** `fragWay` / `bangWay`: game time the thrower first found the doorway shut to his grenade (he waits `fragWayWait` from it) · the leaf's angle and the
   *  time at the last look (its real turn is read off the two) */
  fragWayT?: number; leafA?: number; leafT?: number;
  /** the assignment this module last handed each man (his being on another = he was re-ordered: he is not ours) · his mark in the stack / the stand-off */
  mine: Map<Guard, Assignment>; post: Map<Guard, Post>;
  /** the boot: where from, and when it was thrown (NaN: not yet) — or, `pick` (a SILENT squad: silent.ts), the lock worked from there instead: `at` the last
   *  frame his hands were on it, `by` the man whose hands are (his anim.lockpick is ours to clear), `doneT` when it gave (he is on his way back to his jamb) — or,
   *  `gun` (a man of the stack carries a breaching shotgun: shotgun.ts), the lock shot off from there: `by` him, `at` the lock, `n` shells gone into it, `nextT` his
   *  action is worked; `t` then is when the shell that took it went */
  kick: { pos: Vec3; yaw: number; t: number; pick?: boolean; hand?: boolean; gate?: boolean; onIt?: boolean; at?: number; by?: Guard; doneT?: number; offT?: number; gun?: { by: Guard; at: Vec3; n: number; nextT: number; silent: boolean; own?: boolean; /** it is the big slot's hammer he takes the lock with, not a shotgun (bigkit.ts) */ hammer?: boolean; /** the door was NOT locked against them when he was sent (the big slot's own row, BIGSLOT.shut): it is his while it is shut, and one blow or one shell takes its latch */ shut?: boolean } } | null;
  /** the breaching shotgun (shotgun.ts) has been tried on this door's lock and given up on: the boot */
  noGun?: boolean;
  /** given by the big slot's own row ('hammer and clear' / 'shotgun and clear' — BIGSLOT.shut, bigkit.ts): the door is opened WITH the tool whether it is
   *  locked or not, from beside the frame, and no hand tries it first */
  big?: boolean;
  /** when the leaf was opened (NaN: not yet; −Infinity: it stood open already) · the canister · when it left the hand / landed / went off, and where */
  openT: number; can: Item | null; tossT: number; landT: number; bangT: number; bangAt: Vec3 | null;
  /** the men still to be sent in, and when · the charge's clock at the door */
  queue: { gd: Guard; at: number; pod: { pos: Vec3; face: number } }[]; setT: number;
  /** (workstow.ts stowFight) since when the fight has had the setter off his charge, this stretch of it */
  stowOffT?: number;
  /** each man's point of domination once he has been sent in, and when he got onto it */
  pod: Map<Guard, { pos: Vec3; face: number }>; onPod: Map<Guard, number>;
  /** clear from outside: each man's stations, the one he is on, and since when he has held it with nobody in his sight */
  slice: Map<Guard, { pts: Vec3[]; looks: Vec3[][]; i: number; j: number; t: number; ahead: Guard | null; pie?: { eyes: Vec3[]; lean: number; /** thrPieOne: the side of the one lean that ends his cuts (0: none), and whether he has begun it */ end: number; endOn: boolean; knee: boolean[]; fin: boolean; begun: boolean; at: number; /** toward the middle of the opening from his jamb, a unit; the times the gun has come round on this cut, and when it last did */ inw: Vec3; gun: number; gunAt: number; /** he has stood on this cut · the cut he was stepping to when a man showed and stopped him (he goes on to it once nobody is in his sight) */ on: boolean; to: Vec3 | null } }>;
  /** who crossed the frame line into the far side, and when (the first is 'the first man through') */
  crossed: Map<Guard, number>;
  /** the entry: each man's corners to check, the one he is on, and since when his muzzle has been on it (NaN: not yet) · the room's floor being looked over
   *  (undefined: not asked for yet; null: no room model here, or the search is switched off) */
  corner: Map<Guard, { pts: Vec3[]; i: number; on: number; fight?: number; last?: number; turned?: boolean }>; search: RoomSearch | null | undefined;
  /** the man picked to go in first (leadSeen: picked when the can leaves a hand, or as they go) — undefined: not picked yet · the man who threw this order's can */
  first?: Guard; threw?: Guard;
  /** DOORTOOLS.bangInto: where this bang and clear's can was thrown to (it goes off as it gets there) and where it left the hand reaching round the jamb · the pass from the thrower's own hand to that point, while it is in the air */
  aim?: { at: Vec3; from: Vec3 }; pass?: { to: Vec3; v0: Vec3; T: number };
  beats: { t: number; what: string }[];
  /** given 'on my word': stacked, it waits ('ready') · the word that let it go, while that still has it in step with the others · where it stands with that
   *  word: 0 not there yet, 1 on its door with nothing begun, 2 a step from crossing */
  onWord: boolean; word: Word | null; hold: 0 | 1 | 2;
  /** when its men were all on their marks (NaN: not yet) · the gate it goes through with other doors' orders (null: on its own, the moment it is stacked) */
  readyT: number; gate: DoorGate | null;
  /** worked from the threshold (kind 'threshold'): the stage's own state, from the moment the leaf moves · given to HOLD the jambs until told what next */
  thr?: Threshold; thrHold?: boolean;
  /** the far-jamb man sent onto his diagonal ahead of the door (DOORTOOLS.thrSet), and when */
  thrPre?: { far: Guard; t: number };
  /** let go by a word (its `word` is cleared when it begins: it goes in on its own count) — the cans on such a door stay the player's to call */
  worded?: boolean;
  /** DOORTOOLS.thrLast: the stack went without waiting for its last man (said once) · the hold was not held (said once) · the cans thrown by then */
  lastStack?: boolean; lastHold?: boolean; lastTossed?: number;
  /** DOORTOOLS.otherDoorM: the room's other doors that a man of this entry has his gun on — who, and the point it is held on */
  other?: Map<Door, { gd: Guard; w: Vec3 }>;
  /** DOORTOOLS.threatFirst: what this order's men know of an enemy beyond the door (`known`), the event clock read up to, the room beyond (undefined: not asked
   *  yet), and — dealt as they go in — whose gun is on which known place (`deal`: undefined until the entry's first frame) */
  threat?: { known: KnownThreat[]; evT: number; room?: { cells: Set<number> } | null; roomT?: number; deal?: Map<Guard, ThreatWatch> };
}
/** a place the men of a door order know an enemy to be (or to have fired from) beyond their door: where, when they last knew it, the man (down = dealt with) */
export interface KnownThreat { at: Vec3; t: number; by: Guard | null; how: 'seen' | 'shot' }
/** a man going in with his gun on a known place: `lead` — null: the place is HIS to deal with; a man: he only covers it until that man has a line to it ·
 *  since when his muzzle has been on it with a line (NaN: not) · the seconds the room fight had his gun · when he was let go of it (NaN: not yet) */
export interface ThreatWatch { th: KnownThreat; lead: Guard | null; on: number; fight: number; last: number; doneT: number; faced?: boolean; /** short of the frame: where he last made a stride, and the seconds since that the room fight did not have his gun (`threatStuck`) */ at?: Vec3; still?: number }
/** A room being worked from the threshold. `jamb`: the men at the frame — [0] the lead on his own jamb's diagonal (`thrSlice`: slicing it), [1] the man on the far jamb's (absent: one jamb only has
 *  floor); everybody else of the order is `DoorOp.rear` — the squad's back — until he is wanted. `pods`: the shallow points they step in to. `quietT`: when a
 *  man at the door last had somebody in his sight, or a defender last fired near it (`evT`: the event clock read up to). `up`: their men standing when the
 *  leaf moved, and `down` how many of them have dropped in the room or its doorway since, before anybody went in. `thrower`, the cans thrown, when the next
 *  may go, where the one in the air is to burst (null: a bang and clear's own spot) and the hand it left; `banged`: the floor a burst has had a line to.
 *  `hold`: the jambs are held until Sam says what next (`told`: the door has gone quiet and the lead has said so); `force`: told to bang — a can goes in
 *  while the kit has one, blind pocket or not. */
export interface Threshold {
  jamb: Guard[]; pods: { pos: Vec3; face: number }[];
  quietT: number; evT: number; contact: Set<Guard>; up: Guard[]; down: number; sliced: boolean;
  thrower: Guard | null; tossed: number; nextT: number; aim: Vec3 | null; from: Vec3 | null; banged: Uint8Array | null;
  handed: boolean; losT: number; litC: Int8Array | null; tubeC?: Int8Array; hold: boolean; told: boolean; force: boolean;
  /** DOORTOOLS.thrHand: the seconds this order's cans have waited for a hand at the door, in all (`thrHandWait` is the order's whole wait), and the game time of the last ask of a can waiting now (unset: none is) */
  handW?: number; handT?: number;
  /** DOORTOOLS.thrHand: the order's wait is spent — its cans go as they always did from here on */
  handDone?: boolean;
  /** DOORTOOLS.thrFragHold: the men of the order who ran from a grenade and are not back yet · the game time its wait at the pockets was last counted (unset: not waiting) · 'the door waits' said, and not answered yet ('the men are back', or 'on without …') */
  fled?: Set<Guard>; fragT?: number; fragSaid?: boolean;
  /** … the word's step in (no can) is OWED: the word came while men who ran were out — it goes when they are back, the wait is spent, or the word comes again · when it became owed */
  stepOwed?: boolean; owedT?: number;
  /** DOORTOOLS.wordCuts: the owed step in is waiting for the jamb men's cuts */
  cutOwed?: boolean;
  /** DOORTOOLS.thrFirst: 'can' — the first can is still to go in, nobody on his diagonal yet · 'burst' — it has gone off, the burst it drew is running dry ·
   *  'done' — they stepped out behind it (the hold's own minimum is `thrFirstMin`) · null: the stage as it was. `out`: the step onto the diagonals, kept for then */
  first?: 'can' | 'burst' | 'done' | null; out?: (() => void) | null;
  /** … that can went off on the team's side of the door · the word came while it was in the air (taken as they step out) · Sam's word cut the hold short (the
   *  pockets are his to ask a can for: `thrFirstOnly` does not hold one back) */
  wide?: boolean; early?: boolean; asked?: boolean;
  /** the row that sends a held door in was given on the word with no can asked: the word steps them in, no bang (thenOnWord) */
  wordStep?: boolean;
  /** when a can asked for at a held door found the leaf shut again and the door was opened for it (the 'pocket' stage's leaf look, under `bangWay`) */
  shutT?: number;
}
/** A room's floor while an entry looks it over: the cells' centres, which have been looked at, each man's line-of-sight answers from where he stands (cached
 *  until he has moved), the yaw his muzzle is scanning through and when it last moved, how many times a man has been sent on to see dead floor and how much
 *  was unseen when the last was sent. Two orders into ONE room (the four's pairs on a word, the rescue's two doors) look over one floor between them. */
export interface RoomSearch { room: unknown; pts: Vec3[]; looked: Uint8Array; los: Map<Guard, { at: Vec3; ok: Int8Array }>; yaw: Map<Guard, number>; at: Map<Guard, number>; moves: number; leftAtMove: number; left: number; /** flowOn: walks into (or a bound toward) a part nobody had been in · times the rear closed up on the front · the walk was called · nav cell → index in `pts` · seconds its men have been in a fight */ flows?: number; closes?: number; flowSaid?: boolean; late?: boolean; cellAt?: Map<number, number>; fightT?: number; fightAt?: number }
/** Doors that go on ONE SIGNAL (the hostage rescue's F on both doors of its room — rescue.ts): no order on the gate opens its door until every one of them
 *  still under way has its men stacked and `go` is up (false: stacked and holding for the word); then a door that only needs a shove waits for the boot going
 *  into the door beside it, and the leaves move on one frame. An order that ends drops out; each still throws its own can and goes in on its own count. */
export interface DoorGate { ops: DoorOp[]; go: boolean; openT: number }
/** a charge on a door, waiting for Sam's trigger */
export interface SetCharge { door: Door; side: 1 | -1; t: number; by: string }

/** a point in a door's frame: `a` metres along hinge → latch, `b` out from the frame line on face `side` (orders.ts sideOf's sign) */
export function doorPt(d: Door, a: number, b: number, side: 1 | -1, y = 0): Vec3 {
  const th = d.def.closedDir, ux = Math.cos(th), uz = Math.sin(th), H = d.def.hinge;
  return [H[0] + ux * a - uz * side * b, y, H[1] + uz * a + ux * side * b];
}
export const alongOf = (d: Door, p: Vec3) => { const th = d.def.closedDir; return (p[0] - d.def.hinge[0]) * Math.cos(th) + (p[2] - d.def.hinge[1]) * Math.sin(th); };
/** metres off the frame line on face `side` (negative: the other face) */
export const offOf = (d: Door, p: Vec3, side: 1 | -1) => { const th = d.def.closedDir; return (Math.cos(th) * (p[2] - d.def.hinge[1]) - Math.sin(th) * (p[0] - d.def.hinge[0])) * side; };
const OPEN_RAD = 0.6;
export const isOpen = (d: Door) => !d.latched && Math.abs(d.angle) >= OPEN_RAD;
/** is most of a room's floor under the sky (asked once per room, of every cell or about 64 of them: a line straight up from head height that meets nothing) */
const SKY = new SideMap<object, { n: number; open: boolean }>('doortools.SKY');
function underSky(g: Game, room: { cells: Set<number> }): boolean {
  const hit = SKY.get(room); if (hit && hit.n === room.cells.size) return hit.open;
  const nav = g.col.nav, cs = nav.cell, every = Math.max(1, Math.floor(room.cells.size / 64)); let k = 0, of = 0, sky = 0;
  for (const ci of room.cells) { if (k++ % every) continue; const x = (ci % nav.W + 0.5) * cs, z = (((ci / nav.W) | 0) + 0.5) * cs; of++; if (!g.col.solidBlocked([x, 2.0, z], [x, 9, z])) sky++; }
  const open = sky * 2 > of; SKY.set(room, { n: room.cells.size, open }); return open;
}
/** OPEN FLOOR, NOT A ROOM: a flood past `searchCells`, or (DOORTOOLS.openAir) floor under the sky. An entry onto it is the fixed sweep alone. */
export function openFloor(g: Game, room: { cells: Set<number> }): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('openFloor', undefined, [g, room]); if (__r !== RUST_PASS) return __r as any; } } return room.cells.size > DOORTOOLS.searchCells || (DOORTOOLS.openAir && underSky(g, room)); }

export class DoorTools {
  readonly knobs = DOORTOOLS;
  charges = DOORTOOLS.charges; bangs = DOORTOOLS.bangs; /** the squad's fragmentation grenades (frag.ts FRAG.squad) */ frags = FRAG.squad; private fragNext = false; private bigNext = false; private bigRow: boolean | undefined = undefined;
  ops: DoorOp[] = [];
  /** the last few orders that have ended, newest last (the probes and the console read their beats) */
  done: (DoorOp & { ended: string })[] = [];
  set: SetCharge[] = [];
  /** doors whose set charges go on ONE press of the trigger (a scenario's: rescue.ts links its room's two doors; not touched by reset()) */
  linked: Door[] = [];
  /** a scenario's own switch for the weapon lights of men going in (rescue.ts: its 'team weapon lights' row) — null: the night breach's (not touched by reset()) */
  lights: boolean | null = null;
  log: { t: number; what: string }[] = [];
  stats = { orders: 0, opened: 0, kicks: 0, shot: 0, picks: 0, noBoot: 0, handKept: 0, tested: 0, halted: 0, bangs: 0, ownFlashed: 0, frags: 0, blown: 0, knocked: 0, stunned: 0, backblast: 0, entries: 0, words: 0, steppedBack: 0, podsMoved: 0, sectors: 0,
    /** DOORTOOLS.buddy's counts: men sent on to a point by a room's search or up to a far door, in all · of them (the switch OFF) away from everybody BY
     *  THEMSELVES — by the search (the walks the switch takes away), and `doorAlone` to a far door (the walks `buddyDoors` would) · (ON) partners sent with a man · deals where somebody was refused every point he could have had */
    sentOn: 0, sentAlone: 0, doorAlone: 0, sentPair: 0, notSent: 0, /** flowOn: walks INTO a part of a room nobody had been in (each also counted in sentOn / sentPair) */ flowed: 0, /** …and men sent up behind the front so that it could go on */ closedUp: 0, /** …and entries still at work past `searchCap` on the time the rule gives them */ flowLate: 0 };
  private roster: Guard[] = [];
  /** what floor each teammate's own eyes have been over, and when (floorseen.ts) — kept while `leadSeen` is on, or `seenWatch` asks for it (a probe counting
   *  what the rule WOULD have picked with it off); null until first wanted · entries begun, and how many of them the most-seen man led instead of the lead */
  seen: FloorSeen | null = null; seenWatch = false; led = { entries: 0, bySeen: 0 };
  /** the key the word's row was last registered with ('' = on a digit, null = no such row: the pair; undefined = not dealt for this cast yet) */
  private goKey: string | null | undefined = undefined;
  private words: Word[] = [];
  /** when the word was last given on Sam's key (input()) */
  private wordKeyT = -1e9;
  /** the key the word's row on a held threshold's page was dealt (sync(): undefined = not dealt for this cast yet) */
  private thrKey: string | undefined = undefined;
  private thrSig = -1;
  private thrSigNow(): number { const K = DOORTOOLS; return (K.thrHoldFirst ? 1 : 0) | (K.thrBangFirst ? 2 : 0) | (K.thrOutsideRow ? 4 : 0) | (K.outsidePlain ? 8 : 0); }

  constructor(private g: Game) {
    const S = g.orders, K = DOORTOOLS;
    const stackable = (t: OrderTarget, men: Guard[]) => this.stackable(t, men);
    S.register({ id: TOOLS_PAGE, label: 'door tools', targets: ['door'], opens: TOOLS_PAGE, issue: () => { } });   // the row (5) that turns a door's menu to the page of them
    // the two entry rows are worded by what the team KNOWS of the door (DOORTOOLS.testDoor: 'kick and clear · wedged from inside' once a man has tried it) and,
    // while the men they go to hold that door from outside (DOORTOOLS.outsidePlain), are the second order that sends them in — with no can, or with one
    const held = (t: OrderTarget) => this.heldPlain(t, S.recipients(t));
    const known = (t: OrderTarget) => t.kind === 'door' && this.entryVerb(t) !== 'open' ? ` · ${this.shutWhy(t.door)}` : '';
    S.register({ id: 'openClear', page: TOOLS_PAGE, label: t => held(t) ? 'go in and clear · no bang' : `${this.entryVerb(t)} and clear${known(t)}`, targets: ['door'], ack: ['on it.', 'moving.'], onWord: true,
      only: (_S, men, t) => this.heldPlain(t, men) ? this.heldMen(men, t) : men,
      valid: (_S, t, men) => this.heldPlain(t, men) ? this.canThresholdThen('step', t, men) : stackable(t, men),
      issue: (_S, men, t, o) => this.heldPlain(t, men) ? (o?.onWord ? this.thenOnWord('step', men, t) : this.thresholdThen('step', men, t)) : this.issue('openClear', men, t, !!o?.onWord) });
    S.register({ id: 'bangClear', page: TOOLS_PAGE, label: t => held(t) ? `bang, then go in · ${this.bangs} left` : `${this.entryVerb(t) === 'open' ? 'bang' : `${this.entryVerb(t)}, bang`} and clear · ${this.bangs} left`, targets: ['door'], ack: ['bang and clear.', 'copy, banging it.'], onWord: true,
      only: (_S, men, t) => this.heldPlain(t, men) ? this.heldMen(men, t) : men,
      valid: (_S, t, men) => { if (this.heldPlain(t, men)) return this.canThresholdThen('bang', t, men);
        if (this.bangs <= 0) return 'no bangs left.';
        const who = t.kind === 'door' ? this.canBehind(t.door, t.side, men) : null; if (who) return who === 'you' ? wg("you're behind that door.", 'you are behind that door.') : `${who} is behind that door.`;   // (the squad of four: nobody bangs a room his own men stand in — nor, behindSam, the one Sam stands in)
        return stackable(t, men); },
      issue: (_S, men, t, o) => this.heldPlain(t, men) ? (o?.onWord ? this.thenOnWord('bang', men, t) : this.thresholdThen('bang', men, t)) : this.issue('bangClear', men, t, !!o?.onWord) });
    S.register({ id: 'charge', page: TOOLS_PAGE, label: t => t.kind === 'door' && this.looseOn(t) ? 'stand off · in behind the charge' : t.kind === 'door' && this.chargeOn(t.door) ? 'blow the charge' : `breach with a charge · ${this.charges} left`, targets: ['door'], ack: ['copy.', 'on it.'],
      gone: (_S, t) => t.kind === 'door' && !!t.door.def.steel && !this.chargeOn(t.door),   // (orders.ts OrderDef.gone: a steel door takes no charge — the flat ring does not offer one)
      valid: (_S, t, men) => {
        if (t.kind !== 'door') return false;
        if (this.looseOn(t)) return this.standoffFits(men, t, true);   // (owncharge.ts: a charge the player set by hand, from its own face: they stand off on it)
        if (this.chargeOn(t.door)) return true;
        if (t.door.def.steel) return "steel. a charge won't cut it.";
        if (DOORTOOLS.chargeAnswer ? this.knownOpen(t.door) : isOpen(t.door)) return "it's open.";
        if (this.charges <= 0) return 'no charges left.';
        if (this.ops.some(o => o.kind === 'charge' && o.door === t.door && o.phase === 'place')) return 'already on it.';
        return this.standoffFits(men, t);
      },
      issue: (_S, men, t) => { if (t.kind !== 'door') return 'no door'; if (this.looseOn(t)) return this.startCharge(men, t, true); const c = this.chargeOn(t.door); if (c) { this.trigger(c); return; } return this.startCharge(men, t); } });
    // THE ROOM WORKED FROM OUTSIDE: row 4 of the tools turns to a page of its own (the one-press entries above keep their digits). The threshold's two
    // orders — given to HOLD first: the nearer press — and 'clear from outside', which had this row
    // (DOORTOOLS.outsidePlain: no page — row 4 IS the one order, 'clear from outside'; thrRows deals whichever of the two the switch asks for)
    // frag and clear (frag.ts): the page's digits are all taken (1‥4, and 5 is the word's with the four), so its row is on a key of its own — G, with this page up —
    // at the foot of the page: the four rows above it keep their places and digits
    S.register({ id: 'fragClear', page: TOOLS_PAGE, hotKey: FRAG.orderKey, pageOnly: true, label: () => `frag and clear · ${this.frags} left`, targets: ['door'], ack: ['frag and clear.', 'copy, fragging it.'], onWord: true,
      valid: (_S, t, men) => { if (!FRAG.on) return 'no frags in this kit.'; if (this.frags <= 0) return 'no frags left.';
        if (g.rescue?.hostages.some(h => h.state !== 'dead')) return 'not with hostages on the floor.';   // (a frag spares nobody: with hostages anywhere on the floor it stays on the belt — said whether or not they are behind THIS door, so the row gives nothing away)
        const mate = t.kind === 'door' ? this.fragBehind(t.door, t.side, men) : null; if (mate) return mate === 'you' ? wg("you're behind that door.", 'you are behind that door.') : `${mate} is behind that door.`;
        return stackable(t, men); },
      issue: (_S, men, t, o) => { this.fragNext = true; try { return this.issue('bangClear', men, t, !!o?.onWord); } finally { this.fragNext = false; } } });
    // A THRESHOLD HELD ON THE DOOR (by men the menu's orders go to): row 5 of the door's own menu is WHAT NEXT, standing in for the tools' row for as long
    // as the jambs are held — step in, bang and step in, leave it (thresholdThen), to the men holding; the word's own row is dealt in sync(), with its key
    S.register({ id: HELD_PAGE, label: 'threshold held', targets: ['door'], opens: HELD_PAGE, standsIn: { for: TOOLS_PAGE, when: (_S, t, men) => { if (DOORTOOLS.outsidePlain) return false; const ops = this.heldOn(t, men); return ops.length > 0 && (!DOORTOOLS.thrHeldAll || men.every(m => m.state === 'dead' || ops.some(op => op.men.includes(m)))); } }, issue: () => { } });
    this.thrRows();
  }

  /** The rows of the threshold's page and of a held threshold's, registered in the order the menu's switches give them (DOORTOOLS.thrHoldFirst / thrBangFirst /
   *  thrOutsideRow — a page's digits are dealt from 1 as its rows are registered): at the start, and again from sync() whenever one of the three has moved. */
  private thrRows() {
    const S = this.g.orders, K = DOORTOOLS; this.thrSig = this.thrSigNow();
    for (const id of ['thresholdHold', 'threshold', 'outside', 'thrStep', 'thrBang', 'thrLeave', THR_PAGE]) S.unregister(id);
    if (K.outsidePlain) {
      // ONE ORDER, NO PAGE (the maintainer, 10-11: "it should just clear externally w/o the additional orders of what to do after … one level of external
      // clearance and then you have to give a second order for them to go in"): row 4 of the door's tools holds the door from outside until told; going in
      // is this page's own rows 1 and 2 while they hold it (the constructor's rows), F is still the word, and 'fall in' / 'stack up' take them off it
      S.register({ id: 'thresholdHold', page: TOOLS_PAGE, label: t => this.heldPlain(t, S.recipients(t)) ? 'clear from outside · holding it' : 'clear from outside', targets: ['door'], ack: ['on the jambs. your call after.', 'copy, holding it for you.'], ackNow: ['on the jambs. your call.', 'copy, holding it for you.'], onWord: true,
        valid: (_S, t, men) => this.heldPlain(t, men) ? 'already holding it.' : this.canThreshold(t, men), issue: (_S, men, t, o) => this.thresholdHold(men, t, !!o?.onWord) }, 'fragClear');
      this.thrKey = undefined; return;
    }
    S.register({ id: THR_PAGE, page: TOOLS_PAGE, label: 'work it from the threshold', targets: ['door'], opens: THR_PAGE, issue: () => { } }, 'fragClear');
    const hold = () => S.register({ id: 'thresholdHold', page: THR_PAGE, label: 'hold the door until I say', targets: ['door'], ack: ['on the jambs. your call after.', 'copy, holding it for you.'], ackNow: ['on the jambs. your call.', 'copy, holding it for you.'], onWord: true,
      valid: (_S, t, men) => this.canThreshold(t, men), issue: (_S, men, t, o) => this.thresholdHold(men, t, !!o?.onWord) });
    const step = () => S.register({ id: 'threshold', page: THR_PAGE, label: 'step in once it goes quiet', targets: ['door'], ack: ['on the jambs, then in.', 'copy, working it.'], onWord: true,
      valid: (_S, t, men) => this.canThreshold(t, men), issue: (_S, men, t, o) => this.threshold(men, t, !!o?.onWord) });
    if (K.thrHoldFirst) { hold(); step(); } else { step(); hold(); }
    if (K.thrOutsideRow) S.register({ id: 'outside', page: THR_PAGE, label: 'clear from outside', targets: ['door'], ack: ['working it from here.', 'slicing it.'], onWord: true,
      valid: (_S, t, men) => this.stackable(t, men), issue: (_S, men, t, o) => this.issue('outside', men, t, !!o?.onWord) });
    const then = (id: string, what: 'step' | 'bang' | 'leave', label: OrderDef['label']) => S.register({ id, page: HELD_PAGE, label, targets: ['door'],
      only: (_S, men, t) => this.heldMen(men, t), valid: (_S, t, men) => this.canThresholdThen(what, t, men), issue: (_S, men, t) => this.thresholdThen(what, men, t) });   // (the lead says it: no second voice)
    const plain = () => then('thrStep', 'step', K.thrBangFirst ? 'step in · no bang' : 'step in'), bang = () => then('thrBang', 'bang', () => `bang, then step in · ${this.bangs} left`);
    if (K.thrBangFirst) { bang(); plain(); } else { plain(); bang(); }
    then('thrLeave', 'leave', () => DOORTOOLS.thrLeaveFallIn ? 'leave it · fall in' : 'leave it');
    this.thrKey = undefined;   // (the word's own row is dealt again behind them: sync)
  }

  // ------------------------------------------------------------ counts
  /** spend one (false: none left) */
  take(what: 'charge' | 'bang' | 'frag'): boolean {
    this.sync();
    if (what === 'frag') { if (this.frags <= 0) return false; this.frags--; this.note(`frag spent · ${this.frags} left`); return true; }
    if (what === 'charge') { if (this.charges <= 0) return false; this.charges--; } else { if (this.bangs <= 0) return false; this.bangs--; }
    this.note(`${what} spent · ${this.charges} charges ${this.bangs} bangs left`); return true;
  }
  /** one back in the bag (an order that reserved it and never used it; a charge off a door that came open) */
  refund(what: 'charge' | 'bang' | 'frag') { if (what === 'frag') { this.frags++; this.note(`frag back · ${this.frags} left`); return; } if (what === 'charge') this.charges++; else this.bangs++; this.note(`${what} back · ${this.charges} charges ${this.bangs} bangs left`); }
  /** a new cast (a staging, a restart) is a fresh run — checked wherever an order or a count can come in, not only once a frame */
  sync() { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('DoorTools.sync', this, []); if (__r !== RUST_PASS) return __r as any; } }
    const g = this.g; let i = 0, same = true; for (const gd of g.guards) if (g.orders.mine(gd) && this.roster[i++] !== gd) same = false;
    if (!same || i !== this.roster.length) { this.roster = g.orders.squad(); this.reset(); this.goKey = this.thrKey = undefined; this.bigRow = undefined; }
    // (A MAN ALONE — owncharge.ts: with no cast the roster never changes and no restart comes through reset(). A fresh Player is his new run, the wall
    // charge's own rule: a charge he left on a door goes with the old run, and what his hands spent is back in the bag. kit.ts kitAlone, where his card is
    // put on him, then writes the card's count over it)
    if (this.samOf !== g.player) { const first = !this.samOf; this.samOf = g.player; if (!first && OWNCHARGE.on && !this.roster.length) { this.set = []; this.charges += this.samSpent; } if (!first) this.samSpent = 0; }
    const goKey = this.roster.length > 2 ? (DOORTOOLS.wordOnKey ? DOORTOOLS.blowKey : '') : null;
    // … and under DOORTOOLS.wordSmall one or two teammates have the word too, for a plain stack only: the same order id, on the word's key, on no menu
    // (their menus are as they were: with no menu up the key gives it — input() below)
    const small = DOORTOOLS.wordSmall && this.roster.length > 0 && this.roster.length <= 2, goSig = small ? 'small' : goKey;
    if (small && goSig !== this.goKey) { this.goKey = goSig;
      g.orders.register({ id: 'go', head: WORD_HEAD, hotKey: DOORTOOLS.blowKey, label: t => { const w = this.stackWords(g.orders.recipients(t)); return w ? `go · ${w}` : 'go'; }, targets: ['floor', 'door', 'none'], listed: [], ack: ['going.', 'moving.'],
        only: (_S, men) => { const on = new Set<Guard>(); for (const x of this.stackWaits(men)) for (const m of x.men) on.add(m); const h = this.fragHeld(men); return men.filter(m => on.has(m) || h.some(o => o.men.includes(m))); },
        valid: (_S, _t, men) => this.stackWaits(men).length > 0 || this.fragHeld(men).length > 0 || 'nobody waiting.', issue: (_S, men) => this.goMen(men, this.stackWaits(men)) ?? undefined }); }
    else if (!small && goKey !== this.goKey) { this.goKey = goKey;   // (a new cast — or the word's key knob flipped: the row is dealt its key when it is registered)
      // THE WORD is an order of its own for the squad of four (the pair alone has no second door to wait on: its menu is as it was)
      // (the word is not about what the menu was opened on: its row sits under a header of its own and names who would take it. Its key is Sam's own — input()
      // below gives it, menu or no menu — so the row shows that key and has no digit; it is not LISTED on a door's menu, where that key is 'wait for my word')
      if (goKey !== null) g.orders.register({ id: 'go', head: WORD_HEAD, hotKey: goKey || undefined, label: t => { const w = this.waitingWords(g.orders.recipients(t)); return w ? `go · ${w}` : 'go'; }, targets: ['floor', 'door', 'none'], listed: ['floor', 'none'], ack: ['going.', 'moving.'],
        only: (_S, men) => { const w = this.waitingMen(men); if (w.length) return w; const h = this.fragHeld(men); return men.filter(m => h.some(o => o.men.includes(m))); },   // (a man with nothing waiting — on the F drill, say — is not this order's: his drill is not asked to let go of him)
        valid: (_S, _t, men) => this.waitingAny(men).length > 0 || this.fragHeld(men).length > 0 || 'nobody waiting.', issue: (_S, men) => { this.goMen(men); } });
      else g.orders.unregister('go'); }
    // … and on the page of a HELD THRESHOLD the word has a row for the pair too — on the key that is its word there (the four's row above is not listed on
    // a door's menu). The pair's: the blind pockets banged, then the step in (pairWord). The four's is its one word — everything of theirs that waits on
    // it goes (goMen), so the row names who, as the 'go' row does. Dealt its key when it is registered: again when the knob moves
    if (this.thrSig !== this.thrSigNow()) { this.thrRows(); this.bigRow = undefined; }   // (a menu switch moved: the pages' rows dealt their digits again — and the big slot's row, which stands after them, put back in its place)
    // THE BIG SLOT'S OWN ROW (kit.ts BIGSLOT.shut, bigkit.ts): while a man of the squad carries the hammer or the breaching shotgun, a door's tools have a
    // row that opens it WITH the tool — locked or not, from beside the frame — and goes in. The page's digits are taken, so it is on a key of its own (H),
    // which the row that opens the page shows and which works from the door's first page too: X, H.
    { const big = BIGSLOT.on && BIGSLOT.shut && this.roster.some(m => carriesHammer(m) || (m.state !== 'dead' && kitBig(m) === 'shotgun'));
      if (big !== this.bigRow) { this.bigRow = big; g.orders.unregister(BIG_ORDER);
        if (big) g.orders.register({ id: BIG_ORDER, page: TOOLS_PAGE, hotKey: BIG_KEY, label: t => `${this.bigWord(t, g.orders.recipients(t))} and clear`, targets: ['door'], ack: ['on it.', 'breaching.'], onWord: true,
          gone: (_S, t) => t.kind === 'door' && (isOpen(t.door) || !!t.door.def.steel),   // (orders.ts OrderDef.gone: nothing to breach, or no way through — the flat ring does not offer it)
          valid: (_S, t, men) => { if (t.kind !== 'door') return false; if (isOpen(t.door)) return "it's open."; if (t.door.def.steel) return 'steel. no way through.';
            if (!men.some(m => carriesHammer(m) || (SHOTGUN.on && m.state !== 'dead' && kitBig(m) === 'shotgun'))) return 'nobody here carries one.';
            if (SILENT.kicks && !SILENT.toolsLoud && squadSilent(g) && !silentUrgent(g)) return wg("not while we're silent.", 'not while we are silent.');   // (with SILENT.toolsLoud the row is one of silent.ts LOUD_ROWS: marked LOUD, and picking it is the squad going loud)
            // (a wedge is nothing the menu reads before a hand has tried the door: only the shotgun to hand and a wedge nobody knows of, the row stands and the lead's hand finds it — haltAt)
            if (t.door.locked && t.door.lockId.startsWith('wedge:') && !men.some(m => carriesHammer(m)) && (!DOORTOOLS.testDoor || this.knownShut(t.door, t.side))) return wg("it's wedged. a shell won't open it.", 'wedged. no shell shifts it.');
            return this.stackable(t, men); },
          issue: (_S, men, t, o) => { this.bigNext = true; try { return this.issue('openClear', men, t, !!o?.onWord); } finally { this.bigNext = false; } } }, 'fragClear'); } }
    const S = g.orders, thrKey = goKey === null ? DOORTOOLS.blowKey : goKey;
    if (thrKey !== this.thrKey) { this.thrKey = thrKey;
      S.register({ id: 'thrWord', page: HELD_PAGE, head: WORD_HEAD, hotKey: thrKey || undefined, targets: ['door'],
        label: t => { if (!S.big) return silentNoCans(g) || this.wordQuiet() ? 'go · step in · no bang' : 'go · bang the blind pockets, step in'; const w = this.waitingWords(S.recipients(t)); return w ? `go · ${w}` : 'go'; },
        only: (_S, men, t) => S.big ? this.waitingMen(men) : this.heldMen(men, t),
        valid: (_S, t, men) => this.heldOn(t, men).some(op => !op.word) || wg("nobody's holding that door.", 'nobody is holding that door.'),
        issue: (_S, men) => (S.big ? this.goMen(men) : this.pairWord(men) ? null : 'nobody\'s waiting for you to say go.') ?? undefined }); }
  }
  /** a fresh run: the counts back, nothing set, nothing under way (a can already in the air goes off on its own short fuse) */
  reset() { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('DoorTools.reset', this, []); if (__r !== RUST_PASS) return __r as any; } }
    for (const op of this.ops) { this.defuse(op); this.offLock(op); }
    this.ops = []; this.done = []; this.set = []; this.words = []; this.held.clear(); this.watches = []; this.tried.clear(); this.samSpent = 0; this.charges = seatShare(this.g, DOORTOOLS.charges); this.bangs = seatShare(this.g, DOORTOOLS.bangs); this.frags = seatShare(this.g, FRAG.squad); this.log = [];
    for (const k of Object.keys(this.stats) as (keyof DoorTools['stats'])[]) this.stats[k] = 0;
    this.seen = null; this.led = { entries: 0, bySeen: 0 };
  }
  /** NOBODY CAME WITH HIM (kit.ts kitAlone, owncharge.ts): the count is his card's — nothing set, nothing spent. Nothing with a teammate on the floor. */
  alone(n: number) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('DoorTools.alone', this, [n]); if (__r !== RUST_PASS) return __r as any; } } this.sync(); if (this.roster.length) return; this.set = []; this.samSpent = 0; this.charges = Math.max(0, Math.floor(n)); }
  /** the player whose run this is, and what his own hands have taken from the bag in it (sync: a man alone gets it back with a fresh run) */
  private samOf: object | null = null;
  samSpent = 0;
  /** a checkpoint's charge back on its door (savestate.ts, a man alone): as he set it, nothing taken from the count the file wrote */
  setBack(d: Door, side: 1 | -1) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('DoorTools.setBack', this, [d, side]); if (__r !== RUST_PASS) return __r as any; } } if (d.off || this.chargeOn(d)) return; this.set.push({ door: d, side, t: this.g.time, by: SAM_BY }); }
  chargeOn(d: Door): SetCharge | null { return this.set.find(c => c.door === d) ?? null; }
  /** can these men stack on that face of that door (the entry orders' menu test: floor at a jamb, a way there for each, not steel and locked) */
  private stackable(t: OrderTarget, men: Guard[]): true | string | false {
    if (t.kind !== 'door') return false; const S = this.g.orders;
    const posts = this.dealStack(t.door, t.side, men)?.posts; if (!posts) return S.noRoomWords('no room on that door.', () => this.dealStack(t.door, t.side, men)?.posts);   // (orders.ts ORDERS.inWay: … or that it is Sam who is in the way, where it is)
    for (const [gd, p] of S.pair(men, posts.map(x => x.pos), posts.map(x => x.group ?? 0), t.door.frameCentre)) { const w = S.reach(gd, p); if (w) return w; }   // (the very deal start() makes)
    if (DOORTOOLS.testDoor && !this.knownShut(t.door, t.side)) return true;   // (nobody has tried it: the row is given, and the man at the handle says what he finds — the menu does not read a lock for him)
    if (t.door.def.steel && !isOpen(t.door) && this.lockedOut(t.door, t.side)) return 'steel, and locked.';
    if (this.frameHeld(t.door, t.side) && !this.pickable(t.door) && !this.bigOpens(t.door, men)) return this.noBoot(t.door);   // (it opens toward them and no pick opens it: every entry order would end at the door — doors.ts DOORWAY.frameHolds)
    return true;
  }
  /** is the door locked against the team from that face (Sam's ring and the codes he knows are the team's — orders.ts route) */
  private lockedOut(d: Door, side: 1 | -1): boolean {
    return this.g.doors.lockedOut(d, doorPt(d, d.def.width / 2, 0.6, side), this.g.orders.ring());
  }
  /** LOCKED AGAINST THE TEAM ON THE FACE IT OPENS TOWARD (doors.ts DOORWAY.frameHolds — bootHolds): shut in its frame, which a boot from here only drives it
   *  onto. Not steel (that has its own answer). The team's ways through are the lock picked — a cylinder (pickable) — or a charge. */
  private frameHeld(d: Door, side: 1 | -1): boolean { return !d.def.steel && !isOpen(d) && this.lockedOut(d, side) && this.g.doors.bootHolds(d, doorPt(d, d.def.width / 2, 0.6, side)); }
  /** …and what a man says to an entry order there when no pick opens it (a keypad's bolt): the charge — the one already on that door, one from the bag, or
   *  (the kit may carry none) neither */
  private noBoot(d: Door): string { return this.chargeOn(d) ? 'opens toward us. no boot. blow the charge.' : this.charges > 0 ? 'opens toward us. no boot. charge it.' : 'opens toward us. no boot, no charge.'; }
  /** open()'s answer at a door whose frame holds a boot: no pick opens it → refused aloud, the order ended as on steel; a cylinder → picked, loud squad or not.
   *  false: neither — a silent squad with its picks switched off, which gets silent.ts's own answer from open() (it never boots, and the pick is not its to use) */
  private frameAnswer(op: DoorOp): boolean {
    const g = this.g, d = op.door, lead = op.lead;
    if (!this.pickable(d)) { this.stats.noBoot++; g.say(lead, this.noBoot(d)); this.end(op, 'opens toward us · no boot'); return true; }
    if (silentNoBoot(g)) return false;
    this.toPick(op); this.beat(op, 'opens toward us: pick'); g.say(lead, 'locked, and it opens toward us. picking it.'); return true;
  }
  /** one of these men carries a big-slot tool that takes this door's lock from either face (kit.ts BIGSLOT, bigkit.ts — never with that switch off): the
   *  hammer, or the breaching shotgun where the lock is one a shell takes (not a wedge). What a door locked on the face it opens toward is NOT refused for — never for a silent squad, which
   *  keeps to the pick. */
  private bigOpens(d: Door, men: Guard[]): boolean { const g = this.g; return BIGSLOT.on && !d.def.steel && !(SILENT.kicks && squadSilent(g) && !silentUrgent(g)) /* (silent.ts silentNoBoot's own question, uncounted: a silent squad's loud tools stay slung) */ && men.some(m => carriesHammer(m) || (SHOTGUN.on && m.state !== 'dead' && kitBig(m) === 'shotgun' && !d.lockId.startsWith('wedge:'))); }
  /** the tool the big slot's row names for these men on that door: the shotgun where one of them carries it and a shell takes this door (not a known
   *  wedge), else the hammer — toGun's own order */
  private bigWord(t: OrderTarget, men: Guard[]): 'shotgun' | 'hammer' {
    const wedged = t.kind === 'door' && t.door.locked && t.door.lockId.startsWith('wedge:') && (!DOORTOOLS.testDoor || this.knownShut(t.door, t.side));
    return SHOTGUN.on && !wedged && men.some(m => m.state !== 'dead' && kitBig(m) === 'shotgun') ? 'shotgun' : 'hammer'; }
  /** where the lead boots a door from: beside its latch jamb where the floor allows and no pane stands behind him (DOORWAY.sideKick, kickGlass — he is out of the opening as the leaf goes), else
   *  square to the leaf by the lock, as it always was */
  private bootSpot(d: Door, side: 1 | -1): { pos: Vec3; yaw: number } {
    const g = this.g, sq = g.doors.kickSpot(d, doorPt(d, d.def.width - 0.25, 1, side));
    if (!DOORWAY.sideKick) return sq;
    const s = g.doors.kickSpotSide(d, sq.pos);
    if (g.col.nav.isBlocked(s.pos[0], s.pos[2]) || g.col.solidBlocked([sq.pos[0], 0.5, sq.pos[2]], [s.pos[0], 0.5, s.pos[2]])) return sq;
    if (!DOORWAY.kickGlass || GLASSPEN.stackEyes <= 0 || g.col.nav.isBlocked(sq.pos[0], sq.pos[2])) return s;
    // a pane beside the latch jamb is no wall to kick from behind: from the front, as it was — where clearly fewer of the lines through glass reach him there
    // (two spots a few lines apart are the same spot, as for the stack's hands; asked afresh, so the answer is the place's and not an earlier asker's a hand's breadth off)
    const n = g.orders.glassEyes(d, side, s.pos, false);
    return n >= GLASSPEN.stackEyes && g.orders.glassEyes(d, side, sq.pos, false) <= GLASSPEN.stackLeastK * n ? sq : s;
  }
  // ------------------------------------------------------------ what the team knows of a door (DOORTOOLS.testDoor)
  /** shut doors the team KNOWS are locked against it — one of its men tried the handle, or the player stood at its lock's rows — and the ones tried and found
   *  free (so nobody says it twice). The roster's: a fresh cast knows nothing (reset). */
  private tried = new Map<Door, number>();   // (PER FACE: bits 1 / 2 = known shut from side +1 / −1, 4 / 8 = tried and found free from there — a keyed door is locked from one face only, and the player's prompts at one face say nothing of the other)
  private knows(d: Door, side: 1 | -1, free: boolean): boolean { return ((this.tried.get(d) ?? 0) & ((side > 0 ? 1 : 2) << (free ? 2 : 0))) !== 0; }
  private learn(d: Door, side: 1 | -1, free: boolean | null) { const b = side > 0 ? 1 : 2, was = (this.tried.get(d) ?? 0) & ~(b | b << 2); this.tried.set(d, free === null ? was : was | b << (free ? 2 : 0)); }
  /** why a hand does not open it, in a man's words */
  private shutWhy(d: Door): string { return d.def.steel ? 'steel, and locked' : this.pickable(d) ? 'locked' : d.lockId.startsWith('wedge:') ? 'wedged from inside' : 'keypad lock, no code'; }
  /** does the team know this door is locked against it on that face (a door since opened, unlocked or broken is no longer that door) */
  knownShut(d: Door, side: 1 | -1): boolean {
    if (!this.knows(d, side, false)) return false;
    if (isOpen(d) || d.off || !this.lockedOut(d, side)) { this.learn(d, side, null); return false; }
    return true;
  }
  /** ORDERS.walkTries asks it of a door a walk would be refused for: nobody of the team has tried that face of it, and (testDoor) a door is learnt at its
   *  handle — false with testDoor off, where what the team knows is not kept and the walk is refused as it was */
  untried(d: Door, side: 1 | -1): boolean { return DOORTOOLS.testDoor && !this.knownShut(d, side); }
  /** THE HANDLE TRIED by a man at the jamb: the team knows from here on, and (`say`) he says what he found — once a door for as long as it stays so. A locked
   *  one works in its frame (a handle's small noise, the player's own 'try the handle'; a silent squad's hand is lighter). true: locked against the team. */
  private test(d: Door, side: 1 | -1, gd: Guard, say: boolean): boolean {
    const g = this.g; if (isOpen(d) || d.off) return false;
    const out = this.lockedOut(d, side);
    if (this.knows(d, side, !out)) return out;
    this.learn(d, side, !out); if (out) g.doors.rattle(d, gd.char.id, squadSilent(g) ? 0.06 : 0.12);
    this.stats.tested++; this.note(`${gd.callsign} tried ${d.def.name}: ${out ? this.shutWhy(d) : 'not locked'}`);
    if (say) g.say(gd, out ? `${this.shutWhy(d)}.` : "it's not locked.");
    return out;
  }
  /** THE ORDER HALTS AT A DOOR THAT TURNS OUT SHUT (DOORTOOLS.testDoor): the stack is made, the lead has tried the handle, the team did not know — and what
   *  would come next is the boot or the shotgun, the whole floor told and the men in the frame. He says what he found, the order is over, its men stay
   *  stacked on their marks: the same row again is the order that boots it. true = halted (the op has ended). Not at a door the team knew, one a quiet
   *  hand opens (a silent squad's own answers; a lock picked because the frame holds a boot), an order given by code, a captor's
   *  last warning. */
  private haltAt(op: DoorOp): boolean {
    const g = this.g, d = op.door; if (!DOORTOOLS.testDoor || op.coded || isOpen(d)) return false;
    const knew = this.knownShut(d, op.side), out = this.test(d, op.side, op.lead, false);
    if (!out || knew || this.lastCall(op)) return false;
    if ((squadSilent(g) && SILENT.kicks) || this.frameHeld(d, op.side)) return false;
    const why = this.shutSaid(op.lead, d); this.beat(op, `tried the door · ${why} · halted`);
    this.end(op, 'found shut');
    this.plainStack(op, op.men.filter(m => m.state !== 'dead' && op.post.has(m) && g.orders.of(m) === op.mine.get(m)), ` · ${why}`);
    g.orders.show(op.men);   // (autopilot.ts AUTOPILOT.replan: their own change of plan is drawn for a moment — nothing with the switch off)
    if (DOORTOOLS.haltBack) this.haltBack(op.door, op.side, op.men);
    return true;
  }
  /** … what the man who tried it says of a door found shut, and the count of it: the halt's one line (here, and for a walk that came to the door) */
  private shutSaid(lead: Guard, d: Door): string { const why = this.shutWhy(d); this.stats.halted++; this.g.say(lead, `${why}. holding on it — your call.`); return why; }
  /** … on a door the OTHER pair is charging (DOORTOOLS.haltBack): the men just left stacked on it are inside that charge's backblast, and the step back
   *  a stacked pair is asked for when a charge is ordered (startCharge) was asked before they came. Asked again here, of the same makers, for the halted
   *  men and nobody else, clear of what that order keeps clear — its stand-off marks — so they go to the marks a pair stacked beforehand would have been
   *  sent to, as the door's rear security. Clear, too, of every other mark a teammate has on this door: an entry's stack can put its last man further
   *  down the wall than the makers count as on the door (holders: he is out of the blast and is not asked, as he never was), and his mate is not sent
   *  to stand on him. A teammate's charge left set with its order gone asks it as well. A man with no floor to step back to keeps his mark, his mate
   *  with floor goes; nothing is said by this. */
  private haltBack(d: Door, side: 1 | -1, halted: Guard[]) {
    const S = this.g.orders, ch = this.ops.find(o => o.kind === 'charge' && o.door === d && o.side === side);
    if (!ch && !this.set.some(c => c.door === d && c.side === side && c.by !== SAM_BY)) return;
    const rest = S.men().filter(m => !halted.includes(m)), hold = this.holders(d, side, rest), taken = ch ? [...ch.post.values()].map(P => P.pos) : [];
    for (const m of S.men()) { const A = S.of(m); if (!hold.includes(m) && !ch?.men.includes(m) && A && A.door === d && A.post) taken.push(A.post); }
    const back = this.stepBacks(d, side, rest, taken); if (back) this.makeRoom(d, back);
  }
  /** an order over with its men left a plain stack on the marks they came to the door by — what 'stack up' leaves, for whatever is said next */
  private plainStack(op: DoorOp, left: Guard[], note = '') { this.stackLeft(op.door, op.post, left, note); }
  /** … the same for men with a mark each on that door, whoever dealt it (a door order's own; a walk's — walkTried) */
  private stackLeft(d: Door, post: Map<Guard, Pick<StackPost, 'pos' | 'face' | 'lead' | 'stackSide'>>, left: Guard[], note = '') {
    const S = this.g.orders, name = doorWords(d);
    for (const m of left) { const P = post.get(m)!;
      S.assign(m, { order: 'stack', post: P.pos, face: P.face, watch: [d.frameCentre[0], 1.05, d.frameCentre[2]], door: d, shy: d, stance: P.lead ? 'stack' : 'highReady', upper: 'none', move: { stance: 'none', upper: 'aim' }, stackSide: P.stackSide, going: `stacking · ${name}`, there: `stacked · ${name}${note}` }); }
    S.spareRear(left, d, null);
  }
  /** how an entry row on that door begins, by what the team knows of it: opened by hand, booted, picked */
  private entryVerb(t: OrderTarget): 'open' | 'kick' | 'pick' {
    if (!DOORTOOLS.testDoor || t.kind !== 'door' || !this.knownShut(t.door, t.side) || t.door.def.steel) return 'open';
    const pick = this.pickable(t.door);
    if (pick && silentPick(this.g)) return 'pick';
    return this.frameHeld(t.door, t.side) ? (pick ? 'pick' : 'open') : 'kick';
  }
  /** men these rows go to are holding that door from outside (DOORTOOLS.outsidePlain: the page's entry rows are then the order to go in — to the men who
   *  hold it; `thrHeldAll`: only when every one of them does) */
  private heldPlain(t: OrderTarget, men: Guard[]): boolean {
    if (!DOORTOOLS.outsidePlain || t.kind !== 'door') return false; const up = men.filter(m => m.state !== 'dead'); if (!up.length) return false;
    const ops = this.heldOn(t, up); return ops.length > 0 && (!DOORTOOLS.thrHeldAll || up.every(m => ops.some(op => op.men.includes(m))));   // (any of them, as the held page's own rule: the rows then go to the men who hold — `only`)
  }
  private note(what: string) { this.log.push({ t: this.g.time, what }); if (this.log.length > 300) this.log.splice(0, 100); }
  private beat(op: DoorOp, what: string) { op.beats.push({ t: this.g.time, what }); this.note(`${op.kind} ${op.door.def.name}: ${what}`); }

  // ------------------------------------------------------------ orders in
  /** hand a man this op's next assignment (and remember it as ours) */
  private assign(op: DoorOp, gd: Guard, a: Pick<Assignment, 'post' | 'face' | 'going' | 'there'> & Partial<Assignment>) {
    const S = this.g.orders; S.assign(gd, { order: op.kind, door: op.door, shy: op.door, ...a }); op.mine.set(gd, S.of(gd)!);   // (shy: no mark of this module's but a point of domination lies through the door itself)
  }
  private newOp(kind: DoorToolKind, d: Door, side: 1 | -1, men: Guard[], phase: Phase): DoorOp {
    for (const o of [...this.ops]) { o.men = o.men.filter(m => !men.includes(m)); if (!o.men.length) this.end(o, 're-ordered'); }   // (the men are this order's now)
    const op: DoorOp = { kind, door: d, side, men: [...men], lead: men[0], rear: new Set(), from: new Map(men.map(m => [m, [m.char.pos[0], 0, m.char.pos[2]] as Vec3])), phase, t0: this.g.time, bangHeld: false, mine: new Map(), post: new Map(), kick: null, openT: NaN, can: null, tossT: NaN, landT: NaN, bangT: NaN, bangAt: null,
      queue: [], setT: NaN, pod: new Map(), onPod: new Map(), slice: new Map(), crossed: new Map(), corner: new Map(), search: undefined, beats: [], onWord: false, word: null, hold: 0, readyT: NaN, gate: null };
    this.ops.push(op); this.stats.orders++; return op;
  }
  /** the menu's release (OrderDef.issue: a reason when it cannot be done) */
  private issue(kind: DoorToolKind, men: Guard[], t: OrderTarget, onWord = false): string | void { const r = this.start(kind, men, t, onWord); return typeof r === 'string' ? r : undefined; }
  /** A fresh gate (DoorGate): hand it to order() for each door that is to go on the one signal. `go` false = they stack and hold until somebody raises it. */
  together(go = true): DoorGate { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('DoorTools.together', this, [go]); if (__r !== RUST_PASS) return __r as any; } } return { ops: [], go, openT: NaN }; }
  /** One of the entry orders given by code instead of off the menu (the hostage rescue's F — rescue.ts): the same order, to exactly these men, on that face of
   *  that door; through `gate` when it is to go with other doors'. Returns the order under way, or why not. Nothing is said and orders.ts logs nothing: the
   *  caller words it. */
  order(kind: 'openClear' | 'bangClear' | 'outside' | 'threshold', men: Guard[], door: Door, side: 1 | -1, gate: DoorGate | null = null, hold = false): DoorOp | string { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('DoorTools.order', this, [kind, men, door, side, gate, hold]); if (__r !== RUST_PASS) return __r as any; } }
    men = men.filter(gd => gd.state !== 'dead'); if (!men.length) return 'nobody to take it';
    const t: OrderTarget = { kind: 'door', at: [door.frameCentre[0], 0, door.frameCentre[2]], door, side };
    const ok = this.stackable(t, men); if (ok !== true) return ok || 'no door';   // (what the menu asks before it offers the row)
    silentTool(this.g, kind === 'bangClear');   // (a bang and clear is loud whoever starts it: a silent squad is loud from here — silent.ts)
    const op = this.start(kind, men, t, false, hold);   // (`hold`: the threshold's own — the jambs held until told what next)
    if (typeof op !== 'string') op.coded = true;   // (a scenario's own entry: never halted at a door it finds shut — DOORTOOLS.testDoor is the menu's)
    if (typeof op !== 'string' && gate) { op.gate = gate; gate.ops.push(op); }
    return op;
  }
  /** WORK THE ROOM FROM THE THRESHOLD, as an order's `issue` (orders.ts OrderDef — the THR_PAGE rows 'threshold' and 'thresholdHold'): these men, on that face of that door;
   *  `onWord`: stacked, it waits for Sam's word before the door is touched (the squad of four). A reason when it cannot be done. `canThreshold` is the same
   *  order's `valid`. TWO ORDERS: `threshold` works the room from outside and then steps in by itself (the door quiet for `thrQuiet`); `thresholdHold` works
   *  it and HOLDS the jambs — whoever comes to the door is still shot — until Sam says what next (thresholdThen, or the word). */
  threshold(men: Guard[], t: OrderTarget, onWord = false, hold = false): string | void { const r = this.start('threshold', men, t, onWord, hold); return typeof r === 'string' ? r : undefined; }
  thresholdHold(men: Guard[], t: OrderTarget, onWord = false): string | void { return this.threshold(men, t, onWord, true); }
  canThreshold(t: OrderTarget, men: Guard[]): true | string | false { return this.stackable(t, men); }
  /** … and by a door's NAME (the console, a probe: `__doortools.thresholdOn('breakroom_w')`): the men `who` names, on the face of it they can walk to (the
   *  pair: Sam's) unless `side` says; `hold`: the order that holds the jambs until told. The order under way, or why not. Nothing is said for Sam and the
   *  order log is not written. */
  thresholdOn(door: string, who: Who = 'all', side?: 1 | -1, hold = false): DoorOp | string {
    this.sync();
    const g = this.g, S = g.orders, d = g.doors.byName(door); if (!d) return `no door called ${door}`;
    const men = S.men(who); if (!men.length) return 'nobody to take it';
    const sam = sideOf(d, g.player.char.pos);
    return this.order('threshold', men, d, side ?? (S.big ? S.sideFor(d, men, sam) : sam), null, hold);
  }
  /** the rooms being worked from the threshold whose men are still outside on the jambs, among these men (and on that door, when one is named): what the
   *  word, or a follow-up, would move */
  thresholdHeld(men: Guard[], door: Door | null = null): DoorOp[] { return this.ops.filter(op => !!op.thr && op.phase === 'thr' && (!door || op.door === door) && op.men.some(m => men.includes(m))); }
  /** … on the door a menu is up on (none off a door), and the men among these who hold it: who the held page's rows go to */
  private heldOn(t: OrderTarget, men: Guard[]): DoorOp[] { return t.kind === 'door' ? this.thresholdHeld(men, t.door).filter(op => !DOORTOOLS.thrHandsOff || !!op.thr!.hold) : []; }   // (thrHandsOff: the form that steps in by itself has no what-next on the menu)
  private heldMen(men: Guard[], t: OrderTarget): Guard[] { const ops = this.heldOn(t, men); return men.filter(m => ops.some(op => op.men.includes(m))); }
  /** WHAT NEXT, to men working a room from the threshold (either order of it, any time the jambs are held): 'step' — step in now, no can; 'bang' — the
   *  blind pockets banged (a can goes in while the kit has one: with no pocket worth it, where a bang and clear's would), then the step in; 'leave' — off the
   *  door: the order over ('stood down'), its men a plain stack on their marks, the door left as it stands. Several doors told to step or bang at once cross
   *  together. An OrderDef's `issue` (`t` a door: only that door's) — `canThresholdThen` its `valid` (no cans, or one of ours behind the door: no bang). A
   *  reason when there is nothing to tell. */
  thresholdThen(what: 'step' | 'bang' | 'leave', men: Guard[], t: OrderTarget | null = null): string | void {
    this.sync();
    const ok = this.canThresholdThen(what, t, men); if (ok !== true) return ok;
    const g = this.g, S = g.orders, ops = this.thresholdHeld(men, t?.kind === 'door' ? t.door : null);
    const w: Word | null = what !== 'leave' && ops.length > 1 ? { t0: g.time, go1: g.time } : null; if (w) this.words.push(w);   // (more than one door told at once: they cross together, behind the last burst — wordTick)
    for (const op of ops) {
      this.beat(op, `told · ${what === 'step' ? 'step in' : what === 'bang' ? 'bang and step in' : 'leave it'}`);
      if (what !== 'leave') { op.word = w; op.hold = 0; }
      const owed = !!op.thr!.stepOwed; op.thr!.stepOwed = false; op.thr!.cutOwed = false;   // (the word's owed step in: his say here is the newer one)
      if (what === 'bang') { op.thr!.force = true; if (op.thr!.first || op.thr!.wide) op.thr!.tossed = 0; this.toPockets(op); }   // (… and a can that went in first, unasked, is not the one he is asking for)
      else if (what === 'step') { if (owed) this.wordStepGo(op); else { this.heldBeat(op); this.toStep(op); } }   // (owed: 'the door waits' is answered — 'on without …')
      else {   // off the door: the order over, its men a plain stack on the marks they came to the door by — what 'stack up' leaves, for whatever is said next
        this.heldBeat(op); g.say(op.lead, 'coming off the door.'); this.end(op, 'stood down');
        const d = op.door, left = op.men.filter(m => m.state !== 'dead' && op.post.has(m) && S.of(m) === op.mine.get(m));
        const fallIn = DOORTOOLS.thrLeaveFallIn ? S.defs.find(x => x.id === 'fallIn') : undefined;
        if (fallIn) { fallIn.issue(S, left, t ?? { kind: 'none', at: [d.frameCentre[0], 0, d.frameCentre[2]] }); continue; }   // (the other reading of 'leave it': back to Sam)
        this.plainStack(op, left);
      }
    }
  }
  /** … given with 'wait for my word' on (the four's menu, a door held from outside): the jambs stay held and THE WORD is then what the row said — 'step' goes
   *  in with no can, where the word alone bangs the pockets first; 'bang' is the word as it always was */
  private thenOnWord(what: 'step' | 'bang', men: Guard[], t: OrderTarget): string | void {
    const ok = this.canThresholdThen(what, t, men); if (ok !== true) return ok;
    for (const op of this.thresholdHeld(men, t.kind === 'door' ? t.door : null)) { op.thr!.wordStep = what === 'step'; if (op.thr!.stepOwed) { op.thr!.stepOwed = false; op.thr!.cutOwed = false; op.thr!.fragT = undefined; op.thr!.fragSaid = false; }   // (a step in the last word owed: this is the newer order, and waits for the word again)
      this.beat(op, `on the word · ${what === 'step' ? 'step in, no bang' : 'bang, then step in'}`); }
  }
  canThresholdThen(what: 'step' | 'bang' | 'leave', t: OrderTarget | null, men: Guard[]): true | string {
    const ops = this.thresholdHeld(men, t?.kind === 'door' ? t.door : null); if (!ops.length) return wg("nobody's holding that door.", 'nobody is holding that door.');
    if (what !== 'bang') return true;
    if (this.bangs <= 0) return 'no bangs left.';
    for (const op of ops) { const who = this.canBehind(op.door, op.side, op.men); if (who) return who === 'you' ? wg("you're behind that door.", 'you are behind that door.') : `${who} is behind that door.`; }   // (the squad of four: nobody bangs a room his own men stand in — nor, behindSam, the one Sam stands in)
    return true;
  }
  /** the step in, from the jambs: at once — or, on a word that has other doors to wait for, with them (wordTick sends an order waiting in 'wait' in) */
  private toStep(op: DoorOp) { if (op.word) { op.bangT = -1e9; op.phase = 'wait'; } else this.enter(op); }
  /** … and from the console or a probe: `__doortools.thresholdTell('bang')` — every held threshold of the men `who` names, or only the door named. Null
   *  when it went; else why not. */
  thresholdTell(what: 'step' | 'bang' | 'leave', door: string | null = null, who: Who = 'all'): string | null {
    const g = this.g, d = door ? g.doors.byName(door) : null; if (door && !d) return `no door called ${door}`;
    return this.thresholdThen(what, g.orders.men(who), d ? { kind: 'door', at: [d.frameCentre[0], 0, d.frameCentre[2]], door: d, side: sideOf(d, g.player.char.pos) } : null) ?? null;
  }
  /** open and clear / bang and clear / clear from outside / the threshold: into the stack first */
  private start(kind: DoorToolKind, men: Guard[], t: OrderTarget, onWord = false, hold = false): string | DoorOp {
    if (t.kind !== 'door') return 'no door';
    this.sync();
    const S = this.g.orders, d = t.door; const deal = this.dealStack(d, t.side, men), posts = deal?.posts; if (!deal || !posts) return S.noRoomWords('no room to stack on that door', () => this.dealStack(d, t.side, men)?.posts);
    const frag = kind === 'bangClear' && this.fragNext;
    if (kind === 'bangClear' && !this.take(frag ? 'frag' : 'bang')) return frag ? 'no frags left' : 'no bangs left';   // (the can is this order's from the word: nobody goes in behind a bang that is not there — back in the bag if the order ends before the throw)
    this.makeRoom(d, deal.back);
    const op = this.newOp(kind, d, t.side, men, 'stack'); op.bangHeld = kind === 'bangClear'; if (frag) op.frag = true; if (this.bigNext && kind === 'openClear') op.big = true; op.onWord = onWord && S.big; op.thrHold = hold && kind === 'threshold'; const name = doorWords(d), watch: Vec3 = [d.frameCentre[0], 1.05, d.frameCentre[2]];
    let lead: Guard | null = null, la = -Infinity, lx = false;
    const oh = DOORTOOLS.openHand && kind === 'bangClear' && stackViewOpen(d);   // (openHand: nobody is walked across the front of a door that stands open)
    for (const [gd, p] of S.pair(men, posts.map(x => x.pos), posts.map(x => x.group ?? 0), d.frameCentre)) { const P = posts.find(x => x.pos === p)!; op.post.set(gd, P);
      this.assign(op, gd, { post: P.pos, face: P.face, watch, stance: P.lead ? 'stack' : 'highReady', upper: 'none', move: { stance: 'none', upper: 'aim' }, stackSide: P.stackSide, going: `stacking · ${name}`, there: `stacked · ${name}` });
      const a = P.lead ? alongOf(d, P.pos) : -Infinity, x = oh && P.lead && this.crossesOpen(d, t.side, gd.char.pos, P.pos);
      if (lead && P.lead && x !== lx ? !x : a > la) { la = a; lead = gd; lx = x; } }   // (the lead: the man on the latch-side jamb — the handle and the keep are his; a door standing open has no handle to work: a man who need not cross it first)
    op.lead = lead ?? men[0];
    if (oh) this.leadHand(op, posts, watch);
    const filed = new Set<Guard>();
    if (kind === 'outside' && men.length > 2) {
      // THREE OR MORE working a door from outside: the pair's own way of it for TWO of them — the lead on his jamb and one man (his own pair's, when he is
      // there) behind him on his wall, the two who slice the opening — and the rest on down that wall behind them, backs to the file, watching the way the
      // squad came: nobody stands on the other jamb (the open doorway gives the room's near corner a line onto it), and what comes down the corridor at the
      // noise meets two guns. A man the wall has no floor for keeps the mark he was dealt (stood back a place below, if it is the other jamb's).
      const L = op.post.get(op.lead)!, aL = alongOf(d, L.pos), W = d.def.width, dir = aL > W / 2 ? 1 : -1, mate = (m: Guard) => S.pairOf(m) !== null && S.pairOf(m) === S.pairOf(op.lead) ? 0 : 1;
      const rest = men.filter(m => m !== op.lead).sort((a, b) => mate(a) - mate(b) || flat(op.post.get(a)!.pos, L.pos) - flat(op.post.get(b)!.pos, L.pos)); let k = 0;
      for (const gd of rest) { k++; const rear = k >= 2; if (rear) op.rear.add(gd);
        const P = op.post.get(gd)!, off = offOf(d, L.pos, t.side), floor = (from: Vec3, p: Vec3) => !this.g.col.nav.isBlocked(p[0], p[2]) && !this.g.col.solidBlocked([from[0], 0.45, from[2]], [p[0], 0.45, p[2]]) && stackRoomKeeps(this.g, d, t.side, p);
        let q = doorPt(d, aL + dir * k * DOORTOOLS.bangFile, off, t.side);
        if (!floor(L.pos, q)) { if (!rear) continue;   // (the lead's wall runs out. A rear man: the same on the OTHER wall, a place or more back from its jamb — never on the jamb itself)
          const far = doorPt(d, W - aL, off, t.side); let found: Vec3 | null = null;
          for (let j = 1; j <= 4 && !found; j++) { const c = doorPt(d, W - aL - dir * j * DOORTOOLS.bangFile, off, t.side); if (floor(far, c) && !men.some(m => m !== gd && flat(op.post.get(m)!.pos, c) < 0.7)) found = c; }
          if (!found) continue; q = found; }
        const out = doorPt(d, W / 2 + (k === 2 ? dir : -dir) * 4, 7, t.side, 1.05);   // (the two of them: one out along his own wall's side of the approach, one across it)
        const Q: Post = { ...P, pos: q, face: rear ? yawTo(q, out) : L.face, stackSide: L.stackSide, lead: false }; op.post.set(gd, Q); filed.add(gd);
        this.assign(op, gd, rear ? { post: q, face: Q.face, watch: out, stance: 'none', upper: 'aim', going: `rear security · ${name}`, there: `rear security · ${name}` }
          : { post: q, face: Q.face, watch, stance: 'highReady', upper: 'none', move: { stance: 'none', upper: 'aim' }, stackSide: Q.stackSide, going: `stacking · ${name}`, there: `stacked · ${name}` }); }
    }
    const split = kind === 'openClear' || kind === 'threshold';   // (a man on each jamb, the files behind them: nobody is filed up behind the opener)
    if (!split && men.length > 2) {   // three or more, behind a bang (or a man the file above had no floor for): the lead's own file is already in file behind him; the men on the OTHER jamb stand back a place along their own wall (nobody crosses the door to join the lead's file), out of the angle the open doorway gives the room onto that jamb
      const W = d.def.width, hi = (p: Vec3) => alongOf(d, p) > W / 2, L = op.post.get(op.lead)!;
      for (const gd of [...men].sort((a, b) => flat(op.post.get(b)!.pos, d.frameCentre) - flat(op.post.get(a)!.pos, d.frameCentre))) {   // (the far man first: he makes the room)
        const P = op.post.get(gd)!; if (gd === op.lead || filed.has(gd) || hi(P.pos) === hi(L.pos)) continue;
        const q = doorPt(d, alongOf(d, P.pos) + (hi(P.pos) ? 1 : -1) * DOORTOOLS.bangFile, offOf(d, P.pos, t.side), t.side);
        if (this.g.col.nav.isBlocked(q[0], q[2]) || this.g.col.solidBlocked([P.pos[0], 0.45, P.pos[2]], [q[0], 0.45, q[2]]) || men.some(m => m !== gd && flat(op.post.get(m)!.pos, q) < 0.7) || !stackRoomKeeps(this.g, d, t.side, q)) continue;   // (no floor there — or not this room's: he keeps his place)
        const Q: Post = { ...P, pos: q, lead: false }; op.post.set(gd, Q);
        this.assign(op, gd, { post: q, face: Q.face, watch, stance: 'highReady', upper: 'none', move: { stance: 'none', upper: 'aim' }, stackSide: Q.stackSide, going: `stacking · ${name}`, there: `stacked · ${name}` }); }
    } else if (!split) { let k = 0;   // behind a bang, or working it from outside, only the man who opens (and throws) is at the frame: the rest file up behind him along his wall, out of the angle the open doorway gives the room's far near-corner onto the other jamb
      const L = op.post.get(op.lead)!, aL = alongOf(d, L.pos), dir = aL > d.def.width / 2 ? 1 : -1;
      for (const gd of men) { if (gd === op.lead) continue; k++;
        const P = op.post.get(gd)!, q = doorPt(d, aL + dir * k * DOORTOOLS.bangFile, offOf(d, L.pos, t.side), t.side);
        if (this.g.col.nav.isBlocked(q[0], q[2]) || this.g.col.solidBlocked([L.pos[0], 0.45, L.pos[2]], [q[0], 0.45, q[2]]) || !stackRoomKeeps(this.g, d, t.side, q)) continue;   // (no floor there — or not this room's: he keeps his own jamb)
        const Q: Post = { ...P, pos: q, face: L.face, stackSide: L.stackSide, lead: false }; op.post.set(gd, Q);
        this.assign(op, gd, { post: q, face: Q.face, watch, stance: 'highReady', upper: 'none', move: { stance: 'none', upper: 'aim' }, stackSide: Q.stackSide, going: `stacking · ${name}`, there: `stacked · ${name}` }); } }
    if (oh) this.keepHands(op, posts, watch);
    this.laneCover(op);
    this.beat(op, `ordered · ${men.map(m => m.callsign).join(', ')} · lead ${op.lead.callsign}`);
    return op;
  }
  /** which hand of the opening a point is on, on this face of the door: +1 past the far jamb, -1 before the near one, 0 = on neither (in front of the
   *  opening, on the other face, too far off to matter (stackroom.ts `viewMen`), or with something solid between him and that hand's jamb: a man round
   *  a corner is on no hand of this door) */
  private handOf(d: Door, side: 1 | -1, p: Vec3): 1 | -1 | 0 {
    if (offOf(d, p, side) <= 0 || flat(p, d.frameCentre) > STACKROOM.viewMen) return 0;
    const W = d.def.width, a = alongOf(d, p), h = a >= W + 0.05 ? 1 : a <= -0.05 ? -1 : 0; if (!h) return 0;   // (a jamb man nudged a hand's width toward the opening is still on his hand)
    const j = doorPt(d, h > 0 ? W + 0.35 : -0.35, 0.45, side);
    return flat(p, j) > 0.5 && this.g.col.solidBlocked([p[0], 0.45, p[2]], [j[0], 0.45, j[2]]) ? 0 : h;
  }
  /** a walk from where he stands to that place changes hands in front of the opening */
  private crossesOpen(d: Door, side: 1 | -1, from: Vec3, to: Vec3): boolean {
    const h = this.handOf(d, side, from); return h !== 0 && (alongOf(d, to) > d.def.width / 2 ? 1 : -1) !== h;
  }
  /** DOORTOOLS.openHand: the door stands open. A man of the order whose place is on the other hand of the opening from where he stands takes his own
   *  hand's wall: a place (then two, three) back from that hand's jamb mark, else the jamb mark itself — floor, this room's, clear of every other man's
   *  place. With no jamb mark dealt on his hand, or no floor there, he keeps the place he was dealt */
  private keepHands(op: DoorOp, posts: Post[], watch: Vec3) {
    const g = this.g, d = op.door, side = op.side, W = d.def.width, name = doorWords(d);
    for (const gd of op.men) { const P = op.post.get(gd); if (!P || gd === op.lead || op.rear.has(gd) || !this.crossesOpen(d, side, gd.char.pos, P.pos)) continue;
      const h = this.handOf(d, side, gd.char.pos), J = posts.filter(x => (alongOf(d, x.pos) > W / 2 ? 1 : -1) === h).sort((a, b) => flat(a.pos, d.frameCentre) - flat(b.pos, d.frameCentre))[0]; if (!J) continue;
      const aJ = alongOf(d, J.pos), oJ = offOf(d, J.pos, side);
      const others = [g.orders.samStood, ...g.guards.filter(x => (g.orders.sams ? x.team === 'operative' : g.orders.mine(x)) && x.state !== 'dead' && !op.men.includes(x)).map(x => x.char.pos)];   // (Sam and teammates not on the order: nobody is sent to stand on them)
      const free = (c: Vec3) => !op.men.some(m => m !== gd && flat(op.post.get(m)?.pos ?? m.char.pos, c) < 0.7) && !others.some(p => flat(p, c) < 0.7) && !g.col.nav.isBlocked(c[0], c[2]) && stackRoomKeeps(g, d, side, c);
      let q: Vec3 | null = null;
      for (let j = 1; j <= 3 && !q; j++) { const c = doorPt(d, aJ + h * j * DOORTOOLS.bangFile, oJ, side); if (free(c) && !g.col.solidBlocked([J.pos[0], 0.45, J.pos[2]], [c[0], 0.45, c[2]])) q = c; }
      if (!q && free(J.pos)) q = J.pos; if (!q) continue;
      const Q: Post = { ...J, pos: q, lead: false }; op.post.set(gd, Q); this.stats.handKept++;
      this.assign(op, gd, { post: q, face: Q.face, watch, stance: 'highReady', upper: 'none', move: { stance: 'none', upper: 'aim' }, stackSide: Q.stackSide, going: `stacking · ${name}`, there: `stacked · ${name}` });
      this.beat(op, `hand kept · ${gd.callsign}`); }
  }
  /** openHand, THE LEAD HIMSELF: keepHands leaves the lead where the deal put him, and a deal can leave him no lead mark on his own hand — a man sent
   *  ALONE is dealt the latch jamb, the one place of the face (orders.ts stackPlaces), whichever hand he stands on, and walked across the open doorway to it
   *  (a replay, 10-13: the last man of a pair at a blown storage door, shot in front of it). His place is then his own hand's jamb: a lead mark of the deal
   *  there nobody else has, else the dealt mark mirrored about the doorway's middle (the other jamb's own mark, facing the other way) — floor of this room,
   *  nothing solid between him and it, nobody standing on it. None = the place as dealt, never a refusal */
  private leadHand(op: DoorOp, posts: Post[], watch: Vec3) {
    const g = this.g, d = op.door, side = op.side, gd = op.lead, P = op.post.get(gd), W = d.def.width, name = doorWords(d);
    if (!P || !P.lead || !this.crossesOpen(d, side, gd.char.pos, P.pos)) return;
    const h = this.handOf(d, side, gd.char.pos), at = gd.char.pos;
    const others = [g.orders.samStood, ...g.guards.filter(x => (g.orders.sams ? x.team === 'operative' : g.orders.mine(x)) && x.state !== 'dead' && !op.men.includes(x)).map(x => x.char.pos)];
    const free = (c: Vec3) => !op.men.some(m => m !== gd && flat(op.post.get(m)?.pos ?? m.char.pos, c) < 0.7) && !others.some(p => flat(p, c) < 0.7) && !g.col.nav.isBlocked(c[0], c[2]) && stackRoomKeeps(g, d, side, c)
      && !g.col.solidBlocked([at[0], 0.45, at[2]], [c[0], 0.45, c[2]]);
    const J = posts.find(x => x.lead && x !== P && (alongOf(d, x.pos) > W / 2 ? 1 : -1) === h && free(x.pos));
    const q = J ? J.pos : doorPt(d, W - alongOf(d, P.pos), offOf(d, P.pos, side), side); if (!J && ((alongOf(d, q) > W / 2 ? 1 : -1) !== h || !free(q))) return;
    const Q: Post = J ?? { ...P, pos: q, face: wrapAngle(P.face + Math.PI), stackSide: -P.stackSide as 1 | -1 }; op.post.set(gd, Q); this.stats.handKept++;
    this.assign(op, gd, { post: Q.pos, face: Q.face, watch, stance: 'stack', upper: 'none', move: { stance: 'none', upper: 'aim' }, stackSide: Q.stackSide, going: `stacking · ${name}`, there: `stacked · ${name}` });
    this.beat(op, `hand kept · ${gd.callsign}`);
  }
  /** doorway.ts DOORMANNERS.stackCover: a mark of this stack stands in the lane of ANOTHER door standing open, with floor beyond it the team has not been on
   *  (cleared.ts) — whoever is beyond it looks, and shoots, straight down the stack. The man of the stack furthest from its door who is not the lead (nor a
   *  rear man of a file working it from outside) is sent to cover it first, from the stack's side of it; step() brings him up to his mark (laneIn). No floor
   *  there, no line from there to the opening or to his mark, a place in a doorway or at the ordered door's own opening, the stack nearer that door than the
   *  place would be, or another man's mark on it: he walks to his mark as ever */
  private laneCover(op: DoorOp) {
    const g = this.g, K = DOORMANNERS, d = op.door; if (!K.on || !K.stand || !K.stackCover || op.men.length < 2) return;
    const gd = op.men.filter(m => m !== op.lead && !op.rear.has(m) && op.post.get(m)).sort((a, b) => flat(op.post.get(b)!.pos, d.frameCentre) - flat(op.post.get(a)!.pos, d.frameCentre))[0], A = gd && op.mine.get(gd); if (!gd || !A?.post) return;
    let D: Door | null = null; for (const m of op.men) { const P = op.post.get(m); if (P && (D = doorwayLane(g, P.pos, d))) break; } if (!D) return;
    // the line a man beyond that door shoots the stack down: from the middle of the marks through the middle of its opening. He stands ON it, `coverM` short of
    // the frame, his gun (and his light: `search` — guards.ts LIGHTS) down it into the floor beyond, where whoever has a line onto the stack has to be standing
    let sx = 0, sz = 0, n = 0; for (const m of op.men) { const P = op.post.get(m); if (P) { sx += P.pos[0]; sz += P.pos[2]; n++; } }
    const c = D.frameCentre, l = Math.hypot(sx / n - c[0], sz / n - c[2]); if (l < K.coverM + 0.5) return;
    { const far = g.cleared.roomBy(D, sideOf(D, [sx / n, 0, sz / n]) > 0 ? -1 : 1); if (far && g.cleared.isKnown(far)) return; }   // (the floor beyond it is the team's already: nobody is owed a look)
    const ux = (sx / n - c[0]) / l, uz = (sz / n - c[2]) / l, q: Vec3 = [c[0] + ux * K.coverM, 0, c[2] + uz * K.coverM], w: Vec3 = [c[0] - ux * K.coverDeep, 1.05, c[2] - uz * K.coverDeep];
    if (g.col.nav.isBlocked(q[0], q[2]) || doorwayInWay(g, q) || flat(q, d.frameCentre) < d.def.width / 2 + 1 || g.col.solidBlocked([q[0], 0.45, q[2]], [c[0], 0.45, c[2]]) || g.col.solidBlocked([q[0], 0.45, q[2]], [A.post[0], 0.45, A.post[2]]) || op.men.some(m => m !== gd && flat(op.post.get(m)?.pos ?? m.char.pos, q) < 0.9)) return;
    const back = { post: A.post, face: A.face, watch: A.watch, stance: A.stance, upper: A.upper, move: A.move, stackSide: A.stackSide, going: A.going, there: A.there };
    const words = `covering ${doorWords(D)}`;
    this.assign(op, gd, { post: q, face: yawTo(q, w), watch: w, stance: 'none', upper: 'aim', move: { stance: 'none', upper: 'aim' }, stackSide: A.stackSide, going: words, there: words, search: true, searchT: g.time });   // (search: a searcher's weapon light, where guards.ts LIGHTS gives him one)
    op.lane = { gd, door: D, cover: op.mine.get(gd)!, back }; DOORMANNERS_STATS.covered++; this.beat(op, `lane:${gd.callsign} covers ${D.def.name}`);
  }
  /** breach with a charge: the nearest man to the door goes to it, the rest (and he, after) stand off along the wall */
  private startCharge(men: Guard[], t: OrderTarget, byHand = false): string | void {
    if (t.kind !== 'door') return 'no door';
    this.sync();
    const g = this.g, S = g.orders, d = t.door; const posts = this.standoffPosts(d, t.side, men.length, men); if (!posts) return 'no room to stand off from that door';
    const back = this.stepBacks(d, t.side, men, posts.map(x => x.pos), true); if (back === null) return 'no room to stand off from that door';
    this.makeRoom(d, back);
    // (byHand — owncharge.ts: a charge the PLAYER set stands on it. The order starts at its stand-off stage: the same marks, deal and words, nobody goes to
    // the lock and nothing is taken from the count. blow() sends them in behind it as behind their own — it asks only for a charge order on that door)
    const op = this.newOp('charge', d, t.side, men, byHand ? 'standoff' : 'place'); const name = doorWords(d), watch: Vec3 = [d.frameCentre[0], 1.05, d.frameCentre[2]];
    const deal = this.standoffDeal(men, d, posts); op.lead = deal[0][0];
    const rest = byHand ? men : men.filter(m => m !== op.lead);
    for (const [gd, P] of deal) op.post.set(gd, P);
    if (!byHand) this.toLock(op);
    for (const gd of rest) { const P = op.post.get(gd)!; this.assign(op, gd, { post: P.pos, face: P.face, watch, stance: 'highReady', upper: 'none', move: { stance: 'none', upper: 'aim' }, stackSide: P.stackSide, going: 'standing off', there: `stood off · ${name}` }); }
    this.beat(op, `ordered · ${men.map(m => m.callsign).join(', ')} · ${byHand ? 'stand off on a charge set by hand' : `${op.lead.callsign} sets it`}`);
  }
  // ------------------------------------------------------------ the player's own charge (owncharge.ts)
  /** a charge the PLAYER set by hand stands on this door with no order of the squad's on it, and the menu is on the charge's own face: the charge row is
   *  then 'stand off · in behind the charge' (a teammate's charge keeps its row: 'blow the charge') */
  private looseOn(t: OrderTarget): boolean {
    if (!OWNCHARGE.on || t.kind !== 'door') return false;
    const c = this.chargeOn(t.door);
    return !!c && c.by === SAM_BY && c.side === t.side && !this.ops.some(o => o.kind === 'charge' && o.door === t.door);
  }
  /** can these men stand off from a charge on that face of that door: the marks, a way to each, the other pair's room to step back. `own` = the charge is
   *  already on it, set by the player's hand: then a teammate beyond the door is no reason to stay close to it */
  private standoffFits(men: Guard[], t: OrderTarget, own = false): true | string {
    if (t.kind !== 'door') return 'no door';
    const posts = this.standoffPosts(t.door, t.side, men.length, men); if (!posts) return 'no room to stand off.';
    if (!own) { const mate = this.behind(t.door, t.side, men); if (mate) return `${mate.callsign} is behind that door.`; }
    if (this.stepBacks(t.door, t.side, men, posts.map(x => x.pos), true) === null) return 'no room to stand off.';   // (the other pair, stacked on this door: no floor to get it clear of the blast)
    return this.standoffWalk(men, t.door, posts) ?? true;
  }
  /** can these men walk to these stand-off marks, as the row's why-not asks it: the first why-not, or null (the very dealing the issue makes, for any count: the standoff-fit check holds it) */
  private standoffWalk(men: Guard[], d: Door, posts: Post[]): string | null {
    const S = this.g.orders;
    for (const [gd, P] of this.standoffDeal(men, d, posts)) { const w = S.reach(gd, P.pos); if (w) return w; }   // (the very dealing the issue makes, for any count — two men too: the nearest man takes the first mark, which is not always the least total walk; nor is it with the pairs kept in their files)
    return null;
  }
  /** ownblow.ts: IS THIS SET CHARGE THE SQUAD'S KEY'S to blow — every one with the switch off; with it on, one the squad set, or his own while the squad
   *  works its door (stood off behind it, stacked on it: the key is their word and it goes with them). The key's press, the words and the floor plan ask */
  keyIs(c: SetCharge): boolean { return !OWNBLOW.on || c.by !== SAM_BY || this.ops.some(o => o.door === c.door) || this.stacksOf(this.g.orders.men()).some(x => x.stack === c.door); }
  /** the charge going on under his hands: where on the leaf, and the face's normal (handwork.ts samWork plants him by it; `at` is his to move) */
  get samJob(): { p: Vec3; n: Vec3; at: Vec3 } | null {
    const s = this.g.player.charging; if (!s) return null;
    const d = s.door, o = doorPt(d, 0, 0, s.side), n1 = doorPt(d, 0, 1, s.side);
    return { p: doorPt(d, d.def.width - 0.22, 0.025, s.side, 1.05), n: [n1[0] - o[0], 0, n1[2] - o[2]], at: s.at };
  }
  /** let go of it: nothing was spent (true: there was one) */
  cancelSam(): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('DoorTools.cancelSam', this, []); if (__r !== RUST_PASS) return __r as any; } }
    const pl = this.g.player; if (!pl.charging) return false;
    pl.charging = null; this.g.msg('charge not set'); return true;
  }
  /** who stands off where for a charge: the man nearest the door (he sets it) on the first mark, the rest dealt the others (orders.ts pair) — first entry = the setter */
  private standoffDeal(men: Guard[], d: Door, posts: Post[]): [Guard, Post][] {
    const S = this.g.orders, lead = [...men].sort((a, b) => flat(a.char.pos, d.frameCentre) - flat(b.char.pos, d.frameCentre))[0];
    return [[lead, posts[0]], ...S.pair(men.filter(m => m !== lead), posts.slice(1).map(x => x.pos), posts.slice(1).map(x => x.group ?? 0)).map(([gd, p]) => [gd, posts.find(x => x.pos === p)!] as [Guard, Post])];
  }
  /** the man who sets the charge: to the lock */
  private toLock(op: DoorOp) {
    const d = op.door, ws = this.g.doors.workSpot(d, doorPt(d, d.def.width / 2, 1, op.side), 0.5);
    this.assign(op, op.lead, { post: ws.pos, face: ws.yaw, watch: null, stance: 'none', upper: 'none', going: `charge · ${doorWords(d)}`, there: 'setting the charge' });
  }
  /** a SILENT squad at a door locked against it (silent.ts SILENT.picks): the lead to the keyway, to pick it — the 'kick' stage works the lock from there */
  private toPick(op: DoorOp) {
    const d = op.door, ws = this.g.doors.workSpot(d, doorPt(d, d.def.width / 2, 1, op.side));
    op.kick = { pos: ws.pos, yaw: ws.yaw, t: NaN, pick: true }; op.phase = 'kick'; op.t0 = this.g.time;
    this.assign(op, op.lead, { post: ws.pos, face: ws.yaw, watch: null, stance: 'none', upper: 'none', going: `on the lock · ${doorWords(d)}`, there: 'picking the lock' });
  }
  /** a lock a pick opens: a cylinder — not a steel door's bolt, a keypad's (devices.ts), nor a wedge kicked under the leaf from inside (digin.ts) */
  private pickable(d: Door): boolean { return !d.def.steel && !this.g.devices?.padOf(d) && !d.lockId.startsWith('wedge:'); }
  /** his hands off the lock (whoever's were on it for this op) */
  private offLock(op: DoorOp) { const k = op.kick; if (k?.by) { k.by.char.anim.lockpick = false; k.by = undefined; } }
  /** A door locked against the team and a man of this order with a BREACHING SHOTGUN (shotgun.ts; the lead first, else the nearest man of the stack who has
   *  one): he goes to the lock with it — beside the frame, past the latch edge and off the opening where the wall leaves room (breachSpot), the boot's spot
   *  being square in front of the leaf — and the 'kick' stage shoots it off (gunStep). false: nobody carries one, a wedge under the leaf holds it whatever
   *  becomes of the lock, no spot has floor and a line to the lock, or the gun has been tried and given up on → the boot, as before. */
  private toGun(op: DoorOp): boolean {
    const g = this.g, d = op.door; if (op.noGun) return false;
    const shell = (m: Guard) => SHOTGUN.on && !d.lockId.startsWith('wedge:') && carriesBreacher(m), can = (m: Guard) => shell(m) || carriesHammer(m);   // (bigkit.ts: the hammer in a man's big slot takes a lock the same way, a wedged door too — nobody carries one with BIGSLOT off)
    const near = (a: Guard, b: Guard) => flat(a.char.pos, d.frameCentre) - flat(b.char.pos, d.frameCentre), held = op.men.filter(m => op.post.has(m)).sort(near);
    const by = shell(op.lead) ? op.lead : held.find(shell) ?? (can(op.lead) ? op.lead : held.find(can)); if (!by) return false;   // (a shotgun before a hammer: one shell from beside the frame)
    const hammer = !shell(by);
    const sp = breachSpot(g, d, op.side, [...op.men.filter(m => m !== by).map(m => op.post.get(m)?.pos ?? m.char.pos), g.player.char.pos], hammer ? null : op.post.get(by)?.pos ?? by.char.pos, BIGSLOT.on && BIGSLOT.moves && (hammer || kitBig(by) === 'shotgun') ? (hammer ? 'hammer' : 'shotgun') : undefined); if (!sp) return false;   // (not on a mate's mark, nor on you; his own mark: where he shoots it from when glass has the list's spots — shotgun.ts)
    op.kick = { pos: sp.pos, yaw: sp.yaw, t: NaN, gun: { by, at: sp.at, n: 0, nextT: 0, silent: squadSilent(g), own: sp.own, ...(hammer ? { hammer: true } : {}), ...(this.lockedOut(d, op.side) ? {} : { shut: true }) } }; op.phase = 'kick'; op.t0 = g.time;
    this.assign(op, by, { post: sp.pos, face: sp.yaw, watch: sp.at, stance: 'none', upper: hammer ? 'none' : 'aim', going: `on the lock · ${doorWords(d)}`, there: 'breaching' });
    const lockedNow = this.lockedOut(d, op.side);
    g.say(by, lockedNow ? (hammer ? 'locked. hammer.' : 'locked. shotgun.') : hammer ? 'hammer.' : 'shotgun.'); this.beat(op, `${hammer ? 'hammer' : 'gun'}:${by.callsign}${sp.beside ? '' : ' (in front)'}`); return true;
  }
  /** the stack for this module's entries: orders.ts's marks (stackPosts — the latch jamb first, split when both have floor), each `tuck` further along the wall
   *  from the opening where the floor allows */
  stackPosts(d: Door, side: 1 | -1, men: Guard[], ignore: Guard[] = []): Post[] | null { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('DoorTools.stackPosts', this, [d, side, men, ignore]); if (__r !== RUST_PASS) return __r as any; } }
    return this.g.orders.ownMarkFirst(d, side, men, () => this.stackTucked(d, side, men, ignore));   // (doorway.ts OWN MARK: men who stand stacked — on these tucked marks too — are dealt the marks they stand on, Sam beside them or not)
  }
  private stackTucked(d: Door, side: 1 | -1, men: Guard[], ignore: Guard[]): Post[] | null {
    const g = this.g, posts = g.orders.stackPosts(d, side, men, ignore); if (!posts) return null;
    // (… and the tuck does not take a man off the mark he stands on to put him on Sam: a tucked place within a body of where Sam really stands, that nobody of the order stands on, is not tucked to)
    const sam = g.orders.samStood, own = DOORMANNERS.on && DOORMANNERS.ownMark ? g.orders.ownMarks(d, side, men) : [], mine = (q: Vec3) => own.some(o => flat(o, q) < OWN_EPS);
    const tuckedIn = (P: Post, q: Vec3) => (mine(P.pos) && flat(sam, q) < 0.7 && !mine(q))
      || (DOORMANNERS.on && DOORMANNERS.markClear && !P.lead && doorwayInWay(g, q) && !doorwayInWay(g, P.pos));   // (doorway.ts MARKS: the tuck does not put a mark where a leaf swings)
    if (posts.length > 2) {   // three or more: the files tuck as files — the far man first, each against where the others stand NOW, so a file keeps its gaps
      const out = [...posts], far = (P: Post) => Math.abs(alongOf(d, P.pos) - d.def.width / 2);
      for (const P of [...posts].sort((a, b) => far(b) - far(a))) { const a = alongOf(d, P.pos), dir = a > d.def.width / 2 ? 1 : -1; const q = doorPt(d, a + dir * DOORTOOLS.tuck, offOf(d, P.pos, side), side);
        if (!g.col.nav.isBlocked(q[0], q[2]) && !g.col.solidBlocked([P.pos[0], 0.45, P.pos[2]], [q[0], 0.45, q[2]]) && !out.some(o => o !== P && flat(o.pos, q) < 0.7) && !tuckedIn(P, q) && stackRoomKeeps(g, d, side, q)) out[out.indexOf(P)] = { ...P, pos: q }; }
      return out;
    }
    return posts.map(P => { const a = alongOf(d, P.pos), dir = a > d.def.width / 2 ? 1 : -1; const q = doorPt(d, a + dir * DOORTOOLS.tuck, offOf(d, P.pos, side), side);
      return !g.col.nav.isBlocked(q[0], q[2]) && !g.col.solidBlocked([P.pos[0], 0.45, P.pos[2]], [q[0], 0.45, q[2]]) && !posts.some(o => o !== P && flat(o.pos, q) < 0.7) && !tuckedIn(P, q) && stackRoomKeeps(g, d, side, q) ? { ...P, pos: q } : P; });
  }
  // ------------------------------------------------------------ the squad of four: a door two pairs have a hand in
  /** teammates OUTSIDE this order who are stacked on this door, this face, under a plain 'stack up' (the other pair, holding it) */
  private holders(d: Door, side: 1 | -1, men: Guard[]): Guard[] {
    const S = this.g.orders; if (!S.big) return [];
    // (stackroom.ts: past the makers' 2.6 m only a man waiting DOWN THE DOOR'S WALL out of an open door's view holds it — one stood back on the open floor
    //  is out of the files' way as he always was, and is not asked to step back)
    const held = (p: Vec3) => { const r = flat(p, d.frameCentre), off = offOf(d, p, side); return off > 0 && (r < 2.6 || (r < stackViewReach(d) && off < S.knobs.jambOut + 0.5)); };
    return S.men().filter(gd => { if (men.includes(gd)) return false; const A = S.of(gd); return !!A && A.order === 'stack' && A.door === d && A.post !== null && held(A.post); });
  }
  /** where each of them steps back to, clear of `taken` (the incoming order's own marks): along his own wall first, then the other, `roomBack` past the jamb —
   *  out of the opening's angles, out of a charge's backblast, the door still his to watch. A man with no floor to step back to is left out (he keeps his
   *  mark and the files go past him, as before) — unless `must`: then null (a charge is not set with a man inside its blast). */
  private stepBacks(d: Door, side: 1 | -1, men: Guard[], taken: Vec3[], must = false): Map<Guard, Post> | null {
    const g = this.g, S = g.orders, K = DOORTOOLS, W = d.def.width, out = new Map<Guard, Post>(), off = S.knobs.jambOut, walls = new Set<number>();
    const away = yawTo(doorPt(d, W / 2, 0, side), doorPt(d, W / 2, 1, side)), out2 = stackViewReach(d) > 2.6;   // (straight out from the door, off its wall · the open-door places are live on this door)
    for (const gd of this.holders(d, side, men)) { const from = S.of(gd)!.post!, hi = alongOf(d, from) > W / 2; let found: Post | null = null;
      for (const dir of hi ? [1, -1] : [-1, 1]) { let prev: Vec3 = dir > 0 === hi ? from : doorPt(d, dir > 0 ? W + S.knobs.jambPast : -S.knobs.jambPast, off, side);
        for (const x of K.roomBack) { const q = doorPt(d, dir > 0 ? W + x : -x, off, side);
          if (g.col.nav.isBlocked(q[0], q[2]) || g.col.solidBlocked([prev[0], 0.45, prev[2]], [q[0], 0.45, q[2]]) || !stackRoomKeeps(g, d, side, q)) break;   // (the wall — or the floor, or the room: stackroom.ts — runs out)
          prev = q; if (out2 && flat(q, d.frameCentre) < flat(from, d.frameCentre) - 0.1) continue;   // (stackroom.ts, a stack waiting out of an open door's view down its wall: a step BACK is never a step toward the door, onto a place the room beyond sees — on either wall, nor on the open floor below)
          if ([...taken, ...[...out.values()].map(P => P.pos)].some(o => flat(o, q) < 0.85) || flat(q, g.orders.samStood) < 0.7) continue;
          found = { pos: q, face: walls.has(dir) ? away : yawTo(d.frameCentre, q), stackSide: S.of(gd)!.stackSide, lead: false }; walls.add(dir); break; }   // (his back to the door: the other pair has it — he has the way anybody would come at them: on along the wall; the second man on one wall, out across the floor)
        if (found) break; }
      if (!found) { const r = S.rearPosts(d, side, STACKROOM.on ? 6 : 1, [...taken, ...[...out.values()].map(P => P.pos)]).find(R => stackRoomHas(g, d, side, R.pos) && !(out2 && flat(R.pos, d.frameCentre) < flat(from, d.frameCentre) - 0.1));   // (no wall to step back along: the open floor behind the door, to a hand of it)
        if (r) found = { pos: r.pos, face: yawTo(d.frameCentre, r.pos), stackSide: S.of(gd)!.stackSide, lead: false }; }
      if (found) out.set(gd, found); else if (must && flat(from, d.frameCentre) < K.backblastM + 0.4) return null; }
    return out;
  }
  /** the stack for an order on a door other teammates hold: its own marks as if they had stepped back already, and where they step back to */
  private dealStack(d: Door, side: 1 | -1, men: Guard[]): { posts: Post[]; back: Map<Guard, Post> } | null {
    const hold = this.holders(d, side, men); if (!hold.length) { const posts = this.stackPosts(d, side, men); return posts ? { posts, back: new Map() } : null; }
    let posts = this.stackPosts(d, side, men, hold); if (!posts) { posts = this.stackPosts(d, side, men); return posts ? { posts, back: new Map() } : null; }
    const far = posts.map(P => { const a = alongOf(d, P.pos), W = d.def.width; return doorPt(d, a > W / 2 ? a + DOORTOOLS.bangFile : a - DOORTOOLS.bangFile, offOf(d, P.pos, side), side); });   // (behind a bang the file stands a place further back: kept clear too)
    const back = this.stepBacks(d, side, men, [...posts.map(P => P.pos), ...far])!;
    if (back.size < hold.length) { posts = this.stackPosts(d, side, men, [...back.keys()]); if (!posts) return null; }   // (a man who cannot step back keeps his mark: the files go past him)
    return { posts, back };
  }
  /** … and they do: still on their 'stack up' on this door — the order stands — from a place further back along the wall, as the door's rear security: gun
   *  (and light) up along the wall AWAY from it, on the way anybody answering the noise would come */
  private makeRoom(d: Door, back: Map<Guard, Post>) {
    const g = this.g, S = g.orders, name = doorWords(d);
    for (const [gd, P] of back) { const A = S.of(gd); if (!A) continue; const grp = A.group, p = P.pos;
      const hit = g.col.raycast([p[0], 1.5, p[2]], [Math.sin(P.face), 0, Math.cos(P.face)], 8), r = Math.max(1.5, (hit ? hit.t : 8) - 0.5);
      S.assign(gd, { order: 'stack', post: p, face: P.face, watch: [p[0] + Math.sin(P.face) * r, 1.1, p[2] + Math.cos(P.face) * r], door: d, shy: d, stance: 'none', upper: 'aim', light: NIGHTBREACH.opLights, stackSide: P.stackSide, going: `making room · ${name}`, there: `rear security · ${name}` });
      S.of(gd)!.group = grp; this.stats.steppedBack++; }
    if (back.size) this.note(`made room on ${d.def.name}: ${[...back.keys()].map(m => m.callsign).join(', ')}`);
  }
  /** anybody of the team — a man not on this order, whatever the squad's size, or Sam — on the FAR side of the door with a line to where a frag thrown through it
   *  would land, inside its reach (null: nobody): his callsign, or 'you' */
  private fragBehind(d: Door, side: 1 | -1, men: Guard[]): string | null {
    const g = this.g, at = this.landFor(d, side, FRAG.orderIn), land: Vec3 = [at[0], 0.3, at[2]];   // (where it is really thrown to — the plan's own spot, or as far as the floor runs clear — and the blast's own test of a body, with a metre for the roll)
    const inIt = (p: Vec3) => offOf(d, p, side) < 0 && flat(p, land) < FRAG.woundM + 1 && (exposedTo(g, land, p, [p[0], p[1] + 1.6, p[2]]) > 0 || !g.col.solidBlocked([land[0], 0.9, land[2]], [p[0], 1.1, p[2]]));
    const mate = g.guards.find(gd => gd.team === 'operative' && gd.state !== 'dead' && !men.includes(gd) && inIt(gd.char.pos));
    return mate ? mate.callsign : !g.player.down && inIt(g.player.char.pos) ? 'you' : null;
  }
  /** a teammate outside this order on the FAR side of the door, in the open reach of a charge on it (null: none) — nobody sets one with his own men behind it */
  private behind(d: Door, side: 1 | -1, men: Guard[]): Guard | null {
    const g = this.g, S = g.orders, K = DOORTOOLS; if (!S.big) return null;
    const inn = doorPt(d, d.def.width / 2, 0.45, -side as 1 | -1, 1.1);
    return S.men().find(gd => { const p = gd.char.pos; return !men.includes(gd) && offOf(d, p, side) < 0 && flat(p, d.frameCentre) < K.stunM && !g.col.solidBlocked(inn, [p[0], 1.1, p[2]]); }) ?? null;
  }
  /** is `p` in a stun can's reach through that door: beyond it (its far side, as ever: a man beside a shut door on the squad's own face is not asked
   *  about), inside `stunM`, with a line from the opening. The line is asked without THIS door's own leaf: shoved, or standing ajar, it sweeps through
   *  the point the line starts from — and the can goes in past it. Every other leaf, and the walls, still stand in the way */
  private canReach(d: Door, side: 1 | -1, p: Vec3): boolean {
    const g = this.g; if (!(offOf(d, p, side) < 0 && flat(p, d.frameCentre) < DOORTOOLS.stunM)) return false;
    const inn = doorPt(d, d.def.width / 2, 0.45, -side as 1 | -1, 1.1), col = g.col as unknown as { dynamicBoxes: unknown[] }, keep = col.dynamicBoxes; col.dynamicBoxes = keep.filter(b => b !== d.box);
    try { return !g.col.solidBlocked(inn, [p[0], 1.1, p[2]]); } finally { col.dynamicBoxes = keep; }
  }
  /** DOORTOOLS.behindSam: Sam himself beyond the door, inside a stun can's reach with a line from the opening — the test canFirst has of him, whatever the squad's size */
  private samBehind(d: Door, side: 1 | -1): boolean {
    const g = this.g, K = DOORTOOLS; if (!K.behindSam || g.player.down) return false;
    if ((g.orders as { sams?: boolean }).sams === false) return false;   // (only Sam's own squad counts him as one of ours: `sams` is a getter that arrives with the response team's own cast of orders — undefined here, so nothing changes; a cast that reaches its orders another way must ask that one)
    return this.canReach(d, side, g.player.char.pos);
  }
  /** who of ours the squad's own can through that door would catch (null: nobody): a teammate outside the order — his callsign — or 'you' */
  private canBehind(d: Door, side: 1 | -1, men: Guard[]): string | null {
    const mate = this.behind(d, side, men); return mate ? mate.callsign : this.samBehind(d, side) ? 'you' : null;
  }

  // ------------------------------------------------------------ the word
  /** what is waiting on Sam's word among these men: an order given 'on my word' (stacked or still stacking), a pair on a plain 'stack up', a pair holding
   *  outside a door it has worked ('clear from outside'), a charge set and stood off from */
  private waiting4(men: Guard[]): (DoorOp | { stack: Door; men: Guard[] })[] {
    const S = this.g.orders, out: (DoorOp | { stack: Door; men: Guard[] })[] = [];
    for (const op of this.ops) if (op.men.some(m => men.includes(m)) && !op.word && (op.onWord || op.phase === 'held' || op.phase === 'armed' || op.phase === 'standoff' || op.phase === 'place')) out.push(op);   // (a charge still being set is waited for like a pair still walking to its stack)
    for (const x of this.stacksOf(men)) out.push(x);
    return out;
  }
  /** the men among `men` standing on a plain 'stack up', a door at a time */
  private stacksOf(men: Guard[]): { stack: Door; men: Guard[] }[] {
    const S = this.g.orders, out: { stack: Door; men: Guard[] }[] = [];
    const by = new Map<Door, Guard[][]>();   // per door, the men stacked on each face of it ([0]: Sam's face)
    for (const gd of men) { const A = S.of(gd); if (gd.state !== 'dead' && A && A.order === 'stack' && A.door && A.post && A.there.startsWith('stacked')) {
      const l = by.get(A.door) ?? [[], []]; l[sideOf(A.door, A.post) === sideOf(A.door, this.g.player.char.pos) ? 0 : 1].push(gd); by.set(A.door, l); } }
    for (const [stack, [near, far]] of by) out.push({ stack, men: near.length ? near : far });   // (pairs stacked on BOTH faces of one door: the word takes the men on Sam's face of it — the others keep their stack)
    return out;
  }
  /** THE SMALL SQUAD'S WORD (DOORTOOLS.wordSmall): what of one or two teammates' waits on it — their plain stacks, but for one on a door with a charge
   *  set on it (the key is that charge's trigger first). Nothing with the switch off, and nothing for the squad of four (its word is waitingAny's) */
  stackWaits(men: Guard[]): { stack: Door; men: Guard[] }[] { return DOORTOOLS.wordSmall && !this.g.orders.big ? this.stacksOf(men).filter(x => !this.chargeOn(x.stack)) : []; }
  private stackWords(men: Guard[]): string { const S = this.g.orders; return [...new Set(this.stackWaits(men).map(x => S.whoWords(x.men)))].join(' + '); }
  /** the men among `men` with something waiting on the word */
  private waitingMen(men: Guard[]): Guard[] { const on = new Set<Guard>(); for (const x of this.waitingAny(men)) for (const m of x.men) on.add(m); return men.filter(m => on.has(m)); }
  /** … with the rooms being worked from the threshold: the word sends those in too (the pockets, then the step in) */
  /** THE MEN THE WORD IS FOR: those the orders go to — or, with the squad's only keys (cmdring.ts `only`: there is no who key to turn to them with), whoever
   *  stands waiting on it when none of those does (the maintainer, 10-22: "pressing F only releases the people blocked on it") */
  wordMen(): Guard[] { const S = this.g.orders, men = S.recipients(); if (!CMDRING.only) return men;
    if (S.big ? this.waitingAny(men).length : this.stackWaits(men).length) return men;
    const all = S.men('all'); return (S.big ? this.waitingAny(all).some(x => 'stack' in x || !this.chargeOn(x.door)) : this.stackWaits(all).length > 0) ? all : men; }   // (a set charge with nobody else waiting is not the word's: it has its own line and F blows it, as it did)
  private waitingAny(men: Guard[]): (DoorOp | { stack: Door; men: Guard[] })[] { return [...this.waiting4(men), ...this.thresholdHeld(men).filter(op => !op.word)]; }
  /** what of these men's waits on THAT door, and who (autopilot.ts — the quick key on it): 'stack' = men on a plain 'stack up' there (the word would make an
   *  open-and-clear of it), 'word' = anything else the word would send (an order given on the word, a threshold held, a charge set or being set). Null: nothing */
  waitsOn(men: Guard[], d: Door): { kind: 'stack' | 'word'; men: Guard[]; /** a stack: every man of it stands on his mark */ set: boolean } | null {
    let st: Guard[] | null = null; const S = this.g.orders;
    for (const x of this.waitingAny(men)) { if ('stack' in x) { if (x.stack === d) st ??= x.men; } else if (x.door === d) return { kind: 'word', men: x.men.filter(m => m.state !== 'dead'), set: true }; }
    return st ? { kind: 'stack', men: st, set: st.every(m => { const A = S.of(m); return !!A?.post && (A.arrived || flat(m.char.pos, A.post) < 0.6); }) } : null;
  }
  /** that set charge is on a word that has not gone yet (the key leaves it alone: it goes with the rest) */
  wordPending(c: SetCharge): boolean { return this.onPendingWord(c); }
  /** … in the HUD's words ('red + blue'; '' when nothing of theirs is waiting) */
  private waitingWords(men: Guard[]): string { const S = this.g.orders; return [...new Set(this.waitingAny(men).map(x => S.whoWords(x.men.filter(m => m.state !== 'dead'))))].join(' + '); }
  /** THE WORD. Everything waiting on it among `who` goes TOGETHER: a pair on a plain 'stack up' opens its door and goes in (from the marks it stands on), an
   *  order given 'on my word' is let go, a pair holding outside a door goes in, a set charge is blown — and they cross their thresholds on the same frame:
   *  the loud ones (a bang, a charge) begin at once, the quiet ones do not touch their doors until the last bang's blind burst has run dry (or `wordIn`
   *  seconds). Null when something went; else why not. */
  go(who: Who = 'all'): string | null { return goneWithTick('DoorTools.go') as any; }
  /** the word at this held door is 'step in, no can': told so ('on my word · step in'), or never told what the word is under DOORTOOLS.wordNoCan (a hold
   *  given to be HELD — a plain threshold's own word, which cuts its count short, bangs its pockets as it did; a word told 'bang' stays a bang) */
  /** every door held on the word is one the word steps into with no can (wordNoCan): what the pair's go row says */
  private wordQuiet(): boolean { const K = DOORTOOLS, held = this.ops.filter(o => o.thr); return K.outsidePlain && K.wordNoCan && held.length > 0 && held.every(o => this.wordSteps(o)); }
  /** THE WORD STEPS A HELD DOOR'S MEN IN, NO CAN (wordSteps) — but not past men of the order who ran from a grenade and are not back (thrFragHold): the step
   *  in is OWED, the hold goes on (off the word, if it was on one: it goes in on its own when it goes), and it is paid in the hold's own tick (wordOwed) when
   *  they are back or the order's one wait (`thrHandWait`) is spent. The word AGAIN — outside `wordTap` of the first — pays it at once. Sam's own 'step in'
   *  row is his say there and then, and is not held. */
  private wordStepIn(op: DoorOp) {
    const T = op.thr!, t = this.g.time;
    if (!T.stepOwed && this.cutsOwe(op)) {   // DOORTOOLS.wordCuts: the jamb men are still slicing — the step in is owed to the end of their cuts (asked before the grenade's wait: the cuts first, then wordOwed asks after the men who ran)
      T.stepOwed = true; T.owedT = t; T.cutOwed = true; T.fragT = undefined; if (op.word) { op.word = null; op.worded = true; }
      this.beat(op, 'the word waits for the cuts'); this.g.say(op.lead, 'checking the doorway first.'); return; }
    if (!T.stepOwed && this.thrFragWait(op)) { T.stepOwed = true; T.owedT = t; T.fragT = undefined; if (op.word) { op.word = null; op.worded = true; } return; }
    if (T.stepOwed && t - (T.owedT ?? -1e9) < DOORTOOLS.wordTap) { if (op.word) { op.word = null; op.worded = true; } return; }   // (a second tap of the same word)
    this.wordStepGo(op);
  }
  private wordStepGo(op: DoorOp) {
    const T = op.thr!, K = DOORTOOLS; T.stepOwed = false; T.fragT = undefined; T.cutOwed = false;
    if (T.fragSaid) { T.fragSaid = false; if (T.fled?.size) this.beat(op, `on without ${[...T.fled].map(m => m.callsign).join(', ')} · ${T.handDone || (T.handW ?? 0) >= K.thrHandWait ? "the order's wait is spent" : 'not waited for'}`); }
    this.heldBeat(op); this.toStep(op);
  }
  private wordSteps(op: DoorOp): boolean { const K = DOORTOOLS; return op.thr!.wordStep ?? (K.outsidePlain && K.wordNoCan && !!op.thrHold); }
  /** … and THE PAIR's, which has no word of its own but this: a room it is working from the threshold has its hold cut short there and then — the pockets,
   *  then the step in. True when a hold of these men's took it. (The four's goes with everything else on the word: goMen.) */
  private wordOnHold(men: Guard[]): boolean {
    let any = false;
    for (const op of this.thresholdHeld(men)) { if (op.word) continue; this.beat(op, 'the word'); if (this.wordSteps(op)) this.wordStepIn(op); else { op.thr!.asked = true; this.toPockets(op); } any = true; }
    return this.wordEarly(men) || any;
  }
  /** … as Sam gives it — the key, or its row on a held threshold's page: a set charge goes with it, as with the four's, and the key is stamped (a second tap
   *  inside `wordTap` is not the drill's 'stack up'). False: nothing of these men's was held */
  /** the word while a can that went in first is still in the air or its burst running: nobody is on a diagonal to cut a hold short for yet — it is kept, and
   *  taken the moment they step out (thrOut). True when a door of these men's kept it: the press is not the drill's, nor a trigger's. */
  private wordEarly(men: Guard[]): boolean {
    const ops = this.ops.filter(op => (op.thr?.first === 'can' || op.thr?.first === 'burst') && op.men.some(m => men.includes(m))); if (!ops.length) return false;
    for (const op of ops) op.thr!.early = true; this.wordKeyT = this.g.time; return true;
  }
  /** Would the word's key, pressed now with the menu that is up, be a held threshold's word (the pair's: input()'s own test below, asked and nothing given)?
   *  orders.ts asks before it takes that key for a menu's 'wait for my word' row (planlay.ts puts the row under every menu): the word outranks the row. */
  wordKeyHeld(): boolean {
    const S = this.g.orders; if (S.big || this.g.player.down) return false;
    const own = !!S.menu && this.heldOn(S.menu.target, S.menu.men).length > 0; if (!(DOORTOOLS.thrMenuWord || !S.menu || own)) return false;
    const men = S.recipients(); return this.thresholdHeld(men).some(op => !op.word) || this.ops.some(op => (op.thr?.first === 'can' || op.thr?.first === 'burst') && op.men.some(m => men.includes(m)));
  }
  private pairWord(men: Guard[]): boolean {
    if (!this.wordOnHold(men)) return false;
    this.wordKeyT = this.g.time; for (const c of this.set.filter(c => !this.onPendingWord(c) && this.keyIs(c))) this.trigger(c); return true;   // (ownblow.ts keyIs: not a charge of his own on a door they are not at)
  }
  private goMen(men: Guard[], only?: { stack: Door; men: Guard[] }[]): string | null {
    const g = this.g;
    if (DOORTOOLS.fragHold) {   // (THE WORD AGAIN while it is held for men who ran from a grenade: Sam's call — it waits for nobody)
      const held = this.fragHeld(men);
      if (held.length) { for (const o of held) { this.beat(o, 'the word again'); this.fragOn(o); } this.wordTick(); if (!(only ?? this.waitingAny(men)).length) return null; } }   // (others of these men still waiting for their FIRST word: it is given below)
    const list = only ?? this.waitingAny(men); if (!list.length) return 'nobody\'s waiting for you to say go.';   // (`only`: the small squad's word — its plain stacks, nothing else)
    holdFireOrder(g, 'go');   // (holdfire.ts: the word, by whatever road it came — what waited on it lifted nothing until now)
    const w: Word = { t0: g.time, go1: NaN }; this.words.push(w); this.stats.words++;
    for (const x of list) { const op = 'stack' in x ? this.fromStack(x.men, x.stack) : x; if (!op) continue;
      op.onWord = false; op.word = w; if (op.phase === 'ready' || op.phase === 'held' || op.phase === 'armed' || op.phase === 'thr') op.hold = 1; this.beat(op, 'the word');
      for (const gd of op.men) { const A = op.mine.get(gd); if (A && A.there.startsWith('ready')) A.there = `stacked · ${doorWords(op.door)}`; } }   // (he has it: no longer 'on your word')
    this.note(`go · ${this.ops.filter(o => o.word === w).map(o => `${o.kind} ${o.door.def.name}`).join(' + ')}`);
    this.wordTick();   // (everybody on his door already: it is now)
    return null;
  }
  /** a pair standing on a plain 'stack up' becomes an open-and-clear from the very marks it stands on */
  private fromStack(men: Guard[], d: Door): DoorOp | null {
    const S = this.g.orders, first = S.of(men[0]); if (!first?.post) return null;
    const side = sideOf(d, first.post), name = doorWords(d), W = d.def.width;
    const posts = men.map(gd => { const A = S.of(gd)!; return { pos: v3.copy(A.post!), face: A.face ?? gd.char.bodyYaw, stackSide: A.stackSide, lead: A.stance === 'stack' } as Post; });
    const on = men.map(gd => S.of(gd)!.arrived);
    const op = this.newOp('openClear', d, side, men, 'stack');
    let lead: Guard | null = null, la = -Infinity;
    men.forEach((gd, i) => { const P = posts[i]; op.post.set(gd, P);
      this.assign(op, gd, { post: P.pos, face: P.face, watch: [d.frameCentre[0], 1.05, d.frameCentre[2]], stance: P.lead ? 'stack' : 'highReady', upper: 'none', stackSide: P.stackSide, going: `stacking · ${name}`, there: `stacked · ${name}` });
      if (on[i]) { const A = op.mine.get(gd)!; A.arrived = true; A.arrT = this.g.time; }   // (he is standing on it: the word does not wait a frame for him to arrive again)
      const a = (P.lead ? 10 : 0) - Math.abs(alongOf(d, P.pos) - (W + S.knobs.jambPast)); if (a > la) { la = a; lead = gd; } });   // (the man on the latch jamb; else the nearest it)
    op.lead = lead ?? men[0];
    this.beat(op, `from the stack · ${men.map(m => m.callsign).join(', ')} · lead ${op.lead.callsign}`);
    return op;
  }
  /** once a frame (and on the word itself): the words' two moments */
  private wordTick() {
    const g = this.g, K = DOORTOOLS, t = g.time;
    for (const w of [...this.words]) {
      const ops = this.ops.filter(o => o.word === w); if (!ops.length) { this.words.splice(this.words.indexOf(w), 1); continue; }
      if (isNaN(w.go1)) {   // nobody makes a sound until everybody is on his door (or the late pair has had its time)
        if (K.fragHold && ops.some(o => this.fragWait(o))) { w.t0 += Math.min(t - (w.fledT ?? t), 0.1); w.fledT = t; continue; }   // (men of a door on this word ran from a grenade and are not back: nobody makes a sound yet — `fragHoldCap` a door at most — and the word's own clock for a late door does not run meanwhile)
        if (K.fragHold) w.fledT = undefined;
        if (!ops.every(o => o.hold >= 1) && t - w.t0 < K.wordStack) continue;
        w.go1 = t;
        for (const op of ops) { if (op.hold < 1) { op.word = null; continue; }   // (too late: it carries on alone)
          if (K.fragHold) this.fragOn(op);   // (the signal has gone: nothing of this door waits from here)
          if (op.kind === 'charge') { const c = this.chargeOn(op.door); if (c) this.blow(c); else op.word = null; }   // → 'wait'
          else if (op.phase === 'held') op.hold = 2;
          else if (op.kind === 'threshold' && op.thr) { op.hold = 0; if (op.phase === 'thr') { if (this.wordSteps(op)) this.wordStepIn(op); else { op.thr.asked = true; this.toPockets(op); } } }   // (its men on the jambs already: the pockets now, with the loud ones — and it crosses with the rest, from 'wait')
          else if (op.kind === 'threshold') { op.hold = 0; op.word = null; op.worded = true; this.open(op); }   // (it begins with the loud ones and goes in on its own count: the room is worked for as long as it takes, and nobody else's entry waits on that)
          else if (op.kind === 'bangClear') { op.hold = 0; this.open(op); }
          else if (!isOpen(op.door) && (this.lockedOut(op.door, op.side) || (op.big && BIGSLOT.shut && !op.noGun && this.bigOpens(op.door, op.men)) || toolSpot(this.g, op.door, op.side, op.lead.char.pos, true, true))) { op.hold = 0; this.open(op); }   // (a locked door: its man onto the kick spot now — the boot itself waits; an unlocked one whose lead is not at its handle: onto the handle's place now, handleside.ts — the shove waits)
          else op.hold = 2; }
      }
      const left = this.ops.filter(o => o.word === w);
      if (!left.every(o => o.hold === 2) && (t - w.go1 < K.wordIn || left.some(o => o.frag && o.hold !== 2 && !o.can?.fired))) continue;   // (a frag on this word that has not gone off yet: nobody on the word goes in on it, however long its door takes)
      this.words.splice(this.words.indexOf(w), 1);
      for (const op of left) { op.word = null; if (op.hold !== 2) continue; op.hold = 0;
        if (op.phase === 'ready') this.open(op); else if (op.phase === 'wait' || op.phase === 'held') this.enter(op); }   // (a man on his kick spot: the 'kick' stage sees the word gone and boots it)
    }
  }
  /** on his door, ready: the leaf is opened — the shove, or the boot from the kick spot when it is locked against the team */
  private open(op: DoorOp) {
    const g = this.g, K = DOORTOOLS, d = op.door, t = g.time, lead = op.lead, name = doorWords(d);
    if (isOpen(d)) { this.beat(op, 'open already');
      if (Math.abs(d.angle) < 1.2) { d.closing = false; d.vel = (Math.sign(d.angle) || 1) * Math.max(Math.abs(d.vel), K.shove); d.quietT = -1; d.lastWho = lead.char.id; }   // (a leaf standing half open is pushed the rest of the way: a can thrown at it comes back off it)
      this.opened(op); return; }
    if (this.lockedOut(d, op.side)) {   // locked against the team: the boot, from the kick spot — then back onto his jamb
      if (!(op.big && this.bigOpens(d, op.men)) && this.haltAt(op)) return;   // (DOORTOOLS.testDoor — locked or wedged since the stack was made, and nobody knew; the big slot's own row asked for the tool whatever the lock: nothing to hold on)
      if (d.def.steel) { g.say(lead, "it's steel. no way through."); this.end(op, 'steel and locked'); return; }
      if (silentPick(g) && this.pickable(d)) { this.toPick(op); g.say(lead, 'locked. picking.'); return; }   // (silent.ts: a silent squad's way through a lock — the 'kick' stage, pickStep)
      if (this.frameHeld(d, op.side) && this.bigOpens(d, op.men) && this.toGun(op)) return;   // (bigkit.ts: it opens toward them and a man carries the hammer or the breaching shotgun in his big slot — the lock itself is broken and a hand pulls the leaf; a silent squad keeps to the pick below)
      if (this.frameHeld(d, op.side) && this.frameAnswer(op)) return;   // it opens toward them: the frame is behind the leaf and a boot will not shift it (doors.ts DOORWAY.frameHolds) — a cylinder is picked, loud squad or not; anything else wants a charge, and the order ends here as it does on steel
      const stuck = this.pickable(d) ? null : d.lockId.startsWith('wedge:') ? 'wedged from inside' : 'keypad, no code';   // (no pick opens it — steel has gone above)
      const boot = stuck ? silentBoot(g, d) : null;   // (silent.ts: a COMPROMISED silent squad boots the door nothing quiet opens — and stays silent)
      if (!boot && silentNoBoot(g)) { g.say(lead, stuck && SILENT.bootWedged ? `${stuck}. no quiet way in. go loud and we boot it.` : wg("locked. can't kick it while we're silent.", 'locked. no boot while we are silent.')); this.end(op, 'locked · silent: no boot'); return; }   // (silent.ts: the boot is a loud tool, and nobody picked one)
      if (!boot && this.toGun(op)) return;   // (shotgun.ts: a man of the stack with a breaching shotgun takes the lock from beside the frame — the 'kick' stage, gunStep)
      const ks = this.bootSpot(d, op.side);
      op.kick = { pos: ks.pos, yaw: ks.yaw, t: NaN }; op.phase = 'kick'; op.t0 = t;
      this.assign(op, lead, { post: ks.pos, face: ks.yaw, watch: null, stance: 'none', upper: 'none', going: `on the door · ${name}`, there: 'breaching' });
      if (!boot) g.say(lead, 'locked. kicking it.'); else { this.beat(op, 'silent boot'); if (boot === 'first') g.say(lead, `${stuck}. booting it.`); }
    } else {
      if (op.big && BIGSLOT.shut && this.bigOpens(d, op.men) && this.toGun(op)) return;   // (bigkit.ts: the big slot's own row on a door that is not locked — the latch broken with the tool from beside the frame, the 'kick' stage as on a lock)
      if (!this.toHandle(op, false)) { this.shove(op); this.opened(op); }   // (handleside.ts: a lead who is not at the handle — filed on the hinge jamb, out of reach, in its swing — steps to it first: the 'kick' stage, handStep)
    }
  }
  /** the lead of an unlocked door sent to its handle (handleside.ts toolSpot) — false: he is at it, or it cannot be walked to, and nothing is begun. `gate`
   *  (DOORTOOLS.gateHand): he is sent while the boot beside him on the same signal is still to land, and holds the shove for it (handStep) */
  private toHandle(op: DoorOp, gate: boolean): boolean {
    const g = this.g, d = op.door, lead = op.lead, hs = toolSpot(g, d, op.side, lead.char.pos, false, true);   // (`order`: a door order's lead — HANDLE.toolNear's near stand, or his own mark and no step at all)
    if (!hs) return false;
    op.kick = { pos: hs.pos, yaw: hs.yaw, t: NaN, hand: true, gate }; op.phase = 'kick'; op.t0 = g.time;
    this.assign(op, lead, { post: hs.pos, face: hs.yaw, watch: null, stance: 'none', upper: 'none', going: `on the door · ${doorWords(d)}`, there: 'on the handle' });
    return true;
  }
  private gateAsked = new SideSet<DoorOp>();

  /** n marks in file along the wall, `standoff` metres past a jamb on face `side` (the latch side first), nearest the door first — null: no wall has the floor */
  standoffPosts(d: Door, side: 1 | -1, n: number, men: Guard[] = []): Post[] | null {
    // (stackroom.ts: a stand-off mark is on the floor of the room that face is in too — dealt again clear of the backblast where it was not)
    const g = this.g, fix = (P: Post[] | null) => stackRoomFix(g, d, side, P, [g.player.char.pos, ...g.orders.heldOn(d, men)], DOORTOOLS.backblastM + 0.2);
    if (!(PICTURE.on && PICTURE.standoff && anyIdea(g, men))) return fix(this.standoffPostsAsWere(d, side, n, men));
    // (picture.ts: with an enemy known, every file the wall has room for is weighed AS THE MEN WILL STAND IT — after the room rule above has had its say — and
    // the first no known enemy has a line to is taken, the latch side's among equals; a file the men cannot walk to as it would be dealt is no file: the
    // very test the row's why-not makes of the answer (standoffWalk; orders.ts keeps a walk's answer under a second, so the row's own asking is not paid twice))
    const files: Post[][] = [], first = this.standoffPostsAsWere(d, side, n, men, files);
    const whole = files.map(fix).filter((F): F is Post[] => !!F); if (!whole.length) return fix(first);
    const ranked = safestSets(g, men, whole), pick = ranked.find(F => men.length === n ? !this.standoffWalk(men, d, F) : F.every(P => men.some(m => !g.orders.reach(m, P.pos)))) ?? whole[0];   // (a men list that is not the deal's: each mark walkable by one of them)
    if (pick !== whole[0]) PICTURE_STATS.moved++;
    return pick;
  }
  /** the marks as the wall deals them: the first file with room (`files`: every such file is also handed back, in that order) */
  private standoffPostsAsWere(d: Door, side: 1 | -1, n: number, men: Guard[] = [], files?: Post[][]): Post[] | null {
    const g = this.g, K = DOORTOOLS, W = d.def.width, nav = g.col.nav, front = doorPt(d, W / 2, g.orders.knobs.jambOut, side, 0.45);
    const th = d.def.closedDir, ux = Math.cos(th), uz = Math.sin(th);
    for (const off of K.standoff) for (const dir of [1, -1] as const) {   // dir +1: out past the latch; −1: out past the hinge
      const out: Post[] = [];
      for (let k = 0; k < n; k++) {
        const a = dir > 0 ? W + off + k * g.orders.knobs.fileGap : -off - k * g.orders.knobs.fileGap; const pos = doorPt(d, a, g.orders.knobs.jambOut, side);
        if (nav.isBlocked(pos[0], pos[2]) || g.col.solidBlocked(front, [pos[0], 0.45, pos[2]])) break;
        const fx = -ux * dir, fz = -uz * dir;   // facing back along the wall at the door
        out.push({ pos, face: Math.atan2(fx, fz), stackSide: ((uz * side) * (-fz) + (-ux * side) * fx >= 0 ? 1 : -1) as -1 | 1, lead: k === 0 });
      }
      if (out.length === n) { if (!files) return out; files.push(out); }
    }
    if (files?.length) return files[0];
    if (n > 2) for (const off of K.standoff) {   // three or more with no one wall long enough: a file out past each jamb (the longer past the latch), nearest the door first
      const fileOf = (dir: 1 | -1): Post[] => { const out: Post[] = [];
        for (let k = 0; k < n; k++) { const a = dir > 0 ? W + off + k * g.orders.knobs.fileGap : -off - k * g.orders.knobs.fileGap; const pos = doorPt(d, a, g.orders.knobs.jambOut, side);
          if (nav.isBlocked(pos[0], pos[2]) || g.col.solidBlocked(front, [pos[0], 0.45, pos[2]])) break;
          const fx = -ux * dir, fz = -uz * dir; out.push({ pos, face: Math.atan2(fx, fz), stackSide: ((uz * side) * (-fz) + (-ux * side) * fx >= 0 ? 1 : -1) as -1 | 1, lead: false, group: dir > 0 ? 0 : 1 }); }
        return out; };
      const A = fileOf(1), B = fileOf(-1); let na = Math.min(A.length, Math.ceil(n / 2)); const nb = Math.min(B.length, n - na); na = Math.min(A.length, n - nb);
      const out: Post[] = []; for (let k = 0; k < Math.max(na, nb); k++) { if (k < na) out.push(A[k]); if (k < nb) out.push(B[k]); }
      if (out.length < n) out.push(...g.orders.rearPosts(d, side, n - out.length, out.map(o => o.pos)));   // (… and a door in a corner: the rest stand back from it — rearPosts' marks are two metres and more off the frame line, outside the backblast)
      if (out.length < n) continue;
      out[0].lead = true; return out;
    }
    const M = g.orders.doorMarks.find(m => m.door === d && m.side === side);   // no wall has the floor: the marks somebody authored for this face (orders.ts doorMarks)
    const marks = M ? (M.standoff.length >= n ? M.standoff.slice(0, n) : null) : g.orders.workedPick(d, side, n, [g.player.char.pos, ...g.orders.heldOn(d, men)], K.backblastM + 0.2);   // … or, on a face with no floor at either jamb and none authored, the ones the men work out for themselves (orders.ts workedMarks), clear of the backblast
    if (marks) return marks.map((p, k) => { const face = yawTo(p, d.frameCentre), fx = Math.sin(face), fz = Math.cos(face);
      return { pos: v3.copy(p), face, stackSide: ((uz * side) * (-fz) + (-ux * side) * fx >= 0 ? 1 : -1) as -1 | 1, lead: k === 0 }; });
    if (g.orders.knobs.hallFile) {   // a face with a passage's wall behind a jamb (orders.ts hallPlaces): the file back down it, from beyond the backblast
      const sam = g.player.char.pos, held = g.orders.heldOn(d, men), hall = g.orders.hallPlaces(d, side, n, K.backblastM + 0.4, q => flat(sam, q) < 0.7 || held.some(h => flat(h, q) < 0.7));
      if (hall.length === n) return hall.map((P, k) => ({ pos: P.pos, face: P.face, stackSide: P.stackSide, lead: k === 0 }));
    }
    return null;
  }

  /** a set charge whose order is on a word that has not gone yet (it is waiting for the others to reach their doors) */
  private onPendingWord(c: SetCharge): boolean { return this.ops.some(o => o.kind === 'charge' && o.door === c.door && !!o.word && isNaN(o.word.go1)); }
  /** Sam's trigger as he pulls it (F, or the menu's 'blow the charge'): that charge — and, when its door is one of the `linked`, every other set charge on them
   *  (set is set: a linked charge whose men are still walking off it goes too, as the nearest one does on any F — the trigger is Sam's to time) */
  trigger(c: SetCharge | null = this.nearest()) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('DoorTools.trigger', this, [c]); if (__r !== RUST_PASS) return __r as any; } }
    if (!c) return;
    const also = this.linked.includes(c.door) ? this.set.filter(o => o !== c && this.linked.includes(o.door)) : [];
    this.blow(c); for (const o of also) this.blow(o);
  }
  private nearest(of: (c: SetCharge) => boolean = () => true): SetCharge | null { const p = this.g.player.char.pos; return this.set.filter(of).sort((a, b) => flat(a.door.frameCentre, p) - flat(b.door.frameCentre, p))[0] ?? null; }
  private arrived(op: DoorOp, gd: Guard) { return !!op.mine.get(gd)?.arrived; }
  private end(op: DoorOp, why: string) {
    const i = this.ops.indexOf(op); if (i < 0) return; this.ops.splice(i, 1);
    this.defuse(op); this.offLock(op); if (op.bangHeld) { op.bangHeld = false; this.refund(op.frag ? 'frag' : 'bang'); }
    this.pieOff(op);
    this.beat(op, `end: ${why}`);
    this.done.push(Object.assign(op, { ended: why })); if (this.done.length > 12) this.done.shift();
  }
  /** the leaf stands open in the team's picture of it (fogofwar.ts doorHeld / doorShown: where an eye of the team's last had it — the leaf itself
   *  while the picture is up to date, and always with the fog off) */
  private knownOpen(d: Door): boolean { const F = this.g.fogOfWar; return F.doorHeld(d) ? Math.abs(F.doorShown(d)) >= OPEN_RAD : isOpen(d); }
  /** a canister this op threw and has not fired: left to its own short fuse (never a live can lying about for the long fuse it was thrown on) */
  private defuse(op: DoorOp) { const it = op.can; if (it && !it.fired && op.pass) { it.pos = v3.copy(op.pass.to); it.vel = v3.copy(op.pass.v0); it.heading = v3.yawOf(op.pass.v0); it.bounces = 0; it.settled = false; }   // (bangInto: an order cut short while the can is on its way to the door's edge — it goes in from there all the same, not on down the wall on our side)
    op.pass = undefined; if (it && !it.fired && it.kind !== 'frag') it.fuse = Math.min(it.fuse, DOORTOOLS.fuseAfterLand); }

  /** behind the bang the first man closes up to the very edge of the frame (still behind the wall: the blind burst goes through the opening) — the less floor to
   *  cover when it runs dry, the more of the flash is left in their eyes when he shows */
  private closeUp(op: DoorOp, gd = op.lead) {
    const g = this.g, d = op.door, W = d.def.width, P = op.post.get(gd); if (!P || !op.men.includes(gd)) return;
    const a = alongOf(d, P.pos) > W / 2 ? W + DOORTOOLS.edgeOut : -DOORTOOLS.edgeOut, q = doorPt(d, a, offOf(d, P.pos, op.side), op.side);
    if (g.col.nav.isBlocked(q[0], q[2]) || g.col.solidBlocked([P.pos[0], 0.45, P.pos[2]], [q[0], 0.45, q[2]])) return;
    this.assign(op, gd, { post: q, face: P.face, watch: [d.frameCentre[0], 1.05, d.frameCentre[2]], stance: 'stack', upper: 'none', stackSide: P.stackSide, going: `closing up · ${doorWords(d)}`, there: `on the frame · ${doorWords(d)}` });
  }
  /** unlatched and shoved hard into the room off its hinge, from the jamb (squad.ts RoomClear's keyed shove) */
  private shove(op: DoorOp) {
    const g = this.g, d = op.door, c = op.lead.char; const s = -g.doors.side(d, doorPt(d, d.def.width / 2, 0.6, op.side));
    d.latched = false; d.closing = false; d.vel = s * Math.max(Math.abs(d.vel), DOORTOOLS.shove); d.quietT = -1.0; d.creakArmed = false; d.lastWho = c.id;
    if (silentEase(g, op.lead)) g.audio.play('doorCreak', d.pos, 0.5); else g.audio.play('doorBash', d.pos, 0.5, { rate: 1.15 });   // (a SILENT squad's hand on the leaf, not its shoulder: silent.ts)
    this.beat(op, 'shove');
  }
  /** the leaf is moving (or stood open): what comes next, by the order */
  private opened(op: DoorOp) {
    const t = this.g.time; if (isNaN(op.openT)) op.openT = t; this.stats.opened++; op.t0 = t;
    if (op.kind === 'openClear') this.enter(op);
    else if (op.kind === 'bangClear') op.phase = 'toss';
    else if (op.kind === 'outside') this.toSlice(op);
    else if (op.kind === 'threshold') this.toThreshold(op);
  }
  private landFor(d: Door, side: 1 | -1, deep: number): Vec3 {
    const g = this.g, op = { side }, W = d.def.width; let best = doorPt(d, W / 2, 0.6, -op.side as 1 | -1), prev = best;
    const plan = doorPlan(g, d.def.name) as (RoomClearPlan & { toss?: { land: Vec3 } }) | null;   // (the breach plan's own landing spot, when this door has one from this face: open floor a can stops on)
    if (plan?.toss && sideOf(d, plan.stack[0]) === op.side && offOf(d, plan.toss.land, op.side) < -0.5) return [plan.toss.land[0], 0.1, plan.toss.land[2]];
    for (let b = 0.9; b <= deep + 1e-6; b += 0.3) { const p = doorPt(d, W / 2, b, -op.side as 1 | -1);
      if (g.col.nav.isBlocked(p[0], p[2]) || g.col.solidBlocked([prev[0], 0.45, prev[2]], [p[0], 0.45, p[2]])) break; best = prev = p; }
    return [best[0], 0.1, best[2]];
  }
  /** WHO GOES IN FIRST (`leadSeen`): of the front man of the file on each jamb, the one who has seen most of the room behind the door — when that is
   *  `leadSeenMin` cells more than the lead has; else the lead. Not the man who threw the can while another stands at the frame. `at`: a point inside the room
   *  (the entry's first point of domination; left out, the door's first point as the room's shape gives it — only the room it lies in is read) */
  private firstIn(op: DoorOp, at?: Vec3, byGun = true): Guard {   // (`byGun` false: who it would have been by the room seen alone — asked only to say why he goes first)
    const K = DOORTOOLS, lead = op.lead, d = op.door, W = d.def.width;
    if (!K.leadSeen || op.men.length < 2) return lead;
    if (byGun && op.first && op.first.state !== 'dead' && op.men.includes(op.first)) return op.first;
    const where = (m: Guard) => op.post.get(m)?.pos ?? m.char.pos, hi = (m: Guard) => alongOf(d, where(m)) > W / 2, near = (m: Guard) => flat(where(m), d.frameCentre);
    const fronts = [true, false].map(s => op.men.filter(m => m.state !== 'dead' && hi(m) === s && offOf(d, m.char.pos, op.side) > 0).sort((a, b) => near(a) - near(b))[0]).filter((m): m is Guard => !!m);
    const hands = fronts.filter(m => m !== op.threw || m === lead || fronts.length < 2);   // (a lead who threw it himself — secondThrows off — goes first as he did)
    const rw = rearWatcher(this.g, op), able = rw && hands.some(m => m !== rw) ? hands.filter(m => m !== rw) : hands;   // (rearwatch.ts: the man on the open way at the stack's back is not the one picked to go first while another file has a front man who did not throw the can — he follows last)
    return this.seenMost(lead, byGun ? gunFirst(able) : able, at ?? this.podsAsWere(d, op.side, 2)[0]?.pos);   // (gunroles.ts: a room gun's man before a long gun's — the same men where their guns do not differ)
  }
  /** of `men`, the one who has seen most of the room `at` is in — `lead` unless another has seen `leadSeenMin` cells more than he has (a lead who is not
   *  among them — he threw the can; a man of his own file stands nearer the frame — has no claim over the men who are) */
  private seenMost(lead: Guard, men: Guard[], at: Vec3 | undefined): Guard {
    const g = this.g, K = DOORTOOLS, room = at && this.seen ? g.fight?.roomAt(at) ?? null : null;
    if (!room || !men.length) return men.includes(lead) || !men.length ? lead : men[0];
    const since = g.time - K.leadSeenSecs, n = (m: Guard) => this.seen!.count(m, room.cells, since);
    let best = men.includes(lead) ? lead : men[0], bn = best === lead ? n(lead) + Math.max(0, K.leadSeenMin - 1) : n(best);
    for (const m of men) { if (m === best) continue; const k = n(m); if (k > bn) { bn = k; best = m; } }
    return best;
  }
  /** when he came through the frame ON THIS ENTRY (a man the order found on the far side crossed nothing: NaN until 'going in') */
  private inAt(op: DoorOp, gd: Guard): number { const tx = op.crossed.get(gd); return tx === undefined || op.phase !== 'enter' ? NaN : Math.max(tx, op.t0); }
  /** the men an entry has left holding a room, each with the sector his point gave him (`face`) and how far it has been swung off a mate (`d`) */
  private held = new Map<Guard, { A: Assignment; face: number; d: number; hitT: number; arrT: number; out?: boolean; door?: Door }>();
  /** rearwatch.ts (REARWATCH.room): the men an ended entry has holding one of its room's other doors, each with the door and the order he holds it on */
  doorKeepers(): [Guard, Door, Assignment][] { return goneWithTick('DoorTools.doorKeepers') as any; }
  /** the entries that have ended 'in' with men still holding the room: their rooms' other doors are still dealt (otherDoors) */
  private watches: DoorOp[] = [];
  /** the room this entry goes into, as the room fight has it: asked at a point of domination (free floor inside by construction — the floor just inside a door
   *  can be furniture, and the nearest free cell to THAT can be the corridor's) */
  private roomIn(op: DoorOp) { const pod = op.queue[0]?.pod ?? op.pod.values().next().value; return pod ? this.g.fight?.roomAt(pod.pos) ?? null : null; }
  /** the room this entry is into as floor to look over (built once, on the first tick of the entry) */
  private searchOf(op: DoorOp, at?: Vec3): RoomSearch | null {
    if (op.search !== undefined) return op.search;
    const g = this.g, K = DOORTOOLS; op.search = null; if (!K.search) return null;
    const room = at ? g.fight?.roomAt(at) ?? null : this.roomIn(op); if (!room || openFloor(g, room)) return null;   // (`at`: asked before anybody has a point in there — the threshold)
    for (const o of this.ops) if (o !== op && o.search && o.search.room === room) return op.search = o.search;   // (another order already in this room: one floor, looked over between them)
    const nav = g.col.nav, cs = nav.cell, pts: Vec3[] = [];
    for (const ci of room.cells) pts.push([(ci % nav.W + 0.5) * cs, 1.0, (((ci / nav.W) | 0) + 0.5) * cs]);
    return op.search = { room, pts, looked: new Uint8Array(pts.length), los: new Map(), yaw: new Map(), at: new Map(), moves: 0, leftAtMove: Infinity, left: pts.length };
  }
  /** has `gd` a line from where he stands to cell `i` (asked once per cell per place he stands) */
  private lineTo(SR: RoomSearch, gd: Guard, i: number): boolean {
    const hd = DOORTOOLS.thrLean && gd.script?.lean ? gd.char.bones.head ?? null : null, p = hd ?? gd.char.pos; let L = SR.los.get(gd);   // (a man leaned round an edge looks from where his head is — on a knee, from that height)
    if (!L || flat(L.at, p) > 0.35 || (hd ? Math.abs(L.at[1] - hd[1]) > 0.2 : L.at[1] !== p[1])) { L = { at: v3.copy(p), ok: new Int8Array(SR.pts.length).fill(-1) }; SR.los.set(gd, L); }
    if (L.ok[i] < 0) L.ok[i] = this.g.col.segmentBlocked([L.at[0], hd ? L.at[1] : 1.55, L.at[2]], SR.pts[i]) ? 0 : 1;
    return L.ok[i] === 1;
  }
  /** every cell a man's muzzle is on this frame is looked at (a man who cannot see, or is not through the frame yet, looks at nothing) */
  private look(op: DoorOp, SR: RoomSearch, out: Guard[] | null = null, ok: ((gd: Guard, i: number) => boolean) | null = null) {
    const g = this.g, K = DOORTOOLS;
    for (const gd of out ?? op.men) { if ((!out && isNaN(this.inAt(op, gd))) || g.time < gd.dazzledUntil) continue; const c = gd.char, p = c.pos;   // (`out`: the men working it from outside the frame — nothing counts for them but what a muzzle with a line to it is on)
      for (let i = 0; i < SR.pts.length; i++) { if (SR.looked[i]) continue; const q = SR.pts[i], dx = q[0] - p[0], dz = q[2] - p[2], dd = dx * dx + dz * dz;
        if (dd > K.lookM * K.lookM) continue;
        if ((out !== null || dd > 1.44) && (Math.abs(wrapAngle(Math.atan2(dx, dz) - c.aimYaw)) > K.lookRad || !this.lineTo(SR, gd, i))) continue;
        if (ok && !ok(gd, i)) continue;
        SR.looked[i] = 1; SR.left--; } }
  }
  /** DOORTOOLS.buddy — `gd` would be dealt a walk to `P`. By the way a man walks (`len`), with every teammate taken where he stands or where his order has
   *  him going (`where`) and Sam where he stands:
   *    'alone'  a friend is within `buddyM` of P: one man's walk, he has left nobody
   *    a man    past that: the partner who goes with him — his pair-mate while that man is up (and then only if he is in `pool`, the men who may be sent
   *             now: busy, in a fight or already posted, and it is 'no'); a man who has lost his partner goes with another who has lost his
   *    'no'     no partner · or nobody of the team would stay behind the two (a squad of two, the last two of a four: the room they took is theirs to hold)
   *             · or the walk takes him (them) past `buddyPairM` from everybody who is not of the pair — the squad is two pairs, not two patrols · or
   *             it would leave a man who has company now with nobody inside `buddyLeftM`
   *  `pair` (flowOn: a walk INTO floor nobody has seen): a partner is wanted even with a friend within `buddyM` — every other rule as it stands, but one:
   *  `onDoor` (the men of the entry whose gun is on another door of the room): a man whose partner holds such a door goes with another whose partner does —
   *  the two who are spare, their partners in the room behind them — where without it neither could go.
   *  A classifier only: it deals nothing, and with the switch off its answer is what the callers count (`sentAlone`). */
  /** why away() last said 'no' (the beats' and the probes') */
  private awayWhy = ''; /** …and was it for the walk's length from the men who stay (the flow's close-up answers that one) */ private awayFar = false;
  /** is this room's floor one the whole-room rule works (`flowOn`, and not past `flowMax`)? A room past the size is searched as before the rule — said once an order */
  private bigSaid = new SideSet<DoorOp>();

  /** the points of domination for a door taken from `side`: the level's clear plan when it was authored from that face, else the first free floor inside
   *  and to either hand of the doorway — each with the facing held there. `n` > 2 (the four-man squad): the first two as ever — the DEEP points, the first
   *  two men's — then one more to either hand just inside the entry wall, clear of the opening, for the third and fourth (as many as there is floor for) */
  pods(d: Door, side: 1 | -1, n = 2): { pos: Vec3; face: number }[] { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('DoorTools.pods', this, [d, side, n]); if (__r !== RUST_PASS) return __r as any; } }
    // (entrypoints.ts: a point within 3 m and in the line of an opening nobody of the squad has been beyond is moved off it where the room has the floor)
    const W = d.def.width, out = this.g.entry.clearOf(d, this.podsAsWere(d, side, n), p => [alongOf(d, p) - W / 2, -offOf(d, p, side)]);
    for (const P of out) this.offSwing(P, d, side, out.filter(o => o !== P).map(o => o.pos));
    return out;
  }
  /** doorway.ts MARKS (DOORMANNERS.podClear): a point a man is left standing on after an entry is in no frame's strip and nowhere a leaf swings — the door
   *  they came by, or any other door of the room. Moved IN PLACE (whoever holds the point holds the new one) to the nearest floor clear of both: in the
   *  room it was in, inside the entry door's line if it was, by no opening nobody has been beyond unless it was by one, with a line to `see` if it had
   *  one, off Sam and `taken`. A point with no such floor by it stays where it is. */
  private offSwing(P: { pos: Vec3; watch?: Vec3 }, d: Door | null, side: 1 | -1, taken: Vec3[], see: Vec3 | null = null) {
    const g = this.g; if (!DOORMANNERS.on || !DOORMANNERS.podClear || !doorwayInWay(g, P.pos)) return;
    const W = P.watch && !g.col.segmentBlocked([P.pos[0], 1.5, P.pos[2]], [P.watch[0], 1.5, P.watch[2]]) ? P.watch : null;   // (the opening he was put there to watch: still in his sight from where he is moved to)
    const room = g.fight?.roomAt(P.pos) ?? null, inside = !!d && -offOf(d, P.pos, side) >= 0.3, risky = !!g.entry.riskAt(P.pos, d);
    const q = doorwayClearNear(g, P.pos, q => doorwayInWay(g, q) || (inside && -offOf(d!, q, side) < 0.3) || (!!room && g.fight.roomAt(q) !== room)
      || (!risky && ENTRY.offOpenings && !!g.entry.riskAt(q, d)) || flat(g.orders.samStood, q) < 0.7 || (!!see && g.col.segmentBlocked([q[0], 1.5, q[2]], see)) || (!!W && g.col.segmentBlocked([q[0], 1.5, q[2]], [W[0], 1.5, W[2]])), taken, 0.9);
    if (q) { P.pos[0] = q[0]; P.pos[2] = q[2]; DOORMANNERS_STATS.marks++; }
  }
  /** the points before entrypoints.ts looks at them (ENTRY.offOpenings off: what pods() answers) */
  podsAsWere(d: Door, side: 1 | -1, n = 2): { pos: Vec3; face: number }[] {
    const two = this.pods2(d, side); if (n <= 2 || !two.length) return two;
    const g = this.g, nav = g.col.nav, W = d.def.width, into = -side as 1 | -1, mid = doorPt(d, W / 2, 0.9, into, 0.45), out = [...two];
    const fits = (p: Vec3) => !nav.isBlocked(p[0], p[2]) && !g.col.solidBlocked(mid, [p[0], 0.45, p[2]]) && !out.some(o => flat(o.pos, p) < 0.9) && !this.inFrame(d, mid, p);
    for (const s of [1, -1]) { if (out.length >= n) break;
      find: for (const D of [1.0, 1.4, 0.8, 1.9]) for (const L of [W / 2 + 0.9, W / 2 + 1.4, W / 2 + 0.55, W / 2 + 2.0]) {   // (L past the jamb: never in the opening itself)
        const p = doorPt(d, W / 2 + s * L, D, into); if (!fits(p)) continue;
        out.push({ pos: p, face: yawTo(p, doorPt(d, W / 2 - s * 0.6, 4.5, into)) }); break find; } }
    for (const b of [3.0, 2.2, 3.8]) { if (out.length >= n) break; const p = doorPt(d, W / 2, b, into); if (fits(p)) out.push({ pos: p, face: yawTo(doorPt(d, W / 2, 0, into), doorPt(d, W / 2, 1, into)) }); }   // (a room with no floor to a hand of its door: on down the middle, past the others)
    return out;
  }
  /** DOORTOOLS.podFrames: `p` stands in the opening of a door other than `d`, or the line to it from `mid` (this door's mouth) passes through one */
  private inFrame(d: Door, mid: Vec3, p: Vec3): boolean {
    const K = DOORTOOLS; if (!K.podFrames) return false;
    for (const o of this.g.doors.list) { if (o === d) continue;
      const [hx, hz] = o.def.hinge, W = o.def.width, ux = Math.cos(o.def.closedDir), uz = Math.sin(o.def.closedDir);
      const a = (p[0] - hx) * ux + (p[2] - hz) * uz, b = -(p[0] - hx) * uz + (p[2] - hz) * ux;
      if (a > -0.1 && a < W + 0.1 && Math.abs(b) < K.podFrameM) return true;
      const m = -(mid[0] - hx) * uz + (mid[2] - hz) * ux; if (m * b >= 0) continue;                 // (both on one side of its wall: no line through it)
      const t = m / (m - b), x = (mid[0] + (p[0] - mid[0]) * t - hx) * ux + (mid[2] + (p[2] - mid[2]) * t - hz) * uz;
      if (x > 0 && x < W) return true;
    }
    return false;
  }
  private pods2(d: Door, side: 1 | -1): { pos: Vec3; face: number }[] {
    const g = this.g, nav = g.col.nav, W = d.def.width, into = -side as 1 | -1;
    const plan = doorPlan(g, d.def.name);
    if (plan && sideOf(d, plan.stack[0]) === side) {
      const out = [0, 1].map(k => ({ pos: v3.copy(plan.entry[k][plan.entry[k].length - 1]), face: plan.podFace[k] })).filter(p => !nav.isBlocked(p.pos[0], p.pos[2]) && offOf(d, p.pos, side) < -0.3);
      if (out.length === 2) return out;
    }
    const mid = doorPt(d, W / 2, 0.9, into, 0.45), out: { pos: Vec3; face: number }[] = [];
    for (const s of [1, -1]) {
      find: for (const D of [1.7, 2.2, 1.2]) for (const L of [1.7, 1.3, 0.9]) {
        const p = doorPt(d, W / 2 + s * L, D, into);
        if (nav.isBlocked(p[0], p[2]) || g.col.solidBlocked(mid, [p[0], 0.45, p[2]]) || this.inFrame(d, mid, p)) continue;
        out.push({ pos: p, face: yawTo(p, doorPt(d, W / 2 - s * 1.2, 4, into)) }); break find;
      }
    }
    const inward = yawTo(doorPt(d, W / 2, 0, into), doorPt(d, W / 2, 1, into));
    for (const b of [1.5, 2.4]) { if (out.length >= 2) break; const q = doorPt(d, W / 2, b, into), f = nav.isBlocked(q[0], q[2]) ? nav.nearestFreePoint(q[0], q[2]) : q; if (f && !out.some(o => flat(o.pos, f) < 0.6)) out.push({ pos: [f[0], 0, f[2]], face: inward }); }
    return out;
  }
  /** the men go in: the lead first, the next `second` later, each to his point of domination */
  private enter(op: DoorOp) {
    if (op.thr) { this.stepIn(op); return; }   // (worked from the threshold: the shallow step, not the points of domination)
    const g = this.g, S = g.orders, t = g.time; const pods = this.pods(op.door, op.side, op.men.length > 2 || S.big ? 4 : op.men.length);   // (three men are offered both near points: the third takes the one on his own side; a pair of the four is offered them too — podsApart)
    if (!pods.length) { g.say(op.lead, 'no way in.'); this.end(op, 'no floor inside'); return; }
    const first = this.firstIn(op, pods[0].pos); this.led.entries++;
    if (first !== op.lead && gunBefore(first, op.lead) && this.firstIn(op, pods[0].pos, false) !== first) this.beat(op, `first:${first.callsign} · his gun is the room's`);   // (gunroles.ts: false with its switch off)
    else if (first !== op.lead) { this.led.bySeen++; this.beat(op, `first:${first.callsign} · has seen most of the room`); }
    let men = [first, ...op.men.filter(m => m !== first)];
    if (men.length > 2) {
      // THREE OR MORE: in turn off alternate jambs while both files have men — the lead, the front man of the other file, then the men behind them — the first
      // two to the deep points, the next two to the near ones, each couple dealt the way that walks least (a lone third: the nearer). A man with no point
      // (no floor for a third / fourth inside) keeps his place on the stack, outside, and is not this order's any more.
      const d = op.door, W = d.def.width, hi = (gd: Guard) => alongOf(d, (op.post.get(gd)?.pos ?? gd.char.pos)) > W / 2, near = (gd: Guard) => flat(op.post.get(gd)?.pos ?? gd.char.pos, d.frameCentre);
      const mine = men.filter(m => hi(m) === hi(first)).sort((a, b) => (a === first ? -1 : b === first ? 1 : near(a) - near(b))), theirs = men.filter(m => hi(m) !== hi(first)).sort((a, b) => near(a) - near(b));
      let turn: Guard[] = []; for (let k = 0; k < men.length; k++) { if (mine[k]) turn.push(mine[k]); if (theirs[k]) turn.push(theirs[k]); }
      turn = gunTurn(turn, first);   // (gunroles.ts: the long guns follow last — the turn as it was where the guns do not differ)
      turn = rearLast(g, op, turn, first);   // (rearwatch.ts: the man on the open way at the stack's back goes in last)
      const inMen = turn.slice(0, pods.length), stay = turn.slice(pods.length);
      const deal = new Map<Guard, { pos: Vec3; face: number }>();
      for (const k of [0, 2]) { const m = inMen.slice(k, k + 2), sp = pods.slice(k, k + 2); if (m.length) for (const [gd, pos] of S.pair(m, sp.map(x => x.pos))) deal.set(gd, sp.find(x => x.pos === pos)!); }
      if (stay.length) { op.men = op.men.filter(m => !stay.includes(m)); this.beat(op, `outside · ${stay.map(m => m.callsign).join(', ')}`); }
      op.queue = inMen.map((gd, i) => ({ gd, at: t + i * DOORTOOLS.second, pod: deal.get(gd) ?? pods[0] }));
      for (const q of op.queue) g.entry.dealt(q.pod, op.door);
      op.phase = 'enter'; op.t0 = t; this.beat(op, 'go');
      g.say(op.lead, 'going in.'); return;
    }
    const own = S.big ? this.podsApart(op, pods, men.length) : pods;
    const pairs = S.pair(men, own.slice(0, men.length).map(p => p.pos));
    op.queue = men.map((gd, i) => ({ gd, at: t + i * DOORTOOLS.second, pod: own.find(p => p.pos === pairs.find(x => x[0] === gd)![1]) ?? own[0] }));
    for (const q of op.queue) g.entry.dealt(q.pod, op.door);
    op.phase = 'enter'; op.t0 = t; this.beat(op, 'go');
    g.say(op.lead, 'going in.');
  }

  /** THE SQUAD OF FOUR, a pair going in: its points of domination kept `podApart` clear of the points another order's men are on their way to or standing on in
   *  there (the other pair, through the room's other door on the same word) and of any teammate already posted inside — the deep points first, then the near
   *  ones; with too few left clear it takes what there is */
  private podsApart(op: DoorOp, pods: { pos: Vec3; face: number }[], n: number): { pos: Vec3; face: number }[] {
    const S = this.g.orders, K = DOORTOOLS, taken: Vec3[] = [];
    for (const o of this.ops) { if (o === op) continue; for (const p of o.pod.values()) taken.push(p.pos); for (const q of o.queue) taken.push(q.pod.pos); }
    for (const gd of S.men()) { if (op.men.includes(gd)) continue; const A = S.of(gd); if (A?.post && A.arrived && offOf(op.door, A.post, op.side) < 0) taken.push(A.post); }
    const clear = pods.filter(p => !taken.some(q => flat(q, p.pos) < K.podApart)); if (clear.length < pods.slice(0, n).length || clear.slice(0, n).some((p, i) => p !== pods[i])) this.stats.podsMoved++;
    return clear.length >= n ? clear : [...clear, ...pods.filter(p => !clear.includes(p))];
  }

  // ------------------------------------------------------------ the charge
  /** Sam's trigger: the set charge nearest him (or the one given) goes. Returns what it did, null with none set. */
  blow(c: SetCharge | null = this.nearest()): { knocked: number; stunned: number; backblast: number } | null { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('DoorTools.blow', this, [c]); if (__r !== RUST_PASS) return __r as any; } } return this.g.col.window(() => this.g.fogOfWar.doing(this.g.player.char.id, () => this.blowRaw(c))); }   // (ONE WINDOW ROUND THE BLOW, scene/collision.ts window: in play it is entered inside a tick and this is nothing; entered between ticks — a tool, a page's hook, the debugger's button — its hundreds of lines would each prove the module's copy of the box world for itself)   // (a door charge's blast is the team's doing: fogofwar.ts FOG.ours)
  private blowRaw(c: SetCharge | null): { knocked: number; stunned: number; backblast: number } | null {
    if (!c) return null;
    const g = this.g, K = DOORTOOLS, d = c.door, side = c.side, W = d.def.width, into = -side as 1 | -1;
    const at = this.set.indexOf(c); if (at < 0) return null; this.set.splice(at, 1);
    holdFireOrder(g, 'charge', { blown: true });   // (holdfire.ts: a charge starts the fight when it GOES, not when it is set)
    const out = doorPt(d, W / 2, 0.5, side, 1.1), inn = doorPt(d, W / 2, 0.45, into, 1.1);
    // the leaf: off its hinges and into the room, a loose slab (doorblast.ts tearOff — the crack of it is the same 'kick' the floor hears) — or, with that
    // switched off, as it was: the keep torn out, the leaf flung in on its hinge
    if (tearOff(g, d, out, g.player.char.id, DOORBLAST.chargeK)) { /* gone */ }
    else if (!d.def.steel) { g.doors.kickIn(d, out, g.player.char.id, true); if (Math.abs(d.angle) < 1.4) d.vel = -g.doors.side(d, doorPt(d, W / 2, 0.6, side)) * K.blowVel; d.quietT = -K.holdOpen; }
    // …and it is a CALLER of the destruction goal's blast entry point where the build carries one (walls, cover and whoever listens to a blast hear of it from
    // there; this leaf is already off, so the doors' part of that call finds nothing left to take): at the charge's own small radius, and with that lane's chaos
    // left out — the stun charge below runs its bang
    if (DOORBLAST.on && DOORBLAST.chargeR > 0) (g.destr as unknown as { blast?: (at: Vec3, o: { radius: number; who: number; chaos: boolean }) => unknown }).blast?.(doorPt(d, W / 2, 0.15, side, 1.1), { radius: DOORBLAST.chargeR, who: g.player.char.id, chaos: false });
    // the blast itself — flash, report, scorch, what it knocks about — is the stun charge's (combat.ts), at the foot of the leaf, with nobody on its list: who it
    // hurts is decided below, by which side of the door he is on
    const it = g.items.spawn('flash', doorPt(d, W * 0.7, 0.2, into, 0.5), [0, 0, 0], { fuse: 99, who: g.player.char.id }); it.fired = true;
    const all = g.guards, last = g.messages[g.messages.length - 1]; g.guards = []; try { detonateStun(g, it, true); } finally { g.guards = all; }
    const m = g.messages[g.messages.length - 1]; if (m && m !== last && /flashbang/.test(m.text)) g.messages.pop();   // ('flashbang went off': this was not one)
    const ii = g.items.items.indexOf(it); if (ii >= 0) g.items.items.splice(ii, 1);   // (nor is there a spent can to leave on the floor)
    let knocked = 0, stunned = 0, backblast = 0, samToo = false;
    const leaves = g.col.dynamicBoxes; g.col.dynamicBoxes = leaves.filter(b => b !== d.box);   // (walls and every OTHER shut door stop it; this leaf is what is coming apart)
    try {
      for (const gd of [...g.guards]) {
        if (gd.state === 'dead') continue;
        const p = gd.char.pos, far = offOf(d, p, side) < 0, dist = flat(p, d.frameCentre), chest: Vec3 = [p[0], 1.1, p[2]];
        if (dist > K.stunM || g.col.solidBlocked(far ? inn : out, chest)) continue;
        if (!far) { if (dist < K.backblastM) { this.dazzle(gd, K.backblastSecs, inn); if (gd.team === 'operative') backblast++; else stunned++; } continue; }   // too near the charge on its own side
        if (gd.team === 'operative' && g.orders.big) { this.dazzle(gd, K.backblastSecs, inn); backblast++; continue; }   // (one of ours on the far side — the other pair, inside: the order is refused while he is there (behind()); gone in since, he is dazzled, not felled)
        if (dist < K.knockM) { const dir = v3.normalize([p[0] - d.frameCentre[0], 0.15, p[2] - d.frameCentre[2]]); killGuard(g, gd, dir, false, 'struck', 'player', `${gd.callsign ?? 'a man'} down — behind the door when it went`); knocked++; }
        else { this.dazzle(gd, lerp(K.stunSecs[0], K.stunSecs[1], clamp(dist / K.stunM, 0, 1)), inn); gd.staggerUntil = Math.max(gd.staggerUntil, g.time + K.staggerSecs); gd.weapon.burst = null; stunned++; }
      }
      // …and Sam, as one of ours (samstun.ts): too near on the charge's own side, or beyond the door when it goes — dazzled as a teammate there is, never felled
      eachSeat(g, g => { if (SAMSTUN.on && !g.player.down && !g.puppet && !g.touring) { const p = g.player.char.pos, far = offOf(d, p, side) < 0, dist = flat(p, d.frameCentre);   // (every player's man — seats.ts)
        if (dist < (far ? K.stunM : K.backblastM) && !g.col.solidBlocked(far ? inn : out, [p[0], 1.1, p[2]])) { dazzlePlayer(g, K.backblastSecs, inn); samToo = true; } } });
    } finally { g.col.dynamicBoxes = leaves; }
    moraleBlast(g, inn, g.player.char.id);   // (morale.ts: the men behind the door it did not put down are shaken by it — a return on its first line with the switch off)
    this.stats.blown++; this.stats.knocked += knocked; this.stats.stunned += stunned; this.stats.backblast += backblast;
    g.msg(`charge blown — ${doorWords(d)}${knocked ? ` · ${knocked} down` : ''}${stunned ? ` · ${stunned} stunned` : ''}${backblast ? ` · ${backblast} of yours too close` : ''}${samToo ? ' · you were too close' : ''}`);
    this.note(`blow ${d.def.name}: knocked ${knocked} stunned ${stunned} backblast ${backblast}`);
    for (const op of this.ops) if (op.kind === 'charge' && op.door === d && (op.phase === 'standoff' || op.phase === 'armed')) {   // the squad that set it goes in behind it
      op.openT = g.time; op.bangT = g.time; op.bangAt = inn; op.phase = 'wait'; op.t0 = g.time; this.beat(op, `blow · ${knocked} down ${stunned} stunned`);
    }
    return { knocked, stunned, backblast };
  }
  /** flash-blind (combat.ts detonateStun's own rule for a man with the charge in his open view) */
  private dazzle(gd: Guard, secs: number, at: Vec3) {
    const g = this.g;
    gd.dazzledUntil = Math.max(gd.dazzledUntil, g.time + secs); gd.reactT = Math.max(gd.reactT, secs);
    gd.awareness = Math.max(gd.awareness, 0.6); if (gd.state !== 'alert') word(g, gd, at);
    if (g.aiEnabled) gd.witness.dazzled++;
    gd.path = []; gd.pathGoal = null; gd.speed = 0; gd.dazzledAt = v3.copy(at); gd.char.anim.hitFrom(v3.sub(gd.char.pos, at));
  }

  /** DOORTOOLS.thrPie: the hold is over (the word, a step in, the pockets, the order's end) with a man still mid-cut. He is done cutting; and if nothing has
   *  given him another place since, he stands square where his hips are, off the knee, on the middle of what his cut showed */
  private pieOff(op: DoorOp) {
    const S = this.g.orders;
    for (const [gd, s] of op.slice) { const P = s.pie; if (!P || P.fin || !P.begun) continue; P.fin = true;
      const A = S.of(gd); if (!A || A !== op.mine.get(gd) || !(A.lean || A.crouch) || gd.state === 'dead') continue;
      const p = gd.char.pos, l = s.looks[s.i], mid = l[l.length >> 1];
      this.assign(op, gd, { post: [p[0], 0, p[2]], face: yawTo(p, mid), watch: mid, stance: 'none', upper: 'aim', light: this.lights ?? NIGHTBREACH.opLights, search: A.search, going: A.going, there: A.there }); }
  }
  /** DOORTOOLS.thrPie: a jamb man's stations — `first` (his diagonal, stood square as with the rule off), then the cuts of thrPieArc further in than
   *  those, each with floor under it and a clear step from the one before, none within thrPieApart of `off` (where the other man stands). Each is where his
   *  EYE goes; with thrLean his hips stop thrLeanM short of it, back along his own wall (where the floor lets them), and a cut further in than thrKneeIn is
   *  a knee's. The hinge man of a leaf that stands on this face (thrPull) has no edge to lean round and no cut to take: his one spot, square. */
  private pieSet(op: DoorOp, gd: Guard, sgn: -1 | 1, first: Vec3[], off: Vec3[]): Vec3[] {
    const g = this.g, K = DOORTOOLS, d = op.door, W = d.def.width, nav = g.col.nav, pull = sgn < 0 && this.leafOurs(d, op.side), lm = K.thrLean && !pull ? K.thrLeanM : 0;
    const inOf = (p: Vec3) => sgn < 0 ? alongOf(d, p) : W - alongOf(d, p);   // metres in from his own jamb's edge of the opening
    const at = (a: number, B: number): Vec3 => doorPt(d, sgn < 0 ? a : W - a, B, op.side);
    const floor = (from: Vec3, p: Vec3) => !nav.isBlocked(p[0], p[2]) && !g.col.solidBlocked([from[0], 0.45, from[2]], [p[0], 0.45, p[2]]);
    const eyes: Vec3[] = [...first], pts: Vec3[] = []; for (const [k, e] of first.entries()) { const q = k === 0 || !lm ? e : at(inOf(e) - lm, offOf(d, e, op.side)); pts.push(q === e || floor(pts[k - 1], q) ? q : e); }
    if (!pull) for (const [a, B0] of K.thrPieArc) { const B = B0 * K.sliceBack, prev = pts[pts.length - 1];
      if (a <= inOf(eyes[eyes.length - 1]) + 0.05) continue;
      const p = at(a - lm, B), e = K.thrPieOne ? p : at(a, B);   // (thrPieOne: square on every cut, his hips where a leaning man's stop — never in front of the opening — and his eye over them; the one lean comes after the last)
      if (!floor(prev, p) || off.some(o => flat(o, p) < K.thrPieApart)) continue;
      eyes.push(e); pts.push(p); }
    const fwd = v3.sub(doorPt(d, W / 2, 1, -op.side as 1 | -1), doorPt(d, W / 2, 0, op.side)), inw = v3.sub(at(1, 0), at(0, 0));   // into the room · toward the door's middle from his jamb
    const lean = lm ? Math.sign(inw[0] * -fwd[2] + inw[2] * fwd[0]) : 0;   // (his right hand, facing in, is (−fz, fx): + = the middle is to his right)
    const looks = eyes.map((e, k) => { const l = this.sliceLooks(op, e, sgn); return k === 0 || l.length < 3 ? l : [l[2], l[1]]; });   // (a cut: the ground it opens — the line past his own jamb — then the middle)
    op.slice.set(gd, { pts, looks, i: 0, j: 0, t: NaN, ahead: null, pie: { eyes, lean: K.thrPieOne ? 0 : lean, end: K.thrPieOne ? lean : 0, endOn: false, knee: eyes.map((e, k) => !K.thrPieOne && !!lm && k > 0 && inOf(e) > K.thrKneeIn), fin: false, begun: false, at: g.time, inw: v3.normalize(inw), gun: 0, gunAt: -100, on: false, to: null } });
    return pts;
  }
  /** what a station shows through the frame: the line past the far jamb, the middle, the line past the near jamb (where the next step opens new ground) */
  private sliceLooks(op: DoorOp, p: Vec3, sgn: -1 | 1): Vec3[] {
    const K = DOORTOOLS, d = op.door, W = d.def.width, into = -op.side as 1 | -1;
    const near = sgn < 0 ? K.sliceEdge : W - K.sliceEdge, far = W - near;
    const through = (a: number): Vec3 => { const e = doorPt(d, a, 0, op.side), dx = e[0] - p[0], dz = e[2] - p[2], l = Math.hypot(dx, dz) || 1; return [e[0] + dx / l * K.sliceDeep, 1.1, e[2] + dz / l * K.sliceDeep]; };
    const F = through(far), N = through(near), M: Vec3 = [(F[0] + N[0]) / 2, 1.1, (F[2] + N[2]) / 2];
    return offOf(d, M, op.side) < -0.3 ? [F, M, N] : [doorPt(d, W / 2, K.sliceDeep, into, 1.1)];
  }
  /** each man's stations across the opening, all on Sam's side of the frame line: from wide of his own jamb toward the middle, each with the point inside his gun
   *  is on from there (pivoting through the doorway's middle: from wide of the hinge he sees the latch side of the room) */
  private toSlice(op: DoorOp) {
    const g = this.g, K = DOORTOOLS, d = op.door, W = d.def.width, nav = g.col.nav, name = doorWords(d);
    const looksFrom = (p: Vec3, sgn: -1 | 1) => this.sliceLooks(op, p, sgn);
    const L = op.post.get(op.lead), start: Vec3 = L ? L.pos : [op.lead.char.pos[0], 0, op.lead.char.pos[2]];
    const sgn: -1 | 1 = alongOf(d, start) < W / 2 ? -1 : 1;
    // ONE arc for everybody, from the lead's own jamb — the thinnest slice there is — out, round the front of the door and in to the far wall, each step showing a
    // little more of the room; the lead walks all of it, each man behind him stops two stations short of the man ahead (two muzzles on the opening, two angles)
    const arc: Vec3[] = [start];
    const ours = this.leafOurs(d, op.side);
    for (const [a, B] of [...K.thrArc, [W / 2, 2.0], [W - 0.25, 1.8], [W + 0.45, 1.2]].map(([a, B]) => [a, B * K.sliceBack])) {
      const al = sgn < 0 ? a : W - a, q = doorPt(d, al, B, op.side), prev = arc[arc.length - 1];
      if (ours && al < 0) continue;   // (thrPull: a station beyond the hinge jamb is behind the leaf standing open on this face)
      if (!nav.isBlocked(q[0], q[2]) && (ours && alongOf(d, prev) < 0 || !g.col.solidBlocked([prev[0], 0.45, prev[2]], [q[0], 0.45, q[2]]))) arc.push(q); }   // (… and from a mark beyond it the way out is round that leaf's end: the floor alone is asked)
    const across = (gd: Guard) => { const P = op.post.get(gd); return op.men.length > 2 && (alongOf(d, P ? P.pos : gd.char.pos) < W / 2 ? -1 : 1) !== sgn; };   // (three or more: a man stacked on the OTHER jamb)
    for (const gd of op.men) if (across(gd) && !op.rear.has(gd)) {   // … works the opening from where he stands — his own angle through it, the half of the room the arc's men see last — and never walks across the front of the door to join the arc
      const P = op.post.get(gd), here: Vec3 = P ? P.pos : [gd.char.pos[0], 0, gd.char.pos[2]], looks = looksFrom(here, -sgn as -1 | 1);
      op.slice.set(gd, { pts: [here], looks: [looks], i: 0, j: 0, t: NaN, ahead: null });
      this.assign(op, gd, { post: here, face: yawTo(here, looks[0]), watch: looks[0], stance: 'none', upper: 'aim', light: this.lights ?? NIGHTBREACH.opLights, search: true, going: `slicing · ${name}`, there: `holding outside · ${name}` });
    }
    [op.lead, ...op.men.filter(m => m !== op.lead && !across(m) && !op.rear.has(m))].forEach((gd, k, file) => {
      const n = Math.max(1, arc.length - 2 * k), P = op.post.get(gd), here: Vec3 = P ? P.pos : [gd.char.pos[0], 0, gd.char.pos[2]];
      const pts = k === 0 ? arc.slice(0, n) : [here, ...arc.slice(1, n)];
      const s = { pts, looks: pts.map((q, i) => { const l = looksFrom(q, sgn); return i === 0 || l.length < 3 ? l : [l[2], l[1], l[2]]; }), i: 0, j: 0, t: NaN, ahead: k === 0 ? null : file[k - 1] ?? null };
      op.slice.set(gd, s);
      this.assign(op, gd, { post: s.pts[0], face: yawTo(s.pts[0], s.looks[0][0]), watch: s.looks[0][0], stance: 'none', upper: 'aim', light: this.lights ?? NIGHTBREACH.opLights, search: true, going: `slicing · ${name}`, there: `slicing · ${name}` });
    });
    op.phase = 'slice'; op.t0 = g.time; this.beat(op, 'slice');
    g.say(op.lead, 'working it from outside.');
  }

  // ------------------------------------------------------------ the room worked from the threshold
  /** The leaf is moving and nobody goes in. The LEAD — the opener — and the front man of the other jamb each step out onto their own jamb's diagonal and hold
   *  it, still: between them the two lines cross in the room (`thrSlice`: the lead walks his side of the opening a station at a time instead). Everybody else
   *  of the order keeps the squad's back (DoorOp.rear: tick leaves their eyes spare). Who shoots, and at whom, is the room fight's, as from any mark. */
  /** DOORTOOLS.thrPull: this door's leaf comes round onto face `side` — a one-way door worked from the face it opens toward */
  private leafOurs(d: Door, side: 1 | -1): boolean { const D = this.g.doors; return DOORTOOLS.thrPull && D.swingOf(d).oneWay && D.pullFace(d, doorPt(d, d.def.width / 2, 0.6, side)); }
  /** a jamb man's holding spot (`sgn` −1: the hinge jamb's, +1: the latch jamb's) and where the walk to it is asked from: his jamb's diagonal from his mark —
   *  or, the hinge jamb's on a face the leaf comes round onto, thrPullAt, asked from in front of the latch half (his mark is behind where the leaf will stand) */
  private thrSpot(op: DoorOp, sgn: -1 | 1, mark: Vec3): { q: Vec3; from: Vec3 } {
    const K = DOORTOOLS, d = op.door, W = d.def.width, B = K.thrPullAt[1] * K.sliceBack;
    return sgn < 0 && this.leafOurs(d, op.side) ? { q: doorPt(d, K.thrPullAt[0], B, op.side), from: doorPt(d, W * 0.8, B, op.side) }
      : { q: doorPt(d, sgn < 0 ? -K.thrStill[0] : W + K.thrStill[0], K.thrStill[1] * K.sliceBack, op.side), from: mark };
  }
  /** the far jamb's man (the first man on the other jamb from the lead's — null: one jamb only has floor), the side of the opening he stands off, and his
   *  diagonal: one step out from his mark, where the floor lets him */
  private thrFar(op: DoorOp): { far: Guard; fs: -1 | 1; here: Vec3 } | null {
    const g = this.g, K = DOORTOOLS, d = op.door, W = d.def.width, nav = g.col.nav, lead = op.lead;
    const mark = (gd: Guard): Vec3 => op.post.get(gd)?.pos ?? [gd.char.pos[0], 0, gd.char.pos[2]], hi = (gd: Guard) => alongOf(d, mark(gd)) > W / 2;
    const far = op.men.filter(m => m !== lead && !!op.post.get(m)?.lead && hi(m) !== hi(lead)).sort((a, b) => flat(mark(a), d.frameCentre) - flat(mark(b), d.frameCentre))[0] ?? null;
    if (!far) return null;
    const P = mark(far), fs: -1 | 1 = hi(lead) ? -1 : 1, { q, from } = this.thrSpot(op, fs, P);
    return { far, fs, here: !nav.isBlocked(q[0], q[2]) && !g.col.solidBlocked([from[0], 0.45, from[2]], [q[0], 0.45, q[2]]) ? q : P };
  }
  /** DOORTOOLS.thrLast: is this a threshold order on a hostage room whose captor has given his last warning (rescue.ts) — and the order's own to time (not on
   *  a word, not on a gate with other doors, not given to hold: those are Sam's) */
  private lastCall(op: DoorOp): boolean {
    return DOORTOOLS.thrLast && op.kind === 'threshold' && !op.onWord && !op.word && !op.gate && !op.thrHold && this.g.rescue.lastWarning(op.door);
  }
  /** … heard with a man on his jamb and another still more than `thrLastM` short of his mark: the stack is made with who is there (the stack phase then waits
   *  on nobody). If the man still walking is the LEAD — the latch jamb's, who opens it — the door is handed to the jamb man who is on his mark (as when a lead
   *  drops: open() boots it from wherever its lead stands). A late man whose mark is the OTHER jamb, and who has not passed the opening yet, takes the lead's
   *  wall instead — the first free place behind him, `bangFile` apart, clear of Sam and of men who are not this order's — so nobody walks across a door that
   *  is about to open; with no floor there he keeps his mark (and is no thrower from down the wall: thrThrower) */
  private lastStack(op: DoorOp) {
    const g = this.g, K = DOORTOOLS, d = op.door, W = d.def.width, name = doorWords(d);
    const on = (m: Guard) => { const P = op.post.get(m); return !!P && flat(m.char.pos, P.pos) <= 0.6; }, off2 = (m: Guard) => { const P = op.post.get(m); return !!P && !this.arrived(op, m) && flat(m.char.pos, P.pos) > K.thrLastM; };
    let lead = op.lead, handed = false;
    if (!on(lead)) { if (!off2(lead)) return;
      const sub = op.men.filter(m => m !== lead && m.state !== 'dead' && !!op.post.get(m)?.lead && on(m)).sort((a, b) => flat(a.char.pos, d.frameCentre) - flat(b.char.pos, d.frameCentre))[0]; if (!sub) return;
      lead = sub; handed = true; }
    const late = op.men.filter(m => m !== lead && m.state !== 'dead' && !this.arrived(op, m) && !!op.post.get(m));
    if (!late.length || late.some(m => !off2(m))) return;   // (a man a step from his mark is waited for as ever)
    op.lastStack = true; op.lead = lead;
    const L = op.post.get(lead)!, aL = alongOf(d, L.pos), dir = aL > W / 2 ? 1 : -1, off = offOf(d, L.pos, op.side), watch: Vec3 = [d.frameCentre[0], 1.05, d.frameCentre[2]];
    const others = [g.orders.samStood, ...g.guards.filter(x => (g.orders.sams ? x.team === 'operative' : g.orders.mine(x)) && x.state !== 'dead' && !op.men.includes(x)).map(x => x.char.pos)];
    const free = (c: Vec3, gd: Guard) => !g.col.nav.isBlocked(c[0], c[2]) && !g.col.solidBlocked([L.pos[0], 0.45, L.pos[2]], [c[0], 0.45, c[2]]) && !op.men.some(m => m !== gd && flat(op.post.get(m)?.pos ?? m.char.pos, c) < 0.7) && !others.some(p => flat(p, c) < 0.7) && stackRoomKeeps(g, d, op.side, c);
    for (const gd of late) { const P = op.post.get(gd)!, a = alongOf(d, gd.char.pos);
      if ((alongOf(d, P.pos) > W / 2) === (aL > W / 2)) continue;   // (his mark is on the lead's wall already)
      if (dir > 0 ? a < -0.2 : a > W + 0.2) continue;   // (past the opening already, on his own wall: he goes on to his jamb)
      let q: Vec3 | null = null;
      for (let j = 1; j <= 4 && !q; j++) { const c = doorPt(d, aL + dir * j * K.bangFile, off, op.side); if (free(c, gd)) q = c; }
      if (!q) continue;
      const Q: Post = { ...P, pos: q, face: L.face, stackSide: L.stackSide, lead: false }; op.post.set(gd, Q);
      this.assign(op, gd, { post: q, face: Q.face, watch, stance: 'highReady', upper: 'none', move: { stance: 'none', upper: 'aim' }, stackSide: Q.stackSide, going: `stacking · ${name}`, there: `stacked · ${name}` }); }
    this.beat(op, `last warning · not waiting for ${late.map(m => m.callsign).join(', ')}${handed ? ` · the door is ${lead.callsign}'s` : ''}`); g.say(lead, 'no time. going.');
  }
  private toThreshold(op: DoorOp) {
    const g = this.g, K = DOORTOOLS, d = op.door, W = d.def.width, nav = g.col.nav, name = doorWords(d), t = g.time, lead = op.lead, light = this.lights ?? NIGHTBREACH.opLights;
    const mark = (gd: Guard): Vec3 => op.post.get(gd)?.pos ?? [gd.char.pos[0], 0, gd.char.pos[2]], hi = (gd: Guard) => alongOf(d, mark(gd)) > W / 2;
    const floor = (from: Vec3, p: Vec3) => !nav.isBlocked(p[0], p[2]) && !g.col.solidBlocked([from[0], 0.45, from[2]], [p[0], 0.45, p[2]]);
    const F = this.thrFar(op), far = F?.far ?? null;
    const jamb = far ? [lead, far] : [lead]; op.rear.clear(); for (const m of op.men) if (!jamb.includes(m)) op.rear.add(m);
    // the lead: out onto his own jamb's diagonal — or, slicing, from the mark he stands on (the thinnest slice there is) out off his wall and round to just inside his own edge of the opening
    const sgn: -1 | 1 = hi(lead) ? 1 : -1, pts: Vec3[] = [mark(lead)];
    if (K.thrSlice) for (const [a, B] of K.thrArc) { const q = doorPt(d, sgn < 0 ? a : W - a, B * K.sliceBack, op.side); if (floor(pts[pts.length - 1], q)) pts.push(q); }
    else { const { q, from } = this.thrSpot(op, sgn, pts[0]); if (floor(from, q)) pts[0] = q; }   // (no slice: his own jamb's diagonal, as the far man's)
    const looks = pts.map((q, i) => { const l = this.sliceLooks(op, q, sgn); return !K.thrSlice ? [l[l.length >> 1]] : i === 0 || l.length < 3 ? l : [l[2], l[1], l[2]]; });   // (a step is taken with the muzzle on the ground it opens — the line past his own jamb — then the middle, and back)
    op.slice.set(lead, { pts, looks, i: 0, j: 0, t: NaN, ahead: null });
    if (K.thrPie) {   // (both jamb men cut the opening, each on his own side of its middle: pieSet — or, thrPieOne, the lead alone)
      const diag = (sg: -1 | 1, m: Vec3) => { const { q, from } = this.thrSpot(op, sg, m); return floor(from, q) ? q : null; };
      const lm = mark(lead), ld = diag(sgn, lm), lp = this.pieSet(op, lead, sgn, ld ? [ld] : [lm], F ? [F.here] : []);
      if (far && F && !K.thrPieOne) this.pieSet(op, far, F.fs, [F.here], [lp[lp.length - 1]]);   // (thrPieOne: the far man has no cuts — his diagonal, held, below)
    }
    const sl = op.slice.get(lead)!, sf = K.thrPie && far ? op.slice.get(far) ?? null : null;
    const lit = (gd: Guard) => !(K.thrDark && nvgWorn(g, gd)) && (K.thrLight >= 2 || (K.thrLight === 1 && gd === (K.thrPie && K.thrPieOne ? lead : far ?? lead)));   // (one light on the door: the still gun's — the opener's own when he has the door alone; thrPieOne: the one man cutting carries it, a cut shows him nothing in the dark otherwise)
    // (the step onto the diagonals — as the leaf swings; or, a can going in first, behind its burst: thrOut)
    const out = () => {
    if (op.men.includes(lead)) this.assign(op, lead, { post: sl.pts[0], face: yawTo(sl.pts[0], sl.looks[0][0]), watch: sl.looks[0][0], stance: 'none', upper: 'aim', light, search: lit(lead), ...(K.slicePace > 0 ? { pace: K.slicePace } : {}), going: K.thrSlice || sl.pie ? `slicing · ${name}` : `on the jamb · ${name}`, there: sl.pts.length > 1 ? `slicing · ${name}` : `holding the jamb · ${name}` });
    // the far jamb's man: one step out onto his diagonal, his muzzle down the middle of what it shows — and there he stays
    if (far && sf && op.men.includes(far)) this.assign(op, far, { post: sf.pts[0], face: yawTo(sf.pts[0], sf.looks[0][0]), watch: sf.looks[0][0], stance: 'none', upper: 'aim', light, search: lit(far), ...(K.slicePace > 0 ? { pace: K.slicePace } : {}), going: `slicing · ${name}`, there: sf.pts.length > 1 ? `slicing · ${name}` : `holding the jamb · ${name}` });
    else if (far && F && op.men.includes(far)) { const fs = F.fs, here = F.here;
      const l = this.sliceLooks(op, here, fs), mid = l[l.length >> 1];
      this.assign(op, far, { post: here, face: yawTo(here, mid), watch: mid, stance: 'none', upper: 'aim', light, search: lit(far), ...(K.slicePace > 0 ? { pace: K.slicePace } : {}), going: `on the jamb · ${name}`, there: `holding the jamb · ${name}` }); } };
    const first = !op.thrPre && this.canFirst(op); if (!first) out();
    const pods = this.shallowPods(d, op.side, Math.max(2, op.men.length));
    op.thr = { jamb, pods, quietT: t, evT: t, contact: new Set(), up: g.guards.filter(x => x.state !== 'dead' && x.team === 'hostile'), down: 0, sliced: false, thrower: null, tossed: 0, nextT: 0, aim: null, from: null, banged: null, handed: false, losT: 0, litC: null, hold: !!op.thrHold, told: false, force: false, first: first ? 'can' : null, out: first ? out : null };
    if (pods.length) this.searchOf(op, pods[0].pos);
    op.phase = first ? 'pocket' : 'thr'; op.t0 = t; if (first) op.thr.nextT = t + (op.kick && !isNaN(op.kick.t) ? K.tossAfterKick : K.tossAfterOpen);   // (a can first goes in on the shove or the boot, as a bang and clear's does: every tenth of a second the leaf stands open before it is one a settled gun in there has on the jambs)
    this.beat(op, `threshold · ${hi(lead) ? 'latch' : 'hinge'} jamb ${lead.callsign} ${K.thrSlice || K.thrPie ? 'slices' : 'holds'} · ${far ? `${hi(far) ? 'latch' : 'hinge'} jamb ${far.callsign} ${K.thrPie && !K.thrPieOne ? 'slices' : 'holds'}` : op.lastStack ? 'the other jamb not waited for' : 'no floor at the other jamb'}${op.rear.size ? ` · rear ${[...op.rear].map(m => m.callsign).join(', ')}` : ''}${op.thrHold ? ' · to hold' : ''}${first ? ' · a can first' : ''}`);
    g.say(lead, 'working it from the door.');
  }
  /** DOORTOOLS.thrFirst: does this door get a can before anybody shows in its opening — the kit has one, the stage may spend it, no teammate is in there,
   *  the order is not on a word or a gate (those keep their own count), and the floor is hot in the ears of a man of the order (or the knob says every door) */
  private canFirst(op: DoorOp): boolean {
    const K = DOORTOOLS; if (K.thrFirst <= 0 || K.thrBangs <= 0 || this.bangs <= 0 || op.word || op.onWord || op.worded || op.gate || op.thrHold) return false;
    if (silentNoCans(this.g)) return false;   // (silent.ts: nobody asked for this can)
    const g = this.g, d = op.door; if (!(K.thrFirst >= 2 || op.men.some(m => m.state !== 'dead' && g.fight.hotFor(m)))) return false;
    // (nobody asked for this can: one of ours beyond the door inside its reach with a line from the opening — a teammate of any squad's size, or Sam — and it stays in the kit)
    const inn = doorPt(d, d.def.width / 2, 0.45, -op.side as 1 | -1, 1.1), ours = [...g.guards.filter(x => (g.orders.sams ? x.team === 'operative' && x.unit : g.orders.mine(x)) && x.state !== 'dead' && !op.men.includes(x)).map(x => x.char.pos), ...(g.player.down ? [] : [g.orders.samStood])];
    return !ours.some(p => offOf(d, p, op.side) < 0 && flat(p, d.frameCentre) < K.stunM && !g.col.solidBlocked(inn, [p[0], 1.1, p[2]]));
  }
  /** the stage's men, kept true: a jamb man who is down or re-ordered is off it; with the lead gone the far-jamb man has the door (he holds his diagonal:
   *  nobody new starts a slice), and with both gone the order's next man turns in from the rear to hold the frame from where he stands */
  private thrMen(op: DoorOp) {
    const T = op.thr!, d = op.door; T.jamb = T.jamb.filter(m => op.men.includes(m));
    if (T.thrower && !op.men.includes(T.thrower)) T.thrower = null;
    if (T.jamb.includes(op.lead)) { if (T.jamb[0] !== op.lead) T.jamb = [op.lead, ...T.jamb.filter(m => m !== op.lead)]; return; }
    if (T.jamb.length) { op.lead = T.jamb[0]; return; }
    const gd = op.lead, p = gd.char.pos, name = doorWords(d); T.jamb = [gd]; op.rear.delete(gd);
    if (op.phase === 'enter') return;
    this.assign(op, gd, { post: [p[0], 0, p[2]], face: yawTo(p, d.frameCentre), watch: [d.frameCentre[0], 1.05, d.frameCentre[2]], stance: 'none', upper: 'aim', light: this.lights ?? NIGHTBREACH.opLights, search: !(DOORTOOLS.thrDark && nvgWorn(this.g, gd)), going: `on the door · ${name}`, there: `holding the door · ${name}` });
    this.beat(op, `the door is ${gd.callsign}'s`);
  }
  /** the cans this stage may spend on its door, the one that went in first included: `thrBangs` — or that one alone (`thrFirstOnly`) while Sam has not asked
   *  for a can since (his word, a told 'bang') */
  private thrCans(T: Threshold): number { const K = DOORTOOLS; if (!T.force && silentNoCans(this.g)) return 0;   // (a SILENT squad's cans stay in the kit, on the word too: 'bang, then step in' is the loud row — silent.ts)
    if (K.outsidePlain) return Math.min(1, K.thrBangs);   // (one can a door, on the word and when told: "the squad does waste a lot of bangs in that path")
    return T.first === 'done' && K.thrFirstOnly && !T.force && !T.asked ? Math.min(1, K.thrBangs) : K.thrBangs; }

  /** the hold is over. Both jamb men stand in behind their jambs' edges (`thrCover`: the blind burst a bang draws comes back through the opening — and it is a
   *  step from there to the frame), and the thrower — the man nearest the frame who is on neither jamb; with only the jamb men, the far one; alone, himself —
   *  comes round from the rear */
  private toPockets(op: DoorOp) {
    const g = this.g, K = DOORTOOLS, T = op.thr!, d = op.door, t = g.time, near = (m: Guard) => flat(m.char.pos, d.frameCentre);
    this.heldBeat(op);
    if (T.first === 'done' && K.thrFirstStale > 0 && t - op.bangT >= K.thrFirstStale && T.banged && T.tossed < this.thrCans(T) && this.bangs > 0) { T.banged = null; this.beat(op, `the first can is ${(t - op.bangT).toFixed(1)} s old · its floor is blind again`); }
    T.thrower = this.thrThrower(op); op.rear.delete(T.thrower); T.handT = undefined;   // (thrHand: a fresh trip to the pockets says its own wait; what the order has already waited stays counted)
    this.closeUp(op); if (K.thrCover) for (const m of T.jamb) if (m !== op.lead) this.closeUp(op, m);
    op.phase = 'pocket'; op.t0 = t; T.nextT = t + K.thrTurn;
  }
  /** the hold's own count, as it ends (the pockets, a step in told, the door left) */
  private heldBeat(op: DoorOp) {
    const T = op.thr!, SR = op.search; this.thrMen(op);
    this.beat(op, `held ${(this.g.time - op.t0).toFixed(1)} s · ${T.contact.size} in contact · ${T.down} down${SR ? ` · ${SR.left} of ${SR.pts.length} cells unseen` : ''}`);
  }
  /** who throws: the man nearest the frame who is on neither jamb; with only the jamb men, the far one; alone, himself */
  private thrThrower(op: DoorOp): Guard { return this.thrPick(op, false)!; }
  /** DOORTOOLS.thrHand: is this man a hand AT THE DOOR — within `throwReach` of the frame, on the squad's face of it */
  /** … or BACK: at the door, or on his own mark of the order */
  private thrBack(op: DoorOp, m: Guard): boolean { const A = op.mine.get(m); return this.thrAt(op, m) || (!!A?.post && flat(m.char.pos, A.post) <= 0.6); }
  /** DOORTOOLS.wordCuts: is the word's step in (no can) waiting for the jamb men's cuts — a hold whose men slice (thrPie), the cuts not yet called done
   *  ('sliced'), the rule's cap not spent since the word. Not asked under a captor's last warning, nor of a door that shares a word with another. */
  private cutsOwe(op: DoorOp): boolean {
    const K = DOORTOOLS, T = op.thr!; if (!K.wordCuts || WORDCUTS_BREAK || !K.wordNoCan || T.sliced || T.told || op.phase !== 'thr') return false;   // (`told`: they have said 'holding on the door' — the hold's own cap — and are not sent back to cuts)
    if (!T.jamb.some(m => !!op.slice.get(m)?.pie)) return false;   // (nobody on this door cuts: there is nothing to wait for)
    if (T.cutOwed && this.g.time - (T.owedT ?? -1e9) >= K.wordCutsCap) return false;
    if (this.set.some(c => !this.onPendingWord(c) && this.keyIs(c))) return false;   // (a set charge goes on this same word: they go in behind its blast, not seconds after it)
    return !op.lastStack && !op.lastHold && !this.g.rescue.lastWarning(op.door) && !(op.word && this.ops.some(o => o !== op && o.word === op.word));
  }
  /** DOORTOOLS.thrFragHold: is the stage waiting for men who ran from a grenade — some are not back, and the order's one wait (`thrHandWait`, the can's
   *  own wait counted in it) is not spent. Not asked under a captor's last warning, nor of a door that shares a word with another (as thrHand is not). */
  private thrFragWait(op: DoorOp): boolean {
    const K = DOORTOOLS, T = op.thr!;
    return K.thrFragHold && !!T.fled?.size && !T.handDone && (T.handW ?? 0) < K.thrHandWait && !op.lastStack && !op.lastHold && !this.g.rescue.lastWarning(op.door) && !(op.word && this.ops.some(o => o !== op && o.word === op.word));
  }
  /** DOORTOOLS.fragHold: is this order waiting for men who ran from a grenade — some of them not back, the stage it is in one that holds (fragHolds), its
   *  one wait (`fragHoldCap`) not spent, its signal not gone. Sam's own squad only (the response team's cast of this module is not asked). Not asked under a
   *  captor's last warning. */
  private fragWait(op: DoorOp): boolean {
    const K = DOORTOOLS;
    return K.fragHold && !!op.fled?.size && !op.fledGo && this.g.orders.sams && (op.fledW ?? 0) < K.fragHoldCap && this.fragHolds(op) && !op.lastStack && !op.lastHold && !this.g.rescue.lastWarning(op.door);
  }
  /** … the orders on a word that has been given and is held for men who ran, where these men are on one of its doors (every door of such a word): the word
   *  given AGAIN lets them all go (goMen), and the 'go' row is not greyed for it */
  private fragHeld(men: Guard[]): DoorOp[] {
    if (!DOORTOOLS.fragHold) return [];
    const held = this.words.filter(w => isNaN(w.go1) && this.ops.some(o => o.word === w && o.men.some(m => men.includes(m))) && this.ops.some(o => o.word === w && this.fragWait(o)));
    return held.length ? this.ops.filter(o => !!o.word && held.includes(o.word)) : [];
  }
  /** … the stages that hold, each while its door is still to be opened: the walk to the stack and its call (on one signal for several doors: once the
   *  signal is raised) · a stack that is ready, once it is no longer waiting for Sam's word (the word given, or given up on) · the leaf itself — a boot,
   *  shove or tool not yet begun (a hand on a pick stays on it) · a charge whose own man ran · 'stood off' · a set charge on a word that has been given */
  private fragHolds(op: DoorOp): boolean {
    const k = op.kick;
    switch (op.phase) {
      case 'stack': return !op.gate || op.gate.go;
      case 'ready': return !op.onWord;
      case 'standoff': return !!this.chargeOn(op.door);   // (the charge gone from the door — somebody opened it — is the order's own end, not waited on)
      case 'place': return !!op.fled?.has(op.lead) && !isOpen(op.door);
      case 'armed': return !!op.word && !!this.chargeOn(op.door);
      case 'kick': return !!k && isNaN(k.t) && !k.by && !(k.gun && (k.gun.n > 0 || k.gun.nextT !== 0)) && !isOpen(op.door);   // (a shell gone or a hammer wound up is begun)
      default: return false;
    }
  }
  /** … the wait that was said is over: said so once, where men are still out ('on without …') */
  private fragSaid(op: DoorOp) {
    if (!op.fledSaid) return; op.fledSaid = false; const F = op.fled;
    if (F?.size) this.beat(op, `on without ${[...F].map(m => m.callsign).join(', ')} · ${(op.fledW ?? 0) >= DOORTOOLS.fragHoldCap ? "the order's wait is spent" : 'not waited for'}`);
  }
  /** … the order's signal has gone (its word's first sound, the signal several doors share, Sam's word again): nothing of it waits from here */
  private fragOn(op: DoorOp) { this.fragSaid(op); op.fledGo = true; }
  private thrAt(op: DoorOp, m: Guard): boolean { return flat(m.char.pos, op.door.frameCentre) < DOORTOOLS.throwReach && offOf(op.door, m.char.pos, op.side) > 0; }
  /** the one order of men a can is thrown by. `hand` false: the stage's pick as it always was (thrThrower: never null). `hand` true (DOORTOOLS.thrHand): the
   *  hand a can may leave RIGHT NOW, or null — the same order, every man of it asked to be at the door (thrAt), and the lead's only with no other man of the
   *  order still on his way back (neither at the door nor on the mark the order gave him: a rear man standing on a mark down the wall is waited for by nobody).
   *  `fit`: a further test every man must pass (the gate's own: his end of the doorway has a pocket for the can) */
  private thrPick(op: DoorOp, hand: boolean, fit: (m: Guard) => boolean = () => true): Guard | null {
    const K = DOORTOOLS, T = op.thr!, d = op.door, near = (m: Guard) => flat(m.char.pos, d.frameCentre);
    const ok = (m: Guard) => hand ? this.thrAt(op, m) : !op.lastStack || near(m) < K.throwReach;   // (a stack that went without its far-jamb man: he throws nothing from down the wall — the lead's own hand)
    const back = (m: Guard) => this.thrBack(op, m);
    return op.men.filter(m => !T.jamb.includes(m) && this.thrAt(op, m)).sort((a, b) => near(a) - near(b)).find(fit) ?? T.jamb.find(m => m !== op.lead && ok(m) && fit(m))
      ?? (!hand || (this.thrAt(op, op.lead) && fit(op.lead) && op.men.every(m => m === op.lead || back(m))) ? op.lead : null);
  }
  /** is this cell of the room lit by the place itself — floor a man with no light of his own can see a man standing on (asked once a cell an order) */
  private thrLitCell(T: Threshold, SR: RoomSearch, i: number): boolean {
    const L = T.litC ??= new Int8Array(SR.pts.length).fill(-1), q = SR.pts[i];
    if (L[i] < 0) L[i] = this.g.visibilityOf(lightEstimateAt(this.g, [q[0], 1.2, q[2]], () => true)) >= LIGHTS.litAt ? 1 : 0;
    return L[i] === 1;
  }
  /** the SHALLOW points a threshold stage steps in to, by the door's own geometry (never a level's authored points of domination: those can be deep): first
   *  the free floor to either hand of the doorway hard on the entry wall, clear of the opening — the jamb men's — then one a little deeper to either hand, then
   *  on down the middle: none more than 2.5 m inside. A doorway with no floor like that inside gets the entry's own points (pods). */
  shallowPods(d: Door, side: 1 | -1, n = 2): { pos: Vec3; face: number }[] {
    const g = this.g, nav = g.col.nav, W = d.def.width, into = -side as 1 | -1, mid = doorPt(d, W / 2, 0.9, into, 0.45), out: { pos: Vec3; face: number }[] = [];
    const fits = (p: Vec3) => !nav.isBlocked(p[0], p[2]) && !g.col.solidBlocked(mid, [p[0], 0.45, p[2]]) && !out.some(o => flat(o.pos, p) < 0.9) && !this.inFrame(d, mid, p);
    for (const s of [1, -1]) find: for (const D of [1.0, 1.4, 0.8, 1.9]) for (const L of [W / 2 + 0.9, W / 2 + 1.4, W / 2 + 0.55, W / 2 + 2.0]) {   // (L past the jamb: never in the opening itself)
      const p = doorPt(d, W / 2 + s * L, D, into); if (!fits(p)) continue;
      out.push({ pos: p, face: yawTo(p, doorPt(d, W / 2 - s * 0.6, 4.5, into)) }); break find; }
    for (const s of [1, -1]) { if (out.length >= n) break;
      find: for (const D of [2.2, 1.7, 2.5]) for (const L of [1.7, 1.3, 2.3, 0.9]) {
        const p = doorPt(d, W / 2 + s * L, D, into); if (!fits(p)) continue;
        out.push({ pos: p, face: yawTo(p, doorPt(d, W / 2 - s * 1.2, 5, into)) }); break find; } }
    const inward = yawTo(doorPt(d, W / 2, 0, into), doorPt(d, W / 2, 1, into));
    for (const b of DOORTOOLS.podFrames ? [1.5, 2.4, 2.5] : [1.5, 2.4]) { if (out.length >= n) break; const q = doorPt(d, W / 2, b, into), f = nav.isBlocked(q[0], q[2]) ? nav.nearestFreePoint(q[0], q[2]) : q; if (f && -offOf(d, [f[0], 0, f[2]], side) > 0.3 && -offOf(d, [f[0], 0, f[2]], side) <= 2.5 && !out.some(o => flat(o.pos, f) < 0.9)) out.push({ pos: [f[0], 0, f[2]], face: inward }); }
    if (!out.length) return this.pods(d, side, n);
    const pts = out.slice(0, n); for (const P of pts) this.offSwing(P, d, side, pts.filter(o => o !== P).map(o => o.pos));
    return pts;
  }
  /** THE STEP IN: the men off the jambs first — the lead, the far-jamb man `second` behind him — each to the shallow point that is the least walk, then the
   *  rest in their turn to the points behind those (a man there is no shallow floor for keeps the squad's back, outside, and is not this order's any more).
   *  From here the op is in 'enter': the entry's own corners, search and sectors — the hand-over to the room clear. */
  private stepIn(op: DoorOp) {
    const g = this.g, S = g.orders, K = DOORTOOLS, T = op.thr!, d = op.door, t = g.time, near = (m: Guard) => flat(m.char.pos, d.frameCentre);
    this.thrMen(op);
    if (K.leadSeen && T.jamb.length > 1) { const f = this.seenMost(op.lead, gunFirst(T.jamb.filter(m => m.state !== 'dead' && (m !== op.threw || m === op.lead) && (m === op.lead || !op.lastStack || near(m) < K.throwReach))), T.pods[0]?.pos);   // (the man off the jambs who has seen most of the room steps in first — not the one who threw the pockets' cans, nor, on a last warning's stack, a man still walking up)
      if (f !== T.jamb[0]) { T.jamb = [f, ...T.jamb.filter(m => m !== f)]; this.led.bySeen++; this.beat(op, `first:${f.callsign} · has seen most of the room`); } }
    this.led.entries++;
    const pods = S.big && op.men.length <= 2 ? this.podsApart(op, T.pods, op.men.length) : T.pods;
    if (!pods.length) { g.say(op.lead, 'no way in.'); this.end(op, 'no floor inside'); return; }
    const turn = [...T.jamb, ...op.men.filter(m => !T.jamb.includes(m)).sort((a, b) => near(a) - near(b))], nIn = Math.min(pods.length, Math.max(1, K.thrIn)), inMen = turn.slice(0, nIn), stay = turn.slice(nIn);
    const deal = new Map<Guard, { pos: Vec3; face: number }>();
    for (const k of [0, 2]) { const m = inMen.slice(k, k + 2), sp = pods.slice(k, k + 2); if (m.length) for (const [gd, pos] of S.pair(m, sp.map(x => x.pos))) deal.set(gd, sp.find(x => x.pos === pos)!); }
    if (stay.length) { op.men = op.men.filter(m => !stay.includes(m)); this.beat(op, `outside · ${stay.map(m => m.callsign).join(', ')}`); }
    op.rear.clear();
    op.queue = inMen.map((gd, i) => ({ gd, at: t + i * K.second, pod: deal.get(gd) ?? pods[0] }));
    op.phase = 'enter'; op.t0 = t; this.beat(op, `step in · ${inMen.map(m => m.callsign).join(', ')}`);
    g.say(op.lead, 'stepping in.');
  }

  // ------------------------------------------------------------ the HUD, the console
  summary(): string {
    return `charges ${this.charges} · bangs ${this.bangs} · set ${this.set.map(c => c.door.def.name).join(',') || 'none'} · ${this.ops.map(o => `${o.kind} ${o.door.def.name}: ${o.phase} (${o.men.map(m => m.callsign).join(', ')})`).join(' · ') || 'nothing under way'}`;
  }
}
