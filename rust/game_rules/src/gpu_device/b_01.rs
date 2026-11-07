//! Bodies for skeleton stubs of gpu_device that translated code calls and no part had (the stub round).
#![allow(unused_imports, unused_variables)]
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A GPU buffer of at least 16 bytes, its size rounded up to whole words.   (src/gpu/device.ts makeBuffer)
/// BOUNDARY: the host calls device.createBuffer({ size: Math.max(16, (size + 3) & ~3), usage, label }) here at the swap and keeps the buffer.
pub fn make_buffer(device: crate::Opaque, size: f64, usage: f64, label: Option<&str>) -> crate::Opaque {
    crate::Opaque
}
