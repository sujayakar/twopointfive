// DEPOT AS THE PAGE LOADS IT (src/scene/look.ts): tools/levels/depot.ts's level, box for box, in other clothes — tools/levels/gwotSkin.ts says what may change
// (paint and finish, decals, the paint of a prop) and checks that nothing a round, a charge or a blast reads did. `bun run tools/levels/export.ts depot-gwot`.
// DRAFT (the depot's rework, 11-05): THE WHITE ROOF'S WAY ON A LEVEL THAT IS PLAYED. The maintainer: "rework another level to use something similar to our
// rooftop demo? probably less obvious mirror's edge (so rethink the primary colors a bit)". ITS OWN PLACE: a haulier's freight yard, kept. The ground is pale
// pavers wall to wall and the dark road stops at the wall; the yard wall is cast panels a step under the ground; the huts are painted block, paler, with a
// dado in the firm's colour along the one long face that looks at the camera; two fields of colour and no more (the firm's, and a second for what burns:
// the fuel point, the generating set, the cabin in the trailer park); and what the yard's own people painted for themselves: bays and their numbers, the
// lane a lorry drives from gate to gate, the ways a man walks from door to door along the walls (they are the level's two answers: the stores' way and
// round the back), a letter at each lettered door, a hatched square where nothing may stand, a dock's load limit. No word names a room.
// ONE MECHANISM: the skin below speaks the kit's rows as every look file does, and `of` says which row of the second palette each wears here
// (tools/levels/coastSkin.ts; gwotSkin.ts Skin.of). A palette is a table: `yardOf` is the quiet table with a yard's nine rows changed.
import { build as buildPlain, NOTES as PLAIN_NOTES, DEPOT_ROOMS, DEPOT_PAINT as OWN } from './depot';
import { COAST_PALETTE as C, COAST_MATERIALS, type CoastMat, type GwotMat } from '../../src/scene/gwotPalette';
import { skinLevel, type Skin } from './gwotSkin';
import { COAST_QUIET_OF } from './coastSkin';
import { lookMarks } from './lookMarks';
import { saidAs } from '../../src/scene/stuff';
import { stencilWidth, MARKING_TEXEL, type MarkPal } from '../../src/scene/markings';
import * as MD from '../../src/scene/markingDefaults';
import { WORLD } from '../../src/scene/world';
import type { Level } from '../../src/scene/level';

export const NAME = 'depot-gwot';
export const DRESS_AS = 'depot';
export const MATERIALS = COAST_MATERIALS;
export const NOTES = `${PLAIN_NOTES ?? ''} — AS THE PAGE LOADS IT (a kept haulier's yard, the white roof's way: tools/levels/depot-gwot.ts): the same boxes repainted, decals added; what a man collides with, hides behind or sees through is the plain file's.`;
export const build = buildPlain;
const R = DEPOT_ROOMS;

/** A YARD'S PAINTS: four pale planes (the ground, the yard wall a step under it, the huts' block a step over it, a smooth skim indoors), the huts' floors a
 *  mid value (an inside reads as an inside), two fields, the two line paints (rows of the second palette) */
export interface YardPaints { paver: CoastMat; panel: CoastMat; block: CoastMat; skim: CoastMat; floor: CoastMat; field: CoastMat; second: CoastMat; line: CoastMat; ink: CoastMat }
/** JADE AND ROSE ON WARM LIMESTONE (the one laid): one secondary colour and one earth colour, a value apart, on a stone a little warm */
export const YARD_A: YardPaints = { paver: 'limePaver', panel: 'limePanel', block: 'limeBlock', skim: 'limeSkim', floor: 'limeFloor', field: 'jade', second: 'rose', line: 'lineChalk', ink: 'lineInk' };
/** FIG AND PEAR ON COOL GREY, and PETROL AND OCHRE ON BLUE-WHITE: the two beside it (draft files depot-roof-b / -c) */
export const YARD_B: YardPaints = { paver: 'greyPaver', panel: 'greyPanel', block: 'greyBlock', skim: 'greySkim', floor: 'greyFloor', field: 'fig', second: 'pear', line: 'lineChalk', ink: 'lineInk' };
/** NO ACCENT (the goal's one corner first: a patch that must convince with its planes, edges and marks alone): the fields wear the planes' own rows */
export const YARD_N: YardPaints = { paver: 'limePaver', panel: 'limePanel', block: 'limeBlock', skim: 'limeSkim', floor: 'limeFloor', field: 'limeDeep', second: 'limeDeep', line: 'lineChalk', ink: 'lineInk' };
export const YARD_C: YardPaints = { paver: 'coolPaver', panel: 'coolPanel', block: 'coolBlock', skim: 'coolSkim', floor: 'coolFloor', field: 'petrol', second: 'ochre', line: 'ochre', ink: 'lineInk' };
/** which row of the second palette each of the kit's rows wears in a yard: the quiet table, and what a yard changes in it */
export const yardOf = (Y: YardPaints): Readonly<Record<GwotMat, CoastMat>> => ({ ...COAST_QUIET_OF,
  slab: Y.paver, dust: Y.paver, mudFloor: Y.floor, tWall: Y.panel, block: 'paleGravel', render: Y.block, whitewash: Y.skim, paintCream: Y.skim, mudLight: Y.skim,
  olive: Y.field, paintRust: Y.field, paintGreen: Y.field, genYellow: Y.second, paintBlue: Y.second, drumRed: Y.field,
  oil: 'roadTarmac', dirt: 'roadTarmac', iron: 'graphite', drumBlue: 'graphite', rust: 'graphite', galv: 'brushedSteel', tin: 'brushedSteel' });

export const yardSkin = (Y: YardPaints): Skin => ({
  of: yardOf(Y),
  // WHAT EACH THING WEARS, in the kit's rows (the table above says what each row is here): the hardstanding and the hut floors are the two floors; the
  // lorries and the main gate's leaves are the firm's; the cabin and the generating set are the second field's; the drums are the firm's (they stand on the second field's pad); cast iron is dark steel.
  own: [[OWN.hard, 'slab'], [OWN.raised, 'mudFloor'], [OWN.drab, 'olive'], [OWN.shelter, 'genYellow'], [OWN.set, 'genYellow'], [OWN.oxide, 'drumRed'], [OWN.gate, 'paintRust'], [OWN.iron, 'iron']],
  pick: c => c.name === 'wallDark' ? 'whitewash' : undefined,                          // the wash room's stall partitions
  wall: 'tWall', house: 'render', inner: 'whitewash', ground: 'oil', frameOut: 'paintRust', frameIn: 'paintRust', dado: 'paintRust',
  floor: { carpet: 'mudFloor', stone: 'mudFloor', lino: 'mudFloor' }, noRug: /./, rugs: { crew: 'mat' },
  skirts: false, patches: 0,                                                           // a kept place: no damp foot, no drifts
  metals: ['galv', 'tin'],
  rooms: [{ id: 'duty', bounds: R.duty }],
  dress: (_G, level, T) => {
    const M = lookMarks(T), B = level.boxes, H = WORLD.ceilingY;
    const FIELD = C[Y.field], SECOND = C[Y.second], LINE = C[Y.line], INKP = C[Y.ink];
    const INK: MarkPal = { albedo: INKP, rough: 0.55 }, CHALK: MarkPal = { albedo: C.lineChalk, rough: 0.5 };
    /** a piece of the ground in its own paint, to carry what is painted on it (a box holds four markings: the ground is laid in the pieces that carry its paint) */
    const piece = (x0: number, z0: number, x1: number, z1: number, paint = T.floorAt((x0 + x1) / 2, (z0 + z1) / 2)!) => T.flat((x0 + x1) / 2, (z0 + z1) / 2, x1 - x0, z1 - z0, paint, 0, 0.0116);
    const rectTop = (i: number, x0: number, z0: number, x1: number, z1: number): [number, number, number, number] => [x0 - B[i].c[0], B[i].c[2] - z1, x1 - B[i].c[0], B[i].c[2] - z0];
    /** stencilled text on a piece's top, its middle at x, its foot (the south end: the camera reads a top from the south) at z */
    const stencil = (i: number, text: string, x: number, z: number, pal: MarkPal, scale: number) => T.mark(i, MD.stencilMarks('+y', text, x - stencilWidth(text, scale) * MARKING_TEXEL - B[i].c[0], B[i].c[2] - z, pal, scale));
    /** a figure or a letter on the ground, on a piece of its own */
    const figure = (text: string, x: number, z: number, pal: MarkPal = INK, scale = 3) => { const w = stencilWidth(text, scale) * 2 * MARKING_TEXEL / 2 + 0.1, h = 7 * scale * 2 * MARKING_TEXEL + 0.1; return stencil(piece(x - w, z - h, x + w, z + 0.1), text, x, z, pal, scale); };
    /** a hatched square where nothing may stand, an edge round it */
    const hatch = (x0: number, z0: number, x1: number, z1: number, pal: MarkPal = INK) => { const i = piece(x0, z0, x1, z1); return T.mark(i, MD.hazardMarks('+y', rectTop(i, x0, z0, x1, z1), pal, undefined, 2, 'coarse', 1)); };
    /** a way a man walks: two lines a stride apart through the points (the middle of the way) */
    const walk = (pts: [number, number][], w = 0.9) => { for (const s of [-1, 1]) M.path(pts.map(([x, z], k): [number, number] => { const [ax, az] = pts[Math.max(0, k - 1)], [bx, bz] = pts[Math.min(pts.length - 1, k + 1)], L = Math.hypot(bx - ax, bz - az) || 1; return [x - (bz - az) / L * s * w / 2, z + (bx - ax) / L * s * w / 2]; }), 0.07, LINE); };

    // ---- THE ONE LONG FACE (the north row's yard wall, z 9.5: a quarter of a frame is wall faces, and this is the biggest that looks at the camera): a dado in
    // the firm's colour to 0.9 m, the same blocks under the colour, door to door. And the hut's west face, which the yard's length looks at.
    const DADO = saidAs('gloss block', FIELD, { rough: 0.36 });
    for (const [a, b] of [[4.12, 5.8], [7.0, 11.8], [13.0, 20.8], [22.0, 23.88]] as const) T.band('x', 9.5, a, b, 1, DADO, 0.9);
    T.band('z', 24, 9.62, 15.12, -1, DADO, 0.9);
    // ---- THE HUT'S SOUTH FACE, over the mouth: one whole wall in the second colour (its window is a hole: the colour stops at the sill under it)
    const WALL2 = saidAs('gloss block', SECOND, { rough: 0.4 });
    T.band('x', 15, 23.88, 24.2, 1, WALL2, H); T.band('x', 15, 25.0, 29.62, 1, WALL2, H); T.band('x', 15, 24.2, 25.0, 1, WALL2, 0.95);

    // ---- THE LANE A LORRY DRIVES, gate to gate: a broken line down its middle, an arrow in from each gate, the line to stop at inside the main gate
    M.seg(5.0, 12.6, 5.0, 15.4, 0.2, LINE);
    M.dashed(5.8, 12.9, 18.6, 12.9, 0.1, LINE, 1.2, 0.9); M.dashed(18.6, 12.9, 24.4, 16.7, 0.1, LINE, 1.2, 0.9); M.dashed(24.4, 16.7, 35.2, 17.4, 0.1, LINE, 1.2, 0.9);
    figure('→', 8.0, 13.6, CHALK, 2); figure('←', 32.0, 16.6, CHALK, 2);
    // ---- THE BAYS on the yard's south side (the lorry stands in the first; the second is empty, and an empty bay says there is no cover here), and two for
    // trailers in the east yard. A number at each bay's mouth
    for (const x of [9.6, 15.4, 18.8]) M.seg(x, 14.9, x, 17.4, 0.1, LINE);
    figure('1', 12.5, 14.75); figure('2', 17.1, 14.75);
    for (const x of [30.2, 33.0, 35.75]) M.seg(x, 19.6, x, 23.8, 0.1, LINE);
    figure('3', 31.6, 19.4); figure('4', 34.4, 19.4);
    // ---- THE WAYS A MAN WALKS, door to door along the walls. They are the level's own answers: along the north row; THE STORES' WAY along the south row's
    // wall behind the lorry to the workshop's side door and the store; ROUND THE BACK from the back gate through the gap, up the lane behind the cabin to
    // the generator room's door
    walk([[4.8, 10.25], [23.5, 10.25]]);
    walk([[4.8, 17.87], [23.9, 17.87]], 0.86);
    walk([[35.0, 16.4], [35.0, 6.9], [29.8, 6.9]]);
    // a hatched square before the store's hatch, where nothing may stand (a door's own threshold is worn, and says so itself)
    hatch(19.4, 17.44, 21.4, 18.3);
    // ---- THE LETTERS: A the crew room, B the signals hut, C the store, on the ground at each one's doors (the walls' own are the generator's: depot.ts)
    // (each a stride clear of its door's worn threshold: paint is laid where feet have not taken it off)
    figure('A', 12.4, 12.05); figure('A', 17.6, 7.6);
    figure('B', 27.4, 9.2);
    figure('C', 22.9, 17.0); figure('C', 16.2, 20.3);
    // ---- THE DOCK: a field of the firm's colour against the hut's west wall (a field in the sun before a pale wall: what its colour is thrown onto), an
    // edge to its open sides, its load limit, and the hut's letter where a door would be
    { const dock = piece(21.2, 10.95, 23.75, 13.5, FIELD);
      M.hazard(21.05, 10.95, 21.2, 13.65, INKP, C.lineChalk, 0.3); M.hazard(21.2, 13.5, 23.75, 13.65, INKP, C.lineChalk, 0.3);
      stencil(dock, '3.5T', 22.4, 13.3, CHALK, 1); stencil(dock, 'B', 22.9, 12.4, CHALK, 3); }
    // ---- THE FUEL POINT in the east yard: a pad of the second colour, an edge to it
    piece(26.4, 22.2, 28.8, 23.86, SECOND);
    M.hazard(26.25, 22.05, 28.8, 22.2, INKP, C.lineChalk, 0.3); M.hazard(26.25, 22.2, 26.4, 23.86, INKP, C.lineChalk, 0.3);
    // ---- THE BAY: the inspection pit's edge, round the lorry that stands over it
    M.hazard(10.9, 21.55, 15.9, 21.7, INKP, C.lineChalk); M.hazard(10.9, 21.7, 11.1, 23.7, INKP, C.lineChalk, 0.25); M.hazard(15.7, 21.7, 15.9, 23.7, INKP, C.lineChalk, 0.25);
    // ---- THE SIGNALS HUT: the darker floor round the desk
    { const fa = T.floorAt(27, 12.6); if (fa) M.fill(25.7, 11.7, 28.4, 13.7, T.tone(fa, 0.82)); }
  },
});
export const finish = (level: Level, groups: string[]) => skinLevel(level, groups, yardSkin(YARD_A));
