// B IS THE SQUAD'S AUTOPILOT (AUTOPILOT.on; the maintainer, 10-13: '"B" is the "autopilot" mode where the squad does the reasonable thing (and then maybe
// briefly shows their plan and then fades out), while X is the sweatier mode where you can micromanage them a bit').
//
// The quick key (orders.ts ORDERS.quickKey) used to give the target's FIRST ROW and nothing else: move there on floor, stack up on a door, fall in on
// nothing — and 'stack up' again on a door the men were already stacked on. With the switch on it asks `autoPick` instead:
//   · floor / nothing / a teammate / a door nobody of the addressed men waits on = that first row, as ever;
//   · a door they STAND on a plain 'stack up' at (every man on his mark: a second tap while they still walk is 'stack up' again, not an entry) = THEY GO
//     IN, by the entry they pick (below) — only the men stacked on it (GiveOpts.men): a man left elsewhere is left. (AUTOPILOT.restack, the page's
//     stand: they do NOT go in — the key is 'stack up' again and a man answers that they are stacked; the word's key is the way in);
//   · a door something else of theirs waits on (an order given on the word, a threshold held, a charge set or being set) = the word, as F gives it, to
//     the men on THAT door (the pair, which has no 'go' order: its held threshold's word, or its set charge blown);
//   · a door with a set charge nobody waits on = it is blown (the tools' own row) — never one that is on a word still gathering (doortools.ts
//     onPendingWord: the key refuses that too).
// THE ENTRY, the first that the row's own `valid` takes, from only what the team knows (ONE MODEL: the far room by cleared.ts — has one of them stood in
// it —, an enemy only while the team's fog has him in sight, the door's lock only once a man has tried it):
//   1. the squad is silent (silent.ts): open and clear, and nothing else — the autopilot never makes the squad loud by itself;
//   2. an enemy in sight in the room beyond the door (the fog's own picture where there is one, else the room fight's meters) and a stun left, or a
//      room none of them has stood in and more than `keepBangs` left: bang and clear;
//   3. open (kick, pick: the row's own verb) and clear;
//   4. none of those, loud, and the door known shut: a charge — set, stood off from, and WAITING (the key, or B on it again, blows it).
// Nothing fits: the first choice is given all the same, so a man says why in the row's own words. It never throws a frag, never goes through a wall,
// never sends a second element to a second door: those need the player's intent and are the menu's.
//
// THE PLAN IT SHOWS: a quick-key order's row in the order book is stamped `auto`, and the plan (`view.plan`) draws its men for `showSecs` where the menu's own
// order has PLAN.freshSecs — the player did not pick the entry, so he is given longer to read it. `replan`: the squad's own change of plan with nobody
// asking (doortools.ts haltAt: a door found shut — they stop and stack on it) is drawn the same way (SquadOrders.shows; the module says so the frame it
// re-assigns them). THE HUD'S B LINE (autoHint) is the squad's FIRST choice in the row's own words, asked every frame, so it runs no path search and
// writes nothing: the press itself asks each row's `valid` and may fall back (a mate behind the door: no bang).
//
// PER SEAT, for a second player: the press, its cursor and `who` are the seat's (orders.ts input); nothing here keeps state.
// Off (the module's default, the frozen stand): the quick key is the first row, no row is stamped, nothing more is drawn — the build as it was.

import type { Game, Guard } from './game';
import type { OrderTarget } from './orders';

import { crossToRust } from './crossing';

export const AUTOPILOT = {
  /** the quick key on a door the addressed men wait on sends them in by the entry they pick (false: it is the target's first row, as it was) */
  on: false,
  /** seconds the plan of an order the quick key gave is drawn before it has faded (the menu's own order: floorplan.ts PLAN.freshSecs) */
  showSecs: 4,
  /** a room nobody of the team has stood in gets a stun only while MORE than this many are left: the last is kept for a room they know is held */
  keepBangs: 1,
  /** with no model of the room beyond a door, an enemy in sight this near its doorway on the far face is 'behind that door', metres */
  contactM: 9,
  /** the squad's own change of plan (a door found shut: they stop and stack on it) is drawn and fades as a quick-key order's is */
  replan: true,
  /** THE KEY NEVER PICKS A TOOL (the maintainer, 10-26: "let's also remove the Q vs. F for deciding how to go through a room. F always just does a
   *  basic clear" — a tool is asked for on the ring: "show how to tell the squad to use it with long press Q"): on a door the men are stacked and set
   *  on, the quick key sends THEM in by the plain open and clear, the entry the word's key makes of a stack (doortools.ts fromStack) — never a stun
   *  or a charge of their own choosing. The two keys differ only in who goes: the key is that door's men, now; the word is everyone who waits.
   *  false: the entry they pick (autoEntries: a stun for a room nobody has stood in or one they see held, a charge on a door known shut). Needs
   *  `on`. Off here and on the frozen stand (`TPS_PLAINENTRY=1`, beside `TPS_AUTOPILOT=1`); the page raises it (`?plainentry=0` off) */
  plain: false,
  /** THE KEY ALWAYS SENDS THE ORDER, AND THE WORD'S KEY IS THE WAY IN (the maintainer, 10-26: "let's actually remove that second tap of Q thing.
   *  tapping Q just always sends the order, so tapping Q on a door they're stacked up on would just resend the stack up command, which would then
   *  have them say ("stacked up" or "we're ready to go" …)"): on a door the men are stacked and set on, the quick key is 'stack up' again — nobody
   *  is re-assigned or leaves his mark (orders.ts again), Sam's line is said and one of them answers from `RESTACK_LINES`. Nobody goes in on it:
   *  the word's key does that (doortools.ts fromStack), or a row of the ring. A door something else of theirs waits on (a word, a set charge) is
   *  as it was. false: they go in on it (`plain` says how). Needs `on`. Off here and on the frozen stand (`TPS_RESTACK=1`, beside
   *  `TPS_AUTOPILOT=1`); the page raises it (`?restack=0` off) */
  restack: false,
};
/** what a man answers when 'stack up' is asked of men already stacked and set (AUTOPILOT.restack), by turns */
export const RESTACK_LINES = ['stacked up.', "we're ready to go."];
export const AUTOPILOT_STATS = { presses: 0, first: 0, bang: 0, open: 0, charge: 0, blow: 0, word: 0, refused: 0, shows: 0, again: 0 };

type DoorTarget = OrderTarget & { kind: 'door' };
/** What the quick key would do on a target. `word`: the word, to `men`. `id`: that order — to `men` exactly where it names them (the men on the door),
 *  else to the standing choice. Neither: nothing (a charge on a word still gathering). */
export interface AutoPick { id: string | null; men: Guard[] | null; word: boolean; why: 'first' | 'silent' | 'contact' | 'unknown' | 'known' | 'shut' | 'blow' | 'word' | 'pending' | 'set'; t: OrderTarget; /** a room nobody has been in is opened plain because the last stun is kept back (squadnet.ts says so) */ kept?: boolean }

/** the team has that man in sight (the fog of war's answer where it runs, the room fight's otherwise) */
export function inSight(g: Game, o: Guard): boolean { const F = g.fogOfWar; return F.active ? F.seen(o) : !!g.fight?.teamSees(o); }

/** The entries in the order the squad would take them on a door it is stacked on (`t.side`: the men's face): the first is its choice, the rest what it
 *  falls back to. `shut`: with the charge a door known shut ends on (it asks the door tools what the team knows of the lock — the press; never the HUD). */
export function autoEntries(g: Game, t: DoorTarget, shut = true): { ids: string[]; why: AutoPick['why']; kept?: boolean } { return crossToRust('autoEntries', undefined, [g, t, shut]) as any; }

/** What the quick key does on `t` for the men it would go to. `check`: the entry is the first its row's `valid` takes (a path search per man: the press);
 *  without, the first choice unasked (the HUD's line, every frame: no path search, nothing written). */
export function autoPick(g: Game, t: OrderTarget, check = true, men?: Guard[]): AutoPick { return crossToRust('autoPick', undefined, [g, t, check, men]) as any; }

/** The quick key, pressed on `t`: true when the press was the autopilot's (an order given and stamped as its own in the order book, or refused aloud, or
 *  a charge left alone on its word). False: nothing was there to give. */
export function autoQuick(g: Game, t: OrderTarget): boolean { return crossToRust('autoQuick', undefined, [g, t]) as any; }

/** the quick key's line on the HUD: the row's own words for the squad's first choice on what the cursor is on (null: nothing) */
export function autoHint(g: Game, t: OrderTarget, P = autoPick(g, t, false)): string | null { return crossToRust('autoHint', undefined, [g, t, P]) as any; }

/** seconds the plan of that order-book row is drawn outside the order mode */
export function showSecsOf(row: { auto?: boolean }, fresh: number): number { return row.auto ? Math.max(0.2, AUTOPILOT.showSecs) : fresh; }
