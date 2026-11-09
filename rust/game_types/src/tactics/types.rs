//! The types of `tactics`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, Words, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// a man's price list for nav.findPath, fixed when asked: a cell of his side's dangerous doorways costs door_mult, a cell of his flank's avoid-set at least CORRIDOR_COST, any other 1 (tactics.ts tacticsPenalty)
#[derive(Clone, Debug)]
pub enum TacticsPenalty {
    Cells { w: f64, door_idx: Option<JsSet<crate::Num>>, door_mult: f64, avoid: Option<JsSet<crate::Num>> },
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum RoleKind {
    #[default] Second,
    Presser,
    Flanker,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Role {
    pub kind: crate::tactics::RoleKind,
    pub plan: TacticsPlanId,
    pub hold_at: Option<V3>,
    pub bearing: f64,
    pub avoid: Option<JsSet<crate::Num>>,
    pub set: bool,
    pub holding: bool,
    pub hold_t: f64,
    pub arrived: bool,
    pub said_moving: bool,
    pub said_set: bool,
    pub t0: f64,
    pub ground: Option<f64>,
    pub last_pos: Option<V3>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Plan {
    pub team: crate::teams::Team,
    pub target: V3,
    pub t0: f64,
    pub replanned_at: f64,
    pub members: Vec<GuardId>,
    pub roles: JsMap<GuardId, TacticsRoleId>,
    pub phase: crate::tactics::PlanPhase,
    pub go_t: f64,
    pub first_arrive_t: f64,
    pub last_radio_t: f64,
    pub contact: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum PlanPhase {
    #[default] Push,
    Approach,
    Done,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Lone {
    pub fix: V3,
    pub hold_at: Option<V3>,
    pub hold_t: f64,
    pub called: bool,
    pub released: bool,
    pub released_why: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum DangerWhy {
    #[default] Body,
    Drop,
    Press,
    Rounds,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DangerRec {
    pub door: DoorId,
    pub team: crate::teams::Team,
    pub until: f64,
    pub t0: f64,
    pub why: crate::tactics::DangerWhy,
    pub victim: Option<String>,
    pub banged: bool,
    pub entered: bool,
    pub called: bool,
    pub summoned: Option<GuardId>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum GatePhase {
    #[default] Slice,
    Bang,
    Cross,
    Stack,
    Done,
    Wait,
    Open,
    Toss,
    Stacked,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Gate {
    pub door: DoorId,
    pub rec: TacticsDangerRecId,
    pub phase: crate::tactics::GatePhase,
    pub t: f64,
    pub t0: f64,
    pub side: f64,
    pub near_a: f64,
    pub spot: V3,
    pub face: f64,
    pub role: crate::tactics::GateRole,
    pub lead: Option<GuardId>,
    pub slice_pts: Vec<V3>,
    pub slice_look: Vec<V3>,
    pub slice_i: f64,
    pub slice_t: f64,
    pub legs: Vec<V3>,
    pub leg_i: f64,
    pub cross_at: f64,
    pub look: Option<V3>,
    pub crouch: f64,
    pub path: Option<Vec<V3>>,
    pub path_i: f64,
    pub path_for: Option<V3>,
    pub can: Option<ItemId>,
    pub toss_t: f64,
    pub land_t: f64,
    pub bang_t: f64,
    pub goal_near: bool,
    pub opened: bool,
    pub crossed: bool,
    pub hs: Option<Option<crate::tactics::GateHs>>,
    pub action: crate::tactics::GateAction,
    pub thrower: Option<GuardId>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum GateRole {
    #[default] Lead,
    Second,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GateHs {
    pub pos: V3,
    pub yaw: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum GateAction {
    #[default] Slice,
    Toss,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct TState {
    pub plans: Vec<TacticsPlanId>,
    pub roles: JsMap<GuardId, TacticsRoleId>,
    pub lone: JsMap<GuardId, crate::tactics::Lone>,
    pub danger: Vec<TacticsDangerRecId>,
    pub gates: JsMap<GuardId, crate::tactics::Gate>,
    pub cleared: JsMap<GuardId, JsMap<DoorId, f64>>,
    pub seen_ev: JsSet<u32>,
    pub plan_serial: f64,
    pub last_lone_call_t: f64,
    pub deal_fail_t: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TacMove {
    pub move_: f64,
    pub aim: Option<V3>,
    pub pied: bool,
    pub crouch: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct TacticsModKitValue {
    pub stuns: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LoneCorner {
    pub p: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StackSpot {
    pub pos: V3,
    pub face: f64,
}

#[derive(Clone, Debug, Default)]
pub struct SliceStations {
    pub pts: Vec<V3>,
    pub look: Vec<V3>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GateMateRole {
    Lead,
    Second,
}

#[derive(Clone, Debug, Default)]
pub struct GateMate {
    pub gd: GuardId,
    pub g: crate::tactics::Gate,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TacticsLook {
    pub aim: V3,
    pub crouch: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct TacticsMod {
    /// (src/game/tactics.ts:149 states)
    pub states: Option<crate::tactics::TState>,
    /// (src/game/tactics.ts:156 kit)
    pub kit: JsMap<GuardId, crate::tactics::TacticsModKitValue>,
}
impl TacticsMod {
    pub fn blank() -> Self {
        TacticsMod {
            states: None,
            kit: JsMap::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct TacticsStats {
    pub plans: i32,
    pub replans: i32,
    pub flankers: i32,
    pub seconds: i32,
    pub swaps: i32,
    pub goes: i32,
    pub go_timeouts: i32,
    pub arrivals: i32,
    pub lone_holds: i32,
    pub lone_calls: i32,
    pub lone_releases: i32,
    pub dangers: i32,
    pub gates: i32,
    pub summons: i32,
    pub bangs: i32,
    pub slices: i32,
    pub entries: i32,
    pub lone_entries: i32,
}

/// (src/game/tactics.ts:86 JOIN_M)
pub const JOIN_M: f64 = 6.0;

/// (src/game/tactics.ts:86 SHARE_M)
pub const SHARE_M: f64 = 3.0;

/// (src/game/tactics.ts:86 DISC_M)
pub const DISC_M: f64 = 4.0;

/// (src/game/tactics.ts:86 FLANK_MIN_DEG)
pub const FLANK_MIN_DEG: f64 = 90.0;

/// (src/game/tactics.ts:86 FLANK_MAX_RATIO)
pub const FLANK_MAX_RATIO: f64 = 2.2;

/// (src/game/tactics.ts:86 CORRIDOR_COST)
pub const CORRIDOR_COST: f64 = 6.0;

/// (src/game/tactics.ts:86 CORRIDOR_DILATE)
pub const CORRIDOR_DILATE: f64 = 1.0;

/// (src/game/tactics.ts:86 SECTOR_M)
pub const SECTOR_M: f64 = 5.5;

/// (src/game/tactics.ts:86 SET_M)
pub const SET_M: f64 = 5.0;

/// (src/game/tactics.ts:86 BEHIND_M)
pub const BEHIND_M: f64 = 3.0;

/// (src/game/tactics.ts:86 BEHIND_SLACK_M)
pub const BEHIND_SLACK_M: f64 = 0.8;

/// (src/game/tactics.ts:86 REPLAN_M)
pub const REPLAN_M: f64 = 4.0;

/// (src/game/tactics.ts:89 PLAN_CAP_SECS)
pub const PLAN_CAP_SECS: f64 = 45.0;

/// (src/game/tactics.ts:89 ROLE_STALL_SECS)
pub const ROLE_STALL_SECS: f64 = 4.0;

/// (src/game/tactics.ts:89 REPLAN_GAP_SECS)
pub const REPLAN_GAP_SECS: f64 = 1.0;

/// (src/game/tactics.ts:89 ARRIVE_M)
pub const ARRIVE_M: f64 = 1.3;

/// (src/game/tactics.ts:92 LONE_FIGHT_M)
pub const LONE_FIGHT_M: f64 = 5.0;

/// (src/game/tactics.ts:92 LONE_HOLD_MIN_M)
pub const LONE_HOLD_MIN_M: f64 = 5.5;

/// (src/game/tactics.ts:92 LONE_COVER_R)
pub const LONE_COVER_R: f64 = 1.6;

/// (src/game/tactics.ts:92 ARMED_SHOT_SECS)
pub const ARMED_SHOT_SECS: f64 = 60.0;

/// (src/game/tactics.ts:92 ARMED_SEEN_SECS)
pub const ARMED_SEEN_SECS: f64 = 30.0;

/// (src/game/tactics.ts:97 DOOR_NEAR_M)
pub const DOOR_NEAR_M: f64 = 2.2;

/// (src/game/tactics.ts:97 ROUND_AT_M)
pub const ROUND_AT_M: f64 = 3.0;

/// (src/game/tactics.ts:97 GATE_DETECT_M)
pub const GATE_DETECT_M: f64 = 3.5;

/// (src/game/tactics.ts:97 GATE_GOAL_M)
pub const GATE_GOAL_M: f64 = 2.0;

/// (src/game/tactics.ts:97 DOOR_COST_MIN)
pub const DOOR_COST_MIN: f64 = 20.0;

/// (src/game/tactics.ts:97 STACK_ALONG_M)
pub const STACK_ALONG_M: f64 = 0.9;

/// (src/game/tactics.ts:97 STACK_OFF_M)
pub const STACK_OFF_M: f64 = 0.55;

/// (src/game/tactics.ts:97 STACK_GAP_M)
pub const STACK_GAP_M: f64 = 0.85;

/// (src/game/tactics.ts:98 SLICE_BACK: a table nothing writes)
pub fn slice_back() -> Vec<f64> { alloc::vec![2.5, 2.0, 1.6, 1.25] }

/// (src/game/tactics.ts:98 SLICE_STEPS)
pub const SLICE_STEPS: f64 = 4.0;

/// (src/game/tactics.ts:98 SLICE_DWELL)
pub const SLICE_DWELL: f64 = 0.25;

/// (src/game/tactics.ts:98 SLICE_PACE)
pub const SLICE_PACE: f64 = 2.4;

/// (src/game/tactics.ts:98 ENTRY_IN_M)
pub const ENTRY_IN_M: f64 = 1.5;

/// (src/game/tactics.ts:98 ENTRY_SPLIT_DEG)
pub const ENTRY_SPLIT_DEG: f64 = 60.0;

/// (src/game/tactics.ts:98 ENTRY_LEG_M)
pub const ENTRY_LEG_M: f64 = 1.3;

/// (src/game/tactics.ts:98 GATE_CAP_SECS)
pub const GATE_CAP_SECS: f64 = 22.0;

/// (src/game/tactics.ts:98 SUMMON_M)
pub const SUMMON_M: f64 = 25.0;

/// (src/game/tactics.ts:100 TOSS_IN_M)
pub const TOSS_IN_M: f64 = 2.5;

/// (src/game/tactics.ts:100 TOSS_FUSE)
pub const TOSS_FUSE: f64 = 0.6;

/// (src/game/tactics.ts:100 TOSS_AIR_CAP)
pub const TOSS_AIR_CAP: f64 = 2.2;

/// (src/game/tactics.ts:100 GO_AFTER_BANG)
pub const GO_AFTER_BANG: f64 = 0.35;

