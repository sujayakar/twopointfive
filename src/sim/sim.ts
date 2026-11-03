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
export const SIM_ABI = 9;

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
  // the box world's copy (rust/sim/src/world.rs — written by src/scene/boxMirror.ts, which names the tables)
  world_addr(w: number, table: number): number; world_cap(table: number): number;
  world_shape(w: number, gw: number, gh: number, cs: number, boxes: number, tilted: number): void; world_leaves(w: number, set: number, n: number): void;
  // the line tests through it (rust/sim/src/lines.rs — src/scene/collision.ts segmentBlocked / segmentBlockedDynamic past their length)
  lines_blocked(w: number, a0: number, a1: number, a2: number, b0: number, b1: number, b2: number, len: number, shrink: number, through: number, set: number, ask: number): number;
  lines_leaves(w: number, a0: number, a1: number, a2: number, b0: number, b1: number, b2: number, len: number, need: number, set: number): number;
  // a circle against it (rust/sim/src/collide.rs — src/scene/collision.ts collideCircle / circleTouches / gather)
  collide_circle(w: number, px: number, pz: number, r: number, y0: number, y1: number, iterations: number): number;
  collide_touches(w: number, x: number, z: number, r: number, y0: number, y1: number, cap: number): number;
  collide_gather(w: number, x0: number, z0: number, x1: number, z1: number, y0: number, y1: number, cap: number): number;
  // the fog of war's sight pass (rust/sim/src/fog.rs — src/game/fogofwar.ts through collision.ts sightOpen / eyesSee)
  fog_eyes(w: number): number;
  fog_see(w: number, set: number, order: number, n: number, want: number, kind: number, through: number, a0: number, a1: number, a2: number, b0: number, b1: number, b2: number): number;
  // posing on the module's bench (rust/sim/src/skeleton.rs — src/anim/skeleton.ts)
  pose_addr(table: number): number; pose_rig(nodes: number, order: number): number;
  pose_blend(a: number, b: number, w: number, masked: number, out: number, n: number): void;
  pose_run(p: number, root: number): void; pose_parent_rot(p: number, i: number): void; pose_turn(p: number, i: number, how: number): void;
  pose_ik(p: number, a: number, b: number, c: number, flags: number): void;
  // the senses (rust/sim/src/sight.rs, light.rs, smoke.rs, hear.rs, nav.rs — asked through src/scene/collision.ts, THE SENSES IN THE MODULE). 0xffffffff (−1 here) = ask the TypeScript.
  /** guards.ts eyeContact: 1 visible + 2 in the cone; the distance → scratch 0 */
  sight_contact(w: number, set: number, through: number, q: number, e0: number, e1: number, e2: number, l0: number, l1: number, l2: number, range: number, halfDeg: number, nearM: number, nearDeg: number, vHalfDeg: number, c0: number, c1: number, c2: number, h0: number, h1: number, h2: number): number;
  light_lamps(): number; light_cap(): number;
  /** guards.ts lightEstimateAt's loop over the lamps written: the six faces → scratch 0‥5 */
  light_estimate(w: number, set: number, n: number, p0: number, p1: number, p2: number, sigma: number): number;
  smoke_blobs(): number; smoke_cap(): number;
  /** sense.ts CpuSmoke.trans over the blobs written: the transmittance → scratch 0 */
  smoke_trans(w: number, set: number, n: number, a0: number, a1: number, a2: number, b0: number, b1: number, b2: number, sigma: number, spread: number, widest: number, rise: number, riseRate: number): number;
  nav_addr(table: number): number; nav_cap(table: number): number;
  /** collision.ts NavGrid.findPath with no penalty: the waypoints written (x z each) · −1 no way · −2 ask the TypeScript */
  nav_path(w: number, set: number, ask: number, gw: number, gh: number, cell: number, a0: number, a2: number, b0: number, b2: number, maxIter: number): number;
  /** collision.ts SoundPaths.propagate up to its memory of routed answers; the straight distance → scratch 0 */
  hear_direct(w: number, set: number, ask: number, s0: number, s1: number, s2: number, d0: number, d1: number, d2: number): number;
  /** …and the way round: the path's length and the point it comes through → scratch 0‥3 */
  hear_route(w: number, set: number, ask: number, seal: number, gw: number, gh: number, cell: number, s0: number, s1: number, s2: number, d0: number, d1: number, d2: number, straight: number): number;
  // a game's state in the module (rust/sim/src/state.rs — a block a game, bound to slot g by src/sim/state.ts)
  state_addr(g: number): number; state_len(): number; state_games(): number; state_layout(): number; state_parts(): number; state_part(i: number, what: number): number;
  // the game's own die, in its block (rust/sim/src/dice.rs — src/math/dice.ts simRandom)
  dice_roll(g: number): number;
  // the game's own smoke on the coarse grid (rust/sim/src/bricks.rs; game/sense.ts SmokeGrid): a game's state is grid_len doubles at grid_addr
  grid_addr(w: number): number; grid_len(): number; grid_fresh(w: number): void; grid_dirty(w: number, x0: number, z0: number, x1: number, z1: number): void;
  grid_emit(w: number, x: number, y: number, z: number, radius: number, rate: number, ttl: number): void;
  /** n steps of a tenth of a second: 0 · 0xffffffff (−1 here) = no grid stands on this level */
  grid_step(w: number, set: number, n: number, decay: number, spread: number, rise: number, riseRate: number, gone: number, lamps: number, albedo: number, share: number, passes: number, sweep: number): number;
  grid_trans(w: number, a0: number, a1: number, a2: number, b0: number, b1: number, b2: number, sigma: number): number;
}
/** every export this file expects, for the check that the built module and this face name the same things (tools/qa/rust-check.ts) */
export const SIM_EXPORTS: readonly (keyof SimExports)[] = ['memory', 'sim_abi', 'sim_scratch', 'sim_scratch_len', 'maths_sin', 'maths_cos', 'maths_tan', 'maths_asin', 'maths_acos', 'maths_atan', 'maths_atan2',
  'maths_exp', 'maths_log', 'maths_log2', 'maths_log10', 'maths_pow', 'maths_tanh', 'maths_hypot2', 'maths_hypot3', 'maths_hypot4', 'maths_hypot_n',
  'world_addr', 'world_cap', 'world_shape', 'world_leaves', 'lines_blocked', 'lines_leaves', 'collide_circle', 'collide_touches', 'collide_gather', 'fog_eyes', 'fog_see',
  'pose_addr', 'pose_rig', 'pose_blend', 'pose_run', 'pose_parent_rot', 'pose_turn', 'pose_ik',
  'sight_contact', 'light_lamps', 'light_cap', 'light_estimate', 'smoke_blobs', 'smoke_cap', 'smoke_trans', 'nav_addr', 'nav_cap', 'nav_path', 'hear_direct', 'hear_route',
  'state_addr', 'state_len', 'state_games', 'state_layout', 'state_parts', 'state_part', 'dice_roll',
  'grid_addr', 'grid_len', 'grid_fresh', 'grid_dirty', 'grid_emit', 'grid_step', 'grid_trans'];

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
/** is `b` the module's memory (false where the module was never started: asking does not start it) — what tells a typed array over it from one of the game's own */
export function simOwns(b: ArrayBufferLike): boolean { return X !== null && X.memory.buffer === b; }
let seenBuf: ArrayBufferLike | null = null, F64: Float64Array | null = null, U8: Uint8Array | null = null, U32: Uint32Array | null = null, I32: Int32Array | null = null, F32: Float32Array | null = null;
function views(): void { const b = sim().memory.buffer; if (b !== seenBuf) { seenBuf = b; F64 = new Float64Array(b); U8 = new Uint8Array(b); U32 = new Uint32Array(b); I32 = new Int32Array(b); F32 = new Float32Array(b); } }
/** the module's memory as doubles (index = address / 8). Not to be kept across a call that can allocate. */
export function f64(): Float64Array { views(); return F64!; }
/** the module's memory as bytes. Not to be kept across a call that can allocate. */
export function u8(): Uint8Array { views(); return U8!; }
/** the module's memory as 32-bit words, unsigned and signed (index = address / 4). Not to be kept across a call that can allocate. */
export function u32(): Uint32Array { views(); return U32!; }
export function i32(): Int32Array { views(); return I32!; }
/** the module's memory as singles (index = address / 4): a stored pose. Not to be kept across a call that can allocate. */
export function f32(): Float32Array { views(); return F32!; }

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
let mathsOn = false, linesOn = false, circlesOn = false, fogOn = false, posesOn = false, diceOn = false;
let sightOn = false, lightOn = false, smokeOn = false, soundOn = false, pathsOn = false;
/** A row a system: on = that system of the update runs in the Rust module, off = in TypeScript as it always did. THE TWO ARE THE SAME GAME TO THE BIT — that is
 *  what lets a system move at all, and tools/qa/rust-check.ts holds each row to it (whole missions played both ways, every tick's raw bits). So a row is never
 *  a rule of play: a recording made with one on replays with it off, and the frozen stand's fixtures do not care. A row is ON in the page once its system is
 *  proven and faster there; until then it is a thing a check raises.
 *
 *  maths   the game's own maths (src/math/exact.ts, when EXACT.on has put them in Math's place) from Rust instead: every sin, cos, atan2, exp … and every
 *          length of math/hyp.ts the update asks for crosses into the module. OFF in the page and on both stands (?rustmaths / TPS_RUSTMATHS=1 raise it): a
 *          call from TypeScript into the module costs more than the dozen operations it saves (bun: sin 27 ns here, 44 ns there; V8 about level), so this row
 *          is the proof that the two arithmetics are one — and the Rust systems that follow call the Rust maths with no crossing.
 *  lines   the box world's line tests (src/scene/collision.ts segmentBlocked, shotBlocked through it, segmentBlockedDynamic: every line of sight, every
 *          straight run of a sound, every reach and line of fire) from the module's copy of the box world (src/scene/boxMirror.ts) — INSIDE A TICK: there the
 *          copy is proven against the boxes as the tick opens; a question asked outside one (the page's own, a tool's) is answered in TypeScript as it always
 *          was. ?rustlines / TPS_RUSTLINES=1 raise it.
 *  circles a circle against the box world (collision.ts collideCircle, circleTouches, gather: a body's footprint pushed out of the statics, what a prop or a
 *          rag doll rests against) from the same copy, inside a tick. ?rustcircles / TPS_RUSTCIRCLES=1 raise it.
 *  fog     the fog of war's sight pass (game/fogofwar.ts update: which of the team's eyes has a line to each man and each body): the loop over the eyes and
 *          its lines in the module, a man a question, inside a tick and where the game's own maths stand (EXACT.on: the lengths are worked out there).
 *          ?rustfog / TPS_RUSTFOG=1 raise it.
 *  poses   the posing functions of src/anim/skeleton.ts (a pose blended, run forward, a node turned, the two-bone solve) from the module, a question's pose
 *          written onto its bench and read back: A PROOF, not a speed-up (the copies cost more than the question), until the animator's layers that ask
 *          them are Rust too. Where the game's own maths stand. ?rustposes / TPS_RUSTPOSES=1 raise it.
 *  dice    the game's own die (src/math/dice.ts SimDice: the word every draw of the simulation comes off, and the count of draws) kept in the game's block of
 *          the module's memory (src/sim/state.ts) and rolled there. THE FIRST STATE OF THE GAME THAT LIVES IN THE MODULE: it is what the rollback snapshot,
 *          the saves in memory and the state's word were taught to carry a block for. A proof like `maths` (a draw from TypeScript then costs a call), and the
 *          die has to be there before any rule that rolls it is. A game takes the row as its next tick opens. ?rustdice / TPS_RUSTDICE=1 raise it.
 *
 *  THE SENSES, a row each (asked through src/scene/collision.ts; every one inside a tick and where the game's own maths stand, since each works its lengths out
 *  in the module):
 *  sight   an eye on a man (game/guards.ts eyeContact: the cone, then the lines to his chest and his head) as one question. ?rustsight / TPS_RUSTSIGHT=1.
 *  light   the light on a point (guards.ts lightEstimateAt: every lamp in reach, each shadowed by a line) as one question a point, the host writing the
 *          lamps that may be asked. ?rustlight / TPS_RUSTLIGHT=1.
 *  smoke   the game's own smoke along a line (game/sense.ts CpuSmoke.trans, what SENSE.cpu sees through) as one question a line. ?rustsmoke / TPS_RUSTSMOKE=1.
 *  sound   how a sound reaches a listener (scene/collision.ts SoundPaths.propagate: straight, over the top, or round by the floor). ?rustsound / TPS_RUSTSOUND=1.
 *  paths   a way across the floor (collision.ts NavGrid.findPath with no penalty: the A* and its smoothing). ?rustpaths / TPS_RUSTPATHS=1. */
export const RUST = {
  get maths(): boolean { return mathsOn; },
  set maths(v: boolean) { const on = !!v && !simWhyNot(); if (on === mathsOn) return; mathsFrom(on ? mathsOfRust() : null); mathsOn = on; },   // (a runtime that cannot run the module keeps the row off: the same game either way)
  get lines(): boolean { return linesOn; },
  set lines(v: boolean) { linesOn = !!v && !simWhyNot(); },
  get circles(): boolean { return circlesOn; },
  set circles(v: boolean) { circlesOn = !!v && !simWhyNot(); },
  get fog(): boolean { return fogOn; },
  set fog(v: boolean) { fogOn = !!v && !simWhyNot(); },
  get poses(): boolean { return posesOn; },
  set poses(v: boolean) { posesOn = !!v && !simWhyNot(); },
  get dice(): boolean { return diceOn; },
  set dice(v: boolean) { diceOn = !!v && !simWhyNot(); },
  get sight(): boolean { return sightOn; },
  set sight(v: boolean) { sightOn = !!v && !simWhyNot(); },
  get light(): boolean { return lightOn; },
  set light(v: boolean) { lightOn = !!v && !simWhyNot(); },
  get smoke(): boolean { return smokeOn; },
  set smoke(v: boolean) { smokeOn = !!v && !simWhyNot(); },
  get sound(): boolean { return soundOn; },
  set sound(v: boolean) { soundOn = !!v && !simWhyNot(); },
  get paths(): boolean { return pathsOn; },
  set paths(v: boolean) { pathsOn = !!v && !simWhyNot(); },
};
/** the rows that work their lengths and angles out in the module, and so answer only where the game's own maths stand in Math's place (EXACT.on) */
const OWN_MATHS: readonly string[] = ['maths', 'fog', 'poses', 'sight', 'light', 'smoke', 'sound', 'paths'];
/** the rows by name, for what treats them all alike: the page's address (?rust<row>), the harness's word (TPS_RUST<ROW>=1), a recording's own (journal.ts) */
export const rustRows = (): string[] => Object.keys(RUST);
export function rustRow(row: string, on: boolean): void { if (rustRows().includes(row)) (RUST as Record<string, boolean>)[row] = on; }
/** WHAT EACH ROW HAS RUNNING IN THE MODULE NOW: a row can be on and idle (the maths cross only while the game's own maths stand in Math's place) */
function running(): string[] { const R = RUST as Record<string, boolean>; return rustRows().filter(r => R[r]).map(r => EXACT.on || !OWN_MATHS.includes(r) ? r : `${r} (idle: the game's own maths are off)`); }
/** one line for the page's console: the module started (or why not), and which rows are on. Starts the module — a page that cannot is known at boot, not at
 *  the first system that needs it. */
export function simSay(): string {
  const no = simWhyNot(); if (no) return `rust simulation: NOT RUNNING — ${no}; every system in TypeScript`;
  const on = running();
  return `rust simulation: ${SIM_BYTES} bytes, started in ${simBootMs.toFixed(1)} ms · in Rust: ${on.length ? on.join(' ') : 'nothing yet (rows: ' + Object.keys(RUST).join(' ') + ')'}`;
}
