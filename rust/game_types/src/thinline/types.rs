//! The types of `thinline`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Thin {
    pub hw0: f64,
    pub k: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct ThinlineMod {
    /// (src/game/thinline.ts:12 THIN)
    pub thin: JsMap<AnyBoxId, crate::thinline::Thin>,
    /// (src/game/thinline.ts:14 AIR)
    pub air: JsMap<AnyBoxId, Option<AnyBoxId>>,
}
