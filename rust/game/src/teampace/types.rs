//! The types of `teampace`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RunWhy {
    Blast,
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

/// (TypeScript: Leave, src/game/teampace.ts:111)
#[derive(Clone, Copy, Debug)]
pub struct Leave {
    pub why: crate::teampace::LeaveWhy,
    pub top: f64,
    pub until: f64,
}
impl Leave {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Leave {
            why: crate::teampace::LeaveWhy::Blast,
            top: 0.0,
            until: 0.0,
        }
    }
}
impl Default for Leave { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LeaveWhy {
    Blast,
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

/// (TypeScript: inline at src/game/teampace.ts:103 TEAMPACE_DEBUG, src/game/teampace.ts:103)
#[derive(Clone, Copy, Debug)]
pub struct TEAMPACEDEBUG {
}
impl TEAMPACEDEBUG {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TEAMPACEDEBUG {
        }
    }
}
impl Default for TEAMPACEDEBUG { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/teampace.ts:114 ran, src/game/teampace.ts:114)
#[derive(Clone, Copy, Debug)]
pub struct TeampaceModRanValue {
    pub t: f64,
    pub why: crate::teampace::RunWhy,
}
impl TeampaceModRanValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TeampaceModRanValue {
            t: 0.0,
            why: crate::teampace::RunWhy::Blast,
        }
    }
}
impl Default for TeampaceModRanValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/teampace.ts:116 dash, src/game/teampace.ts:116)
#[derive(Clone, Copy, Debug)]
pub struct TeampaceModDashValue {
    pub t: f64,
    pub x: f64,
    pub z: f64,
    pub yes: bool,
}
impl TeampaceModDashValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TeampaceModDashValue {
            t: 0.0,
            x: 0.0,
            z: 0.0,
            yes: false,
        }
    }
}
impl Default for TeampaceModDashValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/teampace.ts:128 aimSeen, src/game/teampace.ts:128)
#[derive(Clone, Copy, Debug)]
pub struct TeampaceModAimSeenValue {
    pub t: f64,
    pub x: f64,
    pub z: f64,
    pub blind: bool,
    pub since: f64,
}
impl TeampaceModAimSeenValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TeampaceModAimSeenValue {
            t: 0.0,
            x: 0.0,
            z: 0.0,
            blind: false,
            since: 0.0,
        }
    }
}
impl Default for TeampaceModAimSeenValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum IsRunWhy {
    Blast,
    Cover,
    Follow,
    Told,
    Close,
    Fight,
}
impl IsRunWhy {
    pub const fn as_str(self) -> &'static str {
        match self {
            IsRunWhy::Blast => "blast",
            IsRunWhy::Cover => "cover",
            IsRunWhy::Follow => "follow",
            IsRunWhy::Told => "told",
            IsRunWhy::Close => "close",
            IsRunWhy::Fight => "fight",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TeamLeaveWhy {
    Blast,
    Cover,
    Follow,
    Told,
    Close,
    Fight,
}
impl TeamLeaveWhy {
    pub const fn as_str(self) -> &'static str {
        match self {
            TeamLeaveWhy::Blast => "blast",
            TeamLeaveWhy::Cover => "cover",
            TeamLeaveWhy::Follow => "follow",
            TeamLeaveWhy::Told => "told",
            TeamLeaveWhy::Close => "close",
            TeamLeaveWhy::Fight => "fight",
        }
    }
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct TeampaceMod {
    /// (src/game/teampace.ts:112 leave)
    pub leave: JsMap<GuardId, crate::teampace::Leave>,
    /// (src/game/teampace.ts:114 ran)
    pub ran: JsMap<GuardId, crate::teampace::TeampaceModRanValue>,
    /// (src/game/teampace.ts:116 dash)
    pub dash: JsMap<GuardId, crate::teampace::TeampaceModDashValue>,
    /// (src/game/teampace.ts:118 reasonT)
    pub reason_t: JsMap<GuardId, f64>,
    /// (src/game/teampace.ts:128 aimSeen)
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
    pub secs: i32,
}

/// (src/game/teampace.ts:103 TEAMPACE_DEBUG: a table nothing writes)
pub fn teampace_debug() -> crate::teampace::TEAMPACEDEBUG { crate::teampace::TEAMPACEDEBUG {  } }

/// (src/game/teampace.ts:104 PACING_BREAK)
pub const PACING_BREAK: bool = false;

/// (src/game/teampace.ts:105 DOORBRISK_BREAK)
pub const DOORBRISK_BREAK: bool = false;

/// (src/game/teampace.ts:106 WALKAIM_BREAK)
pub const WALKAIM_BREAK: bool = false;

/// (src/game/teampace.ts:107 BREAK)
pub const BREAK: bool = false;

/// (src/game/teampace.ts:120 FRAME)
pub const FRAME: f64 = 0.1;

/// (src/game/teampace.ts:126 AIM_SHORT)
pub const AIM_SHORT: f64 = 0.5;

/// (src/game/teampace.ts:126 WALK_AHEAD)
pub const WALK_AHEAD: f64 = 3.0;
