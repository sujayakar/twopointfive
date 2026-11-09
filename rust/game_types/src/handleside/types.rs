//! The types of `handleside`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

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
    pub at: V3,
    pub dist: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BendA {
    pub k: usize,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ToolSpot {
    pub pos: V3,
    pub yaw: f64,
    pub pull: bool,
    pub far: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum NearStand {
    List(V3),
    Post,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct HandlesideMod {
    /// (src/game/handleside.ts:69 states)
    pub states: JsMap<GuardId, crate::handleside::St>,
    /// (src/game/handleside.ts:71 hands)
    pub hands: JsMap<DoorId, GuardId>,
    /// (src/game/handleside.ts:72 rested)
    pub rested: JsMap<GuardId, crate::handleside::HandlesideModRestedValue>,
    /// (src/game/handleside.ts:73 lined)
    pub lined: JsMap<GuardId, crate::handleside::HandlesideModLinedValue>,
    /// (src/game/handleside.ts:78 faces)
    pub faces: JsMap<DoorId, crate::handleside::HandlesideModFacesValue>,
    /// (src/game/handleside.ts:355 stepped)
    pub stepped: JsMap<GuardId, f64>,
}
impl HandlesideMod {
    pub fn blank() -> Self {
        HandlesideMod {
            states: JsMap::new(),
            hands: JsMap::new(),
            rested: JsMap::new(),
            lined: JsMap::new(),
            faces: JsMap::new(),
            stepped: JsMap::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct HandlesideStats {
    pub bent: i32,
    pub lined: i32,
    pub pulls: i32,
    pub far_pulls: i32,
    pub no_spot: i32,
    pub gave_up: i32,
    pub held: i32,
    pub tool_walks: i32,
    pub tool_no_spot: i32,
    pub tool_nears: i32,
    pub tool_posts: i32,
}

