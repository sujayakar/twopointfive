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
// (The pools themselves refuse more rows than they have: sim/doorshome.ts, sim/slabshome.ts, sim/propshome.ts. This is the same statement, made early.)
import { BoxFlag, type Box } from '../scene/boxes';
import { PANE_CAP } from '../scene/boxMirror';
import { DOOR_CAP } from '../sim/doorshome';
import { PROP_CAP, PROP_PARTS } from '../sim/propshome';
import { LIGHT_CAPS } from '../sim/lightshome';
import { isSheet } from './glass';
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

/** ONE ROW OF THE COUNT: what is counted, how many a stood game has, how many fit, and the share of that a level may fill AS IT LOADS (1, or less for a
 *  table that grows in play: a wall holed stands back up as pieces, each a static, an entry of its cells and a cut row). `table`: the table of the box
 *  world's copy the row is held by, by BoxMirror's own name for it (null: a pool of the rules', which is no table of the copy). */
export interface FitRow { row: string; what: string; has: number; cap: number; room: number; table: string | null }
/** what `fitRows` reads of a stood game (a Game is one) */
interface Stood {
  level: Parameters<typeof levelCounts>[0];
  col: { boxes: readonly Box[]; W: number; H: number; cells: readonly (readonly number[])[]; tiltedAt: readonly number[]; dynamicBoxes: readonly Box[]; mirror: { cutRows: number } };
  props?: { all: readonly { boxes: readonly unknown[] }[] } | null;
}
/** EVERY COUNT of a stood game against what holds it. `tables` = the copy's own sizes (BoxMirror.tables, after a tick's open has asked the module). */
export function fitRows(g: Stood, tables: Readonly<Record<string, number>>): FitRow[] {
  const col = g.col, L = levelCounts(g.level), cap = levelCaps(), t = (k: string) => tables[k] ?? 0;
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
    { row: 'fixtures', what: 'light fixtures (a row of the lights\' columns each)', has: L.fixtures, cap: cap.fixtures, room: 1, table: null },
    { row: 'lamps', what: 'lamps on its fixtures, at most (a row of the table a light question reads)', has: L.lamps, cap: cap.lamps, room: 1, table: null },
    { row: 'cameras', what: 'cameras', has: L.cameras, cap: cap.cameras, room: 1, table: null },
    { row: 'lasers', what: 'tripwires', has: L.lasers, cap: cap.lasers, room: 1, table: null },
    { row: 'beacons', what: 'emergency beacons', has: L.beacons, cap: cap.beacons, room: 1, table: null },
  ];
}
