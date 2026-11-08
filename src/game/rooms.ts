// THE ROOM A FLOOR POINT IS IN, by name: what a page's own books ask (the autosave's 'first time in a room' and its label, the verdict's rows). The game's own rows name their
// rooms in Rust (rust/game_rules/src/chronicle `room_of`).
import type { Vec3 } from '../math/vec';
import { BUILDING } from './floor';   // (a leaf: the office building's footprint — everything outside it is the grounds: the car park south and east, the verge north and west)
import type { Level } from '../scene/level';
import { isOffice, levelRects, ownPlace } from './dialogue';

/** The floor's rooms as the debrief names them: nominal partition lines of the one office level (scene/level.ts — the corridor z 10‥12.2 across the building,
 *  four rooms north of it, three south; the walls themselves are 12 / 24 cm about these lines, which a name does not care about). First match wins. Kept here,
 *  not in the level, while the level format is being turned into data (G1): a `rooms[]` block there replaces this table when it lands. */
export const ROOMS: { name: string; x0: number; x1: number; z0: number; z1: number }[] = [
  { name: 'corridor', x0: 4, x1: 36, z0: 10, z1: 12.2 },
  { name: 'conference room', x0: 4, x1: 14, z0: 4, z1: 10 },
  { name: 'server room', x0: 14, x1: 22, z0: 4, z1: 10 },
  { name: "manager's office", x0: 22, x1: 30, z0: 4, z1: 10 },
  { name: 'storage', x0: 30, x1: 36, z0: 4, z1: 10 },
  { name: 'lobby', x0: 4, x1: 12, z0: 12.2, z1: 24 },
  { name: 'cubicles', x0: 12, x1: 28, z0: 12.2, z1: 24 },
  { name: 'break room', x0: 28, x1: 36, z0: 12.2, z1: 24 },
];
/** … and the rooms of the level a run is on, for the debrief's map: the office's table above, or another level's own names (none: no labels) */
export function roomsOf(level: Level): { name: string; x0: number; x1: number; z0: number; z1: number }[] {
  return isOffice(level) ? ROOMS : (levelRects(level) ?? []).map(r => ({ name: r.name.replace(/^the /, ''), x0: r.x0, x1: r.x1, z0: r.z0, z1: r.z1 }));
}
/** The room a floor point is in, by name; outside the building the side of it ('south lot' — the car park along the facade — 'east side' with the fire exit and
 *  the dumpster, 'north side', 'west side'). `level` = the level the point is on: off the office it is named by that level's own data ('front hall'; the nearest
 *  named place for a point in none; 'building' on a level that names nothing) — without it, the office's table (the autosave's regions, which only need a partition). */
export function roomOf(p: Vec3, level?: Level): string {
  if (level && !isOffice(level)) {   // another level: ITS rooms and places (dialogue.ts levelRects), never this table's
    const w = ownPlace(level, p); return w ? w.rect.name.replace(/^the /, '') : 'building';   // (a noun for 'the … camera', 'in the …': the place he is in, else the nearest the level names)
  }
  const x = p[0], z = p[2];
  if (x >= BUILDING.x0 && x < BUILDING.x1 && z >= BUILDING.z0 && z < BUILDING.z1) { for (const r of ROOMS) if (x >= r.x0 && x < r.x1 && z >= r.z0 && z < r.z1) return r.name; return 'corridor'; }
  if (z >= BUILDING.z1) return 'south lot';
  if (x >= BUILDING.x1) return 'east side';
  if (z < BUILDING.z0) return 'north side';
  return 'west side';
}
/** interior = inside the building's footprint (the map shades the grounds darker) */
export function indoors(p: Vec3): boolean { return p[0] >= BUILDING.x0 && p[0] < BUILDING.x1 && p[2] >= BUILDING.z0 && p[2] < BUILDING.z1; }

