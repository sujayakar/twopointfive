// THE TABLE OF MATERIALS: one row a word, and in a row two things kept apart.
//   · `looks` — the picture draws it. The row's numbers are the look's own, beside its shader, under the same word (render/lookTex.ts STUFFS; a check
//     holds the two lists equal). DRESSING: no rule of the game reads it.
//   · `does`  — what the thing is to a round, a charge and a blast: what a round knocks out of it and whether it goes through, which walls the wall engine
//     takes, how furniture of it breaks, what the destruction index finds of it, whether it is lying snow. GAMEPLAY: the rules read these columns and
//     nothing else of a material. No rule reads a paint, a paint's name or a colour.
// A BOX SAYS TWO WORDS AT MOST, and neither is ever worked out from the other: `stuff` (scene/stuff.ts, Box.stuff: the row it is DRAWN as) and `does`
// (Box.does: the row whose `does` it takes). They cannot be one word. A level and its look twin are one simulation under two pictures (the look's skin
// draws a plasterboard partition as a mud wall, and a charge must still cut it), and one paint is many things (what is drawn as 'paint' is a steel door,
// a timber locker and a concrete kerb to a blast). A row may hold one column or both: 'wood' is drawn as wood and breaks as wood.
// THE WORD IS ON THE BOX. Whoever MAKES a box writes it there once: the loader from the level file (the box's own `does`, else its material row's:
// levelLoad.ts), a kit from the paint it was handed (stuff.ts), the game from the box a piece came off (boxes.ts wordOf). At run time a rule finds the
// word on the box or takes THE DEFAULT ROW ('unnamed'), and asks nothing else. A level file, a recording and a checkpoint carry the WORD, never a row's
// place in this list.
// WHERE THE WORDS CAME FROM (2026-11-05). Until then the rules read a box's COLOUR: an exact match against the office's palette gave a paint's name, a
// paint of the 2010s look carried the name it stood for in a hair's shift of its colour, and a colour nobody had named was classed by how bright and how
// grey it was. Every row below is one answer that rule gave, under a plain name, and every level file was written again with the word its colour said, so
// no rule moved (tools/qa/does-audit.ts lists a box whose word and colour disagree; tools/levels/doesWas.ts keeps the old rule, frozen, to bring forward
// a box nobody has said a word for). So some rows are accidents kept: the 'unnamed' rows are tones, not stuffs. Each is a choice still owed, and making
// it is one edit here or one word in a generator.
// No imports but types: game/ reads this without pulling a kit, a palette or the renderer in.

/** what a round knocks out of a thing, and the class a wall of it is to the wall engine, a round and a charge */
export type Surf = 'metal' | 'plastic' | 'glass' | 'wood' | 'cardboard' | 'drywall' | 'concrete' | 'carpet' | 'fabric' | 'foliage';
export const SURFS: readonly Surf[] = ['metal', 'plastic', 'glass', 'wood', 'cardboard', 'drywall', 'concrete', 'carpet', 'fabric', 'foliage'];
/** how a blast breaks a piece of furniture (game/furnblast.ts FURN_BREAK has a row a class) */
export type BreakClass = 'wood' | 'board' | 'metal' | 'hard' | 'soft';
export const BREAK_CLASSES: readonly BreakClass[] = ['wood', 'board', 'metal', 'hard', 'soft'];
/** lying snow, by how trodden it is (render/snowField.ts, game/snowBog.ts) */
export type SnowKind = 'snow' | 'snowPacked' | 'snowOld';
export const SNOW_KINDS: readonly SnowKind[] = ['snow', 'snowPacked', 'snowOld'];

/** WHAT A MATERIAL DOES: every column a rule reads. An absent flag is false; an absent word is none. */
export interface Does {
  /** a round's hit (game/particleFx.ts), how tough a wall of it is (wallbreak.ts), what a round goes through (wallpen.ts), what a charge is told (wallcharge.ts) */
  surf: Surf;
  /** how a blast breaks furniture of it (furnblast.ts, winbars.ts) */
  breaks: BreakClass;
  /** sheet steel on a frame: a cabinet or a locker of it comes apart as sheets and a carcass (furnblast.ts steel) */
  steel?: true;
  /** the wall engine's partitions (wallbreak.ts): 'board' = a plasterboard wall where it stands full height, low cover where it does not; 'screen' = a
   *  cubicle's partition, a full wall the same way and, thin and low, cloth on a frame */
  wall?: 'board' | 'screen';
  /** plasterboard to a round: it goes through a thin run of it under a roof (wallpen.ts). A pale thing of no row is nobody's plasterboard */
  board?: true;
  /** low cover the wall engine breaks, and furniture a blast takes where it is a panel or a small carton (wallbreak.ts, furnblast.ts) */
  cover?: 'wood' | 'cardboard';
  /** the building, not its furniture: a blast leaves it standing and its mouth throws none of it (furnblast.ts) */
  struct?: true;
  /** … but low down it is a counter or an appliance, which the mouth of a hole does throw (furnblast.ts mouthMine) */
  counter?: true;
  /** a dark casing: a screen's housing, a monitor's foot and neck are found by it (destruction.ts index) */
  casing?: true;
  /** a small thin sheet of it lying on a desk or pinned to a wall is paper (destruction.ts index) */
  paper?: true;
  /** a thin slab of it is a screen or a television (destruction.ts index) */
  screen?: true;
  /** lying snow (snowBog.ts, airSim.ts, the blast's mouth, and the picture's snow field) */
  snow?: SnowKind;
  /** THE WORD IT TAKES WHILE SOAKED, for the one slab the weather soaks in place (the lot: game/weather.ts). Nothing is kept: whoever asks what a round
   *  knocks out of the lot while the ground is wet is answered from this (doesWet below). Absent = the same wet as dry. As measured: wet tarmac sparks. */
  wet?: string;
}
export interface Material { looks?: true; does?: Does }

/** the tarmac's columns, which lying snow shares (snow is the ground to a round and a blast; and a lot of either, soaked, sparks: as measured) */
const GROUND: Does = { surf: 'concrete', breaks: 'hard', struct: true };

export const MATERIALS = {
  // ---- DRAWN AND DONE under one word
  wood:    { looks: true, does: { surf: 'wood', breaks: 'wood', cover: 'wood' } },                 // timber: a desk, a crate, a pallet, a locker
  steel:   { looks: true, does: { surf: 'metal', breaks: 'metal', steel: true } },                // clean painted steel: shelving, a cabinet, a drum, a vehicle's body
  enamel:  { looks: true, does: { surf: 'metal', breaks: 'metal' } },                             // a small enamelled thing: an extinguisher
  fabric:  { looks: true, does: { surf: 'fabric', breaks: 'soft' } },                             // a seat's cloth, a tarpaulin, a sack, a blanket
  carpet:  { looks: true, does: { surf: 'carpet', breaks: 'hard', struct: true } },               // a fitted carpet, a rug
  grass:   { looks: true, does: { surf: 'carpet', breaks: 'hard', struct: true } },               // a lawn
  tarmac:  { looks: true, does: { ...GROUND, casing: true, wet: 'unnamed dark' } },               // the car park, a road, a yard's dust and gravel (`casing`: the office's tarmac is dark, and the index's rule was by tone)
  plastic: { looks: true, does: { surf: 'plastic', breaks: 'metal' } },                           // a crate, a chair's shell, a jerrycan
  // ---- DRAWN ONLY: the look's other materials. A box drawn as one of these says what it does in another row's word, or nothing
  'mud wall': { looks: true }, plaster: { looks: true }, block: { looks: true }, earth: { looks: true }, gravel: { looks: true }, slab: { looks: true }, sheet: { looks: true },
  cloth: { looks: true }, metal: { looks: true }, paint: { looks: true }, veneer: { looks: true }, carpaint: { looks: true }, stucco: { looks: true }, skim: { looks: true },
  'gloss block': { looks: true }, 'board concrete': { looks: true }, 'cast panel': { looks: true }, paver: { looks: true }, brushed: { looks: true }, louvre: { looks: true },
  // ---- DONE ONLY: what the office's paints were to the rules, by what each is
  plasterboard: { does: { surf: 'drywall', breaks: 'hard', wall: 'board', board: true, struct: true } },                       // an interior wall: a charge cuts it, a round goes through
  partition:    { does: { surf: 'drywall', breaks: 'hard', wall: 'screen', board: true, struct: true } },                      // a cubicle's partition: cloth over board
  ceiling:      { does: { surf: 'drywall', breaks: 'hard', board: true, struct: true, paper: true } },                         // ceiling board, a soffit
  laminate:     { does: { surf: 'drywall', breaks: 'hard', board: true, struct: true, counter: true, paper: true } },          // the office's white: a counter, an appliance, a whiteboard, a sheet of paper
  pane:         { does: { surf: 'drywall', breaks: 'hard', board: true, struct: true, paper: true } },                         // the paint of a pane of glass (a pane itself is the pane engine's, by its flag)
  concrete:     { does: { surf: 'concrete', breaks: 'hard', struct: true } },                                                  // structure: a shell, a column, a blast wall
  stone:        { does: { surf: 'concrete', breaks: 'hard', struct: true } },                                                  // a stone or tiled floor
  lino:         { does: { surf: 'concrete', breaks: 'hard', struct: true } },                                                  // a sheet floor
  'dark carpet': { does: { surf: 'carpet', breaks: 'hard', struct: true, casing: true } },                                     // a rug in a dark dye, and whatever small thing wears its paint (`casing`: by tone, as the index read it)
  card:         { does: { surf: 'cardboard', breaks: 'board', cover: 'cardboard' } },                                          // a carton
  casing:       { does: { surf: 'metal', breaks: 'hard', casing: true } },                                                     // the office's black: a rack, a bezel, a chair's base
  frame:        { does: { surf: 'metal', breaks: 'hard', struct: true } },                                                     // a window's frame, an outside door's leaf
  screen:       { does: { surf: 'glass', breaks: 'hard', casing: true, screen: true } },                                       // a dead screen's glass
  plant:        { does: { surf: 'foliage', breaks: 'soft' } },                                                                 // a pot plant, produce, straw
  snow:          { does: { ...GROUND, snow: 'snow', paper: true, wet: 'unnamed dark' } },                                                           // lying snow, fresh (`paper`: it is pale, and the index's rule was by tone)
  'packed snow': { does: { ...GROUND, snow: 'snowPacked', wet: 'unnamed dark' } },                                                  // trodden or driven snow
  'old snow':    { does: { ...GROUND, snow: 'snowOld', wet: 'unnamed dark' } },                                                    // slush, the dirty ring round a cleared patch
  // ---- DONE ONLY: WHAT NOBODY NAMED. A colour that was no paint of any palette was classed by its tone, and these are those classes: how bright (dark,
  // middling, light, pale) and whether a plain grey (a steel cabinet to a blast). 'unnamed' is THE DEFAULT ROW: a box that says nothing takes it
  unnamed:              { does: { surf: 'concrete', breaks: 'metal' } },
  'unnamed grey':       { does: { surf: 'concrete', breaks: 'metal', steel: true } },
  'unnamed dark':       { does: { surf: 'metal', breaks: 'metal', casing: true } },
  'unnamed light':      { does: { surf: 'drywall', breaks: 'metal' } },
  'unnamed light grey': { does: { surf: 'drywall', breaks: 'metal', steel: true } },
  'unnamed pale':       { does: { surf: 'drywall', breaks: 'metal', paper: true } },
  'unnamed pale grey':  { does: { surf: 'drywall', breaks: 'metal', steel: true, paper: true } },
  // … and the look's plastics and its dark glass, which were given a surface and classed by tone for the rest
  'dark plastic': { does: { surf: 'plastic', breaks: 'metal', casing: true } },   // black plastic, rubber
  'grey plastic': { does: { surf: 'plastic', breaks: 'metal', steel: true } },    // a white plastic chair, grey enough to be a cabinet
  'dark glass':   { does: { surf: 'glass', breaks: 'metal', casing: true } },     // solid dark glass: a vehicle's window
} as const satisfies Record<string, Material>;

export type MaterialWord = keyof typeof MATERIALS;
/** a word a box may say for what it does: a row that has the column */
export type DoesWord = { [K in MaterialWord]: typeof MATERIALS[K] extends { does: Does } ? K : never }[MaterialWord];
/** THE DEFAULT ROW's word: what a box that says nothing does. Never written on a box or in a file where nothing else speaks: absent means this */
export const DOES_DEFAULT: DoesWord = 'unnamed';

/** word → its `does`, made once from the table and never written again */
const ROWS: ReadonlyMap<string, Does> = new Map(Object.entries(MATERIALS as Record<string, Material>).flatMap(([w, m]): [string, Does][] => m.does ? [[w, m.does]] : []));
const DEFAULT: Does = ROWS.get(DOES_DEFAULT)!;
/** every word that has a `does`, in the table's order */
export const DOES_WORDS: readonly string[] = [...ROWS.keys()];
/** every word the look draws, in the table's order (a check holds it equal to the look's own list: render/lookTex.ts) */
export const LOOKS_WORDS: readonly string[] = Object.entries(MATERIALS as Record<string, Material>).filter(([, m]) => m.looks).map(([w]) => w);
export const doesWordOk = (v: unknown): v is DoesWord => typeof v === 'string' && ROWS.has(v);
/** the columns of a word (the default row's for a word the table does not have: a file opened leniently can carry any string) */
export const doesOfWord = (w: string | undefined): Does => (w !== undefined && ROWS.get(w)) || DEFAULT;
/** WHAT A BOX DOES: its own word's row, else the default row. The one question a rule asks of a material. */
export const doesOf = (b: { does?: string } | null | undefined): Does => (b && b.does !== undefined && ROWS.get(b.does)) || DEFAULT;
/** … and what it does WHILE SOAKED (the lot's slab under rain: Does.wet) */
export const doesWet = (b: { does?: string } | null | undefined): Does => { const d = doesOf(b); return d.wet !== undefined ? doesOfWord(d.wet) : d; };
/** the word a box carries for `w`: nothing for the default row (absent = default everywhere a box is kept) */
export const doesSaid = (w: string | undefined): { does?: DoesWord } => w !== undefined && w !== DOES_DEFAULT && doesWordOk(w) ? { does: w } : {};
/** THE `does` COLUMNS AS NUMBERS, a row a word, for a module that reads no strings (the Rust crate, when a reader moves there): the host hands this
 *  table over at boot and writes each box's row number from the same list in the same build. The numbers are places in SURFS, BREAK_CLASSES and
 *  SNOW_KINDS (+ 1; 0 = none); `wet` is a row number, the row's own where it is the same wet as dry. Never written to a file. */
export function doesRows(): { words: readonly string[]; cols: readonly string[]; rows: number[][] } {
  const cols = ['surf', 'breaks', 'steel', 'wall', 'board', 'cover', 'struct', 'counter', 'casing', 'paper', 'screen', 'snow', 'wet'];
  const rows = DOES_WORDS.map((w, i) => { const d = ROWS.get(w)!; return [SURFS.indexOf(d.surf), BREAK_CLASSES.indexOf(d.breaks), d.steel ? 1 : 0, d.wall === 'board' ? 1 : d.wall === 'screen' ? 2 : 0, d.board ? 1 : 0,
    d.cover === 'wood' ? 1 : d.cover === 'cardboard' ? 2 : 0, d.struct ? 1 : 0, d.counter ? 1 : 0, d.casing ? 1 : 0, d.paper ? 1 : 0, d.screen ? 1 : 0, d.snow ? 1 + SNOW_KINDS.indexOf(d.snow) : 0, d.wet !== undefined ? DOES_WORDS.indexOf(d.wet) : i]; });
  return { words: DOES_WORDS, cols, rows };
}
