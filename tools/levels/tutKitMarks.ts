// TUT-KIT'S PLACES: where its fights' men come in, stand and shoot, which the generator lays the level round (tools/levels/tut-kit.ts). The game's program of those fights holds the same numbers
// (rust/game_rules/src/script_tut_kit).
import type { Vec3 } from '../../src/math/vec';

/** THE TWO FOR THE LAUNCHER come through the building behind him once he is back at the arch: where they come onto the floor (the lane behind the store,
 *  outside its back door, which is thrown open for them), where they stand (the shop's doorway, one a step behind the other) and what each shoots at until
 *  the grenade or a round finds one of them: the top of the arch's wall either side of its mouth, a man's height and more over anyone standing in it */
export const LAST_IN: Vec3[] = [[37.4, 0, 14.0], [38.4, 0, 14.5]], LAST_MEN: Vec3[] = [[25.7, 0, 13.98], [26.5, 0, 14.5]];
/** THE TWO OF THE WASH YARD come in by its back door from the plain behind the halt while the squad is still inside the building (the cue of the line
 *  that sends him back out): where they come onto the floor, and where they get down, behind the wall of sandbags across the yard. They are down
 *  before anyone in the court can look: a man on his feet at that door shows over the bags */
export const YARD_IN: Vec3[] = [[37.6, 0, 19.5], [38.6, 0, 20.1]], YARD_MEN: Vec3[] = [[29.5, 0, 19.2], [29.5, 0, 20.4]];
