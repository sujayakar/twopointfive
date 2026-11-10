//! The numbers of verdict.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct VerdictK {
    /// seconds between a squad mission's end and its card (the stealth mission's own card keeps its own beat)
    #[k(2.0)] pub win_secs: f64,
    #[k(2.6)] pub loss_secs: f64,
}
