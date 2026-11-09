//! The types of `render_gbuffer`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct SkinnedDrawable {
    pub vertex_buf: crate::Opaque,
    pub joint_buf: crate::Opaque,
    pub joints_format: crate::render_gbuffer::SkinnedDrawableJointsFormat,
    pub index_buf: crate::Opaque,
    pub index_count: f64,
    pub index_format: crate::render_gbuffer::SkinnedDrawableIndexFormat,
    pub joint_mats: crate::Opaque,
    pub char_insts: crate::Opaque,
    pub slot_buf: crate::Opaque,
    pub char_mats: crate::Opaque,
    pub instance_count: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum SkinnedDrawableJointsFormat {
    #[default] Float16,
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
impl SkinnedDrawableJointsFormat {
    pub const fn as_str(self) -> &'static str {
        match self {
            SkinnedDrawableJointsFormat::Float16 => "float16",
            SkinnedDrawableJointsFormat::Float16x2 => "float16x2",
            SkinnedDrawableJointsFormat::Float16x4 => "float16x4",
            SkinnedDrawableJointsFormat::Float32 => "float32",
            SkinnedDrawableJointsFormat::Float32x2 => "float32x2",
            SkinnedDrawableJointsFormat::Float32x3 => "float32x3",
            SkinnedDrawableJointsFormat::Float32x4 => "float32x4",
            SkinnedDrawableJointsFormat::Sint16 => "sint16",
            SkinnedDrawableJointsFormat::Sint16x2 => "sint16x2",
            SkinnedDrawableJointsFormat::Sint16x4 => "sint16x4",
            SkinnedDrawableJointsFormat::Sint32 => "sint32",
            SkinnedDrawableJointsFormat::Sint32x2 => "sint32x2",
            SkinnedDrawableJointsFormat::Sint32x3 => "sint32x3",
            SkinnedDrawableJointsFormat::Sint32x4 => "sint32x4",
            SkinnedDrawableJointsFormat::Sint8 => "sint8",
            SkinnedDrawableJointsFormat::Sint8x2 => "sint8x2",
            SkinnedDrawableJointsFormat::Sint8x4 => "sint8x4",
            SkinnedDrawableJointsFormat::Snorm16 => "snorm16",
            SkinnedDrawableJointsFormat::Snorm16x2 => "snorm16x2",
            SkinnedDrawableJointsFormat::Snorm16x4 => "snorm16x4",
            SkinnedDrawableJointsFormat::Snorm8 => "snorm8",
            SkinnedDrawableJointsFormat::Snorm8x2 => "snorm8x2",
            SkinnedDrawableJointsFormat::Snorm8x4 => "snorm8x4",
            SkinnedDrawableJointsFormat::Uint16 => "uint16",
            SkinnedDrawableJointsFormat::Uint16x2 => "uint16x2",
            SkinnedDrawableJointsFormat::Uint16x4 => "uint16x4",
            SkinnedDrawableJointsFormat::Uint32 => "uint32",
            SkinnedDrawableJointsFormat::Uint32x2 => "uint32x2",
            SkinnedDrawableJointsFormat::Uint32x3 => "uint32x3",
            SkinnedDrawableJointsFormat::Uint32x4 => "uint32x4",
            SkinnedDrawableJointsFormat::Uint8 => "uint8",
            SkinnedDrawableJointsFormat::Uint8x2 => "uint8x2",
            SkinnedDrawableJointsFormat::Uint8x4 => "uint8x4",
            SkinnedDrawableJointsFormat::Unorm1010102 => "unorm10-10-10-2",
            SkinnedDrawableJointsFormat::Unorm16 => "unorm16",
            SkinnedDrawableJointsFormat::Unorm16x2 => "unorm16x2",
            SkinnedDrawableJointsFormat::Unorm16x4 => "unorm16x4",
            SkinnedDrawableJointsFormat::Unorm8 => "unorm8",
            SkinnedDrawableJointsFormat::Unorm8x2 => "unorm8x2",
            SkinnedDrawableJointsFormat::Unorm8x4 => "unorm8x4",
            SkinnedDrawableJointsFormat::Unorm8x4Bgra => "unorm8x4-bgra",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum SkinnedDrawableIndexFormat {
    #[default] Uint16,
    Uint32,
}
impl SkinnedDrawableIndexFormat {
    pub const fn as_str(self) -> &'static str {
        match self {
            SkinnedDrawableIndexFormat::Uint16 => "uint16",
            SkinnedDrawableIndexFormat::Uint32 => "uint32",
        }
    }
}
