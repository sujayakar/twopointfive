#![allow(unused_imports, unused_variables)]
//! THE BRING-UP'S OWN PART of told: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)

pub use ::game_types::told::types;
pub use types::*;
use crate::game::Game;

// TRANSLATED BY THE BRING-UP: the fourth lever's questions. `TOLD.rung` is a ToldRung in the skeleton, so isRung's guard
// against a stray string has nothing to catch.

/// The rung in force: Bare while the lever is off.
pub fn told_now(g: &crate::game::Game) -> crate::told::ToldRung {
    g.k.told.rung
}

/// The rung a level puts the lever on.
pub fn told_of_level(level: crate::difficulty::Difficulty) -> crate::told::ToldRung {
    if level == crate::difficulty::Difficulty::Hard { ToldRung::Bare } else { ToldRung::Standard }
}
