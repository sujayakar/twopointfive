//! The types of `tactics`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// a man's price list for nav.findPath, fixed when asked: a cell of his side's dangerous doorways costs door_mult, a cell of his flank's avoid-set at least CORRIDOR_COST, any other 1 (tactics.ts tacticsPenalty)
#[derive(Clone, Debug)]
pub enum TacticsPenalty {
    Cells { w: f64, door_idx: Option<JsSet<crate::Num>>, door_mult: f64, avoid: Option<JsSet<crate::Num>> },
}

/// (TypeScript: TacticsTune, src/game/tactics.ts:62)
#[derive(Clone, Copy, Debug)]
pub struct TacticsTune {
    pub tactics: bool,
    pub flank_wait: f64,
    pub lone_wait: f64,
    pub door_danger: f64,
    pub door_wait: f64,
}
impl TacticsTune {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TacticsTune {
            tactics: false,
            flank_wait: 0.0,
            lone_wait: 0.0,
            door_danger: 0.0,
            door_wait: 0.0,
        }
    }
}
impl Default for TacticsTune { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RoleKind {
    Second,
    Presser,
    Flanker,
}
impl RoleKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            RoleKind::Second => "second",
            RoleKind::Presser => "presser",
            RoleKind::Flanker => "flanker",
        }
    }
}

/// (TypeScript: Role, src/game/tactics.ts:103)
#[derive(Clone, Debug)]
pub struct Role {
    pub kind: crate::tactics::RoleKind,
    pub plan: TacticsPlanId,
    pub hold_at: Option<V3>,
    pub route: Vec<V3>,
    pub len: f64,
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
impl Role {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Role {
            kind: crate::tactics::RoleKind::Second,
            plan: TacticsPlanId(0),
            hold_at: None,
            route: Vec::new(),
            len: 0.0,
            bearing: 0.0,
            avoid: None,
            set: false,
            holding: false,
            hold_t: 0.0,
            arrived: false,
            said_moving: false,
            said_set: false,
            t0: 0.0,
            ground: None,
            last_pos: None,
        }
    }
}
impl Default for Role { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Plan, src/game/tactics.ts:115)
#[derive(Clone, Debug)]
pub struct Plan {
    pub id: f64,
    pub team: crate::teams::Team,
    pub target: V3,
    pub t0: f64,
    pub replanned_at: f64,
    pub replans: f64,
    pub members: Vec<GuardId>,
    pub roles: JsMap<GuardId, TacticsRoleId>,
    pub phase: crate::tactics::PlanPhase,
    pub go_t: f64,
    pub why: String,
    pub presser_bearing: f64,
    pub presser_len: f64,
    pub corridor: JsSet<crate::Num>,
    pub first_arrive_t: f64,
    pub last_radio_t: f64,
    pub contact: bool,
}
impl Plan {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Plan {
            id: 0.0,
            team: crate::teams::Team::Guard,
            target: [0.0; 3],
            t0: 0.0,
            replanned_at: 0.0,
            replans: 0.0,
            members: Vec::new(),
            roles: JsMap::new(),
            phase: crate::tactics::PlanPhase::Push,
            go_t: 0.0,
            why: String::new(),
            presser_bearing: 0.0,
            presser_len: 0.0,
            corridor: JsSet::new(),
            first_arrive_t: 0.0,
            last_radio_t: 0.0,
            contact: false,
        }
    }
}
impl Default for Plan { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PlanPhase {
    Push,
    Approach,
    Done,
}
impl PlanPhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            PlanPhase::Push => "push",
            PlanPhase::Approach => "approach",
            PlanPhase::Done => "done",
        }
    }
}

/// (TypeScript: Lone, src/game/tactics.ts:122)
#[derive(Clone, Debug)]
pub struct Lone {
    pub fix: V3,
    pub hold_at: Option<V3>,
    pub los: bool,
    pub t0: f64,
    pub hold_t: f64,
    pub called: bool,
    pub released: bool,
    pub released_why: String,
}
impl Lone {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Lone {
            fix: [0.0; 3],
            hold_at: None,
            los: false,
            t0: 0.0,
            hold_t: 0.0,
            called: false,
            released: false,
            released_why: String::new(),
        }
    }
}
impl Default for Lone { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DangerWhy {
    Body,
    Drop,
    Press,
    Rounds,
}
impl DangerWhy {
    pub const fn as_str(self) -> &'static str {
        match self {
            DangerWhy::Body => "body",
            DangerWhy::Drop => "drop",
            DangerWhy::Press => "press",
            DangerWhy::Rounds => "rounds",
        }
    }
}

/// (TypeScript: DangerRec, src/game/tactics.ts:124)
#[derive(Clone, Debug)]
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
impl DangerRec {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DangerRec {
            door: DoorId(0),
            team: crate::teams::Team::Guard,
            until: 0.0,
            t0: 0.0,
            why: crate::tactics::DangerWhy::Body,
            victim: None,
            banged: false,
            entered: false,
            called: false,
            summoned: None,
        }
    }
}
impl Default for DangerRec { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GatePhase {
    Slice,
    Bang,
    Cross,
    Stack,
    Done,
    Wait,
    Open,
    Toss,
    Stacked,
}
impl GatePhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            GatePhase::Slice => "slice",
            GatePhase::Bang => "bang",
            GatePhase::Cross => "cross",
            GatePhase::Stack => "stack",
            GatePhase::Done => "done",
            GatePhase::Wait => "wait",
            GatePhase::Open => "open",
            GatePhase::Toss => "toss",
            GatePhase::Stacked => "stacked",
        }
    }
}

/// (TypeScript: Gate, src/game/tactics.ts:126)
#[derive(Clone, Debug)]
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
    pub stall_ref: Option<V3>,
    pub stall_t: f64,
    pub can: Option<ItemId>,
    pub toss_t: f64,
    pub land_t: f64,
    pub bang_t: f64,
    pub goal_near: bool,
    pub opened: bool,
    pub crossed: bool,
    pub hs: Option<crate::tactics::GateHs>,
    pub action: crate::tactics::GateAction,
    pub thrower: Option<GuardId>,
}
impl Gate {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Gate {
            door: DoorId(0),
            rec: TacticsDangerRecId(0),
            phase: crate::tactics::GatePhase::Slice,
            t: 0.0,
            t0: 0.0,
            side: 0.0,
            near_a: 0.0,
            spot: [0.0; 3],
            face: 0.0,
            role: crate::tactics::GateRole::Lead,
            lead: None,
            slice_pts: Vec::new(),
            slice_look: Vec::new(),
            slice_i: 0.0,
            slice_t: 0.0,
            legs: Vec::new(),
            leg_i: 0.0,
            cross_at: 0.0,
            look: None,
            crouch: 0.0,
            path: None,
            path_i: 0.0,
            path_for: None,
            stall_ref: None,
            stall_t: 0.0,
            can: None,
            toss_t: 0.0,
            land_t: 0.0,
            bang_t: 0.0,
            goal_near: false,
            opened: false,
            crossed: false,
            hs: None,
            action: crate::tactics::GateAction::Slice,
            thrower: None,
        }
    }
}
impl Default for Gate { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GateRole {
    Lead,
    Second,
}
impl GateRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            GateRole::Lead => "lead",
            GateRole::Second => "second",
        }
    }
}

/// (TypeScript: inline at src/game/tactics.ts:137 Gate.hs, src/game/tactics.ts:137)
#[derive(Clone, Copy, Debug)]
pub struct GateHs {
    pub pos: V3,
    pub yaw: f64,
}
impl GateHs {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GateHs {
            pos: [0.0; 3],
            yaw: 0.0,
        }
    }
}
impl Default for GateHs { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GateAction {
    Slice,
    Toss,
}
impl GateAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            GateAction::Slice => "slice",
            GateAction::Toss => "toss",
        }
    }
}

/// (TypeScript: TState, src/game/tactics.ts:141)
#[derive(Clone, Debug)]
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
impl TState {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TState {
            plans: Vec::new(),
            roles: JsMap::new(),
            lone: JsMap::new(),
            danger: Vec::new(),
            gates: JsMap::new(),
            cleared: JsMap::new(),
            seen_ev: JsSet::new(),
            plan_serial: 0.0,
            last_lone_call_t: 0.0,
            deal_fail_t: 0.0,
        }
    }
}
impl Default for TState { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: TacticsSnap, src/game/tactics.ts:272)
#[derive(Clone, Debug)]
pub struct TacticsSnap {
    pub danger: Vec<crate::tactics::TacticsSnapDangerItem>,
    pub cleared: Vec<[f64; 3]>,
    pub last_lone_call_t: f64,
    pub deal_fail_t: f64,
}
impl TacticsSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TacticsSnap {
            danger: Vec::new(),
            cleared: Vec::new(),
            last_lone_call_t: 0.0,
            deal_fail_t: 0.0,
        }
    }
}
impl Default for TacticsSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/tactics.ts:272 TacticsSnap.danger, src/game/tactics.ts:272)
#[derive(Clone, Debug)]
pub struct TacticsSnapDangerItem {
    pub door: f64,
    pub team: crate::teams::Team,
    pub until: f64,
    pub t0: f64,
    pub why: crate::tactics::DangerWhy,
    pub victim: Option<String>,
    pub banged: bool,
    pub entered: bool,
    pub called: bool,
}
impl TacticsSnapDangerItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TacticsSnapDangerItem {
            door: 0.0,
            team: crate::teams::Team::Guard,
            until: 0.0,
            t0: 0.0,
            why: crate::tactics::DangerWhy::Body,
            victim: None,
            banged: false,
            entered: false,
            called: false,
        }
    }
}
impl Default for TacticsSnapDangerItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: TacMove, src/game/tactics.ts:496)
#[derive(Clone, Copy, Debug)]
pub struct TacMove {
    pub move_: f64,
    pub aim: Option<V3>,
    pub pied: bool,
    pub crouch: f64,
}
impl TacMove {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TacMove {
            move_: 0.0,
            aim: None,
            pied: false,
            crouch: 0.0,
        }
    }
}
impl Default for TacMove { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/tactics.ts:63 TACTICS_DEFAULTS, src/game/tactics.ts:63)
#[derive(Clone, Copy, Debug)]
pub struct TACTICSDEFAULTS {
    pub flank_wait: f64,
    pub lone_wait: f64,
    pub door_danger: f64,
    pub door_wait: f64,
}
impl TACTICSDEFAULTS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TACTICSDEFAULTS {
            flank_wait: 0.0,
            lone_wait: 0.0,
            door_danger: 0.0,
            door_wait: 0.0,
        }
    }
}
impl Default for TACTICSDEFAULTS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/tactics.ts:155 kit, src/game/tactics.ts:155)
#[derive(Clone, Copy, Debug)]
pub struct TacticsModKitValue {
    pub stuns: f64,
}
impl TacticsModKitValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TacticsModKitValue {
            stuns: 0.0,
        }
    }
}
impl Default for TacticsModKitValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/tactics.ts:160 TACTICS_DEBUG, src/game/tactics.ts:160)
#[derive(Clone, Copy, Debug)]
pub struct TACTICSDEBUG {
}
impl TACTICSDEBUG {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TACTICSDEBUG {
        }
    }
}
impl Default for TACTICSDEBUG { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/tactics.ts:610 loneCorner ->, src/game/tactics.ts:610)
#[derive(Clone, Copy, Debug)]
pub struct LoneCorner {
    pub p: V3,
    pub los: bool,
}
impl LoneCorner {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LoneCorner {
            p: [0.0; 3],
            los: false,
        }
    }
}
impl Default for LoneCorner { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/tactics.ts:720 stackSpot ->, src/game/tactics.ts:720)
#[derive(Clone, Copy, Debug)]
pub struct StackSpot {
    pub pos: V3,
    pub face: f64,
}
impl StackSpot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StackSpot {
            pos: [0.0; 3],
            face: 0.0,
        }
    }
}
impl Default for StackSpot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/tactics.ts:734 sliceStations ->, src/game/tactics.ts:734)
#[derive(Clone, Debug)]
pub struct SliceStations {
    pub pts: Vec<V3>,
    pub look: Vec<V3>,
}
impl SliceStations {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SliceStations {
            pts: Vec::new(),
            look: Vec::new(),
        }
    }
}
impl Default for SliceStations { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GateMateRole {
    Lead,
    Second,
}
impl GateMateRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            GateMateRole::Lead => "lead",
            GateMateRole::Second => "second",
        }
    }
}

/// (TypeScript: inline at src/game/tactics.ts:778 gateMate ->, src/game/tactics.ts:778)
#[derive(Clone, Debug)]
pub struct GateMate {
    pub gd: GuardId,
    pub g: crate::tactics::Gate,
}
impl GateMate {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GateMate {
            gd: GuardId(0),
            g: crate::tactics::Gate::zeroed(),
        }
    }
}
impl Default for GateMate { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/tactics.ts:1020 tacticsLook ->, src/game/tactics.ts:1020)
#[derive(Clone, Copy, Debug)]
pub struct TacticsLook {
    pub aim: V3,
    pub crouch: f64,
}
impl TacticsLook {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TacticsLook {
            aim: [0.0; 3],
            crouch: 0.0,
        }
    }
}
impl Default for TacticsLook { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct TacticsMod {
    /// (src/game/tactics.ts:148 states)
    pub states: Option<crate::tactics::TState>,
    /// (src/game/tactics.ts:155 kit)
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
    pub reroutes: i32,
}

/// (src/game/tactics.ts:63 TACTICS_DEFAULTS: a table nothing writes)
pub fn tactics_defaults() -> crate::tactics::TACTICSDEFAULTS { crate::tactics::TACTICSDEFAULTS { flank_wait: 6.0, lone_wait: 8.0, door_danger: 25.0, door_wait: 6.0 } }

/// (src/game/tactics.ts:64 LS_KEY)
pub const LS_KEY: &str = "twopointsix.tactics";

/// (src/game/tactics.ts:65 KEPT: a table nothing writes)
pub fn kept() -> [String; 4] { [String::from("flankWait"), String::from("loneWait"), String::from("doorDanger"), String::from("doorWait")] }

/// (src/game/tactics.ts:85 JOIN_M)
pub const JOIN_M: f64 = 6.0;

/// (src/game/tactics.ts:85 SHARE_M)
pub const SHARE_M: f64 = 3.0;

/// (src/game/tactics.ts:85 DISC_M)
pub const DISC_M: f64 = 4.0;

/// (src/game/tactics.ts:85 FLANK_MIN_DEG)
pub const FLANK_MIN_DEG: f64 = 90.0;

/// (src/game/tactics.ts:85 FLANK_MAX_RATIO)
pub const FLANK_MAX_RATIO: f64 = 2.2;

/// (src/game/tactics.ts:85 CORRIDOR_COST)
pub const CORRIDOR_COST: f64 = 6.0;

/// (src/game/tactics.ts:85 CORRIDOR_DILATE)
pub const CORRIDOR_DILATE: f64 = 1.0;

/// (src/game/tactics.ts:85 SECTOR_M)
pub const SECTOR_M: f64 = 5.5;

/// (src/game/tactics.ts:85 SET_M)
pub const SET_M: f64 = 5.0;

/// (src/game/tactics.ts:85 BEHIND_M)
pub const BEHIND_M: f64 = 3.0;

/// (src/game/tactics.ts:85 BEHIND_SLACK_M)
pub const BEHIND_SLACK_M: f64 = 0.8;

/// (src/game/tactics.ts:85 REPLAN_M)
pub const REPLAN_M: f64 = 4.0;

/// (src/game/tactics.ts:88 PLAN_CAP_SECS)
pub const PLAN_CAP_SECS: f64 = 45.0;

/// (src/game/tactics.ts:88 ROLE_STALL_SECS)
pub const ROLE_STALL_SECS: f64 = 4.0;

/// (src/game/tactics.ts:88 REPLAN_GAP_SECS)
pub const REPLAN_GAP_SECS: f64 = 1.0;

/// (src/game/tactics.ts:88 ARRIVE_M)
pub const ARRIVE_M: f64 = 1.3;

/// (src/game/tactics.ts:91 LONE_FIGHT_M)
pub const LONE_FIGHT_M: f64 = 5.0;

/// (src/game/tactics.ts:91 LONE_HOLD_MIN_M)
pub const LONE_HOLD_MIN_M: f64 = 5.5;

/// (src/game/tactics.ts:91 LONE_COVER_R)
pub const LONE_COVER_R: f64 = 1.6;

/// (src/game/tactics.ts:91 ARMED_SHOT_SECS)
pub const ARMED_SHOT_SECS: f64 = 60.0;

/// (src/game/tactics.ts:91 ARMED_SEEN_SECS)
pub const ARMED_SEEN_SECS: f64 = 30.0;

/// (src/game/tactics.ts:96 DOOR_NEAR_M)
pub const DOOR_NEAR_M: f64 = 2.2;

/// (src/game/tactics.ts:96 ROUND_AT_M)
pub const ROUND_AT_M: f64 = 3.0;

/// (src/game/tactics.ts:96 GATE_DETECT_M)
pub const GATE_DETECT_M: f64 = 3.5;

/// (src/game/tactics.ts:96 GATE_GOAL_M)
pub const GATE_GOAL_M: f64 = 2.0;

/// (src/game/tactics.ts:96 DOOR_COST_MIN)
pub const DOOR_COST_MIN: f64 = 20.0;

/// (src/game/tactics.ts:96 STACK_ALONG_M)
pub const STACK_ALONG_M: f64 = 0.9;

/// (src/game/tactics.ts:96 STACK_OFF_M)
pub const STACK_OFF_M: f64 = 0.55;

/// (src/game/tactics.ts:96 STACK_GAP_M)
pub const STACK_GAP_M: f64 = 0.85;

/// (src/game/tactics.ts:97 SLICE_BACK: a table nothing writes)
pub fn slice_back() -> Vec<f64> { alloc::vec![2.5, 2.0, 1.6, 1.25] }

/// (src/game/tactics.ts:97 SLICE_STEPS)
pub const SLICE_STEPS: f64 = 4.0;

/// (src/game/tactics.ts:97 SLICE_DWELL)
pub const SLICE_DWELL: f64 = 0.25;

/// (src/game/tactics.ts:97 SLICE_PACE)
pub const SLICE_PACE: f64 = 2.4;

/// (src/game/tactics.ts:97 ENTRY_IN_M)
pub const ENTRY_IN_M: f64 = 1.5;

/// (src/game/tactics.ts:97 ENTRY_SPLIT_DEG)
pub const ENTRY_SPLIT_DEG: f64 = 60.0;

/// (src/game/tactics.ts:97 ENTRY_LEG_M)
pub const ENTRY_LEG_M: f64 = 1.3;

/// (src/game/tactics.ts:97 GATE_CAP_SECS)
pub const GATE_CAP_SECS: f64 = 22.0;

/// (src/game/tactics.ts:97 SUMMON_M)
pub const SUMMON_M: f64 = 25.0;

/// (src/game/tactics.ts:99 TOSS_IN_M)
pub const TOSS_IN_M: f64 = 2.5;

/// (src/game/tactics.ts:99 TOSS_FUSE)
pub const TOSS_FUSE: f64 = 0.6;

/// (src/game/tactics.ts:99 TOSS_AIR_CAP)
pub const TOSS_AIR_CAP: f64 = 2.2;

/// (src/game/tactics.ts:99 GO_AFTER_BANG)
pub const GO_AFTER_BANG: f64 = 0.35;

/// (src/game/tactics.ts:160 TACTICS_DEBUG: a table nothing writes)
pub fn tactics_debug() -> crate::tactics::TACTICSDEBUG { crate::tactics::TACTICSDEBUG {  } }
