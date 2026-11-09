//! The numbers of air sim.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct AirSimK {
    /// a pane more than this much out lets the air through
    #[k(0.5)] pub glass_open: f64,
}
