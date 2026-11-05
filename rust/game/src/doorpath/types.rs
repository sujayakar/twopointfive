//! The types of `doorpath`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Walk, src/game/doorpath.ts:83)
#[derive(Clone, Debug)]
pub struct Walk {
    pub gd: GuardId,
    pub op: DoorOpId,
    pub door: DoorId,
    pub pod: V3,
    pub n: f64,
    pub t0: f64,
    pub pts: Vec<V3>,
    pub len: Vec<f64>,
    pub s: f64,
    pub laid_t: f64,
    pub near: Vec<crate::doorpath::WalkNearItem>,
    pub lane: f64,
    pub via: Vec<V3>,
    pub arr: Option<u32>,
    pub arr_n: f64,
    pub yield_t: f64,
    pub yields: f64,
    pub held: f64,
    pub off_t: f64,
    pub round: bool,
    pub wait: bool,
    pub waited: f64,
    pub in_m: f64,
    pub pod0: V3,
    pub body_t: f64,
    /// INVENTED by a review: see the proposals
    pub serial: u32,
    /// INVENTED: g.serial() where sent() makes the walk (doorpath.ts:309): mayGo asks by identity whether the man's row is still the walk this door last let go (doorpath.ts:299)
    pub serial_2: u32,
}
impl Walk {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Walk {
            gd: GuardId(0),
            op: DoorOpId(0),
            door: DoorId(0),
            pod: [0.0; 3],
            n: 0.0,
            t0: 0.0,
            pts: Vec::new(),
            len: Vec::new(),
            s: 0.0,
            laid_t: 0.0,
            near: Vec::new(),
            lane: 0.0,
            via: Vec::new(),
            arr: None,
            arr_n: 0.0,
            yield_t: 0.0,
            yields: 0.0,
            held: 0.0,
            off_t: 0.0,
            round: false,
            wait: false,
            waited: 0.0,
            in_m: 0.0,
            pod0: [0.0; 3],
            body_t: 0.0,
            serial: 0,
            serial_2: 0,
        }
    }
}
impl Default for Walk { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorpath.ts:86 Walk.near, src/game/doorpath.ts:86)
#[derive(Clone, Copy, Debug)]
pub struct WalkNearItem {
    pub d: DoorId,
    pub angle: f64,
}
impl WalkNearItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WalkNearItem {
            d: DoorId(0),
            angle: 0.0,
        }
    }
}
impl Default for WalkNearItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: DoorPath, src/game/doorpath.ts:113)
#[derive(Clone, Debug)]
pub struct DoorPath {
    pub walks: JsMap<GuardId, crate::doorpath::Walk>,
    pub last_of: JsMap<DoorOpId, crate::doorpath::DoorPathLastOfValue>,
    pub stats: crate::doorpath::DoorPathStats,
    pub low: JsSet<GuardId>,
    pub lent: Vec<crate::doorpath::DoorPathLentItem>,
}
impl DoorPath {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        DoorPath {
            walks: JsMap::new(),
            last_of: JsMap::new(),
            stats: crate::doorpath::DoorPathStats { sent: 0.0, walked: 0.0, round: 0.0, no_round: 0.0, no_lane: 0.0, late: 0.0, held_s: 0.0, capped: 0.0, yields: 0.0, gave_up: 0.0, taken: 0.0, relaid: 0.0, turned: 0.0, moved: 0.0, wait_s: 0.0, nav: 0.0, gun_s: 0.0, guns: 0.0, gun_ahead: 0.0, stepped: 0.0 },
            low: JsSet::new(),
            lent: Vec::new(),
        }
    }
}
impl Default for DoorPath { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/doorpath.ts:116 DoorPath.lastOf, src/game/doorpath.ts:116)
#[derive(Clone, Copy, Debug)]
pub struct DoorPathLastOfValue {
    pub w: Option<(GuardId, u32)>,
    pub n: f64,
    pub t: f64,
}
impl DoorPathLastOfValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorPathLastOfValue {
            w: None,
            n: 0.0,
            t: 0.0,
        }
    }
}
impl Default for DoorPathLastOfValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorpath.ts:117 DoorPath.stats, src/game/doorpath.ts:117)
#[derive(Clone, Copy, Debug)]
pub struct DoorPathStats {
    pub sent: f64,
    pub walked: f64,
    pub round: f64,
    pub no_round: f64,
    pub no_lane: f64,
    pub late: f64,
    pub held_s: f64,
    pub capped: f64,
    pub yields: f64,
    pub gave_up: f64,
    pub taken: f64,
    pub relaid: f64,
    pub turned: f64,
    pub moved: f64,
    pub wait_s: f64,
    pub nav: f64,
    pub gun_s: f64,
    pub guns: f64,
    pub gun_ahead: f64,
    pub stepped: f64,
}
impl DoorPathStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorPathStats {
            sent: 0.0,
            walked: 0.0,
            round: 0.0,
            no_round: 0.0,
            no_lane: 0.0,
            late: 0.0,
            held_s: 0.0,
            capped: 0.0,
            yields: 0.0,
            gave_up: 0.0,
            taken: 0.0,
            relaid: 0.0,
            turned: 0.0,
            moved: 0.0,
            wait_s: 0.0,
            nav: 0.0,
            gun_s: 0.0,
            guns: 0.0,
            gun_ahead: 0.0,
            stepped: 0.0,
        }
    }
}
impl Default for DoorPathStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorpath.ts:124 DoorPath.lent, src/game/doorpath.ts:124)
#[derive(Clone, Copy, Debug)]
pub struct DoorPathLentItem {
    pub gd: GuardId,
    pub s: u32,
    pub mine: V3,
    pub aim_at: Option<V3>,
    pub face: Option<f64>,
    pub strafe: Option<bool>,
}
impl DoorPathLentItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorPathLentItem {
            gd: GuardId(0),
            s: 0,
            mine: [0.0; 3],
            aim_at: None,
            face: None,
            strafe: None,
        }
    }
}
impl Default for DoorPathLentItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorpath.ts:195 roundLeaves ->, src/game/doorpath.ts:195)
#[derive(Clone, Debug)]
pub struct DoorPathRoundLeaves {
    pub pts: Vec<V3>,
    pub round: bool,
}
impl DoorPathRoundLeaves {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorPathRoundLeaves {
            pts: Vec::new(),
            round: false,
        }
    }
}
impl Default for DoorPathRoundLeaves { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorpath.ts:217 way ->, src/game/doorpath.ts:217)
#[derive(Clone, Debug)]
pub struct DoorPathWay {
    pub pts: Vec<V3>,
    pub round: bool,
}
impl DoorPathWay {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorPathWay {
            pts: Vec::new(),
            round: false,
        }
    }
}
impl Default for DoorPathWay { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorpath.ts:296 mayGo(q), src/game/doorpath.ts:296)
#[derive(Clone, Copy, Debug)]
pub struct DoorPathMayGoQ {
    pub gd: GuardId,
    pub pod: crate::doorpath::DoorPathMayGoQPod,
}
impl DoorPathMayGoQ {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorPathMayGoQ {
            gd: GuardId(0),
            pod: crate::doorpath::DoorPathMayGoQPod::zeroed(),
        }
    }
}
impl Default for DoorPathMayGoQ { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorpath.ts:296 DoorPathMayGoQ.pod, src/game/doorpath.ts:296)
#[derive(Clone, Copy, Debug)]
pub struct DoorPathMayGoQPod {
    pub pos: V3,
}
impl DoorPathMayGoQPod {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorPathMayGoQPod {
            pos: [0.0; 3],
        }
    }
}
impl Default for DoorPathMayGoQPod { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorpath.ts:307 sent(pod), src/game/doorpath.ts:307)
#[derive(Clone, Copy, Debug)]
pub struct DoorPathSentPod {
    pub pos: V3,
}
impl DoorPathSentPod {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorPathSentPod {
            pos: [0.0; 3],
        }
    }
}
impl Default for DoorPathSentPod { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DoorPathDropWhy {
    Taken,
    GaveUp,
}
impl DoorPathDropWhy {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoorPathDropWhy::Taken => "taken",
            DoorPathDropWhy::GaveUp => "gaveUp",
        }
    }
}
