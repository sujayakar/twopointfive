// A DECAL THE LOOK LAID ON A WALL'S FACE GOES WITH ITS WALL — picture only (scene/look.ts; tools/levels/gwotSkin.ts lays them: a painted dado, the damp foot
// of an outside wall). When a wall is holed (game/wallbreak.ts: the authored box is hidden — NoPrimary | NoShadow — and its standing pieces are drawn in its
// place) nothing takes the decal off it: no ray or query ever returned a decal, so the destruction never knew it was there, and a painted sheet would hang
// across the hole. `lookTidy(world)` hides each such decal while its wall is hidden and puts it back with it (a restart restores the wall: the decal follows).
// Called by the PAGE once a frame (main.ts); it reads and writes the flags of drawn-only boxes and nothing else — the game cannot tell it ran.
// SUPERSEDED (2026-11-05) by game/wallskin.ts, which cuts what is painted on ANY wall of ANY level at the hole's edge, on every stand, and writes no flag:
// the page asks `lookTidy` to hide only where that rule is not in force (?wallskin=0, the build as it was; or the cut switch down, where the rule
// cannot draw what stands of a marked decal); where it is, `lookTidy` is told to rest, which puts back whatever it hid while the rule was down (a
// switch turned under a standing page). When that switch is retired this function goes with it.
import { BoxFlag, SHEET_MASK as DECAL_MASK, type Box } from './boxes';
import { lookPaintOf } from './gwotPalette';

interface Tidy { pairs: [decal: number, walls: number[]][]; flags0: number[]; serial: number; rest?: boolean }
const tidies = new WeakMap<object, Tidy | null>();

/** the wall decals of a level and the wall(s) each lies on: a thin upright NoShadow sheet standing on the floor, flat against a BROAD face of a solid box (never
 *  the end face of a wall that meets its plane at a corner); a hand-laid sheet may run along several wall boxes. (A sheet on the GROUND is not this file's: what lies flat follows its ground wet and dry in
 *  game/wetGround.ts, the picture's own step.) */
function find(statics: Box[]): Tidy | null {
  if (!statics.some(b => lookPaintOf(b.albedo, 0.004) !== undefined)) return null;   // (a level that wears none of the look's paints is left exactly as it is: the office's own painted lines and notices are not the look's)
  const pairs: [number, number[]][] = [], flags0: number[] = [];
  const walls: number[] = []; statics.forEach((b, i) => { if (!(b.flags & DECAL_MASK) && !b.yaw && b.c[1] - b.h[1] <= 0.1 && b.c[1] + b.h[1] >= 0.85 && Math.min(b.h[0], b.h[2]) <= 0.3) walls.push(i); });
  statics.forEach((d, i) => {
    if ((d.flags & DECAL_MASK) !== BoxFlag.NoShadow || d.yaw || d.c[1] - d.h[1] > 0.06 || d.h[1] < 0.1 || Math.min(d.h[0], d.h[2]) > 0.006) return;
    const ax = d.h[0] <= d.h[2] ? 0 : 2, al = 2 - ax;   // ax: the sheet's thin axis = the wall's normal; al: along the wall
    const on = walls.filter(w => { const b = statics[w]; return b.h[ax] <= b.h[al] && Math.abs(Math.abs(d.c[ax] - b.c[ax]) - b.h[ax]) <= 0.02 && Math.min(d.c[al] + d.h[al], b.c[al] + b.h[al]) - Math.max(d.c[al] - d.h[al], b.c[al] - b.h[al]) >= 0.2; });
    if (on.length) { pairs.push([i, on]); flags0.push(d.flags); }
  });
  return pairs.length ? { pairs, flags0, serial: -1 } : null;
}
/** once a frame, from the page: cheap while nothing has broken (one compare of the world's geometry serial). A wall's decals go and come back WHOLE with the wall's box — a hole in one cell bares that wall's foot along its length, and the frame the
 *  wall breaks on still shows the sheet (the page tidies before it steps). */
export function lookTidy(world: { statics: Box[]; geomSerial: number }, rest = false): void {
  let T = tidies.get(world.statics); if (T === undefined) { T = find(world.statics); tidies.set(world.statics, T); }   // (found as the level stands when the page first asks, at rest or not: a wall holed since would not be read as a wall)
  if (!T) return;
  if (T.serial === world.geomSerial && T.rest === rest) return;
  T.rest = rest;
  T.pairs.forEach(([d, ws], k) => { const box = world.statics[d]; if (!box) return; const want = !rest && ws.some(w => world.statics[w] && world.statics[w].flags & BoxFlag.NoPrimary) ? T!.flags0[k] | BoxFlag.NoPrimary : T!.flags0[k]; if (box.flags !== want) box.flags = want; });
  // (the sheets' flags are the picture's and no reader of the world's serial asks after them: that number is the game's own count of what broke)
  T.serial = world.geomSerial;
}
