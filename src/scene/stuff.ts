// WHAT A PAINT IS MADE OF, BY ITS NAME.
// A level file's `materials` block names every paint its boxes wear (scene/levelData.ts MaterialData: 'wood', 'carpet', 'tWall'). This table says, for each
// name the kits write, what that paint is MADE OF: one of the look's materials (render/lookTex.ts LOOK_NAMES), or 'plain' (meant to be drawn as a flat colour:
// plastic, laminate, rubber, a dark screen), or 'waits' (a stuff the look has no material for yet: card, foliage, a lawn. Drawn plain, and counted apart).
// The look reads it through the level's own rows (render/gbuffer.ts lookNames): a box that wears a named paint is drawn as that name's stuff, whatever its
// colour is, and the colour is asked only of a paint with no name (lookStuff: the rule the look began with, by which the office's carpet was nobody's and its
// car park was iron). A level's row may say its own (`stuff` on the row): a level's accent is then a material too.
// A NAME NEVER CHANGES A COLOUR. The simulation reads colours (a round's dust by its paint, a sheet of paper by how pale it is): this table gives a paint a
// material and leaves its numbers alone.
// The palette's rows are here in the order the look's shader text lists them (render/gbuffer.ts LOOK_OF is read off this table): keep that order.
export type Stuff = 'plain' | 'waits' | 'mud wall' | 'plaster' | 'block' | 'earth' | 'gravel' | 'slab' | 'wood' | 'sheet' | 'cloth' | 'metal' | 'paint' | 'field stone' | 'mud brick';

export const PAINT_STUFF: Readonly<Record<string, Stuff>> = {
  // ---- the 2010s palette (scene/gwotPalette.ts GWOT_PALETTE)
  mud: 'mud wall', mudDark: 'mud wall', mudLight: 'mud wall', render: 'plaster', whitewash: 'plaster', paintCream: 'plaster', block: 'block', tWall: 'block',
  dust: 'earth', dirt: 'earth', mudFloor: 'earth', damp: 'earth', gravel: 'gravel', slab: 'slab', poplar: 'wood', woodGrey: 'wood', ply: 'wood', galv: 'sheet', tin: 'sheet',
  sack: 'cloth', sandbag: 'cloth', hesco: 'cloth', canvas: 'cloth', tarpBlue: 'cloth', blanketGrey: 'cloth', blanketBrown: 'cloth', rugRed: 'cloth', rugNavy: 'cloth', rugCream: 'cloth', rugOchre: 'cloth', rugGreen: 'cloth', rugMadder: 'cloth', coyote: 'cloth',
  rust: 'metal', iron: 'metal',
  paintBlue: 'paint', paintGreen: 'paint', paintRust: 'paint', drumBlue: 'paint', drumRed: 'paint', genYellow: 'paint', truckWhite: 'paint', truckTan: 'paint', truckRed: 'paint', olive: 'paint',
  oil: 'plain', stencil: 'plain', plasticWhite: 'plain', plasticGreen: 'plain', plasticBlue: 'plain', plasticBlack: 'plain', jerryYellow: 'plain', rubber: 'plain', glassDark: 'plain',
  prodOrange: 'plain', prodGreen: 'plain', prodRed: 'plain', prodYellow: 'plain', straw: 'waits', scrub: 'waits',
  // ---- the office's paints (scene/officeGen.ts OFFICE_PALETTE: the names the simulation knows a paint by, game/particleFx.ts MAT_SURF)
  wood: 'wood', woodDark: 'wood', red: 'paint',
  metal: 'waits', glassFrame: 'waits',   // (an office's steel is CLEAN steel: shelving, desk legs, lamp posts, mullions. The look's 'metal' is bare iron gone to rust and its 'paint' is chipped: a shelf drew as a wreck under the one and splotched under the other, so it stays plain until the look has a clean steel)
  carpet: 'cloth', carpetBlue: 'cloth', carpetRed: 'cloth', chair: 'cloth', partRed: 'cloth', partBlue: 'cloth', partTan: 'cloth',
  wall: 'plaster', ceiling: 'plaster', wallDark: 'block', concrete: 'block', stone: 'slab', lino: 'slab',
  asphalt: 'gravel',   // (a bend, said as one: the look has no tarmac, and gravel is the nearest ground)
  black: 'plain', white: 'plain', screenOff: 'plain', glass: 'plain', glassExt: 'plain',
  cardboard: 'waits', plant: 'waits', grass: 'waits',
  // ---- the snow kit's (the look lays its own snow over a thing; the paint under it stays plain)
  snow: 'plain', snowOld: 'plain', snowPacked: 'plain',
};
/** a level's material rows as the look is handed them: a name, its colour, and the stuff the row itself says, if it says one */
export type PaintRows = Readonly<Record<string, { albedo: readonly number[]; stuff?: string }>>;
