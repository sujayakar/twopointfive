// A RUN'S PAGE: an address the page loaded for itself as one page of a hands-off run — a scene of the benchmark (ui/benchrun.ts), a level of the pricing run
// (ui/pricerun.ts). Nobody sits at it and what it shows is measured, so it stands down what the page gives a visitor: the load brief (the world would wait behind
// it), the reload after a lost device (the run must see the card), the sandbox row this machine kept (with "no AI" kept the mission is refused), the look
// prototype's kit and its fog put down (another scene than the mission's own). ONE question asked in all those places, so a new kind of run adds its word here
// and is a run's page everywhere at once; and a mission picked from such a page never carries the word on (scene/scenarioList.ts quitSearch).
// A leaf: no imports, so any module may ask it at its top level.
export const BENCH_RUN_WORD = 'benchrun', PRICE_RUN_WORD = 'pricerun';
export const RUN_PAGE_WORDS: readonly string[] = [BENCH_RUN_WORD, PRICE_RUN_WORD];
export const runPage = (q: URLSearchParams): boolean => RUN_PAGE_WORDS.some(w => q.has(w));
