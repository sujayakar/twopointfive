//! The types of `scene_ground_field`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct PackGroundFieldF {
    pub mound: Vec<f32>,
    pub w: f64,
    pub h: f64,
    pub slab: f64,
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
