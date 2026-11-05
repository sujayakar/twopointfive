// WHAT A BOX DID BY ITS COLOUR, UNTIL 2026-11-05: THE OLD RULE, FROZEN.
// The game's rules used to ask a box's COLOUR what it was (src/scene/materials.ts tells how). They ask the box's word now (Box.does), and this file keeps
// the old rule for two jobs and no other:
//   · BRINGING A BOX FORWARD. A box whose maker said no word is given, when its level is exported, the word its colour would have had (stampDoes below:
//     tools/levels/export.ts). So a generator that paints a thing in numbers nobody named still gets today's answer, written in the file where anyone
//     can read it and say a better one. Whoever names a paint (scene/stuff.ts) or says a box's word is never asked here.
//   · THE AUDIT (tools/qa/does-audit.ts): what the colour said of every box against what its word says.
// NOTHING IN src/ READS THIS, and nothing here reads the game: the palettes below are COPIES of the numbers as they stood that day (the office's rows, the
// 2010s look's and the snow kit's), so a palette may change its colours and this rule answers as it always did. The hair's shift by which a paint of the
// look stood for another office paint (gwotAs, to 0.0024 a channel) is here too, with its list in its old order: the files written before carry it.
import { DOES_DEFAULT, MATERIALS, doesOfWord, type Does, type DoesWord, type Surf, type BreakClass, type SnowKind } from '../../src/scene/materials.ts';

type V = readonly number[];
const OFFICE: Record<string, V> = { carpet: [0.13, 0.14, 0.17], carpetBlue: [0.09, 0.11, 0.2], carpetRed: [0.22, 0.07, 0.07], lino: [0.33, 0.33, 0.31], stone: [0.38, 0.37, 0.35], asphalt: [0.09, 0.09, 0.1], grass: [0.1, 0.16, 0.08], wall: [0.66, 0.64, 0.6], wallDark: [0.36, 0.38, 0.4], concrete: [0.42, 0.41, 0.4], ceiling: [0.72, 0.72, 0.7], partRed: [0.5, 0.13, 0.1], partBlue: [0.13, 0.17, 0.42], partTan: [0.55, 0.47, 0.35], wood: [0.52, 0.4, 0.27], woodDark: [0.25, 0.18, 0.12], metal: [0.3, 0.31, 0.33], black: [0.04, 0.04, 0.045], chair: [0.14, 0.2, 0.45], cardboard: [0.58, 0.44, 0.26], white: [0.8, 0.8, 0.78], plant: [0.12, 0.3, 0.1], glassFrame: [0.2, 0.22, 0.25], screenOff: [0.02, 0.02, 0.025], red: [0.55, 0.08, 0.06], glass: [0.8, 0.87, 0.83], glassExt: [0.7, 0.75, 0.79] };
const LOOK: Record<string, V> = { mud: [0.42, 0.32, 0.21], mudDark: [0.29, 0.22, 0.15], mudLight: [0.56, 0.46, 0.33], render: [0.57, 0.53, 0.45], whitewash: [0.72, 0.69, 0.61], block: [0.4, 0.39, 0.37], tWall: [0.46, 0.45, 0.43], paintBlue: [0.06, 0.19, 0.32], paintGreen: [0.08, 0.24, 0.16], paintRust: [0.3, 0.075, 0.04], paintCream: [0.66, 0.6, 0.44], dust: [0.205, 0.145, 0.085], dirt: [0.135, 0.09, 0.05], gravel: [0.15, 0.135, 0.115], mudFloor: [0.17, 0.115, 0.07], slab: [0.3, 0.285, 0.26], damp: [0.075, 0.052, 0.034], oil: [0.11, 0.095, 0.08], rugRed: [0.3, 0.052, 0.04], rugNavy: [0.04, 0.06, 0.15], rugCream: [0.6, 0.52, 0.38], rugOchre: [0.5, 0.33, 0.1], rugGreen: [0.09, 0.22, 0.13], canvas: [0.33, 0.31, 0.2], blanketGrey: [0.3, 0.3, 0.31], blanketBrown: [0.27, 0.18, 0.12], tarpBlue: [0.05, 0.17, 0.48], sack: [0.62, 0.58, 0.48], sandbag: [0.45, 0.39, 0.27], hesco: [0.47, 0.44, 0.36], olive: [0.15, 0.17, 0.09], coyote: [0.4, 0.31, 0.2], stencil: [0.62, 0.5, 0.1], rust: [0.28, 0.12, 0.06], iron: [0.098, 0.1, 0.112], galv: [0.5, 0.52, 0.53], tin: [0.56, 0.55, 0.52], drumBlue: [0.06, 0.15, 0.33], drumRed: [0.4, 0.07, 0.05], genYellow: [0.6, 0.42, 0.04], plasticWhite: [0.7, 0.7, 0.66], plasticGreen: [0.08, 0.28, 0.14], plasticBlue: [0.03, 0.12, 0.36], plasticBlack: [0.045, 0.045, 0.05], jerryYellow: [0.62, 0.46, 0.05], rubber: [0.03, 0.03, 0.033], poplar: [0.47, 0.36, 0.22], woodGrey: [0.33, 0.29, 0.23], ply: [0.55, 0.43, 0.27], truckWhite: [0.68, 0.68, 0.65], truckTan: [0.5, 0.42, 0.29], truckRed: [0.42, 0.06, 0.05], glassDark: [0.05, 0.06, 0.08], prodOrange: [0.7, 0.3, 0.04], prodGreen: [0.22, 0.4, 0.1], prodRed: [0.5, 0.07, 0.05], prodYellow: [0.7, 0.55, 0.1], straw: [0.51, 0.43, 0.22], scrub: [0.24, 0.26, 0.13], rugMadder: [0.1, 0.03, 0.035] };
const LOOK_IS: Record<string, string> = { mud: 'concrete', mudDark: 'concrete', mudLight: 'wall', render: 'concrete', whitewash: 'wall', block: 'concrete', tWall: 'concrete', paintBlue: 'metal', paintGreen: 'metal', paintRust: 'metal', paintCream: 'wall', dust: 'asphalt', dirt: 'asphalt', gravel: 'asphalt', mudFloor: 'stone', slab: 'concrete', damp: 'asphalt', oil: 'asphalt', rugRed: 'carpetRed', rugNavy: 'carpetBlue', rugCream: 'carpet', rugOchre: 'carpet', rugGreen: 'carpet', canvas: 'chair', blanketGrey: 'chair', blanketBrown: 'chair', tarpBlue: 'chair', sack: 'chair', sandbag: 'concrete', hesco: 'concrete', olive: 'woodDark', coyote: 'chair', stencil: 'metal', rust: 'metal', iron: 'metal', galv: 'metal', tin: 'metal', drumBlue: 'metal', drumRed: 'metal', genYellow: 'metal', plasticWhite: '~plastic', plasticGreen: '~plastic', plasticBlue: '~plastic', plasticBlack: '~plastic', jerryYellow: '~plastic', rubber: '~plastic', poplar: 'wood', woodGrey: 'wood', ply: 'wood', truckWhite: 'metal', truckTan: 'metal', truckRed: 'metal', glassDark: '~glass', prodOrange: 'plant', prodGreen: 'plant', prodRed: 'plant', prodYellow: 'plant', straw: 'plant', scrub: 'plant', rugMadder: 'carpetRed' };
const STOOD_FOR = ['carpet', 'carpetBlue', 'carpetRed', 'lino', 'stone', 'asphalt', 'grass', 'wall', 'wallDark', 'concrete', 'ceiling', 'partRed', 'partBlue', 'partTan', 'wood', 'woodDark', 'metal', 'black', 'chair', 'cardboard', 'white', 'plant', 'glassFrame', 'screenOff', 'red', '~metal', '~plastic', '~glass', '~wood', '~cardboard', '~drywall', '~concrete', '~carpet', '~fabric', '~foliage', '~steel', '~steelPale'];
const SNOW: Record<string, V> = { snow: [0.66, 0.71, 0.78], snowPacked: [0.43, 0.47, 0.53], snowOld: [0.27, 0.3, 0.34] };
/** what each snow paint stood for: the ground */
const SNOW_IS = 'asphalt';
const STEP = 0.0008;
const shifted = (a: V, code: number): number[] => [0, 1, 2].map(c => Math.round((a[c] + ((code >> (2 * c)) & 3) * STEP) * 1e4) / 1e4);
const WET = (a: V): number[] => [a[0] * 0.5, a[1] * 0.5, a[2] * 0.5 * 1.04];

/** an exact colour → the office paint's name (the first row of those numbers), as game/particleFx.ts paletteName built it */
const officeBy = new Map<string, string>(); for (const [k, a] of Object.entries(OFFICE)) { const key = a.join(','); if (!officeBy.has(key)) officeBy.set(key, k); }
/** an exact colour of the look's → what it stood for, as scene/gwotPalette.ts gwotSimOf built it, in its order */
const lookBy = new Map<string, string>();
for (const m of Object.keys(LOOK)) STOOD_FOR.forEach((s, i) => lookBy.set(shifted(LOOK[m], 1 + i).join(','), s));
for (const m of Object.keys(LOOK)) lookBy.set(LOOK[m].join(','), LOOK_IS[m]);
for (const m of Object.keys(LOOK)) { const a = LOOK_IS[m] === 'asphalt' ? LOOK[m] : shifted(LOOK[m], 1 + STOOD_FOR.indexOf('asphalt')), k = WET(a).join(','); if (!lookBy.has(k)) lookBy.set(k, '~metal'); }
for (const m of Object.keys(SNOW)) { const k = SNOW[m].join(','); if (!lookBy.has(k)) lookBy.set(k, SNOW_IS); }
for (const m of Object.keys(SNOW)) { const k = WET(SNOW[m]).join(','); if (!lookBy.has(k)) lookBy.set(k, '~metal'); }   // (… and soaked, a snow paint said what every wet lot said)

const MAT_SURF: Record<string, Surf> = {
  metal: 'metal', glassFrame: 'metal', black: 'metal', red: 'metal', screenOff: 'glass',
  wood: 'wood', woodDark: 'wood', cardboard: 'cardboard', chair: 'fabric', plant: 'foliage',
  wall: 'drywall', white: 'drywall', ceiling: 'drywall', partRed: 'drywall', partBlue: 'drywall', partTan: 'drywall',
  concrete: 'concrete', wallDark: 'concrete', stone: 'concrete', asphalt: 'concrete', lino: 'concrete',
  carpet: 'carpet', carpetBlue: 'carpet', carpetRed: 'carpet', grass: 'carpet',
};
const STRUCT = new Set(['carpet', 'carpetBlue', 'carpetRed', 'lino', 'stone', 'asphalt', 'grass', 'wall', 'wallDark', 'concrete', 'ceiling', 'white', 'glassFrame', 'glass', 'glassExt', 'partRed', 'partBlue', 'partTan']);
const PART = new Set(['partRed', 'partBlue', 'partTan']);
const STEEL_GREY = { chroma: 0.07, value: [0.3, 0.8] };
const SCREEN_OFF = [0.02, 0.02, 0.025];
export const lum = (a: V): number => 0.2126 * a[0] + 0.7152 * a[1] + 0.0722 * a[2];
const near = (a: V, b: V, e: number): boolean => Math.abs(a[0] - b[0]) < e && Math.abs(a[1] - b[1]) < e && Math.abs(a[2] - b[2]) < e;
const near3 = near;   // (destruction.ts had its own, the same test)

/** WHAT THE RULES MADE OF A COLOUR, each as the code that read it wrote it (the file and the function are named a line) */
export interface Was {
  /** particleFx.ts paletteName: the office paint's name, its own or the one a look's paint stood for */
  name: string | undefined;
  /** particleFx.ts surfOf */
  surf: Surf;
  /** furnblast.ts breakClassOf */
  breaks: BreakClass;
  /** furnblast.ts steelGrey */
  steel: boolean;
  /** wallbreak.ts index, wallpen.ts stuffOf: the wall paints */
  wall: 'board' | 'screen' | undefined;
  /** wallpen.ts stuffOf: a named paint that is plasterboard */
  board: boolean;
  /** wallbreak.ts index (cover), furnblast.ts (a panel, a carton) */
  cover: 'wood' | 'cardboard' | undefined;
  /** furnblast.ts STRUCT; and its one exception, white */
  struct: boolean; counter: boolean;
  /** destruction.ts index: not over 0.1 bright; over 0.7; the dead screen's colour to 0.004 */
  casing: boolean; paper: boolean; screen: boolean;
  /** render/snowField.ts snowPaintOf: a snow paint to 0.002 */
  snow: SnowKind | undefined;
}
export function wasOf(albedo: V): Was {
  const key = albedo.join(','), look = lookBy.get(key);
  const name = officeBy.get(key) ?? (look !== undefined && look[0] !== '~' ? look : undefined);
  const L = lum(albedo);
  const surf: Surf = name !== undefined && MAT_SURF[name] ? MAT_SURF[name]
    : name === undefined && look !== undefined && look[0] === '~' ? (look === '~steel' ? 'concrete' : look === '~steelPale' ? 'drywall' : look.slice(1) as Surf)
    : L > 0.5 ? 'drywall' : L < 0.1 ? 'metal' : 'concrete';
  const breaks: BreakClass = name === undefined ? 'metal' : name === 'wood' || name === 'woodDark' ? 'wood' : name === 'cardboard' ? 'board' : name === 'metal' || name === 'red' ? 'metal' : name === 'chair' || name === 'plant' ? 'soft' : 'hard';
  // the paint a rule that read the paint itself read: the office paint a look's paint stood for
  const stands: V = look === undefined ? albedo : look === '~steel' ? [0.36, 0.36, 0.36] : look === '~steelPale' ? [0.58, 0.58, 0.58] : look[0] === '~' ? albedo : OFFICE[look] ?? albedo;
  const hi = Math.max(stands[0], stands[1], stands[2]), lo = Math.min(stands[0], stands[1], stands[2]);
  const steel = !((name !== undefined && name !== 'metal') || hi - lo > STEEL_GREY.chroma || hi < STEEL_GREY.value[0] || hi > STEEL_GREY.value[1]);
  const snow: SnowKind | undefined = near(albedo, SNOW.snow, 2e-3) ? 'snow' : near(albedo, SNOW.snowPacked, 2e-3) ? 'snowPacked' : near(albedo, SNOW.snowOld, 2e-3) ? 'snowOld' : undefined;
  return { name, surf, breaks, steel, wall: name === 'wall' ? 'board' : name !== undefined && PART.has(name) ? 'screen' : undefined, board: name !== undefined && surf === 'drywall',
    cover: name === 'wood' || name === 'woodDark' ? 'wood' : name === 'cardboard' ? 'cardboard' : undefined, struct: name !== undefined && STRUCT.has(name), counter: name === 'white',
    casing: !(L > 0.1), paper: L > 0.7, screen: near3(albedo, SCREEN_OFF, 0.004), snow };
}
/** is this colour, to the number, a row of the look's palette or a snow paint AS THEY STOOD (not a stand-in a hair off one)? */
export const lookRowWas = (albedo: V): boolean => { const k = albedo.join(','); return Object.values(LOOK).some(a => a.join(',') === k) || Object.values(SNOW).some(a => a.join(',') === k); };
/** … and of the lot's slab while the weather had soaked it (game/weather.ts setWet halved its colour in place, and the rules read that) */
export const wasWetOf = (albedo: V): Was => wasOf(WET(albedo));

/** the row each office paint's name is (scene/materials.ts) */
const NAME_ROW: Record<string, DoesWord> = {
  wall: 'plasterboard', partRed: 'partition', partBlue: 'partition', partTan: 'partition', ceiling: 'ceiling', white: 'laminate', glass: 'pane', glassExt: 'pane',
  concrete: 'concrete', wallDark: 'concrete', stone: 'stone', lino: 'lino', asphalt: 'tarmac', carpet: 'carpet', carpetBlue: 'carpet', carpetRed: 'carpet', grass: 'grass',
  wood: 'wood', woodDark: 'wood', cardboard: 'card', metal: 'steel', red: 'enamel', black: 'casing', glassFrame: 'frame', screenOff: 'screen', chair: 'fabric', plant: 'plant',
};
const SNOW_ROW: Record<SnowKind, DoesWord> = { snow: 'snow', snowPacked: 'packed snow', snowOld: 'old snow' };
/** THE WORD A COLOUR HAD: the row of the table of materials that says what the old rule said of it. A named paint's row by its name (lying snow by its
 *  kind); a colour nobody named, the 'unnamed' row of its surface, its tone and whether it is a plain grey. Where no row holds every column (a tone the
 *  table has no row for) the nearest is given: the audit lists each such box and says whether a rule could tell. */
export function wordWas(albedo: V): DoesWord {
  const w = wasOf(albedo);
  if (w.name !== undefined) { const row = w.snow !== undefined && w.name === SNOW_IS ? SNOW_ROW[w.snow] : NAME_ROW[w.name] ?? DOES_DEFAULT; return row === 'carpet' && w.casing ? 'dark carpet' : row; }   // (one named paint the index could tell dark from light: a small thing in a dark rug's dye)
  if (w.surf === 'plastic') return w.steel ? 'grey plastic' : w.casing ? 'dark plastic' : 'plastic';
  if (w.surf === 'glass') return 'dark glass';
  if (w.surf === 'metal') return 'unnamed dark';
  if (w.surf === 'drywall') return w.paper ? (w.steel ? 'unnamed pale grey' : 'unnamed pale') : w.steel ? 'unnamed light grey' : 'unnamed light';
  return w.steel ? 'unnamed grey' : 'unnamed';
}
/** the columns of a word against what the colour said: the names of the columns that differ ('' = none). `tone` = ask the three the destruction index
 *  read by tone and by the dead screen's colour too (the caller knows whether the box is one the index could have looked at) */
export function differs(d: Does, w: Was, tone: { casing: boolean; paper: boolean; screen: boolean }): string[] {
  const out: string[] = [];
  if (d.surf !== w.surf) out.push(`surf ${w.surf} → ${d.surf}`);
  if (d.breaks !== w.breaks) out.push(`breaks ${w.breaks} → ${d.breaks}`);
  if (!!d.steel !== w.steel) out.push(`steel ${w.steel} → ${!!d.steel}`);
  if (d.wall !== w.wall) out.push(`wall ${w.wall} → ${d.wall}`);
  if (!!d.board !== w.board) out.push(`board ${w.board} → ${!!d.board}`);
  if (d.cover !== w.cover) out.push(`cover ${w.cover} → ${d.cover}`);
  if (!!d.struct !== w.struct) out.push(`struct ${w.struct} → ${!!d.struct}`);
  if (!!d.counter !== w.counter) out.push(`counter ${w.counter} → ${!!d.counter}`);
  if (d.snow !== w.snow) out.push(`snow ${w.snow} → ${d.snow}`);
  if (tone.casing && !!d.casing !== w.casing) out.push(`casing ${w.casing} → ${!!d.casing}`);
  if (tone.paper && !!d.paper !== w.paper) out.push(`paper ${w.paper} → ${!!d.paper}`);
  if (tone.screen && !!d.screen !== w.screen) out.push(`screen ${w.screen} → ${!!d.screen}`);
  return out;
}

type Said = { albedo?: V; does?: string };
/** every thing of a level that wears a paint of its own: its boxes, a shape's pieces that say a paint, its props' parts */
function* painted(level: { boxes: Said[]; props?: { parts: Said[] }[] }): Generator<Said> {
  for (const b of level.boxes) { yield b; for (const w of ((b as { shape?: { with?: Said[] } }).shape?.with ?? [])) if (w.albedo) yield w; }
  for (const p of level.props ?? []) for (const q of p.parts) yield q;
}
const stamped = new WeakSet<object>();
/** GIVE EVERY BOX NOBODY SPOKE FOR THE WORD ITS COLOUR HAD (the exporter, before a look's skin repaints anything and again after it for what the skin
 *  added). A box that already says a word keeps it; one that says the default row's word aloud ends saying nothing, which is the same. Asked once a box:
 *  a box seen here is never asked again, so a repaint after it cannot change what the box does. Returns how many it gave a word. */
export function stampDoes(level: { boxes: Said[]; props?: { parts: Said[] }[] }): number {
  let n = 0;
  for (const o of painted(level)) {
    if (stamped.has(o)) continue; stamped.add(o);
    if (o.does === undefined) { const w = wordWas(o.albedo!); if (w !== DOES_DEFAULT) { o.does = w; n++; } }
    else if (o.does === DOES_DEFAULT) delete o.does;
  }
  return n;
}
export { MATERIALS, doesOfWord };
