//! The types of `gpu_device`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct GpuDeviceModPipelineProgress {
    pub total: f64,
    pub done: f64,
    pub failed: f64,
    pub times: Vec<crate::gpu_device::GpuDeviceModPipelineProgressTimesItem>,
    pub on_change: Option<crate::FnField>,
}

#[derive(Clone, Debug, Default)]
pub struct GpuDeviceModPipelineProgressTimesItem {
    pub label: String,
    pub ms: f64,
}

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
