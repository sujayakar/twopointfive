// THE LOOK'S MAP WORKER (render/lookWorkers.ts starts it; render/lookWorkerSource.ts bundles this file into the plain script it is started from). One message, one
// layer: lookTex.ts lookLayer with the caller's arguments, ON THE CALLER'S MATHS (math/exact.ts: the page computes on the game's own sin and cos, a worker
// starts with the engine's; a map made on the wrong ones differs in a byte here and there), and the two arrays handed back by moving them.
import { EXACT } from '../math/exact';
import { lookLayer } from './lookTex';
interface Ask { id: number; layer: number; n: number; even: boolean; exact: boolean }
const port = self as unknown as { onmessage: ((e: { data: Ask }) => void) | null; postMessage(m: unknown, move?: ArrayBuffer[]): void };
port.onmessage = e => {
  const d = e.data;
  try { EXACT.on = d.exact; const m = lookLayer(d.layer, d.n, d.even); port.postMessage({ id: d.id, a: m.a, b: m.b, tone: m.tone, hollow: m.hollow }, [m.a.buffer as ArrayBuffer, m.b.buffer as ArrayBuffer]); }
  catch (err) { port.postMessage({ id: d.id, error: String(err) }); }
};
