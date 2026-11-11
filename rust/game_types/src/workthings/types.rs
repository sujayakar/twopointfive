//! The types of `workthings`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WorkThing {
    pub kind: crate::workthings::WorkThingKind,
    pub face: V3,
    pub n: V3,
    pub stand: Option<V3>,
    pub name: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum WorkThingKind {
    #[default] Breaker,
    Objective,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct WorkThings {
    #[is(OrdersOrderDefId(0))] pub row: OrdersOrderDefId,
    pub jobs: JsMap<GuardId, crate::workthings::Job>,
    pub hands: JsMap<CharId, crate::workthings::WorkThingsHandsValue>,
    pub cache: Option<crate::workthings::WorkThingsCache>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Job {
    pub thing: crate::workthings::WorkThing,
    pub verb: crate::workthings::WorkVerb,
    pub a: TaskId,
    pub t0: f64,
    pub last_t: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum WorkVerb {
    #[default] Drive,
    BreakerOff,
    BreakerOn,
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

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct WorkthingsMod {
    pub errands: Option<crate::workthings::WorkthingsModErrandsValue>,
}

