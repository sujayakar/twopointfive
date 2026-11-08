// THE STATE'S WORDS, as two machines of one networked game compare them. Host-neutral: the numbers are the game's own, wherever it stands.
import { NET } from './wire';
import { NET_AGREED } from './agreed';

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
/** some of the WHOLE game's words (a word a part: rust/game state_words) folded into one, not chained: each word tick's stands alone, so a tick that differs is
 *  one tick counted. `ix`: which of them, null = every one */
export function foldWhole(words: ArrayLike<number>, ix: readonly number[] | null): string {
  let h = NET_WORD_SEED;
  if (ix) for (const i of ix) h = Math.imul(h ^ words[i], 0x01000193) >>> 0; else for (let i = 0; i < words.length; i++) h = Math.imul(h ^ words[i], 0x01000193) >>> 0;
  return h.toString(36);
}
/** the places in `names` of the parts every kind of host agrees on (agreed.ts), by NAME: a part the module has not got is passed over */
export function agreedOf(names: readonly string[]): number[] { const on = new Set(NET_AGREED); return names.map((n, i) => on.has(n) ? i : -1).filter(i => i >= 0); }
