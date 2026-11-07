// WHO HAS SEEN WHAT FLOOR (doortools.ts DOORTOOLS.leadSeen): for each teammate, when every cell of the nav grid was last inside HIS eye's wedge with a line to it.
// One question is asked of it — of the men about to go through a door, who has already seen most of the room behind it (the footage, the Door Kickers narrator:
// 'i'm going to lead with this man because he's already seen most of the room he knows what it looks like and where the threats are at'; 'peanut here has
// already seen most of the room so we're going to prioritize him entering').
//   The eye is the sight test's own (guards.ts GUARD_EYE: 58° either side of where chest and head point between them, 100° inside 3.5 m), the line is the sight
//   test's own (col.segmentBlocked: statics and door leaves stop it, glass does not), the reach is the door tools' (`leadSeenM`: floor further off than that is
//   not floor he could say anything about). No light is asked for: what he knows from having looked is the room's shape and where a man could stand.
// It is a RECORD: it writes nothing but itself, draws no dice, and moves nobody — kept while the rule is on (or a probe asks for it: DoorTools.seenWatch), each
// living man's brought up to date every EVERY frames, the men staggered; a cell whose line was asked about inside the last REFRESH seconds is not asked about
// again, whichever way it came out (a man costs one wedge of line tests a second, some hundreds, however long he stands).
// (src/game is one import cycle: nothing here is built from another game module's exports at module load.)
import type { Game, Guard } from './game';
import './guards';


export class FloorSeen {
  /** per man: the game time each nav cell was last seen (NEVER: not yet) · and when its line was last asked about */
  private at = new Map<Guard, Float32Array>(); private asked = new Map<Guard, Float32Array>();
  private frame = 0;
  constructor(private g: Game) { }
  /** how many of `cells` (nav cell indices) `gd` has seen since game time `since` */
  count(gd: Guard, cells: Iterable<number>, since: number): number { const A = this.at.get(gd); if (!A) return 0; let n = 0; for (const i of cells) if (A[i] >= since) n++; return n; }
}
