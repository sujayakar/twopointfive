import { Vec3 } from '../math/vec';

/** What the game layer needs from the audio engine (implemented in src/audio; null object until it is started). */
export interface GameAudioLike {
  play(name: string, pos: Vec3 | null, gain?: number, opts?: Record<string, unknown>): void;
  footstep(pos: Vec3, loudness: number, isPlayer: boolean): void;
}
export const nullAudio: GameAudioLike = { play() { /* silent */ }, footstep() { /* silent */ } };
