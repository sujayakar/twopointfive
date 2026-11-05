#![allow(unused_variables)]
//! Every function and method of `noshove`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/noshove.ts:48 noShoveMan)
pub fn no_shove_man(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/noshove.ts:56 noShoveBegin)
pub fn no_shove_begin(g: &mut crate::game::Game, pl: Option<&mut crate::noshove::Shoved>) { todo!() }

/// (src/game/noshove.ts:61 noShoveNote)
pub fn no_shove_note(g: &mut crate::game::Game, dx: f64, dz: f64, pl: Option<&mut crate::noshove::Shoved>) { todo!() }

/// (src/game/noshove.ts:69 noShoveBack)
pub fn no_shove_back(g: &mut crate::game::Game, mx: f64, mz: f64, l: f64, pl: Option<&crate::noshove::Shoved>) -> f64 { todo!() }

/// (src/game/noshove.ts:80 noShoveOwn)
pub fn no_shove_own(g: &mut crate::game::Game, dx: f64, dz: f64) -> Option<[f64; 2]> { todo!() }

/// (src/game/noshove.ts:92 noShoveKeep)
pub fn no_shove_keep(g: &mut crate::game::Game, gd: GuardId, x0: f64, z0: f64, dt: f64) -> bool { todo!() }

/// (src/game/noshove.ts:135 noShoveSummary)
pub fn no_shove_summary(g: &mut crate::game::Game) -> String { todo!() }
