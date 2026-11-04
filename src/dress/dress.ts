// DRESSING IN RUST, FROM TYPESCRIPT — the one place the dressing module (rust/dress, built to WebAssembly: src/dress/built.ts) is loaded and named.
//
// WHICH SIDE THIS IS (docs/agents/rust.md, gameplay and dressing): what only makes the picture. Nothing a rule of play reads comes out of this module, the game
// server never starts it, and two machines need not get the same bits from it — it may use singles, the standard library's functions and relaxed SIMD, which the
// simulation (src/sim/sim.ts) may not. src/game/dress.ts lists the tick's dressing steps; tools/qa/dress-check.ts holds that the game is the same game without them.
//
// ONE WAY, AND ONE WAY ONLY: what dressing needs to know of the game is COPIED IN (`dressIn()`: this module's own memory). The module imports nothing and has
// its own memory, so it has no road to the simulation's; and nothing in src/sim or src/game may read an answer back out of here to decide anything.
// Nothing calls this yet: no dressing system has moved to Rust. A page that cannot run WebAssembly loses nothing of the game by it.
//
// (Imports built.ts and the module starter alone. NOT one of the simulation's folders: no rollback snapshot holds what is in here.)
import { DRESS_WASM } from './built';
import { startModule } from '../sim/load';

/** the version of the exports this file was written against (rust/dress/src/lib.rs ABI) */
export const DRESS_ABI = 1;
export interface DressExports {
  readonly memory: WebAssembly.Memory;
  dress_abi(): number;
  /** where the hand-over is: singles the page writes before a step */
  dress_in(): number;
  dress_in_len(): number;
}
/** every export this file expects (tools/qa/rust-check.ts holds the built module to it) */
export const DRESS_EXPORTS: readonly (keyof DressExports)[] = ['memory', 'dress_abi', 'dress_in', 'dress_in_len'];

let X: DressExports | null = null, why: string | null = null;
/** why this runtime cannot run the module ('' = it can, and it is started); asked once and kept */
export function dressWhyNot(): string {
  if (why === null) {
    try {
      const x = startModule(DRESS_WASM).exports as unknown as DressExports;
      if (x.dress_abi() !== DRESS_ABI) throw new Error(`the Rust dressing is version ${x.dress_abi()} and src/dress/dress.ts was written against ${DRESS_ABI}: bun run tools/sim/build.ts`);
      X = x; why = '';
    } catch (e) { why = String((e as Error)?.message ?? e); }
  }
  return why;
}
/** the module, started on the first call (throws what dressWhyNot says where it cannot run) */
export function dress(): DressExports { const no = dressWhyNot(); if (no) throw new Error(no); return X!; }
let seenBuf: ArrayBufferLike | null = null, IN: Float32Array | null = null;
/** the hand-over: a view of the module's own memory the page writes the game's facts into. The same array until the memory has grown: ask again after any
 *  call that can allocate, never keep one across it. */
export function dressIn(): Float32Array {
  const x = dress(), b = x.memory.buffer;
  if (b !== seenBuf || !IN) { seenBuf = b; IN = new Float32Array(b, x.dress_in(), x.dress_in_len()); }
  return IN;
}
