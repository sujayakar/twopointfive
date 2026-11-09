//! The types of `irlaser`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct IrlaserMod {
    pub view_tube: Option<bool>,
    pub ir_made: JsSet<AnyBoxId>,
}
impl IrlaserMod {
    pub fn blank() -> Self {
        IrlaserMod {
            view_tube: None,
            ir_made: JsSet::new(),
        }
    }
}

pub fn ir_tint() -> V3 { [0.2577, 0.2577, 0.2577] }
