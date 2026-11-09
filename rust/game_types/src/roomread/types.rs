//! The types of `roomread`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct RoomWay {
    pub o: CoverOpeningId,
    pub in_n: [f64; 2],
    pub beyond: V3,
    pub far: f64,
    pub far_known: bool,
    pub shut: bool,
    pub locked: bool,
    pub dist: f64,
}

#[derive(Clone, Debug, Default)]
pub struct RoomRead {
    pub at: V3,
    pub room: f64,
    pub known: bool,
    pub cells: Vec<f64>,
    pub steps: JsMap<crate::Num, f64>,
    pub ways: Vec<crate::roomread::RoomWay>,
}

#[derive(Clone, Debug, Default)]
pub struct SideSignD {
    pub def: crate::roomread::SideSignDDef,
}

#[derive(Clone, Debug)]
pub struct SideSignDDef {
    pub closed_dir: f64,
    pub hinge: crate::roomread::SideSignDDefHinge,
}
impl Default for SideSignDDef {
    fn default() -> Self {
        SideSignDDef {
            closed_dir: 0.0,
            hinge: crate::roomread::SideSignDDefHinge::Array(Vec::new()),
        }
    }
}

#[derive(Clone, Debug)]
pub enum SideSignDDefHinge {
    Array(Vec<f64>),
    List([f64; 2]),
}
