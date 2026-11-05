//! The types of `doorway`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// a walk's price list, nav.findPath's penalty: a cell in the lane of a door of `open` costs doormanners.lane_dear, any other 1. The doors and the knobs are read live from g when priced; `memo` keeps each cell's first answer, by nav.idx, for this one value (doorway.ts doorwayLanePrice)
#[derive(Clone, Debug)]
pub enum DoorwayLanePrice {
    Lanes { open: Vec<DoorId>, memo: JsMap<crate::Num, f64> },
}

/// (TypeScript: St, src/game/doorway.ts:140)
#[derive(Clone, Debug)]
pub struct St {
    pub t: f64,
    pub held_t: f64,
    pub pass: f64,
    pub by: Option<String>,
    pub spot: Option<V3>,
    pub door: Option<DoorId>,
    pub side: f64,
    pub path: Option<u32>,
    pub no_spot: f64,
    pub round_t: f64,
    pub funnel_t: f64,
    pub step: Option<V3>,
    pub step_by: f64,
    pub step_rest: f64,
    pub leaf_x: f64,
    pub leaf_z: f64,
    pub leaf_t: f64,
    pub leaf_seen: f64,
    pub leaf_by: f64,
}
impl St {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        St {
            t: 0.0,
            held_t: 0.0,
            pass: 0.0,
            by: None,
            spot: None,
            door: None,
            side: 0.0,
            path: None,
            no_spot: 0.0,
            round_t: 0.0,
            funnel_t: 0.0,
            step: None,
            step_by: 0.0,
            step_rest: 0.0,
            leaf_x: 0.0,
            leaf_z: 0.0,
            leaf_t: 0.0,
            leaf_seen: 0.0,
            leaf_by: 0.0,
        }
    }
}
impl Default for St { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorway.ts:138 STACK_OWN, src/game/doorway.ts:138)
#[derive(Clone, Copy, Debug)]
pub struct DoorwayModSTACKOWN {
    pub leaf: f64,
    pub mark: f64,
    pub fur: f64,
    pub back: f64,
    pub queue: f64,
}
impl DoorwayModSTACKOWN {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorwayModSTACKOWN {
            leaf: 0.0,
            mark: 0.0,
            fur: 0.0,
            back: 0.0,
            queue: 0.0,
        }
    }
}
impl Default for DoorwayModSTACKOWN { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorway.ts:156 DOORMANNERS_DEBUG, src/game/doorway.ts:156)
#[derive(Clone, Copy, Debug)]
pub struct DOORMANNERSDEBUG {
}
impl DOORMANNERSDEBUG {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DOORMANNERSDEBUG {
        }
    }
}
impl Default for DOORMANNERSDEBUG { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorway.ts:160 rel ->, src/game/doorway.ts:160)
#[derive(Clone, Copy, Debug)]
pub struct Rel {
    pub along: f64,
    pub perp: f64,
    pub f: crate::doorway::RelF,
}
impl Rel {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Rel {
            along: 0.0,
            perp: 0.0,
            f: crate::doorway::RelF::zeroed(),
        }
    }
}
impl Default for Rel { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorway.ts:160 Rel.F, src/game/guards.ts:2013)
#[derive(Clone, Copy, Debug)]
pub struct RelF {
    pub hx: f64,
    pub hz: f64,
    pub ux: f64,
    pub uz: f64,
    pub nx: f64,
    pub nz: f64,
    pub w: f64,
}
impl RelF {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RelF {
            hx: 0.0,
            hz: 0.0,
            ux: 0.0,
            uz: 0.0,
            nx: 0.0,
            nz: 0.0,
            w: 0.0,
        }
    }
}
impl Default for RelF { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorway.ts:264 doorAhead ->, src/game/doorway.ts:264)
#[derive(Clone, Copy, Debug)]
pub struct DoorAhead {
    pub d: DoorId,
    pub side: f64,
    pub at: V3,
    pub dist: f64,
}
impl DoorAhead {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorAhead {
            d: DoorId(0),
            side: 0.0,
            at: [0.0; 3],
            dist: 0.0,
        }
    }
}
impl Default for DoorAhead { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorway.ts:279 friends ->, src/game/doorway.ts:279)
#[derive(Clone, Copy, Debug)]
pub struct FriendsItem {
    pub pos: V3,
    pub vel: V3,
    pub gd: Option<GuardId>,
    pub id: f64,
}
impl FriendsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FriendsItem {
            pos: [0.0; 3],
            vel: [0.0; 3],
            gd: None,
            id: 0.0,
        }
    }
}
impl Default for FriendsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorway.ts:315 through ->, src/game/doorway.ts:315)
#[derive(Clone, Copy, Debug)]
pub struct Through {
    pub dist: f64,
    pub side: f64,
}
impl Through {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Through {
            dist: 0.0,
            side: 0.0,
        }
    }
}
impl Default for Through { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct DoorwayMod {
    /// (src/game/doorway.ts:138 STACK_OWN)
    pub stack_own: crate::doorway::DoorwayModSTACKOWN,
    /// (src/game/doorway.ts:152 states)
    pub states: JsMap<GuardId, crate::doorway::St>,
}
impl DoorwayMod {
    pub fn blank() -> Self {
        DoorwayMod {
            stack_own: crate::doorway::DoorwayModSTACKOWN { leaf: 0.0, mark: 0.0, fur: 0.0, back: 0.0, queue: 0.0 },
            states: JsMap::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct DoorwayStats {
    pub held: i32,
    pub aside: i32,
    pub passed: i32,
    pub stepped: i32,
    pub round: i32,
    pub rest: i32,
    pub marks: i32,
    pub covered: i32,
    pub step_cover: i32,
    pub step_glass: i32,
    pub round_own: i32,
    pub leaf_stall: i32,
}

/// (src/game/doorway.ts:155 LEAFSTALL_BREAK)
pub const LEAFSTALL_BREAK: bool = false;

/// (src/game/doorway.ts:156 DOORMANNERS_DEBUG: a table nothing writes)
pub fn doormanners_debug() -> crate::doorway::DOORMANNERSDEBUG { crate::doorway::DOORMANNERSDEBUG {  } }

/// (src/game/doorway.ts:238 ROUND_PACE)
pub const ROUND_PACE: f64 = 0.5;
