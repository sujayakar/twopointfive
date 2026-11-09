// THE LOOK'S MAP WORKER (render/lookWorkers.ts starts it; render/lookWorkerSource.ts bundles this file into the plain script it is started from). One message, one
// layer: lookTex.ts lookLayer with the caller's arguments, and the two arrays handed back by moving them. THE SMALLER SIZES
// COME WITH THEM (lookTex.ts lookMip, each from the one above, down to one texel): a layer made in play, when a blast shows a stuff no box had been, then
// costs the page's thread its texture writes and nothing else.
import { lookLayer, lookMip } from './lookTex';
interface Ask { id: number; layer: number; n: number; even: boolean }
const port = self as unknown as { onmessage: ((e: { data: Ask }) => void) | null; postMessage(m: unknown, move?: ArrayBuffer[]): void };
port.onmessage = e => {
  const d = e.data;
  try {
    const m = lookLayer(d.layer, d.n, d.even), a = [m.a], b = [m.b];
    for (let n = d.n; n > 1; n >>= 1) { a.push(lookMip(a[a.length - 1], n, false)); b.push(lookMip(b[b.length - 1], n, true)); }
    port.postMessage({ id: d.id, a, b, tone: m.tone, hollow: m.hollow }, [...a, ...b].map(x => x.buffer as ArrayBuffer));
  }
  catch (err) { port.postMessage({ id: d.id, error: String(err) }); }
};
