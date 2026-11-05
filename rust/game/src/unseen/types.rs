//! The types of `unseen`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: State, src/game/unseen.ts:96)
#[derive(Clone, Debug)]
pub struct State {
    pub t: f64,
    pub live: bool,
    pub sneak_t: f64,
    pub cue_t: f64,
    pub hold: Option<u32>,
    pub sgn: f64,
    pub why: String,
}
impl State {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        State {
            t: 0.0,
            live: false,
            sneak_t: 0.0,
            cue_t: 0.0,
            hold: None,
            sgn: 0.0,
            why: String::new(),
        }
    }
}
impl Default for State { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at quiet State.hold, quiet State.hold)
#[derive(Clone, Copy, Debug)]
pub struct StateHold {
}
impl StateHold {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateHold {
        }
    }
}
impl Default for StateHold { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/unseen.ts:94 UNSEEN_LOG, src/game/unseen.ts:94)
#[derive(Clone, Debug)]
pub struct UnseenModUNSEENLOGItem {
    pub t: f64,
    pub what: crate::unseen::UnseenModUNSEENLOGItemWhat,
    pub why: String,
}
impl UnseenModUNSEENLOGItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        UnseenModUNSEENLOGItem {
            t: 0.0,
            what: crate::unseen::UnseenModUNSEENLOGItemWhat::Off,
            why: String::new(),
        }
    }
}
impl Default for UnseenModUNSEENLOGItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum UnseenModUNSEENLOGItemWhat {
    Off,
    On,
}
impl UnseenModUNSEENLOGItemWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            UnseenModUNSEENLOGItemWhat::Off => "off",
            UnseenModUNSEENLOGItemWhat::On => "on",
        }
    }
}

/// (TypeScript: inline at src/game/unseen.ts:177 placed, src/game/unseen.ts:177 placed)
#[derive(Clone, Copy, Debug)]
pub struct UnseenModPlacedValue {
}
impl UnseenModPlacedValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        UnseenModPlacedValue {
        }
    }
}
impl Default for UnseenModPlacedValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/unseen.ts:162 unseenFollow ->, src/game/unseen.ts:162)
#[derive(Clone, Copy, Debug)]
pub struct UnseenFollow {
    pub speed: f64,
    pub crouch: bool,
}
impl UnseenFollow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        UnseenFollow {
            speed: 0.0,
            crouch: false,
        }
    }
}
impl Default for UnseenFollow { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct UnseenMod {
    /// (src/game/unseen.ts:94 UNSEEN_LOG)
    pub unseen_log: Vec<crate::unseen::UnseenModUNSEENLOGItem>,
    /// (src/game/unseen.ts:102 states)
    pub states: JsMap<PlayerId, crate::unseen::State>,
    /// (src/game/unseen.ts:153 onWallAt)
    pub on_wall_at: JsMap<GuardId, f64>,
    /// (src/game/unseen.ts:154 kneelAt)
    pub kneel_at: JsMap<GuardId, f64>,
    /// (src/game/unseen.ts:177 placed)
    pub placed: JsMap<GuardId, u32>,
}
impl UnseenMod {
    pub fn blank() -> Self {
        UnseenMod {
            unseen_log: Vec::new(),
            states: JsMap::new(),
            on_wall_at: JsMap::new(),
            kneel_at: JsMap::new(),
            placed: JsMap::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct UnseenStats {
    pub spells: i32,
    pub seconds: i32,
    pub held: i32,
    pub held_glass: i32,
    pub wall_places: i32,
    pub wall_lost: i32,
    pub wall_past: i32,
    pub cues: i32,
}

/// (src/game/unseen.ts:88 UNSEEN_DEFAULTS: a table nothing writes)
pub fn unseen_defaults() -> crate::knobs::UnseenK { crate::knobs::page_unseen() }

/// (src/game/unseen.ts:108 WALLBEAT_BREAK)
pub const WALLBEAT_BREAK: bool = false;

/// (src/game/unseen.ts:169 POSTED: a table nothing writes)
pub fn posted() -> Vec<String> { alloc::vec![String::from("move"), String::from("cover")] }
