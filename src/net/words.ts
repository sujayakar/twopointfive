// THE STATE'S WORDS, as two machines of one networked game compare them. Host-neutral: the numbers are the game's own, wherever it stands.
import { NET } from './wire';

export const NET_WORD_SEED = 0x811C9DC5;
/** true for a tick (0-based) after which the server says its state word (wire.ts `wordEvery`) */
export const wordTick = (t: number): boolean => (t + 1) % NET.wordEvery === 0;
/** THE STATE'S WORD after a tick, chained from the word before it (wire.ts: after a word tick it is the alarm): the die's count of draws and the hash of every
 *  part of the game's block, in the block's order (every body, door and thing; not a seat's aim, a man's mind or an order book) */
export function foldWord(from: number, die: number, parts: ArrayLike<number>): number {
  let h = Math.imul((from ^ die) >>> 0, 0x01000193) >>> 0;
  for (let i = 0; i < parts.length; i++) h = Math.imul(h ^ parts[i], 0x01000193) >>> 0;
  return h;
}
/** the WHOLE game's words (a word a part: rust/game state_words) folded into one, not chained: each word tick's stands alone, so a tick that differs is
 *  one tick counted */
export function foldWhole(words: ArrayLike<number>): string {
  let h = NET_WORD_SEED;
  for (let i = 0; i < words.length; i++) h = Math.imul(h ^ words[i], 0x01000193) >>> 0;
  return h.toString(36);
}
