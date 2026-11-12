//! The types of `irlaser`.
use crate::{Blank, bytes::ToBytes, ids::*, js::*, V3};

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct IrlaserMod {
    pub ir_made: JsSet<AnyBoxId>,
}

pub fn ir_tint() -> V3 { [0.2577, 0.2577, 0.2577] }
