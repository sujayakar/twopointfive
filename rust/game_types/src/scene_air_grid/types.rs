//! The types of `scene_air_grid`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (src/scene/airGrid.ts:9 AIR_CELL)
pub const AIR_CELL: f64 = 0.5;

/// (src/scene/airGrid.ts:10 AIR_W: a table nothing writes)
pub fn air_w() -> f64 { crate::m::round((crate::scene_world::world().max[0] - crate::scene_world::world().min[0]) / AIR_CELL) }

/// (src/scene/airGrid.ts:11 AIR_H: a table nothing writes)
pub fn air_h() -> f64 { crate::m::round((crate::scene_world::world().max[2] - crate::scene_world::world().min[2]) / AIR_CELL) }

/// (src/scene/airGrid.ts:13 AIR_ROWS: a table nothing writes)
pub fn air_rows() -> f64 { crate::m::ceil(air_w() * air_h() / 4.0) }
