//! The numbers of flinch.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// the switch and its numbers: on (?flinch=0 = nobody ducks at a round but on the suppression
///  model's own edge, as it was). `least` / `full`: what a round must weigh to duck a
///  man at all, and to duck him all the way — between them from `far` of the duck up to the whole; `minTravel`: m from the muzzle inside which a round ducks
///  nobody (the man at the shooter's elbow); `steadyK` × for a steady defender; `friendly`: a round of his own side ducks him too. `run` (off in the module
///  like the switch itself, raised by the page; ?flinchrun=0): a green man alone under close fire for `breakSecs` s drops it and runs; `hotSecs`: how long
///  after a round that ducked him the fire is still on him; `easeK`: s of that count that drain a second while it is not; `askSecs`: how often a man over the
///  count who has a mate by him is asked again. UNTUNED: every number is a first number.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct FlinchK {
    #[k(0.06)] pub least: f64,
    #[k(0.35)] pub full: f64,
    #[k(0.45)] pub far: f64,
    #[k(1.0)] pub min_travel: f64,
    #[k(0.65)] pub steady_k: f64,
    #[k(2.5)] pub break_secs: f64,
    #[k(1.0)] pub hot_secs: f64,
    #[k(0.5)] pub ease_k: f64,
    #[k(0.25)] pub ask_secs: f64,
}
