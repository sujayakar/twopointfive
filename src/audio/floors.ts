// WHAT IS UNDERFOOT, as the mixer and the guards' ears both have it: how loud a step on it is, which of the two samples, how bright.
export type FloorMat = 'carpet' | 'lino' | 'stone' | 'tile' | 'asphalt' | 'grass';

/** Per material: `loud` × the per-step hearing LEVEL (the GameEvent the guards score, player.noise, and the mixer's gain alike — lino = 1 is the reference the
 *  corridor probes were tuned on), `hard` = which of the two footstep samples plays (heel click vs soft scuff), `rate` × the sample's playback rate (stone a touch
 *  brighter, carpet duller — subtle), `label` for the HUD / panel.
 *  What the multipliers come to against rust/game_rules/src/people/guards hearing_check (a step counts when level × (1 − d/9) × carry > 0.12, d the sound path in metres, calm floor):
 *  a walking step (0.26) carries 4.8 m over lino, 5.2 m over stone, 2.0 m over carpet, nowhere over grass; a crouched one (0.125) a hand's width over lino
 *  and nothing at all over carpet; a sprinting one (1.0) 7–8 m whatever is underfoot. And since a calm man sheds 0.1 awareness a second, walking to and fro
 *  three metres behind him is a 'huh?' in about three seconds on lino and never on carpet. */
export const FLOOR: Record<FloorMat, { loud: number; hard: boolean; rate: number; label: string }> = {
  carpet:  { loud: 0.6,  hard: false, rate: 0.94, label: 'carpet' },
  lino:    { loud: 1.0,  hard: true,  rate: 1.0,  label: 'lino' },
  stone:   { loud: 1.1,  hard: true,  rate: 1.08, label: 'stone' },
  tile:    { loud: 1.05, hard: true,  rate: 1.04, label: 'raised floor' },
  asphalt: { loud: 0.9,  hard: true,  rate: 0.97, label: 'asphalt' },
  grass:   { loud: 0.45, hard: false, rate: 0.88, label: 'grass' },
};
