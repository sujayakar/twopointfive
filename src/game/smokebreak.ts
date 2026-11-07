// A SMOKE — a man a level's script has told to have one (director.ts OrderPatch.smoke → GuardScript.smoke; the first is the ledger's night man, who looks
// for a whistle, finds nothing and turns his back: levels/tut-quiet.script.ts). The maintainer, 10-22, of that man standing over the player's prints in the
// snow: 'maybe we should make him stop a bit further away and then turn around for a cigarette break', and 'cigarette break animation (with real smoke!)
// would be sick'.
//   WHO       a living man on his feet whose script says `smoke`, once he stands where it sent him. Not a man in an arm, not a man the script has let go
//             of (the order is the script's: a man handed back to the game has both hands for the game the frame he is).
//   THE HAND  his LEFT (the torch and the gun are the right's): up to his mouth to light it, then down at his side, and up again for a draw every `every`
//             seconds. The animator's own layer (character.ts smokeLayer, its numbers SMOKEARM) carries the arm; this file only says how far up it is asked.
//   THE LIGHT a lighter's flame at his mouth as he lights it, and the end going bright at every draw: the engine's own short lights (LightSet.flash, as a
//             muzzle's), so the picture and the light meter have the same thing. Small: a flame lights a face and a hand's reach of wall.
//   THE SMOKE real smoke: sources on the smoke solver's fine lattice, the one a gun's gas and the cold's breath live on. A thin thread off the end in his
//             hand all the while, and what he breathes out after each draw, off his mouth, thicker than a breath and slower to go. Both are LOOK sources
//             (smoke/types.ts `look`): the game's own smoke, the one sight lines are cut by, never hears of them. With no solver drawing (the harness, a
//             page with smoke off) what he breathes out is the particle system's soft puffs, as a breath in the cold is; the thread is then not shown.
//   NOBODY    reads any of it but the light meter: no eye, ear or foot. Its clock is the frames' dt, its dice are its own (never Math.random, never the
//             game's die).
// THE SWITCH: SMOKEBREAK.on — ON in the page (`?smokebreak=0` puts it off), OFF in the module and on the frozen stand (FROZEN_HOLDS; TPS_SMOKEBREAK=1
// raises it there). Off, a man told to smoke stands where he was sent with his hands as they were. tools/qa/smokebreak-check.ts (SMOKEBREAK_BREAK=1 fails it).
// The numbers are set from the solver's arithmetic and the breath's own; NEVER SEEN ON A GPU.
import { SideMap } from '../math/side';
import { Vec3 } from '../math/vec';
import type { Game, Guard } from './game';
import type { Character } from './character';
import './particleFx';
import { crossToRust, goneWithTick } from './crossing';

export const SMOKEBREAK = {
  on: false,
  /** the clock: s to light it (the hand is up from `lightUp[0]` to `lightUp[1]`, the flame burns `flame` s from `flameAt`), then a draw every `every`
   *  s, the hand up for `draw` s of it, and what he breathes out leaves `outAfter` s after the hand starts down, for `out` s */
  lightSecs: 3.2, lightUp: [0.5, 2.6] as [number, number], flameAt: 1.3, flame: 0.9,
  every: 7.5, draw: 2.1, outAfter: 0.7, out: 1.3,
  /** the lights (LightSet.flash: colour, intensity, range m; the end's is re-filed every `emberEvery` s so it never quite goes out) */
  flameColor: [1.0, 0.62, 0.24] as Vec3, flameK: 2.4, flameRange: 3.2,
  emberColor: [1.0, 0.36, 0.1] as Vec3, emberK: 0.1, emberDrawK: 0.7, emberRange: 1.3, emberEvery: 0.4,
  /** where the end is off the left hand's joint, m along the hand's own heading (worked out from the forearm) and up */
  tipOut: 0.11, tipUp: 0.02,
  /** THE THREAD off the end (the solver's units, as snowBreath.ts BREATHSTEAM: 6 /m of extinction a unit of density; +1.6 m/s² a unit of heat) */
  thread: { radius: 0.02, cells: 1.2, density: 5, temp: 5, speed: 0.25, push: 10, jitter: 0.35 },
  /** frames with a solver drawing and no word from it of his thread (it asks every source where it is, every step) before the thread is taken for gone and filed again */
  threadGone: 30,
  /** WHAT HE BREATHES OUT after a draw */
  breath: { radius: 0.04, cells: 1.5, density: 16, temp: 2.5, speed: 1.1, push: 12, expand: 2.2, attack: 0.1, jitter: 0.3, down: 0.1 },
  /** … as puffs, with no solver drawing: records across `out`, puffs a record, and the puff's own numbers (render/particles.ts; `steam` = its share of the
   *  dust puff's opacity and that it rises) */
  puffs: { records: 4, count: 6, speed: 0.8, spread: 0.26, life: 2.4, size: 0.1, opacity: 0.8 },
  /** what the smoke scatters: a blue grey (the solver's `tint`, the puff's colour) */
  color: [0.72, 0.76, 0.86] as Vec3,
  /** m: the mouth off the head joint, ahead and down (snowBreath.ts's own two numbers) */
  mouthAhead: 0.1, mouthDown: 0.075,
};

export interface SmokeBreakStats { men: number; lit: number; draws: number; flames: number; embers: number; breaths: number; frames: number }
interface Man { /** s since he was first asked */ t: number; flamed: boolean; /** draws begun, breaths let out */ draws: number; outs: number; emberT: number;
  /** this frame he smokes (the sources' tracks read it) */ live: boolean }
/** WHAT OF A SMOKE IS THE HOST'S AND NOT THE GAME'S: whether his smoke goes to the solver or out as puffs hangs on a solver drawing and on a particle pool,
 *  neither of which a recording or a snapshot carries. So the book of what was filed is kept on the HOST (`engine.hostBooks.smokeBreak`, host/api.ts HostBooks; `Game.engine` is
 *  in no snapshot), as an open fire's is (fire.ts FireShown): two games that differ only in their picture are the same game.
 *  AND THE BOOK IS NEVER TAKEN AT ITS WORD: the solver lets a source go without a word to whoever filed it (an encounter restarted, a file loaded, a save
 *  put back, a script's or the panel's clear — each empties it; its `ttl` runs out; a full list drops the first look-only one), and a book that still said
 *  'filed' was a man smoking with no smoke for the rest of his break. A solver asks every source it holds where it is, every step: so the thread's own
 *  track stamps the book (`beat`), and a thread nobody has asked after for `threadGone` frames with a solver drawing is gone and is filed again. Each filing
 *  has its own number (`gen`): a source from an older filing answers 'gone' the next time it is asked, so there is never more than one.
 *  A man by his character's id: `thread` = a thread was filed for him; `left`, `gap` = records of this breath still to file as puffs, and the clock between
 *  them. `frame` = this file's frames on this host; `threads`, `records` = how many this run. No rule reads any of it back. */
export interface SmokeShown { men: Map<number, { thread: boolean; gen: number; beat: number; left: number; gap: number }>; frame: number; threads: number; records: number }
/** the host's side of it, for the panel's line and the tools; undefined before the first frame a man smokes in, and behind a load */
export function smokeShown(g: Game): Readonly<SmokeShown> | undefined { return goneWithTick('smokeShown') as any; }
interface State { men: SideMap<Character, Man>; stats: SmokeBreakStats; seed: number }
const states = new SideMap<Game, State>('smokebreak.states');
const fresh = (): SmokeBreakStats => ({ men: 0, lit: 0, draws: 0, flames: 0, embers: 0, breaths: 0, frames: 0 });
/** the counts a probe or the panel reads (a game nobody has smoked in: a blank row, and none is made for it) */
export function smokeBreakStats(g: Game): SmokeBreakStats { return states.get(g)?.stats ?? fresh(); }

/** is this man smoking now: the switch, a script that says so, on his feet where it sent him, nobody's arm round him */
export function smoking(gd: Guard): boolean { return goneWithTick('smoking') as any; }

/** Once a frame, after the characters are posed (game.ts, beside the breath in the cold). */
export function updateSmokeBreak(g: Game, dt: number): void { return crossToRust('updateSmokeBreak', undefined, [g, dt]) as any; }
