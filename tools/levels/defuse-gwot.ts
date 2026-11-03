// THE TWO CHARGES IN THE DEFAULT LOOK (src/scene/look.ts): tools/levels/defuse.ts's level, box for box, in other clothes — tools/levels/gwotSkin.ts says what may
// change (paint and finish, decals, the paint of a prop) and checks that nothing a round, a charge or a blast reads did. `bun run tools/levels/export.ts defuse`.
// ITS OWN PLACE: a bomb maker's house — mud outside, whitewash in, bare slab in the two workshops. What it alone has: FIRING WIRE. A twin lead in copper runs
// along the floor from each bench, out through its wing's two doors, to one battery plate in the middle of the hall (the level's idea — one clock, two charges,
// a wing apart — read off the ground: two runs parting from one point), with a painted 1 and 2 at the workshop doors, inside and at the back doors (the charges'
// names: game/defuse.ts says 'charge 1' / 'charge 2'). No accent paint on a wall.
import { build as buildPlain, NOTES as PLAIN_NOTES, DEFUSE_ROOMS } from './defuse';
import { GWOT_MATERIALS } from '../../src/scene/gwotKit';
import { GWOT_PALETTE as P } from '../../src/scene/gwotPalette';
import { skinLevel, LOOK_NOTE, type Skin } from './gwotSkin';
import { lookMarks } from './lookMarks';
import type { Level } from '../../src/scene/level';
import type { Vec3 } from '../../src/math/vec';

export const NAME = 'defuse-gwot';
export const DRESS_AS = 'defuse';
export const MATERIALS = GWOT_MATERIALS;
export const NOTES = `${PLAIN_NOTES ?? ''}${LOOK_NOTE}`;
export const build = buildPlain;
const R = DEFUSE_ROOMS;
/** the firing wire's copper. The palette has no paint of this hue, so it is a colour of this file's own and only decals wear it; the outside door's paint is
 *  this, darker: src/scene/look.ts LOOK_DOORS */
const COPPER: Vec3 = [0.46, 0.21, 0.07];
const SKIN: Skin = {
  wall: 'mud', house: 'mud', inner: 'whitewash', frameOut: 'woodGrey', frameIn: 'poplar', dado: null, ground: 'dust', verge: 'dirt', patches: 6,
  floor: { carpet: 'mudFloor', lino: 'slab' },
  // defuse.ts's own colours as the stuff they are: the workshops' sealed floor is the bare slab this file says it is, the hall's runner and the big mat are
  // plain cloth, the two workshops' own mats keep their red and their blue as carpet (from above that is how the two charges' rooms are told apart; the
  // wire's copper stays this place's one accent), and the two cars in the lot wear car paint
  own: [[[0.2, 0.21, 0.24], 'slab'], [[0.3, 0.17, 0.14], 'blanketBrown'], [[0.3, 0.16, 0.13], 'rugRed'], [[0.16, 0.2, 0.3], 'rugNavy'], [[0.32, 0.2, 0.14], 'blanketBrown'],
    [[0.3, 0.27, 0.2], 'truckTan'], [[0.2, 0.22, 0.26], 'paintBlue']],
  noRug: /shop|hall/, metals: ['iron', 'rust', 'tin'],
  rooms: [{ id: 'hall', bounds: R.hall.bounds }],
  dress: (G, _level, T) => {
    const M = lookMarks(T), paint = T.tone(P.stencil, 0.92), K = P.iron, W = 0.035, GAP = 0.09;
    // THE LOT'S GROUND, laid before any paint. WHEELS: the street along the south edge and each car's own way in off it. FEET: the walk up to the front door,
    // and the beat that parts before the house and goes round each end to a workshop's back door: the level's two ways to each charge, worn into the ground.
    { const lot = T.floorAt(20, 22) ?? P.dust;
      /** a beat feet have worn: the lot's own earth darker down its middle, with a fainter shoulder either side (no edge to read as a painted line) */
      const beat = (pts: [number, number][]) => { M.path(pts, 1.2, T.tone(lot, 0.9)); M.path(pts, 0.5, T.tone(lot, 0.78)); };
      G.ruts(0.6, 25.6, 39.4, 25.6); G.ruts(39.4, 20.85, 29.0, 20.66); G.ruts(0.6, 21.5, 10.0, 21.3);
      beat([[20.0, 24.7], [20.0, 17.7]]);
      beat([[19.7, 18.6], [5.3, 18.6], [5.3, 11.4]]); beat([[20.3, 18.6], [34.7, 18.6], [34.7, 11.4]]); }
    /** a twin lead along a run of points: two thin copper lines a hand apart */
    const lead = (pts: [number, number][]) => {
      M.path(pts, W, COPPER);
      M.path(pts.map(([x, z]) => [x + GAP, z + GAP] as [number, number]), W, T.tone(COPPER, 0.6));
    };
    // THE BATTERY PLATE in the middle of the hall, against its far wall: a dark board, two terminals
    M.fill(19.7, 13.25, 20.6, 13.85, K); M.outline(19.7, 13.25, 20.6, 13.85, 0.05, T.tone(K, 1.6));
    M.fill(19.85, 13.4, 20.0, 13.55, COPPER); M.fill(20.3, 13.4, 20.45, 13.55, COPPER);
    // THE WEST RUN: plate → along the hall wall → in at the west door → round the room's rug → through the workshop door → the bench
    lead([[19.85, 13.5], [15.25, 13.5], [15.25, 12.3], [13.2, 12.3], [13.2, 10.25], [10.9, 10.25], [10.9, 8.5], [8.5, 8.5]]);
    // THE EAST RUN, its mirror
    lead([[20.45, 13.5], [25.25, 13.5], [25.25, 12.3], [27.0, 12.3], [27.0, 10.25], [29.1, 10.25], [29.1, 8.5], [31.5, 8.5]]);
    // 1 AND 2 at the workshop doors, on the side the squad comes from
    M.glyph('1', 12.9, 11.2, 0.7, paint); M.glyph('2', 27.1, 11.2, 0.7, paint);
    // …and outside each workshop's back door, on the ground of the house's ends
    M.glyph('1', 4.6, 11.6, 0.7, paint); M.glyph('2', 35.4, 11.6, 0.7, paint);
    // THE WORKSHOPS: scorched slab under each bench, spilt oil by the shelves
    for (const x of [8.5, 31.5]) { const fa = T.floorAt(x, 8.4); if (fa) M.fill(x - 1.1, 7.75, x + 1.1, 8.25, T.tone(fa, 0.6)); }
    G.patch(9.1, 11.8, 0.9, 0.6, 'oil', 0.4); G.patch(30.9, 11.8, 0.8, 0.6, 'oil', -0.3);
  },
};
export const finish = (level: Level, groups: string[]) => skinLevel(level, groups, SKIN);
