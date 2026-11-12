//! The types of `cover`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum OpeningKind {
    #[default] Door,
    Mouth,
    Gap,
    Rear,
    /// floor of the room he is in that nobody has looked over
    Floor,
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
    pub a: TaskId,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CoverStep {
    pub q: V3,
    pub from: V3,
    pub o: CoverOpeningId,
    pub a: TaskId,
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

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Cover {
    pub openings: Vec<CoverOpeningId>,
    pub jobs: JsMap<GuardId, crate::cover::CoverJob>,
    pub live: Vec<crate::cover::LiveOpening>,
    pub steps: JsMap<GuardId, crate::cover::CoverStep>,
    pub stood: JsMap<GuardId, f64>,
    pub no_step: JsMap<GuardId, f64>,
    pub door_try: JsMap<CoverOpeningId, f64>,
    pub nav: bool,
    pub deal_at: f64,
    pub heading: JsMap<GuardId, f64>,
    pub rear: Option<CoverOpeningId>,
    pub rears: JsMap<GuardId, CoverOpeningId>,
    pub floors: JsMap<GuardId, CoverOpeningId>,
    pub reads: JsMap<TaskId, crate::cover::CoverReadsValue>,
    pub kept: JsMap<GuardId, crate::cover::CoverKeptValue>,
    pub outs: JsMap<GuardId, crate::cover::CoverOutsValue>,
    /// the way into his room each man of a held room was dealt, and when they are dealt again
    pub posts: JsMap<GuardId, crate::cover::PostWay>,
    pub posts_at: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PostWay {
    pub room: RoomfightRoomId,
    pub door: DoorId,
    pub t0: f64,
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

pub const EYE: f64 = 1.55;

pub const CHEST: f64 = 1.1;

