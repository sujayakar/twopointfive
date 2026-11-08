// UNASKED COVER (Game.cover, __game.cover): what the teammates watch while they FOLLOW Sam or HOLD with nothing ordered to look at — the maintainer, 10-04, playing ?squad4:
// "as we're progressing through a room, the AI should handle watching all the breaches automatically". The pair and the four alike, in every scenario where an
// operative unit falls in (the room clear, the four, the rescue, the night breach): nothing here knows which one is up.
//
//   AN OPENING     a way a man could come at the squad. Found ONCE per level off what the level already is — no authored marks, so a new level file has them
//                  for nothing: every DOOR (open or shut, at its frame), and every GAP in the nav grid — a run of free floor at most COVER.gapCells wide between
//                  two blocked cells (a doorway with no leaf, the slot between two partitions, a narrow aisle). Short gaps (a hole in a wall) are one opening;
//                  a longer passage has a MOUTH at each end; a blind one deeper than COVER.alcoveRows has one. Plus, while men follow, THE REAR: the way they
//                  came, COVER.rearM back along Sam's own trail past the last man (orders.ts trailBack).
//   LIVE           an opening is somebody's to watch only while it is within COVER.reach of a free man who has a line to it (the collision's own sight test:
//                  glass passes, a wall does not) AND a clear lane (none of the squad within COVER.lane of the line from him to it — a man does not cover
//                  through a mate's back: orders.ts and guards.ts would bring his muzzle down and he would cover nothing), with nobody of the squad standing in
//                  it or either side of it (the file is going through it). A passage looked into from outside is ONE opening, its nearer mouth; with one of
//                  ours inside it, its two mouths are his front and his back. A shut door is as live as an open way.
//   THE DEAL       a few times a second: every live opening weighed (the nearer to the squad the heavier; a shut door a little less than an open one; one Sam's
//                  own gun is on, or a man Sam posted already has, counts for little — "the player's own facing counts as covered"), and the free men dealt one
//                  each so that the heaviest are taken first and NO TWO MEN TAKE THE SAME ONE (every dealing tried: there are four men). Who takes which: the man
//                  who has to turn least and is nearest; a man keeps a NEW one COVER.keepSecs whatever else comes up, and after that unless a re-deal is clearly better
//                  (COVER.sticky); a man with nothing takes the least thing going rather than nothing. With three or more in
//                  the file the LAST man's is the rear whatever else is open; with fewer the rear is weighed with the rest (two men: nearest and most dangerous
//                  first). On the move, what has fallen BEHIND a man who is not the last counts for little to him (COVER.behindK): it is the last
//                  man's before it is his. Re-dealt as the squad moves: an opening passed or walked through drops out, the next one comes in.
//   THE MAN        keeps the feet his order gives him (his place in the file, his spot on a hold) — only his gun, his light and his eyes go: stopped, he squares
//                  up to his opening with the gun on it; walking, the gun is on it over his shoulder while it is within a twist of the way he goes (COVER.twist),
//                  and past that — the rear, an opening the file has just come by — he walks backwards or sideways with it in front of him at a walk
//                  (COVER.backIn / backOut) and turns to run when Sam does (a man whose order has him running is dealt nothing behind him). The gun TRAVELS to a
//                  new opening, round his front (COVER.swing). What he then SEES is what his eyes see (guards.ts canSeeChar off his own chest and
//                  head): a man stepping out of a covered opening is in his cone, in his light, and on the line his gun is already held on (roomfight.ts: no
//                  surprise beat, the pre-aim).
//   SPARE EYES     two more men are dealt to, on their marks: a STACK's last man (COVER.stacked — orders.ts spareRear: the door is the others', his is the
//                  way a man could come at their backs) and the men an ENTRY has left holding a room (COVER.inRoom — doortools.ts: the room's other doors,
//                  not the wall across the floor they have just searched). Assignment.spare says so; with nothing dealt the order's own watch stands
//                  (not at once — REAR below: a gun the deal has just let go of stays on its opening three quarters of a second first).
//   AN ORDER WINS  only men under 'fall in', or under a 'hold' that names nothing to watch, are dealt to. Any other order of Sam's, a drill that owns the man
//                  (F), the rescue's flow sending him to a mark, the fight AI having him: he is not free and his job goes that deal — and he is dealt to again
//                  when he is handed back ('fall in'), with no word from Sam.
//
// The seam is in orders.ts tick() (poseFor: the watch point, the facing, whether he back-pedals) and the view's squad line (the block says what each man is
// on: 'Red One · with you · on the server door'; a man's line in the view: its base, then this tail). Off (COVER.on false, ?nocover): tick() returns on its first line, poseFor
// answers null and the men are the base's, frame for frame.
//   A STEP TO SEE  the one place the cover moves a man's FEET (COVER.step): stood still, dealt an open door he has no line through (its leaf is between him and
//                  the doorway) or nothing with such a door near him, he takes a step or two to where he sees through it, never into an opening, and the
//                  doorway is then his (stepDeal / stepSpot; orders.ts walks him).
// NOT DONE: nobody steps off his place to get a LANE, or a line to anything but an open door (a man in the middle of a straight file can be boxed in by the men
// ahead and behind and take nothing — `live` says what went uncovered), a room known clear does not lower its doors' weight, the men's lights stay as their order has them (dark under
// 'fall in' in the room clear), nobody calls what he takes aloud (the rescue flow's own men still do).
// (src/game is one import cycle: nothing below is built from another game module's exports at module load.)
import { orderOf, teamMen, viewOf } from './seats';
import { SideMap } from '../math/side';
import { Vec3, v3 } from '../math/vec';
import type { Game, Guard } from './game';
import type { Door } from './doors';
import { doorWords, type Assignment } from './orders';
import { WORDS } from './words';
import { watchedPlace } from './dialogue';
import { DOORMANNERS } from './doorway';
import { rearPinOf } from './rearwatch';
import './gunroles';   // (called inside tick only)
import { type Watchable } from './roomfit';
import { RUST_PASS, RUST_SURFACE } from './crossing';

/** The knobs (__game.cover.knobs). */
export const COVER = {
  /** off = nothing here runs: followers and holders look where the order alone has them look */
  on: true,
  /** a STACK's last man (the last two of four) is dealt an opening too — the way a man could come at the stack while it walks up and waits, the others' guns
   *  on the door (orders.ts spareRear; off: every man of a stack faces the door) · the men an ENTRY has left holding a room are dealt its openings (off: each
   *  stays on the sector his point of domination gave him, across the room he has just searched). REAR SECURITY SHIPS ON, ROOM COVER OFF: over 24 seeds of six
   *  scenarios (tools/qa/playthrough.ts; the numbers are in rebuild/polish/log.md) the rear man halves the men shot from outside their arc and takes the
   *  rescues from 40 of 48 to 46 for a few rooms fewer of 96; turning the men in a taken room to its doors gained nothing and cost rooms */
  stacked: true, inRoom: false,
  /** THE REAR MAN AT A MARK WITH NO DOOR READS THE GROUND (a wall charge's stand-off, the mate covering a man at work): off, his rear is 'the way he
   *  came' — at a stand-off that is the spot he stood on when the order was given, two or three metres off, so both rear guns rested on bare ground or a
   *  yard wall (the maintainer, floodlit: "weird that these guys are covering a wall") and nobody had the open gate or the run along the building. On,
   *  once he is within `rearReadIn` of his mark he takes the heaviest thing roomfit.ts's read (`watchablesAt`, out to `rearReadM`) says a man there could
   *  watch — a way in, a long run of floor — that he has a line to with no mate in it, and that the other rear man has not got (the same thing, or one
   *  within `rearReadDeg` of his bearing); looked at again every `rearReadSecs`, changed only for something `rearReadSticky` × heavier; the gun is put at
   *  most `rearLook` m out along it. Nothing there to watch = the old rule. Off in the module (the frozen stand); on in the page. */
  rearRead: false, rearReadIn: 1.0, rearReadM: 14, rearReadDeg: 35, rearReadSecs: 1.0, rearReadSticky: 1.3, rearLook: 6,
  /** the rear man takes the rear only inside this many metres of his mark: further out on the walk to the stack he keeps up with the file, gun forward —
   *  or on a door standing open by his way (doorway.ts DOORMANNERS.watch: `walkOnly / walkDoor` below) — (a
   *  large number = the rear is his the whole walk: he came the whole way backwards, metres behind his mate, and the pair reached the door apart). 5, not
   *  the 3 it first shipped at: over 72 seeded runs the last three metres alone left 98 teammates of 192 standing where 5 m leaves 119, 8 m 121 and the
   *  whole walk 119 — the men who come at a stack come while it is still forming. At 8 the stack forms a second and three quarters later than at 3, and
   *  two pairs going on one word lost two men on a seed (pairs-go) */
  rearWalkM: 5,
  /** a door standing OPEN is a man's to watch only if he can see INTO its doorway: from the side its leaf has swung to, the leaf is between him and whoever
   *  steps through (off = as it was: a line to short of the frame was enough, and a man was dealt a doorway he was looking at the back of the door of) */
  leaf: true,
  /** … and where the leaf does hide the doorway from him, the leaf's FREE EDGE is his to watch instead, if he has a line to it: whoever comes through that door
   *  comes into his view there — but only when he has nothing else to watch, and the doorway stays live for a man with a line into it: a man standing IN the
   *  doorway is hidden from the edge's watcher (false: he watches nothing, as it was) */
  edge: true,
  /** an opening further than this from a man is not his; the deal is redone this often (game seconds) */
  reach: 9, dealSecs: 0.25,
  /** a free run of nav cells (half a metre each) this wide or less between two blocked cells is a gap · a passage up to this many cells long is ONE opening,
   *  longer has a mouth at each end · a blind passage this deep or deeper has a mouth worth a gun · a passage runs on past this many rows where it widens */
  gapCells: 3, throughRows: 3, alcoveRows: 3, bridge: 3,
  /** the squad's own men this near an opening are IN it (it is being walked through, not watched) */
  inM: 0.9,
  /** one of the squad's own within this of the line from a man to an opening: it is not his to take (m; `laneKeep` for the one he already has — taken clear
   *  of where orders.ts's own muzzle lane lets a gun back up, laneOut 0.7, and kept clear of where it brings one down, laneIn 0.45) · a man STOOD on his opening
   *  keeps it this long with a mate crossing his line (s: his muzzle dips and comes back; he does not turn away and back) */
  lane: 0.7, laneKeep: 0.5, laneSecs: 0.6,
  /** Sam's gun within this of an opening he has a line to: his · what such an opening, and one a posted man has, still weighs (a spare man doubles up on it) */
  samDeg: 35, samK: 0.2, heldK: 0.1,
  /** the weights: a shut door against an open way · the rear · how fast nearness tells (m) */
  shutK: 0.7, rearK: 0.8, nearM: 3,
  /** a man who had a living enemy in sight this recently is the contact's, not an opening's (s) */
  contactSecs: 4,
  /** the rear: this far back along the trail past the last man · the file this long gives its last man the rear outright */
  rearM: 4, rearFrom: 3, rearDeg: 60,
  /** what a man already on an opening has over a rival for it · he KEEPS a new one this long whatever else comes up, while it stays live and his to take (s) ·
   *  the last man keeps the rear thing he has unless another behind him is this much heavier */
  sticky: 1.2, keepSecs: 1.5, rearKeep: 1.25,
  /** on the move, an opening behind a man who is not the file's last weighs this much of itself to him */
  behindK: 0.2,
  /** walking: the gun goes over his shoulder to an opening within this of the way he goes (rad); past it he walks backwards / sideways while his pace is under
   *  backIn (m/s; he turns and runs over backOut): set over a walking Sam's file closing a gap, under a running Sam's — the men cover while Sam walks and run
   *  when he runs. (Entering the back-pedal mid-rush, backIn 2.3‥2.6, is what tipped orders-edges' lean-out check: see cover.md) */
  twist: 1.45, backIn: 3.0, backOut: 3.4,
  /** a man ALREADY walking backwards on his opening does not turn to walk forward while his body still faces away from the way he is going (its cosine
   *  to that way under `backBody`: 0 = more than a quarter turn off it). The frame he walks forward he walks along his BODY (the path follower's rule while a man turns), so with his body to the rear he
   *  went backwards, which read as 'going the other way' and put him back in the back-pedal — and that read as going forward again: a change of mind every
   *  frame, the two turns cancelling, two seconds on the spot beside a door. He keeps back-pedalling the fraction of a second his body takes to come round to
   *  his opening (which is within `twist` of his way, so inside that), then walks forward. Off = as it was. KEEP `backBody` UNDER cos(twist) (0.12 at twist 1.45) OR THE HOLD NEED
   *  NOT END: a body squared to an opening that far round from his way would still read as facing away. (At −0.3 the walk is the same but for the
   *  moment he lets go; a number, no Tab row.) */
  backKeep: true, backBody: 0,
  /** …and a man walking backwards whose opening the deal LETS GO OF (nobody of ours has taken him, nothing else re-poses him) comes out of it on his feet: the
   *  order's own pose is 'walk forward', which a man walks along his BODY — still turned to the rear, so he went a quarter of a metre back the way he came and
   *  hooked round, three quarters of a second gone, every time (48 times on the registry level's walk with four behind Sam). His feet keep the path he is on
   *  (the runner's strafe) while his body comes round to the way he goes, his gun in front of it: until it is within `backTurnBody` (a cosine: 0.7 = an eighth
   *  of a turn) of that way, or `backTurnSecs` are up; stood, run on or dealt a new opening he is that rule's again (a new job meets him as a man walking
   *  backwards, so backKeep holds him the same way). Off = as it was. (The man run on over backOut still turns as he did: a number, no row.) */
  backTurn: false, backTurnBody: 0.7, backTurnSecs: 1,
  /** how fast the muzzle travels to a newly dealt opening (1/s: it travels, it does not snap) */
  swing: 9,
  /** STEP TO SEE: a door standing open by the squad that NOBODY has a line into (every free man is behind its leaf or off its line; the leaf's edge is the
   *  best the deal can hand out) gets one man: stood still, with nothing or only that edge to watch — else on something that weighs less than the door — he
   *  takes the step or two to a spot he does see through it from, and is then dealt the doorway itself. His FEET are the order's everywhere else in this
   *  file; here alone the cover moves them: a follower only while Sam stands (he falls back in when Sam walks), a plain holder by walking to the spot
   *  while his mark stays where the order put it (orders.ts asks stepOf; the next order takes him off it). Never into an opening: the spot is on his own side of that door, `stepOffM` clear of every opening's width, off a mate's lane,
   *  reached in a straight walk that crosses none. (off = he stands where his order put him, with the leaf's edge or something further off: as it was)
   *  · no further than `stepM` from the place his order gave him, however many steps · only once he has stood `stepAfter` s · a door further than `stepReach`
   *  from him is not his to step for · a step not made in `stepSecs` is given up where he stands
   *  · a FOLLOWER steps only from his place: stood within `stepPlaceM` of where 'fall in' has him (0 = from wherever he stands still, as it was — a man stopped
   *  metres short of his place was dealt a step there and kept it for as long as Sam stood)
   *  MEASURED (10-06; rebuild/away-build/polish-items/open-door.md): `probes.ts cover-step` — a pair behind the leaf of every office door that can stand open:
   *  off, 0 of 8 see through and nobody moves; on, 7 of 8 are dealt the doorway, median 2.8 s after the order. The scripted play-throughs hardly meet it (their
   *  commander does not stand by open doors: 3 steps begun in 168 runs) and are the same within a run or two — 106 of 168 won either way, 263 / 264 of 432 up —
   *  so they show no harm, not a gain. Every number on this line is a first guess that passed the probe. */
  step: true, stepM: 1.8, stepAfter: 0.8, stepOffM: 0.7, stepReach: 6, stepSecs: 5, stepPlaceM: 0.6,
};

/** ONE JOB AT A TIME (the knobs: Team feel ▸ 'rear ▸ …'). The deal is redone four times a second and an opening is 'live' for a man only while a dozen things
 *  hold that frame (a line to it, nobody of ours in it, the file not going through it, nobody better placed) — so a job was handed out, taken back a quarter
 *  of a second later and another handed out the deal after. Between the two the man's gun went back to what his ORDER has him watch: for a stack's rear man the
 *  door, for a follower the way the file is going. Rear, door, rear, door: the play-through watcher's 'twitch' (four reversals in 2 s).
 *  `settle`: a gun the deal has let go of STAYS on its opening for `secs` before the order's own watch has it back, the man carrying himself for it as he was
 *  (squared up to it stood, over his shoulder or still backing on the walk); a job dealt inside that takes the gun straight from the one opening to the other.
 *  The deal itself is not touched — who is dealt what, and when, is as it was (a first cut that kept the JOB a while cost the deal its freedom to re-deal).
 *  It ends at once when anything else has him (an order that leaves him no spare eyes, the entry, a fight, a contact: free() says so), when Sam or a mate is
 *  in the line of the gun or stood in the opening, when he is told to run with it behind him. settle false = the cover as it was, frame for frame
 *  (threshold-identity pins it off). `file`: the same for a follower / a holder, not a stack's rear men only — SHIPS OFF: it takes most of the file's own
 *  twitches too, but depot-four loses teammates with it on every seed set tried.
 *  Measured (tools/qa/playthrough.ts, the two four-man scenarios × 72 seeds, off → on): a stack's rear men's twitches 151 → 8, all twitches 397 → 214,
 *  won 113 → 116 of 144, teammates up 365 → 376 of 576 (inside the scatter). With `file` on as well: 10, 149, 117 and 335 (depot-four 207 → 180 of 288). */
export const REAR = { settle: true, secs: 0.75, };

export type OpeningKind = 'door' | 'gap' | 'mouth' | 'rear';
/** One opening. `at`: its middle, chest high (a door: its frame's centre) · `n`: the unit line THROUGH it in plan (which way is beyond depends on who asks) ·
 *  `half`: half its width · `door`: its leaf, when it has one · `pass` / `far`: a mouth of a passage with two — the passage's name and its other mouth. */
export interface Opening { id: string; kind: OpeningKind; at: Vec3; n: [number, number]; half: number; door: Door | null; pass: string | null; far: Vec3 | null }
/** What a man was dealt: the opening, the point his gun is on (this object is his script's aimAt), the bearing and range that point is held at from him (THEY
 *  ease onto the opening: the muzzle sweeps round him, it never cuts through him), since when, whether he is walking backwards on it,
 *  when his lane to it was last clear, `off`: his order has him running with it behind him — he is not on it this frame (the next deal takes it off him), and
 *  the pose his order had for him (put back when an order or a drill takes him without re-posing him). */
export interface CoverJob { o: Opening; to: Vec3; aim: Vec3; yaw: number; r: number; t0: number; aimT: number; back: boolean; laneT: number; off: boolean; pace: number; A: Assignment }
/** A step to see through an open door (COVER.step): `q` where he is going / stands · `from` the place his order gave him (every step of his is within COVER.stepM
 *  of it) · `o` the door · `A` the order it was taken under (another order: it is void) · `t` when it was begun (done: when he got there, or gave it up) · `saw`: he has since
 *  been dealt that doorway itself */
export interface CoverStep { q: Vec3; from: Vec3; o: Opening; A: Assignment; t: number; done: boolean; saw: boolean }
/** One live opening as the last deal saw it: its weight, the free men with a line to it, who has it, and whether Sam's own gun / a posted man was on it. */
export interface LiveOpening { o: Opening; w: number; can: Guard[]; edge: Guard[]; at: Map<Guard, Vec3>; by: Guard | null; edgeBy: Guard | null; sam: boolean; held: boolean }

const EYE = 1.55, CHEST = 1.1;
const flat = v3.distXZ, yawTo = v3.yawTo;
const COMPASS = ['south', 'south-east', 'east', 'north-east', 'north', 'north-west', 'west', 'south-west'];   // yaw 0 = +z = south (orders.ts)

/** The gaps of a nav grid (pure: the grid's blocked cells and its cell size in, openings out — tools/qa/cover-probes.ts draws them). `alongX`: the runs are laid
 *  along x and the passages they stack into run along z; the caller asks both ways. A passage is not ended by a row or two where it WIDENS (COVER.bridge: a side
 *  doorway off a corridor is its own opening, found the other way round — the corridor is still one passage past it). `solid(a, b)`: is the line a → b shut to
 *  SIGHT (the collision's own test) — with it, a run counts only between two things a man cannot be seen over: the slot between a desk and a table is no opening. */
export function gapsOf(nav: { W: number; H: number; cell: number; blocked: Uint8Array }, alongX: boolean, K = COVER, solid?: (a: Vec3, b: Vec3) => boolean): Opening[] {
  const W = alongX ? nav.W : nav.H, H = alongX ? nav.H : nav.W, c = nav.cell;
  const blk = (u: number, v: number) => u < 0 || v < 0 || u >= W || v >= H || nav.blocked[alongX ? v * nav.W + u : u * nav.W + v] === 1;
  interface Run { a: number; b: number; v: number }
  const chains: Run[][] = [];
  for (let v = 0; v < H; v++) {
    for (let u = 0; u < W;) {
      if (blk(u, v)) { u++; continue; }
      const a = u; while (u < W && !blk(u, v)) u++; const b = u - 1;
      if (b - a + 1 > K.gapCells || a === 0 || b === W - 1) continue;   // (a run that reaches the grid's edge is open ground, not a gap)
      const link = chains.find(ch => { const l = ch[ch.length - 1]; if (l.v >= v || l.v < v - 1 - K.bridge || l.a > b || l.b < a) return false;
        for (let w = l.v + 1; w < v; w++) for (let x = Math.max(a, l.a); x <= Math.min(b, l.b); x++) if (blk(x, w)) return false;   // (bridged rows: wider there, not walled)
        return true; });
      if (link) link.push({ a, b, v }); else chains.push([{ a, b, v }]);
    }
  }
  const out: Opening[] = [];
  const pt = (u: number, v: number): Vec3 => alongX ? [u, CHEST, v] : [v, CHEST, u];
  const at = (r: Run): Vec3 => pt((r.a + r.b + 1) * 0.5 * c, (r.v + 0.5) * c);
  const open = (r: Run, v: number) => { for (let u = r.a; u <= r.b; u++) if (!blk(u, v)) return true; return false; };
  // (a wall can stand anywhere across the cells it blocks: the line is tried down the middle of the run's row and along both its edges, each side on its own)
  const side = (ch: Run[], lo: boolean) => !solid || ch.some(r => [0.5, 0.02, 0.98].some(k => { const v = (r.v + k) * c, u = (r.a + r.b + 1) * 0.5 * c;
    return solid(pt(u, v), pt(lo ? (r.a - 0.5) * c - 0.45 : (r.b + 1.5) * c + 0.45, v)); }));
  const n: [number, number] = alongX ? [0, 1] : [1, 0];
  chains.forEach((ch, ci) => {
    const first = ch[0], last = ch[ch.length - 1], L = last.v - first.v + 1;
    const lo = open(first, first.v - 1), hi = open(last, last.v + 1);
    if (!side(ch, true) || !side(ch, false)) return;
    const mk = (kind: OpeningKind, r: Run, other: Run | null): Opening => { const p = at(r);
      return { id: `${kind}@${p[0].toFixed(2)},${p[2].toFixed(2)}`, kind, at: p, n, half: (r.b - r.a + 1) * c * 0.5 + 0.34, door: null, pass: other ? `${alongX ? 'z' : 'x'}${ci}` : null, far: other ? at(other) : null }; };
    if (lo && hi) {
      if (L <= K.throughRows) out.push(mk('gap', ch[ch.length >> 1], null));
      else { out.push(mk('mouth', first, last)); out.push(mk('mouth', last, first)); }
    } else if ((lo || hi) && L >= K.alcoveRows) out.push(mk('mouth', lo ? first : last, null));
  });
  return out;
}

export class Cover {
  readonly knobs = COVER;
  /** every opening of the level (built on the first tick; rebuilt when the level's nav grid is another object) */
  openings: Opening[] = [];
  /** who is on what */
  jobs = new Map<Guard, CoverJob>();
  /** the last deal's picture: every live opening, heaviest first */
  live: LiveOpening[] = [];
  /** the free men of the last deal, in file order (the holders after) */
  men: Guard[] = [];
  /** … `blind`: man-deals (one a man a deal, COVER.dealSecs apart) spent dealt an open door he has no line through — the leaf's edge · `unseen`: door-deals an
   *  open door within COVER.stepReach of a free man had NOBODY with a line into it (not Sam's gun, not a posted man's) · `steps`: steps begun · `stepped`:
   *  made · `stepSaw`: … and the doorway itself then dealt him */
  stats = { deals: 0, takes: 0, swaps: 0, rear: 0, uncovered: 0, kept: 0, blind: 0, unseen: 0, steps: 0, stepped: 0, stepSaw: 0, rearRead: 0, turned: 0 };
  /** who has stepped (or is stepping) to see through which open door */
  steps = new Map<Guard, CoverStep>();
  /** since when each man has stood · no step for him before · no step tried for this door before (game seconds: all three go with a restart — reset()) */
  private stood = new SideMap<Guard, number>(); private noStep = new SideMap<Guard, number>(); private doorTry = new SideMap<Opening, number>();
  /** every change of hands: when, who, onto what (null: let go) — the probes' record (the newest 400) */
  log: { t: number; who: string; id: string | null }[] = [];
  private nav: object | null = null; private dealAt = 0;
  /** the way each man is going (baulked to a stop by the man ahead: the way he WAS going) */
  private heading = new SideMap<Guard, number>();
  private rear: Opening = { id: 'rear', kind: 'rear', at: [0, CHEST, 0], n: [0, 1], half: 1, door: null, pass: null, far: null };
  /** … and one per man whose eyes are spare on a mark (a stack's last man): the way HE came to it */
  private rears = new Map<Guard, Opening>();
  /** COVER.rearRead: what each such man's read last gave him (per order: a new Assignment reads afresh), when, and where he stood */
  private reads = new SideMap<Assignment, { t: number; at: Vec3; pick: Watchable | null }>();
  /** REAR.settle: the job the deal last took off each man, and when — his gun stays on it REAR.secs (poseFor) unless something else has him first */
  private kept = new Map<Guard, { J: CoverJob; t: number }>();
  /** COVER.backTurn: the men coming out of a back-pedal the deal let go of — the gun point carried in front of the body, when it began and the last frame it posed him */
  private outs = new Map<Guard, { J: CoverJob; aim: Vec3; t0: number; t: number; on: boolean }>();
  constructor(private g: Game) { }

  /** nothing dealt, nothing known (a restart: Game.resetGuards) */
  reset() { this.jobs.clear(); this.steps.clear(); this.stood = new SideMap(); this.noStep = new SideMap(); this.doorTry = new SideMap(); this.rears.clear(); this.reads = new SideMap(); this.kept.clear(); this.outs.clear(); this.live = []; this.men = []; this.dealAt = 0; for (const k of Object.keys(this.stats) as (keyof Cover['stats'])[]) this.stats[k] = 0; this.log.length = 0; }
  of(gd: Guard): CoverJob | null { return this.jobs.get(gd) ?? null; }
  /** the opening his gun has been left on (REAR.settle), if it is */
  keptJob(gd: Guard): CoverJob | null { return this.kept.get(gd)?.J ?? null; }

  /** the level's openings, built if they are not yet (entrypoints.ts asks before anybody has fallen in: the list is the statics' alone, the same whenever it is made) */
  ready(): Opening[] { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Cover.ready', this, []); if (__r !== RUST_PASS) return __r as any; } } if (this.nav !== this.g.col.nav) this.build(); return this.openings; }
  /** the level's openings: its doors, and the gaps of its nav grid that are not a door's own doorway */
  build() {
    const g = this.g, nav = g.col.nav; this.nav = nav;
    const doors: Opening[] = g.doors.list.map(d => { const th = d.def.closedDir, fc = d.frameCentre;
      return { id: `door:${d.def.name}`, kind: 'door' as const, at: [fc[0], CHEST, fc[2]] as Vec3, n: [-Math.sin(th), Math.cos(th)] as [number, number], half: d.def.width * 0.5, door: d, pass: null, far: null }; });
    const gaps: Opening[] = [];
    const solid = (a: Vec3, b: Vec3) => g.col.segmentBlocked(a, b);
    // (the STATICS' sight alone decides what a gap is: with the leaves in, a door standing open beside a slot the first time anybody fell in would make an opening
    //  of it for the rest of the level)
    const leaves = g.col.dynamicBoxes; g.col.dynamicBoxes = [];
    try {
      for (const o of [...gapsOf(nav, true, COVER, solid), ...gapsOf(nav, false, COVER, solid)]) {
        if (doors.some(d => flat(d.at, o.at) < 1.0) || gaps.some(x => flat(x.at, o.at) < 0.6)) continue;   // (a door's own doorway · the same hole found both ways)
        gaps.push(o);
      }
    } finally { g.col.dynamicBoxes = leaves; }
    this.openings = [...doors, ...gaps];
  }

  /** is this man free to be dealt an opening: alive, under 'fall in' or a 'hold' that names nothing to watch, on his order's own script (not the fight AI's),
   *  not in a fight, not blind */
  /** his order leaves his eyes spare NOW: a stack's rear man (`from`: the way he came) while the switch is on and a living mate's gun is still on that door —
   *  one man alone on a door keeps his eyes on it — or a man an entry left holding the room */
  private spare(gd: Guard, A: Assignment): boolean {
    if (!A.spare || !A.post) return false;
    if (!A.from) return COVER.inRoom;
    // (on the walk he keeps up with the file, gun forward: his rear is his for the last steps and on the mark — but a door standing open by the way is his the whole walk: walkOnly)
    if (flat(gd.char.pos, A.post) > COVER.rearWalkM && !(DOORMANNERS.on && DOORMANNERS.stand && DOORMANNERS.watch)) return false;
    // (rearwatch.ts: the man an entry left outside on the way behind is not 'alone on a door' — his mates went through it, and their backs are what he keeps)
    return COVER.stacked && (!!rearPinOf(viewOf(this.g, gd), gd)?.out || teamMen(this.g).some(o => { if (o === gd) return false; const B = orderOf(this.g, o); return !!B && !!B.post && !B.spare && B.door === A.door; }));
  }
  private free(gd: Guard): Assignment | null {
    const g = this.g; if (gd.state === 'dead' || !gd.script || g.time < gd.dazzledUntil || gd.held) return null;
    const A = orderOf(g, gd); if (!A) return null;
    if (!(A.order === 'fallIn' || (A.order === 'hold' && !A.watch) || this.spare(gd, A))) return null;   // (… or his order leaves his eyes spare: on its mark, and on the walk to it)
    if (g.fight.engaged(gd)) return null;
    // a contact still warm is his before any opening is: a man who had a living enemy in his eyes inside COVER.contactSecs keeps looking where his order and the
    // fight leave him looking (down the corridor at the man who ducked back), never turned off it to a door behind him
    const O = g.fight.opOf(gd);
    if (O) for (const [h, M] of O.meters) if (h.state !== 'dead' && g.time - M.seenT < COVER.contactSecs) return null;
    return A;
  }
  private eyeOf(p: Vec3): Vec3 { return [p[0], EYE, p[2]]; }
  /** (rearwatch.ts) the point of this opening a man stood at `p` can watch — sight()'s answer from there */
  sightFrom(p: Vec3, o: Opening): Vec3 | null { return this.sight(this.eyeOf(p), o); }
  /** the point of this opening a man at `eye` can watch: the opening itself — or, an open door whose leaf stands between him and its doorway, a point just
   *  past the leaf's free edge (COVER.edge) · null: no line to either */
  private sight(eye: Vec3, o: Opening): Vec3 | null {
    const d = flat(eye, o.at); if (d > COVER.reach) return null; if (d < 0.3) return o.at;
    const k = o.door ? Math.max(0, 1 - 0.7 / d) : 1;
    if (this.g.col.segmentBlocked(eye, [eye[0] + (o.at[0] - eye[0]) * k, CHEST, eye[2] + (o.at[2] - eye[2]) * k])) return null;
    if (!(COVER.leaf && o.door && !o.door.off && Math.abs(o.door.angle) >= 0.5 && this.g.col.segmentBlocked(eye, [o.at[0], CHEST, o.at[2]]))) return o.at;   // (shut, its own leaf stands IN the frame: hence the line to short of it)
    if (!COVER.edge) return null;
    const D = o.door, u = D.dir(), r = D.def.width + 0.35, T: Vec3 = [D.def.hinge[0] + u[0] * r, CHEST, D.def.hinge[1] + u[1] * r];
    return flat(eye, T) > COVER.inM && !this.g.col.segmentBlocked(eye, T) ? T : null;
  }


  private note(gd: Guard, id: string | null) { this.log.push({ t: this.g.time, who: gd.callsign, id }); if (this.log.length > 400) this.log.splice(0, this.log.length - 400); }


  /** an opening in a few words, as this man would say it */
  nameOf(o: Opening, from?: Vec3): string {
    if (o.kind === 'rear') return 'the rear';
    if (o.door) return `the ${doorWords(o.door)}`;
    if (!from) return `${o.kind} (${o.at[0].toFixed(1)}, ${o.at[2].toFixed(1)})`;
    const what = o.kind === 'mouth' ? 'aisle' : 'gap', yaw = yawTo(from, o.at);
    if (WORDS.on) { const p = watchedPlace(this.g.level, from, [o.at[0] + Math.sin(yaw), 0, o.at[2] + Math.cos(yaw)]); return p ? `the ${what} to ${p}` : `the ${what}`; }   // (words.ts: the place it leads to by the level's own name — never a bearing, the HUD has no compass)
    return `the ${what} ${COMPASS[((Math.round(yaw / (Math.PI / 4)) % 8) + 8) % 8]}`;
  }
  /** has he an opening dealt (and is on it, not running with it behind him) */
  has(gd: Guard): boolean { const J = this.onOf(gd); return !!J && !J.off; }
  /** the opening his gun is on: dealt, or let go of and still his for the moment (REAR.settle) */
  private onOf(gd: Guard): CoverJob | undefined { return this.jobs.get(gd) ?? this.kept.get(gd)?.J; }
  /** one line for the console */
  summary(): string { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Cover.summary', this, []); if (__r !== RUST_PASS) return __r as any; } }
    if (!COVER.on) return 'cover: off';
    if (!this.men.length) return `cover: nobody free · ${this.openings.length} openings on the level`;
    const on = this.men.map(gd => `${gd.callsign} ${this.jobs.has(gd) ? this.nameOf(this.jobs.get(gd)!.o) : '—'}`).join(' · ');
    const open = this.live.filter(L => !L.by).map(L => `${this.nameOf(L.o)}${L.sam ? ' (player)' : L.held ? ' (posted)' : ''}`).join(', ');
    return `cover: ${on} · ${this.live.length} live${open ? ` · nobody on: ${open}` : ''} · ${this.stats.takes} takes, ${this.stats.swaps} swaps`;
  }
}
