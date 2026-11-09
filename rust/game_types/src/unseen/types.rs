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
pub struct UnseenFollow {
    pub speed: f64,
    pub crouch: bool,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct UnseenMod {
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

