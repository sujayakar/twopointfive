//! The types of `shutdoor`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Spot {
    pub pos: V3,
    pub yaw: f64,
    pub side: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Job {
    pub door: DoorId,
    pub spot: crate::shutdoor::Spot,
    pub back: bool,
    pub phase: crate::shutdoor::JobPhase,
    pub t0: f64,
    pub t: f64,
    pub pulls: f64,
    pub moved: f64,
    pub near: Option<f64>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum JobPhase {
    #[default] Go,
    Shut,
    Wait,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct K {
    pub open: f64,
    pub body: f64,
    pub slack: f64,
    pub reach: f64,
    pub far_reach: f64,
    pub handle_in: f64,
    pub cross: f64,
    pub go_cap: f64,
    pub settle: f64,
    pub blocked_cap: f64,
    pub latch_cap: f64,
    pub again: f64,
    pub hand_to: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct ShutdoorModSpottedValue {
    pub key: String,
    pub spots: Vec<Option<crate::shutdoor::Spot>>,
}

#[derive(Clone, Debug)]
pub enum Plan {
    Text(String),
    Arm1(crate::shutdoor::PlanArm1),
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PlanArm1 {
    pub gd: GuardId,
    pub spot: crate::shutdoor::Spot,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct ShutdoorMod {
    pub jobs: JsMap<GuardId, crate::shutdoor::Job>,
    pub spotted: Option<crate::shutdoor::ShutdoorModSpottedValue>,
}

pub fn k() -> crate::shutdoor::K { crate::shutdoor::K { open: 0.6, body: 0.37, slack: 0.1, reach: 0.95, far_reach: 1.5, handle_in: 0.14, cross: 4.0, go_cap: 25.0, settle: 1.5, blocked_cap: 4.0, latch_cap: 6.0, again: 0.8, hand_to: 0.22 } }
