//! The numbers of aimcost.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct AimcostK {
    /// m: the round's line this near the bulb's own point is a hit
    #[k(0.12)] pub bare: f64,
    /// m: a fixture's own boxes are grown this much for the round
    #[k(0.03)] pub pad: f64,
    /// m: how far past where the round stopped the thing may sit (a bulb set into the ceiling it stopped in)
    #[k(0.15)] pub slack: f64,
    /// × the cone's feet term
    #[k(2.0)] pub move_k: f64,
    /// s: the remembered feet let go over this
    #[k(0.35)] pub stop_tau: f64,
    /// m/s²: taken straight off the remembered feet as well, so they reach nothing in a finite time
    #[k(0.1)] pub stop_bleed: f64,
    /// 0‥1: how much of the rifle's brace reaches the feet term (1 = all of it, as it was)
    #[k(0.5)] pub brace_feet: f64,
    /// m: the same for a wall device (a camera's body is longer than a bulb)
    #[k(0.16)] pub bare_device: f64,
    /// m/s: a guard starts no burst while his remembered feet are over this (never looser than his own `stopToShoot`)
    #[k(1.2)] pub plant_at: f64,
    /// m/s: a man takes an aimed round PAST A COLLEAGUE in Sam's arm only with his remembered feet under this —
    ///  the line is 0.45 m clear of his own man, and a cone still open from the walk round found him (standoff-flank, the page's stand)
    #[k(0.3)] pub flank_at: f64,
}
