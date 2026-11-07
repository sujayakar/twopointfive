//! The types of `render_kernels`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/render/kernels.ts:32 COMPACT, src/render/kernels.ts:32)
#[derive(Clone, Copy, Debug)]
pub struct COMPACT {
    pub on: bool,
}
impl COMPACT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        COMPACT {
            on: false,
        }
    }
}
impl Default for COMPACT { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/kernels.ts:40 KERNELS, src/render/kernels.ts:36)
#[derive(Clone, Debug)]
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
impl KERNELS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KERNELS {
            fgather: String::new(),
            gather_common: String::new(),
            reflect_p5: String::new(),
            volumetrics: String::new(),
            composite: String::new(),
            particles: String::new(),
            direct: String::new(),
            sharp_direct: String::new(),
            restir: String::new(),
            sdf_rig: String::new(),
        }
    }
}
impl Default for KERNELS { fn default() -> Self { Self::zeroed() } }

/// (src/render/kernels.ts:32 COMPACT: a table nothing writes)
pub fn compact() -> crate::render_kernels::COMPACT { todo!() }

/// (src/render/kernels.ts:40 KERNELS: a table nothing writes)
pub fn kernels() -> crate::render_kernels::KERNELS { todo!() }
