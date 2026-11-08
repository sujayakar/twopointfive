// THE BOX WORLD, ASKED: is a line stopped, what does a ray meet, how does a sound reach. Whoever steps the game answers (src/host/rustgame.ts in a page): the Rust game
// is asked of the boxes as they stand, shared, and nothing is proven, kept or changed by a question (rust/game_rules/src/ask WorldAsk). The camera, the sound
// and the cursor ask here, and never the page's own copy of the level.
//
// Nothing under src/view imports the game (tools/qa/view-check.ts).
import type { Vec3 } from '../math/vec';

/** how a sound reaches: how far it comes, through where, from where it seems to come, how muffled it is (0 to 1) and how much of it carries */
export interface Heard { direct: boolean; routed: boolean; pathLen: number; via: Vec3; apparent: Vec3; muffle: number; carry: number }
export interface WorldAsk {
  /** is a line of sight from a to b stopped (glass is passed) */
  blocked(a: Vec3, b: Vec3): boolean;
  /** …by anything solid, whole glass with it */
  solid(a: Vec3, b: Vec3): boolean;
  /** the nearest thing along a ray (d a unit vector) within `max`: how far, and the face it is entered by */
  ray(o: Vec3, d: Vec3, max: number): { t: number; n: Vec3 } | null;
  /** how a sound at `from` reaches a listener at `to` */
  hear(from: Vec3, to: Vec3): Heard;
  /** several questions at once (a page whose own copy answers proves it once for them all) */
  together<T>(f: () => T): T;
}
/** a sound's answer as the module writes it: ten doubles */
export function heardOf(r: ArrayLike<number>): Heard {
  const how = r[9]; return { direct: how >= 2, routed: how % 2 === 1, pathLen: r[0], via: [r[1], r[2], r[3]], apparent: [r[4], r[5], r[6]], muffle: r[7], carry: r[8] };
}
