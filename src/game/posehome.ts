// THE POSES OF A GAME, AT HOME IN THE RUST MODULE (rust/sim/src/poses.rs; anim/skeleton.ts Pose, PoseSet) — while the row RUST.poses is on, every pose a
// character's animator keeps from one tick to the next lives in the game's block of the module's memory (sim/state.ts), and the posing functions that are
// in Rust (the blend, the run forward, a node turned, the two-bone solve) are asked of it THERE, with no copy across. Off, a pose is an array of its own, as
// it always was. Either way the layers of the animator read and write `pose.data`, and it is the same game to the bit.
//
// This moves the DATA and no layer: a layer written in Rust then finds the poses where it runs. Posing is gameplay, whole (docs/agents/rust.md).
import { simTenant, type SimState } from '../sim/state';
import { sim, RUST } from '../sim/sim';
import { poseSetIn, poseSetOut, type PoseSet } from '../anim/skeleton';
import { men } from './seats';
import type { Game } from './game';

/** THE POOL HOLDS THE POSES OF THE CHARACTERS WHO ARE IN THE GAME, and no others. As a tick opens under the row: is every character of the game's lists at
 *  home, and is every set that is at home one of theirs? Then nothing is done. A set at home that is no longer on a list (a restart made the guards anew;
 *  a man was taken off the floor) means everybody comes out, the pool is emptied, and those who are there go back in: one rule for whoever left, however he
 *  left, and one who comes back goes in again like one who arrived. A pose the pool has no room for stays an array of its own, which is the same game, and
 *  is tried again each tick. */
function moveIn(g: Game, st: SimState): void {
  const held = g.poseSets; let there = 0, late = false;
  const see = (s: PoseSet): void => { if (s.home !== st) late = true; else { there++; if (s.stray !== 0) late = true; } };
  each(g, see);
  if (there !== held.length) { moveOut(g, st); late = true; }
  if (late) each(g, s => {
    if (s.home !== st) { if (s.home !== null) poseSetOut(s); held.push(s); }   // (at home in ANOTHER game's block: his floats come out of there first)
    if (s.home !== st || s.stray !== 0) poseSetIn(s, st); });
}
/** every character the game poses: the players' men, the shift, the civilians */
function each(g: Game, f: (s: PoseSet) => void): void {
  for (const pl of men(g)) f(pl.char.anim.poses);
  for (const gd of g.guards) f(gd.char.anim.poses);
  for (const c of g.civilians) f(c.char.anim.poses);
}
/** every set that went in comes out, and the pool is empty again */
function moveOut(g: Game, st: SimState): void { for (const s of g.poseSets) poseSetOut(s); g.poseSets.length = 0; sim().pose_clear(st.bind()); }
// (a top-level statement that uses src/sim's export alone: no part of src/game's import cycle is touched as this module loads)
simTenant({ name: 'poses', bit: 2,
  wants(_g: Game) { return RUST.poses; },   // (whichever maths stand: the QUESTIONS need the game's own, anim/skeleton.ts benchFor, but where the poses live does not)
  into: moveIn,
  outOf: moveOut,
  tend: moveIn });
