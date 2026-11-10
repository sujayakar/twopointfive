//! The numbers of seats.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// `max`: how many seats a game may be stood up with.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct SeatsK {
    #[k(2.0)] pub max: f64,
    /// how far from the first man a later seat's man is stood where nothing says otherwise (m)
    #[k(0.9)] pub apart: f64,
    /// an enemy with nobody as his foe keeps the player he has in mind until another is this much nearer (m): no flicker between two who stand together
    #[k(2.0)] pub keep: f64,
}
