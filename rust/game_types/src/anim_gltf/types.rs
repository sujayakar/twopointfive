//! The types of `anim_gltf`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct GltfNode {
    pub name: String,
    pub parent: f64,
    pub children: Vec<f64>,
    pub t: V3,
    pub r: Quat,
    pub s: V3,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Track {
    pub node: f64,
    pub path: f64,
    pub times: Vec<f32>,
    pub values: Vec<f32>,
    pub step: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Clip {
    pub name: String,
    pub duration: f64,
    pub tracks: Vec<crate::anim_gltf::Track>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SkinData {
    pub joints: Vec<f64>,
    pub inverse_bind: Vec<f32>,
}

#[derive(Clone, Debug, ToBytes)]
pub struct MeshData {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub weights: Vec<f32>,
    pub joints: crate::anim_gltf::MeshDataJoints,
    pub mat_sel: Vec<f32>,
    pub indices: Vec<u32>,
    pub vertex_count: f64,
    pub bbox_min: V3,
    pub bbox_max: V3,
    pub part: Option<Vec<u8>>,
    pub mat_index: Option<Vec<u8>>,
}
impl Default for MeshData {
    fn default() -> Self {
        MeshData {
            positions: Vec::new(),
            normals: Vec::new(),
            weights: Vec::new(),
            joints: crate::anim_gltf::MeshDataJoints::Uint16Array(Vec::new()),
            mat_sel: Vec::new(),
            indices: Vec::new(),
            vertex_count: 0.0,
            bbox_min: [0.0; 3],
            bbox_max: [0.0; 3],
            part: None,
            mat_index: None,
        }
    }
}

#[derive(Clone, Debug, ToBytes)]
pub enum MeshDataJoints {
    Uint16Array(Vec<u16>),
    Uint8Array(Vec<u8>),
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CastVariant {
    pub name: String,
    pub side: crate::anim_gltf::CastSide,
    pub mask: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default)]
pub enum CastSide {
    #[default] Civilian,
    Operator,
    Defender,
    Responder,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CastInfo {
    pub parts: Vec<String>,
    pub variants: Vec<AnimGltfCastVariantId>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct GltfCharacter {
    pub nodes: Vec<crate::anim_gltf::GltfNode>,
    pub order: Vec<f64>,
    pub skin: crate::anim_gltf::SkinData,
    pub mesh: AnimGltfMeshDataId,
    pub clips: JsMap<String, ClipId>,
    pub node_by_name: JsMap<String, f64>,
    pub cast: Option<AnimGltfCastInfoId>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AnimGltfModNvgBitsValue {
    pub up: f64,
    pub down: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct AnimGltfMod {
    pub nvg_bits: JsMap<AnimGltfCastInfoId, crate::anim_gltf::AnimGltfModNvgBitsValue>,
    pub suit_bits: JsMap<AnimGltfCastInfoId, f64>,
}
