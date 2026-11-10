//! The types of `digin`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default)]
pub enum Phase {
    #[default] Set,
    Idle,
    Shut,
    Haul,
    Think,
    ToRoom,
    ToProp,
    ToDoor,
    Wedge,
    ToCover,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Man {
    pub far: bool,
    pub owe: f64,
    pub haul_by: f64,
    pub deep: bool,
    pub look: Option<V3>,
    pub gather_to: Option<RoomfightRoomId>,
    pub dug_once: bool,
    pub ordered: bool,
    pub phase: crate::digin::Phase,
    pub until: f64,
    pub know: V3,
    pub know_t: f64,
    pub room: Option<RoomfightRoomId>,
    pub entry: f64,
    pub brace: f64,
    pub prop: Option<PropId>,
    pub stand: V3,
    pub rest: V3,
    pub scrape_t: f64,
    pub set_at: f64,
    pub spot: Option<V3>,
    pub eyes_t: Option<f64>,
    pub lane_t: Option<f64>,
    pub home_t: Option<f64>,
    pub home_d: Option<f64>,
    pub home_at: Option<f64>,
    pub pull_a: Option<f64>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Words)]
pub enum Cue {
    Kick,
    Scrape,
    Radio,
    Shout,
    Tried,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct RoomSt {
    pub room: RoomfightRoomId,
    pub expected: f64,
    pub second: f64,
    pub at: f64,
    pub t0: f64,
    pub men: Vec<GuardId>,
    pub bracer: Option<GuardId>,
    pub wedged: f64,
    pub cue_at: f64,
    pub cues: JsSet<crate::digin::Cue>,
    pub entered_at: f64,
    pub ready_at: f64,
    pub callout: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct St {
    pub men: JsMap<GuardId, crate::digin::Man>,
    pub rooms: JsMap<RoomfightRoomId, crate::digin::RoomSt>,
    pub wedges: JsMap<DoorId, crate::digin::StWedgesValue>,
    pub fix: Option<crate::digin::StFix>,
    pub called: JsSet<GuardId>,
    pub used: JsSet<PropId>,
    pub away: JsSet<DoorId>,
    pub frame_t: f64,
    pub bang_call: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct StWedgesValue {
    pub key_side: f64,
    pub lock_id: String,
    pub kicked: bool,
    pub tried: bool,
    pub by_t: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StFix {
    pub at: V3,
    pub t: f64,
    pub room: Option<RoomfightRoomId>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DiginModDIGINSHUT {
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ListenersItem {
    pub gd: Option<GuardId>,
    pub head: V3,
    pub vel: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HeardNoise {
    pub gd: Option<GuardId>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CoverSpot {
    pub spot: Option<V3>,
    pub here: bool,
    pub look: Option<V3>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DigInRefuge {
    pub spot: V3,
    pub door: Option<DoorId>,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct DiginMod {
    #[is(crate::digin::DiginModDIGINSHUT { })] pub digin_shut: crate::digin::DiginModDIGINSHUT,
    pub states: Option<crate::digin::St>,
}

pub fn digin_defaults() -> crate::knobs::DiginK { crate::knobs::DiginK::page() }

pub const HAUL_NEAR: f64 = 0.9;

pub const HOME_REACH: f64 = 1.2;
