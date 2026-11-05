// WHAT A BOX WAS DRAWN AS BEFORE IT HAD TO SAY SO: the look's rule up to 2026-11-04, FROZEN.
// Until then the look decided what a box is made of in three steps (render/gbuffer.ts lookWordOf, lookNamed, lookStuff, as they were): the box's own word;
// else the NAME of the first row of the level file that has the box's colour, by the table of names, unless that name was one of six that are more than one
// thing; else its COLOUR, matched against the 2010s palette exactly and then darkened in one piece. Nothing in src/ does that any more (scene/stuff.ts: what a
// thing is made of is said). This file keeps the old rule, with the tables it read copied in as they stood, for two jobs and no other:
//   · tools/levels/stuff-bake.ts brings a level file written before forward: every box says what it WAS drawn as, so the picture does not move;
//   · tools/qa/stuff-audit.ts prints what every box of every level was and is, so that "nothing moved" is a count and not a claim.
// NEVER EDIT THE TABLES BELOW to follow the game: they are a record. When no level file from before is left to bring forward, both tools and this go.

type Vec = readonly number[];
/** how near a paint was its palette row, and how far a classed paint could be darkened in one piece and still be that stuff */
const MATCH = { near: 0.006, darkLo: 0.55, darkHi: 0.99 };
/** the 2010s palette as it stood, in its own order: name, colour, and the stuff its colour was drawn as (null = none: plain) */
const PALETTE: readonly (readonly [string, number, number, number, string | null])[] = [
  ['mud', 0.42, 0.32, 0.21, 'mud wall'], ['mudDark', 0.29, 0.22, 0.15, 'mud wall'], ['mudLight', 0.56, 0.46, 0.33, 'mud wall'], ['render', 0.57, 0.53, 0.45, 'plaster'],
  ['whitewash', 0.72, 0.69, 0.61, 'plaster'], ['block', 0.4, 0.39, 0.37, 'block'], ['tWall', 0.46, 0.45, 0.43, 'block'], ['paintBlue', 0.06, 0.19, 0.32, 'paint'],
  ['paintGreen', 0.08, 0.24, 0.16, 'paint'], ['paintRust', 0.3, 0.075, 0.04, 'paint'], ['paintCream', 0.66, 0.6, 0.44, 'plaster'], ['dust', 0.205, 0.145, 0.085, 'earth'],
  ['dirt', 0.135, 0.09, 0.05, 'earth'], ['gravel', 0.15, 0.135, 0.115, 'gravel'], ['mudFloor', 0.17, 0.115, 0.07, 'earth'], ['slab', 0.3, 0.285, 0.26, 'slab'],
  ['damp', 0.075, 0.052, 0.034, 'earth'], ['oil', 0.11, 0.095, 0.08, null], ['rugRed', 0.3, 0.052, 0.04, 'cloth'], ['rugNavy', 0.04, 0.06, 0.15, 'cloth'],
  ['rugCream', 0.6, 0.52, 0.38, 'cloth'], ['rugOchre', 0.5, 0.33, 0.1, 'cloth'], ['rugGreen', 0.09, 0.22, 0.13, 'cloth'], ['canvas', 0.33, 0.31, 0.2, 'cloth'],
  ['blanketGrey', 0.3, 0.3, 0.31, 'cloth'], ['blanketBrown', 0.27, 0.18, 0.12, 'cloth'], ['tarpBlue', 0.05, 0.17, 0.48, 'cloth'], ['sack', 0.62, 0.58, 0.48, 'cloth'],
  ['sandbag', 0.45, 0.39, 0.27, 'cloth'], ['hesco', 0.47, 0.44, 0.36, 'cloth'], ['olive', 0.15, 0.17, 0.09, 'paint'], ['coyote', 0.4, 0.31, 0.2, 'cloth'],
  ['stencil', 0.62, 0.5, 0.1, null], ['rust', 0.28, 0.12, 0.06, 'metal'], ['iron', 0.098, 0.1, 0.112, 'metal'], ['galv', 0.5, 0.52, 0.53, 'sheet'],
  ['tin', 0.56, 0.55, 0.52, 'sheet'], ['drumBlue', 0.06, 0.15, 0.33, 'paint'], ['drumRed', 0.4, 0.07, 0.05, 'paint'], ['genYellow', 0.6, 0.42, 0.04, 'paint'],
  ['plasticWhite', 0.7, 0.7, 0.66, null], ['plasticGreen', 0.08, 0.28, 0.14, null], ['plasticBlue', 0.03, 0.12, 0.36, null], ['plasticBlack', 0.045, 0.045, 0.05, null],
  ['jerryYellow', 0.62, 0.46, 0.05, null], ['rubber', 0.03, 0.03, 0.033, null], ['poplar', 0.47, 0.36, 0.22, 'wood'], ['woodGrey', 0.33, 0.29, 0.23, 'wood'],
  ['ply', 0.55, 0.43, 0.27, 'wood'], ['truckWhite', 0.68, 0.68, 0.65, 'paint'], ['truckTan', 0.5, 0.42, 0.29, 'paint'], ['truckRed', 0.42, 0.06, 0.05, 'paint'],
  ['glassDark', 0.05, 0.06, 0.08, null], ['prodOrange', 0.7, 0.3, 0.04, null], ['prodGreen', 0.22, 0.4, 0.1, null], ['prodRed', 0.5, 0.07, 0.05, null],
  ['prodYellow', 0.7, 0.55, 0.1, null], ['straw', 0.51, 0.43, 0.22, null], ['scrub', 0.24, 0.26, 0.13, null], ['rugMadder', 0.1, 0.03, 0.035, 'cloth'],
];
/** the order the darkened test tried the classed rows in (a darkened paint was the FIRST row it fitted) */
const DARK_ORDER = 'mud mudDark mudLight render whitewash paintCream block tWall dust dirt mudFloor damp gravel slab poplar woodGrey ply galv tin sack sandbag hesco canvas tarpBlue blanketGrey blanketBrown rugRed rugNavy rugCream rugOchre rugGreen rugMadder coyote rust iron paintBlue paintGreen paintRust drumBlue drumRed genYellow truckWhite truckTan truckRed olive'.split(' ');
/** what each paint NAME said (the table of names as it stood; 'mixed' = one name, more than one thing: left to the colour) */
const NAMES: Readonly<Record<string, string>> = {
  mud: 'mud wall', mudDark: 'mud wall', mudLight: 'mud wall', render: 'plaster', whitewash: 'plaster', paintCream: 'plaster', block: 'block', tWall: 'block', dust: 'earth', dirt: 'earth',
  mudFloor: 'earth', damp: 'earth', gravel: 'gravel', slab: 'slab', poplar: 'wood', woodGrey: 'wood', ply: 'wood', galv: 'sheet', tin: 'sheet', sack: 'cloth', sandbag: 'cloth', hesco: 'cloth',
  canvas: 'cloth', tarpBlue: 'cloth', blanketGrey: 'cloth', blanketBrown: 'cloth', rugRed: 'cloth', rugNavy: 'cloth', rugCream: 'cloth', rugOchre: 'cloth', rugGreen: 'cloth', rugMadder: 'cloth',
  coyote: 'cloth', rust: 'metal', iron: 'metal', paintBlue: 'paint', paintGreen: 'paint', paintRust: 'paint', drumBlue: 'paint', drumRed: 'paint', genYellow: 'paint', truckWhite: 'paint',
  truckTan: 'paint', truckRed: 'paint', olive: 'paint', oil: 'plain', stencil: 'plain', plasticWhite: 'plastic', plasticGreen: 'plastic', plasticBlue: 'plastic', plasticBlack: 'plastic',
  jerryYellow: 'plain', rubber: 'plain', glassDark: 'plain', prodOrange: 'plain', prodGreen: 'plain', prodRed: 'plain', prodYellow: 'plain', straw: 'waits', scrub: 'waits', wood: 'wood',
  woodDark: 'wood', carpet: 'cloth', carpetBlue: 'cloth', carpetRed: 'cloth', chair: 'fabric', wall: 'plaster', ceiling: 'plaster', wallDark: 'block', concrete: 'block', stone: 'slab',
  lino: 'slab', black: 'plain', white: 'plain', screenOff: 'plain', glass: 'plain', glassExt: 'plain', cardboard: 'waits', plant: 'waits', grass: 'waits', metal: 'steel', partRed: 'mixed',
  partBlue: 'mixed', partTan: 'mixed', red: 'mixed', asphalt: 'mixed', glassFrame: 'mixed', snow: 'plain', snowOld: 'plain', snowPacked: 'plain',
};
const ROW = new Map(PALETTE.map(r => [r[0], r] as const));

/** HOW a box got its stuff under the old rule */
export type WasBy = 'own' | 'name' | 'colour' | 'darkened' | 'none';
export interface Was { /** the word it was drawn as ('plain' = none) */ stuff: string; by: WasBy; /** the palette row its colour was matched to, where it was */ row?: string }

/** what a COLOUR alone was drawn as: the palette's own row first (a stand-in is that row), then a classed paint darkened in one piece */
export function wasByColour(a: Vec): Was {
  const e = MATCH.near, near = (p: Vec, k: number) => Math.abs(a[0] - k * p[0]) < e && Math.abs(a[1] - k * p[1]) < e && Math.abs(a[2] - k * p[2]) < e;
  for (const r of PALETTE) if (near([r[1], r[2], r[3]], 1)) return r[4] ? { stuff: r[4], by: 'colour', row: r[0] } : { stuff: 'plain', by: 'none', row: r[0] };
  for (const m of DARK_ORDER) { const r = ROW.get(m)!, p = [r[1], r[2], r[3]], k = (a[0] * p[0] + a[1] * p[1] + a[2] * p[2]) / (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]);
    if (k > MATCH.darkLo && k < MATCH.darkHi && near(p, k)) return { stuff: r[4]!, by: 'darkened', row: m }; }
  return { stuff: 'plain', by: 'none' };
}
/** the key a colour was kept under (13 bits a channel), or -1 for one out of range */
const paintKey = (a: Vec): number => { const r = Math.round(a[0] * 8192), g = Math.round(a[1] * 8192), b = Math.round(a[2] * 8192); return r >= 0 && r < 16384 && g >= 0 && g < 16384 && b >= 0 && b < 16384 ? r + 16384 * (g + 16384 * b) : -1; };
type Rows = Readonly<Record<string, { albedo: Vec }>>;
/** a level's rows as the old rule read them: each colour's FIRST row spoke for that colour, if the table knew its name as one thing */
function named(rows: Rows): Map<number, string> {
  const out = new Map<number, string>(), seen = new Set<number>();
  for (const [name, row] of Object.entries(rows)) { const key = paintKey(row.albedo); if (key < 0 || seen.has(key)) continue; seen.add(key);
    const s = Object.hasOwn(NAMES, name) ? NAMES[name] : undefined; if (s !== undefined && s !== 'mixed') out.set(key, s); }
  return out;
}
/** WHAT A LEVEL FILE'S BOXES AND PARTS WERE DRAWN AS: `was(o)` for a box or a prop's part as the file writes it (`material` or `albedo`, maybe `stuff`) */
export function wasOf(rows: Rows): (o: { material?: string; albedo?: Vec; stuff?: string }) => Was {
  const names = named(rows);
  return o => {
    if (o.stuff !== undefined) return { stuff: o.stuff, by: 'own' };
    const a = o.material !== undefined ? rows[o.material].albedo : o.albedo!, byName = names.get(paintKey(a));
    return byName !== undefined ? { stuff: byName, by: 'name' } : wasByColour(a);
  };
}
/** two words that draw alike: the same word, or both no material (plain and waits) */
export const drawnAlike = (a: string | undefined, b: string | undefined): boolean => { const n = (w: string | undefined) => w === undefined || w === 'waits' ? 'plain' : w; return n(a) === n(b); };
