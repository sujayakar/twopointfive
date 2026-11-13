//! The numbers of carlamps.
use crate::bytes::ToBytes;
use crate::V3;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct CarlampsK {
    /// the paint of a broken lamp: what is left of the lens
    #[k([0.035, 0.035, 0.04])] pub dark: V3,
}
