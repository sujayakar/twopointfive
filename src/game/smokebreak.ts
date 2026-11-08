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
//             page with smoke off) neither is shown.
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
  /** WHAT HE BREATHES OUT after a draw */
  breath: { radius: 0.04, cells: 1.5, density: 16, temp: 2.5, speed: 1.1, push: 12, expand: 2.2, attack: 0.1, jitter: 0.3, down: 0.1 },
  /** what the smoke scatters: a blue grey (the solver's `tint`, the puff's colour) */
  color: [0.72, 0.76, 0.86] as Vec3,
  /** m: the mouth off the head joint, ahead and down (snowBreath.ts's own two numbers) */
  mouthAhead: 0.1, mouthDown: 0.075,
};

export interface SmokeBreakStats { men: number; lit: number; draws: number; flames: number; embers: number; breaths: number; frames: number }
interface Man { /** s since he was first asked */ t: number; flamed: boolean; /** draws begun, breaths let out */ draws: number; outs: number; emberT: number;
  /** this frame he smokes (the sources' tracks read it) */ live: boolean }
interface State { men: SideMap<Character, Man>; stats: SmokeBreakStats; seed: number }
const states = new SideMap<Game, State>('smokebreak.states');
const fresh = (): SmokeBreakStats => ({ men: 0, lit: 0, draws: 0, flames: 0, embers: 0, breaths: 0, frames: 0 });
/** the counts a probe or the panel reads (a game nobody has smoked in: a blank row, and none is made for it) */
export function smokeBreakStats(g: Game): SmokeBreakStats { return states.get(g)?.stats ?? fresh(); }

/** is this man smoking now: the switch, a script that says so, on his feet where it sent him, nobody's arm round him */
export function smoking(gd: Guard): boolean { return goneWithTick('smoking') as any; }

/** Once a frame, after the characters are posed (game.ts, beside the breath in the cold). */
export function updateSmokeBreak(g: Game, dt: number): void { return crossToRust('updateSmokeBreak', undefined, [g, dt]) as any; }
