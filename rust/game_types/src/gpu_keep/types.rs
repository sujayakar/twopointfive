//! The types of `gpu_keep`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/gpu/keep.ts:260 KEPT, src/gpu/keep.ts:260)
#[derive(Clone, Debug)]
pub struct GpuKeepModKEPT {
    pub lease: Option<crate::gpu_keep::Lease>,
    pub shelled: bool,
}
impl GpuKeepModKEPT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GpuKeepModKEPT {
            lease: None,
            shelled: false,
        }
    }
}
impl Default for GpuKeepModKEPT { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Lease, src/gpu/keep.ts:52)
#[derive(Clone, Debug)]
pub struct Lease {
    pub adapter: crate::Opaque,
    pub device: crate::Opaque,
    pub asks: Vec<crate::Opaque>,
    pub n: f64,
    pub fresh: bool,
    pub waited: Option<f64>,
    pub stats: crate::gpu_keep::KeepStats,
    pub shell_ms: f64,
    pub shell_nav: String,
    pub frozen: Option<crate::gpu_keep::KeepStats>,
    pub alive: bool,
}
impl Lease {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Lease {
            adapter: crate::Opaque,
            device: crate::Opaque,
            asks: Vec::new(),
            n: 0.0,
            fresh: false,
            waited: None,
            stats: crate::gpu_keep::KeepStats::zeroed(),
            shell_ms: 0.0,
            shell_nav: String::new(),
            frozen: None,
            alive: false,
        }
    }
}
impl Default for Lease { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: KeepStats, src/gpu/keep.ts:50)
#[derive(Clone, Debug)]
pub struct KeepStats {
    pub asked: f64,
    pub pipelines: f64,
    pub modules: f64,
    pub labels: Vec<String>,
}
impl KeepStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KeepStats {
            asked: 0.0,
            pipelines: 0.0,
            modules: 0.0,
            labels: Vec::new(),
        }
    }
}
impl Default for KeepStats { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct GpuKeepMod {
    /// (src/gpu/keep.ts:260 KEPT)
    pub kept: crate::gpu_keep::GpuKeepModKEPT,
}
impl GpuKeepMod {
    pub fn blank() -> Self {
        GpuKeepMod {
            kept: crate::gpu_keep::GpuKeepModKEPT { lease: None, shelled: false },
        }
    }
}
