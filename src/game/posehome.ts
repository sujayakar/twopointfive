// THE POSES OF A GAME, AT HOME IN THE RUST MODULE (rust/sim/src/poses.rs; anim/skeleton.ts Pose, PoseSet) — every pose a character's animator keeps
// from one tick to the next lives in the game's block of the module's memory (sim/state.ts), whatever row or maths stand: the animator's layers are
// written once, in Rust, and pose a man where his poses stand. The posing functions that are in Rust (the blend, the run forward, a node turned, the
// two-bone solve) are asked of a pose THERE, with no copy across, by whoever the row RUST.poses says answers them. The host reads and writes
// `pose.cols`, the same floats, and it is the same game to the bit.
//
// Posing is gameplay, whole (docs/agents/rust.md).
import { simTenant, type SimState } from '../sim/state';
import { sim } from '../sim/sim';
import { poseSetIn, poseSetOut, poseSetMost, type PoseSet } from '../anim/skeleton';
import { men } from './seats';
import type { Game } from './game';

/** THE POOL HOLDS THE POSES OF THE CHARACTERS WHO ARE IN THE GAME, and no others. As a tick opens under the row: is every character of the game's lists at
 *  home, and is every set that is at home one of theirs? Then nothing is done. A set at home that is no longer on a list (a restart made the guards anew;
 *  a man was taken off the floor) means everybody comes out, the pool is emptied, and those who are there go back in: one rule for whoever left, however he
 *  left, and one who comes back goes in again like one who arrived. THE POOL HOLDS EVERY MAN A GAME CAN HOLD, each with every pose his animator can
 *  make: the game's one count says so as the game is made (game/levelfit.ts fitRows, the row `poses`; `manSingles` below is its term). */
function moveIn(g: Game, st: SimState): void {
  const held = g.poseSets; let there = 0, late = false;
  const see = (s: PoseSet): void => { if (s.home !== st) late = true; else { there++; if (s.stray !== 0) late = true; } };
  eachPosed(g, see);
  if (there !== held.length) { moveOut(g, st); late = true; }
  if (late) eachPosed(g, s => {
    if (s.home !== st) { if (s.home !== null) poseSetOut(s); held.push(s); }   // (at home in ANOTHER game's block: his floats come out of there first)
    if (s.home !== st || s.stray !== 0) poseSetIn(s, st); });
}
/** what of a game says who is posed in it */
export type Cast = Pick<Game, 'seats' | 'player' | 'guards' | 'civilians'>;
/** every character the game poses: the players' men, the shift, the civilians */
export function eachPosed(g: Cast, f: (s: PoseSet) => void): void {
  for (const pl of men(g)) f(pl.char.anim.poses);
  for (const gd of g.guards) f(gd.char.anim.poses);
  for (const c of g.civilians) f(c.char.anim.poses);
}
/** SINGLES OF THE POOL ONE MAN FILLS AT MOST: every pose his animator can make (PoseSet.most), by the biggest rig among the men who stand (they are
 *  one rig today). 0 for a thing with no cast: a check's bare stand. The count that multiplies it by the men a game can hold is game/levelfit.ts fitRows. */
export function manSingles(g: Partial<Cast>): number {
  let worst = 0; if (g.seats && g.player && g.guards && g.civilians) eachPosed(g as Cast, s => { const m = poseSetMost(s); if (m > worst) worst = m; });
  return worst;
}
/** every set that went in comes out, and the pool is empty again */
function moveOut(g: Game, st: SimState): void { for (const s of g.poseSets) poseSetOut(s); g.poseSets.length = 0; sim().pose_clear(st.bind()); }
// (a top-level statement that uses src/sim's export alone: no part of src/game's import cycle is touched as this module loads)
simTenant({ name: 'poses', bit: 2,
  wants(g: Game) { return !!g.poseSets; },   // (a game's poses, always, whichever row or maths stand: the animator's layers are written once, in Rust, and pose a man where his poses stand in the module — anim/skeleton.ts layerSlot. The row `poses` holds who answers a pose's QUESTIONS, benchFor, not where the poses live. A thing with no cast, a check's bare die, has nothing to move)
  into: moveIn,
  outOf: moveOut,
  tend: moveIn });
