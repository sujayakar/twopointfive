//! The types of `roomread`.
use crate::{ids::*, js::*, V3};
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, Default)]
pub struct RoomWay {
    pub o: CoverOpeningId,
    pub in_n: [f64; 2],
    pub beyond: V3,
    pub far_known: bool,
    pub shut: bool,
    pub locked: bool,
    pub dist: f64,
}

#[derive(Clone, Debug, Default)]
pub struct RoomRead {
    pub cells: Vec<f64>,
    pub steps: JsMap<crate::Num, f64>,
    pub ways: Vec<crate::roomread::RoomWay>,
}
