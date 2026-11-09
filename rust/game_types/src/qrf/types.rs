//! The types of `qrf`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Marks {
    pub door: Option<String>,
    pub name: String,
    pub muster: Vec<V3>,
    pub entry: Vec<V3>,
    pub entry_face: Vec<f64>,
    pub coming: String,
    pub clear: Option<String>,
    pub stairs: bool,
    pub posts: Vec<crate::scene_scenarios::ResponsePost>,
    pub outside: Vec<V3>,
    pub site: String,
    pub at: String,
    pub team: String,
    pub tag: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Words)]
pub enum QrfState {
    Idle,
    Requested,
    Inbound,
    Arriving,
    Deployed,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Words)]
pub enum QrfReason {
    Gunfire,
    Panel,
    Silent,
    Bodies,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes)]
pub enum Phase {
    Bang,
    Fan,
    Breach,
    Hold,
    Move,
    Stack,
    Called,
    Drill,
    Fight,
    Flow,
    Regroup,
    Landing,
    File,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Target {
    pub kind: crate::qrf::TargetKind,
    pub plan: Option<crate::scene_level::RoomClearPlan>,
    pub room: String,
    pub pos: V3,
    pub word: Option<f64>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum TargetKind {
    #[default] Room,
    Point,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Perimeter {
    pub state: crate::qrf::PerimeterState,
    pub eta_at: f64,
    pub set_at: f64,
    pub men: Vec<crate::qrf::PerimeterMenItem>,
    pub spawned: f64,
    pub peeled: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum PerimeterState {
    #[default] Set,
    Called,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct PerimeterMenItem {
    pub gd: GuardId,
    pub post: crate::scene_scenarios::ResponsePost,
    pub arrived_at: f64,
    pub fighting: bool,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Qrf {
    #[is(crate::qrf::QrfState::Idle)] pub state: crate::qrf::QrfState,
    pub reason: Option<crate::qrf::QrfReason>,
    #[is(-1.0)] pub requested_at: f64,
    #[is(-1.0)] pub arrive_at: f64,
    #[is(-1.0)] pub deployed_at: f64,
    pub answered: bool,
    pub said30: bool,
    pub said10: bool,
    #[is(-100.0)] pub refused_at: f64,
    pub everyone_said: bool,
    pub men: Vec<GuardId>,
    pub perim: Option<crate::qrf::Perimeter>,
    pub bangs: f64,
    pub lost: f64,
    pub got_sam: bool,
    pub sam_was_down: bool,
    #[is(-1000000000.0)] pub loud_t: f64,
    #[is(-1000000000.0)] pub raid_t: f64,
    pub heard_at: JsMap<GuardId, f64>,
    pub shots_since: Vec<f64>,
    pub heard_events: JsSet<u32>,
    #[is(-1.0)] pub silent_at: f64,
    pub silent_said: bool,
    pub silent_spent: bool,
    pub log: Vec<crate::qrf::QrfLogItem>,
    #[is(crate::qrf::Phase::Landing)] pub phase: crate::qrf::Phase,
    pub phase_t: f64,
    pub target: Option<crate::qrf::Target>,
    #[is(-100.0)] pub target_at: f64,
    pub fix: Option<V3>,
    #[is(-100.0)] pub fix_t: f64,
    pub cue_rush: bool,
    pub cleared: JsMap<String, f64>,
    pub wps: JsMap<GuardId, Vec<V3>>,
    pub trail: Vec<V3>,
    pub station_t: f64,
    #[is(-1.0)] pub point_id: f64,
    #[is(2.15)] pub move_speed: f64,
    pub door_pulled: bool,
    pub filed: f64,
    pub flowed: f64,
    pub can: Option<ItemId>,
    #[is(f64::NAN)] pub can_land_t: f64,
    pub stack_marks: Vec<V3>,
    #[is(f64::NAN)] pub shoved_at: f64,
    pub seen_ids: JsSet<crate::Num>,
    pub aim_pts: JsMap<GuardId, V3>,
    pub stations: JsMap<GuardId, V3>,
    pub cast: Option<crate::squadcast::SquadCast>,
    pub op: Option<DoorOpId>,
    pub op_beat: f64,
    pub drills: f64,
    pub drills_in: f64,
    pub spent: Option<crate::qrf::QrfSpent>,
    #[is(-1000000000.0)] pub swept_t: f64,
    pub way: f64,
    #[is(-1.0)] pub size: f64,
    pub marks: Option<crate::qrf::Marks>,
    pub marks_on: bool,
    #[is(-1.0)] pub marks_way: f64,
    pub word: JsMap<crate::qrf::QrfWordKey, crate::qrf::QrfWordValue>,
    #[is(-1000000000.0)] pub word_t: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct QrfLogItem {
    pub what: String,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct QrfSpent {
    pub room: String,
    pub fix_t: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes)]
pub enum QrfWordKey {
    Guard(GuardId),
    Player,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct QrfWordValue {
    pub at: V3,
    pub t: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SideOfTeam {
    Guard,
    Hostile,
}

#[derive(Clone, Copy, Debug)]
pub enum BudgetOf {
    Arm0(crate::qrf::BudgetOfArm0),
    Arm1(crate::qrf::BudgetOfArm1),
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BudgetOfArm0 {
    pub budget: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BudgetOfArm1 {
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BestFix {
    pub pos: V3,
    pub t: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct NetWord {
    pub pos: V3,
    pub t: f64,
    pub n: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FixOf {
    pub pos: V3,
    pub t: f64,
}

#[derive(Clone, Debug, Default)]
pub struct ResponseWords {
    pub site: String,
    pub team: String,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Dealt {
    pub way: f64,
    pub men: f64,
    pub eta: f64,
}

#[derive(Clone, Debug, Default)]
pub struct SweepPlacesItem {
    pub name: String,
    pub pos: V3,
}

#[derive(Clone, Debug, Default)]
pub struct FlowLegs {
    pub pts: Vec<V3>,
    pub face: f64,
    pub aim: V3,
}

pub const LOUD_MEMORY_SECS: f64 = 180.0;

pub const SILENT_CHECK_SECS: f64 = 6.0;

pub const SILENT_GO_SECS: f64 = 14.0;

pub const ANSWER_SECS: f64 = 1.6;

pub const CALL30: f64 = 30.0;

pub const CALL10: f64 = 10.0;

pub const FILE_GAP: f64 = 0.55;

pub const ENTRY_HOLD: f64 = 1.6;

pub const MOVE_SPEED: f64 = 2.15;

pub const RUSH_SPEED: f64 = 2.6;

pub const FILE_SPACING: f64 = 1.3;

pub const FILE_OFFSET: f64 = 0.42;

pub const STATION_SECS: f64 = 0.35;

pub const RETARGET_M: f64 = 4.0;

pub const RETARGET_SECS: f64 = 20.0;

pub const RECLEAR_SECS: f64 = 90.0;

pub const STACK_CAP: f64 = 5.5;

pub const BANG_DEPTH: f64 = 2.3;

pub const BANG_FUSE: f64 = 0.7;

pub const FLOW_GAP: f64 = 0.42;

pub const FLOW_CAP: f64 = 6.5;

pub const HOLD360_SECS: f64 = 1.6;

pub const DRILL_CAP: f64 = 120.0;

pub const REGROUP_SECS: f64 = 8.0;

pub const HUNT_AWARENESS: f64 = 0.7;

pub const GUNFIGHT_SHOTS: f64 = 3.0;

pub const GUNFIGHT_WINDOW: f64 = 20.0;

pub const SETTLE_IN_SECS: f64 = 8.0;

pub const DOWN_FOR_PERIMETER: f64 = 3.0;

pub const WALKOUT_ENGAGE_M: f64 = 6.0;

pub const POST_SPEED: f64 = 2.6;

pub const SWEEP_SECS: f64 = 5.0;

