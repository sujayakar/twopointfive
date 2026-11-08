//! The types of `journal`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: View, src/game/journal.ts:252)
#[derive(Clone, Copy, Debug)]
pub struct View {
    pub client_width: f64,
    pub client_height: f64,
    pub width: f64,
    pub height: f64,
}
impl View {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        View {
            client_width: 0.0,
            client_height: 0.0,
            width: 0.0,
            height: 0.0,
        }
    }
}
impl Default for View { fn default() -> Self { Self::zeroed() } }
