//! The types of `irlaser`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct IrlaserMod {
    pub view_tube: Option<bool>,
    pub ir_made: JsSet<AnyBoxId>,
}

pub fn ir_tint() -> V3 { [0.2577, 0.2577, 0.2577] }
