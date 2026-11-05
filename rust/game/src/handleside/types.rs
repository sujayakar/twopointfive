//! The types of `handleside`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: St, src/game/handleside.ts:68)
#[derive(Clone, Debug)]
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
impl St {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        St {
            door: DoorId(0),
            side: 0.0,
            spot: [0.0; 3],
            phase: crate::handleside::StPhase::Go,
            t: 0.0,
            seen: 0.0,
            way: None,
            at: None,
            at_t: None,
        }
    }
}
impl Default for St { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum StPhase {
    Go,
    Wait,
    Thru,
    Turn,
}
impl StPhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            StPhase::Go => "go",
            StPhase::Wait => "wait",
            StPhase::Thru => "thru",
            StPhase::Turn => "turn",
        }
    }
}

/// (TypeScript: Spot, src/game/handleside.ts:76)
#[derive(Clone, Copy, Debug)]
pub struct Spot {
    pub pos: V3,
    pub yaw: f64,
    pub pull: bool,
    pub far: Option<bool>,
}
impl Spot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Spot {
            pos: [0.0; 3],
            yaw: 0.0,
            pull: false,
            far: None,
        }
    }
}
impl Default for Spot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Face, src/game/handleside.ts:77)
#[derive(Clone, Copy, Debug)]
pub struct Face {
    pub stand: Option<crate::handleside::Spot>,
    pub via: Option<[V3; 2]>,
}
impl Face {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Face {
            stand: None,
            via: None,
        }
    }
}
impl Default for Face { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/handleside.ts:66 HANDLE_DEBUG, src/game/handleside.ts:66)
#[derive(Clone, Copy, Debug)]
pub struct HANDLEDEBUG {
}
impl HANDLEDEBUG {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HANDLEDEBUG {
        }
    }
}
impl Default for HANDLEDEBUG { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/handleside.ts:72 rested, src/game/handleside.ts:72)
#[derive(Clone, Copy, Debug)]
pub struct HandlesideModRestedValue {
    pub door: DoorId,
    pub until: f64,
}
impl HandlesideModRestedValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HandlesideModRestedValue {
            door: DoorId(0),
            until: 0.0,
        }
    }
}
impl Default for HandlesideModRestedValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/handleside.ts:73 lined, src/game/handleside.ts:73)
#[derive(Clone, Copy, Debug)]
pub struct HandlesideModLinedValue {
    pub door: DoorId,
    pub until: f64,
}
impl HandlesideModLinedValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HandlesideModLinedValue {
            door: DoorId(0),
            until: 0.0,
        }
    }
}
impl Default for HandlesideModLinedValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/handleside.ts:78 faces, src/game/handleside.ts:78)
#[derive(Clone, Copy, Debug)]
pub struct HandlesideModFacesValue {
    pub swing: f64,
    pub at: [Option<crate::handleside::Face>; 2],
}
impl HandlesideModFacesValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HandlesideModFacesValue {
            swing: 0.0,
            at: [None; 2],
        }
    }
}
impl Default for HandlesideModFacesValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/handleside.ts:155 ahead ->, src/game/handleside.ts:155)
#[derive(Clone, Copy, Debug)]
pub struct Ahead {
    pub d: DoorId,
    pub k: f64,
    pub at: V3,
    pub dist: f64,
}
impl Ahead {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Ahead {
            d: DoorId(0),
            k: 0.0,
            at: [0.0; 3],
            dist: 0.0,
        }
    }
}
impl Default for Ahead { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/handleside.ts:171 bend(A), src/game/handleside.ts:171)
#[derive(Clone, Copy, Debug)]
pub struct BendA {
    pub k: f64,
}
impl BendA {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BendA {
            k: 0.0,
        }
    }
}
impl Default for BendA { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/handleside.ts:315 toolSpot ->, src/game/handleside.ts:315)
#[derive(Clone, Copy, Debug)]
pub struct ToolSpot {
    pub pos: V3,
    pub yaw: f64,
    pub pull: bool,
    pub far: bool,
}
impl ToolSpot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ToolSpot {
            pos: [0.0; 3],
            yaw: 0.0,
            pull: false,
            far: false,
        }
    }
}
impl Default for ToolSpot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum NearStand {
    List(V3),
    Post,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
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
    /// (src/game/handleside.ts:186 feet)
    pub feet: JsMap<GuardId, f64>,
    /// (src/game/handleside.ts:357 stepped)
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
            feet: JsMap::new(),
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

/// (src/game/handleside.ts:66 HANDLE_DEBUG: a table nothing writes)
pub fn handle_debug() -> crate::handleside::HANDLEDEBUG { crate::handleside::HANDLEDEBUG {  } }
