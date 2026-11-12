//! The types of `pairorders`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct PairOrders {
    pub jobs: JsMap<GuardId, crate::pairorders::Job>,
    pub habit: JsMap<GuardId, [f64; 2]>,
    pub glance: JsMap<GuardId, crate::pairorders::PairOrdersGlanceValue>,
    pub stood: JsMap<GuardId, f64>,
    pub swept: JsMap<GuardId, f64>,
    pub dark: JsMap<GuardId, crate::pairorders::PairOrdersDarkValue>,
    pub dark_memo: JsMap<String, crate::pairorders::PairOrdersDarkMemoValue>,
    pub ways: JsMap<GuardId, crate::pairorders::PairOrdersWaysValue>,
    pub seated: bool,
    pub hands_was: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Job {
    pub foe: GuardId,
    pub mode: crate::pairorders::JobMode,
    pub go: bool,
    pub set: bool,
    pub set_said: bool,
    pub t0: f64,
    pub born: f64,
    pub check_t: f64,
    pub wait_t: f64,
    pub heel_t: f64,
    pub going: bool,
    pub scuffled: bool,
    pub was_following: bool,
    pub handed: Option<bool>,
    pub watcher: Option<GuardId>,
    pub watch_t: f64,
    pub seen_t: f64,
    pub force: bool,
    pub cue_t: f64,
    pub cued: bool,
    pub back_said: Option<bool>,
    pub back_t: Option<f64>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum JobMode {
    #[default] Mark,
    Now,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PairOrdersGlanceValue {
    pub on: bool,
    pub end_t: f64,
    pub h0: Option<[f64; 2]>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PairOrdersDarkValue {
    pub check_t: f64,
    pub lit_t: f64,
    pub said_t: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PairOrdersDarkMemoValue {
    pub t: f64,
    pub spot: Option<V3>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct PairOrdersWaysValue {
    pub to: V3,
    pub path: Vec<V3>,
    pub t: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct PairordersModPAIRLOGItem {
    pub what: String,
    /// dealt at the push: the lesson holds a row as a cursor by it
    pub serial: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum PairOrdersMarkBy {
    Row,
    Hands,
    Api,
    #[word("F")] F,
}

#[derive(Clone, Debug, Default)]
pub struct PairOrdersStalk {
    pub path: Vec<V3>,
    pub exposed: bool,
    pub by: Option<GuardId>,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct PairordersMod {
    pub pair_log: Vec<crate::pairorders::PairordersModPAIRLOGItem>,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct PairordersStats {
    pub marks: i32,
}

