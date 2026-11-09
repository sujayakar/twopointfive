//! The types of `qrf`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, Words, ids::*, js::*, Quat, V3};
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

#[derive(Clone, Debug, ToBytes)]
pub struct Qrf {
    pub state: crate::qrf::QrfState,
    pub reason: Option<crate::qrf::QrfReason>,
    pub requested_at: f64,
    pub arrive_at: f64,
    pub deployed_at: f64,
    pub answered: bool,
    pub said30: bool,
    pub said10: bool,
    pub refused_at: f64,
    pub everyone_said: bool,
    pub men: Vec<GuardId>,
    pub perim: Option<crate::qrf::Perimeter>,
    pub bangs: f64,
    pub lost: f64,
    pub got_sam: bool,
    pub sam_was_down: bool,
    pub loud_t: f64,
    pub raid_t: f64,
    pub heard_at: JsMap<GuardId, f64>,
    pub shots_since: Vec<f64>,
    pub heard_events: JsSet<u32>,
    pub silent_at: f64,
    pub silent_said: bool,
    pub silent_spent: bool,
    pub log: Vec<crate::qrf::QrfLogItem>,
    pub phase: crate::qrf::Phase,
    pub phase_t: f64,
    pub target: Option<crate::qrf::Target>,
    pub target_at: f64,
    pub fix: Option<V3>,
    pub fix_t: f64,
    pub cue_rush: bool,
    pub cleared: JsMap<String, f64>,
    pub wps: JsMap<GuardId, Vec<V3>>,
    pub trail: Vec<V3>,
    pub station_t: f64,
    pub point_id: f64,
    pub move_speed: f64,
    pub door_pulled: bool,
    pub filed: f64,
    pub flowed: f64,
    pub can: Option<ItemId>,
    pub can_land_t: f64,
    pub stack_marks: Vec<V3>,
    pub shoved_at: f64,
    pub seen_ids: JsSet<crate::Num>,
    pub aim_pts: JsMap<GuardId, V3>,
    pub stations: JsMap<GuardId, V3>,
    pub cast: Option<crate::squadcast::SquadCast>,
    pub op: Option<DoorOpId>,
    pub op_beat: f64,
    pub drills: f64,
    pub drills_in: f64,
    pub spent: Option<crate::qrf::QrfSpent>,
    pub swept_t: f64,
    pub way: f64,
    pub size: f64,
    pub marks: Option<crate::qrf::Marks>,
    pub marks_on: bool,
    pub marks_way: f64,
    pub word: JsMap<crate::qrf::QrfWordKey, crate::qrf::QrfWordValue>,
    pub word_t: f64,
}
impl Qrf {
    /// what a new one holds
    pub fn blank() -> Self {
        Qrf {
            state: crate::qrf::QrfState::Idle,
            reason: None,
            requested_at: -1.0,
            arrive_at: -1.0,
            deployed_at: -1.0,
            answered: false,
            said30: false,
            said10: false,
            refused_at: -100.0,
            everyone_said: false,
            men: Vec::new(),
            perim: None,
            bangs: 0.0,
            lost: 0.0,
            got_sam: false,
            sam_was_down: false,
            loud_t: -1000000000.0,
            raid_t: -1000000000.0,
            heard_at: JsMap::new(),
            shots_since: Vec::new(),
            heard_events: JsSet::new(),
            silent_at: -1.0,
            silent_said: false,
            silent_spent: false,
            log: Vec::new(),
            phase: crate::qrf::Phase::Landing,
            phase_t: 0.0,
            target: None,
            target_at: -100.0,
            fix: None,
            fix_t: -100.0,
            cue_rush: false,
            cleared: JsMap::new(),
            wps: JsMap::new(),
            trail: Vec::new(),
            station_t: 0.0,
            point_id: -1.0,
            move_speed: 2.15,
            door_pulled: false,
            filed: 0.0,
            flowed: 0.0,
            can: None,
            can_land_t: f64::NAN,
            stack_marks: Vec::new(),
            shoved_at: f64::NAN,
            seen_ids: JsSet::new(),
            aim_pts: JsMap::new(),
            stations: JsMap::new(),
            cast: None,
            op: None,
            op_beat: 0.0,
            drills: 0.0,
            drills_in: 0.0,
            spent: None,
            swept_t: -1000000000.0,
            way: 0.0,
            size: -1.0,
            marks: None,
            marks_on: false,
            marks_way: -1.0,
            word: JsMap::new(),
            word_t: -1000000000.0,
        }
    }
}
impl Default for Qrf { fn default() -> Self { Self::blank() } }

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

/// (src/game/qrf.ts:165 LOUD_MEMORY_SECS)
pub const LOUD_MEMORY_SECS: f64 = 180.0;

/// (src/game/qrf.ts:168 SILENT_CHECK_SECS)
pub const SILENT_CHECK_SECS: f64 = 6.0;

/// (src/game/qrf.ts:168 SILENT_GO_SECS)
pub const SILENT_GO_SECS: f64 = 14.0;

/// (src/game/qrf.ts:171 ANSWER_SECS)
pub const ANSWER_SECS: f64 = 1.6;

/// (src/game/qrf.ts:171 CALL30)
pub const CALL30: f64 = 30.0;

/// (src/game/qrf.ts:171 CALL10)
pub const CALL10: f64 = 10.0;

/// (src/game/qrf.ts:173 FILE_GAP)
pub const FILE_GAP: f64 = 0.55;

/// (src/game/qrf.ts:173 ENTRY_HOLD)
pub const ENTRY_HOLD: f64 = 1.6;

/// (src/game/qrf.ts:176 MOVE_SPEED)
pub const MOVE_SPEED: f64 = 2.15;

/// (src/game/qrf.ts:176 RUSH_SPEED)
pub const RUSH_SPEED: f64 = 2.6;

/// (src/game/qrf.ts:176 FILE_SPACING)
pub const FILE_SPACING: f64 = 1.3;

/// (src/game/qrf.ts:176 FILE_OFFSET)
pub const FILE_OFFSET: f64 = 0.42;

/// (src/game/qrf.ts:176 STATION_SECS)
pub const STATION_SECS: f64 = 0.35;

/// (src/game/qrf.ts:178 RETARGET_M)
pub const RETARGET_M: f64 = 4.0;

/// (src/game/qrf.ts:178 RETARGET_SECS)
pub const RETARGET_SECS: f64 = 20.0;

/// (src/game/qrf.ts:178 RECLEAR_SECS)
pub const RECLEAR_SECS: f64 = 90.0;

/// (src/game/qrf.ts:181 STACK_CAP)
pub const STACK_CAP: f64 = 5.5;

/// (src/game/qrf.ts:181 BANG_DEPTH)
pub const BANG_DEPTH: f64 = 2.3;

/// (src/game/qrf.ts:181 BANG_FUSE)
pub const BANG_FUSE: f64 = 0.7;

/// (src/game/qrf.ts:181 FLOW_GAP)
pub const FLOW_GAP: f64 = 0.42;

/// (src/game/qrf.ts:181 FLOW_CAP)
pub const FLOW_CAP: f64 = 6.5;

/// (src/game/qrf.ts:181 HOLD360_SECS)
pub const HOLD360_SECS: f64 = 1.6;

/// (src/game/qrf.ts:184 DRILL_CAP)
pub const DRILL_CAP: f64 = 120.0;

/// (src/game/qrf.ts:186 REGROUP_SECS)
pub const REGROUP_SECS: f64 = 8.0;

/// (src/game/qrf.ts:186 HUNT_AWARENESS)
pub const HUNT_AWARENESS: f64 = 0.7;

/// (src/game/qrf.ts:189 GUNFIGHT_SHOTS)
pub const GUNFIGHT_SHOTS: f64 = 3.0;

/// (src/game/qrf.ts:189 GUNFIGHT_WINDOW)
pub const GUNFIGHT_WINDOW: f64 = 20.0;

/// (src/game/qrf.ts:189 SETTLE_IN_SECS)
pub const SETTLE_IN_SECS: f64 = 8.0;

/// (src/game/qrf.ts:189 DOWN_FOR_PERIMETER)
pub const DOWN_FOR_PERIMETER: f64 = 3.0;

/// (src/game/qrf.ts:191 WALKOUT_ENGAGE_M)
pub const WALKOUT_ENGAGE_M: f64 = 6.0;

/// (src/game/qrf.ts:193 POST_SPEED)
pub const POST_SPEED: f64 = 2.6;

/// (src/game/qrf.ts:193 SWEEP_SECS)
pub const SWEEP_SECS: f64 = 5.0;

