/** THE PACES A REPLAY PLAYS AT, slow motion first. A pace is how fast the tape's steps are fed to the game and nothing else: a step is
 *  the recorded step whatever the pace — its inputs, its length, its sounds and what it is compared with — so slow motion is each recorded step held for longer,
 *  not a finer run. */
export const TAPE_SPEEDS = [0.25, 0.5, 1, 2, 4];
export const speedName = (s: number): string => s === 0.25 ? '¼×' : s === 0.5 ? '½×' : `${s}×`;
/** One pace slower (`dir` < 0) or faster than `s`; the ends hold, and a pace off the list counts as 1×. */
export const speedStep = (s: number, dir: number): number => { const i = TAPE_SPEEDS.indexOf(s); return TAPE_SPEEDS[Math.max(0, Math.min(TAPE_SPEEDS.length - 1, (i < 0 ? TAPE_SPEEDS.indexOf(1) : i) + Math.sign(dir)))]; };
