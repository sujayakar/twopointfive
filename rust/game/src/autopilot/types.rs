//! The types of `autopilot`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum DoorTarget {
    Arm0,
    Door(crate::autopilot::DoorTargetDoor),
    Arm2,
    Arm3,
}

/// (TypeScript: inline at src/game/autopilot.ts:73 DoorTarget, src/game/autopilot.ts:73 DoorTarget)
#[derive(Clone, Copy, Debug)]
pub struct DoorTargetDoor {
    pub at: V3,
    pub door: DoorId,
    pub side: f64,
}
impl DoorTargetDoor {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorTargetDoor {
            at: [0.0; 3],
            door: DoorId(0),
            side: 0.0,
        }
    }
}
impl Default for DoorTargetDoor { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: AutoPick, src/game/autopilot.ts:76)
#[derive(Clone, Debug)]
pub struct AutoPick {
    pub id: Option<String>,
    pub men: Option<Vec<GuardId>>,
    pub word: bool,
    pub why: crate::autopilot::AutoPickWhy,
    pub t: crate::orders::OrderTarget,
    pub kept: Option<bool>,
}
impl AutoPick {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AutoPick {
            id: None,
            men: None,
            word: false,
            why: crate::autopilot::AutoPickWhy::Set,
            t: crate::orders::OrderTarget::Floor(crate::orders::OrderTargetFloor::zeroed()),
            kept: None,
        }
    }
}
impl Default for AutoPick { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AutoPickWhy {
    Set,
    Blow,
    Contact,
    Pending,
    Unknown,
    Shut,
    First,
    Silent,
    Known,
    Word,
}
impl AutoPickWhy {
    pub const fn as_str(self) -> &'static str {
        match self {
            AutoPickWhy::Set => "set",
            AutoPickWhy::Blow => "blow",
            AutoPickWhy::Contact => "contact",
            AutoPickWhy::Pending => "pending",
            AutoPickWhy::Unknown => "unknown",
            AutoPickWhy::Shut => "shut",
            AutoPickWhy::First => "first",
            AutoPickWhy::Silent => "silent",
            AutoPickWhy::Known => "known",
            AutoPickWhy::Word => "word",
        }
    }
}

/// (TypeScript: inline at src/game/autopilot.ts:93 autoEntries ->, src/game/autopilot.ts:93)
#[derive(Clone, Debug)]
pub struct AutoEntries {
    pub ids: Vec<String>,
    pub why: crate::autopilot::AutoEntriesWhy,
    pub kept: Option<bool>,
}
impl AutoEntries {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AutoEntries {
            ids: Vec::new(),
            why: crate::autopilot::AutoEntriesWhy::Set,
            kept: None,
        }
    }
}
impl Default for AutoEntries { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AutoEntriesWhy {
    Set,
    Blow,
    Contact,
    Pending,
    Unknown,
    Shut,
    First,
    Silent,
    Known,
    Word,
}
impl AutoEntriesWhy {
    pub const fn as_str(self) -> &'static str {
        match self {
            AutoEntriesWhy::Set => "set",
            AutoEntriesWhy::Blow => "blow",
            AutoEntriesWhy::Contact => "contact",
            AutoEntriesWhy::Pending => "pending",
            AutoEntriesWhy::Unknown => "unknown",
            AutoEntriesWhy::Shut => "shut",
            AutoEntriesWhy::First => "first",
            AutoEntriesWhy::Silent => "silent",
            AutoEntriesWhy::Known => "known",
            AutoEntriesWhy::Word => "word",
        }
    }
}

/// (TypeScript: inline at src/game/autopilot.ts:152 showSecsOf(row), src/game/autopilot.ts:152)
#[derive(Clone, Copy, Debug)]
pub struct ShowSecsOfRow {
    pub auto: Option<bool>,
}
impl ShowSecsOfRow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShowSecsOfRow {
            auto: None,
        }
    }
}
impl Default for ShowSecsOfRow { fn default() -> Self { Self::zeroed() } }

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct AutopilotStats {
    pub presses: i32,
    pub first: i32,
    pub bang: i32,
    pub open: i32,
    pub charge: i32,
    pub blow: i32,
    pub word: i32,
    pub refused: i32,
    pub shows: i32,
    pub again: i32,
}

/// (src/game/autopilot.ts:70 RESTACK_LINES: a table nothing writes)
pub fn restack_lines() -> Vec<String> { alloc::vec![String::from("stacked up."), String::from("we're ready to go.")] }
