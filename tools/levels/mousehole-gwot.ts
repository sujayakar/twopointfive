// MOUSEHOLE IN THE DEFAULT LOOK (src/scene/look.ts): tools/levels/mousehole.ts's clinic, box for box, in other clothes — tools/levels/gwotSkin.ts says what may
// change (paint and finish, decals, the paint of a prop) and checks that nothing a round, a charge or a blast reads did. `bun run tools/levels/export.ts mousehole`.
// ITS OWN PLACE: a district clinic — cement render outside, whitewash in, bare slab underfoot. What it alone has: THE QUEUE. A dashed line in the clinic's teal
// runs the length of the corridor from the porch to a 'wait here' box under the duty room's hatch (the lane the hatch gun looks down, read off the ground), with
// a standing square in front of each room's door, and a teal cross on the porch. No accent paint on a wall.
import { build as buildPlain, NOTES as PLAIN_NOTES, MOUSEHOLE_ROOMS, MOUSEHOLE_PAINT as OWN } from './mousehole';
import { GWOT_MATERIALS } from '../../src/scene/gwotKit';
import { skinLevel, LOOK_NOTE, type Skin } from './gwotSkin';
import { lookMarks } from './lookMarks';
import type { Level } from '../../src/scene/level';
import type { Vec3 } from '../../src/math/vec';

export const NAME = 'mousehole-gwot';
export const DRESS_AS = 'mousehole';
export const MATERIALS = GWOT_MATERIALS;
export const NOTES = `${PLAIN_NOTES ?? ''}${LOOK_NOTE}`;
export const build = buildPlain;
const R = MOUSEHOLE_ROOMS;
/** the clinic's teal. The palette has no paint of this hue, so it is a colour of this file's own and only decals wear it; the outside door's paint is this,
 *  darker: src/scene/look.ts LOOK_DOORS */
const TEAL: Vec3 = [0.07, 0.3, 0.29];
const SKIN: Skin = {
  // WHAT EACH THING IS MADE OF, where the shared rules had no word for it or a wrong one (the look draws a paint as a material by its palette row, and a paint
  // with no material is drawn flat). `own`: the clinic's own colours, by the row each wears — the exam room's sheet is the slab the other rooms stand on
  // (the shared rule read a pale floor as block), the car on the lot and the water tank are painted steel, the kerbs precast concrete (they were army olive). `pick`: a clinic's white things are enamelled
  // steel or cloth, never moulded plastic
  own: [[OWN.sheet, 'slab'], [OWN.carDark, 'paintBlue'], [OWN.tank, 'drumBlue'], [OWN.kerb, 'tWall']],
  pick: c => {
    const n = c.name;
    if (c.group === 'ward' && n === 'metal') return 'truckWhite';                     // the cots' frames: white enamel
    if (n === 'partBlue') return 'blanketGrey';                                       // a folded blanket (grey wool: the navy rug is too dark to stand for the paint it was)
    if (n === 'white' && !c.full) return (c.y1 - c.y0 < 0.15 && c.long > 1.5) || (c.thin < 0.06 && c.y1 > 1.5) ? 'sack' : 'truckWhite';   // a mattress (lying, a man long), the folding screen (thin, head high): cloth. Lockers, the nurse's table, the fridge: enamel
    return undefined;
  },
  wall: 'render', house: 'render', inner: 'whitewash', frameOut: 'woodGrey', frameIn: 'poplar', dado: null, ground: 'dust', verge: 'dirt', patches: 5,
  floor: { carpet: 'slab', lino: 'slab', stone: 'slab' },
  noRug: /ward|exam|store|corridor/, metals: ['galv', 'tin', 'plasticWhite', 'iron'],
  rooms: [{ id: 'corridor', bounds: R.corridor.bounds }],
  dress: (G, _level, T) => {
    const M = lookMarks(T), worn = T.tone(TEAL, 0.8);
    // THE QUEUE: porch → the hatch, a stride off the benches on the south wall
    M.dashed(9.6, 17.6, 25.2, 17.6, 0.08, worn, 0.7, 0.5);
    M.path([[25.2, 17.6], [26.1, 17.6], [26.1, 16.4]], 0.08, worn);
    // WAIT HERE: the box under the hatch
    M.outline(25.5, 15.3, 26.7, 16.3, 0.08, TEAL);
    // one at a time: a standing square in front of each room's door
    for (const x of [12.05, 18.05, 24.05]) M.outline(x - 0.5, 15.25, x + 0.5, 16.05, 0.06, worn);
    // THE CROSS on the porch
    M.fill(7.3, 16.2, 7.8, 17.7, TEAL); M.fill(6.8, 16.7, 8.3, 17.2, TEAL);
    // THE GROUND AS THE PLACE IS USED (from above it is most of the picture, and it was two sheets of one paint each): where wheels and feet go it is its
    // own paint trodden darker, and what the clinic's work leaves is a stain where it is left. Nothing here is a colour laid on the ground.
    const lot = T.floorAt(20, 23), yard = T.floorAt(20, 3);
    if (lot && yard) {
      const trod = (pts: [number, number][], w: number, of: Vec3, k = 0.8) => M.path(pts, w, T.tone(of, k));
      // the lot: the wheels' way in to each car's tail (the clinic's own from the west, the other from the street), the queue's way from the street to the porch, and from the porch round to the pump
      G.ruts(0.4, 20.08, 3.2, 20.36, 'dirt'); G.ruts(20.0, 27.7, 20.0, 26.1, 'dirt'); G.patch(20.1, 22.4, 1.3, 0.9, 'oil', 0.3);
      trod([[13.0, 27.2], [11.2, 23.6], [9.0, 20.0], [8.2, 18.9]], 1.1, lot);
      trod([[6.9, 15.9], [5.6, 14.3], [5.0, 13.5]], 0.8, lot); trod([[3.3, 9.4], [3.2, 7.2]], 0.7, lot);   // (each stops short of the wet round the pump: a path is never paler than what it crosses)
      // the back yard: the same path on to the burn drums and the ash round them, the bare strip under the washing, the wet under the tank's tap
      trod([[3.2, 6.6], [3.6, 5.2], [4.6, 3.4]], 0.7, yard, 0.76); G.patch(5.0, 2.3, 2.4, 1.9, 'damp', 0.4); G.patch(5.3, 2.1, 1.3, 1.0, 'damp', -0.5, 0.0128);
      trod([[16.4, 1.5], [22.8, 1.5]], 1.0, yard, 0.78); G.patch(31.2, 4.4, 1.9, 1.2, 'damp', 0.2);
      // the side: what the generator drips, the few steps from the fire exit to it
      G.patch(35.5, 13.5, 1.7, 1.1, 'damp', 0.5); trod([[34.2, 9.6], [34.5, 10.9]], 0.7, yard, 0.76);
      trod([[36.9, 14.4], [37.3, 18.6], [36.6, 22.6], [35.2, 26.6]], 0.8, yard, 0.76); G.patch(38.2, 19.2, 1.8, 1.3, 'damp', 1.1);   // the way round to the lot, past what was thrown out
    }
  },
};
export const finish = (level: Level, groups: string[]) => skinLevel(level, groups, SKIN);
