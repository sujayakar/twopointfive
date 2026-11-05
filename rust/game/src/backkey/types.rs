//! The types of `backkey`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Keys, src/game/backkey.ts:15)
#[derive(Clone, Debug)]
pub struct Keys {
    pub pressed: JsSet<String>,
}
impl Keys {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Keys {
            pressed: JsSet::new(),
        }
    }
}
impl Default for Keys { fn default() -> Self { Self::zeroed() } }
