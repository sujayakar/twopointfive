//! The types of `cast`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct CastMod {
    /// (src/game/cast.ts:26 worn)
    pub worn: JsMap<CharId, AnimGltfCastVariantId>,
    /// (src/game/cast.ts:32 fieldBits)
    pub field_bits: JsMap<crate::ObjKey, f64>,
    /// (src/game/cast.ts:65 hasNvg)
    pub has_nvg: JsMap<AnimGltfCastVariantId, bool>,
}
impl CastMod {
    pub fn blank() -> Self {
        CastMod {
            worn: JsMap::new(),
            field_bits: JsMap::new(),
            has_nvg: JsMap::new(),
        }
    }
}

/// (src/game/cast.ts:31 FIELD_PARTS: a table nothing writes)
pub fn field_parts() -> Vec<String> { alloc::vec![String::from("hat_straw"), String::from("hat_boonie"), String::from("bandana"), String::from("bandolier"), String::from("tank_top")] }
