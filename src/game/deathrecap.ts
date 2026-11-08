/** EVERY DEATH EXPLAINS ITSELF. It is the Rust game's: rust/game_rules/src/deathrecap keeps who of ours fell, to whom, from where and with what and the floor's last seconds
 *  (the end card is told as it opens: src/view/debriefview.ts), and what the squad knows of a man lost while the run is on (`g.recap.lostLive`, on the bridge). The panel is
 *  src/ui/recap.ts. Here: the table the knobs carry, and how the HUD reads the squad's own. */
import type { Game } from './game';
export { MAN_STRIDE } from '../view/debriefview';
export type { RecapCause, RecapMan, RecapDeath, RecapShot, RecapFrame, RecapDoor, RecapTake, DeathRecap } from '../view/debriefview';

export const RECAP = {
  on: true,
  /** seconds of the floor kept before a death */ secs: 8,
  /** seconds kept after it (he is seen to fall, the shooter to move on) */ after: 1.2,
  /** samples a second */ hz: 20,
  /** deaths kept with their own last seconds (a player's is never the one dropped) */ keep: 6,
  /** WHILE THE RUN IS ON, squad's view only (`lostLive`): a teammate lost has his row struck, and — only if somebody still up SAW him go down or HEARD the
   *  shot — the squad KNOWS the way it came from, to the nearest eighth, and the man who saw says the side. Never who, never from where; a man found later is
   *  only down. Whether that way is also DRAWN (an arrow on his row, a stroke on the ground at him) is `liveArrows` below: off as shipped. Off = the row as it was. */
  live: true,
  /** …and whether that way is DRAWN: the arrow on his row and the stroke on the ground at him. False, as the page has it (the maintainer, 10-14: "we shouldn't
   *  show the arrows, feels unrealistic"): the row is struck and the man who saw still says which side — nothing is drawn. ?recaparrows=1 = both back. */
  liveArrows: false,
  /** saw him go down = a line to him, within this far, inside this long of the fall · heard = gunfire (a burst) whose sound path to an ear is this long at most */
  liveSeeM: 25, liveSawSecs: 1, liveHearM: 25,
  /** the beat before the squad has it, s · the man who saw says which side only with the player this near the fallen man (his shoulders mean nothing further
   *  off) · how long the stroke on the ground stays, s */
  liveBeat: 0.6, liveSideM: 8, liveMarkSecs: 20,
};

/** One of the squad lost while the run is on, as much as the squad knows: `from` = the way off him to where the round (the blast) came from, a unit vector on
 *  the plan to the nearest EIGHTH of the compass — a direction, never a place, and never fine enough to lay a ruler along — none until the squad has it
 *  (`knownAt`), and for good when nobody still up saw him go down or heard it. */
export interface LostMan { id: number; name: string; t: number; at: [number, number]; from?: [number, number] | null; knownAt: number; }
/** the game's own counts of it, as the bridge brings them */
export const RECAP_STATS = { deaths: 0, takes: 0, rounds: 0, known: 0, calls: 0 };

const NONE: readonly LostMan[] = [];
/** The squad's men lost so far this run, as the squad knows them (the game's record; the HUD's read while the run is on). */
export function lostLive(g: Game): readonly LostMan[] { return RECAP.on && RECAP.live ? g.recap.lostLive : NONE; }
/** is the live rule drawing at all (the squad block strikes every man of the squad who is down while it is: the record only adds the way it came from, drawn only under `liveArrows`) */
export function lostShown(): boolean { return RECAP.on && RECAP.live; }
/** is the way a lost man's round came from DRAWN: the arrow on his row, the stroke on the ground (RECAP.liveArrows; ui/overlay.ts asks before either) */
export function lostArrowsShown(): boolean { return lostShown() && RECAP.liveArrows; }
export function lostOf(g: Game, id: number): LostMan | null { for (const L of lostLive(g)) if (L.id === id) return L; return null; }
