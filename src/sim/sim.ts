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
export const SIM_ABI = 16;

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
  // posing (rust/sim/src/skeleton.rs — src/anim/skeleton.ts). A question says WHERE ITS POSE IS: g = −1 → place `at` of the module's bench (the host wrote it
  // there and reads the answer back) · else game g's pool (rust/sim/src/poses.rs), the pose starting at single `at` of that part. 1 = done, 0 = not a pose.
  pose_addr(table: number): number; pose_rig(nodes: number, order: number): number;
  pose_blend(g: number, a: number, b: number, w: number, masked: number, out: number, n: number): number;
  pose_run(g: number, at: number, root: number): number; pose_parent_rot(g: number, at: number, i: number): number;
  pose_turn(g: number, at: number, nodes: number, i: number, how: number): number;
  pose_ik(g: number, at: number, a: number, b: number, c: number, flags: number): number;
  knob_addr(table: number): number; knob_len(table: number): number;
  layer_mv_trunk(g: number, at: number): number; layer_long_body(g: number, at: number): number; layer_life(g: number, at: number): number; layer_mv_pre(g: number, at: number): number;
  layer_stance_body(g: number, at: number): number; layer_stand_pose(g: number, at: number, atB: number, atC: number): number; layer_crouch_pose(g: number, at: number, atB: number, atC: number): number; layer_soft_knees(g: number, at: number): number; layer_run_plant(g: number, at: number): number; layer_sneak_body(g: number, at: number): number; layer_steady(g: number, at: number, atC: number): number; layer_creep_body(g: number, at: number): number; layer_kick_body(g: number, at: number, atC: number): number; layer_toes_up(g: number, at: number): number; layer_wall_body(g: number, at: number, atC: number, atB: number, atRest: number): number; layer_watch_slip(g: number, at: number): number; layer_watch_look(g: number, at: number): number; layer_mv_post_body(g: number, at: number): number; layer_turn_weigh(): number; layer_lean_body(g: number, at: number): number; layer_gaze_keep(g: number, at: number): number; layer_sneak_arms(g: number, at: number, atB: number): number; layer_carry_arms(g: number, at: number, atB: number, atAim: number): number; layer_wall_arms(g: number, at: number, atB: number, atRest: number): number; layer_act_pose(g: number, at: number, atB: number, atRest: number): number; layer_wound_body(g: number, at: number, atB: number, atRest: number): number; layer_wound_limbs(g: number, at: number, atB: number, atRest: number): number; layer_mv_post_arms(g: number, at: number, atB: number, atRest: number): number; layer_watch_arm(g: number, at: number, atB: number, atRest: number): number; layer_creep_arms(g: number, at: number, atB: number, atRest: number): number; layer_clutch_arm(g: number, at: number, atB: number, atRest: number): number; layer_bound_arms(g: number, at: number, atB: number, atRest: number): number; layer_reload_arm(g: number, at: number, atB: number, atRest: number): number; layer_pistol_reload_arm(g: number, at: number, atB: number, atRest: number): number; layer_hands_pass(g: number, at: number): number; layer_door_layer(g: number, at: number, atB: number, atRest: number): number; layer_work_layer(g: number, at: number, atB: number, atRest: number): number; layer_ready_layer(g: number, at: number, atB: number, atRest: number): number; layer_kit_layer(g: number, at: number, atB: number, atRest: number): number; layer_taken_layer(g: number, at: number): number; layer_blow_legs(g: number, at: number): number; layer_blow_trunk(g: number, at: number): number; layer_blow_arms(g: number, at: number, atB: number, atRest: number): number; layer_knife_pose(g: number, at: number, atB: number, atRest: number): number; layer_cable_pose(g: number, at: number, atB: number, atRest: number): number; layer_stealth_layer(g: number, at: number, atB: number, atRest: number): number; layer_swap_pose(g: number, at: number, atB: number, atRest: number, atSw: number): number; layer_lockpick_layer(g: number, at: number, atB: number, atRest: number): number; layer_haul_layer(g: number, at: number, atB: number, atRest: number): number; layer_torch_up(g: number, at: number, atB: number): number; layer_radio_pose(g: number, at: number, atB: number, atRest: number): number; layer_smoke_pose(g: number, at: number, atB: number, atRest: number): number; layer_nv_pose(g: number, at: number, atB: number, atRest: number): number; layer_signal_layer(g: number, at: number, atB: number, atRest: number): number; layer_tell_layer(g: number, at: number, atB: number, atRest: number): number; layer_long_arms(g: number, at: number, atB: number, atAim: number, atRest: number): number;
  /** skeleton.ts sampleClip: clip `clip` (its number from clip_end) at time t into the pose, whose own count of nodes is `nodes` */
  pose_sample(g: number, at: number, nodes: number, clip: number, t: number, looped: number): number;
  // the clips (rust/sim/src/clips.rs): a character file's animations, copied in once. clip_begin (0 = no room) · clip_track for each track → where its times
  // are written, an index into the singles, its values after them (−1 read unsigned = refused) · clip_end → the clip's number (−1 = refused, nothing kept)
  clip_begin(duration: number, tracks: number): number; clip_track(node: number, path: number, step: number, keys: number): number; clip_end(): number;
  /** room for a pose of `nodes` nodes in game g's pool, zeros: where it starts (−1 here = the pool is full) */
  pose_new(g: number, nodes: number): number;
  /** game g's pool empty again */
  pose_clear(g: number): void;
  // the senses (rust/sim/src/sight.rs, light.rs, smoke.rs, hear.rs, nav.rs — asked through src/scene/collision.ts, THE SENSES IN THE MODULE). 0xffffffff (−1 here) = ask the TypeScript.
  /** guards.ts eyeContact: 1 visible + 2 in the cone; the distance → scratch 0 */
  sight_contact(w: number, set: number, through: number, q: number, e0: number, e1: number, e2: number, l0: number, l1: number, l2: number, range: number, halfDeg: number, nearM: number, nearDeg: number, vHalfDeg: number, c0: number, c1: number, c2: number, h0: number, h1: number, h2: number): number;
  light_lamps(): number; light_cap(): number;
  /** guards.ts lightEstimateAt's loop over the lamps written: the six faces → scratch 0‥5 */
  light_estimate(w: number, set: number, n: number, p0: number, p1: number, p2: number, sigma: number, g: number, stamp: number): number;
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
  // the game's own smoke on the coarse grid (rust/sim/src/bricks.rs; game/sense.ts SmokeGrid): its state is a part of the game's block (state.ts), so a call
  // names the block's slot `g` (`st.bind()`), with the block's `stamp` where it reads the tables made from the state, and a step the box world and its copy
  grid_reset(g: number, home: number): void; grid_clear(g: number): void; grid_dirty(w: number, x0: number, z0: number, x1: number, z1: number): void;
  grid_step(g: number, stamp: number, w: number, copy: number, set: number, n: number, emits: number, decay: number, spread: number, rise: number, riseRate: number, gone: number, lamps: number, albedo: number, share: number, passes: number, sweep: number): number;
  grid_trans(g: number, stamp: number, a0: number, a1: number, a2: number, b0: number, b1: number, b2: number, sigma: number): number;
  // the doors' step (rust/sim/src/doors.rs — src/game/doors.ts Doors.update, over the hand-over src/sim/doorshome.ts names)
  doors_addr(table: number): number; doors_cap(table: number): number;
  /** every leaf of the game in slot g stepped and placed, over world w: the noises listed */
  doors_update(g: number, w: number): number;
  // what falls and what it leaves (rust/sim/src/falls.rs — src/sim/fallshome.ts FallStore, whose rules game/shards.ts calls)
  falls_schedule(g: number, list: number, t: number, x: number, y: number, z: number, name: number, gain: number, rate: number, amount: number, who: number, event: number, level: number, dust: number, cap: number): number;
  falls_deposit(g: number, x: number, z: number, amount: number): void;
  /** the landings due: how many, each 10 doubles of the scratch */
  falls_due(g: number, list: number, time: number, level: number, gap: number): number;
  /** the level a footfall is filed at; how much glass was underfoot → scratch 0 */
  falls_step(g: number, x: number, z: number, level: number, crunchFull: number, grind: number, gain: number, add: number): number;
  falls_heard(g: number, list: number, time: number, gap: number): number;
  falls_clear(g: number, list: number, all: number, events: number): void; falls_cap(what: number): number;
}
/** every export this file expects, for the check that the built module and this face name the same things (tools/qa/rust-check.ts) */
export const SIM_EXPORTS: readonly (keyof SimExports)[] = ['memory', 'sim_abi', 'sim_scratch', 'sim_scratch_len', 'maths_sin', 'maths_cos', 'maths_tan', 'maths_asin', 'maths_acos', 'maths_atan', 'maths_atan2',
  'maths_exp', 'maths_log', 'maths_log2', 'maths_log10', 'maths_pow', 'maths_tanh', 'maths_hypot2', 'maths_hypot3', 'maths_hypot4', 'maths_hypot_n',
  'world_addr', 'world_cap', 'world_shape', 'world_leaves', 'lines_blocked', 'lines_leaves', 'collide_circle', 'collide_touches', 'collide_gather', 'fog_eyes', 'fog_see',
  'pose_addr', 'pose_rig', 'pose_blend', 'pose_run', 'pose_parent_rot', 'pose_turn', 'pose_ik', 'pose_sample', 'clip_begin', 'clip_track', 'clip_end', 'knob_addr', 'knob_len', 'layer_mv_trunk', 'layer_long_body', 'layer_life', 'layer_mv_pre', 'layer_stance_body', 'layer_stand_pose', 'layer_crouch_pose', 'layer_soft_knees', 'layer_run_plant', 'layer_sneak_body', 'layer_steady', 'layer_creep_body', 'layer_kick_body', 'layer_toes_up', 'layer_wall_body', 'layer_watch_slip', 'layer_watch_look', 'layer_mv_post_body', 'layer_turn_weigh', 'layer_lean_body', 'layer_gaze_keep', 'layer_sneak_arms', 'layer_carry_arms', 'layer_wall_arms', 'layer_act_pose', 'layer_wound_body', 'layer_wound_limbs', 'layer_long_arms', 'layer_mv_post_arms', 'layer_watch_arm', 'layer_creep_arms', 'layer_clutch_arm', 'layer_bound_arms', 'layer_reload_arm', 'layer_pistol_reload_arm', 'layer_hands_pass', 'layer_door_layer', 'layer_work_layer', 'layer_ready_layer', 'layer_kit_layer', 'layer_taken_layer', 'layer_blow_legs', 'layer_blow_trunk', 'layer_blow_arms', 'layer_knife_pose', 'layer_cable_pose', 'layer_stealth_layer', 'layer_swap_pose', 'layer_lockpick_layer', 'layer_haul_layer', 'layer_torch_up', 'layer_radio_pose', 'layer_smoke_pose', 'layer_nv_pose', 'layer_signal_layer', 'layer_tell_layer',
  'sight_contact', 'light_lamps', 'light_cap', 'light_estimate', 'smoke_blobs', 'smoke_cap', 'smoke_trans', 'nav_addr', 'nav_cap', 'nav_path', 'hear_direct', 'hear_route',
  'state_addr', 'state_len', 'state_games', 'state_layout', 'state_parts', 'state_part', 'dice_roll',
  'grid_reset', 'grid_clear', 'grid_dirty', 'grid_step', 'grid_trans',
  'pose_new', 'pose_clear', 'doors_addr', 'doors_cap', 'doors_update',
  'falls_schedule', 'falls_deposit', 'falls_due', 'falls_step', 'falls_heard', 'falls_clear', 'falls_cap'];

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
export function simOwns(b: ArrayBufferLike): boolean { return X !== null && f64().buffer === b; }
// THE VIEWS OVER THE MODULE'S MEMORY are made once and kept. A memory that grew would leave them over a buffer the engine has DETACHED, and a view over
// a detached buffer has length 0: that is the test an ask makes (a property of the array, read in place), where it was a call into the engine for
// `memory.buffer` — asked a thousand times a tick by the senses (a lamp, an eye, a blob each write through one). Nothing in the module allocates, so in
// play the memory never does grow (the one memory.grow in its bytes is the standard library's allocator, which rust/prim links and nothing calls).
let F64: Float64Array | null = null, U8: Uint8Array | null = null, U32: Uint32Array | null = null, I32: Int32Array | null = null, F32: Float32Array | null = null;
function views(): Float64Array { const b = sim().memory.buffer; U8 = new Uint8Array(b); U32 = new Uint32Array(b); I32 = new Int32Array(b); F32 = new Float32Array(b); return F64 = new Float64Array(b); }
/** the module's memory as doubles (index = address / 8). Ask again after a call into the module that could allocate: none can today, and a view kept over a
 *  memory that grew would read nothing and write nowhere without a word. */
export function f64(): Float64Array { const v = F64; return v !== null && v.length !== 0 ? v : views(); }
/** the module's memory as bytes (as f64: not to be kept across a call that can allocate) */
export function u8(): Uint8Array { f64(); return U8!; }
/** the module's memory as 32-bit words, unsigned and signed (index = address / 4; as f64: not to be kept across a call that can allocate) */
export function u32(): Uint32Array { f64(); return U32!; }
export function i32(): Int32Array { f64(); return I32!; }
/** the module's memory as singles (index = address / 4): a stored pose (as f64: not to be kept across a call that can allocate) */
export function f32(): Float32Array { f64(); return F32!; }

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
let sightOn = false, lightOn = false, smokeOn = false, soundOn = false, pathsOn = false, walkOn = false;
let doorsOn = false, fallsOn = false;
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
 *  poses   the poses a character keeps (src/anim/skeleton.ts Pose, PoseSet) live in its game's block of the module's memory (rust/sim/src/poses.rs, a tenant:
 *          game/posehome.ts), and the posing functions (a clip sampled, a pose blended, run forward, a node turned, the two-bone solve) are asked of the module, of a kept
 *          pose where it lives and of a function's scratch pose by a copy onto the bench: A PROOF, not a speed-up (a call a question costs more than the
 *          question), until the animator's layers that ask them are Rust too. Where the game's own maths stand. ?rustposes / TPS_RUSTPOSES=1 raise it.
 *  dice    the game's own die (src/math/dice.ts SimDice: the word every draw of the simulation comes off, and the count of draws) kept in the game's block of
 *          the module's memory (src/sim/state.ts) and rolled there. THE FIRST STATE OF THE GAME THAT LIVES IN THE MODULE: it is what the rollback snapshot,
 *          the saves in memory and the state's word were taught to carry a block for. A proof like `maths` (a draw from TypeScript then costs a call), and the
 *          die has to be there before any rule that rolls it is. A game takes the row as its next tick opens. ?rustdice / TPS_RUSTDICE=1 raise it.
 *  walk    the animator's layers (game/character.ts CharacterAnimator: what makes a blend of clips a man who leans into his walk, breathes, keeps his soles
 *          where he put them, flinches, holds a wall), a layer at a time (rust/sim/src/layers.rs): each asked once on the pose where it lives, so it needs
 *          `poses` at home and the game's own maths; the TypeScript method stays behind the row as `<name>Own`. Not yet: the two holds, the big tools, the throw, and the update itself.
 *          ?rustwalk / TPS_RUSTWALK=1 raise it.
 *
 *  THE SENSES, a row each (asked through src/scene/collision.ts; every one inside a tick and where the game's own maths stand, since each works its lengths out
 *  in the module):
 *  sight   an eye on a man (game/guards.ts eyeContact: the cone, then the lines to his chest and his head) as one question. ?rustsight / TPS_RUSTSIGHT=1.
 *  light   the light on a point (guards.ts lightEstimateAt: every lamp in reach, each shadowed by a line) as one question a point, the host writing the
 *          lamps that may be asked. ?rustlight / TPS_RUSTLIGHT=1.
 *  smoke   the game's own smoke along a line (game/sense.ts CpuSmoke.trans, what SENSE.cpu sees through) as one question a line. ?rustsmoke / TPS_RUSTSMOKE=1.
 *  sound   how a sound reaches a listener (scene/collision.ts SoundPaths.propagate: straight, over the top, or round by the floor). ?rustsound / TPS_RUSTSOUND=1.
 *  paths   a way across the floor (collision.ts NavGrid.findPath with no penalty: the A* and its smoothing). ?rustpaths / TPS_RUSTPATHS=1.
 *
 *  THE THINGS of a level, a row a family (each keeps its state in the game's block while its row is on, and WRITES the box world's rows there: the Box
 *  objects the picture draws follow, scene/boxMirror.ts `follow`):
 *  doors   the doors' step (game/doors.ts Doors.update: what was written onto each leaf, the men and bodies in its swing, its swing, stops, closer, latch and
 *          lock's clocks, and where its box stands) from rust/sim/src/doors.rs, the leaves' own numbers at home in the block (sim/doorshome.ts). The state is
 *          at home on either maths; the step runs there inside a tick, where the game's own maths stand and the box world's copy does. ?rustdoors / TPS_RUSTDOORS=1.
 *  falls   what falls and what it leaves (sim/fallshome.ts FallStore: the landings the simulation has scheduled for things that let go, and the litter map of
 *          broken glass a footfall is louder on) kept in the game's block and worked there (rust/sim/src/falls.rs). Either maths. ?rustfalls / TPS_RUSTFALLS=1. */
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
  get doors(): boolean { return doorsOn; },
  set doors(v: boolean) { doorsOn = !!v && !simWhyNot(); },
  get falls(): boolean { return fallsOn; },
  set falls(v: boolean) { fallsOn = !!v && !simWhyNot(); },
  get walk(): boolean { return walkOn; },
  set walk(v: boolean) { walkOn = !!v && !simWhyNot(); },
};
/** the rows that work their lengths and angles out in the module, and so answer only where the game's own maths stand in Math's place (EXACT.on) */
const OWN_MATHS: readonly string[] = ['maths', 'fog', 'poses', 'sight', 'light', 'smoke', 'sound', 'paths', 'doors', 'walk'];
/** THE ROWS THAT ARE ON WHERE THE GAME IS PLAYED (the page, and so the shipped stand; `?rust<row>=0` / `TPS_RUST<ROW>=0` puts one off, `?rust=0` all of
 *  them): the systems that are the same game to the bit in the module AND cost nothing there, measured. The box world's lines are faster in the module;
 *  a circle against it and the fog's sight pass ask the same copy of the boxes and were measured no slower (docs/agents/rust.md, "Rows that are on", has
 *  the numbers). A row that is not here is a proof and no more yet: its state or its layers are right in the module, but while what calls them is
 *  TypeScript every question crosses, and a crossing costs more than the answer (the poses, the walk's layers, the die, the doors, what falls, the maths). The senses' rows
 *  answer only on the CPU-sense stand. A runtime with no WebAssembly keeps every row off whatever this says (the setters below), and plays the same game. */
export const RUST_ON: readonly (keyof typeof RUST)[] = ['lines', 'circles', 'fog'];
/** is `row` one of them */
export const rustOn = (row: string): boolean => (RUST_ON as readonly string[]).includes(row);
/** THE ADDRESS'S WORDS ABOUT THE ROWS (`rust`, and `rust<row>` a row: host/profile.ts pageRows reads them). They are the VISIT's, never a run's: which
 *  language works a system out is this machine's business, so a page sent on to a recipe's address (a replay, a networked game: replay/engineplay.ts
 *  recipeSearch) keeps the visitor's and drops the recipe's own (ui/replaykey.ts VISIT_FLAGS) */
export const rustWords = (): string[] => ['rust', ...Object.keys(RUST).map(r => 'rust' + r)];
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
