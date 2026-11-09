// GOALS — Shift + the squad's key gives what the cursor is on to a PART of the squad, and the rest go on as they were (cmdring.ts `goals`, `goalRing`).
// This file was plan mode's (the maintainer, 10-26: "let's remove plan mode"): the mode, its chains, its picks, its view and its picture are gone, and
// what is left is the rule for who takes a goal and the key that asks for one. The file keeps its name for the modules that import it.
//
//   · A TAP of Shift + the key on a thing: `planGoal` — the men `goalMen` names take what the key alone gives there (autopilot `quick`: on a bare spot they
//     move there). The maintainer, 11-08: "i think shift Q on a point should just move the squad there, should match what regular Q does. you should need
//     to open the radial menu to get covering a point".
//   · KEPT DOWN, it is the order ring for those men alone (orders.ts ringUp / goalPart, `Menu.part`): its first wedge is the tap's order, and covering a spot is a wedge.
//   · Every order is given through the order book: a refusal and its reason are a man's own. Nothing here moves a man.
//
// STATE: none. PLAN_STATS is a tally for the checks.
import type { Guard } from './game';
import type { Input } from '../host/input';
import type { OrderTarget, SquadOrders } from './orders';
import { CMDRING } from './cmdring';
import { crossToRust } from './crossing';
import { locked } from '../view/seatview';

export const PLAN_STATS = { goals: 0, split: 0, rings: 0, none: 0 };

/** SHIFT + THE KEY ADDS A GOAL (CMDRING.goals): on Sam's own squad — and on a tutorial mission only from the step that teaches it (the frame's action
 *  `goal`, rails.ts: locked on every lesson that never lists it, and until its step on the one that does) */
export const goalsOn = (S: SquadOrders): boolean => CMDRING.goals && CMDRING.on && S.sams && !locked(S.g.view, 'goal');
/** what the men who have nobody to give say */
export const GOAL_NONE = "nobody's free for that.";
/** WHO TAKES A GOAL ADDED ON `t` (CMDRING.goals has the rule), and how they come to: 'his' — a teammate under the cursor (where it still picks one,
 *  `wholeSquad` off): his alone · 'all' — on himself, or nothing going: everyone · 'same' — a door some of them are on already: those men ·
 *  'idle' — the men with nothing to do · 'split' — half of the biggest group on one thing, a whole pair where it has two (CMDRING.goalHalf: and half
 *  of a squad with nothing going, where it is more than one man). Null: every group is one
 *  man and nobody is free (the caller says so). Nothing is ordered and nothing counted here: the tap (planGoal) and the ring (orders.ts ringUp,
 *  CMDRING.goalRing) both ask it. */
export function goalMen(S: SquadOrders, t: OrderTarget): { men: Guard[]; how: 'his' | 'all' | 'same' | 'idle' | 'split' } | null { return crossToRust('goalMen', undefined, [S, t]) as any; }

/** Shift + the ring's key is the goal's own this frame (false: the key orders as it does with no Shift, orders.ts ringFree): only where the ring has no
 *  hold to wait out — with one the key is the ring's, Shift or no, and its TAP adds the goal (orders.ts ringInput), at a walk or a run */
export function goalShift(S: SquadOrders, input: Input): boolean {
  return (input.down('ShiftLeft') || input.down('ShiftRight')) && goalsOn(S) && !(CMDRING.holdSecs > 0);
}
