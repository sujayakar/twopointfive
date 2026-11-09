//! The types of `teampace`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum RunWhy {
    #[default] Blast,
    Cover,
    Follow,
    Told,
}
impl RunWhy {
    pub const fn as_str(self) -> &'static str {
        match self {
            RunWhy::Blast => "blast",
            RunWhy::Cover => "cover",
            RunWhy::Follow => "follow",
            RunWhy::Told => "told",
        }
    }
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Leave {
    pub why: crate::teampace::LeaveWhy,
    pub top: f64,
    pub until: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum LeaveWhy {
    #[default] Blast,
    Cover,
    Follow,
    Told,
    Close,
    Fight,
}
impl LeaveWhy {
    pub const fn as_str(self) -> &'static str {
        match self {
            LeaveWhy::Blast => "blast",
            LeaveWhy::Cover => "cover",
            LeaveWhy::Follow => "follow",
            LeaveWhy::Told => "told",
            LeaveWhy::Close => "close",
            LeaveWhy::Fight => "fight",
        }
    }
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
#[derive(Clone, Debug, ToBytes)]
pub struct TeampaceMod {
    pub leave: JsMap<GuardId, crate::teampace::Leave>,
    pub ran: JsMap<GuardId, crate::teampace::TeampaceModRanValue>,
    pub dash: JsMap<GuardId, crate::teampace::TeampaceModDashValue>,
    pub reason_t: JsMap<GuardId, f64>,
    pub aim_seen: JsMap<GuardId, crate::teampace::TeampaceModAimSeenValue>,
}
impl TeampaceMod {
    pub fn blank() -> Self {
        TeampaceMod {
            leave: JsMap::new(),
            ran: JsMap::new(),
            dash: JsMap::new(),
            reason_t: JsMap::new(),
            aim_seen: JsMap::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct TeampaceStats {
    pub follow: i32,
    pub blast: i32,
    pub cover: i32,
    pub told: i32,
    pub secs: f64,
}

pub const FRAME: f64 = 0.1;

pub const AIM_SHORT: f64 = 0.5;

pub const WALK_AHEAD: f64 = 3.0;
