//! The types of `qrf`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// one row of the response team's knobs as this machine kept them (qrf.ts saveQrfTune, read back by storedTune)
#[derive(Clone, Debug)]
pub enum StoredTuneRow {
    Bool { v: bool },
    Num { v: f64 },
}

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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum QrfState {
    Idle,
    Requested,
    Inbound,
    Arriving,
    Deployed,
}
impl QrfState {
    pub const fn as_str(self) -> &'static str {
        match self {
            QrfState::Idle => "idle",
            QrfState::Requested => "requested",
            QrfState::Inbound => "inbound",
            QrfState::Arriving => "arriving",
            QrfState::Deployed => "deployed",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum QrfReason {
    Gunfire,
    Panel,
    Silent,
    Bodies,
}
impl QrfReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            QrfReason::Gunfire => "gunfire",
            QrfReason::Panel => "panel",
            QrfReason::Silent => "silent",
            QrfReason::Bodies => "bodies",
        }
    }
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
impl Phase {
    pub const fn as_str(self) -> &'static str {
        match self {
            Phase::Bang => "bang",
            Phase::Fan => "fan",
            Phase::Breach => "breach",
            Phase::Hold => "hold",
            Phase::Move => "move",
            Phase::Stack => "stack",
            Phase::Called => "called",
            Phase::Drill => "drill",
            Phase::Fight => "fight",
            Phase::Flow => "flow",
            Phase::Regroup => "regroup",
            Phase::Landing => "landing",
            Phase::File => "file",
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Target {
    pub kind: crate::qrf::TargetKind,
    pub plan: Option<crate::scene_level::RoomClearPlan>,
    pub room: String,
    pub pos: V3,
    pub word: Option<f64>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum TargetKind {
    #[default] Room,
    Point,
}
impl TargetKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            TargetKind::Room => "room",
            TargetKind::Point => "point",
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Perimeter {
    pub state: crate::qrf::PerimeterState,
    pub called_at: f64,
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
impl PerimeterState {
    pub const fn as_str(self) -> &'static str {
        match self {
            PerimeterState::Set => "set",
            PerimeterState::Called => "called",
        }
    }
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
    pub t: f64,
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
    pub how: String,
}

#[derive(Clone, Debug, Default)]
pub struct ResponseWordSnap {
    pub who: f64,
    pub at: V3,
    pub age: f64,
    pub how: String,
}

#[derive(Clone, Debug, Default)]
pub struct QRFMARKS {
    pub door: String,
    pub landing: Vec<V3>,
    pub entry: Vec<V3>,
    pub entry_face: Vec<f64>,
    pub posts: Vec<crate::qrf::QRFMARKSPostsItem>,
    pub outside_spawn: Vec<V3>,
}

#[derive(Clone, Debug, Default)]
pub struct QRFMARKSPostsItem {
    pub name: String,
    pub pos: V3,
    pub yaw: f64,
    pub sweep: Vec<V3>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct QRFTUNEDEFAULTS {
    pub qrf: bool,
    pub qrf_eta: f64,
    pub qrf_size: f64,
    pub perimeter: bool,
    pub perimeter_eta: f64,
    pub perimeter_size: f64,
    pub qrf_silent: bool,
    pub qrf_squad: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SideOfTeam {
    Guard,
    Hostile,
}
impl SideOfTeam {
    pub const fn as_str(self) -> &'static str {
        match self {
            SideOfTeam::Guard => "guard",
            SideOfTeam::Hostile => "hostile",
        }
    }
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
    pub budget: Option<()>,
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
    pub at: String,
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

#[derive(Clone, Copy, Debug, Default)]
pub struct RoutePatch {
    pub goal: Option<V3>,
    pub speed: Option<f64>,
    pub face: Option<f64>,
    pub strafe: Option<bool>,
    pub aim_at: Option<V3>,
    pub stance: Option<crate::character::Stance>,
    pub stack_side: Option<f64>,
    pub crouch: Option<bool>,
    pub lean: Option<f64>,
    pub upper: Option<crate::character::UpperMode>,
    pub light: Option<bool>,
    pub arrived: Option<bool>,
    pub hot: Option<bool>,
    pub flinch: Option<bool>,
    pub direct: Option<bool>,
    pub smoke: Option<bool>,
    pub over: Option<crate::game::GuardScriptOver>,
}

/// (src/game/qrf.ts:71 QRF_MARKS: a table nothing writes)
pub fn qrf_marks() -> crate::qrf::QRFMARKS { crate::qrf::QRFMARKS { door: String::from("stairwell"), landing: alloc::vec![[3.1, 0.0, 11.25], [2.25, 0.0, 11.25], [3.1, 0.0, 10.3], [2.25, 0.0, 10.3]], entry: alloc::vec![[6.7, 0.0, 11.2], [5.9, 0.0, 10.55], [5.9, 0.0, 11.8], [5.4, 0.0, 11.25]], entry_face: alloc::vec![1.5707963267948966, 2.4, 0.75, 1.5707963267948966], posts: alloc::vec![crate::qrf::QRFMARKSPostsItem { name: String::from("fire exit"), pos: [38.9, 0.0, 24.2], yaw: -2.48, sweep: alloc::vec![[36.15, 1.0, 19.9], [36.15, 1.0, 21.5]] }, crate::qrf::QRFMARKSPostsItem { name: String::from("break-room door"), pos: [27.4, 0.0, 25.7], yaw: 1.95, sweep: alloc::vec![[30.9, 1.0, 24.15], [32.4, 1.0, 24.15]] }, crate::qrf::QRFMARKSPostsItem { name: String::from("lobby front"), pos: [2.3, 0.0, 20.6], yaw: 1.5707963267948966, sweep: alloc::vec![[3.9, 1.0, 17.6], [3.9, 1.0, 22.4]] }, crate::qrf::QRFMARKSPostsItem { name: String::from("south lot"), pos: [17.4, 0.0, 26.4], yaw: 3.141592653589793, sweep: alloc::vec![[14.5, 1.2, 24.15], [23.5, 1.2, 24.15]] }], outside_spawn: alloc::vec![[38.6, 0.0, 26.6], [37.4, 0.0, 26.9], [38.8, 0.0, 25.4], [37.2, 0.0, 25.6]] } }

/// (src/game/qrf.ts:163 CHAR_BUDGET)
pub const CHAR_BUDGET: f64 = 8.0;

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

/// (src/game/qrf.ts:261 TUNE_KEY)
pub const TUNE_KEY: &str = "twopointsix.qrf";

/// (src/game/qrf.ts:280 QRF_TUNE_DEFAULTS: a table nothing writes)
pub fn qrf_tune_defaults() -> crate::qrf::QRFTUNEDEFAULTS { crate::qrf::QRFTUNEDEFAULTS { qrf: true, qrf_eta: 75.0, qrf_size: 4.0, perimeter: true, perimeter_eta: 45.0, perimeter_size: 2.0, qrf_silent: true, qrf_squad: true } }
