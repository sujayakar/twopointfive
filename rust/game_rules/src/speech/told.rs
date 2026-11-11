#![allow(unused_imports, unused_variables)]
pub use ::game_types::told::types;
pub use types::*;
use crate::game::Game;

// The fourth lever's questions. `g.k.told.rung` is a ToldRung, so no stray string can reach them.

/// The rung in force: Bare while the lever is off.
pub fn told_now(g: &crate::game::Game) -> crate::told::ToldRung {
    g.k.told.rung
}

/// The rung a level puts the lever on.
pub fn told_of_level(level: crate::difficulty::Difficulty) -> crate::told::ToldRung {
    if level == crate::difficulty::Difficulty::Hard { ToldRung::Bare } else { ToldRung::Standard }
}
