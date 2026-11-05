// THE GAME'S OWN NUMBERS OF THE PICTURE. Some of what the simulation works out depends on numbers the picture also has: what a night vision tube makes of
// a dark floor depends on the exposure the scene is taken at and on the tube's gain; an illuminator's strength is said as what the tube makes of it. The
// picture's copies live in the engine's settings, and a player moves them: Brightness (Options > Display), the bracket keys, the debugger's sliders. Those
// are HIS: two players of one game have two, a recording carries neither, and a server has its own. So the game has its own, and no sum of the simulation
// reads the picture's:
//   ENV            a knob table (knobs.ts: a recipe, a recording and a game server's setup carry it): the exposure of a level's own night and the tube's
//                  gain, equal to the picture's own defaults (src/settings.ts; tools/qa/settings-check.ts holds the two equal)
//   exposureOf(g)  (hours.ts) the exposure the game's scene is taken at now: the hour's own when an hour is in, the show's day's when that is, else the
//                  night's here. Worked out from state the game already keeps (which hour, whether the day is in): there is no second copy to fall out of step
//   tubeGain()     the tube's
// nvg.ts and irlight.ts read these. The picture is told as it always was (hours.ts and lighting.ts write the engine's settings), and a player's Brightness
// is a factor of his picture alone.
// NEXT OF THE SAME KIND, still read from the engine's settings by game code and named in tools/qa/settings-check.ts until they move here: the sky's two
// colours for the hour, the emissive scale, a torch's radius, the debris cap. And one that is not a setting and so not on that list: the picture's
// CAMERA (engine.viewFocus, read by wind.ts, airSim.ts, fire.ts and snowBreath.ts): two players of one game have two.
// (This file imports nothing: src/game is one import cycle, and a table every other module may read at any moment must not wait on one of them.)

const BUILT = { exposure: 2.0, tubeGain: 475 };
/** the level's own night's exposure (the linear gain the scene is taken at: the picture's settings.exposure before any turn of Brightness) and the
 *  intensifier's log gain (the picture's settings.nvGain) */
export const ENV = { exposure: BUILT.exposure, tubeGain: BUILT.tubeGain };
/** the build's own numbers back: a recording or a recipe from before the table was made on them (journal.ts applyKnobs, replay/recipestand.ts) */
export function envAsBuilt(): void { ENV.exposure = BUILT.exposure; ENV.tubeGain = BUILT.tubeGain; }
/** the tube's gain (never under 1: the curve is ln(1 + L·gain) / ln(1 + gain)) */
export function tubeGain(): number { return Math.max(ENV.tubeGain > 0 ? ENV.tubeGain : 1, 1); }
