// WHAT A LIST OF BOXES ASKS OF A FRAME, COUNTED: the one way a level (tools/qa/level-audit.ts) and a single piece of the kit (a catalogue of the library's
// objects) are counted, so that a piece's price and a level's are in the same terms. Pure: boxes in, numbers out; no level, no game, none of the look's code
// (it runs on a tree that has no look), and nothing happens on import.
//   const { countBoxes, countPiece } = await import('./level-count.ts');
//   countPiece({ statics: K.drum(...), moving: [], lights: [] })   →  { statics: {...}, moving: {...}, lamps: 0, all: {...} }
import { resolve, dirname } from 'node:path';
const ROOT_DIR = resolve(dirname(new URL(import.meta.url).pathname), '../..');
const { BoxFlag } = await import(`${ROOT_DIR}/src/scene/boxes.ts`);

/** the fields of a box the count reads (scene/boxes.ts Box has them all) */
export interface CountedBox { flags: number; emissive?: ArrayLike<number>; cut?: readonly unknown[]; cutRound?: unknown; round?: boolean; rot?: unknown; marks?: unknown }
export interface BoxCount {
  /** every box · the ones the camera draws (not NoPrimary) · the ones that stand in light's way (not NoShadow) */ boxes: number; drawn: number; castShadow: number;
  /** cut boxes and the planes they carry · boxes that are round (a cut prism, or the round rule's cylinder) */ cut: number; planes: number; round: number;
  /** tilted boxes (they get no material map) · panes of glass · boxes that carry markings · drawn boxes that emit */ tilted: number; glass: number; marked: number; glowing: number;
}
export function countBoxes(boxes: readonly CountedBox[]): BoxCount {
  const c: BoxCount = { boxes: boxes.length, drawn: 0, castShadow: 0, cut: 0, planes: 0, round: 0, tilted: 0, glass: 0, marked: 0, glowing: 0 };
  for (const b of boxes) { const drawn = !(b.flags & BoxFlag.NoPrimary), planes = b.cut?.length ?? 0;
    if (drawn) c.drawn++; if (!(b.flags & BoxFlag.NoShadow)) c.castShadow++; if (planes > 0) { c.cut++; c.planes += planes; } if (b.cutRound || b.round === true) c.round++;
    if (b.rot) c.tilted++; if (b.flags & BoxFlag.Glass) c.glass++; if (b.marks) c.marked++;
    if (drawn && b.emissive && b.emissive[0] + b.emissive[1] + b.emissive[2] > 0) c.glowing++; }
  return c;
}
/** a piece, or a level: its boxes that stand, its boxes that move (a prop's parts, a door's leaf) and its lamps */
export function countPiece(p: { statics?: readonly CountedBox[]; moving?: readonly CountedBox[]; lights?: readonly unknown[] }): { statics: BoxCount; moving: BoxCount; all: BoxCount; lamps: number } {
  const statics = countBoxes(p.statics ?? []), moving = countBoxes(p.moving ?? []), all = countBoxes([...(p.statics ?? []), ...(p.moving ?? [])]);
  return { statics, moving, all, lamps: p.lights?.length ?? 0 };
}

/** THE BARS of the budget check (`level-audit.ts --check`; the table page says them too). The engine's caps are facts; the rest are PROPOSED by the task that
 *  wrote them, and nobody has ruled on them */
export const AUDIT_BARS = {
  /** of the engine's own caps on boxes and on lamps burning: a level keeps this much of the cap and no more (a breach adds boxes, a fight adds lamps) */
  headroom: 0.9,
  /** × the heaviest level of the list BEFORE dressing (the baseline), for each count that loads a frame: over it is red. Those levels ran in the build as it
   *  shipped; a dressed level that asks a quarter more than any of them is asking for something nobody has played */
  overHeaviest: 1.25,
  /** × the PASS's own count: over it is not red but is said, for a GPU (the pass is the one level the look itself was timed on) */
  overPass: 1.25,
  /** share of the picture with NO material a level may keep and pass (snow, glass and a thing that glows are not "none") */
  bare: 0.1,
};
