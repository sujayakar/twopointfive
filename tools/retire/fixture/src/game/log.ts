/** what a case did besides answer: dice drawn, calls made, modules loaded */
export const LOG = { draws: 0, calls: 0, sum: 0, loaded: 0 };
let seed = 7;
/** the game's die: every draw moves it, so a draw the fold lost changes every answer after it */
export function die(): number { LOG.draws++; seed = (seed * 1103515245 + 12345) & 0x7fffffff; return seed / 0x7fffffff; }
export function tell(x: number): number { LOG.calls++; LOG.sum += x; return x; }
export function reset(): void { LOG.draws = 0; LOG.calls = 0; LOG.sum = 0; seed = 7; }
