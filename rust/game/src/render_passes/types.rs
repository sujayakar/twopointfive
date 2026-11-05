//! The types of `render_passes`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: FireRecord, src/render/passes.ts:110)
#[derive(Clone, Copy, Debug)]
pub struct FireRecord {
}
impl FireRecord {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FireRecord {
        }
    }
}
impl Default for FireRecord { fn default() -> Self { Self::zeroed() } }
