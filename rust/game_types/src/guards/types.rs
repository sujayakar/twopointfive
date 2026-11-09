//! The types of `guards`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// what litPlace last read the light for besides the man himself: the man he has, or the point he is turning to (guards.ts litPlace's `key`; null is the Option's None)
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
    pub result: Option<crate::guards::EngageResult>,
    pub kills: Vec<crate::guards::EngageResultKillsItem>,
    pub rounds: crate::guards::EngagementRounds,
    pub alive: JsSet<GuardId>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct EngageResult {
    pub outcome: crate::guards::EngageResultOutcome,
    pub secs: f64,
    pub kills: Vec<crate::guards::EngageResultKillsItem>,
    pub rounds: crate::guards::EngageResultRounds,
    pub operatives_left: f64,
    pub hostiles_left: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum EngageResultOutcome {
    #[default] Wiped,
    Reset,
    Cleared,
    Timeout,
    Superseded,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct EngageResultKillsItem {
    pub victim: String,
    pub team: crate::teams::Team,
    pub by: String,
    pub t: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct EngageResultRounds {
    pub guard: Option<f64>,
    pub operative: Option<f64>,
    pub hostile: Option<f64>,
    pub player: Option<f64>,
    pub civilian: Option<f64>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct EngagementRounds {
    pub guard: Option<f64>,
    pub operative: Option<f64>,
    pub hostile: Option<f64>,
    pub player: Option<f64>,
    pub civilian: Option<f64>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Standoff {
    pub pair: GuardId,
    pub since: f64,
    pub side: f64,
    pub way: f64,
    pub orbit_goal: Option<V3>,
    pub replan_t: f64,
    pub rest_t: f64,
    pub seen_t: f64,
    pub last_bark_t: f64,
    pub line_open: bool,
    pub shots: f64,
    pub orbit_m: f64,
    pub stalls: f64,
    pub cornered: bool,
    pub backing: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Pair {
    pub key: u32,
    pub held: GuardId,
    pub sam: V3,
    pub held_axis: V3,
    pub centre: V3,
    pub facing: f64,
    pub aim: V3,
    pub vel: V3,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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
impl StandoffMoment {
    pub const fn as_str(self) -> &'static str {
        match self {
            StandoffMoment::Lost => "lost",
            StandoffMoment::Enter => "enter",
            StandoffMoment::Cornered => "cornered",
            StandoffMoment::NoShot => "noShot",
            StandoffMoment::Line => "line",
            StandoffMoment::Dropped => "dropped",
            StandoffMoment::Freed => "freed",
            StandoffMoment::HitHim => "hitHim",
        }
    }
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
    pub hidden: bool,
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
    pub visits: f64,
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
    pub e: f64,
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

#[derive(Clone, Debug, Default)]
pub struct STANDOFFLINESValue {
    pub steady: Option<Vec<String>>,
    pub nervy: Option<Vec<String>>,
    pub hard: Option<Vec<String>>,
    pub under: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default)]
pub struct STANDOFFNAMELESS {
    pub lost: Option<String>,
    pub enter: Option<String>,
    pub cornered: Option<String>,
    pub no_shot: Option<String>,
    pub line: Option<String>,
    pub dropped: Option<String>,
    pub freed: Option<String>,
    pub hit_him: Option<String>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GuardsModUNSTICK {
    pub legs: f64,
    pub snaps: f64,
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
pub struct GuardsModDarkSeenValue {
    pub t: f64,
    pub dark: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GuardsModWallSeenValue {
    pub t: f64,
    pub since: f64,
    pub dark: bool,
    pub out: Option<bool>,
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
    pub in_cone: bool,
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
pub enum AlertCause {
    Body,
    Radio,
    Sound,
    Sight,
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
    pub clear: f64,
    pub dist: f64,
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
    pub in_cone: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CanSeeChar {
    pub visible: bool,
    pub dist: f64,
    pub in_cone: bool,
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RigOf {
    Pistol,
    Carbine,
    Sc20k,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LongRigOf {
    Carbine,
    Sc20k,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HandCarry {
    Torch,
    Weapon,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum InHand {
    Torch,
    Weapon,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RaiseEscalationCause {
    Search,
    Civilian,
    Device,
    Missing,
    Body,
    Hostage,
}
impl RaiseEscalationCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            RaiseEscalationCause::Search => "search",
            RaiseEscalationCause::Civilian => "civilian",
            RaiseEscalationCause::Device => "device",
            RaiseEscalationCause::Missing => "missing",
            RaiseEscalationCause::Body => "body",
            RaiseEscalationCause::Hostage => "hostage",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PlanClearsCause {
    Search,
    Missing,
    Body,
    Panel,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum EndEngagementOutcome {
    Wiped,
    Reset,
    Cleared,
    Timeout,
    Superseded,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct GuardsMod {
    /// (src/game/guards.ts:270 groundTrack)
    pub ground_track: JsMap<PlayerId, crate::guards::GuardsModGroundTrackValue>,
    /// (src/game/guards.ts:370 bodyMeters)
    pub body_meters: JsMap<GuardId, crate::guards::GuardsModBodyMetersValue>,
    /// (src/game/guards.ts:874 holdsCalled)
    pub holds_called: JsSet<u32>,
    /// (src/game/guards.ts:874 floorBarkT)
    pub floor_bark_t: Option<f64>,
    /// (src/game/guards.ts:874 lastBark)
    pub last_bark: JsMap<GuardId, String>,
    /// (src/game/guards.ts:915 heldMeters)
    pub held_meters: JsMap<u32, crate::guards::GuardsModHeldMetersValue>,
    /// (src/game/guards.ts:1364 unstickLegs)
    pub unstick_legs: JsMap<GuardId, crate::guards::UnstickLeg>,
    /// (src/game/guards.ts:1366 unstickRetryAt)
    pub unstick_retry_at: JsMap<GuardId, f64>,
    /// (src/game/guards.ts:1368 heldLastFrame)
    pub held_last_frame: JsSet<GuardId>,
    /// (src/game/guards.ts:1371 UNSTICK)
    pub unstick: crate::guards::GuardsModUNSTICK,
    /// (src/game/guards.ts:1545 litSeen)
    pub lit_seen: JsMap<GuardId, crate::guards::GuardsModLitSeenValue>,
    /// (src/game/guards.ts:1563 darkSeen)
    pub dark_seen: JsMap<GuardId, crate::guards::GuardsModDarkSeenValue>,
    /// (src/game/guards.ts:1577 litUntil)
    pub lit_until: JsMap<GuardId, f64>,
    /// (src/game/guards.ts:1579 wallSeen)
    pub wall_seen: JsMap<GuardId, crate::guards::GuardsModWallSeenValue>,
    /// (src/game/guards.ts:1642 torchLowSeen)
    pub torch_low_seen: JsMap<GuardId, crate::guards::GuardsModTorchLowSeenValue>,
    /// (src/game/guards.ts:1665 litManSeen)
    pub lit_man_seen: JsMap<GuardId, crate::guards::GuardsModLitManSeenValue>,
    /// (src/game/guards.ts:1801 doorCellsOf)
    pub door_cells_of: Option<JsMap<crate::Num, DoorId>>,
    /// (src/game/guards.ts:3035 rollcalls)
    pub rollcalls: Option<crate::guards::Rollcall>,
    /// (src/game/guards.ts:3472 unitSerial)
    pub unit_serial: f64,
    /// (src/game/guards.ts:3535 engagements)
    pub engagements: Option<crate::guards::Engagement>,
}
impl GuardsMod {
    pub fn blank() -> Self {
        GuardsMod {
            ground_track: JsMap::new(),
            body_meters: JsMap::new(),
            holds_called: JsSet::new(),
            floor_bark_t: None,
            last_bark: JsMap::new(),
            held_meters: JsMap::new(),
            unstick_legs: JsMap::new(),
            unstick_retry_at: JsMap::new(),
            held_last_frame: JsSet::new(),
            unstick: crate::guards::GuardsModUNSTICK { legs: 0.0, snaps: 0.0, last_snap_t: -1.0, last_snap_who: None },
            lit_seen: JsMap::new(),
            dark_seen: JsMap::new(),
            lit_until: JsMap::new(),
            wall_seen: JsMap::new(),
            torch_low_seen: JsMap::new(),
            lit_man_seen: JsMap::new(),
            door_cells_of: None,
            rollcalls: None,
            unit_serial: 0.0,
            engagements: None,
        }
    }
}

/// (src/game/guards.ts:58 OCP_MUTTER: a table nothing writes)
pub fn ocp_mutter() -> Vec<String> { alloc::vec![String::from("light's on the blink again"), String::from("hm — bulb's going"), String::from("power dip?"), String::from("these fittings…"), String::from("that light again")] }

/// (src/game/guards.ts:261 MOTION_STILL)
pub const MOTION_STILL: f64 = 0.35;

/// (src/game/guards.ts:261 MOTION_LO)
pub const MOTION_LO: f64 = 0.05;

/// (src/game/guards.ts:261 MOTION_HI)
pub const MOTION_HI: f64 = 1.2;

/// (src/game/guards.ts:261 MOTION_LIT_LO)
pub const MOTION_LIT_LO: f64 = 0.3;

/// (src/game/guards.ts:261 MOTION_LIT_HI)
pub const MOTION_LIT_HI: f64 = 0.75;

/// (src/game/guards.ts:263 MOTION_RADIAL)
pub const MOTION_RADIAL: f64 = 0.4;

/// (src/game/guards.ts:266 MOTION_SPRINT)
pub const MOTION_SPRINT: f64 = 1.6;

/// (src/game/guards.ts:266 MOTION_SPRINT_MS)
pub const MOTION_SPRINT_MS: f64 = 3.5;

/// (src/game/guards.ts:266 MOTION_NEAR_M)
pub const MOTION_NEAR_M: f64 = 2.5;

/// (src/game/guards.ts:266 MOTION_NEAR_FLOOR)
pub const MOTION_NEAR_FLOOR: f64 = 0.8;

/// (src/game/guards.ts:266 MOTION_HUNT_FLOOR)
pub const MOTION_HUNT_FLOOR: f64 = 0.7;

/// (src/game/guards.ts:493 BODY_DUTY_CAP)
pub const BODY_DUTY_CAP: f64 = 45.0;

/// (src/game/guards.ts:661 MUZZLE_CLEAR_M)
pub const MUZZLE_CLEAR_M: f64 = 0.35;

/// (src/game/guards.ts:661 MUZZLE_LEAD_SECS)
pub const MUZZLE_LEAD_SECS: f64 = 0.3;

/// (src/game/guards.ts:661 MUZZLE_LEAD_MAX_M)
pub const MUZZLE_LEAD_MAX_M: f64 = 0.4;

/// (src/game/guards.ts:661 MUZZLE_PAST_M)
pub const MUZZLE_PAST_M: f64 = 1.0;

/// (src/game/guards.ts:661 MUZZLE_REACH_M)
pub const MUZZLE_REACH_M: f64 = 8.0;

/// (src/game/guards.ts:661 MUZZLE_UP_SECS)
pub const MUZZLE_UP_SECS: f64 = 0.2;

/// (src/game/guards.ts:664 LINE_STEP_MARGIN_M)
pub const LINE_STEP_MARGIN_M: f64 = 0.12;

/// (src/game/guards.ts:664 LINE_STEP_MAX_M)
pub const LINE_STEP_MAX_M: f64 = 2.2;

/// (src/game/guards.ts:664 LINE_PUSH_MIN_M)
pub const LINE_PUSH_MIN_M: f64 = 2.5;

/// (src/game/guards.ts:664 LINE_BARK_SECS)
pub const LINE_BARK_SECS: f64 = 10.0;

/// (src/game/guards.ts:668 LINE_STUCK_SECS)
pub const LINE_STUCK_SECS: f64 = 0.4;

/// (src/game/guards.ts:668 LINE_FAIL_SECS)
pub const LINE_FAIL_SECS: f64 = 5.0;

/// (src/game/guards.ts:668 LINE_FAIL_M)
pub const LINE_FAIL_M: f64 = 0.75;

/// (src/game/guards.ts:671 LINE_WALK_CLEAR_M)
pub const LINE_WALK_CLEAR_M: f64 = 0.5;

/// (src/game/guards.ts:826 STANDOFF_MIN_M)
pub const STANDOFF_MIN_M: f64 = 3.5;

/// (src/game/guards.ts:826 STANDOFF_MAX_M)
pub const STANDOFF_MAX_M: f64 = 7.0;

/// (src/game/guards.ts:826 STANDOFF_ORBIT_M)
pub const STANDOFF_ORBIT_M: f64 = 4.5;

/// (src/game/guards.ts:826 STANDOFF_SETTLE_DEG)
pub const STANDOFF_SETTLE_DEG: f64 = 22.0;

/// (src/game/guards.ts:826 STANDOFF_FIRE_MAX_M)
pub const STANDOFF_FIRE_MAX_M: f64 = 5.0;

/// (src/game/guards.ts:831 STANDOFF_REAR_DEG)
pub const STANDOFF_REAR_DEG: f64 = 70.0;

/// (src/game/guards.ts:831 STANDOFF_CLEAR_M)
pub const STANDOFF_CLEAR_M: f64 = 0.45;

/// (src/game/guards.ts:831 STANDOFF_SHOT_SECS)
pub const STANDOFF_SHOT_SECS: f64 = 1.2;

/// (src/game/guards.ts:831 STANDOFF_AIM_Y)
pub const STANDOFF_AIM_Y: f64 = 1.42;

/// (src/game/guards.ts:833 SAM_R)
pub const SAM_R: f64 = 0.28;

/// (src/game/guards.ts:837 STANDOFF_VIEW_M)
pub const STANDOFF_VIEW_M: f64 = 16.0;

/// (src/game/guards.ts:837 STANDOFF_VIEW_DEG)
pub const STANDOFF_VIEW_DEG: f64 = 60.0;

/// (src/game/guards.ts:837 STANDOFF_LOSE_SECS)
pub const STANDOFF_LOSE_SECS: f64 = 4.0;

/// (src/game/guards.ts:837 STANDOFF_REPLAN_SECS)
pub const STANDOFF_REPLAN_SECS: f64 = 1.0;

/// (src/game/guards.ts:837 STANDOFF_STALL_SECS)
pub const STANDOFF_STALL_SECS: f64 = 2.5;

/// (src/game/guards.ts:837 STANDOFF_REST_SECS)
pub const STANDOFF_REST_SECS: f64 = 1.5;

/// (src/game/guards.ts:839 STANDOFF_BARK_SECS)
pub const STANDOFF_BARK_SECS: f64 = 6.0;

/// (src/game/guards.ts:839 STANDOFF_BARK_FLOOR_SECS)
pub const STANDOFF_BARK_FLOOR_SECS: f64 = 3.0;

/// (src/game/guards.ts:846 SHIELD_RELEASE_BEAT_MIN)
pub const SHIELD_RELEASE_BEAT_MIN: f64 = 0.35;

/// (src/game/guards.ts:846 SHIELD_RELEASE_BEAT_JITTER)
pub const SHIELD_RELEASE_BEAT_JITTER: f64 = 0.3;

/// (src/game/guards.ts:900 PAIR_LEGIBLE)
pub const PAIR_LEGIBLE: f64 = 3.0;

/// (src/game/guards.ts:914 HELD_METER: a table nothing writes)
pub fn held_meter() -> crate::guards::HELDMETER { crate::guards::HELDMETER { gap_secs: 0.5, moved_m: 1.0 } }

/// (src/game/guards.ts:1003 STANDOFF_LINES: a table nothing writes)
pub fn standoff_lines() -> JsMap<String, crate::guards::STANDOFFLINESValue> { { let mut t = JsMap::new(); t.set(String::from("enter"), crate::guards::STANDOFFLINESValue { steady: Some(alloc::vec![String::from("{held}, hold still — we've got you"), String::from("easy. easy. nobody has to get hurt here")]), nervy: Some(alloc::vec![String::from("{held}! —jesus— okay, okay, nobody shoot!"), String::from("oh god— he's got {held}—")]), hard: Some(alloc::vec![String::from("let {him} go. now."), String::from("you. hands off {him}.")]), under: None }); t.set(String::from("noShot"), crate::guards::STANDOFFLINESValue { steady: Some(alloc::vec![String::from("let {him} go and we talk about it"), String::from("{held}, stay loose — I'm coming round")]), nervy: Some(alloc::vec![String::from("{held}! —I can't— I haven't got a line—"), String::from("don't you hurt {him}— don't—")]), hard: Some(alloc::vec![String::from("you've got nowhere to take {him}"), String::from("the second {he} slips, you're done")]), under: Some(alloc::vec![String::from("let {him} go!"), String::from("drop {him} — NOW"), String::from("{held}, don't move"), String::from("I don't have a shot"), String::from("no shot — {held}'s square in front of him")]) }); t.set(String::from("line"), crate::guards::STANDOFFLINESValue { steady: None, nervy: Some(alloc::vec![String::from("I— I think I have him—")]), hard: Some(alloc::vec![String::from("there you are.")]), under: Some(alloc::vec![String::from("I have him."), String::from("got a line —")]) }); t.set(String::from("cornered"), crate::guards::STANDOFFLINESValue { steady: None, nervy: Some(alloc::vec![String::from("just— just let {him} go, please—")]), hard: Some(alloc::vec![String::from("one way out of this for you. through us.")]), under: Some(alloc::vec![String::from("can't get round him — hold what you've got"), String::from("no way round. {held}, hang on"), String::from("nowhere for either of us to go — let {him} walk and this ends quiet")]) }); t.set(String::from("dropped"), crate::guards::STANDOFFLINESValue { steady: None, nervy: Some(alloc::vec![String::from("{held}! —he's— shoot, SHOOT—")]), hard: None, under: Some(alloc::vec![String::from("he's dropped {held} — TAKE HIM"), String::from("{held}'s down — light him up!")]) }); t.set(String::from("freed"), crate::guards::STANDOFFLINESValue { steady: None, nervy: None, hard: None, under: Some(alloc::vec![String::from("{held}, DOWN — get clear!"), String::from("he's loose — {held}, out of the way!")]) }); t.set(String::from("lost"), crate::guards::STANDOFFLINESValue { steady: None, nervy: Some(alloc::vec![String::from("where— where did they go—")]), hard: None, under: Some(alloc::vec![String::from("lost them — where'd he take {him}?"), String::from("I've lost sight — moving up")]) }); t.set(String::from("hitHim"), crate::guards::STANDOFFLINESValue { steady: None, nervy: Some(alloc::vec![String::from("no— no no no— {held}—")]), hard: Some(alloc::vec![String::from("…{held}'s hit. that's on him. keep on him—")]), under: Some(alloc::vec![String::from("…christ— I hit {him}. I hit {held}—")]) }); t } }

/// (src/game/guards.ts:1014 STANDOFF_NOW: a table nothing writes)
pub fn standoff_now() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("{held}, stay loose — I'm coming round"), String::from("{held}, stay loose — I'm coming around")); t.set(String::from("{held}! —I can't— I haven't got a line—"), String::from("{held}! —I can't— I don't have a shot—")); t.set(String::from("I have him."), String::from("I've got him.")); t.set(String::from("I— I think I have him—"), String::from("I— I think I've got him—")); t.set(String::from("can't get round him — hold what you've got"), String::from("can't get around him — hold what you've got")); t.set(String::from("no way round. {held}, hang on"), String::from("no way around. {held}, hang on")); t } }

/// (src/game/guards.ts:1020 STANDOFF_NAMELESS: a table nothing writes)
pub fn standoff_nameless() -> crate::guards::STANDOFFNAMELESS { crate::guards::STANDOFFNAMELESS { lost: None, enter: None, cornered: None, no_shot: None, line: None, dropped: Some(String::from("he's dropped {him} — TAKE HIM")), freed: Some(String::from("he's loose — get clear!")), hit_him: Some(String::from("…christ— I hit {him}—")) } }

/// (src/game/guards.ts:1360 UNSTICK_REACH_M)
pub const UNSTICK_REACH_M: f64 = 2.4;

/// (src/game/guards.ts:1360 UNSTICK_STALL_SECS)
pub const UNSTICK_STALL_SECS: f64 = 1.0;

/// (src/game/guards.ts:1360 UNSTICK_DONE_M)
pub const UNSTICK_DONE_M: f64 = 0.08;

/// (src/game/guards.ts:1360 UNSTICK_SNAP_REACH_M)
pub const UNSTICK_SNAP_REACH_M: f64 = 3.5;

/// (src/game/guards.ts:1360 UNSTICK_BODY_R)
pub const UNSTICK_BODY_R: f64 = 0.15;

/// (src/game/guards.ts:1586 FLOOR_TOP)
pub const FLOOR_TOP: f64 = 0.249;

/// (src/game/guards.ts:1759 SEARCH_RADIUS_M)
pub const SEARCH_RADIUS_M: f64 = 9.0;

/// (src/game/guards.ts:1759 SEARCH_SPOTS)
pub const SEARCH_SPOTS: f64 = 12.0;

/// (src/game/guards.ts:1759 SEARCH_CONTROLS)
pub const SEARCH_CONTROLS: f64 = 3.0;

/// (src/game/guards.ts:1759 SEARCH_SPACING_M)
pub const SEARCH_SPACING_M: f64 = 1.5;

/// (src/game/guards.ts:1762 SEARCH_SWEEP0_SECS)
pub const SEARCH_SWEEP0_SECS: f64 = 1.2;

/// (src/game/guards.ts:1762 SEARCH_LOOK_SECS)
pub const SEARCH_LOOK_SECS: f64 = 1.5;

/// (src/game/guards.ts:1762 SEARCH_LOOK_DIST_M)
pub const SEARCH_LOOK_DIST_M: f64 = 1.9;

/// (src/game/guards.ts:1765 SEARCH_MIN_SECS)
pub const SEARCH_MIN_SECS: f64 = 12.0;

/// (src/game/guards.ts:1765 SEARCH_CAP_SECS)
pub const SEARCH_CAP_SECS: f64 = 26.0;

/// (src/game/guards.ts:1765 SEARCH_REFLOOD_SECS)
pub const SEARCH_REFLOOD_SECS: f64 = 2.0;

/// (src/game/guards.ts:1769 PROBE_BUDGET)
pub const PROBE_BUDGET: f64 = 8.0;

/// (src/game/guards.ts:1769 PROBE_HEIGHT)
pub const PROBE_HEIGHT: f64 = 0.5;

/// (src/game/guards.ts:1769 DARK_WINDOW_SECS)
pub const DARK_WINDOW_SECS: f64 = 3.0;

/// (src/game/guards.ts:1772 LATCH_SECS)
pub const LATCH_SECS: f64 = 5.5;

/// (src/game/guards.ts:1774 PLAUS_SLACK_SECS)
pub const PLAUS_SLACK_SECS: f64 = 1.2;

/// (src/game/guards.ts:1774 PLAUS_RAMP_SECS)
pub const PLAUS_RAMP_SECS: f64 = 2.5;

/// (src/game/guards.ts:1990 PIE_DETECT_M)
pub const PIE_DETECT_M: f64 = 3.2;

/// (src/game/guards.ts:1990 PIE_SLOW_M)
pub const PIE_SLOW_M: f64 = 1.5;

/// (src/game/guards.ts:1990 PIE_SLICE_M)
pub const PIE_SLICE_M: f64 = 1.7;

/// (src/game/guards.ts:1990 PIE_SLICE_SECS)
pub const PIE_SLICE_SECS: f64 = 1.0;

/// (src/game/guards.ts:1990 PIE_CORNER_SECS)
pub const PIE_CORNER_SECS: f64 = 0.4;

/// (src/game/guards.ts:1990 PIE_AGAIN_SECS)
pub const PIE_AGAIN_SECS: f64 = 6.0;

/// (src/game/guards.ts:1990 PIE_CAP_SECS)
pub const PIE_CAP_SECS: f64 = 7.0;

/// (src/game/guards.ts:1992 PIE_EYES_SECS)
pub const PIE_EYES_SECS: f64 = 1.0;

/// (src/game/guards.ts:2549 GUARD_SWEEP_STEP)
pub const GUARD_SWEEP_STEP: f64 = 0.2;

/// (src/game/guards.ts:2551 SEP_MAX_M)
pub const SEP_MAX_M: f64 = 0.25;

/// (src/game/guards.ts:2555 SEP_SETTLE_MIN)
pub const SEP_SETTLE_MIN: f64 = 0.02;

/// (src/game/guards.ts:2601 NO_SIGHT: a table nothing writes)
pub fn no_sight() -> crate::guards::NOSIGHT { crate::guards::NOSIGHT { visible: false, dist: 99.0, in_cone: false } }

/// (src/game/guards.ts:2799 BURST_BROKEN_CD)
pub const BURST_BROKEN_CD: f64 = 0.4;

/// (src/game/guards.ts:2799 SUPPRESS_FIRST_MIN_M)
pub const SUPPRESS_FIRST_MIN_M: f64 = 10.0;

/// (src/game/guards.ts:2799 SUPPRESS_FIRST_OFF_M)
pub const SUPPRESS_FIRST_OFF_M: f64 = 0.6;

/// (src/game/guards.ts:2799 SUPPRESS_FIRST_JITTER_M)
pub const SUPPRESS_FIRST_JITTER_M: f64 = 0.4;

/// (src/game/guards.ts:2799 SUPPRESS_FIRST_UP_M)
pub const SUPPRESS_FIRST_UP_M: f64 = 0.3;

/// (src/game/guards.ts:2799 SUPPRESS_FIRST_WALK_M)
pub const SUPPRESS_FIRST_WALK_M: f64 = 0.07;

/// (src/game/guards.ts:2799 SUPPRESS_FIRST_COVER_M)
pub const SUPPRESS_FIRST_COVER_M: f64 = 2.5;

/// (src/game/guards.ts:2799 SUPPRESS_FIRST_LAPSE)
pub const SUPPRESS_FIRST_LAPSE: f64 = 8.0;

/// (src/game/guards.ts:2959 HEIGHTENED_SECS)
pub const HEIGHTENED_SECS: f64 = 75.0;

/// (src/game/guards.ts:2959 LOCKDOWN_SECS)
pub const LOCKDOWN_SECS: f64 = 120.0;

/// (src/game/guards.ts:2959 STEPDOWN_SECS)
pub const STEPDOWN_SECS: f64 = 45.0;

/// (src/game/guards.ts:2961 KEEN)
pub const KEEN: f64 = 1.25;

/// (src/game/guards.ts:3023 ROLLCALL_CALM_SECS)
pub const ROLLCALL_CALM_SECS: f64 = 90.0;

/// (src/game/guards.ts:3023 ROLLCALL_ALARMED_SECS)
pub const ROLLCALL_ALARMED_SECS: f64 = 45.0;

/// (src/game/guards.ts:3023 ROLLCALL_JITTER)
pub const ROLLCALL_JITTER: f64 = 0.1;

/// (src/game/guards.ts:3025 ROLLCALL_ANSWER_SECS)
pub const ROLLCALL_ANSWER_SECS: f64 = 2.2;

/// (src/game/guards.ts:3025 ROLLCALL_ASK_SECS)
pub const ROLLCALL_ASK_SECS: f64 = 4.0;

/// (src/game/guards.ts:3025 ROLLCALL_SEND_SECS)
pub const ROLLCALL_SEND_SECS: f64 = 8.0;

/// (src/game/guards.ts:3028 ROLLCALL_BUSY_RETRY_SECS)
pub const ROLLCALL_BUSY_RETRY_SECS: f64 = 15.0;

/// (src/game/guards.ts:3028 ROLLCALL_QUIET_AFTER_SECS)
pub const ROLLCALL_QUIET_AFTER_SECS: f64 = 30.0;

/// (src/game/guards.ts:3028 ROLLCALL_SEND_GIVEUP_SECS)
pub const ROLLCALL_SEND_GIVEUP_SECS: f64 = 40.0;

/// (src/game/guards.ts:3030 CHECK_LOOK_SECS)
pub const CHECK_LOOK_SECS: f64 = 1.2;

/// (src/game/guards.ts:3030 CHECK_CAP_SECS)
pub const CHECK_CAP_SECS: f64 = 150.0;

/// (src/game/guards.ts:3032 PARTNER_UNSEEN_SECS)
pub const PARTNER_UNSEEN_SECS: f64 = 20.0;

/// (src/game/guards.ts:3032 PARTNER_SPOT_M)
pub const PARTNER_SPOT_M: f64 = 2.5;

/// (src/game/guards.ts:3205 RECLEAR_SECS)
pub const RECLEAR_SECS: f64 = 60.0;

/// (src/game/guards.ts:3207 TAMPER_MEMORY_SECS)
pub const TAMPER_MEMORY_SECS: f64 = 120.0;

/// (src/game/guards.ts:3341 PARTNER_TRUST_SECS)
pub const PARTNER_TRUST_SECS: f64 = 3.0;

/// (src/game/guards.ts:3467 MAX_CHARACTERS)
pub const MAX_CHARACTERS: f64 = 8.0;

/// (src/game/guards.ts:3469 MAX_SKINNED)
pub const MAX_SKINNED: f64 = 16.0;

/// (src/game/guards.ts:3471 UNIT_ID0)
pub const UNIT_ID0: f64 = 20.0;

/// (src/game/guards.ts:3471 UNIT_ID1)
pub const UNIT_ID1: f64 = 250.0;

/// (src/game/guards.ts:3474 TEAM_TINT: a table nothing writes)
pub fn team_tint() -> JsMap<String, V3> { { let mut t = JsMap::new(); t.set(String::from("guard"), [0.42, 0.36, 0.25]); t.set(String::from("operative"), [0.07, 0.09, 0.1]); t.set(String::from("hostile"), [0.36, 0.33, 0.24]); t.set(String::from("player"), [0.05, 0.06, 0.07]); t.set(String::from("civilian"), [0.75, 0.62, 0.08]); t } }

/// (src/game/guards.ts:3475 TEAM_TINT2: a table nothing writes)
pub fn team_tint2() -> JsMap<String, V3> { { let mut t = JsMap::new(); t.set(String::from("guard"), [0.02, 0.02, 0.025]); t.set(String::from("operative"), [0.05, 0.07, 0.06]); t.set(String::from("hostile"), [0.09, 0.08, 0.05]); t.set(String::from("player"), [0.1, 0.32, 0.18]); t.set(String::from("civilian"), [0.05, 0.06, 0.1]); t } }

