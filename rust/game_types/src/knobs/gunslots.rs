//! The numbers of gunslots.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct GunslotsK {
    #[k(10.0)] pub near: f64,
    /// seconds from the dry click to the sidearm on his man: the long gun let go onto its sling, the pistol out of the holster and up
    #[k(0.7)] pub out: f64,
    /// seconds with nobody in front of him before the sidearm goes back, and the seconds that takes on top of the primary's own reload
    /// (as long as your own swap back to the long gun: no quicker hands than yours)
    #[k(1.6)] pub lull: f64,
    #[k(0.65)] pub back: f64,
}
