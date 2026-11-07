#![allow(unused_variables)]
//! Every function and method of `wet_ground`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/wetGround.ts:46 top)
pub fn top(g: &mut crate::game::Game, b: BoxId) -> f64 { todo!() }

/// (src/game/wetGround.ts:51 soaks)
pub fn soaks(g: &mut crate::game::Game, b: BoxId) -> bool { todo!() }

/// (src/game/wetGround.ts:53 groundSheet)
pub fn ground_sheet(g: &mut crate::game::Game, b: BoxId) -> bool { todo!() }

/// (src/game/wetGround.ts:55 groundFlat)
pub fn ground_flat(g: &mut crate::game::Game, b: BoxId) -> bool { todo!() }

/// (src/game/wetGround.ts:60 wetGroundOf)
pub fn wet_ground_of(g: &mut crate::game::Game, statics: &[BoxId], roofs: &[[f64; 4]], lot: Option<BoxId>, picked: bool) -> Vec<BoxId> { todo!() }

/// (src/game/wetGround.ts:83 standing)
pub fn standing(g: &mut crate::game::Game, statics: &[BoxId], lot: Option<BoxId>) -> f64 { todo!() }

/// (src/game/wetGround.ts:101 WetGround.follow)
pub fn wet_ground_follow(g: &mut crate::game::Game) { todo!() }

/// (src/game/wetGround.ts:113 WetGround.dry)
pub fn wet_ground_dry(g: &mut crate::game::Game) { todo!() }
