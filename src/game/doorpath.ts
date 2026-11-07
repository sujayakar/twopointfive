// EACH MAN WALKS A REAL PATH THROUGH THE DOOR (`DOORPATH.on`; ON in the page — `?nodoorpath` puts it off, `?doorpath=gun` adds the gun rule —, off in the
// harness unless TPS_DOORPATH=1[,knob=value…]; off: the entry exactly as it was).
//
// What a room entry did (doortools.ts, the 'enter' stage): every man was let go a quarter second after the one before, each handed his point of domination and
// the nav grid's path to it. The grid knows walls; it does not know a LEAF standing open, nor the man in front. So the second man in hooked tight round the
// hinge jamb for his corner, walked into the leaf's open face and pushed it to its stop — where the door's own rule (doors.ts: a keyed man pressed on a leaf at
// its stop pulls it to) swung it shut THROUGH him and through the next man in the opening; and four men let go on a clock met in the frame shoulder to
// shoulder. With the switch on:
//   THE LANE    each man's walk is laid by hand from where he stands: to a point `outM` short of the frame line, across it at `at` of the opening from the
//               hinge (the half the leaf is not in), on to a point `inM` inside — one file through the middle of the gap, square to the frame
//   THE LEAF    from there to his point a leg may not come within `leafClear` of any leaf AS IT STANDS (his own door's, the next room's): where the straight
//               leg would, he goes ROUND THE FREE TIP on an arc that keeps that much air (re-laid as the leaf moves). A point with NO way round — the pocket between an
//               open leaf and the wall behind it, its mouth narrower than a man and that clearance — is not walked into through the leaf: his point is
//               moved to the nearest floor of the same room he can reach and stand on clear of every leaf (`moved`; the order's own record of his point
//               moves with it). While the leaf is still swinging nothing is decided: he walks his lane and waits at its end for it to stand. Only a point
//               with no such floor near it is walked straight, as before, and counted (`noRound`)
//   THE FILE    the men go in the order the door order dealt, one at a time: a man is let go when the man before him will be `gapM` ahead of him as he
//               reaches the frame line (0: the moment that man will be first at the line — the second gun is through half a second behind the first); and a man who comes within `shortM` of one sent before him, with that man in front of him, eases off until there is
//               room. Nobody is ever kept longer than `cap` seconds either way — a file that cannot form goes as the old one did
//   THE GUN     (`gun` — SHIPS OFF, see the knob) a barrel is not carried, and a weapon light not shone, through a door. A teammate WALKING on an order whose aim point lies across a
//               leaf standing off its frame, inside the gun's reach of him, carries the gun up along the way he goes until the leaf is behind it (never
//               onto the back of one of his own in that line: `gunLane`), then onto his point; where the gun as it lies is in a leaf, or he stands with his point across one, it comes down to the
//               low ready until the line is clear (Guard.muzzleDown: the pose, the hysteresis and the fire gate a friend in the line already has). Any
//               man under one of Sam's orders with his long gun at the shoulder, on a walk of this module or not — but never a man with an enemy in
//               his sights (`gunOver`, off), nor a drill's or a show's cast
// The walk itself is the runner's own (squad.ts → guards.ts followPath): this module only keeps the man's path two points long — a point `look` metres ahead
// of him on his line, then his goal — so his legs, his pace, his lean and his collisions are everybody's. followPath's stuck watchdog still has him: when it
// side-steps or re-plans, the walk stands back for a moment and is re-laid from where that left him; twice, and the nav has him for good (`gaveUp`).
// But for a point moved out from behind a leaf it changes no point of domination, and it changes no order and no gun: where the men end up and what they
// do there is the door order's — but for the gun lowered at a leaf.
import { SideMap, SideSet } from '../math/side';
import type { Game, Guard } from './game';
import type { Door } from './doors';
import type { DoorOp } from './doortools';
import type { Vec3 } from '../math/vec';
import { RUST_SURFACE, goneWithTick } from './crossing';

export const DOORPATH = {
  on: false,
  /** where the lane cuts the frame line: this share of the opening from the hinge (the leaf's root is at 0). 0.52 since the doors are real size
   *  (scene/doorSize.ts): between a leaf standing square on its stop and the latch stop a man's body has 0.325 to 0.556 m of a 0.86 m leaf's frame line to
   *  put his middle on, and the lane runs down the middle of that (0.6, as it was for the 1.16 m leaves, ran him a hand off the latch stop) */
  at: 0.52,
  /** the lane's ends: metres short of the frame line, metres past it */
  outM: 0.7, inM: 0.7,
  /** how far ahead of him on his line the point he walks at is kept */
  look: 0.5,
  /** no leg nearer than this to a leaf's centre line as it stands (the door's own contact starts at 0.365) */
  leafClear: 0.45,
  /** the arc round a leaf's free tip is laid this far out from it (a man walking at a point ahead of him cuts a corner by a hand or two) */
  tipM: 0.75,
  /** let go when the man before him will be this far ahead at the frame line · eased off inside this of a man sent before him (stopped 0.35 nearer).
   *  0 and 0.6 = the next man goes the moment the man before him will reach the line first, and gives way only to a body's width: the four cross inside
   *  about 1.1 s, half a second apart. (0.5 and 1.05, as first built, spaced them 0.6 s apart and brought the second man through the depot's first door
   *  0.2 s after the old entry did — his partner fought the room alone for that long, and the depot lost 24 teammates in 576 over 144 seeds) */
  gapM: 0, shortM: 0.6,
  /** the most a man is held back, seconds: at the stack, and again on the way */
  cap: 2.5,
  /** a point behind a leaf is moved at most this far · and stays this far from another man's point */
  moveM: 0, apartM: 0.7,   // (moveM 0 since the doors rules share this tree: where a body rests behind a leaf is theirs; 1.8 = the walk moving a point by itself)
  /** the gun comes down where its line would cross an open leaf · how far that line is taken from his chest (the shouldered muzzle stands ~0.8 m out) · a leaf this
   *  near its frame (rad) is the wall's: the stack's own carriage is for that · the line is also tried where he will stand this many seconds on ·
   *  and this far either side of his centre · with an enemy in his sights the rule leaves him alone unless this is set.
   *  THE GUN RULE SHIPS OFF, set by its tally (144 seeds a mission, the page's settings, the same seeds with and without it): with it on the office
   *  kept 23 fewer teammates of 576 (interval 0 to 47) and the depot 12 fewer (inside the scatter) — a gun at the low ready has no sights and takes its
   *  light off the room, 2.7 s an entry in the office. What it buys is real: in the office two men carry rifle and light through the server door's leaf
   *  for half a second in every run without it (rifle spells of 0.2 s and over 370 → 175, weapon light 339 → 17; the depot 27 → 0). Wanted in its
   *  place: a carry that pulls the rifle in and keeps eyes and light up — no such pose exists */
  gun: false, gunM: 0.95, gunAjar: 0.12, gunLead: 0.3, gunSide: 0.15, gunOver: false,
  /** the gun rule also has a man who STANDS (on his mark, in the stack, holding his point): off, only a man on his way somewhere is touched — a standing man
   *  keeps his gun where his order has it, through a leaf or not (a lowered gun takes his sights and his light off the room) */
  gunStill: true,
  /** the gun rule may bring a gun DOWN to the low ready: off, it only ever carries the gun up along a walking man's way (nothing is lowered: a gun at the
   *  low ready sees nothing, lights nothing and fires nothing, and a contested doorway is where that costs men) */
  gunDown: true,
  /** the gun is carried along his way only while nobody of his own stands in that line (a gun turned onto the back of the man ahead in the file is a gun
   *  the room fight then lowers for him): with a friend there it stays where his order has it */
  gunLane: true,
  /** a man down within this of a point still being walked to: the point is moved off him (0: never — a man walks onto the fallen as before) · the new
   *  point stands this far from where any man down lies · and at most this far from the point as dealt */
  bodyM: 0, bodyOff: 0.6, bodyMove: 1.2,
};

interface Walk {
  gd: Guard; op: DoorOp; door: Door; pod: Vec3; n: number; t0: number;
  /** his line (pts[0] where he stood when it was laid), its running length, how far along he is · the leaf angles it was laid against */
  pts: Vec3[]; len: number[]; s: number; laidT: number; near: { d: Door; angle: number }[]; lane: number;
  /** what this module last put under him (the watchdog's hand shows as a change) · standing back until · how often · seconds held short · off his order since */
  via: Vec3[]; arr: Vec3[] | null; arrN: number; yieldT: number; yields: number; held: number; offT: number; round: boolean;
  /** a leaf by his line is still swinging: his line ends at the lane's end until it stands · seconds waited so */
  wait: boolean; waited: number;
  /** the lane's inner end as it was laid (metres past the frame line) · his point as the door order dealt it (a moved point is looked for round this one) */
  inM: number; pod0: Vec3;
  /** when his point was last looked at for a man down on it */
  bodyT: number;
}

const flat = (a: ArrayLike<number>, b: ArrayLike<number>) => Math.hypot(a[0] - b[0], a[2] - b[2]);

export class DoorPath {
  walks = new Map<Guard, Walk>();
  /** per door order: the walk last let go, how many so far */
  private lastOf = new SideMap<DoorOp, { W: Walk | null; n: number; t: number }>();
  stats = { sent: 0, walked: 0, round: 0, noRound: 0, noLane: 0, late: 0, heldS: 0, capped: 0, yields: 0, gaveUp: 0, taken: 0, relaid: 0, turned: 0, moved: 0, waitS: 0, nav: 0, gunS: 0, guns: 0, gunAhead: 0, stepped: 0 };
  constructor(private g: Game) {}
  reset() { this.walks.clear(); this.lent.length = 0; }

  /** THE GUN: every teammate under one of Sam's orders, his long gun up at the shoulder — its line across an open leaf has it carried along his walk, or
   *  brought down (and kept down while it would cross). What it writes on his script is his for THIS frame's pose only: `after()` puts it back */
  private low = new SideSet<Guard>();
  private lent: { gd: Guard; s: NonNullable<Guard['script']>; mine: Vec3; aimAt: Vec3 | null | undefined; face: number | null | undefined; strafe: boolean | undefined }[] = [];





  /** doortools.ts, the entry: may the queue's i-th man be let go now? One at a time, in the order dealt, each once the man before him has his start. */
  mayGo(op: DoorOp, q: { gd: Guard; pod: { pos: Vec3 } }, i: number): boolean { return goneWithTick('DoorPath.mayGo') as any; }

  /** the order's own way round (orders.ts route: waypoints to its door) is this walk's while it lasts: taken off the order, kept, and handed back if the
   *  walk is dropped before he is through */
  private own(W: Walk) { const A = this.g.orders.of(W.gd); if (!A || !A.post || flat(A.post, W.pod) > 0.1 || !A.via.length) return false; W.via = A.via.slice(); A.via.length = 0; A.goal = [A.post[0], A.post[1], A.post[2]]; return true; }
  /** once a frame, after the orders have posed their men and before anybody moves */
  tick(dt: number): void { if (RUST_SURFACE?.tapFiring('DoorPath.tick')) return RUST_SURFACE.tapAnswer() as any; return goneWithTick('DoorPath.tick') as any; }
}
