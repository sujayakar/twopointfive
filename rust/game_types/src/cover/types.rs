//! The types of `cover`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum OpeningKind {
    #[default] Door,
    Mouth,
    Gap,
    Rear,
}
impl OpeningKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            OpeningKind::Door => "door",
            OpeningKind::Mouth => "mouth",
            OpeningKind::Gap => "gap",
            OpeningKind::Rear => "rear",
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CoverStep {
    pub q: V3,
    pub from: V3,
    pub o: CoverOpeningId,
    pub a: OrdersAssignmentId,
    pub t: f64,
    pub done: bool,
    pub saw: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Debug, ToBytes)]
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
    /// what a new one holds
    pub fn blank() -> Self {
        Cover {
            knobs: crate::knobs::CoverK { rear_read_in: 1.0, rear_read_m: 14.0, rear_read_deg: 35.0, rear_read_secs: 1.0, rear_read_sticky: 1.3, rear_look: 6.0, rear_walk_m: 5.0, reach: 9.0, deal_secs: 0.25, gap_cells: 3.0, through_rows: 3.0, alcove_rows: 3.0, bridge: 3.0, in_m: 0.9, lane: 0.7, lane_keep: 0.5, lane_secs: 0.6, sam_deg: 35.0, sam_k: 0.2, held_k: 0.1, shut_k: 0.7, rear_k: 0.8, near_m: 3.0, contact_secs: 4.0, rear_m: 4.0, rear_from: 3.0, rear_deg: 60.0, sticky: 1.2, keep_secs: 1.5, rear_keep: 1.25, behind_k: 0.2, twist: 1.45, back_in: 3.0, back_out: 3.4, back_body: 0.0, back_turn_body: 0.7, back_turn_secs: 1.0, swing: 9.0, step_m: 1.8, step_after: 0.8, step_off_m: 0.7, step_reach: 6.0, step_secs: 5.0, step_place_m: 0.6 },
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
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

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CoverLogItem {
    pub t: f64,
    pub who: String,
    pub id: Option<String>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CoverReadsValue {
    pub t: f64,
    pub at: V3,
    pub pick: Option<crate::roomfit::Watchable>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CoverKeptValue {
    pub j: crate::cover::CoverJob,
    pub t: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CoverOutsValue {
    pub j: crate::cover::CoverJob,
    pub aim: V3,
    pub t0: f64,
    pub t: f64,
    pub on: bool,
}

#[derive(Clone, Debug, Default)]
pub struct GapsOfNav {
    pub w: f64,
    pub h: f64,
    pub cell: f64,
    pub blocked: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CoverStepSpot {
    pub q: V3,
    pub c: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CoverTurnOut {
    pub at: V3,
    pub face: Option<f64>,
    pub strafe: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CoverPoseFor {
    pub at: V3,
    pub face: Option<f64>,
    pub strafe: bool,
}

/// (src/game/cover.ts:189 EYE)
pub const EYE: f64 = 1.55;

/// (src/game/cover.ts:189 CHEST)
pub const CHEST: f64 = 1.1;

