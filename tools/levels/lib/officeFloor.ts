// WHAT IS UNDERFOOT IN THE OFFICE, where its walls are: written into its level file's `surfaces` (tools/level-export.ts).
import type { FloorMat } from '../../../src/audio/floors';

/** the building's footprint inside its exterior walls (level.ts: walls on x 4 / 36, z 4 / 24); outside it is ground */
export const BUILDING = { x0: 4, x1: 36, z0: 4, z1: 24 };
/** the lawn: level.ts's two grass strips, z 0‥3.2 the full width (north) and x 0‥3.2 down the west side — everything else outside is asphalt (lot, walkway, east yard) */
export const GRASS_EDGE = 3.2;
/** The finishes laid over the interior carpet base, first match wins — level.ts 'interior floor finishes' rect for rect (the same regions audio.ts's old
 *  surfaceHard tested). Everything inside the walls not listed is carpet: conference room, manager's office, storage, the reception end of the lobby
 *  (z 12.2‥16) and the whole cubicle farm, its rugs included. */
export const FINISHES: { mat: FloorMat; x0: number; z0: number; x1: number; z1: number; room: string }[] = [
  { mat: 'lino',  x0: 4,  z0: 10,   x1: 36, z1: 12.2, room: 'corridor, wall to wall (level.ts floor(4, 10, 36, 12.2, lino); the runner down its middle is a decal)' },
  { mat: 'stone', x0: 4,  z0: 16,   x1: 12, z1: 24,   room: 'lobby south of the reception counter (floor(4, 16, 12, 24, stone))' },
  { mat: 'tile',  x0: 14, z0: 4,    x1: 22, z1: 10,   room: 'server room raised floor (floor(14, 4, 22, 10))' },
  { mat: 'lino',  x0: 28, z0: 12.2, x1: 36, z1: 24,   room: 'break room (floor(28, 12.2, 36, 24, lino))' },
];
