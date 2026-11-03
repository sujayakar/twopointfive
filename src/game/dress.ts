// DRESSING: the steps of a tick that only make the picture (docs/agents/rust.md, gameplay and dressing).
// A row here names a step the tick takes for whoever is looking: with the row false the step is not taken, and THE GAME IS THE SAME GAME TO THE BIT
// (tools/qa/dress-check.ts plays the comparison missions with every row down and with every row up and compares the raw bits after every update,
// game/digest.ts rawBits). That is what "dressing" means in this repo, and the only way onto this list: a step whose absence moves one bit of what the
// comparison hashes is gameplay, runs on the server, and has no row. A host that draws nothing (the game server) may put every row down.
// Every row is true in the page and on both stands: this table is not a rule of play, so no recording carries it and no stand holds it.
// What a dressing step may do: read anything; write what only the picture reads (the GPU's lists, a drawn box, a particle, a look's own side table).
// What it may not: push a sound a man hears, draw from the game's die, move a clock a rule reads, write a field a rule reads (a pose, a light), or put
// smoke where the senses read it (`smoke.emit` feeds the CPU's own smoke, sense.ts ownSmoke; `smoke.push` and the wind move only the solver's).
// A new row is a step READ for those and then passed by the check: the check alone sees only what its five missions do.
export const DRESS = {
  air: true,        // the air as a simulated field handed to the engine (airSim.ts)
  moths: true,      // the moths under the lamps (moths.ts): their orbits and their specks
  snow: true,       // lying snow as a surface, the prints and marks pressed into it, a blast's dig (snowField / snowTracks / snowMarks / snowBlast)
  rain: true,       // the near drops, the drips and the stand-back disc (rainFx.ts)
  fires: true,      // the open fires as the picture draws them (fire.ts fireDraws; the lamp's flicker itself is a light, and gameplay)
  wake: true,       // men wading through smoke push the solver's smoke (the picture's smoke; the CPU's own smoke is not pushed)
  men: true,        // the men handed to the renderer: skin instances, materials, shadow proxies (game.ts drawMen; what each WEARS is still dealt: his kit is placed off it)
  cues: true,       // the defenders' cues read off the settled frame for the player (cues.ts)
  recap: true,      // every death explains itself (deathrecap.ts): reads only
  blastFeel: true,  // a wall charge going off: dust ring, soot, the camera's kick written for the render (blastFeel.ts)
  fixtures: true,   // a fixture that broke: its tray, its glass, the arcing lead (fixtureBreak.ts)
  fx: true,         // the short-lived drawn boxes (a muzzle flash's card, a chip): their time to live
  stature: true,    // each man's drawn scale about his feet (stature.ts)
  cases: true,      // spent cases (particleFx.ts updateParticleFx)
  net: true,        // the squad's net: the click a line arrives with (squadnet.ts)
  wind: true,       // the wind's sources to the particles and the solver (wind.ts)
};
export type DressRow = keyof typeof DRESS;
/** every row up or down at once: a host that draws nothing puts them down */
export function dressAll(on: boolean): void { for (const k of Object.keys(DRESS) as DressRow[]) DRESS[k] = on; }
