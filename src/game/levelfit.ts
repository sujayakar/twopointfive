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
//   stoodOver(game, …) the same count as a REFUSAL: a stood game with a row over what holds it is not a game (the last statement of Game's constructor).
//                      A box world too big for the copy's own tables is refused sooner, in the same words, by the copy itself at its first
//                      question — the paths' bake, as the box world is made (boxMirror.ts outgrown) — for the copy hands a level's questions
//                      to nobody else.
//   breakRoom(…)       THE ONE THING THAT GROWS IN PLAY: a wall holed stands back up as pieces, each a static, an entry in the cells of the box it
//                      was cut from, and at most one cut row. What the copy holds beyond the level as it loaded is that level's ROOM for pieces,
//                      taken once (game/wallbreak.ts WallBreak.room), and the rule that stands a piece up refuses one past it, as it refuses one past
//                      its own knob: by the smaller of the two. A small level under a raised knob breaks less; it is never an error.
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
import { MEN_ROWS } from '../sim/men/schema';
import { isSheet } from './glass';
import { MAX_SKINNED } from './guards';
import { SEATS } from './seats';
import { BODY_DOOR_PARTICLES } from './game';
import { WALLS } from './wallbreak';

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
import { LIGHT_CAPS } from '../sim/lightshome';
import { findCarLamps } from './carlamps';

/** what of a level is counted before it is built, and what it has of each: doors · loose things of furniture · the boxes those are made of · sheets of glass */
export interface LevelFit { doors: number; things: number; parts: number; sheets: number; fixtures: number; cameras: number; lasers: number; beacons: number; lamps: number }
/** …and how many of each the game holds */
export const levelCaps = (): LevelFit => ({ doors: DOOR_CAP, things: PROP_CAP, parts: PROP_PARTS, sheets: PANE_CAP, fixtures: LIGHT_CAPS.targets, cameras: LIGHT_CAPS.cameras, lasers: LIGHT_CAPS.lasers, beacons: LIGHT_CAPS.beacons, lamps: LIGHT_CAPS.rows });
export function levelCounts(level: { doors: readonly unknown[]; props: readonly { parts: readonly unknown[] }[]; boxes: readonly Box[]; lights: readonly unknown[]; switchables: readonly { boxes: readonly number[]; group: string }[]; beacons: readonly unknown[]; devices?: readonly { kind: string }[] }): LevelFit {
  let parts = 0, sheets = 0; for (const p of level.props) parts += p.parts.length; for (const b of level.boxes) if (isSheet(b)) sheets++;
  let cameras = 0, lasers = 0; for (const d of level.devices ?? []) { if (d.kind === 'camera') cameras++; else if (d.kind === 'laser') lasers++; }
  // (the lamps its fixtures can feed, at most: a box of a switchable is at most an area light and one for each of its markings. Game's constructor works out which are, game.ts; the fullest level today has 52)
  let flat = 0; for (const s of level.switchables) for (const bi of s.boxes) flat += 1 + (level.boxes[bi]?.marks?.length ?? 0);
  return { doors: level.doors.length, things: level.props.length, parts, sheets, fixtures: level.lights.length + level.switchables.length + findCarLamps(level.boxes).length + cameras + lasers + 1, cameras, lasers, beacons: level.beacons.length, lamps: level.lights.length + flat + level.beacons.length + cameras };
}
/** why this level cannot be loaded, in words, or null: it fits */
export function levelOver(level: Parameters<typeof levelCounts>[0]): string | null {
  const has = levelCounts(level), cap = levelCaps(), over: string[] = [];
  if (has.doors > cap.doors) over.push(`${has.doors} doors (the game holds ${cap.doors})`);
  if (has.things > cap.things) over.push(`${has.things} loose things of furniture (${cap.things})`);
  if (has.parts > cap.parts) over.push(`loose furniture of ${has.parts} boxes (${cap.parts})`);
  if (has.sheets > cap.sheets) over.push(`${has.sheets} sheets of glass (${cap.sheets})`);
  if (has.fixtures > cap.fixtures) over.push(`${has.fixtures} light fixtures (${cap.fixtures})`);
  if (has.lamps > cap.lamps) over.push(`lamps on its fixtures, up to ${has.lamps} (${cap.lamps})`);
  if (has.cameras > cap.cameras) over.push(`${has.cameras} cameras (${cap.cameras})`);
  if (has.lasers > cap.lasers) over.push(`${has.lasers} tripwires (${cap.lasers})`);
  if (has.beacons > cap.beacons) over.push(`${has.beacons} emergency beacons (${cap.beacons})`);
  return over.length ? `this level is too big to load: ${over.join(', ')}` : null;
}

/** THE ROOM A LEVEL LEAVES FOR ITS PIECES: how many more statics, entries of the grid's cells and cut rows the copy holds than the level stands with
 *  as it loads (each Infinity where there is no module: no copy to hold to) */
export interface BreakRoom { boxes: number; items: number; cuts: number }
/** statics that carry a cut record (a cut or a shaped box; each takes a cut row of the copy whenever it is drawn cut, so each is counted whatever the
 *  cut-pieces switch says now) */
export const cutStatics = (boxes: readonly Box[]): number => { let n = 0; for (const b of boxes) if (b.cut !== undefined && b.cut.length > 0) n++; return n; };
export function breakRoom(col: Stood['col'], tables: Readonly<Record<string, number>>): BreakRoom {
  if (!tables.boxes) return { boxes: Infinity, items: Infinity, cuts: Infinity };
  return { boxes: tables.boxes - col.boxes.length, items: tables.items - col.gridCount().entries, cuts: tables.cuts - cutStatics(col.boxes) };
}

/** ONE ROW OF THE COUNT: what is counted, how many a stood game has, and how many fit. `table`: the table of the box world's copy the row is held by,
 *  by BoxMirror's own name for it (null: a pool of the rules', or a hand-over, which is no table of the copy). `play`: the row is no reason to refuse a
 *  level, for the game holds itself to it in play (the pieces a broken level stands up: wallbreak.ts refuses one past the room); the check of the shipped
 *  levels still reads it, for a shipped level is to have the room for every piece its knob allows. */
export interface FitRow { row: string; what: string; has: number; cap: number; table: string | null; play?: true }
/** what `fitRows` reads of a stood game (a Game is one) */
interface Stood {
  guards?: readonly unknown[]; civilians?: readonly unknown[];
  level: Parameters<typeof levelCounts>[0] & { civilians?: readonly unknown[] };
  col: { boxes: readonly Box[]; dynamicBoxes: readonly Box[]; gridCount(): { cells: number; entries: number; tilted: number } };
  props?: { all: readonly { boxes: readonly unknown[] }[] } | null;
  destr?: { walls: { piecesSure(): number } } | null;
}
/** EVERY COUNT of a stood game against what holds it. `tables` = the copy's own sizes (BoxMirror.tables; all 0 where there is no module, and then no
 *  table of the copy holds anything to a number: Infinity). */
export function fitRows(g: Stood, tables: Readonly<Record<string, number>>): FitRow[] {
  const col = g.col, L = levelCounts(g.level), cap = levelCaps(), t = (k: string) => tables.boxes ? tables[k] ?? 0 : Infinity, civs = Math.max(g.civilians?.length ?? 0, g.level.civilians?.length ?? 0), most = handOverMost(civs), held = !simWhyNot(), dio = held ? doorIo() : null, pio = held ? propIo() : null;   // (no module: no hand-over to hold to, and every row of one reads Infinity)
  let glass = 0, parts = 0; const grid = col.gridCount();
  for (const b of col.boxes) if (b.flags & BoxFlag.Glass) glass++;
  for (const p of g.props?.all ?? []) parts += p.boxes.length;
  return [
    { row: 'boxes', what: 'statics', has: col.boxes.length, cap: t('boxes'), table: 'boxes' },
    { row: 'cells', what: 'cells of the floor grid', has: grid.cells, cap: t('cells'), table: 'cells' },
    { row: 'items', what: 'entries of those cells', has: grid.entries, cap: t('items'), table: 'items' },
    { row: 'tilted', what: 'tilted statics', has: grid.tilted, cap: t('tilted'), table: 'tilted' },
    { row: 'cuts', what: 'statics that carry a cut record (a cut or a shaped static takes a cut row; so does a piece with a ragged end)', has: cutStatics(col.boxes), cap: t('cuts'), table: 'cuts' },
    { row: 'pieces', what: 'pieces a broken level may stand up (the wall breaker\'s knob) against the pieces its room in the three tables above surely holds', has: WALLS.maxPieces, cap: g.destr?.walls.piecesSure() ?? Infinity, table: null, play: true },
    { row: 'leaves', what: 'leaves (the doors\' list of the leaves\' table)', has: col.dynamicBoxes.length, cap: t('leaves'), table: 'leaves' },
    { row: 'parts', what: 'boxes of the loose furniture (another list of the leaves\' table)', has: parts, cap: Math.min(t('leaves'), cap.parts), table: 'leaves' },
    { row: 'things', what: 'loose things of furniture', has: g.props?.all.length ?? 0, cap: Math.min(t('things'), cap.things), table: 'things' },
    { row: 'panes', what: 'glass one line can note (no more than the level\'s glass)', has: glass, cap: t('panes'), table: 'panes' },
    { row: 'sheets', what: 'sheets of glass (a row a pane that is no longer whole)', has: L.sheets, cap: Math.min(t('glass'), cap.sheets), table: 'glass' },
    { row: 'view', what: 'sheets of glass (a row a pane a burst\'s view names)', has: L.sheets, cap: t('view'), table: 'view' },
    { row: 'doors', what: 'doors (a row a door, and a row a leaf off its hinges)', has: L.doors, cap: cap.doors, table: null },
    // what a game holds of men, and what one step of a rule is handed of them (the rules' hand-overs: no table of the copy)
    { row: 'men', what: 'the first seat\'s man and the guards as the level is stood up (what guards.ts spawnUnit counts against the picture\'s budget of characters)', has: 1 + (g.guards?.length ?? 0), cap: MAX_SKINNED, table: null },
    { row: 'manrows', what: 'rows of the men\'s columns against every man a game can hold (menMost: at most what a step\'s hand-over holds, by civMost; a man who has left keeps no row)', has: menMost(civs), cap: MEN_ROWS, table: null },
    { row: 'civilians', what: 'civilians: the level\'s own, or those in the game as it stands (a rescue\'s hostages and the man a snatch is for join in play; civilians.ts join holds every one to this number as he joins)', has: civs, cap: civMost(), table: null },
    { row: 'doormen', what: 'contacts one step of the doors is handed at most (every man a game holds: himself, and his body\'s particles near a doorway)', has: most.doorMen, cap: dio?.maxContacts ?? Infinity, table: null },
    { row: 'propmen', what: 'men one step of the furniture is handed at most', has: most.propMen, cap: pio?.maxContacts ?? Infinity, table: null },
    { row: 'frames', what: 'door frames one step of the furniture is handed (a frame a door)', has: L.doors, cap: pio?.maxWays ?? Infinity, table: null },
    { row: 'sweeps', what: 'leaves one step of the furniture is handed (a leaf a door)', has: L.doors, cap: pio?.maxLeaves ?? Infinity, table: null },
    { row: 'fixtures', what: 'light fixtures (a row of the lights\' columns each)', has: L.fixtures, cap: cap.fixtures, table: null },
    { row: 'lamps', what: 'lamps on its fixtures, at most (a row of the table a light question reads)', has: L.lamps, cap: cap.lamps, table: null },
    { row: 'cameras', what: 'cameras', has: L.cameras, cap: cap.cameras, table: null },
    { row: 'lasers', what: 'tripwires', has: L.lasers, cap: cap.lasers, table: null },
    { row: 'beacons', what: 'emergency beacons', has: L.beacons, cap: cap.beacons, table: null },
  ];
}

/** why this stood game cannot be played, in words, or null: every row of the count is inside what holds it (a row the game holds itself to in play is
 *  no reason: `play`) */
export function stoodOver(g: Stood, tables: Readonly<Record<string, number>>): string | null {
  const over = fitRows(g, tables).filter(r => !r.play && r.has > r.cap);
  return over.length ? `this level is too big to load: ${over.map(r => `${r.has} ${r.what} (${r.cap})`).join(', ')}` : null;
}
