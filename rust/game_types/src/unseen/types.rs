//! The types of `unseen`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub t: f64,
    pub live: bool,
    pub sneak_t: f64,
    pub cue_t: f64,
    pub hold: Option<u32>,
    pub sgn: f64,
    pub why: String,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StateHold {
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct UnseenModUNSEENLOGItem {
    pub t: f64,
    pub what: crate::unseen::UnseenModUNSEENLOGItemWhat,
    pub why: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum UnseenModUNSEENLOGItemWhat {
    #[default] Off,
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

#[derive(Clone, Copy, Debug, Default)]
pub struct UnseenModPlacedValue {
}

#[derive(Clone, Copy, Debug, Default)]
pub struct UnseenFollow {
    pub speed: f64,
    pub crouch: bool,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
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
    pub seconds: f64,
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
