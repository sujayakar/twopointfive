//! THE ONE FILE OF THE SIMULATION THAT NAMES A SINGLE. The game's arithmetic is in doubles; a local pose is STORED in singles, because the TypeScript keeps it
//! in a Float32Array (src/anim/skeleton.ts Pose). Reading one gives the double it stands for and writing one rounds the double to the nearest single, ties to
//! even: exactly what a Float32Array does. Nothing is ever worked out in singles, so the build's rule against the word stands everywhere else
//! (tools/sim/build.ts RULES names this file as the exception).

/// a stored pose value: nothing outside this file can read one, write one or do arithmetic on one but through `get` and `put`
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct S(f32);
impl S {
    pub const ZERO: S = S(0.0);
}
/// the double a stored value stands for
#[inline(always)]
pub fn get(s: &[S], i: usize) -> f64 {
    s[i].0 as f64
}
/// store a double as a Float32Array does
#[inline(always)]
pub fn put(s: &mut [S], i: usize, v: f64) {
    s[i] = S(v as f32);
}
