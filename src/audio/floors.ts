// WHAT IS UNDERFOOT, as the mixer has it: which of the two samples, how bright, its name. How loud a step on it is comes with the footfall (rust/game_rules/src/world/floor.rs tread).
export type FloorMat = 'carpet' | 'lino' | 'stone' | 'tile' | 'asphalt' | 'grass';

/** Per material: `hard` = which of the two footstep samples plays (heel click vs soft scuff), `rate` × the sample's playback rate (stone a touch brighter, carpet duller — subtle),
 *  `label` for the HUD / panel. */
export const FLOOR: Record<FloorMat, { hard: boolean; rate: number; label: string }> = {
  carpet:  { hard: false, rate: 0.94, label: 'carpet' },
  lino:    { hard: true,  rate: 1.0,  label: 'lino' },
  stone:   { hard: true,  rate: 1.08, label: 'stone' },
  tile:    { hard: true,  rate: 1.04, label: 'raised floor' },
  asphalt: { hard: true,  rate: 0.97, label: 'asphalt' },
  grass:   { hard: false, rate: 0.88, label: 'grass' },
};
