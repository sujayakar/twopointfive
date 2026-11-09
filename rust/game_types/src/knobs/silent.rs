//! The numbers of silent.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct SilentK {
    /// THE switch: off = no rows on the menu, no word on the HUD, the team as it was whatever the state says
    #[k(true)] pub on: bool,
    /// the posture a freshly staged squad stands up in: true = silent (the loadout screen's row) · false = loud, as ever
    #[k(false)] pub start: bool,
    /// his legs, silent (m/s; 0 = no cap) — and whether the cap lifts for a man loud has reached since the order
    #[k(1.6)] pub pace: f64,
    /// a walking footfall's hearing level, silent (the walk's is 0.33, a run's 0.8; Sam: walk 0.26, crouched 0.125; 0 = as loud)
    #[k(0.2)] pub step: f64,
    #[k(0.5)] pub noticed: f64,
}
