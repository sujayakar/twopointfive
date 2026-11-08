// THE SQUAD REACHES YOU, AND THE QUICK KEY SAYS WHY. Five rules, a switch each, all on in the page and off here and on the frozen stand:
//   SQUADNET.on     A TEAMMATE'S LINE SAID WHERE YOU CANNOT SEE IT STILL REACHES YOU. A bubble was drawn only over a head on the screen: with the squad two
//                   rooms off, 'stacked up.', 'can't get there.' and a room called clear were said to nobody. With the rule the line is drawn NEAR YOU, on
//                   the side he is on, with his name and an arrow his way (ui/bubbles.ts), and the net clicks in your ear as it arrives. A TEAMMATE GOING DOWN
//                   is a row of its own ('<name> is down.', drawn the same way from where he fell) with a sound of its own: the net's click, pitched down.
//                   Nothing here writes a man's bubble: his line is the line he said, and only where it is drawn changes.
//   SQUADNET.edge   A FAR LINE SITS AT THE EDGE OF THE SCREEN, ON HIS SIDE. The first rule drew it a short way out from you, in the middle of the picture
//                   and on top of what you were looking at. With this one it is drawn where the way from you to him leaves the view, a little inside the
//                   border (ui/netplace.ts; ui/bubbles.ts draws by it). Off: near you, as the first rule placed it. The page's alone, like the first.
//   SQUADNET.why    THE QUICK KEY SAYS WHY WHEN ITS PICK IS NOT THE PLAIN ONE (autopilot.ts picks the entry on a door the men wait on). The pick was shown and
//                   answered ('bang and clear · 2 left', 'bang and clear.'); the reason it held was shown nowhere. With the rule the quick key's row ends on
//                   the reason and the man who answers says it: an enemy the team can see inside, a room nobody has been in, the last stun kept back for a
//                   room they know is held, a door they know is shut. A plain open says nothing more. No press waits on any of it.
//   SQUADNET.clear  'CLEAR' MEANS CLEAR. A room was called clear when no enemy stood in it, with floor the search never saw. With the rule the call is made
//                   only when the search ended by its own bar (DOORTOOLS.deadCells: less floor unseen than sends a man on); with a search cut short and
//                   nobody in sight there, the lead says the room is quiet and that they did not see all of it. An entry onto open floor has no search
//                   and is called as it was.
//   SQUADNET.endCall  EVERY ENTRY ENDS WITH A CALL. The lead called only a room the mission has a name for (the room clear's own rooms): an entry into any
//                   other room (the office's conference room, a hall, every room of a mission with no room list) ended with nobody saying anything. With
//                   the rule the lead calls those too, by the same rule and of the room fight's own flood: 'room clear.', 'room's quiet. we didn't see
//                   all of it.', and of open floor, which nobody searches, 'clear so far.' while no enemy on it is in the team's sight. A lead who
//                   ends outside every room of the mission's calls the named room the entry's own points are in. Sam's own squad only: the response
//                   team runs the same door orders and says none of it.
// WHAT IT COSTS: `on` and `edge` are the page's alone (where a line is drawn, a click): no man's state, no die, no digest. `why` and `clear` change WORDS
// a man says, so the length of his bubble, which bark clocks read: a run with either on is not the run with it off by digest once such a line is said.
// `endCall` adds a line where there was none: the same.
// SWITCHES: ?squadnet=0 · ?netedge=0 · ?qwhy=0 · ?honestclear=0 · ?endcall=0 on the address; the frozen stand holds all five off (TPS_SQUADNET=1 ·
// TPS_NETEDGE=1 · TPS_QWHY=1 · TPS_HONESTCLEAR=1 · TPS_ENDCALL=1 raise them). A recording from before a key replays with it off (journal.ts applyKnobs).
// STATE: one SquadNet on the Game (rows, the men already called down, what is owed the ear this frame): in the rollback snapshot with everything else the
// Game reaches. THIS FILE ONLY READS THE GAME, as cues.ts does.
// CHECK: bun run tools/qa/squadnet-check.ts (probes.ts squadnet), both stands; SQUADNET_BREAK=1 must fail.
import './seats';
import type { Vec3 } from '../math/vec';
import type { Game } from './game';
import './guards';   // (asked inside a call only)
import { crossToRust, goneWithTick } from './crossing';

export const SQUADNET = {
  /** a teammate's line said off your screen is drawn near you, the net clicks, and a teammate going down is called */
  on: false,
  /** the quick key's row and the man who answers it say why the pick is not a plain open */
  why: false,
  /** a room is called clear only when its search was not cut short */
  clear: false,
  /** an entry into a room the mission has no name for ends with the lead's call too */
  endCall: false,
  /** how far from you a line from off the screen is drawn, as a share of the view's shorter side (with `edge` off) */
  near: 0.24,
  /** a line from off the screen is drawn at the edge of the view on his side, not near you */
  edge: false,
  /** how far inside the view's border such a line is kept, as a share of the view's shorter side */
  edgeIn: 0.04,
  /** the click a line from off the screen arrives with (gain; 0 = silent), pitched up off the defenders' net's own squelch (cues.ts plays the same voice at rate 1) */
  click: 0.3, clickRate: 1.3,
  /** a teammate down: the same click, louder and pitched down (gain; rate) */
  downGain: 0.7, downRate: 0.55,
  /** seconds a 'down' row stands */
  downSecs: 5,
};

/** a row of the squad's net that no living man's bubble carries (a man down): where it came from, and until when */
export interface NetRow { text: string; at: Vec3; t: number; dur: number }
const newStats = () => ({ lines: 0, downs: 0, whys: 0, quiets: 0, clears: 0, ends: 0 });

export class SquadNet {
  rows: NetRow[] = [];
  stats = newStats();
  private down: string[] = []; private seeded = false; private clickDue = false; private downDue = false; private lastT = -1;
  constructor(private g: Game) { }
  reset() { this.rows = []; this.down = []; this.seeded = false; this.clickDue = this.downDue = false; this.stats = newStats(); }

  /** a teammate said a line with his head off your screen (guards.ts say asks, with the switch on): the net clicks */
  line() { this.stats.lines++; this.clickDue = true; }

  /** once a frame, after everybody has moved: who went down this frame, rows that have had their time, and the one sound owed */
  tick(): void { return crossToRust('SquadNet.tick', this, []) as any; }
}


/** the call a lead makes when an entry is over (doortools.ts): `left` = floor of the room its search was cut short of (0 where it ended by its own bar, and
 *  where the entry had no search of its own: open floor), `inside` = a living enemy stands in the room, `seen` = one of them the team has in sight. Null:
 *  nothing is said. */
export function roomCall(g: Game, name: string, left: number, inside: boolean, seen: boolean): string | null { return goneWithTick('roomCall') as any; }

/** the call at the end of an entry into a room the mission has no name for (doortools.ts): `open` = open floor (a hall, a yard: nobody searches it), the
 *  rest as roomCall's, counted of the room fight's own flood. Null: nothing is said — and with the switch off nothing ever is, as it was. */
export function plainCall(g: Game, open: boolean, left: number, inside: boolean, seen: boolean): string | null { return goneWithTick('plainCall') as any; }
