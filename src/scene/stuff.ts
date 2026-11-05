// WHAT A THING IS MADE OF IS SAID, NEVER READ OFF ITS COLOUR.
// Three places say it, and each is data somebody wrote:
//   · a PAINT'S NAME. A kit's palette names its paints ('wood', 'carpet', 'tWall') and this table says what each name is made of: one of the look's materials
//     (render/lookTex.ts LOOK_NAMES); 'plain', meant to be drawn as a flat colour (laminate, a dark screen); 'waits', a stuff the look has no
//     material for yet (card, foliage; drawn plain. A lawn's paint still says it, though the look has had 'grass' since 11-05: see asphalt below); or one of the two the look draws plain and a round's hit reads, 'glass' and 'rubber' (BOX_STUFFS below). Where one name is more than one thing, the piece that knows better says its own word;
//   · a level file's MATERIAL ROW (levelData.ts MaterialData.stuff), written by the exporter from this table: the file says it, and the loader asks no table;
//   · a BOX or a prop's part (Box.stuff, a file's `stuff` on a box), where it is another thing than its paint's row says, or wears a paint written as numbers.
// A box takes its own word, else its row's, when the file is loaded (levelLoad.ts), and a kit's add() does the same for a box it makes with a named paint
// (paintOf below). The look draws what Box.stuff says and nothing else (render/gbuffer.ts lookWordOf): a box that says nothing is plain. NOTHING ANYWHERE
// ASKS A COLOUR WHAT IT IS MADE OF. (Until 11-04 the look matched a box's colour against the 2010s palette, exactly and darkened in one piece, and the kits
// found a paint's finish by its numbers: a wall repainted a slightly different grey was silently another material. tools/levels/stuffWas.ts keeps that rule,
// frozen, for one job: bringing a level file written before forward, each box saying what it was drawn as.)
// A NAME NEVER CHANGES A COLOUR. The simulation reads colours (a round's dust by its paint, a sheet of paper by how pale it is): this table gives a paint a
// material and leaves its numbers alone. Nothing in the simulation reads this table.
import type { GwotMat, CoastMat } from './gwotPalette';
import { LOOK_NAMES, lookStuffOf } from '../render/lookTex';   // (a module of arithmetic with no imports of its own: the look's list of materials is the one list)

export type Stuff = typeof LOOK_NAMES[number] | 'plain' | 'waits' | 'glass' | 'rubber';

/** the 2010s palette (scene/gwotPalette.ts GWOT_PALETTE): every row says what it is made of (the compiler holds the two lists together) */
const PALETTE_STUFF = {
  mud: 'mud wall', mudDark: 'mud wall', mudLight: 'mud wall', render: 'plaster', whitewash: 'plaster', paintCream: 'plaster', block: 'block', tWall: 'block',
  dust: 'earth', dirt: 'earth', mudFloor: 'earth', damp: 'earth', gravel: 'gravel', slab: 'slab', poplar: 'wood', woodGrey: 'wood', ply: 'wood', galv: 'sheet', tin: 'sheet',
  sack: 'cloth', sandbag: 'cloth', hesco: 'cloth', canvas: 'cloth', tarpBlue: 'cloth', blanketGrey: 'cloth', blanketBrown: 'cloth', rugRed: 'cloth', rugNavy: 'cloth', rugCream: 'cloth', rugOchre: 'cloth', rugGreen: 'cloth', rugMadder: 'cloth', coyote: 'cloth',
  rust: 'metal', iron: 'metal',
  paintBlue: 'paint', paintGreen: 'paint', paintRust: 'paint', drumBlue: 'paint', drumRed: 'paint', genYellow: 'paint', truckWhite: 'paint', truckTan: 'paint', truckRed: 'paint', olive: 'paint',   // (the truck enamels are a cot, a locker, a floor line and a sign as well as a vehicle: a vehicle's body says 'carpaint' itself — gwotKit.ts coat, carShapes.ts carSays)
  rubber: 'rubber', glassDark: 'glass',   // (a tyre and a mat; a vehicle's window, a counter's top, a solar panel, a tube: SOLID glass, nothing modelled behind it — not a pane, which says so with its flag)
  oil: 'plain', stencil: 'plain', plasticWhite: 'plastic', plasticGreen: 'plastic', plasticBlue: 'plastic', plasticBlack: 'plastic', jerryYellow: 'plain',
  prodOrange: 'plain', prodGreen: 'plain', prodRed: 'plain', prodYellow: 'plain', straw: 'waits', scrub: 'waits',
} as const satisfies Record<GwotMat, Stuff>;

/** the coast skin's palette (scene/gwotPalette.ts COAST_PALETTE): every row a stuff the look HAS. Two wait for one it does not have yet and say the
 *  nearest honest stuff meanwhile: `sand` waits for sand (earth), `roofTile` for roof tile (sheet). Fired clay (`terracotta`) is drawn as rough render. */
const COAST_STUFF = {
  stuccoWhite: 'stucco', stuccoSand: 'stucco', terracotta: 'stucco', skimWhite: 'skim', fairConcrete: 'board concrete', castPanel: 'cast panel', paleBlock: 'block',
  paverWarm: 'paver', paverGrey: 'paver', roadTarmac: 'tarmac', paleSlab: 'slab', paleGravel: 'gravel', sand: 'earth', soil: 'earth', lawn: 'grass',
  deck: 'wood', deckGrey: 'wood', roofTile: 'sheet',
  paintTeal: 'paint', paintCoral: 'paint', paintSun: 'paint', paintNavy: 'paint', enamelWhite: 'enamel',
  brushedSteel: 'brushed', darkSteel: 'steel', binGreen: 'plastic', plasticPale: 'plastic', plasticDark: 'plastic',
  tyreRubber: 'rubber', canvasWhite: 'cloth', canvasBlue: 'cloth', towelStripe: 'cloth', foliage: 'waits',
  stonePale: 'paver', stoneShade: 'skim', stoneBand: 'skim', dryEarth: 'earth', timber: 'wood', bronze: 'steel', cypress: 'waits', sage: 'waits', poolDark: 'enamel', clay: 'skim', saffron: 'cloth',
  limestone: 'skim', cream: 'skim', rubble: 'block', cedar: 'veneer', poolPale: 'enamel', stoneSlab: 'skim',
} as const satisfies Record<CoastMat, Stuff>;

export const PAINT_STUFF: Readonly<Record<string, Stuff>> = {
  ...PALETTE_STUFF,
  ...COAST_STUFF,
  // ---- the office's paints (scene/officeGen.ts OFFICE_PALETTE: the names the simulation knows a paint by, game/particleFx.ts MAT_SURF)
  wood: 'wood', woodDark: 'wood',   // (planks: the name is a bench, a crate and a pallet as well as a desk. A desk's smooth board is 'veneer', and the desk says so itself)
  carpet: 'cloth', carpetBlue: 'cloth', carpetRed: 'cloth', chair: 'fabric',   // (a carpet is the coarse weave; a seat's cloth the fine one)
  wall: 'plaster', ceiling: 'plaster', wallDark: 'block', concrete: 'block', stone: 'slab', lino: 'slab',
  black: 'plain', white: 'plain', screenOff: 'plain', glass: 'plain', glassExt: 'plain',
  cardboard: 'waits', plant: 'waits', grass: 'waits',
  metal: 'steel',   // (an office's steel is CLEAN painted steel: shelving, desk legs, lamp posts, bins. The look's 'metal' is bare iron gone to rust and its 'paint' is chipped)
  // ---- ONE NAME, MORE THAN ONE THING. The kits have used these six paints for two things each (a level file's writer names a box by the row that has its
  // numbers: levelLoad.ts levelToData), so the name cannot say what its boxes are and a PIECE that knows better says its own word: a cubicle partition says
  // fabric. NOBODY CHOSE THE WORDS HERE: each is what the look made of the paint's COLOUR until 11-04, kept so that no box moved the day the colour rule went.
  partRed: 'plain', partBlue: 'plain',   // (cubicle partitions, which say fabric themselves; and a car's body, lived things)
  partTan: 'cloth',     // (a partition; and a hut's stucco walls, drawn as cloth because the tan is a cream rug's, darkened)
  red: 'plain',         // (an extinguisher's enamel; and tape, a ball, a mast's lamps)
  asphalt: 'plain',     // (the car park. The look has had 'tarmac' and 'grass' since 11-05 (render/lookTex.ts); this row and grass's above still say what they did because a level's FILE carries its own word a material, so they move with the export of the levels that are dressed in them: docs/agents/render.md, TARMAC AND GRASS ARE STUFFS. Until 11-04 its grey was drawn as bare iron, iron's grey darkened; the lived kit's iron fittings, in the same numbers, say metal themselves: livedPieces.ts INK.iron)
  glassFrame: 'plain',  // (a window's frame; and every outside door's leaf, painted from the same numbers as the game runs)
  // ---- the snow kit's (the look lays its own snow over a thing; the paint under it stays plain)
  snow: 'plain', snowOld: 'plain', snowPacked: 'plain',
};
/** what the table says of a paint name: undefined for a name it has no row for (never a method of every object: a level may name a paint `constructor`) */
export const stuffOfName = (name: string): Stuff | undefined => Object.hasOwn(PAINT_STUFF, name) ? PAINT_STUFF[name] : undefined;
/** what a level file's material ROW says of itself (levelData.ts MaterialData.stuff), by the row's name: the table's word, 'plain' and 'waits' written
 *  out; nothing for a name the table does not know. Spread into the row the exporter writes. */
export const rowWord = (name: string): { stuff?: Stuff } => { const w = stuffOfName(name); return w !== undefined ? { stuff: w } : {}; };
/** WHAT A BOX MAY SAY IT IS MADE OF (Box.stuff, a level file's `stuff` on a box or a prop's part): a material of the look, plain, waits, or one of TWO THINGS THE LOOK HAS NO
 *  MATERIAL FOR and draws plain, by the box's finish, but which the picture of a round's hit reads (game/particleFx.ts HITSTUFF): 'glass' — a solid thing of
 *  glass with nothing modelled behind it (a car's window), which is NOT a pane (a sheet a round shatters says so with its flag, BoxFlag.Glass: the pane
 *  engine's) — and 'rubber' (a tyre: dark, and by its colour alone it would be taken for metal). A material is a
 *  name the look has a NUMBER for (lookTex.ts lookStuffOf): the stone and the brick under a mud wall are layers the mud wall shows, and no box is made of them. */
export const BOX_STUFFS: readonly string[] = [...LOOK_NAMES.filter(n => lookStuffOf(n) > 0), 'plain', 'waits', 'glass', 'rubber'];
/** what a box may say: any stuff the table of names may, but the two layers under a mud wall (no stuff: BOX_STUFFS) */
export type BoxStuff = Exclude<Stuff, 'field stone' | 'mud brick'>;
export const boxStuffOk = (v: unknown): v is BoxStuff => typeof v === 'string' && BOX_STUFFS.includes(v);
/** A WORD BELONGS TO THE PAINT IT WAS SAID WITH. A piece says what it is made of for its OWN paints (`own`) and no other: handed another paint (a desk's
 *  shape as a steel bench, a white table) it says nothing, and that paint's name speaks. So does whoever REPAINTS a box (the look's skin, the editor, the
 *  cars trial): a new paint drops the old word. Spread the answer into the box being made. */
export const saidFor = <S extends BoxStuff>(stuff: S, paint: readonly number[], ...own: readonly (readonly number[])[]): { stuff?: S } =>
  own.includes(paint) ? { stuff } : {};   // (the row handed over, not its numbers: a copy of the kit's wood is nobody's paint and gets no veneer, as it gets no finish)
// ---- A PAINT IS KNOWN BY THE ROW HANDED OVER, NOT BY ITS NUMBERS ----
/** what a kit knows of one of its paints: its name in the palette, its finish, and what it is made of ('plain' is no word: absent) */
export interface Paint { readonly name: string; readonly rough?: number; readonly metal?: boolean; readonly stuff?: BoxStuff }
/** each palette row's own array → what its kit said of it. Keyed by the ARRAY (`albedo: C.wood` hands the row over), never by its numbers: two paints
 *  with one colour are two paints, and a colour written out as numbers is nobody's paint. Rows are added as their palette's module loads; never removed. */
const PAINTS = new WeakMap<readonly number[], Paint>();
/** the word a paint name gives a box: the table's, less 'plain' (absent = plain everywhere a box is kept) and the two layers no box is made of */
const wordOfName = (name: string): BoxStuff | undefined => { const s = stuffOfName(name); return s === undefined || s === 'plain' || !boxStuffOk(s) ? undefined : s; };
/** A KIT NAMES ITS PAINTS: every row of `palette` is known from here on by its own array (paintOf), with its finish from `finish` and its stuff from the
 *  table of names by its name, or under the name `as(name)` gives where the palette's names are not the table's. Call it once, beside the palette. */
type Finish = { rough?: number; metal?: boolean };
/** a paint's record with only the keys it means: its name, its finish where it has one (a metal flag goes with a roughness, never alone), its stuff unless
 *  that is plain */
const record = (name: string, finish: Finish | undefined, stuff: BoxStuff | undefined): Paint =>
  ({ name, ...(finish?.rough !== undefined ? { rough: finish.rough, ...(finish.metal ? { metal: true } : {}) } : {}), ...(stuff !== undefined && stuff !== 'plain' ? { stuff } : {}) });
export function namePaints(palette: Readonly<Record<string, readonly number[]>>, finish: Readonly<Record<string, { rough: number; metal?: boolean } | undefined>> = {}): void {
  for (const [name, row] of Object.entries(palette)) PAINTS.set(row, record(name, finish[name], wordOfName(name)));
}
/** … and one paint that is a named paint to the eye in other numbers (gwotPalette.ts gwotAs: the same paint standing for something else to a round): the
 *  name's stuff, and the finish only if `finish` is given (a stand-in's finish is written by whoever lays it) */
export function namePaint(row: readonly number[], name: string, finish?: { rough: number; metal?: boolean }): void { PAINTS.set(row, record(name, finish, wordOfName(name))); }
/** … and a paint of a kit's OWN that the table of names does not know (its names are its own, or its name there is another kit's paint): what it is
 *  made of and its finish, said outright */
export function sayPaint(row: readonly number[], name: string, said: { stuff?: BoxStuff } & Finish): void { PAINTS.set(row, record(name, said, said.stuff)); }
/** ONE PAINT, ANOTHER THING: the same paint to the eye (its numbers, its name, its finish) said to be made of `stuff` where a kit paints two things with
 *  it and the row can say only one: `saidAs('plain', C.asphalt)` is the lot under a level, whose paint's row says metal for the iron fittings.
 *  `finish` = the thing's own, where its stuff has another than the paint's row (as `madeOf` takes one: the word and its finish are said together, so
 *  nobody writes a roughness by hand beside the word): `saidAs('paint', dust, { rough: 0.5 })` is an enamel in the colour of dust. */
export function saidAs(stuff: BoxStuff, paint: readonly number[], finish?: { rough: number; metal?: boolean }): [number, number, number] {
  const was = PAINTS.get(paint), out: [number, number, number] = [paint[0], paint[1], paint[2]]; PAINTS.set(out, record(was?.name ?? '', finish ?? was, stuff)); return out;
}
/** A COLOUR WRITTEN AS NUMBERS SAYS ITS WORD BESIDE IT: `madeOf('cloth', [0.3, 0.1, 0.09])` is those numbers, known from here on as a paint made of
 *  that (and of that finish, if one is given), wherever they are handed: a box's albedo, a helper's argument, a constant a generator names once. */
export function madeOf(stuff: BoxStuff, colour: [number, number, number], finish?: { rough: number; metal?: boolean }): [number, number, number] { sayPaint(colour, '', { stuff, ...finish }); return colour; }
/** … and the same, where NOBODY CHOSE the word: `drawnAs('metal', [0.08, 0.08, 0.09])` is what the look made of those numbers by their COLOUR until 11-04
 *  (a dark grey is iron's grey, darkened: a monitor's bezel was drawn as bare iron), written beside them so that no box moved the day the colour rule went.
 *  Every call is a place a choice is still owed: when somebody makes it, the call becomes a named paint or madeOf. (`grep drawnAs` is the list.) */
export const drawnAs: typeof madeOf = (stuff, colour, finish) => madeOf(stuff, colour, finish);
/** TWO PAINTS MANY LEVELS WRITE OUT, named once so that what each is made of is one decision in one place: the grey of a breaker panel's door (sheet steel),
 *  and the grey of a stacking chair (still as the look drew its colour: nobody has chosen). */
export const PANEL_GREY = madeOf('sheet', [0.42, 0.44, 0.45]), CHAIR_GREY = drawnAs('paint', [0.5, 0.5, 0.48]);
/** … and a third that somebody did choose: the lot's concrete kerb (it was drawn as sheet metal) */
export const KERB_GREY = madeOf('slab', [0.5, 0.5, 0.47]);
/** … and a fourth, laid by four generators: a server rack's body, with its sheen. Nobody chose its word either (it was drawn as bare iron by its colour) */
export const RACK_BLACK = drawnAs('metal', [0.06, 0.06, 0.07], { rough: 0.3 });
/** the paint this array IS, if a kit named it: asked of the array a maker was handed (`b.albedo`), before it is copied */
export const paintOf = (albedo: readonly number[] | undefined): Paint | undefined => albedo ? PAINTS.get(albedo) : undefined;
/** A PAINT TONED IS THE SAME STUFF: `out` is the paint `from` in another tone (worn ground is its ground darker, a faded door its paint paler), so it
 *  keeps the word of the paint it was made from; it has no finish of its own (whoever lays it writes one). A paint nobody named tones to numbers nobody
 *  named. Returns `out`. */
export function tonedAs<T extends readonly number[]>(from: readonly number[], out: T): T {
  const was = PAINTS.get(from); if (was) PAINTS.set(out, { name: was.name, ...(was.stuff !== undefined ? { stuff: was.stuff } : {}) });
  return out;
}
/** … `paint` × k, a number or one a channel */
export function toned(paint: readonly number[], k: number | readonly number[]): [number, number, number] {
  const m = typeof k === 'number' ? [k, k, k] : k; return tonedAs(paint, [paint[0] * m[0], paint[1] * m[1], paint[2] * m[2]]);
}
/** the word a box made with `albedo` carries: what the maker said (`said`; 'plain' = nothing, said aloud over a paint that says something), else the
 *  paint's own. Spread the answer into the box being made. */
export function wordFor(albedo: readonly number[] | undefined, said?: BoxStuff): { stuff?: BoxStuff } {
  const w = said ?? paintOf(albedo)?.stuff; return w !== undefined && w !== 'plain' ? { stuff: w } : {};
}
/** … and the same for a thing already made (a prop's part as a kit wrote it): its word is put on it, or taken off where it is plain. Returns it. */
export function worded<T extends { albedo?: readonly number[]; stuff?: BoxStuff }>(o: T): T {
  const w = wordFor(o.albedo, o.stuff).stuff; if (w !== undefined) o.stuff = w; else delete o.stuff;
  return o;
}
/** A NAMED COAT IS A KIND OF A PLAIN ONE: 'carpaint' is paint, on a vehicle. One row a named coat (the look's own material for it, and one thing to a round:
 *  game/particleFx.ts STUFF_SURF), beside the plain coat it is a kind of — which is a coat on anything and says nothing of what is under it. */
export const COAT_KIND: Partial<Record<BoxStuff, BoxStuff>> = { carpaint: 'paint' };
/** the word of a thing that said `was` and is painted again with a paint that says `now`: the new paint's (whoever repaints a thing does not keep the
 *  old paint's word) — but A NAMED COAT STAYS UNDER A PLAIN ONE: a car's body given another enamel is still a car's body. */
export const repaintWord = (was: BoxStuff | undefined, now: BoxStuff | undefined): BoxStuff | undefined =>
  was !== undefined && now !== undefined && Object.hasOwn(COAT_KIND, was) && COAT_KIND[was] === now ? was : now;
/** A BOX PAINTED AGAIN, the one place it is done (the default look's skin, for boxes and for props' parts: tools/levels/gwotSkin.ts): the new paint, and
 *  the word repaintWord gives. Returns it. */
export function repainted<T extends { albedo?: readonly number[]; stuff?: BoxStuff }, A extends NonNullable<T['albedo']>>(o: T, albedo: A): T {
  const was = o.stuff; o.albedo = albedo; delete o.stuff; worded(o);
  const w = repaintWord(was, o.stuff); if (w !== undefined) o.stuff = w; else delete o.stuff;
  return o;
}
