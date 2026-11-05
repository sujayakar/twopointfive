#![allow(unused_variables)]
//! Every function and method of `gpu_device`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/gpu/device.ts:185 makeBuffer)
pub fn make_buffer(device: crate::Opaque, size: f64, usage: f64, label: Option<&str>) -> crate::Opaque { todo!() }

/// (src/gpu/device.ts:188 makeBufferWithData)
pub fn make_buffer_with_data(device: crate::Opaque, data: crate::Opaque, usage: f64, label: Option<&str>) -> crate::Opaque { todo!() }

/// (src/gpu/device.ts:196 writeWordsIfChanged)
pub fn write_words_if_changed(device: crate::Opaque, buf: crate::Opaque, offset: f64, now: &[u32], sent: &mut Vec<u32>, words: Option<f64>) -> bool { todo!() }

/// (src/gpu/device.ts:223 track)
pub fn track(g: &mut crate::game::Game, label: &str, p: crate::Opaque) -> crate::Opaque { todo!() }

/// (src/gpu/device.ts:426 computePipelineAsync)
pub fn compute_pipeline_async(g: &mut crate::game::Game, device: crate::Opaque, desc: crate::Opaque) -> crate::Opaque { todo!() }

/// (src/gpu/device.ts:468 makeShader)
pub fn make_shader(g: &mut crate::game::Game, device: crate::Opaque, source: &str, label: &str) -> crate::Opaque { todo!() }
