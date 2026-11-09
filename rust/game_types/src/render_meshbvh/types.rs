//! The types of `render_meshbvh`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct MeshBvh {
    pub num_tris: f64,
    pub num_nodes: f64,
    pub max_depth: f64,
    pub corners: Vec<u32>,
    pub tri_order: Vec<u32>,
    pub skip: Vec<u32>,
    pub right: Vec<i32>,
    pub tri_start: Vec<u32>,
    pub tri_count: Vec<u32>,
    pub subtree_end: Vec<u32>,
    pub depth: Vec<u16>,
    pub level_nodes: Vec<u32>,
    pub level_off: Vec<u32>,
    pub stats: crate::render_meshbvh::MeshBvhStats,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct MeshBvhStats {
    pub leaves: f64,
    pub avg_leaf: f64,
    pub max_leaf: f64,
    pub median_splits: f64,
}

/// (src/render/meshbvh.ts:15 LEAF_MAX)
pub const LEAF_MAX: f64 = 4.0;

/// (src/render/meshbvh.ts:17 MESH_MAX_CHARS)
pub const MESH_MAX_CHARS: f64 = 8.0;

/// (src/render/meshbvh.ts:18 BINS)
pub const BINS: f64 = 12.0;

/// (src/render/meshbvh.ts:19 MAX_DEPTH)
pub const MAX_DEPTH: f64 = 40.0;

/// (src/render/meshbvh.ts:186 LEAF_PAD)
pub const LEAF_PAD: f64 = 0.0001;
