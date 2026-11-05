// THE LOOK'S MAPS ARE MADE ON WORKERS (11-05). A map is a third of a second of arithmetic and a level names twenty: six seconds of one thread, which the boot
// waits for (main.ts awaits gbuffer.needLook beside the pipelines). Where the shaders are compiling for half a minute nobody sees it; on a machine whose
// shaders are cached it is the last thing the start card waits for. The layers do not depend on each other, so each is made by render/lookTex.ts lookLayer on
// a worker and handed back (its two arrays are moved, not copied). THE SAME BYTES: a worker runs the same file (lookWorkerSource.ts hands it over as text when
// the page is bundled) on the same engine, and tools/qa/lookworkers-check.ts sets every layer made there against the one made here.
// WHERE THERE ARE NONE (no Worker in this place, one that will not start, a layer a worker failed on): the caller makes the layer itself, in slices, as it
// always did (gbuffer.ts makeLayers). So the workers are only ever a faster way to the same maps.
// THE SWITCH: LOOK_WORKERS.on — ON in the page (`?lookworkers=0` = every layer on the page's own thread, as it was), OFF here, so a tool under bun makes its
// maps as it did. Nothing of the game reads it and no recording carries it: the maps are the same either way.
import { lookWorkerSource } from './lookWorkerSource' with { type: 'macro' };
import type { LookMaps } from './lookTex';

/** `most` = workers at once at the most (a layer holds about 10 MB while it is made); one core is always left to the page */
export const LOOK_WORKERS = { on: false, most: 8 };
/** render/lookTex.ts as this build compiled it (written in when the page is bundled: the macro above). A bundle-time string, not a call in the page */
const GENERATOR = lookWorkerSource() as unknown as string;
/** what a worker does with a message: make the layer, hand its two arrays back by moving them */
const ANSWER = `
self.onmessage = e => { const d = e.data; try { const m = lookLayer(d.layer, d.n, d.even); self.postMessage({ id: d.id, a: m.a, b: m.b, tone: m.tone, hollow: m.hollow }, [m.a.buffer, m.b.buffer]); } catch (err) { self.postMessage({ id: d.id, error: String(err) }); } };
`;
/** a worker's whole text (a check reads it, and breaks it) */
export const lookWorkerText = (): string => GENERATOR + ANSWER;

interface Job { id: number; layer: number; n: number; even: boolean; res: (m: LookMaps) => void; rej: (why: Error) => void }
interface Reply { id: number; a?: Uint8Array; b?: Uint8Array; tone?: number; hollow?: number; error?: string }

/** a few workers for one batch of layers: open(), make() each layer, close(). A job's promise is refused where its worker failed; the caller makes that layer itself */
export class LookPool {
  private idle: Worker[] = []; private all: Worker[] = []; private queue: Job[] = []; private at = new Map<Worker, Job>(); private ids = 1; private url = '';
  private constructor() { /* open() */ }
  /** how many workers it started */
  get size(): number { return this.all.length; }
  /** workers for `jobs` layers, or null where there are none to be had (the rule down, no Worker here, one that would not start) */
  static open(jobs: number, text: string = lookWorkerText()): LookPool | null {
    if (!LOOK_WORKERS.on || jobs < 1 || typeof Worker === 'undefined' || typeof Blob === 'undefined' || typeof URL === 'undefined' || typeof URL.createObjectURL !== 'function') return null;
    const cores = typeof navigator !== 'undefined' && navigator.hardwareConcurrency > 0 ? navigator.hardwareConcurrency : 2;
    const n = Math.max(1, Math.min(jobs, LOOK_WORKERS.most, cores - 1)), pool = new LookPool();
    try {
      pool.url = URL.createObjectURL(new Blob([text], { type: 'text/javascript' }));
      for (let i = 0; i < n; i++) {
        const w = new Worker(pool.url, { type: 'module', name: 'look maps' });   // (a module: the generator's text keeps its `export`s)
        w.onmessage = e => pool.done(w, e.data as Reply);
        w.onerror = e => { (e as Event).preventDefault?.(); pool.lost(w, new Error(`a look map worker failed: ${(e as ErrorEvent).message ?? 'no word'}`)); };
        w.onmessageerror = () => pool.lost(w, new Error('a look map worker sent what could not be read'));
        pool.all.push(w); pool.idle.push(w);
      }
    } catch { pool.close(); return null; }
    return pool;
  }
  /** the layer's maps, made on a worker (lookTex.ts lookLayer's own arguments) */
  make(layer: number, n: number, even: boolean): Promise<LookMaps> {
    return new Promise<LookMaps>((res, rej) => { if (!this.all.length) { rej(new Error('no look map worker is left')); return; } this.queue.push({ id: this.ids++, layer, n, even, res, rej }); this.pump(); });
  }
  /** stop every worker; a job still out is refused */
  close(): void {
    const why = new Error('the look map workers were closed');
    for (const w of this.all) { try { w.terminate(); } catch { /* gone already */ } }
    for (const j of this.at.values()) j.rej(why); for (const j of this.queue) j.rej(why);
    this.all.length = 0; this.idle.length = 0; this.queue.length = 0; this.at.clear();
    if (this.url) { try { URL.revokeObjectURL(this.url); } catch { /* */ } this.url = ''; }
  }
  private pump(): void { while (this.idle.length && this.queue.length) { const w = this.idle.pop()!, j = this.queue.shift()!; this.at.set(w, j); w.postMessage({ id: j.id, layer: j.layer, n: j.n, even: j.even }); } }
  private done(w: Worker, r: Reply): void {
    const j = this.at.get(w); if (!j || j.id !== r.id) return;
    this.at.delete(w); this.idle.push(w);
    if (r.error !== undefined || !(r.a instanceof Uint8Array) || !(r.b instanceof Uint8Array) || r.a.length !== j.n * j.n * 4 || r.b.length !== j.n * j.n * 4 || typeof r.tone !== 'number' || typeof r.hollow !== 'number') j.rej(new Error(`a look map worker did not make layer ${j.layer}: ${r.error ?? 'its answer is not two maps'}`));
    else j.res({ a: r.a, b: r.b, tone: r.tone, hollow: r.hollow });
    this.pump();
  }
  /** a worker that failed is not used again: its job is refused, and with no worker left so is every job waiting */
  private lost(w: Worker, why: Error): void {
    const j = this.at.get(w); this.at.delete(w);
    this.all = this.all.filter(x => x !== w); this.idle = this.idle.filter(x => x !== w);
    try { w.terminate(); } catch { /* */ }
    j?.rej(why);
    if (!this.all.length) { for (const q of this.queue) q.rej(why); this.queue.length = 0; } else this.pump();
  }
}
