//! The numbers of takedown.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct TakedownK {
    /// the partner's blow: s after his hands start that it LANDS (the man is counted out at the takedown's own clock, as ever) · s the move lasts — never past the
    ///  takedown's own 0.6 s (PARTNER_SECS): the game hands his arms back to his gun then
    #[k(0.3)] pub partner_hit: f64,
    #[k(0.58)] pub partner_end: f64,
    /// breathing, out cold: breaths a second · m the chest rises · s it takes to come in once the body has settled
    #[k(0.24)] pub breath_hz: f64,
    #[k(0.02)] pub breath_rise: f64,
    #[k(1.2)] pub breath_in: f64,
}
