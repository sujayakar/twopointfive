// WHAT A GAME WAS STOOD WITH: its own record, set once at the stand, and the one rules read. The module tables it is taken from (weatherKinds.ts WEATHER_BOOT,
// hours.ts HOUR, kit.ts KIT_SOLO) are PICKS, what a page, a menu or a tool chose before any game exists: they are the process's, no rule reads them, and the
// bridge to the Rust game does not carry them, so one game's record cannot leak over the next game's and a page and a headless server hold the same one.
import { WEATHER_BOOT, type WeatherKind } from './weatherKinds';
import { HOUR, type HourId } from './hours';
import { KIT_SOLO } from './kit';

export interface StoodWith {
  /** the weather the game is born in, null the level's own */
  weather: WeatherKind | null;
  /** the weather is the address's for the life of the game (Weather.setKind refuses another, canLoad refuses a checkpoint made in another) */
  locked: boolean;
  /** the hour the stand lays over the level */
  hour: HourId;
  /** that hour's sun's strength, 0 to 1 */
  sun: number;
  /** the strength was said (?sun=, PT_SUN): it stands whatever cloud a picked weather brings */
  sunGiven: boolean;
  /** the hour holds the level's own storm off */
  dry: boolean;
  /** a harness game stood plain, whose missions ask the solo card's question by kitSoloStand */
  soloStand: boolean;
}

/** a game stood with nothing said for it takes the picks as they stand: a fresh object, so that nothing it holds is shared with them */
export function stoodOfPick(): StoodWith {
  return { weather: WEATHER_BOOT.kind, locked: false, hour: HOUR.id, sun: HOUR.sun, sunGiven: HOUR.sunGiven, dry: HOUR.dry, soloStand: KIT_SOLO.stand };
}
