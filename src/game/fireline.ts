// OFF THE LINE OF HIS FIRE (FIRELINE.on). The maintainer's card, after friendly fire came in: "Teammates never walk into your line of fire" — "a teammate
// about to cross your line while you are firing waits a beat short of it and crosses when you stop."
//
// Friendly fire (friendlyfire.ts) made a round of Sam's side a round to the men of Sam's side, and gave a teammate the care not to PRESS with a friend where
// his round could go. Nothing gave him the other half of that care: an order walked him across the lane the player was shooting down, and the round found him.
// On, a teammate WALKING on an order (to a mark, in the file behind Sam, falling in) whose next steps would take him into the lane of the player's fire stops
// `shortM` short of it and goes on when the fire stops:
//   firing    a gun in the player's hands and a round out of it inside the last `fireS` (one careless shot is a second's wait, a string is as long as it lasts —
//             through a magazine changed in the middle of it and `reloadS` after; slow aimed fire is a string too: `paceS`, `paceK`). Rounds are counted off
//             the gun itself, once a frame (fireLineTick): a launcher's load or a thrown thing is no round
//   the lane  the round's own, as friendlyfire.ts friendInCone reasons about a teammate's: from the player's back along the gun's heading, a body (`FRIENDLYFIRE.bodyM`)
//             and `clearM` either side, widening by the share of its spread a round can be thrown (`FRIENDLYFIRE.coneK` × `spread`, the widest his hands give a
//             string on the move), as far as a round flies — through what it pierces, nothing solid between
//   the wait  where he stands, the order's own goal kept, at no speed, saying so once; `maxS` at most. After that he looks (every `againS`) for a way to his
//             place that keeps out of the lane, goes through no doorway and is no more than `wayMoreM` longer — behind the player, as a rule — and walks it
//             at the order's own pace; with none he waits on.
//             The fire stopped, or the gun swung off his way: the order's own legs, at once. A lane that comes back inside `lapseS` (the gap before a
//             second slow round) is the same wait: nothing said twice, the ceiling's clock kept — and if it comes back with him already walking in it, he
//             walks on out. A man waiting whom the gun swings ONTO stands as he stood, where the player can see him, to the ceiling.
// What it never does, by the card ("Left out: any change to your aim, to auto aim or to the enemies"): no round is stopped, bent or passed; the player's gun
// is not touched; a man ALREADY in the lane when the fire starts is not moved (he walks on out of it, as he would have); a hold is not a shove — careless fire
// still hits. And never: a man with a fight of his own (a man in his sight, on his gun, or just lost), a man past the stack on a door order (the count is the
// entry's), a man on a wall charge (its stand-off and its entry are the charge's), a man in a doorway (he is committed through it), a man stepping out of Sam's own way or mid-crossing for the silent squad, a man who would be
// left standing where an enemy he knows of has a line to him (his own picture, picture.ts), a show's cast. And not at all with friendly fire off (`FRIENDLYFIRE.on`: the
// player's round is then no round to his own men, and there is nothing to keep out of). Nobody runs: the way round is walked at the pace the order gave him.
//
// The seam is orders.ts, where every ordered man's feet are posed each frame: fireLineTick() before the men, fireLineFeet() just before each pose, fireLineLive() where the order keeps its
// stuck clocks (a man waiting on purpose is not a man who cannot get through). The room fight's own lane (roomfight.ts friendInLine / holdShort / sideStep —
// a TEAMMATE's shot masked by a friend) and clearline.ts (his step off that line) are other rules and are not read or changed here.
//
// ON in the page (`?nofireline` = as it was), OFF in the module and on the frozen harness stand (`TPS_FIRELINE=1`): a FROZEN_HOLDS row, in the knob registry
// and a journal's tables; a recording from before the table replays with it off. State: two named side tables (a man's wait and his way round; when the
// player's fire was last live), in a rollback snapshot and in no checkpoint — a load in the middle of a wait begins it afresh.
// No dice, no clock but the game's. (src/game is one import cycle: nothing below is built from another game module's exports at module load.)
import { SideMap } from '../math/side';
import type { Vec3 } from '../math/vec';
import type { Game, Guard, Player } from './game';
import './friendlyfire';
import './glasspen';
import './wallpen';
import './doorway';
import './picture';

export const FIRELINE = {
  /** the switch (false = the build as it was, run for run) */
  on: false,
  /** s: the player is firing while a round left his gun inside this long — and through a reload begun while he was, and `reloadS` after it (a magazine
   *  changed in the middle of a string is the string) */
  fireS: 1.0, reloadS: 0.7,
  /** slow aimed fire is fire too: two rounds inside `paceS` of each other, and the lane outlasts the last of them by `paceK` × the gap between them (at most
   *  `paceS`) — a round every second and a bit is one string, not a string of chances to cross */
  paceS: 2.5, paceK: 1.5,
  /** s: a lane that comes back this soon after it lapsed on a waiting man is the same wait (no second word; the ceiling's clock kept) */
  lapseS: 2.5,
  /** the widest a round of his is thrown (combat.ts fireWeapon's `spread`): the rifle on the move, a string's growth and the aim model's cone on top */
  spread: 0.06,
  /** m: room either side of the round's lane on top of a body's (FRIENDLYFIRE.bodyM) — he stops clear of it, not on its edge */
  clearM: 0.25,
  /** m: he stops when his way enters the lane inside this far of walking (`keepM` more once he is waiting: no dithering on the edge) */
  shortM: 1.4, keepM: 0.4,
  /** m: …and, however shallow his way comes at it, before his next step is this near the lane's side (`lookM`: as far up his way as the lane is looked for) */
  gapM: 1.0, lookM: 6,
  /** he says so once, `cueS` into the wait (a man who stops for no reason he gives reads as stuck; a stop that short is not worth a word) */
  cue: true, cueS: 0.35,
  /** s: the longest he waits before he looks for a way round, and how often he looks again */
  maxS: 4, againS: 2,
  /** m: a way round may be this much longer than the way across; the floor this near the lane is dear to the search for one (over half a nav cell's diagonal: a corner cut between two cells clear of it stays out of the lane) */
  wayMoreM: 10, berthM: 0.4,
  /** m: a way round is asked of a lane this much wider than the one a wait keeps off (a walker cuts his corners), and — once he is on it — of the round's own lane, `clearM` narrower (the gun sways) */
  edgeM: 0.15,
  /** m: the lane's length at most */
  reachM: 40,
};
/** counted, read by nothing in the game: waits begun, their seconds, the words said, ways round taken, looks that found none, frames a wait was not begun or kept for a fight of his own / for an enemy's line */
export const FIRELINE_STATS = { holds: 0, heldS: 0, cues: 0, ways: 0, noWay: 0, fight: 0, enemy: 0 };

interface St { /** when the wait began */ since: number; /** the last frame the rule had his feet */ t: number; /** the way round, corner by corner (the goal object handed to the pose is `way[0]`) */ way: Vec3[] | null; /** when a way round was last looked for */ lookT: number;
  /** when the lane lapsed with him waiting (he walks on; a lane back inside `lapseS` is the same wait), -1 = held */ goT: number; /** he has said so */ said: boolean }
const states = new SideMap<Guard, St>('fireline.states');
/** per player: the gun his rounds are counted on and its count, when the last round left it and the gap before that one, the last frame his fire was live, and
 *  the last frame a reload was what kept it so */
interface Fired { gun: string; shots: number; roundT: number; gap: number; liveT: number; relT: number }
const fired = new SideMap<Player, Fired>('fireline.fired');



function end(gd: Guard, st: St | undefined): null { if (st) states.delete(gd); return null; }
/** he stands: the wait's seconds counted as they pass */
function hold(g: Game, st: St): { speed: number } { FIRELINE_STATS.heldS += Math.min(0.1, Math.max(0, g.time - st.t)); st.t = g.time; return { speed: 0 }; }

