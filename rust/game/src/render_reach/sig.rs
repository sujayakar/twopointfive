#![allow(unused_variables)]
//! Every function and method of `render_reach`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/render/reach.ts:54 buildWallRaster)
pub fn build_wall_raster(g: &mut crate::game::Game, statics: &[crate::scene_boxes::PlanBox], doors: Option<&[crate::render_reach::ReachDoor]>) -> Vec<u8> { todo!() }

/// (src/render/reach.ts:81 jumpsOf)
pub fn jumps_of(g: &mut crate::game::Game, solid: &[u8]) -> Vec<u8> { todo!() }

/// (src/render/reach.ts:85 jumpsFor)
pub fn jumps_for(g: &mut crate::game::Game, solid: &[u8]) -> Vec<u8> { todo!() }

/// (src/render/reach.ts:147 sightLine)
pub fn sight_line(g: &mut crate::game::Game, solid: &[u8], jump: &[u8], ax: f64, az: f64, bx: f64, bz: f64) -> f64 { todo!() }

/// (src/render/reach.ts:163 lineTouchesBox)
pub fn line_touches_box(ax: f64, az: f64, bx: f64, bz: f64, b: [f64; 4]) -> bool { todo!() }

/// (src/render/reach.ts:177 buildInteriorCells)
pub fn build_interior_cells(g: &mut crate::game::Game, statics: &[BoxId]) -> Vec<u8> { todo!() }

/// (src/render/reach.ts:198 segNearRect)
pub fn seg_near_rect(ax: f64, az: f64, bx: f64, bz: f64, r: [f64; 4], pad: f64) -> bool { todo!() }

/// (src/render/reach.ts:239 reachMaskFor)
pub fn reach_mask_for(g: &mut crate::game::Game, solid: &[u8], pos: V3, dir: V3, radius: Option<f64>, interior: Option<&[u8]>, only: Option<&crate::render_reach::ReachOnly>) -> Vec<u32> { todo!() }

/// (src/render/reach.ts:246 derive)
pub fn derive(g: &mut crate::game::Game, solid: &[u8], pos: V3, dir: V3, radius: f64, interior: Option<&[u8]>, only: Option<&crate::render_reach::ReachOnly>, job: Option<&mut crate::render_reach::ReachJob>) -> Option<Vec<u32>> { todo!() }

/// (src/render/reach.ts:339 reachMaskForDirectionalSteps)
pub fn reach_mask_for_directional_steps(g: &mut crate::game::Game, dir_to_light: V3, interior: &[u8], blocked: &mut dyn FnMut(&mut crate::game::Game, V3, V3, f64) -> bool, only: Option<&crate::render_reach::ReachOnly>, free: Option<&mut dyn FnMut(&mut crate::game::Game, V3) -> Option<V3>>) -> crate::Opaque { todo!() }
