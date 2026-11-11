//! The types of `squad`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Sweep {
    pub pts: Vec<V3>,
    pub t0: f64,
    #[is(1.0)] pub dur: f64,
    pub active: bool,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Clearing {
    #[is(crate::squad::ClearStage::Muster)] pub stage: crate::squad::ClearStage,
    pub room_i: usize,
    pub drill: Option<RoomClearId>,
    pub cur: Option<crate::scene_level::RoomClearPlan>,
    pub t: f64,
    pub total: f64,
    pub eta: f64,
    pub t_drill: f64,
    #[is(f64::NAN)] pub t_near: f64,
    #[is(alloc::vec![0.0, 0.0])] pub aw: Vec<f64>,
    #[is([Vec::new(), Vec::new()])] pub wps: [Vec<V3>; 2],
    pub dark: bool,
    pub rooms: Vec<Vec<crate::scene_level::RoomClearPlan>>,
    #[is(GuardId(0))] pub a: GuardId,
    pub b: Option<GuardId>,
    /// dealt where a clear is begun
    pub serial: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum ClearStage {
    Approach,
    Done,
    Muster,
    Drill,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct RoomClear {
    #[is(crate::squad::RoomClearPhase::Stack)] pub phase: crate::squad::RoomClearPhase,
    #[is(f64::NAN)] pub t_kick: f64,
    #[is(f64::NAN)] pub t_enter: f64,
    #[is(f64::NAN)] pub t_search: f64,
    #[is(f64::NAN)] pub t_resolve: f64,
    pub door: Option<DoorId>,
    pub next_door: Option<DoorId>,
    pub live: bool,
    pub done: JsSet<String>,
    pub flung: bool,
    pub b_go: bool,
    pub no_mark: bool,
    #[is(f64::NAN)] pub t_set: f64,
    #[is(f64::NAN)] pub t_go: f64,
    pub t_now: f64,
    pub a_home: bool,
    pub b_home: bool,
    pub aim_a: V3,
    pub aim_b: V3,
    pub sw_a: crate::squad::Sweep,
    pub sw_b: crate::squad::Sweep,
    pub rest_a: Option<Vec<V3>>,
    pub rest_b: Option<Vec<V3>>,
    pub wp_a: Vec<V3>,
    pub wp_b: Vec<V3>,
    pub plan: crate::scene_level::RoomClearPlan,
    #[is(GuardId(0))] pub a: GuardId,
    pub b: Option<GuardId>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum RoomClearPhase {
    Search,
    Kick,
    Breach,
    Stack,
    Enter,
    Resolve,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SquadModSettleWatchValue {
    pub goal: V3,
    pub best: f64,
    pub t: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SquadModWalkWatchValue {
    pub x: f64,
    pub z: f64,
    pub t: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CENTRECHECK {
    pub corner: f64,
    pub total: f64,
    pub min_leg: f64,
}

#[derive(Clone, Debug, Default)]
pub struct BODYLINES {
    pub stack1: Option<String>,
    pub stack2: Option<String>,
    pub go2: Option<String>,
    pub in1: Option<String>,
    pub side2: Option<String>,
    pub side1: Option<String>,
    pub push1: Option<String>,
    pub under2: Option<String>,
    pub nothing2: Option<String>,
    pub corner1: Option<String>,
    pub empty2: Option<String>,
    pub radio1: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct BODYLINESNOW {
    pub stack1: Option<String>,
    pub stack2: Option<String>,
    pub go2: Option<String>,
    pub in1: Option<String>,
    pub side2: Option<String>,
    pub side1: Option<String>,
    pub push1: Option<String>,
    pub under2: Option<String>,
    pub nothing2: Option<String>,
    pub corner1: Option<String>,
    pub empty2: Option<String>,
    pub radio1: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct BODYLINESCOLDNOW {
    pub stack1: Option<String>,
    pub stack2: Option<String>,
    pub go2: Option<String>,
    pub in1: Option<String>,
    pub side2: Option<String>,
    pub side1: Option<String>,
    pub push1: Option<String>,
    pub under2: Option<String>,
    pub nothing2: Option<String>,
    pub corner1: Option<String>,
    pub empty2: Option<String>,
    pub radio1: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct BODYLINESCOLD {
    pub stack1: Option<String>,
    pub stack2: Option<String>,
    pub go2: Option<String>,
    pub in1: Option<String>,
    pub side2: Option<String>,
    pub side1: Option<String>,
    pub push1: Option<String>,
    pub under2: Option<String>,
    pub nothing2: Option<String>,
    pub corner1: Option<String>,
    pub empty2: Option<String>,
    pub radio1: Option<String>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct OrderPatch {
    pub goal: Option<Option<V3>>,
    pub speed: Option<Option<f64>>,
    pub face: Option<Option<f64>>,
    pub strafe: Option<Option<bool>>,
    pub aim_at: Option<Option<V3>>,
    pub stance: Option<Option<crate::character::Stance>>,
    pub stack_side: Option<Option<f64>>,
    pub crouch: Option<Option<bool>>,
    pub lean: Option<Option<f64>>,
    pub upper: Option<Option<crate::character::UpperMode>>,
    pub light: Option<Option<bool>>,
    pub arrived: Option<Option<bool>>,
    pub hot: Option<Option<bool>>,
    pub flinch: Option<Option<bool>>,
    pub direct: Option<Option<bool>>,
    pub smoke: Option<Option<bool>>,
    pub over: Option<Option<crate::game::GuardScriptOver>>,
    /// true only from a caller that passes a goal it keeps and moves: that one compares by value. goal Some(Some(_)) from anyone else is a new leg, even at the same spot
    pub held: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RoomClearNewOpts {
    pub live: Option<bool>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum RoomClearSayKey {
    Stack1,
    Stack2,
    Go2,
    In1,
    Side2,
    Side1,
    Push1,
    Under2,
    Nothing2,
    Corner1,
    Empty2,
    Radio1,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Perceive {
    Suspicious,
    Alert,
    Body,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct SquadMod {
    pub settle_watch: JsMap<GuardId, crate::squad::SquadModSettleWatchValue>,
    pub walk_watch: JsMap<GuardId, crate::squad::SquadModWalkWatchValue>,
}

pub const SETTLE: f64 = 0.6;

pub const FACE_WITHIN: f64 = 1.0;

pub const STACK_CAP: f64 = 6.0;

pub const KICK_BACK: f64 = 0.7;

pub fn centre_check() -> crate::squad::CENTRECHECK { crate::squad::CENTRECHECK { corner: 0.95, total: 3.0, min_leg: 0.6 } }

pub fn body_lines() -> crate::squad::BODYLINES { crate::squad::BODYLINES { stack1: None, stack2: None, go2: None, in1: None, side2: Some(String::from("door side clear — apart from him.")), side1: Some(String::from("my side's clear. …he's right here.")), push1: None, under2: Some(String::from("checking under… just him.")), nothing2: Some(String::from("nothing else down here.")), corner1: Some(String::from("corner's clear. only him.")), empty2: Some(String::from("room's clear — nobody but him.")), radio1: None } }

pub fn body_lines_now() -> crate::squad::BODYLINESNOW { crate::squad::BODYLINESNOW { stack1: None, stack2: None, go2: None, in1: None, side2: Some(String::from("door side clear — except for him.")), side1: None, push1: None, under2: None, nothing2: None, corner1: None, empty2: None, radio1: None } }

pub fn body_lines_cold_now() -> crate::squad::BODYLINESCOLDNOW { crate::squad::BODYLINESCOLDNOW { stack1: None, stack2: None, go2: None, in1: None, side2: None, side1: None, push1: None, under2: None, nothing2: Some(String::from("nothing else down here. he took a hard hit.")), corner1: None, empty2: None, radio1: None } }

pub fn body_lines_cold() -> crate::squad::BODYLINESCOLD { crate::squad::BODYLINESCOLD { stack1: None, stack2: None, go2: None, in1: None, side2: Some(String::from("door side clear — just him, out cold.")), side1: Some(String::from("my side's clear. …he's here — still breathing.")), push1: None, under2: Some(String::from("checking under… just him. breathing.")), nothing2: Some(String::from("nothing else down here. he took a proper hit.")), corner1: Some(String::from("corner's clear. only him.")), empty2: Some(String::from("room's clear — nobody but him, and he's breathing.")), radio1: None } }

pub const SUSPECT: f64 = 0.4;

pub const APPROACH_SPEED: f64 = 1.6;

pub const DRILL_CAP: f64 = 32.0;
