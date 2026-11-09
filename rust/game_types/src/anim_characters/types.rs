//! The types of `anim_characters`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes)]
pub struct CharacterRenderer {
    pub index_count: f64,
    pub index_format: crate::anim_characters::CharacterRendererIndexFormat,
    pub instance_count: f64,
    /// (holds nothing: in no bytes)
    #[bytes(skip)]
    pub char_mats: crate::Opaque,
    pub mat_data: Vec<f32>,
    pub mat_dirty: bool,
    pub slots_up: bool,
    pub joints_format: crate::anim_characters::CharacterRendererJointsFormat,
    pub inst_u32: Vec<u32>,
    pub joint_staging: Vec<f32>,
    pub inst_sent: Vec<u32>,
    /// (holds nothing: in no bytes)
    #[bytes(skip)]
    pub device: crate::Opaque,
    pub ch: RigId,
    pub part_spans: Option<Vec<u32>>,
    pub masks: Vec<u32>,
    pub runs_by_mask: JsMap<crate::Num, Vec<u32>>,
    pub scratch: Vec<f32>,
    pub joint_floats: f64,
}
impl CharacterRenderer {
    /// what a new one holds
    pub fn blank() -> Self {
        CharacterRenderer {
            index_count: 0.0,
            index_format: crate::anim_characters::CharacterRendererIndexFormat::Uint32,
            instance_count: 0.0,
            char_mats: crate::Opaque,
            mat_data: alloc::vec![0.0; 1536],
            mat_dirty: true,
            slots_up: false,
            joints_format: crate::anim_characters::CharacterRendererJointsFormat::Float16,
            inst_u32: alloc::vec![0; 384],
            joint_staging: alloc::vec![0.0; 16384],
            inst_sent: alloc::vec![0; 384],
            device: crate::Opaque,
            ch: RigId(0),
            part_spans: None,
            masks: alloc::vec![0; 16],
            runs_by_mask: JsMap::new(),
            scratch: alloc::vec![0.0; 96],
            joint_floats: 1024.0,
        }
    }
}
impl Default for CharacterRenderer { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum CharacterRendererIndexFormat {
    Uint16,
    Uint32,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum CharacterRendererJointsFormat {
    Float16,
    Float16x2,
    Float16x4,
    Float32,
    Float32x2,
    Float32x3,
    Float32x4,
    Sint16,
    Sint16x2,
    Sint16x4,
    Sint32,
    Sint32x2,
    Sint32x3,
    Sint32x4,
    Sint8,
    Sint8x2,
    Sint8x4,
    Snorm16,
    Snorm16x2,
    Snorm16x4,
    Snorm8,
    Snorm8x2,
    Snorm8x4,
    Uint16,
    Uint16x2,
    Uint16x4,
    Uint32,
    Uint32x2,
    Uint32x3,
    Uint32x4,
    Uint8,
    Uint8x2,
    Uint8x4,
    Unorm1010102,
    Unorm16,
    Unorm16x2,
    Unorm16x4,
    Unorm8,
    Unorm8x2,
    Unorm8x4,
    Unorm8x4Bgra,
}

/// (src/anim/characters.ts:9 MAX_CHARS)
pub const MAX_CHARS: f64 = 16.0;

