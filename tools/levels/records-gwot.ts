// RECORDS IN THE DEFAULT LOOK (src/scene/look.ts): tools/levels/records.ts's level, box for box, in other clothes — tools/levels/gwotSkin.ts says what may change
// (paint and finish, decals, the paint of a prop) and checks that nothing a round, a charge or a blast reads did. `bun run tools/levels/export.ts records-gwot`.
// ITS OWN PLACE: a district land office — the counter where a claim is filed, the plan books on their island of shelving, the title deeds behind the one locked
// door, the surveyors' room. Cream wash, black steel frames, and the surveyors' marker MAGENTA, on the ground only. What it alone has: SURVEYORS' MARKS. A closed
// TRAVERSE is chained round the building with a numbered station wherever a pane reads a room, and each station's SIGHT LINE runs on through its glass and across
// the floor to the first thing that stands in it: what that pane shows, drawn (the level's idea — look first, then choose the door — the same from either
// start). The plan books' two lanes are N and S; a bench mark's broad arrow lies at the strong room's door; a north point in the lot. No tiles, no measures.
// UNDER THE MARKS, THE GROUND AS IT IS USED (no line of its own): grit against the kerbs, the three bays' wheel ruts and oil, paving to the front door, the
// staff's trodden way round the west side to the back door, the back lane's bed and ruts along the north, a drain down the west, damp where the roofs shed.
import { build as buildPlain, NOTES as PLAIN_NOTES, RECORDS_ROOMS, RECORDS_PAINT } from './records';
import { GWOT_MATERIALS } from '../../src/scene/gwotKit';
import { GWOT_PALETTE } from '../../src/scene/gwotPalette';
import { skinLevel, LOOK_NOTE, type Skin } from './gwotSkin';
import { lookMarks } from './lookMarks';
import type { Level } from '../../src/scene/level';
import type { Vec3 } from '../../src/math/vec';

export const NAME = 'records-gwot';
export const DRESS_AS = 'records';
export const MATERIALS = GWOT_MATERIALS;
export const NOTES = `${PLAIN_NOTES ?? ''}${LOOK_NOTE}`;
export const build = buildPlain;
const R = RECORDS_ROOMS;
/** the marker paint. The palette (src/scene/gwotPalette.ts) has no paint of this hue, so it is a colour of this file's own and only decals wear it */
const MAGENTA: Vec3 = [0.58, 0.06, 0.36];
/** THE TRAVERSE, clockwise from the front door's sidelight. A station [x, z, toward the wall x, z, from, to] stands 1.2 m off a pane (records.ts's wall openings:
 *  the two sidelights, the five windows); from‥to = metres along its sight line where that line lies on the room's floor, from the wall's inside face (or past
 *  what stands under the pane) to the first thing standing in it, read off records.ts's furniture. A bare [x, z] is a turning point at a corner of the building */
const TRAVERSE: ([number, number, number, number, number, number] | [number, number])[] = [
  [14.25, 23.8, 0, -1, 1.42, 3.7],       // 1 the front door's sidelight: the front office, to the duty desk
  [8.7, 23.8],
  [8.7, 15.0, 1, 0, 1.42, 21.18],        // 2 the hall's end window: the whole length of the hall, to the fire exit
  [8.7, 11.6, 1, 0, 1.42, 9.24],         // 3 the records room's west window: the length of its south lane
  [8.7, 5.7],
  [14.1, 5.7, 0, 1, 1.42, 3.7],          // 4 its north window: across its north lane, to the island
  [25.75, 5.7, 0, 1, 1.42, 3.5],         // 5 the staff room's north window: to the chairs at the table
  [31.3, 5.7],
  [31.3, 10.25, -1, 0, 1.42, 4.3],       // 6 the east door's sidelight: the staff room, to the table (the one station that stands under the street lamp)
  [31.3, 18.3], [18.3, 18.3],
  [18.3, 20.3, -1, 0, 2.1, 8.18],        // 7 the front office's east window: over the bench under it, south of the duty desk, to the west wall
  [18.3, 23.8],
];
const SKIN: Skin = {
  // (the dado is gloss paint, a tan a step under the cream above it: the palette's cloth of that colour drew a woven wall)
  wall: 'paintCream', house: 'paintCream', inner: 'whitewash', frameOut: 'iron', frameIn: 'poplar', dado: 'truckTan',
  // WHAT THE GENERATOR'S OWN COLOURS ARE: car paint and dark glass on the two cars (a grey body cannot wear the white truck paint: a pale grey stands for
  // steel to a blast and the car was not), precast concrete for the kerbs, cloth for the old kit's sofa (its grey sat near enough a darkened slab to be drawn
  // as concrete), cinders under the burn drum, coir where the readers stand. ([0.07, 0.08, 0.1] is scene/levelKit.ts's own: its car's cabin.)
  own: [[RECORDS_PAINT.carLot, 'truckTan'], [RECORDS_PAINT.carYard, 'truckRed'], [[0.07, 0.08, 0.1], 'glassDark'], [RECORDS_PAINT.kerb, 'tWall'], [RECORDS_PAINT.sofa, 'blanketBrown'],
    [RECORDS_PAINT.ash, 'gravel'], [RECORDS_PAINT.coir, 'coyote']],
  floor: { lino: 'slab', carpet: 'canvas' }, metals: ['olive', 'galv', 'iron'], patches: 0,   // (no patches: one lay over the chain line)
  // (the two rooms people sit in get the dado; the front office keeps the level's own door mat, the staff room's felt is the floor's own tone: both are seen
  // through glass, where a dark square reads as a man. Records has an island where a rug would lie. The hall takes a corridor's scuffed band.)
  pick: c => c.group === 'front' && c.name === undefined && c.y1 < 0.03 ? 'coyote' : undefined,   // (that mat: coir, not the office's charcoal)
  rooms: [{ id: 'hall', bounds: R.hall.bounds }], noRug: /./, rugs: { front: 'felt', staff: 'felt' },
  dress: (G, _level, T) => {
    // (two sets of marks: M on the ground, Hi indoors — over the rooms' rugs, whose tops stand at 0.024)
    // THE GROUND AS IT IS USED, laid first and low (a kit patch at 8 mm tops out under every mark) so the surveyors' marks lie over it. No painted line of
    // its own: the kit's patches and ruts in paints of the palette, and the ground's own tone a step darker where feet go (STYLE.md: a patch is darker than
    // its ground and never under a third of it). The paving at the front door is a finish, not a patch: a slab, paler than the lot, as the kit's is.
    { const LOW = 0.008, worn = (x: number, z: number, w: number, d: number, k: number, yaw = 0) => { const fa = T.floorAt(x, z); if (fa) T.flat(x, z, w, d, T.tone(fa, k), yaw); };
      /** a stain: three boxes over each other, so it has no one outline */
      const blot = (x: number, z: number, s: number, yaw: number, mat: 'oil' | 'damp') => { G.patch(x, z, s, s * 0.7, mat, yaw, LOW); G.patch(x + s * 0.2, z - s * 0.1, s * 0.62, s * 0.5, mat, yaw + 0.6, LOW); G.patch(x - s * 0.18, z + s * 0.12, s * 0.45, s * 0.4, mat, yaw - 0.7, LOW); };
      // the grit the rain leaves against the kerbs: the south one's whole length, the east one's as far as the lot runs (north of it the verge is darker than grit)
      G.patch(20, 27.5, 39.4, 0.72, 'gravel', 0, LOW); G.patch(8.6, 27.2, 9.5, 0.5, 'gravel', 0.02, LOW); G.patch(31.5, 27.25, 11, 0.5, 'gravel', -0.015, LOW); G.patch(39.48, 17.1, 0.8, 20.4, 'gravel', 0, LOW); G.patch(39.2, 21.5, 0.4, 7.5, 'gravel', 0.02, LOW);
      // the three bays: wheels in over the kerb to each stop, and what an engine leaves where it stands
      for (const x of [19.4, 22.2, 25.0]) G.ruts(x, 27.7, x, 23.55);
      for (const [x, z, s, yaw] of [[19.5, 24.3, 0.45, 0.3], [22.35, 24.7, 0.32, -0.5], [25.05, 24.2, 0.5, 0.9], [12.75, 26.3, 0.36, 0.2], [23.9, 21.45, 0.4, -0.2]]) blot(x, z, s, yaw, 'oil');
      // the public's way: paving from the lot to the front door (the skin's own wear at the door lies on it), and the worn line on round the parked car to the bays
      T.flat(13.25, 23.52, 1.7, 1.8, GWOT_PALETTE.slab); worn(16.0, 24.0, 3.6, 0.9, 0.82, 0.03);
      // the staff's way: round the west side, past the burn drum, to the back door
      worn(7.8, 16.5, 0.8, 14.4, 0.76, 0.006); worn(8.35, 9.0, 1.4, 0.8, 0.76, 0.7);
      // the back lane along the north side: its bed a step darker than the verge, two ruts in it
      worn(20, 1.6, 39.6, 2.7, 0.8); G.ruts(0.4, 1.6, 39.6, 1.6, 'damp');
      // the street's end on the east side: where a car turns in between the barriers
      G.ruts(39.6, 15.2, 34.6, 15.2);
      // the west side's drain: a damp line along the yard's far edge, blots where it stands
      G.patch(1.3, 14.6, 0.8, 17.6, 'damp', 0.004, LOW); blot(1.7, 10.4, 1.4, 0.4, 'damp'); blot(1.5, 20.3, 1.2, -0.3, 'damp');
      // damp where the roofs shed: the front office's corner, under the bins
      blot(17.6, 23.0, 1.0, 0.2, 'damp'); blot(30.5, 17.9, 1.3, 1.3, 'damp'); }
    const M = lookMarks(T), Hi = lookMarks({ ...T, flat: (x, z, w, d, a, yaw = 0, y = 0.011) => T.flat(x, z, w, d, a, yaw, y + 0.0146) }), paint = MAGENTA, chain = T.tone(MAGENTA, 0.8);
    // (an N of its own, two uprights and the diagonal, in the font's 3 : 5: the font's N has no room for a diagonal and reads as an M from above)
    const N = (x: number, z: number, h: number) => { const px = h / 5; for (const s of [-1, 1]) M.seg(x + s * px, z - h / 2, x + s * px, z + h / 2, px, paint); M.seg(x - px, z - h / 2 + px / 2, x + px, z + h / 2 - px / 2, px * 0.9, paint); };
    // THE TRAVERSE: the chain line from point to point, a peg at each; a station is a cross on its peg, its sight line (whole to the foot of the pane, broken across the floor inside) and its number
    let n = 0;
    TRAVERSE.forEach((p, i) => {
      const q = TRAVERSE[(i + 1) % TRAVERSE.length], [x, z] = p;
      M.dashed(x, z, q[0], q[1], 0.06, chain, 1.1, 0.7);
      if (p.length === 2) { M.fill(x - 0.09, z - 0.09, x + 0.09, z + 0.09, paint); return; }
      const [, , tx, tz, a, b] = p;
      // (the surveyor's station: a triangle round its peg, apex toward the pane)
      { const r = 0.5, px = -tz, pz = tx, ax = x + tx * r, az = z + tz * r, bx = x - tx * r * 0.5 + px * r * 0.87, bz = z - tz * r * 0.5 + pz * r * 0.87, cx = x - tx * r * 0.5 - px * r * 0.87, cz = z - tz * r * 0.5 - pz * r * 0.87;
        M.seg(ax, az, bx, bz, 0.07, paint); M.seg(bx, bz, cx, cz, 0.07, paint); M.seg(cx, cz, ax, az, 0.07, paint); M.fill(x - 0.12, z - 0.12, x + 0.12, z + 0.12, paint); }
      M.seg(x + tx * 0.5, z + tz * 0.5, x + tx * 1.0, z + tz * 1.0, 0.05, paint); Hi.dashed(x + tx * a, z + tz * a, x + tx * b, z + tz * b, 0.05, chain, 0.5, 0.5);
      M.glyph(String(++n), x + tx * 0.62 + Math.abs(tz) * 0.62, z + tz * 0.62 + Math.abs(tx) * 0.62, 0.38, paint);   // (small, beside the triangle's apex: four other levels letter their floors large)
    });
    // THE PLAN BOOKS' TWO LANES, one letter each, mid-lane against the island (off every mark a man is dealt, off sight lines 3 and 4): N is the lane the west door
    // rakes, S the lane the hall door opens on
    N(15.0, 8.75, 0.7); M.glyph('S', 13.2, 11.05, 0.7, paint);
    // THE BENCH MARK at the strong room's door, on the records face: the bar and the broad arrow under it, pointing at the one door that is locked
    M.seg(17.4, 10.0, 17.4, 11.0, 0.1, paint); for (const dz of [-0.45, 0, 0.45]) M.seg(16.35 - (dz ? 0 : 0.1), 10.5 + dz, 17.3, 10.5, 0.08, paint);
    // THE NORTH POINT in the open lot: the doors are called by their sides (south lot, west yard, east side)
    { const x = 30; M.seg(x, 25.2, x, 22.0, 0.1, paint); M.seg(x, 21.95, x - 0.5, 22.75, 0.1, paint); M.seg(x, 21.95, x + 0.5, 22.75, 0.1, paint); M.seg(x - 0.6, 23.9, x + 0.6, 23.9, 0.07, paint);
      N(x, 21.15, 0.8); M.glyph('S', x, 26.0, 0.8, paint); }
    // THE PRACTICE PLOTS pegged out in the dead east end of the lot: three parcels in chain, a peg at every corner — a cadastral plan at full size
    { const p: [number, number][] = [[32.6, 19.6], [35.4, 19.9], [35.1, 22.6], [32.4, 22.2], [38.6, 19.5], [38.8, 23.1], [38.5, 26.6], [32.9, 26.3]];
      for (const [i, j] of [[0, 1], [1, 2], [2, 3], [3, 0], [1, 4], [4, 5], [5, 2], [5, 6], [6, 7], [7, 3]]) M.dashed(p[i][0], p[i][1], p[j][0], p[j][1], 0.06, chain, 0.9, 0.5);
      for (const [x, z] of p) M.fill(x - 0.09, z - 0.09, x + 0.09, z + 0.09, paint); }
  },
};
export const finish = (level: Level, groups: string[]) => skinLevel(level, groups, SKIN);
