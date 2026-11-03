//! THE SIMULATION IN RUST — the game's update, moved here a system at a time and built to WebAssembly for the page, the game server and the headless tools
//! (docs/agents/rust.md says how a system moves, and what holds it to the TypeScript one it replaces).
//!
//! What every line in this crate owes the game:
//!   · one arithmetic. Doubles, and only + − × ÷ √ and what the standard makes exact of them (floor, trunc, abs, comparisons, the bits). Nothing that a
//!     compiler or a machine may round as it likes: no `f64::sin` and friends (maths.rs has the game's own), no `mul_add`, no fast-math, no `f32` where the
//!     TypeScript holds a double.
//!   · one order. Nothing whose order is not written down: no `HashMap` / `HashSet` iteration, no pointer values as keys, no threads.
//!   · no world outside. No clock, no random source of its own (the game's die is handed in), no files.
//! The exports are plain C functions named `<module>_<thing>`; src/sim/sim.ts is their TypeScript face.
//!
//! WHO HOLDS IT. The compiler: the crate is `no_std`, so no line of it can name a hash map, a thread, a clock or a file (`std::…` does not resolve here).
//! tools/sim/build.ts (its `rules` line, run by tools/qa/rust-check.ts): the float methods the compiler cannot keep out once any crate links the standard
//! library (`.sin()`, `.powf()`, `.mul_add()` …), an `f32`, and the SIMD that rounds as the machine likes. The reader: the order of the arithmetic.
//! √, floor and trunc, which the core library lacks, come from rust/prim.
#![no_std]

pub mod maths;
mod pow10;

/// How many doubles the scratch holds.
pub const SCRATCH_LEN: usize = 1024;
/// A few arguments or answers too many for a call's own parameters go through here: the caller writes, calls, reads. Nothing is kept in it between calls.
static mut SCRATCH: [f64; SCRATCH_LEN] = [0.0; SCRATCH_LEN];

/// The version of the exports: src/sim/sim.ts names the one it was written against, and a page that loaded another says so instead of calling into it.
pub const ABI: u32 = 1;

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
