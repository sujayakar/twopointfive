//! The numbers of holdfire.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct HoldfireK {
    /// how long his shot or his order keeps the hold off after the last of them (s; ROOMFIGHT.teamHot's 45)
    #[k(45.0)] pub lift_secs: f64,
}
