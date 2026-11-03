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

pub mod collide;
pub mod dice;
pub mod fog;
pub mod hear;
pub mod light;
pub mod lines;
pub mod maths;
pub mod nav;
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
pub const ABI: u32 = 7;

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
/// skeleton.ts blendPose over the poses in places `a`, `b` → `out`
#[no_mangle]
pub extern "C" fn pose_blend(a: usize, b: usize, w: f64, masked: u32, out: usize, n: usize) {
    skeleton::bench_blend(a, b, w, masked != 0, out, n)
}
/// PoseFK.run (root < 0) / runFrom
#[no_mangle]
pub extern "C" fn pose_run(p: usize, root: i32) {
    skeleton::bench_run(p, root)
}
/// PoseFK.parentRot → args 0‥3
#[no_mangle]
pub extern "C" fn pose_parent_rot(p: usize, i: usize) {
    skeleton::bench_parent_rot(p, i)
}
/// rotateLocal (0), rotateModel (1), setModelRot (2)
#[no_mangle]
pub extern "C" fn pose_turn(p: usize, i: usize, how: u32) {
    skeleton::bench_turn(p, i, how)
}
/// twoBoneIK
#[no_mangle]
pub extern "C" fn pose_ik(p: usize, a: usize, b: usize, c: usize, flags: u32) {
    skeleton::bench_ik(p, a, b, c, flags)
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
/// part `i` of a block: where it starts (`what` 0) or how long it is (1), in bytes
#[no_mangle]
pub extern "C" fn state_part(i: usize, what: u32) -> usize {
    if i >= state::PARTS.len() {
        return 0;
    }
    if what == 0 {
        state::PARTS[i].off
    } else {
        state::PARTS[i].len
    }
}

// ---- the game's own die (dice.rs)
/// dice.ts simRandom: a draw off the die of the game in slot `g`
#[no_mangle]
pub extern "C" fn dice_roll(g: usize) -> f64 {
    dice::roll_of(g)
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
/// guards.ts lightEstimateAt's loop over the first `n` lamps written: the six faces → scratch 0‥5 · 0xffffffff = ask the TypeScript
#[no_mangle]
pub extern "C" fn light_estimate(w: usize, set: i32, n: usize, p0: f64, p1: f64, p2: f64) -> u32 {
    light::estimate(w, set, n, [p0, p1, p2], &mut scratch()[..6])
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
