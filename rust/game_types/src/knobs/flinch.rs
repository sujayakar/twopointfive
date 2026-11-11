//! The numbers of flinch.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// what a round must do to duck a man. `least` / `full`: what it must weigh (the suppression model's `w_near` by how near it passed + `w_hit` by how near it struck) to duck a man
/// at all, and to duck him all the way — between them from `far` of the duck up to the whole; `min_travel`: m from the muzzle inside which a round ducks nobody (the man at the
/// shooter's elbow); `steady_k` × for a steady defender. A round of his own side ducks nobody. THE GREEN MAN'S COUNT: `hot_secs`: how long after a round that ducked him the fire is
/// still on him; `ease_k`: s of that count that drain a second while it is not; `break_secs`: the count at which he breaks; `ask_secs`: how often a man over the count who has a mate
/// by him is asked again. UNTUNED: every number is a first number.
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
