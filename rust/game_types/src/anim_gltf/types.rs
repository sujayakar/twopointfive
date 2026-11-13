//! The types of `anim_gltf`.
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct GltfNode {
    pub name: String,
    pub parent: f64,
    pub children: Vec<f64>,
    pub t: V3,
    pub r: Quat,
    pub s: V3,
}

#[derive(Clone, Debug, Default)]
pub struct Track {
    pub node: usize,
    pub path: Path,
    pub times: Vec<f32>,
    pub values: Vec<f32>,
    /// the value of the key before, not the line between two
    pub step: bool,
    /// which set of times of its clip it is on (LINES: one of its own)
    pub line: usize,
}
/// the sets of times of one clip that a sample keeps its place on
pub const LINES: usize = 8;

/// what of a node a track says
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Path { #[default] Move, Turn, Scale }
impl Track {
    /// where in a pose's ten columns its values go
    pub fn chan(&self) -> usize { match self.path { Path::Move => 0, Path::Turn => 3, Path::Scale => 7 } }
    /// numbers a key
    pub fn comps(&self) -> usize { if self.path == Path::Turn { 4 } else { 3 } }
}
impl Clip {
    /// A clip of tracks as a file has them: each has a key and a value for every key of it, and stands on its line, the first with the same times, single for single.
    pub fn new(name: String, duration: f64, tracks: Vec<Track>) -> Clip {
        for t in &tracks { assert!(!t.times.is_empty() && t.values.len() == t.times.len() * t.comps(), "the clip \"{name}\" has a track of {} times and {} values on node {}", t.times.len(), t.values.len(), t.node); }
        Clip { name, duration, tracks }.lined()
    }
    fn lined(mut self) -> Clip {
        let mut first: Vec<usize> = Vec::new();
        for k in 0..self.tracks.len() {
            let at = first.iter().position(|&f| { let (a, b) = (&self.tracks[f].times, &self.tracks[k].times); a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.to_bits() == y.to_bits()) });
            self.tracks[k].line = at.unwrap_or_else(|| if first.len() < LINES { first.push(k); first.len() - 1 } else { LINES });
        }
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct Clip {
    pub name: String,
    pub duration: f64,
    pub tracks: Vec<crate::anim_gltf::Track>,
}

#[derive(Clone, Debug, Default)]
pub struct SkinData {
    pub joints: Vec<f64>,
    pub inverse_bind: Vec<f32>,
}

#[derive(Clone, Debug)]
pub struct MeshData {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub weights: Vec<f32>,
    pub joints: crate::anim_gltf::MeshDataJoints,
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

#[derive(Clone, Debug, Default)]
pub struct CastVariant {
    pub name: String,
    pub side: crate::anim_gltf::CastSide,
    pub mask: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default, crate::Words)]
pub enum CastSide {
    #[default] Civilian,
    Operator,
    Defender,
    Responder,
}

#[derive(Clone, Debug, Default)]
pub struct CastInfo {
    pub parts: Vec<String>,
    pub variants: Vec<AnimGltfCastVariantId>,
}

#[derive(Clone, Debug, Default)]
pub struct GltfCharacter {
    pub nodes: Vec<crate::anim_gltf::GltfNode>,
    /// each node's parent (−1: none), and the nodes parents first
    pub parent: Vec<i32>,
    pub order: Vec<u32>,
    /// which of them plays which part
    pub roles: crate::anim_skeleton::roles::Roles,
    pub skin: crate::anim_gltf::SkinData,
    pub mesh: AnimGltfMeshDataId,
    pub clips: JsMap<String, ClipId>,
    pub node_by_name: JsMap<String, f64>,
    pub cast: Option<AnimGltfCastInfoId>,
    /// what its meshes have of packs and a back to lie on (none without a cast)
    pub back: Option<crate::packside::Back>,
}

/// what was read of character files
#[derive(Clone, Debug, Default)]
pub struct Art {
    pub clip: Tab<ClipId, crate::anim_gltf::Clip>,
    pub mesh: Tab<AnimGltfMeshDataId, crate::anim_gltf::MeshData>,
    pub variant: Tab<AnimGltfCastVariantId, crate::anim_gltf::CastVariant>,
    pub cast: Tab<AnimGltfCastInfoId, crate::anim_gltf::CastInfo>,
    pub rig: Tab<RigId, crate::anim_gltf::GltfCharacter>,
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
