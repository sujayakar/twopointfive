//! The types of `digin`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
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
    pub tried_deep: bool,
    pub gathered: bool,
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
    pub also: Option<f64>,
    pub also_n: Option<f64>,
    pub home_t: Option<f64>,
    pub home_d: Option<f64>,
    pub home_at: Option<f64>,
    pub hand_t: Option<f64>,
    pub pull_t: Option<f64>,
    pub pushes: Option<f64>,
    pub pull_a: Option<f64>,
    pub stalls: Option<f64>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes)]
pub enum Cue {
    Kick,
    Scrape,
    Radio,
    Shout,
    Tried,
}
impl Cue {
    pub const fn as_str(self) -> &'static str {
        match self {
            Cue::Kick => "kick",
            Cue::Scrape => "scrape",
            Cue::Radio => "radio",
            Cue::Shout => "shout",
            Cue::Tried => "tried",
        }
    }
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
    pub heard: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DiginModDIGINLOGItem {
    pub t: f64,
    pub who: String,
    pub what: String,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DiginModDIGINSHUT {
    pub home: f64,
    pub held: f64,
    pub again: f64,
    pub more: f64,
    pub left: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ListenersItem {
    pub gd: Option<GuardId>,
    pub head: V3,
    pub vel: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HeardBy {
    pub gd: Option<GuardId>,
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
#[derive(Clone, Debug, ToBytes)]
pub struct DiginMod {
    /// (src/game/digin.ts:189 DIGIN_LOG)
    pub digin_log: Vec<crate::digin::DiginModDIGINLOGItem>,
    /// (src/game/digin.ts:192 DIGIN_SHUT)
    pub digin_shut: crate::digin::DiginModDIGINSHUT,
    /// (src/game/digin.ts:220 states)
    pub states: Option<crate::digin::St>,
}
impl DiginMod {
    pub fn blank() -> Self {
        DiginMod {
            digin_log: Vec::new(),
            digin_shut: crate::digin::DiginModDIGINSHUT { home: 0.0, held: 0.0, again: 0.0, more: 0.0, left: 0.0 },
            states: None,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct DiginStats {
    pub heard: i32,
    pub far: i32,
    pub dug: i32,
    pub moved: i32,
    pub stayed: i32,
    pub hauls: i32,
    pub brace_beside: i32,
    pub brace_passed: i32,
    pub braced: i32,
    pub kicked: i32,
    pub pulled_to: i32,
    pub radio: i32,
    pub shouts: i32,
    pub scrapes: i32,
    pub callouts: i32,
    pub aborted: i32,
    pub caught: i32,
    pub steady: i32,
    pub tried: i32,
    pub deep: i32,
    pub gathered: i32,
    pub gather_lost: i32,
    pub refuge: i32,
    pub lane_moved: i32,
    pub lane_kept: i32,
    pub rooms: i32,
    pub entered: i32,
    pub cue_first: i32,
    pub ready_first: i32,
    pub half_ready: i32,
    pub cue_radio: i32,
    pub cue_shout: i32,
    pub cue_scrape: i32,
    pub cue_tried: i32,
    pub cue_kick: i32,
    pub wedged_at_entry: i32,
    pub know_to_entry: f64,
    pub know_to_ready: f64,
    pub ready_n: i32,
    pub opens_away: i32,
    pub wedged_away: i32,
}

/// (src/game/digin.ts:163 DIGIN_DEFAULTS: a table nothing writes)
pub fn digin_defaults() -> crate::knobs::DiginK { crate::knobs::DiginK::page() }

/// (src/game/digin.ts:639 HAUL_NEAR)
pub const HAUL_NEAR: f64 = 0.9;

/// (src/game/digin.ts:639 HOME_REACH)
pub const HOME_REACH: f64 = 1.2;
