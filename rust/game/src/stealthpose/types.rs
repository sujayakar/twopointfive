//! The types of `stealthpose`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: StealthFeed, src/game/stealthpose.ts:44)
#[derive(Clone, Copy, Debug)]
pub struct StealthFeed {
    pub free: bool,
    pub cable: bool,
}
impl StealthFeed {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StealthFeed {
            free: false,
            cable: false,
        }
    }
}
impl Default for StealthFeed { fn default() -> Self { Self::zeroed() } }
