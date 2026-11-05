//! The types of `env`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/game/env.ts:18 BUILT, src/game/env.ts:18)
#[derive(Clone, Copy, Debug)]
pub struct BUILT {
    pub exposure: f64,
    pub tube_gain: f64,
}
impl BUILT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BUILT {
            exposure: 0.0,
            tube_gain: 0.0,
        }
    }
}
impl Default for BUILT { fn default() -> Self { Self::zeroed() } }

/// (src/game/env.ts:18 BUILT: a table nothing writes)
pub fn built() -> crate::env::BUILT { crate::env::BUILT { exposure: 2.0, tube_gain: 475.0 } }
