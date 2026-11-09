// The mixer's LOOPS as data: what the room hums are made of and how a smoke canister's hiss is shaped. One place, read by the mixer (audio.ts setupAmbience / hiss) and by the
// listening page (src/sounds/main.ts), so the page plays what the game plays. No game code is imported here.
import type { SfxBuffer, SfxContext } from './backend';
import { noiseBuffer, toneNoise } from './webaudio';

export type AmbienceKind = 'hum' | 'servers' | 'wind' | 'buzz';
/** what hums in a room, by its kind (a level says where: docs/level-format.md 'Hums'): its loop (fresh noise each call, off the system die) and its own tone filter */
export interface AmbienceLoop { made: (sc: SfxContext) => SfxBuffer; filter: { type: 'lowpass' | 'bandpass'; freq: number; Q?: number }; wobble?: { rateHz: number; depthHz: number } }
export const AMBIENCE: Record<AmbienceKind, AmbienceLoop> = {
  servers: { made: sc => toneNoise(sc, 3, [118, 236, 355], 0.5), filter: { type: 'lowpass', freq: 1800 } },
  hum: { made: sc => toneNoise(sc, 3, [50, 100, 150], 0.15), filter: { type: 'lowpass', freq: 600 } },
  buzz: { made: sc => toneNoise(sc, 2, [100, 200, 300, 400], 0.05), filter: { type: 'bandpass', freq: 240, Q: 2 } },
  wind: { made: sc => noiseBuffer(sc, 4), filter: { type: 'lowpass', freq: 500, Q: 0.5 }, wobble: { rateHz: 0.11, depthHz: 250 } },
};
/** what the listening page plays of each, at a level the office's has */
export const HUMS: { what: string; kind: AmbienceKind; base: number }[] = [
  { what: 'server room', kind: 'servers', base: 0.5 }, { what: 'vending machines', kind: 'buzz', base: 0.35 }, { what: 'air handling', kind: 'hum', base: 0.25 }, { what: 'wind outside', kind: 'wind', base: 0.5 },
];
/** the ambience panners' distance law */
export const AMBIENCE_DISTANCE = { ref: 2, rolloff: 1 };

/** a smoke canister's hiss: a 2 s noise loop through a band, risen over `rise`, held, faded over the last `tail` seconds */
export const HISS = { seconds: 2, filter: { type: 'bandpass' as const, freq: 2600, Q: 0.8 }, peak: 0.5, rise: 0.3, tail: 2, distance: { ref: 1.2, rolloff: 1.3 }, pop: 0.9 };
