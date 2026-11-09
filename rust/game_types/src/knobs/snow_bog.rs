//! The numbers of snow bog.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct SnowbogK {
    /// of its bounce a thing loses in deep snow (0 = the bounce as on a floor, 1 = none at all)
    #[k(0.85)] pub dead: f64,
    /// of the slide it would have kept, lost (on top of the item's own friction)
    #[k(0.8)] pub grip: f64,
    /// m/s: it stops dead where it lands moving slower than its floor speed + this (a floor's: 0.35)
    #[k(1.1)] pub stick: f64,
    /// × all three on trodden snow (the road)
    #[k(0.4)] pub packed: f64,
    /// … on old snow
    #[k(0.7)] pub old: f64,
    /// m: a landing higher than this is not on the GROUND (the ground's collision is flat at 0 under the drawn hills; a thin lay tops out at 3 cm): a kerb,
    ///    a pallet, a desk, a sill — and bogs only if snow stands or lies that high under it: a bank, the cap on a roof (was 0.45: a low kerb bogged as deep snow)
    #[k(0.06)] pub top: f64,
    /// m: … snow whose top is within this of the landing is what it landed in
    #[k(0.15)] pub bank_m: f64,
    /// the squad's forecast of a throw: the run-on past the landing is a floor's × (1 − bog) to this power
    #[k(2.0)] pub run: f64,
}
