// A TEAMMATE SHOWS WHY HE IS NOT ACTING (SHOWWHY.on — ON in the page, `?showwhy=0` puts it off; OFF here and on the frozen stand, TPS_SHOWWHY=1 raises it
// there; FROZEN_HOLDS has the row). tools/qa/showwhy-check.ts (probes.ts showwhy; SHOWWHY_BREAK=1 fails it), run on both stands.
//
// What it answers: a teammate who was not doing the plain thing looked exactly like one who was idle. His gun went down because the player stood in his
// line and nothing said so; a stack that was made and waiting on the word stood like one still
// walking up; a man a blast had hurt fell behind with nothing but a hunch; a man kept on a taken room's far door after 'fall in' (rearwatch.ts) read as a
// man who had not heard the order. The only standing read of any of it was a row of words in the corner panel.
//
// On, the man himself shows it: his body or his hand first, a short line second, and NOTHING when he is doing what he was asked.
//   LINE    his round at a man he has is held because the PLAYER is in its line (the fire gate's own test, asked again here: roomfight.ts friendInLine) for
//           `lineAfter` s, and no mate of his stands in that line nearer than the player does (a man in file behind another waits on HIM: one man waves,
//           not four in step): his free hand comes off the gun and waves the player off the line (the animator's 'aside'), to the side of it the player
//           is on; once more `waveEvery` s later, then only every `waveLater` s. "you're in my line." stands over him for as long as it is true — one
//           voice at a time, and `lineSayGap` s before another man takes it up. His eyes stay on his man: a head is what a man sees with (guards.ts,
//           fogofwar.ts), so nothing here turns it. Not while a fire hold has the round anyway (a silent squad, a squad nobody has found, the sneak): no
//           round is wanted, so none is masked.
//   STACK   a door order that is MADE AND WAITING (doortools.ts: 'ready' on the word, stacked behind a gate, a charge armed, the jambs or the outside held):
//           the squeeze goes up the file from the last man (each man who faces a mate in reach ahead of him, `squeezeGap` apart; a man with his back to
//           the file on rear security keeps his hands; none with the animator's squeeze off — it would play as a fist), then the lead's fist goes up — and again after `holdEvery` s, twice that the next time, `holdMax`
//           times in all (a fist every few seconds for minutes is a metronome). Hands only: the lead's own words ('set.') are the door tools'.
//   DOOR    his walk is held short of the PLAYER standing in a doorway (doorway.ts's own hold and its own answer to 'for whom', `doorAfter` s; not in a
//           fight): the knife hand that
//           points the way ('go'), and "after you." — once in `afterYouGap` s for the whole squad. A mate in the doorway is not said: that wait is a
//           second long and the men sort it out.
//   HURT    a blast left him standing and hurt (condition.ts; the hunch and the short step are that file's): "I'm hit.", once, when he is back on his
//           feet. Under 'fall in' with the player at a run and him `lagM` behind: "I can't keep up.", `lagEvery` s apart and `lagMax` times a man — a
//           hurt man is capped at a walk.
//   KEEP    he keeps a taken room's far door while the squad leaves it (rearwatch.ts roomKept; his feet and his gun are that file's): "I've got this
//           door.", `keepGap` s after a mate's when two doors are kept, and not again from him inside `keepEvery` s.
// showWhyOf() is the standing read for anybody who wants to say it another way (a menu row, a voice): what holds this man right now, or null.
//
// Nothing here moves a man's feet, his gun, his eyes or a round, draws a die or reads the camera: it reads what the other files decided this frame and
// writes his left hand (a signal: picture only) and his line. The line is guards.ts say, as every squad line is: a bubble, which the run digest carries
// and the bark clocks read like any other line of his (so a run with the switch on is not digest-equal to one with it off once a line is said; with the
// bubbles left out it is — the check's 'same' line). A line waits until his own words are spent and is never laid over another line of his.
// Asked once a frame after the orders, the door tools and the room fight (game.ts). Off = every export returns on its first line: the build as it was, by
// run digest. State is side tables here (a snapshot holds them), each stamped with the clock so a restart or a load voids it.
// (src/game is one import cycle: nothing below is built from another game module's exports at module load.)
import { SideMap } from '../math/side';
import type { Game, Guard } from './game';
import type { DoorOp } from './doortools';
import type { SignalKind } from './character';
import './character';
import { isHurt } from './condition';
import { roomKept } from './rearwatch';
import './doorway';
import './unseen';
import './silent';
import './holdfire';

export const SHOWWHY = {
  /** the switch (false = the build as it was, run for run) */
  on: false,
  /** each case by itself, for an A/B (all true as shipped) */
  stack: true, hurt: true, keep: true,
  /** LINE. s the player has masked his round without a break before he shows it · a break shorter than this is no break · s from his first wave to his
   *  second · s between waves after that · the line stands at least this long once said · s after one man's line before another man says it · m either
   *  side of his line a mate counts as standing in it */
  lineAfter: 0.6, lineGap: 0.3, waveEvery: 2.6, waveLater: 6, lineMin: 1.2, lineSayGap: 4, mateHalf: 0.45,
  /** STACK. s between two squeezes up the file · m a hand reaches the man ahead · s after the last squeeze (or, with none, after the stack is made) before
   *  the lead's fist · s to the second fist (doubled each time after) · fists in all for one wait */
  squeezeGap: 0.35, squeezeReach: 1.15, fistAfter: 0.45, holdEvery: 8, holdMax: 3,
  /** DOOR. s held at a doorway the player stands in before the hand · s before the same man shows it again · s between two "after you." from the squad */
  doorAfter: 0.5, doorEvery: 4, afterYouGap: 10,
  /** HURT. "I'm hit." is not said later than this many s after the blast (a load, a man folded for a long time) · m behind a running player before
   *  "I can't keep up." · s between two of those from one man · how many he says in all */
  hurtLate: 6, lagM: 6, lagEvery: 20, lagMax: 2,
  /** KEEP. s he has kept the door before he says so · s after a mate's keep line before his · s before the same man says it again */
  keepAfter: 0.4, keepGap: 1.5, keepEvery: 30,
  /** a line owed is dropped when it has waited this long for his own words to be spent */
  owedFor: 5,
};

/** what holds a man from the plain thing */
export type Why = 'line' | 'stack' | 'door' | 'hurt' | 'keep';
/** the lines, one place (written as the squad's other lines over a head are: no capital to open on) */
export const SHOWWHY_LINES = {
  line: "you're in my line.",
  door: 'after you.',
  hurt: "I'm hit.",
  lag: "I can't keep up.",
  keep: "I've got this door.",
};
export interface WhyNote { t: number; who: string; why: Why; signal: SignalKind | null; line: string | null }
export interface ShowWhyStats { waves: number; squeezes: number; fists: number; ways: number; lines: number; holds: number }

interface Owed { text: string; why: Why; by: number }
interface Mem {
  /** the clock as last seen (a restart or a load takes it back: everything below is void) */
  t: number;
  /** LINE: masked since · last masked · it is showing · when it last stopped showing · when the next wave may go · waves so far (a showing that
   *  follows the last by a few seconds goes on counting: a man let go of and taken again does not start over) · the stamp of the bubble that is this rule's
   *  (NaN: none) */
  lineT0: number; lineSeenT: number; lineOn: boolean; lineEndT: number; waveT: number; waves: number; lineB: number;
  /** DOOR: held since · last held · shown this hold · when he may show it again */
  doorT0: number; doorSeenT: number; doorShown: boolean; doorT: number;
  /** HURT: the blast (`hurtAt`) his line has been said for · when he may next say he cannot keep up · how often he has */
  hurtSaid: number; lagT: number; lagN: number;
  /** KEEP: kept since (NaN: not kept) · queued this keep · when he last said it */
  keepT0: number; keepSaid: boolean; keepSaidT: number;
  /** lines owed (his own words were still over him), the oldest first: each said once they are spent and its cause still stands, dropped at `by` */
  owed: Owed[];
}
/** a door order that is made and waiting: the clock as last seen · the squeezes still to go up its file · when the lead's fist next goes up · fists so far */
interface OpMem { t: number; turns: { gd: Guard; at: number }[]; fistT: number; fists: number }
interface State { stats: ShowWhyStats; log: WhyNote[]; afterYouT: number; keepSayT: number; lineSayT: number; t: number }

const mem = new SideMap<Guard, Mem>('showwhy.mem');
const opMem = new SideMap<DoorOp, OpMem>('showwhy.opMem');
const states = new SideMap<Game, State>('showwhy.states');
const fresh = (): ShowWhyStats => ({ waves: 0, squeezes: 0, fists: 0, ways: 0, lines: 0, holds: 0 });

/** what the rule has shown this game (the check's, the debugger's) */
export function showWhyStats(g: Game): ShowWhyStats { return states.get(g)?.stats ?? fresh(); }   // (a read makes no state: a panel asking with the switch off leaves the snapshot as it was)
export function showWhyLog(g: Game): WhyNote[] { return states.get(g)?.log ?? []; }
/** a fresh encounter: nothing shown, nothing owed */
export function resetShowWhy(g: Game): void { states.delete(g); }


/** the side of his line to his man the player stands on: 1 his left, -1 his right (the way the wave sends him: the short way out) */
function lineSide(g: Game, gd: Guard): 1 | -1 {
  const h = g.fight.opOf(gd)?.tgt, c = gd.char.pos, pl = g.player.char.pos; if (!h) return 1;
  const dx = h.char.pos[0] - c[0], dz = h.char.pos[2] - c[2];   // (his left of that line is (dz, -dx): the model's +x under a yaw of atan2(dx, dz))
  return ((pl[0] - c[0]) * dz - (pl[2] - c[2]) * dx) / Math.max(1e-3, Math.hypot(dx, dz)) >= -0.15 ? 1 : -1;   // (on the line itself, or a hand off it: the wide push out to his left, the one that reads from overhead)
}


/** What holds this man from the plain thing right now, or null — the standing read (a menu row, a voice pass, a check). Null with the switch off. */
export function showWhyOf(g: Game, gd: Guard): Why | null {
  const K = SHOWWHY; if (!K.on || gd.state === 'dead' || !g.orders?.mine(gd)) return null;   // (the player's own men: a defender a blast left hurt is not this rule's)
  const M = mem.get(gd), t = g.time, live = !!M && t >= M.t && t - M.t <= 0.25;   // (a memory the tick has not touched lately — a showcase began, he has no order — says nothing)
  if (live && M!.lineOn) return 'line';
  if (live && M!.doorShown && t - M!.doorSeenT <= 0.3 && t >= M!.doorSeenT) return 'door';
  if (K.keep && roomKept(g, gd)) return 'keep';
  if (K.stack) for (const op of g.doorTools.ops) { const R = op.men.includes(gd) ? opMem.get(op) : undefined; if (R && t >= R.t && t - R.t <= 0.25) return 'stack'; }
  if (K.hurt && isHurt(g, gd)) return 'hurt';
  return null;
}
