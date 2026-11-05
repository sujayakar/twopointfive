// THE GWOT PALETTE — the default look's colours and finishes by name (scene/gwotKit.ts builds with them; scene/look.ts is the switch).
// WHAT EACH ONE IS TO A ROUND AND A BLAST IS SAID, NOT READ OFF IT: a row's name says what a box of it does (scene/stuff.ts PAINT_DOES, a word of the table
// of materials), the kit puts that word on the box (Box.does), and the rules read the box. `gwotAs(name, does)` is the same paint, to the eye exactly,
// on a thing that does something else: a mud PARTITION ('plasterboard': a charge cuts it) beside a mud SHELL ('concrete').
// (Until 2026-11-05 the rules read a box's colour, and a stand-in was the paint a hair off, up to 0.0024 a channel, with the name it stood for in the
// offset. A file written then still carries those colours until it is exported again; tools/levels/doesWas.ts reads them, and nothing here does.)
// No imports but a type and the table of what a paint is made of and does (scene/stuff.ts, which imports only the look's list of names): game/ reads this
// without pulling the kit in.
import type { Vec3 } from '../math/vec';
import { namePaints, namePaint, sayPaint, doingAs, doesOfName, type BoxStuff } from './stuff';
import type { DoesWord } from './materials';

/** THE PALETTE (linear albedo), by name — a generator that exports `MATERIALS = GWOT_MATERIALS` has these written into its level file's `materials` */
export const GWOT_PALETTE = {
  // walls
  mud: [0.42, 0.32, 0.21] as Vec3, mudDark: [0.29, 0.22, 0.15] as Vec3, mudLight: [0.56, 0.46, 0.33] as Vec3,
  render: [0.57, 0.53, 0.45] as Vec3, whitewash: [0.72, 0.69, 0.61] as Vec3, block: [0.4, 0.39, 0.37] as Vec3, tWall: [0.46, 0.45, 0.43] as Vec3,
  // paint (doors, gates, dados, trunks)
  paintBlue: [0.06, 0.19, 0.32] as Vec3, paintGreen: [0.08, 0.24, 0.16] as Vec3, paintRust: [0.3, 0.075, 0.04] as Vec3, paintCream: [0.66, 0.6, 0.44] as Vec3,
  // ground — DARKER THAN ANY WALL, on purpose: under the midday sun the renderer's exposure lifts every earth tone into one pale band, and ground at a wall's
  // value loses the plan (and a man in pale cloth) — so the ground sits two stops under the mud walls, just over the asphalt the levels were lit for (0.09);
  // a patch is darker than its ground, never paler, and a stain never under a third of it (oil at 0.05 read as a hole in the floor)
  dust: [0.205, 0.145, 0.085] as Vec3, dirt: [0.135, 0.09, 0.05] as Vec3, gravel: [0.15, 0.135, 0.115] as Vec3, mudFloor: [0.17, 0.115, 0.07] as Vec3,
  slab: [0.3, 0.285, 0.26] as Vec3, damp: [0.075, 0.052, 0.034] as Vec3, oil: [0.11, 0.095, 0.08] as Vec3,
  // cloth
  rugRed: [0.3, 0.052, 0.04] as Vec3, rugNavy: [0.04, 0.06, 0.15] as Vec3, rugCream: [0.6, 0.52, 0.38] as Vec3, rugOchre: [0.5, 0.33, 0.1] as Vec3,
  rugGreen: [0.09, 0.22, 0.13] as Vec3, canvas: [0.33, 0.31, 0.2] as Vec3, blanketGrey: [0.3, 0.3, 0.31] as Vec3, blanketBrown: [0.27, 0.18, 0.12] as Vec3,
  tarpBlue: [0.05, 0.17, 0.48] as Vec3, sack: [0.62, 0.58, 0.48] as Vec3, sandbag: [0.45, 0.39, 0.27] as Vec3, hesco: [0.47, 0.44, 0.36] as Vec3,
  // gear
  olive: [0.15, 0.17, 0.09] as Vec3, coyote: [0.4, 0.31, 0.2] as Vec3, stencil: [0.62, 0.5, 0.1] as Vec3,
  // metal, plastic, rubber
  rust: [0.28, 0.12, 0.06] as Vec3, iron: [0.098, 0.1, 0.112] as Vec3, galv: [0.5, 0.52, 0.53] as Vec3, tin: [0.56, 0.55, 0.52] as Vec3,
  drumBlue: [0.06, 0.15, 0.33] as Vec3, drumRed: [0.4, 0.07, 0.05] as Vec3, genYellow: [0.6, 0.42, 0.04] as Vec3,
  plasticWhite: [0.7, 0.7, 0.66] as Vec3, plasticGreen: [0.08, 0.28, 0.14] as Vec3, plasticBlue: [0.03, 0.12, 0.36] as Vec3, plasticBlack: [0.045, 0.045, 0.05] as Vec3,
  jerryYellow: [0.62, 0.46, 0.05] as Vec3, rubber: [0.03, 0.03, 0.033] as Vec3,
  // wood
  poplar: [0.47, 0.36, 0.22] as Vec3, woodGrey: [0.33, 0.29, 0.23] as Vec3, ply: [0.55, 0.43, 0.27] as Vec3,
  // vehicles
  truckWhite: [0.68, 0.68, 0.65] as Vec3, truckTan: [0.5, 0.42, 0.29] as Vec3, truckRed: [0.42, 0.06, 0.05] as Vec3, glassDark: [0.05, 0.06, 0.08] as Vec3,
  // produce, washing, the odd accent
  prodOrange: [0.7, 0.3, 0.04] as Vec3, prodGreen: [0.22, 0.4, 0.1] as Vec3, prodRed: [0.5, 0.07, 0.05] as Vec3, prodYellow: [0.7, 0.55, 0.1] as Vec3,
  straw: [0.51, 0.43, 0.22] as Vec3, scrub: [0.24, 0.26, 0.13] as Vec3,
  // (added later — a palette's ORDER is in nothing, its names are in the level files)
  rugMadder: [0.1, 0.03, 0.035] as Vec3,
};
export type GwotMat = keyof typeof GWOT_PALETTE;
namePaints(GWOT_PALETTE);   // (a box made with a row of this palette is made of what the row's NAME says, and does what it says: scene/stuff.ts. No finish is named here: whoever lays a GWOT paint writes its finish, as ever)
/** THE FINISHES (GGX roughness, bare metal), by the palette's names; absent = matte cloth / earth (0.95). Nothing here is polished: this look has no mirror. */
export const GWOT_FINISH: Readonly<Partial<Record<GwotMat, { rough: number; metal?: boolean }>>> = {
  mud: { rough: 0.97 }, mudDark: { rough: 0.97 }, mudLight: { rough: 0.95 }, render: { rough: 0.9 }, whitewash: { rough: 0.88 }, block: { rough: 0.92 }, tWall: { rough: 0.9 },
  paintBlue: { rough: 0.6 }, paintGreen: { rough: 0.6 }, paintRust: { rough: 0.65 }, paintCream: { rough: 0.7 },
  dust: { rough: 1 }, dirt: { rough: 1 }, gravel: { rough: 1 }, mudFloor: { rough: 0.97 }, slab: { rough: 0.9 }, damp: { rough: 0.7 }, oil: { rough: 0.35 },
  olive: { rough: 0.75 }, coyote: { rough: 0.85 }, stencil: { rough: 0.7 },
  rust: { rough: 0.85 }, iron: { rough: 0.5, metal: true }, galv: { rough: 0.38, metal: true }, tin: { rough: 0.42, metal: true },
  drumBlue: { rough: 0.5 }, drumRed: { rough: 0.5 }, genYellow: { rough: 0.5 },
  plasticWhite: { rough: 0.45 }, plasticGreen: { rough: 0.45 }, plasticBlue: { rough: 0.45 }, plasticBlack: { rough: 0.5 }, jerryYellow: { rough: 0.45 }, rubber: { rough: 0.85 },
  poplar: { rough: 0.8 }, woodGrey: { rough: 0.85 }, ply: { rough: 0.75 },
  truckWhite: { rough: 0.35 }, truckTan: { rough: 0.4 }, truckRed: { rough: 0.35 }, glassDark: { rough: 0.1 },
};
export const ROUGH_CLOTH = 0.95;
/** THE SNOW PAINTS — lying snow as paint, for a level built in winter (tools/levels/pass.ts): `snow` fresh, a blue-tinted near-white; `snowPacked` trodden / driven snow, darker and greyer; `snowOld` slush and old snow, the dirty
 *  ring round a cleared patch and the foot of a wall where the sun never comes. Each step down is over a third of the one it lies on (STYLE.md: a patch is
 *  darker than its ground, never under a third of it), and earth (`dust`) on `snowOld` holds the same rule.
 *  A TABLE OF ITS OWN, not rows of GWOT_PALETTE: that table is written whole into the `materials` of every level built on the kit (GWOT_MATERIALS), and a new
 *  row there makes every one of those files stale. A snow level's generator exports `MATERIALS = { ...GWOT_MATERIALS, ...SNOW_MATERIALS }`.
 *  TO A ROUND AND A BLAST snow is the ground (the rows 'snow', 'packed snow' and 'old snow' of the table of materials: the tarmac's columns, and which
 *  snow it is): a round knocks a puff of its own colour out of it, a charge cuts nothing of it, and it is never steel, glass or wood. A box is lying
 *  snow because it SAYS so (Box.does), whatever white it is painted. */
export const SNOW_PALETTE = {
  snow: [0.66, 0.71, 0.78] as Vec3, snowPacked: [0.43, 0.47, 0.53] as Vec3, snowOld: [0.27, 0.3, 0.34] as Vec3,
};
export type SnowMat = keyof typeof SNOW_PALETTE;
namePaints(SNOW_PALETTE);
export const SNOW_FINISH: Readonly<Record<SnowMat, { rough: number }>> = { snow: { rough: 1 }, snowPacked: { rough: 0.9 }, snowOld: { rough: 0.95 } };
export const SNOW_MATERIALS: Readonly<Record<string, { albedo: Vec3; rough: number }>> = Object.fromEntries(
  (Object.keys(SNOW_PALETTE) as SnowMat[]).map(n => [n, { albedo: SNOW_PALETTE[n], rough: SNOW_FINISH[n].rough }]));
/** the palette + finishes as the exporter's `materials` rows (tools/levels/export.ts: a generator's `MATERIALS` export) */
export const GWOT_MATERIALS: Readonly<Record<string, { albedo: Vec3; rough: number; metal?: boolean }>> = Object.fromEntries(
  (Object.keys(GWOT_PALETTE) as GwotMat[]).map(n => [n, { albedo: GWOT_PALETTE[n], rough: GWOT_FINISH[n]?.rough ?? ROUGH_CLOTH, ...(GWOT_FINISH[n]?.metal ? { metal: true } : {}) }]));
/** WHICH FOOTSTEP SURFACE a ground material is (the six the game has — a level's `surfaces` block is separate data; scene/levelData.ts SurfacesData) */
export const GWOT_SURFACE: Readonly<Partial<Record<GwotMat, 'carpet' | 'lino' | 'stone' | 'tile' | 'asphalt' | 'grass'>>> = {
  dust: 'asphalt', dirt: 'asphalt', gravel: 'asphalt', mudFloor: 'stone', slab: 'stone', rugRed: 'carpet', rugNavy: 'carpet', rugCream: 'carpet', rugMadder: 'carpet',
};

/** A ROW'S DARKER ROW: the same stuff a shade down, where the palette holds one (the three mud walls are one earth in three tones). What a piece that
 *  lays one paint in two tones reaches for (gwotKit.ts rubble: a scatter's darker stones); a row with none here is laid in its one tone. Data, like the
 *  palette: a family joins by a row here, never by a case in a piece. */
export const GWOT_DARKER: Readonly<Partial<Record<GwotMat, GwotMat>>> = { mudLight: 'mud', mud: 'mudDark' };
/** THE PAINT `m` ON A THING THAT DOES `does` (a row of the table of materials: scene/materials.ts): the palette's own row where that is what the row's
 *  name says anyway, else the same numbers as a paint of their own, known by the row's name and by what it does. Handed to a kit's add() it makes a box
 *  drawn exactly as the row and doing the other thing; in a level file the box wears the row and says its own `does`. */
export function gwotAs(m: GwotMat, does: DoesWord): Vec3 { return doingAs(GWOT_PALETTE[m], does); }
/** THE PAINTS A KIT IS HANDED: a colour and a finish for every row the kit knows by name. The kit's own are the palette itself (GWOT_PAINTS); a level in
 *  other clothes hands it another set (paintsOf below), and no piece knows: a piece is written once, in the kit's row names. */
export interface KitPaints { readonly paints: Readonly<Record<GwotMat, Vec3>>; readonly finish: Readonly<Partial<Record<GwotMat, { rough: number; metal?: boolean }>>> }
export const GWOT_PAINTS: KitPaints = { paints: GWOT_PALETTE, finish: GWOT_FINISH };
/** THE KIT'S ROWS IN ANOTHER PALETTE'S PAINT. `of` says which row of `palette` each row of the kit wears (data: one line a row, and the compiler holds it
 *  to every row of the kit). What comes back is, for each kit row, that palette row's numbers as a paint of its own: KNOWN BY THE PALETTE ROW'S NAME (so a
 *  box of it is drawn as what that name is made of, or as `words` says the row is), with that row's finish, and DOING WHAT THE KIT'S ROW DOES (scene/stuff.ts
 *  PAINT_DOES; scene/materials.ts): a drum is steel to a round in coral as it was in red, and a partition a charge cuts is one in any plaster. Many kit
 *  rows may wear one palette row; each is its own paint, since they do different things. Hand the answer to a kit through its host (gwotKit.ts
 *  GwotHost.paints) or to a look file's skin (tools/levels/gwotSkin.ts Skin.of). In a level file a box of it wears the palette's row by its numbers. */
export function paintsOf(palette: Readonly<Record<string, Vec3>>, finish: Readonly<Record<string, { rough: number; metal?: boolean } | undefined>>, of: Readonly<Record<GwotMat, string>>, words: Readonly<Record<string, BoxStuff>> = {}): KitPaints {
  const paints = {} as Record<GwotMat, Vec3>, fin: Partial<Record<GwotMat, { rough: number; metal?: boolean }>> = {};
  const own = <T>(o: Readonly<Record<string, T>>, k: string): T | undefined => Object.hasOwn(o, k) ? o[k] : undefined;   // (a row may be named anything: 'constructor' is a name, not a member)
  for (const m of Object.keys(GWOT_PALETTE) as GwotMat[]) { const name = own(of as Record<string, string>, m), row = name === undefined ? undefined : own(palette, name);
    if (name === undefined || !row) throw new Error(`paintsOf: the kit's row '${m}' wears ${name === undefined ? 'nothing' : `'${name}', which is no row of the palette`}`);
    const a: Vec3 = [row[0], row[1], row[2]], f = own(finish, name), w = own(words, name);
    if (w !== undefined) sayPaint(a, name, { stuff: w, does: doesOfName(m), ...(f ?? {}) }); else namePaint(a, name, f, doesOfName(m));
    paints[m] = a; if (f) fin[m] = f; }
  return { paints, finish: fin };
}
/** which paint of the look's a colour is, by its numbers: a row of the palette or of the snow paints, or undefined. `eps` > 0 = to within that much a
 *  channel (a file written before 2026-11-05 carries stand-ins up to 0.0024 off their rows). THE PICTURE'S AND THE LEVEL TOOLS' QUESTION (does this level
 *  wear the look, is this box already in the look's paint): no rule of the game asks a colour anything. */
export function lookPaintOf(albedo: readonly number[], eps = 0): GwotMat | SnowMat | undefined {
  const is = (r: Vec3) => Math.abs(albedo[0] - r[0]) <= eps && Math.abs(albedo[1] - r[1]) <= eps && Math.abs(albedo[2] - r[2]) <= eps;
  for (const m in GWOT_PALETTE) if (is(GWOT_PALETTE[m as GwotMat])) return m as GwotMat;
  for (const m in SNOW_PALETTE) if (is(SNOW_PALETTE[m as SnowMat])) return m as SnowMat;
  return undefined;
}
