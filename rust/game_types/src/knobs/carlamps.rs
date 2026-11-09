//! The numbers of carlamps.
use crate::bytes::ToBytes;
use crate::V3;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct CarlampsK {
    /// the paint of a broken lamp: what is left of the lens
    #[k([0.035, 0.035, 0.04])] pub dark: V3,
    /// how far a lamp's half size or its twin's place may sit from what the generators write, in metres, and its paint or glow per channel
    #[k(0.004)] pub fit: f64,
    #[k(0.004)] pub tone: f64,
    #[k(0.03)] pub twin: f64,
}
