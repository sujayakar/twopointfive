//! The numbers of winbars.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// the numbers (constants until somebody has played it)
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct WinbarsK {
    /// bars one round can take on its way (a round down a row of windows): the one after is the wall it was to that round
    #[k(4.0)] pub per_round: f64,
    /// the lengths a bar is cut into, and how uneven the cuts are (0 = equal lengths)
    #[k(3.0)] pub pieces: f64,
    #[k(0.5)] pub uneven: f64,
    /// the pieces: speed along the round's way, m/s (the struck length at the top of it, the others at `restK` of theirs) · up · sideways either way · spin, rad/s
    #[k([1.6, 3.4])] pub fly: [f64; 2],
    #[k(0.45)] pub rest_k: f64,
    #[k([0.2, 1.3])] pub up: [f64; 2],
    #[k(0.8)] pub side: f64,
    #[k(7.0)] pub spin: f64,
    /// the chips off it (GPU particles: nothing without them): how many, how fast
    #[k(18.0)] pub chips: f64,
    #[k(2.8)] pub chip_v: f64,
    /// a blast takes the bars within its breaking radius × this (0 = a blast leaves them standing)
    #[k(1.0)] pub blast_k: f64,
}
