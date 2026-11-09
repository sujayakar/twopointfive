//! The types of `rustknobs`.
#![allow(unused_imports)]
use crate::{Blank, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct RustknobsModRUSTKNOBS {
    pub written: f64,
    pub tick: bool,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, Blank)]
pub struct RustknobsMod {
    /// (src/game/rustknobs.ts:18 RUST_KNOBS)
    #[is(crate::rustknobs::RustknobsModRUSTKNOBS { written: 0.0, tick: false })] pub rust_knobs: crate::rustknobs::RustknobsModRUSTKNOBS,
}
