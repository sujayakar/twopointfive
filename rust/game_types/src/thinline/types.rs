//! The types of `thinline`.
use crate::{Blank, bytes::ToBytes, ids::*, js::*};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Thin {
    pub hw0: f64,
    pub k: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct ThinlineMod {
    pub thin: JsMap<AnyBoxId, crate::thinline::Thin>,
    pub air: JsMap<AnyBoxId, Option<AnyBoxId>>,
}
