//! The types of `powerout`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, Words, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Calm {
    pub awareness: f64,
    pub hold: bool,
    pub pinned: bool,
    pub station: V3,
    pub home: V3,
    pub room: Option<RoomfightRoomId>,
    pub ev: Vec<f64>,
    pub watch: f64,
    pub want: f64,
    pub pt: V3,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default)]
pub enum Phase {
    #[default] Out,
    Back,
    Wait,
    There,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Man {
    pub gd: GuardId,
    pub role: crate::powerout::ManRole,
    pub phase: crate::powerout::Phase,
    pub goal: V3,
    pub from: V3,
    pub watch: Option<V3>,
    pub tries: f64,
    pub step_t: f64,
    pub heard0: f64,
    pub there_t: f64,
    pub calm: Option<crate::powerout::Calm>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default)]
pub enum ManRole {
    #[default] Cover,
    Lever,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct PowerOutEvent {
    pub t: f64,
    pub what: crate::powerout::PowerOutEventWhat,
    pub who: String,
    pub why: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum PowerOutEventWhat {
    #[default] Stay,
    Lost,
    Home,
    Go,
    There,
    Said,
    Noticed,
    Nobody,
    Abort,
    Wrecked,
    Restored,
    Overdue,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub since: f64,
    pub decide_at: f64,
    pub go_at: f64,
    pub way_m: f64,
    pub overdue_at: f64,
    pub fell: Option<V3>,
    pub seen: Vec<crate::powerout::StateSeenItem>,
    pub look_at: f64,
    pub men: Vec<crate::powerout::Man>,
    pub decided: bool,
    pub word: bool,
    pub parties: f64,
    pub lost: bool,
    pub events: Vec<crate::powerout::PowerOutEvent>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StateSeenItem {
    pub i: f64,
    pub lit: f64,
}

#[derive(Clone, Debug, Default)]
pub struct WayTo {
    pub path: Vec<V3>,
    pub l: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CoverSpot {
    pub spot: V3,
    pub watch: V3,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct PoweroutMod {
    /// (src/game/powerout.ts:111 STATES)
    pub states: Option<crate::powerout::State>,
}
