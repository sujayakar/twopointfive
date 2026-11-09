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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum SkinnedDrawableIndexFormat {
    #[default] Uint16,
    Uint32,
}
