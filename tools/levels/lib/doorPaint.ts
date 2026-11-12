// WHAT A LEVEL'S LEAVES ARE PAINTED (the file's `doorPaint`; rust/game_rules/src/fittings/doors/paint.rs puts it on): a generator's `DOOR_PAINT` export. What a side is made of is what its paint says (src/scene/stuff.ts).
import type { Vec3 } from '../../../src/math/vec';
import { GWOT_PALETTE } from '../../../src/scene/gwotPalette';
import { wordFor } from '../../../src/scene/stuff';
import type { DoorPaintData, LeafPaintData } from './levelRows';

const leaf = (paint: Readonly<Vec3>): LeafPaintData => { const { stuff } = wordFor(paint); return { albedo: [paint[0], paint[1], paint[2]], ...(stuff !== undefined ? { stuff } : {}) }; };
/** leaves that lead outside in this paint, and the rest in that (weathered grey wood, unless a level says another) */
export const leavesOf = (outside: Readonly<Vec3>, inside: Readonly<Vec3> = GWOT_PALETTE.woodGrey): DoorPaintData => ({ outside: leaf(outside), inside: leaf(inside) });
/** the look's three door paints */
export const BLUE_LEAVES = (): DoorPaintData => leavesOf(GWOT_PALETTE.paintBlue);
export const GREEN_LEAVES = (): DoorPaintData => leavesOf(GWOT_PALETTE.paintGreen);
export const RUST_LEAVES = (): DoorPaintData => leavesOf(GWOT_PALETTE.paintRust);
