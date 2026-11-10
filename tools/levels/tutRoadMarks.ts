// TUT-ROAD'S PLACES: where its fights' men come in, stand and shoot, which the generator lays the level round (tools/levels/tut-road.ts). The game's program of those fights holds the same numbers
// (rust/game_rules/src/missions/script_tut_road.rs).
import type { Vec3 } from '../../src/math/vec';

/** the truck's flank as the sentry sees it through the gateway: every line to it is clear of both jambs, and passes a man's width and more from where the
 *  player takes the wall beside the gateway */
export const TRUCK_FLANK: Vec3[] = [[5.6, 1.35, 13.05], [5.6, 0.85, 13.4], [5.6, 1.5, 13.75], [5.6, 1.1, 13.2]];
/** THE MAN IN THE HUT'S BACK ROOM: where he waits (a nook behind a stub of the hut's own wall: no round through the door, its leaf, the boards or the window
 *  reaches it, so a burst at the door from any stand leaves him for his own line) and where he steps to when the boards' line comes up (behind the boards
 *  from every stand in the doorway and out on the road, the boards and nothing else between) */
export const HUT_NOOK: Vec3 = [21.75, 0, 10.25], HUT_MARK: Vec3 = [20.7, 0, 11.45];
/** what the man behind the hut's plank wall shoots at, blind, through the boards: the head of the front room's far wall, the one the view sees the face of. Every
 *  line to it leaves the boards a man's height up and climbing, and passes two paces from the doorway the player stands in */
export const HUT_WALL: Vec3[] = [[18.3, 2.3, 9.83], [19.0, 2.2, 9.83], [19.6, 2.35, 9.83], [18.7, 2.4, 9.83]];
/** what the gun shoots at as he comes out of the hut: the hut's end wall and the head of its wind wall, on the gun's side of both. No line to any of them
 *  crosses floor he can reach from the side door without walking round the roadblock into the open */
export const HUT_END: Vec3[] = [[22.53, 1.5, 10.6], [22.53, 2.1, 11.6], [22.53, 1.2, 12.5], [22.53, 2.3, 14.3], [22.53, 1.8, 13.2], [22.53, 2.2, 15.0], [22.53, 1.4, 11.1], [22.53, 2.0, 14.7]];
/** along the sandbags, a point every half metre: a hand over their top, so over a crouched head in the lane behind them and through a standing chest. The
 *  gun walks its fire up and down this line (traversing fire: no man walks its length standing between two passes) */
export const BAGS_TOP: Vec3[] = [22.8, 23.3, 23.8, 24.3, 24.8, 25.3, 25.8, 26.3, 26.8, 27.2].map((x): Vec3 => [x, 1.42, 15.5]);
/** the roadblock's wall, the gun's face of it: where he is, once he is on it. Short of its bare end by a man's width (the end is his to lean round) */
export const WALL_FACE: Vec3[] = [[27.8, 1.3, 15.37], [28.5, 1.7, 15.37], [29.1, 1.2, 15.37], [28.1, 1.9, 15.37], [28.8, 1.5, 15.37]];
/** THE LAST THREE: where each waits outside the upper gate (the lane the road leaves the map by, out of every line through the gate), where each stands
 *  once he has come in by it (in the open, in the gate's lamp, far enough from the roadblock's wall to be seen whole from its end), and what each shoots
 *  at until a round finds one of them: the head of the roadblock's wall, over a man standing behind it */
export const LAST_OUT: Vec3[] = [[38.6, 0, 12.2], [38.6, 0, 16.4], [39.4, 0, 12.6]], LAST_MEN: Vec3[] = [[36.1, 0, 13.3], [35.4, 0, 15.3], [36.1, 0, 16.1]];
