//! The numbers of entrypoints.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// The knobs (__game.entry.knobs).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct EntryK {
    /// risky = within this of an uncleared opening (m, to the nearest of its width) … and inside this many degrees off its jambs (the wedge it shows)
    #[k(3.0)] pub near_m: f64,
    #[k(45.0)] pub line_deg: f64,
    /// one of the squad this near an opening, on a side with a line to it, makes that side known (m)
    #[k(4.0)] pub known_m: f64,
    /// how far a point may be moved (m) · its replacement keeps this far from the entry's other points (m)
    #[k(3.0)] pub move_m: f64,
    #[k(0.9)] pub apart: f64,
}
