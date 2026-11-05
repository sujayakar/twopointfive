//! The types of `gpu_device`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/gpu/device.ts:212 pipelineProgress, src/gpu/device.ts:212)
#[derive(Clone, Debug)]
pub struct GpuDeviceModPipelineProgress {
    pub total: f64,
    pub done: f64,
    pub failed: f64,
    pub times: Vec<crate::gpu_device::GpuDeviceModPipelineProgressTimesItem>,
    pub on_change: Option<crate::FnField>,
}
impl GpuDeviceModPipelineProgress {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GpuDeviceModPipelineProgress {
            total: 0.0,
            done: 0.0,
            failed: 0.0,
            times: Vec::new(),
            on_change: None,
        }
    }
}
impl Default for GpuDeviceModPipelineProgress { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/gpu/device.ts:216 GpuDeviceModPipelineProgress.times, src/gpu/device.ts:216)
#[derive(Clone, Debug)]
pub struct GpuDeviceModPipelineProgressTimesItem {
    pub label: String,
    pub ms: f64,
}
impl GpuDeviceModPipelineProgressTimesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GpuDeviceModPipelineProgressTimesItem {
            label: String::new(),
            ms: 0.0,
        }
    }
}
impl Default for GpuDeviceModPipelineProgressTimesItem { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct GpuDeviceMod {
    /// (src/gpu/device.ts:212 pipelineProgress)
    pub pipeline_progress: crate::gpu_device::GpuDeviceModPipelineProgress,
}
impl GpuDeviceMod {
    pub fn blank() -> Self {
        GpuDeviceMod {
            pipeline_progress: crate::gpu_device::GpuDeviceModPipelineProgress { total: 0.0, done: 0.0, failed: 0.0, times: Vec::new(), on_change: None },
        }
    }
}
