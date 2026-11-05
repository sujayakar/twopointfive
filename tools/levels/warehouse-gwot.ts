// WAREHOUSE IN THE DEFAULT LOOK (src/scene/look.ts): tools/levels/warehouse.ts's level, box for box, in other clothes — tools/levels/gwotSkin.ts says what may change
// (paint and finish, decals, the paint of a prop) and checks that nothing a round, a charge or a blast reads did. `bun run tools/levels/export.ts warehouse-gwot`.
// ITS OWN PLACE: a bonded customs shed on the ring road — bare block, hazard yellow and black, rust. What it alone has: LOGISTICS PAINT (a hazard band
// across both sides of every hall door, a number on the apron outside each of the five ways in, lane lines along the racks, pallet squares on the marshalling
// floor and the dock, truck bays outside the shutters, the staff's parking by their door). No rugs, no dados, no earth indoors.
import { build as buildPlain, NOTES as PLAIN_NOTES, WAREHOUSE_ROOMS } from './warehouse';
import { GWOT_MATERIALS } from '../../src/scene/gwotKit';
import { GWOT_PALETTE as P, type GwotMat } from '../../src/scene/gwotPalette';
import { skinLevel, LOOK_NOTE, type Skin } from './gwotSkin';
import { lookMarks } from './lookMarks';
import type { Level } from '../../src/scene/level';

export const NAME = 'warehouse-gwot';
export const DRESS_AS = 'warehouse';
export const MATERIALS = GWOT_MATERIALS;
export const NOTES = `${PLAIN_NOTES ?? ''}${LOOK_NOTE}`;
export const build = buildPlain;
const R = WAREHOUSE_ROOMS;
/** WHAT THE PLAIN FILE WROTE AS COLOURS OF ITS OWN, by what each is (the skin's `own`: the look can only draw such a colour flat, and the shared rules took a
 *  lived floor for furniture — the hardstanding came out galvanised sheet, the green lino a blanket). The shed's yellow steel (the rolling stair, the bollards,
 *  the sack trucks, the forklift, the clamp) is generator yellow; then the lived floors: the container's hardstanding concrete, the tea room's offcut brown
 *  felt, the porters' duckboard grey timber, the manifest office's offcut green matting, the queue's wear on the cashier's floor that floor trodden brown, the
 *  wear in the verge bare earth */
const OWN: Skin['own'] = [[[0.75, 0.55, 0.08], 'genYellow'],
  [[0.4, 0.38, 0.34], 'slab'], [[0.3, 0.4, 0.5], 'blanketBrown'], [[0.34, 0.26, 0.17], 'woodGrey'], [[0.16, 0.27, 0.2], 'rugGreen'], [[0.12, 0.12, 0.14], 'blanketBrown'], [[0.25, 0.21, 0.15], 'mudFloor']];
const LOADS: GwotMat[] = ['sandbag', 'olive', 'sandbag', 'tarpBlue', 'olive', 'ply', 'sandbag', 'olive', 'sandbag', 'ply'];   // (sacking is burlap brown: in the palette's pale `sack` four loads in ten were the palest things on the floor, white cubes under a panel)
const SKIN: Skin = {
  wall: 'block', house: 'block', inner: 'block', frameOut: 'genYellow', frameIn: 'woodGrey', verge: 'gravel', dado: null, noRug: /./,
  floor: { lino: 'slab', concrete: 'slab', carpet: 'canvas' },
  metals: ['rust', 'galv', 'iron', 'genYellow'],
  rooms: [{ id: 'north_corridor', bounds: R.ncorr }, { id: 'west_corridor', bounds: R.wcorr }],   // (the scuffed band along a corridor's walls)
  // THE HALL: its racking's blue uprights and orange beams are a European distribution shed's, its loads taped cartons — here rusted frames, grey timber
  // decks, and on them burlap sacks, olive crates, a tarp-wrapped pallet, plywood cases (each standing for exactly what it was: a carton is still a carton to a round)
  pick: c => { const a = c.b.albedo;
    if (c.name === undefined && a[2] > 0.4 && a[0] < 0.15) return 'rust';
    if (c.name === undefined && a[0] > 0.6 && a[1] < 0.4 && a[2] < 0.1) return 'woodGrey';   // (the racking's orange beams — not the forklift's yellow body: that is the shed's yellow, OWN below)
    if (c.name === 'cardboard' && c.y1 - c.y0 > 0.5) { const m = LOADS[(c.i * 7 + 3) % LOADS.length]; return m === 'sandbag' && c.group === 'exterior.yard' ? 'tarpBlue' : m; }   // (sacks left out on the apron are under a tarp)
    if (c.name === 'wood' && c.y1 - c.y0 > 0.8 && c.thin > 0.7) return 'ply';
    // THE LIVED THINGS IN AN OFFICE PAINT the shared rules leave be or take for something else
    if (!c.group.startsWith('lived.')) return undefined;
    if (c.name === 'red') return 'drumRed';                                                   // drink crates, the bay's and the bond's red lines: red enamel
    if (c.y1 <= 0.1 && c.name === 'white') return 'truckWhite';                               // the scale's edge, the electrician's mat: white paint
    if (c.y1 <= 0.1 && c.name === 'woodDark' && c.group === 'lived.wcorr') return 'coyote';   // the coir mat inside the staff door
    return undefined; },
  own: OWN,
  dress: (G, _level, T) => {
    const M = lookMarks(T), Y = P.genYellow, K = P.iron, paint = T.tone(Y, 0.95);   // (floor paint in the shed's own yellow, a little worn: the look draws a classed row darkened in one piece as that row's stuff, and the stencil ochre these were in has no stuff, so 110 decals drew flat)
    G.ruts(30.0, 26.0, 39.0, 26.0); G.patch(20, 16.6, 1.5, 1.0, 'oil', 0.4); G.patch(20.5, 16.3, 0.9, 0.7, 'oil', 1.2, 0.0125); G.patch(27, 19, 1.3, 0.9, 'oil', 1.1); G.patch(16, 21.4, 2.6, 1.6, 'damp', 0.2);
    // lane lines along the racks' feet, worn through in places
    for (const z of [13.7, 15.3, 18.7, 20.3]) for (let x = 14.4, k = 0; x < 26.4; x += 2.0, k++) { if ((k * 5 + Math.round(z * 10)) % 7 === 0) continue; M.seg(x, z, x + 1.4, z, 0.16, paint); }
    // A HAZARD BAND either side of every door of the hall (and the dock's own north door): where the lanes are entered is what the eye finds first
    const band = (x: number, z: number, wallAlongX: boolean) => { for (const s of [-1, 1]) { if (wallAlongX) M.hazard(x - 0.9, z + s * 0.62 - 0.2, x + 0.9, z + s * 0.62 + 0.2, Y, K); else M.hazard(x + s * 0.62 - 0.2, z - 0.9, x + s * 0.62 + 0.2, z + 0.9, Y, K); } };
    band(13.0, 19.4, false); band(22.4, 11.0, true); band(30.0, 14.4, false); band(30.0, 21.4, false); band(33.4, 11.0, true);
    // THE FIVE WAYS IN, numbered on the apron outside each: the staff door, the fire exit, the plant room's door, the dock's two
    M.glyph('1', 11.4, 25.7, 1.4, paint); M.glyph('2', 2.4, 9.4, 1.4, paint); M.glyph('3', 6.4, 2.4, 1.4, paint); M.glyph('4', 37.7, 13.4, 1.4, paint); M.glyph('5', 37.7, 21.4, 1.4, paint);
    // THE STAFF'S PARKING off the south-west corner, by their door: three bays' lines, worn through at the wheels
    for (const x of [4.4, 6.9, 9.4]) { M.seg(x, 24.8, x, 25.85, 0.1, paint); M.seg(x, 26.25, x, 27.25, 0.1, paint); }
    // two TRUCK BAYS outside the shutters, backed up to and driven off a hundred times
    M.outline(36.5, 14.9, 39.9, 17.3, 0.12, paint, 'e'); M.outline(36.5, 17.3, 39.9, 19.7, 0.12, paint, 'e'); G.patch(38.2, 16.1, 1.6, 0.9, 'oil', 0.2); G.patch(38.0, 18.6, 1.2, 0.8, 'oil', -0.3);
    // PALLET SQUARES on the marshalling floor and in the dock, some standing empty, each with its tally
    const sq = (x: number, z: number, n: number) => { M.outline(x - 0.8, z - 0.7, x + 0.8, z + 0.7, 0.07, paint); if (n) M.tally(x, z + 0.95, n, paint, true, 0.3, 0.06, 0.14); };
    sq(27.4, 13.2, 1); sq(27.4, 15.3, 2); sq(27.4, 17.4, 3); sq(27.6, 22.4, 4); sq(34.9, 16.0, 1); sq(34.9, 18.6, 2); sq(32.6, 17.6, 0); sq(32.8, 20.4, 3); sq(32.8, 14.6, 4);
  },
};
export const finish = (level: Level, groups: string[]) => skinLevel(level, groups, SKIN);
