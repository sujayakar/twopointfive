// A LEVEL'S LEAVES ARE PAINTED AS THE GAME IS STOOD (rust/game_rules/src/game, where the doors are made; rust/game_rules/src/doors/paint.rs is the same rule at the same place). A leaf
// is built in the office's colours whatever the level (rust/game_rules/src/doors: a veneer inside, the fire exits' blue-grey outside), and on a mud compound those read as office
// doors. A level that WEARS the look has an outside door in its own accent paint (its row in look.ts LOOK_DOORS, else the one its dado is painted in, counted off the
// level's boxes; army green where it has none) and an inside one in weathered grey wood (one in four with that paint faded on it), each a shade of its own by its name.
// Steel doors are left alone. It is the box's own colour and word, so what a breach throws off a leaf and a round's chips take it. No rule asks what colour a leaf is
// (a round into one is wood whatever its paint: rust/game_rules/src/particle_fx), no dice are drawn, and a level that carries none of the look's paints is not touched.
import { GWOT_PALETTE, lookPaintOf } from '../../../src/scene/gwotPalette';
import { paintOf } from '../../../src/scene/stuff';
import type { Box } from '../../../src/scene/boxes';
import { LOOK_DOORS, type LookDoorStuff } from '../../../src/scene/look';
import type { Vec3 } from '../../../src/math/vec';
import { hashStr } from '../../../src/math/dice';

const ACCENTS = ['paintBlue', 'paintGreen', 'paintRust'] as const;
const near = (a: Vec3, b: Vec3): boolean => Math.abs(a[0] - b[0]) < 0.004 && Math.abs(a[1] - b[1]) < 0.004 && Math.abs(a[2] - b[2]) < 0.004;   // (a paint, or one of the stand-ins a file written before 2026-11-05 carries: up to 0.0024 off its row)
/** with the look on; returns how many leaves it painted.
 *  WHAT A LEAF IS MADE OF GOES WITH ITS PAINT (scene/stuff.ts): a row's own word for that side of the house, else what the paint handed here is made of (the
 *  palette's accent rows are paint, its grey wood is wood); a leaf painted in a row's bare numbers with no word, and one with old paint faded on it, is plain.
 *  A leaf's own shade is a tone of its paint, so every leaf in one paint is one stuff. */
export function paintLeaves(doors: readonly { def: { name: string; exterior?: boolean; steel?: boolean }; box: Box }[], statics: readonly Box[], paint?: { out: readonly [number, number, number]; in?: readonly [number, number, number]; stuff?: LookDoorStuff }): number {
  const count = [0, 0, 0]; let wears = false;
  for (const b of statics) { if (lookPaintOf(b.albedo, 0.004) === undefined) continue; wears = true; ACCENTS.forEach((a, i) => { if (near(b.albedo, GWOT_PALETTE[a])) count[i]++; }); }
  if (!wears) return 0;
  const top = Math.max(...count), accent: Vec3 = paint ? [paint.out[0], paint.out[1], paint.out[2]] : GWOT_PALETTE[top > 0 ? ACCENTS[count.indexOf(top)] : 'paintGreen'];   // (a level with a row in look.ts LOOK_DOORS wears that row; else the paint counted off its boxes)
  let n = 0;
  const wood: Vec3 = paint?.in ? [paint.in[0], paint.in[1], paint.in[2]] : GWOT_PALETTE.woodGrey, faded: Vec3 = [accent[0] * 0.7 + wood[0] * 0.3, accent[1] * 0.7 + wood[1] * 0.3, accent[2] * 0.7 + wood[2] * 0.3];
  const said = (side: 'out' | 'in') => typeof paint?.stuff === 'string' ? paint.stuff : paint?.stuff?.[side];
  for (const d of doors) { if (d.def.steel) continue;
    const h = hashStr(d.def.name), k = 0.9 + (h % 1000) / 1000 * 0.2, a = d.def.exterior ? accent : (h >>> 10) % 4 === 0 ? faded : wood;   // (one inside door in four keeps some old paint)
    const w = typeof paint?.stuff === 'string' ? paint.stuff : a === faded ? undefined : said(d.def.exterior ? 'out' : 'in') ?? paintOf(a)?.stuff;
    d.box.albedo = [a[0] * k, a[1] * k, a[2] * k]; d.box.stuff = w !== 'plain' ? w : undefined; n++; }
  return n;
}
/** a level's row of paints, by the level's own name */
export function leafRowOf(level: string | null) { return LOOK_DOORS[`${level ?? ''}.json`]; }
