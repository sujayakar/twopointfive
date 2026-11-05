// THE LOOK'S MAPS ARE MADE ON WORKERS (11-05). A map is a third of a second of arithmetic and a level names twenty: seconds of one thread, which the boot
// waits for (main.ts awaits gbuffer.needLook beside the pipelines). Where the shaders are compiling for half a minute nobody sees it; on a machine whose
// shaders are cached it is the last thing the start card waits for. The layers do not depend on each other, so each is made by render/lookTex.ts lookLayer on
// a worker and handed back (its two arrays are moved, not copied). THE SAME BYTES: a worker runs the same file (lookWorkerSource.ts hands it over as text when
// the page is bundled) on the same engine AND THE SAME MATHS (the page computes on the game's own sin and cos, math/exact.ts; a worker is told which with
// each layer), and tools/qa/lookworkers-check.ts sets every layer made there against the one made here, on both.
// WHERE THERE ARE NONE (no Worker in this place, one that will not start or does not answer, a layer a worker failed on): the caller makes the layer itself,
// in slices, as it always did (gbuffer.ts makeLayers). So the workers are only ever a faster way to the same maps. A place where a worker itself failed is
// said once on the console and remembered for the tab (a mission is a page load): no more are started there.
// THE SWITCH: LOOK_WORKERS.on — ON in the page (`?lookworkers=0` = every layer on the page's own thread, as it was), OFF here, so a tool under bun makes its
// maps as it did. Nothing of the game reads it and no recording carries it: the maps are the same either way.
import type { LookMaps } from './lookTex';
import { EXACT } from '../math/exact';

/** `most` = workers at once at the most (a layer holds about 10 MB while it is made); one core is always left to the page. `waitMs` = how long a worker may
 *  hold a layer before it is taken for dead (a layer is under a second of work; a boot must never wait on a worker that will not answer) */
export const LOOK_WORKERS = { on: false, most: 8, waitMs: 30000 };
/** a worker's whole text: render/lookWorkerMain.ts bundled by itself, written into render/lookWorkerText.ts when the page is bundled. Asked for only when a
 *  worker is about to be started (a check reads it, and breaks it) */
export const lookWorkerText = async (): Promise<string> => (await import('./lookWorkerText')).LOOK_WORKER_TEXT;
/** why no more workers are started here ('' = they are): the first worker that failed as a worker (would not start, did not answer, sent nonsense) */
let gaveUp = '', asked = false;
/** THE TAB REMEMBERS (sessionStorage): a mission is a page load, and a place whose workers start and never answer would wait `waitMs` at every one */
const GAVE_UP_KEY = 'twopointsix.lookWorkersGaveUp';
const tabStore = (): Storage | null => { try { return typeof sessionStorage !== 'undefined' ? sessionStorage : null; } catch { return null; } };
function remembered(): string { if (!gaveUp && !asked) { asked = true; try { gaveUp = tabStore()?.getItem(GAVE_UP_KEY) ?? ''; } catch { /* no storage here */ } } return gaveUp; }
export const lookWorkersGaveUp = (): string => remembered();
/** (a check starts over) */
export function lookWorkersAgain(): void { gaveUp = ''; asked = false; try { tabStore()?.removeItem(GAVE_UP_KEY); } catch { /* no storage here */ } }
function giveUp(why: string): void { if (remembered()) return; gaveUp = why; try { tabStore()?.setItem(GAVE_UP_KEY, why); } catch { /* no storage here */ } console.warn(`look maps: made on the page's own thread from here on (${why})`); }

interface Job { id: number; layer: number; n: number; even: boolean; exact: boolean; res: (m: LookMaps) => void; rej: (why: Error) => void; timer?: ReturnType<typeof setTimeout> }
interface Reply { id: number; a?: Uint8Array; b?: Uint8Array; tone?: number; hollow?: number; error?: string }

/** a few workers for one batch of layers: await open(), make() each layer, close(). A job's promise is refused where its worker failed; the caller makes that layer itself */
export class LookPool {
  private idle: Worker[] = []; private all: Worker[] = []; private queue: Job[] = []; private at = new Map<Worker, Job>(); private ids = 1; private url = '';
  private constructor() { /* open() */ }
  /** how many workers it started */
  get size(): number { return this.all.length; }
  /** workers for `jobs` layers, or null where there are none to be had (the rule down, no Worker here, one that failed here before, a text that is not there) */
  static async open(jobs: number, given?: string): Promise<LookPool | null> {
    if (!LOOK_WORKERS.on || remembered() || jobs < 1 || typeof Worker === 'undefined' || typeof Blob === 'undefined' || typeof URL === 'undefined' || typeof URL.createObjectURL !== 'function') return null;
    const cores = typeof navigator !== 'undefined' && navigator.hardwareConcurrency > 0 ? navigator.hardwareConcurrency : 2;
    const n = Math.max(1, Math.min(jobs, LOOK_WORKERS.most, cores - 1)), pool = new LookPool();
    try {
      const text = given ?? await lookWorkerText();
      if (!text) { giveUp('this build carries no worker text'); return null; }
      pool.url = URL.createObjectURL(new Blob([text], { type: 'text/javascript' }));
      for (let i = 0; i < n; i++) {
        const w = new Worker(pool.url, { name: 'look maps' });   // (a plain script, not a module: a module from a blob is refused on a page opened from disk)
        w.onmessage = e => pool.done(w, e.data as Reply);
        w.onerror = e => { (e as Event).preventDefault?.(); pool.lost(w, `a worker failed: ${(e as ErrorEvent).message || 'it would not start'}`); };
        w.onmessageerror = () => pool.lost(w, 'a worker sent what could not be read');
        pool.all.push(w); pool.idle.push(w);
      }
    } catch (e) { pool.close(); giveUp(`a worker could not be started: ${String(e)}`); return null; }
    return pool;
  }
  /** the layer's maps, made on a worker (lookTex.ts lookLayer's own arguments), on the maths this thread computes on now (math/exact.ts) */
  make(layer: number, n: number, even: boolean): Promise<LookMaps> {
    return new Promise<LookMaps>((res, rej) => { if (!this.all.length) { rej(new Error('no look map worker is left')); return; } this.queue.push({ id: this.ids++, layer, n, even, exact: EXACT.on, res, rej }); this.pump(); });
  }
  /** stop every worker; a job still out is refused */
  close(): void {
    const why = new Error('the look map workers were closed');
    for (const w of this.all) { try { w.terminate(); } catch { /* gone already */ } }
    for (const j of this.at.values()) { clearTimeout(j.timer); j.rej(why); } for (const j of this.queue) j.rej(why);
    this.all.length = 0; this.idle.length = 0; this.queue.length = 0; this.at.clear();
    if (this.url) { try { URL.revokeObjectURL(this.url); } catch { /* */ } this.url = ''; }
  }
  private pump(): void {
    while (this.idle.length && this.queue.length) {
      const w = this.idle.pop()!, j = this.queue.shift()!; this.at.set(w, j);
      j.timer = setTimeout(() => this.lost(w, `a worker did not answer in ${Math.round(LOOK_WORKERS.waitMs / 1000)} s`), LOOK_WORKERS.waitMs);
      w.postMessage({ id: j.id, layer: j.layer, n: j.n, even: j.even, exact: j.exact });
    }
  }
  private done(w: Worker, r: Reply): void {
    const j = this.at.get(w); if (!j || j.id !== r.id) return;
    clearTimeout(j.timer); this.at.delete(w); this.idle.push(w);
    // (a layer the GENERATOR threw on is that layer's own trouble, refused alone; the worker is sound)
    if (r.error !== undefined || !(r.a instanceof Uint8Array) || !(r.b instanceof Uint8Array) || r.a.length !== j.n * j.n * 4 || r.b.length !== j.n * j.n * 4 || typeof r.tone !== 'number' || typeof r.hollow !== 'number') j.rej(new Error(`a look map worker did not make layer ${j.layer}: ${r.error ?? 'its answer is not two maps'}`));
    else j.res({ a: r.a, b: r.b, tone: r.tone, hollow: r.hollow });
    this.pump();
  }
  /** a worker that failed is not used again, here or in a later batch: its job is refused, and with no worker left so is every job waiting */
  private lost(w: Worker, why: string): void {
    if (!this.all.includes(w)) return;
    giveUp(why);
    const j = this.at.get(w); this.at.delete(w);
    this.all = this.all.filter(x => x !== w); this.idle = this.idle.filter(x => x !== w);
    try { w.terminate(); } catch { /* */ }
    if (j) { clearTimeout(j.timer); j.rej(new Error(why)); }
    if (!this.all.length) { for (const q of this.queue) q.rej(new Error(why)); this.queue.length = 0; } else this.pump();
  }
}
