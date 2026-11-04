// GOALS — Shift + the squad's key gives what the cursor is on to a PART of the squad, and the rest go on as they were (cmdring.ts `goals`, `goalRing`).
// This file was plan mode's (the maintainer, 10-26: "let's remove plan mode"): the mode, its chains, its picks, its view and its picture are gone, and
// what is left is the rule for who takes a goal and the key that asks for one. The file keeps its name for the modules that import it.
//
//   · A TAP of Shift + the key on a thing: `planGoal` — the men `goalMen` names take the key's own order there (a bare spot: they cover it).
//   · KEPT DOWN, it is the order ring for those men alone (orders.ts ringUp / goalPart, `Menu.part`).
//   · Every order is given through the order book (`quickFor`, `coverFor`): a refusal and its reason are a man's own. Nothing here moves a man.
//
// STATE: none. PLAN_STATS is a tally for the checks.
import type { Guard } from './game';
import type { Input } from './input';
import type { FollowCamera } from './camera';
import type { View } from '../host/hud';
import type { OrderTarget, SquadOrders } from './orders';
import { CMDRING } from './cmdring';

export const PLAN_STATS = { goals: 0, split: 0, rings: 0, none: 0 };

/** SHIFT + THE KEY ADDS A GOAL (CMDRING.goals): on Sam's own squad — and on a tutorial mission only from the step that teaches it (the frame's action
 *  `goal`, rails.ts: locked on every lesson that never lists it, and until its step on the one that does) */
export const goalsOn = (S: SquadOrders): boolean => CMDRING.goals && CMDRING.on && S.sams && !S.g.rails?.locks('goal');
/** what the men who have nobody to give say */
export const GOAL_NONE = "nobody's free for that.";
/** the squad as the goals it is on now: the men with nothing of their own to do (no order, or in the file behind him), and the rest by what they were
 *  sent to — a door's men together, whoever sent them; any other order by its own giving */
function goalParts(S: SquadOrders): { idle: Guard[]; groups: Guard[][] } {
  const idle: Guard[] = [], by = new Map<string, Guard[]>(), doors = S.g.doors.list;
  for (const m of S.men('all')) {
    const A = S.of(m); if (!A || A.post === null) { idle.push(m); continue; }
    const k = A.door ? `d${doors.indexOf(A.door)}` : `g${A.group}`, l = by.get(k); if (l) l.push(m); else by.set(k, [m]);
  }
  return { idle, groups: [...by.values()] };
}
/** WHO TAKES A GOAL ADDED ON `t` (CMDRING.goals has the rule), and how they come to: 'his' — a teammate under the cursor (where it still picks one,
 *  `wholeSquad` off): his alone · 'all' — on himself, or nothing going: everyone · 'same' — a door some of them are on already: those men ·
 *  'idle' — the men with nothing to do · 'split' — half of the biggest group on one thing, a whole pair where it has two. Null: every group is one
 *  man and nobody is free (the caller says so). Nothing is ordered and nothing counted here: the tap (planGoal) and the ring (orders.ts ringUp,
 *  CMDRING.goalRing) both ask it. */
export function goalMen(S: SquadOrders, t: OrderTarget): { men: Guard[]; how: 'his' | 'all' | 'same' | 'idle' | 'split' } | null {
  const all = S.men('all'), { idle, groups } = goalParts(S), spot = t.kind === 'floor' && !t.thing;   // (a bare spot, by a wall or not: a place to watch)
  if (t.kind === 'none' && t.man) return { men: S.recipients(t), how: 'his' };
  if (t.kind === 'none' || !groups.length) return { men: all, how: 'all' };
  const on = t.kind === 'door' ? groups.find(G => G.every(m => S.of(m)?.door === t.door)) : null;
  if (on) return { men: on, how: 'same' };
  if (idle.length) return { men: idle, how: 'idle' };
  const far = (men: Guard[]) => men.reduce((s, m) => s + Math.hypot(m.char.pos[0] - t.at[0], m.char.pos[2] - t.at[2]), 0) / men.length;
  const line = (men: Guard[]) => spot ? men.filter(m => S.hasLine(m, t)).length / men.length : 0;   // (a spot to watch: the men who can see it)
  const best = (ls: Guard[][]) => ls.reduce((a, b) => line(b) > line(a) || (line(b) === line(a) && far(b) < far(a) - 1e-6) ? b : a);
  const size = Math.max(...groups.map(G => G.length)); if (size < 2) return null;
  const G = best(groups.filter(x => x.length === size));
  const parts = S.pairs().map(id => G.filter(m => S.pairOf(m) === id)).filter(l => l.length * 2 <= G.length && l.length > 0);
  return { men: parts.length ? best(parts) : [...G].sort((a, b) => line([b]) - line([a]) || far([a]) - far([b])).slice(0, G.length >> 1), how: 'split' };
}
/** … and Shift + the key kept down is the ring for those men (CMDRING.goalRing: orders.ts ringUp) */
export const goalRingOn = (S: SquadOrders): boolean => CMDRING.goalRing && goalsOn(S);
/** Shift + the key on `t`: a part of the squad takes it and the rest go on as they were (CMDRING.goals has the rule). Every order is given through
 *  the order book (quickFor, coverFor): a refusal and its reason are a man's own, and nothing here moves anybody. */
export function planGoal(S: SquadOrders, t: OrderTarget): void {
  const g = S.g; if (!S.men('all').length) return;
  PLAN_STATS.goals++;
  const T = goalMen(S, t), spot = t.kind === 'floor' && !t.thing; if (!T) { g.msg(GOAL_NONE); return; }
  // ('his': the key's own order, his alone · 'same': the key's order there, for the men on that door)
  if (T.how === 'his' || T.how === 'same') { S.quickFor(T.men, t); return; }
  if (T.how === 'split') PLAN_STATS.split++;
  if (spot) S.coverFor(T.men, t.at); else S.quickFor(T.men, t);
}

/** Shift + the ring's key is the goal's own this frame (false: the key orders as it does with no Shift, orders.ts ringFree): only where the ring has no
 *  hold to wait out — with one the key is the ring's, Shift or no, and its TAP adds the goal (orders.ts ringInput), at a walk or a run */
export function goalShift(S: SquadOrders, input: Input): boolean {
  return (input.down('ShiftLeft') || input.down('ShiftRight')) && goalsOn(S) && !(CMDRING.holdSecs > 0);
}
/** orders.ts input, once a tick with a squad on the floor and Sam up — after the who key, ahead of the ring and the menu: where the ring has no hold to
 *  wait out, the pair adds the goal here, as the key goes down */
export function goalInput(S: SquadOrders, input: Input, cam: FollowCamera, canvas: View): void {
  if (goalShift(S, input) && !S.cmdRing && S.backFree() && input.pressed.delete(CMDRING.key)) planGoal(S, S.targetUnder(input, cam, canvas));
}
