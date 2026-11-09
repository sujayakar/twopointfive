//! The types of `irlaser`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct IrlaserMod {
    /// (src/game/irlaser.ts:73 viewTube)
    pub view_tube: Option<bool>,
    /// (src/game/irlaser.ts:77 IR_MADE)
    pub ir_made: JsSet<AnyBoxId>,
}

/// (src/game/irlaser.ts:65 IR_TINT: a table nothing writes)
pub fn ir_tint() -> V3 { [0.2577, 0.2577, 0.2577] }
