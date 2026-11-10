//! The types of `showwhy`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default)]
pub enum Why {
    #[default] Door,
    Stack,
    Line,
    Keep,
    Hurt,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ShowWhyStats {
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Owed {
    pub text: String,
    pub why: crate::showwhy::Why,
    pub by: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Mem {
    pub t: f64,
    pub line_t0: f64,
    pub line_seen_t: f64,
    pub line_on: bool,
    pub line_end_t: f64,
    pub wave_t: f64,
    pub waves: i32,
    pub line_b: f64,
    pub door_t0: f64,
    pub door_seen_t: f64,
    pub door_shown: bool,
    pub door_t: f64,
    pub hurt_said: f64,
    pub lag_t: f64,
    pub lag_n: i32,
    pub keep_t0: f64,
    pub keep_said: bool,
    pub keep_said_t: f64,
    pub owed: Vec<crate::showwhy::Owed>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct OpMem {
    pub t: f64,
    pub turns: Vec<crate::showwhy::OpMemTurnsItem>,
    pub fist_t: f64,
    pub fists: i32,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct OpMemTurnsItem {
    pub gd: GuardId,
    pub at: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub stats: crate::showwhy::ShowWhyStats,
    pub after_you_t: f64,
    pub keep_say_t: f64,
    pub line_say_t: f64,
    pub t: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct ShowwhyMod {
    pub mem: JsMap<GuardId, crate::showwhy::Mem>,
    pub op_mem: JsMap<DoorOpId, crate::showwhy::OpMem>,
    pub states: JsMap<PlayerId, crate::showwhy::State>,
}
