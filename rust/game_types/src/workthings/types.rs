//! The types of `workthings`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WorkThing {
    pub kind: crate::workthings::WorkThingKind,
    pub face: V3,
    pub n: V3,
    pub stand: Option<V3>,
    pub name: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum WorkThingKind {
    #[default] Breaker,
    Objective,
}
impl WorkThingKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            WorkThingKind::Breaker => "breaker",
            WorkThingKind::Objective => "objective",
        }
    }
}

#[derive(Clone, Debug, ToBytes)]
pub struct WorkThings {
    pub row: OrdersOrderDefId,
    pub jobs: JsMap<GuardId, crate::workthings::Job>,
    pub hands: JsMap<CharId, crate::workthings::WorkThingsHandsValue>,
    pub cache: Option<crate::workthings::WorkThingsCache>,
    pub log: Vec<crate::workthings::WorkThingsLogItem>,
}
impl WorkThings {
    /// what a new one holds
    pub fn blank() -> Self {
        WorkThings {
            row: OrdersOrderDefId(0),
            jobs: JsMap::new(),
            hands: JsMap::new(),
            cache: None,
            log: Vec::new(),
        }
    }
}
impl Default for WorkThings { fn default() -> Self { Self::blank() } }

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Job {
    pub thing: crate::workthings::WorkThing,
    pub verb: crate::workthings::WorkVerb,
    pub a: OrdersAssignmentId,
    pub t0: f64,
    pub last_t: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum WorkVerb {
    #[default] Drive,
    BreakerOff,
    BreakerOn,
}
impl WorkVerb {
    pub const fn as_str(self) -> &'static str {
        match self {
            WorkVerb::Drive => "drive",
            WorkVerb::BreakerOff => "breakerOff",
            WorkVerb::BreakerOn => "breakerOn",
        }
    }
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WorkThingsHandsValue {
    pub push: crate::character::DoorPush,
    pub until: f64,
    pub seen: f64,
    pub verb: crate::workthings::WorkVerb,
    pub t0: f64,
    pub need: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WorkThingsCache {
    pub key: String,
    pub t: f64,
    pub list: Vec<crate::workthings::WorkThing>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WorkThingsLogItem {
    pub t: f64,
    pub who: String,
    pub what: String,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WorkJob {
    pub verb: crate::workthings::WorkVerb,
    pub secs: f64,
    pub need: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Area {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BreakerErrand {
    pub goal: V3,
    pub mid: V3,
    pub at: V3,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WorkthingsModErrandsValue {
    pub t: f64,
    pub key: String,
    pub e: Option<crate::workthings::BreakerErrand>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PanelFace {
    pub face: V3,
    pub n: V3,
}

#[derive(Clone, Debug)]
pub struct WorkThingsNear {
    pub d: f64,
    pub target: crate::orders::OrderTarget,
}
impl Default for WorkThingsNear {
    fn default() -> Self {
        WorkThingsNear {
            d: 0.0,
            target: crate::orders::OrderTarget::Floor(crate::orders::OrderTargetFloor::default()),
        }
    }
}

#[derive(Clone, Debug)]
pub enum WorkThingsOn {
    Floor(crate::orders::OrderTargetFloor),
    Door(crate::orders::OrderTargetDoor),
    None(crate::orders::OrderTargetNone),
    Foe(crate::orders::OrderTargetFoe),
}

#[derive(Clone, Debug)]
pub enum WorkThingsIssue {
    Text(String),
    Arm1(()),
}

#[derive(Clone, Debug)]
pub struct WorkNear {
    pub d: f64,
    pub target: crate::orders::OrderTarget,
}
impl Default for WorkNear {
    fn default() -> Self {
        WorkNear {
            d: 0.0,
            target: crate::orders::OrderTarget::Floor(crate::orders::OrderTargetFloor::default()),
        }
    }
}

#[derive(Clone, Debug)]
pub enum WorkOn {
    Floor(crate::orders::OrderTargetFloor),
    Door(crate::orders::OrderTargetDoor),
    None(crate::orders::OrderTargetNone),
    Foe(crate::orders::OrderTargetFoe),
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct WorkthingsMod {
    /// (src/game/workthings.ts:136 errands)
    pub errands: Option<crate::workthings::WorkthingsModErrandsValue>,
}
impl WorkthingsMod {
    pub fn blank() -> Self {
        WorkthingsMod {
            errands: None,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct WorkthingsStats {
    pub given: i32,
    pub done: i32,
    pub moot: i32,
}
