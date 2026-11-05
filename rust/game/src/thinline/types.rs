//! The types of `thinline`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Thin, src/game/thinline.ts:11)
#[derive(Clone, Copy, Debug)]
pub struct Thin {
    pub hw0: f64,
    pub k: f64,
    pub e: [f64; 3],
}
impl Thin {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Thin {
            hw0: 0.0,
            k: 0.0,
            e: [0.0; 3],
        }
    }
}
impl Default for Thin { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct ThinlineMod {
    /// (src/game/thinline.ts:12 THIN)
    pub thin: JsMap<BoxId, crate::thinline::Thin>,
    /// (src/game/thinline.ts:14 AIR)
    pub air: JsMap<BoxId, Option<BoxId>>,
}
impl ThinlineMod {
    pub fn blank() -> Self {
        ThinlineMod {
            thin: JsMap::new(),
            air: JsMap::new(),
        }
    }
}
