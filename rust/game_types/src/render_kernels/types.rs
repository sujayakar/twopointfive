//! The types of `render_kernels`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct COMPACT {
    pub on: bool,
}

#[derive(Clone, Debug, Default)]
pub struct KERNELS {
    pub fgather: String,
    pub gather_common: String,
    pub reflect_p5: String,
    pub volumetrics: String,
    pub composite: String,
    pub particles: String,
    pub direct: String,
    pub sharp_direct: String,
    pub restir: String,
    pub sdf_rig: String,
}

/// (src/render/kernels.ts:32 COMPACT: a table nothing writes)
pub fn compact() -> crate::render_kernels::COMPACT { todo!() }

/// (src/render/kernels.ts:40 KERNELS: a table nothing writes)
pub fn kernels() -> crate::render_kernels::KERNELS { todo!() }
