#![allow(unused_variables)]
//! Every function and method of `render_wear`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/render/wear.ts:41 buildWearMap)
pub fn build_wear_map(g: &mut crate::game::Game, long_foot: Option<bool>, walked: Option<bool>) -> crate::render_wear::WearMap { todo!() }

/// (src/render/wear.ts:87 standsOn)
pub fn stands_on(g: &mut crate::game::Game, b: BoxId) -> bool { todo!() }

/// (src/render/wear.ts:91 bandByOccupancy)
pub fn band_by_occupancy(g: &mut crate::game::Game, boxes: &[BoxId], w: f64, h: f64, ox: f64, oz: f64) -> Vec<f64> { todo!() }

/// (src/render/wear.ts:139 faceRuns)
pub fn face_runs(g: &mut crate::game::Game, boxes: &[BoxId]) -> Vec<f32> { todo!() }

/// (src/render/wear.ts:177 bandByFaces)
pub fn band_by_faces(g: &mut crate::game::Game, boxes: &[BoxId], w: f64, h: f64, ox: f64, oz: f64) -> Vec<f64> { todo!() }
