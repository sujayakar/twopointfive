//! The types of `rustknobs`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct RustknobsModRUSTKNOBS {
    pub written: f64,
    pub tick: bool,
}

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
