// THE SIMULATION IN RUST, FROM TYPESCRIPT — the one place the Rust module (rust/sim, built to WebAssembly: src/sim/built.ts) is loaded and named.
//
// WHAT THIS IS FOR: the game's update is moving out of TypeScript into Rust a system at a time (docs/agents/rust.md has the order, the rules and what holds
// each step to the TypeScript it replaces). Everything that calls into the module goes through `sim()`; everything that says WHICH systems run there is a row
// of RUST below.
//
// ONE WAY IN, EVERYWHERE. The module's bytes are text in built.ts, so the page, the game server, bun and node load them as they load any source file; and it
// is compiled and started in one synchronous step the first time it is asked for, so nothing that uses it has to be written around a promise. (Chrome allows a
// synchronous compile on the page's own thread up to 8 MB; tools/sim/build.ts is red long before that.)
//
// MEMORY: the module owns one linear memory. A typed array over it dies when the memory GROWS — never keep one across a call that can allocate; ask `f64()`
// / `u8()` again (they hand back the same array until the buffer has changed).
//
// (Imports built.ts, the module starter and src/math alone: like src/math, this is below the game.)
import { SIM_WASM, SIM_BYTES } from './built';
import { startModule } from './load';
import { mathsFrom, EXACT, type Maths } from '../math/exact';
import { kahanN } from '../math/hyp';

/** the version of the exports this file was written against (rust/sim/src/lib.rs ABI) */
export const SIM_ABI = 1;

/** the module's exports, by name (rust/sim/src/lib.rs: plain C functions, `<module>_<thing>`) */
export interface SimExports {
  readonly memory: WebAssembly.Memory;
  sim_abi(): number;
  /** where the scratch is: a few doubles a call's own parameters cannot carry go through it (write, call, read; nothing is kept there between calls) */
  sim_scratch(): number;
  sim_scratch_len(): number;
  // the game's own maths (rust/sim/src/maths.rs — src/math/exact.ts to the bit)
  maths_sin(x: number): number; maths_cos(x: number): number; maths_tan(x: number): number;
  maths_asin(x: number): number; maths_acos(x: number): number; maths_atan(x: number): number; maths_atan2(y: number, x: number): number;
  maths_exp(x: number): number; maths_log(x: number): number; maths_log2(x: number): number; maths_log10(x: number): number;
  maths_pow(x: number, y: number): number; maths_tanh(x: number): number;
  maths_hypot2(a: number, b: number): number; maths_hypot3(a: number, b: number, c: number): number; maths_hypot4(a: number, b: number, c: number, d: number): number;
  /** the length of the first n doubles of the scratch */
  maths_hypot_n(n: number): number;
}
/** every export this file expects, for the check that the built module and this face name the same things (tools/qa/rust-check.ts) */
export const SIM_EXPORTS: readonly (keyof SimExports)[] = ['memory', 'sim_abi', 'sim_scratch', 'sim_scratch_len', 'maths_sin', 'maths_cos', 'maths_tan', 'maths_asin', 'maths_acos', 'maths_atan', 'maths_atan2',
  'maths_exp', 'maths_log', 'maths_log2', 'maths_log10', 'maths_pow', 'maths_tanh', 'maths_hypot2', 'maths_hypot3', 'maths_hypot4', 'maths_hypot_n'];

let X: SimExports | null = null, why: string | null = null;
/** how long the one compile-and-start took, in milliseconds (0 until it has happened): the page's boot line and the check print it */
export let simBootMs = 0;
function start(): SimExports {
  const M = startModule(SIM_WASM), x = M.exports as unknown as SimExports;   // (load.ts: nothing is handed in — the module imports nothing)
  if (x.sim_abi() !== SIM_ABI) throw new Error(`the Rust simulation is version ${x.sim_abi()} and src/sim/sim.ts was written against ${SIM_ABI}: bun run tools/sim/build.ts`);
  simBootMs = M.ms;
  return x;
}
/** WHY THIS RUNTIME CANNOT RUN THE MODULE ('' = it can, and it is started). Asked once and kept: the module is started here, so a runtime that has no
 *  WebAssembly (a browser whose policy turns its JIT off has none), one that will not compile it (a page whose policy forbids it) and a built.ts of another
 *  version are all the one answer. Such a page plays on the TypeScript simulation for as long as there is one, and says so (simSay) — it could not play a
 *  build whose update is in Rust alone. */
export function simWhyNot(): string {
  if (why === null) { try { X = start(); why = ''; } catch (e) { why = String((e as Error)?.message ?? e); } }
  return why;
}
/** the module, started on the first call (throws what simWhyNot says where it cannot run) */
export function sim(): SimExports {
  const no = simWhyNot(); if (no) throw new Error(no);
  return X!;
}

// ---- the module's memory
let seenBuf: ArrayBufferLike | null = null, F64: Float64Array | null = null, U8: Uint8Array | null = null;
function views(): void { const b = sim().memory.buffer; if (b !== seenBuf) { seenBuf = b; F64 = new Float64Array(b); U8 = new Uint8Array(b); } }
/** the module's memory as doubles (index = address / 8). Not to be kept across a call that can allocate. */
export function f64(): Float64Array { views(); return F64!; }
/** the module's memory as bytes. Not to be kept across a call that can allocate. */
export function u8(): Uint8Array { views(); return U8!; }

// ---- the game's own maths, worked out in Rust
let rustMaths: Maths | null = null;
/** the fourteen and the three lengths as the module works them out: THE set RUST.maths puts in Math's place (and the one tools/qa/rust-check.ts folds) */
export function mathsOfRust(): Maths {
  if (rustMaths) return rustMaths;
  const x = sim(), at = x.sim_scratch() / 8, cap = x.sim_scratch_len();
  const hypot = function (): number {
    const n = arguments.length;
    if (n === 2) return x.maths_hypot2(+arguments[0], +arguments[1]);
    if (n === 3) return x.maths_hypot3(+arguments[0], +arguments[1], +arguments[2]);
    if (n === 4) return x.maths_hypot4(+arguments[0], +arguments[1], +arguments[2], +arguments[3]);
    if (n > cap) return kahanN(arguments as unknown as ArrayLike<number>);   // (more terms than the scratch holds: the same sum, here)
    const m = f64(); for (let i = 0; i < n; i++) m[at + i] = +arguments[i];
    return x.maths_hypot_n(n);
  } as (...v: number[]) => number;
  return rustMaths = { sin: x.maths_sin, cos: x.maths_cos, tan: x.maths_tan, asin: x.maths_asin, acos: x.maths_acos, atan: x.maths_atan, atan2: x.maths_atan2, exp: x.maths_exp, log: x.maths_log,
    log2: x.maths_log2, log10: x.maths_log10, pow: x.maths_pow, tanh: x.maths_tanh, hypot, hyp2: x.maths_hypot2, hyp3: x.maths_hypot3, hyp4: x.maths_hypot4 };
}

// ---- WHICH SYSTEMS RUN IN RUST
let mathsOn = false;
/** A row a system: on = that system of the update runs in the Rust module, off = in TypeScript as it always did. THE TWO ARE THE SAME GAME TO THE BIT — that is
 *  what lets a system move at all, and tools/qa/rust-check.ts holds each row to it (whole missions played both ways, every tick's raw bits). So a row is never
 *  a rule of play: a recording made with one on replays with it off, and the frozen stand's fixtures do not care. A row is ON in the page once its system is
 *  proven and faster there; until then it is a thing a check raises.
 *
 *  maths   the game's own maths (src/math/exact.ts, when EXACT.on has put them in Math's place) from Rust instead: every sin, cos, atan2, exp … and every
 *          length of math/hyp.ts the update asks for crosses into the module. OFF in the page and on both stands (?rustmaths / TPS_RUSTMATHS=1 raise it): a
 *          call from TypeScript into the module costs more than the dozen operations it saves (bun: sin 27 ns here, 44 ns there; V8 about level), so this row
 *          is the proof that the two arithmetics are one — and the Rust systems that follow call the Rust maths with no crossing. */
export const RUST = {
  get maths(): boolean { return mathsOn; },
  set maths(v: boolean) { const on = !!v && !simWhyNot(); if (on === mathsOn) return; mathsFrom(on ? mathsOfRust() : null); mathsOn = on; },   // (a runtime that cannot run the module keeps the row off: the same game either way)
};
/** WHAT EACH ROW HAS RUNNING IN THE MODULE NOW: a row can be on and idle (the maths cross only while the game's own maths stand in Math's place) */
function running(): string[] { const out: string[] = []; if (mathsOn) out.push(EXACT.on ? 'maths' : 'maths (idle: the game\'s own maths are off)'); return out; }
/** one line for the page's console: the module started (or why not), and which rows are on. Starts the module — a page that cannot is known at boot, not at
 *  the first system that needs it. */
export function simSay(): string {
  const no = simWhyNot(); if (no) return `rust simulation: NOT RUNNING — ${no}; every system in TypeScript`;
  const on = running();
  return `rust simulation: ${SIM_BYTES} bytes, started in ${simBootMs.toFixed(1)} ms · in Rust: ${on.length ? on.join(' ') : 'nothing yet (rows: ' + Object.keys(RUST).join(' ') + ')'}`;
}
