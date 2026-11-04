// DEPOT IN THE DEFAULT LOOK (src/scene/look.ts): tools/levels/depot.ts's level, box for box, in other clothes — tools/levels/gwotSkin.ts says what may change
// (paint and finish, decals, the paint of a prop) and checks that nothing a round, a charge or a blast reads did. `bun run tools/levels/export.ts depot-gwot`.
// ITS OWN PLACE: an old army motor pool and its signals hut — cream over oxide red, oil black. What it alone has: VEHICLES AND LETTERED DOORS. Each held
// room's letter is painted on the ground at its doors (A the crew room, B the signals hut — at its one door, and again on the yard under the window that
// has no door beside it — C the store), the yard has its parking bays, the stop line at the gate and a keep-clear box at the store's hatch, the bay its
// inspection pit. No green, no mud.
import { build as buildPlain, NOTES as PLAIN_NOTES, DEPOT_ROOMS, DEPOT_PAINT as OWN } from './depot';
import { GWOT_MATERIALS } from '../../src/scene/gwotKit';
import { GWOT_PALETTE as P } from '../../src/scene/gwotPalette';
import { skinLevel, LOOK_NOTE, type Skin } from './gwotSkin';
import { lookMarks } from './lookMarks';
import type { Level } from '../../src/scene/level';

export const NAME = 'depot-gwot';
export const DRESS_AS = 'depot';
export const MATERIALS = GWOT_MATERIALS;
export const NOTES = `${PLAIN_NOTES ?? ''}${LOOK_NOTE}`;
export const build = buildPlain;
const R = DEPOT_ROOMS;
const SKIN: Skin = {
  // WHAT EACH THING IS MADE OF, where the shared rules had no word for it or a wrong one (the look draws a paint as a material by its palette row, and a paint
  // with no material is drawn flat). THE YARD IS POURED: the hardstanding wore `block`, which the look lays in courses like a wall on its side, over a
  // quarter of the picture; a motor pool's yard is slab, in bays with joints, and the signals hut's raised floor is the same pour. The rooms that had lino
  // are boarded (`floor`). `own`: the depot's own colours, by the row each wears — army paint on the trucks, the shelter and the generating set, red oxide
  // on a drum and on the main gate's leaves, bare iron for the stove, a hub, a picket. (The shelter's chassis is a hair too dark for any row to stand for it.)
  own: [[OWN.hard, 'slab'], [OWN.raised, 'slab'], [OWN.drab, 'olive'], [OWN.shelter, 'olive'], [OWN.set, 'olive'], [OWN.oxide, 'drumRed'], [OWN.gate, 'paintRust'], [OWN.iron, 'iron']],
  pick: c => c.name === 'wallDark' ? 'woodGrey' : undefined,                           // the wash room's stall partitions: boards
  wall: 'tWall', house: 'tWall', inner: 'paintCream', frameOut: 'paintRust', frameIn: 'woodGrey', dado: 'paintRust',
  floor: { carpet: 'slab', stone: 'slab', lino: 'woodGrey' }, noRug: /./, rugs: { crew: 'mat' },
  metals: ['olive', 'rust', 'drumRed', 'iron'],
  rooms: [{ id: 'duty', bounds: R.duty }],
  dress: (G, _level, T) => {
    const M = lookMarks(T), paint = T.tone(P.stencil, 0.92), Y = P.genYellow, K = P.iron;
    // THE YARD'S WEAR, LAID FIRST so that every painted line and letter below lies over it (the pour is one paint from wall to wall, and from above it is a quarter of the picture): the ways feet take between the doors,
    // trodden darker — the crew room to the bay's side door past the truck's tail, the battery room to the store, the gatehouse to the gate, the back
    // gate to the mouth, and the back way itself from the gap to the generator room's door. The slab's own paint darker, never a colour laid on it.
    { const fa = T.floorAt(14, 13); if (fa) { const w = T.tone(fa, 0.84), trod = (pts: [number, number][], wd = 0.8) => M.path(pts, wd, w);
      trod([[12.4, 10.6], [15.0, 14.0], [15.9, 15.3], [16.5, 17.4]]); trod([[21.4, 10.6], [22.3, 14.4], [22.9, 17.4]]); trod([[6.4, 10.6], [5.3, 12.6]], 0.7);
      trod([[34.8, 17.4], [30.0, 17.1], [24.8, 17.3]]); trod([[35.0, 14.4], [35.0, 10.6], [31.4, 8.0], [30.4, 6.9]], 0.7); } }
    // OUTSIDE THE WALL: the track the patrol drives round it, and the road past the gate
    G.ruts(1.9, 0.6, 1.9, 27.4, 'dirt'); G.ruts(1.0, 26.0, 39.1, 26.0, 'dirt'); G.ruts(38.2, 27.0, 38.2, 1.0, 'dirt'); G.ruts(39.1, 1.9, 1.0, 1.9, 'dirt');
    // THE LETTERS: A the crew room (its yard door, its wash-room door), B the signals hut (its one door, from the generator room — and on the yard under its
    // west window, where a door would be), C the store (its yard door, its bay door)
    M.glyph('A', 12.25, 10.7, 0.85, paint); M.glyph('A', 16.5, 6.75, 0.85, paint);
    M.glyph('B', 25.75, 8.3, 0.85, paint); M.glyph('B', 22.9, 11.9, 0.85, paint);
    M.glyph('C', 22.75, 17.4, 0.85, paint); M.glyph('C', 16.9, 20.75, 0.85, paint);
    // THE YARD: the stop line inside the gate, two parking bays on the south side, a keep-clear box in front of the store's hatch, the ruts in from the road
    M.seg(4.7, 12.6, 4.7, 15.4, 0.14, paint);
    M.outline(9.6, 15.5, 15.6, 18.3, 0.12, paint, 'n'); M.outline(15.6, 15.5, 19.0, 18.3, 0.12, paint, 'n');
    M.outline(19.3, 17.3, 21.5, 18.3, 0.07, paint); for (let x = 19.5; x < 21.3; x += 0.6) M.seg(x, 18.25, x + 0.35, 17.35, 0.06, paint);
    G.ruts(0.6, 14.0, 9.0, 14.0, 'oil'); G.patch(12.4, 16.9, 1.5, 1.0, 'oil', 0.1); G.patch(17.2, 16.8, 1.1, 0.8, 'oil', 1.3); G.patch(15.0, 12.6, 0.9, 0.7, 'oil', 0.2);
    // THE BAY: the inspection pit's hazard edge, drawn round the truck that stands over it
    M.hazard(10.9, 21.55, 15.9, 21.7, Y, K); M.hazard(10.9, 21.7, 11.1, 23.7, Y, K, 0.25); M.hazard(15.7, 21.7, 15.9, 23.7, Y, K, 0.25);
    // THE EAST YARD: the fuel point's bund
    M.outline(26.4, 22.2, 28.8, 23.86, 0.1, Y);
    // THE SIGNALS HUT: the darker anti-static floor round the desk
    { const fa = T.floorAt(27, 12.6); if (fa) M.fill(25.7, 11.7, 28.4, 13.7, T.tone(fa, 0.78)); }
  },
};
export const finish = (level: Level, groups: string[]) => skinLevel(level, groups, SKIN);
