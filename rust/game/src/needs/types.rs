//! The types of `needs`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum NeedKind {
    Smoke,
    Coffee,
    Water,
    Stretch,
}
impl NeedKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            NeedKind::Smoke => "smoke",
            NeedKind::Coffee => "coffee",
            NeedKind::Water => "water",
            NeedKind::Stretch => "stretch",
        }
    }
}

/// (TypeScript: NeedsSnap, src/game/needs.ts:407)
#[derive(Clone, Debug)]
pub struct NeedsSnap {
    pub men: Vec<Option<crate::needs::NeedsManSnap>>,
    pub last_depart_at: f64,
}
impl NeedsSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NeedsSnap {
            men: Vec::new(),
            last_depart_at: 0.0,
        }
    }
}
impl Default for NeedsSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: NeedsManSnap, src/game/needs.ts:406)
#[derive(Clone, Debug)]
pub struct NeedsManSnap {
    pub needs: Vec<crate::needs::Need>,
    pub trip: Option<crate::needs::NeedsManSnapTrip>,
    pub last_back_at: f64,
    pub trips: f64,
    pub cancels: f64,
    pub shaken: bool,
    pub dealt: bool,
}
impl NeedsManSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NeedsManSnap {
            needs: Vec::new(),
            trip: None,
            last_back_at: 0.0,
            trips: 0.0,
            cancels: 0.0,
            shaken: false,
            dealt: false,
        }
    }
}
impl Default for NeedsManSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Need, src/game/needs.ts:92)
#[derive(Clone, Copy, Debug)]
pub struct Need {
    pub kind: crate::needs::NeedKind,
    pub level: f64,
    pub fill_min: f64,
    pub wp: f64,
    pub trip_m: f64,
}
impl Need {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Need {
            kind: crate::needs::NeedKind::Smoke,
            level: 0.0,
            fill_min: 0.0,
            wp: 0.0,
            trip_m: 0.0,
        }
    }
}
impl Default for Need { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/needs.ts:406 NeedsManSnap.trip, src/game/needs.ts:406 NeedsManSnap.trip)
#[derive(Clone, Debug)]
pub struct NeedsManSnapTrip {
    pub kind: crate::needs::NeedKind,
    pub radio: bool,
    pub t0: f64,
    pub wp: f64,
    pub phase: crate::needs::TripPhase,
    pub line: String,
    pub told: bool,
    pub depart_at: f64,
    pub watched: bool,
    pub replied: bool,
    pub necked: bool,
    pub forced: bool,
    pub left_at: f64,
    pub arrived_at: f64,
    pub dwell_until: f64,
    pub back_at: f64,
    pub out_yaw: f64,
    pub reply_at: f64,
    pub loop_i: f64,
    pub gap_before: f64,
    pub partner: f64,
}
impl NeedsManSnapTrip {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NeedsManSnapTrip {
            kind: crate::needs::NeedKind::Smoke,
            radio: false,
            t0: 0.0,
            wp: 0.0,
            phase: crate::needs::TripPhase::Going,
            line: String::new(),
            told: false,
            depart_at: 0.0,
            watched: false,
            replied: false,
            necked: false,
            forced: false,
            left_at: 0.0,
            arrived_at: 0.0,
            dwell_until: 0.0,
            back_at: 0.0,
            out_yaw: 0.0,
            reply_at: 0.0,
            loop_i: 0.0,
            gap_before: 0.0,
            partner: 0.0,
        }
    }
}
impl Default for NeedsManSnapTrip { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TripPhase {
    Going,
    Telegraph,
    Dwell,
    Returning,
}
impl TripPhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            TripPhase::Going => "going",
            TripPhase::Telegraph => "telegraph",
            TripPhase::Dwell => "dwell",
            TripPhase::Returning => "returning",
        }
    }
}

/// (TypeScript: NeedSpot, src/game/needs.ts:37)
#[derive(Clone, Debug)]
pub struct NeedSpot {
    pub kind: crate::needs::NeedKind,
    pub name: String,
    pub pos: V3,
    pub face: f64,
    pub loop_: Option<Vec<V3>>,
}
impl NeedSpot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NeedSpot {
            kind: crate::needs::NeedKind::Smoke,
            name: String::new(),
            pos: [0.0; 3],
            face: 0.0,
            loop_: None,
        }
    }
}
impl Default for NeedSpot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Trip, src/game/needs.ts:94)
#[derive(Clone, Debug)]
pub struct Trip {
    pub kind: crate::needs::NeedKind,
    pub spot: crate::needs::NeedSpot,
    pub phase: crate::needs::TripPhase,
    pub forced: bool,
    pub t0: f64,
    pub depart_at: f64,
    pub left_at: f64,
    pub arrived_at: f64,
    pub dwell_until: f64,
    pub back_at: f64,
    pub wp: f64,
    pub out_yaw: f64,
    pub watched: bool,
    pub told: bool,
    pub partner: Option<GuardId>,
    pub radio: bool,
    pub reply_at: f64,
    pub replied: bool,
    pub necked: bool,
    pub loop_i: f64,
    pub line: String,
    pub gap_before: f64,
}
impl Trip {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Trip {
            kind: crate::needs::NeedKind::Smoke,
            spot: crate::needs::NeedSpot::zeroed(),
            phase: crate::needs::TripPhase::Going,
            forced: false,
            t0: 0.0,
            depart_at: 0.0,
            left_at: 0.0,
            arrived_at: 0.0,
            dwell_until: 0.0,
            back_at: 0.0,
            wp: 0.0,
            out_yaw: 0.0,
            watched: false,
            told: false,
            partner: None,
            radio: false,
            reply_at: 0.0,
            replied: false,
            necked: false,
            loop_i: 0.0,
            line: String::new(),
            gap_before: 0.0,
        }
    }
}
impl Default for Trip { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: NeedMem, src/game/needs.ts:109)
#[derive(Clone, Debug)]
pub struct NeedMem {
    pub needs: Vec<crate::needs::Need>,
    pub trip: Option<crate::needs::Trip>,
    pub last_back_at: f64,
    pub trips: f64,
    pub cancels: f64,
    pub shaken: bool,
    pub dealt: bool,
}
impl NeedMem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NeedMem {
            needs: Vec::new(),
            trip: None,
            last_back_at: 0.0,
            trips: 0.0,
            cancels: 0.0,
            shaken: false,
            dealt: false,
        }
    }
}
impl Default for NeedMem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/needs.ts:81 NEEDS_DEFAULTS, src/game/needs.ts:81)
#[derive(Clone, Copy, Debug)]
pub struct NEEDSDEFAULTS {
    pub rate_k: f64,
    pub max_away: f64,
    pub announce_lead: f64,
    pub dwell: crate::needs::NEEDSDEFAULTSDwell,
}
impl NEEDSDEFAULTS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NEEDSDEFAULTS {
            rate_k: 0.0,
            max_away: 0.0,
            announce_lead: 0.0,
            dwell: crate::needs::NEEDSDEFAULTSDwell::zeroed(),
        }
    }
}
impl Default for NEEDSDEFAULTS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/needs.ts:81 NEEDSDEFAULTS.dwell, src/game/needs.ts:81)
#[derive(Clone, Copy, Debug)]
pub struct NEEDSDEFAULTSDwell {
    pub smoke: f64,
    pub coffee: f64,
    pub water: f64,
    pub stretch: f64,
}
impl NEEDSDEFAULTSDwell {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NEEDSDEFAULTSDwell {
            smoke: 0.0,
            coffee: 0.0,
            water: 0.0,
            stretch: 0.0,
        }
    }
}
impl Default for NEEDSDEFAULTSDwell { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/needs.ts:120 floorOf, src/game/needs.ts:120)
#[derive(Clone, Copy, Debug)]
pub struct NeedsModFloorOfValue {
    pub last_depart_at: f64,
}
impl NeedsModFloorOfValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NeedsModFloorOfValue {
            last_depart_at: 0.0,
        }
    }
}
impl Default for NeedsModFloorOfValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/needs.ts:125 NEEDS_LOG, src/game/needs.ts:125)
#[derive(Clone, Debug)]
pub struct NeedsModNEEDSLOGItem {
    pub t: f64,
    pub who: String,
    pub what: String,
}
impl NeedsModNEEDSLOGItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NeedsModNEEDSLOGItem {
            t: 0.0,
            who: String::new(),
            what: String::new(),
        }
    }
}
impl Default for NeedsModNEEDSLOGItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/needs.ts:126 NEEDS_DEBUG, src/game/needs.ts:126)
#[derive(Clone, Copy, Debug)]
pub struct NEEDSDEBUG {
}
impl NEEDSDEBUG {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NEEDSDEBUG {
        }
    }
}
impl Default for NEEDSDEBUG { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/needs.ts:53 ownSpot(s), src/game/needs.ts:53)
#[derive(Clone, Debug)]
pub struct OwnSpotS {
    pub name: String,
    pub pos: V3,
    pub face: f64,
    pub loop_: Option<Vec<V3>>,
}
impl OwnSpotS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OwnSpotS {
            name: String::new(),
            pos: [0.0; 3],
            face: 0.0,
            loop_: None,
        }
    }
}
impl Default for OwnSpotS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/needs.ts:113 needsMem ->, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1597)
#[derive(Clone, Debug)]
pub struct NeedsMem {
    pub needs: Vec<crate::needs::Need>,
    pub trip: Option<crate::needs::Trip>,
    pub last_back_at: f64,
    pub trips: f64,
    pub cancels: f64,
    pub shaken: bool,
    pub dealt: bool,
}
impl NeedsMem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NeedsMem {
            needs: Vec::new(),
            trip: None,
            last_back_at: 0.0,
            trips: 0.0,
            cancels: 0.0,
            shaken: false,
            dealt: false,
        }
    }
}
impl Default for NeedsMem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/needs.ts:115 needsTrip ->, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1597)
#[derive(Clone, Debug)]
pub struct NeedsTrip {
    pub kind: crate::needs::NeedKind,
    pub spot: crate::needs::NeedSpot,
    pub phase: crate::needs::TripPhase,
    pub forced: bool,
    pub t0: f64,
    pub depart_at: f64,
    pub left_at: f64,
    pub arrived_at: f64,
    pub dwell_until: f64,
    pub back_at: f64,
    pub wp: f64,
    pub out_yaw: f64,
    pub watched: bool,
    pub told: bool,
    pub partner: Option<GuardId>,
    pub radio: bool,
    pub reply_at: f64,
    pub replied: bool,
    pub necked: bool,
    pub loop_i: f64,
    pub line: String,
    pub gap_before: f64,
}
impl NeedsTrip {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NeedsTrip {
            kind: crate::needs::NeedKind::Smoke,
            spot: crate::needs::NeedSpot::zeroed(),
            phase: crate::needs::TripPhase::Going,
            forced: false,
            t0: 0.0,
            depart_at: 0.0,
            left_at: 0.0,
            arrived_at: 0.0,
            dwell_until: 0.0,
            back_at: 0.0,
            wp: 0.0,
            out_yaw: 0.0,
            watched: false,
            told: false,
            partner: None,
            radio: false,
            reply_at: 0.0,
            replied: false,
            necked: false,
            loop_i: 0.0,
            line: String::new(),
            gap_before: 0.0,
        }
    }
}
impl Default for NeedsTrip { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/needs.ts:137 departurePoint ->, src/game/needs.ts:137)
#[derive(Clone, Copy, Debug)]
pub struct DeparturePoint {
    pub wp: f64,
    pub m: f64,
}
impl DeparturePoint {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DeparturePoint {
            wp: 0.0,
            m: 0.0,
        }
    }
}
impl Default for DeparturePoint { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/needs.ts:384 saveNeeds(tune), src/game/needs.ts:384)
#[derive(Clone, Copy, Debug)]
pub struct SaveNeedsTune {
    pub needs: bool,
}
impl SaveNeedsTune {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SaveNeedsTune {
            needs: false,
        }
    }
}
impl Default for SaveNeedsTune { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/needs.ts:388 loadNeeds(tune), src/game/needs.ts:388)
#[derive(Clone, Copy, Debug)]
pub struct LoadNeedsTune {
    pub needs: bool,
}
impl LoadNeedsTune {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LoadNeedsTune {
            needs: false,
        }
    }
}
impl Default for LoadNeedsTune { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/needs.ts:399 resetNeeds(tune), src/game/needs.ts:399)
#[derive(Clone, Copy, Debug)]
pub struct ResetNeedsTune {
    pub needs: bool,
}
impl ResetNeedsTune {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ResetNeedsTune {
            needs: false,
        }
    }
}
impl Default for ResetNeedsTune { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct NeedsMod {
    /// (src/game/needs.ts:52 owned)
    pub owned: JsMap<crate::needs::NeedKind, crate::needs::NeedSpot>,
    /// (src/game/needs.ts:110 memOf)
    pub mem_of: JsMap<GuardId, crate::needs::NeedMem>,
    /// (src/game/needs.ts:120 floorOf)
    pub floor_of: Option<crate::needs::NeedsModFloorOfValue>,
    /// (src/game/needs.ts:125 NEEDS_LOG)
    pub needs_log: Vec<crate::needs::NeedsModNEEDSLOGItem>,
}
impl NeedsMod {
    pub fn blank() -> Self {
        NeedsMod {
            owned: JsMap::new(),
            mem_of: JsMap::new(),
            floor_of: None,
            needs_log: Vec::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct NeedsStats {
    pub dealt: i32,
    pub scheduled: i32,
    pub forced: i32,
    pub departed: i32,
    pub arrived: i32,
    pub completed: i32,
    pub cancelled: i32,
    pub radioed: i32,
    pub told: i32,
    pub replies: i32,
    pub stuck: i32,
}

/// (src/game/needs.ts:34 NEED_KINDS: a table nothing writes)
pub fn need_kinds() -> Vec<String> { alloc::vec![String::from("coffee"), String::from("smoke"), String::from("water"), String::from("stretch")] }

/// (src/game/needs.ts:38 NEED_SPOTS: a table nothing writes)
pub fn need_spots() -> JsMap<String, crate::needs::NeedSpot> { { let mut t = JsMap::new(); t.set(String::from("coffee"), crate::needs::NeedSpot { kind: crate::needs::NeedKind::Coffee, name: String::from("the break-room counter"), pos: [34.6, 0.0, 14.4], face: 1.5707963267948966, loop_: None }); t.set(String::from("smoke"), crate::needs::NeedSpot { kind: crate::needs::NeedKind::Smoke, name: String::from("outside the fire exit"), pos: [37.6, 0.0, 22.4], face: 1.1, loop_: None }); t.set(String::from("water"), crate::needs::NeedSpot { kind: crate::needs::NeedKind::Water, name: String::from("the water cooler"), pos: [26.9, 0.0, 22.8], face: 0.7853981633974483, loop_: None }); t.set(String::from("stretch"), crate::needs::NeedSpot { kind: crate::needs::NeedKind::Stretch, name: String::from("the lobby"), pos: [8.4, 0.0, 17.0], face: 0.0, loop_: Some(alloc::vec![[10.8, 0.0, 18.6], [9.6, 0.0, 21.4], [7.4, 0.0, 22.8], [5.4, 0.0, 18.4], [8.4, 0.0, 17.0]]) }); t } }

/// (src/game/needs.ts:81 NEEDS_DEFAULTS: a table nothing writes)
pub fn needs_defaults() -> crate::needs::NEEDSDEFAULTS { crate::needs::NEEDSDEFAULTS { rate_k: 1.0, max_away: 1.0, announce_lead: 20.0, dwell: crate::needs::NEEDSDEFAULTSDwell { smoke: 40.0, coffee: 25.0, water: 12.0, stretch: 0.0 } } }

/// (src/game/needs.ts:84 FILL_MIN: a table nothing writes)
pub fn fill_min() -> JsMap<String, [f64; 2]> { { let mut t = JsMap::new(); t.set(String::from("coffee"), [6.0, 9.0]); t.set(String::from("smoke"), [8.0, 12.0]); t.set(String::from("water"), [6.0, 10.0]); t.set(String::from("stretch"), [5.0, 7.0]); t } }

/// (src/game/needs.ts:85 LEVEL0: a table nothing writes)
pub fn level0() -> Vec<f64> { alloc::vec![0.3, 0.7] }

/// (src/game/needs.ts:87 LEVEL_CAP)
pub const LEVEL_CAP: f64 = 1.5;

/// (src/game/needs.ts:87 CALM_AWARENESS)
pub const CALM_AWARENESS: f64 = 0.3;

/// (src/game/needs.ts:87 SCHEDULE_AWARENESS)
pub const SCHEDULE_AWARENESS: f64 = 0.15;

/// (src/game/needs.ts:89 GLANCE_AT)
pub const GLANCE_AT: f64 = 1.1;

/// (src/game/needs.ts:89 GLANCE_SECS)
pub const GLANCE_SECS: f64 = 2.2;

/// (src/game/needs.ts:126 NEEDS_DEBUG: a table nothing writes)
pub fn needs_debug() -> crate::needs::NEEDSDEBUG { crate::needs::NEEDSDEBUG {  } }

/// (src/game/needs.ts:383 LS_KEY)
pub const LS_KEY: &str = "twopointsix.needs";
