//! The numbers of told.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ToldK {
    /// the rung in force while the lever is on
    #[k(crate::told::ToldRung::Standard)] pub rung: crate::told::ToldRung,
    /// seconds a last-seen mark stays on Guided, and on Standard (Bare: none)
    #[k(10.0)] pub guided: f64,
    #[k(4.0)] pub standard: f64,
}
