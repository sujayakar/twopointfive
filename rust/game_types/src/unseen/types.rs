//! The types of `unseen`.
use crate::{Blank, bytes::ToBytes, ids::*, js::*};
use alloc::string::String;

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
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct UnseenMod {
    pub states: JsMap<GroupId, crate::unseen::State>,
    pub on_wall_at: JsMap<GuardId, f64>,
    pub kneel_at: JsMap<GuardId, f64>,
    pub placed: JsMap<GuardId, u32>,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct UnseenStats {
    pub cues: i32,
}

