// THE BOX WORLD, ASKED: is a line stopped, what does a ray meet, how does a sound reach. Whoever steps the game answers (src/host/drawgame.ts in a page): the Rust game
// is asked of the boxes as they stand, shared, and nothing is kept or changed by a question (rust/game_face/src/ask.rs). The camera, the sound
// and the cursor ask here, and never the page's own copy of the level.
//
// Nothing under src/view imports the game (tools/qa/view-check.ts).
import type { Vec3 } from '../math/vec';
import type { Met, SoundPath } from './read';

export interface WorldAsk {
  /** is a line of sight from a to b stopped (glass is passed) */
  blocked(a: Vec3, b: Vec3): boolean;
  /** …by anything solid, whole glass with it */
  solid(a: Vec3, b: Vec3): boolean;
  /** the nearest thing along a ray (d a unit vector) within `max`: how far, and the face it is entered by */
  ray(o: Vec3, d: Vec3, max: number): Met | null;
  /** how a sound at `from` reaches a listener at `to` */
  hear(from: Vec3, to: Vec3): SoundPath;
  /** the way on the floor from a to b, its corners (null: the floor has none); a search, so whoever asks keeps the answer by its own clock */
  way(a: Vec3, b: Vec3, maxIter: number): Vec3[] | null;
  /** the light that stands on a point, leaving out the carried lamps of `owners`; `steady`: what the place lights, with nothing that passes; `leave`: lamps of the level's own left
   *  out, by what feeds them (rust/game_rules/src/air/fixtures.rs ROW_BIT) */
  light(p: Vec3, owners: readonly number[], steady: boolean, leave: number): number;
}
