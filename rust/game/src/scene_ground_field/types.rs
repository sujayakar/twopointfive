//! The types of `scene_ground_field`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/scene/groundField.ts:16 groundRayConfig, src/scene/groundField.ts:16)
#[derive(Clone, Copy, Debug)]
pub struct SceneGroundFieldModGroundRayConfig {
    pub on: bool,
    pub roll: f64,
}
impl SceneGroundFieldModGroundRayConfig {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SceneGroundFieldModGroundRayConfig {
            on: false,
            roll: 0.0,
        }
    }
}
impl Default for SceneGroundFieldModGroundRayConfig { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/groundField.ts:27 packGroundField(f), src/scene/groundField.ts:27)
#[derive(Clone, Debug)]
pub struct PackGroundFieldF {
    pub mound: Vec<f32>,
    pub w: f64,
    pub h: f64,
    pub slab: f64,
}
impl PackGroundFieldF {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PackGroundFieldF {
            mound: Vec::new(),
            w: 0.0,
            h: 0.0,
            slab: 0.0,
        }
    }
}
impl Default for PackGroundFieldF { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SceneGroundFieldMod {
    /// (src/scene/groundField.ts:16 groundRayConfig)
    pub ground_ray_config: crate::scene_ground_field::SceneGroundFieldModGroundRayConfig,
}
impl SceneGroundFieldMod {
    pub fn blank() -> Self {
        SceneGroundFieldMod {
            ground_ray_config: crate::scene_ground_field::SceneGroundFieldModGroundRayConfig { on: false, roll: 0.0 },
        }
    }
}

/// (src/scene/groundField.ts:17 GH_RES)
pub const GH_RES: f64 = 10.0;

/// (src/scene/groundField.ts:18 GH_W: a table nothing writes)
pub fn gh_w() -> f64 { 400.0 }

/// (src/scene/groundField.ts:19 GH_H: a table nothing writes)
pub fn gh_h() -> f64 { 280.0 }

/// (src/scene/groundField.ts:21 GH_HEAD_ROWS: a table nothing writes)
pub fn gh_head_rows() -> f64 { 281.0 }

/// (src/scene/groundField.ts:22 GH_ROWS: a table nothing writes)
pub fn gh_rows() -> f64 { 28281.0 }

/// (src/scene/groundField.ts:24 GH_UNDER)
pub const GH_UNDER: f64 = -0.012;
