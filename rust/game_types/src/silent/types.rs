//! The types of `silent`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum By {
    #[default] Start,
    Order,
    Tool,
    Api,
}
impl By {
    pub const fn as_str(self) -> &'static str {
        match self {
            By::Start => "start",
            By::Order => "order",
            By::Tool => "tool",
            By::Api => "api",
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub cast: Vec<GuardId>,
    pub silent: bool,
    pub since: f64,
    pub hot: bool,
    pub was: Option<bool>,
    pub booted: Option<JsSet<DoorId>>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SilentModSILENTLOGItem {
    pub t: f64,
    pub to: crate::silent::SilentModSILENTLOGItemTo,
    pub by: crate::silent::By,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum SilentModSILENTLOGItemTo {
    #[default] Silent,
    Loud,
}
impl SilentModSILENTLOGItemTo {
    pub const fn as_str(self) -> &'static str {
        match self {
            SilentModSILENTLOGItemTo::Silent => "silent",
            SilentModSILENTLOGItemTo::Loud => "loud",
        }
    }
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SilentModSeatsValue {
    pub on: bool,
    pub go_digit: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SilentBoot {
    First,
    Again,
}
impl SilentBoot {
    pub const fn as_str(self) -> &'static str {
        match self {
            SilentBoot::First => "first",
            SilentBoot::Again => "again",
        }
    }
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct SilentMod {
    /// (src/game/silent.ts:110 states)
    pub states: JsMap<PlayerId, crate::silent::State>,
    /// (src/game/silent.ts:112 SILENT_LOG)
    pub silent_log: Vec<crate::silent::SilentModSILENTLOGItem>,
    /// (src/game/silent.ts:233 seats)
    pub seats: JsMap<PlayerId, crate::silent::SilentModSeatsValue>,
}
impl SilentMod {
    pub fn blank() -> Self {
        SilentMod {
            states: JsMap::new(),
            silent_log: Vec::new(),
            seats: JsMap::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct SilentStats {
    pub silent: i32,
    pub loud: i32,
    pub locked: i32,
    pub picked: i32,
    pub booted: i32,
    pub held: i32,
    pub held_glass: i32,
    pub compromised: i32,
    pub tools: i32,
}

/// (src/game/silent.ts:222 TARGETS: a table nothing writes)
pub fn targets() -> Vec<crate::orders::TargetKind> { alloc::vec![crate::orders::TargetKind::Floor, crate::orders::TargetKind::Door, crate::orders::TargetKind::None] }

/// (src/game/silent.ts:222 LISTED: a table nothing writes)
pub fn listed() -> Option<Vec<crate::orders::TargetKind>> { Some(alloc::vec![crate::orders::TargetKind::Floor, crate::orders::TargetKind::None]) }

/// (src/game/silent.ts:223 HEAD)
pub const HEAD: &str = "the whole squad";
