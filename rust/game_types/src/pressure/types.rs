//! The types of `pressure`.
use crate::{Blank, bytes::ToBytes, ids::*, js::*};
use alloc::vec::Vec;

/// a man who has given his room up: on his way out, then planted beyond it
#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Out {
    pub door: Option<DoorId>,
    pub def: RoomfightDefId,
    pub moving: bool,
    pub plant_t: f64,
    pub fought: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub outs: Vec<crate::pressure::Out>,
    /// when he gives it up, of a man who is about to
    pub last: JsMap<RoomfightDefId, f64>,
    /// when he last looked for a way out
    pub tried: JsMap<RoomfightDefId, f64>,
    pub ear: JsMap<RoomfightDefId, crate::pressure::StateEarValue>,
}

/// the door of his room he last heard the team's boots at
#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StateEarValue {
    pub t: f64,
    pub i: f64,
    pub at: f64,
}

/// why he gives it up
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LastWhy {
    Door,
    Seen,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct PressureMod {
    pub states: Option<crate::pressure::State>,
    pub kitted: JsSet<GuardId>,
}

pub const ROOM_CELLS: f64 = 500.0;

pub fn pressure_defaults() -> crate::knobs::PressureK { crate::knobs::PressureK::page() }
