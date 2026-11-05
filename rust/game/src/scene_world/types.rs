//! The types of `scene_world`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/scene/world.ts:2 WORLD, src/scene/world.ts:2)
#[derive(Clone, Copy, Debug)]
pub struct WORLD {
    pub min: [f64; 3],
    pub max: [f64; 3],
    pub grid_cell: f64,
    pub grid_w: f64,
    pub grid_h: f64,
    pub grid_y0: f64,
    pub grid_y1: f64,
    pub grid_y_levels: f64,
    pub grid_y_pad: f64,
    pub grid_jitter: f64,
    pub ceiling_y: f64,
    pub max_boxes: f64,
    pub rc_min: [f64; 3],
    pub rc_size: [f64; 3],
}
impl WORLD {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WORLD {
            min: [0.0; 3],
            max: [0.0; 3],
            grid_cell: 0.0,
            grid_w: 0.0,
            grid_h: 0.0,
            grid_y0: 0.0,
            grid_y1: 0.0,
            grid_y_levels: 0.0,
            grid_y_pad: 0.0,
            grid_jitter: 0.0,
            ceiling_y: 0.0,
            max_boxes: 0.0,
            rc_min: [0.0; 3],
            rc_size: [0.0; 3],
        }
    }
}
impl Default for WORLD { fn default() -> Self { Self::zeroed() } }

/// (src/scene/world.ts:2 WORLD: a table nothing writes)
pub fn world() -> crate::scene_world::WORLD { crate::scene_world::WORLD { min: [0.0, -0.25, 0.0], max: [40.0, 8.0, 28.0], grid_cell: 1.0, grid_w: 40.0, grid_h: 28.0, grid_y0: -0.25, grid_y1: 3.35, grid_y_levels: 240.0, grid_y_pad: 0.04, grid_jitter: 0.00005, ceiling_y: 3.0, max_boxes: 4096.0, rc_min: [0.0, 0.0, 0.0], rc_size: [40.0, 3.0, 28.0] } }
