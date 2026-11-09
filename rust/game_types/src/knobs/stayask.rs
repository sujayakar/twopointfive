//! The numbers of stayask.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// UNTUNED by play.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct StayaskK {
    /// men a squad that stays sends to a silent squad's posts (it keeps a man at home, so a squad of fewer than this and one sends nobody)
    #[k(1.0)] pub men: f64,
}
