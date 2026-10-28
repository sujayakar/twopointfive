// THE LOOK PROTOTYPE's LIGHT VALUES IN REAL UNITS (the page /look; the panel's "Real light values" row, render/frame.ts lookConfig.lamps). The maintainer, 10-25:
// "we don't have consistent, physically accurate light values (lumens?) across the image + the right HDR mapping to screen", and 10-28, of fixing that
// properly: "I think my answer would be yes".
//
// WHAT THE NUMBERS ALREADY ARE. Every level is authored as a night, and its fixtures are real: a point light's `intensity` I lights a face d metres away with
// I / d², and with ONE UNIT = 3.3 LUX a store's bulb (26) is 86 candela, about a 1000 lumen bulb, and puts 15 lux on the floor under it; a street lamp (260 from
// 5 m) 34 lux; a lantern (4.5) 15 candela. What is NOT real at night is everything a real night would white the picture out with: a weapon light is 10,000
// candela where the game's is 200, a headlamp 15,000 where the game's is 400 to 500, a flash or a blast more again. Those are held down so that the night's one
// exposure can show them, as a camera's highlights are, and the night keeps them as they are.
// An HOUR lays a sun of 1 to 6 over that night with nearly the night's exposure, where the real sun is 100,000 lux = 30,000 units. So an hour's sun and sky are
// not in the night's units at all: they are the real ones ALREADY MULTIPLIED by that hour's exposure (the renderer's buffers hold exposed light, which is what
// keeps its thresholds and clamps where they were tuned). The hour's sun and its sky stand in a believable proportion to each other (midday: the sky gives 0.29
// of what the sun puts on the ground, a real clear noon 0.2 to 0.25) and are left as authored.
//
// THE RULE. What an hour's picture stands for is in HOUR_SUN_LUX; from it the hour's EXPOSURE (hourExposure: 1 at night, 1/5,050 at midday). Every other source
// is drawn at   min(as authored, its real strength × the hour's exposure):   a fixture's real strength is its authored one, a hand light's and a headlamp's are
// TRUE_CD, a flash's FLASH_TRUE × its authored one, a glowing face's EMISSIVE_TRUE × its authored one. The `min` is the night's hold: nothing is drawn brighter
// than the night draws it. The weather's cloud dims the sun and the sky under an hour's one exposure (game/hours.ts), so under rain at noon the lamps count for
// more by themselves.
//
// PICTURE ONLY, on the look page only: applied to what the GPU is handed (render/lights.ts upload, the frame's emissive scale) and never written to a light, so
// the game's own readers of the list (guards.ts lightEstimateAt) still take the authored strength. That break is to be settled before any of it goes to the build.
// The check: tools/qa/lookunits-check.ts.

/** lux per unit of the renderer's irradiance (so candela per unit of a light's `intensity`): the night's fixtures say so (above) */
export const LUX_PER_UNIT = 3.3;

/** WHAT EACH HOUR'S SUN STANDS FOR: the illuminance (lux) on a face turned to it under that hour's own sky, which the hour's authored `sun.intensity` is the
 *  exposed picture of. midday: a clear tropical noon. overcast: the bright patch of a deck over a 10,000 lux day (the hour's sky carries the other 7,000).
 *  dawn / dusk: the first and last minutes of sun, 600 and 500 lux, with the photocell's lamps already burning (HourDef.lamps), as the hour is drawn: its sun
 *  stands 22° up for the length of its shadows, not for its strength. */
export const HOUR_SUN_LUX: Readonly<Record<string, number>> = { dawn: 600, midday: 100000, overcast: 3800, dusk: 500 };

/** real peak candela of the beamed sources the night holds down: a weapon or hand light (a 1,000 lumen one throws 10,000 to 16,000), a vehicle's dipped headlamp */
export const TRUE_CD = { hand: 10000, vehicle: 15000 } as const;
/** a flash's or a blast's real light over the frames it is drawn for, × its authored one: a rifle's flash is some 100,000 candela for a millisecond or two, 500
 *  units over the game's 0.09 s where the game has 130 to 180 (× 3); a stun grenade's million for ten is 21,000 over its 0.14 s where the game has 1,600 (× 13) */
export const FLASH_TRUE = 6;
/** a glowing face's real luminance × its authored one: a lamp's glass is some 10,000 cd/m² where the game's is 3 units = 10 */
export const EMISSIVE_TRUE = 1000;

/** the hour's exposure against the night's (1): what a real illuminance is multiplied by to be the number the renderer draws at that hour.
 *  `sunAuthored` = the hour's own `sun.intensity` (game/hours.ts HOURS, before any cloud). An hour this table does not know is a night. */
export function hourExposure(id: string, sunAuthored: number): number {
  const lux = HOUR_SUN_LUX[id]; return lux && sunAuthored > 0 ? Math.min(1, sunAuthored * LUX_PER_UNIT / lux) : 1;
}

export type LightClass = 'none' | 'fixture' | 'hand' | 'vehicle' | 'flash';
interface LightLike { name: string; group: string; kind: number; ttl: number; transient?: boolean; card?: boolean; steady?: unknown; intensity: number; peakIntensity: number }
const VEHICLE = /^(van|gun_truck|truck|pickup|lorry|jeep|car)\w*_lamp/;
/** what a light is, for the rule. NONE (drawn as authored at every hour): the sun and the moon (the hour's own), a lightning stroke (the sky's), a device's
 *  sight cone (a tell, not a lamp) and the Team & kit figure's staging. */
export function lightClass(l: LightLike): LightClass {
  if (l.kind === 2 || l.transient || l.group === 'devices' || l.name.startsWith('kitview')) return 'none';
  if (l.ttl >= 0 || l.group === 'tracer' || l.steady) return 'flash';   // (steady: a light game/fire.ts flickers, a fire's own: held down at night as a flash is)
  if (l.card || l.group === 'flashlights') return 'hand';
  return VEHICLE.test(l.name) ? 'vehicle' : 'fixture';
}
/** × a light's uploaded strength at an hour of exposure `exp` (1 = a night: 1 exactly). `amount` 0‥1 = the panel's row, how far toward the real value
 *  (in stops: 0 = as authored, 1 = real). */
export function lightDayK(l: LightLike, exp: number, amount = 1): number {
  if (exp >= 1 || amount <= 0) return 1;
  const c = lightClass(l); if (c === 'none') return 1;
  const authored = l.peakIntensity > 0 ? l.peakIntensity : l.intensity;
  const real = c === 'fixture' ? 1 : c === 'flash' ? FLASH_TRUE : authored > 0 ? TRUE_CD[c] / LUX_PER_UNIT / authored : 1;   // (real strength ÷ authored)
  return Math.pow(Math.min(1, Math.max(real, 1) * exp), Math.min(1, amount));
}
/** × the frame's emissive scale at that hour: a glowing face as the eye sees it */
export function emissiveDayK(exp: number, amount = 1): number { return exp >= 1 || amount <= 0 ? 1 : Math.pow(Math.min(1, EMISSIVE_TRUE * exp), Math.min(1, amount)); }
