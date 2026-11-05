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
import { standOf, type Stand } from '../../src/host/standEnv.ts';
export const STAND: Stand = standOf(process.env.TPS_PROFILE);
process.env.TPS_PROFILE = STAND;
if (STAND === 'frozen' && !process.env.QA_STAND_SAID) console.error('THE OLD (FROZEN) STAND, by TPS_PROFILE=frozen: every newer rule held off. Kept for the switch cleanup only: with no word a tool stands the game as the page opens it.');
process.env.QA_STAND_SAID = '1';
