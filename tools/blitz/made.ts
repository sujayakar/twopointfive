// WHAT A CHECKOUT'S GAME IS MADE OF, AS ONE WORD: two checkouts of one word step one game alike, whatever else of them differs (how a page draws, what it says). A networked game is of one such word, so
// a page built since plays on in a game begun before it, where nothing of the game itself changed.
import { createHash } from 'node:crypto';
import { readdirSync, readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { characterFiles } from './character.ts';
import { sourcesHash } from './face-build.ts';

const ROOT = resolve(import.meta.dir, '../..');
/** the module's sources, the levels, the characters a game stands, and how a game is stepped and its words said between machines (src/net) */
export function gameMade(): string {
  const h = createHash('sha256').update(sourcesHash());
  for (const [dir, ext] of [['levels', '.json'], ['src/net', '.ts']]) for (const n of readdirSync(resolve(ROOT, dir)).filter(n => n.endsWith(ext)).sort()) h.update(n).update('\0').update(readFileSync(resolve(ROOT, dir, n))).update('\0');
  for (const f of characterFiles(ROOT)) h.update(f);
  return h.digest('hex').slice(0, 16);
}
