// WHAT A PAINT IS MADE OF, BY ITS NAME.
// A level file's `materials` block names every paint its boxes wear (scene/levelData.ts MaterialData: 'wood', 'carpet', 'tWall'). This table says, for each
// name the kits write, what that paint is MADE OF:
//   · one of the look's materials (render/lookTex.ts LOOK_NAMES);
//   · 'plain': meant to be drawn as a flat colour (plastic, laminate, rubber, a dark screen);
//   · 'waits': a stuff the look has no material for yet (card, foliage, a lawn, a clean steel). Drawn plain;
//   · 'mixed': ONE NAME, MORE THAN ONE THING. A level file's writer names a box by its COLOUR (levelLoad.ts levelToData: the first row with those numbers), and
//     the kits have used one colour for two things: a car's body has the blue partition's numbers, a hut's stucco the tan partition's, an iron typewriter the
//     car park's. Such a name is left to the colour rule, as before there was a table, until a box can say its own word.
// The look reads it through the level's own rows (render/gbuffer.ts lookWordOf): a box that wears a named paint is drawn as that name's stuff, whatever its
// colour is, and the colour is asked only of a paint with no name or a mixed one (lookStuff: the rule the look began with, by which the office's carpet, its
// desks and its chairs were nobody's).
// A NAME NEVER CHANGES A COLOUR. The simulation reads colours (a round's dust by its paint, a sheet of paper by how pale it is): this table gives a paint a
// material and leaves its numbers alone. Nothing in the simulation reads this table.
import type { GwotMat } from './gwotPalette';
import { LOOK_NAMES, lookStuffOf } from '../render/lookTex';   // (a module of arithmetic with no imports of its own: the look's list of materials is the one list)

export type Stuff = typeof LOOK_NAMES[number] | 'plain' | 'waits' | 'mixed';

/** the 2010s palette (scene/gwotPalette.ts GWOT_PALETTE), IN THE ORDER THE LOOK'S SHADER TEXT LISTS ITS ROWS (render/gbuffer.ts LOOK_OF is read off this: a
 *  darkened paint is the first row it fits) — keep that order, and add a new row at the end of its material's run */
const PALETTE_STUFF = {
  mud: 'mud wall', mudDark: 'mud wall', mudLight: 'mud wall', render: 'plaster', whitewash: 'plaster', paintCream: 'plaster', block: 'block', tWall: 'block',
  dust: 'earth', dirt: 'earth', mudFloor: 'earth', damp: 'earth', gravel: 'gravel', slab: 'slab', poplar: 'wood', woodGrey: 'wood', ply: 'wood', galv: 'sheet', tin: 'sheet',
  sack: 'cloth', sandbag: 'cloth', hesco: 'cloth', canvas: 'cloth', tarpBlue: 'cloth', blanketGrey: 'cloth', blanketBrown: 'cloth', rugRed: 'cloth', rugNavy: 'cloth', rugCream: 'cloth', rugOchre: 'cloth', rugGreen: 'cloth', rugMadder: 'cloth', coyote: 'cloth',
  rust: 'metal', iron: 'metal',
  paintBlue: 'paint', paintGreen: 'paint', paintRust: 'paint', drumBlue: 'paint', drumRed: 'paint', genYellow: 'paint', truckWhite: 'paint', truckTan: 'paint', truckRed: 'paint', olive: 'paint',
  oil: 'plain', stencil: 'plain', plasticWhite: 'plastic', plasticGreen: 'plastic', plasticBlue: 'plastic', plasticBlack: 'plastic', jerryYellow: 'plain', rubber: 'plain', glassDark: 'plain',
  prodOrange: 'plain', prodGreen: 'plain', prodRed: 'plain', prodYellow: 'plain', straw: 'waits', scrub: 'waits',
} as const satisfies Record<GwotMat, Stuff>;

export const PAINT_STUFF: Readonly<Record<string, Stuff>> = {
  ...PALETTE_STUFF,
  // ---- the office's paints (scene/officeGen.ts OFFICE_PALETTE: the names the simulation knows a paint by, game/particleFx.ts MAT_SURF)
  wood: 'wood', woodDark: 'wood',   // (planks: the name is a bench, a crate and a pallet as well as a desk. A desk's smooth board is 'veneer', and the desk says so itself)
  carpet: 'cloth', carpetBlue: 'cloth', carpetRed: 'cloth', chair: 'fabric',   // (a carpet is the coarse weave; a seat's cloth the fine one)
  wall: 'plaster', ceiling: 'plaster', wallDark: 'block', concrete: 'block', stone: 'slab', lino: 'slab',
  black: 'plain', white: 'plain', screenOff: 'plain', glass: 'plain', glassExt: 'plain',
  cardboard: 'waits', plant: 'waits', grass: 'waits',
  metal: 'steel',   // (an office's steel is CLEAN painted steel: shelving, desk legs, lamp posts, bins. The look's 'metal' is bare iron gone to rust and its 'paint' is chipped)
  // ---- mixed: what else wears the name's numbers is said beside each
  partRed: 'mixed', partBlue: 'mixed', partTan: 'mixed',   // (cubicle partitions; and a car's body, a hut's stucco walls, lived things)
  red: 'mixed',          // (an extinguisher's enamel; and tape, a ball, a mast's lamps)
  asphalt: 'mixed',      // (the car park; and the lived kit's iron fittings, fourteen boxes to one)
  glassFrame: 'mixed',   // (a window's frame; and every outside door's leaf, painted from the same numbers as the game runs)
  // ---- the snow kit's (the look lays its own snow over a thing; the paint under it stays plain)
  snow: 'plain', snowOld: 'plain', snowPacked: 'plain',
};
/** THE STUFFS THE COLOUR RULE MAY GIVE: the eleven the look began with. A palette row of one of these is also found by its colour, and by its colour darkened
 *  (render/gbuffer.ts LOOK_OF, lookStuff, and the shader's own text). A stuff added since (veneer, plastic, fabric, steel) is given by a NAME or by a box's own
 *  word only: a grey thing is not plastic because a plastic chair is grey. */
export const BY_COLOUR: ReadonlySet<Stuff> = new Set<Stuff>(['mud wall', 'plaster', 'block', 'earth', 'gravel', 'slab', 'wood', 'sheet', 'cloth', 'metal', 'paint']);
/** what the table says of a paint name: undefined for a name it has no row for (never a method of every object: a level may name a paint `constructor`) */
export const stuffOfName = (name: string): Stuff | undefined => Object.hasOwn(PAINT_STUFF, name) ? PAINT_STUFF[name] : undefined;
/** WHAT A BOX MAY SAY IT IS MADE OF (Box.stuff, a level file's `stuff` on a box or a prop's part): a material of the look, plain, or waits. A material is a
 *  name the look has a NUMBER for (lookTex.ts lookStuffOf): the stone and the brick under a mud wall are layers the mud wall shows, and no box is made of them. */
export const BOX_STUFFS: readonly string[] = [...LOOK_NAMES.filter(n => lookStuffOf(n) > 0), 'plain', 'waits'];
export const boxStuffOk = (v: unknown): v is Exclude<Stuff, 'mixed'> => typeof v === 'string' && BOX_STUFFS.includes(v);
/** a level's material rows as the look is handed them: a name and its colour */
export type PaintRows = Readonly<Record<string, { albedo: readonly number[] }>>;
