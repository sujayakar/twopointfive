//! The types of `scene_place_plan`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (src/scene/placePlan.ts:15 PLACE_RES)
pub const PLACE_RES: f64 = 2.5;

/// (src/scene/placePlan.ts:16 PLACE_W: a table nothing writes)
pub fn place_w() -> f64 { 100.0 }

/// (src/scene/placePlan.ts:16 PLACE_H: a table nothing writes)
pub fn place_h() -> f64 { 70.0 }
