// SO MANY AT A TIME. A check that starts a child a case (a page boot, a played run) starts `jobs` of them side by side and the next as each ends: started all
// at once, forty whole games in forty processes take a small box's memory and the kernel kills one of them, which reads as a red of the game's. Results come
// back in the order of `items`. When one case throws, the pool throws that and starts no case after it (those already running end by themselves). It imports
// nothing: a tool that only starts children can load it.
export async function pool<T, R>(items: readonly T[], jobs: number, fn: (x: T) => Promise<R>): Promise<R[]> {
  const out: R[] = new Array(items.length), n = Math.max(1, Math.min(Number.isFinite(jobs) ? Math.floor(jobs) : 1, items.length)); let k = 0, threw = false;
  const worker = async () => { while (!threw && k < items.length) { const i = k++; try { out[i] = await fn(items[i]); } catch (e) { threw = true; throw e; } } };
  await Promise.all(Array.from({ length: n }, worker));
  return out;
}
/** how many a tool runs side by side, from what its own word or flag was given: that number, whole, no fewer than one and no more than `most` (so a box short
 *  of memory that asks for the least gets one); nothing given, or no number at all, is `byDefault` */
export const jobsOf = (given: string | undefined, byDefault: number, most = Infinity): number => { const n = given === undefined || given.trim() === '' ? NaN : Math.floor(Number(given)); return Math.min(most, Number.isFinite(n) ? Math.max(1, n) : byDefault); };
