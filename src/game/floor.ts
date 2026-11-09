// The floor you walk on: ONE material map read by everything that listens to a footstep — the mixer (audio.ts footstep: which sample, how loud, how bright),
// the guards' ears (guards.ts hearingCheck scores the 'step' event's level) and the SOUND meter (player.noise takes the same level). Before this the mixer knew
// carpet from lino (its private surfaceHard) and the AI did not: a violation of the one-model rule. The map reproduces the finishes level.ts lays over the
// interior's carpet base ('interior floor finishes': floor(x0, z0, x1, z1, albedo)) and the exterior ground (asphalt slab, grass strips north and west); rugs,
// runners and mats are decals (NoShadow, a few mm proud) and deliberately do NOT count — the finish under them does — so the corridor is lino wall to wall,
// runner or not, and stays the reference every hearing number was tuned on.
import { Vec3 } from '../math/vec';
import type { Game } from './game';
import type { SurfacesData } from '../scene/levelData';
import { crossToRust } from './crossing';

export { FLOOR, type FloorMat } from '../audio/floors';
import type { FloorMat } from '../audio/floors';

import { BUILDING, FINISHES, GRASS_EDGE } from '../scene/officeFloor';
export { BUILDING, FINISHES, GRASS_EDGE };

/** The material under a point on the floor (y ignored). `S` = the level's own `surfaces` block (Level.surfaces, the level file's): the same rule over its
 *  numbers — outside `building` the ground (`verge` where x or z < grassEdge, else `outside`), inside it the first zone that holds the point, else `inside`.
 *  Without one (every level file that declares none, and callers with no level to hand) it is the office's map below. */
export function floorAt(p: Vec3, S?: SurfacesData): FloorMat {
  const x = p[0], z = p[2];
  if (S) {
    const B = S.building;
    if (x < B.x0 || x > B.x1 || z < B.z0 || z > B.z1) return x < S.grassEdge || z < S.grassEdge ? S.verge : S.outside;
    for (const f of S.zones) if (x >= f.x0 && x <= f.x1 && z >= f.z0 && z <= f.z1) return f.mat;
    return S.inside;
  }
  if (x < BUILDING.x0 || x > BUILDING.x1 || z < BUILDING.z0 || z > BUILDING.z1) return x < GRASS_EDGE || z < GRASS_EDGE ? 'grass' : 'asphalt';
  for (const f of FINISHES) if (x >= f.x0 && x <= f.x1 && z >= f.z0 && z <= f.z1) return f.mat;
  return 'carpet';
}

/** the material under a point on THIS game's level: its own `surfaces` block; without one, the office's map — but a level that carries a quiet mission and
 *  declares no surfaces (scene/scenarios.ts QuietData: a floor that is none of the office's, whose patrols and whose player are heard by their footsteps) is
 *  plain lino everywhere rather than the office's carpet and stone laid over another building */
export function floorOf(g: Game, pos: Vec3): FloorMat { const L = g.level; return !L.surfaces && L.scenarios?.quiet ? 'lino' : floorAt(pos, L.surfaces); }

/** One footfall, for the mixer and the model alike: resolves the material, scales the step's base hearing level by its loudness, plays it (audio.footstep gets
 *  the scaled level AND the material: sample choice and brightness follow it) and returns the level — what the caller pushes as the 'step' GameEvent's level
 *  and into player.noise (player.ts), or nothing more for a guard's tread (presentation only: guards.ts / squad.ts). */
export function tread(g: Game, pos: Vec3, base: number, isPlayer: boolean): number { return crossToRust('tread', undefined, [g, pos, base, isPlayer]) as any; }
