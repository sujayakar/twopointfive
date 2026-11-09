//! The numbers of noshove.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct NoshoveK {
    /// how near the player's centre such a man's own steps take him (m; the player's own keep-off is 0.55)
    #[k(0.6)] pub ring: f64,
    /// a man left standing inside the player's keep-off steps back out at this pace (m/s)
    #[k(1.2)] pub back: f64,
}
