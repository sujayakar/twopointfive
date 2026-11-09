//! The numbers of seenwalk.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct SeenwalkK {
    /// floor is lit at this much of the meter (0‥1) · what a block of it costs the walker (× the metre)
    #[k(0.1)] pub lit_at: f64,
    #[k(6.0)] pub dear: f64,
    /// a way round is taken when it is at most this many times as long (+ 1 m) · and this many metres or more the less in view
    #[k(1.35)] pub long: f64,
    #[k(1.5)] pub less: f64,
    /// an eye every this many nav cells each way of an uncleared room's floor · at this height (m)
    #[k(4.0)] pub eye_step: f64,
    #[k(1.6)] pub eye_y: f64,
    /// floor is read inside the short way's box grown by this (m); beyond it every block is dear: a way round does not go further out than that
    #[k(8.0)] pub margin: f64,
}
