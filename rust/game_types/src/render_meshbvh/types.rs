//! The types of `render_meshbvh`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: MeshBvh, src/render/meshbvh.ts:21)
#[derive(Clone, Debug, ToBytes)]
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
impl MeshBvh {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MeshBvh {
            num_tris: 0.0,
            num_nodes: 0.0,
            max_depth: 0.0,
            corners: Vec::new(),
            tri_order: Vec::new(),
            skip: Vec::new(),
            right: Vec::new(),
            tri_start: Vec::new(),
            tri_count: Vec::new(),
            subtree_end: Vec::new(),
            depth: Vec::new(),
            level_nodes: Vec::new(),
            level_off: Vec::new(),
            stats: crate::render_meshbvh::MeshBvhStats::zeroed(),
        }
    }
}
impl Default for MeshBvh { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/meshbvh.ts:35 MeshBvh.stats, src/render/meshbvh.ts:35)
#[derive(Clone, Copy, Debug, ToBytes)]
pub struct MeshBvhStats {
    pub leaves: f64,
    pub avg_leaf: f64,
    pub max_leaf: f64,
    pub median_splits: f64,
}
impl MeshBvhStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MeshBvhStats {
            leaves: 0.0,
            avg_leaf: 0.0,
            max_leaf: 0.0,
            median_splits: 0.0,
        }
    }
}
impl Default for MeshBvhStats { fn default() -> Self { Self::zeroed() } }

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
