//! The types of `render_probe`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: IrradianceProbe, src/render/probe.ts:34)
#[derive(Clone, Copy, Debug)]
pub struct IrradianceProbe {
    pub layout: crate::Opaque,
    pub mod_: crate::Opaque,
    pub pipe_layout: crate::Opaque,
    pub query_buf: crate::Opaque,
    pub result_buf: crate::Opaque,
    pub param_buf: crate::Opaque,
    pub device: crate::Opaque,
}
impl IrradianceProbe {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        IrradianceProbe {
            layout: crate::Opaque,
            mod_: crate::Opaque,
            pipe_layout: crate::Opaque,
            query_buf: crate::Opaque,
            result_buf: crate::Opaque,
            param_buf: crate::Opaque,
            device: crate::Opaque,
        }
    }
}
impl Default for IrradianceProbe { fn default() -> Self { Self::blank() } }

/// (src/render/probe.ts:13 MAX_QUERIES)
pub const MAX_QUERIES: f64 = 64.0;
