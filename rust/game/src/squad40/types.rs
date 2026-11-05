//! The types of `squad40`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Shot, src/game/squad40.ts:79)
#[derive(Clone, Debug)]
pub struct Shot {
    pub gd: GuardId,
    pub from: V3,
    pub to: V3,
    pub sol: crate::throwables::ThrowSolution,
    pub burst: V3,
    pub aimed: bool,
}
impl Shot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Shot {
            gd: GuardId(0),
            from: [0.0; 3],
            to: [0.0; 3],
            sol: crate::throwables::ThrowSolution::zeroed(),
            burst: [0.0; 3],
            aimed: false,
        }
    }
}
impl Default for Shot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Turn, src/game/squad40.ts:81)
#[derive(Clone, Debug)]
pub struct Turn {
    pub at: V3,
    pub t0: f64,
    pub fired_t: f64,
    pub shot: crate::squad40::Shot,
    pub mates: Vec<GuardId>,
    pub gd: GuardId,
    pub t: crate::orders::OrderTarget,
    pub a: Option<OrdersAssignmentId>,
    pub home: V3,
    pub home_yaw: f64,
    pub back: Option<crate::squad40::TurnBack>,
}
impl Turn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Turn {
            at: [0.0; 3],
            t0: 0.0,
            fired_t: 0.0,
            shot: crate::squad40::Shot::zeroed(),
            mates: Vec::new(),
            gd: GuardId(0),
            t: crate::orders::OrderTarget::Floor(crate::orders::OrderTargetFloor::zeroed()),
            a: None,
            home: [0.0; 3],
            home_yaw: 0.0,
            back: None,
        }
    }
}
impl Default for Turn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadthrow.ts:128 Turn.back, src/game/squadthrow.ts:128 Turn.back)
#[derive(Clone, Debug)]
pub struct TurnBack {
    pub post: Option<V3>,
    pub going: String,
    pub order: String,
    pub face: Option<f64>,
    pub there: String,
    pub t0: Option<f64>,
    pub follow: Option<f64>,
    pub watch: Option<V3>,
    pub door: Option<DoorId>,
    pub strafe: Option<bool>,
    pub stance: Option<crate::character::Stance>,
    pub upper: Option<crate::character::UpperMode>,
    pub stack_side: Option<f64>,
    pub move_: Option<crate::doortools::DoorToolsDoneItemLaneBackMove>,
    pub set_line: Option<String>,
    pub group: Option<f64>,
    pub set_said: Option<bool>,
    pub goal: Option<V3>,
    pub arrived: Option<bool>,
    pub held_set: Option<bool>,
    pub held_fire: Option<bool>,
    pub arr_t: Option<f64>,
    pub moved_t: Option<f64>,
    pub light: Option<bool>,
    pub low: Option<bool>,
    pub yield_t: Option<f64>,
    pub face_now: Option<f64>,
    pub search: Option<bool>,
    pub search_t: Option<f64>,
    pub via: Option<Vec<V3>>,
    pub anchor: Option<V3>,
    pub mark: Option<V3>,
    pub mark_t: Option<f64>,
    pub had: Option<V3>,
    pub back_n: Option<f64>,
    pub shy: Option<DoorId>,
    pub thru: Option<DoorId>,
    pub via_for: Option<V3>,
    pub via_t: Option<f64>,
    pub pace: Option<f64>,
    pub spare: Option<bool>,
    pub from: Option<V3>,
    pub foe: Option<GuardId>,
    pub crouch: Option<bool>,
    pub lean: Option<f64>,
    pub fit: Option<crate::roomfit::Fit>,
    pub tries: Option<crate::doortools::DoorToolsDoneItemLaneBackTries>,
}
impl TurnBack {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TurnBack {
            post: None,
            going: String::new(),
            order: String::new(),
            face: None,
            there: String::new(),
            t0: None,
            follow: None,
            watch: None,
            door: None,
            strafe: None,
            stance: None,
            upper: None,
            stack_side: None,
            move_: None,
            set_line: None,
            group: None,
            set_said: None,
            goal: None,
            arrived: None,
            held_set: None,
            held_fire: None,
            arr_t: None,
            moved_t: None,
            light: None,
            low: None,
            yield_t: None,
            face_now: None,
            search: None,
            search_t: None,
            via: None,
            anchor: None,
            mark: None,
            mark_t: None,
            had: None,
            back_n: None,
            shy: None,
            thru: None,
            via_for: None,
            via_t: None,
            pace: None,
            spare: None,
            from: None,
            foe: None,
            crouch: None,
            lean: None,
            fit: None,
            tries: None,
        }
    }
}
impl Default for TurnBack { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: State, src/game/squad40.ts:82)
#[derive(Clone, Debug)]
pub struct State {
    pub rounds: JsMap<GuardId, f64>,
    pub brought: f64,
    pub last: JsMap<GuardId, f64>,
    pub turns: JsMap<GuardId, crate::squad40::Turn>,
    pub noted: bool,
    pub shown: Option<crate::squad40::StateShown>,
    pub kept: JsMap<crate::squad40::Asker, crate::squad40::StateKeptValue>,
}
impl State {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        State {
            rounds: JsMap::new(),
            brought: 0.0,
            last: JsMap::new(),
            turns: JsMap::new(),
            noted: false,
            shown: None,
            kept: JsMap::new(),
        }
    }
}
impl Default for State { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squad40.ts:83 State.shown, src/game/squad40.ts:83)
#[derive(Clone, Debug)]
pub struct StateShown {
    pub t: f64,
    pub text: String,
}
impl StateShown {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateShown {
            t: 0.0,
            text: String::new(),
        }
    }
}
impl Default for StateShown { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Asker {
    Only,
    Valid,
}
impl Asker {
    pub const fn as_str(self) -> &'static str {
        match self {
            Asker::Only => "only",
            Asker::Valid => "valid",
        }
    }
}

/// (TypeScript: inline at src/game/squad40.ts:83 State.kept, src/game/squad40.ts:83)
#[derive(Clone, Debug)]
pub struct StateKeptValue {
    pub key: String,
    pub t: f64,
    pub a: crate::squad40::StateKeptValueA,
    pub who: Option<String>,
}
impl StateKeptValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateKeptValue {
            key: String::new(),
            t: 0.0,
            a: crate::squad40::StateKeptValueA::Text(String::new()),
            who: None,
        }
    }
}
impl Default for StateKeptValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum StateKeptValueA {
    Text(String),
    Shot(crate::squad40::Shot),
}

/// (TypeScript: Squad40Plan, src/game/squad40.ts:246)
#[derive(Clone, Debug)]
pub struct Squad40Plan {
    pub gd: GuardId,
    pub from: V3,
    pub points: Vec<V3>,
    pub land: V3,
    pub kind: crate::squad40::Squad40PlanKind,
    pub preview: Option<bool>,
    pub walk: Option<()>,
}
impl Squad40Plan {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Squad40Plan {
            gd: GuardId(0),
            from: [0.0; 3],
            points: Vec::new(),
            land: [0.0; 3],
            kind: crate::squad40::Squad40PlanKind::Shell,
            preview: None,
            walk: None,
        }
    }
}
impl Default for Squad40Plan { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Squad40PlanKind {
    Shell,
}
impl Squad40PlanKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Squad40PlanKind::Shell => "shell",
        }
    }
}

/// (TypeScript: inline at src/game/squad40.ts:90 SQUAD40_LOG, src/game/squad40.ts:90)
#[derive(Clone, Debug)]
pub struct Squad40ModSQUAD40LOGItem {
    pub t: f64,
    pub who: String,
    pub from: V3,
    pub at: V3,
    pub burst: V3,
    pub left: f64,
    pub turned: bool,
}
impl Squad40ModSQUAD40LOGItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Squad40ModSQUAD40LOGItem {
            t: 0.0,
            who: String::new(),
            from: [0.0; 3],
            at: [0.0; 3],
            burst: [0.0; 3],
            left: 0.0,
            turned: false,
        }
    }
}
impl Default for Squad40ModSQUAD40LOGItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squad40.ts:116 bodies ->, src/game/squad40.ts:116)
#[derive(Clone, Debug)]
pub struct Bodies {
    pub friends: Vec<CharId>,
    pub foes: Vec<CharId>,
    pub mates: Vec<CharId>,
}
impl Bodies {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Bodies {
            friends: Vec::new(),
            foes: Vec::new(),
            mates: Vec::new(),
        }
    }
}
impl Default for Bodies { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum ShotOf {
    Shot(crate::squad40::Shot),
    Close,
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum Plan {
    Text(String),
    Arm1(crate::squad40::PlanArm1),
}

/// (TypeScript: inline at src/game/squad40.ts:152 plan ->, src/game/squad40.ts:152)
#[derive(Clone, Debug)]
pub struct PlanArm1 {
    pub a: crate::squad40::Shot,
    pub who: Option<String>,
}
impl PlanArm1 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlanArm1 {
            a: crate::squad40::Shot::zeroed(),
            who: None,
        }
    }
}
impl Default for PlanArm1 { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct Squad40Mod {
    /// (src/game/squad40.ts:87 states)
    pub states: JsMap<PlayerId, crate::squad40::State>,
    /// (src/game/squad40.ts:90 SQUAD40_LOG)
    pub squad40_log: Vec<crate::squad40::Squad40ModSQUAD40LOGItem>,
}
impl Squad40Mod {
    pub fn blank() -> Self {
        Squad40Mod {
            states: JsMap::new(),
            squad40_log: Vec::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct Squad40Stats {
    pub staged: i32,
    pub fired: i32,
    pub turned: i32,
    pub refused: i32,
    pub gave_up: i32,
}

/// (src/game/squad40.ts:73 BREAK)
pub const BREAK: bool = false;

/// (src/game/squad40.ts:77 SHELL_THERE)
pub const SHELL_THERE: &str = "shellThere";

/// (src/game/squad40.ts:148 none)
pub const none: &str = "no 40 mm left.";
