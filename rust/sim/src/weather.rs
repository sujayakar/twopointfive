//! WHAT THE LOOPS ASK OF THE WEATHER: is a place under a roof. What the weather is and does is the rules' (game_rules air/weather.rs); the dust's field is stepped in air.rs.

/// the roofs a table holds (a level's four biggest)
pub const ROOFS: usize = crate::snow::MAX_ROOFS;

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
