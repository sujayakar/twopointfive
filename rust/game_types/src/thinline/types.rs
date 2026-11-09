//! The types of `thinline`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Thin {
    pub hw0: f64,
    pub k: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct ThinlineMod {
    pub thin: JsMap<AnyBoxId, crate::thinline::Thin>,
    pub air: JsMap<AnyBoxId, Option<AnyBoxId>>,
}
impl ThinlineMod {
    pub fn blank() -> Self {
        ThinlineMod {
            thin: JsMap::new(),
            air: JsMap::new(),
        }
    }
}
