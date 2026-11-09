//! The types of `hands`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Haul {
    pub end: crate::hands::HaulEnd,
    pub phase: crate::hands::HaulPhase,
    pub t: f64,
    pub i_l: f64,
    pub i_r: f64,
    pub half: f64,
    pub from: [V3; 2],
    pub trunk: f64,
    pub carry: Option<crate::hands::HaulCarry>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum HaulEnd {
    #[default] Head,
    Feet,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum HaulPhase {
    #[default] Set,
    Reach,
    Rise,
    Carry,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HaulCarry {
    pub dim: [f64; 4],
    pub lie: crate::hands::Lie,
    pub to: Option<crate::hands::Lie>,
    pub yaw: f64,
    pub at: Option<[f64; 3]>,
    pub spot: Option<V3>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Lie {
    pub p: V3,
    pub u: V3,
    pub t: V3,
    pub kl: V3,
    pub kr: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DoorMove {
    pub d: DoorId,
    pub open: bool,
    pub target: f64,
    pub t: f64,
    pub stage: crate::hands::DoorMoveStage,
    pub goal: Option<V3>,
    pub stall_t: f64,
    pub best: f64,
    pub step_best: f64,
    pub step_stall: f64,
    pub round: Option<bool>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum DoorMoveStage {
    #[default] Hand,
    Swing,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CarryRidesC {
    pub pos: V3,
    pub body_yaw: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LayLie {
    pub lie: crate::hands::Lie,
    pub yaw: f64,
}

pub fn y() -> V3 { [0.0, 1.0, 0.0] }
