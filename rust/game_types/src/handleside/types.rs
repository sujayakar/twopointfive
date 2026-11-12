//! The types of `handleside`.
use crate::{Blank, bytes::ToBytes, ids::*, js::*, V3};
use alloc::vec::Vec;

#[derive(Clone, Debug, ToBytes, Default)]
pub struct St {
    pub door: DoorId,
    pub side: f64,
    pub spot: V3,
    pub phase: crate::handleside::StPhase,
    pub t: f64,
    pub seen: f64,
    pub way: Option<Vec<V3>>,
    pub at: Option<V3>,
    pub at_t: Option<f64>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum StPhase {
    #[default] Go,
    Wait,
    Thru,
    Turn,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Spot {
    pub pos: V3,
    pub yaw: f64,
    pub pull: bool,
    pub far: Option<bool>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Face {
    pub stand: Option<crate::handleside::Spot>,
    pub via: Option<[V3; 2]>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HandlesideModRestedValue {
    pub door: DoorId,
    pub until: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HandlesideModLinedValue {
    pub door: DoorId,
    pub until: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HandlesideModFacesValue {
    pub swing: f64,
    pub at: [Option<crate::handleside::Face>; 2],
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Ahead {
    pub d: DoorId,
    pub k: usize,
    pub dist: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ToolSpot {
    pub pos: V3,
    pub yaw: f64,
    pub far: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum NearStand {
    List(V3),
    Post,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct HandlesideMod {
    pub states: JsMap<GuardId, crate::handleside::St>,
    pub hands: JsMap<DoorId, GuardId>,
    pub rested: JsMap<GuardId, crate::handleside::HandlesideModRestedValue>,
    pub lined: JsMap<GuardId, crate::handleside::HandlesideModLinedValue>,
    pub faces: JsMap<DoorId, crate::handleside::HandlesideModFacesValue>,
    pub stepped: JsMap<GuardId, f64>,
}

