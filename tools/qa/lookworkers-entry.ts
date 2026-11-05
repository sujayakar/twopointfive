// what tools/qa/lookworkers-page.cjs loads in a blank page: the look's workers and its generator, and one function that makes every layer both ways
// in the browser's own engine and says whether they are the same maps
import { LOOK_WORKERS, LookPool } from '../../src/render/lookWorkers';
import { LOOK_NAMES, LOOK_N, lookLayer } from '../../src/render/lookTex';
import { sameMaps as same } from './lookworkers-same';
import { EXACT } from '../../src/math/exact';
(window as unknown as { __lookWorkers: unknown }).__lookWorkers = {
  cores: navigator.hardwareConcurrency,
  async run(most: number, exact: boolean) {
    LOOK_WORKERS.on = true; LOOK_WORKERS.most = most; EXACT.on = exact;
    const layers = LOOK_NAMES.map((_, l) => l), pool = await LookPool.open(layers.length); if (!pool) return { error: 'no pool opened' };
    const t0 = performance.now(), made = await Promise.all(layers.map(l => pool.make(l, LOOK_N, true).then(m => m, (e: unknown) => String(e)))), onWorkers = performance.now() - t0, workers = pool.size; pool.close();
    const t1 = performance.now(), bad: string[] = [];
    for (const l of layers) { const m = made[l]; if (typeof m === 'string') bad.push(`${LOOK_NAMES[l]}: ${m}`); else if (!same(m, lookLayer(l, LOOK_N, true))) bad.push(LOOK_NAMES[l]); }
    return { layers: layers.length, workers, onWorkers: Math.round(onWorkers), here: Math.round(performance.now() - t1), bad };
  },
};
