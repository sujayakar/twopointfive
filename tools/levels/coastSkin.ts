// THE COAST SKIN: the kit's pieces in other clothes, AS THEY ARE LAID. A level that wears it builds with the look's kit over `coastHost(host)`
// (src/scene/gwotKit.ts gwotKit: `gwotKit(coastHost(gwotOfOffice(K)))`), and every box, trim, marking and prop part the kit hands the host in one of
// ITS OWN rows (scene/gwotPalette.ts GWOT_PALETTE) is laid in the coast row `COAST_OF` gives that row (COAST_PALETTE). No kit function knows: a piece is
// written once, in the kit's rows, and has two skins.
// WHAT STAYS, for each thing laid:
//   · WHAT IT IS TO A ROUND, A CHARGE AND A BLAST: the coast paint stands for whatever the kit's paint stood for (gwotPalette.ts coastAs, as gwotAs: the
//     colour itself where that is what the coast row is anyway, else a hair off). A drum is steel in coral as it was in red.
//   · ITS TONE: a kit paint laid darker in one piece (worn ground, a rib a shade down) is the coast row at the same share, a channel.
//   · A WORD IT SAID ITSELF: a thing that said another stuff than its paint's row says (a vehicle's body: 'carpaint') says it still; everything else says
//     what its coast row says (scene/stuff.ts COAST_STUFF), which is the point of a skin: a mud wall becomes render.
//   · A FINISH IT WROTE ITSELF: a roughness or a bare-metal flag that was not its row's own (a sooty pot) is kept; the row's own becomes the coast row's.
// WHAT IS NOT TOUCHED: a paint that is no row of the kit's palette (a coast row the generator laid itself, a snow paint, numbers somebody wrote), lights,
// and the shape of everything.
// THE TABLE IS DATA: which coast row a kit row wears is a choice of look, one line a row, and the compiler holds it to every row of the kit's palette.
//   tools/qa/coast-check.ts holds the rest (every key the coast rows take in the table a round reads was free; a skinned piece is to a round what the
//   plain one is; no skinned thing says a stuff of the old setting).
import type { Vec3 } from '../../src/math/vec';
import type { Box } from '../../src/scene/boxes';
import type { GwotHost } from '../../src/scene/gwotKit';
import type { ShapePiece } from '../../src/scene/shapes';
import type { MarkingDef } from '../../src/scene/markings';
import { GWOT_PALETTE, GWOT_FINISH, ROUGH_CLOTH, COAST_PALETTE, COAST_FINISH, coastAs, gwotSimOf, type GwotMat, type CoastMat } from '../../src/scene/gwotPalette';
import { paintOf, saidAs, stuffOfName, tonedAs, type BoxStuff } from '../../src/scene/stuff';

/** WHICH COAST ROW EACH ROW OF THE KIT WEARS */
export const COAST_OF: Readonly<Record<GwotMat, CoastMat>> = {
  // walls: mud is render, in the house's white, a sand and a terracotta; the old render is fair concrete, whitewash a smooth skim
  mud: 'stuccoWhite', mudDark: 'stuccoSand', mudLight: 'skimWhite', render: 'fairConcrete', whitewash: 'skimWhite', block: 'paleBlock', tWall: 'castPanel',
  paintBlue: 'paintTeal', paintGreen: 'paintNavy', paintRust: 'paintCoral', paintCream: 'skimWhite',
  // ground: the walked earth of a yard is paving, a slab a pale slab; bare dirt is a bed's soil, damp the same watered; an oil stain is the road's black
  dust: 'paverWarm', dirt: 'soil', gravel: 'paleGravel', mudFloor: 'paverWarm', slab: 'paleSlab', damp: 'soil', oil: 'roadTarmac',
  // cloth: rugs are towels and canvas; a tarpaulin and a blanket are awning canvas
  rugRed: 'towelStripe', rugNavy: 'canvasBlue', rugCream: 'canvasWhite', rugOchre: 'towelStripe', rugGreen: 'canvasBlue', rugMadder: 'towelStripe',
  canvas: 'canvasWhite', blanketGrey: 'canvasWhite', blanketBrown: 'towelStripe', tarpBlue: 'canvasBlue', sack: 'canvasWhite', sandbag: 'canvasWhite', hesco: 'castPanel',
  // gear: the army's green is the sea's colour, its tan a sand
  olive: 'paintTeal', coyote: 'canvasWhite', stencil: 'paintSun',
  // metal, plastic, rubber
  rust: 'paintCoral', iron: 'darkSteel', galv: 'brushedSteel', tin: 'brushedSteel', drumBlue: 'paintNavy', drumRed: 'paintCoral', genYellow: 'paintSun',
  plasticWhite: 'plasticPale', plasticGreen: 'binGreen', plasticBlue: 'paintNavy', plasticBlack: 'plasticDark', jerryYellow: 'paintSun', rubber: 'tyreRubber',
  // wood
  poplar: 'deck', woodGrey: 'deckGrey', ply: 'deck',
  // vehicles
  truckWhite: 'enamelWhite', truckTan: 'paintTeal', truckRed: 'paintCoral', glassDark: 'plasticDark',
  // produce and plants
  prodOrange: 'paintCoral', prodGreen: 'foliage', prodRed: 'paintCoral', prodYellow: 'paintSun', straw: 'sand', scrub: 'foliage',
};

/** THE QUIET WAY TO WEAR IT (a proposal beside the first, 11-05; the maintainer: "something that feels mirrors edge like", then "a minimalist hotel"): the
 *  same skin with FEWER rows, after the white roof study (tools/levels/white-roof.ts): the ground is a white paver, every wall cast panels a step under it,
 *  their copings and what is plastered a smooth skim, timber and plastic and painted metal white. ONE kit row wears each colour, so a level puts colour only
 *  where it asks for it by name: `paintRust` is the warm one, `paintBlue` the water's. Only what differs from COAST_OF is written */
export const COAST_QUIET_OF: Readonly<Record<GwotMat, CoastMat>> = { ...COAST_OF,
  mud: 'castPanel', mudDark: 'castPanel', render: 'castPanel', block: 'castPanel', tWall: 'castPanel', mudLight: 'skimWhite', whitewash: 'skimWhite', paintCream: 'skimWhite',
  paintBlue: 'accentBlue', paintGreen: 'enamelWhite', paintRust: 'accentRed',
  dust: 'paverWhite', mudFloor: 'paverWhite', slab: 'paverWhite', gravel: 'paverWhite', dirt: 'paverWhite', damp: 'paverWhite', oil: 'paverWhite',
  rugRed: 'canvasWhite', rugNavy: 'canvasWhite', rugOchre: 'canvasWhite', rugGreen: 'canvasWhite', rugMadder: 'canvasWhite', blanketBrown: 'canvasWhite', tarpBlue: 'canvasWhite',
  olive: 'enamelWhite', stencil: 'plasticDark',
  rust: 'enamelWhite', drumBlue: 'enamelWhite', drumRed: 'enamelWhite', genYellow: 'enamelWhite',
  plasticGreen: 'plasticPale', plasticBlue: 'plasticPale', jerryYellow: 'enamelWhite',
  poplar: 'enamelWhite', woodGrey: 'enamelWhite', ply: 'enamelWhite',
  truckTan: 'enamelWhite', truckRed: 'enamelWhite',
  prodOrange: 'enamelWhite', prodRed: 'enamelWhite', prodYellow: 'enamelWhite', straw: 'canvasWhite', prodGreen: 'cypress', scrub: 'cypress',
};

/** tools/qa/coast-check.ts's red path: every coast row laid as itself, standing for nothing */
const BREAK = process.env.COAST_BREAK === '1';
const KIT_ROWS = new Map<string, GwotMat>((Object.keys(GWOT_PALETTE) as GwotMat[]).map(m => [GWOT_PALETTE[m].join(','), m]));
const r4 = (v: number) => Math.round(v * 1e4) / 1e4;
/** a kit paint in its coast clothes: the paint, the coast row it is, and the kit row it was; nothing for a paint that is no row of the kit's */
type CoastTable = Readonly<Record<GwotMat, CoastMat>>;
function dressed(of: CoastTable, a: readonly number[] | undefined): { albedo: Vec3; coast: CoastMat; kit: GwotMat; said?: BoxStuff } | null {
  if (!a) return null;
  const p = paintOf(a), kit = p && Object.hasOwn(GWOT_PALETTE, p.name) ? p.name as GwotMat : KIT_ROWS.get(a.join(','));   // (the row handed over, a stand-in or a tone of it; or, where a copy was handed, its exact numbers)
  if (!kit) return null;
  const coast = of[kit], base = GWOT_PALETTE[kit], sim = gwotSimOf(a as Vec3);
  // a row or one of its stand-ins is the coast row standing for the same thing; any other numbers under the row's name are a tone of it
  let albedo: Vec3 = sim !== undefined ? (BREAK ? COAST_PALETTE[coast] : coastAs(coast, sim)) : tonedAs(COAST_PALETTE[coast], COAST_PALETTE[coast].map((v, c) => r4(v * (base[c] > 0 ? a[c] / base[c] : 1))) as Vec3);
  // what the thing said for itself, where that was not its row's word, it says still, with the coast row's finish
  const rowWord = stuffOfName(kit), said = p?.stuff !== undefined && p.stuff !== rowWord ? p.stuff : undefined;
  if (said) albedo = saidAs(said, albedo, COAST_FINISH[coast]);
  return { albedo, coast, kit, ...(said ? { said } : {}) };
}
/** the finish of a thing in its coast clothes: the coast row's where it wore its kit row's own, its own where it had written another */
function finish(d: { coast: CoastMat; kit: GwotMat }, rough: number | undefined, metal: boolean | undefined): { rough?: number; metal?: boolean } {
  const was = GWOT_FINISH[d.kit], now = COAST_FINISH[d.coast], own = rough !== undefined && rough !== (was?.rough ?? ROUGH_CLOTH);
  if (own) return { rough, ...(metal ? { metal: true } : {}) };
  return { ...(rough !== undefined ? { rough: now.rough } : {}), ...(now.metal ? { metal: true } : {}) };
}
/** the word a thing carries in its coast clothes, where it carries one itself (a shape's trim, a box that said its own): what it said, where that was not
 *  its kit row's word; else its coast row's */
const wordOf = (d: { coast: CoastMat; kit: GwotMat; said?: BoxStuff }, explicit: BoxStuff | undefined): { stuff?: BoxStuff } => {
  const own = explicit !== undefined && explicit !== stuffOfName(d.kit) ? explicit : d.said, row = stuffOfName(d.coast);
  return own !== undefined ? { stuff: own } : row !== undefined && row !== 'plain' && row !== 'waits' ? { stuff: row as BoxStuff } : {};
};
const trim = (of: CoastTable) => (t: ShapePiece): ShapePiece => { const d = dressed(of, t.albedo); if (!d) return t; const { stuff, metal, rough, ...rest } = t; return { ...rest, albedo: d.albedo, ...finish(d, rough, metal), ...wordOf(d, stuff) }; };
const marks = (of: CoastTable, ms: MarkingDef[] | undefined): MarkingDef[] | undefined => ms?.map(m => ({ ...m, pal: m.pal.map(e => { const d = dressed(of, e.albedo); return d ? { ...e, albedo: d.albedo, ...(e.rough !== undefined ? { rough: finish(d, e.rough, false).rough } : {}) } : e; }) }));
/** one box as the kit handed it, in its coast clothes (a new object: the kit's own are left as they are) */
function box<T extends Partial<Box>>(of: CoastTable, b: T): T {
  const d = dressed(of, b.albedo), out: T = { ...b };
  if (d) { out.albedo = d.albedo; const f = finish(d, b.rough, b.metal); if (f.rough !== undefined) out.rough = f.rough; if (f.metal) out.metal = true; else delete out.metal; delete out.stuff; if (b.stuff !== undefined && b.stuff !== stuffOfName(d.kit)) out.stuff = b.stuff; }   // (a word of its own it keeps; its paint's the host writes from the coast row)
  if (b.shape?.with) out.shape = { ...b.shape, with: b.shape.with.map(trim(of)) };
  if (b.marks) out.marks = marks(of, b.marks);
  return out;
}

/** A HOST THAT LAYS THE KIT'S PIECES IN THE COAST SKIN: everything the kit hands it goes on to `host` in its coast clothes. `of`: which coast row each row
 *  of the kit wears (COAST_OF; COAST_QUIET_OF is the same skin with fewer colours and quieter surfaces) */
export function coastHost(host: GwotHost, of: CoastTable = COAST_OF): GwotHost {
  /** a wall: its paint, and the finish its builder wrote beside it */
  const wall = (albedo: Vec3, extra: Partial<Box> | undefined): [Vec3, Partial<Box> | undefined] => { const d = dressed(of, albedo); if (!d) return [albedo, extra ? box(of, extra) : extra];
    const ex = extra ? box(of, extra) : extra, f = extra ? finish(d, extra.rough, extra.metal) : {}; if (ex && f.rough !== undefined) ex.rough = f.rough; if (ex && f.metal) ex.metal = true; return [d.albedo, ex]; };
  return {
    add: b => host.add(box(of, b)),
    prop: (kind, x, z, yaw, mass, parts, half) => host.prop(kind, x, z, yaw, mass, parts.map(p => { const d = dressed(of, p.albedo); return d ? { ...p, albedo: d.albedo } : p; }), half),
    wallX: (z, x0, x1, t, albedo, openings, extra, height) => { const [a, ex] = wall(albedo, extra); return host.wallX(z, x0, x1, t, a, openings, ex, height); },
    wallZ: (x, z0, z1, t, albedo, openings, extra, height) => { const [a, ex] = wall(albedo, extra); return host.wallZ(x, z0, z1, t, a, openings, ex, height); },
    lights: host.lights, switchables: host.switchables, marks: host.marks, texel: host.texel,
  };
}
