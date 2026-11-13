//! The types of `doorforce`.
use crate::{Blank, bytes::ToBytes, ids::*, js::*, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Belief {
    pub door: DoorId,
    pub sure: f64,
    pub at: V3,
    pub t: f64,
    pub seen: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Stand {
    pub door: DoorId,
    pub t0: f64,
    pub n: f64,
    pub fired_t: f64,
    pub next: f64,
    pub lay: f64,
    pub fired: f64,
    pub no_shot_t: f64,
    pub to: Option<V3>,
    pub to_key: [f64; 3],
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Kick {
    pub door: DoorId,
    pub spot: V3,
    pub yaw: f64,
    pub phase: crate::doorforce::KickPhase,
    pub t0: f64,
    pub set_t: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default)]
pub enum KickPhase {
    #[default] Kick,
    Set,
    Walk,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Man {
    pub beliefs: JsMap<DoorId, crate::doorforce::Belief>,
    pub saw: Option<crate::doorforce::ManSaw>,
    pub on: Option<crate::doorforce::Stand>,
    pub kick: Option<crate::doorforce::Kick>,
    pub rest: JsMap<DoorId, f64>,
    pub served: f64,
    pub cf_t: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ManSaw {
    pub at: V3,
    pub t: f64,
    pub used: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub last_t: f64,
    pub men: JsMap<GuardId, crate::doorforce::Man>,
    pub kickers: JsMap<DoorId, GuardId>,
    pub boots: JsMap<crate::Num, f64>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorForceHearProp {
    pub direct: bool,
    pub routed: bool,
    pub path_len: f64,
    pub carry: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorForce {
    pub aim: V3,
    pub move_: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct KickStep {
    pub aim: V3,
    pub move_: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct DoorforceMod {
    pub doorforce_log: Vec<String>,
    pub states: Option<crate::doorforce::State>,
}

