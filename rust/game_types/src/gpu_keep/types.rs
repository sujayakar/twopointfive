//! The types of `gpu_keep`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct GpuKeepModKEPT {
    pub lease: Option<crate::gpu_keep::Lease>,
    pub shelled: bool,
}

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug, Default)]
pub struct KeepStats {
    pub asked: f64,
    pub pipelines: f64,
    pub modules: f64,
    pub labels: Vec<String>,
}

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
