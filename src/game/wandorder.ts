// A TEAMMATE LOOKS UNDER A SHUT DOOR WITH THE OPTICAL WAND — ON THE PLAYER'S ORDER, NEVER ON HIS OWN.
//
// The maintainer, 10-16, on whether a teammate may carry the wand and look first: "a, but yeah you have to order it". So this file is ONE ORDER and
// nothing else: no teammate ever decides to use a wand, no door tool has him look first, no drill asks for it. With `WAND.mate` (wand.ts) off, with
// nobody on the floor carrying one, or with no fog of war to look into, nothing here runs past its first line and the order is not on any menu.
//
// THE ORDER (id `wandLook`): the orders menu opened on a door (X, or the middle button) lists 'look under it · wand' on a key of its own, V
// (`WAND.orderKey`: every digit of a door's menu is taken and nothing is ever on a digit above 5; V is the whistle's key, eaten only while a menu that
// lists the row is up). It is ONE man's order (`only`): the nearest of the addressed men who carries a wand and can walk to the door's face they are on;
// everybody else is left exactly as he was. Greyed with the reason where it cannot be done: nobody addressed carries one, the door is steel (no gap),
// not shut, off its hinges, or nobody can get to it.
//
// THE MAN: walks to the middle of the leaf on the men's face (the player's own kneeling spot: Doors.workSpot), kneels with the lock work's pose, and
// `WAND.feedSecs` later (`WAND.quick`: as he gets down, the first time — wand.ts wandThrough) his lens is LIVE. He keeps it there until he is told something else, the leaf moves (he steps off beside the latch jamb, gun on
// the doorway), the fight takes him (the lens comes out; fed again from the start when he is back on his knee), or he is hit.
//
// THE LENS IS HIS EYE (fogofwar.ts asks `mateWandEye` in place of his own eye: no sixth eye): the player's lens by every number (wand.ts wandLens /
// wandFov / wandPanMax), swept from one end of its arc to the other and back over `WAND.sweepSecs` on the game's clock, since nobody aims it for him. So
// while he looks, what is round HIM is fog unless another eye has it — the same price the player pays. A hostile it shows keeps the wand's '(?)' mark.
// Under 'only my own eyes' it is a device feed like a sticky camera's and still lifts the fog.
//
// Nobody notices a lens; it files no sound; no dice; nothing reads the camera. No state in a checkpoint: an order is not saved (a checkpoint is refused
// with a squad on the floor), and after a load he simply is not looking.
import { viewOf } from './seats';
import { SideMap } from '../math/side';
import { Vec3 } from '../math/vec';
import type { Game, Guard } from './game';
import type { Door } from './doors';
import type { FogEye } from './fogofwar';
import type { Assignment } from './orders';
import './orders';
import { WAND, wandFits, wandLens, wandFov, wandPanMax } from './wand';
import './kit';

export const WAND_ORDER = 'wandLook';
/** looks ordered · lenses gone live · looks ended by the leaf moving */
export const WANDORDER_STATS = { ordered: 0, live: 0, moved: 0 };

interface Job {
  door: Door; side: 1 | -1;
  /** where he kneels, and the way he faces there */
  stand: Vec3; yaw: number;
  /** where he steps to if the leaf moves (beside the latch jamb, out of the opening), found while it was shut; null = no floor there */
  aside: { pos: Vec3; yaw: number } | null;
  /** the assignment the job was given under: another one on him = the job is over */
  A: Assignment;
  /** game time he got onto his knee at the leaf (−1: not there), when the lens went live, and the last tick that ran for him */
  t0: number; liveT: number; lastT: number;
  live: boolean;
  /** this file put the lock work's pose on him (and so takes it off) */
  knelt: boolean;
  /** he has been on his knee at this door and was taken off it (the fight, a flash): what follows is a feed — WAND.feedSecs — whatever WAND.quick says */
  again: boolean;
}
interface State { jobs: Map<Guard, Job> }
const states = new SideMap<Game, State>('wandorder.states');

const live = (g: Game) => WAND.on && WAND.mate && g.fogOfWar.active;







/** HIS LENS AS HIS EYE among the fog's (fogofwar.ts), or null while he is not looking: the player's lens by every number, swept over its arc on the
 *  game's clock */
export function mateWandEye(g: Game, gd: Guard): FogEye | null {
  g = viewOf(g, gd);   // (his seat's jobs: seats.ts)
  const st = states.get(g); if (!st || !st.jobs.size) return null;
  const J = st.jobs.get(gd); if (!J || !J.live || !live(g)) return null;
  const now = g.time; if (gd.state === 'dead' || now - J.lastT > 0.25 || now < gd.dazzledUntil || gd.script?.over || !wandFits(J.door)) return null;
  const L = wandLens(J.door, J.side), ph = ((now - J.liveT) / Math.max(0.5, WAND.sweepSecs) + 0.25) % 1, a = L.yaw + wandPanMax() * (1 - 4 * Math.abs(ph - 0.5));
  return { id: gd.char.id, pos: L.pos, look: [Math.sin(a), 0, Math.cos(a)], at: L.pos, fov: wandFov(), near: 0, wand: true };
}
/** every teammate's live lens (fogofwar.ts under 'only my own eyes': a lens is a device feed there, like a sticky camera's) */
export function mateWandEyes(g: Game): FogEye[] {
  const st = states.get(g); if (!st || !st.jobs.size) return [];
  const out: FogEye[] = []; for (const gd of st.jobs.keys()) { const e = mateWandEye(g, gd); if (e) out.push(e); }
  return out;
}
/** who is looking under what, for the checks and the debugger */
export function wandOrderJobs(g: Game): { gd: Guard; door: Door; side: 1 | -1; live: boolean; knelt: boolean }[] {
  const st = states.get(g); return st ? [...st.jobs].map(([gd, J]) => ({ gd, door: J.door, side: J.side, live: J.live, knelt: J.knelt })) : [];
}
