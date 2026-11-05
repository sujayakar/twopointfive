//! The types of `render_mesh_shadows`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: MeshShadows, src/render/meshShadows.ts:109)
#[derive(Clone, Debug)]
pub struct MeshShadows {
    pub bvh: crate::render_meshbvh::MeshBvh,
    pub digest: String,
    pub buf: crate::Opaque,
    pub bytes: f64,
    pub tri_base: f64,
    pub bg: crate::Opaque,
    pub mod_: crate::Opaque,
    pub pl: crate::Opaque,
    pub device: crate::Opaque,
    pub joint_mats: crate::Opaque,
}
impl MeshShadows {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        MeshShadows {
            bvh: crate::render_meshbvh::MeshBvh::zeroed(),
            digest: String::new(),
            buf: crate::Opaque,
            bytes: 0.0,
            tri_base: 0.0,
            bg: crate::Opaque,
            mod_: crate::Opaque,
            pl: crate::Opaque,
            device: crate::Opaque,
            joint_mats: crate::Opaque,
        }
    }
}
impl Default for MeshShadows { fn default() -> Self { Self::blank() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct RenderMeshShadowsMod {
    /// (src/render/meshShadows.ts:27 bvhCache)
    pub bvh_cache: JsMap<AnimGltfMeshDataId, crate::render_meshbvh::MeshBvh>,
}
impl RenderMeshShadowsMod {
    pub fn blank() -> Self {
        RenderMeshShadowsMod {
            bvh_cache: JsMap::new(),
        }
    }
}

/// (src/render/meshShadows.ts:23 NODE_BASE)
pub const NODE_BASE: f64 = 4.0;

/// (src/render/meshShadows.ts:24 LEVEL_HDR)
pub const LEVEL_HDR: f64 = 64.0;

/// (src/render/meshShadows.ts:25 REFIT_WG)
pub const REFIT_WG: f64 = 256.0;
