//! The types of `anim_characters`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: CharacterRenderer, src/anim/characters.ts:20)
#[derive(Clone, Debug)]
pub struct CharacterRenderer {
    pub vertex_buf: crate::Opaque,
    pub joint_buf: crate::Opaque,
    pub index_buf: crate::Opaque,
    pub index_count: f64,
    pub index_format: crate::anim_characters::CharacterRendererIndexFormat,
    pub joint_mats: crate::Opaque,
    pub char_insts: crate::Opaque,
    pub instance_count: f64,
    pub slot_buf: crate::Opaque,
    pub char_mats: crate::Opaque,
    pub mat_data: Vec<f32>,
    pub mat_dirty: bool,
    pub slots_up: bool,
    pub joints_format: crate::anim_characters::CharacterRendererJointsFormat,
    pub inst_u32: Vec<u32>,
    pub joint_staging: Vec<f32>,
    pub inst_sent: Vec<u32>,
    pub device: crate::Opaque,
    pub ch: RigId,
    pub part_spans: Option<Vec<u32>>,
    pub masks: Vec<u32>,
    pub runs_by_mask: JsMap<crate::Num, Vec<u32>>,
    pub scratch: Vec<f32>,
    pub joint_floats: f64,
}
impl CharacterRenderer {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        CharacterRenderer {
            vertex_buf: crate::Opaque,
            joint_buf: crate::Opaque,
            index_buf: crate::Opaque,
            index_count: 0.0,
            index_format: crate::anim_characters::CharacterRendererIndexFormat::Uint32,
            joint_mats: crate::Opaque,
            char_insts: crate::Opaque,
            instance_count: 0.0,
            slot_buf: crate::Opaque,
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

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CharacterRendererIndexFormat {
    Uint16,
    Uint32,
}
impl CharacterRendererIndexFormat {
    pub const fn as_str(self) -> &'static str {
        match self {
            CharacterRendererIndexFormat::Uint16 => "uint16",
            CharacterRendererIndexFormat::Uint32 => "uint32",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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
impl CharacterRendererJointsFormat {
    pub const fn as_str(self) -> &'static str {
        match self {
            CharacterRendererJointsFormat::Float16 => "float16",
            CharacterRendererJointsFormat::Float16x2 => "float16x2",
            CharacterRendererJointsFormat::Float16x4 => "float16x4",
            CharacterRendererJointsFormat::Float32 => "float32",
            CharacterRendererJointsFormat::Float32x2 => "float32x2",
            CharacterRendererJointsFormat::Float32x3 => "float32x3",
            CharacterRendererJointsFormat::Float32x4 => "float32x4",
            CharacterRendererJointsFormat::Sint16 => "sint16",
            CharacterRendererJointsFormat::Sint16x2 => "sint16x2",
            CharacterRendererJointsFormat::Sint16x4 => "sint16x4",
            CharacterRendererJointsFormat::Sint32 => "sint32",
            CharacterRendererJointsFormat::Sint32x2 => "sint32x2",
            CharacterRendererJointsFormat::Sint32x3 => "sint32x3",
            CharacterRendererJointsFormat::Sint32x4 => "sint32x4",
            CharacterRendererJointsFormat::Sint8 => "sint8",
            CharacterRendererJointsFormat::Sint8x2 => "sint8x2",
            CharacterRendererJointsFormat::Sint8x4 => "sint8x4",
            CharacterRendererJointsFormat::Snorm16 => "snorm16",
            CharacterRendererJointsFormat::Snorm16x2 => "snorm16x2",
            CharacterRendererJointsFormat::Snorm16x4 => "snorm16x4",
            CharacterRendererJointsFormat::Snorm8 => "snorm8",
            CharacterRendererJointsFormat::Snorm8x2 => "snorm8x2",
            CharacterRendererJointsFormat::Snorm8x4 => "snorm8x4",
            CharacterRendererJointsFormat::Uint16 => "uint16",
            CharacterRendererJointsFormat::Uint16x2 => "uint16x2",
            CharacterRendererJointsFormat::Uint16x4 => "uint16x4",
            CharacterRendererJointsFormat::Uint32 => "uint32",
            CharacterRendererJointsFormat::Uint32x2 => "uint32x2",
            CharacterRendererJointsFormat::Uint32x3 => "uint32x3",
            CharacterRendererJointsFormat::Uint32x4 => "uint32x4",
            CharacterRendererJointsFormat::Uint8 => "uint8",
            CharacterRendererJointsFormat::Uint8x2 => "uint8x2",
            CharacterRendererJointsFormat::Uint8x4 => "uint8x4",
            CharacterRendererJointsFormat::Unorm1010102 => "unorm10-10-10-2",
            CharacterRendererJointsFormat::Unorm16 => "unorm16",
            CharacterRendererJointsFormat::Unorm16x2 => "unorm16x2",
            CharacterRendererJointsFormat::Unorm16x4 => "unorm16x4",
            CharacterRendererJointsFormat::Unorm8 => "unorm8",
            CharacterRendererJointsFormat::Unorm8x2 => "unorm8x2",
            CharacterRendererJointsFormat::Unorm8x4 => "unorm8x4",
            CharacterRendererJointsFormat::Unorm8x4Bgra => "unorm8x4-bgra",
        }
    }
}

/// (src/anim/characters.ts:8 JOINT_STRIDE)
pub const JOINT_STRIDE: f64 = 64.0;

/// (src/anim/characters.ts:9 MAX_CHARS)
pub const MAX_CHARS: f64 = 16.0;

/// (src/anim/characters.ts:14 CHAR_INST_BYTES)
pub const CHAR_INST_BYTES: f64 = 96.0;

/// (src/anim/characters.ts:15 INST_F32)
pub const INST_F32: f64 = 24.0;

/// (src/anim/characters.ts:16 PARTS_AT)
pub const PARTS_AT: f64 = 22.0;

/// (src/anim/characters.ts:18 CHAR_MAT_BYTES)
pub const CHAR_MAT_BYTES: f64 = 384.0;
