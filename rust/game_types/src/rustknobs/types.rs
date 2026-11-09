//! The types of `rustknobs`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/game/rustknobs.ts:18 RUST_KNOBS, src/game/rustknobs.ts:18)
#[derive(Clone, Debug)]
pub struct RustknobsModRUSTKNOBS {
    pub written: f64,
    pub tick: bool,
}
impl RustknobsModRUSTKNOBS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RustknobsModRUSTKNOBS {
            written: 0.0,
            tick: false,
        }
    }
}
impl Default for RustknobsModRUSTKNOBS { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct RustknobsMod {
    /// (src/game/rustknobs.ts:18 RUST_KNOBS)
    pub rust_knobs: crate::rustknobs::RustknobsModRUSTKNOBS,
}
impl RustknobsMod {
    pub fn blank() -> Self {
        RustknobsMod {
            rust_knobs: crate::rustknobs::RustknobsModRUSTKNOBS { written: 0.0, tick: false },
        }
    }
}
