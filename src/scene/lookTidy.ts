// A DECAL THE LOOK LAID ON A WALL'S FACE GOES WITH ITS WALL — picture only (scene/look.ts; tools/levels/gwotSkin.ts lays them: a painted dado, the damp foot
// of an outside wall). When a wall is holed (game/wallbreak.ts: the authored box is hidden — NoPrimary | NoShadow — and its standing pieces are drawn in its
// place) nothing takes the decal off it: no ray or query ever returned a decal, so the destruction never knew it was there, and a painted sheet would hang
// across the hole. `lookTidy(world)` hides each such decal while its wall is hidden and puts it back with it (a restart restores the wall: the decal follows).
// Called by the PAGE once a frame (main.ts); it reads and writes the flags of drawn-only boxes and nothing else — the game cannot tell it ran.
import { BoxFlag, SHEET_MASK as DECAL_MASK, type Box } from './boxes';
import { GWOT_PALETTE, gwotSimOf } from './gwotPalette';
import type { Vec3 } from '../math/vec';
import { hashStr } from '../math/dice';

interface Tidy { pairs: [decal: number, walls: number[]][]; flags0: number[]; serial: number }
const tidies = new WeakMap<object, Tidy | null>();
const HIDE = BoxFlag.NoPrimary | BoxFlag.NoShadow;

/** the wall decals of a level and the wall(s) each lies on: a thin upright NoShadow sheet standing on the floor, flat against a BROAD face of a solid box (never
 *  the end face of a wall that meets its plane at a corner); a hand-laid sheet may run along several wall boxes. (A sheet on the GROUND is not this file's: what lies flat follows its ground wet and dry in
 *  game/wetGround.ts, the picture's own step.) */
function find(statics: Box[]): Tidy | null {
  if (!statics.some(b => gwotSimOf(b.albedo) !== undefined)) return null;   // (a level that wears none of the look's paints is left exactly as it is: the office's own painted lines and notices are not the look's)
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
export function lookTidy(world: { statics: Box[]; geomSerial: number }): void {
  let T = tidies.get(world.statics); if (T === undefined) { T = find(world.statics); tidies.set(world.statics, T); }
  if (!T) return;
  if (T.serial === world.geomSerial) return;
  let changed = false;
  T.pairs.forEach(([d, ws], k) => { const box = world.statics[d]; if (!box) return; const want = ws.some(w => world.statics[w] && world.statics[w].flags & BoxFlag.NoPrimary) ? T!.flags0[k] | BoxFlag.NoPrimary : T!.flags0[k]; if (box.flags !== want) { box.flags = want; changed = true; } });
  if (changed) world.geomSerial++;
  T.serial = world.geomSerial;
}

// DOOR LEAVES IN THE LOOK — picture only, as above. A leaf is built in the office's colours whatever the level (game/doors.ts: a veneer inside, the fire
// exits' blue-grey outside), and on a mud compound those read as office doors. `lookDoors` repaints the leaves of a level that WEARS the look, once, after the
// game is stood up: an outside door in the level's own accent paint (its row in look.ts LOOK_DOORS, else the one its dado is painted in — counted off the
// level's boxes; army green where it has none), an inside one in weathered grey wood (one in four with that paint faded on it), each a shade of its own by its name. Steel doors are left alone. Nothing in the game asks what colour a leaf is
// (a round into one is wood whatever its paint — particleFx.ts; the chips take the colour), no dice are drawn, and a level that carries none of the look's
// paints — the office under the switch — is not touched.

const ACCENTS = ['paintBlue', 'paintGreen', 'paintRust'] as const;
const near = (a: Vec3, b: Vec3): boolean => Math.abs(a[0] - b[0]) < 0.004 && Math.abs(a[1] - b[1]) < 0.004 && Math.abs(a[2] - b[2]) < 0.004;   // (a paint or one of its stand-ins: gwotAs shifts by ≤ 0.0024)
/** the page calls it once the game stands, with the switch on (main.ts); returns how many leaves it painted */
export function lookDoors(doors: readonly { def: { name: string; exterior?: boolean; steel?: boolean }; box: Box }[], statics: readonly Box[], paint?: { out: readonly [number, number, number]; in?: readonly [number, number, number] }): number {
  const count = [0, 0, 0]; let wears = false;
  for (const b of statics) { if (gwotSimOf(b.albedo) === undefined) continue; wears = true; ACCENTS.forEach((a, i) => { if (near(b.albedo, GWOT_PALETTE[a])) count[i]++; }); }
  if (!wears) return 0;
  const top = Math.max(...count), accent: Vec3 = paint ? [paint.out[0], paint.out[1], paint.out[2]] : GWOT_PALETTE[top > 0 ? ACCENTS[count.indexOf(top)] : 'paintGreen'];   // (a level with a row in look.ts LOOK_DOORS wears that row; else the paint counted off its boxes)
  let n = 0;
  const wood: Vec3 = paint?.in ? [paint.in[0], paint.in[1], paint.in[2]] : GWOT_PALETTE.woodGrey, faded: Vec3 = [accent[0] * 0.7 + wood[0] * 0.3, accent[1] * 0.7 + wood[1] * 0.3, accent[2] * 0.7 + wood[2] * 0.3];
  for (const d of doors) { if (d.def.steel) continue;
    const h = hashStr(d.def.name), k = 0.9 + (h % 1000) / 1000 * 0.2, a = d.def.exterior ? accent : (h >>> 10) % 4 === 0 ? faded : wood;   // (one inside door in four keeps some old paint)
    d.box.albedo = [a[0] * k, a[1] * k, a[2] * k]; n++; }
  return n;
}
