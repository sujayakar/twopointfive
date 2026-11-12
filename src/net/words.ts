// THE STATE'S WORD, as two machines of one networked game compare it. Host-neutral: the numbers are the game's own, wherever it stands.
import { NET } from './wire';

export const NET_WORD_SEED = 0x811C9DC5;
/** true for a tick (0-based) after which the server says its state word (wire.ts `wordEvery`) */
export const wordTick = (t: number): boolean => (t + 1) % NET.wordEvery === 0;
/** THE STATE'S WORD, chained from the word before it: after every tick the die's count of draws, the clock and where every man and leaf stands (`tick_word`); after a word
 *  tick, before the word is said, the hash of EVERY PART of the game (rust/game state_words) too. What a machine may honestly differ in is in none of them: a field
 *  says so beside itself (`#[bytes(skip)]`) */
export function foldWord(from: number, words: ArrayLike<number>): number {
  let h = from;
  for (let i = 0; i < words.length; i++) h = Math.imul(h ^ words[i], 0x01000193) >>> 0;
  return h;
}
