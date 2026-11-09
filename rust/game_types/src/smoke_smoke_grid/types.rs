//! The types of `smoke_smoke_grid`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct SmokeLevel {
    pub voxel: f64,
    pub dims: [f64; 3],
    pub page_base: f64,
    pub tex_y: f64,
    pub tile: f64,
    pub tile_base: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SmokeGrid {
    pub cap: f64,
    pub levels: [crate::smoke_smoke_grid::SmokeLevel; 2],
    pub n0: f64,
    pub n_page: f64,
    pub n_tiles: f64,
    pub page_tex: [f64; 3],
    pub macro_y: f64,
    pub macro_dims: [f64; 3],
    pub atlas_blocks: [f64; 3],
    pub atlas_tex: [f64; 3],
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SmokeSmokeGridModSmokeGridConfig {
    pub cap: f64,
    pub voxel: [f64; 2],
    pub height: [f64; 2],
    pub tile: [f64; 2],
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct SmokeSmokeGridMod {
    /// (src/smoke/smokeGrid.ts:6 smokeGridConfig)
    pub smoke_grid_config: crate::smoke_smoke_grid::SmokeSmokeGridModSmokeGridConfig,
}
impl SmokeSmokeGridMod {
    pub fn blank() -> Self {
        SmokeSmokeGridMod {
            smoke_grid_config: crate::smoke_smoke_grid::SmokeSmokeGridModSmokeGridConfig { cap: 4096.0, voxel: [0.0625, 0.022], height: [crate::scene_world::world().max[1], 4.2], tile: [8.0, 10.0] },
        }
    }
}
