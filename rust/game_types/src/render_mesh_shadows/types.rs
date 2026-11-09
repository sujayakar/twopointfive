//! The types of `render_mesh_shadows`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes)]
pub struct MeshShadows {
    pub bvh: crate::render_meshbvh::MeshBvh,
    pub digest: String,
    pub bytes: f64,
    pub tri_base: f64,
    /// (holds nothing: in no bytes)
    #[bytes(skip)]
    pub pl: crate::Opaque,
    /// (holds nothing: in no bytes)
    #[bytes(skip)]
    pub device: crate::Opaque,
}
impl MeshShadows {
    /// what a new one holds
    pub fn blank() -> Self {
        MeshShadows {
            bvh: crate::render_meshbvh::MeshBvh::default(),
            digest: String::new(),
            bytes: 0.0,
            tri_base: 0.0,
            pl: crate::Opaque,
            device: crate::Opaque,
        }
    }
}
impl Default for MeshShadows { fn default() -> Self { Self::blank() } }

