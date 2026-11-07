// TAP TO STEP (TAPSTEP) — a prototype in the sneak lab: crouched, the move keys stop being a walk and become steps. The maintainer, of the ideas for the creep: "i love
// the tap to step idea! curious how we can make this feel awesome. and also generalize between creeping on a guy to grab him vs slithering through guards". One verb
// for both: there is no mode for a man's back, the same step just knows where it lands.
//
//   A STEP       a press lifts the foot, the key kept down reaches further (to `reachM`), the key let go plants it as soon as he can stop softly (a tap is `tapM`). A step is a
//                smooth curve from rest to rest, the way a hand reaches, over `secs` (less for a shorter one): no corner in his speed at either end, so the foot comes down
//                slowly. The maintainer of the first cut, a trapezoid: "the rhythm of them is nice but it looks like it'd be loud. the legs move in abrupt movements". Between
//                steps he stands dead still in the stride he stopped in: a step takes the legs on to the next plant from wherever they are (character.ts tapU).
//   THE RHYTHM   after a plant his weight settles for `settle` s. The key kept down waits that out and steps again, so a held key is always the safe pace. A press made
//                while he is still settling steps at once and that step lands as a scuff, louder the earlier it was. A press inside `beat` s of his settling is on
//                the beat: a tap then is a `beatM` step, so a player who has the rhythm goes further for a tap and still stops where he means to.
//   A MAN'S STEP a plant inside `maskSecs` of the footfall of a man within `maskM` of Sam is under it: `mask` of its level.
//   HIS BACK     a press toward the back of a man he could stalk (stalk.ts stalkCould), within `landM` of the spot `STALK.keyM` behind him and `landCos` of its line, is a step
//                that bends onto that spot, so the last step lands in reach and the grab's own key is what is left. It is settled at the press and the keys can still take it
//                back: turned away from the spot it is a plain step again. At the spot already, a key is a plain step (the first cut took any key there for a step of no
//                length: "weird snapping to the guard, stuck on them afterwards"). Behind a man who is walking, `follow` of his speed goes into the step.
// What it does not touch: what the men see of a crouched Sam (a still man is a still man), Shift in a crouch (the quick crouch walk, for when the steps are too slow), the
// held click, the wall hold, a body in tow.
// A prototype: up in the sneak lab and with ?tapstep=1, down in the plain page and on the frozen stand, so no FROZEN_HOLDS row. Off = the build as it was.
import { SideMap } from '../math/side';
import { Vec3 } from '../math/vec';
import type { Game, Guard } from './game';
import './stalk';
import './kitload';
import { crossToRust } from './crossing';

export const TAPSTEP = {
  on: false,
  /** m: a tap's step · a tap's on the beat · the most a held key reaches */
  tapM: 0.28, beatM: 0.45, reachM: 0.65,
  /** s: how long a step of `reachM` takes, rest to rest (a shorter one less, by the root of its length) · how much of that a rushed step loses at the worst */
  secs: 0.75, rushed: 0.35,
  /** s: his weight coming down after a plant · how long after that a press is still on the beat */
  settle: 0.08, beat: 0.16,
  /** a plant's footfall: on time (the crouch walk's is 0.125) · at the very start of the settling, less the later the press */
  quiet: 0.08, scuff: 0.4,
  /** under a man's footfall: how long after it · how near Sam the man · the share of its level left */
  maskSecs: 0.16, maskM: 7, mask: 0.3,
  /** his back: how far off the spot a step takes it · cos of how far off the step's line it may be · the share of a walking man's speed a step behind him takes on */
  landM: 0.95, landCos: 0.8, follow: 0.6,
  /** 1/s: how quickly a step in the air comes round to the keys, or onto his back · how quickly the legs follow the ground a step has covered */
  turn: 10, legs: 14,
};
export const TAPSTEP_STATS = { steps: 0, scuffs: 0, beats: 0, masked: 0, lands: 0, last: '' };

interface St { /** 0 at rest · 1 in the air · 2 settling */ st: 0 | 1 | 2; /** s: settling left · at rest, how long he has been */ t: number; /** the frame this last ran on */ at: number; keys: boolean;
  dir: [number, number]; v: number; a: number; /** m/s: this frame's ground */ go: number; /** s: left of this step · since it began · at the most */ left: number; age: number; secs: number; done: number; len: number; least: number; /** how early the press that began this step was, 0‥1 */ rush: number; u: number; n: number; way: 1 | -1; stuck: number; prev: Vec3 | null; asked: number;
  man: Guard | null; maskT: number; /** the key is up and `len` is where this step lands */ set: boolean }
const STATE = new SideMap<Game, St>('tapstep.STATE');

/** a man's footfall (guards.ts): a plant just after it is under it */
export function tapHears(g: Game, pos: ArrayLike<number>): void { return crossToRust('tapHears', undefined, [g, pos]) as any; }
export function tapReset(g: Game) { STATE.delete(g); }

