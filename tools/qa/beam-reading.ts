// WHAT A CARRIED BEAM ENDS ON — the one reading the lights' checks take of a beam (probes.ts light-wall, flashweight-lights, hours-team, torch-wall). The
// geometry is the game's and is handed in (`Beam`: guards.ts beamFan and beamRayEnd, bound to a game — per ray 3 = something that stands, met above the
// ground's band · 2 = the foot of such a thing · 1 = the ground · 0 = nothing inside the distance), so there is one geometry and no copy of it here. What a
// check adds is its own: it reads the beam AFTER the step, as the frame is drawn, and it judges only what is PLAIN at a margin either side of a rule's
// distance — the rules' clocks, their two distances and who is asked are what the checks hold; the geometry itself is held by light-wall's line of beams
// laid by hand under WALLFLOOR_BREAK=1.
// The meaning is the rules': a beam is into a wall when its MIDDLE ends on something that stands inside a distance of the lens and no ray of it runs clear
// to that distance. The ground is neither: a ray that comes down to the floor has ended, and not on a wall.

/** the game's beam geometry, bound to a game: the directions across a beam about `d` (its axis first) and what one ray from `o` along `r` ends on inside `m` */
export interface Beam { fan(d: number[]): number[][]; ray(o: number[], r: number[], m: number): 0 | 1 | 2 | 3 }
/** the margin a check keeps either side of a rule's distance, in metres along the beam */
export const BEAM_MARGIN = 0.15;
/** PLAINLY INTO A WALL at a rule's distance `m`: a margin inside `m` the middle of the beam from `o` about `d` already ends on something that stands (above
 *  the ground's band; or, `held`, on its foot too) and every other ray of it already ends, on such a thing or on the ground */
export const plainWall = (B: Beam, o: number[], d: number[], m: number, held = false): boolean =>
  B.fan(d).every((r, i) => { const e = B.ray(o, r, m - BEAM_MARGIN); return i === 0 ? e === 3 || (held && e === 2) : e !== 0; });
/** PLAINLY NOT at distance `m`: a margin past `m` some ray of it still meets nothing, wall or ground — or a margin inside `m` its middle is already down
 *  on the ground, with nothing standing in its way as far as a margin past `m` */
export const plainOpen = (B: Beam, o: number[], d: number[], m: number): boolean => { const fan = B.fan(d);
  return fan.some(r => B.ray(o, r, m + BEAM_MARGIN) === 0) || (B.ray(o, fan[0], m - BEAM_MARGIN) === 1 && B.ray(o, fan[0], m + BEAM_MARGIN) === 1); };
/** ON A FOOT at distance `m`: a margin either side of `m` the beam's middle ends on the foot of something that stands, inside the ground's band, and every
 *  other ray of it ends — a wall to a rule that held it a moment ago, not one to start on */
export const plainFoot = (B: Beam, o: number[], d: number[], m: number): boolean =>
  B.fan(d).every((r, i) => i === 0 ? B.ray(o, r, m - BEAM_MARGIN) === 2 && B.ray(o, r, m + BEAM_MARGIN) === 2 : B.ray(o, r, m - BEAM_MARGIN) !== 0);
