//! The types of `morale`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum Training {
    #[default] None,
    Trained,
    Green,
    Difficulty,
    Mixed,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum Nerve {
    #[default] Steady,
    Normal,
    Brittle,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum MoraleAct {
    #[default] Down,
    Reload,
    Burst,
    None,
    Back,
    Hold,
    Stand,
    Flinch,
    Duck,
    Bait,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Mind {
    pub shock: f64,
    pub alone: bool,
    pub dark: bool,
    pub lit: f64,
    pub ask: f64,
    pub rung: f64,
    pub green: bool,
    pub leader_down: bool,
    pub fall: f64,
    pub lost: bool,
    pub by: Option<GuardId>,
    pub heard: f64,
    pub said: bool,
    pub broke_t: f64,
    pub act: crate::morale::MoraleAct,
    pub t: f64,
    pub hole: Option<V3>,
    pub goal: Option<V3>,
    pub rounds: f64,
    pub next: f64,
    pub calls: f64,
    pub call_t: f64,
    pub baits: f64,
    pub at_hole: bool,
    pub bark: f64,
    pub shout: bool,
    pub drive: bool,
    pub drive_t: f64,
    pub script: crate::game::GuardScript,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Blast {
    pub at: V3,
    pub t: f64,
    pub who: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Words)]
pub enum LineKind {
    Clear,
    Out,
    Lost,
    Spent,
    Order,
    Again,
    Duck,
    Call,
    Okay,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct MoraleModMixAtValue {
    pub key: String,
    pub mix: crate::morale::MoraleModMixAtValueMix,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum MoraleModMixAtValueMix {
    #[default] Hard,
    Mixed,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum MixNow {
    Hard,
    Mixed,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TrainingOf {
    Steady,
    Green,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MoraleOf {
    pub t: f64,
    pub act: crate::morale::MoraleAct,
    pub alone: bool,
    pub green: bool,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct MoraleMod {
    pub state_of: JsMap<GuardId, crate::morale::Mind>,
    pub blasts_of: Option<Vec<crate::morale::Blast>>,
    pub mix_at: Option<crate::morale::MoraleModMixAtValue>,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct MoraleStats {
    pub orders: i32,
    pub falls: i32,
}

pub fn morale_at() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("standard"), String::from("mixed")); t.set(String::from("hard"), String::from("hard")); t } }

pub const ALERT_AW: f64 = 0.85;

