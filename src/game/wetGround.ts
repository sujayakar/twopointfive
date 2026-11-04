// WET GROUND, AS THE PICTURE DRAWS IT — dressing (src/game/dress.ts `wetGround`; docs/agents/weather.md).
// WHERE WATER LIES IS ONE RULE: WHAT LIES FLAT UNDER OPEN SKY IS WET WHILE THE GROUND IS. In the order the step works it out:
//   1 the ground SLABS a picked rain falls on (weather.ts openGround: the slab everybody stands on, the lot's y = 0 or a base a millimetre over it, with sky
//     over some of it). The lot itself is the weather's own: see below;
//   2 a FINISH LAID ON such a slab or on the lot: a solid flat thing whose top is within WET_OVER of the slab's top, its middle inside the slab's footprint, with
//     sky over MOST of it (weather.ts skyOverMost). The newer kit lays a yard, a road and a pad as finishes 4 to 12 mm thick over the lot (scene/gwotKit.ts
//     `ground`): until 11-04 only the slab under them was wetted, where nobody sees it, and a dressed yard stayed dry in the rain with every mark on it;
//   3 a DRAWN-ONLY SHEET lying on the ground (a stain, ruts, a patch, a painted line, the look's wear) follows the topmost flat under its middle while most of
//     the sheet itself is under the sky: wet with its ground, dry with it, and dry under a roof whatever its ground is (the lot is one slab, wet whole, and
//     runs under a hut with no floor of its own). (This was scene/lookTidy.ts wetTidy, the page's own step, on levels in the look's paints only and with no
//     sky test; it is this step's now, on every level and off the page too, and there is one writer of a sheet's wet word.)
// A BOX IS WET WHOLE OR NOT AT ALL (the renderer reads one word a box): that is why a slab asks for SOME sky and a thing laid for one place for MOST of it.
// HOW WET SHOWS IS WHAT THE THING IS MADE OF. What sheds water goes darker and takes the film (Box.wetK under 1; glossy, BoxFlag.Gloss, a water film's
// roughness, its lobe stretched along the view). What SOAKS it up goes darker and takes no film and no puddle (a lawn, a mat, a rug, card). Bare METAL takes
// the film and does not go darker: its albedo is its mirror (F0), and water dims no mirror (Box.wetK is 1 on it: wet, and the same colour). Today a box says
// what it is made of through its paint (particleFx.ts surfOf, what the thing is to a round) and its `metal`: `soaks` below reads the first, and reads the
// box's own word when a box has one.
// THE LOT IS NOT THIS STEP'S. weather.ts setWet soaks the lot's slab in place (its albedo halved: a round's material is read from that, and the runs were
// measured with it), under the level's own storm and a picked rain alike. This step never writes an albedo; a sheet lying on the lot follows it.
// WHEN: weather.ts setWet tells the step in the very call that wets or dries the lot (`follow`), so the lot and the yard are never one wet and one dry for a
// frame drawn with no update between; and the picture's block of every update asks again (game.ts), which is what follows a level whose boxes changed.
// DRESSING: it reads the weather (is the ground wet, is a rain picked, the roofs, the lot) and the statics; it WRITES Box.wetK, BoxFlag.Gloss, rough and
// aniso of a static and nothing else — fields only the renderer reads (the Rust module's copy of the boxes carries none of them: scene/boxMirror.ts
// MIRRORED; no digest walks a static; no footstep, ear or eye of the simulation reads them). With the row down nothing is wet by this step, whenever it
// was put down, and the game is the same game to the bit (tools/qa/dress-check.ts, on a dry night and with `--weather downpour`).
// WHAT IT DOES NOT DO: a static broken WHILE it is wet hands its wet finish to its pieces (breakTable.ts, wallbreak.ts copy `rough` and `aniso`), which never
// dry. No break takes a ground slab, a finish or a sheet today (the wall engine breaks walls, a blast furniture), so it is not met.
// CHECK: tools/qa/wetground-check.ts (probes.ts `wetground`, family `weather`). Never seen on a GPU.
import { BoxFlag, SHEET_MASK, type Box } from '../scene/boxes';
import type { Game } from './game';
import { DRESS } from './dress';
import { openGround, skyOverMost, WET_ALBEDO, WET_ROUGH } from './weather';
import type { Rect } from './weatherKinds';
import { surfOf } from './particleFx';

/** how far over a wet slab's top a finish may lie and still be the ground (m). It sits in the gap between what LIES and what STANDS: the kit's table of
 *  what lies on what tops out at 15 mm (scene/gwotKit.ts `lie`), 17 with the exporter's steps on it; a mat, a rug and a duckboard, the lowest things that
 *  stand on the ground, start at 22. The check holds the kit's table under this number (wetground-check.ts line 1). */
export const WET_OVER = 0.02;
const top = (b: Box) => b.c[1] + b.h[1];
/** does what this box is made of soak water up (no film on it, no puddle): cloth, a lawn, card. Read from its paint, the one word a box has for it today */
export const soaks = (b: Box): boolean => { const s = surfOf(b.albedo); return s === 'carpet' || s === 'fabric' || s === 'foliage' || s === 'cardboard'; };
/** a sheet lying on the ground: drawn only, thin, low, not under the ground */
export const groundSheet = (b: Box): boolean => (b.flags & SHEET_MASK) === BoxFlag.NoShadow && b.h[1] <= 0.02 && top(b) <= 0.03 && b.c[1] - b.h[1] >= 0;
/** a flat a sheet may lie on: solid, square to the world, low, broader than a kerb */
export const groundFlat = (b: Box): boolean => (b.flags & SHEET_MASK) === 0 && !b.yaw && top(b) <= 0.06 && b.h[0] > 0.4 && b.h[2] > 0.4;

/** THE GROUND THAT IS WET for the picture while the weather's ground is, of these statics: the rule at the head, in its order. `picked` = a rain falls by
 *  the level's own roofs (weather.ts levelRoofs: a picked kind, or a squad level's own storm): only then are there slabs and finishes; a sheet follows wet
 *  ground either way. `lot` = the slab the weather has soaked (wet, and not in the answer). A function of the level, the roofs and those two alone. */
export function wetGroundOf(statics: readonly Box[], roofs: readonly Rect[], lot: Box | undefined, picked: boolean): Box[] {
  const wet = new Set<Box>(), bases: Box[] = lot ? [lot] : [];
  if (picked) {
    for (const b of statics) if (b !== lot && openGround(b, roofs)) { wet.add(b); bases.push(b); }
    const on = (b: Box, s: Box) => { const up = top(b) - top(s); return up > 0 && up <= WET_OVER && Math.abs(b.c[0] - s.c[0]) <= s.h[0] && Math.abs(b.c[2] - s.c[2]) <= s.h[2]; };
    for (const b of statics) if (b !== lot && !wet.has(b) && (b.flags & SHEET_MASK) === 0 && bases.some(s => on(b, s)) && skyOverMost(b, roofs)) wet.add(b);
  }
  // a sheet follows the topmost flat under its middle (the first of two as high)
  const grounds = statics.filter(groundFlat), sheets: Box[] = [];
  for (const d of statics) {
    if (!groundSheet(d)) continue;
    let g: Box | undefined, at = -1e9; for (const b of grounds) if (Math.abs(b.c[0] - d.c[0]) <= b.h[0] && Math.abs(b.c[2] - d.c[2]) <= b.h[2] && top(b) > at) { at = top(b); g = b; }
    if (g && (g === lot || wet.has(g)) && skyOverMost(d, roofs)) sheets.push(d);
  }
  return [...wet, ...sheets];
}

export class WetGround {
  /** what this step has wetted, with the finish to put back (a box keeps its one layout: fields are set, never deleted — scene/boxes.ts boxOf) */
  private wetted: { box: Box; flags: number; rough: number | undefined; aniso: boolean | undefined; film: boolean }[] = [];
  /** what `wetted` was worked out for: the level's boxes (their count, and the world's serial, which a box hidden or put back moves with no box added), the
   *  roofs and whose they are, the lot */
  private count = -1; private serial = -1; private roofs: readonly Rect[] | null = null; private picked = false; private lot: Box | undefined = undefined;

  /** THE GROUND IS AS WET AS THE WEATHER SAYS, whatever stood before (a restart, a checkpoint, a kind picked or taken away, the row put down): asked by the
   *  weather when it wets or dries the lot, and once an update in the picture's block */
  follow(g: Game): void {
    const W = g.weather, world = g.engine.world, statics = world.statics;
    if (!DRESS.wetGround || !W.groundWet) { this.dry(); return; }
    if (this.count !== statics.length || this.serial !== world.geomSerial || this.roofs !== W.roofs || this.picked !== W.levelRoofs || this.lot !== W.lot) {
      this.dry(); this.count = statics.length; this.serial = world.geomSerial; this.roofs = W.roofs; this.picked = W.levelRoofs; this.lot = W.lot;
      for (const box of wetGroundOf(statics, W.roofs, W.lot, W.levelRoofs)) this.wetted.push({ box, flags: box.flags, rough: box.rough, aniso: box.aniso, film: !soaks(box) });
    }
    for (const t of this.wetted) { const b = t.box; if (b.wetK !== undefined) continue;   // (the list says what is wet, the box says whether it is: a box something else dried is wetted again)
      b.wetK = b.metal ? 1 : WET_ALBEDO; if (t.film) { b.flags |= BoxFlag.Gloss; b.rough = WET_ROUGH; b.aniso = true; } }
  }
  /** everything this step wetted, as it was */
  private dry(): void {
    for (const t of this.wetted) { const b = t.box; if (b.wetK === undefined) continue; b.wetK = undefined; if (t.film) { b.flags = (b.flags & ~BoxFlag.Gloss) | (t.flags & BoxFlag.Gloss); b.rough = t.rough; b.aniso = t.aniso; } }
    this.wetted.length = 0; this.count = -1; this.serial = -1; this.roofs = null; this.lot = undefined;
  }
  /** how many statics the step holds wet (a check's reading: dress-check.ts) */
  get wet(): number { return this.wetted.length; }
}
