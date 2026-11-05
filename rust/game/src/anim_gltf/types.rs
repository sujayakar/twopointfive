//! The types of `anim_gltf`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: GltfNode, src/anim/gltf.ts:5)
#[derive(Clone, Debug)]
pub struct GltfNode {
    pub name: String,
    pub parent: f64,
    pub children: Vec<f64>,
    pub t: V3,
    pub r: Quat,
    pub s: V3,
}
impl GltfNode {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GltfNode {
            name: String::new(),
            parent: 0.0,
            children: Vec::new(),
            t: [0.0; 3],
            r: [0.0; 4],
            s: [0.0; 3],
        }
    }
}
impl Default for GltfNode { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Track, src/anim/gltf.ts:6)
#[derive(Clone, Debug)]
pub struct Track {
    pub node: f64,
    pub path: f64,
    pub times: Vec<f32>,
    pub values: Vec<f32>,
    pub step: bool,
}
impl Track {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Track {
            node: 0.0,
            path: 0.0,
            times: Vec::new(),
            values: Vec::new(),
            step: false,
        }
    }
}
impl Default for Track { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Clip, src/anim/gltf.ts:10)
#[derive(Clone, Debug)]
pub struct Clip {
    pub name: String,
    pub duration: f64,
    pub tracks: Vec<crate::anim_gltf::Track>,
}
impl Clip {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Clip {
            name: String::new(),
            duration: 0.0,
            tracks: Vec::new(),
        }
    }
}
impl Default for Clip { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SkinData, src/anim/gltf.ts:11)
#[derive(Clone, Debug)]
pub struct SkinData {
    pub joints: Vec<f64>,
    pub inverse_bind: Vec<f32>,
}
impl SkinData {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SkinData {
            joints: Vec::new(),
            inverse_bind: Vec::new(),
        }
    }
}
impl Default for SkinData { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: MeshData, src/anim/gltf.ts:12)
#[derive(Clone, Debug)]
pub struct MeshData {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub weights: Vec<f32>,
    pub joints: crate::anim_gltf::MeshDataJoints,
    pub mat_sel: Vec<f32>,
    pub indices: Vec<u32>,
    pub vertex_count: f64,
    pub joints_are_u16: bool,
    pub bbox_min: V3,
    pub bbox_max: V3,
    pub part: Option<Vec<u8>>,
    pub mat_index: Option<Vec<u8>>,
    pub material_names: Option<Vec<String>>,
}
impl MeshData {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MeshData {
            positions: Vec::new(),
            normals: Vec::new(),
            weights: Vec::new(),
            joints: crate::anim_gltf::MeshDataJoints::Uint16Array(Vec::new()),
            mat_sel: Vec::new(),
            indices: Vec::new(),
            vertex_count: 0.0,
            joints_are_u16: false,
            bbox_min: [0.0; 3],
            bbox_max: [0.0; 3],
            part: None,
            mat_index: None,
            material_names: None,
        }
    }
}
impl Default for MeshData { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum MeshDataJoints {
    Uint16Array(Vec<u16>),
    Uint8Array(Vec<u8>),
}

/// (TypeScript: CastVariant, src/anim/gltf.ts:23)
#[derive(Clone, Debug)]
pub struct CastVariant {
    pub name: String,
    pub side: crate::anim_gltf::CastSide,
    pub parts: Vec<String>,
    pub mask: f64,
    pub tris: f64,
}
impl CastVariant {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CastVariant {
            name: String::new(),
            side: crate::anim_gltf::CastSide::Civilian,
            parts: Vec::new(),
            mask: 0.0,
            tris: 0.0,
        }
    }
}
impl Default for CastVariant { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CastSide {
    Civilian,
    Operator,
    Defender,
    Responder,
}
impl CastSide {
    pub const fn as_str(self) -> &'static str {
        match self {
            CastSide::Civilian => "civilian",
            CastSide::Operator => "operator",
            CastSide::Defender => "defender",
            CastSide::Responder => "responder",
        }
    }
}

/// (TypeScript: CastInfo, src/anim/gltf.ts:25)
#[derive(Clone, Debug)]
pub struct CastInfo {
    pub parts: Vec<String>,
    pub part_tris: Vec<f64>,
    pub variants: Vec<AnimGltfCastVariantId>,
}
impl CastInfo {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CastInfo {
            parts: Vec::new(),
            part_tris: Vec::new(),
            variants: Vec::new(),
        }
    }
}
impl Default for CastInfo { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: GltfCharacter, src/anim/gltf.ts:26)
#[derive(Clone, Debug)]
pub struct GltfCharacter {
    pub nodes: Vec<crate::anim_gltf::GltfNode>,
    pub order: Vec<f64>,
    pub roots: Vec<f64>,
    pub skin: crate::anim_gltf::SkinData,
    pub mesh: AnimGltfMeshDataId,
    pub clips: JsMap<String, ClipId>,
    pub node_by_name: JsMap<String, f64>,
    pub cast: Option<AnimGltfCastInfoId>,
}
impl GltfCharacter {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GltfCharacter {
            nodes: Vec::new(),
            order: Vec::new(),
            roots: Vec::new(),
            skin: crate::anim_gltf::SkinData::zeroed(),
            mesh: AnimGltfMeshDataId(0),
            clips: JsMap::new(),
            node_by_name: JsMap::new(),
            cast: None,
        }
    }
}
impl Default for GltfCharacter { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/anim/gltf.ts:33 nvgBits, src/anim/gltf.ts:33)
#[derive(Clone, Copy, Debug)]
pub struct AnimGltfModNvgBitsValue {
    pub up: f64,
    pub down: f64,
}
impl AnimGltfModNvgBitsValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AnimGltfModNvgBitsValue {
            up: 0.0,
            down: 0.0,
        }
    }
}
impl Default for AnimGltfModNvgBitsValue { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct AnimGltfMod {
    /// (src/anim/gltf.ts:33 nvgBits)
    pub nvg_bits: JsMap<AnimGltfCastInfoId, crate::anim_gltf::AnimGltfModNvgBitsValue>,
    /// (src/anim/gltf.ts:44 suitBits)
    pub suit_bits: JsMap<AnimGltfCastInfoId, f64>,
}
impl AnimGltfMod {
    pub fn blank() -> Self {
        AnimGltfMod {
            nvg_bits: JsMap::new(),
            suit_bits: JsMap::new(),
        }
    }
}
