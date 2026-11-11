//! The types of `rearwatch`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RearPin {
    pub o: CoverOpeningId,
    pub front: V3,
    pub door: DoorId,
    pub side: f64,
    pub op: Option<DoorOpId>,
    pub seen_t: f64,
    pub a: TaskId,
    pub going: Option<bool>,
    pub raised: Option<bool>,
    pub out: Option<bool>,
    pub to: Option<V3>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct RoomKeep {
    pub pin: crate::rearwatch::RearPin,
    pub room: RoomfightRoomId,
    pub out_t: f64,
    pub behind: Option<Vec<GuardId>>,
    pub at: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Book {
    pub pins: JsMap<GuardId, crate::rearwatch::RearPin>,
    pub by_op: JsMap<DoorOpId, GuardId>,
    pub short: JsMap<TaskId, f64>,
    pub let_off: JsSet<TaskId>,
    pub log: Vec<String>,
    pub rows: f64,
    pub keeps: JsMap<GuardId, crate::rearwatch::RoomKeep>,
    pub was: Vec<(GuardId, DoorId, TaskId)>,
    pub shut: JsMap<DoorId, crate::rearwatch::BookShutValue>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BookShutValue {
    pub o: CoverOpeningId,
    pub room: RoomfightRoomId,
    pub at: V3,
}

#[derive(Clone, Debug, Default)]
pub struct Group {
    pub op: Option<DoorOpId>,
    pub door: DoorId,
    pub side: f64,
    pub men: Vec<GuardId>,
    pub marks: Vec<V3>,
    pub cands: Vec<GuardId>,
    pub going: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RearWaysItem {
    pub o: CoverOpeningId,
    pub front: V3,
    pub cls: f64,
    pub d: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct RearwatchMod {
    pub books: JsMap<GroupId, crate::rearwatch::Book>,
}

pub const EYE: f64 = 1.55;

pub const CHEST: f64 = 1.1;

pub const FORMED_M: f64 = 1.5;

pub const FALLING: &str = "falling in";
