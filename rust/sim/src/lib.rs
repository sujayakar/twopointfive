//! THE SIMULATION IN RUST — the game's update, moved here a system at a time and built to WebAssembly for the page, the game server and the headless tools
//! (docs/agents/rust.md says how a system moves, and what holds it to the TypeScript one it replaces).
//!
//! What every line in this crate owes the game:
//!   · one arithmetic. Doubles, and only + − × ÷ √ and what the standard makes exact of them (floor, trunc, abs, comparisons, the bits). Nothing that a
//!     compiler or a machine may round as it likes: no `f64::sin` and friends (maths.rs has the game's own), no `mul_add`, no fast-math, no `f32` where the
//!     TypeScript holds a double.
//!   · one order. Nothing whose order is not written down: no `HashMap` / `HashSet` iteration, no pointer values as keys, no threads.
//!   · no world outside. No clock, no random source of its own (the game's die is in its block: dice.rs), no files.
//!   · state in one place. What a game keeps here from one call to the next is in its block (state.rs: indices, never addresses); every other `static mut`
//!     is scratch of one call or a copy the host proves.
//! The exports are plain C functions named `<module>_<thing>`; src/sim/sim.ts is their TypeScript face.
//!
//! WHO HOLDS IT. The compiler: the crate is `no_std`, so no line of it can name a hash map, a thread, a clock or a file (`std::…` does not resolve here).
//! tools/sim/build.ts (its `rules` line, run by tools/qa/rust-check.ts): the float methods the compiler cannot keep out once any crate links the standard
//! library (`.sin()`, `.powf()`, `.mul_add()` …), an `f32`, and the SIMD that rounds as the machine likes. The reader: the order of the arithmetic.
//! √, floor and trunc, which the core library lacks, come from rust/prim.
#![no_std]

pub mod bricks;
pub mod clips;
pub mod collide;
pub mod dice;
pub mod doors;
pub mod falls;
pub mod fog;
pub mod glow;
pub mod hear;
pub mod knobs;
pub mod layers;
pub mod light;
pub mod lines;
pub mod maths;
pub mod nav;
pub mod poses;
mod pow10;
pub mod sight;
pub mod single;
pub mod skeleton;
pub mod smoke;
pub mod state;
pub mod world;

/// How many doubles the scratch holds.
pub const SCRATCH_LEN: usize = 1024;
/// A few arguments or answers too many for a call's own parameters go through here: the caller writes, calls, reads. Nothing is kept in it between calls.
static mut SCRATCH: [f64; SCRATCH_LEN] = [0.0; SCRATCH_LEN];

/// The version of the exports: src/sim/sim.ts names the one it was written against, and a page that loaded another says so instead of calling into it.
pub const ABI: u32 = 16;

// ---- the C face (wasm32: these are the module's exports)
#[no_mangle]
pub extern "C" fn sim_abi() -> u32 {
    ABI
}
#[no_mangle]
pub extern "C" fn sim_scratch() -> *mut f64 {
    core::ptr::addr_of_mut!(SCRATCH) as *mut f64
}
#[no_mangle]
pub extern "C" fn sim_scratch_len() -> usize {
    SCRATCH_LEN
}

macro_rules! export1 {
    ($($name:ident => $f:path),* $(,)?) => { $( #[no_mangle] pub extern "C" fn $name(x: f64) -> f64 { $f(x) } )* };
}
export1! {
    maths_sin => maths::sin, maths_cos => maths::cos, maths_tan => maths::tan, maths_asin => maths::asin, maths_acos => maths::acos, maths_atan => maths::atan,
    maths_exp => maths::exp, maths_log => maths::log, maths_log2 => maths::log2, maths_log10 => maths::log10, maths_tanh => maths::tanh,
}
#[no_mangle]
pub extern "C" fn maths_atan2(y: f64, x: f64) -> f64 {
    maths::atan2(y, x)
}
#[no_mangle]
pub extern "C" fn maths_pow(x: f64, y: f64) -> f64 {
    maths::pow(x, y)
}
#[no_mangle]
pub extern "C" fn maths_hypot2(a: f64, b: f64) -> f64 {
    maths::hypot2(a, b)
}
#[no_mangle]
pub extern "C" fn maths_hypot3(a: f64, b: f64, c: f64) -> f64 {
    maths::hypot3(a, b, c)
}
#[no_mangle]
pub extern "C" fn maths_hypot4(a: f64, b: f64, c: f64, d: f64) -> f64 {
    maths::hypot4(a, b, c, d)
}
/// the length of the first `n` doubles of the scratch
#[no_mangle]
pub extern "C" fn maths_hypot_n(n: usize) -> f64 {
    let n = if n > SCRATCH_LEN { SCRATCH_LEN } else { n };
    // (one thread, and nothing else holds the scratch across a call)
    maths::hypot_n(unsafe { core::slice::from_raw_parts(core::ptr::addr_of!(SCRATCH) as *const f64, n) })
}

// ---- the box world's copy (world.rs: written by src/scene/boxMirror.ts) and the line tests through it (lines.rs)
/// where table `table` of world `w` is (a byte address in the module's memory)
#[no_mangle]
pub extern "C" fn world_addr(w: usize, table: u32) -> usize {
    world::addr(w, table)
}
/// how many rows a table holds
#[no_mangle]
pub extern "C" fn world_cap(table: u32) -> usize {
    world::cap(table)
}
/// the grid of world `w` (cells across, cells deep, a cell's side) and how many statics and tilted statics its tables now hold
#[no_mangle]
pub extern "C" fn world_shape(w: usize, gw: u32, gh: u32, cs: f64, boxes: u32, tilted: u32) {
    let o = world::world(w);
    o.w = gw;
    o.h = gh;
    o.cs = cs;
    o.boxes = boxes;
    o.n_tilted = tilted;
}
/// how many leaves list `set` of world `w` now holds
#[no_mangle]
pub extern "C" fn world_leaves(w: usize, set: usize, n: u32) {
    if set < world::LEAF_SETS {
        world::world(w).leaves[set] = n;
    }
}
/// collision.ts segmentBlocked: −1 blocked · n ≥ 0 clear with n panes listed · −2 the pane list is full
#[no_mangle]
pub extern "C" fn lines_blocked(w: usize, a0: f64, a1: f64, a2: f64, b0: f64, b1: f64, b2: f64, len: f64, shrink: f64, through: u32, set: i32, ask: u32) -> i32 {
    lines::blocked(w, a0, a1, a2, b0, b1, b2, len, shrink, through, set, ask)
}
/// collision.ts segmentBlockedDynamic: 1 = a leaf of list `set` carrying every flag of `need` is across the segment
#[no_mangle]
pub extern "C" fn lines_leaves(w: usize, a0: f64, a1: f64, a2: f64, b0: f64, b1: f64, b2: f64, len: f64, need: u32, set: i32) -> u32 {
    lines::leaves_blocked(w, a0, a1, a2, b0, b1, b2, len, need, set) as u32
}

// ---- a circle against the box world (collide.rs)
/// collision.ts collideCircle: 1 = it touched; where it ends is table 12 of the world (x, z)
#[no_mangle]
pub extern "C" fn collide_circle(w: usize, px: f64, pz: f64, r: f64, y0: f64, y1: f64, iterations: u32) -> u32 {
    collide::circle(w, px, pz, r, y0, y1, iterations)
}
/// collision.ts circleTouches: −1 the host's list is too short · −2 longer than the module lists · else touching × 65536 + gathered, in table 13
#[no_mangle]
pub extern "C" fn collide_touches(w: usize, x: f64, z: f64, r: f64, y0: f64, y1: f64, cap: usize) -> i32 {
    collide::touches(w, x, z, r, y0, y1, cap)
}
/// collision.ts gather: how many statics were listed in table 13 (as many as `cap`)
#[no_mangle]
pub extern "C" fn collide_gather(w: usize, x0: f64, z0: f64, x1: f64, z1: f64, y0: f64, y1: f64, cap: usize) -> usize {
    collide::gather(w, x0, z0, x1, z1, y0, y1, cap)
}

// ---- the fog of war's sight pass (fog.rs)
/// where world `w`'s eyes are written: fog::EYE doubles an eye
#[no_mangle]
pub extern "C" fn fog_eyes(w: usize) -> usize {
    fog::eyes_addr(w)
}
/// fogofwar.ts lineToMan / eyeOnBody over the eyes named in `order`: found × 2²⁸ + the eyes found, four bits each · 0xffffffff = ask the TypeScript
#[no_mangle]
pub extern "C" fn fog_see(w: usize, set: i32, order: u32, n: u32, want: u32, kind: u32, through: u32, a0: f64, a1: f64, a2: f64, b0: f64, b1: f64, b2: f64) -> u32 {
    fog::see(w, set, order, n, want, kind, through, a0, a1, a2, b0, b1, b2)
}

// ---- the knobs and the rig's named nodes that Rust code reads (knobs.rs), and the animator's layers that have moved (layers.rs)
/// where a table of knobs.rs is: 0 the knobs' values · 1 their names · 2 the roles' nodes · 3 their names
#[no_mangle]
pub extern "C" fn knob_addr(table: u32) -> usize {
    knobs::addr(table)
}
/// …and how long (values, bytes, nodes, bytes)
#[no_mangle]
pub extern "C" fn knob_len(table: u32) -> usize {
    knobs::len(table)
}
// A LAYER (layers.rs) is asked as a question of a pose is (skeleton.rs): 0 = not a pose, or a role with no node · else it ran (bit 0), and a bit above for
// all of the run-forward bones on the bench being current (bit 1) and for each list of nodes it ran forward there (from bit 2).
/// CharacterAnimator.mvTrunk
#[no_mangle]
pub extern "C" fn layer_mv_trunk(g: usize, at: usize) -> u32 {
    layers::mv_trunk(g, at)
}
/// CharacterAnimator.longBody
#[no_mangle]
pub extern "C" fn layer_long_body(g: usize, at: usize) -> u32 {
    layers::long_body(g, at)
}
/// CharacterAnimator.mvPre
#[no_mangle]
pub extern "C" fn layer_mv_pre(g: usize, at: usize) -> u32 {
    layers::mv_pre(g, at)
}
/// CharacterAnimator.stanceBody
#[no_mangle]
pub extern "C" fn layer_stance_body(g: usize, at: usize) -> u32 {
    layers::stance_body(g, at)
}
/// CharacterAnimator.standPose (jogShape and sprintShape are steps of it), crouchPose
#[no_mangle]
pub extern "C" fn layer_stand_pose(g: usize, at: usize, at_b: usize, at_c: usize) -> u32 {
    layers::stand_pose(g, at, at_b, at_c)
}
#[no_mangle]
pub extern "C" fn layer_crouch_pose(g: usize, at: usize, at_b: usize, at_c: usize) -> u32 {
    layers::crouch_pose(g, at, at_b, at_c)
}
/// CharacterAnimator.softKnees
#[no_mangle]
pub extern "C" fn layer_soft_knees(g: usize, at: usize) -> u32 {
    layers::soft_knees(g, at)
}
/// CharacterAnimator.runPlant
#[no_mangle]
pub extern "C" fn layer_run_plant(g: usize, at: usize) -> u32 {
    layers::run_plant(g, at)
}
/// CharacterAnimator.sneakBody
#[no_mangle]
pub extern "C" fn layer_sneak_body(g: usize, at: usize) -> u32 {
    layers::sneak_body(g, at)
}
/// CharacterAnimator.steadyLayer (`at_c`: the pose that holds the cycle's own sample of this frame)
#[no_mangle]
pub extern "C" fn layer_steady(g: usize, at: usize, at_c: usize) -> u32 {
    layers::steady_layer(g, at, at_c)
}
/// CharacterAnimator.kickBody (`at_c`: its scratch pose)
#[no_mangle]
pub extern "C" fn layer_kick_body(g: usize, at: usize, at_c: usize) -> u32 {
    layers::kick_body(g, at, at_c)
}
/// CharacterAnimator.toesUp
#[no_mangle]
pub extern "C" fn layer_toes_up(g: usize, at: usize) -> u32 {
    layers::toes_up(g, at)
}
/// CharacterAnimator.wallBody (`at_c`, `at_b`: the standing and the crouched idle, sampled by the host)
#[no_mangle]
pub extern "C" fn layer_wall_body(g: usize, at: usize, at_c: usize, at_b: usize, at_rest: usize) -> u32 {
    layers::wall_body(g, at, at_c, at_b, at_rest)
}
/// CharacterAnimator.watchSlip, watchLook
#[no_mangle]
pub extern "C" fn layer_watch_slip(g: usize, at: usize) -> u32 {
    layers::watch_slip(g, at)
}
#[no_mangle]
pub extern "C" fn layer_watch_look(g: usize, at: usize) -> u32 {
    layers::watch_look(g, at)
}
#[no_mangle]
pub extern "C" fn layer_mv_post_body(g: usize, at: usize) -> u32 {
    layers::mv_post_body(g, at)
}
#[no_mangle]
pub extern "C" fn layer_turn_weigh() -> u32 {
    layers::turn_weigh()
}
#[no_mangle]
pub extern "C" fn layer_lean_body(g: usize, at: usize) -> u32 {
    layers::lean_body(g, at)
}
#[no_mangle]
pub extern "C" fn layer_gaze_keep(g: usize, at: usize) -> u32 {
    layers::gaze_keep(g, at)
}
#[no_mangle]
pub extern "C" fn layer_sneak_arms(g: usize, at: usize, at_b: usize) -> u32 {
    layers::sneak_arms(g, at, at_b)
}
#[no_mangle]
pub extern "C" fn layer_carry_arms(g: usize, at: usize, at_b: usize, at_aim: usize) -> u32 {
    layers::carry_arms(g, at, at_b, at_aim)
}
#[no_mangle]
pub extern "C" fn layer_wall_arms(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    layers::wall_arms(g, at, at_b, at_rest)
}
#[no_mangle]
pub extern "C" fn layer_act_pose(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    layers::act_pose(g, at, at_b, at_rest)
}
#[no_mangle]
pub extern "C" fn layer_wound_body(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    layers::wound_body(g, at, at_b, at_rest)
}
#[no_mangle]
pub extern "C" fn layer_wound_limbs(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    layers::wound_limbs(g, at, at_b, at_rest)
}
#[no_mangle]
pub extern "C" fn layer_long_arms(g: usize, at: usize, at_b: usize, at_aim: usize, at_rest: usize) -> u32 {
    layers::long_arms(g, at, at_b, at_aim, at_rest)
}
/// CharacterAnimator.watchArm
#[no_mangle]
pub extern "C" fn layer_watch_arm(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    layers::watch_arm(g, at, at_b, at_rest)
}
/// CharacterAnimator.creepArms
#[no_mangle]
pub extern "C" fn layer_creep_arms(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    layers::creep_arms(g, at, at_b, at_rest)
}
/// CharacterAnimator.clutchArm
#[no_mangle]
pub extern "C" fn layer_clutch_arm(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    layers::clutch_arm(g, at, at_b, at_rest)
}
/// CharacterAnimator.boundArms
#[no_mangle]
pub extern "C" fn layer_bound_arms(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    layers::bound_arms(g, at, at_b, at_rest)
}
/// CharacterAnimator.reloadArm
#[no_mangle]
pub extern "C" fn layer_reload_arm(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    layers::reload_arm(g, at, at_b, at_rest)
}
/// CharacterAnimator.pistolReloadArm
#[no_mangle]
pub extern "C" fn layer_pistol_reload_arm(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    layers::pistol_reload_arm(g, at, at_b, at_rest)
}
/// CharacterAnimator.smokePose
#[no_mangle]
pub extern "C" fn layer_smoke_pose(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    layers::smoke_pose(g, at, at_b, at_rest)
}
/// CharacterAnimator.nvPose
#[no_mangle]
pub extern "C" fn layer_nv_pose(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    layers::nv_pose(g, at, at_b, at_rest)
}
/// CharacterAnimator.signalLayer
#[no_mangle]
pub extern "C" fn layer_signal_layer(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    layers::signal_layer(g, at, at_b, at_rest)
}
/// CharacterAnimator.tellLayer
#[no_mangle]
pub extern "C" fn layer_tell_layer(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    layers::tell_layer(g, at, at_b, at_rest)
}
/// CharacterAnimator.torchUpLayer
#[no_mangle]
pub extern "C" fn layer_torch_up(g: usize, at: usize, at_b: usize) -> u32 {
    layers::torch_up(g, at, at_b)
}
/// CharacterAnimator.radioPose
#[no_mangle]
pub extern "C" fn layer_radio_pose(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    layers::radio_pose(g, at, at_b, at_rest)
}
/// CharacterAnimator.lockpickLayer
#[no_mangle]
pub extern "C" fn layer_lockpick_layer(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    layers::lockpick_layer(g, at, at_b, at_rest)
}
/// CharacterAnimator.haulLayer
#[no_mangle]
pub extern "C" fn layer_haul_layer(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    layers::haul_layer(g, at, at_b, at_rest)
}
/// CharacterAnimator.swapPose
#[no_mangle]
pub extern "C" fn layer_swap_pose(g: usize, at: usize, at_b: usize, at_rest: usize, at_sw: usize) -> u32 {
    layers::swap_pose(g, at, at_b, at_rest, at_sw)
}
/// CharacterAnimator.stealthLayer
#[no_mangle]
pub extern "C" fn layer_stealth_layer(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    layers::stealth_layer(g, at, at_b, at_rest)
}
/// CharacterAnimator.cablePose
#[no_mangle]
pub extern "C" fn layer_cable_pose(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    layers::cable_pose(g, at, at_b, at_rest)
}
/// CharacterAnimator.takenLayer
#[no_mangle]
pub extern "C" fn layer_taken_layer(g: usize, at: usize) -> u32 {
    layers::taken_layer(g, at)
}
/// CharacterAnimator.blowLegs
#[no_mangle]
pub extern "C" fn layer_blow_legs(g: usize, at: usize) -> u32 {
    layers::blow_legs(g, at)
}
/// CharacterAnimator.blowTrunk
#[no_mangle]
pub extern "C" fn layer_blow_trunk(g: usize, at: usize) -> u32 {
    layers::blow_trunk(g, at)
}
/// CharacterAnimator.blowArms
#[no_mangle]
pub extern "C" fn layer_blow_arms(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    layers::blow_arms(g, at, at_b, at_rest)
}
/// CharacterAnimator.knifePose
#[no_mangle]
pub extern "C" fn layer_knife_pose(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    layers::knife_pose(g, at, at_b, at_rest)
}
/// CharacterAnimator.readyLayer
#[no_mangle]
pub extern "C" fn layer_ready_layer(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    layers::ready_layer(g, at, at_b, at_rest)
}
/// CharacterAnimator.kitLayer
#[no_mangle]
pub extern "C" fn layer_kit_layer(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    layers::kit_layer(g, at, at_b, at_rest)
}
/// CharacterAnimator.doorLayer
#[no_mangle]
pub extern "C" fn layer_door_layer(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    layers::door_layer(g, at, at_b, at_rest)
}
/// CharacterAnimator.workLayer
#[no_mangle]
pub extern "C" fn layer_work_layer(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    layers::work_layer(g, at, at_b, at_rest)
}
/// CharacterAnimator.handsPass
#[no_mangle]
pub extern "C" fn layer_hands_pass(g: usize, at: usize) -> u32 {
    layers::hands_pass(g, at)
}
/// CharacterAnimator.mvPostArms
#[no_mangle]
pub extern "C" fn layer_mv_post_arms(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    layers::mv_post_arms(g, at, at_b, at_rest)
}
/// CharacterAnimator.creepBody
#[no_mangle]
pub extern "C" fn layer_creep_body(g: usize, at: usize) -> u32 {
    layers::creep_body(g, at)
}
/// CharacterAnimator.lifeLayer
#[no_mangle]
pub extern "C" fn layer_life(g: usize, at: usize) -> u32 {
    layers::life_layer(g, at)
}

// ---- posing on the bench (skeleton.rs: the host writes a question's pose there, asks, and reads the answer back)
/// where a table of the bench is: 0 rot · 1 pos · 2 args · 3 pose · 4 mask · 5 parent · 6 order
#[no_mangle]
pub extern "C" fn pose_addr(table: u32) -> usize {
    skeleton::addr(table)
}
/// the rig written to the bench has `nodes` nodes, `order` of them listed parents first: 1 = it is one the walks can index
#[no_mangle]
pub extern "C" fn pose_rig(nodes: u32, order: u32) -> u32 {
    skeleton::bench_rig(nodes, order);
    skeleton::bench_rig_ok() as u32
}
// A QUESTION OF A POSE says where the pose is (skeleton.rs): `g` = 0xffffffff → place `at` of the bench · else game `g`'s pool, the pose starting at single
// `at` of the part. 1 = done · 0 = not a pose (or not a node), nothing written.
/// skeleton.ts blendPose: poses `a`, `b` → `out`
#[no_mangle]
pub extern "C" fn pose_blend(g: usize, a: usize, b: usize, w: f64, masked: u32, out: usize, n: usize) -> u32 {
    skeleton::ask_blend(g, a, b, w, masked != 0, out, n) as u32
}
/// PoseFK.run (root < 0) / runFrom
#[no_mangle]
pub extern "C" fn pose_run(g: usize, at: usize, root: i32) -> u32 {
    skeleton::ask_run(g, at, root) as u32
}
/// PoseFK.parentRot → args 0‥3
#[no_mangle]
pub extern "C" fn pose_parent_rot(g: usize, at: usize, i: usize) -> u32 {
    skeleton::ask_parent_rot(g, at, i) as u32
}
/// rotateLocal (0), rotateModel (1), setModelRot (2)
#[no_mangle]
pub extern "C" fn pose_turn(g: usize, at: usize, nodes: usize, i: usize, how: u32) -> u32 {
    skeleton::ask_turn(g, at, nodes, i, how) as u32
}
/// twoBoneIK
#[no_mangle]
pub extern "C" fn pose_ik(g: usize, at: usize, a: usize, b: usize, c: usize, flags: u32) -> u32 {
    skeleton::ask_ik(g, at, a, b, c, flags) as u32
}
/// skeleton.ts sampleClip: clip `clip` at time `t` into the pose (`nodes` = the pose's own count of nodes)
#[no_mangle]
pub extern "C" fn pose_sample(g: usize, at: usize, nodes: usize, clip: usize, t: f64, looped: u32) -> u32 {
    skeleton::ask_sample(g, at, nodes, clip, t, looped != 0) as u32
}
// ---- the clips a pose is sampled from (clips.rs: a character file's animations, copied in once by src/anim/skeleton.ts)
/// a clip of `tracks` tracks is about to be written: 0 = no room, the host samples it itself
#[no_mangle]
pub extern "C" fn clip_begin(duration: f64, tracks: usize) -> u32 {
    clips::begin(duration, tracks) as u32
}
/// its next track: where the host writes the `keys` times, the values after them (an index into the module's singles) · 0xffffffff = refused
#[no_mangle]
pub extern "C" fn clip_track(node: usize, path: u32, step: u32, keys: usize) -> u32 {
    clips::track(node, path, step != 0, keys)
}
/// the clip is whole: its number · −1 = refused, nothing of it kept
#[no_mangle]
pub extern "C" fn clip_end() -> i32 {
    clips::end()
}
// ---- the poses a game keeps (poses.rs)
/// room for a pose of `nodes` nodes in game `g`'s pool, zeros: where it starts · 0xffffffff = the pool is full
#[no_mangle]
pub extern "C" fn pose_new(g: usize, nodes: usize) -> u32 {
    poses::new_of(g, nodes)
}
/// game `g`'s pool empty again (every pose taken out of it first: game/posehome.ts)
#[no_mangle]
pub extern "C" fn pose_clear(g: usize) {
    poses::clear_of(g)
}

// ---- the game's state in the module (state.rs: a block a game, bound to a slot by src/sim/state.ts)
/// where slot `g`'s block is (a byte address)
#[no_mangle]
pub extern "C" fn state_addr(g: usize) -> usize {
    state::addr(g)
}
/// a block's length in bytes
#[no_mangle]
pub extern "C" fn state_len() -> usize {
    state::LEN
}
/// how many blocks the module holds at once
#[no_mangle]
pub extern "C" fn state_games() -> usize {
    state::GAMES
}
/// the layout's word
#[no_mangle]
pub extern "C" fn state_layout() -> u32 {
    state::LAYOUT
}
/// how many parts a block has
#[no_mangle]
pub extern "C" fn state_parts() -> usize {
    state::PARTS.len()
}
/// part `i` of a block: where it starts (`what` 0) or how long it is (1), in bytes; whether it is a pool, its first word the bytes of it in use (2)
#[no_mangle]
pub extern "C" fn state_part(i: usize, what: u32) -> usize {
    if i >= state::PARTS.len() {
        return 0;
    }
    match what {
        0 => state::PARTS[i].off,
        1 => state::PARTS[i].len,
        _ => state::PARTS[i].pool as usize,
    }
}

// ---- the game's own die (dice.rs)
/// dice.ts simRandom: a draw off the die of the game in slot `g`
#[no_mangle]
pub extern "C" fn dice_roll(g: usize) -> f64 {
    dice::roll_of(g)
}

// ---- the doors' step (doors.rs: the leaves' state in the game's block, the men near them handed in, each leaf's row of the box world written)
/// where a table of the step's hand-over is: 0 the level's doors · 1 what comes back of each · 2 the contacts · 3 the noises · 4 the step's numbers
#[no_mangle]
pub extern "C" fn doors_addr(table: u32) -> usize {
    doors::addr(table)
}
/// how many rows a table holds (5: the columns of the doors' part of a block)
#[no_mangle]
pub extern "C" fn doors_cap(table: u32) -> usize {
    doors::cap(table)
}
/// doors.ts Doors.update for the game in slot `g`, over world `w`: how many noises were listed
#[no_mangle]
pub extern "C" fn doors_update(g: usize, w: usize) -> u32 {
    doors::update(g, w)
}

// ---- what falls and what it leaves (falls.rs: the landings the simulation has scheduled, a list a kind of thing, and the glass lying on the floor, in the
// game's block). `l` is the list; one the module does not have is answered as nothing.
/// 1 = the landing waits · 0 = the list was at its cap, and its glass is laid now
#[no_mangle]
pub extern "C" fn falls_schedule(g: usize, l: u32, t: f64, x: f64, y: f64, z: f64, name: f64, gain: f64, rate: f64, amount: f64, who: f64, event: f64, level: f64, dust: f64, cap: f64) -> u32 {
    if l as usize >= falls::NLIST {
        return 0;
    }
    falls::schedule(g, l as usize, t, x, y, z, name, gain, rate, amount, who, event, level, dust, cap)
}
/// shards.ts deposit
#[no_mangle]
pub extern "C" fn falls_deposit(g: usize, x: f64, z: f64, amount: f64) {
    falls::deposit(&mut state::state(g).falls, x, z, amount)
}
/// the landings of list `l` due at `time`: how many, each falls::DUE doubles of the scratch
#[no_mangle]
pub extern "C" fn falls_due(g: usize, l: u32, time: f64, level: f64, gap: f64) -> u32 {
    if l as usize >= falls::NLIST {
        return 0;
    }
    falls::due(g, l as usize, time, level, gap, &mut scratch()[..])
}
/// shards.ts step: the level a footfall at (x, z) is filed at; how much glass was underfoot → scratch 0
#[no_mangle]
pub extern "C" fn falls_step(g: usize, x: f64, z: f64, level: f64, crunch_full: f64, grind: f64, gain: f64, add: f64) -> f64 {
    falls::step(g, x, z, level, crunch_full, grind, gain, add, &mut scratch()[0])
}
/// nothing waits in list `l`; all = 1: the floor's memory of that list's last landing is set to `events` too, and with the glass's list the glass that lies goes
#[no_mangle]
pub extern "C" fn falls_clear(g: usize, l: u32, all: u32, events: f64) {
    if (l as usize) < falls::NLIST {
        falls::clear(g, l as usize, all != 0, events)
    }
}
/// 1 = the floor is told of a landing of list `l` now (and that is remembered)
#[no_mangle]
pub extern "C" fn falls_heard(g: usize, l: u32, time: f64, gap: f64) -> u32 {
    if l as usize >= falls::NLIST {
        return 0;
    }
    falls::heard(g, l as usize, time, gap)
}

/// the falls' own sizes: 0 cells across · 1 cells deep · 2 landings that can wait in a list · 3 columns a landing · 4 doubles a due landing · 5 lists
#[no_mangle]
pub extern "C" fn falls_cap(what: u32) -> usize {
    match what {
        0 => falls::CELLS_X,
        1 => falls::CELLS_Z,
        2 => falls::MAXP,
        3 => falls::NP,
        4 => falls::DUE,
        5 => falls::NLIST,
        _ => 0,
    }
}

// ---- the senses (sight.rs, light.rs, smoke.rs, hear.rs, nav.rs): what a man sees, how lit he is, what the smoke hides, how a sound reaches him
/// the scratch, for an answer of more than one number (one thread, and nothing else holds it across a call)
#[inline(always)]
fn scratch() -> &'static mut [f64; SCRATCH_LEN] {
    unsafe { &mut *core::ptr::addr_of_mut!(SCRATCH) }
}
/// guards.ts eyeContact: 1 visible + 2 in the cone · 0xffffffff = ask the TypeScript. The distance → scratch 0.
#[no_mangle]
pub extern "C" fn sight_contact(w: usize, set: i32, through: u32, q: u32, e0: f64, e1: f64, e2: f64, l0: f64, l1: f64, l2: f64, range: f64, half_deg: f64, near_m: f64, near_deg: f64, v_half_deg: f64, c0: f64, c1: f64, c2: f64, h0: f64, h1: f64, h2: f64) -> u32 {
    sight::contact(w, set, through, q, [e0, e1, e2], [l0, l1, l2], range, half_deg, near_m, near_deg, v_half_deg, [c0, c1, c2], [h0, h1, h2], &mut scratch()[0])
}
/// where a question's lamps are written (light::LAMP doubles a lamp), and how many it can carry
#[no_mangle]
pub extern "C" fn light_lamps() -> usize {
    light::lamps_addr()
}
#[no_mangle]
pub extern "C" fn light_cap() -> usize {
    light::MAX_LAMPS
}
/// guards.ts lightEstimateAt's loop over the first `n` lamps written: the six faces → scratch 0‥5 · 0xffffffff = ask the TypeScript. `sigma` > 0: each
/// lamp is dimmed by the smoke of the coarse grid of the game in slot `g`, whose block's stamp is `stamp` (bricks.rs); 0: neither is read
#[no_mangle]
pub extern "C" fn light_estimate(w: usize, set: i32, n: usize, p0: f64, p1: f64, p2: f64, sigma: f64, g: usize, stamp: f64) -> u32 {
    light::estimate(w, set, n, [p0, p1, p2], sigma, g, stamp, &mut scratch()[..6])
}
/// where a question's blobs are written (smoke::BLOB doubles a blob), and how many it can carry
#[no_mangle]
pub extern "C" fn smoke_blobs() -> usize {
    smoke::blobs_addr()
}
#[no_mangle]
pub extern "C" fn smoke_cap() -> usize {
    smoke::MAX_BLOBS
}
/// sense.ts CpuSmoke.trans over the first `n` blobs written: the transmittance → scratch 0 · 0xffffffff = ask the TypeScript
#[no_mangle]
pub extern "C" fn smoke_trans(w: usize, set: i32, n: usize, a0: f64, a1: f64, a2: f64, b0: f64, b1: f64, b2: f64, sigma: f64, spread: f64, widest: f64, rise: f64, rise_rate: f64) -> u32 {
    smoke::trans(w, set, n, [a0, a1, a2], [b0, b1, b2], sigma, spread, widest, rise, rise_rate, &mut scratch()[0])
}
/// the nav grid's tables: 0 the shut cells (a byte a cell, written by the host before it asks) · 1 a way's waypoints (x z a point)
#[no_mangle]
pub extern "C" fn nav_addr(table: u32) -> usize {
    match table {
        0 => nav::shut_addr(),
        1 => nav::way_addr(),
        _ => 0,
    }
}
#[no_mangle]
pub extern "C" fn nav_cap(table: u32) -> usize {
    match table {
        0 => nav::MAX_NAV,
        1 => nav::MAX_WAY,
        _ => 0,
    }
}
/// collision.ts NavGrid.findPath with no penalty: how many waypoints were written · −1 = no way · −2 = ask the TypeScript
#[no_mangle]
pub extern "C" fn nav_path(w: usize, set: i32, ask: u32, gw: usize, gh: usize, cell: f64, a0: f64, a2: f64, b0: f64, b2: f64, max_iter: u32) -> i32 {
    nav::find_path(w, set, ask, gw, gh, cell, a0, a2, b0, b2, max_iter)
}
/// collision.ts SoundPaths.propagate, up to its memory of routed answers: hear::DIRECT (+ the grazes) / OVER / FAR / WALLED · 0xffffffff = ask the
/// TypeScript. The straight distance → scratch 0.
#[no_mangle]
pub extern "C" fn hear_direct(w: usize, set: i32, ask: u32, s0: f64, s1: f64, s2: f64, d0: f64, d1: f64, d2: f64) -> u32 {
    hear::direct(w, set, ask, [s0, s1, s2], [d0, d1, d2], &mut scratch()[0])
}
/// …and past it: the way round. hear::SEALED / ROUTED / ROUTED_DOOR · 0xffffffff = ask the TypeScript. The path's length and the point it comes through →
/// scratch 0‥3.
#[no_mangle]
pub extern "C" fn hear_route(w: usize, set: i32, ask: u32, seal: u32, gw: usize, gh: usize, cell: f64, s0: f64, s1: f64, s2: f64, d0: f64, d1: f64, d2: f64, straight: f64) -> u32 {
    hear::route(w, set, ask, seal, gw, gh, cell, [s0, s1, s2], [d0, d1, d2], straight, &mut scratch()[..4])
}

// ---- the game's own smoke on the coarse grid (bricks.rs): its state is a part of the game's block (state.rs), so a call names the block's slot `g`;
// one that reads the tables made from the state names the block's `stamp` too (src/sim/state.ts SimState.stamp), and a step the box world `w` it is
// taken over and that copy's number `copy` (src/scene/boxMirror.ts)
/// the game in slot `g` has a grid, an empty one (`home` 1), or has none and its part is zeros (0)
#[no_mangle]
pub extern "C" fn grid_reset(g: usize, home: u32) {
    bricks::reset(g, home != 0)
}
/// no smoke anywhere in the grid of the game in slot `g` (the plan and the light that comes round stay)
#[no_mangle]
pub extern "C" fn grid_clear(g: usize) {
    bricks::empty(g)
}
/// a static of box world `w` was written inside this rectangle of the plan: the faces near it are asked again at the next step
#[no_mangle]
pub extern "C" fn grid_dirty(w: usize, x0: f64, z0: f64, x1: f64, z1: f64) {
    bricks::dirty(w, x0, z0, x1, z1)
}
/// `n` steps of a tenth of a second under the model's numbers, over the copy as it stands and the leaves of list `set`: 0 · 0xffffffff = no grid stands
/// on this level, and nothing was done. `emits`: that many emitters join the list first, written in the scratch from its start, six doubles each
/// (where, how wide in metres, what it lets out a second in unit·m³, for how long). `lamps` ≥ 0 = the light that comes round is stepped too (glow.rs),
/// from that many lamps written as for a question about the light
#[no_mangle]
pub extern "C" fn grid_step(g: usize, stamp: f64, w: usize, copy: f64, set: i32, n: u32, emits: usize, decay: f64, spread: f64, rise: f64, rise_rate: f64, gone: f64, lamps: i32, albedo: f64, share: f64, passes: u32, sweep: f64) -> u32 {
    let k = bricks::K { decay, spread, rise, rise_rate, gone };
    let emits = &scratch()[..if emits > SCRATCH_LEN / 6 { SCRATCH_LEN / 6 * 6 } else { emits * 6 }];
    if lamps < 0 {
        bricks::step(g, stamp, w, copy, set, n, emits, &k, None)
    } else {
        bricks::step(g, stamp, w, copy, set, n, emits, &k, Some(&glow::K { albedo, share, passes, sweep, lamps: lamps as usize }))
    }
}
/// how much of the light along a → b comes through the grid's smoke (1 = clear air)
#[no_mangle]
pub extern "C" fn grid_trans(g: usize, stamp: f64, a0: f64, a1: f64, a2: f64, b0: f64, b1: f64, b2: f64, sigma: f64) -> f64 {
    bricks::trans(g, stamp, [a0, a1, a2], [b0, b1, b2], sigma)
}
