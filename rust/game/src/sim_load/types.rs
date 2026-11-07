//! The types of `sim_load`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/sim/load.ts:10 startModule ->, src/sim/load.ts:10)
#[derive(Clone, Debug)]
pub struct StartModule {
    pub exports: JsMap<String, crate::Opaque>,
    pub ms: f64,
}
impl StartModule {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StartModule {
            exports: JsMap::new(),
            ms: 0.0,
        }
    }
}
impl Default for StartModule { fn default() -> Self { Self::zeroed() } }
