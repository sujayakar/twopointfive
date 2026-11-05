//! The types of `render_gbuffer`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: SkinnedDrawable, src/render/gbuffer.ts:92)
#[derive(Clone, Copy, Debug)]
pub struct SkinnedDrawable {
}
impl SkinnedDrawable {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SkinnedDrawable {
        }
    }
}
impl Default for SkinnedDrawable { fn default() -> Self { Self::zeroed() } }
