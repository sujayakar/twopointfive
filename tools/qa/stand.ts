// THE STAND A TOOL RUNS ON, WRITTEN DOWN ONCE (2026-11-05). The rule is src/host/standEnv.ts standOf: with no word a tool stands the game as the page
// opens it ('shipped', and saying so is the same run); 'frozen' is the old stand, every newer rule held off (src/host/profile.ts FROZEN_HOLDS), kept by that
// one word for the switch cleanup until the off paths are gone; any other word stops the tool.
//
// This file WRITES the word into the environment as it loads, so that a tool which reads `process.env.TPS_PROFILE` itself, and every child it starts, sees
// the stand's name and never "no word". A tool that reads the word imports this file (or headless.ts, which does) as a STATIC import: static imports load
// before a module's own lines, so the read cannot come first. `shipped-check.ts` is red on a tool that reads the word with neither, unless it names its
// stand itself first. It imports nothing of the game: a tool that only starts children can load it.
// A RUN ON THE OLD STAND SAYS SO, once, on stderr: the process the word was given to says it and its children do not (QA_STAND_SAID rides the
// environment), so a tool that asks the old stand of a child by name keeps that child's stderr for the child's own words.
import { standOf, STAND_ENV, type Stand } from '../../src/host/standEnv.ts';
export const STAND: Stand = standOf(process.env.TPS_PROFILE);
process.env.TPS_PROFILE = STAND;
if (STAND === 'frozen' && !process.env.QA_STAND_SAID) console.error('THE OLD (FROZEN) STAND, by TPS_PROFILE=frozen: every newer rule held off. Kept for the switch cleanup only: with no word a tool stands the game as the page opens it.');
process.env.QA_STAND_SAID = '1';

/** THE WORDS OF THE STAND THIS PROCESS WAS GIVEN BY HAND, as it started: every word of the stands beside the profile (src/host/standEnv.ts STAND_ENV: a rule
 *  raised or held off by name) and a check's own arm (QA_HOLD). Read once, here, before any tool's own line: a tool that sets a word itself, or a probe that
 *  writes one and deletes it, does not change what the run was started with. A run with any of them is not the stand as it opens. */
export const BY_HAND: readonly string[] = handWords();
/** …and the same question asked now (tools/qa/ident-pin.ts asks it after its tool has set its own words): `WORD=value` for every such word in the environment */
export function handWords(): string[] { return [...STAND_ENV, 'QA_HOLD'].filter(k => k !== 'TPS_PROFILE' && process.env[k] !== undefined).sort().map(k => `${k}=${process.env[k]}`); }
/** THE WORDS A HELD ROW IS RAISED BY: the first WORD=value of each row's note in src/host/profile.ts FROZEN_HOLDS, once each (a row whose note names no word
 *  of its own has none). What tools/qa/stand-bisect.ts searches over, and what a row of tools/qa/expected-red.ts may name. */
export function heldWords(holds: Record<string, string>): string[] {
  const words: string[] = [];
  for (const note of Object.values(holds)) { const w = wordOf(note); if (w && !words.includes(w)) words.push(w); }
  return words;
}
/** the word a row's note opens with, or none */
export const wordOf = (note: string): string | undefined => /^([A-Z][A-Z0-9_]*=[A-Za-z0-9-]+)/.exec(note)?.[1];
