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
// "Some" and "most" are two answers to one question only because of that. The rain is already drawn by the roofs' rectangles: were the wet film masked a
// pixel by those same rectangles, every flat thing could ask for ANY sky, the picture would be right at a canopy's edge, and the 13 of 25 would go. That is
// the renderer's follow-up, not this step's.
// HOW WET SHOWS IS WHAT THE THING IS MADE OF. What sheds water takes the film (glossy, BoxFlag.Gloss, a water film's roughness, its lobe stretched along
// the view). What SOAKS it up takes no film and no puddle (a lawn, a mat, a rug, card). DARKER is what a porous thing does as it drinks: dust, concrete,
// asphalt, cloth; a sealed one (a painted line, plastic, a glazed tile) barely darkens in life, and bare METAL not at all — its albedo is its mirror (F0),
// and water dims no mirror. Today the darkening is ONE NUMBER STANDING FOR TWO (Box.wetK = weather.ts WET_ALBEDO, 0.5, on everything but metal, sealed and
// porous alike: the measured look of the wet lot), and 1 on metal (wet, and the same colour). How much a thing darkens belongs in its stuff's row beside
// whether it soaks, when the look's rows say it. Until then `soaks` below reads what the box is to a round (scene/materials.ts, Box.does: its row's
// surface) and its `metal`.
// THE LOT IS NOT THIS STEP'S. weather.ts setWet soaks the lot's slab in place (its albedo halved: a round's material is read from that, and the runs were
// measured with it), under the level's own storm and a picked rain alike. This step never writes an albedo; a sheet lying on the lot follows it.
// WHEN: weather.ts setWet tells the step in the very call that wets or dries the lot (`follow`), so the lot and the yard are never one wet and one dry for a
// frame drawn with no update between; and the picture's block of every update asks again (game.ts), which is what follows a level whose boxes changed
// (`standing` below: the list is worked out again when what the rule reads of the boxes has moved, and only then).
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
import { doesOf } from '../scene/materials';

/** how far over a wet slab's top a finish may lie and still be the ground (m). It sits in the gap between what LIES and what STANDS: the kit's table of
 *  what lies on what tops out at 15 mm (scene/gwotKit.ts `lie`), 17 with the exporter's steps on it; a mat, a rug and a duckboard, the lowest things that
 *  stand on the ground, start at 22. The check holds the kit's table under this number (wetground-check.ts line 1). */
export const WET_OVER = 0.02;
const top = (b: Box) => b.c[1] + b.h[1];
/** the highest top of a flat a sheet may lie on (m): no box that stands higher is read by the rule, but a finish on a lot that itself stands higher */
const FLAT_TOP = 0.06;
/** does what this box is made of soak water up (no film on it, no puddle): cloth, a lawn, card. Read from the box's word for what it does, the one word
 *  that says it today. This step never writes an albedo and never asks it of the lot, the one box the weather soaks in place (weather.ts setWet) */
export const soaks = (b: Box): boolean => { const s = doesOf(b).surf; return s === 'carpet' || s === 'fabric' || s === 'foliage' || s === 'cardboard'; };
/** a sheet lying on the ground: drawn only, thin, low, not under the ground */
export const groundSheet = (b: Box): boolean => (b.flags & SHEET_MASK) === BoxFlag.NoShadow && b.h[1] <= 0.02 && top(b) <= 0.03 && b.c[1] - b.h[1] >= 0;
/** a flat a sheet may lie on: solid, square to the world, low, broader than a kerb */
export const groundFlat = (b: Box): boolean => (b.flags & SHEET_MASK) === 0 && !b.yaw && top(b) <= FLAT_TOP && b.h[0] > 0.4 && b.h[2] > 0.4;

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

/** WHAT THE RULE READS OF THE BOXES, AS ONE WORD: how many there are and, of every box low enough to be ground or to lie on it, its place in the list, the
 *  flags the rule asks, its middle, its size and its turn (to a 65536th of a metre). The wet list is worked out again when this word moves, and only then: a
 *  low box hidden, put back or moved in place moves it with no box added and nobody told; a wall blown does not (no wall is this low). It is a word of the
 *  level AS IT STANDS, not a count of what has happened to it (the world's `geomSerial` is one of those: 52 on the office after a fight, 0 on the same
 *  office left alone). So a level restarted after its walls were blown and the same level never touched hold the same state here, to the bit, and the
 *  restart's check needs no row for this step (tools/qa/cleanrestart-check.ts line 5). Nothing this step writes is in the word (Gloss is not in SHEET_MASK). */
function standing(statics: readonly Box[], lot: Box | undefined): number {
  const hi = lot ? Math.max(FLAT_TOP, top(lot) + WET_OVER) : FLAT_TOP, P = 0x01000193, Q = 65536; let w = statics.length;
  for (let i = 0; i < statics.length; i++) { const b = statics[i]; if (b.c[1] + b.h[1] > hi) continue;
    w = Math.imul(w ^ i, P); w = Math.imul(w ^ (b.flags & SHEET_MASK), P); w = Math.imul(w ^ (b.yaw * Q), P);
    w = Math.imul(w ^ (b.c[0] * Q), P); w = Math.imul(w ^ (b.c[1] * Q), P); w = Math.imul(w ^ (b.c[2] * Q), P);
    w = Math.imul(w ^ (b.h[0] * Q), P); w = Math.imul(w ^ (b.h[1] * Q), P); w = Math.imul(w ^ (b.h[2] * Q), P); }
  return w >>> 0;
}

export class WetGround {
  /** what this step has wetted, with the finish to put back (a box keeps its one layout: fields are set, never deleted — scene/boxes.ts boxOf) */
  private wetted: { box: Box; flags: number; rough: number | undefined; aniso: boolean | undefined; film: boolean }[] = [];
  /** what `wetted` was worked out for: the level's boxes as they stand (`standing`: never negative, so -1 is "for nothing"), the roofs and whose they are,
   *  the lot */
  private stood = -1; private roofs: readonly Rect[] | null = null; private picked = false; private lot: Box | undefined = undefined;

  /** THE GROUND IS AS WET AS THE WEATHER SAYS, whatever stood before (a restart, a checkpoint, a kind picked or taken away, the row put down): asked by the
   *  weather when it wets or dries the lot, and once an update in the picture's block */
  follow(g: Game): void {
    const W = g.weather, statics = g.engine.world.statics;
    if (!DRESS.wetGround || !W.groundWet) { this.dry(); return; }
    const stood = standing(statics, W.lot);
    if (this.stood !== stood || this.roofs !== W.roofs || this.picked !== W.levelRoofs || this.lot !== W.lot) {
      this.dry(); this.stood = stood; this.roofs = W.roofs; this.picked = W.levelRoofs; this.lot = W.lot;
      for (const box of wetGroundOf(statics, W.roofs, W.lot, W.levelRoofs)) this.wetted.push({ box, flags: box.flags, rough: box.rough, aniso: box.aniso, film: !soaks(box) });
    }
    for (const t of this.wetted) { const b = t.box; if (b.wetK !== undefined) continue;   // (the list says what is wet, the box says whether it is: a box something else dried is wetted again)
      b.wetK = b.metal ? 1 : WET_ALBEDO; if (t.film) { b.flags |= BoxFlag.Gloss; b.rough = WET_ROUGH; b.aniso = true; } }
  }
  /** everything this step wetted, as it was */
  private dry(): void {
    for (const t of this.wetted) { const b = t.box; if (b.wetK === undefined) continue; b.wetK = undefined; if (t.film) { b.flags = (b.flags & ~BoxFlag.Gloss) | (t.flags & BoxFlag.Gloss); b.rough = t.rough; b.aniso = t.aniso; } }
    this.wetted.length = 0; this.stood = -1; this.roofs = null; this.lot = undefined;
  }
  /** how many statics the step holds wet (a check's reading: dress-check.ts) */
  get wet(): number { return this.wetted.length; }
}
