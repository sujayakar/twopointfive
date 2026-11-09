//! The types of `pressure`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Man {
    pub d: RoomfightDefId,
    pub role: crate::pressure::ManRole,
    pub door: DoorId,
    pub spot: V3,
    pub from: Option<RoomfightRoomId>,
    pub moving: bool,
    pub set: bool,
    pub set_at: f64,
    pub tries: f64,
    pub step_t: f64,
    pub small0: f64,
    pub cue_t: f64,
    pub free_at: f64,
    pub land_t: f64,
    pub arrived: bool,
    pub heard: bool,
    pub fell: bool,
    pub replant_t: f64,
    pub fought: bool,
    pub out: bool,
    pub ask_t: Option<f64>,
    pub charged: Option<bool>,
    pub ear_t: Option<f64>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default, Words)]
pub enum ManRole {
    #[default] Press,
    Flank,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Press {
    pub id: f64,
    pub room: RoomfightRoomId,
    pub at: V3,
    pub t0: f64,
    pub phase: crate::pressure::PressPhase,
    pub team_in: bool,
    pub set_at: f64,
    pub lone_at: Option<f64>,
    pub sprung: Option<bool>,
    pub moving: bool,
    pub word_at: f64,
    pub go_at: f64,
    pub n0: f64,
    pub lone: bool,
    pub men: Vec<PressureManId>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum PressPhase {
    #[default] Go,
    Word,
    Gather,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Out {
    pub door: Option<DoorId>,
    pub def: RoomfightDefId,
    pub moving: bool,
    pub plant_t: f64,
    pub fought: bool,
    pub step_t: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Warn {
    pub phase: crate::pressure::WarnPhase,
    pub t0: f64,
    pub by: RoomfightDefId,
    pub call_t: f64,
    pub last_t: f64,
    pub room: Option<RoomfightRoomId>,
    pub hit_t: Option<f64>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum WarnPhase {
    #[default] Heard,
    Called,
    Over,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub presses: Vec<crate::pressure::Press>,
    pub of: JsMap<RoomfightDefId, PressureManId>,
    pub outs: Vec<crate::pressure::Out>,
    pub serial: f64,
    pub stale: bool,
    pub think: JsMap<RoomfightDefId, crate::pressure::StateThinkValue>,
    pub stay: JsMap<RoomfightDefId, RoomfightRoomId>,
    pub called: JsMap<RoomfightDefId, crate::pressure::StateCalledValue>,
    pub stood_t: f64,
    pub sent: JsSet<RoomfightRoomId>,
    pub done_at: JsMap<RoomfightDefId, f64>,
    pub last: JsMap<RoomfightDefId, f64>,
    pub tried: JsMap<RoomfightDefId, f64>,
    pub ear: JsMap<RoomfightDefId, crate::pressure::StateEarValue>,
    pub warn: Option<crate::pressure::Warn>,
    pub told: JsMap<RoomfightDefId, crate::pressure::StateToldValue>,
    pub heard: Vec<crate::pressure::StateHeardItem>,
    pub ev_seen: JsSet<u32>,
    pub marks: JsMap<DoorId, crate::pressure::StateMarksValue>,
    pub bangs: JsMap<(DoorId, crate::teams::Team), bool>,
    pub last_held: JsSet<RoomfightDefId>,
    pub word: Option<crate::pressure::StateWord>,
    pub swept: f64,
    pub look: Option<crate::pressure::StateLook>,
    pub out: Option<Vec<RoomfightDefId>>,
    pub look_lost: bool,
    pub look_t: f64,
    pub stats: crate::pressure::StateStats,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StateThinkValue {
    pub room: RoomfightRoomId,
    pub at: f64,
    pub held: Option<f64>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StateCalledValue {
    pub room: RoomfightRoomId,
    pub t: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StateEarValue {
    pub t: f64,
    pub i: f64,
    pub at: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StateToldValue {
    pub room: RoomfightRoomId,
    pub at: V3,
    pub t: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StateHeardItem {
    pub t: f64,
    pub p: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StateMarksValue {
    pub r: Option<RoomfightRoomId>,
    pub team: crate::teams::Team,
    pub own: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StateWord {
    pub t: f64,
    pub p: V3,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct StateLook {
    pub at: V3,
    pub go_t: f64,
    pub w: f64,
    pub men: Vec<RoomfightDefId>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StateStats {
    pub press_formed: f64,
    pub press_men: f64,
    pub press_sieges: f64,
    pub press_converge: f64,
    pub press_words: f64,
    pub press_lone: f64,
    pub press_arrived: f64,
    pub press_late: f64,
    pub press_landed: f64,
    pub press_cue_first: f64,
    pub press_heard: f64,
    pub press_lead: f64,
    pub flank_sent: f64,
    pub flank_set: f64,
    pub flank_arrived: f64,
    pub flank_late: f64,
    pub flank_landed: f64,
    pub flank_cue_first: f64,
    pub flank_heard: f64,
    pub flank_lead: f64,
    pub fb_push: f64,
    pub fb_last: f64,
    pub fb_early: f64,
    pub fb_planted: f64,
    pub fb_fought: f64,
    pub fb_told: f64,
    pub warn_calls: f64,
    pub warn_stood_down: f64,
    pub warn_hits: f64,
    pub warn_lead: f64,
    pub second_stood: f64,
    pub second_calls: f64,
    pub pairs_told: f64,
    pub pairs_waited: f64,
    pub pairs_went: f64,
    pub stun_marked: f64,
    pub stun_held: f64,
    pub stun_held_in: f64,
    pub stun_late: f64,
    pub stun_off: f64,
    pub stun_bangs: f64,
    pub stun_empty: f64,
    pub last_held: f64,
    pub look_called: f64,
    pub look_men: f64,
    pub look_off: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HookedWhy {
    Push,
    Door,
    Seen,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Hooked {
    pub spot: V3,
    pub door: Option<DoorId>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Target {
    pub t: RoomfightRoomId,
    pub at: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GatherSpot {
    pub spot: V3,
    pub l: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StandOffSpot {
    pub spot: V3,
    pub l: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TeamPlaced {
    pub known: bool,
    pub moved: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Aim {
    pub t: RoomfightRoomId,
    pub at: V3,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct PressureMod {
    /// (src/game/pressure.ts:297 STATES)
    pub states: Option<crate::pressure::State>,
    /// (src/game/pressure.ts:305 KITTED)
    pub kitted: JsSet<GuardId>,
}

/// (src/game/pressure.ts:306 ROOM_CELLS)
pub const ROOM_CELLS: f64 = 500.0;

/// (src/game/pressure.ts:307 STAND_OFF_BAND)
pub const STAND_OFF_BAND: f64 = 7.0;

/// (src/game/pressure.ts:307 STAND_OFF_TRIES)
pub const STAND_OFF_TRIES: f64 = 10.0;

/// (src/game/pressure.ts:308 PAIR_WAIT_SECS)
pub const PAIR_WAIT_SECS: f64 = 6.0;

/// (src/game/pressure.ts:309 CUE_FIRST_SECS)
pub const CUE_FIRST_SECS: f64 = 1.0;

/// (src/game/pressure.ts:1001 CALLED_SECS)
pub const CALLED_SECS: f64 = 30.0;

/// (src/game/pressure.ts:1108 PRESSURE_DEFAULTS: a table nothing writes)
pub fn pressure_defaults() -> crate::knobs::PressureK { crate::knobs::PressureK::page() }
