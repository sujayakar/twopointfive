//! The types of `teampace`.
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum RunWhy {
    #[default] Blast,
    Cover,
    Follow,
    Told,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Leave {
    pub why: crate::teampace::LeaveWhy,
    pub top: f64,
    pub until: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum LeaveWhy {
    #[default] Blast,
    Cover,
    Follow,
    Told,
    Close,
    Fight,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct TeampaceModRanValue {
    pub t: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct TeampaceModDashValue {
    pub t: f64,
    pub x: f64,
    pub z: f64,
    pub yes: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct TeampaceModAimSeenValue {
    pub t: f64,
    pub x: f64,
    pub z: f64,
    pub blind: bool,
    pub since: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct TeampaceMod {
    pub leave: JsMap<GuardId, crate::teampace::Leave>,
    pub ran: JsMap<GuardId, crate::teampace::TeampaceModRanValue>,
    pub dash: JsMap<GuardId, crate::teampace::TeampaceModDashValue>,
    pub aim_seen: JsMap<GuardId, crate::teampace::TeampaceModAimSeenValue>,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct TeampaceStats {
    pub cover: i32,
}

pub const FRAME: f64 = 0.1;

pub const AIM_SHORT: f64 = 0.5;

pub const WALK_AHEAD: f64 = 3.0;
