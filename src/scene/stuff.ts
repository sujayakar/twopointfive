// WHAT A THING IS MADE OF IS SAID, NEVER READ OFF ITS COLOUR.
// Three places say it, and each is data somebody wrote:
//   · a PAINT'S NAME. A kit's palette names its paints ('wood', 'carpet', 'tWall') and this table says what each name is made of: one of the look's materials
//     (render/lookTex.ts LOOK_NAMES); 'plain', meant to be drawn as a flat colour (laminate, a dark screen); 'waits', a stuff the look has no
//     material for yet (card, foliage; drawn plain); or one of the two the look draws plain and a round's hit reads, 'glass' and 'rubber' (BOX_STUFFS below). Where one name is more than one thing, the piece that knows better says its own word;
//   · a level file's MATERIAL ROW (levelData.ts MaterialData.stuff), written by the exporter from this table: the file says it, and the loader asks no table;
//   · a BOX or a prop's part (Box.stuff, a file's `stuff` on a box), where it is another thing than its paint's row says, or wears a paint written as numbers.
// A box takes its own word, else its row's, when the file is loaded (levelLoad.ts), and a kit's add() does the same for a box it makes with a named paint
// (paintOf below). The look draws what Box.stuff says and nothing else (render/gbuffer.ts lookWordOf): a box that says nothing is plain. NOTHING ANYWHERE
// ASKS A COLOUR WHAT IT IS MADE OF. (Until 11-04 the look matched a box's colour against the 2010s palette, exactly and darkened in one piece, and the kits
// found a paint's finish by its numbers: a wall repainted a slightly different grey was silently another material. tools/levels/stuffWas.ts keeps that rule,
// frozen, for one job: bringing a level file written before forward, each box saying what it was drawn as.)
// A NAME NEVER CHANGES A COLOUR, and a colour says nothing: this table gives a paint what it is drawn as and leaves its numbers alone.
// WHAT A PAINT DOES IS SAID THE SAME WAY, IN ANOTHER WORD (PAINT_DOES below; scene/materials.ts has the rows). A paint's name says what a box of it is to
// a round, a charge and a blast; a kit puts that word on the box beside `stuff` (Box.does), and the two are never worked out from one another: `stuff` is
// the picture's and a repaint changes it, `does` is the rules' and a repaint leaves it. The rules read the box (materials.ts doesOf), never this table.
import type { GwotMat } from './gwotPalette';
import type { DoesWord } from './materials';   // (a type: what a paint DOES is a word of the table of materials)
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

export const PAINT_STUFF: Readonly<Record<string, Stuff>> = {
  ...PALETTE_STUFF,
  // ---- the office's paints (scene/officeGen.ts OFFICE_PALETTE: the names the simulation knows a paint by, game/particleFx.ts MAT_SURF)
  wood: 'wood', woodDark: 'wood',   // (planks: the name is a bench, a crate and a pallet as well as a desk. A desk's smooth board is 'veneer', and the desk says so itself)
  carpet: 'cloth', carpetBlue: 'cloth', carpetRed: 'cloth', chair: 'fabric',   // (a carpet is the coarse weave; a seat's cloth the fine one)
  wall: 'plaster', ceiling: 'plaster', wallDark: 'block', concrete: 'block', stone: 'slab', lino: 'slab',
  black: 'plain', white: 'plain', screenOff: 'plain', glass: 'plain', glassExt: 'plain',
  cardboard: 'waits', plant: 'waits',
  grass: 'grass',       // (a lawn, a verge: the look's grass since 11-05. No round meets one: a lawn is a strip no ray meets, laid on the lot's slab, and a round into it is a round into that)
  metal: 'steel',   // (an office's steel is CLEAN painted steel: shelving, desk legs, lamp posts, bins. The look's 'metal' is bare iron gone to rust and its 'paint' is chipped)
  // ---- ONE NAME, MORE THAN ONE THING. The kits have used these six paints for two things each (a level file's writer names a box by the row that has its
  // numbers: levelLoad.ts levelToData), so the name cannot say what its boxes are and a PIECE that knows better says its own word: a cubicle partition says
  // fabric. NOBODY CHOSE THE WORDS HERE (but the car park's, 11-05): each is what the look made of the paint's COLOUR until 11-04, kept so that no box moved the day the colour rule went.
  partRed: 'plain', partBlue: 'plain',   // (cubicle partitions, which say fabric themselves; and a car's body, lived things)
  partTan: 'cloth',     // (a partition; and a hut's stucco walls, drawn as cloth because the tan is a cream rug's, darkened)
  red: 'plain',         // (an extinguisher's enamel; and tape, a ball, a mast's lamps)
  asphalt: 'tarmac',    // (the car park, the road: the look's tarmac since 11-05, and concrete to the picture of a round's hit wet or dry (game/particleFx.ts STUFF_SURF: dry, its paint always was; wet, its tone read as metal). Until 11-04 its grey was drawn as bare iron, iron's grey darkened; the lived kit's iron fittings, in the same numbers, say metal themselves: livedPieces.ts INK.iron. A thing painted in it that is no lot says its own word: the compound's mud at the pump is earth)
  glassFrame: 'plain',  // (a window's frame; and every outside door's leaf, painted from the same numbers as the game runs)
  // ---- the snow kit's (the look lays its own snow over a thing; the paint under it stays plain)
  snow: 'plain', snowOld: 'plain', snowPacked: 'plain',
};
/** WHAT EACH PAINT DOES, by its name: the row of the table of materials (scene/materials.ts) a box of that paint takes its `does` from. Every word here is
 *  what the rules made of the paint's COLOUR until 2026-11-05, written out so that no rule moved the day they stopped reading colours: so two rugs are
 *  'dark carpet' because their dye is dark, and the white plastic is 'grey plastic' because a plain light grey was a cabinet to a blast. Each is a choice
 *  somebody may now make by changing one word. A paint with no row here says nothing, and a box of it takes the default row unless it says its own. */
const PALETTE_DOES = {
  mud: 'concrete', mudDark: 'concrete', render: 'concrete', block: 'concrete', tWall: 'concrete', slab: 'concrete', sandbag: 'concrete', hesco: 'concrete',   // (structure: a shell, a yard wall, a T-wall)
  mudLight: 'plasterboard', whitewash: 'plasterboard', paintCream: 'plasterboard',   // (a partition: a charge cuts it, a round goes through)
  dust: 'tarmac', dirt: 'tarmac', gravel: 'tarmac', damp: 'tarmac', oil: 'tarmac', mudFloor: 'stone',   // (the ground)
  rugRed: 'carpet', rugCream: 'carpet', rugOchre: 'carpet', rugGreen: 'carpet', rugNavy: 'dark carpet', rugMadder: 'dark carpet',
  canvas: 'fabric', blanketGrey: 'fabric', blanketBrown: 'fabric', tarpBlue: 'fabric', sack: 'fabric', coyote: 'fabric',
  paintBlue: 'steel', paintGreen: 'steel', paintRust: 'steel', stencil: 'steel', rust: 'steel', iron: 'steel', galv: 'steel', tin: 'steel', drumBlue: 'steel', drumRed: 'steel', genYellow: 'steel',
  truckWhite: 'steel', truckTan: 'steel', truckRed: 'steel',
  olive: 'wood', poplar: 'wood', woodGrey: 'wood', ply: 'wood',   // (olive is a wooden locker and an ammunition box)
  plasticWhite: 'grey plastic', plasticGreen: 'plastic', plasticBlue: 'plastic', jerryYellow: 'plastic', plasticBlack: 'dark plastic', rubber: 'dark plastic', glassDark: 'dark glass',
  prodOrange: 'plant', prodGreen: 'plant', prodRed: 'plant', prodYellow: 'plant', straw: 'plant', scrub: 'plant',
} as const satisfies Record<GwotMat, DoesWord>;
export const PAINT_DOES: Readonly<Record<string, DoesWord>> = {
  ...PALETTE_DOES,
  // ---- the office's paints, which the kit's palette shares (scene/officeGen.ts OFFICE_PALETTE, levelKit.ts KIT_PALETTE)
  wall: 'plasterboard', partRed: 'partition', partBlue: 'partition', partTan: 'partition', ceiling: 'ceiling', white: 'laminate', glass: 'pane', glassExt: 'pane',
  concrete: 'concrete', wallDark: 'concrete', stone: 'stone', lino: 'lino', asphalt: 'tarmac', carpet: 'carpet', carpetBlue: 'carpet', carpetRed: 'carpet', grass: 'grass',
  wood: 'wood', woodDark: 'wood', cardboard: 'card', metal: 'steel', red: 'enamel', black: 'casing', glassFrame: 'frame', screenOff: 'screen', chair: 'fabric', plant: 'plant',
  // ---- the snow kit's
  snow: 'snow', snowPacked: 'packed snow', snowOld: 'old snow',
};
/** what a paint name does: undefined for a name the table has no row for */
export const doesOfName = (name: string): DoesWord | undefined => Object.hasOwn(PAINT_DOES, name) ? PAINT_DOES[name] : undefined;
/** what a level file's material ROW says its boxes do, by the row's name. Spread into the row the exporter writes (which then asks the paint itself) */
export const rowDoes = (name: string): { does?: DoesWord } => { const w = doesOfName(name); return w !== undefined ? { does: w } : {}; };
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
/** what a kit knows of one of its paints: its name in the palette, its finish, what it is made of ('plain' is no word: absent), and what it does
 *  (scene/materials.ts; absent = the paint says nothing, and a box of it is asked no further here) */
export interface Paint { readonly name: string; readonly rough?: number; readonly metal?: boolean; readonly stuff?: BoxStuff; readonly does?: DoesWord }
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
const record = (name: string, finish: Finish | undefined, stuff: BoxStuff | undefined, does?: DoesWord): Paint =>
  ({ name, ...(finish?.rough !== undefined ? { rough: finish.rough, ...(finish.metal ? { metal: true } : {}) } : {}), ...(stuff !== undefined && stuff !== 'plain' ? { stuff } : {}), ...(does !== undefined ? { does } : {}) });
const fin = (p: Paint): string => `${p.stuff ?? 'plain'}${p.rough !== undefined ? ` at ${p.rough}${p.metal ? ', metal' : ''}` : ''}${p.does !== undefined ? `, doing ${p.does}` : ''}`;
/** ONE ARRAY IS ONE PAINT, FOR GOOD. A paint is known by its ARRAY (above), so saying a second thing of an array that already is a paint would change
 *  what every box painted from it is made of AND WHAT IT DOES (the rules read that word), wherever it was handed: `madeOf('steel', C.black)` would turn
 *  a kit's every black thing to steel, and the last module to load would win. So THE FIRST SAYING STANDS, its name with it, and a second saying that
 *  differs from it in its stuff, its finish or what it does is REFUSED (no finish is a finish, no word a word: a paint that has none cannot be given
 *  one in place). The same thing said twice is nothing (a module evaluated again, a helper called per level). A second thing in the same colour is
 *  another array: `saidAs`, or the numbers written out again. ONE THING IS ADDED IN PLACE, by its own function: what a colour WRITTEN AS NUMBERS does
 *  (`doing` below, after `madeOf` or alone), once; a kit's NAMED row does what the table of names says and nothing else.
 *  HOW IT IS REFUSED (ONE_PAINT.refuses): OUT LOUD, a throw, wherever a level is MADE or checked: the exporters, the generators, every tool and the
 *  headless harness, which all run under bun. IN THE PAGE it is told once to the console for that paint and the first saying is kept: the page never
 *  stops on a writer's slip. Check: tools/qa/stuff-check.ts, the line that builds every generator in one process. */
export const ONE_PAINT = { refuses: (globalThis as { Bun?: unknown }).Bun !== undefined };
const told = new WeakSet<readonly number[]>();
/** the saying is refused: thrown, or in the page told once and passed over (false = the caller leaves the paint as it was) */
function refuse(row: readonly number[], was: Paint, next: Paint): false {
  const what = `stuff: the paint [${row.join(', ')}]${was.name ? ` '${was.name}'` : ''} is ${fin(was)} and cannot also be ${fin(next)}${next.name && next.name !== was.name ? ` ('${next.name}')` : ''}: one array is one paint. A second thing in the same colour is its own array (saidAs, or the numbers written again)`;
  if (ONE_PAINT.refuses) throw new Error(what);
  if (!told.has(row)) { told.add(row); console.error(`${what}. The first saying is kept.`); }
  return false;
}
function keep(row: readonly number[], next: Paint): void {
  const was = PAINTS.get(row);
  if (was === undefined) { PAINTS.set(row, next); return; }
  if (was.stuff !== next.stuff || was.rough !== next.rough || !!was.metal !== !!next.metal || was.does !== next.does) refuse(row, was, next);
}
export function namePaints(palette: Readonly<Record<string, readonly number[]>>, finish: Readonly<Record<string, { rough: number; metal?: boolean } | undefined>> = {}): void {
  for (const [name, row] of Object.entries(palette)) keep(row, record(name, finish[name], wordOfName(name), doesOfName(name)));
}
/** … and one paint that is a named paint to the eye in other numbers (gwotPalette.ts gwotAs: the same paint standing for something else to a round): the
 *  name's stuff, and the finish only if `finish` is given (a stand-in's finish is written by whoever lays it) */
export function namePaint(row: readonly number[], name: string, finish?: { rough: number; metal?: boolean }, does?: DoesWord): void { keep(row, record(name, finish, wordOfName(name), does ?? doesOfName(name))); }   // (`does`: what the stand-in stands for to a round, where that is another thing than the name's)
/** … and a paint of a kit's OWN that the table of names does not know (its names are its own, or its name there is another kit's paint): what it is
 *  made of and its finish, said outright */
export function sayPaint(row: readonly number[], name: string, said: { stuff?: BoxStuff; does?: DoesWord } & Finish): void { keep(row, record(name, said, said.stuff, said.does)); }
/** ONE PAINT, ANOTHER THING: the same paint to the eye (its numbers, its name, its finish) said to be made of `stuff` where a kit paints two things with
 *  it and the row can say only one: `saidAs('earth', C.asphalt)` is the compound's mud at the pump, laid in the car park's paint for its darkness.
 *  `finish` = the thing's own, where its stuff has another than the paint's row (as `madeOf` takes one: the word and its finish are said together, so
 *  nobody writes a roughness by hand beside the word): `saidAs('paint', dust, { rough: 0.5 })` is an enamel in the colour of dust. */
export function saidAs(stuff: BoxStuff, paint: readonly number[], finish?: { rough: number; metal?: boolean }): [number, number, number] {
  const was = PAINTS.get(paint), out: [number, number, number] = [paint[0], paint[1], paint[2]]; PAINTS.set(out, record(was?.name ?? '', finish ?? was, stuff, was?.does)); return out;   // (drawn as another thing, it does what the paint does)
}
/** A COLOUR WRITTEN AS NUMBERS SAYS ITS WORD BESIDE IT: `madeOf('cloth', [0.3, 0.1, 0.09])` is those numbers, known from here on as a paint made of
 *  that (and of that finish, if one is given), wherever they are handed: a box's albedo, a helper's argument, a constant a generator names once. */
export function madeOf(stuff: BoxStuff, colour: [number, number, number], finish?: { rough: number; metal?: boolean }): [number, number, number] { sayPaint(colour, '', { stuff, ...finish }); return colour; }
/** THE SAME PAINT ON A THING THAT DOES SOMETHING ELSE: `paint` itself where that is what it does anyway, else its numbers as a paint of their own, known by
 *  everything `paint` is known by (its name, its finish, what it is drawn as) and doing `does`. The one way a stand-in is made: a kit's row on another
 *  thing (gwotPalette.ts gwotAs), and the same over paints a host hands a kit (gwotKit.ts). Returns the paint to lay. */
export function doingAs(paint: readonly number[], does: DoesWord): [number, number, number] {
  const was = PAINTS.get(paint); if (was?.does === does) return paint as [number, number, number];
  const out: [number, number, number] = [paint[0], paint[1], paint[2]]; PAINTS.set(out, { ...(was ?? { name: '' }), does }); return out;
}
/** … AND WHAT IT DOES, the same way (scene/materials.ts): `doing('steel', [0.58, 0.58, 0.56])` is those numbers, known from here on as a paint a box of
 *  which does what steel does, whatever else was said of them (madeOf first, then this, or either alone). A colour nobody says this of is brought forward
 *  by the exporter with the word its tone had (tools/levels/doesWas.ts). Returns the colour. */
export function doing<T extends readonly number[]>(does: DoesWord, colour: T): T {
  const was = PAINTS.get(colour), next: Paint = { ...(was ?? { name: '' }), does };
  // (ONE ARRAY IS ONE PAINT, above: said once, of numbers written out. A kit's named row, or a colour that already does another thing, is refused)
  if (was !== undefined && was.does !== does && (was.does !== undefined || was.name !== '')) refuse(colour, was, next); else if (was?.does !== does) PAINTS.set(colour, next);
  return colour;
}
/** … and the same, where NOBODY CHOSE the word: `drawnAs('paint', [0.5, 0.5, 0.48])` is what the look made of those numbers by their COLOUR until 11-04
 *  (a mid grey is a faded paint's; a dark grey was iron's, darkened: a keyboard was drawn as bare iron until 11-05), written beside them so that no box moved the day the colour rule went.
 *  Every call is a place a choice is still owed: when somebody makes it, the call becomes a named paint or madeOf. (`grep drawnAs` is the list.) */
export const drawnAs: typeof madeOf = (stuff, colour, finish) => madeOf(stuff, colour, finish);
/** TWO PAINTS MANY LEVELS WRITE OUT, named once so that what each is made of is one decision in one place: the grey of a breaker panel's door (sheet steel),
 *  and the grey of a stacking chair (still as the look drew its colour: nobody has chosen). */
export const PANEL_GREY = madeOf('sheet', [0.42, 0.44, 0.45]), CHAIR_GREY = drawnAs('paint', [0.5, 0.5, 0.48]);
/** … and a third that somebody did choose: the lot's concrete kerb (it was drawn as sheet metal) */
export const KERB_GREY = madeOf('slab', [0.5, 0.5, 0.47]);
/** … and a fourth, laid by four generators and the kit: a server rack's body, with its sheen. STEEL under a powder coat (11-05: by its colour it was drawn as
 *  bare iron, and the look's iron is iron somebody left out: on a GPU a rack stood speckled with rust). The look's `metal` is that iron and nothing kept */
export const RACK_BLACK = madeOf('steel', [0.06, 0.06, 0.07], { rough: 0.3 });
/** … and the dark plastics a desk has in every kit: a keyboard, and a telephone (the office's handset, the dressing's set: both were drawn as bare iron
 *  by their colour) */
export const KEY_BLACK = madeOf('plastic', [0.08, 0.08, 0.09]), PHONE_BLACK = madeOf('plastic', [0.07, 0.07, 0.08]);
/** the paint this array IS, if a kit named it: asked of the array a maker was handed (`b.albedo`), before it is copied */
export const paintOf = (albedo: readonly number[] | undefined): Paint | undefined => albedo ? PAINTS.get(albedo) : undefined;
/** A PAINT TONED IS THE SAME STUFF: `out` is the paint `from` in another tone (worn ground is its ground darker, a faded door its paint paler), so it
 *  keeps the word of the paint it was made from; it has no finish of its own (whoever lays it writes one). A paint nobody named tones to numbers nobody
 *  named. NOT WHAT IT DOES: a tone was never its paint to the rules (they knew a paint by its exact numbers, and a tone was classed by how bright it
 *  was), so a toned paint says nothing of what it does, as measured; whoever means the toned thing to do what its paint does says so (`doing`). Returns `out`. */
export function tonedAs<T extends readonly number[]>(from: readonly number[], out: T): T {
  const was = PAINTS.get(from); if (was) keep(out, { name: was.name, ...(was.stuff !== undefined ? { stuff: was.stuff } : {}) });
  return out;
}
/** … `paint` × k, a number or one a channel */
export function toned(paint: readonly number[], k: number | readonly number[]): [number, number, number] {
  const m = typeof k === 'number' ? [k, k, k] : k; return tonedAs(paint, [paint[0] * m[0], paint[1] * m[1], paint[2] * m[2]]);
}
/** the words a box made with `albedo` carries: what the maker said (`said`; 'plain' = nothing, said aloud over a paint that says something), else the
 *  paint's own; and what it does (`does`, else the paint's own, else nothing said). Spread the answer into the box being made. */
export function wordFor(albedo: readonly number[] | undefined, said?: BoxStuff, does?: DoesWord): { stuff?: BoxStuff; does?: DoesWord } {
  const p = paintOf(albedo), w = said ?? p?.stuff, d = does ?? p?.does; return { ...(w !== undefined && w !== 'plain' ? { stuff: w } : {}), ...(d !== undefined ? { does: d } : {}) };
}
/** … and the same for a thing already made (a prop's part as a kit wrote it): its word is put on it, or taken off where it is plain. Returns it. */
export function worded<T extends { albedo?: readonly number[]; stuff?: BoxStuff; does?: DoesWord }>(o: T): T {
  const w = wordFor(o.albedo, o.stuff, o.does); if (w.stuff !== undefined) o.stuff = w.stuff; else delete o.stuff;
  if (w.does !== undefined) o.does = w.does;   // (what it does: its own word, else its paint's; a thing nobody spoke for keeps the key it had, absent or `undefined`)
  return o;
}
/** THE PLAIN COATS: the words that are a coat on anything and say nothing of what is under them (a brushed paint, a sprayed enamel). A palette paints a
 *  thing in whichever its look has, so a rule about "a plain coat" is about all of them. */
export const PLAIN_COATS: readonly BoxStuff[] = ['paint', 'enamel'];
/** A NAMED COAT IS A KIND OF A PLAIN ONE: 'carpaint' is a coat, on a vehicle. One row a named coat (the look's own material for it, and one thing to a round:
 *  game/particleFx.ts STUFF_SURF): `under` = the plain coats it is a kind of, and stays under when a look repaints the thing; `rough` = HOW SMOOTH THE
 *  COAT IS, whatever it lies on (coatRough below). A car's clear coat: 0.2, under every preset's cut-off for the traced mirror less its fade
 *  (ui/quality.ts RtTier.maxRough), the number agreed with the look's car paint map (docs/agents/render.md "CAR PAINT IS A STUFF"). */
export interface NamedCoat { readonly under: readonly BoxStuff[]; readonly rough: number }
export const NAMED_COATS: Partial<Record<BoxStuff, NamedCoat>> = { carpaint: { under: PLAIN_COATS, rough: 0.2 } };
export const namedCoat = (word: BoxStuff | undefined): NamedCoat | undefined => word !== undefined && Object.hasOwn(NAMED_COATS, word) ? NAMED_COATS[word] : undefined;
/** A NAMED COAT'S FINISH GOES WHEREVER ITS WORD GOES: a box that says a named coat is drawn no rougher than that coat, whatever row painted it and
 *  whatever number a kit, a skin or a hand wrote beside the word (a vehicle's body took a truck enamel's satin, a cot's and a locker's too, or nothing at
 *  all: it showed next to nothing of what stands round it). The word decides; a thing that should not shine like a car does not say it is car paint.
 *  Smoother stays smoother (a body wet with rain). ROUGHNESS ONLY: whether a thing is bare metal stays its paint's (a piece the game breaks off a thing
 *  is given the thing's own finish and flag, game/breakTable.ts; what a thing DOES is its other word, scene/materials.ts, and reads neither).
 *  WHERE: read by the one function that hands a box's finish to the GPU (scene/boxes.ts matWord), so it holds for a level's statics, a shape's pieces, a
 *  piece blown off a car and anything made later that says the word. No box is changed and no level file carries the number: `Box.rough` is what the file
 *  gave, always.
 *  THE SWITCH: COATFINISH.on — ON in the page (`?coatfinish=0` = every box the finish its file gives it, as it was), OFF in the module and on the frozen
 *  stand (FROZEN_HOLDS; TPS_COATFINISH=1 raises it there); in the registry (game/knobs.ts) and the journal. A picture only.
 *  CHECK: tools/qa/coatfinish-check.ts (COATFINISH_BREAK=1 fails it), on both stands. */
export const COATFINISH = { on: false };
/** the roughness a named coat lends the thing that says `word`, or nothing (the rule down, no word, a word that is no named coat) */
export const coatRough = (word: BoxStuff | undefined): number | undefined => COATFINISH.on && word !== undefined ? namedCoat(word)?.rough : undefined;
/** A THING OF SOLID GLASS IS DARK AND SMOOTH: a box that says 'glass' and is no pane (a vehicle's window, a display case's top, a handset's screen:
 *  nothing is modelled behind them) is sent to the GPU as a neutral near black and polished, whatever grey its row or its maker gave it — glass is dark
 *  because what is seen through it is an unlit inside, and it is what mirrors most on a car. A PANE (BoxFlag.Glass) is the glass engine's and is not
 *  touched; nor is a face that carries MARKINGS (a mark is laid out in its own colours on the colour under it: a panel's grid, a screen's picture).
 *  THE COLOUR IS NOT REPAINTED: `Box.albedo` stays the file's. This is a rule of the PICTURE, keyed on the word a box is DRAWN as (Box.stuff): what a
 *  box DOES is its other word (Box.does, scene/materials.ts: a kit vehicle's window does what 'dark glass' does) and is neither asked nor touched. So it
 *  goes wherever the word goes, whoever painted the box (the kits' dark glass, a plain car's own grey), and the game still finds the box's own colour
 *  where it takes one (what a piece broken off it is painted, a chip's tint, a lamp known by its tone). The pack sends it dark, in the one place it
 *  sends a wet box darker (scene/boxes.ts sentAlbedo), and no rougher than `rough` (matWord). Darker and smoother stay as they are. NEUTRAL, not the
 *  row's hue scaled down: under about 0.03 a hue does not survive the 8-bit albedo target (render/shaders/gbuffer.wgsl), and a channel on a rounding
 *  edge would flicker.
 *  `lum` and `rough` ride in the table, so the page's knobs and a recording carry them.
 *  THE SWITCH: SOLIDGLASS.on — ON in the page (`?solidglass=0` = every such box as its file gives it, as it was), OFF in the module and on the frozen
 *  stand (FROZEN_HOLDS; TPS_SOLIDGLASS=1 raises it there); in the registry (game/knobs.ts) and the journal. A picture only on the harness; on a GPU page
 *  the light meter's bounce sees the darker glass, as it sees a wet box's darker ground (Box.wetK).
 *  CHECK: tools/qa/coatfinish-check.ts lines 6 and 7 (SOLIDGLASS_BREAK=1 fails both), on both stands. */
export const SOLIDGLASS = { on: false, lum: 0.02, rough: 0.05 };
/** the word of a thing that said `was` and is painted again with a paint that says `now`: the new paint's (whoever repaints a thing does not keep the
 *  old paint's word) — but A NAMED COAT STAYS UNDER A PLAIN ONE, whichever plain coat the look has: a car's body given another paint or another enamel is
 *  still a car's body. */
export const repaintWord = (was: BoxStuff | undefined, now: BoxStuff | undefined): BoxStuff | undefined =>
  now !== undefined && namedCoat(was)?.under.includes(now) ? was : now;
/** A BOX PAINTED AGAIN, the one place it is done (the default look's skin, for boxes and for props' parts: tools/levels/gwotSkin.ts): the new paint, and
 *  the word repaintWord gives. WHAT IT DOES IS NOT TOUCHED (Box.does): a thing in other clothes is the same thing to a round. Returns it. */
export function repainted<T extends { albedo?: readonly number[]; stuff?: BoxStuff; does?: DoesWord }, A extends NonNullable<T['albedo']>>(o: T, albedo: A): T {
  const was = o.stuff, did = o.does, had = 'does' in o; o.albedo = albedo; delete o.stuff; worded(o); if (had) o.does = did; else delete o.does;
  const w = repaintWord(was, o.stuff); if (w !== undefined) o.stuff = w; else delete o.stuff;
  return o;
}
