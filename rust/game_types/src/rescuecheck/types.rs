//! The types of `rescuecheck`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default)]
pub enum Phase {
    #[default] Out,
    Back,
    There,
}
impl Phase {
    pub const fn as_str(self) -> &'static str {
        match self {
            Phase::Out => "out",
            Phase::Back => "back",
            Phase::There => "there",
        }
    }
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

#[derive(Clone, Debug, ToBytes, Default)]
pub struct RescueCheckEvent {
    pub t: f64,
    pub what: crate::rescuecheck::RescueCheckEventWhat,
    pub who: String,
    pub why: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
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
impl RescueCheckEventWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            RescueCheckEventWhat::Lost => "lost",
            RescueCheckEventWhat::Home => "home",
            RescueCheckEventWhat::There => "there",
            RescueCheckEventWhat::Word => "word",
            RescueCheckEventWhat::Found => "found",
            RescueCheckEventWhat::Kept => "kept",
            RescueCheckEventWhat::Empty => "empty",
            RescueCheckEventWhat::Check => "check",
            RescueCheckEventWhat::Asks => "asks",
            RescueCheckEventWhat::Nobody => "nobody",
            RescueCheckEventWhat::Abort => "abort",
            RescueCheckEventWhat::Goes => "goes",
        }
    }
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
    pub events: Vec<crate::rescuecheck::RescueCheckEvent>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum StateStage {
    #[default] Idle,
    Called,
    Asked,
}
impl StateStage {
    pub const fn as_str(self) -> &'static str {
        match self {
            StateStage::Idle => "idle",
            StateStage::Called => "called",
            StateStage::Asked => "asked",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum StateWordedValue {
    Door,
    Noise,
}
impl StateWordedValue {
    pub const fn as_str(self) -> &'static str {
        match self {
            StateWordedValue::Door => "door",
            StateWordedValue::Noise => "noise",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct RescueCheckSnap {
    pub next_at: f64,
    pub n: f64,
    pub stage: crate::rescuecheck::RescueCheckSnapStage,
    pub t0: f64,
    pub caller: f64,
    pub about: f64,
    pub known: Vec<f64>,
    pub where_: Vec<(f64, V3)>,
    pub worded: Vec<(f64, crate::rescuecheck::RescueCheckSnapWordedItem1)>,
    pub kept: Vec<f64>,
    pub out: Option<crate::rescuecheck::RescueCheckSnapOut>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum RescueCheckSnapStage {
    #[default] Idle,
    Called,
    Asked,
}
impl RescueCheckSnapStage {
    pub const fn as_str(self) -> &'static str {
        match self {
            RescueCheckSnapStage::Idle => "idle",
            RescueCheckSnapStage::Called => "called",
            RescueCheckSnapStage::Asked => "asked",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RescueCheckSnapWordedItem1 {
    Door,
    Noise,
}
impl RescueCheckSnapWordedItem1 {
    pub const fn as_str(self) -> &'static str {
        match self {
            RescueCheckSnapWordedItem1::Door => "door",
            RescueCheckSnapWordedItem1::Noise => "noise",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct RescueCheckSnapOut {
    pub i: f64,
    pub about: f64,
    pub goal: V3,
    pub from: V3,
    pub phase: crate::rescuecheck::Phase,
    pub there_t: f64,
    pub tries: f64,
    pub step_t: f64,
    pub heard0: f64,
    pub calm: crate::powerout::CalmSnap,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum EventWhat {
    Lost,
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
impl EventWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            EventWhat::Lost => "lost",
            EventWhat::Home => "home",
            EventWhat::There => "there",
            EventWhat::Word => "word",
            EventWhat::Found => "found",
            EventWhat::Kept => "kept",
            EventWhat::Empty => "empty",
            EventWhat::Check => "check",
            EventWhat::Asks => "asks",
            EventWhat::Nobody => "nobody",
            EventWhat::Abort => "abort",
            EventWhat::Goes => "goes",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Goer {
    pub d: RoomfightDefId,
    pub gd: GuardId,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct RescuecheckMod {
    /// (src/game/rescuecheck.ts:80 STATES)
    pub states: Option<crate::rescuecheck::State>,
}
impl RescuecheckMod {
    pub fn blank() -> Self {
        RescuecheckMod {
            states: None,
        }
    }
}
