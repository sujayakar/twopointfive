// A RUN'S PAGE: an address the page loaded for itself as one page of a hands-off run — a scene of the benchmark (ui/benchrun.ts), a level of the pricing run
// (ui/pricerun.ts). Nobody sits at it and what it shows is measured, so it stands down what the page gives a visitor: the load brief (the world would wait behind
// it), the reload after a lost device (the run must see the card), the sandbox row this machine kept (with "no AI" kept the mission is refused), the look
// prototype's kit and its fog put down (another scene than the mission's own). ONE question asked in all those places, so a new kind of run adds its word here
// and is a run's page everywhere at once; and a mission picked from such a page never carries the word on (scene/scenarioList.ts quitSearch).
// A leaf: no imports, so any module may ask it at its top level.
export const BENCH_RUN_WORD = 'benchrun', PRICE_RUN_WORD = 'pricerun';
/** the words that ASK for a run: the benchmark's, the pricing run's */
export const BENCH_WORD = 'bench', PRICE_LEVELS_WORD = 'pricelevels';
/** … and the words that shape a pricing run on the address that asks (ui/pricerun.ts): a run's own, never a mission's */
export const PRICE_ASK_WORDS: readonly string[] = [PRICE_LEVELS_WORD, 'pricesecs', 'pricepage'];
export const RUN_PAGE_WORDS: readonly string[] = [BENCH_RUN_WORD, PRICE_RUN_WORD];
export const runPage = (q: URLSearchParams): boolean => RUN_PAGE_WORDS.some(w => q.has(w));
/** A RUN KEEPS ITS OWN PAGE (host/keeppage.ts `own`): the address that asks for one and every page of it load whole, each a boot of its own — that boot is what a
 *  run measures, and a run walks from page to page by loading the next address, which a frame under a kept shell is not free to do. */
export const ownPage = (q: URLSearchParams): boolean => q.has(BENCH_WORD) || q.has(PRICE_LEVELS_WORD) || runPage(q);
/** … and A PAGE WHOSE PICTURE IS PRICED: a level of the pricing run. Its address says the page's own words for a held picture (`quality=<preset>&adapt=0`:
 *  the ladder's one reader pins the preset and switches the ladder off before a frame is sampled; `giveway=0`: its window gives way to nobody), so nothing
 *  of the picture moves with this machine's frame rate and a level's two pages stand on the same rows (ui/pricerun.ts ONE RUNG). What the page still asks
 *  here: its sandbox row stands off for the page's whole life. Not the benchmark's scene: what its ladder does with the scene is part of what it reports. */
export const heldPage = (q: URLSearchParams): boolean => q.has(PRICE_RUN_WORD);
/** … the two words its address says for that, as the page reads them (ui/quality.ts visit; host/pace.ts giveway) · and their names: a mission picked from a
 *  priced page sheds them with the run's own word (scene/scenarioList.ts quitSearch) */
export const HELD_LADDER_WORD = 'adapt=0', HELD_WINDOW_WORD = 'giveway=0';
export const HELD_PAGE_WORDS: readonly string[] = [HELD_LADDER_WORD, HELD_WINDOW_WORD].map(w => w.split('=')[0]);
