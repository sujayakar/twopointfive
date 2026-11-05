//! The types of `rustknobs`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Line, src/game/rustknobs.ts:14)
#[derive(Clone, Debug)]
pub struct Line {
    pub path: String,
    pub root: JsMap<String, crate::Opaque>,
    pub keys: Vec<String>,
}
impl Line {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Line {
            path: String::new(),
            root: JsMap::new(),
            keys: Vec::new(),
        }
    }
}
impl Default for Line { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rustknobs.ts:17 RUST_KNOBS, src/game/rustknobs.ts:17)
#[derive(Clone, Debug)]
pub struct RustknobsModRUSTKNOBS {
    pub lines: Option<Vec<crate::rustknobs::Line>>,
    pub at: f64,
    pub written: f64,
    pub tick: bool,
}
impl RustknobsModRUSTKNOBS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RustknobsModRUSTKNOBS {
            lines: None,
            at: 0.0,
            written: 0.0,
            tick: false,
        }
    }
}
impl Default for RustknobsModRUSTKNOBS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rustknobs.ts:28 rustKnobLines ->, src/game/rustknobs.ts:28)
#[derive(Clone, Debug)]
pub struct RustKnobLinesItem {
    pub path: String,
    pub value: Option<f64>,
}
impl RustKnobLinesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RustKnobLinesItem {
            path: String::new(),
            value: None,
        }
    }
}
impl Default for RustKnobLinesItem { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct RustknobsMod {
    /// (src/game/rustknobs.ts:17 RUST_KNOBS)
    pub rust_knobs: crate::rustknobs::RustknobsModRUSTKNOBS,
}
impl RustknobsMod {
    pub fn blank() -> Self {
        RustknobsMod {
            rust_knobs: crate::rustknobs::RustknobsModRUSTKNOBS { lines: None, at: 0.0, written: 0.0, tick: false },
        }
    }
}
