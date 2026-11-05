//! The types of `smoke_smoke_grid`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: SmokeLevel, src/smoke/smokeGrid.ts:17)
#[derive(Clone, Copy, Debug)]
pub struct SmokeLevel {
    pub voxel: f64,
    pub dims: [f64; 3],
    pub tex_y: f64,
}
impl SmokeLevel {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SmokeLevel {
            voxel: 0.0,
            dims: [0.0; 3],
            tex_y: 0.0,
        }
    }
}
impl Default for SmokeLevel { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SmokeGrid, src/smoke/smokeGrid.ts:18)
#[derive(Clone, Copy, Debug)]
pub struct SmokeGrid {
    pub levels: [crate::smoke_smoke_grid::SmokeLevel; 2],
    pub macro_y: f64,
    pub atlas_blocks: [f64; 3],
    pub atlas_tex: [f64; 3],
}
impl SmokeGrid {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SmokeGrid {
            levels: [crate::smoke_smoke_grid::SmokeLevel::zeroed(); 2],
            macro_y: 0.0,
            atlas_blocks: [0.0; 3],
            atlas_tex: [0.0; 3],
        }
    }
}
impl Default for SmokeGrid { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/smoke/smokeGrid.ts:6 smokeGridConfig, src/smoke/smokeGrid.ts:6)
#[derive(Clone, Copy, Debug)]
pub struct SmokeSmokeGridModSmokeGridConfig {
    pub cap: f64,
    pub voxel: [f64; 2],
    pub height: [f64; 2],
    pub tile: [f64; 2],
}
impl SmokeSmokeGridModSmokeGridConfig {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SmokeSmokeGridModSmokeGridConfig {
            cap: 0.0,
            voxel: [0.0; 2],
            height: [0.0; 2],
            tile: [0.0; 2],
        }
    }
}
impl Default for SmokeSmokeGridModSmokeGridConfig { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SmokeSmokeGridMod {
    /// (src/smoke/smokeGrid.ts:6 smokeGridConfig)
    pub smoke_grid_config: crate::smoke_smoke_grid::SmokeSmokeGridModSmokeGridConfig,
    /// (src/smoke/smokeGrid.ts:18)
    pub smoke_grid: crate::smoke_smoke_grid::SmokeGrid,
}
impl SmokeSmokeGridMod {
    pub fn blank() -> Self {
        SmokeSmokeGridMod {
            smoke_grid_config: crate::smoke_smoke_grid::SmokeSmokeGridModSmokeGridConfig { cap: 4096.0, voxel: [0.055, 0.022], height: [crate::scene_world::world().max[1], 4.2], tile: [8.0, 10.0] },
            smoke_grid: crate::smoke_smoke_grid::SmokeGrid::zeroed(),
        }
    }
}
