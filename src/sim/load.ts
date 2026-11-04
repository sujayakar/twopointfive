// A RUST MODULE OF THE GAME, STARTED (src/sim/sim.ts the simulation, src/dress/dress.ts dressing): the bytes a built.ts holds as text, compiled and started in one
// synchronous step, with NOTHING handed in — a module of the game imports nothing (tools/sim/build.ts is red on one that does), so there is no way for one to
// reach a clock, the network, or the other module's memory. Each module owns one linear memory; the two are never the same one.
function bytesOf(b64: string): Uint8Array {
  const bin = atob(b64), out = new Uint8Array(bin.length);
  for (let i = 0; i < bin.length; i++) out[i] = bin.charCodeAt(i);
  return out;
}
/** the module's exports, and how long the compile and start took in milliseconds. Throws where the runtime has no WebAssembly or will not compile it. */
export function startModule(wasm: readonly string[]): { exports: Record<string, unknown>; ms: number } {
  const W = (globalThis as { WebAssembly?: { Module?: unknown; Instance?: unknown } }).WebAssembly;
  if (!W || typeof W.Module !== 'function' || typeof W.Instance !== 'function') throw new Error('this runtime has no WebAssembly (a browser with its JIT turned off has none)');
  const now = (): number => (globalThis as { performance?: { now(): number } }).performance?.now() ?? 0, t0 = now();
  const inst = new WebAssembly.Instance(new WebAssembly.Module(bytesOf(wasm.join('')) as BufferSource), {});
  return { exports: inst.exports as Record<string, unknown>, ms: now() - t0 };
}
