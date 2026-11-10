import { Vec3 } from '../math/vec';
import type { FloorMat } from './floors';

/** What the game layer needs from the audio engine (implemented in src/audio; null object until it is started). */
export interface GameAudioLike {
  play(name: string, pos: Vec3 | null, gain?: number, opts?: Record<string, unknown>): void;
  /** one footfall: `loudness` is the hearing-model level of the step (already scaled by the floor material — rust/game_rules/src/world/floor.rs tread), `mat` the material it landed on
   *  (sample choice and brightness; said by the game, so the mixer can never disagree with the guards' ears) */
  footstep(pos: Vec3, loudness: number, isPlayer: boolean, mat: FloorMat): void;
  /** the rain bed's level 0‥1 (weather.ts: the rate × where the listener stands; 0 = silence it). Optional. */
  rain?(level: number): void;
}
