// what tools/qa/lookworkers-page.cjs loads in a blank page: the look's workers and its generator, and one function that makes every layer both ways
// in the browser's own engine and says whether they are the same maps
import { LOOK_WORKERS, LookPool } from '../../src/render/lookWorkers';
import { LOOK_NAMES, LOOK_N, lookLayer, type LookMaps } from '../../src/render/lookTex';
const same = (a: LookMaps, b: LookMaps) => { if (a.tone !== b.tone || a.hollow !== b.hollow || a.a.length !== b.a.length || a.b.length !== b.b.length) return false; for (let i = 0; i < a.a.length; i++) if (a.a[i] !== b.a[i] || a.b[i] !== b.b[i]) return false; return true; };
(window as unknown as { __lookWorkers: unknown }).__lookWorkers = {
  cores: navigator.hardwareConcurrency,
  async run(most: number) {
    LOOK_WORKERS.on = true; LOOK_WORKERS.most = most;
    const layers = LOOK_NAMES.map((_, l) => l), pool = LookPool.open(layers.length); if (!pool) return { error: 'no pool opened' };
    const t0 = performance.now(), made = await Promise.all(layers.map(l => pool.make(l, LOOK_N, true).then(m => m, (e: unknown) => String(e)))), onWorkers = performance.now() - t0, workers = pool.size; pool.close();
    const t1 = performance.now(), bad: string[] = [];
    for (const l of layers) { const m = made[l]; if (typeof m === 'string') bad.push(`${LOOK_NAMES[l]}: ${m}`); else if (!same(m, lookLayer(l, LOOK_N, true))) bad.push(LOOK_NAMES[l]); }
    return { layers: layers.length, workers, onWorkers: Math.round(onWorkers), here: Math.round(performance.now() - t1), bad };
  },
};
