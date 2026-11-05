//! The types of `buddy`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Man, src/game/buddy.ts:72)
#[derive(Clone, Copy, Debug)]
pub struct Man {
    pub had: bool,
    pub call_at: f64,
    pub called_t: f64,
    pub calls: f64,
    pub open: bool,
    pub look_t: f64,
    pub done_t: f64,
    pub keep_t: f64,
}
impl Man {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Man {
            had: false,
            call_at: 0.0,
            called_t: 0.0,
            calls: 0.0,
            open: false,
            look_t: 0.0,
            done_t: 0.0,
            keep_t: 0.0,
        }
    }
}
impl Default for Man { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Errand, src/game/buddy.ts:73)
#[derive(Clone, Copy, Debug)]
pub struct Errand {
    pub caller: RoomfightDefId,
    pub buddy: RoomfightDefId,
    pub phase: crate::buddy::ErrandPhase,
    pub go_at: f64,
    pub spot: V3,
    pub home: V3,
    pub tries: f64,
    pub step_t: f64,
    pub t0: f64,
    pub l: f64,
}
impl Errand {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Errand {
            caller: RoomfightDefId(0),
            buddy: RoomfightDefId(0),
            phase: crate::buddy::ErrandPhase::Back,
            go_at: 0.0,
            spot: [0.0; 3],
            home: [0.0; 3],
            tries: 0.0,
            step_t: 0.0,
            t0: 0.0,
            l: 0.0,
        }
    }
}
impl Default for Errand { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ErrandPhase {
    Back,
    Run,
    Beat,
}
impl ErrandPhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            ErrandPhase::Back => "back",
            ErrandPhase::Run => "run",
            ErrandPhase::Beat => "beat",
        }
    }
}

/// (TypeScript: State, src/game/buddy.ts:75)
#[derive(Clone, Debug)]
pub struct State {
    pub men: JsMap<RoomfightDefId, crate::buddy::Man>,
    pub errands: Vec<crate::buddy::Errand>,
    pub stats: crate::buddy::StateStats,
    pub stale: bool,
}
impl State {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        State {
            men: JsMap::new(),
            errands: Vec::new(),
            stats: crate::buddy::StateStats::zeroed(),
            stale: false,
        }
    }
}
impl Default for State { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/buddy.ts:75 State.stats, src/game/buddy.ts:74)
#[derive(Clone, Copy, Debug)]
pub struct StateStats {
    pub calls: f64,
    pub answered: f64,
    pub arrived: f64,
    pub met_on_way: f64,
    pub turned_back: f64,
    pub died_on_way: f64,
    pub short: f64,
    pub no_one: f64,
    pub secs: f64,
    pub metres: f64,
}
impl StateStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateStats {
            calls: 0.0,
            answered: 0.0,
            arrived: 0.0,
            met_on_way: 0.0,
            turned_back: 0.0,
            died_on_way: 0.0,
            short: 0.0,
            no_one: 0.0,
            secs: 0.0,
            metres: 0.0,
        }
    }
}
impl Default for StateStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/buddy.ts:140 answerer ->, src/game/buddy.ts:140)
#[derive(Clone, Copy, Debug)]
pub struct Answerer {
    pub x: RoomfightDefId,
    pub spot: V3,
    pub l: f64,
}
impl Answerer {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Answerer {
            x: RoomfightDefId(0),
            spot: [0.0; 3],
            l: 0.0,
        }
    }
}
impl Default for Answerer { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/buddy.ts:152 send(a), src/game/buddy.ts:152)
#[derive(Clone, Copy, Debug)]
pub struct SendA {
    pub x: RoomfightDefId,
    pub spot: V3,
    pub l: f64,
}
impl SendA {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SendA {
            x: RoomfightDefId(0),
            spot: [0.0; 3],
            l: 0.0,
        }
    }
}
impl Default for SendA { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct BuddyMod {
    /// (src/game/buddy.ts:76 STATES)
    pub states: Option<crate::buddy::State>,
}
impl BuddyMod {
    pub fn blank() -> Self {
        BuddyMod {
            states: None,
        }
    }
}

/// (src/game/buddy.ts:62 BUDDY_DEFAULTS: a table nothing writes)
pub fn buddy_defaults() -> crate::knobs::BuddyK { crate::knobs::page_buddy() }

/// (src/game/buddy.ts:63 LS)
pub const LS: &str = "twopointsix.buddy";

/// (src/game/buddy.ts:65 KEPT: a table nothing writes)
pub fn kept() -> [String; 4] { [String::from("on"), String::from("staged"), String::from("emptyRoom"), String::from("calls")] }
