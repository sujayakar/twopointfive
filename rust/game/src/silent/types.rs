//! The types of `silent`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum By {
    Start,
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

/// (TypeScript: State, src/game/silent.ts:107)
#[derive(Clone, Debug)]
pub struct State {
    pub cast: Vec<GuardId>,
    pub silent: bool,
    pub since: f64,
    pub hot: bool,
    pub was: Option<bool>,
    pub booted: Option<JsSet<DoorId>>,
}
impl State {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        State {
            cast: Vec::new(),
            silent: false,
            since: 0.0,
            hot: false,
            was: None,
            booted: None,
        }
    }
}
impl Default for State { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/silent.ts:110 SILENT_LOG, src/game/silent.ts:110)
#[derive(Clone, Copy, Debug)]
pub struct SilentModSILENTLOGItem {
    pub t: f64,
    pub to: crate::silent::SilentModSILENTLOGItemTo,
    pub by: crate::silent::By,
}
impl SilentModSILENTLOGItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SilentModSILENTLOGItem {
            t: 0.0,
            to: crate::silent::SilentModSILENTLOGItemTo::Silent,
            by: crate::silent::By::Start,
        }
    }
}
impl Default for SilentModSILENTLOGItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SilentModSILENTLOGItemTo {
    Silent,
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

/// (TypeScript: inline at src/game/silent.ts:229 seats, src/game/silent.ts:229)
#[derive(Clone, Copy, Debug)]
pub struct SilentModSeatsValue {
    pub on: bool,
    pub go_digit: bool,
}
impl SilentModSeatsValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SilentModSeatsValue {
            on: false,
            go_digit: false,
        }
    }
}
impl Default for SilentModSeatsValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
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
#[derive(Clone, Debug)]
pub struct SilentMod {
    /// (src/game/silent.ts:108 states)
    pub states: JsMap<PlayerId, crate::silent::State>,
    /// (src/game/silent.ts:110 SILENT_LOG)
    pub silent_log: Vec<crate::silent::SilentModSILENTLOGItem>,
    /// (src/game/silent.ts:229 seats)
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

/// (src/game/silent.ts:105 LOUD_ROWS: a table nothing writes)
pub fn loud_rows() -> Vec<String> { alloc::vec![String::from("bangClear"), String::from("bigClear"), String::from("charge"), String::from("thrBang"), String::from("fragClear"), String::from("wallCharge"), String::from("fragThere"), String::from("stunThere"), String::from("shellThere")] }

/// (src/game/silent.ts:218 TARGETS: a table nothing writes)
pub fn targets() -> Vec<String> { alloc::vec![String::from("floor"), String::from("door"), String::from("none")] }

/// (src/game/silent.ts:218 LISTED: a table nothing writes)
pub fn listed() -> Option<Vec<String>> { Some(alloc::vec![String::from("floor"), String::from("none")]) }

/// (src/game/silent.ts:219 HEAD)
pub const HEAD: &str = "the whole squad";
