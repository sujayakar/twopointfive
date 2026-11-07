// THE HARNESS'S DIE. No game a tool stands rolls the clock's dice.
// A game stood with no seed takes its die off the tool's `Math.random` (the Rust game's seed is one draw of it as the game is stood, tools/blitz/rust-host.ts),
// so a tool that had put no die there stood ANOTHER GAME EVERY RUN: a check read green on one run and red on the next, and nothing said which game it had
// asked. Two places close that, and both read this file:
//   · the stand-up (headless.ts standUp): a game stood with no seed while `Math.random` is still the engine's own gets the harness's die put there first, for
//     the rest of the process;
//   · the fence round a probe (holds.ts asProbe): every probe begins on the harness's die, and whatever die it leaves in `Math.random` is taken out after it.
// A tool that names a stream (`Math.random = seeded(7)`, `standUp({ seed })`, TPS_SEED) is as it was.
// QA_STAND_DIE=0 in front of a tool: neither is done (the harness as it was until 2026-11-07), for standdie-check.ts's red path; any other value but 1 stops the tool.
import { mulberry32 } from '../../src/math/dice.ts';

/** the seed of the harness's die */
export const HARNESS_SEED = 1;
/** mulberry32, the die every tool's own `seeded` is (src/math/dice.ts) */
export const seeded = mulberry32;
const ON = ((v: string | undefined): boolean => { if (v === undefined || v === '1') return true; if (v === '0') return false; throw new Error(`QA_STAND_DIE=${v}: want 0 (no harness die: the clock's dice, as it was) or 1`); })(process.env.QA_STAND_DIE);
/** `Math.random` is the engine's own: no tool has put a die in its place. Asked of the function itself, not of a copy kept as this file loaded: a tool may
 *  seed first and load the harness after. */
export const clockDice = (): boolean => /\[native code\]/.test(Function.prototype.toString.call(Math.random));
/** a new die at the start of the harness's stream, or null where the harness's die is switched off (QA_STAND_DIE=0) */
export const harnessDie = (): (() => number) | null => ON ? mulberry32(HARNESS_SEED) : null;
