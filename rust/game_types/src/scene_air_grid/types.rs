//! The types of `scene_air_grid`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

pub const AIR_CELL: f64 = 0.5;

pub fn air_w() -> f64 { crate::m::round((crate::scene_world::world().max[0] - crate::scene_world::world().min[0]) / AIR_CELL) }

pub fn air_h() -> f64 { crate::m::round((crate::scene_world::world().max[2] - crate::scene_world::world().min[2]) / AIR_CELL) }

