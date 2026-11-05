//! The types of `gpu_keep`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/gpu/keep.ts:260 KEPT, src/gpu/keep.ts:260)
#[derive(Clone, Copy, Debug)]
pub struct GpuKeepModKEPT {
    pub shelled: bool,
}
impl GpuKeepModKEPT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GpuKeepModKEPT {
            shelled: false,
        }
    }
}
impl Default for GpuKeepModKEPT { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct GpuKeepMod {
    /// (src/gpu/keep.ts:260 KEPT)
    pub kept: crate::gpu_keep::GpuKeepModKEPT,
}
impl GpuKeepMod {
    pub fn blank() -> Self {
        GpuKeepMod {
            kept: crate::gpu_keep::GpuKeepModKEPT { shelled: false },
        }
    }
}
