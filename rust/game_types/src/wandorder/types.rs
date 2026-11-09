//! The types of `wandorder`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Job {
    pub door: DoorId,
    pub side: f64,
    pub stand: V3,
    pub yaw: f64,
    pub aside: Option<crate::wandorder::JobAside>,
    pub a: OrdersAssignmentId,
    pub t0: f64,
    pub live_t: f64,
    pub last_t: f64,
    pub live: bool,
    pub knelt: bool,
    pub again: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct JobAside {
    pub pos: V3,
    pub yaw: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub jobs: JsMap<GuardId, crate::wandorder::Job>,
}

#[derive(Clone, Debug)]
pub enum Pick {
    Text(String),
    Arm1(crate::wandorder::PickArm1),
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PickArm1 {
    pub gd: GuardId,
    pub side: f64,
    pub spot: crate::wandorder::PickArm1Spot,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PickArm1Spot {
    pub pos: V3,
    pub yaw: f64,
}

#[derive(Clone, Debug)]
pub enum Issue {
    Text(String),
    Arm1(()),
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WandOrderJobsItem {
    pub gd: GuardId,
    pub door: DoorId,
    pub side: f64,
    pub live: bool,
    pub knelt: bool,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct WandorderMod {
    /// (src/game/wandorder.ts:58 states)
    pub states: JsMap<PlayerId, crate::wandorder::State>,
}
impl WandorderMod {
    pub fn blank() -> Self {
        WandorderMod {
            states: JsMap::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct WandorderStats {
    pub ordered: i32,
    pub live: i32,
    pub moved: i32,
}
