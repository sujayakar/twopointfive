// Hinged doors: leaves that characters push open by walking into them — and, one-way (DOORWAY, below), pull open from the side they open into — with inertia, limit stops,
// a slow hydraulic closer and a latch. Rendered/traced as thin dynamic boxes, so light spills through the gap as
// they swing (radiance cascades pick the change up over a couple of frames). Also used as occluders for guard
// vision, hearing and audio propagation.
import { Vec3, clamp } from '../math/vec';
import { Box, BoxFlag, makeBox, MKEY } from '../scene/boxes';
import { doorLeafMarks } from '../scene/markingDefaults';
import { LEAF_H } from '../scene/doorSize';
import { leafWrote, type BoxMirror } from '../scene/boxMirror';
import { type StaticCollision } from '../scene/collision';
import '../math/exact';
import { DoorStore, DC, DO, DOOR_CAP, DRIVE_KINDS, doorIo, doorMem } from '../sim/doorshome';
import { SimState } from '../sim/state';
import { SlabStore, SLF, type SLAB_F64, type SlabNow } from '../sim/slabshome';
import { RUST_PASS, RUST_SURFACE, crossToRust, goneWithTick } from './crossing';

/** ONE-WAY LEAVES (the maintainer, 10-08: "doors could use some work too. only open one direction"). Every door has a hinge jamb, a handle jamb and ONE side it opens
 *  into (Door.swing): its leaf turns through angles of that sign only, 0 (shut, hard on its frame) to its open stop — whoever opens it, from whichever face.
 *  From the far face it is PUSHED as ever; from the face it opens into it is PULLED: a man walking into it there (or using it, easing it, booting it, shoving
 *  it by a drill's own write of Door.vel) brings it round toward himself, and for PULL_SECS the swinging leaf is not felt by the living (it passes them, as a
 *  leaf a keyed man pulls shut passes him — `passers`). Nothing swings it through its frame: a push at the shut leaf from the side it opens into is a pull, a
 *  leaf coming to from open stops on the frame, an angle or a velocity written the wrong way by anybody is turned round (Door.keep / Doors.update).
 *  `oneWay: false` (the page's ?twoway) = the double-acting leaves as they were, instruction for instruction: swing is still dealt, nothing reads it.
 *  THE FRAME HOLDS (`frameHolds`; the maintainer, 10-08: "for doors just make it realistic"). A leaf shut in its frame rests on the frame's stop all the way round on
 *  the face it opens toward: a boot there loads the whole frame, not the keep, and the door does not open (Doors.bootHolds / kickIn — a loud thud, nothing
 *  broken); what does open it from that face is its lock worked or destroyed (a pick, a key, a charge) and then a hand. The same fact from the inside: a wedge
 *  or a dragged desk holds only a door that swings toward the man who put it there (digin.ts). Only with oneWay on, never while a showcase has the stage
 *  (Doors.frameRule); `frameHolds: false` (the page's ?frameholds=0) = a boot at the pull face flings the leaf round toward the kicker and a wedge holds
 *  any door, as they did. */
export const DOORWAY = { oneWay: true,
  /** FIRE EXITS AND OUTSIDE DOORS OPEN OUTWARD (the maintainer, 10-08: "for doors just make it realistic"): a door the level marks `exterior` with a roof over one face
   *  only opens toward the open air (doorswing.ts 'outside' — Door.outward), whatever room it serves; an authored `swing` and a folding stop still speak first.
   *  Read live (Door.swing): false = every door the side the room rules deal it, as before the switch. Nothing reads it with oneWay off.
   *  SHIPS OFF (the doors goal's call, 10-08: a paired look on the barracks' first seeds lost teammates with it on, and nobody has played it): the page's
   *  ?exitsout, the Tab row or `--set '{"DOORWAY":{"exitsOut":true}}'` turns it on. */
  exitsOut: false,
  /** THE FRAME HOLDS: see the head comment. false (?frameholds=0) = a boot at the pull face flings the leaf, a wedge holds any door. */
  frameHolds: true,
  /** A BOOT FROM BESIDE THE DOOR (the maintainer, 10-11: "shouldn't it be possible to kick the door standing on its side?"): a kick may be thrown from beside the
   *  latch jamb, behind the wall, at the lock — Doors.kickSpotSide — so the kicker is never in the opening when the leaf goes. A squad's lead boots from there
   *  wherever the floor allows; the player does when he stands beside the latch jamb (square in front of the leaf it is the square kick, as ever). Off in the
   *  module and on the frozen stand (TPS_SIDEKICK=1), on in the page (?sidekick=0): off = every kick from Doors.kickSpot, as it was. */
  sideKick: false,
  /** … NEVER FROM IN FRONT OF GLASS (the maintainer, 10-13, of a lead booting from beside a latch jamb with a sidelight behind him, lit and shot through it:
   *  "yeah maybe this should force kicking from the front?"): where the spot beside the latch jamb is one a round reaches through a pane from the floor beyond
   *  the door (orders.ts glassEyes — the stack's own 'glass is not cover' question, asked of the kicker's spot: GLASSPEN.stackEyes lines or more), the squad's
   *  lead boots square in front of the leaf, as every kick was before sideKick — where there is floor there and clearly fewer such lines reach it (at most
   *  GLASSPEN.stackLeastK of them: glazing along the whole wall looks onto both, and beside the jamb he is at least out of the opening). The player's own kick is thrown from where he chose to stand, as it was. Nothing to ask with sideKick off or while
   *  rounds stop in glass. Off in the module and on the frozen stand (TPS_KICKGLASS=1), on in the page (?kickglass=0): off = the side spot wherever there is floor. */
  kickGlass: false,
  /** A LEAF PULLED OPEN IS SOLID TO THE PLAYER (the maintainer, 10-22, having walked through one: "it's very easy to clip through a door when walking towards it
   *  and it swings out towards you, player should have to open it and walk aroudn it"). A leaf brought round toward the men on the face it opens into has always
   *  passed the living there (PULL_SECS, `passers`): a man of the AI is at the handle and his walk is planned round the sweep, but the player walks straight at it,
   *  the press pulls it after PRESS_SECS and it came round through him. With this the swinging leaf moves HIM instead (DoorContact.solid: game.ts sets it on the
   *  player alone): clear of its face and along it toward its free end (SOLID_SLIDE: a step back and to the latch side), where the walls let him stand
   *  (Doors.settle); where they do not, it stops on him and rests there, and he walks out round its end. SOLID_TRIES such stops close together and it passes
   *  him as it always did (Door.solidT): never a door he cannot open from a cupboard. And so that a walk at a door is not a shove off it every time, HIS HAND
   *  REACHES IT BEFORE HIS BODY: walking at that face (SOLID_INTO m/s into it, on a line that meets the leaf) he takes it SOLID_REACH short of his chest and brings
   *  it round ahead of himself — its edge passes in front of him and he goes round it without breaking step; at a run the run's own rules stand (the boot, the rip,
   *  the haul), and a leaf hauled round faster than a hand's pull is slowed to one by the body it meets. Nobody else's contact changes. Off in the module and on the frozen stand
   *  (TPS_DOORSOLID=1), on in the page (?doorsolid=0): off = a pulled leaf passes him, as it did. */
  solid: false,
  /** A DOOR OPENS THE WAY ITS LEAF LEAVES ROOM (the maintainer, 10-27, of his four on the compound: "feels like squad gets stuck when stacking up or going
   *  through doors more now"). The compound's spare room door was dealt INTO the room by the floor rule, and the room's cot stands a metre inside it: the open
   *  leaf hung across the one strip between cot and wall, the room behind it could not be walked to the doorway from, and four men queued behind a leaf
   *  that turned shut under the first and was pulled open onto the second. With this a door whose open leaf shuts floor off from its own doorway on the side
   *  the rules deal it (doorswing.ts leafCut: LEAF_CUT m² or more, for a man's width, square open AND at the angle a leaf rests at) is hung to open the OTHER
   *  way where the leaf has room to swing and leaves the way clear (Door.clearway, 'leaf room'). Read live (Door.swing), like exitsOut, which speaks first; an authored `swing`, a
   *  folding stop, a drill plan's fling and a leaf authored standing open are never turned. What it costs: from the side such a door now opens toward it is
   *  pulled, and its frame holds a boot there. Nothing reads it with oneWay off. Off in the module and on the frozen stand (TPS_LEAFWAY=1), on in the page
   *  (?leafway=0): off = every door the side the room rules deal it. */
  leafWay: false };
try { const q = new URLSearchParams(location.search); if (q.has('oneway')) DOORWAY.oneWay = q.get('oneway') !== '0'; if (q.has('twoway')) DOORWAY.oneWay = false;
  if (q.has('exitsout')) DOORWAY.exitsOut = q.get('exitsout') !== '0'; if (q.has('exitsin')) DOORWAY.exitsOut = false;
  if (q.has('frameholds')) DOORWAY.frameHolds = q.get('frameholds') !== '0'; } catch { /* no page (the harness) */ }
/** the side kick's stand: metres past the latch edge along the wall, metres off the frame line (0.5 and 0.55 across = the kick's 0.74 m reach to the lock) */
export const SIDE_KICK = { past: 0.3, out: 0.55 };
/** …after he has pressed on it this long (it does not give that way: a beat of the hand finding the handle) — and never a leaf somebody is swinging INTO him: one
 *  pushed, shoved or booted open inside the last DRIVEN_SECS, or still coming round faster than DRIVEN_VEL, meets him as it always did (it stops against him) */
const DRIVEN_SECS = 0.25;
/** a leaf standing nearer its frame than this is home in it: its far face is hard on the stop (DOORWAY.frameHolds — a boot from the face it opens toward) */
const SHUT_HOME = 0.05;

export interface DoorDef {
  name: string;
  hinge: [number, number];        // x, z of the hinge axis
  closedDir: number;               // world angle of the closed leaf (0 → +X, π/2 → +Z)
  width: number;                   // leaf length from the hinge
  height?: number; thickness?: number;
  minAngle?: number; maxAngle?: number;   // swing limits relative to closed (radians)
  angle?: number;                  // initial angle
  /** the side it opens into (DOORWAY): +1 = the CCW side of the closed leaf (positive angles; Doors.side's +1), −1 the other. Absent = dealt from the level
   *  (game/doorswing.ts: into the room it serves, away from the corridor). */
  swing?: 1 | -1;
  albedo?: Vec3;
  closer?: boolean;                // hydraulic closer (default true); false = stays where it was left
  exterior?: boolean;              // opens onto the lot (level.ts): guards tidying doors in a lockdown leave it alone
  /** A keyed lever set: while the leaf is latched it will not open from the `keySide` (the corridor) to anyone without a key — guards carry keys, the player picks
   *  it (Doors.pick) or kicks it in (Doors.kickIn), or takes a key off a man he questions (player.ts takeKey) and it opens to him as it does to them. The room
   *  side is a thumb-turn: it always opens from there, so you can shut yourself in but never lock yourself in. The lock only bites once the latch has caught:
   *  a leaf still swinging to can be caught and pushed like any other. */
  locked?: boolean;
  /** which face carries the cylinder: +1 = the CCW side of the closed leaf (+perp in Doors' side convention — the corridor for every door on the z=10 wall), −1 the other */
  keySide?: 1 | -1;
  /** which key turns it (Door.lockId): absent = FLOOR_KEY, the one key every man on nights carries and every keyed door on this floor takes. A second cylinder
   *  would name its own ('server', say) and want a holder (level.ts PatrolRoute.keys). */
  lock?: string;
  /** a steel fire door in a steel frame (the west stair door — game/qrf.ts): a boot does nothing to it but make a noise (kickIn: no broken keep, no swing, a clang
   *  the men nearby come to look at), and its corridor face has no keyway to pick (player.ts offers no 'pick' row). It opens like any door to a key that fits,
   *  and always from its far side. */
  steel?: boolean;
}
/** the lock every keyed door takes unless its def says otherwise, and the key every guard carries unless his route says otherwise (game.ts Guard.keys) */
export const FLOOR_KEY = 'floor';

/** DOORS STAY WHERE THEY ARE LEFT (the shipped default; `on: false` = every leaf as it was, by run digest): no leaf has a closer, whatever its def says — an opened
 *  door stands at the angle it was left until a hand (use / crack / pullTo — the player's row, a staff man shutting his own keyed door behind him or tidying in a
 *  lockdown (guards.ts lockdownDoors), a man digging in), a body walking
 *  into it, a boot or a blast moves it. The lock of a keyed door therefore bites again only once somebody SHUTS it (it never springs locked behind a key holder),
 *  and a keyed man pinned by a leaf on its stop, or snagged on its free edge, no longer pulls it to behind him (the old `yank`): he steps through its edge
 *  (Door.passers) and the leaf stays where it stood. A close somebody asked for (Door.closing: pullTo) is still the sprung pull the latch catches.
 *  NOBODY SHUTS A DOOR BY WALKING INTO IT WITHOUT MEANING TO (Door.swept): a leaf a keyed man's BODY alone has brought round to its frame — herded shut ahead of
 *  him as he walks the wall behind it — rests to there, UNLATCHED: no click, and whoever opens it next gets the creak a shut door gives. The latch catches what
 *  a hand shut (pullTo), what the player leaned shut himself, what a man ran into, whatever a boot, a blast or a script sent home. A keyed door is no
 *  exception: its bolt bites once somebody SHUTS it (above), and a man brushing past it has not — a teammate's shoulder never locks Sam out.
 *  NOT while a showcase has the stage (Doors.staged ← Game.touring): those films were choreographed against the closers, beat for beat, and run with them. */
export const DOORSTAY = { on: true };

/** A DOOR OPENED INTO A MAN MOVES HIM AND STARTLES HIM; A DOOR KICKED INTO HIM THROWS HIM (the maintainer: 'opening a door into the enemy should majorly
 *  confuse them' · 'kicking down a door when an enemy on the side should stun or knock them out' · having read the first cut, where a pushed door folded a
 *  man for two seconds: 'opening a door into an enemy standing on the other side should just alert him, it's not like we're slamming these doors' ·
 *  'confusion here being put on alert but being very confused and investigating what's going on'). A leaf DRIVEN OPEN — walked open by a man on its other
 *  face, turned by a hand, pushed, shoved or flung by a door order, shouldered at a run, booted — used to stop dead against whoever stood in its swing, and
 *  he held it shut on the man opening it. Now it wins: he is carried clear ahead of it and along it toward its free end (Doors.update), and what it did to
 *  him is handed out on his contact (DoorContact.struck) for doorthrow.ts to make of it, by HOW the leaf was driven —
 *    OPENED (walked, handed, pushed, shoved, flung)   he is moved aside and no more. A defender is startled for `startle` s and goes to see what is at
 *                                                     the door (suspicious, the doorway his place to look): no fold, no blindness.
 *    SLAMMED (a leaf in a kicked flight that is no     folded and blind for `pushSecs`. The tier is the LEAF's (Door.sinceBash), never whoever last wrote
 *      boot's: a shoulder at a run, a door ripped       the drive: the runner's own body pressing after his shoulder is the same slam.
 *      or flung open at that pace)
 *    BOOTED (kickIn: a boot, the ram, a shell)        folded and blind for `kickSecs`, and on a knee if it met him before it had turned `floorRad` from
 *                                                     where it was booted (he stood right at the door)
 *  A drive strikes a man ONCE (DoorDrive.hit); after that it goes on carrying him.
 *  By behaviour, never by hit points: nothing here hurts anybody. The same for whoever it meets — a defender, Sam, a teammate, a civilian.
 *  NOTHING AT ALL: a leaf eased (a hand slower than `handRate`, a crouched or silent man's body — DoorContact.quiet — or any leaf slower than `minSpeed` where
 *  it meets him), pulled (it passes the living on that side as ever), swinging shut, or one the man walked into himself; a leaf opened short of a boot by a
 *  CALM man (MAN_CALM: staff not alert or searching, any civilian — he opens a door with his hand at a walk: the patrol's round, the cleaner's, move nobody
 *  and raise no alarm of their own, and a man hidden behind a door stays hidden); a leaf a man of HIS OWN side opened any way short of a boot (the squad's
 *  file through a door, a pair's shove into a room the other pair is already in, are not a brawl — a leaf one of them BOOTS meets him like anyone); a man
 *  already folded (he is carried, whoever's leaf it is and however quietly it comes: a folded man holds no door shut) or just up from it (`again` s: no
 *  second throw — an opened leaf moves and startles him like anyone); a man in somebody's arm; a showcase on the stage. A man with no floor to be moved onto (his back
 *  to a wall) is struck where he stands and the leaf rests on him.
 *  `on`: ON in the page (src/host/profile.ts; ?doorthrow=0 = every leaf stops against a standing man, as it did), OFF here and on the harness's frozen stand
 *  (TPS_DOORTHROW=1). */
export const DOORTHROW = { on: false,
  /** s folded and blind: a leaf shouldered open at a run into him · a booted one */
  pushSecs: 2.0, kickSecs: 4.0,
  /** a leaf merely OPENED into a defender: s he stands startled before he goes to look · the awareness he is left with (suspicious: the decoy's own) */
  startle: 1.0, wary: 0.5,
  /** an opened leaf only startles (false = the first cut, which a recording made on it replays: any driven leaf short of a boot folds him for `pushSecs`) */
  soft: true,
  /** the startled defender gets the shared start (startle.ts giveStartle: the jolt toward the door, his gun down for the beat; false = the animator's flinch,
   *  which a recording from before the shared start replays) */
  jolt: true,
  /** m/s a swinging leaf must still be doing where it meets him (slower, it comes to rest against him as ever) */
  minSpeed: 0.9,
  /** rad/s a hand must turn a leaf at for it to be a push (the silent ease is 0.55; the hand's own push 2.6) */
  handRate: 1.2,
  /** rad a booted leaf may have turned when it meets him for him to have stood right at the door: he goes onto a knee for it */
  floorRad: 0.6,
  /** m/s a leaf carries a thrown man ahead of it at most (a frame's turn beyond that and it rests on him this frame) · m/s he is moved along it toward its free end */
  rate: 9, slide: 1.5,
  /** s after a fold has run out in which no leaf throws him again */
  again: 3.0,
  /** the awareness a man comes back with when the leaf was a foe's (the ring's own: alert) */
  aware: 0.92,
};
/** who last DROVE a leaf open and how (DOORTHROW; Door.drive): `dir` the sign of the turn, `kind` a boot (kickIn) · any other swing
 *  written onto it (a push, a shove, a fling) · a body walking it open · a hand turning it; `t` seconds since; `from` the angle it stood at; `quiet` a crouched, silent or
 *  calm man's; `side` whose man he was when he drove it (MAN_SIDE; 0 nobody's); `rate` rad/s a hand was turning it at, or a body would turn it at by its own
 *  stride — where he stood when the last update ended (`px`, `pz`) to where he pressed it from, over his lever: never the leaf's own turn, which two men
 *  pressing one leaf from both faces see-saw up to anything; `hit` the man it last struck (−1 nobody; a new drive starts with nobody): it strikes him once, for `again` s after
 *  the drive it goes on carrying him ahead of it, and he takes no handle on it */
export interface DoorDrive { who: number; dir: 1 | -1; kind: 'boot' | 'swing' | 'body' | 'hand'; t: number; from: number; quiet: boolean; side: number; rate: number; px: number; pz: number; hit: number; }
/** what Doors.man answers about a man (DOORTHROW): whose he is in the low two bits (1 Sam and his squad · 2 the defenders · 3 a civilian · 0 nobody's) | is he
 *  folded right now (he holds no leaf: one driven toward him carries him) | braced (just up from a fold, or in somebody's arm: no throw) | calm (above) |
 *  held (in somebody's arm: no leaf moves him) */
export const MAN_SIDE = 3, MAN_FOLDED = 4, MAN_BRACED = 8, MAN_CALM = 16, MAN_HELD = 32;
/** what a driven leaf did to the man it met (DOORTHROW; DoorContact.struck — once a drive): whose leaf, merely opened · slammed · booted, did he stand
 *  right at a booted one, and the way he was moved (x, z) */
export interface DoorStrike { door: Door; by: number; tier: 'open' | 'slam' | 'boot'; floor: boolean; dir: [number, number]; }

/** weak: a limp thing (a corpse's particle) — it never unlatches or bashes a leaf, and a leaf that someone stronger is pushing this frame, or a latched one,
 *  pushes IT (pos is corrected) instead of yielding; otherwise the leaf rests against it like against anyone (the closer cannot shut a door on a body).
 *  key: staff (guards) — every locked leaf unlatches for them like an unlocked one, and they handle leaves the way staff do (pull one to behind them, take the
 *  blame for what it does, are let through one a colleague is pulling shut). keys: the keys a keyless body carries all the same (the player, off a man he
 *  questioned — Player.keys): a locked leaf whose lock one of them fits (Door.lockId) is no wall to him; nothing else about how leaves treat him changes.
 *  blockedBy (out): the locked leaf that stopped this contact this frame. */
export interface DoorContact { pos: Vec3; radius: number; bash: boolean; who: number; quiet: boolean; weak?: boolean; key?: boolean; keys?: ReadonlySet<string>; blockedBy?: Door | null;
  /** a man of the AI who opens doors by the HANDLE (handleside.ts — game.ts sets it): his body against a leaf on the face it opens into is not his hand on it — the
   *  leaf is a wall to him, as it is to anyone for PRESS_SECS, until he has leaned on it this many seconds (wedged against it with no other way: then as before) */
  hand?: number;
  /** a man who walks ROUND a leaf standing open across his way (doorway.ts DOORMANNERS.leafRound — game.ts sets it for Sam's squad): pressed on its back at a
   *  walk he does not turn it toward shut, he goes along it and round its free end (ROUND_*) */
  round?: boolean;
  /** a RUN at a door that has weight (sprintdoor.ts — game.ts sets it on a sprinting player while SPRINTDOOR is live; absent = every contact as it was): the
   *  rad/s a leaf leaves his shoulder at when he runs it open from its push face, and the rad/s + loudness of one he rips round by the handle from the face it
   *  opens toward (instead of the 7.5 haul at a bash's full noise). bashed / ripped (out): the leaf he did that to this frame (the last of each, where two
   *  leaves meet him in one frame: one cost a frame, never two). */
  run?: { bash: number; rip: number; ripLevel: number }; bashed?: Door | null; ripped?: Door | null;
  /** (out, with bashed) the angle the leaf stood at when his shoulder met it: what `square on` is measured against */
  bashedAt?: number;
  /** DOORTHROW — struck (out): the driven leaf that threw him this frame · moved (out): a leaf carried him this frame (his place is the caller's to settle
   *  against the walls) · man: Doors.man's answer about him, asked once a frame and only when he is at a leaf */
  struck?: DoorStrike | null; moved?: boolean; man?: number;
  /** DOORWAY.solid (game.ts sets it on the player while the switch is live): a leaf being pulled open does not pass him — it moves him, or stops on him */
  solid?: boolean;
  /** his own speed over the floor, m/s (game.ts, with `solid`): what a walk AT a leaf is read from, and what he is slid along a leaf against */
  vel?: Vec3; }
/** does this contact's key ring get it through d's lock: staff always, anyone else by a key that fits */
const opens = (c: DoorContact, d: Door) => !!c.key || (!!c.keys && c.keys.has(d.lockId));
/** rattle: the lever tried against a thrown bolt · pick: one click of the pick working · unlock: the cylinder going over and the bolt drawing · kick: a boot through the lock */
export type DoorSound = 'creak' | 'bang' | 'latch' | 'bash' | 'rattle' | 'pick' | 'unlock' | 'kick' | 'clang' | 'thud';   // clang: a boot on a steel door that does not give (DoorDef.steel) · thud: a boot at the face a shut door opens toward — the frame takes it (DOORWAY.frameHolds)
export interface DoorEvent { door: Door; sound: DoorSound; pos: Vec3; who: number; level: number; }
/** A leaf OFF ITS HINGES (doorblast.ts — a blast tore it out of its frame, or gunfire took it apart: DOORSHOT): the loose slab. `p` its centre, `v` its velocity, `a` the world angle of its width axis
 *  (what closedDir + angle was when it went, turned since by `spin`), `tip` 0 (upright) ‥ π/2 (flat on the floor) with its top fallen toward `side` of that axis
 *  (+1 = the CCW normal (−sin a, cos a)), `rest` once it lies still — from then on its box is an ordinary flat yaw-only box and nothing steps it. */
export type DoorSlab = SlabNow;   // (sim/slabshome.ts: its numbers are a row of the slabs' columns; this is the row read out, and what is written into one)
/** …as `Door.slab` hands it out: read-only by its type. A slab is changed by setting `Door.slab` whole (doorblast.ts, which also steps the ones in
 *  the air over the columns). */
export interface SlabSeen { readonly p: readonly [number, number, number]; readonly v: readonly [number, number, number]; readonly a: number; readonly spin: number; readonly tip: number; readonly tipV: number; readonly side: 1 | -1; readonly rest: boolean; readonly landed: boolean; readonly who: number; readonly t: number; /** the angle the door READS as while off (±π/2: wide open, the way the leaf left) */ readonly ang: number; /** doorblast.ts DOORSHOT: gunfire took the leaf apart and this is its lock rail, lying — half width, half length, and where on it the handles are (absent: the whole leaf) */ readonly bits?: readonly [number, number, number, number]; readonly wedged?: number }
/** a door's slab as its row holds it (Door.slab while there is one): every field read from the row as it stands NOW, so one of these held across a
 *  step is the slab after the step. Good only while the door has a slab (`Door.slab` answers null when it has none). `p`, `v` and `bits` are made anew
 *  at each read: take them once. */
class SlabRow implements SlabSeen {
  constructor(private readonly T: SlabStore, private readonly door: number) {}
  private g(k: (typeof SLAB_F64)[number]): number { const T = this.T; return T.d[T.b8 + T.O[k] + T.rowOf(this.door)]; }
  private f(): number { const T = this.T; return T.u[T.b4 + T.O.flags + T.rowOf(this.door)]; }
  get p(): [number, number, number] { return [this.g('px'), this.g('py'), this.g('pz')]; }
  get v(): [number, number, number] { return [this.g('vx'), this.g('vy'), this.g('vz')]; }
  get a(): number { return this.g('a'); }
  get spin(): number { return this.g('spin'); }
  get tip(): number { return this.g('tip'); }
  get tipV(): number { return this.g('tipV'); }
  get t(): number { return this.g('t'); }
  get ang(): number { return this.g('ang'); }
  get wedged(): number { return this.g('wedged'); }
  get side(): 1 | -1 { return this.f() & SLF.back ? -1 : 1; }
  get rest(): boolean { return (this.f() & SLF.rest) !== 0; }
  get landed(): boolean { return (this.f() & SLF.landed) !== 0; }
  get who(): number { const T = this.T; return T.u[T.b4 + T.O.who + T.rowOf(this.door)] | 0; }
  get bits(): [number, number, number, number] | undefined { return this.f() & SLF.bits ? [this.g('bw'), this.g('bl'), this.g('bx'), this.g('by')] : undefined; }
}

/** a door's drive as its columns hold it (Door.drive while there is one): the DoorDrive the rules read and write, field for field */
class DriveCols implements DoorDrive {
  constructor(private readonly S: DoorStore, private readonly i: number) {}
  private g(K: number): number { const S = this.S; return S.d[S.base + K * S.stride + this.i]; }
  private p(K: number, v: number): void { const S = this.S; S.d[S.base + K * S.stride + this.i] = v; }
  get who(): number { return this.g(DC.driveWho); } set who(v: number) { this.p(DC.driveWho, v); }
  get dir(): 1 | -1 { return this.g(DC.driveDir) as 1 | -1; } set dir(v: 1 | -1) { this.p(DC.driveDir, v); }
  get kind(): DoorDrive['kind'] { return DRIVE_KINDS[this.g(DC.driveKind)]; } set kind(v: DoorDrive['kind']) { this.p(DC.driveKind, DRIVE_KINDS.indexOf(v)); }
  get t(): number { return this.g(DC.driveT); } set t(v: number) { this.p(DC.driveT, v); }
  get from(): number { return this.g(DC.driveFrom); } set from(v: number) { this.p(DC.driveFrom, v); }
  get quiet(): boolean { return this.g(DC.driveQuiet) !== 0; } set quiet(v: boolean) { this.p(DC.driveQuiet, v ? 1 : 0); }
  get side(): number { return this.g(DC.driveSide); } set side(v: number) { this.p(DC.driveSide, v); }
  get rate(): number { return this.g(DC.driveRate); } set rate(v: number) { this.p(DC.driveRate, v); }
  get px(): number { return this.g(DC.drivePx); } set px(v: number) { this.p(DC.drivePx, v); }
  get pz(): number { return this.g(DC.drivePz); } set pz(v: number) { this.p(DC.drivePz, v); }
  get hit(): number { return this.g(DC.driveHit); } set hit(v: number) { this.p(DC.driveHit, v); }
}
/** the drive's own columns: DC's names that begin `drive`, all of them but the flag (rust/sim/src/doors.rs DRIVE_COLS is the same list, held to DC by
 *  tools/qa/rust-check.ts) */
const DRIVE_COLS: readonly number[] = (Object.keys(DC) as (keyof typeof DC)[]).filter(k => /^drive[A-Z]/.test(k)).map(k => DC[k]);
/** a door's drive as a plain record (what a checkpoint file keeps: savestate.ts), or null */
export function driveOf(d: Door): DoorDrive | null { const D = d.drive; return D ? { who: D.who, dir: D.dir, kind: D.kind, t: D.t, from: D.from, quiet: D.quiet, side: D.side, rate: D.rate, px: D.px, pz: D.pz, hit: D.hit } : null; }

export class Door {
  /** THIS DOOR'S OWN NUMBERS ARE COLUMNS (sim/doorshome.ts DoorStore: a column a number, a door an index — a part of the game's block from
   *  the moment the doors are made, where the doors' step runs whichever side takes it): every field below that changes as the game runs is an accessor over them, so whoever
   *  reads or writes a door reads and writes the same values wherever they stand. `S` the columns, `i` this door's index in them. */
  readonly S: DoorStore; readonly i: number;
  private g(K: number): number { const S = this.S; return S.d[S.base + K * S.stride + this.i]; }
  private p(K: number, v: number): void { const S = this.S; S.d[S.base + K * S.stride + this.i] = v; }
  get angle(): number { return this.g(DC.angle); } set angle(v: number) { this.p(DC.angle, v); }
  get vel(): number { return this.g(DC.vel); } set vel(v: number) { this.p(DC.vel, v); }
  get quietT(): number { return this.g(DC.quietT); } set quietT(v: number) { this.p(DC.quietT, v); }
  get latched(): boolean { return this.g(DC.latched) !== 0; } set latched(v: boolean) { this.p(DC.latched, v ? 1 : 0); }
  get lastWho(): number { return this.g(DC.lastWho); } set lastWho(v: number) { this.p(DC.lastWho, v); }
  get sinceBash(): number { return this.g(DC.sinceBash); } set sinceBash(v: number) { this.p(DC.sinceBash, v); }
  get closing(): boolean { return this.g(DC.closing) !== 0; } set closing(v: boolean) { this.p(DC.closing, v ? 1 : 0); }
  /** lock state (see DoorDef.locked): `locked` clears when picked or kicked; `lockBroken` = kicked in (behaves as unlocked for the rest of the encounter, and a calm
   *  guard passing close will notice it once: `noticed`); `pick` = how far the cylinder has been worked 0‥1 (kept a couple of seconds after letting go, then it
   *  drifts back: Doors.update); `rattleT` > 0 just after the handle was tried (marker flash + a hair of leaf shake). All restored by reset(). */
  get locked(): boolean { return this.g(DC.locked) !== 0; } set locked(v: boolean) { this.p(DC.locked, v ? 1 : 0); }
  get lockBroken(): boolean { return this.g(DC.lockBroken) !== 0; } set lockBroken(v: boolean) { this.p(DC.lockBroken, v ? 1 : 0); }
  get noticed(): boolean { return this.g(DC.noticed) !== 0; } set noticed(v: boolean) { this.p(DC.noticed, v ? 1 : 0); }
  get noticedT(): number { return this.g(DC.noticedT); } set noticedT(v: number) { this.p(DC.noticedT, v); }   // (game time of that notice — evidence ages: guards.ts planClears)
  get pick(): number { return this.g(DC.pick); } set pick(v: number) { this.p(DC.pick, v); }
  get picking(): boolean { return this.g(DC.picking) !== 0; } set picking(v: boolean) { this.p(DC.picking, v ? 1 : 0); }
  get pickIdle(): number { return this.g(DC.pickIdle); } set pickIdle(v: number) { this.p(DC.pickIdle, v); }
  get pickClock(): number { return this.g(DC.pickClock); } set pickClock(v: number) { this.p(DC.pickClock, v); }
  get pickN(): number { return this.g(DC.pickN); } set pickN(v: number) { this.p(DC.pickN, v); }
  get rattleT(): number { return this.g(DC.rattleT); } set rattleT(v: number) { this.p(DC.rattleT, v); }
  /** game time a guard last pulled this leaf to / threw its bolt in a lockdown (guards.ts lockdownDoors: one man deals with a door, not all three every pass) */
  get guardT(): number { return this.g(DC.guardT); } set guardT(v: number) { this.p(DC.guardT, v); }
  /** the keyed men (a bit per contact id, ids < 32) this leaf is passing through right now — each pulled it shut past himself, or walked into it while somebody pulled
   *  it to (Doors.update): until they are apart again the leaf neither feels him nor moves him, and it will not latch inside him. 0 = nobody. */
  get passers(): number { return this.g(DC.passers); } set passers(v: number) { this.p(DC.passers, v); }
  readonly keySide: 1 | -1;
  /** the key that turns this lock (DoorDef.lock, FLOOR_KEY by default) — what a keyless contact's `keys` and the player's own ring are asked for */
  readonly lockId: string;
  readonly height: number; readonly thick: number;
  /** the swing limits as authored (DoorDef.minAngle / maxAngle): what the leaf's stops are with DOORWAY.oneWay off */
  readonly minAuth: number; readonly maxAuth: number;
  /** THE SIDE IT OPENS INTO (DOORWAY): the sign of every angle it can stand at. +1 = the CCW side of the closed leaf (Doors.side's +1): a point p with
   *  Doors.side(d, p) === d.swing is on its PULL face. Dealt once per level (game/doorswing.ts dealSwings → setSwing), the def's own `swing` first:
   *  `dealt` = the side the room rules give it, `outward` = the open-air side of an outside door (0 = it is not one) — taken while DOORWAY.exitsOut. */
  get swing(): 1 | -1 { const o = this.outward; if (o && DOORWAY.exitsOut) return o; const c = this.clearway; return c && DOORWAY.leafWay ? c : this.dealt; }
  get dealt(): 1 | -1 { return this.g(DC.dealt) as 1 | -1; } set dealt(v: 1 | -1) { this.p(DC.dealt, v); }
  get outward(): 1 | -1 | 0 { return this.g(DC.outward) as 1 | -1 | 0; } set outward(v: 1 | -1 | 0) { this.p(DC.outward, v); }
  /** DOORWAY.leafWay: the side it opens into where the dealt side's open leaf would shut floor off from the doorway (doorswing.ts 'leaf room'; 0 = it would not) */
  get clearway(): 1 | -1 | 0 { return this.g(DC.clearway) as 1 | -1 | 0; } set clearway(v: 1 | -1 | 0) { this.p(DC.clearway, v); }
  /** its stops: the authored limits — and, one-way, 0 on the side it does not open into (the frame) */
  get minA(): number { return DOORWAY.oneWay && this.swing > 0 ? 0 : this.minAuth; }
  get maxA(): number { return DOORWAY.oneWay && this.swing < 0 ? 0 : this.maxAuth; }
  /** > 0: being pulled open — the swing toward the men on that side passes the living (Doors.update); the Door.vel the last update / place() left, so a
   *  velocity somebody ELSE has written since (a drill's shove, a kick) is known for one (Doors.update turns a wrong-way opening shove round) */
  get pullT(): number { return this.g(DC.pullT); } set pullT(v: number) { this.p(DC.pullT, v); }
  /** pressT: how long a living man has pressed on it from the side it opens into, it standing in his way (Doors.update: PRESS_SECS of that and he pulls it);
   *  openT: seconds since somebody last DROVE it open — a push from its other face, a shove, a boot, an easing hand (a leaf being swung into a man is not his to
   *  pull); seenAngle: the angle the last update / place() left (crack() turns it between updates); mirrored: how many times keep() has had to turn an angle
   *  written on the far side round (QA: an opener never causes one) */
  /** DOORSTAY: the swing this leaf is on toward its frame is a keyed man's body's doing and nobody's order — at its frame it rests to, unlatched (update).
   *  Dropped by whatever moves it next that is not such a body: a hand, a boot, a blast, a script, the player's own push */
  get swept(): boolean { return this.g(DC.swept) !== 0; } set swept(v: boolean) { this.p(DC.swept, v ? 1 : 0); }
  /** DOORTHROW: who last drove it open and how (null: nobody since the reset, or the rule is off). The record is this door's own (its columns): written
   *  through, never swapped for another object. */
  get drive(): DoorDrive | null { return this.g(DC.drive) !== 0 ? this.driveCols : null; }
  set drive(v: DoorDrive | null) {
    if (v === null) { this.p(DC.drive, 0); for (const K of DRIVE_COLS) this.p(K, 0); return; }   // (A DRIVE THAT IS OVER LEAVES NOTHING: its columns are zeros, as its record was gone when a drive was an object — so two games that are the same game are the same columns, restarted, loaded or played on; doors.rs drive_off is this line)
    const D = this.driveCols;
    if (v !== D) { D.who = v.who; D.dir = v.dir; D.kind = v.kind; D.t = v.t; D.from = v.from; D.quiet = v.quiet; D.side = v.side; D.rate = v.rate; D.px = v.px; D.pz = v.pz; D.hit = v.hit; }
    this.p(DC.drive, 1);
  }
  private readonly driveCols: DriveCols;
  /** DOORWAY.solid: the pulls that have lately stopped on the player for want of floor to move him onto (fading: Doors.update; saved with it) */
  get solidT(): number { return this.g(DC.solidT); } set solidT(v: number) { this.p(DC.solidT, v); }
  get pressT(): number { return this.g(DC.pressT); } set pressT(v: number) { this.p(DC.pressT, v); }
  get openT(): number { return this.g(DC.openT); } set openT(v: number) { this.p(DC.openT, v); }
  get mirrored(): number { return this.g(DC.mirrored); } set mirrored(v: number) { this.p(DC.mirrored, v); }
  readonly box: Box; readonly handleBoxes: Box[];
  /** OFF ITS HINGES (doorblast.ts): the leaf is no longer a door — `box` is the loose slab `slab` describes (flying, then flat on the floor: 5 cm of wood no line
   *  of sight, sound, shot or smoke bake meets), update() leaves it alone, no verb moves it, and to everything that reads a door's state it is one standing wide
   *  open with its lock gone (`angle` ±OFF_ANGLE, unlatched, lockBroken): an open doorway. reset() hangs it again. */
  get off(): boolean { return this.g(DC.off) !== 0; } set off(v: boolean) { this.p(DC.off, v ? 1 : 0); }
  /** the loose slab while the leaf is off (null: it hangs). Its numbers are a row of the level's slab columns (`slabs`, sim/slabshome.ts): this reads
   *  the row (SlabRow), setting it writes a row whole (or, with null, gives the row up). Setting a door's slab to what `slab` just answered is nothing. */
  get slab(): SlabSeen | null { return this.slabs.rowOf(this.i) < 0 ? null : this.slabRow; }
  set slab(s: SlabSeen | null) {
    const T = this.slabs; if (s === null) { T.leave(this.i); return; } if (s === this.slabRow) return;
    const now: DoorSlab = { p: [s.p[0], s.p[1], s.p[2]], v: [s.v[0], s.v[1], s.v[2]], a: s.a, spin: s.spin, tip: s.tip, tipV: s.tipV, side: s.side, rest: s.rest, landed: s.landed, who: s.who, t: s.t, ang: s.ang, wedged: s.wedged, bits: s.bits ? [s.bits[0], s.bits[1], s.bits[2], s.bits[3]] : undefined };   // (read whole before a row moves: `s` may be another door's row)
    T.save(T.enter(this.i, !now.rest), now);
  }
  readonly slabs: SlabStore; private readonly slabRow: SlabRow;
  /** doorblast.ts DOORSHOT: the rounds' work this leaf has taken, in the wall engine's unit (with DOORSHOT.flat: rifle rounds, whatever the destruction scale) — one count for the whole leaf. reset() clears it. */
  get shot(): number { return this.g(DC.shot); } set shot(v: number) { this.p(DC.shot, v); }
  /** the hung leaf's half extents and markings (the slab at rest lies under other ones — doorblast.ts; reset() and the fog's remembered leaf put these back) */
  readonly leafH: Vec3; readonly leafMarks: Box['marks'];
  get creakArmed(): boolean { return this.g(DC.creakArmed) !== 0; } set creakArmed(v: boolean) { this.p(DC.creakArmed, v ? 1 : 0); }
  constructor(readonly def: DoorDef, store?: DoorStore, index = 0, slabs?: SlabStore) {
    const lone = store && slabs ? null : new SimState();   // (a lone door — the level editor's, a check's — has a block of its own: a store is a part of one)
    this.S = store ?? new DoorStore(lone!, 1); this.i = index; this.slabs = slabs ?? new SlabStore(lone!, index + 1); this.slabRow = new SlabRow(this.slabs, index); this.driveCols = new DriveCols(this.S, index);
    // (what the fields' own initialisers were)
    this.vel = 0; this.quietT = 10; this.lastWho = -1; this.sinceBash = 10; this.closing = false; this.lockBroken = false; this.noticed = false; this.noticedT = -1000; this.pick = 0; this.picking = false;
    this.pickIdle = 0; this.pickClock = 0; this.pickN = 0; this.rattleT = 0; this.guardT = -100; this.passers = 0; this.outward = 0; this.clearway = 0; this.pullT = 0; this.p(DC.velSeen, 0); this.swept = false;
    this.drive = null; this.solidT = 0; this.pressT = 0; this.openT = 10; this.mirrored = 0; this.p(DC.angleSeen, 0); this.off = false; this.shot = 0; this.creakArmed = true; this.p(DC.posAngle, NaN); this.p(DC.posX, 0); this.p(DC.posZ, 0);
    this.height = def.height ?? LEAF_H; this.thick = def.thickness ?? 0.05;
    this.minAuth = def.minAngle ?? -1.92; this.maxAuth = def.maxAngle ?? 1.92;
    this.dealt = def.swing ?? (Math.sign(def.angle ?? 0) || (this.maxAuth >= -this.minAuth ? 1 : -1)) as 1 | -1;   // (until dealSwings: the way it is authored standing open, else its wider side)
    this.locked = !!def.locked; this.keySide = def.keySide ?? 1; this.lockId = def.lock ?? FLOOR_KEY;
    this.angle = clamp(this.sided(def.angle ?? 0), this.minA, this.maxA); this.latched = Math.abs(this.angle) < 1e-3;
    const leafH: Vec3 = [def.width / 2 - 0.01, this.height / 2, this.thick / 2];
    this.leafH = leafH;
    this.box = makeBox({ c: [0, 0, 0], h: leafH, albedo: def.albedo ?? [0.42, 0.3, 0.19], flags: BoxFlag.Dynamic, name: 'door_' + def.name, ...(def.steel ? { rough: 0.3, metal: true } : { rough: 0.55, marks: doorLeafMarks(leafH) }) });   // finish for the reflection pass (Box.rough): a painted steel fire door has a satin sheen, the veneered leaves only just (ultra); MARKINGS: kick plates both faces + a push plate by the latch (the markings lane — a steel door is its own plate)
    // push plate + lever handle (raster only)
    this.handleBoxes = [0, 1].map(() => makeBox({ c: [0, 0, 0], h: [0.06, 0.012, 0.012], albedo: [0.55, 0.55, 0.52], stuff: 'paint', flags: BoxFlag.Dynamic | BoxFlag.NoShadow, rough: 0.25, metal: true }));   // (`stuff`: what the look made of this grey by its COLOUR until 11-04, kept so the picture did not move; nobody chose it. scene/stuff.ts drawnAs)
    this.leafMarks = this.box.marks;
    this.frameCentre = [def.hinge[0] + Math.cos(def.closedDir) * def.width * 0.5, 1.0, def.hinge[1] + Math.sin(def.closedDir) * def.width * 0.5];
    this.place();
  }
  /** Leaf centre (1 m up) at the current angle. Read in per-frame loops (interactables, body contacts, every emitted DoorEvent), so it is only recomputed
   *  when the angle has moved — as a fresh array each time, so a caller holding an earlier result keeps the snapshot it always got. */
  get pos(): Vec3 {
    if (this.off) return [this.frameCentre[0], this.frameCentre[1], this.frameCentre[2]];   // (off its hinges: 'the door' is its doorway — the slab on the floor is nobody's to use, and the fog looks for the frame)
    const a = this.angle;
    if (a !== this.g(DC.posAngle)) { this.p(DC.posAngle, a); const u = this.dir(); this.keptPos([this.def.hinge[0] + u[0] * this.def.width * 0.5, 1.0, this.def.hinge[1] + u[1] * this.def.width * 0.5]); }
    return this.posAt;
  }
  /** the kept centre: the angle it was made for is a column (posAngle), and so are its x and z beside the array (a step in the module makes it anew for a
   *  noise it lists, and the array is made from them: keptPos) */
  private posAt: Vec3 = [0, 1, 0];
  /** `p` is the kept centre from here on (the array itself: who holds it holds what he always got) */
  keptPos(p: Vec3): void { this.posAt = p; this.p(DC.posX, p[0]); this.p(DC.posZ, p[2]); }
  /** `fn` asked aside (a question of the picture's — orders.ts hover): reading `pos` after a leaf has swung makes its kept centre anew; what each of `doors`
   *  kept is put back after, so a game that is drawn keeps what the simulation's own reads left */
  static posAside<T>(doors: readonly Door[], fn: () => T): T {
    const a = doors.map(d => d.g(DC.posAngle)), p = doors.map(d => d.posAt);
    try { return fn(); } finally { for (let i = 0; i < doors.length; i++) { doors[i].p(DC.posAngle, a[i]); doors[i].keptPos(p[i]); } }
  }
  /** Static doorway centre (where the closed leaf's middle sits). */
  readonly frameCentre: Vec3;
  /** 'Closed enough' that using it means opening it (shared by use() and the HUD prompt). */
  isClosed() { return Math.abs(this.angle) < 0.25; }
  dir(): [number, number] { const a = this.def.closedDir + this.angle; return [Math.cos(a), Math.sin(a)]; }
  /** Back to the level as authored: angle, latch, lock, and everything the encounter did to it (pick progress, a kicked lock, a guard having noticed it). */
  reset() { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Door.reset', this, []); if (__r !== RUST_PASS) return __r as any; } }
    this.shot = 0;
    if (this.off) { this.off = false; this.slab = null; this.box.h = this.leafH; this.box.marks = this.leafMarks; this.box.rot = undefined; for (const hb of this.handleBoxes) hb.rot = undefined; }   // hung again, as authored
    this.angle = clamp(this.sided(this.def.angle ?? 0), this.minA, this.maxA);   // (one-way: a leaf authored standing open the way it is not dealt stands as wide the way it is — as setSwing left it)
    this.vel = 0; this.latched = Math.abs(this.angle) < 1e-3; this.closing = false; this.swept = false; this.quietT = 10; this.sinceBash = 10; this.creakArmed = true; this.lastWho = -1;   // (lastWho too: the fresh encounter's first bang / latch is nobody's yet — it used to keep the last man of the encounter before, review B7.12)
    this.locked = !!this.def.locked; this.lockBroken = false; this.noticed = false; this.noticedT = -1000; this.pick = 0; this.picking = false; this.pickIdle = 0; this.pickClock = 0; this.rattleT = 0; this.guardT = -100; this.passers = 0;
    this.pullT = 0; this.pressT = 0; this.solidT = 0; this.openT = 10; this.drive = null;
    this.place();
  }
  /** Deal the side it opens into (game/doorswing.ts, once per level): a leaf authored standing open the other way is stood as far open this way. */
  setSwing(s: 1 | -1, outward: 1 | -1 | 0 = 0, clearway: 1 | -1 | 0 = 0) { this.dealt = s; this.outward = outward; this.clearway = clearway; if (DOORWAY.oneWay) { this.keep(); this.latched = Math.abs(this.angle) < 1e-3; this.place(); } }
  /** One-way: an angle on the side it does not open into (written by a script, a staging, an old save) is the same opening on the side it does — and whatever
   *  velocity came with it turns with it. */
  keep() { if (this.angle * this.swing < 0) { this.angle = clamp(-this.angle, this.minA, this.maxA); this.vel = -this.vel; this.mirrored++; } }
  /** an angle (or a turn rate) as this door can have it: one-way, the same opening on the side it opens into (savestate.ts: a file from before it had one) */
  sided(a: number): number { return DOORWAY.oneWay && a * this.swing < 0 ? -a : a; }
  place() { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Door.place', this, []); if (__r !== RUST_PASS) return __r as any; } }
    if (this.off) return;   // (the slab's box is doorblast.ts's to pose)
    if (DOORWAY.oneWay) this.keep();
    this.p(DC.velSeen, this.vel); this.p(DC.angleSeen, this.angle);
    const jig = this.rattleT > 0.35 ? Math.sin(this.rattleT * 90) * 0.005 : 0;   // the handle being tried: the leaf works a hair in its frame (drawn only — dir(), which the contacts read, stays put)
    const P = this.poseAt(this.angle + jig);
    this.box.c = P.c; this.box.yaw = P.yaw;
    this.box.flags = Math.abs(this.angle) < 0.3 ? this.box.flags | BoxFlag.SoundSeal : this.box.flags & ~BoxFlag.SoundSeal;   // an open leaf standing in the room does not seal the doorway acoustically
    for (let s = 0; s < 2; s++) { const hb = this.handleBoxes[s]; hb.c = P.handles[s]; hb.yaw = P.yaw; }
    leafWrote();   // (the box world's copy in the Rust module copies the leaves again: scene/boxMirror.ts)
  }
  /** Where the leaf's box and its two handles stand with the leaf at `angle`: place() writes these for the angle it has, and the fog of war (fogofwar.ts) poses
   *  the leaf the team last SAW with it. */
  poseAt(angle: number): { c: Vec3; yaw: number; handles: [Vec3, Vec3] } {
    const w = this.def.width; const [hx, hz] = this.def.hinge;
    const a = this.def.closedDir + angle; const u: [number, number] = [Math.cos(a), Math.sin(a)];
    const yaw = -a;                                             // box local +X maps to world (cos yaw, -sin yaw)
    const n: [number, number] = [-u[1], u[0]]; const off = this.thick / 2 + 0.03;
    const handle = (side: number): Vec3 => [hx + u[0] * (w - 0.14) + n[0] * off * side, 1.0, hz + u[1] * (w - 0.14) + n[1] * off * side];
    return { c: [hx + u[0] * (w * 0.5 + 0.01), this.height / 2 + 0.01, hz + u[1] * (w * 0.5 + 0.01)], yaw, handles: [handle(1), handle(-1)] };
  }
}


/** Doors.swingOf: a door's jambs, the side it opens into and the floor its leaf sweeps. */
export interface DoorSwing {
  /** the hinge jamb and the handle (latch) jamb, on the floor: the two ends of the closed leaf */
  hinge: Vec3; handle: Vec3;
  /** the sign of the angles it opens through (Door.swing): Doors.side(d, p) === swing ⇔ p is on the PULL face */
  swing: 1 | -1;
  /** unit normal (x, z) of the closed leaf, pointing into the side it opens into */
  into: [number, number];
  /** rad from shut to its open stop · the leaf's length · the world angle of the closed leaf (DoorDef.closedDir): it sweeps the sector about `hinge`, radius
   *  `radius`, from `closedDir` round by swing × open */
  open: number; radius: number; closedDir: number;
  /** is the one-way rule on (DOORWAY.oneWay)? Off, the leaf is double-acting as it was and this sweep is only the side the level deals */
  oneWay: boolean;
  /** is the floor point inside the area the leaf sweeps, with `pad` m of slack (a body's radius) */
  sweeps(x: number, z: number, pad?: number): boolean;
}

/** WHAT THE DOORS' STEP NEEDS OF ITS GAME TO RUN IN THE RUST MODULE (set by the Game: Doors.host). The module asks the statics for floor as the Game's own two
 *  functions do (rust/sim/src/doors.rs standable, settle), so it steps only while `Doors.standable` and `Doors.settle` are still these. */
export interface DoorsHost {
  /** the box world the leaves stand in: its copy in the module is where the step finds its floor and writes the leaves' rows */
  col: StaticCollision;
  standable: NonNullable<Doors['standable']>; settle: NonNullable<Doors['settle']>;
  /** the head of the span the settled man is pushed out of the statics over (player.ts settleSpot: standing or crouched) */
  settleY1(): number;
}

export class Doors {
  list: Door[];
  /** the leaves' own numbers, a column each (sim/doorshome.ts): a part of the game's block */
  readonly store: DoorStore;
  /** the leaves off their hinges, a row each (sim/slabshome.ts): a part of the game's block */
  readonly slabs: SlabStore;
  /** the Game's hand-over for the step in the module (null: no game — the step is this file's) */
  host: DoorsHost | null = null;
  constructor(defs: DoorDef[], private emit: (e: DoorEvent) => void, st: SimState = new SimState()) { this.store = new DoorStore(st, defs.length); this.slabs = new SlabStore(st, defs.length); this.list = defs.map((d, i) => new Door(d, this.store, i, this.slabs)); this.list.forEach((d, i) => { d.box.mkey = MKEY.door + i * 4; d.handleBoxes[0].mkey = MKEY.door + i * 4 + 1; d.handleBoxes[1].mkey = MKEY.door + i * 4 + 2; }); }   // motion identity for the leaf and its handles (scene/boxes.ts MKEY)

  byName(n: string) { return this.list.find(d => d.def.name === n); }

  /** Is `from` on d's PULL face — the side it opens into, where the leaf comes toward you (the other face pushes it away)? Read whatever DOORWAY.oneWay says:
   *  with it off this is only the side the level deals. */
  pullFace(d: Door, from: Vec3): boolean { const c = d.def.closedDir; return ((from[0] - d.def.hinge[0]) * -Math.sin(c) + (from[2] - d.def.hinge[1]) * Math.cos(c)) * d.swing >= 0; }
  /** DOORWAY.frameHolds as it applies right now: one-way leaves, the switch on, no showcase on the stage */
  get frameRule(): boolean { return DOORWAY.oneWay && DOORWAY.frameHolds && !this.staged(); }
  /** WOULD THE FRAME HOLD A BOOT thrown at d from `from` (DOORWAY.frameHolds): the leaf is home in its frame and `from` is on the face it opens toward, where it
   *  rests on the stop all the way round. What kickIn refuses, and what whoever plans a boot (the door tools, Sam's rows, a sprint into it) asks first. */
  bootHolds(d: Door, from: Vec3): boolean { return this.frameRule && !d.off && !d.def.steel && Math.abs(d.angle) < SHUT_HOME && this.pullFace(d, from); }   // (steel has its own answer from either face: kickIn's clang)
  /** THE HOLD TEST for a wedge or dragged furniture put against `d` by a man standing at `from` (digin.ts): it holds only a leaf that opens TOWARD him — the
   *  thing sits in the leaf's path. A door that opens away from him is pulled open from the far side whatever stands behind it. Off (or two-way doors, or a
   *  staged scene) = any door holds, as it was. Read live, never kept: a door's side can be re-dealt under a running game */
  wedgeHolds(d: Door, from: Vec3): boolean { return goneWithTick('Doors.wedgeHolds') as any; }
  /** THE DOOR AS A HINGED THING, for whoever plans round it (the AI's opener going to the handle, the men keeping out of its swing): where its two jambs are,
   *  which side it opens into and the floor its leaf sweeps getting there. Constant per door (nothing here reads the angle it stands at). */
  swingOf(d: Door): DoorSwing {
    const [hx, hz] = d.def.hinge, W = d.def.width, c = d.def.closedDir, S = d.swing; const ux = Math.cos(c), uz = Math.sin(c);
    const open = Math.abs(S > 0 ? d.maxAuth : d.minAuth);
    const seg = (x: number, z: number, a: number) => { const vx = Math.cos(a), vz = Math.sin(a), t = clamp((x - hx) * vx + (z - hz) * vz, 0, W); return Math.hypot(x - hx - vx * t, z - hz - vz * t); };
    return { hinge: [hx, 0, hz], handle: [hx + ux * W, 0, hz + uz * W], swing: S, into: [-uz * S, ux * S], open, radius: W, closedDir: c, oneWay: DOORWAY.oneWay,
      sweeps: (x, z, pad = 0) => {
        const px = x - hx, pz = z - hz, r = Math.hypot(px, pz); if (r > W + pad) return false;
        let th = (Math.atan2(pz, px) - c) * S; th -= Math.round(th / (2 * Math.PI)) * 2 * Math.PI; if (th < 0 && open > Math.PI && th + 2 * Math.PI <= open) th += 2 * Math.PI;   // the point's bearing from the closed leaf, the way it opens
        return (th >= 0 && th <= open) || seg(x, z, c) <= pad || seg(x, z, c + S * open) <= pad;
      } };
  }
  /** a showcase / the tour / the attract reel has the stage (the Game hands this its `touring`): its choreography runs with the closers it was written for */
  staged: () => boolean = () => false;
  /** DOORSTAY as it applies right now: leaves stay where they are left */
  get stay(): boolean { return DOORSTAY.on && !this.staged(); }
  /** (DoorContact.round) how many times a leaf's back has been gone round — one for each unbroken press of it (checks read it) */
  rounds = 0;
  /** does this leaf's hydraulic closer work (what waits on a door swinging to by itself asks this, never def.closer) */
  hasCloser(d: Door): boolean { return goneWithTick('Doors.hasCloser') as any; }

  /** Which face of d the point is on: +1 = the CCW side of the leaf (perp > 0 in update()'s convention), −1 the other. */
  side(d: Door, from: Vec3): 1 | -1 { const u = d.dir(); const px = from[0] - d.def.hinge[0], pz = from[2] - d.def.hinge[1]; return u[0] * pz - u[1] * px >= 0 ? 1 : -1; }
  /** The lock is against you: latched, still locked, you are on the keyed side (see DoorDef.locked) — and nothing on your ring fits it (`keys`: Player.keys;
   *  absent = no keys at all). What use / crack / a keyless push run into. */
  lockedOut(d: Door, from: Vec3, keys?: ReadonlySet<string>): boolean { return d.locked && d.latched && this.side(d, from) === d.keySide && !(keys && keys.has(d.lockId)); }

  /** DOORTHROW: is there floor for a body of radius r at (x, z)? Set by the Game (this file knows no walls); null = anywhere */
  standable: ((x: number, z: number, r: number) => boolean) | null = null;
  /** DOORWAY.solid: put a standing body of radius r at p where the statics let it be (game.ts: the player's own collision; absent: `standable` decides) */
  settle: ((p: Vec3, r: number) => void) | null = null;
  /** DOORWAY.solid: frames a pulled leaf moved the player · pulls that stopped on him · pulls that passed him after SOLID_TRIES stops (a QA read) */
  solids = { carried: 0, stopped: 0, passed: 0, reached: 0, held: 0, aside: 0 };
  /** DOORTHROW: what the Game knows of the man with this id (MAN_SIDE | MAN_FOLDED | MAN_BRACED | MAN_CALM); null = nothing of anybody */
  man: ((who: number) => number) | null = null;
  /** DOORTHROW as it applies right now: the switch on, no showcase on the stage */
  get throws(): boolean { return DOORTHROW.on && !this.staged(); }
  /** would a turn of sign `dir` take d further OPEN (one-way: the way it opens; else away from its frame) */
  private opening(d: Door, dir: number): boolean { return DOORWAY.oneWay ? dir === d.swing : Math.abs(d.angle) < 0.05 || dir * d.angle > 0; }
  /** DOORTHROW: somebody drove d open (Door.drive) */
  private driven(d: Door, who: number, dir: number, kind: DoorDrive['kind'], quiet: boolean, rate: number) {
    const D = d.drive, s = dir >= 0 ? 1 : -1, m = this.man ? this.man(who) : 0;
    if (kind !== 'boot' && m & MAN_CALM) quiet = true;
    if (D && D.who === who && D.dir === s && D.kind === kind) { if ((kind !== 'body' && kind !== 'hand') || D.t >= DRIVEN_SECS) D.hit = -1; D.t = 0; D.quiet = quiet; D.rate = rate; D.side = m & MAN_SIDE; if (kind !== 'body' && kind !== 'hand') D.from = d.angle; }   // (a walk or a hand goes on: one drive, from where it began)
    else d.drive = { who, dir: s, kind, t: 0, from: d.angle, quiet, side: m & MAN_SIDE, rate, px: NaN, pz: NaN, hit: -1 };
  }

  /** Resolve contacts (mutates contact positions when a door can't yield), integrate swing, closer, latch — in the Rust module where the step can run there
   *  (RUST.doors: `there`), else here; the same game to the bit either way (tools/qa/door-ident.ts, tools/qa/rust-check.ts). */
  update(dt: number, contacts: DoorContact[]): void { return crossToRust('Doors.update', this, [dt, contacts]) as any; }

  /** door i's box and handles made what the module's step placed (its row of the box world's leaves, and the handles it handed back): Door.place's writes */
  private follow(M: BoxMirror, i: number): void {
    const io = doorIo(), m = doorMem(), o = io.out + i, C = DOOR_CAP; if (m[o + DO.placed * C] === 0) return;
    const d = this.list[i]; this.handles(d, M.followLeaf(i, d.box), m[o + DO.h1x * C], m[o + DO.h1z * C], m[o + DO.h2x * C], m[o + DO.h2z * C]);
  }
  private handles(d: Door, yaw: number, x1: number, z1: number, x2: number, z2: number): void { const h = d.handleBoxes; h[0].c = [x1, 1.0, z1]; h[0].yaw = yaw; h[1].c = [x2, 1.0, z2]; h[1].yaw = yaw; }
  /** THE STEP BOTH WAYS (collision.ts BOTH: tools/qa/rust-check.ts --both, tools/qa/door-ident.ts): taken in the module with nothing of it kept, then here from
   *  the same columns and the same contacts, and everything the two leave compared to the bit — every column of every door, where each contact ends and what
   *  was done to him, the noises in order, each leaf's box and handles, the tallies. The game goes on with this file's. → '' or the first difference in words. */
  updateBoth(dt: number, contacts: DoorContact[], M: BoxMirror): string { return goneWithTick('Doors.updateBoth') as any; }
  /** the step as this file takes it (the TypeScript's own: what the module's is held to) */
  updateHere(dt: number, contacts: DoorContact[]): void { return crossToRust('Doors.updateHere', this, [dt, contacts]) as any; }


  /** Hold-to-crack: ease a (near-)closed leaf open away from `from`, silently, up to `maxAngle`; the closer stays off while it is being held.
   *  The wedge of light through the gap widens as you inch it — and nobody hears a thing unless you let it swing. (A lock against you: nothing gives —
   *  unless one of `keys` fits it, and then the key goes round with the handle as softly as the rest.) */
  crack(d: Door, from: Vec3, who: number, dt: number, keys?: ReadonlySet<string>, rate = 0.55, maxAngle = 0.6) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Doors.crack', this, [d, from, who, dt, keys, rate, maxAngle]); if (__r !== RUST_PASS) return __r as any; } }
    if (d.off || this.lockedOut(d, from, keys)) return;
    const [hx, hz] = d.def.hinge; const u = d.dir(); const px = from[0] - hx, pz = from[2] - hz; const perp = u[0] * pz - u[1] * px;
    const sgn = DOORWAY.oneWay ? d.swing : Math.abs(d.angle) > 0.02 ? Math.sign(d.angle) : (perp >= 0 ? -1 : 1);
    if (DOORWAY.oneWay && (perp >= 0 ? 1 : -1) === d.swing) d.pullT = Math.max(d.pullT, 0.3);   // eased toward himself: the leaf comes past the hand that has it
    d.latched = false; d.closing = false; d.vel = 0; d.quietT = -1.5; d.lastWho = who;
    if (Math.abs(d.angle) < maxAngle) d.angle = clamp(d.angle + sgn * rate * dt, d.minA, d.maxA);
  }

  /** Player 'use' (a tap on F): closed → swing open away from the user; open → pull/push it shut (closer takes it the last bit fast); locked against you → just
   *  the rattle — or, with a key that fits on your ring (`keys`), the key's clack (KEY_LEVEL) and it swings open like any other: the bolt bites again behind
   *  you when the leaf next latches — by its closer, or, where doors stay as they are left (DOORSTAY), when a hand shuts it. */
  use(d: Door, from: Vec3, who: number, keys?: ReadonlySet<string>): void { return crossToRust('Doors.use', this, [d, from, who, keys]) as any; }
  /** Pull a leaf to from wherever it stands, even a hand's breadth ajar (use() would push that one open): a stiff sprung close that the latch catches — works for
   *  hold-open doors too. What a guard tidying up in a lockdown does (guards.ts lockdownDoors). */
  /** a contact's bit in Door.passers: his id's own under 32. DOORSTAY: a keyed man past id 31 — a big squad map's last few units — has one of bits 2‥9, ids
   *  nobody is dealt (Sam is 1, the shift 10 up, units 20 up, civilians 40 up and keyless): sharing `id & 31` gave unit 33 Sam's bit, and a leaf one of the
   *  two was stepping through then did not feel the other. With it off he has none, as ever. */
  private passBit(c: DoorContact): number { return goneWithTick('Doors.passBit') as any; }
  /** how far a leaf stands open the way a shove of sign `s` (a plan's fling) would send it: what 'already open, leave it' is tested on. One-way, that is simply
   *  how far open it stands — it has one side, and a fling written against it is turned into a pull (update) — so a leaf left wide open is never shoved again:
   *  the old test read such a leaf as 'not open my way', and the shove, past 1.2 rad a shove toward shut, slammed it on its frame in front of the entry. */
  openFor(d: Door, s: number): number { return DOORWAY.oneWay ? Math.abs(d.angle) : d.angle * s; }
  pullTo(d: Door, who: number) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Doors.pullTo', this, [d, who]); if (__r !== RUST_PASS) return __r as any; } } if (d.latched || d.off) return; d.closing = true; d.swept = false; d.vel = -Math.sign(d.angle) * 1.2; d.lastWho = who; }
  /** A hand on the handle from the face it opens into (handleside.ts — the AI's opener, standing at the latch jamb clear of the sweep): exactly what a man
   *  pressing on it there gets in update() — unlatched with its creak, brought round toward him at the pull's pace — asked for outright, without his body in it. */
  pull(d: Door, who: number, quiet = false): void { return crossToRust('Doors.pull', this, [d, who, quiet]) as any; }

  // ---------------------------------------------------------------- locks
  /** seconds of actual cylinder work to pick a lock (the plant / hands-up beat before it comes on top: ~4.5 s door to door) */
  static readonly PICK_SECS = 4.2;
  /** hearing levels (GameEvent 'door', see guards.ts hearingCheck: a = level·(1−d/12)·carry must beat 0.08 to register at all, 0.3 awareness = 'huh?'):
   *  a pick click every ~0.7 s at 0.16 walks a patrolling man within ~3 m up to 'huh?' over one full pick and shows as ticks on his marker to ~5 m; the
   *  cylinder going over (0.3, a door creak's worth) tips one at ~4 m; the tried handle (0.12) is a tick at arm's length and nothing across the room, and
   *  a key turning it (KEY_LEVEL, use() with a key that fits) is the same small clack — eased round on a held handle (crack) it is nothing at all. */
  static readonly PICK_LEVEL = 0.16; static readonly UNLOCK_LEVEL = 0.3; static readonly KEY_LEVEL = 0.12;

  /** Try the lever against the bolt: a clack, the leaf working in its frame, the marker flashing 'won't budge' (Door.rattleT). Rate-limited by the flash itself. */
  rattle(d: Door, who: number, level = 0.12) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Doors.rattle', this, [d, who, level]); if (__r !== RUST_PASS) return __r as any; } }
    if (d.rattleT > 0 || d.off) return;
    d.rattleT = 0.6; d.lastWho = who; this.emit({ door: d, sound: 'rattle', pos: d.pos, who, level });
  }

  /** Where to work d's lock from `from`'s side: the root half a metre square off the leaf in front of the keyway (under the lever, 14 cm in from the latch edge —
   *  CharacterAnimator.lockpickAt reaches 0.45 m dead ahead), facing the leaf. `stand` = the stand-off (the kick uses the same spot further out). */
  workSpot(d: Door, from: Vec3, stand = 0.5, along = d.def.width - 0.14): { pos: Vec3; yaw: number } {
    const [hx, hz] = d.def.hinge; const u = d.dir(); const s = this.side(d, from); const nx = -u[1] * s, nz = u[0] * s;   // unit normal out of the leaf on `from`'s side
    return { pos: [hx + u[0] * along + nx * stand, 0, hz + u[1] * along + nz * stand], yaw: Math.atan2(-nx, -nz) };        // (Character.forward = [sin yaw, 0, cos yaw]: face back into the leaf)
  }
  /** Where a kick is thrown from: square to the leaf 0.76 m off it (the sole arrives ~0.75 m ahead of the root at KICK_IMPACT), at the runner's own station
   *  along the leaf but kept on its outer half — by the lock, where a kick belongs and the lever arm is longest. */
  kickSpot(d: Door, from: Vec3): { pos: Vec3; yaw: number } {
    const u = d.dir(); const along = (from[0] - d.def.hinge[0]) * u[0] + (from[2] - d.def.hinge[1]) * u[1];
    return this.workSpot(d, from, 0.76, clamp(along, d.def.width * 0.5, d.def.width - 0.18));
  }
  /** The same kick thrown from BESIDE the door (DOORWAY.sideKick): the root past the latch edge, behind the wall on `from`'s face — about where a stack's
   *  jamb man stands — turned to the lock, the sole's three-quarters of a metre from it. Out of the opening and outside the leaf's sweep on either face.
   *  Geometry only: whether there is floor there is the caller's to ask. */
  kickSpotSide(d: Door, from: Vec3): { pos: Vec3; yaw: number } {
    const [hx, hz] = d.def.hinge; const u = d.dir(); const s = this.side(d, from); const nx = -u[1] * s, nz = u[0] * s, W = d.def.width;
    const pos: Vec3 = [hx + u[0] * (W + SIDE_KICK.past) + nx * SIDE_KICK.out, 0, hz + u[1] * (W + SIDE_KICK.past) + nz * SIDE_KICK.out];
    const lx = hx + u[0] * (W - 0.2), lz = hz + u[1] * (W - 0.2);
    return { pos, yaw: Math.atan2(lx - pos[0], lz - pos[2]) };
  }

  /** One frame of working the lock (the caller keeps the picker planted at workSpot and decides when he has to break off). Advances Door.pick; the pick clicks
   *  in an uneven ~0.7 s rhythm the room can hear (PICK_LEVEL), and when the cylinder goes over the bolt draws with a louder snap (UNLOCK_LEVEL) and the door
   *  is simply unlocked from then on. Progress survives a pause (see update()). Returns true on the frame it opens. */
  pick(d: Door, who: number, dt: number): boolean { return goneWithTick('Doors.pick') as any; }

  /** A boot beside the lock: the keep tears out of the jamb (lockBroken — it is an unlocked door for the rest of the encounter), the leaf is flung to its stop
   *  harder than any shoulder bash, and the crack of it carries through the whole floor (DoorEvent 'kick' → GameEvent 'kick', heard like a shot). Works on any
   *  leaf the kicker can reach, locked or not; one already standing wide open just gets the noise.
   *  THE FRAME HOLDS (bootHolds): at the face a shut leaf opens toward, the boot lands on a door backed by its frame — a loud thud, the leaf shivering in it
   *  ('won't budge'), the keep whole, the lock still thrown. `lock`: what hit it destroys the LOCK ITSELF rather than driving the leaf (a charge on the keep,
   *  a breaching round): from that face the keep is gone and the leaf not flung — it stands home in its frame, unlocked, for a hand to pull (or for the caller's
   *  own write of Door.vel); from the push face, and with the rule off, the argument changes nothing. Returns whether the door GAVE (false: off its hinges,
   *  steel, or its frame held the boot) — the one answer a caller needs, so nobody asks bootHolds a second time to learn what its kick did. */
  kickIn(d: Door, from: Vec3, who: number, lock = false): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Doors.kickIn', this, [d, from, who, lock]); if (__r !== RUST_PASS) return __r as any; } }
    if (d.off) return false;
    if (d.def.steel) { d.rattleT = 0.6; d.lastWho = who; this.emit({ door: d, sound: 'clang', pos: d.pos, who, level: 1.0 }); return false; }   // steel in a steel frame: the boot rings off it, the leaf shivers in its frame ('won't budge'), the bolt holds — a loud noise and nothing else
    const held = this.bootHolds(d, from);
    if (held && !lock) { d.rattleT = 0.6; d.lastWho = who; this.emit({ door: d, sound: 'thud', pos: d.pos, who, level: 1.0 }); return false; }   // it opens toward the boot: the whole frame is behind the leaf, and nothing gives — as loud as a boot on steel, and as useless
    if (!d.lockBroken) d.noticed = false;    // a picked lock they had already remarked on, now kicked through: the splintered jamb is news again (a second kick at a broken keep is not)
    d.locked = false; d.lockBroken = true;   // (only a leaf locked against the kicker ever gets kicked: game.ts)
    d.pick = 0; d.latched = false; d.closing = false; d.swept = false; d.quietT = -0.5; d.lastWho = who; d.creakArmed = false;
    if (held) { /* (the lock destroyed from the face it opens toward: nothing drove the leaf — the keep is gone and it stands home in its frame, unlocked; left alone it clicks back onto its latch like any shut door, and a hand opens it from either face) */ }
    else if (Math.abs(d.angle) < 1.4) { d.vel = -this.side(d, from) * 9; d.sinceBash = 0; if (this.throws && this.opening(d, Math.sign(d.vel))) this.driven(d, who, d.vel, 'boot', false, 0); }   // away from the boot (side +1 pushes the leaf CW = negative angle), on the fast 'kicked' path in update()
    this.emit({ door: d, sound: 'kick', pos: d.pos, who, level: 1.0 });
    return true;
  }

  // ---------------------------------------------------------------- off its hinges (doorblast.ts)
  /** THE DOORS' SIDE OF A BLAST — the one call a blast makes about doors, whatever set it off (the destruction engine's blast entry point, a frag, a charge): it
   *  went off at `at`, and takes an ordinary leaf off its hinges inside `tearM` metres of it (0 = this kind of blast never does). Every leaf it reaches by a line
   *  no wall is in comes out of its frame as a loose slab (doorblast.ts doorBlast — steel doors and ones already off are not touched); the doors it took come
   *  back. `who`: the character blamed for the noise. Nothing with the switch off (DOORBLAST.on) or before a Game has given the doors their world. */
  blast(at: Vec3, tearM: number, who: number): Door[] { if (RUST_SURFACE?.tapFiring('Doors.blast')) return RUST_SURFACE.tapAnswer() as any; return this.blaster ? this.blaster(at, tearM, who) : []; }
  /** set by the Game (doorblast.ts needs its collision world, events and audio — this file knows none of them) */
  blaster: ((at: Vec3, tearM: number, who: number) => Door[]) | null = null;
  /** a noise of d's that is not a leaf's own doing (doorblast.ts: the frame letting go = a 'kick', the slab landing = a 'bang') — heard and played as any other */
  sound(d: Door, sound: DoorSound, pos: Vec3, who: number, level: number) { this.emit({ door: d, sound, pos, who, level }); }
  /** told of every leaf that comes off, by whatever road (a blast through blast() above, a charge set on the leaf, gunfire: doorblast.ts DOORSHOT): `fn(door, who, cause)` on the frame it leaves its
   *  frame — for a change feed, a tally, the AI. Returns the unsubscribe. */
  onTorn(fn: (d: Door, who: number, cause: 'blast' | 'fire') => void): () => void { this.tornSubs.push(fn); return () => { const i = this.tornSubs.indexOf(fn); if (i >= 0) this.tornSubs.splice(i, 1); }; }
  torn(d: Door, who: number, cause: 'blast' | 'fire' = 'blast') { for (const fn of [...this.tornSubs]) fn(d, who, cause); }
  private tornSubs: ((d: Door, who: number, cause: 'blast' | 'fire') => void)[] = [];

  /** Insert approach/exit points where a path crosses a doorway so characters take doors head-on and clear the
   *  swung leaf before turning (the leaf can stand perpendicular to the wall at its stop). */
  threadPath(path: Vec3[], start: Vec3, isFree: (x: number, z: number) => boolean = () => true): Vec3[] {
    if (!path.length) return path;
    const out: Vec3[] = []; let prev = start;
    for (const p of path) {
      const ins: { t: number; pts: Vec3[] }[] = [];
      for (const d of this.list) {
        const [hx, hz] = d.def.hinge; const u: [number, number] = [Math.cos(d.def.closedDir), Math.sin(d.def.closedDir)]; const n: [number, number] = [-u[1], u[0]];
        // intersect segment prev→p with the closed-door line (hinge .. hinge+u*W) in 2D
        const ax = prev[0] - hx, az = prev[2] - hz, bx = p[0] - hx, bz = p[2] - hz;
        const da = ax * n[0] + az * n[1], db = bx * n[0] + bz * n[1];
        if (da * db >= 0 || Math.abs(da - db) < 1e-6) continue;
        const t = da / (da - db); const ix = ax + (bx - ax) * t, iz = az + (bz - az) * t; const along = ix * u[0] + iz * u[1];
        if (along < -0.1 || along > d.def.width + 0.1) continue;
        const mid = d.def.width * 0.55; const sgn = da < 0 ? 1 : -1;      // travelling toward +n if da<0
        const cx = hx + u[0] * mid, cz = hz + u[1] * mid;
        const pick = (dists: number[], side: number): Vec3 | null => { for (const dd of dists) { const q: Vec3 = [cx + n[0] * dd * side, 0, cz + n[1] * dd * side]; if (isFree(q[0], q[2])) return q; } return null; };
        // only add points the existing waypoints don't already cover, otherwise the guard overshoots and doubles back in the doorway
        const before = -da * sgn, beyond = db * sgn;                         // how far prev / p sit from the door line along the travel direction (both ≥ 0)
        const pts: Vec3[] = [];
        if (before > 0.75 + 0.5) { const a = pick([0.75, 0.55], -sgn); if (a) pts.push(a); }            // approach square-on
        if (beyond > 1.35 + 0.5) { const b = pick([1.35, 1.1, 0.85, 0.6], sgn); if (b) pts.push(b); }   // clear the leaf before turning
        if (pts.length) ins.push({ t, pts });
      }
      ins.sort((p0, p1) => p0.t - p1.t); for (const i of ins) out.push(...i.pts);                        // in travel order if one leg crosses two doorways
      out.push(p); prev = p;
    }
    return out;
  }

  /** How close (planar, to the leaf centre or the doorway centre) a character must be to use a door (player.ts buildInteractables). */
  static readonly USE_REACH = 1.35;

  boxes(out: Box[]) { for (const d of this.list) { out.push(d.box, d.handleBoxes[0], d.handleBoxes[1]); } }

  /** Leaf boxes, for registering as dynamic occluders with the collision layer. */
  leafBoxes(): Box[] { return this.list.map(d => d.box); }
  /** `fn` asked aside of every door here (Door.posAside) */
  posAside<T>(fn: () => T): T { return Door.posAside(this.list, fn); }
}
