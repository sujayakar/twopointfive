//! The types of `wandorder`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Job, src/game/wandorder.ts:41)
#[derive(Clone, Copy, Debug)]
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
impl Job {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Job {
            door: DoorId(0),
            side: 0.0,
            stand: [0.0; 3],
            yaw: 0.0,
            aside: None,
            a: OrdersAssignmentId(0),
            t0: 0.0,
            live_t: 0.0,
            last_t: 0.0,
            live: false,
            knelt: false,
            again: false,
        }
    }
}
impl Default for Job { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wandorder.ts:46 Job.aside, src/game/wandorder.ts:46)
#[derive(Clone, Copy, Debug)]
pub struct JobAside {
    pub pos: V3,
    pub yaw: f64,
}
impl JobAside {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        JobAside {
            pos: [0.0; 3],
            yaw: 0.0,
        }
    }
}
impl Default for JobAside { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: State, src/game/wandorder.ts:57)
#[derive(Clone, Debug)]
pub struct State {
    pub jobs: JsMap<GuardId, crate::wandorder::Job>,
}
impl State {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        State {
            jobs: JsMap::new(),
        }
    }
}
impl Default for State { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum Pick {
    Text(String),
    Arm1(crate::wandorder::PickArm1),
}

/// (TypeScript: inline at src/game/wandorder.ts:71 pick ->, src/game/wandorder.ts:71)
#[derive(Clone, Copy, Debug)]
pub struct PickArm1 {
    pub gd: GuardId,
    pub side: f64,
    pub spot: crate::wandorder::PickArm1Spot,
}
impl PickArm1 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PickArm1 {
            gd: GuardId(0),
            side: 0.0,
            spot: crate::wandorder::PickArm1Spot::zeroed(),
        }
    }
}
impl Default for PickArm1 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wandorder.ts:71 PickArm1.spot, src/game/wandorder.ts:71)
#[derive(Clone, Copy, Debug)]
pub struct PickArm1Spot {
    pub pos: V3,
    pub yaw: f64,
}
impl PickArm1Spot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PickArm1Spot {
            pos: [0.0; 3],
            yaw: 0.0,
        }
    }
}
impl Default for PickArm1Spot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum Issue {
    Text(String),
    Arm1(()),
}

/// (TypeScript: inline at src/game/wandorder.ts:182 wandOrderJobs ->, src/game/wandorder.ts:182)
#[derive(Clone, Copy, Debug)]
pub struct WandOrderJobsItem {
    pub gd: GuardId,
    pub door: DoorId,
    pub side: f64,
    pub live: bool,
    pub knelt: bool,
}
impl WandOrderJobsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WandOrderJobsItem {
            gd: GuardId(0),
            door: DoorId(0),
            side: 0.0,
            live: false,
            knelt: false,
        }
    }
}
impl Default for WandOrderJobsItem { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
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

/// (src/game/wandorder.ts:37 WAND_ORDER)
pub const WAND_ORDER: &str = "wandLook";
