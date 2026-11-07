//! The types of `render_place_field`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct RenderPlaceFieldMod {
    /// (src/render/placeField.ts:27 NONE)
    pub none: JsMap<crate::ObjKey, crate::Opaque>,
}
impl RenderPlaceFieldMod {
    pub fn blank() -> Self {
        RenderPlaceFieldMod {
            none: JsMap::new(),
        }
    }
}

/// (src/render/placeField.ts:22 PLACE_WORDS)
pub const PLACE_WORDS: f64 = 4.0;

/// (src/render/placeField.ts:24 PLACE_NEAR: a table nothing writes)
pub fn place_near() -> f64 { todo!() }

/// (src/render/placeField.ts:26 PLACE_BYTES: a table nothing writes)
pub fn place_bytes() -> f64 { todo!() }
