//! The types of `cover`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum OpeningKind {
    Door,
    Gap,
    Mouth,
    Rear,
}
impl OpeningKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            OpeningKind::Door => "door",
            OpeningKind::Gap => "gap",
            OpeningKind::Mouth => "mouth",
            OpeningKind::Rear => "rear",
        }
    }
}

/// (TypeScript: Opening, src/game/cover.ts:176)
#[derive(Clone, Debug)]
pub struct Opening {
    pub id: String,
    pub kind: crate::cover::OpeningKind,
    pub at: V3,
    pub n: [f64; 2],
    pub half: f64,
    pub door: Option<DoorId>,
    pub pass: Option<String>,
    pub far: Option<V3>,
}
impl Opening {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Opening {
            id: String::new(),
            kind: crate::cover::OpeningKind::Door,
            at: [0.0; 3],
            n: [0.0; 2],
            half: 0.0,
            door: None,
            pass: None,
            far: None,
        }
    }
}
impl Default for Opening { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: CoverJob, src/game/cover.ts:181)
#[derive(Clone, Copy, Debug)]
pub struct CoverJob {
    pub o: CoverOpeningId,
    pub to: V3,
    pub aim: V3,
    pub yaw: f64,
    pub r: f64,
    pub t0: f64,
    pub aim_t: f64,
    pub back: bool,
    pub lane_t: f64,
    pub off: bool,
    pub pace: f64,
    pub a: OrdersAssignmentId,
}
impl CoverJob {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CoverJob {
            o: CoverOpeningId(0),
            to: [0.0; 3],
            aim: [0.0; 3],
            yaw: 0.0,
            r: 0.0,
            t0: 0.0,
            aim_t: 0.0,
            back: false,
            lane_t: 0.0,
            off: false,
            pace: 0.0,
            a: OrdersAssignmentId(0),
        }
    }
}
impl Default for CoverJob { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: CoverStep, src/game/cover.ts:185)
#[derive(Clone, Copy, Debug)]
pub struct CoverStep {
    pub q: V3,
    pub from: V3,
    pub o: CoverOpeningId,
    pub a: OrdersAssignmentId,
    pub t: f64,
    pub done: bool,
    pub saw: bool,
}
impl CoverStep {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CoverStep {
            q: [0.0; 3],
            from: [0.0; 3],
            o: CoverOpeningId(0),
            a: OrdersAssignmentId(0),
            t: 0.0,
            done: false,
            saw: false,
        }
    }
}
impl Default for CoverStep { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: LiveOpening, src/game/cover.ts:187)
#[derive(Clone, Debug)]
pub struct LiveOpening {
    pub o: CoverOpeningId,
    pub w: f64,
    pub can: Vec<GuardId>,
    pub edge: Vec<GuardId>,
    pub at: JsMap<GuardId, V3>,
    pub by: Option<GuardId>,
    pub edge_by: Option<GuardId>,
    pub sam: bool,
    pub held: bool,
}
impl LiveOpening {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LiveOpening {
            o: CoverOpeningId(0),
            w: 0.0,
            can: Vec::new(),
            edge: Vec::new(),
            at: JsMap::new(),
            by: None,
            edge_by: None,
            sam: false,
            held: false,
        }
    }
}
impl Default for LiveOpening { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Cover, src/game/cover.ts:235)
#[derive(Clone, Debug)]
pub struct Cover {
    pub knobs: crate::knobs::CoverK,
    pub openings: Vec<CoverOpeningId>,
    pub jobs: JsMap<GuardId, crate::cover::CoverJob>,
    pub live: Vec<crate::cover::LiveOpening>,
    pub men: Vec<GuardId>,
    pub stats: crate::cover::CoverStats,
    pub steps: JsMap<GuardId, crate::cover::CoverStep>,
    pub stood: JsMap<GuardId, f64>,
    pub no_step: JsMap<GuardId, f64>,
    pub door_try: JsMap<CoverOpeningId, f64>,
    pub log: Vec<crate::cover::CoverLogItem>,
    pub nav: bool,
    pub deal_at: f64,
    pub heading: JsMap<GuardId, f64>,
    pub rear: Option<CoverOpeningId>,
    pub rears: JsMap<GuardId, CoverOpeningId>,
    pub reads: JsMap<OrdersAssignmentId, crate::cover::CoverReadsValue>,
    pub kept: JsMap<GuardId, crate::cover::CoverKeptValue>,
    pub outs: JsMap<GuardId, crate::cover::CoverOutsValue>,
}
impl Cover {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Cover {
            knobs: crate::knobs::CoverK { on: true, stacked: true, in_room: false, rear_read: false, rear_read_in: 1.0, rear_read_m: 14.0, rear_read_deg: 35.0, rear_read_secs: 1.0, rear_read_sticky: 1.3, rear_look: 6.0, rear_walk_m: 5.0, leaf: true, edge: true, reach: 9.0, deal_secs: 0.25, gap_cells: 3.0, through_rows: 3.0, alcove_rows: 3.0, bridge: 3.0, in_m: 0.9, lane: 0.7, lane_keep: 0.5, lane_secs: 0.6, lane_walk: true, sam_deg: 35.0, sam_k: 0.2, held_k: 0.1, shut_k: 0.7, rear_k: 0.8, near_m: 3.0, contact_secs: 4.0, rear_m: 4.0, rear_from: 3.0, rear_deg: 60.0, sticky: 1.2, keep_secs: 1.5, rear_keep: 1.25, behind_k: 0.2, twist: 1.45, back_in: 3.0, back_out: 3.4, back_keep: true, back_body: 0.0, back_turn: false, back_turn_body: 0.7, back_turn_secs: 1.0, swing: 9.0, step: true, step_m: 1.8, step_after: 0.8, step_off_m: 0.7, step_reach: 6.0, step_secs: 5.0, step_place_m: 0.6 },
            openings: Vec::new(),
            jobs: JsMap::new(),
            live: Vec::new(),
            men: Vec::new(),
            stats: crate::cover::CoverStats { deals: 0.0, takes: 0.0, swaps: 0.0, rear: 0.0, uncovered: 0.0, kept: 0.0, blind: 0.0, unseen: 0.0, steps: 0.0, stepped: 0.0, step_saw: 0.0, rear_read: 0.0, turned: 0.0 },
            steps: JsMap::new(),
            stood: JsMap::new(),
            no_step: JsMap::new(),
            door_try: JsMap::new(),
            log: Vec::new(),
            nav: false,
            deal_at: 0.0,
            heading: JsMap::new(),
            rear: None,
            rears: JsMap::new(),
            reads: JsMap::new(),
            kept: JsMap::new(),
            outs: JsMap::new(),
        }
    }
}
impl Default for Cover { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/cover.ts:248 Cover.stats, src/game/cover.ts:248)
#[derive(Clone, Copy, Debug)]
pub struct CoverStats {
    pub deals: f64,
    pub takes: f64,
    pub swaps: f64,
    pub rear: f64,
    pub uncovered: f64,
    pub kept: f64,
    pub blind: f64,
    pub unseen: f64,
    pub steps: f64,
    pub stepped: f64,
    pub step_saw: f64,
    pub rear_read: f64,
    pub turned: f64,
}
impl CoverStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CoverStats {
            deals: 0.0,
            takes: 0.0,
            swaps: 0.0,
            rear: 0.0,
            uncovered: 0.0,
            kept: 0.0,
            blind: 0.0,
            unseen: 0.0,
            steps: 0.0,
            stepped: 0.0,
            step_saw: 0.0,
            rear_read: 0.0,
            turned: 0.0,
        }
    }
}
impl Default for CoverStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/cover.ts:254 Cover.log, src/game/cover.ts:254)
#[derive(Clone, Debug)]
pub struct CoverLogItem {
    pub t: f64,
    pub who: String,
    pub id: Option<String>,
}
impl CoverLogItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CoverLogItem {
            t: 0.0,
            who: String::new(),
            id: None,
        }
    }
}
impl Default for CoverLogItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at quiet Cover.nav, quiet Cover.nav)
#[derive(Clone, Copy, Debug)]
pub struct CoverNav {
}
impl CoverNav {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CoverNav {
        }
    }
}
impl Default for CoverNav { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/cover.ts:262 Cover.reads, src/game/cover.ts:262)
#[derive(Clone, Debug)]
pub struct CoverReadsValue {
    pub t: f64,
    pub at: V3,
    pub pick: Option<crate::roomfit::Watchable>,
}
impl CoverReadsValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CoverReadsValue {
            t: 0.0,
            at: [0.0; 3],
            pick: None,
        }
    }
}
impl Default for CoverReadsValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/cover.ts:264 Cover.kept, src/game/cover.ts:264)
#[derive(Clone, Copy, Debug)]
pub struct CoverKeptValue {
    pub j: crate::cover::CoverJob,
    pub t: f64,
}
impl CoverKeptValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CoverKeptValue {
            j: crate::cover::CoverJob::zeroed(),
            t: 0.0,
        }
    }
}
impl Default for CoverKeptValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/cover.ts:266 Cover.outs, src/game/cover.ts:266)
#[derive(Clone, Copy, Debug)]
pub struct CoverOutsValue {
    pub j: crate::cover::CoverJob,
    pub aim: V3,
    pub t0: f64,
    pub t: f64,
    pub on: bool,
}
impl CoverOutsValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CoverOutsValue {
            j: crate::cover::CoverJob::zeroed(),
            aim: [0.0; 3],
            t0: 0.0,
            t: 0.0,
            on: false,
        }
    }
}
impl Default for CoverOutsValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/cover.ts:197 gapsOf(nav), src/game/cover.ts:197)
#[derive(Clone, Debug)]
pub struct GapsOfNav {
    pub w: f64,
    pub h: f64,
    pub cell: f64,
    pub blocked: Vec<u8>,
}
impl GapsOfNav {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GapsOfNav {
            w: 0.0,
            h: 0.0,
            cell: 0.0,
            blocked: Vec::new(),
        }
    }
}
impl Default for GapsOfNav { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/cover.ts:592 stepSpot ->, src/game/cover.ts:592)
#[derive(Clone, Copy, Debug)]
pub struct CoverStepSpot {
    pub q: V3,
    pub c: f64,
}
impl CoverStepSpot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CoverStepSpot {
            q: [0.0; 3],
            c: 0.0,
        }
    }
}
impl Default for CoverStepSpot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/cover.ts:652 turnOut ->, src/game/cover.ts:652)
#[derive(Clone, Copy, Debug)]
pub struct CoverTurnOut {
    pub at: V3,
    pub face: Option<f64>,
    pub strafe: bool,
}
impl CoverTurnOut {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CoverTurnOut {
            at: [0.0; 3],
            face: None,
            strafe: false,
        }
    }
}
impl Default for CoverTurnOut { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/cover.ts:690 poseFor ->, src/game/cover.ts:690)
#[derive(Clone, Copy, Debug)]
pub struct CoverPoseFor {
    pub at: V3,
    pub face: Option<f64>,
    pub strafe: bool,
}
impl CoverPoseFor {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CoverPoseFor {
            at: [0.0; 3],
            face: None,
            strafe: false,
        }
    }
}
impl Default for CoverPoseFor { fn default() -> Self { Self::zeroed() } }

/// (src/game/cover.ts:189 EYE)
pub const EYE: f64 = 1.55;

/// (src/game/cover.ts:189 CHEST)
pub const CHEST: f64 = 1.1;

/// (src/game/cover.ts:191 COMPASS: a table nothing writes)
pub fn compass() -> Vec<String> { alloc::vec![String::from("south"), String::from("south-east"), String::from("east"), String::from("north-east"), String::from("north"), String::from("north-west"), String::from("west"), String::from("south-west")] }
