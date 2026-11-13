//! The types of `entrypoints`.
use crate::{Blank, bytes::ToBytes, ids::*, js::*, V3};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Pod {
    pub pos: V3,
    pub face: f64,
    pub watch: Option<V3>,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct EntryPoints {
    pub been: JsMap<CoverOpeningId, f64>,
    #[is(-1.0)] pub last: f64,
}

pub const EYE: f64 = 1.55;

pub const CHEST: f64 = 1.1;
