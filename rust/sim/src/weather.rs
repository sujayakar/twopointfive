//! WHAT THE LOOPS ASK OF THE WEATHER: is a place under a roof. What the weather is and does is the rules' (game_rules air/weather.rs); the dust's field is air.rs's, and
//! its part of a game's block stands here.
use crate::air::Air;
use crate::state::state;
use core::mem::size_of;

/// the roofs a table holds (a level's four biggest)
pub const ROOFS: usize = crate::snow::MAX_ROOFS;

/// the weather's part of a game's block: the dust
#[repr(C)]
pub struct Weather {
    pub air: Air,
}
// (a whole number of sixteens: the pools after it start on a 16-byte boundary, state.rs)
const _: () = assert!(size_of::<Weather>() % 16 == 0);

/// Is (x, z) under one of the first `n` footprints of a flat table (inclusive edges): the thrown things' walk hands its own over.
#[inline]
pub fn under(roofs: &[f64], n: usize, x: f64, z: f64) -> bool {
    roofs.chunks_exact(4).take(n).any(|r| inside(x, z, r[0], r[1], r[2], r[3]))
}
/// …under one of these.
#[inline]
pub fn under_any(roofs: &[[f64; 4]], x: f64, z: f64) -> bool {
    roofs.iter().any(|r| inside(x, z, r[0], r[1], r[2], r[3]))
}
#[inline(always)]
fn inside(x: f64, z: f64, x0: f64, z0: f64, x1: f64, z1: f64) -> bool {
    x >= x0 && z >= z0 && x <= x1 && z <= z1
}

/// the weather's part of slot `g`
#[inline(always)]
pub fn of(g: usize) -> &'static mut Weather {
    &mut state(g).weather
}
/// the same, to read
#[inline(always)]
pub fn of_read(g: usize) -> &'static Weather {
    &crate::state::state_read(g).weather
}
