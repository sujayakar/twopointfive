// A LEVEL THE GAME CAN HOLD — THE ONE COUNT. The rules of the things keep their numbers in pools of a fixed size (the Rust module's rows: a row a door, a
// row a thing of loose furniture, a row a pane of glass that is no longer whole), and the box world is copied into tables of a fixed size (rust/sim/src/
// world.rs; src/scene/boxMirror.ts). There is no other step for the doors, the furniture or the glass of a bigger level, and no other answer to a line
// through a level the copy cannot hold. So:
//   levelOver(level)   what a level SAYS of itself — doors, loose things, the boxes those are made of, sheets of glass — read BEFORE anything of it is
//                      built; a level over one is not loaded (the first statement of Game's constructor), and scene/levelData.ts's validator says the
//                      same of the document whole.
//   fitRows(game, …)   every count of a STOOD game against what holds it: those four, and every table of the box world's copy (statics, the floor grid's
//                      cells and their entries, tilted statics, cut rows, leaves, the glass a line can note). tools/qa/copyfit-check.ts reads every
//                      level of levels/ by it, and is red when the copy has a table that no row here counts: a table added to the copy is a row added
//                      HERE.
//   handOverMost()     what ONE STEP of a rule is handed of the game, at most, against what the step's hand-over holds: the men at the doors and at the
//                      furniture, by the men a game holds at once (`menMost`: the picture's own budget of characters, and the other seats' men). So a
//                      step handed more than its tables hold is no case of play: it cannot happen, the rows below say why, and where it is met it
//                      is a mistake said by name (doors.ts there, props.ts there).
// (The pools themselves refuse more rows than they have: sim/doorshome.ts, sim/slabshome.ts, sim/propshome.ts. This is the same statement, made early.)
import { BoxFlag, type Box } from '../scene/boxes';
import { PANE_CAP } from '../scene/boxMirror';
import { simWhyNot } from '../sim/sim';
import { DOOR_CAP, doorIo } from '../sim/doorshome';
import { PROP_CAP, PROP_PARTS, propIo } from '../sim/propshome';
import { isSheet } from './glass';
import { MAX_SKINNED } from './guards';
import { SEATS } from './seats';
import { BODY_DOOR_PARTICLES } from './game';

/** MEN A GAME HOLDS AT ONCE, AT MOST: the first seat's man and the guards and units (guards.ts spawnUnit refuses one more than MAX_SKINNED of those, the
 *  picture's own budget, whoever asks), its `civilians` (the level's own night people, a rescue's hostages, the man a snatch is for: each joins the game
 *  in one place, which holds them to `civMost`) and every other seat's man */
export const menMost = (civilians: number): number => MAX_SKINNED + civilians + SEATS.max - 1;
/** MEN ONE STEP CAN BE HANDED: the doors' hand-over in contacts, by the contacts one man is at most (`handOverMost`), and the furniture's in men. Infinity
 *  where there is no module: no hand-over to hold to. */
export const menHeld = (): number => simWhyNot() ? Infinity : Math.min(Math.floor(doorIo().maxContacts / (BODY_DOOR_PARTICLES.length + 1)), propIo().maxContacts);
/** CIVILIANS A GAME HOLDS AT ONCE, AT MOST: what a step's hand-over leaves after every other man a game may hold. civilians.ts `join`, the one place a
 *  civilian comes into a game, refuses one more — so the arithmetic holds whoever adds a civilian, and when. */
export const civMost = (): number => menHeld() - menMost(0);
/** WHAT ONE STEP IS HANDED OF THEM, AT MOST. At the doors a man is one contact, two while he lies wounded, and — his body near a doorway — a few of his
 *  ragdoll's particles (game.ts BODY_DOOR_PARTICLES): never more than those and his own. At the furniture a man is one. */
export const handOverMost = (civilians: number): { doorMen: number; propMen: number } => ({ doorMen: menMost(civilians) * (BODY_DOOR_PARTICLES.length + 1), propMen: menMost(civilians) });

/** what of a level is counted before it is built, and what it has of each: doors · loose things of furniture · the boxes those are made of · sheets of glass */
export interface LevelFit { doors: number; things: number; parts: number; sheets: number }
/** …and how many of each the game holds */
export const levelCaps = (): LevelFit => ({ doors: DOOR_CAP, things: PROP_CAP, parts: PROP_PARTS, sheets: PANE_CAP });
export function levelCounts(level: { doors: readonly unknown[]; props: readonly { parts: readonly unknown[] }[]; boxes: readonly Box[] }): LevelFit {
  let parts = 0, sheets = 0; for (const p of level.props) parts += p.parts.length; for (const b of level.boxes) if (isSheet(b)) sheets++;
  return { doors: level.doors.length, things: level.props.length, parts, sheets };
}
/** why this level cannot be loaded, in words, or null: it fits */
export function levelOver(level: Parameters<typeof levelCounts>[0]): string | null {
  const has = levelCounts(level), cap = levelCaps(), over: string[] = [];
  if (has.doors > cap.doors) over.push(`${has.doors} doors (the game holds ${cap.doors})`);
  if (has.things > cap.things) over.push(`${has.things} loose things of furniture (${cap.things})`);
  if (has.parts > cap.parts) over.push(`loose furniture of ${has.parts} boxes (${cap.parts})`);
  if (has.sheets > cap.sheets) over.push(`${has.sheets} sheets of glass (${cap.sheets})`);
  return over.length ? `this level is too big to load: ${over.join(', ')}` : null;
}

/** ONE ROW OF THE COUNT: what is counted, how many a stood game has, how many fit, and the share of that a level may fill AS IT LOADS (1, or less for a
 *  table that grows in play: a wall holed stands back up as pieces, each a static, an entry of its cells and a cut row). `table`: the table of the box
 *  world's copy the row is held by, by BoxMirror's own name for it (null: a pool of the rules', which is no table of the copy). */
export interface FitRow { row: string; what: string; has: number; cap: number; room: number; table: string | null }
/** what `fitRows` reads of a stood game (a Game is one) */
interface Stood {
  guards?: readonly unknown[]; civilians?: readonly unknown[];
  level: Parameters<typeof levelCounts>[0] & { civilians?: readonly unknown[] };
  col: { boxes: readonly Box[]; W: number; H: number; cells: readonly (readonly number[])[]; tiltedAt: readonly number[]; dynamicBoxes: readonly Box[]; mirror: { cutRows: number } };
  props?: { all: readonly { boxes: readonly unknown[] }[] } | null;
}
/** EVERY COUNT of a stood game against what holds it. `tables` = the copy's own sizes (BoxMirror.tables, after a tick's open has asked the module). */
export function fitRows(g: Stood, tables: Readonly<Record<string, number>>): FitRow[] {
  const col = g.col, L = levelCounts(g.level), cap = levelCaps(), t = (k: string) => tables[k] ?? 0, civs = Math.max(g.civilians?.length ?? 0, g.level.civilians?.length ?? 0), most = handOverMost(civs), held = !simWhyNot(), dio = held ? doorIo() : null, pio = held ? propIo() : null;   // (no module: no hand-over to hold to, and every row of one reads Infinity)
  let entries = 0, glass = 0, parts = 0;
  for (const c of col.cells) entries += c.length;
  for (const b of col.boxes) if (b.flags & BoxFlag.Glass) glass++;
  for (const p of g.props?.all ?? []) parts += p.boxes.length;
  return [
    { row: 'boxes', what: 'statics', has: col.boxes.length, cap: t('boxes'), room: 0.5, table: 'boxes' },
    { row: 'cells', what: 'cells of the floor grid', has: col.W * col.H, cap: t('cells'), room: 1, table: 'cells' },
    { row: 'items', what: 'entries of those cells', has: entries, cap: t('items'), room: 0.5, table: 'items' },
    { row: 'tilted', what: 'tilted statics', has: col.tiltedAt.length / 5, cap: t('tilted'), room: 1, table: 'tilted' },
    { row: 'cuts', what: 'cut rows (a cut or a shaped static takes one; a blast\'s pieces are cut boxes)', has: col.mirror.cutRows, cap: t('cuts'), room: 0.5, table: 'cuts' },
    { row: 'leaves', what: 'leaves (the doors\' list of the leaves\' table)', has: col.dynamicBoxes.length, cap: t('leaves'), room: 1, table: 'leaves' },
    { row: 'parts', what: 'boxes of the loose furniture (another list of the leaves\' table)', has: parts, cap: Math.min(t('leaves'), cap.parts), room: 1, table: 'leaves' },
    { row: 'things', what: 'loose things of furniture', has: g.props?.all.length ?? 0, cap: Math.min(t('things'), cap.things), room: 1, table: 'things' },
    { row: 'panes', what: 'glass one line can note (no more than the level\'s glass)', has: glass, cap: t('panes'), room: 1, table: 'panes' },
    { row: 'sheets', what: 'sheets of glass (a row a pane that is no longer whole)', has: L.sheets, cap: Math.min(t('glass'), cap.sheets), room: 1, table: 'glass' },
    { row: 'view', what: 'sheets of glass (a row a pane a burst\'s view names)', has: L.sheets, cap: t('view'), room: 1, table: 'view' },
    { row: 'doors', what: 'doors (a row a door, and a row a leaf off its hinges)', has: L.doors, cap: cap.doors, room: 1, table: null },
    // what a game holds of men, and what one step of a rule is handed of them (the rules' hand-overs: no table of the copy)
    { row: 'men', what: 'the first seat\'s man and the guards as the level is stood up (what guards.ts spawnUnit counts against the picture\'s budget of characters)', has: 1 + (g.guards?.length ?? 0), cap: MAX_SKINNED, room: 1, table: null },
    { row: 'civilians', what: 'civilians: the level\'s own, or those in the game as it stands (a rescue\'s hostages and the man a snatch is for join in play; civilians.ts join holds every one to this number as he joins)', has: civs, cap: civMost(), room: 1, table: null },
    { row: 'doormen', what: 'contacts one step of the doors is handed at most (every man a game holds: himself, and his body\'s particles near a doorway)', has: most.doorMen, cap: dio?.maxContacts ?? Infinity, room: 1, table: null },
    { row: 'propmen', what: 'men one step of the furniture is handed at most', has: most.propMen, cap: pio?.maxContacts ?? Infinity, room: 1, table: null },
    { row: 'frames', what: 'door frames one step of the furniture is handed (a frame a door)', has: L.doors, cap: pio?.maxWays ?? Infinity, room: 1, table: null },
    { row: 'sweeps', what: 'leaves one step of the furniture is handed (a leaf a door)', has: L.doors, cap: pio?.maxLeaves ?? Infinity, room: 1, table: null },
  ];
}
