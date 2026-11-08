// A NETWORKED GAME'S START: what every machine of it hands the game's own start (rust/game_start), so that they stand one game. The server says it in its
// welcome; the character's files are the build's, and every machine of a game is of one build.
import { specOfQuery } from '../scene/scenarioList';

/** `file`: the level's file (one of levels/) · `words`: the address's words, and nothing of any machine's own address · the die · how many seats · `commands`: what
 *  the host has set before the game is made (src/draw/commands.ts' bytes as base64; '' = nothing: the words say it all) */
export interface NetStart { file: string; words: string; seed: number; seats: number; commands: string }

/** A LEVEL FILE IS A NAME, never a path: the start comes from whoever asked for the game, and what it names is read off this machine's disk */
export const levelName = (file: unknown): file is string => typeof file === 'string' && /^[A-Za-z0-9][A-Za-z0-9_-]*\.json$/.test(file);
const BASE64 = /^[A-Za-z0-9+/]*={0,2}$/;
/** why no game can be stood from `s` ('' = one can) */
export function startWhy(s: unknown): string {
  const x = s as NetStart | null;
  if (!x || typeof x !== 'object') return 'the welcome carries no start';
  if (!levelName(x.file)) return `${String(x.file).slice(0, 60)} is not the name of a level file`;
  if (typeof x.words !== 'string' || x.words.length > 2048) return 'the start\'s words are not an address\'s';
  if (!specOfQuery(x.words)) return `the words ${x.words.slice(0, 60)} name no mission`;
  if (!Number.isInteger(x.seed) || x.seed < 0 || x.seed > 0xFFFFFFFF) return 'the start\'s die is not a whole number of 32 bits';
  if (x.seats !== 1 && x.seats !== 2) return `a game has one seat or two, not ${String(x.seats)}`;
  if (typeof x.commands !== 'string' || x.commands.length > 65536 || !BASE64.test(x.commands)) return 'the start\'s commands are not bytes';
  return '';
}
/** the commands' bytes of a start, and the other way */
export const commandBytes = (s: NetStart): Uint8Array => Uint8Array.from(atob(s.commands), c => c.charCodeAt(0));
export const commandText = (b: Uint8Array): string => { let s = ''; for (const x of b) s += String.fromCharCode(x); return btoa(s); };
