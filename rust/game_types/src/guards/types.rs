//! The types of `guards`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// what litPlace last read the light for besides the man himself: the man he has, or the point he is turning to
/// THE LIGHT ON A MAN: what stands on him, and how visible that makes him, eased
#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Shown {
    pub visibility: f64,
    pub irradiance: f64,
}

#[derive(Clone, Debug, ToBytes)]
pub enum LitSeenTgt {
    Man { gd: GuardId },
    Pt,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Engagement {
    pub operatives: Vec<GuardId>,
    pub hostiles: Vec<GuardId>,
    pub t0: f64,
    pub timeout: f64,
    pub done: bool,
    pub rounds: crate::guards::EngagementRounds,
    pub alive: JsSet<GuardId>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct EngagementRounds {
    pub guard: Option<f64>,
    pub operative: Option<f64>,
    pub hostile: Option<f64>,
    pub civilian: Option<f64>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Standoff {
    pub pair: GuardId,
    pub side: f64,
    pub way: f64,
    pub orbit_goal: Option<V3>,
    pub replan_t: f64,
    pub rest_t: f64,
    pub seen_t: f64,
    pub last_bark_t: f64,
    pub line_open: bool,
    pub stalls: f64,
    pub cornered: bool,
    pub backing: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Pair {
    pub key: u32,
    pub held: GuardId,
    pub sam: V3,
    pub centre: V3,
    pub facing: f64,
    pub aim: V3,
    pub vel: V3,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum StandoffMoment {
    Lost,
    Enter,
    Cornered,
    NoShot,
    Line,
    Dropped,
    Freed,
    HitHim,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Eye {
    pub pos: V3,
    pub look: V3,
    pub range: f64,
    pub half_deg: f64,
    pub near_m: f64,
    pub near_deg: f64,
    pub v_half_deg: Option<f64>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct UnstickLeg {
    pub to: V3,
    pub best: f64,
    pub stall_t: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SearchSpot {
    pub pos: V3,
    pub cell: f64,
    pub kind: crate::guards::SearchSpotKind,
    pub cover: f64,
    pub dark: f64,
    pub e: Option<f64>,
    pub need: f64,
    pub plaus: f64,
    pub score: f64,
    pub visited: bool,
    pub by: f64,
    pub unreachable: bool,
    pub samples: Vec<crate::guards::SearchSpotSamplesItem>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum SearchSpotKind {
    #[default] Wall,
    Prop,
    Open,
    Corner,
    Leaf,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SearchSpotSamplesItem {
    pub t: f64,
    pub e: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SearchPlan {
    pub fix: V3,
    pub t0: f64,
    pub serial: f64,
    pub built_at: f64,
    pub flood_at: f64,
    pub spots: Vec<crate::guards::SearchSpot>,
    pub current: f64,
    pub phase: crate::guards::SearchPlanPhase,
    pub phase_t: f64,
    pub rr: f64,
    pub said: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum SearchPlanPhase {
    #[default] Done,
    Look,
    Walk,
    Sweep,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Pieing {
    pub door: DoorId,
    pub phase: crate::guards::PieingPhase,
    pub side: f64,
    pub at: V3,
    pub near: f64,
    pub t: f64,
    pub t0: f64,
    pub path_ref: u32,
    pub bent: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum PieingPhase {
    #[default] Slice,
    Approach,
    Cross,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Rollcall {
    pub next_at: f64,
    pub stage: crate::guards::RollcallStage,
    pub t0: f64,
    pub caller: Option<GuardId>,
    pub missing: Option<GuardId>,
    pub answered: bool,
    pub checks: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum RollcallStage {
    #[default] Idle,
    Called,
    Asked,
}

#[derive(Clone, Debug, Default)]
pub struct UnitOpts {
    pub team: crate::teams::Team,
    pub pos: V3,
    pub yaw: Option<f64>,
    pub weapon: Option<crate::weapons::WeaponId>,
    pub tint: Option<V3>,
    pub tint2: Option<V3>,
    pub carbine_rig: Option<bool>,
    pub name: Option<String>,
    pub light: Option<bool>,
    pub pair: Option<crate::game::PairId>,
    pub budget: Option<f64>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GuardsModGroundTrackValue {
    pub pos: V3,
    pub t: f64,
    pub vel: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GuardsModBodyMetersValue {
    pub vis: f64,
    pub t: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HELDMETER {
    pub gap_secs: f64,
    pub moved_m: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GuardsModHeldMetersValue {
    pub vis: f64,
    pub t: f64,
    pub at: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GuardsModUNSTICK {
    pub last_snap_t: f64,
    pub last_snap_who: Option<GuardId>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct GuardsModLitSeenValue {
    pub t: f64,
    pub lit: bool,
    pub a: Option<OrdersAssignmentId>,
    pub tgt: Option<crate::guards::LitSeenTgt>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GuardsModWallSeenValue {
    pub t: f64,
    pub since: f64,
    pub dark: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GuardsModTorchLowSeenValue {
    pub t: f64,
    pub low: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GuardsModLitManSeenValue {
    pub t: f64,
    pub tgt: GuardId,
    pub v: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct NOSIGHT {
    pub visible: bool,
    pub dist: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ShotRelationE {
    pub kind: crate::director::EventKind,
    pub who: Option<f64>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ShotRelation {
    None,
    Own,
    Threat,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ColleagueFirePosE {
    pub who: Option<f64>,
    pub pos: V3,
    pub to: Option<V3>,
    pub blind: Option<bool>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StepIsThreatE {
    pub who: Option<f64>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LightOutSeenE {
    pub pos: V3,
    pub from: Option<V3>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DoorTamper {
    Picked,
    Forced,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BodyAftermath {
    pub move_: f64,
    pub aim: Option<V3>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LineGeom {
    pub t: f64,
    pub e: f64,
    pub d: f64,
    pub clear: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LineStep {
    Clear,
    None,
    Step,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FlankShot {
    pub ok: bool,
    pub rear: f64,
}

#[derive(Clone, Copy, Debug)]
pub enum StandoffBarkForce {
    Bool(bool),
    Always,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StandoffAlert {
    pub move_: f64,
    pub aim: V3,
    pub crouch: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CanSee {
    pub visible: bool,
    pub dist: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CanSeeChar {
    pub visible: bool,
    pub dist: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct EyeContact {
    pub visible: bool,
    pub dist: f64,
    pub in_cone: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FreeCellNearTest {
    Room,
    Walk,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum UnstickBegin {
    Leg,
    Snapped,
    No,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum UnstickStep {
    Done,
    Stale,
    Walking,
}

#[derive(Clone, Debug, Default)]
pub struct FloodFrom {
    pub secs: Vec<f32>,
    pub metres: Vec<f32>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CrossesDoor {
    pub t: f64,
    pub side: f64,
    pub at: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PieDoorway {
    pub speed: f64,
    pub look: Option<V3>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ChaseFix {
    pub move_: f64,
    pub aim: Option<V3>,
    pub pied: bool,
    pub crouch: f64,
}

#[derive(Clone, Copy, Debug)]
pub enum SteadyLamps {
    Bool(bool),
    LampRows(crate::scene_collision::LampRows),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum RaiseEscalationCause {
    Search,
    Civilian,
    Device,
    Missing,
    Body,
    Hostage,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PlanClearsCause {
    Search,
    Missing,
    Body,
    Panel,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct GuardsMod {
    pub ground_track: JsMap<PlayerId, crate::guards::GuardsModGroundTrackValue>,
    pub body_meters: JsMap<GuardId, crate::guards::GuardsModBodyMetersValue>,
    pub holds_called: JsSet<u32>,
    pub floor_bark_t: Option<f64>,
    pub last_bark: JsMap<GuardId, String>,
    pub held_meters: JsMap<u32, crate::guards::GuardsModHeldMetersValue>,
    pub unstick_legs: JsMap<GuardId, crate::guards::UnstickLeg>,
    pub unstick_retry_at: JsMap<GuardId, f64>,
    pub held_last_frame: JsSet<GuardId>,
    #[is(crate::guards::GuardsModUNSTICK { last_snap_t: -1.0, last_snap_who: None })] pub unstick: crate::guards::GuardsModUNSTICK,
    pub lit_seen: JsMap<GuardId, crate::guards::GuardsModLitSeenValue>,
    pub lit_until: JsMap<GuardId, f64>,
    pub wall_seen: JsMap<GuardId, crate::guards::GuardsModWallSeenValue>,
    pub torch_low_seen: JsMap<GuardId, crate::guards::GuardsModTorchLowSeenValue>,
    pub lit_man_seen: JsMap<GuardId, crate::guards::GuardsModLitManSeenValue>,
    pub door_cells_of: Option<JsMap<crate::Num, DoorId>>,
    pub rollcalls: Option<crate::guards::Rollcall>,
    pub unit_serial: f64,
    pub engagements: Option<crate::guards::Engagement>,
}

pub fn ocp_mutter() -> Vec<String> { alloc::vec![String::from("light's on the blink again"), String::from("hm — bulb's going"), String::from("power dip?"), String::from("these fittings…"), String::from("that light again")] }

pub const MOTION_STILL: f64 = 0.35;

pub const MOTION_LO: f64 = 0.05;

pub const MOTION_HI: f64 = 1.2;

pub const MOTION_LIT_LO: f64 = 0.3;

pub const MOTION_LIT_HI: f64 = 0.75;

pub const MOTION_RADIAL: f64 = 0.4;

pub const MOTION_SPRINT: f64 = 1.6;

pub const MOTION_SPRINT_MS: f64 = 3.5;

pub const MOTION_NEAR_M: f64 = 2.5;

pub const MOTION_NEAR_FLOOR: f64 = 0.8;

pub const MOTION_HUNT_FLOOR: f64 = 0.7;

pub const BODY_DUTY_CAP: f64 = 45.0;

pub const MUZZLE_CLEAR_M: f64 = 0.35;

pub const MUZZLE_LEAD_SECS: f64 = 0.3;

pub const MUZZLE_LEAD_MAX_M: f64 = 0.4;

pub const MUZZLE_PAST_M: f64 = 1.0;

pub const MUZZLE_REACH_M: f64 = 8.0;

pub const MUZZLE_UP_SECS: f64 = 0.2;

pub const LINE_STEP_MARGIN_M: f64 = 0.12;

pub const LINE_STEP_MAX_M: f64 = 2.2;

pub const LINE_PUSH_MIN_M: f64 = 2.5;

pub const LINE_BARK_SECS: f64 = 10.0;

pub const LINE_STUCK_SECS: f64 = 0.4;

pub const LINE_FAIL_SECS: f64 = 5.0;

pub const LINE_FAIL_M: f64 = 0.75;

pub const LINE_WALK_CLEAR_M: f64 = 0.5;

pub const STANDOFF_MIN_M: f64 = 3.5;

pub const STANDOFF_MAX_M: f64 = 7.0;

pub const STANDOFF_ORBIT_M: f64 = 4.5;

pub const STANDOFF_SETTLE_DEG: f64 = 22.0;

pub const STANDOFF_FIRE_MAX_M: f64 = 5.0;

pub const STANDOFF_REAR_DEG: f64 = 70.0;

pub const STANDOFF_CLEAR_M: f64 = 0.45;

pub const STANDOFF_SHOT_SECS: f64 = 1.2;

pub const STANDOFF_AIM_Y: f64 = 1.42;

pub const SAM_R: f64 = 0.28;

pub const STANDOFF_VIEW_M: f64 = 16.0;

pub const STANDOFF_VIEW_DEG: f64 = 60.0;

pub const STANDOFF_LOSE_SECS: f64 = 4.0;

pub const STANDOFF_REPLAN_SECS: f64 = 1.0;

pub const STANDOFF_STALL_SECS: f64 = 2.5;

pub const STANDOFF_REST_SECS: f64 = 1.5;

pub const STANDOFF_BARK_SECS: f64 = 6.0;

pub const STANDOFF_BARK_FLOOR_SECS: f64 = 3.0;

pub const SHIELD_RELEASE_BEAT_MIN: f64 = 0.35;

pub const SHIELD_RELEASE_BEAT_JITTER: f64 = 0.3;

pub const PAIR_LEGIBLE: f64 = 3.0;

pub fn held_meter() -> crate::guards::HELDMETER { crate::guards::HELDMETER { gap_secs: 0.5, moved_m: 1.0 } }

pub const UNSTICK_REACH_M: f64 = 2.4;

pub const UNSTICK_STALL_SECS: f64 = 1.0;

pub const UNSTICK_DONE_M: f64 = 0.08;

pub const UNSTICK_SNAP_REACH_M: f64 = 3.5;

pub const UNSTICK_BODY_R: f64 = 0.15;

pub const FLOOR_TOP: f64 = 0.249;

pub const SEARCH_RADIUS_M: f64 = 9.0;

pub const SEARCH_SPOTS: f64 = 12.0;

pub const SEARCH_CONTROLS: f64 = 3.0;

pub const SEARCH_SPACING_M: f64 = 1.5;

pub const SEARCH_SWEEP0_SECS: f64 = 1.2;

pub const SEARCH_LOOK_SECS: f64 = 1.5;

pub const SEARCH_LOOK_DIST_M: f64 = 1.9;

pub const SEARCH_MIN_SECS: f64 = 12.0;

pub const SEARCH_CAP_SECS: f64 = 26.0;

pub const SEARCH_REFLOOD_SECS: f64 = 2.0;

pub const PROBE_BUDGET: f64 = 8.0;

pub const PROBE_HEIGHT: f64 = 0.5;

pub const DARK_WINDOW_SECS: f64 = 3.0;

pub const LATCH_SECS: f64 = 5.5;

pub const PLAUS_SLACK_SECS: f64 = 1.2;

pub const PLAUS_RAMP_SECS: f64 = 2.5;

pub const PIE_DETECT_M: f64 = 3.2;

pub const PIE_SLOW_M: f64 = 1.5;

pub const PIE_SLICE_M: f64 = 1.7;

pub const PIE_SLICE_SECS: f64 = 1.0;

pub const PIE_CORNER_SECS: f64 = 0.4;

pub const PIE_AGAIN_SECS: f64 = 6.0;

pub const PIE_CAP_SECS: f64 = 7.0;

pub const PIE_EYES_SECS: f64 = 1.0;

pub const GUARD_SWEEP_STEP: f64 = 0.2;

pub const SEP_MAX_M: f64 = 0.25;

pub const SEP_SETTLE_MIN: f64 = 0.02;

pub fn no_sight() -> crate::guards::NOSIGHT { crate::guards::NOSIGHT { visible: false, dist: 99.0 } }

pub const BURST_BROKEN_CD: f64 = 0.4;

pub const SUPPRESS_FIRST_MIN_M: f64 = 10.0;

pub const SUPPRESS_FIRST_OFF_M: f64 = 0.6;

pub const SUPPRESS_FIRST_JITTER_M: f64 = 0.4;

pub const SUPPRESS_FIRST_UP_M: f64 = 0.3;

pub const SUPPRESS_FIRST_WALK_M: f64 = 0.07;

pub const SUPPRESS_FIRST_COVER_M: f64 = 2.5;

pub const SUPPRESS_FIRST_LAPSE: f64 = 8.0;

pub const HEIGHTENED_SECS: f64 = 75.0;

pub const LOCKDOWN_SECS: f64 = 120.0;

pub const STEPDOWN_SECS: f64 = 45.0;

pub const KEEN: f64 = 1.25;

pub const ROLLCALL_CALM_SECS: f64 = 90.0;

pub const ROLLCALL_ALARMED_SECS: f64 = 45.0;

pub const ROLLCALL_JITTER: f64 = 0.1;

pub const ROLLCALL_ANSWER_SECS: f64 = 2.2;

pub const ROLLCALL_ASK_SECS: f64 = 4.0;

pub const ROLLCALL_SEND_SECS: f64 = 8.0;

pub const ROLLCALL_BUSY_RETRY_SECS: f64 = 15.0;

pub const ROLLCALL_QUIET_AFTER_SECS: f64 = 30.0;

pub const ROLLCALL_SEND_GIVEUP_SECS: f64 = 40.0;

pub const CHECK_LOOK_SECS: f64 = 1.2;

pub const CHECK_CAP_SECS: f64 = 150.0;

pub const PARTNER_UNSEEN_SECS: f64 = 20.0;

pub const PARTNER_SPOT_M: f64 = 2.5;

pub const RECLEAR_SECS: f64 = 60.0;

pub const TAMPER_MEMORY_SECS: f64 = 120.0;

pub const PARTNER_TRUST_SECS: f64 = 3.0;

pub const MAX_CHARACTERS: f64 = 8.0;

pub const MAX_SKINNED: f64 = 16.0;

pub const UNIT_ID0: f64 = 20.0;

pub const UNIT_ID1: f64 = 250.0;

pub fn team_tint() -> JsMap<String, V3> { { let mut t = JsMap::new(); t.set(String::from("guard"), [0.42, 0.36, 0.25]); t.set(String::from("operative"), [0.07, 0.09, 0.1]); t.set(String::from("hostile"), [0.36, 0.33, 0.24]); t.set(String::from("player"), [0.05, 0.06, 0.07]); t.set(String::from("civilian"), [0.75, 0.62, 0.08]); t } }

pub fn team_tint2() -> JsMap<String, V3> { { let mut t = JsMap::new(); t.set(String::from("guard"), [0.02, 0.02, 0.025]); t.set(String::from("operative"), [0.05, 0.07, 0.06]); t.set(String::from("hostile"), [0.09, 0.08, 0.05]); t.set(String::from("player"), [0.1, 0.32, 0.18]); t.set(String::from("civilian"), [0.05, 0.06, 0.1]); t } }

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum InHand {
    Torch,
    Weapon,
}
