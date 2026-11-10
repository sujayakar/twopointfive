// SHAPES MADE BY MANY HANDS: each is its own, so as many are made at once as there are workers, the slow ones first.
import type { Packed } from './template';
const SLOW = /^(body|odd:|cast|hard|metal|soft|wood)/;
export async function shapesBy(worker: () => Worker, names: string[], many: number, say: (done: number, of: number) => void = () => {}): Promise<Packed[]> {
  const queue = [...names].sort((a, b) => +(b === 'body') - +(a === 'body') || +SLOW.test(b) - +SLOW.test(a) || b.length - a.length), out = new Map<string, Packed>(); let done = 0;
  await Promise.all(Array.from({ length: Math.min(many, names.length) }, () => new Promise<void>((ok, no) => { const w = worker(), next = () => { const n = queue.shift(); if (n === undefined) { w.terminate(); ok(); return; } w.postMessage({ shape: n }); }; w.onmessage = e => { if (e.data.error) { no(new Error(e.data.error)); return; } out.set(e.data.packed.name, e.data.packed); say(++done, names.length); next(); }; w.onerror = e => no(new Error(e.message)); next(); })));
  return names.map(n => out.get(n)!);
}
/** what a worker does */
export function serve(make: (name: string) => Packed) { (globalThis as never as { onmessage: unknown }).onmessage = (e: MessageEvent) => { const post = postMessage as (m: unknown, t?: Transferable[]) => void; try { const packed = make(e.data.shape); post({ packed }, [packed.data.buffer]); } catch (x) { post({ error: String(x) }); } }; }
