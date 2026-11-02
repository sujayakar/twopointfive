//! THE THREE OPERATIONS THE LANGUAGE'S CORE DOES NOT CARRY: √, floor and trunc. Each is exact by the standard (one right answer, the nearest double), so
//! whoever works it out, the bits are the same: in the module the page and the server run, WebAssembly's own `f64.sqrt`, `f64.floor` and `f64.trunc`.
//!
//! WHY A CRATE OF ITS OWN: the simulation (rust/sim) is `no_std`, so that nothing in it can name a hash map, a thread, a clock or a file; and the core library
//! has no square root. This crate is the one place the standard library's is reached, and it is these three functions and nothing else. Add nothing here that
//! is not exact by the standard.

#[inline(always)]
pub fn sqrt(x: f64) -> f64 {
    x.sqrt()
}
#[inline(always)]
pub fn floor(x: f64) -> f64 {
    x.floor()
}
#[inline(always)]
pub fn trunc(x: f64) -> f64 {
    x.trunc()
}
