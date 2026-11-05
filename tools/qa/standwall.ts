// A WALL THE LEVEL DOES NOT HAVE, for a check — and a line, or a whole level, with nothing of the level standing on it.
//
// A check used to lay a function of its own over a game's line test (`g.col.segmentBlocked = () => true`). Only the TypeScript that calls the method met
// it: a question answered inside the Rust module (an eye on a man, the lamps on a point, the smoke along a line, a sound's way, a way across the floor)
// reads the module's copy of the BOXES and never calls the method, so the stand-in was met by half the game. A wall that is a box is met by every
// reader, in either language — a line, a ray, a body — as a wall of the level is. So a check that needs a wall stands one, and takes it down:
//
//   const down = standWall(g, [wallBetween(sam, him)]);   …   down();        a wall where there was none
//   const back = clearLine(g, eye, him);   …   back();                    nothing of the level on one line, for as long as a game is played
//   const back = noWalls(g);   …   back();                                no static at all, for a question asked at once
//
// The boxes go on the END of the box world's list (collision.ts addStatic, the road a holed wall's pieces take) and `down()` cuts the list back to where
// it stood (truncateStatics): take walls down last one up first, and stand none across a restart. The floor's grid is not baked again: a man does not
// plan his way round the wall, he only meets it (a check that wants the grid to know rebakes it itself, as tools/qa/underfoot-check.ts does).
import { BoxFlag, makeBox, type Box } from '../../src/scene/boxes.ts';

/** what a check says of a wall: its middle, its half sizes and its turn about the upright */
export interface WallSpec { c: [number, number, number]; h: [number, number, number]; yaw?: number }
type P3 = ArrayLike<number>;
interface World { col: { boxes: Box[]; addStatic(b: Box): number; truncateStatics(n: number): void; wrote(): void; segmentBlocked(a: any, b: any): boolean; raycast(ro: any, rd: any, tmax: number): { t: number; index: number } | null } }

/** the walls stood, plain boxes solid to everything (boxes.ts makeBox, flags 0); → the function that takes them down again. NOTHING ELSE MAY ADD A STATIC
 *  WHILE A WALL STANDS (a wall holed, a crate chewed: their pieces go on the end of the same list): `down()` refuses to cut a list that is not the one it left */
export function standWall(g: World, walls: WallSpec[]): () => void {
  const col = g.col, n = col.boxes.length;
  for (const w of walls) col.addStatic(makeBox({ c: [...w.c], h: [...w.h], yaw: w.yaw ?? 0, flags: 0 }));
  return () => { if (col.boxes.length !== n + walls.length) throw new Error(`standWall: the box world's list is not as the wall left it (${col.boxes.length} boxes for ${n + walls.length}): a restart, a static added while it stood, or a wall taken down out of turn`); col.truncateStatics(n); };
}
/** A THIN WALL ACROSS THE LINE a → b, upright from the floor to `top` m, `half` m to each side of the line, `at` of the way along it (0.5 = the middle;
 *  with b a metre from a, `at` is metres): what 'something solid comes between them' is */
export function wallBetween(a: P3, b: P3, at = 0.5, half = 0.6, top = 2.4): WallSpec {
  const dx = b[0] - a[0], dz = b[2] - a[2];
  return { c: [a[0] + dx * at, top / 2, a[2] + dz * at], h: [half, top / 2, 0.04], yaw: Math.atan2(dx, dz) };   // (a box's own z runs along its turn: its thin side faces down the line)
}
/** hidden as the game hides a box of its own (BoxFlag.NoShadow raised: no line, ray or body meets it); → the function that lowers the flag again on the
 *  boxes it was raised on HERE, and on no other (what the game wrote meanwhile stands) */
function hide(g: World, boxes: Box[]): () => void {
  const mine = boxes.filter(b => !(b.flags & BoxFlag.NoShadow)); for (const b of mine) b.flags |= BoxFlag.NoShadow; g.col.wrote();
  return () => { for (const b of mine) b.flags &= ~BoxFlag.NoShadow; g.col.wrote(); };
}
/** NO STATIC STANDS: every box of the level hidden — the floor too, so ask a question under it and do not play a game under it (`clearLine` is for
 *  that). A door's leaf is no static and still stands. → the function that puts the level back */
export function noWalls(g: World): () => void { return hide(g, g.col.boxes); }
/** NOTHING OF THE LEVEL STANDS ON THE LINE a → b: the statics the line meets, one after another from a, are hidden and nothing else is — the level
 *  stays a level to walk and fall in, so a game may be played under it. A pane on the line goes too (a ray meets glass, though an eye does not), and
 *  a door's leaf cannot (it is no static): → the function that puts them back, with `clear` = is the line a line now */
export function clearLine(g: World, a: P3, b: P3): (() => void) & { clear: boolean } {
  const col = g.col, d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]], L = Math.hypot(d[0], d[1], d[2]), rd = [d[0] / L, d[1] / L, d[2] / L], backs: (() => void)[] = [];
  for (let k = 0; k < 64 && L > 0; k++) { const hit = col.raycast([a[0], a[1], a[2]], rd, L); if (!hit || !(hit.index >= 0) || !col.boxes[hit.index]) break; backs.push(hide(g, [col.boxes[hit.index]])); }
  return Object.assign(() => { for (const f of backs.reverse()) f(); }, { clear: !col.segmentBlocked([a[0], a[1], a[2]], [b[0], b[1], b[2]]) });
}
