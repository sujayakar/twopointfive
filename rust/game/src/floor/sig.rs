#![allow(unused_variables)]
//! Every function and method of `floor`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/floor.ts:48 floorAt)
pub fn floor_at(p: V3, s: Option<&crate::scene_level_data::SurfacesData>) -> crate::floor::FloorMat { todo!() }

/// (src/game/floor.ts:63 stepMat)
pub fn step_mat(g: &mut crate::game::Game, pos: V3) -> crate::floor::FloorMat { todo!() }

/// (src/game/floor.ts:67 floorOf)
pub fn floor_of(g: &mut crate::game::Game, pos: V3) -> crate::floor::FloorMat { todo!() }

/// (src/game/floor.ts:72 tread)
pub fn tread(g: &mut crate::game::Game, pos: V3, base: f64, is_player: bool) -> f64 { todo!() }
