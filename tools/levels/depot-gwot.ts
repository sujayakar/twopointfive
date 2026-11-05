// DEPOT AS THE PAGE LOADS IT (src/scene/look.ts): tools/levels/depot.ts's level, box for box, in other clothes — tools/levels/gwotSkin.ts says what may change
// (paint and finish, decals, the paint of a prop) and checks that nothing a round, a charge or a blast reads did. `bun run tools/levels/export.ts depot-gwot`.
// DRAFT (the depot's rework, 11-05): THE WHITE ROOF'S WAY ON A LEVEL THAT IS PLAYED. The maintainer: "rework another level to use something similar to our
// rooftop demo? probably less obvious mirror's edge (so rethink the primary colors a bit)". ITS OWN PLACE: a haulier's freight yard, kept, and played at
// night under three floodlights. So the ground is a warm stone a full step UNDER the huts' skim (a floodlit yard keeps three values; pale ground under
// pale walls with one colour on everything is the roof's own formula, and clips to one value under a lamp); the dark road's tarmac comes in at one gate
// as the lane a lorry drives; the yard wall is cast panels between the two; the huts are painted block with a dado in the firm's deep green along the
// one long face that looks at the camera; a second colour, a brick, for the hut's end wall and for what burns (the fuel point, the generating set).
// The ground is painted AS A PRIVATE YARD PAINTS IT: one edge line on the side men walk and none down the middle (a middle line is a public road's), a
// steel channel where the gate closes, bays outlined with their long side, a dock hatched on the bare ground, aprons a man walks from door to door
// along the walls (they are the level's two answers: the stores' way and round the back). NOTHING IS PAINTED FOR THE PLAYER (the maintainer, 11-05:
// "let's avoid large markings (e.g. numbers or room names) where they don't seem realistic"): no letter, no room's name, no figure. What stands in for
// them is small and has a cause: where an engine drips, a slab that was relaid. (He chose this lay on 11-05 over a pale one with jade, a dusty rose,
// road lines and bay numbers; three more palettes were drawn and dropped.)
// ONE MECHANISM: the skin below speaks the kit's rows as every look file does, and `of` says which row of the second palette each wears here
// (tools/levels/coastSkin.ts; gwotSkin.ts Skin.of). A palette is a table: `yardOf` is the quiet table with a yard's rows changed.
import { build as buildPlain, NOTES as PLAIN_NOTES, DEPOT_ROOMS, DEPOT_PAINT as OWN } from './depot';
import { COAST_PALETTE as C, COAST_MATERIALS, type CoastMat, type GwotMat } from '../../src/scene/gwotPalette';
import { skinLevel, type Skin } from './gwotSkin';
import { COAST_QUIET_OF } from './coastSkin';
import { lookMarks } from './lookMarks';
import * as MD from '../../src/scene/markingDefaults';
import { WORLD } from '../../src/scene/world';
import type { Level } from '../../src/scene/level';
import type { Vec3 } from '../../src/math/vec';

export const NAME = 'depot-gwot';
export const DRESS_AS = 'depot';
export const MATERIALS = COAST_MATERIALS;
export const NOTES = `${PLAIN_NOTES ?? ''} — AS THE PAGE LOADS IT (a kept haulier's yard, the white roof's way: tools/levels/depot-gwot.ts): the same boxes repainted, decals added; what a man collides with, hides behind or sees through is the plain file's.`;
export const build = buildPlain;
const R = DEPOT_ROOMS;

/** HOW KEPT THE PLACE IS (docs/level-format.md): a yard somebody keeps. Its paint is whole and its walls are clean; a floor is still worn where it is walked */
export const KEPT = 1;

/** A YARD'S PAINTS (rows of the second palette). FOUR VALUES, each with a reason: the lane a lorry drives the road's own dark tarmac; the ground a warm
 *  stone a step over it; the yard wall cast panels a step over that; the huts and the aprons a man walks along them the palest, a smooth skim; the huts'
 *  floors the darkest stone (an inside reads as an inside). Then two fields, each as an enamel on a thing and as the huts' block painted (a band on a wall
 *  is a wall's stuff in the field's colour), what the cabin wears, and the painter's two tins for lines: one worn off-white on the tarmac (a white line
 *  blooms under the lamp), one faded for the stone */
export interface YardPaints { paver: CoastMat; panel: CoastMat; skim: CoastMat; floor: CoastMat; field: CoastMat; second: CoastMat; fieldWall: CoastMat; secondWall: CoastMat; cabin: CoastMat; lane: CoastMat; bay: CoastMat }
/** BOTTLE GREEN AND BRICK ON A WARM STONE */
export const YARD: YardPaints = { paver: 'deepPaver', panel: 'deepPanel', skim: 'deepSkim', floor: 'deepFloor', field: 'bottle', second: 'oxide', fieldWall: 'bottleBlock', secondWall: 'oxideBlock', cabin: 'graphite', lane: 'lineWorn', bay: 'lineFaded' };
/** which row of the second palette each of the kit's rows wears in a yard: the quiet table, and what a yard changes in it */
export const yardOf = (Y: YardPaints): Readonly<Record<GwotMat, CoastMat>> => ({ ...COAST_QUIET_OF,
  slab: Y.paver, dust: Y.paver, mudFloor: Y.floor, tWall: Y.panel, block: 'paleGravel', render: Y.skim, whitewash: Y.skim, paintCream: Y.skim, mudLight: Y.skim,
  olive: Y.field, paintRust: Y.field, genYellow: Y.second, tarpBlue: Y.cabin, drumRed: Y.field, paintGreen: Y.fieldWall, paintBlue: Y.secondWall,
  oil: 'yardTarmac', dirt: 'yardTarmac', scrub: 'soil', straw: 'soil', prodGreen: 'soil',   // (the scrub outside the wall: the look has no leaf yet, so it is a dark clump at the wall's foot and not a green block)
  iron: 'graphite', drumBlue: 'graphite', rust: 'graphite', galv: 'brushedSteel', tin: 'brushedSteel' });

export const yardSkin = (Y: YardPaints): Skin => ({
  of: yardOf(Y),
  // WHAT EACH THING WEARS, in the kit's rows (the table above says what each row is here): the hardstanding and the hut floors are the two floors; the
  // main gate's leaves, the door frames and a lorry's chassis are the firm's, and a lorry's body is nobody's livery (the pick below: one colour on
  // everything is a designer's field, not a firm's paint); the generating set is the second field's and the cabin dark steel (two fields of one hot
  // colour side by side, the hut's wall and the cabin, read as one red block on the card); the drums are the firm's (they stand on the second field's pad); cast iron is dark steel.
  own: [[OWN.hard, 'slab'], [OWN.raised, 'mudFloor'], [OWN.drab, 'olive'], [OWN.shelter, 'tarpBlue'], [OWN.set, 'genYellow'], [OWN.oxide, 'drumRed'], [OWN.gate, 'paintRust'], [OWN.iron, 'iron']],
  // (a lorry's body says car paint and says it still in the huts' cream: the row's own word is a plain skim)
  pick: c => c.name === 'wallDark' ? 'whitewash' : c.b.stuff === 'carpaint' ? { paint: 'whitewash', stuff: 'carpaint', rough: 0.26 } : undefined,   // (and the wash room's stall partitions)
  wall: 'tWall', house: 'render', inner: 'whitewash', ground: 'oil', frameOut: 'paintRust', frameIn: 'paintRust', dado: 'paintRust',
  floor: { carpet: 'mudFloor', stone: 'mudFloor', lino: 'mudFloor' }, noRug: /./, rugs: { crew: 'mat' },
  skirts: false, patches: 0,                                                           // a kept place: no damp foot, no drifts
  metals: ['galv', 'tin'],
  rooms: [{ id: 'duty', bounds: R.duty }],
  dress: (G, level, T) => {
    const M = lookMarks(T), B = level.boxes, H = WORLD.ceilingY;
    const SECOND = C[Y.second], APRON = C[Y.skim], ROAD = C.yardTarmac, BAYP = C[Y.bay], LANEP = C[Y.lane], HZA = C.lineInk, HZB = C.lineChalk, SLOT = C.darkSteel;
    /** a field on the ground in a paint: under every line, and under a door's worn threshold */
    const field = (x0: number, z0: number, x1: number, z1: number, paint: Vec3) => T.flat((x0 + x1) / 2, (z0 + z1) / 2, x1 - x0, z1 - z0, paint, 0, 0.0104);
    /** a piece of the ground in its own paint (or the paint of the field it lies on), to carry what is painted on it (a box holds four markings: the ground is laid in the pieces that carry its paint) */
    const piece = (x0: number, z0: number, x1: number, z1: number, paint = T.floorAt((x0 + x1) / 2, (z0 + z1) / 2)!) => T.flat((x0 + x1) / 2, (z0 + z1) / 2, x1 - x0, z1 - z0, paint, 0, 0.0116);
    const rectTop = (i: number, x0: number, z0: number, x1: number, z1: number): [number, number, number, number] => [x0 - B[i].c[0], B[i].c[2] - z1, x1 - B[i].c[0], B[i].c[2] - z0];
    /** a hatched square where nothing may stand, an edge round it */
    const hatch = (x0: number, z0: number, x1: number, z1: number, on: Vec3) => { const i = piece(x0, z0, x1, z1, on); return T.mark(i, MD.hazardMarks('+y', rectTop(i, x0, z0, x1, z1), { albedo: HZA, rough: 0.55 }, undefined, 2, 'coarse', 1)); };
    /** a line `off` to the side of a way through points (square joints) */
    const beside = (pts: [number, number][], off: number): [number, number][] => pts.map(([x, z], k) => { const [ax, az] = pts[Math.max(0, k - 1)], [bx, bz] = pts[Math.min(pts.length - 1, k + 1)], L = Math.hypot(bx - ax, bz - az) || 1; return [x - (bz - az) / L * off, z + (bx - ax) / L * off]; });
    /** a strip of a paint through points, `w` wide, under every line */
    const strip = (pts: [number, number][], w: number, paint: Vec3) => {
      const dir = (k: number) => Math.atan2(pts[k + 1][1] - pts[k][1], pts[k + 1][0] - pts[k][0]);
      /** how far a length runs on past a joint so that its outer edge meets the next length's at the mitre (half the width by the tangent of half the turn) */
      const past = (k: number) => k <= 0 || k + 1 >= pts.length ? 0 : w / 2 * Math.abs(Math.tan((dir(k) - dir(k - 1)) / 2));
      for (let k = 0; k + 1 < pts.length; k++) { const [ax, az] = pts[k], [bx, bz] = pts[k + 1], L = Math.hypot(bx - ax, bz - az), a = dir(k), e0 = past(k), e1 = past(k + 1), m = (e1 - e0) / 2;
        T.flat((ax + bx) / 2 + Math.cos(a) * m, (az + bz) / 2 + Math.sin(a) * m, L + e0 + e1, w, paint, -a, 0.0104); } };

    // ---- THE ONE LONG FACE (the north row's yard wall, z 9.5: a quarter of a frame is wall faces, and this is the biggest that looks at the camera): the wall
    // itself is smooth (the ground is jointed: where the ground is, the wall is quiet); a dado of painted block in the firm's colour to 0.9 m, door to door
    // (the look kit's own band from the floor, in the kit row that wears the field's block). And the hut's west face, which the yard's length looks at.
    for (const [a, b] of [[4.12, 5.8], [7.0, 11.8], [13.0, 20.8], [22.0, 23.88]] as const) G.skirt('x', 9.5, a, b, 1, 'paintGreen', 0.9, 0.24);
    G.skirt('z', 24, 9.62, 15.12, -1, 'paintGreen', 0.9, 0.24);
    // ---- THE HUT'S SOUTH FACE, over the mouth: one whole wall in the second colour (its window is a hole: the colour stops at the sill under it)
    G.skirt('x', 15, 23.88, 24.2, 1, 'paintBlue', H, 0.24); G.skirt('x', 15, 25.0, 29.62, 1, 'paintBlue', H, 0.24); G.skirt('x', 15, 24.2, 25.0, 1, 'paintBlue', 0.95, 0.24);

    // ---- THE LANE A LORRY DRIVES: the road's own tarmac in at the main gate, down the yard, round the hut's corner in an eased bend, through the mouth
    // and on to the back gate. The one dark field of the yard, and where nobody on foot should stand. One worn line along the side men walk; a steel
    // channel let into the ground where the main gate's leaves close
    const LANE: [number, number][] = [[4.12, 12.7], [17.2, 12.7], [19.4, 13.2], [22.4, 15.5], [24.9, 16.8], [35.88, 17.4]], LW = 2.8;
    strip(LANE, LW, ROAD);
    M.path(beside(LANE, -(LW / 2 - 0.05)), 0.1, LANEP);
    M.seg(4.6, 11.3, 4.6, 14.1, 0.16, SLOT);
    // ---- THE APRONS A MAN WALKS, door to door along the walls, in the huts' own smooth skim, a slot drain between each and the yard. They are the level's
    // own answers: along the north row; THE STORES' WAY along the south row's wall behind the lorry to the workshop's side door and the store; ROUND THE
    // BACK from the back gate through the gap, up the lane behind the cabin to the generator room's door
    field(4.12, 9.62, 23.88, 10.6, APRON); field(4.12, 10.6, 23.88, 10.7, SLOT);
    field(4.12, 17.42, 24.0, 18.38, APRON); field(4.12, 17.32, 24.0, 17.42, SLOT);
    strip([[35.0, 15.9], [35.0, 6.9], [29.62, 6.9]], 0.9, APRON);
    M.path(beside([[35.0, 15.9], [35.0, 6.9], [29.62, 6.9]], 0.5), 0.07, BAYP); M.path(beside([[35.0, 15.9], [35.0, 6.9], [29.62, 6.9]], -0.5), 0.07, BAYP);
    // a hatched square before the store's hatch, where nothing may stand (a door's own threshold is worn, and says so itself)
    hatch(19.4, 17.5, 21.4, 18.3, APRON);
    // ---- THE BAYS on the yard's south side (the lorry stands in the first, ALONG the lane, so a bay's long side is drawn; the second is empty, and an
    // empty bay says there is no cover here), and two for trailers in the east yard. A faded line on the stone; no numbers
    for (const x of [9.6, 15.4, 18.8]) M.seg(x, 14.85, x, 17.32, 0.1, BAYP);
    M.seg(9.6, 14.85, 18.8, 14.85, 0.1, BAYP);
    for (const x of [30.2, 33.0, 35.75]) M.seg(x, 19.6, x, 23.8, 0.1, BAYP);
    // ---- WHAT USE LEFT, each with its cause and most of the ground blank: a drip where an engine stands in each bay (the lorry's own is under it); a
    // slab that was lifted and relaid, a shade newer, cut to the joints. (Tyre polish into the first bay was tried as two films and read on the card as a
    // pale bar lying on the lane: a curve of wear is the look's own to draw along a way, not a box's.)
    { const fa = T.floorAt(14, 16); if (fa) { const drip = T.tone(fa, 0.66), fresh = T.tone(fa, 1.07);
      for (const [x, z, w, d, yaw] of [[18.0, 16.25, 0.42, 0.3, 0.3], [31.9, 22.6, 0.45, 0.32, 0.8], [34.7, 22.9, 0.36, 0.26, 0.2]] as const) T.flat(x, z, w, d, drip, yaw, 0.0117);
      T.flat(7.25, 16.0, 1.5, 1.0, fresh, 0, 0.0105); T.flat(27.75, 19.5, 1.0, 2.0, fresh, 0, 0.0105); } }
    // ---- THE DOCK against the hut's west wall: hatched on the bare ground, its approach edge in the second colour. (A field of the firm's colour here
    // burnt out to a blank pale sheet under the hut's floodlight; a hatch leaves dark detail when the lamp blooms.) A hatch is one bitmap tall: 1.6 m
    { const fa = T.floorAt(22.4, 11.9); if (fa) hatch(21.2, 11.32, 23.75, 12.9, fa); M.seg(21.2, 12.97, 23.75, 12.97, 0.14, SECOND); }
    // ---- THE FUEL POINT in the east yard: a pad of the second colour, an edge to it
    field(26.4, 22.2, 28.8, 23.86, SECOND);
    M.hazard(26.25, 22.05, 28.8, 22.2, HZA, HZB, 0.3); M.hazard(26.25, 22.2, 26.4, 23.86, HZA, HZB, 0.3);
    // ---- THE BAY: the inspection pit's edge, round the lorry that stands over it
    M.hazard(10.9, 21.55, 15.9, 21.7, HZA, HZB); M.hazard(10.9, 21.7, 11.1, 23.7, HZA, HZB, 0.25); M.hazard(15.7, 21.7, 15.9, 23.7, HZA, HZB, 0.25);
    // ---- THE SIGNALS HUT: the darker floor round the desk
    { const fa = T.floorAt(27, 12.6); if (fa) M.fill(25.7, 11.7, 28.4, 13.7, T.tone(fa, 0.82)); }
  },
});
export const finish = (level: Level, groups: string[]) => skinLevel(level, groups, yardSkin(YARD));
