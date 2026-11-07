//! The types of `shutdoor`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Spot, src/game/shutdoor.ts:56)
#[derive(Clone, Copy, Debug)]
pub struct Spot {
    pub pos: V3,
    pub yaw: f64,
    pub side: f64,
    pub far: bool,
}
impl Spot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Spot {
            pos: [0.0; 3],
            yaw: 0.0,
            side: 0.0,
            far: false,
        }
    }
}
impl Default for Spot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Job, src/game/shutdoor.ts:57)
#[derive(Clone, Copy, Debug)]
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
impl Job {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Job {
            door: DoorId(0),
            spot: crate::shutdoor::Spot::zeroed(),
            back: false,
            phase: crate::shutdoor::JobPhase::Go,
            t0: 0.0,
            t: 0.0,
            pulls: 0.0,
            moved: 0.0,
            near: None,
        }
    }
}
impl Default for Job { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum JobPhase {
    Go,
    Shut,
    Wait,
}
impl JobPhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            JobPhase::Go => "go",
            JobPhase::Shut => "shut",
            JobPhase::Wait => "wait",
        }
    }
}

/// (TypeScript: inline at src/game/shutdoor.ts:36 K, src/game/shutdoor.ts:36)
#[derive(Clone, Copy, Debug)]
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
impl K {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        K {
            open: 0.0,
            body: 0.0,
            slack: 0.0,
            reach: 0.0,
            far_reach: 0.0,
            handle_in: 0.0,
            cross: 0.0,
            go_cap: 0.0,
            settle: 0.0,
            blocked_cap: 0.0,
            latch_cap: 0.0,
            again: 0.0,
            hand_to: 0.0,
        }
    }
}
impl Default for K { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/shutdoor.ts:54 SHUTDOOR_LOG, src/game/shutdoor.ts:54)
#[derive(Clone, Debug)]
pub struct ShutdoorModSHUTDOORLOGItem {
    pub t: f64,
    pub who: String,
    pub door: String,
    pub what: String,
}
impl ShutdoorModSHUTDOORLOGItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShutdoorModSHUTDOORLOGItem {
            t: 0.0,
            who: String::new(),
            door: String::new(),
            what: String::new(),
        }
    }
}
impl Default for ShutdoorModSHUTDOORLOGItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/shutdoor.ts:108 spotted, src/game/shutdoor.ts:108)
#[derive(Clone, Debug)]
pub struct ShutdoorModSpottedValue {
    pub key: String,
    pub spots: Vec<Option<crate::shutdoor::Spot>>,
}
impl ShutdoorModSpottedValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShutdoorModSpottedValue {
            key: String::new(),
            spots: Vec::new(),
        }
    }
}
impl Default for ShutdoorModSpottedValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum Plan {
    Text(String),
    Arm1(crate::shutdoor::PlanArm1),
}

/// (TypeScript: inline at src/game/shutdoor.ts:119 plan ->, src/game/shutdoor.ts:119)
#[derive(Clone, Copy, Debug)]
pub struct PlanArm1 {
    pub gd: GuardId,
    pub spot: crate::shutdoor::Spot,
}
impl PlanArm1 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlanArm1 {
            gd: GuardId(0),
            spot: crate::shutdoor::Spot::zeroed(),
        }
    }
}
impl Default for PlanArm1 { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct ShutdoorMod {
    /// (src/game/shutdoor.ts:54 SHUTDOOR_LOG)
    pub shutdoor_log: Vec<crate::shutdoor::ShutdoorModSHUTDOORLOGItem>,
    /// (src/game/shutdoor.ts:58 jobs)
    pub jobs: JsMap<GuardId, crate::shutdoor::Job>,
    /// (src/game/shutdoor.ts:108 spotted)
    pub spotted: Option<crate::shutdoor::ShutdoorModSpottedValue>,
}
impl ShutdoorMod {
    pub fn blank() -> Self {
        ShutdoorMod {
            shutdoor_log: Vec::new(),
            jobs: JsMap::new(),
            spotted: None,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct ShutdoorStats {
    pub given: i32,
    pub shut: i32,
    pub gave_up: i32,
    pub to_file: i32,
    pub stayed: i32,
}

/// (src/game/shutdoor.ts:33 SHUT_ID)
pub const SHUT_ID: &str = "shutDoor";

/// (src/game/shutdoor.ts:35 STOOD)
pub const STOOD: &str = "charge";

/// (src/game/shutdoor.ts:36 K: a table nothing writes)
pub fn k() -> crate::shutdoor::K { crate::shutdoor::K { open: 0.6, body: 0.37, slack: 0.1, reach: 0.95, far_reach: 1.5, handle_in: 0.14, cross: 4.0, go_cap: 25.0, settle: 1.5, blocked_cap: 4.0, latch_cap: 6.0, again: 0.8, hand_to: 0.22 } }
