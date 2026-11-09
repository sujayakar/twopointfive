//! The types of `rustknobs`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct Line {
    pub path: String,
    pub root: JsMap<String, crate::Opaque>,
    pub keys: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct RustknobsModRUSTKNOBS {
    pub lines: Option<Vec<crate::rustknobs::Line>>,
    pub at: f64,
    pub written: f64,
    pub tick: bool,
}

#[derive(Clone, Debug, Default)]
pub struct RustKnobLinesItem {
    pub path: String,
    pub value: Option<f64>,
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
            rust_knobs: crate::rustknobs::RustknobsModRUSTKNOBS { lines: None, at: 0.0, written: 0.0, tick: false },
        }
    }
}
