// CLEAR THE LINE (CLEARLINE.on — off as shipped; `?clearline` for a visit, Tab ▸ Sandbox ▸ Line of fire, CLEARLINE=1 in the harness's environment).
//
// What it answers (rebuild/line-of-fire/fix.md has the probes' numbers): a teammate under one of Sam's orders whose shot at a man HE CAN SEE is masked by a
// friend — Sam between the gun and its man, or a mate — already holds the round and lowers the gun (roomfight.ts friendInLine / samInLine: untouched here,
// the friend's lane is exactly as wide as it was). What he did next was nothing: roomfight's own side step is handed to his script for one frame and taken back
// by the order that poses him the frame after (orders.ts re-orders every man every frame), and with a MATE in the line the runner's muzzle discipline has his
// gun down before the fire gate is ever reached, so no step is even asked for. He stood, gun down, in front of a man aiming at him.
//
// On, he does what a trained man does about a masked gun, in this order, and SAYS which:
//   STEP   the nearest spot within `reachM` he can walk straight to from which the round is clear of every friend by the fire gate's own test, clear of
//          furniture / glass / a jamb (the shot's own test), off every open doorway, a shoulder's width off every body, out of every other gun's line (a
//          mate's on his own man, Sam's along his reticle) and no nearer the man than point-blank. Beside the line, back off it, or UP LEVEL WITH THE FRIEND
//          (a corridor too narrow to step square: two guns abreast) — whichever is the shortest walk, a spot that closes on the man costing more. He walks
//          it at `pace` (under the team's walking pace: nobody runs), squared to the man, and shoots from it; the man gone, the order's own legs are his again.
//   COVER  no such spot: the nearest place a crouched chest is hidden from the man (suppression.ts coverFrom — the pinned man's own search), for `coverSecs`
//          and then for as long as the place he left is still masked, `holdMax` at most; a line that opens on the way there is taken (he plants and shoots).
//   KNEEL  neither: he goes down on a knee where he is, says the friend is in his line, and takes the step the moment the floor opens; he stays down
//          while the line he would stand back up into is still masked, `holdMax` at most.
// After `holdMax` the order's legs are his again and the rule leaves him be for `rest` s (the base behaviour): an order always ends.
// A MATE in the line is a narrower case than Sam in it (clearLineBlocked): a file, a stack, an entry put one man behind another on purpose.
// A man walking on an order (to a stack mark, in file behind Sam) is answered exactly as a planted one: the step is taken out of his walk and the walk goes on
// after. A silent squad holding its fire on a man who has not noticed it is not masked (silent.ts): nothing here runs for him. A man held short in the dark
// on a lit man (LIGHTS.litMan, roomfight.ts holdShort) takes the step but never cover or the knee: the dark he stands in is that rule's cover.
//
// CLEARLINE.inEntry false (true as shipped = all of the above everywhere): none of it for a man between the shove and his point on a door tool's order —
// the seconds a bang buys are the entry's (`entering`, which only READS doortools.ts's record of the order).
// CLEARLINE.holdBehind 2 (as shipped; 0 = all of the above, the rule as first built): no cover and no knee while the friend in his line is AHEAD OF HIM AND CLOSING ON THAT
// SAME MAN — the friend is in that fight, and a man who leaves it for cover loses the man he was about to shoot. The step is still taken; with none he
// holds behind the friend as the base does, says the friend is in his line, and has the ladder back that many seconds after the friend stops closing.
// CLEARLINE.still 2 (0 as shipped = all of the above): none of it — the step included — while the PLAYER in his line is on the move, nor for that many
// seconds after: what a mate on the move has always had (he clears the line himself). The rule is then for a player who has stood in a line.
// CLEARLINE.holdStep (true as shipped; with holdBehind): the hold behind a lead who is walking at the man stops the STEP as well — a man in file keeps his place.
// (Those two ship ON since the lost-surprise tally: as first built the switch cost the office rescue 25 of 144 runs with the defenders warned; with them 1.)
// CLEARLINE.stillFar 8 (0 as shipped; with still): `still` only while his man is farther off than this — close in he gets off the line at once.
//
// The seams: roomfight.ts asks clearLineBlocked() where it asked for its own side step (and once more for a mate in the line with the gun already down);
// orders.ts asks clearLineFeet() as it poses each man and clearLineLive() where it keeps its stuck clocks. State is a WeakMap here: nothing in a checkpoint or
// the digest, and with the switch off every export returns on its first line.
// (src/game is one import cycle: nothing below is built from another game module's exports at module load.)
import { SideMap } from '../math/side';
import { Vec3 } from '../math/vec';
import type { Game, Guard } from './game';
import type { DoorOp } from './doortools';
import type { Assignment } from './orders';
import './guards';
import './suppression';
import './silent';
import { crossToRust } from './crossing';

export const CLEARLINE = {
  /** the switch (false = the build as it was, run for run) */
  on: false,
  /** s a friend has masked his shot before he moves: a breath, not a wait (roomfight's own side step asked at 0.5 s — FIGHT_SKILL.operative.line) */
  after: 0.3,
  /** m: how far a step to a clear line may be */
  reachM: 3,
  /** m/s he walks it at (never over the team's own walking pace, guards.ts PACE.max) */
  pace: 2,
  /** m: no nearer than this to anybody where he ends up, and along the way */
  bodyM: 0.6,
  /** m: the spot must clear the friend's lane by this much to spare (the lane itself is the fire gate's, untouched) */
  spareM: 0.15,
  /** nowhere to step: go to where the man cannot see him, for this many seconds / kneel where he is when there is no such place either */
  coverSecs: 3, 
  /** s between two looks for a spot while he is still masked; s without his man in his eyes before the order's own legs are his again */
  again: 0.5, lose: 1,
  /** s at most in cover or on a knee for one masked line — then his order's legs are his again whatever is in the line, and the rule leaves him be for `rest` s
   *  (an order must end: Sam stood in a corridor for a minute does not park a man sent to a door behind a cabinet for that minute) */
  holdMax: 10, rest: 8,
  /** s on a spot he cannot shoot from after all (the man behind glass or a jamb from there) before the step is void */
  stale: 1.5,
  /** SAM AT THE LANE'S EDGE. s his shot has been masked WITHOUT A BREAK before he moves, when the only friend in the line is Sam and only by the wider lane
   *  a player on the move is owed (0.45 m standing, up to 0.95 m walking — the fire gate's, untouched): stood still where he is, the round would pass him.
   *  That line mostly opens by itself inside a quarter second. Never shorter than `after`; 0 = no such case, the rule as it was run for run. */
  afterEdge: 0,
  /** BEHIND A FRIEND WHO IS CLOSING ON THE MAN, seconds (2 as shipped). 0 = the rule as first built, run for run. More = neither cover nor the knee while the friend in his line is
   *  ahead of him and walking AT that same man (0.5 m/s or more toward him, and more toward him than across the line — the number a mate 'on the move' is
   *  read by), nor for this many seconds after he was last seen doing so: the friend is in that fight and the line is his to open. He takes the step if there is one; with none he holds behind
   *  the friend, eyes on the man, as the base does, and says the friend is in his line. Cover took both men of a pair out of a corridor fight their
   *  commander was walking them into; the man lived and shot one of them walking back (rebuild/line-of-fire/fix.md, hostage rescue seed 5). At 1 the
   *  pair still left as their commander halted, a second before he stepped off the line himself, and lost a man in 5 of 144 rescues; 2 gives the halt
   *  its beat. */
  holdBehind: 2,
  /** … AND NO STEP EITHER (with `holdBehind` above 0; true as shipped). false = the hold above stops cover and the knee only, as it was run for run. true = the step as
   *  well: a man in file behind a lead who is walking at that man keeps his place in the file — he neither steps out from behind his lead nor leaves
   *  for cover; the lead is going to the man and the line is his to open. A friend CROSSING the line, or stepping off it, is not leading: for him the rule
   *  is as it was. With surprise lost the office rescue's losing runs all opened with a follower's STEP out from behind a commander walking down the
   *  corridor at the Door man, 13 m off (rebuild/line-of-fire/fix.md). */
  holdStep: true,
  /** SAM ON THE MOVE, seconds. 0 = the rule as it was run for run. More = NOTHING of this rule's (no step, no cover, no knee) while the player in his line
   *  is really getting somewhere (0.5 m/s or more of displacement, any direction — the number a MATE 'on the move' has always been read by: he clears the
   *  line himself), nor for this many seconds after he was last seen to: he holds as the base does, eyes on the man, and says Sam is in his line. The rule
   *  is then for a player who has STOOD in a line. With surprise lost the office rescue's losses all opened with a follower stepping out from behind a
   *  commander walking down the corridor at the man, and every cover in them was for a commander crossing the line (rebuild/line-of-fire/fix.md). */
  still: 0,
  /** … ONLY AT RANGE, metres (with `still` above 0). 0 = `still` at any range, as it was run for run. More = `still` only while his man is farther
   *  off than this; nearer, the rule is as first built whatever the player is doing. Close in a masked half second is the fight, so he gets off the line at
   *  once; down a long corridor a player on the move opens the line himself before a far man has lined up, and a man who breaks file there takes a gun
   *  off the corridor. 8 = aim.ts `rangeRef`: the range at and past which a man needs his whole time to line up (nearer, the target fills the sights and
   *  it goes quicker in proportion). The rule's acts in the levels it gains on are mostly 2 to 6 m from the man; in the office rescue's corridor all of
   *  them are 8 m or more (rebuild/line-of-fire/fix.md). */
  stillFar: 0,
};
export const CLEARLINE_STATS = { blocked: 0, steps: 0, up: 0, planted: 0, covers: 0, kneels: 0, cues: 0, ended: 0, nothing: 0, behind: 0, moving: 0 };
export interface ClearNote { t: number; who: string; what: 'step' | 'cover' | 'kneel' | 'end'; by: string; at: [number, number] | null; walk: number }
export const CLEARLINE_LOG: ClearNote[] = [];


type What = 'step' | 'cover' | 'kneel';
/** what he is doing about it: where, about which man, since when; `onT` = when that man was last his (roomfight's target), `until` = a cover's end */
interface Doing { what: What; spot: Vec3; of: Guard; t0: number; onT: number; until: number; askT: number; lookT: number; from: Vec3; orderT: number;
  /** when he should be on the spot by (a walk that is going nowhere is void) · when he last had the round from where he stands */
  eta: number; goodT: number;
  /** the gate's point on the man was his HEAD (the chest is behind something chest-high): what every later look at this line is asked of too */ head: boolean }
/** `restUntil`: the rule leaves him be until then (base behaviour: his order's legs) · `tryT`: nothing was found — the next look */
interface Mem { since: number; seenT: number; saidT: number; said: string; doing: Doing | null; restUntil: number; tryT: number; /** masked without a break, on this man, since (the edge wait's clock) */ runT: number; runOf: Guard | null;
  /** when the friend in his line was last seen closing on the man (holdBehind) · where that friend stood the last time it was asked, and when */ leadT: number; leadP: [number, number, number] | null; leadOf: unknown; /** … and on which man (a lead seen on one man is not a lead on the next) */ leadFor: Guard | null }
const mem = new SideMap<Guard, Mem>('clearline.mem');
/** what a LIVING man is doing (a man who dropped mid-step holds no spot against the others and is not on the panel) */
const liveDoing = (gd: Guard): Doing | null => (gd.state === 'dead' || !gd.script ? null : mem.get(gd)?.doing ?? null);   // (… nor a man the fight AI has: nothing of his is being walked)
/** THE PLAYER IS ON THE MOVE (still): LEAD_V m/s or more of real displacement, any direction, now or inside the last `secs` seconds. Read off where he
 *  really got to between two asks a tenth of a second or more apart, by any masked man (the pace asked of his legs the first time and after a gap nobody
 *  was masked in: a walk into a wall is no walk). One record a game — every man reads the same commander. */
const samAt = new SideMap<Game, { p: [number, number, number]; moveT: number }>('clearline.samAt');
/** the entry assignment each man was first seen on, per door order (`past`: he has been on his point since, or been sent on from it) */
const entered = new SideMap<Guard, { op: DoorOp; A: Assignment; past: boolean }>('clearline.entered');

/** the panel's line */
export function clearLineSummary(g: Game): string {
  if (!CLEARLINE.on) return 'off';
  const S = CLEARLINE_STATS; const now = g.guards.filter(gd => !!liveDoing(gd)).map(gd => `${gd.callsign} ${liveDoing(gd)!.what}`);
  return `on · steps ${S.steps} (${S.up} up level) · cover ${S.covers} · knee ${S.kneels} · said ${S.cues}${now.length ? ` · now: ${now.join(', ')}` : ''}`;
}
/** what a man is doing about it this moment (the probes', the panel's) */
export function clearLineOf(g: Game, gd: Guard): { what: What; spot: Vec3; of: Guard } | null { return crossToRust('clearLineOf', undefined, [g, gd]) as any; }
