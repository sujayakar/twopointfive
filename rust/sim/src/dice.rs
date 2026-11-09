//! THE DIE OF THE GAME IN A SLOT: the die itself is rust/maths's.
pub use maths::dice::*;
use crate::state::state;

/// a draw off the die of the game in slot `g`
pub fn roll_of(g: usize) -> f64 {
    roll(&mut state(g).dice)
}
