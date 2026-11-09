//! The types of `rescuecheck`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, Words, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default)]
pub enum Phase {
    #[default] Out,
    Back,
    There,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Out {
    pub gd: GuardId,
    pub about: GuardId,
    pub goal: V3,
    pub from: V3,
    pub phase: crate::rescuecheck::Phase,
    pub there_t: f64,
    pub tries: f64,
    pub step_t: f64,
    pub heard0: f64,
    pub calm: crate::powerout::Calm,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum RescueCheckEventWhat {
    #[default] Lost,
    Home,
    There,
    Word,
    Found,
    Kept,
    Empty,
    Check,
    Asks,
    Nobody,
    Abort,
    Goes,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CastItem {
    pub gd: GuardId,
    pub role: crate::rescue::Role,
    pub in_room: bool,
    pub post: V3,
    pub learned_at: f64,
    pub released: bool,
    pub crewed: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub next_at: f64,
    pub n: f64,
    pub stage: crate::rescuecheck::StateStage,
    pub t0: f64,
    pub caller: Option<GuardId>,
    pub about: Option<GuardId>,
    pub known: JsSet<GuardId>,
    pub where_: JsMap<GuardId, V3>,
    pub worded: JsMap<GuardId, crate::rescuecheck::StateWordedValue>,
    pub kept: JsSet<GuardId>,
    pub out: Option<crate::rescuecheck::Out>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum StateStage {
    #[default] Idle,
    Called,
    Asked,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum StateWordedValue {
    Door,
    Noise,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Goer {
    pub d: RoomfightDefId,
    pub gd: GuardId,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct RescuecheckMod {
    /// (src/game/rescuecheck.ts:80 STATES)
    pub states: Option<crate::rescuecheck::State>,
}
