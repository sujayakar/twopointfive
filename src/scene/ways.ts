// HOW A LEVEL IS WALKED — one map a level of where feet go, from what the level already says of itself: the routes its shift walks, where a team starts
// and the rooms and things it goes for, its doors, and whatever its author adds by name. The maintainer, 11-04: "wear, paths in the ground, different
// materials can all tell a story for how people walk through the level, its logic, leading the player for game play".
// WHAT IT IS: four counts on the walk grid's own cells (scene/collision.ts NavGrid: half a metre), each the number of ways that cross the cell, NOT summed
// and NOT saturated: how much a patrol's lane counts against a team's, and what either does to a floor, is the reader's (render/wear.ts turns them into the
// traffic a surface shows; a tool may draw them; a level's check may ask whether its start has a lane to its door).
//   patrol   the level's routes, each leg and the leg back to its first point
//   team     every place a team may start (each scenario's, in the file's order; the level's own spawn where no scenario has one) to everything it goes
//            for: each room of the room clear, the rescue's room, each quiet spot and each way out
//   doors    one for every door, on the floor at both faces of its frame (a door is used, whoever walks to it), and THE YARD'S OWN WAYS: every pair of
//            the level's outer doors, door to door (a gate is a door; the ground between two buildings' doors is walked, with no route and no team)
//   hint     the author's own: from a named thing to a named thing, × its weight
// A NAME is looked for IN THIS ORDER and the first found is it: `spawn`, a door's id, a mark's name, a route's (its first point), a room clear room's id
// or name, a place's name (a room's and a place's: the middle of its floor). So a door and a room called the same are the door. One it cannot find is
// listed in `unknown` and walks nothing. Every way is the walk grid's own path (its search, on tables of this function's own: the grid's are left as
// they were), so a way goes through the doors a man would take; one the search finds no path for is the straight line over free floor and is counted in
// `lost` (a level's check holds that at none). With no grid every way is the straight line.
// BESIDE THE CELLS, `paths`: the same ways as the lines they were traced along, for a picture that wants a lane and not a staircase of cells.
// PURE: the same level gives the same map; no module state, no die, no clock. Nothing in src/game imports it (it is a picture's and a tool's).
import type { Vec3 } from '../math/vec';
import type { Level } from './level';
import type { LevelScenarios, Rect, TeamStart } from './scenarios';
import type { NavGrid } from './collision';
import { WORLD } from './world';

/** a way the author adds (the level file's `scenarios.walked`): from a named thing to a named thing (the head of this file says what a name is); `weight` = how many ways it counts as (absent: one) */
export interface WayHint { from: string; to: string; weight?: number }
export interface Ways {
  /** the corner of cell (0, 0) and a cell's side (m): cell (i, j) covers x0 + i·cell ‥ x0 + (i + 1)·cell */
  x0: number; z0: number; cell: number; w: number; h: number;
  patrol: Float32Array; team: Float32Array; doors: Float32Array; hint: Float32Array;
  /** every way as it was traced: which count it is in, what it counts as, and its line on the floor */
  paths: { kind: 'patrol' | 'team' | 'doors' | 'hint'; weight: number; points: Vec3[] }[];
  /** names a hint asked for that the level does not have; and how many ways the search found no path for */
  unknown: string[]; lost: number;
}
/** the middle of a door's frame on the floor (game/doors.ts frameCentre's x and z) */
export const doorMid = (d: { hinge: readonly [number, number]; closedDir: number; width: number }): Vec3 => [d.hinge[0] + Math.cos(d.closedDir) * d.width * 0.5, 0, d.hinge[1] + Math.sin(d.closedDir) * d.width * 0.5];

const PATH_ITER = 6000, STEP = 0.2;   // (the search's bound, as every asker of the grid has it; a way is laid on the cells every 20 cm of its length)

/** `opts.hints`: the author's ways, where they are not the level's own (absent: `scenarios.walked`) */
export function levelWays(level: Level, scenarios: LevelScenarios | undefined, nav: NavGrid | null, opts: { hints?: readonly WayHint[] } = {}): Ways {
  const cell = nav?.cell ?? 0.5, w = nav?.W ?? Math.round(WORLD.gridW * WORLD.gridCell / cell), h = nav?.H ?? Math.round(WORLD.gridH * WORLD.gridCell / cell);
  const out: Ways = { x0: 0, z0: 0, cell, w, h, patrol: new Float32Array(w * h), team: new Float32Array(w * h), doors: new Float32Array(w * h), hint: new Float32Array(w * h), paths: [], unknown: [], lost: 0 };
  const mark = new Int32Array(w * h); let gen = 0;
  /** one way from a to b: each cell it crosses counts `weight`, once */
  const walk = (kind: 'patrol' | 'team' | 'doors' | 'hint', a: Vec3, b: Vec3, weight = 1) => {
    const ch = out[kind]; let path = nav ? nav.findPath(a, b, PATH_ITER) : null;
    if (nav && (!path || !path.length)) { out.lost++; path = null; }
    const pts: Vec3[] = path ? [a, ...path] : [a, b]; gen++; out.paths.push({ kind, weight, points: pts });
    const put = (x: number, z: number) => { const i = Math.floor(x / cell), j = Math.floor(z / cell); if (i < 0 || j < 0 || i >= w || j >= h) return; const k = j * w + i; if (mark[k] !== gen && !(nav && nav.blocked[k])) { mark[k] = gen; ch[k] += weight; } };
    for (let k = 0; k + 1 < pts.length; k++) { const p = pts[k], q = pts[k + 1], len = Math.hypot(q[0] - p[0], q[2] - p[2]), n = Math.max(1, Math.ceil(len / STEP));
      for (let s = 0; s <= n; s++) put(p[0] + (q[0] - p[0]) * s / n, p[2] + (q[2] - p[2]) * s / n); }
  };
  const mid = (r: Rect): Vec3 => [(r.x0 + r.x1) / 2, 0, (r.z0 + r.z1) / 2];
  const doorAt = (id: string): Vec3 | null => { const d = level.doors.find(x => x.name === id); return d ? doorMid(d) : null; };

  // ---- patrol: the shift's routes
  for (const r of level.routes) { const p = r.points; if (p.length < 2) continue; for (let i = 0; i < p.length; i++) walk('patrol', p[i], p[(i + 1) % p.length]); }

  // ---- team: every start to everything a team goes for
  const sc = scenarios ?? {}, starts: Vec3[] = [], goals: Vec3[] = [];
  const start = (t: TeamStart | undefined) => { if (t) starts.push(t.sam); };
  start(sc.roomClear?.team); for (const t of sc.roomClear?.starts ?? []) start(t); start(sc.rescue?.team); start(sc.nightBreach?.team); for (const q of sc.quiet?.starts ?? []) starts.push(q.sam);
  if (!starts.length) starts.push(level.playerSpawn);
  for (const r of sc.roomClear?.rooms ?? []) goals.push(mid(r.bounds));
  if (sc.rescue) { const g = sc.rescue.room.bounds ? mid(sc.rescue.room.bounds) : doorAt(sc.rescue.room.door); if (g) goals.push(g); }
  for (const s of sc.quiet?.spots ?? []) goals.push(mid(s.area));
  for (const e of sc.quiet?.exits ?? []) goals.push([e.x, 0, e.z]);
  for (const s of starts) for (const g of goals) walk('team', s, g);

  // ---- doors: the floor at both faces of every frame
  for (const d of level.doors) { const c = doorAt(d.name)!, nx = -Math.sin(d.closedDir), nz = Math.cos(d.closedDir); gen++;
    for (const sg of [-1, 1]) for (const t of [0.25, 0.75]) { const x = c[0] + nx * sg * t * cell * 1.5, z = c[2] + nz * sg * t * cell * 1.5, i = Math.floor(x / cell), j = Math.floor(z / cell);
      if (i < 0 || j < 0 || i >= w || j >= h) continue; const k = j * w + i; if (mark[k] !== gen) { mark[k] = gen; out.doors[k] += 1; } } }

  { const outer = level.doors.filter(d => d.exterior); for (let i = 0; i < outer.length; i++) for (let k = i + 1; k < outer.length; k++) walk('doors', doorMid(outer[i]), doorMid(outer[k])); }

  // ---- hint: the author's own, by name
  const named = (n: string): Vec3 | null => {
    if (n === 'spawn') return level.playerSpawn;
    const d = doorAt(n); if (d) return d;
    const m = level.marks?.find(x => x.name === n); if (m) return m.pos;
    const r = level.routes.find(x => x.name === n); if (r && r.points.length) return r.points[0];
    const room = sc.roomClear?.rooms.find(x => x.id === n || x.name === n); if (room) return mid(room.bounds);
    const area = sc.places?.areas.find(x => x.name === n); if (area) return mid(area.bounds);
    return null;
  };
  for (const hnt of opts.hints ?? sc.walked ?? []) { const a = named(hnt.from), b = named(hnt.to);
    if (!a && !out.unknown.includes(hnt.from)) out.unknown.push(hnt.from); if (!b && !out.unknown.includes(hnt.to)) out.unknown.push(hnt.to);
    if (a && b) walk('hint', a, b, hnt.weight ?? 1); }
  return out;
}
