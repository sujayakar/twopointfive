//! The die of the game in a slot.
pub use prim::dice::*;
use crate::state::state;

/// a draw off the die of the game in slot `g`
pub fn roll_of(g: usize) -> f64 {
    roll(&mut state(g).dice)
}
