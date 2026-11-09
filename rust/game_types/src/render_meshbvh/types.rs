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

